use n64_math::{Vec2, Vec3};
use zerocopy::LayoutVerified;

pub struct StaticModelData {
    pub verts: &'static [u8],
    pub uvs: &'static [u8],
    pub colors: &'static [u8],
    pub indices: &'static [u8],
}

impl StaticModelData {
    pub fn as_model_data(&self) -> ModelData {
        let verts = LayoutVerified::<_, [Vec3]>::new_slice(self.verts)
            .unwrap()
            .into_slice();

        let uvs = LayoutVerified::<_, [Vec2]>::new_slice(self.uvs)
            .unwrap()
            .into_slice();

        let colors = LayoutVerified::<_, [u32]>::new_slice(self.colors)
            .unwrap()
            .into_slice();

        let indices = LayoutVerified::<_, [[u8; 3]]>::new_slice(self.indices)
            .unwrap()
            .into_slice();

        ModelData {
            verts,
            uvs,
            colors,
            indices,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ModelData<'a> {
    pub verts: &'a [Vec3],
    pub uvs: &'a [Vec2],
    pub colors: &'a [u32],
    pub indices: &'a [[u8; 3]],
}
