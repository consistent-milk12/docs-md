*[crossbeam_utils](../index.md) / [atomic](index.md)*

---

# Module `atomic`

Atomic types.

* [`AtomicCell`](atomic_cell/index.md), a thread-safe mutable memory location.
* [`AtomicConsume`](consume/index.md), for reading from primitive atomic types with "consume" ordering.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`seq_lock`](#seq-lock) | mod |  |
| [`atomic_cell`](#atomic-cell) | mod |  |
| [`consume`](#consume) | mod |  |
| [`AtomicCell`](#atomiccell) | struct |  |
| [`AtomicConsume`](#atomicconsume) | trait |  |

## Modules

- [`seq_lock`](seq_lock/index.md)
- [`atomic_cell`](atomic_cell/index.md)
- [`consume`](consume/index.md)

## Structs

### `AtomicCell<T>`

```rust
struct AtomicCell<T> {
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:30-45`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L30-L45)*

A thread-safe mutable memory location.

This type is equivalent to `Cell`, except it can also be shared among multiple threads.

Operations on `AtomicCell`s use atomic instructions whenever possible, and synchronize using
global locks otherwise. You can call `AtomicCell::<T>::is_lock_free()` to check whether
atomic instructions or locks will be used.

Atomic loads use the `Acquire` ordering and atomic stores use the `Release` ordering.





#### Fields

- **`value`**: `core::cell::UnsafeCell<core::mem::MaybeUninit<T>>`

  The inner value.
  
  If this value can be transmuted into a primitive atomic type, it will be treated as such.
  Otherwise, all potentially concurrent operations on this data will be protected by a global
  lock.
  
  Using MaybeUninit to prevent code outside the cell from observing partially initialized state:
  <https://github.com/crossbeam-rs/crossbeam/issues/833>
  (This rustc bug has been fixed in Rust 1.64.)
  
  Note:
  - we'll never store uninitialized `T` due to our API only using initialized `T`.
  - this `MaybeUninit` does *not* fix <https://github.com/crossbeam-rs/crossbeam/issues/315>.

#### Implementations

- <span id="atomiccell-new"></span>`const fn new(val: T) -> AtomicCell<T>` — [`AtomicCell`](atomic_cell/index.md#atomiccell)

  Creates a new atomic cell initialized with `val`.

  

  # Examples

  

  ```rust

  use crossbeam_utils::atomic::AtomicCell;

  

  let a = AtomicCell::new(7);

  ```

- <span id="atomiccell-into-inner"></span>`fn into_inner(self) -> T`

  Consumes the atomic and returns the contained value.

  

  This is safe because passing `self` by value guarantees that no other threads are

  concurrently accessing the atomic data.

  

  # Examples

  

  ```rust

  use crossbeam_utils::atomic::AtomicCell;

  

  let a = AtomicCell::new(7);

  let v = a.into_inner();

  

  assert_eq!(v, 7);

  ```

- <span id="atomiccell-is-lock-free"></span>`const fn is_lock_free() -> bool`

  Returns `true` if operations on values of this type are lock-free.

  

  If the compiler or the platform doesn't support the necessary atomic instructions,

  `AtomicCell<T>` will use global locks for every potentially concurrent atomic operation.

  

  # Examples

  

  ```rust

  use crossbeam_utils::atomic::AtomicCell;

  

  // This type is internally represented as `AtomicUsize` so we can just use atomic

  // operations provided by it.

  assert_eq!(AtomicCell::<usize>::is_lock_free(), true);

  

  // A wrapper struct around `isize`.

  struct Foo {

      bar: isize,

  }

  // `AtomicCell<Foo>` will be internally represented as `AtomicIsize`.

  assert_eq!(AtomicCell::<Foo>::is_lock_free(), true);

  

  // Operations on zero-sized types are always lock-free.

  assert_eq!(AtomicCell::<()>::is_lock_free(), true);

  

  // Very large types cannot be represented as any of the standard atomic types, so atomic

  // operations on them will have to use global locks for synchronization.

  assert_eq!(AtomicCell::<[u8; 1000]>::is_lock_free(), false);

  ```

- <span id="atomiccell-store"></span>`fn store(&self, val: T)`

  Stores `val` into the atomic cell.

  

  # Examples

  

  ```rust

  use crossbeam_utils::atomic::AtomicCell;

  

  let a = AtomicCell::new(7);

  

  assert_eq!(a.load(), 7);

  a.store(8);

  assert_eq!(a.load(), 8);

  ```

- <span id="atomiccell-swap"></span>`fn swap(&self, val: T) -> T`

  Stores `val` into the atomic cell and returns the previous value.

  

  # Examples

  

  ```rust

  use crossbeam_utils::atomic::AtomicCell;

  

  let a = AtomicCell::new(7);

  

  assert_eq!(a.load(), 7);

  assert_eq!(a.swap(8), 7);

  assert_eq!(a.load(), 8);

  ```

- <span id="atomiccell-as-ptr"></span>`fn as_ptr(&self) -> *mut T`

  Returns a raw pointer to the underlying data in this atomic cell.

  

  # Examples

  

  ```rust

  use crossbeam_utils::atomic::AtomicCell;

  

  let a = AtomicCell::new(5);

  

  let ptr = a.as_ptr();

  ```

#### Trait Implementations

##### `impl<T> Any for AtomicCell<T>`

- <span id="atomiccell-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicCell<T>`

- <span id="atomiccell-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicCell<T>`

- <span id="atomiccell-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Copy + fmt::Debug> Debug for AtomicCell<T>`

- <span id="atomiccell-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for AtomicCell<T>`

- <span id="atomiccell-default"></span>`fn default() -> AtomicCell<T>` — [`AtomicCell`](atomic_cell/index.md#atomiccell)

##### `impl<T> Drop for AtomicCell<T>`

- <span id="atomiccell-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for AtomicCell<T>`

- <span id="atomiccell-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AtomicCell<T>`

- <span id="atomiccell-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> RefUnwindSafe for AtomicCell<T>`

##### `impl<T: Send> Send for AtomicCell<T>`

##### `impl<T: Send> Sync for AtomicCell<T>`

##### `impl<T, U> TryFrom for AtomicCell<T>`

- <span id="atomiccell-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomiccell-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AtomicCell<T>`

- <span id="atomiccell-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomiccell-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> UnwindSafe for AtomicCell<T>`

## Traits

### `AtomicConsume`

```rust
trait AtomicConsume { ... }
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/consume.rs:5-25`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/consume.rs#L5-L25)*

Trait which allows reading from primitive atomic types with "consume" ordering.

#### Associated Types

- `type Val`

#### Required Methods

- `fn load_consume(&self) -> <Self as >::Val`

  Loads a value from the atomic using a "consume" memory ordering.

#### Implementors

- `core::sync::atomic::AtomicBool`
- `core::sync::atomic::AtomicI16`
- `core::sync::atomic::AtomicI32`
- `core::sync::atomic::AtomicI64`
- `core::sync::atomic::AtomicI8`
- `core::sync::atomic::AtomicIsize`
- `core::sync::atomic::AtomicPtr<T>`
- `core::sync::atomic::AtomicU16`
- `core::sync::atomic::AtomicU32`
- `core::sync::atomic::AtomicU64`
- `core::sync::atomic::AtomicU8`
- `core::sync::atomic::AtomicUsize`

