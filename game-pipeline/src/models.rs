use crate::utils::{write_binary_file_if_changed, write_file_if_changed};
use assert_into::AssertInto;
use blend::{Blend, Instance};
use meshopt::{generate_vertex_remap, remap_index_buffer, remap_vertex_buffer};
use n64_math::{vec2, Vec2, Vec3};
use std::{env, ffi::OsStr, fs, path::Path};
use zerocopy::AsBytes;

#[rustfmt::skip]
macro_rules! MODEL_TEMPLATE { () => {
r##"pub static {name}: StaticModelData = StaticModelData {{
    verts: include_bytes_align_as!(Vec3, {verts_path:?}),
    uvs: include_bytes_align_as!(Vec2, {uvs_path:?}),
    colors: include_bytes_align_as!(u32, {colors_path:?}),
    indices: include_bytes_align_as!(u8, {indices_path:?}),
    size: const_vec2!([{model_width}_f32, {model_height}_f32]),
}};
"##
}; }

#[rustfmt::skip]
macro_rules! MODELS_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

use crate::model::StaticModelData;
use n64::include_bytes_align_as;
use n64_math::{{Vec2, Vec3, const_vec2}};

{models}"##
}; }

#[derive(Debug)]
struct Model {
    verts: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<u32>,
    indices: Vec<u8>,
    size: Vec2,
}

fn parse_model(mesh: Instance) -> Option<Model> {
    if !mesh.is_valid("mpoly")
        || !mesh.is_valid("mloop")
        || !mesh.is_valid("mvert")
        || !mesh.is_valid("mloopuv")
        || !mesh.is_valid("mloopcol")
    {
        return None;
    }

    let faces = mesh.get_iter("mpoly").collect::<Vec<_>>();
    let loops = mesh.get_iter("mloop").collect::<Vec<_>>();
    let mverts = mesh.get_iter("mvert").collect::<Vec<_>>();
    let muvs = mesh.get_iter("mloopuv").collect::<Vec<_>>();
    let mcols = mesh.get_iter("mloopcol").collect::<Vec<_>>();

    let mut face_indice_count = 0;
    for face in &faces {
        let len = face.get_i32("totloop");
        let mut indexi = 1;

        while indexi < len {
            face_indice_count += 3;
            indexi += 2;
        }
    }

    let mut verts = vec![[0.0; 3]; face_indice_count];
    let mut uvs = vec![[0.0; 2]; face_indice_count];
    let mut colors = vec![0; face_indice_count];

    let mut index_count = 0;

    let mut max_x = f32::MIN;
    let mut min_x = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_y = f32::MAX;

    for face in &faces {
        let len = face.get_i32("totloop");
        let start = face.get_i32("loopstart");
        let mut indexi = 1;

        while indexi < len {
            for l in 0..3 {
                let index = if (indexi - 1) + l < len {
                    start + (indexi - 1) + l
                } else {
                    start
                };

                let v = loops[index as usize].get_i32("v");

                let co = mverts[v as usize].get_f32_vec("co");
                verts[index_count as usize] = [co[0], co[1], co[2]];

                max_x = max_x.max(co[0]);
                min_x = min_x.min(co[0]);
                max_y = max_y.max(co[1]);
                min_y = min_y.min(co[1]);

                let uv = muvs[index as usize].get_f32_vec("uv");
                uvs[index_count as usize] = [uv[0], uv[1]];

                colors[index_count as usize] = ((mcols[index as usize].get_u8("r")) as u32) << 24
                    | ((mcols[index as usize].get_u8("g")) as u32) << 16
                    | ((mcols[index as usize].get_u8("b")) as u32) << 8
                    | (mcols[index as usize].get_u8("a")) as u32;

                index_count += 1;
            }

            indexi += 2;
        }
    }

    if index_count > 255 {
        panic!("Only 255 indices per model are supported");
    }

    let vertex_remap = generate_vertex_remap(&verts, None);
    let mut verts = remap_vertex_buffer(&verts, vertex_remap.0, &vertex_remap.1);
    let uvs = remap_vertex_buffer(&uvs, vertex_remap.0, &vertex_remap.1);
    let colors = remap_vertex_buffer(&colors, vertex_remap.0, &vertex_remap.1);

    let indices = remap_index_buffer(None, face_indice_count, &vertex_remap.1)
        .iter()
        .copied()
        .map(|i| i.assert_into())
        .collect::<Vec<u8>>();

    let offset = (Vec3::new(max_x, max_y, 0.0) + Vec3::new(min_x, min_y, 0.0)) / 2.0;

    for vert in &mut verts {
        vert[0] -= offset.x;
        vert[1] -= offset.y;
        vert[2] -= offset.z;
    }

    Some(Model {
        verts,
        uvs,
        colors,
        indices,
        size: vec2(max_x - min_x, max_y - min_y),
    })
}

