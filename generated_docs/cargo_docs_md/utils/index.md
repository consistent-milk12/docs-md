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

#### Trait Implementations

##### `impl Instrument for PathUtils`

##### `impl IntoEither for PathUtils`

##### `impl OwoColorize for PathUtils`

##### `impl Pointable for PathUtils`

- <span id="pathutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="pathutils-pointable-type-init"></span>`type Init = T`

- <span id="pathutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="pathutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="pathutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="pathutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for PathUtils`

