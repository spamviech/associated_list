//! Helper-trait to reduce the amount of required cfg-pragmas.

#[cfg(feature = "allocator_api")]
use core::alloc::Allocator as ActualAllocator;

#[cfg(feature = "allocator_api")]
use alloc::alloc::Global;

#[cfg(feature = "allocator_api")]
/// Helper-trait to reduce the amount of required cfg-pragmas.
///
/// When the feature `allocator_api` is active, resolves to an alias-trait for [`core::alloc::Allocator`].
/// Otherwise, it is just an empty trait, only implemented for
/// [`DummyAllocator`](sealed::DummyAllocator).
pub trait Allocator: ActualAllocator {}
#[cfg(feature = "allocator_api")]
impl<T: ActualAllocator> Allocator for T {}
#[cfg(not(feature = "allocator_api"))]
/// Helper-trait to reduce the amount of required cfg-pragmas.
///
/// When the feature `allocator_api` is active, resolves to an alias-trait for [`core::alloc::Allocator`].
/// Otherwise, it is just an empty trait, only implemented for
/// [`DummyAllocator`](sealed::DummyAllocator).
pub trait Allocator: sealed::Sealed {}

#[cfg(not(feature = "allocator_api"))]
/// Private Module to seal the allocator trait.
mod sealed {
    use crate::Allocator;
    /// Public trait with private Name,
    /// ensuring [`Allocator`](crate::Allocator) can't be implemented.
    pub trait Sealed {}
    /// Public type with private Name.
    /// Used as [`DefaultAllocator`](crate::DefaultAllocator)
    /// if the feature `allocator_api` is not enabled.
    ///
    /// This type only exists as a placeholder, and will not be constructed.
    #[allow(missing_copy_implementations, missing_debug_implementations)]
    pub struct DummyAllocator;
    impl Sealed for DummyAllocator {}
    impl Allocator for DummyAllocator {}
}

#[cfg(feature = "allocator_api")]
/// The default type for the [`Allocator`]-parameter of an [`AssocList`](crate::AssocList).
///
/// When the feature `allocator_api` is active, resolves to [`Global`](alloc::alloc::Global).
/// Otherwise, it resolves to [`DummyAllocator`](sealed::DummyAllocator).
pub(crate) type DefaultAllocator = Global;
#[cfg(not(feature = "allocator_api"))]
/// The default type for the [`Allocator`]-parameter of an [`AssocList`](crate::AssocList).
///
/// When the feature `allocator_api` is active, resolves to [`Global`](alloc::alloc::Global).
/// Otherwise, it resolves to [`DummyAllocator`](sealed::DummyAllocator).
pub(crate) type DefaultAllocator = sealed::DummyAllocator;
