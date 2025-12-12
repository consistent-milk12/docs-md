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

*Defined in [`rustc-demangle-0.1.26/src/legacy.rs:5-9`](../../../.source_1765521767/rustc-demangle-0.1.26/src/legacy.rs#L5-L9)*

Representation of a demangled symbol name.

#### Fields

- **`elements`**: `usize`

  The number of ::-separated elements in the original name.

#### Trait Implementations

##### `impl Display for Demangle<'a>`

- <span id="demangle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Result<(Demangle<'_>, &str), ()>
```

*Defined in [`rustc-demangle-0.1.26/src/legacy.rs:49-98`](../../../.source_1765521767/rustc-demangle-0.1.26/src/legacy.rs#L49-L98)*

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

*Defined in [`rustc-demangle-0.1.26/src/legacy.rs:101-103`](../../../.source_1765521767/rustc-demangle-0.1.26/src/legacy.rs#L101-L103)*

