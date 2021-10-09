use crate::const_init::ConstInit;
use crate::AllocErr;
use core::ptr::NonNull;
use memory_units::{Bytes, Pages};
use spin::Mutex;

pub const SCRATCH_LEN_BYTES: usize = 3 * 1024 * 1024 - 512;

#[repr(align(4096))]
struct ScratchHeap([u8; SCRATCH_LEN_BYTES]);

static mut SCRATCH_HEAP: ScratchHeap = ScratchHeap([0; SCRATCH_LEN_BYTES]);
pub static OFFSET: Mutex<usize> = Mutex::new(0);

pub(crate) unsafe fn alloc_pages(pages: Pages) -> Result<NonNull<u8>, AllocErr> {
    let bytes: Bytes = pages.into();
    let mut offset = OFFSET.lock();
    let end = bytes.0.checked_add(*offset).ok_or(AllocErr)?;
    if end < SCRATCH_LEN_BYTES {
        let ptr = SCRATCH_HEAP.0[*offset..end].as_mut_ptr() as *mut u8;
        *offset = end;
        NonNull::new(ptr).ok_or(AllocErr)
    } else {
        Err(AllocErr)
    }
}

pub(crate) struct Exclusive<T> {
    inner: Mutex<T>,
}

impl<T: ConstInit> ConstInit for Exclusive<T> {
    const INIT: Self = Exclusive {
        inner: Mutex::new(T::INIT),
    };
}

impl<T> Exclusive<T> {
    /// Get exclusive, mutable access to the inner value.
    ///
    /// # Safety
    ///
    /// It is the callers' responsibility to ensure that `f` does not re-enter
    /// this method for this `Exclusive` instance.
    //
    // XXX: If we don't mark this function inline, then it won't be, and the
    // code size also blows up by about 200 bytes.
    #[inline]
    pub(crate) unsafe fn with_exclusive_access<'a, F, U>(&'a self, f: F) -> U
    where
        for<'x> F: FnOnce(&'x mut T) -> U,
    {
        let mut guard = self.inner.lock();
        f(&mut guard)
    }
}
