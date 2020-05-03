
pub(crate) static TEXUTRE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub(crate) struct DstTexture {
    pub buffer: wgpu::Buffer,
    pub tex_extent: wgpu::Extent3d,
    pub tex: wgpu::Texture,
    pub tex_view: wgpu::TextureView,
}

impl DstTexture {
    pub(crate) fn new(device: &wgpu::Device, width: i32, height: i32) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (4 * width * height) as u64,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        });

        let tex_extent = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth: 1,
        };
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TEXUTRE_FORMAT,
            usage: wgpu::TextureUsage::COPY_DST
                | wgpu::TextureUsage::COPY_SRC
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });
        let tex_view = tex.create_default_view();

        Self {
            buffer,
            tex_extent,
            tex,
            tex_view,
        }
    }
}