fn byteswap_u32_slice(data: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(data.len());

    for part in data.chunks_exact(4) {
        res.push(part[3]);
        res.push(part[2]);
        res.push(part[1]);
        res.push(part[0]);
    }

    res
}

fn parse_gltf_model(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> Option<Model> {
    if mesh.primitives().count() > 1 {
        panic!("Only one primitive per gltf file is supported");
    }

    if let Some(primitive) = mesh.primitives().next() {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let mut verts = reader.read_positions().unwrap().collect::<Vec<_>>();
        let uvs = reader
            .read_tex_coords(0)
            .unwrap()
            .into_f32()
            .collect::<Vec<_>>();
        let colors = reader
            .read_colors(0)
            .unwrap()
            .into_rgba_u8()
            .map(|c| {
                ((c[0] as u32) << 24) | ((c[1] as u32) << 16) | ((c[2] as u32) << 8) | (c[3] as u32)
            })
            .collect::<Vec<_>>();
        let indices = reader
            .read_indices()
            .unwrap()
            .into_u32()
            .map(|i| {
                if i > 255 {
                    panic!("Only 255 indices per model are supported");
                }
                i as u8
            })
            .collect::<Vec<_>>();

        let size = primitive.bounding_box();

        let offset = (Vec3::from_slice(&size.max) + Vec3::from_slice(&size.min)) / 2.0;

        for vert in &mut verts {
            vert[0] -= offset.x;
            vert[1] -= offset.y;
            vert[2] -= offset.z;
        }

        return Some(Model {
            verts,
            uvs,
            colors,
            indices,
            size: Vec2::new(size.max[0] - size.min[0], size.max[1] - size.min[1]),
        });
    }

    None
}

pub(crate) fn parse() {
    let mut models = String::new();

    for path in fs::read_dir("models")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("blend")))
    {
        println!("rerun-if-changed={:?}", &path);

        if let Some(file_name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let blend = Blend::from_path(&path).unwrap();

            for obj in blend.instances_with_code(*b"OB") {
                if obj.is_valid("data") && obj.get("data").code()[0..=1] == *b"ME" {
                    let data = obj.get("data");

                    let name = format!("{}", file_name);
                    let out_base_path = path.canonicalize().unwrap().with_file_name(&name);

                    if let Some(model) = parse_model(data) {
                        output_model(&mut models, &name, &out_base_path, &model);
                    }
                    break;
                }
            }
        }
    }

    for path in fs::read_dir("models")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("glb")))
    {
        println!("rerun-if-changed={:?}", &path);

        if let Some(file_name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let name = format!("{}", file_name);
            let out_base_path = path.canonicalize().unwrap().with_file_name(&name);

            let (gltf, buffers, _) = gltf::import(&path).unwrap();

            for mesh in gltf.meshes() {
                if let Some(model) = parse_gltf_model(&mesh, &buffers) {
                    output_model(&mut models, &name, &out_base_path, &model);
                    break;
                }
            }
        }
    }

    let models = format!(MODELS_TEMPLATE!(), models = models);

    write_file_if_changed(
        env::current_dir().unwrap().join("src").join("models.rs"),
        models,
    )
    .unwrap();
}

fn output_model(models: &mut String, name: &str, out_base_path: &Path, model: &Model) {
    let verts_path = out_base_path.with_extension("nvert");
    let uvs_path = out_base_path.with_extension("nuv");
    let colors_path = out_base_path.with_extension("ncol");
    let indices_path = out_base_path.with_extension("nind");

    write_binary_file_if_changed(&verts_path, byteswap_u32_slice(model.verts.as_bytes())).unwrap();
    write_binary_file_if_changed(&uvs_path, byteswap_u32_slice(model.uvs.as_bytes())).unwrap();
    write_binary_file_if_changed(&colors_path, byteswap_u32_slice(model.colors.as_bytes()))
        .unwrap();
    write_binary_file_if_changed(&indices_path, model.indices.as_bytes()).unwrap();

    models.push_str(&format!(
        MODEL_TEMPLATE!(),
        name = name.to_uppercase(),
        verts_path = verts_path,
        uvs_path = uvs_path,
        colors_path = colors_path,
        indices_path = indices_path,
        model_width = model.size.x,
        model_height = model.size.y,
    ));
}
