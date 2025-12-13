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
  - [`try_demangle`](#try-demangle)
  - [`is_symbol_like`](#is-symbol-like)
  - [`is_ascii_alphanumeric`](#is-ascii-alphanumeric)
  - [`is_ascii_punctuation`](#is-ascii-punctuation)
- [Constants](#constants)
  - [`MAX_SIZE`](#max-size)

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
| [`try_demangle`](#try-demangle) | fn | The same as `demangle`, except return an `Err` if the string does not appear to be a Rust symbol, rather than "demangling" the given string as a no-op. |
| [`is_symbol_like`](#is-symbol-like) | fn |  |
| [`is_ascii_alphanumeric`](#is-ascii-alphanumeric) | fn |  |
| [`is_ascii_punctuation`](#is-ascii-punctuation) | fn |  |
| [`MAX_SIZE`](#max-size) | const |  |

## Modules

- [`legacy`](legacy/index.md)
- [`v0`](v0/index.md)

## Structs

### `Demangle<'a>`

```rust
struct Demangle<'a> {
    style: Option<DemangleStyle<'a>>,
    original: &'a str,
    suffix: &'a str,
}
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:66-70`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L66-L70)*

Representation of a demangled symbol name.

#### Implementations

- <span id="demangle-as-str"></span>`fn as_str(&self) -> &'a str`

  Returns the underlying string that's being demangled.

#### Trait Implementations

##### `impl Any for Demangle<'a>`

- <span id="demangle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Demangle<'a>`

- <span id="demangle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Demangle<'a>`

- <span id="demangle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Demangle<'a>`

- <span id="demangle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

### `TryDemangleError`

```rust
struct TryDemangleError {
    _priv: (),
}
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:219-221`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L219-L221)*

Error returned from the `try_demangle` function below when demangling fails.

#### Trait Implementations

##### `impl Any for TryDemangleError`

- <span id="trydemangleerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryDemangleError`

- <span id="trydemangleerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryDemangleError`

- <span id="trydemangleerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TryDemangleError`

- <span id="trydemangleerror-clone"></span>`fn clone(&self) -> TryDemangleError` — [`TryDemangleError`](#trydemangleerror)

##### `impl CloneToUninit for TryDemangleError`

- <span id="trydemangleerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TryDemangleError`

- <span id="trydemangleerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryDemangleError`

- <span id="trydemangleerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryDemangleError`

- <span id="trydemangleerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TryDemangleError`

- <span id="trydemangleerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trydemangleerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryDemangleError`

- <span id="trydemangleerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trydemangleerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SizeLimitExhausted`

```rust
struct SizeLimitExhausted;
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:293`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L293)*

#### Trait Implementations

##### `impl Any for SizeLimitExhausted`

- <span id="sizelimitexhausted-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SizeLimitExhausted`

- <span id="sizelimitexhausted-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SizeLimitExhausted`

- <span id="sizelimitexhausted-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SizeLimitExhausted`

- <span id="sizelimitexhausted-clone"></span>`fn clone(&self) -> SizeLimitExhausted` — [`SizeLimitExhausted`](#sizelimitexhausted)

##### `impl CloneToUninit for SizeLimitExhausted`

- <span id="sizelimitexhausted-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SizeLimitExhausted`

##### `impl Debug for SizeLimitExhausted`

- <span id="sizelimitexhausted-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SizeLimitExhausted`

- <span id="sizelimitexhausted-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SizeLimitExhausted`

- <span id="sizelimitexhausted-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SizeLimitExhausted`

- <span id="sizelimitexhausted-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sizelimitexhausted-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SizeLimitExhausted`

- <span id="sizelimitexhausted-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sizelimitexhausted-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SizeLimitedFmtAdapter<F>`

```rust
struct SizeLimitedFmtAdapter<F> {
    remaining: Result<usize, SizeLimitExhausted>,
    inner: F,
}
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:295-298`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L295-L298)*

#### Trait Implementations

##### `impl Any for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sizelimitedfmtadapter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sizelimitedfmtadapter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<F: fmt::Write> Write for SizeLimitedFmtAdapter<F>`

- <span id="sizelimitedfmtadapter-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

## Enums

### `DemangleStyle<'a>`

```rust
enum DemangleStyle<'a> {
    Legacy(legacy::Demangle<'a>),
    V0(v0::Demangle<'a>),
}
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:72-75`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L72-L75)*

#### Trait Implementations

##### `impl Any for DemangleStyle<'a>`

- <span id="demanglestyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DemangleStyle<'a>`

- <span id="demanglestyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DemangleStyle<'a>`

- <span id="demanglestyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for DemangleStyle<'a>`

- <span id="demanglestyle-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DemangleStyle<'a>`

- <span id="demanglestyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DemangleStyle<'a>`

- <span id="demanglestyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DemangleStyle<'a>`

- <span id="demanglestyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="demanglestyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DemangleStyle<'a>`

- <span id="demanglestyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="demanglestyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `demangle`

```rust
fn demangle(s: &str) -> Demangle<'_>
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:92-146`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L92-L146)*

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

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:237-244`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L237-L244)*

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

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:253-259`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L253-L259)*

### `is_ascii_alphanumeric`

```rust
fn is_ascii_alphanumeric(c: char) -> bool
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:262-267`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L262-L267)*

### `is_ascii_punctuation`

```rust
fn is_ascii_punctuation(c: char) -> bool
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:270-278`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L270-L278)*

## Constants

### `MAX_SIZE`
```rust
const MAX_SIZE: usize = 1_000_000usize;
```

*Defined in [`rustc-demangle-0.1.26/src/lib.rs:290`](../../.source_1765521767/rustc-demangle-0.1.26/src/lib.rs#L290)*

