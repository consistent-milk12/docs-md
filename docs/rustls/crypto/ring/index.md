*[rustls](../../index.md) / [crypto](../index.md) / [ring](index.md)*

---

# Module `ring`

*ring* based CryptoProvider.

## Modules

- [`sign`](sign/index.md) - Using software keys for authentication.
- [`cipher_suite`](cipher_suite/index.md) - All defined cipher suites supported by *ring* appear in this module.
- [`kx_group`](kx_group/index.md) - All defined key exchange groups supported by *ring* appear in this module.

## Structs

### `Ticketer`

```rust
struct Ticketer {
}
```

A concrete, safe ticket creation mechanism.

#### Implementations

- `fn new() -> Result<alloc::sync::Arc<dyn ProducesTickets>, Error>`
  Make the recommended `Ticketer`.  This produces tickets

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

## Functions

### `default_provider`

```rust
fn default_provider() -> crate::crypto::CryptoProvider
```

A `CryptoProvider` backed by the [*ring*] crate.

[*ring*]: https://github.com/briansmith/ring

