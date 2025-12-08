*[once_cell](../../index.md) / [race](../index.md) / [once_box](index.md)*

---

# Module `once_box`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceBox`](#oncebox) | struct | A thread-safe cell which can be written to only once. |
| [`_dummy`](#_dummy) | fn | ```compile_fail |

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

