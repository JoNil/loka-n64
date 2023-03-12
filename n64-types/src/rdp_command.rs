#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct RdpCommand(pub u64);

#[repr(C, align(8))]
pub struct RdpBlock {
    pub block_len: u64,
    pub rdp_data: [RdpCommand; 127],
}

impl Default for RdpBlock {
    fn default() -> Self {
        Self {
            block_len: 0,
            rdp_data: [RdpCommand(0); 127],
        }
    }
}
