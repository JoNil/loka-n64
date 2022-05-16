#![no_std]
#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::cast_ptr_alignment)]
#![allow(clippy::needless_lifetimes)]

extern crate alloc;

mod const_init;
mod imp_static_array;
mod neighbors;
mod size_classes;

use const_init::ConstInit;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::Cell;
use core::cmp;
use core::marker::Sync;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicI32, Ordering};
use imp_static_array as imp;
use memory_units::{size_of, ByteSize, Bytes, Pages, RoundUpTo, Words};
use neighbors::Neighbors;

pub(crate) struct AllocErr;

#[inline]
fn checked_round_up_to<T>(b: Bytes) -> Option<T>
where
    T: ByteSize,
    Bytes: RoundUpTo<T>,
{
    if b.0.checked_add(T::BYTE_SIZE.0).is_none() {
        None
    } else {
        Some(b.round_up_to())
    }
}

#[repr(C)]
#[derive(Default, Debug)]
struct CellHeader<'a> {
    neighbors: Neighbors<'a, CellHeader<'a>>,
}

impl<'a> AsRef<Neighbors<'a, CellHeader<'a>>> for CellHeader<'a> {
    fn as_ref(&self) -> &Neighbors<'a, CellHeader<'a>> {
        &self.neighbors
    }
}

unsafe impl<'a> neighbors::HasNeighbors<'a, CellHeader<'a>> for CellHeader<'a> {
    #[inline]
    unsafe fn next_checked(
        neighbors: &Neighbors<'a, CellHeader<'a>>,
        next: *const CellHeader<'a>,
    ) -> Option<&'a CellHeader<'a>> {
        if next.is_null() || CellHeader::next_cell_is_invalid(neighbors) {
            None
        } else {
            Some(&*next)
        }
    }

    #[inline]
    unsafe fn prev_checked(
        _neighbors: &Neighbors<'a, CellHeader<'a>>,
        prev: *const CellHeader<'a>,
    ) -> Option<&'a CellHeader<'a>> {
        if prev.is_null() {
            None
        } else {
            Some(&*prev)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
struct AllocatedCell<'a> {
    header: CellHeader<'a>,
}

#[test]
fn allocated_cell_layout() {
    assert_eq!(
        size_of::<CellHeader>(),
        size_of::<AllocatedCell>(),
        "Safety and correctness depends on AllocatedCell being the same as CellHeader"
    );

    assert_eq!(
        core::mem::align_of::<CellHeader>(),
        core::mem::align_of::<AllocatedCell>()
    );
}

#[repr(C)]
#[derive(Debug)]
struct FreeCell<'a> {
    header: CellHeader<'a>,
    next_free_raw: Cell<*const FreeCell<'a>>,
}

#[test]
fn free_cell_layout() {
    assert_eq!(
        size_of::<CellHeader>() + Words(1),
        size_of::<FreeCell>(),
        "Safety and correctness depends on FreeCell being only one word larger than CellHeader"
    );

    assert_eq!(
        core::mem::align_of::<CellHeader>(),
        core::mem::align_of::<AllocatedCell>()
    );
}

impl<'a> CellHeader<'a> {
    // ### Semantics of Low Bits in Neighbors Pointers
    //
    // If `self.neighbors.next_bit_1` is set, then the cell is allocated, and
    // should never be in the free list. If the bit is not set, then this cell
    // is free, and must be in the free list (or is in the process of being
    // added to the free list).
    //
    // The `self.neighbors.next` pointer always points to the byte just *after*
    // this cell. If the `self.neighbors.next_bit_2` bit is not set, then it
    // points to the next cell. If that bit is set, then it points to the
    // invalid memory that follows this cell.

    fn is_allocated(&self) -> bool {
        self.neighbors.get_next_bit_1()
    }

    fn is_free(&self) -> bool {
        !self.is_allocated()
    }

    fn set_allocated(neighbors: &Neighbors<'a, Self>) {
        neighbors.set_next_bit_1();
    }

    fn set_free(neighbors: &Neighbors<'a, Self>) {
        neighbors.clear_next_bit_1();
    }

    fn next_cell_is_invalid(neighbors: &Neighbors<'a, Self>) -> bool {
        neighbors.get_next_bit_2()
    }

    fn set_next_cell_is_invalid(neighbors: &Neighbors<'a, Self>) {
        neighbors.set_next_bit_2();
    }

    fn clear_next_cell_is_invalid(neighbors: &Neighbors<'a, Self>) {
        neighbors.clear_next_bit_2();
    }

