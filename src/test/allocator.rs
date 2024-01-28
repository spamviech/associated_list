//! Unit tests for the `allocator_api`-feature methods of an [`AssocList`].

use core::{
    alloc::{AllocError, Allocator as ActualAllocator, Layout},
    cell::{Cell, UnsafeCell},
    pin::{pin, Pin},
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{test::unique_ord_keys, AssocList};

/// Simplistic [`Allocator`] for using in the test.
///
/// Once acquired, memory will never be freed again.
struct TestAllocator<'t> {
    /// Control access to the other fields.
    used: AtomicBool,
    /// The available memory.
    memory: UnsafeCell<Pin<&'t mut [u8]>>,
    /// The next free memory location.
    next: Cell<usize>,
}

impl<'t> TestAllocator<'t> {
    /// Create a new [`TestAllocator`] using the provided `memory`.
    ///
    /// It can be created using the [`pin!`](core::pin::pin)-macro.
    #[must_use]
    fn new(memory: Pin<&'t mut [u8]>) -> Self {
        TestAllocator {
            used: AtomicBool::new(false),
            memory: UnsafeCell::new(memory),
            next: Cell::new(0),
        }
    }
}

#[allow(unsafe_code)]
// SAFETY: the whole point of the feature ;)
unsafe impl ActualAllocator for TestAllocator<'_> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // helper-function, assuming we have access. This allows using the `?`-operator.
        fn allocate_impl(allocator: &TestAllocator<'_>, layout: Layout) -> Option<NonNull<[u8]>> {
            // make sure the returned memory is aligned correctly
            let next_rounded_up = allocator.next.get().checked_next_multiple_of(layout.align())?;
            // make sure the layout fits into the remaining memory
            let size = layout.size();
            let last_bit = next_rounded_up.checked_add(size)?;

            #[allow(unsafe_code)]
            // SAFETY: protected by spin-looping on `self.used` before calling this function
            if last_bit > unsafe { &*allocator.memory.get() }.len() {
                return None;
            }
            #[allow(unsafe_code)]
            // SAFETY: protected by spin-looping on `self.used` before calling this function
            let slice: &mut [u8] = unsafe { &mut *allocator.memory.get() };
            let non_null = NonNull::new(slice)?;
            allocator.next.set(last_bit.saturating_add(1));
            Some(non_null)
        }
        // spin-loop, wait for access
        while !self.used.swap(true, Ordering::AcqRel) {}
        // allocate the memory
        let ret = allocate_impl(self, layout);
        // free usage again
        self.used.store(false, Ordering::Release);
        // return result
        ret.ok_or(AllocError)
    }

    #[allow(unsafe_code)]
    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // simplistic, we don't free any memory
    }
}

#[test]
fn new_in() {
    let memory = pin!([0; 0]);
    let test_allocator = TestAllocator::new(memory);
    let assoc_list: AssocList<usize, f64, _> = AssocList::new_in(test_allocator);
    assert!(assoc_list.vec.is_empty());
    assert!(unique_ord_keys(assoc_list));
}

#[test]
fn with_capacity_in() {
    const CAPACITY: usize = 3;
    let memory = pin!([0; 1024]);
    let test_allocator = TestAllocator::new(memory);
    let assoc_list: AssocList<u16, f32, _> = AssocList::with_capacity_in(CAPACITY, test_allocator);
    assert!(assoc_list.vec.is_empty());
    assert_eq!(assoc_list.vec.capacity(), CAPACITY);
    assert!(unique_ord_keys(assoc_list));
}
