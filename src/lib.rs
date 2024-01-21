#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "doc_auto_cfg", feature(doc_auto_cfg))]
#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

#[cfg(feature = "allocator_api")]
use core::alloc::Allocator as ActualAllocator;
use core::{
    borrow::Borrow,
    marker::PhantomData,
    mem,
    ops::{Index, IndexMut},
    slice::{self, Iter},
};

extern crate alloc;
#[cfg(feature = "allocator_api")]
use alloc::alloc::Global;
use alloc::vec::{self, IntoIter, Vec};

#[cfg(feature = "allocator_api")]
/// Helper-trait to reduce the amount of required cfg-pragmas.
///
/// When the feature `allocator_api` is active, resolves to an alias-trait for [`core::alloc::Allocator`].
/// Otherwise, it is just an empty trait, only implemented for [`()`].
pub trait Allocator: ActualAllocator {}
#[cfg(feature = "allocator_api")]
impl<T: ActualAllocator> Allocator for T {}
#[cfg(not(feature = "allocator_api"))]
/// Helper-trait to reduce the amount of required cfg-pragmas.
///
/// When the feature `allocator_api` is active, resolves to an alias-trait for [`core::alloc::Allocator`].
/// Otherwise, it is just an empty trait, only implemented for [`()`].
pub trait Allocator {}
#[cfg(not(feature = "allocator_api"))]
impl Allocator for () {}

#[cfg(feature = "allocator_api")]
/// The default type for the [`Allocator`]-parameter of an [`AssocList`].
///
/// When the feature `allocator_api` is active, resolves to [`Global`](alloc::alloc::Global).
/// Otherwise, it resolves to [`()`].
type DefaultAllocator = Global;
#[cfg(not(feature = "allocator_api"))]
/// The default type for the [`Allocator`]-parameter of an [`AssocList`].
///
/// When the feature `allocator_api` is active, resolves to [`Global`](alloc::alloc::Global).
/// Otherwise, it resolves to [`()`].
type DefaultAllocator = ();

/// An associated list based on a [`Vec`], providing the usual map functionality.
///
/// The methods are purely based on the [`PartialEq`] implementation of the key types,
/// so most have a runtime characteristic of `O(n)`.
///
/// In general, you should prefer to use either a [`HashMap`](std::collections::HashMap),
/// or a [`BTreeMap`](std::collections::BTreeMap).
/// The [`AssocList`] exists as a fallback if the key implements neither [`Hash`](std::hash::Hash) nor [`Ord`].
///
/// Note: All methods only require [`PartialEq`] for the key, but there is a strong argument to only use key types
/// that are also (at least nearly) [`Ord`]. For example, elements associated with a [`f32::NAN`]
/// cannot be found or deleted ([`PartialEq::eq`] will alway return `false`).
#[derive(Debug, Clone)]
pub struct AssocList<K, V, A: Allocator = DefaultAllocator> {
    #[cfg(feature = "allocator_api")]
    /// The vector of the [`AssocList`].
    /// Invariant: all keys (first element of the tuple) are unique.
    vec: Vec<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The vector of the [`AssocList`].
    /// Invariant: all keys (first element of the tuple) are unique.
    vec: Vec<(K, V)>,
    /// PhantomData
    phantom: PhantomData<A>,
}

impl<K, V> AssocList<K, V> {
    /// Create a new [`AssocList`].
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        AssocList {
            vec: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Create a new [`AssocList`] with at least the specified `capacity`.
    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        AssocList {
            vec: Vec::with_capacity(capacity),
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "allocator_api")]
impl<K, V, A: Allocator> AssocList<K, V, A> {
    /// Create a new [`AssocList`] with the provided allocator.
    #[must_use]
    #[inline]
    pub const fn new_in(alloc: A) -> Self {
        AssocList {
            vec: Vec::new_in(alloc),
            phantom: PhantomData,
        }
    }

    /// Create a new [`AssocList`] with at least the specified `capacity` with the provided allocator.
    #[must_use]
    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        AssocList {
            vec: Vec::with_capacity_in(capacity, alloc),
            phantom: PhantomData,
        }
    }
}

