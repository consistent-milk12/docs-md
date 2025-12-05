*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [shiftor](index.md)*

---

# Module `shiftor`

An implementation of the [Shift-Or substring search algorithm][shiftor](#shiftor).

[shiftor](#shiftor): https://en.wikipedia.org/wiki/Bitap_algorithm

## Structs

### `Finder`

```rust
struct Finder {
    // [REDACTED: Private Fields]
}
```

A forward substring searcher using the Shift-Or algorithm.

#### Implementations

- `fn new(needle: &[u8]) -> Option<Finder>`
  Create a new Shift-Or forward searcher for the given `needle`.

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`
  Return the first occurrence of the needle given to `Finder::new` in

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

