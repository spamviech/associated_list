//! Unit tests for an [`AssocList`].

use alloc::collections::BTreeMap;

#[cfg(feature = "allocator_api")]
use core::{
    alloc::{AllocError, Allocator as ActualAllocator, Layout},
    cell::{Cell, UnsafeCell},
    pin::{pin, Pin},
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{Allocator, AssocList};

// O(n*log(n))
fn unique_ord_keys<K: Ord, V, A: Allocator>(assoc_list: AssocList<K, V, A>) -> bool {
    let size = assoc_list.len();
    let b_tree_map: BTreeMap<_, _> = assoc_list.vec.into_iter().collect();
    size == b_tree_map.len()
}

#[test]
fn new() {
    let assoc_list: AssocList<usize, f64> = AssocList::new();
    assert!(assoc_list.vec.is_empty());
    assert!(unique_ord_keys(assoc_list));
}

#[test]
fn default() {
    let assoc_list: AssocList<i32, &'static str> = AssocList::default();
    assert!(assoc_list.vec.is_empty());
    assert!(unique_ord_keys(assoc_list));
}

#[test]
fn with_capacity() {
    const CAPACITY: usize = 23_571;
    let assoc_list: AssocList<u16, f32> = AssocList::with_capacity(CAPACITY);
    assert!(assoc_list.vec.is_empty());
    assert_eq!(assoc_list.vec.capacity(), CAPACITY);
    assert!(unique_ord_keys(assoc_list));
}

#[cfg(feature = "allocator_api")]
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

#[cfg(feature = "allocator_api")]
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

#[cfg(feature = "allocator_api")]
#[allow(unsafe_code)]
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

#[cfg(feature = "allocator_api")]
#[test]
fn new_in() {
    let memory = pin!([0; 0]);
    let test_allocator = TestAllocator::new(memory);
    let assoc_list: AssocList<usize, f64, _> = AssocList::new_in(test_allocator);
    assert!(assoc_list.vec.is_empty());
    assert!(unique_ord_keys(assoc_list));
}

#[cfg(feature = "allocator_api")]
#[test]
fn with_capacity_in() {
    let memory = pin!([0; 24]);
    let test_allocator = TestAllocator::new(memory);
    const CAPACITY: usize = 3;
    let assoc_list: AssocList<u16, f32, _> = AssocList::with_capacity_in(CAPACITY, test_allocator);
    assert!(assoc_list.vec.is_empty());
    assert_eq!(assoc_list.vec.capacity(), CAPACITY);
    assert!(unique_ord_keys(assoc_list));
}

#[test]
fn keys() {
    todo!()
}

#[test]
fn into_keys() {
    todo!()
}

#[test]
fn values() {
    todo!()
}

#[test]
fn values_mut() {
    todo!()
}

#[test]
fn into_values() {
    todo!()
}

#[test]
fn iter() {
    todo!()
}

#[test]
fn iter_mut() {
    todo!()
}

#[test]
fn drain() {
    todo!()
}

#[test]
fn len() {
    todo!()
}

#[test]
fn is_empty() {
    todo!()
}

#[test]
fn clear() {
    todo!()
}

#[test]
fn entry() {
    todo!()
}

#[test]
fn occupied_entry() {
    todo!()
}

#[test]
fn vacant_entry() {
    todo!()
}

#[test]
fn contains_key() {
    todo!()
}

#[test]
fn get() {
    todo!()
}

#[test]
fn get_key_value() {
    todo!()
}

#[test]
fn get_mut() {
    todo!()
}

#[test]
fn insert() {
    todo!()
}

#[test]
fn remove() {
    todo!()
}

#[test]
fn remove_entry() {
    todo!()
}

#[test]
fn partial_eq() {
    todo!()
}

#[test]
fn extend() {
    todo!()
}

#[test]
fn extend_ref() {
    todo!()
}

#[test]
fn index() {
    todo!()
}

#[test]
fn index_mut() {
    todo!()
}

#[test]
fn into_iter() {
    todo!()
}

#[test]
fn into_iter_ref() {
    todo!()
}

#[test]
fn into_iter_mut() {
    todo!()
}
