*[ring](../index.md) / [agreement](index.md)*

---

# Module `agreement`

Key Agreement: ECDH, including X25519.

# Example

Note that this example uses X25519, but ECDH using NIST P-256/P-384 is done
exactly the same way, just substituting
`agreement::ECDH_P256`/`agreement::ECDH_P384` for `agreement::X25519`.

```
use ring::{agreement, rand};

let rng = rand::SystemRandom::new();

let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;

// Make `my_public_key` a byte slice containing my public key. In a real
// application, this would be sent to the peer in an encoded protocol
// message.
let my_public_key = my_private_key.compute_public_key()?;

let peer_public_key_bytes = {
    // In a real application, the peer public key would be parsed out of a
    // protocol message. Here we just generate one.
    let peer_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    peer_private_key.compute_public_key()?
};

let peer_public_key = agreement::UnparsedPublicKey::new(
    &agreement::X25519,
    peer_public_key_bytes);

agreement::agree_ephemeral(
    my_private_key,
    &peer_public_key,
    |_key_material| {
        // In a real application, we'd apply a KDF to the key material and the
        // public keys (as recommended in RFC 7748) and then derive session
        // keys from the result. We omit all that here.
    },
)?;

# Ok::<(), ring::error::Unspecified>(())
```

## Structs

### `Algorithm`

```rust
struct Algorithm {
}
```

A key agreement algorithm.

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

### `EphemeralPrivateKey`

```rust
struct EphemeralPrivateKey {
}
```

An ephemeral private key for use (only) with `agree_ephemeral`. The
signature of `agree_ephemeral` ensures that an `EphemeralPrivateKey` can be
used for at most one key agreement.

#### Implementations

- `fn generate(alg: &'static Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified>`
  Generate a new ephemeral private key for the given algorithm.

- `fn compute_public_key(self: &Self) -> Result<PublicKey, error::Unspecified>`
  Computes the public key from the private key.

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The algorithm for the private key.

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

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `PublicKey`

```rust
struct PublicKey {
}
```

A public key for key agreement.

#### Implementations

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The algorithm for the public key.

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `UnparsedPublicKey<B>`

```rust
struct UnparsedPublicKey<B> {
}
```

An unparsed, possibly malformed, public key for key agreement.

#### Implementations

- `fn new(algorithm: &'static Algorithm, bytes: B) -> Self`
  Constructs a new `UnparsedPublicKey`.

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The algorithm for the public key.

- `fn bytes(self: &Self) -> &B`
  TODO: doc

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef<B>`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<B: $crate::clone::Clone>`

- `fn clone(self: &Self) -> UnparsedPublicKey<B>`

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

## Functions

### `agree_ephemeral`

```rust
fn agree_ephemeral<B: AsRef<[u8]>, R>(my_private_key: EphemeralPrivateKey, peer_public_key: &UnparsedPublicKey<B>, kdf: impl FnOnce(&[u8]) -> R) -> Result<R, error::Unspecified>
```

Performs a key agreement with an ephemeral private key and the given public
key.

`my_private_key` is the ephemeral private key to use. Since it is moved, it
will not be usable after calling `agree_ephemeral`, thus guaranteeing that
the key is used for only one key agreement.

`peer_public_key` is the peer's public key. `agree_ephemeral` will return
`Err(error_value)` if it does not match `my_private_key's` algorithm/curve.
`agree_ephemeral` verifies that it is encoded in the standard form for the
algorithm and that the key is *valid*; see the algorithm's documentation for
details on how keys are to be encoded and what constitutes a valid key for
that algorithm.

After the key agreement is done, `agree_ephemeral` calls `kdf` with the raw
key material from the key agreement operation and then returns what `kdf`
returns.

