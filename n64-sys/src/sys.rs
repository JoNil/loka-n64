use core::mem::size_of;

#[inline]
pub unsafe fn data_cache_hit_writeback_invalidate<T>(block: &[T]) {
    let mut addr = (block.as_ptr() as usize) & 0xffff_fffc;
    let mut len = block.len() * size_of::<T>();

    while len > 0 {
        asm!("cache $0, ($1)"
        :
        : "i" (0x15), "r" (addr)
        :
        : "volatile"
        );

        len -= 4;
        addr += 4;
    }
}

#[inline]
pub unsafe fn data_cache_hit_writeback<T>(block: &[T]) {
    let mut addr = (block.as_ptr() as usize) & 0xffff_fffc;
    let mut len = block.len() * size_of::<T>();

    while len > 0 {
        asm!("cache $0, ($1)"
        :
        : "i" (0x19), "r" (addr)
        :
        : "volatile"
        );

        len -= 4;
        addr += 4;
    }
}

#[inline]
pub unsafe fn data_cache_hit_invalidate<T>(block: &[T]) {
    let mut addr = (block.as_ptr() as usize) & 0xffff_fffc;
    let mut len = block.len() * size_of::<T>();

    while len > 0 {
        asm!("cache $0, ($1)"
        :
        : "i" (0x11), "r" (addr)
        :
        : "volatile"
        );

        len -= 4;
        addr += 4;
    }
}

#[inline]
pub(crate) fn uncached_addr<T>(address: *const T) -> *const T {
    ((address as usize) | 0x2000_0000) as *const T
}

#[inline]
pub(crate) fn uncached_addr_mut<T>(address: *mut T) -> *mut T {
    ((address as usize) | 0x2000_0000) as *mut T
}

#[inline]
pub(crate) fn virtual_to_physical<T>(address: *const T) -> usize {
    (address as usize) & 0x1fff_ffff
}

#[inline]
pub(crate) fn virtual_to_physical_mut<T>(address: *mut T) -> usize {
    (address as usize) & 0x1fff_ffff
}

#[inline]
pub(crate) unsafe fn memory_barrier() {
    asm!("" ::: "memory" : "volatile");
}

#[inline]
fn get_tick_rate() -> f32 {
    93750000.0 / 2.0
}

#[inline]
fn get_ticks() -> u32 {
    let res;

    unsafe {
        asm!("mfc0 $0,$$9
            nop"
            : "=r" (res));
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
