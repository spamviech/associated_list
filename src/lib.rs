#![doc = include_str!("../README.md")]
#![no_std]

use core::{
    borrow::Borrow,
    mem,
    ops::{Index, IndexMut},
    slice::{self, Iter},
};

extern crate alloc;
use alloc::vec::{Drain, IntoIter, Vec};
// TODO allocator-feature with nightly requirements

/// An associated list based on a [Vec], providing the usual map functionality.
///
/// The methods are purely based on the [`PartialEq`] implementation of the key types,
/// so most have a runtime characteristic of `O(n)`.
///
/// In general, you should prefer to use either a [`HashMap`](std::collections::HashMap),
/// or a [`BTreeMap`](std::collections::BTreeMap). The [`AssocList`]
/// zu bevorzugen. Die [`AssocList`] exists as a fallback if the key implements
/// neither [`Hash`](std::hash::Hash) nor [`Ord`].
///
/// Note: All methods only require [`PartialEq`] for the key, but there is a strong argument to only use key types
/// that are also (at least nearly) [`Ord`]. For example, elements associated with a [`f32::NAN`]
/// cannot be found or deleted ([`PartialEq::eq`] will alway return `false`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssocList<K, V>(Vec<(K, V)>);

impl<K, V> AssocList<K, V> {
    /// Create a new [`AssocList`].
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        AssocList(Vec::new())
    }

    /// Create a new [`AssocList`] with at least the specified `capacity`.
    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        AssocList(Vec::with_capacity(capacity))
    }

    /// Return an iterator for all keys in the [`AssocList`].
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(self.0.iter())
    }

    /// Return a consuming iterator for all keys in the [`AssocList`].
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys(self.0.into_iter())
    }

    /// Return an iterator for all values in the [`AssocList`].
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values(self.0.iter())
    }

    /// Return an iterator for mutable access to all values in the [`AssocList`].
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut(self.0.iter_mut())
    }

    /// Return a consuming iterator for all values in the [`AssocList`].
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues(self.0.into_iter())
    }

    /// Return an iterator for all key-value pairs in the [`AssocList`].
    #[inline]
    pub fn iter(&self) -> Iter<'_, (K, V)> {
        self.0.iter()
    }

    /// Return an iterator for all key-value pairs in the [`AssocList`].
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut(self.0.iter_mut())
    }

    /// Removes all key-value pairs from the [`AssocList`] in bulk, returning all removed elements as an iterator.
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed elements.
    ///
    /// ## Leaking
    ///
    /// See [`Vec::drain`].
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, (K, V)> {
        self.0.drain(..)
    }

    /// Return the number of key-value pairs currently contained in the [`AssocList`].
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the [`AssocList`] currently contains no element.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears the [`AssocList`], removing all key-value pairs.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Get the [`Entry`] associated with the `key`.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: PartialEq,
    {
        for (index, (contained_key, _contained_value)) in self.0.iter_mut().enumerate() {
            if contained_key == &key {
                return Entry::Occupied(OccupiedEntry {
                    vec: &mut self.0,
                    key,
                    index,
                });
            }
        }
        Entry::Vacant(VacantEntry {
            vec: &mut self.0,
            key,
        })
    }

    /// Get a reference to the value associated with the `key`.
    #[must_use]
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for (contained_key, contained_value) in &self.0 {
            if contained_key.borrow() == key {
                return Some(contained_value);
            }
        }
        None
    }

    /// Get a reference to the key-value pair inside the [`AssocList`] associated with the `key`.
    #[must_use]
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for (contained_key, contained_value) in &self.0 {
            if contained_key.borrow() == key {
                return Some((contained_key, contained_value));
            }
        }
        None
    }

    /// Does the [`AssocList`] contain a value associated with the `key`.
    #[must_use]
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for (contained_key, _contained_value) in &self.0 {
            if contained_key.borrow() == key {
                return true;
            }
        }
        false
    }

    /// Get mutable access to the value associated with the `key`.
    #[must_use]
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for (contained_key, contained_value) in &mut self.0 {
            if Borrow::<Q>::borrow(contained_key) == key {
                return Some(contained_value);
            }
        }
        None
    }

    /// Insert a new element for the given `key`.
    /// If the [`AssocList`] already contains an element associated with the key, it is replaced and returned.
    #[must_use]
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: PartialEq,
    {
        for (contained_key, contained_value) in &mut self.0 {
            if contained_key == &key {
                let bisher = mem::replace(contained_value, value);
                return Some(bisher);
            }
        }
        self.0.push((key, value));
        None
    }

    /// Remove the element associated with the `key` from the [`AssocList`] and return it.
    #[must_use]
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for (index, (enthaltener_key, _enthaltener_value)) in self.0.iter().enumerate() {
            if enthaltener_key.borrow() == key {
                let (_old_key, old_value) = self.0.swap_remove(index);
                return Some(old_value);
            }
        }
        None
    }

    /// Remove the key-value pair associated with the `key` from the [`AssocList`] and return it.
    #[must_use]
    #[inline]
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for (index, (enthaltener_key, _enthaltener_value)) in self.0.iter().enumerate() {
            if enthaltener_key.borrow() == key {
                let old_pair = self.0.swap_remove(index);
                return Some(old_pair);
            }
        }
        None
    }
}

