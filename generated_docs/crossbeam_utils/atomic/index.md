*[crossbeam_utils](../index.md) / [atomic](index.md)*

---

# Module `atomic`

Atomic types.

* [`AtomicCell`](#atomiccell), a thread-safe mutable memory location.
* [`AtomicConsume`](#atomicconsume), for reading from primitive atomic types with "consume" ordering.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`seq_lock`](#seq_lock) | mod |  |
| [`atomic_cell`](#atomic_cell) | mod |  |
| [`consume`](#consume) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | trait |  |

## Modules

- [`seq_lock`](seq_lock/index.md) - 
- [`atomic_cell`](atomic_cell/index.md) - 
- [`consume`](consume/index.md) - 

## Structs

### `AtomicCell<T>`

```rust
struct AtomicCell<T> {
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

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

- <span id="atomiccell-fetch-add"></span>`fn fetch_add(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-sub"></span>`fn fetch_sub(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-and"></span>`fn fetch_and(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-nand"></span>`fn fetch_nand(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-or"></span>`fn fetch_or(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-xor"></span>`fn fetch_xor(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-max"></span>`fn fetch_max(&self, val: u128) -> u128`

- <span id="atomiccell-fetch-min"></span>`fn fetch_min(&self, val: u128) -> u128`

#### Trait Implementations

##### `impl<T: Copy + fmt::Debug> Debug for AtomicCell<T>`

- <span id="atomiccell-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for AtomicCell<T>`

- <span id="atomiccell-default"></span>`fn default() -> AtomicCell<T>` — [`AtomicCell`](#atomiccell)

##### `impl<T> Drop for AtomicCell<T>`

- <span id="atomiccell-drop"></span>`fn drop(&mut self)`

##### `impl<T> RefUnwindSafe for AtomicCell<T>`

##### `impl<T: Send> Send for AtomicCell<T>`

##### `impl<T: Send> Sync for AtomicCell<T>`

##### `impl<T> UnwindSafe for AtomicCell<T>`

## Traits