    fn size(&self) -> Bytes {
        let data = unsafe { (self as *const CellHeader<'a>).offset(1) };
        let data = data as usize;

        let next = self.neighbors.next_unchecked();
        let next = next as usize;

        Bytes(next - data)
    }

    fn as_free_cell(&self) -> Option<&FreeCell<'a>> {
        if self.is_free() {
            Some(unsafe { &*(self as *const CellHeader as *const FreeCell) })
        } else {
            None
        }
    }

    // Get a pointer to this cell's data without regard to whether this cell is
    // allocated or free.
    unsafe fn unchecked_data(&self) -> *const u8 {
        (self as *const CellHeader).offset(1) as *const u8
    }

    // Is this cell aligned to the given power-of-2 alignment?
    fn is_aligned_to<B: Into<Bytes>>(&self, align: B) -> bool {
        let align = align.into();

        let data = unsafe { self.unchecked_data() } as usize;
        data & (align.0 - 1) == 0
    }
}

impl<'a> FreeCell<'a> {
    // Low bits in `FreeCell::next_free_raw`.
    //
    // If `NEXT_FREE_CELL_CAN_MERGE` is set, then the following invariants hold
    // true:
    //
    // * `FreeCell::next_free_raw` (and'd with the mask) is not null.
    // * `FreeCell::next_free_raw` is the adjacent `CellHeader::prev_cell_raw`.
    //
    // Therefore, this free cell can be merged into a single, larger, contiguous
    // free cell with its previous neighbor, which is also the next cell in the
    // free list.
    const NEXT_FREE_CELL_CAN_MERGE: usize = 0b01;
    const _RESERVED: usize = 0b10;
    const MASK: usize = !0b11;

    fn next_free_can_merge(&self) -> bool {
        self.next_free_raw.get() as usize & Self::NEXT_FREE_CELL_CAN_MERGE != 0
    }

    fn set_next_free_can_merge(&self) {
        let next_free = self.next_free_raw.get() as usize;
        let next_free = next_free | Self::NEXT_FREE_CELL_CAN_MERGE;
        self.next_free_raw.set(next_free as *const FreeCell);
    }

    fn clear_next_free_can_merge(&self) {
        let next_free = self.next_free_raw.get() as usize;
        let next_free = next_free & !Self::NEXT_FREE_CELL_CAN_MERGE;
        self.next_free_raw.set(next_free as *const FreeCell);
    }

    fn next_free(&self) -> *const FreeCell<'a> {
        let next_free = self.next_free_raw.get() as usize & Self::MASK;
        next_free as *const FreeCell<'a>
    }

    unsafe fn from_uninitialized(
        raw: NonNull<u8>,
        _size: Bytes,
        next_free: Option<*const FreeCell<'a>>,
        _policy: &dyn AllocPolicy<'a>,
    ) -> *const FreeCell<'a> {
        let next_free = next_free.unwrap_or(ptr::null_mut());

        let raw = raw.as_ptr() as *mut FreeCell;
        ptr::write(
            raw,
            FreeCell {
                header: CellHeader::default(),
                next_free_raw: Cell::new(next_free),
            },
        );

        raw
    }

    fn as_allocated_cell(&self, _policy: &dyn AllocPolicy<'a>) -> &AllocatedCell<'a> {
        CellHeader::set_allocated(&self.header.neighbors);
        unsafe { &*(self as *const FreeCell as *const AllocatedCell) }
    }

    // Try and satisfy the given allocation request with this cell.
    fn try_alloc<'b>(
        &'b self,
        previous: &'b Cell<*const FreeCell<'a>>,
        alloc_size: Words,
        align: Bytes,
        policy: &dyn AllocPolicy<'a>,
    ) -> Option<&'b AllocatedCell<'a>> {
        // First, do a quick check that this cell can hold an allocation of the
        // requested size.
        let size: Bytes = alloc_size.into();
        if self.header.size() < size {
            return None;
        }

        // Next, try and allocate by splitting this cell in two, and returning
        // the second half.
        //
        // We allocate from the end of this cell, rather than the beginning,
        // because it allows us to satisfy alignment requests. Since we can
        // choose to split at some alignment and return the aligned cell at the
        // end.
        let next = self.header.neighbors.next_unchecked() as usize;
        let split_and_aligned = (next - size.0) & !(align.0 - 1);
        let data = unsafe { self.header.unchecked_data() } as usize;
        let min_cell_size: Bytes = policy.min_cell_size(alloc_size).into();
        if data + size_of::<CellHeader>().0 + min_cell_size.0 <= split_and_aligned {
            let split_cell_head = split_and_aligned - size_of::<CellHeader>().0;
            let split_cell = unsafe {
                &*FreeCell::from_uninitialized(
                    unchecked_unwrap(NonNull::new(split_cell_head as *mut u8)),
                    Bytes(next - split_cell_head) - size_of::<CellHeader>(),
                    None,
                    policy,
                )
            };

            Neighbors::append(&self.header, &split_cell.header);
            self.clear_next_free_can_merge();
            if CellHeader::next_cell_is_invalid(&self.header.neighbors) {
                CellHeader::clear_next_cell_is_invalid(&self.header.neighbors);
                CellHeader::set_next_cell_is_invalid(&split_cell.header.neighbors);
            }

            return Some(split_cell.as_allocated_cell(policy));
        }

        // There isn't enough room to split this cell and still satisfy the
        // requested allocation. Because of the early check, we know this cell
        // is large enough to fit the requested size, but is the cell's data
        // properly aligned?
        if self.header.is_aligned_to(align) {
            previous.set(self.next_free());
            let allocated = self.as_allocated_cell(policy);
            return Some(allocated);
        }

        None
    }

