*[portable_atomic](../../index.md) / [imp](../index.md) / [core_atomic](index.md)*

---

# Module `core_atomic`

## Contents

- [Structs](#structs)
  - [`NotRefUnwindSafe`](#notrefunwindsafe)
  - [`AtomicPtr`](#atomicptr)
  - [`AtomicIsize`](#atomicisize)
  - [`AtomicUsize`](#atomicusize)
  - [`AtomicI8`](#atomici8)
  - [`AtomicU8`](#atomicu8)
  - [`AtomicI16`](#atomici16)
  - [`AtomicU16`](#atomicu16)
  - [`AtomicI32`](#atomici32)
  - [`AtomicU32`](#atomicu32)
  - [`AtomicI64`](#atomici64)
  - [`AtomicU64`](#atomicu64)
- [Macros](#macros)
  - [`atomic_int!`](#atomic_int)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NotRefUnwindSafe`](#notrefunwindsafe) | struct |  |
| [`AtomicPtr`](#atomicptr) | struct |  |
| [`AtomicIsize`](#atomicisize) | struct |  |
| [`AtomicUsize`](#atomicusize) | struct |  |
| [`AtomicI8`](#atomici8) | struct |  |
| [`AtomicU8`](#atomicu8) | struct |  |
| [`AtomicI16`](#atomici16) | struct |  |
| [`AtomicU16`](#atomicu16) | struct |  |
| [`AtomicI32`](#atomici32) | struct |  |
| [`AtomicU32`](#atomicu32) | struct |  |
| [`AtomicI64`](#atomici64) | struct |  |
| [`AtomicU64`](#atomicu64) | struct |  |
| [`atomic_int!`](#atomic_int) | macro |  |

## Structs

### `NotRefUnwindSafe`

```rust
struct NotRefUnwindSafe(core::cell::UnsafeCell<()>);
```

#### Trait Implementations

##### `impl Sync for NotRefUnwindSafe`

### `AtomicPtr<T>`

```rust
struct AtomicPtr<T> {
    inner: core::sync::atomic::AtomicPtr<T>,
    _not_ref_unwind_safe: core::marker::PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="atomicptr-compare-exchange"></span>`fn compare_exchange(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>` — [`Ordering`](../../index.md)

- <span id="atomicptr-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: *mut T, new: *mut T, success: Ordering, failure: Ordering) -> Result<*mut T, *mut T>` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl<T> Deref for AtomicPtr<T>`

- <span id="atomicptr-target"></span>`type Target = AtomicPtr<T>`

- <span id="atomicptr-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicPtr<T>`

- <span id="atomicptr-target"></span>`type Target = T`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: core::sync::atomic::AtomicIsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="atomicisize-new"></span>`const fn new(v: isize) -> Self`

- <span id="atomicisize-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicisize-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicisize-load"></span>`fn load(&self, order: Ordering) -> isize` — [`Ordering`](../../index.md)

- <span id="atomicisize-store"></span>`fn store(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicisize-as-ptr"></span>`const fn as_ptr(&self) -> *mut isize`

#### Trait Implementations

##### `impl Deref for AtomicIsize`

- <span id="atomicisize-target"></span>`type Target = AtomicIsize`

- <span id="atomicisize-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicIsize`

- <span id="atomicisize-target"></span>`type Target = T`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: core::sync::atomic::AtomicUsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="atomicusize-compare-exchange"></span>`fn compare_exchange(&self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>` — [`Ordering`](../../index.md)

- <span id="atomicusize-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: usize, new: usize, success: Ordering, failure: Ordering) -> Result<usize, usize>` — [`Ordering`](../../index.md)

- <span id="atomicusize-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> usize` — [`Ordering`](../../index.md)

- <span id="atomicusize-fetch-max"></span>`fn fetch_max(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](../../index.md)

- <span id="atomicusize-fetch-min"></span>`fn fetch_min(&self, val: usize, order: Ordering) -> usize` — [`Ordering`](../../index.md)

- <span id="atomicusize-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> usize` — [`Ordering`](../../index.md)

- <span id="atomicusize-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> usize` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicUsize`

- <span id="atomicusize-target"></span>`type Target = AtomicUsize`

- <span id="atomicusize-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicUsize`

- <span id="atomicusize-target"></span>`type Target = T`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: core::sync::atomic::AtomicI8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="supercore-atomicatomici8-not"></span>`fn not(&self, _order: Ordering)` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomici8-neg"></span>`fn neg(&self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI8`

- <span id="atomici8-target"></span>`type Target = AtomicI8`

- <span id="atomici8-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI8`

- <span id="atomici8-target"></span>`type Target = T`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: core::sync::atomic::AtomicU8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="supercore-atomicatomicu8-not"></span>`fn not(&self, _order: Ordering)` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomicu8-neg"></span>`fn neg(&self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU8`

- <span id="atomicu8-target"></span>`type Target = AtomicU8`

- <span id="atomicu8-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU8`

- <span id="atomicu8-target"></span>`type Target = T`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: core::sync::atomic::AtomicI16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="atomici16-compare-exchange"></span>`fn compare_exchange(&self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` — [`Ordering`](../../index.md)

- <span id="atomici16-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i16, new: i16, success: Ordering, failure: Ordering) -> Result<i16, i16>` — [`Ordering`](../../index.md)

- <span id="atomici16-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> i16` — [`Ordering`](../../index.md)

- <span id="atomici16-fetch-max"></span>`fn fetch_max(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](../../index.md)

- <span id="atomici16-fetch-min"></span>`fn fetch_min(&self, val: i16, order: Ordering) -> i16` — [`Ordering`](../../index.md)

- <span id="atomici16-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i16` — [`Ordering`](../../index.md)

- <span id="atomici16-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i16` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI16`

- <span id="atomici16-target"></span>`type Target = AtomicI16`

- <span id="atomici16-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI16`

- <span id="atomici16-target"></span>`type Target = T`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: core::sync::atomic::AtomicU16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="supercore-atomicatomicu16-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomicu16-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomicu16-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU16`

- <span id="atomicu16-target"></span>`type Target = AtomicU16`

- <span id="atomicu16-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU16`

- <span id="atomicu16-target"></span>`type Target = T`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: core::sync::atomic::AtomicI32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="supercore-atomicatomici32-not"></span>`fn not(&self, _order: Ordering)` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomici32-neg"></span>`fn neg(&self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI32`

- <span id="atomici32-target"></span>`type Target = AtomicI32`

- <span id="atomici32-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI32`

- <span id="atomici32-target"></span>`type Target = T`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: core::sync::atomic::AtomicU32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="atomicu32-compare-exchange"></span>`fn compare_exchange(&self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` — [`Ordering`](../../index.md)

- <span id="atomicu32-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u32, new: u32, success: Ordering, failure: Ordering) -> Result<u32, u32>` — [`Ordering`](../../index.md)

- <span id="atomicu32-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> u32` — [`Ordering`](../../index.md)

- <span id="atomicu32-fetch-max"></span>`fn fetch_max(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](../../index.md)

- <span id="atomicu32-fetch-min"></span>`fn fetch_min(&self, val: u32, order: Ordering) -> u32` — [`Ordering`](../../index.md)

- <span id="atomicu32-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u32` — [`Ordering`](../../index.md)

- <span id="atomicu32-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u32` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU32`

- <span id="atomicu32-target"></span>`type Target = AtomicU32`

- <span id="atomicu32-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU32`

- <span id="atomicu32-target"></span>`type Target = T`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: core::sync::atomic::AtomicI64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="atomici64-compare-exchange"></span>`fn compare_exchange(&self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` — [`Ordering`](../../index.md)

- <span id="atomici64-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i64, new: i64, success: Ordering, failure: Ordering) -> Result<i64, i64>` — [`Ordering`](../../index.md)

- <span id="atomici64-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> i64` — [`Ordering`](../../index.md)

- <span id="atomici64-fetch-max"></span>`fn fetch_max(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](../../index.md)

- <span id="atomici64-fetch-min"></span>`fn fetch_min(&self, val: i64, order: Ordering) -> i64` — [`Ordering`](../../index.md)

- <span id="atomici64-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i64` — [`Ordering`](../../index.md)

- <span id="atomici64-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i64` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI64`

- <span id="atomici64-target"></span>`type Target = AtomicI64`

- <span id="atomici64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI64`

- <span id="atomici64-target"></span>`type Target = T`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: core::sync::atomic::AtomicU64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- <span id="supercore-atomicatomicu64-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomicu64-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- <span id="supercore-atomicatomicu64-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU64`

- <span id="atomicu64-target"></span>`type Target = AtomicU64`

- <span id="atomicu64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU64`

- <span id="atomicu64-target"></span>`type Target = T`

## Macros

### `atomic_int!`

