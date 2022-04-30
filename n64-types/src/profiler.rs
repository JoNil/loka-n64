use zerocopy::{AsBytes, FromBytes, Unaligned};

#[repr(C, packed)]
#[derive(Copy, Clone, FromBytes, AsBytes, Unaligned)]
pub struct ScopeData {
    pub start: i32,
    pub end: i32,
    pub depth: i16,
    pub id: i16,
}

impl ScopeData {
    #[inline]
    pub const fn default() -> Self {
        ScopeData {
            start: 0,
            end: 0,
            depth: 0,
            id: 0,
        }
    }
}
