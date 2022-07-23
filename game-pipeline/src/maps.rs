use crate::{
    image::load_png, image::Image, utils::write_binary_file_if_changed,
    utils::write_file_if_changed,
};
use assert_into::AssertInto;
use std::{
    collections::HashMap,
    collections::HashSet,
    env,
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::BufReader,
    path::Path,
    path::PathBuf,
};
use tiled::{LayerData, Map, ObjectTemplate, Tileset};

#[rustfmt::skip]
macro_rules! TILE_TEMPLATE { () => {
r##"static {tile_ident}: StaticTexture = StaticTexture::from_static({width}, {height}, include_bytes_align_as!(Color, {tile_path:?}));
"##
}; }

#[rustfmt::skip]
macro_rules! TILE_IDENT_TEMPLATE { () => {
r##"    &{tile_ident},
"##
}; }

fn find_tileset_with_gid(gid: u32, tilesets: &[Tileset]) -> Result<&Tileset, Box<dyn Error>> {
    for tileset in tilesets {
        let effective_gid = gid as i32 - tileset.first_gid as i32;

        if effective_gid >= 0
            && (effective_gid as u32) < tileset.tilecount.ok_or("Tileset needs tilecount")?
        {
            return Ok(tileset);
        }
    }

    Err(format!("GID {} Not Found", gid).into())
}

fn load_tile_image(
    gid: u32,
    map_path: &Path,
    tileset: &Tileset,
    tileset_image_cache: &mut HashMap<PathBuf, Image>,
    width: i32,
    height: i32,
    rotate_180: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut effective_gid = gid - tileset.first_gid;

    {
        let tile_width = tileset.tile_width;
        let tile_height = tileset.tile_height;
        let tile_size = tile_width * tile_height;

        for tileset_image in tileset.images.iter() {
            let image_size = (tileset_image.width * tileset_image.height) as u32;
            let image_tiles = image_size / tile_size;

            let image_path = tileset
                .source
                .as_ref()
                .map(Path::new)
                .unwrap_or_else(|| Path::new(map_path))
                .with_file_name(&tileset_image.source);

            if effective_gid < image_tiles {
                assert!(tile_width == width as u32);
                assert!(tile_height == height as u32);

                let image = tileset_image_cache
                    .entry(image_path.clone())
                    .or_insert_with(|| {
                        println!("rerun-if-changed={:?}", image_path);
                        load_png(image_path, rotate_180, None).unwrap()
                    });

                let mut res = Vec::new();
                res.resize_with(2 * tile_size as usize, Default::default);

                let image_width_tiles = image.width as u32 / tile_width;

                let tile_x = effective_gid % image_width_tiles;
                let tile_y = effective_gid / image_width_tiles;

                let start_x = tile_x * tile_width;
                let start_y = tile_y * tile_height;

                let image_stride = image.width as u32;

                for y in 0..tile_height {
                    for x in 0..tile_width {
                        let out_index = 2 * (x + tile_width * y) as usize;
                        let image_index =
                            2 * ((start_x + x) + image_stride * (start_y + y)) as usize;

                        res[out_index] = image.data[image_index];
                        res[out_index + 1] = image.data[image_index + 1];
                    }
                }

                return Ok(res);
            }

            effective_gid -= image_tiles;
        }
    }

    for tile in &tileset.tiles {
        if let Some(image) = tile.images.get(0) {
            if tile.id == effective_gid {
                let image_path = tileset
                    .source
                    .as_ref()
                    .map(Path::new)
                    .unwrap_or_else(|| Path::new(map_path))
                    .with_file_name(&image.source);

                println!("rerun-if-changed={:?}", image_path);
                let image = load_png(image_path, rotate_180, Some((width, height))).unwrap();

                return Ok(image.data);
            }
        }
    }

    Err(format!("GID {} Not Found In Tileset Images", gid).into())
}

