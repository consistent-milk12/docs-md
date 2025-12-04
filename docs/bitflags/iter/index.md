*[bitflags](../index.md) / [iter](index.md)*

---

# Module `iter`

Yield the bits of a source flags value in a set of contained flags values.

## Structs

### `Iter<B: 'static>`

```rust
struct Iter<B: 'static> {
    // [REDACTED: Private Fields]
}
```

An iterator over flags values.

This iterator will yield flags values for contained, defined flags first, with any remaining bits yielded
as a final flags value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<B: Flags>`

- `type Item = B`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `IterNames<B: 'static>`

```rust
struct IterNames<B: 'static> {
    // [REDACTED: Private Fields]
}
```

An iterator over flags values.

This iterator only yields flags values for contained, defined, named flags. Any remaining bits
won't be yielded, but can be found with the `IterNames::remaining` method.

#### Implementations

- `fn remaining(self: &Self) -> &B`
  Get a flags value of any remaining bits that haven't been yielded yet.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<B: Flags>`

- `type Item = (&'static str, B)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `IterDefinedNames<B: 'static>`

```rust
struct IterDefinedNames<B: 'static> {
    // [REDACTED: Private Fields]
}
```

An iterator over all defined named flags.

This iterator will yield flags values for all defined named flags, regardless of
whether they are contained in a particular flags value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<B: Flags>`

- `type Item = (&'static str, B)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

