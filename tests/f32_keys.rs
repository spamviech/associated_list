//! Testing [`f32`] as an example for keys not implementing [`Ord`] and [`Hash`].

// integration tests
#![allow(unused_crate_dependencies)]
#![allow(clippy::tests_outside_test_module)]

use associated_list::AssocList;

#[test]
fn insert() {
    const DUPLICATED_KEY: f32 = 5.3;
    const SOME_VALUE: &str = "Some value";
    const SOME_OTHER_VALUE: &str = "Some other value";
    const ANOTHER_KEY: f32 = -6.8;
    const ANOTHER_VALUE: &str = "Another value";

    let mut assoc_list = AssocList::new();

    let initial = assoc_list.insert(DUPLICATED_KEY, SOME_VALUE);
    assert!(initial.is_none(), "Initially, no key should have a value");

    let previous = assoc_list.insert(DUPLICATED_KEY, SOME_OTHER_VALUE);
    assert_eq!(previous, Some(SOME_VALUE), "The returned value shall be the previous value");

    let another_initial = assoc_list.insert(ANOTHER_KEY, ANOTHER_VALUE);
    assert!(another_initial.is_none(), "Initially, no key should have a value");

    assert!(unique_keys(assoc_list), "Keys shall be unique");
}

// O(n^2)
fn unique_keys<K: PartialEq, V>(assoc_list: AssocList<K, V>) -> bool {
    let mut vec: Vec<_> = assoc_list.into_iter().collect();
    while let Some((key, _value)) = vec.pop() {
        if vec.iter().any(|(remaining_key, _remaining_value)| key == *remaining_key) {
            return false;
        }
    }
    true
}
