*[rustls](../index.md) / [time_provider](index.md)*

---

# Module `time_provider`

The library's source of time.

## Structs

### `DefaultTimeProvider`

```rust
struct DefaultTimeProvider;
```

Default `TimeProvider` implementation that uses `std`

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

##### `impl TimeProvider`

- `fn current_time(self: &Self) -> Option<UnixTime>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `TimeProvider`

```rust
trait TimeProvider: Debug + Send + Sync { ... }
```

An object that provides the current time.

This is used to, for example, check if a certificate has expired during
certificate validation, or to check the age of a ticket.

#### Required Methods

- `fn current_time(self: &Self) -> Option<UnixTime>`

  Returns the current wall time.

