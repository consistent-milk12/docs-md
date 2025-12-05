*[rustls](../../index.md) / [crypto](../index.md) / [hpke](index.md)*

---

# Module `hpke`

Hybrid public key encryption (RFC 9180).

## Structs

### `HpkeSuite`

```rust
struct HpkeSuite {
    pub kem: crate::msgs::enums::HpkeKem,
    pub sym: crate::msgs::handshake::HpkeSymmetricCipherSuite,
}
```

An HPKE suite, specifying a key encapsulation mechanism and a symmetric cipher suite.

#### Fields

- **`kem`**: `crate::msgs::enums::HpkeKem`

  The choice of HPKE key encapsulation mechanism.

- **`sym`**: `crate::msgs::handshake::HpkeSymmetricCipherSuite`

  The choice of HPKE symmetric cipher suite.
  
  This combines a choice of authenticated encryption with additional data (AEAD) algorithm
  and a key derivation function (KDF).

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

- `fn clone(self: &Self) -> HpkeSuite`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HpkeSuite) -> bool`

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

### `HpkePublicKey`

```rust
struct HpkePublicKey(alloc::vec::Vec<u8>);
```

An HPKE public key.

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

- `fn clone(self: &Self) -> HpkePublicKey`

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

### `HpkePrivateKey`

```rust
struct HpkePrivateKey();
```

An HPKE private key.

#### Implementations

- `fn secret_bytes(self: &Self) -> &[u8]`
  Return the private key bytes.

#### Trait Implementations

##### `impl From`

- `fn from(bytes: Vec<u8>) -> Self`

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

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `HpkeKeyPair`

```rust
struct HpkeKeyPair {
    pub public_key: HpkePublicKey,
    pub private_key: HpkePrivateKey,
}
```

An HPKE key pair, made of a matching public and private key.

#### Fields

- **`public_key`**: `HpkePublicKey`

  A HPKE public key.

- **`private_key`**: `HpkePrivateKey`

  A HPKE private key.

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

### `EncapsulatedSecret`

```rust
struct EncapsulatedSecret(alloc::vec::Vec<u8>);
```

An encapsulated secret returned from setting up a sender or receiver context.

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

### `Hpke`

```rust
trait Hpke: Debug + Send + Sync { ... }
```

An HPKE instance that can be used for base-mode single-shot encryption and decryption.

#### Required Methods

- `fn seal(self: &Self, info: &[u8], aad: &[u8], plaintext: &[u8], pub_key: &HpkePublicKey) -> Result<(EncapsulatedSecret, Vec<u8>), Error>`

  Seal the provided `plaintext` to the recipient public key `pub_key` with application supplied

- `fn setup_sealer(self: &Self, info: &[u8], pub_key: &HpkePublicKey) -> Result<(EncapsulatedSecret, Box<dyn HpkeSealer>), Error>`

  Set up a sealer context for the receiver public key `pub_key` with application supplied `info`.

- `fn open(self: &Self, enc: &EncapsulatedSecret, info: &[u8], aad: &[u8], ciphertext: &[u8], secret_key: &HpkePrivateKey) -> Result<Vec<u8>, Error>`

  Open the provided `ciphertext` using the encapsulated secret `enc`, with application

- `fn setup_opener(self: &Self, enc: &EncapsulatedSecret, info: &[u8], secret_key: &HpkePrivateKey) -> Result<Box<dyn HpkeOpener>, Error>`

  Set up an opener context for the secret key `secret_key` with application supplied `info`.

- `fn generate_key_pair(self: &Self) -> Result<(HpkePublicKey, HpkePrivateKey), Error>`

  Generate a new public key and private key pair compatible with this HPKE instance.

- `fn fips(self: &Self) -> bool`

  Return whether the HPKE instance is FIPS compatible.

- `fn suite(self: &Self) -> HpkeSuite`

  Return the [HpkeSuite] that this HPKE instance supports.

### `HpkeSealer`

```rust
trait HpkeSealer: Debug + Send + Sync + 'static { ... }
```

An HPKE sealer context.

This is a stateful object that can be used to seal messages for receipt by
a receiver.

#### Required Methods

- `fn seal(self: &mut Self, aad: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, Error>`

  Seal the provided `plaintext` with additional data `aad`, returning

### `HpkeOpener`

```rust
trait HpkeOpener: Debug + Send + Sync + 'static { ... }
```

An HPKE opener context.

This is a stateful object that can be used to open sealed messages sealed
by a sender.

#### Required Methods

- `fn open(self: &mut Self, aad: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Error>`

  Open the provided `ciphertext` with additional data `aad`, returning plaintext.

