*[cargo_docs_md](../index.md) / [utils](index.md)*

---

# Module `utils`

Shared utility functions used across the documentation generator.

This module contains small, general-purpose helpers that are used
by multiple components and don't belong to any specific domain.

# Organization

Utilities are organized into unit structs by category:
- [`PathUtils`](#pathutils): Rust path manipulation utilities

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PathUtils`](#pathutils) | struct | Utilities for working with Rust paths (e.g., `std::vec::Vec`). |

## Structs

### `PathUtils`

```rust
struct PathUtils;
```

*Defined in `src/utils.rs:31`*

Utilities for working with Rust paths (e.g., `std::vec::Vec`).

Rust paths use `::` as a separator. This struct provides methods
for common path operations used throughout the documentation generator.

# Design Note

This is a unit struct (no fields) that serves as a namespace for
related utility functions. All methods are associated functions
that don't require an instance.

# Examples

```rust
use cargo_docs_md::utils::PathUtils;

// Extract the short name from a qualified path
assert_eq!(PathUtils::short_name("std::vec::Vec"), "Vec");
assert_eq!(PathUtils::short_name("Clone"), "Clone");
```

#### Implementations

- <span id="pathutils-short-name"></span>`fn short_name(path: &str) -> &str`

  Extract the short name (last segment) from a qualified Rust path.

  

  Rust paths use `::` as a separator. This function returns the final

  segment, which is typically the item's simple name without module prefix.

  

  # Examples

  

  ```rust

  use cargo_docs_md::utils::PathUtils;

  

  assert_eq!(PathUtils::short_name("std::vec::Vec"), "Vec");

  assert_eq!(PathUtils::short_name("std::collections::HashMap"), "HashMap");

  assert_eq!(PathUtils::short_name("Clone"), "Clone");

  assert_eq!(PathUtils::short_name(""), "");

  ```

  

  # Edge Cases

  

  - Empty string returns empty string

  - Path ending with `::` returns empty string (e.g., `"foo::"` -> `""`)

  - Single segment returns itself (e.g., `"Vec"` -> `"Vec"`)

#### Trait Implementations

##### `impl Any for PathUtils`

- <span id="pathutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathUtils`

- <span id="pathutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathUtils`

- <span id="pathutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for PathUtils`

- <span id="pathutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for PathUtils`

##### `impl<U> Into for PathUtils`

- <span id="pathutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PathUtils`

##### `impl OwoColorize for PathUtils`

##### `impl Pointable for PathUtils`

- <span id="pathutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="pathutils-pointable-type-init"></span>`type Init = T`

- <span id="pathutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="pathutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="pathutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="pathutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PathUtils`

- <span id="pathutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathUtils`

- <span id="pathutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for PathUtils`

