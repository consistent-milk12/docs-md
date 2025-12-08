*[portable_atomic](../index.md) / [utils](index.md)*

---

# Module `utils`

## Contents

- [Modules](#modules)
  - [`generated`](#generated)
  - [`ptr`](#ptr)
- [Structs](#structs)
  - [`Pair`](#pair)
- [Functions](#functions)
  - [`_assert_is_bool`](#_assert_is_bool)
  - [`assert_unchecked`](#assert_unchecked)
  - [`assert_load_ordering`](#assert_load_ordering)
  - [`assert_store_ordering`](#assert_store_ordering)
  - [`assert_compare_exchange_ordering`](#assert_compare_exchange_ordering)
  - [`upgrade_success_ordering`](#upgrade_success_ordering)
- [Macros](#macros)
  - [`static_assert!`](#static_assert)
  - [`static_assert_layout!`](#static_assert_layout)
  - [`doc_comment!`](#doc_comment)
  - [`ifunc!`](#ifunc)
  - [`fn_alias!`](#fn_alias)
  - [`const_fn!`](#const_fn)
  - [`impl_debug_and_serde!`](#impl_debug_and_serde)
  - [`impl_debug!`](#impl_debug)
  - [`impl_default_no_fetch_ops!`](#impl_default_no_fetch_ops)
  - [`impl_default_bit_opts!`](#impl_default_bit_opts)
  - [`items!`](#items)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`generated`](#generated) | mod |  |
| [`ptr`](#ptr) | mod |  |
| [`Pair`](#pair) | struct |  |
| [`_assert_is_bool`](#_assert_is_bool) | fn |  |
| [`assert_unchecked`](#assert_unchecked) | fn |  |
| [`assert_load_ordering`](#assert_load_ordering) | fn |  |
| [`assert_store_ordering`](#assert_store_ordering) | fn |  |
| [`assert_compare_exchange_ordering`](#assert_compare_exchange_ordering) | fn |  |
| [`upgrade_success_ordering`](#upgrade_success_ordering) | fn |  |
| [`static_assert!`](#static_assert) | macro |  |
| [`static_assert_layout!`](#static_assert_layout) | macro |  |
| [`doc_comment!`](#doc_comment) | macro |  |
| [`ifunc!`](#ifunc) | macro | # Safety |
| [`fn_alias!`](#fn_alias) | macro |  |
| [`const_fn!`](#const_fn) | macro | Make the given function const if the given condition is true. |
| [`impl_debug_and_serde!`](#impl_debug_and_serde) | macro | Implements `core::fmt::Debug` and `serde::{Serialize, Deserialize}` (when serde |
| [`impl_debug!`](#impl_debug) | macro |  |
| [`impl_default_no_fetch_ops!`](#impl_default_no_fetch_ops) | macro |  |
| [`impl_default_bit_opts!`](#impl_default_bit_opts) | macro |  |
| [`items!`](#items) | macro |  |

## Modules

- [`generated`](generated/index.md) - 
- [`ptr`](ptr/index.md) - 

## Structs

### `Pair<T: Copy>`

```rust
struct Pair<T: Copy> {
    lo: T,
    hi: T,
}
```

#### Trait Implementations

##### `impl<T: clone::Clone + Copy> Clone for Pair<T>`

- <span id="pair-clone"></span>`fn clone(&self) -> Pair<T>` — [`Pair`](#pair)

##### `impl<T: marker::Copy + Copy> Copy for Pair<T>`

## Functions

### `_assert_is_bool`

```rust
const fn _assert_is_bool(v: bool) -> bool
```

### `assert_unchecked`

```rust
unsafe fn assert_unchecked(cond: bool)
```

### `assert_load_ordering`

```rust
fn assert_load_ordering(order: core::sync::atomic::Ordering)
```

### `assert_store_ordering`

```rust
fn assert_store_ordering(order: core::sync::atomic::Ordering)
```

### `assert_compare_exchange_ordering`

```rust
fn assert_compare_exchange_ordering(success: core::sync::atomic::Ordering, failure: core::sync::atomic::Ordering)
```

### `upgrade_success_ordering`

```rust
fn upgrade_success_ordering(success: core::sync::atomic::Ordering, failure: core::sync::atomic::Ordering) -> core::sync::atomic::Ordering
```

## Macros

### `static_assert!`

### `static_assert_layout!`

### `doc_comment!`

### `ifunc!`

# Safety

- the caller must uphold the safety contract for the function returned by $detect_body.
- the memory pointed by the function pointer returned by $detect_body must be visible from any threads.

The second requirement is always met if the function pointer is to the function definition.
(Currently, all uses of this macro in our code are in this case.)

### `fn_alias!`

### `const_fn!`

Make the given function const if the given condition is true.

### `impl_debug_and_serde!`

Implements `core::fmt::Debug` and `serde::{Serialize, Deserialize}` (when serde
feature is enabled) for atomic bool, integer, or float.

### `impl_debug!`

### `impl_default_no_fetch_ops!`

### `impl_default_bit_opts!`

### `items!`

