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
    inner: atomic::AtomicUsize,
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`

- `fn get(self: &Self) -> Option<NonZeroUsize>`

- `unsafe fn get_unchecked(self: &Self) -> NonZeroUsize`

- `fn set(self: &Self, value: NonZeroUsize) -> Result<(), ()>`

- `fn get_or_init<F>(self: &Self, f: F) -> NonZeroUsize`

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<NonZeroUsize, E>`

- `fn init<E>(self: &Self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E>`

- `fn compare_exchange(self: &Self, val: NonZeroUsize) -> Result<usize, usize>`

#### Trait Implementations

##### `impl Debug for OnceNonZeroUsize`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for OnceNonZeroUsize`

- `fn default() -> OnceNonZeroUsize` — [`OnceNonZeroUsize`](#oncenonzerousize)

### `OnceBool`

```rust
struct OnceBool {
    inner: OnceNonZeroUsize,
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`

- `fn get(self: &Self) -> Option<bool>`

- `fn set(self: &Self, value: bool) -> Result<(), ()>`

- `fn get_or_init<F>(self: &Self, f: F) -> bool`

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<bool, E>`

- `fn from_usize(value: NonZeroUsize) -> bool`

- `fn to_usize(value: bool) -> NonZeroUsize`

#### Trait Implementations

##### `impl Debug for OnceBool`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for OnceBool`

- `fn default() -> OnceBool` — [`OnceBool`](#oncebool)

### `OnceRef<'a, T>`

```rust
struct OnceRef<'a, T> {
    inner: atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<core::cell::UnsafeCell<&'a T>>,
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`

- `fn get(self: &Self) -> Option<&'a T>`

- `fn set(self: &Self, value: &'a T) -> Result<(), ()>`

- `fn get_or_init<F>(self: &Self, f: F) -> &'a T`

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&'a T, E>`

- `fn init<E>(self: &Self, f: impl FnOnce() -> Result<&'a T, E>) -> Result<&'a T, E>`

- `fn compare_exchange(self: &Self, value: &'a T) -> Result<(), *const T>`

- `fn _dummy()`

#### Trait Implementations

##### `impl<'a, T> Debug for OnceRef<'a, T>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<'a, T> Default for OnceRef<'a, T>`

- `fn default() -> Self`

##### `impl<'a, T: Sync> Sync for OnceRef<'a, T>`

### `OnceBox<T>`

```rust
struct OnceBox<T> {
    inner: super::atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<Option<alloc::boxed::Box<T>>>,
}
```

A thread-safe cell which can be written to only once.

#### Implementations

- `const fn new() -> Self`

- `fn with_value(value: Box<T>) -> Self`

- `fn get(self: &Self) -> Option<&T>`

- `fn set(self: &Self, value: Box<T>) -> Result<(), Box<T>>`

- `fn get_or_init<F>(self: &Self, f: F) -> &T`

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>`

- `fn init<E>(self: &Self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E>`

#### Trait Implementations

##### `impl<T: Clone> Clone for OnceBox<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T> Debug for OnceBox<T>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for OnceBox<T>`

- `fn default() -> Self`

##### `impl<T> Drop for OnceBox<T>`

- `fn drop(self: &mut Self)`

##### `impl<T: Sync + Send> Sync for OnceBox<T>`

