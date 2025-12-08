*[portable_atomic](../../index.md) / [imp](../index.md) / [core_atomic](index.md)*

---

# Module `core_atomic`

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

- `const fn new(v: *mut T) -> Self`

- `fn is_lock_free() -> bool`

- `const IS_ALWAYS_LOCK_FREE: bool`

- `fn load(self: &Self, order: Ordering) -> *mut T` — [`Ordering`](../../index.md)

- `fn store(self: &Self, ptr: *mut T, order: Ordering)` — [`Ordering`](../../index.md)

- `const fn as_ptr(self: &Self) -> *mut *mut T`

#### Trait Implementations

##### `impl<T> Deref for AtomicPtr<T>`

- `type Target = AtomicPtr<T>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicPtr<T>`

- `type Target = T`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: core::sync::atomic::AtomicIsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `const fn new(v: isize) -> Self`

- `fn is_lock_free() -> bool`

- `const IS_ALWAYS_LOCK_FREE: bool`

- `fn load(self: &Self, order: Ordering) -> isize` — [`Ordering`](../../index.md)

- `fn store(self: &Self, val: isize, order: Ordering)` — [`Ordering`](../../index.md)

- `const fn as_ptr(self: &Self) -> *mut isize`

#### Trait Implementations

##### `impl Deref for AtomicIsize`

- `type Target = AtomicIsize`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicIsize`

- `type Target = T`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: core::sync::atomic::AtomicUsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn not(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicUsize`

- `type Target = AtomicUsize`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicUsize`

- `type Target = T`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: core::sync::atomic::AtomicI8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn not(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI8`

- `type Target = AtomicI8`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI8`

- `type Target = T`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: core::sync::atomic::AtomicU8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn compare_exchange(self: &Self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](../../index.md)

- `fn compare_exchange_weak(self: &Self, current: u8, new: u8, success: Ordering, failure: Ordering) -> Result<u8, u8>` — [`Ordering`](../../index.md)

- `fn fetch_update_<F>(self: &Self, order: Ordering, f: F) -> u8` — [`Ordering`](../../index.md)

- `fn fetch_max(self: &Self, val: u8, order: Ordering) -> u8` — [`Ordering`](../../index.md)

- `fn fetch_min(self: &Self, val: u8, order: Ordering) -> u8` — [`Ordering`](../../index.md)

- `fn fetch_not(self: &Self, order: Ordering) -> u8` — [`Ordering`](../../index.md)

- `fn fetch_neg(self: &Self, order: Ordering) -> u8` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU8`

- `type Target = AtomicU8`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU8`

- `type Target = T`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: core::sync::atomic::AtomicI16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn not(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI16`

- `type Target = AtomicI16`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI16`

- `type Target = T`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: core::sync::atomic::AtomicU16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn bit_set(self: &Self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- `fn bit_clear(self: &Self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- `fn bit_toggle(self: &Self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU16`

- `type Target = AtomicU16`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU16`

- `type Target = T`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: core::sync::atomic::AtomicI32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn not(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI32`

- `type Target = AtomicI32`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI32`

- `type Target = T`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: core::sync::atomic::AtomicU32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn not(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicU32`

- `type Target = AtomicU32`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU32`

- `type Target = T`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: core::sync::atomic::AtomicI64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `fn not(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, _order: Ordering)` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Deref for AtomicI64`

- `type Target = AtomicI64`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicI64`

- `type Target = T`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: core::sync::atomic::AtomicU64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

#### Implementations

- `const fn new(v: u64) -> Self`

- `fn is_lock_free() -> bool`

- `const IS_ALWAYS_LOCK_FREE: bool`

- `fn load(self: &Self, order: Ordering) -> u64` — [`Ordering`](../../index.md)

- `fn store(self: &Self, val: u64, order: Ordering)` — [`Ordering`](../../index.md)

- `const fn as_ptr(self: &Self) -> *mut u64`

#### Trait Implementations

##### `impl Deref for AtomicU64`

- `type Target = AtomicU64`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for AtomicU64`

- `type Target = T`

## Macros

### `atomic_int!`

