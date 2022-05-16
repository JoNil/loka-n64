use core::{arch::asm, mem::size_of};

#[inline]
pub unsafe fn data_cache_hit_writeback_invalidate<T>(block: &[T]) {
    let addr = (block.as_ptr() as usize) & 0xffff_fff0;
    let len = block.len() * size_of::<T>() + (block.as_ptr() as usize - addr as usize);
    let mut i = 0;

    while i < len {
        let cur = addr + i;

        asm!("cache 0x15, ({})", in(reg) cur);

        i += 16;
    }
}

#[inline]
pub unsafe fn data_cache_hit_writeback_invalidate_single(addr: usize) {
    let addr = addr & 0xffff_fff0;

    asm!("cache 0x15, ({})", in(reg) addr);
}

#[inline]
pub unsafe fn data_cache_hit_writeback<T>(block: &[T]) {
    let addr = (block.as_ptr() as usize) & 0xffff_fff0;
    let len = block.len() * size_of::<T>() + (block.as_ptr() as usize - addr as usize);
    let mut i = 0;

    while i < len {
        let cur = addr + i;

        asm!("cache 0x19, ({})", in(reg) cur);

        i += 16;
    }
}

#[inline]
pub unsafe fn data_cache_hit_writeback_single(addr: usize) {
    let addr = addr & 0xffff_fff0;

    asm!("cache 0x19, ({})", in(reg) addr);
}

#[inline]
pub unsafe fn data_cache_hit_invalidate<T>(block: &[T]) {
    let addr = (block.as_ptr() as usize) & 0xffff_fff0;
    let len = block.len() * size_of::<T>() + (block.as_ptr() as usize - addr as usize);
    let mut i = 0;

    while i < len {
        let cur = addr + i;

        asm!("cache 0x11, ({})", in(reg) cur);

        i += 16;
    }
}

#[inline]
pub unsafe fn data_cache_hit_invalidate_single(addr: usize) {
    let addr = addr & 0xffff_fff0;

    asm!("cache 0x11, ({})", in(reg) addr);
}

#[inline]
pub fn uncached_addr<T>(address: *const T) -> *const T {
    ((address as usize) | 0x2000_0000) as *const T
}

#[inline]
pub fn uncached_addr_mut<T>(address: *mut T) -> *mut T {
    ((address as usize) | 0x2000_0000) as *mut T
}

#[inline]
pub fn virtual_to_physical<T>(address: *const T) -> usize {
    (address as usize) & 0x1fff_ffff
}

#[inline]
pub fn virtual_to_physical_mut<T>(address: *mut T) -> usize {
    (address as usize) & 0x1fff_ffff
}

#[inline]
fn get_tick_rate() -> f32 {
    93750000.0 / 2.0
}

#[inline]
fn get_ticks() -> u32 {
    let res;

    unsafe {
        asm!(
            "mfc0 {}, $9
            nop",
            lateout(reg) res,
        );
    }

    res
}

static mut LAST_TICKS: u32 = 0;
static mut TIME: i64 = 0;

#[inline]
pub fn current_time_us() -> i64 {
    unsafe {
        let now_ticks = get_ticks();

        if now_ticks < LAST_TICKS {
            TIME += 0x1_0000_0000 - LAST_TICKS as i64;
            TIME += now_ticks as i64;
        } else {
            TIME += (now_ticks - LAST_TICKS) as i64;
        }

        LAST_TICKS = now_ticks;

        (TIME as f32 / (get_tick_rate() / 1_000_000.0)) as i64
    }
}
