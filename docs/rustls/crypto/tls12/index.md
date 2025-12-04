*[rustls](../../index.md) / [crypto](../index.md) / [tls12](index.md)*

---

# Module `tls12`

Cryptography specific to TLS1.2.

## Structs

### `PrfUsingHmac<'a>`

```rust
struct PrfUsingHmac<'a>(&'a dyn hmac::Hmac);
```

Implements [`Prf`](crypto/tls12/index.md) using a `hmac::Hmac`.

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

##### `impl Prf`

- `fn for_key_exchange(self: &Self, output: &mut [u8; 48], kx: Box<dyn ActiveKeyExchange>, peer_pub_key: &[u8], label: &[u8], seed: &[u8]) -> Result<(), Error>`

- `fn for_secret(self: &Self, output: &mut [u8], secret: &[u8], label: &[u8], seed: &[u8])`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Prf`

```rust
trait Prf: Send + Sync { ... }
```

An instantiation of the TLS1.2 PRF with a specific, implicit hash function.

See the definition in [RFC5246 section 5](https://www.rfc-editor.org/rfc/rfc5246#section-5).

See [`PrfUsingHmac`](crypto/tls12/index.md) as a route to implementing this trait with just
an implementation of `hmac::Hmac`.

#### Required Methods

- `fn for_key_exchange(self: &Self, output: &mut [u8; 48], kx: Box<dyn ActiveKeyExchange>, peer_pub_key: &[u8], label: &[u8], seed: &[u8]) -> Result<(), Error>`

  Computes `PRF(secret, label, seed)` using the secret from a completed key exchange.

- `fn for_secret(self: &Self, output: &mut [u8], secret: &[u8], label: &[u8], seed: &[u8])`

  Computes `PRF(secret, label, seed)`, writing the result into `output`.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