    fn insert_into_free_list<'b>(
        &'b self,
        head: &'b Cell<*const FreeCell<'a>>,
        _policy: &dyn AllocPolicy<'a>,
    ) -> &'b Cell<*const FreeCell<'a>> {
        self.next_free_raw.set(head.get());
        head.set(self);
        head
    }
}

impl<'a> AllocatedCell<'a> {
    unsafe fn as_free_cell(&self, _policy: &dyn AllocPolicy<'a>) -> &FreeCell<'a> {
        CellHeader::set_free(&self.header.neighbors);
        let free: &FreeCell = &*(self as *const AllocatedCell as *const FreeCell);
        free.next_free_raw.set(ptr::null_mut());
        free
    }

    fn data(&self) -> *const u8 {
        let cell = &self.header as *const CellHeader;
        unsafe { cell.offset(1) as *const u8 }
    }
}

trait AllocPolicy<'a> {
    unsafe fn new_cell_for_free_list(
        &self,
        size: Words,
        align: Bytes,
    ) -> Result<*const FreeCell<'a>, AllocErr>;

    fn min_cell_size(&self, alloc_size: Words) -> Words;

    fn should_merge_adjacent_free_cells(&self) -> bool;
}

struct LargeAllocPolicy;
static LARGE_ALLOC_POLICY: LargeAllocPolicy = LargeAllocPolicy;

impl LargeAllocPolicy {
    const MIN_CELL_SIZE: Words = Words(size_classes::SizeClasses::NUM_SIZE_CLASSES * 2);
}

impl<'a> AllocPolicy<'a> for LargeAllocPolicy {
    unsafe fn new_cell_for_free_list(
        &self,
        size: Words,
        align: Bytes,
    ) -> Result<*const FreeCell<'a>, AllocErr> {
        // To assure that an allocation will always succeed after refilling the
        // free list with this new cell, make sure that we allocate enough to
        // fulfill the requested alignment, and still have the minimum cell size
        // left over.
        let size: Bytes = cmp::max(size.into(), (align + Self::MIN_CELL_SIZE) * Words(2));

        let pages: Pages = (size + size_of::<CellHeader>()).round_up_to();
        let new_pages = imp::alloc_pages(pages)?;
        let allocated_size: Bytes = pages.into();

        let free_cell = &*FreeCell::from_uninitialized(
            new_pages,
            allocated_size - size_of::<CellHeader>(),
            None,
            self as &dyn AllocPolicy<'a>,
        );

        let next_cell = (new_pages.as_ptr() as *const u8).add(allocated_size.0);
        free_cell
            .header
            .neighbors
            .set_next(next_cell as *const CellHeader);
        CellHeader::set_next_cell_is_invalid(&free_cell.header.neighbors);
        Ok(free_cell)
    }

    fn min_cell_size(&self, _alloc_size: Words) -> Words {
        Self::MIN_CELL_SIZE
    }

    fn should_merge_adjacent_free_cells(&self) -> bool {
        true
    }
}

#[inline]
unsafe fn unchecked_unwrap<T>(o: Option<T>) -> T {
    match o {
        Some(t) => t,
        None => core::hint::unreachable_unchecked(),
    }
}

