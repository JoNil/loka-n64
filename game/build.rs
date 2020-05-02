use png;
use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use n64_math::Color;

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
                let color = Color::from_bytes(pixel.try_into()?);
                out_image.extend(&color.value().to_le_bytes());
            }

            fs::write(&out_path, &out_image)?;

            res.push_str(&format!(
                "pub static {}: Texture = Texture::from_static({}, {}, include_bytes!({:?}));\n",
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
