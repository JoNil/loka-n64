use n64_math::Color;
use png;
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::{collections::HashMap, io::BufReader, path::{PathBuf, Path}};
use tiled::{Map, Tileset};

struct Image {
    width: i32,
    height: i32,
    data: Vec<u8>,
}

fn write_file_if_changed(
    path: impl AsRef<Path>,
    content: impl AsRef<str>,
) -> Result<(), Box<dyn Error>> {
    let s = match fs::read_to_string(path.as_ref()) {
        Ok(s) => s,
        Err(_) => {
            return fs::write(path.as_ref(), content.as_ref())
                .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
        }
    };

    if s != content.as_ref() {
        return fs::write(path.as_ref(), content.as_ref())
            .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
    }

    Ok(())
}

fn write_binary_file_if_changed(
    path: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
) -> Result<(), Box<dyn Error>> {
    let s = match fs::read(path.as_ref()) {
        Ok(s) => s,
        Err(_) => {
            return fs::write(path.as_ref(), content.as_ref())
                .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
        }
    };

    if s != content.as_ref() {
        return fs::write(path.as_ref(), content.as_ref())
            .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
    }

    Ok(())
}

fn load_png(path: impl AsRef<Path>) -> Result<Image, Box<dyn Error>> {
    let file = File::open(path.as_ref())
        .map_err(|e| format!("Unable to open {}: {}", path.as_ref().to_string_lossy(), e))?;
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    if info.color_type != png::ColorType::RGBA
        || info.bit_depth != png::BitDepth::Eight
        || info.buffer_size() != (4 * info.width * info.height) as usize
    {
        return Err("Image format not supported!")?;
    }

    let mut data = Vec::with_capacity((2 * info.width * info.height) as usize);

    for pixel in buf.chunks_exact(4) {
        let color = Color::from_bytes(pixel.try_into()?);
        data.extend(&color.value().to_be_bytes());
    }

    Ok(Image {
        width: info.width as i32,
        height: info.height as i32,
        data,
    })
}

fn parse_textures(out_dir: &str) -> Result<(), Box<dyn Error>> {
    let mut res = String::new();

    for path in fs::read_dir("textures")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("png")))
    {
        println!("rerun-if-changed={}", path.to_string_lossy());

        if let Some(name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let out_path = path.canonicalize()?.with_extension("ntex");
            let image = load_png(path.as_path())?;

            write_binary_file_if_changed(&out_path, &image.data)?;

            res.push_str(&format!(
                "pub static {name}: StaticTexture = StaticTexture::from_static({width}, {height}, include_bytes!({path:?}));\n",
                name = name.to_uppercase(),
                width = image.width,
                height = image.height,
                path = out_path
            ));
        }
    }

    write_file_if_changed(Path::new(out_dir).join("texture_includes.rs"), res)?;

    Ok(())
}

#[rustfmt::skip]
macro_rules! TILE_TEMPLATE { () => {
r##"static {tile_ident}: StaticTexture = StaticTexture::from_static({width}, {height}, include_bytes!({tile_path:?}));
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
            return dbg!(Ok(tileset));
        }
    }

    Err(format!("GID {} Not Found", gid).into())
}

fn load_tile_image(
    gid: u32,
    map_path: &Path,
    tileset: &Tileset,
    tileset_image_cache: &mut HashMap<PathBuf, Image>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut effective_gid = gid - tileset.first_gid;
    let tile_size = tileset.tile_width * tileset.tile_height;

    for tileset_image in tileset.images.iter() {
        let image_size = (tileset_image.width * tileset_image.height) as u32;
        let image_tiles = image_size / tile_size;

        let image_path = map_path.join(&tileset.source).join(&tileset_image.source);

        if effective_gid < image_tiles {
            let image = tileset_image_cache
                .entry(image_path.clone())
                .or_insert_with(|| load_png(image_path).unwrap());

            let res = Vec::new();

            panic!("{}", effective_gid);

            return Ok(res);
        }

        effective_gid -= image_tiles;
    }

    Err(format!("GID {} Not Found In Tileset Images", gid).into())
}

