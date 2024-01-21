//! Unit tests for an [`AssocList`].

use alloc::collections::BTreeMap;

use crate::AssocList;

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
    let assoc_list: AssocList<usize, f64> = AssocList::default();
    assert!(assoc_list.vec.is_empty());
    assert!(unique_ord_keys(assoc_list));
}

#[test]
fn with_capacity() {
    const CAPACITY: usize = 23571;
    let assoc_list: AssocList<usize, f64> = AssocList::with_capacity(CAPACITY);
    assert!(assoc_list.vec.is_empty());
    assert_eq!(assoc_list.vec.capacity(), CAPACITY);
    assert!(unique_ord_keys(assoc_list));
}

#[test]
fn new_in() {
    todo!()
}

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