fn parse_map_tiles(
    out_dir: &Path,
    map_path: &Path,
    name: &str,
    uppercase_name: &str,
    map: &Map,
    used_tile_ids: &[u32],
    tileset_image_cache: &mut HashMap<PathBuf, Image>,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut map_tiles = Vec::new();
    let mut map_tile_refs = Vec::new();

    for id in used_tile_ids.iter() {
        if *id == 0 {
            continue;
        }

        let width: i32 = map.tile_width.assert_into();
        let height: i32 = map.tile_height.assert_into();

        let tile_path = out_dir
            .join(format!("{}_tile_{}", name, *id))
            .with_extension("ntex");
        let tile_path = tile_path.to_str().ok_or("Bad Path")?;

        let tileset = find_tileset_with_gid(*id, &map.tilesets)?;
        let tile_image = load_tile_image(
            *id,
            map_path,
            tileset,
            tileset_image_cache,
            width,
            height,
            false,
        )?;

        write_binary_file_if_changed(&tile_path, &tile_image)?;

        let tile_ident = format!("{}_TILE_{}", uppercase_name, id);

        let tile = format!(
            TILE_TEMPLATE!(),
            tile_ident = tile_ident,
            width = width,
            height = height,
            tile_path = tile_path,
        );

        let tile_ref = format!(TILE_IDENT_TEMPLATE!(), tile_ident = tile_ident);

        map_tiles.push(tile);
        map_tile_refs.push(tile_ref);
    }

    Ok((map_tiles, map_tile_refs))
}

#[rustfmt::skip]
macro_rules! OBJECT_TEXTURE_TEMPLATE { () => {
r##"static {object_texture_ident}: StaticTexture = StaticTexture::from_static({width}, {height}, include_bytes_align_as!(Color, {object_texture_path:?}));
"##
}; }

#[rustfmt::skip]
macro_rules! OBJECT_TEMPLATE { () => {
r##"    StaticObject {{
        x: {x}_f32,
        y: {y}_f32,
        texture: &{object_texture_ident},
    }},
"##
}; }

fn parse_map_objects(
    map: &Map,
    out_dir: &Path,
    map_path: &Path,
    tileset_image_cache: &mut HashMap<PathBuf, Image>,
    emitted_object_texture: &mut HashSet<String>,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut objects = Vec::new();
    let mut object_textures = Vec::new();

    for object_group in &map.object_groups {
        for object in &object_group.objects {
            if let Some(ObjectTemplate {
                object: Some(template_object),
                tileset: Some(tileset),
            }) = &object.template.as_deref()
            {
                let object_texture_ident = format!(
                    "OBJECT_TEXTURE_{}_{}{}_{}X{}",
                    tileset.name.to_uppercase(),
                    template_object.gid,
                    if template_object.rotation as i32 == 180 {
                        "_ROT_180"
                    } else {
                        ""
                    },
                    template_object.width,
                    template_object.height,
                );

                objects.push(format!(
                    OBJECT_TEMPLATE!(),
                    x = object.x - template_object.width / 2.0,
                    y = object.y - template_object.height / 2.0,
                    object_texture_ident = object_texture_ident,
                ));

                if !emitted_object_texture.contains(&object_texture_ident) {
                    let object_texture_path = out_dir
                        .join(format!("object_{}_{}", tileset.name, template_object.gid))
                        .with_extension("ntex");

                    let texture_image = load_tile_image(
                        template_object.gid,
                        map_path,
                        tileset,
                        tileset_image_cache,
                        template_object.width as i32,
                        template_object.height as i32,
                        template_object.rotation as i32 == 180,
                    )?;

                    assert!(
                        texture_image.len()
                            == 2 * template_object.width as usize * template_object.height as usize
                    );

                    write_binary_file_if_changed(&object_texture_path, &texture_image)?;

                    object_textures.push(format!(
                        OBJECT_TEXTURE_TEMPLATE!(),
                        object_texture_ident = object_texture_ident,
                        width = template_object.width as i32,
                        height = template_object.height as i32,
                        object_texture_path = object_texture_path,
                    ));

                    emitted_object_texture.insert(object_texture_ident);
                }
            }
        }
    }

    Ok((objects, object_textures))
}

