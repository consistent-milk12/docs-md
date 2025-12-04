*[ring](../../index.md) / [aead](../index.md) / [quic](index.md)*

---

# Module `quic`

QUIC Header Protection.

See draft-ietf-quic-tls.

## Structs

### `HeaderProtectionKey`

```rust
struct HeaderProtectionKey {
    // [REDACTED: Private Fields]
}
```

A key for generating QUIC Header Protection masks.

#### Implementations

- `fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>`
  Create a new header protection key.

- `fn new_mask(self: &Self, sample: &[u8]) -> Result<[u8; 5], error::Unspecified>`
  Generate a new QUIC Header Protection mask.

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The key's algorithm.

#### Trait Implementations

##### `impl From`

- `fn from(okm: hkdf::Okm<'_, &'static Algorithm>) -> Self`

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

### `Algorithm`

```rust
struct Algorithm {
    // [REDACTED: Private Fields]
}
```

A QUIC Header Protection Algorithm.

#### Implementations

- `fn key_len(self: &Self) -> usize`
  The length of the key.

- `fn sample_len(self: &Self) -> usize`
  The required sample length.

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

## Type Aliases

### `Sample`

```rust
type Sample = [u8; 16];
```

QUIC sample for new key masks

