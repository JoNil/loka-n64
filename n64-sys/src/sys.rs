use core::ffi::c_void;

pub(crate) unsafe fn data_cache_hit_writeback_invalidate(block: &mut [u64]) {
    let mut addr = ((block.as_mut_ptr() as usize) & (!3)) as *mut c_void;
    let mut len = block.len() * 8;

    while len > 0 {
        asm!("cache $0, ($1)"
            :
            : "i" (0x15), "r" (addr)
            );

        len -= 4;
        addr = addr.offset(4);
    }
}

#[inline]
pub(crate) unsafe fn uncached_addr_mut<T>(address: *mut T) -> *mut T {
    ((address as usize) | 0x20000000) as *mut T
}

#[inline]
pub(crate) unsafe fn enable_interrupts() {
    asm!("mfc0 $$8,$$12
        ori $$8,1
        mtc0 $$8,$$12
        nop":::"$$8");
}

#[inline]
pub(crate) unsafe fn disable_interrupts() {
    asm!("mfc0 $$8,$$12
        la $$9,~1
        and $$8,$$9
        mtc0 $$8,$$12
        nop":::"$$8","$$9");
}

#[inline]
pub(crate) unsafe fn memory_barrier() {
    asm!("" ::: "memory" : "volatile");
}