impl<K: PartialEq, V> Extend<(K, V)> for AssocList<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            // Bei wiederholten Schlüssel werden frühere Werte überschrieben.
            let _ = self.insert(key, value);
        }
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for AssocList<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            // Bei wiederholten Schlüssel werden frühere Werte überschrieben.
            let _ = self.insert(key.clone(), value.clone());
        }
    }
}

impl<K: PartialEq, V, const N: usize> From<[(K, V); N]> for AssocList<K, V> {
    #[inline]
    fn from(array: [(K, V); N]) -> Self {
        let mut assoc_list = AssocList::with_capacity(N);
        for (key, value) in array {
            // Bei wiederholten Schlüssel werden frühere Werte überschrieben.
            let _ = assoc_list.insert(key, value);
        }
        assoc_list
    }
}

impl<Q: PartialEq, K: Borrow<Q>, V> Index<Q> for AssocList<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, key: Q) -> &Self::Output {
        self.get(&key).expect("Unknown key")
    }
}

impl<Q: PartialEq, K: Borrow<Q>, V> IndexMut<Q> for AssocList<K, V> {
    #[inline]
    fn index_mut(&mut self, key: Q) -> &mut Self::Output {
        self.get_mut(&key).expect("Unknown key")
    }
}

impl<K, V> IntoIterator for AssocList<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<(K, V)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a AssocList<K, V> {
    type Item = &'a (K, V);

    type IntoIter = Iter<'a, (K, V)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Mutable Iterator for an [`AssocList`]. It is created by the [`iter_mut`](AssocList::iter_mut)-method.
#[derive(Debug)]
#[must_use]
pub struct IterMut<'a, K, V>(slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, value)| (&*key, value))
    }
}

impl<'a, K, V> IntoIterator for &'a mut AssocList<K, V> {
    type Item = (&'a K, &'a mut V);

    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.0.iter_mut())
    }
}

impl<K: PartialEq, V> FromIterator<(K, V)> for AssocList<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut assoc_list = AssocList::new();
        assoc_list.extend(iter);
        assoc_list
    }
}

/// Iterator for the keys of an [`AssocList`]. It is created by the [`keys`](AssocList::keys)-method.
#[derive(Debug)]
#[must_use]
pub struct Keys<'a, K, V>(Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, _value)| key)
    }
}

/// Consuming Iterator for the keys of an [`AssocList`]. It is created by the [`into_keys`](AssocList::into_keys)-method.
#[derive(Debug)]
#[must_use]
pub struct IntoKeys<K, V>(IntoIter<(K, V)>);

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, _value)| key)
    }
}

/// Iterator for the values of an [`AssocList`]. It is created by the [`values`](AssocList::values)-method.
#[derive(Debug)]
#[must_use]
pub struct Values<'a, K, V>(Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_key, value)| value)
    }
}

/// Iterator for the mutable values of an [`AssocList`]. It is created by the [`values_mut`](AssocList::values_mut)-method.
#[derive(Debug)]
#[must_use]
pub struct ValuesMut<'a, K, V>(slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_key, value)| value)
    }
}

