*[ring](../index.md) / [hkdf](index.md)*

---

# Module `hkdf`

HMAC-based Extract-and-Expand Key Derivation Function.

HKDF is specified in [RFC 5869].

[RFC 5869]: https://tools.ietf.org/html/rfc5869

## Structs

### `Algorithm`

```rust
struct Algorithm();
```

An HKDF algorithm.

#### Implementations

- `fn hmac_algorithm(self: &Self) -> hmac::Algorithm`
  The underlying HMAC algorithm.

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

##### `impl Clone`

- `fn clone(self: &Self) -> Algorithm`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl KeyType`

- `fn len(self: &Self) -> usize`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Algorithm) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Salt`

```rust
struct Salt();
```

A salt for HKDF operations.

#### Implementations

- `fn new(algorithm: Algorithm, value: &[u8]) -> Self`
  Constructs a new `Salt` with the given value based on the given digest

- `fn extract(self: &Self, secret: &[u8]) -> Prk`
  The [HKDF-Extract] operation.

- `fn algorithm(self: &Self) -> Algorithm`
  The algorithm used to derive this salt.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(okm: Okm<'_, Algorithm>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Prk`

```rust
struct Prk();
```

A HKDF PRK (pseudorandom key).

#### Implementations

- `fn new_less_safe(algorithm: Algorithm, value: &[u8]) -> Self`
  Construct a new `Prk` directly with the given value.

- `fn expand<'a, L: KeyType>(self: &'a Self, info: &'a [&'a [u8]], len: L) -> Result<Okm<'a, L>, error::Unspecified>`
  The [HKDF-Expand] operation.

#### Trait Implementations

##### `impl From`

- `fn from(okm: Okm<'_, Algorithm>) -> Self`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Prk`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Okm<'a, L: KeyType>`

```rust
struct Okm<'a, L: KeyType> {
    // [REDACTED: Private Fields]
}
```

An HKDF OKM (Output Keying Material)

Intentionally not `Clone` or `Copy` as an OKM is generally only safe to
use once.

#### Implementations

- `fn len(self: &Self) -> &L`
  The `OkmLength` given to `Prk::expand()`.

- `fn fill(self: Self, out: &mut [u8]) -> Result<(), error::Unspecified>`
  Fills `out` with the output of the HKDF-Expand operation for the given

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, L: $crate::fmt::Debug + KeyType>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `KeyType`

```rust
trait KeyType { ... }
```

The length of the OKM (Output Keying Material) for a `Prk::expand()` call.

#### Required Methods

- `fn len(self: &Self) -> usize`

  The length that `Prk::expand()` should expand its input to.