unsafe fn walk_free_list<'a, F, T>(
    head: &Cell<*const FreeCell<'a>>,
    _policy: &dyn AllocPolicy<'a>,
    mut f: F,
) -> Result<T, AllocErr>
where
    F: FnMut(&Cell<*const FreeCell<'a>>, &FreeCell<'a>) -> Option<T>,
{
    // The previous cell in the free list (not to be confused with the current
    // cell's previously _adjacent_ cell).
    let previous_free = head;

    loop {
        let current_free = previous_free.get();

        if current_free.is_null() {
            return Err(AllocErr);
        }

        let current_free = Cell::new(current_free);

        // Now check if this cell can merge with the next cell in the free
        // list.
        //
        // We don't re-check `policy.should_merge_adjacent_free_cells()` because
        // the `NEXT_FREE_CELL_CAN_MERGE` bit only gets set after checking with
        // the policy.
        while (*current_free.get()).next_free_can_merge() {
            let current = &*current_free.get();
            current.clear_next_free_can_merge();

            let prev_neighbor = unchecked_unwrap(
                current
                    .header
                    .neighbors
                    .prev()
                    .and_then(|p| p.as_free_cell()),
            );

            current.header.neighbors.remove();
            if CellHeader::next_cell_is_invalid(&current.header.neighbors) {
                CellHeader::set_next_cell_is_invalid(&prev_neighbor.header.neighbors);
            }

            previous_free.set(prev_neighbor);
            current_free.set(prev_neighbor);
        }

        if let Some(result) = f(previous_free, &*current_free.get()) {
            return Ok(result);
        }

        previous_free.set(&*(*current_free.get()).next_free_raw.get());
    }
}

/// Do a first-fit allocation from the given free list.
unsafe fn alloc_first_fit<'a>(
    size: Words,
    align: Bytes,
    head: &Cell<*const FreeCell<'a>>,
    policy: &dyn AllocPolicy<'a>,
) -> Result<NonNull<u8>, AllocErr> {
    walk_free_list(head, policy, |previous, current| {
        if let Some(allocated) = current.try_alloc(previous, size, align, policy) {
            return Some(unchecked_unwrap(NonNull::new(allocated.data() as *mut u8)));
        }

        None
    })
}

unsafe fn alloc_with_refill<'a, 'b>(
    size: Words,
    align: Bytes,
    head: &'b Cell<*const FreeCell<'a>>,
    policy: &dyn AllocPolicy<'a>,
) -> Result<NonNull<u8>, AllocErr> {
    if let Ok(result) = alloc_first_fit(size, align, head, policy) {
        return Ok(result);
    }

    let cell = policy.new_cell_for_free_list(size, align)?;
    let head = (*cell).insert_into_free_list(head, policy);
    alloc_first_fit(size, align, head, policy)
}

/// A n64 allocator.
///
/// # Safety
///
/// When used in unix environments, cannot move in memory. Typically not an
/// issue if you're just using this as a `static` global allocator.
pub struct N64Alloc<'a> {
    head: imp::Exclusive<*const FreeCell<'a>>,
    size_classes: size_classes::SizeClasses<'a>,
}

unsafe impl<'a> Sync for N64Alloc<'a> {}

impl<'a> ConstInit for N64Alloc<'a> {
    const INIT: N64Alloc<'a> = N64Alloc {
        head: imp::Exclusive::INIT,
        size_classes: size_classes::SizeClasses::INIT,
    };
}

impl<'a> N64Alloc<'a> {
    /// An initial `const` default construction of a `N64Alloc` allocator.
    ///
    /// This is usable for initializing `static`s that get set as the global
    /// allocator.
    pub const INIT: Self = <Self as ConstInit>::INIT;

    unsafe fn with_free_list_and_policy_for_size<F, T>(&self, size: Words, align: Bytes, f: F) -> T
    where
        F: for<'b> FnOnce(&'b Cell<*const FreeCell<'a>>, &'b dyn AllocPolicy<'a>) -> T,
    {
        if align <= size_of::<usize>() {
            if let Some(head) = self.size_classes.get(size) {
                let policy = size_classes::SizeClassAllocPolicy(&self.head);
                let policy = &policy as &dyn AllocPolicy<'a>;
                return head.with_exclusive_access(|head| {
                    let head_cell = Cell::new(*head);
                    let result = f(&head_cell, policy);
                    *head = head_cell.get();
                    result
                });
            }
        }

        let policy = &LARGE_ALLOC_POLICY as &dyn AllocPolicy<'a>;
        self.head.with_exclusive_access(|head| {
            let head_cell = Cell::new(*head);
            let result = f(&head_cell, policy);
            *head = head_cell.get();
            result
        })
    }

