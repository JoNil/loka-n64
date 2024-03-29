pub(crate) static TEXUTRE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
pub(crate) static DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub(crate) struct DstTexture {
    pub buffer: wgpu::Buffer,
    pub tex_extent: wgpu::Extent3d,
    pub tex: wgpu::Texture,
    pub tex_view: wgpu::TextureView,
    pub _depth: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
}

impl DstTexture {
    pub(crate) fn new(device: &wgpu::Device, width: i32, height: i32) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (4 * width * height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let tex_extent = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TEXUTRE_FORMAT,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TEXUTRE_FORMAT],
        });
        let tex_view = tex.create_view(&Default::default());

        let depth = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: tex_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[DEPTH_FORMAT],
        });
        let depth_view = depth.create_view(&Default::default());

        Self {
            buffer,
            tex_extent,
            tex,
            tex_view,
            _depth: depth,
            depth_view,
        }
    }
}
