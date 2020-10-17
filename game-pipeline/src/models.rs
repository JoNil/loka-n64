use blend::{Blend, Instance};
use n64_math::{Vec2, Vec3};

use crate::utils::write_file_if_changed;
use std::{env, error::Error, ffi::OsStr, fs};

#[rustfmt::skip]
macro_rules! MODEL_TEMPLATE { () => {
r##"pub static {name}: StaticSoundData = StaticSoundData {{ data: include_bytes_align_as!(i16, {path:?}) }};
"##
}; }

#[rustfmt::skip]
macro_rules! MODELS_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

//use crate::sound::StaticSoundData;
//use n64::include_bytes_align_as;

{models}"##
}; }

#[derive(Debug)]
struct Mesh {
    verts: Vec<Vec3>,
    uvs: Vec<Vec2>,
    colors: Vec<u32>,
    indices: Vec<u8>,
}

#[derive(Debug)]
struct Object {
    name: String,
    mesh: Mesh,
}

/*fn instance_to_mesh(mesh: Instance) -> Option<Mesh> {
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

    let mut uv_buffer = vec![0f32; face_indice_count * 2];
    let mut normal_buffer = vec![0f32; face_indice_count * 3];
    let mut verts_array_buff = vec![0f32; face_indice_count * 3];

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

                //Normals are compressed into 16 bit integers
                let no = vert.get_i16_vec("no");
                normal_buffer[index_count * 3] = f32::from(no[0]) / 32767.0;
                normal_buffer[index_count * 3 + 1] = f32::from(no[1]) / 32767.0;
                normal_buffer[index_count * 3 + 2] = f32::from(no[2]) / 32767.0;

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

    let faces: Vec<_> = (&verts_array_buff[..])
        .chunks(3)
        .enumerate()
        .map(|(i, pos)| {
            (
                [pos[0], pos[1], pos[2]],
                [
                    normal_buffer[i * 3],
                    normal_buffer[i * 3 + 1],
                    normal_buffer[i * 3 + 2],
                ],
                [uv_buffer[i * 2], uv_buffer[i * 2 + 1]],
            )
        })
        .collect::<Vec<Vertex>>();

    let faces: Vec<_> = faces.chunks(3).map(|f| [f[0], f[1], f[2]]).collect();

    Some(Mesh { faces })
}*/

pub(crate) fn parse() -> Result<(), Box<dyn Error>> {
    let mut models = String::new();

    /*for path in fs::read_dir("models")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("blend")))
    {
        let blend = Blend::from_path(&path);

        for obj in blend.get_by_code(*b"OB") {
            if obj.is_valid("data") && obj.get("data").code()[0..=1] == *b"ME" {
                let name = obj.get("id").get_string("name");
                let position = obj.get_f32_vec("loc");
                let scale = obj.get_f32_vec("size");
                let rotation = obj.get_f32_vec("rot");

                let data = obj.get("data");

                if let Some(mesh) = instance_to_mesh(data) {
                    objects.push(Object {
                        name: obj.get("id").get_string("name"),
                        location: [loc[0], loc[1], loc[2]],
                        rotation: [rot[0], rot[1], rot[2]],
                        scale: [size[0], size[1], size[2]],
                        mesh,
                    });
                }
            }
        }

        panic!("{:?}", path);*/

    /*write_binary_file_if_changed(&out_path, wav.as_bytes())?;

        models.push_str(&format!(
            MODEL_TEMPLATE!(),
            name = name.to_uppercase(),
            path = out_path,
        ));
    }*/

    //let models = format!(MODELS_TEMPLATE!(), models = models);

    //write_file_if_changed(env::current_dir()?.join("src").join("models.rs"), models)?;

    Ok(())
}
