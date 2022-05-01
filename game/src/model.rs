use alloc::borrow::Cow;
use n64_math::Vec2;
use zerocopy::LayoutVerified;

#[derive(Clone)]
pub struct ModelData<'a> {
    pub verts: Cow<'a, [[f32; 3]]>,
    pub uvs: Cow<'a, [[f32; 2]]>,
    pub colors: Cow<'a, [u32]>,
    pub indices: Cow<'a, [[u8; 3]]>,
    pub size: Vec2,
}

pub struct StaticModelData {
    pub verts: &'static [u8],
    pub uvs: &'static [u8],
    pub colors: &'static [u8],
    pub indices: &'static [u8],
    pub size: Vec2,
}

impl StaticModelData {
    pub fn as_model_data(&self) -> ModelData {
        #[cfg(target_vendor = "nintendo64")]
        {
            let verts = LayoutVerified::<_, [[f32; 3]]>::new_slice(self.verts)
                .unwrap()
                .into_slice();

            let uvs = LayoutVerified::<_, [[f32; 2]]>::new_slice(self.uvs)
                .unwrap()
                .into_slice();

            let colors = LayoutVerified::<_, [u32]>::new_slice(self.colors)
                .unwrap()
                .into_slice();

            let indices = LayoutVerified::<_, [[u8; 3]]>::new_slice(self.indices)
                .unwrap()
                .into_slice();

            ModelData {
                verts: Cow::Borrowed(verts),
                uvs: Cow::Borrowed(uvs),
                colors: Cow::Borrowed(colors),
                indices: Cow::Borrowed(indices),
                size: self.size,
            }
        }

        #[cfg(not(target_vendor = "nintendo64"))]
        {
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

            let verts_in = byteswap_u32_slice(self.verts);
            let uvs_in = byteswap_u32_slice(self.uvs);
            let colors_in = byteswap_u32_slice(self.colors);
            let indices_in = self.indices;

            let verts = LayoutVerified::<_, [[f32; 3]]>::new_slice(verts_in.as_slice())
                .unwrap()
                .into_slice()
                .to_owned();

            let uvs = LayoutVerified::<_, [[f32; 2]]>::new_slice(uvs_in.as_slice())
                .unwrap()
                .into_slice()
                .to_owned();

            let colors = LayoutVerified::<_, [u32]>::new_slice(colors_in.as_slice())
                .unwrap()
                .into_slice()
                .to_owned();

            let indices = LayoutVerified::<_, [[u8; 3]]>::new_slice(indices_in)
                .unwrap()
                .into_slice();

            ModelData {
                verts: Cow::Owned(verts),
                uvs: Cow::Owned(uvs),
                colors: Cow::Owned(colors),
                indices: Cow::Borrowed(indices),
                size: self.size,
            }
        }
    }
}
