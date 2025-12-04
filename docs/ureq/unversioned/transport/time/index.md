*[ureq](../../../index.md) / [unversioned](../../index.md) / [transport](../index.md) / [time](index.md)*

---

# Module `time`

Internal time wrappers

## Enums

### `Instant`

```rust
enum Instant {
    AlreadyHappened,
    Exact(time::Instant),
    NotHappening,
}
```

Wrapper for [`std::time::Instant`](#instant) that provides additional time points in the past or future

#### Variants

- **`AlreadyHappened`**

  A time in the past that already happened.

- **`Exact`**

  An exact instant.

- **`NotHappening`**

  A time in the future that will never happen.

#### Implementations

- `fn now() -> Self`
  Current time.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Add`

- `type Output = Instant`

- `fn add(self: Self, rhs: Duration) -> <Self as >::Output`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Instant`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Instant) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

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

### `Duration`

```rust
enum Duration {
    Exact(time::Duration),
    NotHappening,
}
```

Wrapper for [`std::time::Duration`](#duration) that provides a duration to a distant future

#### Variants

- **`Exact`**

  An exact duration.

- **`NotHappening`**

  A duration so long it will never happen.

#### Implementations

- `fn from_secs(secs: u64) -> Duration`
  Creates a duration from seconds.

- `fn is_not_happening(self: &Self) -> bool`
  Tells if this duration will ever happen.

#### Trait Implementations

##### `impl From`

- `fn from(value: std::time::Duration) -> Self`

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

- `fn clone(self: &Self) -> Duration`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Duration) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl Receiver<P, T>`

- `type Target = T`

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

##### `impl Deref`

- `type Target = Duration`

- `fn deref(self: &Self) -> &<Self as >::Target`

