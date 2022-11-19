use assert_into::AssertInto;
use image::{imageops::FilterType, DynamicImage};
use n64_math::Color;
use std::{error::Error, fs::File, path::Path};

pub(crate) struct Image {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) data: Vec<u8>,
}

pub(crate) fn load_png(
    path: impl AsRef<Path>,
    rotate_180: bool,
    size: Option<(i32, i32)>,
) -> Result<Image, Box<dyn Error>> {
    println!("rerun-if-changed={}", path.as_ref().to_string_lossy());

    let file = File::open(path.as_ref())
        .map_err(|e| format!("Unable to open {}: {}", path.as_ref().to_string_lossy(), e))?;
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf)?;

    let info = reader.info();

    if info.color_type != png::ColorType::Rgba
        || info.bit_depth != png::BitDepth::Eight
        || reader.output_buffer_size() != (4 * info.width * info.height) as usize
    {
        return Err("Image format not supported!".into());
    }

    let mut image = image::ImageBuffer::from_raw(info.width, info.height, buf).unwrap();

    if rotate_180 {
        let rotated_image = DynamicImage::ImageRgba8(image).rotate180();
        image = rotated_image.into_rgba8();
    }

    if let Some((width, height)) = size {
        if info.width != width.assert_into() || info.height != height.assert_into() {
            let buf = image.into_raw();

            let mut color_in = Vec::with_capacity((3 * info.width * info.height).assert_into());
            let mut alpha_in = Vec::with_capacity((info.width * info.height).assert_into());

            for p in buf.chunks_exact(4) {
                color_in.push(p[0]);
                color_in.push(p[1]);
                color_in.push(p[2]);
                alpha_in.push(p[3]);
            }

            let color_image =
                image::ImageBuffer::from_raw(info.width, info.height, color_in).unwrap();
            let alpha_image =
                image::ImageBuffer::from_raw(info.width, info.height, alpha_in).unwrap();

            let scaled_color_image = DynamicImage::ImageRgb8(color_image).resize_exact(
                width as u32,
                height as u32,
                FilterType::Gaussian,
            );

            let scaled_alpha_image = DynamicImage::ImageLuma8(alpha_image).resize_exact(
                width as u32,
                height as u32,
                FilterType::Nearest,
            );

            let color_out = scaled_color_image.into_rgb8();
            let alpha_out = scaled_alpha_image.into_luma8();

            let mut out_buf = Vec::with_capacity((4 * width * height).assert_into());

            for (color, alpha) in color_out.chunks_exact(3).zip(alpha_out.iter()) {
                out_buf.push(color[0]);
                out_buf.push(color[1]);
                out_buf.push(color[2]);
                out_buf.push(*alpha);
            }

            image = image::ImageBuffer::from_raw(width as u32, height as u32, out_buf).unwrap();
        }
    }

    let image_width = image.width().assert_into();
    let image_height = image.height().assert_into();
    let buf = image.into_raw();

    let mut data = Vec::with_capacity((2 * image_width * image_height) as usize);

    for pixel in buf.chunks_exact(4) {
        let color = Color::from_bytes(pixel.assert_into());
        data.extend(color.value().to_be_bytes());
    }

    Ok(Image {
        width: image_width,
        height: image_height,
        data,
    })
}
