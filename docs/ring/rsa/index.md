*[ring](../index.md) / [rsa](index.md)*

---

# Module `rsa`

RSA.

## Structs

### `RsaParameters`

```rust
struct RsaParameters {
    // [REDACTED: Private Fields]
}
```

Parameters for RSA verification.

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

##### `impl VerificationAlgorithm`

- `fn verify(self: &Self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `KeyPair`

```rust
struct KeyPair {
    // [REDACTED: Private Fields]
}
```

An RSA key pair, used for signing.

#### Implementations

- `fn sign(self: &Self, padding_alg: &'static dyn RsaEncoding, rng: &dyn rand::SecureRandom, msg: &[u8], signature: &mut [u8]) -> Result<(), error::Unspecified>`
  Computes the signature of `msg` and writes it into `signature`.

- `fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, KeyRejected>`
  Parses an unencrypted PKCS#8-encoded RSA private key.

- `fn from_der(input: &[u8]) -> Result<Self, KeyRejected>`
  Parses an RSA private key that is not inside a PKCS#8 wrapper.

- `fn from_components<Public, Private>(components: &KeyPairComponents<Public, Private>) -> Result<Self, KeyRejected>`
  Constructs an RSA private key from its big-endian-encoded components.

- `fn public(self: &Self) -> &PublicKey`
  Returns a reference to the public key.

- `fn public_modulus_len(self: &Self) -> usize`
  Returns the length in bytes of the key pair's public modulus.

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

##### `impl KeyPair`

- `type PublicKey = PublicKey`

- `fn public_key(self: &Self) -> &<Self as >::PublicKey`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `KeyPairComponents<Public, Private>`

```rust
struct KeyPairComponents<Public, Private> {
    pub public_key: super::PublicKeyComponents<Public>,
    pub d: Private,
    pub p: Private,
    pub q: Private,
    pub dP: Private,
    pub dQ: Private,
    pub qInv: Private,
}
```

RSA key pair components.

#### Fields

- **`public_key`**: `super::PublicKeyComponents<Public>`

  The public key components.

- **`d`**: `Private`

  The private exponent.

- **`p`**: `Private`

  The first prime factor of `d`.

- **`q`**: `Private`

  The second prime factor of `d`.

- **`dP`**: `Private`

  `p`'s public Chinese Remainder Theorem exponent.

- **`dQ`**: `Private`

  `q`'s public Chinese Remainder Theorem exponent.

- **`qInv`**: `Private`

  `q**-1 mod p`.

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

##### `impl Clone<Public: $crate::clone::Clone, Private: $crate::clone::Clone>`

- `fn clone(self: &Self) -> KeyPairComponents<Public, Private>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<Public: $crate::marker::Copy, Private: $crate::marker::Copy>`

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

##### `impl Debug<Public, Private>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `PublicKey`

```rust
struct PublicKey {
    // [REDACTED: Private Fields]
}
```

An RSA Public Key.

#### Implementations

- `fn modulus_len(self: &Self) -> usize`
  The length, in bytes, of the public modulus.

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

- `fn clone(self: &Self) -> PublicKey`

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

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `PublicKeyComponents<B>`

```rust
struct PublicKeyComponents<B> {
    pub n: B,
    pub e: B,
}
```

RSA public key components.

`B` must implement `AsRef<[u8](#u8)
>` like `&[u8](#u8)
` or `Vec<u8>`.

#### Fields

- **`n`**: `B`

  The public modulus, encoded in big-endian bytes without leading zeros.

- **`e`**: `B`

  The public exponent, encoded in big-endian bytes without leading zeros.

#### Implementations

- `fn verify(self: &Self, params: &RsaParameters, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>`
  Verifies that `signature` is a valid signature of `message` using `self`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<B>`

- `fn from(public_key: &PublicKey) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<B: $crate::clone::Clone>`

- `fn clone(self: &Self) -> PublicKeyComponents<B>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<B: $crate::marker::Copy>`

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

##### `impl Debug<B>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