#[rustfmt::skip]
macro_rules! MAP_TEMPLATE { () => {
r##"static {tiles_name_ident}: &[&StaticTexture] = &[
{map_tile_refs}];

{object_textures}
pub static {objects_name_ident}: &[&[StaticObject]] = &[&[
{objects}]];

pub static {map_name_ident}: &StaticMapData = &StaticMapData {{
    width_in_tiles: {map_width},
    height_in_tiles: {map_height},
    tile_width: {tile_width},
    tile_height: {tile_height},
    tiles: {tiles_name_ident},
    layers: include_bytes!({map_data_path:?}),
    objects: {objects_name_ident},
}};"##
}; }

#[rustfmt::skip]
macro_rules! MAPS_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

use crate::map::{{StaticMapData, StaticObject}};
use n64_math::Color;
use n64::gfx::StaticTexture;
use n64::include_bytes_align_as;

{tiles}
{maps}
"##
}; }

pub fn parse(out_dir: &Path) {
    let mut maps = Vec::new();
    let mut tiles = Vec::new();

    let mut used_tile_ids_map = HashMap::new();
    let mut used_tile_ids = Vec::new();

    used_tile_ids_map.insert(0, 0);
    used_tile_ids.push(0);

    let mut tileset_image_cache = HashMap::new();
    let mut emitted_object_texture = HashSet::new();

    for path in fs::read_dir("maps")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("tmx")))
    {
        if path.to_string_lossy().contains("map_2") {
            continue;
        }

        println!("rerun-if-changed={}", path.to_string_lossy());

        let name = path.file_stem().unwrap().to_str().unwrap();
        let uppercase_name = name.to_uppercase();

        let map = {
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            tiled::parse_with_path(reader, &path).unwrap()
        };

        let mut layers = Vec::new();

        for layer in map.layers.iter() {
            if let LayerData::Finite(tiles) = &layer.tiles {
                for row in tiles.iter() {
                    for tile in row.iter() {
                        if let Some(id) = used_tile_ids_map.get(&tile.gid) {
                            layers.push(*id);
                        } else {
                            let new_id = used_tile_ids.len().assert_into();
                            used_tile_ids_map.insert(tile.gid, new_id);
                            used_tile_ids.push(tile.gid);
                            layers.push(new_id);
                        }
                    }
                }
            }
        }

        let (map_tiles, map_tile_refs) = parse_map_tiles(
            out_dir,
            &path,
            name,
            &uppercase_name,
            &map,
            &used_tile_ids,
            &mut tileset_image_cache,
        )
        .unwrap();

        tiles.extend_from_slice(&map_tiles);

        let map_data_path = out_dir.join(name).with_extension("nmap");
        let map_data_path = map_data_path.to_str().unwrap();

        write_binary_file_if_changed(map_data_path, &layers).unwrap();

        let (objects, object_textures) = parse_map_objects(
            &map,
            out_dir,
            &path,
            &mut tileset_image_cache,
            &mut emitted_object_texture,
        )
        .unwrap();

        let map_name_ident = uppercase_name.to_string();
        let tiles_name_ident = format!("{}_TILES", &uppercase_name);
        let objects_name_ident = format!("{}_OBJECTS", &uppercase_name);
        let map_width = map.width as i32;
        let map_height = map.height as i32;
        let tile_width = map.tile_width as i32;
        let tile_height = map.tile_height as i32;

        let map = format!(
            MAP_TEMPLATE!(),
            map_name_ident = map_name_ident,
            tiles_name_ident = tiles_name_ident,
            map_tile_refs = map_tile_refs.join(""),
            map_width = map_width,
            map_height = map_height,
            tile_width = tile_width,
            tile_height = tile_height,
            map_data_path = map_data_path,
            object_textures = object_textures.join(""),
            objects = objects.join(""),
            objects_name_ident = objects_name_ident,
        );

        maps.push(map);
    }

    let maps = format!(
        MAPS_TEMPLATE!(),
        tiles = tiles.join(""),
        maps = maps.join(""),
    );

    write_file_if_changed(
        env::current_dir().unwrap().join("src").join("maps.rs"),
        maps,
    )
    .unwrap();
}
