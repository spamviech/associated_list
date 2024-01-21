//! Unit tests for an [`AssocList`].

use core::marker::PhantomData;

use alloc::collections::BTreeMap;

use crate::AssocList;

// O(n^2)
fn unique_keys<K: PartialEq, V>(assoc_list: AssocList<K, V>) -> bool {
    let AssocList { mut vec, phantom: PhantomData } = assoc_list;
    while let Some((key, _value)) = vec.pop() {
        if vec.iter().any(|(remaining_key, _remaining_value)| key == *remaining_key) {
            return false;
        }
    }
    true
}

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
