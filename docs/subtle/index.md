# Crate `subtle`

# subtle [![](https://img.shields.io/crates/v/subtle.svg)](https://crates.io/crates/subtle) [![](https://img.shields.io/badge/dynamic/json.svg?label=docs&uri=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fsubtle%2Fversions&query=%24.versions%5B0%5D.num&colorB=4F74A6)](https://doc.dalek.rs/subtle) [![](https://travis-ci.org/dalek-cryptography/subtle.svg?branch=master)](https://travis-ci.org/dalek-cryptography/subtle)

**Pure-Rust traits and utilities for constant-time cryptographic implementations.**

It consists of a `Choice` type, and a collection of traits using `Choice`
instead of `bool` which are intended to execute in constant-time.  The `Choice`
type is a wrapper around a `u8` that holds a `0` or `1`.

```toml
subtle = "2.6"
```

This crate represents a “best-effort” attempt, since side-channels
are ultimately a property of a deployed cryptographic system
including the hardware it runs on, not just of software.

The traits are implemented using bitwise operations, and should execute in
constant time provided that a) the bitwise operations are constant-time and
b) the bitwise operations are not recognized as a conditional assignment and
optimized back into a branch.

For a compiler to recognize that bitwise operations represent a conditional
assignment, it needs to know that the value used to generate the bitmasks is
really a boolean `i1` rather than an `i8` byte value. In an attempt to
prevent this refinement, the crate tries to hide the value of a `Choice`'s
inner `u8` by passing it through a volatile read. For more information, see
the _About_ section below.

Rust versions from 1.51 or higher have const generics support. You may enable
`const-generics` feautre to have `subtle` traits implemented for arrays `[T; N]`.

Versions prior to `2.2` recommended use of the `nightly` feature to enable an
optimization barrier; this is not required in versions `2.2` and above.

Note: the `subtle` crate contains `debug_assert`s to check invariants during
debug builds. These invariant checks involve secret-dependent branches, and
are not present when compiled in release mode. This crate is intended to be
used in release mode.

## Documentation

