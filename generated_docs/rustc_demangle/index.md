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

## Contents

- [Modules](#modules)
  - [`legacy`](#legacy)
  - [`v0`](#v0)
- [Structs](#structs)
  - [`Demangle`](#demangle)
  - [`TryDemangleError`](#trydemangleerror)
  - [`SizeLimitExhausted`](#sizelimitexhausted)
  - [`SizeLimitedFmtAdapter`](#sizelimitedfmtadapter)
- [Enums](#enums)
  - [`DemangleStyle`](#demanglestyle)
- [Functions](#functions)
  - [`demangle`](#demangle)
  - [`try_demangle`](#try_demangle)
  - [`is_symbol_like`](#is_symbol_like)
  - [`is_ascii_alphanumeric`](#is_ascii_alphanumeric)
  - [`is_ascii_punctuation`](#is_ascii_punctuation)
- [Constants](#constants)
  - [`MAX_SIZE`](#max_size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`legacy`](#legacy) | mod |  |
| [`v0`](#v0) | mod |  |
| [`Demangle`](#demangle) | struct | Representation of a demangled symbol name. |
| [`TryDemangleError`](#trydemangleerror) | struct | Error returned from the `try_demangle` function below when demangling fails. |
| [`SizeLimitExhausted`](#sizelimitexhausted) | struct |  |
| [`SizeLimitedFmtAdapter`](#sizelimitedfmtadapter) | struct |  |
| [`DemangleStyle`](#demanglestyle) | enum |  |
| [`demangle`](#demangle) | fn | De-mangles a Rust symbol into a more readable version |
| [`try_demangle`](#try_demangle) | fn | The same as `demangle`, except return an `Err` if the string does not appear |
| [`is_symbol_like`](#is_symbol_like) | fn |  |
| [`is_ascii_alphanumeric`](#is_ascii_alphanumeric) | fn |  |
| [`is_ascii_punctuation`](#is_ascii_punctuation) | fn |  |
| [`MAX_SIZE`](#max_size) | const |  |

## Modules

- [`legacy`](legacy/index.md) - 
- [`v0`](v0/index.md) - 

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

- <span id="demangle-as-str"></span>`fn as_str(&self) -> &'a str`

#### Trait Implementations

##### `impl<'a> Debug for Demangle<'a>`

- <span id="demangle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Display for Demangle<'a>`

- <span id="demangle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TryDemangleError`

```rust
struct TryDemangleError {
    _priv: (),
}
```

Error returned from the `try_demangle` function below when demangling fails.

#### Trait Implementations

##### `impl Clone for TryDemangleError`

- <span id="trydemangleerror-clone"></span>`fn clone(&self) -> TryDemangleError` — [`TryDemangleError`](#trydemangleerror)

##### `impl Debug for TryDemangleError`

- <span id="trydemangleerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SizeLimitExhausted`

```rust
struct SizeLimitExhausted;
```

#### Trait Implementations

##### `impl Clone for SizeLimitExhausted`

- <span id="sizelimitexhausted-clone"></span>`fn clone(&self) -> SizeLimitExhausted` — [`SizeLimitExhausted`](#sizelimitexhausted)

##### `impl Copy for SizeLimitExhausted`

##### `impl Debug for SizeLimitExhausted`

- <span id="sizelimitexhausted-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SizeLimitedFmtAdapter<F>`

```rust
struct SizeLimitedFmtAdapter<F> {
    remaining: Result<usize, SizeLimitExhausted>,
    inner: F,
}
```

#### Trait Implementations

##### `impl<F: fmt::Write> Write for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

## Enums

### `DemangleStyle<'a>`

```rust
enum DemangleStyle<'a> {
    Legacy(legacy::Demangle<'a>),
    V0(v0::Demangle<'a>),
}
```

#### Trait Implementations

##### `impl<'a> Display for DemangleStyle<'a>`

- <span id="demanglestyle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

### `is_symbol_like`

```rust
fn is_symbol_like(s: &str) -> bool
```

### `is_ascii_alphanumeric`

```rust
fn is_ascii_alphanumeric(c: char) -> bool
```

### `is_ascii_punctuation`

```rust
fn is_ascii_punctuation(c: char) -> bool
```

## Constants

### `MAX_SIZE`

```rust
const MAX_SIZE: usize = 1_000_000usize;
```

