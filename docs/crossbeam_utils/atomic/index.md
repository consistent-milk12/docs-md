*[crossbeam_utils](../index.md) / [atomic](index.md)*

---

# Module `atomic`

Atomic types.

* [`AtomicCell`](atomic_cell/index.md), a thread-safe mutable memory location.
* [`AtomicConsume`](consume/index.md), for reading from primitive atomic types with "consume" ordering.

## Structs

### `AtomicCell<T>`

```rust
struct AtomicCell<T> {
    value: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>,
}
```

A thread-safe mutable memory location.

This type is equivalent to [`Cell`](#cell), except it can also be shared among multiple threads.

Operations on `AtomicCell`s use atomic instructions whenever possible, and synchronize using
global locks otherwise. You can call `AtomicCell::<T>::is_lock_free()` to check whether
atomic instructions or locks will be used.

Atomic loads use the [`Acquire`](#acquire) ordering and atomic stores use the [`Release`](#release) ordering.





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

- `fn fetch_add(self: &Self, val: u8) -> u8`

- `fn fetch_sub(self: &Self, val: u8) -> u8`

- `fn fetch_and(self: &Self, val: u8) -> u8`

- `fn fetch_nand(self: &Self, val: u8) -> u8`

- `fn fetch_or(self: &Self, val: u8) -> u8`

- `fn fetch_xor(self: &Self, val: u8) -> u8`

- `fn fetch_max(self: &Self, val: u8) -> u8`

- `fn fetch_min(self: &Self, val: u8) -> u8`

#### Trait Implementations

##### `impl Debug<T: Copy + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T: Default>`

- `fn default() -> AtomicCell<T>` — [`AtomicCell`](../../atomic/atomic_cell/index.md)

##### `impl Drop<T>`

- `fn drop(self: &mut Self)`

##### `impl RefUnwindSafe<T>`

##### `impl Send<T: Send>`

##### `impl Sync<T: Send>`

##### `impl UnwindSafe<T>`

## Traits

