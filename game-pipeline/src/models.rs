use crate::utils::{write_binary_file_if_changed, write_file_if_changed};
use blend::{Blend, Instance};
use meshopt::generate_vertex_remap;
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
    let uvs = mesh.get_iter("mloopuv").collect::<Vec<_>>();
    let verts = mesh.get_iter("mvert").collect::<Vec<_>>();

    let mut index_count = 0;
    let mut face_indice_count = 0;
    for face in &faces {
        let len = face.get_i32("totloop");
        let mut indexi = 1;

        while indexi < len {
            face_indice_count += 3;
            indexi += 2;
        }
    }

    let mut verts_array_buff = vec![Vec3::zero(); face_indice_count];
    let mut uv_buffer = vec![Vec2::<ero(); face_indice_count];

    for face in &faces {
        let len = face.get_i32("totloop");
        let start = face.get_i32("loopstart");
        let mut indexi = 1;

        while indexi < len {
            let mut index;

            for l in 0..3 {
                if (indexi - 1) + l < len {
                    index = start + (indexi - 1) + l;
                } else {
                    index = start;
                }

                let v = loops[index as usize].get_i32("v");
                let vert = &verts[v as usize];

                let co = vert.get_f32_vec("co");
                verts_array_buff[index_count * 3] = co[0];
                verts_array_buff[index_count * 3 + 1] = co[1];
                verts_array_buff[index_count * 3 + 2] = co[2];

                let uv = uvs[index as usize].get_f32_vec("uv");
                let uv_x = uv[0];
                let uv_y = uv[1];
                uv_buffer[index_count * 2] = uv_x;
                uv_buffer[index_count * 2 + 1] = uv_y;

                index_count += 1;
            }

            indexi += 2;
        }
    }

    let vertex_remap = generate_vertex_remap(, None)

    Some(Model {
        verts: vec![Default::default(); 16],
        uvs: vec![Default::default(); 16],
        colors: vec![Default::default(); 16],
        indices: vec![Default::default(); 16],
    })
}

pub(crate) fn parse() -> Result<(), Box<dyn Error>> {
    let mut models = String::new();

    for path in fs::read_dir("models")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("blend")))
    {
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

                        write_binary_file_if_changed(&verts_path, model.verts.as_bytes())?;
                        write_binary_file_if_changed(&uvs_path, model.uvs.as_bytes())?;
                        write_binary_file_if_changed(&colors_path, model.colors.as_bytes())?;
                        write_binary_file_if_changed(&indices_path, model.indices.as_bytes())?;

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
