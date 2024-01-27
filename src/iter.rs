//! Iterators for an [`AssocList`](crate::AssocList) where using the [`Vec`](alloc::vec::Vec)-Iterators directly was not possible.

use core::{
    marker::PhantomData,
    slice::{self, Iter},
};

use alloc::vec::{self, IntoIter};

use crate::allocator::Allocator;

/// Draining Iterator for an [`AssocList`](crate::AssocList).
/// It is created by the [`drain`](crate::AssocList::drain)-method.
#[derive(Debug)]
#[must_use]
pub struct Drain<'a, K, V, A: Allocator> {
    #[cfg(feature = "allocator_api")]
    /// The Iterator from a [`Vec`](alloc::vec::Vec) the implementation is based on.
    pub(crate) iter: vec::Drain<'a, (K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The Iterator from a [`Vec`](alloc::vec::Vec) the implementation is based on.
    pub(crate) iter: vec::Drain<'a, (K, V)>,
    /// PhantomData
    pub(crate) phantom: PhantomData<A>,
}

impl<K, V, A: Allocator> Iterator for Drain<'_, K, V, A> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// following convention for Iterator-names
#[allow(clippy::module_name_repetitions)]
/// Mutable Iterator for an [`AssocList`](crate::AssocList).
/// It is created by the [`iter_mut`](crate::AssocList::iter_mut)-method.
#[derive(Debug)]
#[must_use]
pub struct IterMut<'a, K, V>(pub(crate) slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, value)| (&*key, value))
    }
}

/// Iterator for the keys of an [`AssocList`](crate::AssocList).
/// It is created by the [`keys`](crate::AssocList::keys)-method.
#[derive(Debug)]
#[must_use]
pub struct Keys<'a, K, V>(pub(crate) Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, _value)| key)
    }
}

/// Consuming Iterator for the keys of an [`AssocList`](crate::AssocList).
/// It is created by the [`into_keys`](crate::AssocList::into_keys)-method.
#[derive(Debug)]
#[must_use]
pub struct IntoKeys<K, V, A: Allocator> {
    #[cfg(feature = "allocator_api")]
    /// The Iterator from a [`Vec`](alloc::vec::Vec) the implementation is based on.
    pub(crate) iter: IntoIter<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The Iterator from a [`Vec`](alloc::vec::Vec) the implementation is based on.
    pub(crate) iter: IntoIter<(K, V)>,
    /// PhantomData
    pub(crate) phantom: PhantomData<A>,
}

impl<K, V, A: Allocator> Iterator for IntoKeys<K, V, A> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _value)| key)
    }
}

/// Iterator for the values of an [`AssocList`](crate::AssocList).
/// It is created by the [`values`](crate::AssocList::values)-method.
#[derive(Debug)]
#[must_use]
pub struct Values<'a, K, V>(pub(crate) Iter<'a, (K, V)>);

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_key, value)| value)
    }
}

/// Iterator for the mutable values of an [`AssocList`](crate::AssocList).
/// It is created by the [`values_mut`](crate::AssocList::values_mut)-method.
#[derive(Debug)]
#[must_use]
pub struct ValuesMut<'a, K, V>(pub(crate) slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_key, value)| value)
    }
}

/// Consuming Iterator for the values of an [`AssocList`](crate::AssocList).
/// It is created by the [`into_values`](crate::AssocList::into_values)-method.
#[derive(Debug)]
#[must_use]
pub struct IntoValues<K, V, A: Allocator> {
    #[cfg(feature = "allocator_api")]
    /// The Iterator from a [`Vec`](alloc::vec::Vec) the implementation is based on.
    pub(crate) iter: IntoIter<(K, V), A>,
    #[cfg(not(feature = "allocator_api"))]
    /// The Iterator from a [`Vec`](alloc::vec::Vec) the implementation is based on.
    pub(crate) iter: IntoIter<(K, V)>,
    /// PhantomData
    pub(crate) phantom: PhantomData<A>,
}

impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_key, value)| value)
    }
}
