# Changelog for associated_list

## Unreleased changes

- Introduce new "allocator_ext"-feature after stabilization of "allocator_api".
- Remove "doc_auto_cfg"-feature. It is available in stable rust and always active.
- Increase msrv to 1.100.

## v0.2.0

- Seal the `Allocator`-trait if the feature "allocator_api" is not enabled.
    Additionally, change the `DefaultAllocator` to a new unnameable type.
- Add `assoc_list!`-macro to create an `AssocList` with elements.
- New methods
  - `capacity`
  - `reserve`
  - `try_reserve`
  - `reserve_exact`
  - `try_reserve_exact`
  - `shrink`
  - `shrink_to_fit`.
- Introduce modules, to improve code-structure.
- Add unit tests.

## v0.1.0

- Initial Release
