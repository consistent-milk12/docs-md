*[ring](../index.md) / [hmac](index.md)*

---

# Module `hmac`

HMAC is specified in [RFC 2104].

After a `Key` is constructed, it can be used for multiple signing or
verification operations. Separating the construction of the key from the
rest of the HMAC operation allows the per-key precomputation to be done
only once, instead of it being done in every HMAC operation.

Frequently all the data to be signed in a message is available in a single
contiguous piece. In that case, the module-level `sign` function can be
used. Otherwise, if the input is in multiple parts, `Context` should be
used.

# Examples:

## Signing a value and verifying it wasn't tampered with

```
use ring::{hmac, rand};

let rng = rand::SystemRandom::new();
let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;

let msg = "hello, world";

let tag = hmac::sign(&key, msg.as_bytes());

// [We give access to the message to an untrusted party, and they give it
// back to us. We need to verify they didn't tamper with it.]

hmac::verify(&key, msg.as_bytes(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

## Using the one-shot API:

```
use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

let msg = "hello, world";

// The sender generates a secure key value and signs the message with it.
// Note that in a real protocol, a key agreement protocol would be used to
// derive `key_value`.
let rng = rand::SystemRandom::new();
let key_value: [u8; digest::SHA256_OUTPUT_LEN] = rand::generate(&rng)?.expose();

let s_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
let tag = hmac::sign(&s_key, msg.as_bytes());

// The receiver (somehow!) knows the key value, and uses it to verify the
// integrity of the message.
let v_key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
hmac::verify(&v_key, msg.as_bytes(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

## Using the multi-part API:
```
use ring::{digest, hmac, rand};
use ring::rand::SecureRandom;

let parts = ["hello", ", ", "world"];

// The sender generates a secure key value and signs the message with it.
// Note that in a real protocol, a key agreement protocol would be used to
// derive `key_value`.
let rng = rand::SystemRandom::new();
let mut key_value: [u8; digest::SHA384_OUTPUT_LEN] = rand::generate(&rng)?.expose();

let s_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
let mut s_ctx = hmac::Context::with_key(&s_key);
for part in &parts {
    s_ctx.update(part.as_bytes());
}
let tag = s_ctx.sign();

// The receiver (somehow!) knows the key value, and uses it to verify the
// integrity of the message.
let v_key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());
let mut msg = Vec::<u8>::new();
for part in &parts {
    msg.extend(part.as_bytes());
}
hmac::verify(&v_key, &msg.as_ref(), tag.as_ref())?;

# Ok::<(), ring::error::Unspecified>(())
```

[RFC 2104]: https://tools.ietf.org/html/rfc2104

## Structs

### `Algorithm`

```rust
struct Algorithm();
```

An HMAC algorithm.

#### Implementations

- `fn digest_algorithm(self: &Self) -> &'static digest::Algorithm`
  The digest algorithm this HMAC algorithm is based on.

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

### `Tag`

```rust
struct Tag();
```

An HMAC tag.

For a given tag `t`, use `t.as_ref()` to get the tag value as a byte slice.

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

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Key`

```rust
struct Key {
    // [REDACTED: Private Fields]
}
```

A key to use for HMAC signing.

#### Implementations

- `fn generate(algorithm: Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified>`
  Generate an HMAC signing key using the given digest algorithm with a

- `fn new(algorithm: Algorithm, key_value: &[u8]) -> Self`
  Construct an HMAC signing key using the given digest algorithm and key

- `fn algorithm(self: &Self) -> Algorithm`
  The digest algorithm for the key.

#### Trait Implementations

##### `impl From`

- `fn from(okm: hkdf::Okm<'_, Algorithm>) -> Self`

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

- `fn clone(self: &Self) -> Key`

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

### `Context`

```rust
struct Context {
    // [REDACTED: Private Fields]
}
```

A context for multi-step (Init-Update-Finish) HMAC signing.

Use `sign` for single-step HMAC signing.

#### Implementations

- `fn with_key(signing_key: &Key) -> Self`
  Constructs a new HMAC signing context using the given digest algorithm

- `fn update(self: &mut Self, data: &[u8])`
  Updates the HMAC with all the data in `data`. `update` may be called

- `fn sign(self: Self) -> Tag`
  Finalizes the HMAC calculation and returns the HMAC value. `sign`

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

- `fn clone(self: &Self) -> Context`

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

## Functions

### `sign`

```rust
fn sign(key: &Key, data: &[u8]) -> Tag
```

Calculates the HMAC of `data` using the key `key` in one step.

Use `Context` to calculate HMACs where the input is in multiple parts.

It is generally not safe to implement HMAC verification by comparing the
return value of `sign` to a tag. Use `verify` for verification instead.

### `verify`

```rust
fn verify(key: &Key, data: &[u8], tag: &[u8]) -> Result<(), error::Unspecified>
```

Calculates the HMAC of `data` using the signing key `key`, and verifies
whether the resultant value equals `tag`, in one step.

This is logically equivalent to, but more efficient than, constructing a
`Key` with the same value as `key` and then using `verify`.

The verification will be done in constant time to prevent timing attacks.

