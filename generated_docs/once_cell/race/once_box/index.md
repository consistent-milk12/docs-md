*[once_cell](../../index.md) / [race](../index.md) / [once_box](index.md)*

---

# Module `once_box`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceBox`](#oncebox) | struct | A thread-safe cell which can be written to only once. |
| [`_dummy`](#dummy) | fn | ```compile_fail struct S(*mut ()); unsafe impl Sync for S {} |

## Structs

### `OnceBox<T>`

```rust
struct OnceBox<T> {
    inner: super::atomic::AtomicPtr<T>,
    ghost: core::marker::PhantomData<Option<alloc::boxed::Box<T>>>,
}
```

*Defined in [`once_cell-1.21.3/src/race.rs:361-364`](../../../../.source_1765633015/once_cell-1.21.3/src/race.rs#L361-L364)*

A thread-safe cell which can be written to only once.

#### Implementations

- <span id="oncebox-new"></span>`const fn new() -> Self`

  Creates a new empty cell.

- <span id="oncebox-with-value"></span>`fn with_value(value: Box<T>) -> Self`

  Creates a new cell with the given value.

- <span id="oncebox-get"></span>`fn get(&self) -> Option<&T>`

  Gets a reference to the underlying value.

- <span id="oncebox-set"></span>`fn set(&self, value: Box<T>) -> Result<(), Box<T>>`

  Sets the contents of this cell to `value`.

  

  Returns `Ok(())` if the cell was empty and `Err(value)` if it was

  full.

- <span id="oncebox-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

  Gets the contents of the cell, initializing it with `f` if the cell was

  empty.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncebox-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>`

  Gets the contents of the cell, initializing it with `f` if

  the cell was empty. If the cell was empty and `f` failed, an

  error is returned.

  

  If several threads concurrently run `get_or_init`, more than one `f` can

  be called. However, all threads will return the same value, produced by

  some `f`.

- <span id="oncebox-init"></span>`fn init<E>(&self, f: impl FnOnce() -> Result<Box<T>, E>) -> Result<&T, E>`

#### Trait Implementations

##### `impl<T> Any for OnceBox<T>`

- <span id="oncebox-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceBox<T>`

- <span id="oncebox-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceBox<T>`

- <span id="oncebox-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone> Clone for OnceBox<T>`

- <span id="oncebox-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for OnceBox<T>`

- <span id="oncebox-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for OnceBox<T>`

- <span id="oncebox-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for OnceBox<T>`

- <span id="oncebox-default"></span>`fn default() -> Self`

##### `impl<T> Drop for OnceBox<T>`

- <span id="oncebox-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for OnceBox<T>`

- <span id="oncebox-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OnceBox<T>`

- <span id="oncebox-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Sync + Send> Sync for OnceBox<T>`

##### `impl<T> ToOwned for OnceBox<T>`

- <span id="oncebox-toowned-type-owned"></span>`type Owned = T`

- <span id="oncebox-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="oncebox-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for OnceBox<T>`

- <span id="oncebox-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncebox-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OnceBox<T>`

- <span id="oncebox-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncebox-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `_dummy`

```rust
fn _dummy()
```

*Defined in [`once_cell-1.21.3/src/race.rs:497`](../../../../.source_1765633015/once_cell-1.21.3/src/race.rs#L497)*

```compile_fail
struct S(*mut ());
unsafe impl Sync for S {}

fn share<T: Sync>(_: &T) {}
share(&once_cell::race::OnceBox::<S>::new());
```

