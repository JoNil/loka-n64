const SI_BASE: usize = 0xA480_0000;

const SI_ADDR: *mut usize = (SI_BASE + 0x00) as *mut usize;
const SI_START_WRITE: *mut usize = (SI_BASE + 0x04) as *mut usize;
const SI_START_READ: *mut usize = (SI_BASE + 0x10) as *mut usize;
const SI_STATUS: *mut usize = (SI_BASE + 0x18) as *mut usize;
