//! Unit tests for the [`Entry`]-API of an [`AssocList`].

use crate::{assoc_list, AssocList, Entry};

#[test]
fn entry() {
    let mut assoc_list = assoc_list!(("occupied", "value"), ("another", "another value"));

    let occupied_entry = assoc_list.entry("occupied");
    assert_eq!(*occupied_entry.key(), "occupied");
    assert!(matches!(occupied_entry, Entry::Occupied(_occupied_entry)));

    let another_entry = assoc_list.entry("another");
    assert_eq!(*another_entry.key(), "another");
    assert_eq!(*another_entry.or_insert("some other value"), "another value");

    let vacant_entry = assoc_list.entry("vacant");
    assert_eq!(*vacant_entry.key(), "vacant");
    assert!(matches!(vacant_entry, Entry::Vacant(_vacant_entry)));

    let another_vacant_entry = assoc_list.entry("another vacant");
    assert_eq!(*another_vacant_entry.key(), "another vacant");
    assert_eq!(*another_vacant_entry.or_insert("yet another value"), "yet another value");
}

#[test]
fn occupied_entry() {
    todo!()
}

#[test]
fn vacant_entry() {
    todo!()
}
