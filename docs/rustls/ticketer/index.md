*[rustls](../index.md) / [ticketer](index.md)*

---

# Module `ticketer`

APIs for implementing TLS tickets

## Structs

### `TicketSwitcher`

```rust
struct TicketSwitcher {
    // [REDACTED: Private Fields]
}
```

A ticketer that has a 'current' sub-ticketer and a single
'previous' ticketer.  It creates a new ticketer every so
often, demoting the current ticketer.

#### Implementations

- `fn new(lifetime: u32, generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>) -> Result<Self, Error>`
  Creates a new `TicketSwitcher`, which rotates through sub-ticketers

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

##### `impl ProducesTickets`

- `fn lifetime(self: &Self) -> u32`

- `fn enabled(self: &Self) -> bool`

- `fn encrypt(self: &Self, message: &[u8]) -> Option<Vec<u8>>`

- `fn decrypt(self: &Self, ciphertext: &[u8]) -> Option<Vec<u8>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TicketRotator`

```rust
struct TicketRotator {
    // [REDACTED: Private Fields]
}
```

A ticketer that has a 'current' sub-ticketer and a single
'previous' ticketer.  It creates a new ticketer every so
often, demoting the current ticketer.

#### Implementations

- `fn new(lifetime: u32, generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>) -> Result<Self, Error>`
  Creates a new `TicketRotator`, which rotates through sub-ticketers

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

##### `impl ProducesTickets`

- `fn lifetime(self: &Self) -> u32`

- `fn enabled(self: &Self) -> bool`

- `fn encrypt(self: &Self, message: &[u8]) -> Option<Vec<u8>>`

- `fn decrypt(self: &Self, ciphertext: &[u8]) -> Option<Vec<u8>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