fn parse_map_tiles(
    out_dir: &str,
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
        let width: i32 = map.tile_width.try_into().unwrap();
        let height: i32 = map.tile_height.try_into().unwrap();

        let tile_path = Path::new(out_dir)
            .join(format!("{}_tile_{}", name, *id))
            .with_extension("ntex");
        let tile_path = tile_path.to_str().ok_or("Bad Path")?;

        let tileset = find_tileset_with_gid(*id, &map.tilesets)?;
        let tile_image = load_tile_image(*id, map_path, tileset, tileset_image_cache)?;

        write_binary_file_if_changed(&tile_path, &tile_image)?;

        let tile_ident = format!("{}_TILE_{}", uppercase_name, id);

        let tile = format!(
            TILE_TEMPLATE!(),
            tile_ident = tile_ident,
            width = width,
            height = height,
            tile_path = tile_path,
        );

        let tile_ref = format!(TILE_IDENT_TEMPLATE!(), tile_ident = tile_ident,);

        map_tiles.push(tile);
        map_tile_refs.push(tile_ref);
    }

    Ok((map_tiles, map_tile_refs))
}

#[rustfmt::skip]
macro_rules! MAP_TEMPLATE { () => {
r##"pub static {tiles_name_ident}: &'static [&'static StaticTexture] = &[
{map_tile_refs}];

pub static {map_name_ident}: &'static StaticMapData = &StaticMapData {{
    width: {map_width},
    height: {map_height},
    layers: include_bytes!({map_data_path:?}),
}};"##
}; }

#[rustfmt::skip]
macro_rules! MAPS_TEMPLATE { () => {
r##"// This file is generated

use crate::map::{{StaticTileDesc, StaticMapData}};
use crate::textures::SHIP_2_SMALL;
use n64::gfx::StaticTexture;
use n64_math::Vec2;

{tiles}
{maps}

"##
}; }

fn parse_maps(out_dir: &str) -> Result<(), Box<dyn Error>> {
    let mut maps = Vec::new();
    let mut tiles = Vec::new();

    let mut used_tile_ids_map = HashMap::new();
    let mut used_tile_ids = Vec::new();

    let mut tileset_image_cache = HashMap::new();

    for path in fs::read_dir("maps")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("tmx")))
    {
        println!("rerun-if-changed={}", path.to_string_lossy());

        let name = path
            .file_stem()
            .ok_or("No File Stem")?
            .to_str()
            .ok_or("Bad Os String")?;
        let uppercase_name = name.to_uppercase();

        let map_path = Path::new("maps");

        let map = {
            let file =
                File::open(&path).map_err(|e| format!("Unable to open {}: {}", path.to_string_lossy(), e))?;
            let reader = BufReader::new(file);
            tiled::parse_with_path(reader, &map_path)?
        };

        let mut layers = Vec::new();

        for layer in map.layers.iter() {
            for row in layer.tiles.iter() {
                for tile in row.iter() {
                    if tile.gid == 0 {
                        continue;
                    }

                    if let Some(id) = used_tile_ids_map.get(&tile.gid) {
                        layers.push(*id);
                    } else {
                        let new_id = used_tile_ids.len().try_into().unwrap();
                        used_tile_ids_map.insert(tile.gid, new_id);
                        used_tile_ids.push(tile.gid);
                        layers.push(new_id);
                    }
                }
            }
        }

        let (map_tiles, map_tile_refs) = parse_map_tiles(
            out_dir,
            &map_path,
            &name,
            &uppercase_name,
            &map,
            &used_tile_ids,
            &mut tileset_image_cache,
        )?;

        tiles.extend_from_slice(&map_tiles);

        let map_data_path = Path::new(out_dir).join(name).with_extension("nmap");
        let map_data_path = map_data_path.to_str().ok_or("Bad Path")?;

        write_binary_file_if_changed(map_data_path, &layers)?;

        let map_name_ident = format!("{}", &uppercase_name);
        let tiles_name_ident = format!("{}_TILES", &uppercase_name);
        let map_width = map.width as i32;
        let map_height = map.height as i32;

        let map = format!(
            MAP_TEMPLATE!(),
            map_name_ident = map_name_ident,
            tiles_name_ident = tiles_name_ident,
            map_tile_refs = map_tile_refs.join(""),
            map_width = map_width,
            map_height = map_height,
            map_data_path = map_data_path,
        );

        maps.push(map);
    }

    let maps = format!(
        MAPS_TEMPLATE!(),
        tiles = tiles.join(""),
        maps = maps.join(""),
    );

    write_file_if_changed(env::current_dir()?.join("src").join("maps.rs"), maps)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;

    parse_textures(&out_dir)?;
    parse_maps(&out_dir)?;

    Ok(())
}
