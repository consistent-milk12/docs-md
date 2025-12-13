*[crossbeam_epoch](../../index.md) / [primitive](../index.md) / [cell](index.md)*

---

# Module `cell`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnsafeCell`](#unsafecell) | struct |  |

## Structs

### `UnsafeCell<T>`

```rust
struct UnsafeCell<T>(::core::cell::UnsafeCell<T>);
```

*Defined in [`crossbeam-epoch-0.9.18/src/lib.rs:97`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/lib.rs#L97)*

#### Implementations

- <span id="unsafecell-new"></span>`const fn new(data: T) -> UnsafeCell<T>` — [`UnsafeCell`](#unsafecell)

- <span id="unsafecell-with"></span>`fn with<R>(&self, f: impl FnOnce(*const T) -> R) -> R`

- <span id="unsafecell-with-mut"></span>`fn with_mut<R>(&self, f: impl FnOnce(*mut T) -> R) -> R`

#### Trait Implementations

##### `impl<T> Any for UnsafeCell<T>`

- <span id="unsafecell-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnsafeCell<T>`

- <span id="unsafecell-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnsafeCell<T>`

- <span id="unsafecell-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for UnsafeCell<T>`

- <span id="unsafecell-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnsafeCell<T>`

- <span id="unsafecell-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for UnsafeCell<T>`

- <span id="unsafecell-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for UnsafeCell<T>`

- <span id="unsafecell-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unsafecell-pointable-type-init"></span>`type Init = T`

- <span id="unsafecell-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="unsafecell-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unsafecell-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unsafecell-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for UnsafeCell<T>`

- <span id="unsafecell-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unsafecell-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnsafeCell<T>`

- <span id="unsafecell-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unsafecell-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

