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

- <span id="atomicptr-new"></span>`const fn new(v: *mut T) -> Self`

- <span id="atomicptr-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicptr-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicptr-load"></span>`fn load(&self, order: Ordering) -> *mut T` — [`Ordering`](../../index.md)

- <span id="atomicptr-store"></span>`fn store(&self, ptr: *mut T, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicptr-as-ptr"></span>`const fn as_ptr(&self) -> *mut *mut T`

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

- <span id="atomicisize-compare-exchange"></span>`fn compare_exchange(&self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>` — [`Ordering`](../../index.md)

- <span id="atomicisize-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: isize, new: isize, success: Ordering, failure: Ordering) -> Result<isize, isize>` — [`Ordering`](../../index.md)

- <span id="atomicisize-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> isize` — [`Ordering`](../../index.md)

- <span id="atomicisize-fetch-max"></span>`fn fetch_max(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](../../index.md)

- <span id="atomicisize-fetch-min"></span>`fn fetch_min(&self, val: isize, order: Ordering) -> isize` — [`Ordering`](../../index.md)

- <span id="atomicisize-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> isize` — [`Ordering`](../../index.md)

- <span id="atomicisize-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> isize` — [`Ordering`](../../index.md)

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

- <span id="atomicusize-new"></span>`const fn new(v: usize) -> Self`

- <span id="atomicusize-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicusize-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicusize-load"></span>`fn load(&self, order: Ordering) -> usize` — [`Ordering`](../../index.md)

- <span id="atomicusize-store"></span>`fn store(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicusize-as-ptr"></span>`const fn as_ptr(&self) -> *mut usize`

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

- <span id="atomici8-compare-exchange"></span>`fn compare_exchange(&self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>` — [`Ordering`](../../index.md)

- <span id="atomici8-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i8, new: i8, success: Ordering, failure: Ordering) -> Result<i8, i8>` — [`Ordering`](../../index.md)

- <span id="atomici8-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> i8` — [`Ordering`](../../index.md)

- <span id="atomici8-fetch-max"></span>`fn fetch_max(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](../../index.md)

- <span id="atomici8-fetch-min"></span>`fn fetch_min(&self, val: i8, order: Ordering) -> i8` — [`Ordering`](../../index.md)

- <span id="atomici8-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> i8` — [`Ordering`](../../index.md)

- <span id="atomici8-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> i8` — [`Ordering`](../../index.md)

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

- <span id="atomicu8-compare-exchange"></span>`fn compare_exchange(&self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](../../index.md)

- <span id="atomicu8-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](../../index.md)

- <span id="atomicu8-fetch-update"></span>`fn fetch_update_<F>(&self, order: Ordering, f: F) -> u8` — [`Ordering`](../../index.md)

- <span id="atomicu8-fetch-max"></span>`fn fetch_max(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](../../index.md)

- <span id="atomicu8-fetch-min"></span>`fn fetch_min(&self, val: u8, order: Ordering) -> u8` — [`Ordering`](../../index.md)

- <span id="atomicu8-fetch-not"></span>`fn fetch_not(&self, order: Ordering) -> u8` — [`Ordering`](../../index.md)

- <span id="atomicu8-fetch-neg"></span>`fn fetch_neg(&self, order: Ordering) -> u8` — [`Ordering`](../../index.md)

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

- <span id="atomicu16-add"></span>`fn add(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-sub"></span>`fn sub(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-and"></span>`fn and(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-or"></span>`fn or(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-xor"></span>`fn xor(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

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

- <span id="atomici32-add"></span>`fn add(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-sub"></span>`fn sub(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-and"></span>`fn and(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-or"></span>`fn or(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-xor"></span>`fn xor(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

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

- <span id="atomicu32-add"></span>`fn add(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-sub"></span>`fn sub(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-and"></span>`fn and(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-or"></span>`fn or(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-xor"></span>`fn xor(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

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

- <span id="atomici64-new"></span>`const fn new(v: i64) -> Self`

- <span id="atomici64-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici64-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomici64-load"></span>`fn load(&self, order: Ordering) -> i64` — [`Ordering`](../../index.md)

- <span id="atomici64-store"></span>`fn store(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici64-as-ptr"></span>`const fn as_ptr(&self) -> *mut i64`

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

- <span id="atomicu64-new"></span>`const fn new(v: u64) -> Self`

- <span id="atomicu64-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu64-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicu64-load"></span>`fn load(&self, order: Ordering) -> u64` — [`Ordering`](../../index.md)

- <span id="atomicu64-store"></span>`fn store(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu64-as-ptr"></span>`const fn as_ptr(&self) -> *mut u64`

#### Trait Implementations

##### `impl Deref for AtomicU64`

- <span id="atomicu64-target"></span>`type Target = AtomicU64`

- <span id="atomicu64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU64`

- <span id="atomicu64-target"></span>`type Target = T`

## Macros

### `atomic_int!`

