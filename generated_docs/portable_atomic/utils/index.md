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
  - [`_assert_is_bool`](#assert-is-bool)
  - [`assert_unchecked`](#assert-unchecked)
  - [`assert_load_ordering`](#assert-load-ordering)
  - [`assert_store_ordering`](#assert-store-ordering)
  - [`assert_compare_exchange_ordering`](#assert-compare-exchange-ordering)
  - [`upgrade_success_ordering`](#upgrade-success-ordering)
- [Macros](#macros)
  - [`static_assert!`](#static-assert)
  - [`static_assert_layout!`](#static-assert-layout)
  - [`doc_comment!`](#doc-comment)
  - [`ifunc!`](#ifunc)
  - [`fn_alias!`](#fn-alias)
  - [`const_fn!`](#const-fn)
  - [`impl_debug_and_serde!`](#impl-debug-and-serde)
  - [`impl_debug!`](#impl-debug)
  - [`impl_default_no_fetch_ops!`](#impl-default-no-fetch-ops)
  - [`impl_default_bit_opts!`](#impl-default-bit-opts)
  - [`items!`](#items)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`generated`](#generated) | mod |  |
| [`ptr`](#ptr) | mod |  |
| [`Pair`](#pair) | struct |  |
| [`_assert_is_bool`](#assert-is-bool) | fn |  |
| [`assert_unchecked`](#assert-unchecked) | fn |  |
| [`assert_load_ordering`](#assert-load-ordering) | fn |  |
| [`assert_store_ordering`](#assert-store-ordering) | fn |  |
| [`assert_compare_exchange_ordering`](#assert-compare-exchange-ordering) | fn |  |
| [`upgrade_success_ordering`](#upgrade-success-ordering) | fn |  |
| [`static_assert!`](#static-assert) | macro |  |
| [`static_assert_layout!`](#static-assert-layout) | macro |  |
| [`doc_comment!`](#doc-comment) | macro |  |
| [`ifunc!`](#ifunc) | macro | # Safety |
| [`fn_alias!`](#fn-alias) | macro |  |
| [`const_fn!`](#const-fn) | macro | Make the given function const if the given condition is true. |
| [`impl_debug_and_serde!`](#impl-debug-and-serde) | macro | Implements `core::fmt::Debug` and `serde::{Serialize, Deserialize}` (when serde feature is enabled) for atomic bool, integer, or float. |
| [`impl_debug!`](#impl-debug) | macro |  |
| [`impl_default_no_fetch_ops!`](#impl-default-no-fetch-ops) | macro |  |
| [`impl_default_bit_opts!`](#impl-default-bit-opts) | macro |  |
| [`items!`](#items) | macro |  |

## Modules

- [`generated`](generated/index.md)
- [`ptr`](ptr/index.md)

## Structs

### `Pair<T: Copy>`

```rust
struct Pair<T: Copy> {
    lo: T,
    hi: T,
}
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:393-411`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L393-L411)*

#### Trait Implementations

##### `impl<T> Any for Pair<T>`

- <span id="pair-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pair<T>`

- <span id="pair-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pair<T>`

- <span id="pair-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + Copy> Clone for Pair<T>`

- <span id="pair-clone"></span>`fn clone(&self) -> Pair<T>` — [`Pair`](#pair)

##### `impl<T> CloneToUninit for Pair<T>`

- <span id="pair-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy + Copy> Copy for Pair<T>`

##### `impl<T> From for Pair<T>`

- <span id="pair-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Pair<T>`

- <span id="pair-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Pair<T>`

- <span id="pair-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pair-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Pair<T>`

- <span id="pair-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pair-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `_assert_is_bool`

```rust
const fn _assert_is_bool(v: bool) -> bool
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:16-18`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L16-L18)*

### `assert_unchecked`

```rust
unsafe fn assert_unchecked(cond: bool)
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:273-282`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L273-L282)*

### `assert_load_ordering`

```rust
fn assert_load_ordering(order: core::sync::atomic::Ordering)
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:287-294`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L287-L294)*

### `assert_store_ordering`

```rust
fn assert_store_ordering(order: core::sync::atomic::Ordering)
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:298-305`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L298-L305)*

### `assert_compare_exchange_ordering`

```rust
fn assert_compare_exchange_ordering(success: core::sync::atomic::Ordering, failure: core::sync::atomic::Ordering)
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:309-324`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L309-L324)*

### `upgrade_success_ordering`

```rust
fn upgrade_success_ordering(success: core::sync::atomic::Ordering, failure: core::sync::atomic::Ordering) -> core::sync::atomic::Ordering
```

*Defined in [`portable-atomic-1.11.1/src/utils.rs:330-337`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L330-L337)*

## Macros

### `static_assert!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:11-15`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L11-L15)*

### `static_assert_layout!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:20-27`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L20-L27)*

### `doc_comment!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:30-35`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L30-L35)*

### `ifunc!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:56-79`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L56-L79)*

# Safety

- the caller must uphold the safety contract for the function returned by $detect_body.
- the memory pointed by the function pointer returned by $detect_body must be visible from any threads.

The second requirement is always met if the function pointer is to the function definition.
(Currently, all uses of this macro in our code are in this case.)

### `fn_alias!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:92-116`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L92-L116)*

### `const_fn!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:119-132`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L119-L132)*

Make the given function const if the given condition is true.

### `impl_debug_and_serde!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:136-170`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L136-L170)*

Implements `core::fmt::Debug` and `serde::{Serialize, Deserialize}` (when serde
feature is enabled) for atomic bool, integer, or float.

### `impl_debug!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:171-181`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L171-L181)*

### `impl_default_no_fetch_ops!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:185-234`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L185-L234)*

### `impl_default_bit_opts!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:235-258`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L235-L258)*

### `items!`

*Defined in [`portable-atomic-1.11.1/src/utils.rs:261-265`](../../../.source_1765521767/portable-atomic-1.11.1/src/utils.rs#L261-L265)*

