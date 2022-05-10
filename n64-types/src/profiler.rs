use core::mem::size_of;
use zerocopy::{AsBytes, FromBytes, Unaligned};

use crate::static_assert;

#[repr(C, packed)]
#[derive(Copy, Clone, FromBytes, AsBytes, Unaligned)]
pub struct ScopeData {
    pub start: i32,
    pub end: i32,
    pub depth: u8,
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

#[repr(C, packed)]
#[derive(AsBytes, FromBytes, Unaligned)]
pub struct ProfilerMessageBuffer {
    pub message_header_buffer: u8,
    pub scope: ScopeData,
    pub index: i16,
    pub count: i16,
}

static_assert!(size_of::<ProfilerMessageBuffer>() == 16);

impl ProfilerMessageBuffer {
    pub fn get_scope_from_be(&self) -> ScopeData {
        ScopeData {
            start: i32::from_be(self.scope.start),
            end: i32::from_be(self.scope.end),
            depth: self.scope.depth,
            id: i16::from_be(self.scope.id),
        }
    }
}
