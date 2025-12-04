*[ring](../index.md) / [aead](index.md)*

---

# Module `aead`

Authenticated Encryption with Associated Data (AEAD).

See [Authenticated encryption: relations among notions and analysis of the
generic composition paradigm][AEAD] for an introduction to the concept of
AEADs.

[AEAD]: https://eprint.iacr.org/2000/025.pdf


## Modules

- [`chacha20_poly1305_openssh`](chacha20_poly1305_openssh/index.md) - The [chacha20-poly1305@openssh.com] AEAD-ish construct.
- [`quic`](quic/index.md) - QUIC Header Protection.

## Structs

### `Algorithm`

```rust
struct Algorithm {
    // [REDACTED: Private Fields]
}
```

An AEAD Algorithm.

#### Implementations

- `fn key_len(self: &Self) -> usize`
  The length of the key.

- `fn tag_len(self: &Self) -> usize`
  The length of a tag.

- `fn nonce_len(self: &Self) -> usize`
  The length of the nonces.

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `LessSafeKey`

```rust
struct LessSafeKey {
    // [REDACTED: Private Fields]
}
```

Immutable keys for use in situations where `OpeningKey`/`SealingKey` and
`NonceSequence` cannot reasonably be used.

Prefer to use `OpeningKey`/`SealingKey` and `NonceSequence` when practical.

#### Implementations

- `fn new(key: UnboundKey) -> Self`
  Constructs a `LessSafeKey`.

- `fn open_in_place_separate_tag<'in_out, A>(self: &Self, nonce: Nonce, aad: Aad<A>, tag: Tag, in_out: &'in_out mut [u8], ciphertext: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>`
  Like [open_in_place](Self::open_in_place), except the authentication tag is

- `fn open_in_place<'in_out, A>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out mut [u8]) -> Result<&'in_out mut [u8], error::Unspecified>`
  Like [`super::OpeningKey::open_in_place()`], except it accepts an

- `fn open_within<'in_out, A>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>`
  Like [`super::OpeningKey::open_within()`], except it accepts an

- `fn seal_in_place_append_tag<A, InOut>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: &mut InOut) -> Result<(), error::Unspecified>`
  Like [`super::SealingKey::seal_in_place_append_tag()`], except it

- `fn seal_in_place_separate_tag<A>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: &mut [u8]) -> Result<Tag, error::Unspecified>`
  Like `super::SealingKey::seal_in_place_separate_tag()`, except it

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The key's AEAD algorithm.

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

- `fn clone(self: &Self) -> LessSafeKey`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `Nonce`

```rust
struct Nonce();
```

A nonce for a single AEAD opening or sealing operation.

The user must ensure, for a particular key, that each nonce is unique.

`Nonce` intentionally doesn't implement `Clone` to ensure that each one is
consumed at most once.

#### Implementations

- `fn try_assume_unique_for_key(value: &[u8]) -> Result<Self, error::Unspecified>`
  Constructs a `Nonce` with the given value, assuming that the value is

- `fn assume_unique_for_key(value: [u8; 12]) -> Self`
  Constructs a `Nonce` with the given value, assuming that the value is

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8; 12]`

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

### `OpeningKey<N: NonceSequence>`

```rust
struct OpeningKey<N: NonceSequence> {
    // [REDACTED: Private Fields]
}
```

An AEAD key for authenticating and decrypting ("opening"), bound to a nonce
sequence.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the nonce sequence.

#### Implementations

- `fn open_in_place<'in_out, A>(self: &mut Self, aad: Aad<A>, in_out: &'in_out mut [u8]) -> Result<&'in_out mut [u8], error::Unspecified>`
  Authenticates and decrypts (“opens”) data in place.

- `fn open_within<'in_out, A>(self: &mut Self, aad: Aad<A>, in_out: &'in_out mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out mut [u8], error::Unspecified>`
  Authenticates and decrypts (“opens”) data in place, with a shift.

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

##### `impl BoundKey<N: NonceSequence>`

- `fn new(key: UnboundKey, nonce_sequence: N) -> Self`

- `fn algorithm(self: &Self) -> &'static Algorithm`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<N: NonceSequence>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `SealingKey<N: NonceSequence>`

```rust
struct SealingKey<N: NonceSequence> {
    // [REDACTED: Private Fields]
}
```

An AEAD key for encrypting and signing ("sealing"), bound to a nonce
sequence.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the nonce sequence.

#### Implementations

- `fn seal_in_place_append_tag<A, InOut>(self: &mut Self, aad: Aad<A>, in_out: &mut InOut) -> Result<(), error::Unspecified>`
  Encrypts and signs (“seals”) data in place, appending the tag to the

- `fn seal_in_place_separate_tag<A>(self: &mut Self, aad: Aad<A>, in_out: &mut [u8]) -> Result<Tag, error::Unspecified>`
  Encrypts and signs (“seals”) data in place.

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

##### `impl BoundKey<N: NonceSequence>`

- `fn new(key: UnboundKey, nonce_sequence: N) -> Self`

- `fn algorithm(self: &Self) -> &'static Algorithm`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<N: NonceSequence>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `UnboundKey`

```rust
struct UnboundKey {
    // [REDACTED: Private Fields]
}
```

An AEAD key without a designated role or nonce sequence.

#### Implementations

- `fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>`
  Constructs a `UnboundKey`.

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The key's AEAD algorithm.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(okm: hkdf::Okm<'_, &'static Algorithm>) -> Self`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `Aad<A>`

```rust
struct Aad<A>();
```

The additionally authenticated data (AAD) for an opening or sealing
operation. This data is authenticated but is **not** encrypted.

The type `A` could be a byte slice `&[u8](#u8)
`, a byte array `[u8; N]`
for some constant `N`, `Vec<u8>`, etc.

#### Implementations

- `fn empty() -> Self`
  Construct an empty `Aad`.

- `fn from(aad: A) -> Self`
  Construct the `Aad` from the given bytes.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef<A>`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<A: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Aad<A>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<A: $crate::marker::Copy>`

##### `impl Eq<A>`

##### `impl PartialEq<A>`

- `fn eq(self: &Self, other: &Self) -> bool`

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

##### `impl Debug<A>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Tag`

```rust
struct Tag();
```

A possibly valid authentication tag.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: [u8; 16]) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Tag`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = Unspecified`

- `fn try_from(value: &[u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `NonceSequence`

```rust
trait NonceSequence { ... }
```

A sequences of unique nonces.

A given `NonceSequence` must never return the same `Nonce` twice from
`advance()`.

A simple counter is a reasonable (but probably not ideal) `NonceSequence`.

Intentionally not `Clone` or `Copy` since cloning would allow duplication
of the sequence.

#### Required Methods

- `fn advance(self: &mut Self) -> Result<Nonce, error::Unspecified>`

  Returns the next nonce in the sequence.

### `BoundKey<N: NonceSequence>`

```rust
trait BoundKey<N: NonceSequence>: core::fmt::Debug { ... }
```

An AEAD key bound to a nonce sequence.

#### Required Methods

- `fn new(key: UnboundKey, nonce_sequence: N) -> Self`

  Constructs a new key from the given `UnboundKey` and `NonceSequence`.

- `fn algorithm(self: &Self) -> &'static Algorithm`

  The key's AEAD algorithm.

## Constants

### `MAX_TAG_LEN`

```rust
const MAX_TAG_LEN: usize = 16usize;
```

The maximum length of a tag for the algorithms in this module.

