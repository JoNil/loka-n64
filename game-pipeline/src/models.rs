use crate::utils::{write_binary_file_if_changed, write_file_if_changed};
use assert_into::AssertInto;
use blend::{Blend, Instance};
use meshopt::{generate_vertex_remap, remap_index_buffer, remap_vertex_buffer};
use n64_math::{Vec2, Vec3};
use std::{env, error::Error, ffi::OsStr, fs};
use zerocopy::AsBytes;

#[rustfmt::skip]
macro_rules! MODEL_TEMPLATE { () => {
r##"pub static {name}: StaticModelData = StaticModelData {{
    verts: include_bytes_align_as!(Vec3, {verts_path:?}),
    uvs: include_bytes_align_as!(Vec2, {uvs_path:?}),
    colors: include_bytes_align_as!(u32, {colors_path:?}),
    indices: include_bytes_align_as!(u8, {indices_path:?}),
}};
"##
}; }

#[rustfmt::skip]
macro_rules! MODELS_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

use crate::model::StaticModelData;
use n64::include_bytes_align_as;
use n64_math::{{Vec2, Vec3}};

{models}"##
}; }

#[derive(Debug)]
struct Model {
    verts: Vec<Vec3>,
    uvs: Vec<Vec2>,
    colors: Vec<u32>,
    indices: Vec<u8>,
}

fn parse_model(mesh: Instance) -> Option<Model> {
    if !mesh.is_valid("mpoly")
        || !mesh.is_valid("mloop")
        || !mesh.is_valid("mloopuv")
        || !mesh.is_valid("mvert")
    {
        return None;
    }

    let faces = mesh.get_iter("mpoly").collect::<Vec<_>>();
    let loops = mesh.get_iter("mloop").collect::<Vec<_>>();
    let muvs = mesh.get_iter("mloopuv").collect::<Vec<_>>();
    let mverts = mesh.get_iter("mvert").collect::<Vec<_>>();

    let mut face_indice_count = 0;
    for face in &faces {
        let len = face.get_i32("totloop");
        let mut indexi = 1;

        while indexi < len {
            face_indice_count += 3;
            indexi += 2;
        }
    }

    let mut verts = vec![Vec3::zero(); face_indice_count];
    let mut uvs = vec![Vec2::zero(); face_indice_count];

    let mut index_count = 0;

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
                let vert = &mverts[v as usize];

                let co = vert.get_f32_vec("co");
                verts[index_count as usize] = Vec3::new(co[0], co[1], co[2]);

                let uv = muvs[index as usize].get_f32_vec("uv");
                uvs[index_count as usize] = Vec2::new(uv[0], uv[1]);

                index_count += 1;
            }

            indexi += 2;
        }
    }

    let vertex_remap = generate_vertex_remap(&verts, None);
    let verts = remap_vertex_buffer(&verts, vertex_remap.0, &vertex_remap.1);
    let uvs = remap_vertex_buffer(&uvs, vertex_remap.0, &vertex_remap.1);

    let indices = remap_index_buffer(None, face_indice_count, &vertex_remap.1)
        .iter()
        .copied()
        .map(|i| i.assert_into())
        .collect::<Vec<u8>>();

    Some(Model {
        verts,
        uvs,
        colors: vec![0x801010ff; vertex_remap.0],
        indices,
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

pub(crate) fn parse() -> Result<(), Box<dyn Error>> {
    let mut models = String::new();

    for path in fs::read_dir("models")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("blend")))
    {
        println!("rerun-if-changed={:?}", &path);

        if let Some(file_name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let blend = Blend::from_path(&path);

            for obj in blend.get_by_code(*b"OB") {
                if obj.is_valid("data") && obj.get("data").code()[0..=1] == *b"ME" {
                    let model_name = obj.get("id").get_string("name");
                    let model_name = model_name.trim_start_matches("OB");
                    let data = obj.get("data");

                    let name = format!("{}_{}", file_name, model_name);
                    let out_base_path = path.canonicalize()?.with_file_name(&name);

                    if let Some(model) = parse_model(data) {
                        let verts_path = out_base_path.with_extension("nvert");
                        let uvs_path = out_base_path.with_extension("nuv");
                        let colors_path = out_base_path.with_extension("ncol");
                        let indices_path = out_base_path.with_extension("nind");

                        write_binary_file_if_changed(
                            &verts_path,
                            byteswap_u32_slice(model.verts.as_bytes()),
                        )?;
                        write_binary_file_if_changed(
                            &uvs_path,
                            byteswap_u32_slice(model.uvs.as_bytes()),
                        )?;
                        write_binary_file_if_changed(
                            &colors_path,
                            byteswap_u32_slice(model.colors.as_bytes()),
                        )?;
                        write_binary_file_if_changed(
                            &indices_path,
                            byteswap_u32_slice(model.indices.as_bytes()),
                        )?;

                        models.push_str(&format!(
                            MODEL_TEMPLATE!(),
                            name = name.to_uppercase(),
                            verts_path = verts_path,
                            uvs_path = uvs_path,
                            colors_path = colors_path,
                            indices_path = indices_path,
                        ));
                    }
                }
            }
        }
    }

    let models = format!(MODELS_TEMPLATE!(), models = models);

    write_file_if_changed(env::current_dir()?.join("src").join("models.rs"), models)?;

    Ok(())
}
