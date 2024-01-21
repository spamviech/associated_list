#![doc = include_str!("../README.md")]
#![no_std]

use core::{
    borrow::Borrow,
    iter::Map,
    mem,
    slice::{Iter, IterMut},
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

    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(self.0.iter())
    }

    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys(self.0.into_iter())
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values(self.0.iter())
    }

    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut(self.0.iter_mut())
    }

    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues(self.0.into_iter())
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, (K, V)> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, (K, V)> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<'_, (K, V)> {
        self.0.drain(..)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Get the [`Entry`] associated with the `key`.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: PartialEq,
    {
        for (index, (contained_key, contained_value)) in self.0.iter_mut().enumerate() {
            if contained_key == &key {
                return Entry::Occupied(OccupiedEntry {
                    vec: &mut self.0,
                    index,
                });
            }
        }
        Entry::Vacant(VacantEntry {
            vec: &mut self.0,
            key,
        })
    }

    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        todo!()
    }

    #[inline]
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        todo!()
    }

    #[inline]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        todo!()
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        todo!()
    }

    /// Insert a new element for the given `key`.
    /// If the [`AssocList`] already contains an element associated with the key, it is replaced and returned.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: PartialEq,
    {
        for (enthaltener_key, enthaltener_value) in &mut self.0 {
            if enthaltener_key == &key {
                let bisher = mem::replace(enthaltener_value, value);
                return Some(bisher);
            }
        }
        self.0.push((key, value));
        None
    }

    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        todo!()
    }

    #[inline]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        todo!()
    }
}

impl<K, V> Extend<(K, V)> for AssocList<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        todo!()
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for AssocList<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        todo!()
    }
}

impl<K: PartialEq, V, const N: usize> From<[(K, V); N]> for AssocList<K, V> {
    #[inline]
    fn from(value: [(K, V); N]) -> Self {
        let mut assoc_list = AssocList::with_capacity(N);
        for (key, value) in value {
            // Bei wiederholten Schlüssel werden frühere Werte überschrieben.
            let _ = assoc_list.insert(key, value);
        }
        assoc_list
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

impl<'a, K, V> IntoIterator for &'a mut AssocList<K, V> {
    type Item = &'a mut (K, V);

    type IntoIter = IterMut<'a, (K, V)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<K: PartialEq, V> FromIterator<(K, V)> for AssocList<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut assoc_list = AssocList::new();
        for (key, value) in iter {
            // Bei wiederholten Schlüssel werden frühere Werte überschrieben.
            let _ = assoc_list.insert(key, value);
        }
        assoc_list
    }
}

pub struct Keys<'a, K, V>(Iter<'a, (K, V)>);

impl<'a, K, V> IntoIterator for Keys<'a, K, V> {
    type Item = &'a K;

    type IntoIter = Map<Iter<'a, (K, V)>, fn(&'a (K, V)) -> &'a K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct IntoKeys<K, V>(IntoIter<(K, V)>);

impl<K, V> IntoIterator for IntoKeys<K, V> {
    type Item = K;

    type IntoIter = Map<IntoIter<(K, V)>, fn((K, V)) -> K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct Values<'a, K, V>(Iter<'a, (K, V)>);

impl<'a, K, V> IntoIterator for Values<'a, K, V> {
    type Item = &'a V;

    type IntoIter = Map<Iter<'a, (K, V)>, fn(&'a (K, V)) -> &'a V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct ValuesMut<'a, K, V>(IterMut<'a, (K, V)>);

impl<'a, K, V> IntoIterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    type IntoIter = Map<IterMut<'a, (K, V)>, fn(&'a mut (K, V)) -> &'a mut V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct IntoValues<K, V>(IntoIter<(K, V)>);

impl<K, V> IntoIterator for IntoValues<K, V> {
    type Item = V;

    type IntoIter = Map<IntoIter<(K, V)>, fn((K, V)) -> V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

/// A view into an [`AssocList`] for a single element.
/// It can be either present or missing.
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(occupied) => occupied.key(),
            Entry::Vacant(vacant) => vacant.key(),
        }
    }

    #[inline]
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(occupied) => occupied.get_mut(),
            Entry::Vacant(vacant) => vacant.insert(value),
        }
    }
}

pub struct OccupiedEntry<'a, K, V> {
    vec: &'a mut Vec<(K, V)>,
    index: usize,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        let (key, _value) = self.vec.get(self.index).expect("Index out of bounds!");
        key
    }

    #[inline]
    pub fn get(self) -> &'a V {
        let (_key, value) = self.vec.get(self.index).expect("Index out of bounds!");
        value
    }

    #[inline]
    pub fn get_mut(self) -> &'a mut V {
        let (_key, value) = self.vec.get_mut(self.index).expect("Index out of bounds!");
        value
    }

    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        self.vec.swap_remove(self.index)
    }

    #[inline]
    pub fn remove(self) -> V {
        let (_key, value) = self.vec.swap_remove(self.index);
        value
    }

    #[inline]
    pub fn insert(&mut self, neuer_value: V) -> V {
        let (_key, value) = self.vec.get_mut(self.index).expect("Index out of bounds!");
        mem::replace(value, neuer_value)
    }
}

pub struct VacantEntry<'a, K, V> {
    vec: &'a mut Vec<(K, V)>,
    key: K,
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Add a new element associated with the [`key`](VacantEntry::key).
    ///
    /// ## Panics
    ///
    /// programming error if [`Vec::last_mut`] returns [`None`] directly after a [`Vec::push`].
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V {
        self.vec.push((self.key, value));
        let (_key, value) = self
            .vec
            .last_mut()
            .expect("Element wurde gerade hinzugefügt!");
        value
    }
}
