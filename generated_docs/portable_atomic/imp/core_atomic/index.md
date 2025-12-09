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

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:22`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L22)*

#### Trait Implementations

##### `impl Sync for NotRefUnwindSafe`

### `AtomicPtr<T>`

```rust
struct AtomicPtr<T> {
    inner: core::sync::atomic::AtomicPtr<T>,
    _not_ref_unwind_safe: core::marker::PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:27-31`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L27-L31)*

#### Implementations

- <span id="atomicptr-new"></span>`const fn new(v: *mut T) -> Self`

- <span id="atomicptr-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicptr-const-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicptr-load"></span>`fn load(&self, order: Ordering) -> *mut T` — [`Ordering`](../../index.md)

- <span id="atomicptr-store"></span>`fn store(&self, ptr: *mut T, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicptr-as-ptr"></span>`const fn as_ptr(&self) -> *mut *mut T`

#### Trait Implementations

##### `impl<T> Deref for AtomicPtr<T>`

- <span id="atomicptr-type-target"></span>`type Target = AtomicPtr<T>`

- <span id="atomicptr-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicPtr<T>`

- <span id="atomicptr-type-target"></span>`type Target = T`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: core::sync::atomic::AtomicIsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:398`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L398)*

#### Implementations

- <span id="atomicisize-add"></span>`fn add(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicisize-sub"></span>`fn sub(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicisize-and"></span>`fn and(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicisize-or"></span>`fn or(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicisize-xor"></span>`fn xor(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicIsize`

- <span id="atomicisize-type-target"></span>`type Target = AtomicIsize`

- <span id="atomicisize-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicIsize`

- <span id="atomicisize-type-target"></span>`type Target = T`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: core::sync::atomic::AtomicUsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:399`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L399)*

#### Implementations

- <span id="atomicusize-add"></span>`fn add(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicusize-sub"></span>`fn sub(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicusize-and"></span>`fn and(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicusize-or"></span>`fn or(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicusize-xor"></span>`fn xor(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicUsize`

- <span id="atomicusize-type-target"></span>`type Target = AtomicUsize`

- <span id="atomicusize-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicUsize`

- <span id="atomicusize-type-target"></span>`type Target = T`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: core::sync::atomic::AtomicI8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:401`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L401)*

#### Implementations

- <span id="atomici8-add"></span>`fn add(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici8-sub"></span>`fn sub(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici8-and"></span>`fn and(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici8-or"></span>`fn or(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici8-xor"></span>`fn xor(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI8`

- <span id="atomici8-type-target"></span>`type Target = AtomicI8`

- <span id="atomici8-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicI8`

- <span id="atomici8-type-target"></span>`type Target = T`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: core::sync::atomic::AtomicU8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:403`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L403)*

#### Implementations

- <span id="atomicu8-add"></span>`fn add(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu8-sub"></span>`fn sub(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu8-and"></span>`fn and(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu8-or"></span>`fn or(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu8-xor"></span>`fn xor(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU8`

- <span id="atomicu8-type-target"></span>`type Target = AtomicU8`

- <span id="atomicu8-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicU8`

- <span id="atomicu8-type-target"></span>`type Target = T`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: core::sync::atomic::AtomicI16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:405`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L405)*

#### Implementations

- <span id="atomici16-add"></span>`fn add(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici16-sub"></span>`fn sub(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici16-and"></span>`fn and(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici16-or"></span>`fn or(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici16-xor"></span>`fn xor(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI16`

- <span id="atomici16-type-target"></span>`type Target = AtomicI16`

- <span id="atomici16-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicI16`

- <span id="atomici16-type-target"></span>`type Target = T`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: core::sync::atomic::AtomicU16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:407`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L407)*

#### Implementations

- <span id="atomicu16-add"></span>`fn add(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-sub"></span>`fn sub(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-and"></span>`fn and(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-or"></span>`fn or(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu16-xor"></span>`fn xor(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU16`

- <span id="atomicu16-type-target"></span>`type Target = AtomicU16`

- <span id="atomicu16-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicU16`

- <span id="atomicu16-type-target"></span>`type Target = T`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: core::sync::atomic::AtomicI32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:410`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L410)*

#### Implementations

- <span id="atomici32-add"></span>`fn add(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-sub"></span>`fn sub(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-and"></span>`fn and(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-or"></span>`fn or(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici32-xor"></span>`fn xor(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI32`

- <span id="atomici32-type-target"></span>`type Target = AtomicI32`

- <span id="atomici32-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicI32`

- <span id="atomici32-type-target"></span>`type Target = T`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: core::sync::atomic::AtomicU32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:413`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L413)*

#### Implementations

- <span id="atomicu32-add"></span>`fn add(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-sub"></span>`fn sub(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-and"></span>`fn and(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-or"></span>`fn or(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu32-xor"></span>`fn xor(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU32`

- <span id="atomicu32-type-target"></span>`type Target = AtomicU32`

- <span id="atomicu32-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicU32`

- <span id="atomicu32-type-target"></span>`type Target = T`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: core::sync::atomic::AtomicI64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:422`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L422)*

#### Implementations

- <span id="atomici64-add"></span>`fn add(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici64-sub"></span>`fn sub(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici64-and"></span>`fn and(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici64-or"></span>`fn or(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici64-xor"></span>`fn xor(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI64`

- <span id="atomici64-type-target"></span>`type Target = AtomicI64`

- <span id="atomici64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicI64`

- <span id="atomici64-type-target"></span>`type Target = T`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: core::sync::atomic::AtomicU64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:431`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L431)*

#### Implementations

- <span id="atomicu64-add"></span>`fn add(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu64-sub"></span>`fn sub(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu64-and"></span>`fn and(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu64-or"></span>`fn or(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu64-xor"></span>`fn xor(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU64`

- <span id="atomicu64-type-target"></span>`type Target = AtomicU64`

- <span id="atomicu64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for AtomicU64`

- <span id="atomicu64-type-target"></span>`type Target = T`

## Macros

### `atomic_int!`

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:118-396`](../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/core_atomic.rs#L118-L396)*

