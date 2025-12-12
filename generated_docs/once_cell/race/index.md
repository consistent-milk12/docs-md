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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`once_box`](#once-box) | mod |  |
| [`OnceNonZeroUsize`](#oncenonzerousize) | struct | A thread-safe cell which can be written to only once. |
| [`OnceBool`](#oncebool) | struct | A thread-safe cell which can be written to only once. |
| [`OnceRef`](#onceref) | struct | A thread-safe cell which can be written to only once. |
| [`OnceBox`](#oncebox) | struct |  |

## Modules

- [`once_box`](once_box/index.md)

## Structs

### `OnceNonZeroUsize`

```rust
struct OnceNonZeroUsize {
    inner: atomic::AtomicUsize,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:43-45`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L43-L45)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncenonzerousize-new"></span>`const fn new() -> Self`

- <span id="oncenonzerousize-get"></span>`fn get(&self) -> Option<NonZeroUsize>`

- <span id="oncenonzerousize-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> NonZeroUsize`

- <span id="oncenonzerousize-set"></span>`fn set(&self, value: NonZeroUsize) -> Result<(), ()>`

- <span id="oncenonzerousize-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> NonZeroUsize`

- <span id="oncenonzerousize-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>`

- <span id="oncenonzerousize-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<NonZeroUsize, E>) -> Result<NonZeroUsize, E>`

- <span id="oncenonzerousize-compare-exchange"></span>`fn compare_exchange(&self, val: NonZeroUsize) -> Result<usize, usize>`

#### Trait Implementations

##### `impl Debug for OnceNonZeroUsize`

- <span id="oncenonzerousize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OnceNonZeroUsize`

- <span id="oncenonzerousize-default"></span>`fn default() -> OnceNonZeroUsize` — [`OnceNonZeroUsize`](#oncenonzerousize)

### `OnceBool`

```rust
struct OnceBool {
    inner: OnceNonZeroUsize,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:167-169`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L167-L169)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncebool-new"></span>`const fn new() -> Self`

- <span id="oncebool-get"></span>`fn get(&self) -> Option<bool>`

- <span id="oncebool-set"></span>`fn set(&self, value: bool) -> Result<(), ()>`

- <span id="oncebool-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> bool`

- <span id="oncebool-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>`

- <span id="oncebool-from-usize"></span>`fn from_usize(value: NonZeroUsize) -> bool`

- <span id="oncebool-to-usize"></span>`fn to_usize(value: bool) -> NonZeroUsize`

#### Trait Implementations

##### `impl Debug for OnceBool`

- <span id="oncebool-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OnceBool`

- <span id="oncebool-default"></span>`fn default() -> OnceBool` — [`OnceBool`](#oncebool)

### `OnceRef<'a, T>`

```rust
struct OnceRef<'a, T> {
    inner: atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<core::cell::UnsafeCell<&'a T>>,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:232-235`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L232-L235)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="onceref-new"></span>`const fn new() -> Self`

- <span id="onceref-get"></span>`fn get(&self) -> Option<&'a T>`

- <span id="onceref-set"></span>`fn set(&self, value: &'a T) -> Result<(), ()>`

- <span id="onceref-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &'a T`

- <span id="onceref-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&'a T, E>`

- <span id="onceref-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<&'a T, E>) -> Result<&'a T, E>`

- <span id="onceref-compare-exchange"></span>`fn compare_exchange(&self, value: &'a T) -> Result<(), *const T>`

- <span id="onceref-dummy"></span>`fn _dummy()`

#### Trait Implementations

##### `impl<T> Debug for OnceRef<'a, T>`

- <span id="onceref-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for OnceRef<'a, T>`

- <span id="onceref-default"></span>`fn default() -> Self`

##### `impl<T: Sync> Sync for OnceRef<'a, T>`

### `OnceBox<T>`

```rust
struct OnceBox<T> {
    inner: super::atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<Option<alloc::boxed::Box<T>>>,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:361-364`](../../../.source_1765521767/once_cell-1.21.3/src/race.rs#L361-L364)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncebox-new"></span>`const fn new() -> Self`

- <span id="oncebox-with-value"></span>`fn with_value(value: Box<T>) -> Self`

- <span id="oncebox-get"></span>`fn get(&self) -> Option<&T>`

- <span id="oncebox-set"></span>`fn set(&self, value: Box<T>) -> Result<(), Box<T>>`

- <span id="oncebox-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

- <span id="oncebox-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>`

- <span id="oncebox-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E>`

#### Trait Implementations

##### `impl<T: Clone> Clone for OnceBox<T>`

- <span id="oncebox-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for OnceBox<T>`

- <span id="oncebox-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for OnceBox<T>`

- <span id="oncebox-default"></span>`fn default() -> Self`

##### `impl<T> Drop for OnceBox<T>`

- <span id="oncebox-drop"></span>`fn drop(&mut self)`

##### `impl<T: Sync + Send> Sync for OnceBox<T>`

