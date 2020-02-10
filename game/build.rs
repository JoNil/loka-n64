use png;
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

struct Image {
    width: i32,
    height: i32,
    data: Vec<u8>,
}

fn load_png(path: &Path) -> Result<Image, Box<dyn Error>> {
    let decoder = png::Decoder::new(fs::File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    if info.color_type != png::ColorType::RGBA
        || info.bit_depth != png::BitDepth::Eight
        || info.buffer_size() != (4 * info.width * info.height) as usize
    {
        return Err("Image format not supported!")?;
    }

    Ok(Image {
        width: info.width as i32,
        height: info.height as i32,
        data: buf,
    })
}

fn rgba_to_5551(rgba: u32) -> u16 {
    let r = ((rgba >> 24) & 0xff) as f32 / 255.0;
    let g = ((rgba >> 16) & 0xff) as f32 / 255.0;
    let b = ((rgba >> 8) & 0xff) as f32 / 255.0;
    let a = if (rgba >> 0) & 0xff > 0 { 1 } else { 0 };

    (((r * 31.0) as u16) << 11) | (((g * 31.0) as u16) << 6) | (((b * 31.0) as u16) << 1) | a
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;

    let mut res = String::new();

    for path in fs::read_dir("textures")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("png")))
    {
        if let Some(name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let image = load_png(path.as_path())?;

            let out_path = path.canonicalize()?.with_extension("ntex");

            let mut out_image = Vec::with_capacity((2 * image.width * image.height) as usize);

            for pixel in image.data.chunks(4) {
                let u32_pixel = u32::from_ne_bytes(pixel.try_into()?);
                out_image.extend(&rgba_to_5551(u32_pixel).to_ne_bytes());
            }

            fs::write(&out_path, &out_image)?;

            res.push_str(&format!(
                "static {}: Texture = Texture::from_static({}, {}, include_bytes!({:?}));",
                name.to_uppercase(),
                image.width,
                image.height,
                out_path
            ));

            println!("rerun-if-changed={}", path.to_string_lossy());
        }
    }

    fs::write(format!("{}/texture_includes.rs", out_dir), res)?;

    Ok(())
}