Documentation is available [here][docs](#docs)
.

## Minimum Supported Rust Version

Rust **1.41** or higher.

Minimum supported Rust version can be changed in the future, but it will be done with a minor version bump.

## About

This library aims to be the Rust equivalent of Go’s `crypto/subtle` module.

Old versions of the optimization barrier in `impl From<u8> for Choice` were
based on Tim Maclean's [work on `rust-timing-shield`][rust-timing-shield],
which attempts to provide a more comprehensive approach for preventing
software side-channels in Rust code.
From version `2.2`, it was based on Diane Hosfelt and Amber Sprenkels' work on
"Secret Types in Rust".

`subtle` is authored by isis agora lovecruft and Henry de Valence.

## Warning

This code is a low-level library, intended for specific use-cases implementing
cryptographic protocols.  It represents a best-effort attempt to protect
against some software side-channels.  Because side-channel resistance is not a
property of software alone, but of software together with hardware, any such
effort is fundamentally limited.

**USE AT YOUR OWN RISK**

[docs](#docs)
: https://docs.rs/subtle
[rust-timing-shield]: https://www.chosenplaintext.ca/open-source/rust-timing-shield/security

## Structs

### `Choice`

```rust
struct Choice();
```

The `Choice` struct represents a choice for use in conditional assignment.

It is a wrapper around a `u8`, which should have the value either `1` (true)
or `0` (false).

The conversion from `u8` to `Choice` passes the value through an optimization
barrier, as a best-effort attempt to prevent the compiler from inferring that
the `Choice` value is a boolean. This strategy is based on Tim Maclean's
[work on `rust-timing-shield`][rust-timing-shield], which attempts to provide
a more comprehensive approach for preventing software side-channels in Rust
code.

The `Choice` struct implements operators for AND, OR, XOR, and NOT, to allow
combining `Choice` values. These operations do not short-circuit.

[rust-timing-shield]:
https://www.chosenplaintext.ca/open-source/rust-timing-shield/security

#### Implementations

- `fn unwrap_u8(self: &Self) -> u8`
  Unwrap the `Choice` wrapper to reveal the underlying `u8`.

#### Trait Implementations

##### `impl From`

- `fn from(input: u8) -> Choice`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl BitAnd`

- `type Output = Choice`

- `fn bitand(self: Self, rhs: Choice) -> Choice`

##### `impl BitAndAssign`

- `fn bitand_assign(self: &mut Self, rhs: Choice)`

##### `impl BitOr`

- `type Output = Choice`

- `fn bitor(self: Self, rhs: Choice) -> Choice`

##### `impl BitOrAssign`

- `fn bitor_assign(self: &mut Self, rhs: Choice)`

##### `impl BitXor`

- `type Output = Choice`

- `fn bitxor(self: Self, rhs: Choice) -> Choice`

##### `impl BitXorAssign`

- `fn bitxor_assign(self: &mut Self, rhs: Choice)`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Choice`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ConditionallySelectable`

- `fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self`

##### `impl ConstantTimeEq`

- `fn ct_eq(self: &Self, rhs: &Choice) -> Choice`

##### `impl Copy`

##### `impl Not`

- `type Output = Choice`

- `fn not(self: Self) -> Choice`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CtOption<T>`

```rust
struct CtOption<T> {
    // [REDACTED: Private Fields]
}
```

The `CtOption<T>` type represents an optional value similar to the
[`Option<T>`](core::option::Option) type but is intended for
use in constant time APIs.

Any given `CtOption<T>` is either `Some` or `None`, but unlike
`Option<T>` these variants are not exposed. The
[`is_some()`](CtOption::is_some) method is used to determine if
the value is `Some`, and [`unwrap_or()`](CtOption::unwrap_or) and
[`unwrap_or_else()`](CtOption::unwrap_or_else) methods are
provided to access the underlying value. The value can also be
obtained with [`unwrap()`](CtOption::unwrap) but this will panic
if it is `None`.

Functions that are intended to be constant time may not produce
valid results for all inputs, such as square root and inversion
operations in finite field arithmetic. Returning an `Option<T>`
from these functions makes it difficult for the caller to reason
about the result in constant time, and returning an incorrect
value burdens the caller and increases the chance of bugs.

#### Implementations

- `fn new(value: T, is_some: Choice) -> CtOption<T>`
  This method is used to construct a new `CtOption<T>` and takes

- `fn expect(self: Self, msg: &str) -> T`
  Returns the contained value, consuming the `self` value.

- `fn unwrap(self: Self) -> T`
  This returns the underlying value but panics if it

- `fn unwrap_or(self: Self, def: T) -> T`
  This returns the underlying value if it is `Some`

- `fn unwrap_or_else<F>(self: Self, f: F) -> T`
  This returns the underlying value if it is `Some`

- `fn is_some(self: &Self) -> Choice`
  Returns a true `Choice` if this value is `Some`.

- `fn is_none(self: &Self) -> Choice`
  Returns a true `Choice` if this value is `None`.

- `fn map<U, F>(self: Self, f: F) -> CtOption<U>`
  Returns a `None` value if the option is `None`, otherwise

- `fn and_then<U, F>(self: Self, f: F) -> CtOption<U>`
  Returns a `None` value if the option is `None`, otherwise

- `fn or_else<F>(self: Self, f: F) -> CtOption<T>`
  Returns `self` if it contains a value, and otherwise returns the result of

- `fn into_option(self: Self) -> Option<T>`
  Convert the `CtOption<T>` wrapper into an `Option<T>`, depending on whether

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> CtOption<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ConditionallySelectable<T: ConditionallySelectable>`

- `fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self`

##### `impl ConstantTimeEq<T: ConstantTimeEq>`

- `fn ct_eq(self: &Self, rhs: &CtOption<T>) -> Choice`
  Two `CtOption<T>`s are equal if they are both `Some` and

##### `impl Copy<T: $crate::marker::Copy>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BlackBox<T: Copy>`

```rust
struct BlackBox<T: Copy>();
```

Wrapper type which implements an optimization barrier for all accesses.

#### Implementations

- `fn new(value: T) -> Self`
  Constructs a new instance of `BlackBox` which will wrap the specified value.

- `fn get(self: Self) -> T`
  Read the inner value, applying an optimization barrier on access.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone + Copy>`

- `fn clone(self: &Self) -> BlackBox<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<T: $crate::marker::Copy + Copy>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug + Copy>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `ConstantTimeEq`

```rust
trait ConstantTimeEq { ... }
```

An `Eq`-like trait that produces a `Choice` instead of a `bool`.

# Example

```
use subtle::ConstantTimeEq;
let x: u8 = 5;
let y: u8 = 13;

assert_eq!(x.ct_eq(&y).unwrap_u8(), 0);
assert_eq!(x.ct_eq(&x).unwrap_u8(), 1);
```

#### Required Methods

- `fn ct_eq(self: &Self, other: &Self) -> Choice`

  Determine if two items are equal.

- `fn ct_ne(self: &Self, other: &Self) -> Choice`

  Determine if two items are NOT equal.

### `ConditionallySelectable`

```rust
trait ConditionallySelectable: Copy { ... }
```

A type which can be conditionally selected in constant time.

This trait also provides generic implementations of conditional
assignment and conditional swaps.

#### Required Methods

- `fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self`

  Select `a` or `b` according to `choice`.

- `fn conditional_assign(self: &mut Self, other: &Self, choice: Choice)`

  Conditionally assign `other` to `self`, according to `choice`.

- `fn conditional_swap(a: &mut Self, b: &mut Self, choice: Choice)`

  Conditionally swap `self` and `other` if `choice == 1`; otherwise,

### `ConditionallyNegatable`

```rust
trait ConditionallyNegatable { ... }
```

A type which can be conditionally negated in constant time.

# Note

A generic implementation of `ConditionallyNegatable` is provided
for types `T` which are `ConditionallySelectable` and have `Neg`
implemented on `&T`.

#### Required Methods

- `fn conditional_negate(self: &mut Self, choice: Choice)`

  Negate `self` if `choice == Choice(1)`; otherwise, leave it

### `ConstantTimeGreater`

```rust
trait ConstantTimeGreater { ... }
```

A type which can be compared in some manner and be determined to be greater
than another of the same type.

#### Required Methods

- `fn ct_gt(self: &Self, other: &Self) -> Choice`

  Determine whether `self > other`.

### `ConstantTimeLess`

```rust
trait ConstantTimeLess: ConstantTimeEq + ConstantTimeGreater { ... }
```

A type which can be compared in some manner and be determined to be less
than another of the same type.

#### Required Methods

- `fn ct_lt(self: &Self, other: &Self) -> Choice`

  Determine whether `self < other`.

