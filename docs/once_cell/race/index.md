*[once_cell](../index.md) / [race](index.md)*

---

# Module `race`

Thread-safe, non-blocking, "first one wins" flavor of `OnceCell`.

If two threads race to initialize a type from the `race` module, they
don't block, execute initialization function together, but only one of
them stores the result.

This module does not require `std` feature.

# Atomic orderings

All types in this module use `Acquire` and `Release`
[atomic orderings](Ordering) for all their operations. While this is not
strictly necessary for types other than `OnceBox`, it is useful for users as
it allows them to be certain that after `get` or `get_or_init` returns on
one thread, any side-effects caused by the setter thread prior to them
calling `set` or `get_or_init` will be made visible to that thread; without
it, it's possible for it to appear as if they haven't happened yet from the
getter thread's perspective. This is an acceptable tradeoff to make since
`Acquire` and `Release` have very little performance overhead on most
architectures versus `Relaxed`.

## Structs

### `OnceNonZeroUsize`

```rust
struct OnceNonZeroUsize {
    // [REDACTED: Private Fields]
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`
  Creates a new empty cell.

- `fn get(self: &Self) -> Option<NonZeroUsize>`
  Gets the underlying value.

- `unsafe fn get_unchecked(self: &Self) -> NonZeroUsize`
  Get the reference to the underlying value, without checking if the cell

- `fn set(self: &Self, value: NonZeroUsize) -> Result<(), ()>`
  Sets the contents of this cell to `value`.

- `fn get_or_init<F>(self: &Self, f: F) -> NonZeroUsize`
  Gets the contents of the cell, initializing it with `f` if the cell was

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<NonZeroUsize, E>`
  Gets the contents of the cell, initializing it with `f` if

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

##### `impl Default`

- `fn default() -> OnceNonZeroUsize`

### `OnceBool`

```rust
struct OnceBool {
    // [REDACTED: Private Fields]
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`
  Creates a new empty cell.

- `fn get(self: &Self) -> Option<bool>`
  Gets the underlying value.

- `fn set(self: &Self, value: bool) -> Result<(), ()>`
  Sets the contents of this cell to `value`.

- `fn get_or_init<F>(self: &Self, f: F) -> bool`
  Gets the contents of the cell, initializing it with `f` if the cell was

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<bool, E>`
  Gets the contents of the cell, initializing it with `f` if

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

##### `impl Default`

- `fn default() -> OnceBool`

### `OnceRef<'a, T>`

```rust
struct OnceRef<'a, T> {
    // [REDACTED: Private Fields]
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`
  Creates a new empty cell.

- `fn get(self: &Self) -> Option<&'a T>`
  Gets a reference to the underlying value.

- `fn set(self: &Self, value: &'a T) -> Result<(), ()>`
  Sets the contents of this cell to `value`.

- `fn get_or_init<F>(self: &Self, f: F) -> &'a T`
  Gets the contents of the cell, initializing it with `f` if the cell was

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&'a T, E>`
  Gets the contents of the cell, initializing it with `f` if

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

##### `impl Sync<'a, T: Sync>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default<'a, T>`

- `fn default() -> Self`

### `OnceBox<T>`

```rust
struct OnceBox<T> {
    // [REDACTED: Private Fields]
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`
  Creates a new empty cell.

- `fn with_value(value: Box<T>) -> Self`
  Creates a new cell with the given value.

- `fn get(self: &Self) -> Option<&T>`
  Gets a reference to the underlying value.

- `fn set(self: &Self, value: Box<T>) -> Result<(), Box<T>>`
  Sets the contents of this cell to `value`.

- `fn get_or_init<F>(self: &Self, f: F) -> &T`
  Gets the contents of the cell, initializing it with `f` if the cell was

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>`
  Gets the contents of the cell, initializing it with `f` if

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

##### `impl Clone<T: Clone>`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Drop<T>`

- `fn drop(self: &mut Self)`

##### `impl Sync<T: Sync + Send>`

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

##### `impl Debug<T>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default<T>`

- `fn default() -> Self`

