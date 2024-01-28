//! Unit tests for the iterators of an [`AssocList`].

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};

use quickcheck_macros::quickcheck;

use crate::{test::split_tuple_refs, AssocList};

#[test]
fn keys() {
    const CAPACITY: usize = 5;
    let mut assoc_list: AssocList<i8, i8> = AssocList::with_capacity(CAPACITY);
    for i in 0..i8::try_from(CAPACITY).expect("small test number") {
        let previous = assoc_list.insert(-i - 1, i);
        assert!(previous.is_none());
    }
    assert!(assoc_list.keys().copied().all(i8::is_negative));
}

#[test]
fn into_keys() {
    const CAPACITY: usize = 17;
    let mut assoc_list: AssocList<i8, i8> = AssocList::with_capacity(CAPACITY);
    for i in 0..i8::try_from(CAPACITY).expect("small test number") {
        let previous = assoc_list.insert(i + 1, -i);
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
