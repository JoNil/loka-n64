use naga::{
    back::spv,
    front::glsl,
    valid::{Capabilities, ValidationFlags, Validator},
    ShaderStage,
};
use wgpu::ShaderModule;

fn compile_stage(device: &wgpu::Device, src: &str, stage: ShaderStage) -> ShaderModule {
    let module = glsl::Parser::default()
        .parse(&glsl::Options::from(stage), src)
        .unwrap();

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());

    let module_info = validator.validate(&module).unwrap();

    let output = spv::write_vec(
        &module,
        &module_info,
        &spv::Options::default(),
        Some(&spv::PipelineOptions {
            shader_stage: stage,
            entry_point: "main".to_owned(),
        }),
    )
    .unwrap();

    device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::SpirV(output.into()),
    })
}

pub(crate) fn compile(
    device: &wgpu::Device,
    vs_src: &str,
    fs_src: &str,
) -> (ShaderModule, ShaderModule) {
    (
        compile_stage(device, vs_src, ShaderStage::Vertex),
        compile_stage(device, fs_src, ShaderStage::Fragment),
    )
}
