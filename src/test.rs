//! Unit tests for an [`AssocList`].

#[cfg(feature = "allocator_api")]
use core::{
    alloc::{AllocError, Allocator as ActualAllocator, Layout},
    cell::{Cell, UnsafeCell},
    pin::{pin, Pin},
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};

use quickcheck_macros::quickcheck;

use crate::{assoc_list, Allocator, AssocList};

// O(n*log(n))
fn unique_ord_keys<K: Ord, V, A: Allocator>(assoc_list: AssocList<K, V, A>) -> bool {
    let size = assoc_list.len();
    let b_tree_map: BTreeMap<_, _> = assoc_list.vec.into_iter().collect();
    size == b_tree_map.len()
}

// Move the reference from the tuple to the elements
fn split_tuple_refs<A, B>((key, value): &(A, B)) -> (&A, &B) {
    (key, value)
}

#[test]
fn new() {
    const ASSOC_LIST: AssocList<usize, f64> = AssocList::new();
    assert!(ASSOC_LIST.vec.is_empty());
    assert!(unique_ord_keys(ASSOC_LIST));
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

#[test]
fn assoc_list_macro() {
    macro_rules! test_macro {
        ($([$($input: tt)*]),* $(,)?) => {$(
            let reference_map = BTreeMap::from([$($input)*]);
            let mut reference_map_vec: Vec<(i32, i32)> = reference_map.into_iter().collect();
            let assoc_list: AssocList<i32, i32> = assoc_list![$($input)*];
            let mut assoc_list_vec = assoc_list.vec.clone();
            // check keys are unique
            assert!(unique_ord_keys(assoc_list));
            // check the result is identical to a BTreeMap
            reference_map_vec.sort();
            assoc_list_vec.sort();
            assert_eq!(assoc_list_vec, reference_map_vec);
        )*};
    }

    test_macro!(
        [],                                                            // empty
        [(3, 7), (8, -1), (9, 0), (0, 4)],                             // unique keys
        [(-3, 7), (-3, -1), (9, 0), (0, 4)],                           // negative keys
        [(3, 7), (8, -1), (3, 0), (0, 4)],                             // duplicated key
        [(8, -1), (3, 0), (0, 4), (-8, 1), (8, 2), (0, 1), (-8, 267)], // duplicated & negative keys
    );
}

#[test]
fn from() {
    macro_rules! test_from {
        ($([$($input: tt)*]),* $(,)?) => {$(
            let reference_map = BTreeMap::from([$($input)*]);
            let mut reference_map_vec: Vec<(i32, i32)> = reference_map.into_iter().collect();
            let assoc_list: AssocList<i32, i32> = AssocList::from([$($input)*]);
            let mut assoc_list_vec = assoc_list.vec.clone();
            // check keys are unique
            assert!(unique_ord_keys(assoc_list));
            // check the result is identical to a BTreeMap
            reference_map_vec.sort();
            assoc_list_vec.sort();
            assert_eq!(assoc_list_vec, reference_map_vec);
        )*};
    }

    test_from!(
        [],                                                            // empty
        [(3, 7), (8, -1), (9, 0), (0, 4)],                             // unique keys
        [(-3, 7), (-3, -1), (9, 0), (0, 4)],                           // negative keys
        [(3, 7), (8, -1), (3, 0), (0, 4)],                             // duplicated key
        [(8, -1), (3, 0), (0, 4), (-8, 1), (8, 2), (0, 1), (-8, 267)], // duplicated & negative keys
    );
}

#[test]
fn from_iterator() {
    macro_rules! test_collect {
        ($([$($input: tt)*]),* $(,)?) => {$(
            let reference_map = BTreeMap::from([$($input)*]);
            let mut reference_map_vec: Vec<(i32, i32)> = reference_map.into_iter().collect();
            let assoc_list: AssocList<i32, i32> = [$($input)*].into_iter().collect();
            let mut assoc_list_vec = assoc_list.vec.clone();
            // check keys are unique
            assert!(unique_ord_keys(assoc_list));
            // check the result is identical to a BTreeMap
            reference_map_vec.sort();
            assoc_list_vec.sort();
            assert_eq!(assoc_list_vec, reference_map_vec);
        )*};
    }

    test_collect!(
        [],                                                            // empty
        [(3, 7), (8, -1), (9, 0), (0, 4)],                             // unique keys
        [(-3, 7), (-3, -1), (9, 0), (0, 4)],                           // negative keys
        [(3, 7), (8, -1), (3, 0), (0, 4)],                             // duplicated key
        [(8, -1), (3, 0), (0, 4), (-8, 1), (8, 2), (0, 1), (-8, 267)], // duplicated & negative keys
    );
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
    const CAPACITY: usize = 5;
    let mut assoc_list: AssocList<i8, i8> = AssocList::with_capacity(CAPACITY);
    for i in 0..i8::try_from(CAPACITY).expect("small test number") {
        let previous = assoc_list.insert(-i, i);
        assert!(previous.is_none());
    }
    assert!(assoc_list.keys().copied().all(i8::is_negative));
}

#[test]
fn into_keys() {
    const CAPACITY: usize = 17;
    let mut assoc_list: AssocList<i8, i8> = AssocList::with_capacity(CAPACITY);
    for i in 0..i8::try_from(CAPACITY).expect("small test number") {
        let previous = assoc_list.insert(i, -i);
        assert!(previous.is_none());
    }
    assert!(assoc_list.into_keys().all(i8::is_positive));
}

#[test]
fn values() {
    const CAPACITY: usize = 62;
    let capacity_i16 = i16::try_from(CAPACITY).expect("small test number");
    let mut assoc_list = AssocList::with_capacity(CAPACITY);
    let mut reference_map = BTreeMap::new();
    for i in (-capacity_i16..capacity_i16).rev() {
        #[allow(clippy::integer_division)]
        let key = i % (capacity_i16 / 7);
        let value = i.leading_ones();
        // there might be duplicate values
        let _ = reference_map.insert(key, value);
        let _ = assoc_list.insert(key, value);
    }
    let expected_values: BTreeSet<_> = reference_map.values().copied().collect();
    let actual_values: BTreeSet<_> = assoc_list.values().copied().collect();
    assert_eq!(actual_values, expected_values);
}

#[test]
fn values_mut() {
    const CAPACITY: usize = 62;
    let capacity_i16 = i16::try_from(CAPACITY).expect("small test number");
    let mut assoc_list = AssocList::with_capacity(CAPACITY);
    let mut reference_map = BTreeMap::new();
    for i in (-capacity_i16..capacity_i16).rev() {
        #[allow(clippy::integer_division)]
        let key = i % (capacity_i16 / 4);
        let value = i.trailing_zeros();
        // there might be duplicate values
        let _ = reference_map.insert(key, value);
        let _ = assoc_list.insert(key, value);
    }
    let expected_values: BTreeSet<_> = reference_map.values_mut().map(|key| &*key).collect();
    let actual_values: BTreeSet<_> = assoc_list.values_mut().map(|key| &*key).collect();
    assert_eq!(
        actual_values, expected_values,
        "values_mut returns (mutable) references to the values"
    );

    let new_value = 3938;
    assoc_list.values_mut().for_each(|value| *value = new_value);
    assert!(
        assoc_list.values().all(|value| *value == new_value),
        "values_mut elements can influence the values in the AssocList"
    );
}

#[test]
fn into_values() {
    const CAPACITY: usize = 62;
    let capacity_i16 = i16::try_from(CAPACITY).expect("small test number");
    let mut assoc_list = AssocList::with_capacity(CAPACITY);
    let mut reference_map = BTreeMap::new();
    for i in (-capacity_i16..capacity_i16).rev() {
        #[allow(clippy::integer_division)]
        let key = i % (capacity_i16 / 9);
        let value = (i * 17) % 205;
        // there might be duplicate values
        let _ = reference_map.insert(key, value);
        let _ = assoc_list.insert(key, value);
    }
    let expected_values: BTreeSet<_> = reference_map.into_values().collect();
    let actual_values: BTreeSet<_> = assoc_list.into_values().collect();
    assert_eq!(actual_values, expected_values);
}

#[quickcheck]
fn iter(input: Vec<(u64, i32)>) {
    let assoc_list: AssocList<_, _> = input.iter().copied().collect();
    let reference_map: BTreeMap<_, _> = input.into_iter().collect();
    let mut actual_values: Vec<_> = assoc_list.iter().map(split_tuple_refs).collect();
    let mut expected_values: Vec<_> = reference_map.iter().collect();

    actual_values.sort_by_key(|(key, _value)| *key);
    expected_values.sort_by_key(|(key, _value)| *key);
    assert_eq!(actual_values, expected_values);
}

#[quickcheck]
fn iter_mut(input: Vec<(i64, i32)>) {
    let mut assoc_list: AssocList<_, _> = input.iter().copied().collect();
    let mut reference_map: BTreeMap<_, _> = input.into_iter().collect();
    let mut actual_values: Vec<_> = assoc_list.iter_mut().collect();
    let mut expected_values: Vec<_> = reference_map.iter_mut().collect();

    actual_values.sort_by_key(|(key, _value)| *key);
    expected_values.sort_by_key(|(key, _value)| *key);
    assert_eq!(actual_values, expected_values, "iter_mut returns the correct elements");

    let new_value = 1267;
    assoc_list.values_mut().for_each(|value| *value = new_value);
    assert!(
        assoc_list.values().all(|value| *value == new_value),
        "iter_mut elements can influence the values in the AssocList"
    );
}

#[quickcheck]
fn drain(input: Vec<(i16, u8)>) {
    let mut assoc_list: AssocList<_, _> = input.iter().copied().collect();
    let reference_map: BTreeMap<_, _> = input.into_iter().collect();
    let actual_values: BTreeMap<_, _> = assoc_list.drain().collect();

    assert_eq!(actual_values, reference_map, "drain returns the correct elements");

    assert!(assoc_list.is_empty(), "drain removes all elements from the AssocList");
}

#[quickcheck]
fn into_iter(input: Vec<(u8, i32)>) {
    let assoc_list: AssocList<_, _> = input.iter().copied().collect();
    let reference_map: BTreeMap<_, _> = input.into_iter().collect();
    let actual_values: BTreeMap<_, _> = assoc_list.into_iter().collect();

    assert_eq!(actual_values, reference_map);
}

// required by quickcheck
#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn into_iter_ref(input: Vec<(i64, i16)>) {
    let assoc_list: AssocList<_, _> = input.iter().copied().collect();
    #[allow(clippy::into_iter_on_ref)]
    let reference_map: BTreeMap<_, _> = (&input).into_iter().map(split_tuple_refs).collect();
    let actual_values: BTreeMap<_, _> = (&assoc_list).into_iter().map(split_tuple_refs).collect();

    assert_eq!(actual_values, reference_map);
}

#[quickcheck]
fn into_iter_mut(input: Vec<(i64, u32)>) {
    let mut assoc_list: AssocList<_, _> = input.iter().copied().collect();
    let mut reference_map: BTreeMap<_, _> = input.into_iter().collect();
    let mut actual_values: Vec<_> = (&mut assoc_list).into_iter().collect();
    #[allow(clippy::into_iter_on_ref)]
    let mut expected_values: Vec<_> = (&mut reference_map).into_iter().collect();

    actual_values.sort_by_key(|(key, _value)| *key);
    expected_values.sort_by_key(|(key, _value)| *key);
    assert_eq!(actual_values, expected_values, "&mut into_iter returns the correct elements");

    let new_value = 2347;
    assoc_list.values_mut().for_each(|value| *value = new_value);
    assert!(
        assoc_list.values().all(|value| *value == new_value),
        "&mut into_iter elements can influence the values in the AssocList"
    );
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
fn shrink_to() {
    todo!()
}

#[test]
fn shrink_to_fit() {
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