/// Consuming Iterator for the values of an [`AssocList`]. It is created by the [`into_values`](AssocList::into_values)-method.
#[derive(Debug)]
#[must_use]
pub struct IntoValues<K, V>(IntoIter<(K, V)>);

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_key, value)| value)
    }
}

/// A view into an [`AssocList`] for a single element.
/// It can be either present or missing.
#[derive(Debug)]
#[must_use]
pub enum Entry<'a, K, V> {
    /// The [`AssocList`] contains a value for the [`key`](Entry::key).
    Occupied(OccupiedEntry<'a, K, V>),
    /// The [`AssocList`] doesn't contain a value for the [`key`](Entry::key).
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V> {
    /// Return the `key` used to create the [`Entry`].
    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(occupied) => occupied.key(),
            Entry::Vacant(vacant) => vacant.key(),
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty,
    /// and returns a mutable reference to the value in the entry.
    #[must_use]
    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(occupied) => occupied.get_mut(),
            Entry::Vacant(vacant) => vacant.insert(default),
        }
    }
}

/// A view into an occupied entry in an [`AssocList`]. It is part of the [`Entry`] enum.
#[derive(Debug)]
#[must_use]
pub struct OccupiedEntry<'a, K, V> {
    /// The vector of the [`AssocList`].
    vec: &'a mut Vec<(K, V)>,
    /// The index of the element.
    index: usize,
    /// The key used to create the [`Entry`].
    key: K,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    /// Return the `key` used to create the [`Entry`].
    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Get a reference to the Element contained in the [`AssocList`].
    ///
    /// ## Panics
    ///
    /// Programming error: if the index of the [`Entry`] is out-of-bounds.
    #[must_use]
    #[inline]
    pub fn get(self) -> &'a V {
        let (_key, value) = self.vec.get(self.index).expect("Index out of bounds!");
        value
    }

    /// Get a mutable reference to the Element contained in the [`AssocList`].
    ///
    /// ## Panics
    ///
    /// Programming error: if the index of the [`Entry`] is out-of-bounds.
    #[must_use]
    #[inline]
    pub fn get_mut(self) -> &'a mut V {
        let (_key, value) = self.vec.get_mut(self.index).expect("Index out of bounds!");
        value
    }

    /// Remove the element from the [`AssocList`], returning the key-value pair.
    ///
    /// ## Panics
    ///
    /// Programming error: if the index of the [`Entry`] is out-of-bounds.
    #[must_use]
    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        self.vec.swap_remove(self.index)
    }

    /// Remove the element from the [`AssocList`], returning the value.
    ///
    /// ## Panics
    ///
    /// Programming error: if the index of the [`Entry`] is out-of-bounds.
    #[must_use]
    #[inline]
    pub fn remove(self) -> V {
        let (_key, value) = self.vec.swap_remove(self.index);
        value
    }

    /// Replace the element from the [`AssocList`], returning the previous value.
    ///
    /// ## Panics
    ///
    /// Programming error: if the index of the [`Entry`] is out-of-bounds.
    #[must_use]
    #[inline]
    pub fn insert(&mut self, neuer_value: V) -> V {
        let (_key, value) = self.vec.get_mut(self.index).expect("Index out of bounds!");
        mem::replace(value, neuer_value)
    }
}

/// A view into a vacant entry in an [`AssocList`]. It is part of the [`Entry`] enum.
#[derive(Debug)]
#[must_use]
pub struct VacantEntry<'a, K, V> {
    /// The vector of the [`AssocList`].
    vec: &'a mut Vec<(K, V)>,
    /// The key used to create the [`Entry`].
    key: K,
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    /// Return the `key` used to create the [`Entry`].
    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Add a new element associated with the [`key`](VacantEntry::key).
    ///
    /// ## Panics
    ///
    /// Programming error: if [`Vec::last_mut`] returns [`None`] directly after a [`Vec::push`].
    #[must_use]
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V {
        self.vec.push((self.key, value));
        let (_key, inserted_value) = self.vec.last_mut().expect("Element has just been added!");
        inserted_value
    }
}
