*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [once_lock](index.md)*

---

# Module `once_lock`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceLock`](#oncelock) | struct |  |

## Structs

### `OnceLock<T>`

```rust
struct OnceLock<T> {
    once: std::sync::Once,
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/once_lock.rs:9-14`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/sync/once_lock.rs#L9-L14)*

#### Implementations

- <span id="oncelock-new"></span>`const fn new() -> Self`

  Creates a new empty cell.

- <span id="oncelock-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

  Gets the contents of the cell, initializing it with `f` if the cell

  was empty.

  

  Many threads may call `get_or_init` concurrently with different

  initializing functions, but it is guaranteed that only one function

  will be executed.

  

  # Panics

  

  If `f` panics, the panic is propagated to the caller, and the cell

  remains uninitialized.

  

  It is an error to reentrantly initialize the cell from `f`. The

  exact outcome is unspecified. Current implementation deadlocks, but

  this may be changed to a panic in the future.

- <span id="oncelock-initialize"></span>`fn initialize<F>(&self, f: F)`

- <span id="oncelock-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

  # Safety

  

  The value must be initialized

#### Trait Implementations

##### `impl<T> Any for OnceLock<T>`

- <span id="oncelock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceLock<T>`

- <span id="oncelock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceLock<T>`

- <span id="oncelock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Drop for OnceLock<T>`

- <span id="oncelock-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for OnceLock<T>`

- <span id="oncelock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OnceLock<T>`

- <span id="oncelock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for OnceLock<T>`

- <span id="oncelock-pointable-const-align"></span>`const ALIGN: usize`

- <span id="oncelock-pointable-type-init"></span>`type Init = T`

- <span id="oncelock-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="oncelock-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="oncelock-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="oncelock-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Send for OnceLock<T>`

##### `impl<T: Sync + Send> Sync for OnceLock<T>`

##### `impl<T, U> TryFrom for OnceLock<T>`

- <span id="oncelock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncelock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OnceLock<T>`

- <span id="oncelock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncelock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

