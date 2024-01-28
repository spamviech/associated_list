//! Types for the [`Entry`]-API of an [`AssocList`](crate::AssocList).

use core::{marker::PhantomData, mem};

use alloc::vec::Vec;

use crate::allocator::{Allocator, DefaultAllocator};

/// A view into an [`AssocList`](crate::AssocList) for a single element.
/// It can be either present or missing.
#[derive(Debug)]
#[must_use]
pub enum Entry<'a, K, V, A: Allocator = DefaultAllocator> {
    /// The [`AssocList`](crate::AssocList) contains a value for the [`key`](Entry::key).
    Occupied(OccupiedEntry<'a, K, V, A>),
    /// The [`AssocList`](crate::AssocList) doesn't contain a value for the [`key`](Entry::key).
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

// mimicking Entry-API for e.g. BTreeMap
#[allow(clippy::module_name_repetitions)]
/// A view into an occupied entry in an [`AssocList`](crate::AssocList).
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
#[must_use]
pub struct OccupiedEntry<'a, K, V, A: Allocator = DefaultAllocator> {
    #[cfg(feature = "allocator_api")]
    /// The vector of the [`AssocList`](crate::AssocList).
    pub(crate) vec: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The vector of the [`AssocList`](crate::AssocList).
    pub(crate) vec: &'a mut Vec<(K, V)>,
    /// PhantomData
    pub(crate) phantom: PhantomData<A>,
    /// The index of the element.
    pub(crate) index: usize,
    /// The key used to create the [`Entry`].
    pub(crate) key: K,
}

impl<'a, K, V, A: Allocator> OccupiedEntry<'a, K, V, A> {
    /// Return the `key` used to create the [`Entry`].
    #[must_use]
    #[inline]
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Get a reference to the Element contained in the [`AssocList`](crate::AssocList).
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

    /// Get a mutable reference to the Element contained in the [`AssocList`](crate::AssocList).
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

    /// Remove the element from the [`AssocList`](crate::AssocList), returning the key-value pair.
    ///
    /// ## Panics
    ///
    /// Programming error: if the index of the [`Entry`] is out-of-bounds.
    #[must_use]
    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        self.vec.swap_remove(self.index)
    }

    /// Remove the element from the [`AssocList`](crate::AssocList), returning the value.
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

    /// Replace the element from the [`AssocList`](crate::AssocList), returning the previous value.
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

// mimicking Entry-API for e.g. BTreeMap
#[allow(clippy::module_name_repetitions)]
/// A view into a vacant entry in an [`AssocList`](crate::AssocList).
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
#[must_use]
pub struct VacantEntry<'a, K, V, A: Allocator = DefaultAllocator> {
    #[cfg(feature = "allocator_api")]
    /// The vector of the [`AssocList`](crate::AssocList).
    pub(crate) vec: &'a mut Vec<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The vector of the [`AssocList`](crate::AssocList).
    pub(crate) vec: &'a mut Vec<(K, V)>,
    /// PhantomData
    pub(crate) phantom: PhantomData<A>,
    /// The key used to create the [`Entry`].
    pub(crate) key: K,
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
    /// Programming error: if [`slice::last_mut`] returns [`None`] directly after a [`Vec::push`].
    #[must_use]
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V {
        self.vec.push((self.key, value));
        let (_key, inserted_value) = self.vec.last_mut().expect("Element has just been added!");
        inserted_value
    }
}
