//! Unit tests for the [`Entry`]-API of an [`AssocList`].

// signal failure in cases where `assert!` doesn't work, e.g. in a let-else.
#![allow(clippy::panic)]

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
fn occupied_entry_key() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Occupied(occupied_entry) = assoc_list.entry("occupied") else {
        panic!("Entry not occupied!");
    };
    assert_eq!(*occupied_entry.key(), OCCUPIED);
}

#[test]
fn occupied_entry_get() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Occupied(occupied_entry) = assoc_list.entry(OCCUPIED) else {
        panic!("Entry not occupied!");
    };
    assert_eq!(*occupied_entry.get(), VALUE);
}

#[test]
fn occupied_entry_get_mut() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    const NEW_VALUE: &str = "new value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Occupied(occupied_entry) = assoc_list.entry(OCCUPIED) else {
        panic!("Entry not occupied!");
    };
    let value = occupied_entry.get_mut();
    assert_eq!(*value, VALUE);
    *value = NEW_VALUE;
    assert_eq!(assoc_list.get(OCCUPIED), Some(&NEW_VALUE));
    assert_eq!(assoc_list.get(ANOTHER), Some(&ANOTHER_VALUE));
}

#[test]
fn occupied_entry_remove_entry() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Occupied(occupied_entry) = assoc_list.entry(OCCUPIED) else {
        panic!("Entry not occupied!");
    };
    let (key, value) = occupied_entry.remove_entry();
    assert_eq!(key, OCCUPIED);
    assert_eq!(value, VALUE);
    assert!(!assoc_list.contains_key(OCCUPIED));
    assert!(assoc_list.contains_key(ANOTHER));
}

#[test]
fn occupied_entry_remove() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Occupied(occupied_entry) = assoc_list.entry(OCCUPIED) else {
        panic!("Entry not occupied!");
    };
    let value = occupied_entry.remove();
    assert_eq!(value, VALUE);
    assert!(!assoc_list.contains_key(OCCUPIED));
    assert_eq!(assoc_list.get(ANOTHER), Some(&ANOTHER_VALUE));
}

#[test]
fn occupied_entry_insert() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    const NEW_VALUE: &str = "new value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Occupied(mut occupied_entry) = assoc_list.entry(OCCUPIED) else {
        panic!("Entry not occupied!");
    };
    let value = occupied_entry.insert(NEW_VALUE);
    assert_eq!(value, VALUE);
    assert_eq!(*occupied_entry.get(), NEW_VALUE);
    assert_eq!(assoc_list.get(OCCUPIED), Some(&NEW_VALUE));
    assert_eq!(assoc_list.get(ANOTHER), Some(&ANOTHER_VALUE));
}

#[test]
fn vacant_entry_key() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    const VACANT: &str = "vacant";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Vacant(vacant_entry) = assoc_list.entry(VACANT) else {
        panic!("Entry not vacant!");
    };
    assert_eq!(*vacant_entry.key(), VACANT);
}

#[test]
fn vacant_entry_insert() {
    const OCCUPIED: &str = "occupied";
    const VALUE: &str = "value";
    const ANOTHER: &str = "another";
    const ANOTHER_VALUE: &str = "another value";
    const VACANT: &str = "vacant";
    const NEW_VALUE: &str = "new value";
    let mut assoc_list = assoc_list!((OCCUPIED, VALUE), (ANOTHER, ANOTHER_VALUE));
    let Entry::Vacant(vacant_entry) = assoc_list.entry(VACANT) else {
        panic!("Entry not vacant!");
    };
    let new_value = vacant_entry.insert(NEW_VALUE);
    assert_eq!(*new_value, NEW_VALUE);
    assert_eq!(assoc_list.get(VACANT), Some(&NEW_VALUE));
    assert_eq!(assoc_list.get(OCCUPIED), Some(&VALUE));
    assert_eq!(assoc_list.get(ANOTHER), Some(&ANOTHER_VALUE));
}
