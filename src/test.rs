//! Unit tests for an [`AssocList`].

use alloc::collections::BTreeMap;

use crate::AssocList;
#[cfg(feature = "allocator_api")]
use alloc::sync::Arc;
#[cfg(feature = "allocator_api")]
use core::{
    alloc::{AllocError, Allocator, Layout},
    cell::Cell,
    pin::Pin,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

// O(n*log(n))
fn unique_ord_keys<K: Ord, V>(assoc_list: AssocList<K, V>) -> bool {
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
struct TestAllocator<const N: usize> {
    /// Control access to the other fields.
    current: AtomicBool,
    /// The available memory.
    memory: Pin<Arc<[u8; N]>>,
    /// The next free memory location.
    next: Cell<usize>,
}

#[cfg(feature = "allocator_api")]
impl<const N: usize> TestAllocator<N> {
    /// Create a new [`TestAllocator`].
    #[must_use]
    fn new() -> Self {
        TestAllocator { used: AtomicBool::new(false), memory: Arc::pin([0; N]), next: Cell::new(0) }
    }
}

#[cfg(feature = "allocator_api")]
impl<const N: usize> Allocator for TestAllocator<N> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // helper-function, assuming we have access. This allows using the `?`-operator.
        fn allocate_impl(allocator: &Self, layout: Layout) -> Option<NonNull<[u8]>> {
            // make sure the returned memory is aligned correctly
            let next_rounded_up = allocator.next.get().checked_next_multiple_of(align)?;
            // make sure the layout fits into the remaining memory
            let size = layout.size();
            let last_bit = next_rounded_up.checked_add(size)?;
            if (last_bit > N) {
                return None;
            }
            let slice = &allocator.memory[next_rounded_up..=last_bit];
            let non_null = NonNull::new(slice.as_mut_ptr())?;
            self.next.set(last_bit.saturating_add(1));
            Some(non_null)
        }
        // spin-loop, wait for access
        while (!self.used.swap(true, Ordering::AcqRel)) {}
        // allocate the memory
        let ret = allocate_impl(self, layout);
        // free usage again
        self.used.store(false, Ordering::Release);
        // return result
        ret.ok_or(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // simplistic, we don't free any memory
    }
}

#[cfg(feature = "allocator_api")]
#[test]
fn new_in() {
    todo!()
}

#[cfg(feature = "allocator_api")]
#[test]
fn with_capacity_in() {
    todo!()
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
