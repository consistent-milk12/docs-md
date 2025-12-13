*[crossbeam_utils](../../index.md) / [atomic](../index.md) / [atomic_cell](index.md)*

---

# Module `atomic_cell`

## Contents

- [Structs](#structs)
  - [`AtomicCell`](#atomiccell)
  - [`AtomicUnit`](#atomicunit)
- [Functions](#functions)
  - [`can_transmute`](#can-transmute)
  - [`lock`](#lock)
  - [`atomic_is_lock_free`](#atomic-is-lock-free)
  - [`atomic_load`](#atomic-load)
  - [`atomic_store`](#atomic-store)
  - [`atomic_swap`](#atomic-swap)
  - [`atomic_compare_exchange_weak`](#atomic-compare-exchange-weak)
- [Macros](#macros)
  - [`atomic!`](#atomic)
  - [`impl_arithmetic!`](#impl-arithmetic)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AtomicCell`](#atomiccell) | struct | A thread-safe mutable memory location. |
| [`AtomicUnit`](#atomicunit) | struct | An atomic `()`. |
| [`can_transmute`](#can-transmute) | fn | Returns `true` if values of type `A` can be transmuted into values of type `B`. |
| [`lock`](#lock) | fn | Returns a reference to the global lock associated with the `AtomicCell` at address `addr`. |
| [`atomic_is_lock_free`](#atomic-is-lock-free) | fn | Returns `true` if operations on `AtomicCell<T>` are lock-free. |
| [`atomic_load`](#atomic-load) | fn | Atomically reads data from `src`. |
| [`atomic_store`](#atomic-store) | fn | Atomically writes `val` to `dst`. |
| [`atomic_swap`](#atomic-swap) | fn | Atomically swaps data at `dst` with `val`. |
| [`atomic_compare_exchange_weak`](#atomic-compare-exchange-weak) | fn | Atomically compares data at `dst` to `current` and, if equal byte-for-byte, exchanges data at `dst` with `new`. |
| [`atomic!`](#atomic) | macro |  |
| [`impl_arithmetic!`](#impl-arithmetic) | macro |  |

## Structs

### `AtomicCell<T>`

```rust
struct AtomicCell<T> {
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:30-45`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L30-L45)*

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

- <span id="atomiccell-new"></span>`const fn new(val: T) -> AtomicCell<T>` — [`AtomicCell`](#atomiccell)

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

- <span id="atomiccell-default"></span>`fn default() -> AtomicCell<T>` — [`AtomicCell`](#atomiccell)

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

### `AtomicUnit`

```rust
struct AtomicUnit;
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:1015`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L1015)*

An atomic `()`.

All operations are noops.

#### Implementations

- <span id="atomicunit-load"></span>`fn load(&self, _order: Ordering)`

- <span id="atomicunit-store"></span>`fn store(&self, _val: (), _order: Ordering)`

- <span id="atomicunit-swap"></span>`fn swap(&self, _val: (), _order: Ordering)`

- <span id="atomicunit-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, _current: (), _new: (), _success: Ordering, _failure: Ordering) -> Result<(), ()>`

#### Trait Implementations

##### `impl Any for AtomicUnit`

- <span id="atomicunit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicUnit`

- <span id="atomicunit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicUnit`

- <span id="atomicunit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicUnit`

- <span id="atomicunit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicUnit`

- <span id="atomicunit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AtomicUnit`

- <span id="atomicunit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicunit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicUnit`

- <span id="atomicunit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicunit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `can_transmute`

```rust
const fn can_transmute<A, B>() -> bool
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:965-968`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L965-L968)*

Returns `true` if values of type `A` can be transmuted into values of type `B`.

### `lock`

```rust
fn lock(addr: usize) -> &'static super::seq_lock::SeqLock
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:980-1010`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L980-L1010)*

Returns a reference to the global lock associated with the `AtomicCell` at address `addr`.

This function is used to protect atomic data which doesn't fit into any of the primitive atomic
types in `std::sync::atomic`. Operations on such atomics must therefore use a global lock.

However, there is not only one global lock but an array of many locks, and one of them is
picked based on the given address. Having many locks reduces contention and improves
scalability.

### `atomic_is_lock_free`

```rust
const fn atomic_is_lock_free<T>() -> bool
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:1040-1042`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L1040-L1042)*

Returns `true` if operations on `AtomicCell<T>` are lock-free.

### `atomic_load`

```rust
unsafe fn atomic_load<T>(src: *mut T) -> T
where
    T: Copy
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:1048-1084`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L1048-L1084)*

Atomically reads data from `src`.

This operation uses the `Acquire` ordering. If possible, an atomic instructions is used, and a
global lock otherwise.

### `atomic_store`

```rust
unsafe fn atomic_store<T>(dst: *mut T, val: T)
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:1090-1103`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L1090-L1103)*

Atomically writes `val` to `dst`.

This operation uses the `Release` ordering. If possible, an atomic instructions is used, and a
global lock otherwise.

### `atomic_swap`

```rust
unsafe fn atomic_swap<T>(dst: *mut T, val: T) -> T
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:1109-1123`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L1109-L1123)*

Atomically swaps data at `dst` with `val`.

This operation uses the `AcqRel` ordering. If possible, an atomic instructions is used, and a
global lock otherwise.

### `atomic_compare_exchange_weak`

```rust
unsafe fn atomic_compare_exchange_weak<T>(dst: *mut T, current: T, new: T) -> Result<T, T>
where
    T: Copy + Eq
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:1133-1182`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L1133-L1182)*

Atomically compares data at `dst` to `current` and, if equal byte-for-byte, exchanges data at
`dst` with `new`.

Returns the old value on success, or the current value at `dst` on failure.

This operation uses the `AcqRel` ordering. If possible, an atomic instructions is used, and a
global lock otherwise.

## Macros

### `atomic!`

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:321-349`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L321-L349)*

### `impl_arithmetic!`

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:351-778`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L351-L778)*

