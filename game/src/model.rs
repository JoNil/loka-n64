use alloc::borrow::Cow;
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
        cfg_if::cfg_if! {
            if #[cfg(target_vendor = "nintendo64")] {

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
                    verts: Cow::Borrowed(verts),
                    uvs: Cow::Borrowed(uvs),
                    colors: Cow::Borrowed(colors),
                    indices: Cow::Borrowed(indices),
                }
            } else {

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
                let indices_in = byteswap_u32_slice(self.indices);

                let verts = LayoutVerified::<_, [Vec3]>::new_slice(verts_in.as_slice())
                    .unwrap()
                    .into_slice()
                    .to_owned();

                let uvs = LayoutVerified::<_, [Vec2]>::new_slice(uvs_in.as_slice())
                    .unwrap()
                    .into_slice()
                    .to_owned();

                let colors = LayoutVerified::<_, [u32]>::new_slice(colors_in.as_slice())
                    .unwrap()
                    .into_slice()
                    .to_owned();

                let indices = LayoutVerified::<_, [[u8; 3]]>::new_slice(indices_in.as_slice())
                    .unwrap()
                    .into_slice()
                    .to_owned();

                ModelData {
                    verts: Cow::Owned(verts),
                    uvs: Cow::Owned(uvs),
                    colors: Cow::Owned(colors),
                    indices: Cow::Owned(indices),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ModelData<'a> {
    pub verts: Cow<'a, [Vec3]>,
    pub uvs: Cow<'a, [Vec2]>,
    pub colors: Cow<'a, [u32]>,
    pub indices: Cow<'a, [[u8; 3]]>,
}
