#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct RdpCommand(pub u64);

#[repr(C, align(8))]
pub struct RdpBlock {
    pub rdp_data: [RdpCommand; 128],
}

impl Default for RdpBlock {
    fn default() -> Self {
        Self {
            rdp_data: [RdpCommand(0); 128],
        }
    }
}