impl<K, V, A: Allocator> AssocList<K, V, A> {
    /// Return an iterator for all keys in the [`AssocList`].
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(self.vec.iter())
    }

    /// Return a consuming iterator for all keys in the [`AssocList`].
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        IntoKeys {
            iter: self.vec.into_iter(),
            phantom: self.phantom,
        }
    }

    /// Return an iterator for all values in the [`AssocList`].
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values(self.vec.iter())
    }

    /// Return an iterator for mutable access to all values in the [`AssocList`].
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut(self.vec.iter_mut())
    }

    /// Return a consuming iterator for all values in the [`AssocList`].
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V, A> {
        IntoValues {
            iter: self.vec.into_iter(),
            phantom: self.phantom,
        }
    }

    /// Return an iterator for all key-value pairs in the [`AssocList`].
    #[inline]
    pub fn iter(&self) -> Iter<'_, (K, V)> {
        self.vec.iter()
    }

    /// Return an iterator for all key-value pairs in the [`AssocList`].
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut(self.vec.iter_mut())
    }

    /// Removes all key-value pairs from the [`AssocList`] in bulk, returning all removed elements as an iterator.
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed elements.
    ///
    /// ## Leaking
    ///
    /// See [`Vec::drain`].
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, K, V, A> {
        Drain {
            iter: self.vec.drain(..),
            phantom: self.phantom,
        }
    }

    /// Return the number of key-value pairs currently contained in the [`AssocList`].
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns `true` if the [`AssocList`] currently contains no element.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Clears the [`AssocList`], removing all key-value pairs.
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Get the [`Entry`] associated with the `key`.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: PartialEq,
    {
        for (index, (contained_key, _contained_value)) in self.vec.iter_mut().enumerate() {
            if contained_key == &key {
                return Entry::Occupied(OccupiedEntry {
                    vec: &mut self.vec,
                    phantom: self.phantom,
                    key,
                    index,
                });
            }
        }
        Entry::Vacant(VacantEntry {
            vec: &mut self.vec,
            phantom: self.phantom,
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
        for (contained_key, contained_value) in &self.vec {
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
        for (contained_key, contained_value) in &self.vec {
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
        for (contained_key, _contained_value) in &self.vec {
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
        for (contained_key, contained_value) in &mut self.vec {
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
        for (contained_key, contained_value) in &mut self.vec {
            if contained_key == &key {
                let bisher = mem::replace(contained_value, value);
                return Some(bisher);
            }
        }
        self.vec.push((key, value));
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
        for (index, (enthaltener_key, _enthaltener_value)) in self.vec.iter().enumerate() {
            if enthaltener_key.borrow() == key {
                let (_old_key, old_value) = self.vec.swap_remove(index);
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
        for (index, (enthaltener_key, _enthaltener_value)) in self.vec.iter().enumerate() {
            if enthaltener_key.borrow() == key {
                let old_pair = self.vec.swap_remove(index);
                return Some(old_pair);
            }
        }
        None
    }
}

impl<K: PartialEq, V: PartialEq, A: Allocator> PartialEq for AssocList<K, V, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            // If the lengths don't match the list can't be equal.
            return false;
        }
        // Since all keys are unique, and both lists have the same length,
        // we only have to loop over the first and lookup in the second AssocList:
        // If there is `key` in `other`, that is not part of `self`,
        // then there is at least one `key` in `self`, that is not part of `other`,
        // causing a `false` return value.
        for (key, value) in self {
            if other.get(key) != Some(value) {
                return false;
            }
        }
        true
    }
}

impl<K: Eq, V: Eq, A: Allocator> Eq for AssocList<K, V, A> {}

impl<K: Default, V: Default> Default for AssocList<K, V> {
    #[inline]
    fn default() -> Self {
        Self {
            vec: Vec::new(),
            phantom: PhantomData,
        }
    }
}

impl<K: PartialEq, V, A: Allocator> Extend<(K, V)> for AssocList<K, V, A> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            // Bei wiederholten Schlüssel werden frühere Werte überschrieben.
            let _ = self.insert(key, value);
        }
    }
}

impl<'a, K, V, A: Allocator> Extend<(&'a K, &'a V)> for AssocList<K, V, A>
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

impl<Q: PartialEq, K: Borrow<Q>, V, A: Allocator> Index<Q> for AssocList<K, V, A> {
    type Output = V;

    #[inline]
    fn index(&self, key: Q) -> &Self::Output {
        self.get(&key).expect("Unknown key")
    }
}

impl<Q: PartialEq, K: Borrow<Q>, V, A: Allocator> IndexMut<Q> for AssocList<K, V, A> {
    #[inline]
    fn index_mut(&mut self, key: Q) -> &mut Self::Output {
        self.get_mut(&key).expect("Unknown key")
    }
}

impl<K, V, A: Allocator> IntoIterator for AssocList<K, V, A> {
    type Item = (K, V);

    #[cfg(feature = "allocator_api")]
    type IntoIter = IntoIter<(K, V), A>;
    #[cfg(not(feature = "allocator_api"))]
    type IntoIter = IntoIter<(K, V)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, K, V, A: Allocator> IntoIterator for &'a AssocList<K, V, A> {
    type Item = &'a (K, V);

    type IntoIter = Iter<'a, (K, V)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

/// Draining Iterator for an [`AssocList`]. It is created by the [`drain`](AssocList::drain)-method.
#[derive(Debug)]
#[must_use]
pub struct Drain<'a, K, V, A: Allocator> {
    #[cfg(feature = "allocator_api")]
    /// The Iterator from a [`Vec`] the implementation is based on.
    iter: vec::Drain<'a, (K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The Iterator from a [`Vec`] the implementation is based on.
    iter: vec::Drain<'a, (K, V)>,
    /// PhantomData
    phantom: PhantomData<A>,
}

impl<K, V, A: Allocator> Iterator for Drain<'_, K, V, A> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
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

impl<'a, K, V, A: Allocator> IntoIterator for &'a mut AssocList<K, V, A> {
    type Item = (&'a K, &'a mut V);

    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.vec.iter_mut())
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
pub struct IntoKeys<K, V, A: Allocator> {
    #[cfg(feature = "allocator_api")]
    /// The Iterator from a [`Vec`] the implementation is based on.
    iter: IntoIter<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The Iterator from a [`Vec`] the implementation is based on.
    iter: IntoIter<(K, V)>,
    /// PhantomData
    phantom: PhantomData<A>,
}

impl<K, V, A: Allocator> Iterator for IntoKeys<K, V, A> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _value)| key)
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
pub struct IntoValues<K, V, A: Allocator> {
    #[cfg(feature = "allocator_api")]
    /// The Iterator from a [`Vec`] the implementation is based on.
    iter: IntoIter<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The Iterator from a [`Vec`] the implementation is based on.
    iter: IntoIter<(K, V)>,
    /// PhantomData
    phantom: PhantomData<A>,
}

impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_key, value)| value)
    }
}

/// A view into an [`AssocList`] for a single element.
/// It can be either present or missing.
#[derive(Debug)]
#[must_use]
pub enum Entry<'a, K, V, A: Allocator = DefaultAllocator> {
    /// The [`AssocList`] contains a value for the [`key`](Entry::key).
    Occupied(OccupiedEntry<'a, K, V, A>),
    /// The [`AssocList`] doesn't contain a value for the [`key`](Entry::key).
    Vacant(VacantEntry<'a, K, V, A>),
}

impl<'a, K, V, A: Allocator> Entry<'a, K, V, A> {
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
pub struct OccupiedEntry<'a, K, V, A: Allocator = DefaultAllocator> {
    #[cfg(feature = "allocator_api")]
    /// The vector of the [`AssocList`].
    vec: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The vector of the [`AssocList`].
    vec: &'a mut Vec<(K, V)>,
    /// PhantomData
    phantom: PhantomData<A>,
    /// The index of the element.
    index: usize,
    /// The key used to create the [`Entry`].
    key: K,
}

impl<'a, K, V, A: Allocator> OccupiedEntry<'a, K, V, A> {
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
pub struct VacantEntry<'a, K, V, A: Allocator = DefaultAllocator> {
    #[cfg(feature = "allocator_api")]
    /// The vector of the [`AssocList`].
    vec: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The vector of the [`AssocList`].
    vec: &'a mut Vec<(K, V)>,
    /// PhantomData
    phantom: PhantomData<A>,
    /// The key used to create the [`Entry`].
    key: K,
}

impl<'a, K, V, A: Allocator> VacantEntry<'a, K, V, A> {
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
