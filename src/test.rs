//! Unit tests for an [`AssocList`].

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use quickcheck_macros::quickcheck;

use crate::{assoc_list, Allocator, AssocList};

#[cfg(feature = "allocator_api")]
mod allocator;
mod entry;
mod iter;

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
    assert!(assoc_list.vec.capacity() >= CAPACITY);
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

#[quickcheck]
fn len(input: Vec<(f32, i8)>) {
    let input_len = input.len();
    let assoc_list: AssocList<_, _> = input.into_iter().collect();
    assert!(assoc_list.len() <= input_len);
    assert_eq!(assoc_list.len(), assoc_list.vec.len());
}

#[quickcheck]
fn capacity(input: Vec<(f64, String)>) {
    let assoc_list: AssocList<_, _> = input.into_iter().collect();
    assert!(assoc_list.len() <= assoc_list.capacity());
    assert_eq!(assoc_list.capacity(), assoc_list.vec.capacity());
}

#[test]
fn is_empty() {
    let mut assoc_list = AssocList::new();
    assert!(assoc_list.is_empty(), "new AssocList is empty");

    let _ = assoc_list.insert((), ());
    assert!(!assoc_list.is_empty(), "after inserting an element, the AssocList is not empty");

    let _ = assoc_list.remove(&());
    assert!(assoc_list.is_empty(), "after removing the only element, the AssocList is empty");
}

#[quickcheck]
fn clear(input: Vec<(f32, i8)>) {
    let mut assoc_list: AssocList<_, _> = input.into_iter().collect();
    // clear removes all elements
    assoc_list.clear();
    assert!(assoc_list.is_empty());
    // a second clear works and leaves the AssocList empty
    assoc_list.clear();
    assert!(assoc_list.is_empty());
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
fn reserve() {
    todo!()
}

#[test]
fn reserve_exact() {
    todo!()
}

#[test]
fn try_reserve() {
    todo!()
}

#[test]
fn try_reserve_exact() {
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
