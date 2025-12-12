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

*Defined in [`crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs:30-45`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/atomic/atomic_cell.rs#L30-L45)*

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

- <span id="atomiccell-into-inner"></span>`fn into_inner(self) -> T`

- <span id="atomiccell-is-lock-free"></span>`const fn is_lock_free() -> bool`

- <span id="atomiccell-store"></span>`fn store(&self, val: T)`

- <span id="atomiccell-swap"></span>`fn swap(&self, val: T) -> T`

- <span id="atomiccell-as-ptr"></span>`fn as_ptr(&self) -> *mut T`

#### Trait Implementations

##### `impl<T: Copy + fmt::Debug> Debug for AtomicCell<T>`

- <span id="atomiccell-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for AtomicCell<T>`

- <span id="atomiccell-default"></span>`fn default() -> AtomicCell<T>` — [`AtomicCell`](atomic_cell/index.md#atomiccell)

##### `impl<T> Drop for AtomicCell<T>`

- <span id="atomiccell-drop"></span>`fn drop(&mut self)`

##### `impl<T> RefUnwindSafe for AtomicCell<T>`

##### `impl<T: Send> Send for AtomicCell<T>`

##### `impl<T: Send> Sync for AtomicCell<T>`

##### `impl<T> UnwindSafe for AtomicCell<T>`

## Traits

### `AtomicConsume`

```rust
trait AtomicConsume { ... }
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/consume.rs:5-25`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/atomic/consume.rs#L5-L25)*

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

