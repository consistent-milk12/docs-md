*[rustc_demangle](../index.md) / [legacy](index.md)*

---

# Module `legacy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Demangle`](#demangle) | struct | Representation of a demangled symbol name. |
| [`demangle`](#demangle) | fn | De-mangles a Rust symbol into a more readable version |
| [`is_rust_hash`](#is-rust-hash) | fn |  |

## Structs

### `Demangle<'a>`

```rust
struct Demangle<'a> {
    inner: &'a str,
    elements: usize,
}
```

*Defined in [`rustc-demangle-0.1.26/src/legacy.rs:5-9`](../../../.source_1765633015/rustc-demangle-0.1.26/src/legacy.rs#L5-L9)*

Representation of a demangled symbol name.

#### Fields

- **`elements`**: `usize`

  The number of ::-separated elements in the original name.

#### Trait Implementations

##### `impl Any for Demangle<'a>`

- <span id="demangle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Demangle<'a>`

- <span id="demangle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Demangle<'a>`

- <span id="demangle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Demangle<'a>`

- <span id="demangle-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Demangle<'a>`

- <span id="demangle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Demangle<'a>`

- <span id="demangle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Demangle<'a>`

- <span id="demangle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="demangle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Demangle<'a>`

- <span id="demangle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="demangle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Result<(Demangle<'_>, &str), ()>
```

*Defined in [`rustc-demangle-0.1.26/src/legacy.rs:49-98`](../../../.source_1765633015/rustc-demangle-0.1.26/src/legacy.rs#L49-L98)*

De-mangles a Rust symbol into a more readable version

All Rust symbols by default are mangled as they contain characters that
cannot be represented in all object files. The mangling mechanism is similar
to C++'s, but Rust has a few specifics to handle items like lifetimes in
symbols.

This function will take a **mangled** symbol and return a value. When printed,
the de-mangled version will be written. If the symbol does not look like
a mangled symbol, the original value will be written instead.

# Examples

```rust
use rustc_demangle::demangle;

assert_eq!(demangle("_ZN4testE").to_string(), "test");
assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
assert_eq!(demangle("foo").to_string(), "foo");
```

### `is_rust_hash`

```rust
fn is_rust_hash(s: &str) -> bool
```

*Defined in [`rustc-demangle-0.1.26/src/legacy.rs:101-103`](../../../.source_1765633015/rustc-demangle-0.1.26/src/legacy.rs#L101-L103)*