    unsafe fn alloc_impl(&self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let size = Bytes(layout.size());
        let align = if layout.align() == 0 {
            Bytes(1)
        } else {
            Bytes(layout.align())
        };

        if size.0 == 0 {
            // Ensure that our made up pointer is properly aligned by using the
            // alignment as the pointer.
            return Ok(NonNull::new_unchecked(align.0 as *mut u8));
        }

        let word_size: Words = checked_round_up_to(size).ok_or(AllocErr)?;

        self.with_free_list_and_policy_for_size(word_size, align, |head, policy| {
            alloc_with_refill(word_size, align, head, policy)
        })
    }

    unsafe fn dealloc_impl(&self, ptr: NonNull<u8>, layout: Layout) {
        let size = Bytes(layout.size());
        if size.0 == 0 {
            return;
        }

        let size: Words = size.round_up_to();
        let align = Bytes(layout.align());

        self.with_free_list_and_policy_for_size(size, align, |head, policy| {
            let cell = (ptr.as_ptr() as *mut CellHeader<'a> as *const CellHeader<'a>).offset(-1);
            let cell = &*cell;
            let cell: &AllocatedCell<'a> = &*(cell as *const CellHeader as *const AllocatedCell);

            let free = cell.as_free_cell(policy);

            if policy.should_merge_adjacent_free_cells() {
                // Merging with the _previous_ adjacent cell is easy: it is
                // already in the free list, so folding this cell into it is all
                // that needs to be done. The free list can be left alone.
                //
                // Merging with the _next_ adjacent cell is a little harder. It
                // is already in the free list, but we need to splice it out
                // from the free list, since its header will become invalid
                // after consolidation, and it is *this* cell's header that
                // needs to be in the free list. But we don't have access to the
                // pointer pointing to the soon-to-be-invalid header, and
                // therefore can't adjust that pointer. So we have a delayed
                // consolidation scheme. We insert this cell just after the next
                // adjacent cell in the free list, and set the next adjacent
                // cell's `NEXT_FREE_CAN_MERGE` bit. The next time that we walk
                // the free list for allocation, the bit will be checked and the
                // consolidation will happen at that time.
                //
                // If _both_ the previous and next adjacent cells are free, we
                // are faced with a dilemma. We cannot merge all previous,
                // current, and next cells together because our singly-linked
                // free list doesn't allow for that kind of arbitrary appending
                // and splicing. There are a few different kinds of tricks we
                // could pull here, but they would increase implementation
                // complexity and code size. Instead, we use a heuristic to
                // choose whether to merge with the previous or next adjacent
                // cell. We could choose to merge with whichever neighbor cell
                // is smaller or larger, but we don't. We prefer the previous
                // adjacent cell because we can greedily consolidate with it
                // immediately, whereas the consolidating with the next adjacent
                // cell must be delayed, as explained above.

                if let Some(prev) = free
                    .header
                    .neighbors
                    .prev()
                    .and_then(|p| (*p).as_free_cell())
                {
                    free.header.neighbors.remove();
                    if CellHeader::next_cell_is_invalid(&free.header.neighbors) {
                        CellHeader::set_next_cell_is_invalid(&prev.header.neighbors);
                    }

                    return;
                }

                if let Some(next) = free
                    .header
                    .neighbors
                    .next()
                    .and_then(|n| (*n).as_free_cell())
                {
                    free.next_free_raw.set(next.next_free());
                    next.next_free_raw.set(free);
                    next.set_next_free_can_merge();

                    return;
                }
            }

            // Either we don't want to merge cells for the current policy, or we
            // didn't have the opportunity to do any merging with our adjacent
            // neighbors. In either case, push this cell onto the front of the
            // free list.
            let _head = free.insert_into_free_list(head, policy);
        });
    }
}

pub static ALLOC_BYTES_LEFT: AtomicI32 = AtomicI32::new(imp::SCRATCH_LEN_BYTES as i32);
pub static ALLOC_BYTES_USED: AtomicI32 = AtomicI32::new(0);
pub use imp::OFFSET as ALLOC_PAGE_OFFSET;

unsafe impl GlobalAlloc for N64Alloc<'static> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_BYTES_LEFT.fetch_sub(layout.size() as i32, Ordering::SeqCst);
        ALLOC_BYTES_USED.fetch_add(layout.size() as i32, Ordering::SeqCst);

        match self.alloc_impl(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(AllocErr) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOC_BYTES_LEFT.fetch_add(layout.size() as i32, Ordering::SeqCst);
        ALLOC_BYTES_USED.fetch_sub(layout.size() as i32, Ordering::SeqCst);

        if let Some(ptr) = NonNull::new(ptr) {
            self.dealloc_impl(ptr, layout);
        }
    }
}
