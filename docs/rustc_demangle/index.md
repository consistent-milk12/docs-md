# Crate `rustc_demangle`

Demangle Rust compiler symbol names.

This crate provides a `demangle` function which will return a `Demangle`
sentinel value that can be used to learn about the demangled version of a
symbol name. The demangled representation will be the same as the original
if it doesn't look like a mangled symbol name.

`Demangle` can be formatted with the `Display` trait. The alternate
modifier (`#`) can be used to format the symbol name without the
trailing hash value.

# Examples

```rust
use rustc_demangle::demangle;

assert_eq!(demangle("_ZN4testE").to_string(), "test");
assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
assert_eq!(demangle("foo").to_string(), "foo");
// With hash
assert_eq!(format!("{}", demangle("_ZN3foo17h05af221e174051e9E")), "foo::h05af221e174051e9");
// Without hash
assert_eq!(format!("{:#}", demangle("_ZN3foo17h05af221e174051e9E")), "foo");
```

## Structs

### `Demangle<'a>`

```rust
struct Demangle<'a> {
    style: Option<DemangleStyle<'a>>,
    original: &'a str,
    suffix: &'a str,
}
```

Representation of a demangled symbol name.

#### Implementations

- `fn as_str(self: &Self) -> &'a str`

#### Trait Implementations

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TryDemangleError`

```rust
struct TryDemangleError {
    _priv: (),
}
```

Error returned from the `try_demangle` function below when demangling fails.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> TryDemangleError` — [`TryDemangleError`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Demangle<'_>
```

De-mangles a Rust symbol into a more readable version

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

### `try_demangle`

```rust
fn try_demangle(s: &str) -> Result<Demangle<'_>, TryDemangleError>
```

The same as `demangle`, except return an `Err` if the string does not appear
to be a Rust symbol, rather than "demangling" the given string as a no-op.

```rust
extern crate rustc_demangle;

let not_a_rust_symbol = "la la la";

// The `try_demangle` function will reject strings which are not Rust symbols.
assert!(rustc_demangle::try_demangle(not_a_rust_symbol).is_err());

// While `demangle` will just pass the non-symbol through as a no-op.
assert_eq!(rustc_demangle::demangle(not_a_rust_symbol).as_str(), not_a_rust_symbol);
```

