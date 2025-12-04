*[rustls](../../index.md) / [crypto](../index.md) / [tls13](index.md)*

---

# Module `tls13`

Cryptography specific to TLS1.3.

## Structs

### `HkdfExpanderUsingHmac`

```rust
struct HkdfExpanderUsingHmac();
```

Implementation of `HkdfExpander` via `hmac::Key`.

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

##### `impl HkdfExpander`

- `fn expand_slice(self: &Self, info: &[&[u8]], output: &mut [u8]) -> Result<(), OutputLengthError>`

- `fn expand_block(self: &Self, info: &[&[u8]]) -> OkmBlock`

- `fn hash_len(self: &Self) -> usize`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `HkdfUsingHmac<'a>`

```rust
struct HkdfUsingHmac<'a>(&'a dyn hmac::Hmac);
```

Implementation of `Hkdf` (and thence `HkdfExpander`) via `hmac::Hmac`.

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

##### `impl Hkdf`

- `fn extract_from_zero_ikm(self: &Self, salt: Option<&[u8]>) -> Box<dyn HkdfExpander>`

- `fn extract_from_secret(self: &Self, salt: Option<&[u8]>, secret: &[u8]) -> Box<dyn HkdfExpander>`

- `fn expander_for_okm(self: &Self, okm: &OkmBlock) -> Box<dyn HkdfExpander>`

- `fn hmac_sign(self: &Self, key: &OkmBlock, message: &[u8]) -> hmac::Tag`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `OkmBlock`

```rust
struct OkmBlock {
}
```

Output key material from HKDF, as a value type.

#### Implementations

- `fn new(bytes: &[u8]) -> Self`
  Build a single OKM block by copying a byte slice.

- `const MAX_LEN: usize`

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

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OkmBlock`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop`

- `fn drop(self: &mut Self)`

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

### `OutputLengthError`

```rust
struct OutputLengthError;
```

An error type used for `HkdfExpander::expand_slice` when
the slice exceeds the maximum HKDF output length.

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `HkdfExpander`

```rust
trait HkdfExpander: Send + Sync { ... }
```

Implementation of `HKDF-Expand` with an implicitly stored and immutable `PRK`.

#### Required Methods

- `fn expand_slice(self: &Self, info: &[&[u8]], output: &mut [u8]) -> Result<(), OutputLengthError>`

  `HKDF-Expand(PRK, info, L)` into a slice.

- `fn expand_block(self: &Self, info: &[&[u8]]) -> OkmBlock`

  `HKDF-Expand(PRK, info, L=HashLen)` returned as a value.

- `fn hash_len(self: &Self) -> usize`

  Return what `HashLen` is for this instance.

### `Hkdf`

```rust
trait Hkdf: Send + Sync { ... }
```

A HKDF implementation oriented to the needs of TLS1.3.

See [RFC5869](https://datatracker.ietf.org/doc/html/rfc5869) for the terminology
used in this definition.

You can use [`HkdfUsingHmac`](rustls/crypto/tls13/index.md) which implements this trait on top of an implementation
of [`hmac::Hmac`](#hmac).

#### Required Methods

- `fn extract_from_zero_ikm(self: &Self, salt: Option<&[u8]>) -> Box<dyn HkdfExpander>`

  `HKDF-Extract(salt, 0_HashLen)`

- `fn extract_from_secret(self: &Self, salt: Option<&[u8]>, secret: &[u8]) -> Box<dyn HkdfExpander>`

  `HKDF-Extract(salt, secret)`

- `fn extract_from_kx_shared_secret(self: &Self, salt: Option<&[u8]>, kx: Box<dyn ActiveKeyExchange>, peer_pub_key: &[u8]) -> Result<Box<dyn HkdfExpander>, Error>`

  `HKDF-Extract(salt, shared_secret)` where `shared_secret` is the result of a key exchange.

- `fn expander_for_okm(self: &Self, okm: &OkmBlock) -> Box<dyn HkdfExpander>`

  Build a `HkdfExpander` using `okm` as the secret PRK.

- `fn hmac_sign(self: &Self, key: &OkmBlock, message: &[u8]) -> hmac::Tag`

  Signs `message` using `key` viewed as a HMAC key.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

## Functions

### `expand`

```rust
fn expand<T, const N: usize>(expander: &dyn HkdfExpander, info: &[&[u8]]) -> T
where
    T: From<[u8; N]>
```

`HKDF-Expand(PRK, info, L)` to construct any type from a byte array.

- `PRK` is the implicit key material represented by this instance.
- `L := N`; N is the size of the byte array.
- `info` is a slice of byte slices, which should be processed sequentially
  (or concatenated if that is not possible).

This is infallible, because the set of types (and therefore their length) is known
at compile time.

