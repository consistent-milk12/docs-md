*[once_cell](../../index.md) / [race](../index.md) / [once_box](index.md)*

---

# Module `once_box`

## Structs

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

## Functions

### `_dummy`

```rust
fn _dummy()
```

```compile_fail
struct S(*mut ());
unsafe impl Sync for S {}

fn share<T: Sync>(_: &T) {}
share(&once_cell::race::OnceBox::<S>::new());
```

