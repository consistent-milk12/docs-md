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
  - [`atomic_int!`](#atomic-int)

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
| [`atomic_int!`](#atomic-int) | macro |  |

## Structs

### `NotRefUnwindSafe`

```rust
struct NotRefUnwindSafe(core::cell::UnsafeCell<()>);
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:22`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L22)*

#### Trait Implementations

##### `impl Any for NotRefUnwindSafe`

- <span id="notrefunwindsafe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NotRefUnwindSafe`

- <span id="notrefunwindsafe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NotRefUnwindSafe`

- <span id="notrefunwindsafe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for NotRefUnwindSafe`

- <span id="notrefunwindsafe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NotRefUnwindSafe`

- <span id="notrefunwindsafe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for NotRefUnwindSafe`

##### `impl<U> TryFrom for NotRefUnwindSafe`

- <span id="notrefunwindsafe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="notrefunwindsafe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NotRefUnwindSafe`

- <span id="notrefunwindsafe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="notrefunwindsafe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicPtr<T>`

```rust
struct AtomicPtr<T> {
    inner: core::sync::atomic::AtomicPtr<T>,
    _not_ref_unwind_safe: core::marker::PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:27-31`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L27-L31)*

#### Implementations

- <span id="atomicptr-new"></span>`const fn new(v: *mut T) -> Self`

- <span id="atomicptr-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicptr-const-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicptr-load"></span>`fn load(&self, order: Ordering) -> *mut T` — [`Ordering`](../../index.md#ordering)

- <span id="atomicptr-store"></span>`fn store(&self, ptr: *mut T, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicptr-as-ptr"></span>`const fn as_ptr(&self) -> *mut *mut T`

#### Trait Implementations

##### `impl<T> Any for AtomicPtr<T>`

- <span id="atomicptr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicPtr<T>`

- <span id="atomicptr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicPtr<T>`

- <span id="atomicptr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Deref for AtomicPtr<T>`

- <span id="atomicptr-deref-type-target"></span>`type Target = AtomicPtr<T>`

- <span id="atomicptr-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicPtr<T>`

- <span id="atomicptr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AtomicPtr<T>`

- <span id="atomicptr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for AtomicPtr<T>`

- <span id="atomicptr-receiver-type-target"></span>`type Target = T`

##### `impl<T, U> TryFrom for AtomicPtr<T>`

- <span id="atomicptr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicptr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AtomicPtr<T>`

- <span id="atomicptr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicptr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicIsize`

```rust
struct AtomicIsize {
    inner: core::sync::atomic::AtomicIsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:398`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L398)*

#### Implementations

- <span id="atomicisize-add"></span>`fn add(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicisize-sub"></span>`fn sub(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicisize-and"></span>`fn and(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicisize-or"></span>`fn or(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicisize-xor"></span>`fn xor(&self, val: isize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicIsize`

- <span id="atomicisize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicIsize`

- <span id="atomicisize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicIsize`

- <span id="atomicisize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicIsize`

- <span id="atomicisize-deref-type-target"></span>`type Target = AtomicIsize`

- <span id="atomicisize-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicIsize`

- <span id="atomicisize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicIsize`

- <span id="atomicisize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicIsize`

- <span id="atomicisize-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicIsize`

- <span id="atomicisize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicisize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicIsize`

- <span id="atomicisize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicisize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicUsize`

```rust
struct AtomicUsize {
    inner: core::sync::atomic::AtomicUsize,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:399`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L399)*

#### Implementations

- <span id="atomicusize-add"></span>`fn add(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicusize-sub"></span>`fn sub(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicusize-and"></span>`fn and(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicusize-or"></span>`fn or(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicusize-xor"></span>`fn xor(&self, val: usize, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicUsize`

- <span id="atomicusize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicUsize`

- <span id="atomicusize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicUsize`

- <span id="atomicusize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicUsize`

- <span id="atomicusize-deref-type-target"></span>`type Target = AtomicUsize`

- <span id="atomicusize-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicUsize`

- <span id="atomicusize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicUsize`

- <span id="atomicusize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicUsize`

- <span id="atomicusize-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicUsize`

- <span id="atomicusize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicusize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicUsize`

- <span id="atomicusize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicusize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI8`

```rust
struct AtomicI8 {
    inner: core::sync::atomic::AtomicI8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:401`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L401)*

#### Implementations

- <span id="atomici8-add"></span>`fn add(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici8-sub"></span>`fn sub(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici8-and"></span>`fn and(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici8-or"></span>`fn or(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici8-xor"></span>`fn xor(&self, val: i8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicI8`

- <span id="atomici8-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI8`

- <span id="atomici8-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI8`

- <span id="atomici8-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicI8`

- <span id="atomici8-deref-type-target"></span>`type Target = AtomicI8`

- <span id="atomici8-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicI8`

- <span id="atomici8-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI8`

- <span id="atomici8-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicI8`

- <span id="atomici8-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicI8`

- <span id="atomici8-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici8-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI8`

- <span id="atomici8-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici8-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU8`

```rust
struct AtomicU8 {
    inner: core::sync::atomic::AtomicU8,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:403`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L403)*

#### Implementations

- <span id="atomicu8-add"></span>`fn add(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu8-sub"></span>`fn sub(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu8-and"></span>`fn and(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu8-or"></span>`fn or(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu8-xor"></span>`fn xor(&self, val: u8, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicU8`

- <span id="atomicu8-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU8`

- <span id="atomicu8-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU8`

- <span id="atomicu8-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicU8`

- <span id="atomicu8-deref-type-target"></span>`type Target = AtomicU8`

- <span id="atomicu8-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicU8`

- <span id="atomicu8-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU8`

- <span id="atomicu8-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicU8`

- <span id="atomicu8-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicU8`

- <span id="atomicu8-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu8-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU8`

- <span id="atomicu8-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu8-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI16`

```rust
struct AtomicI16 {
    inner: core::sync::atomic::AtomicI16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:405`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L405)*

#### Implementations

- <span id="atomici16-add"></span>`fn add(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici16-sub"></span>`fn sub(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici16-and"></span>`fn and(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici16-or"></span>`fn or(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici16-xor"></span>`fn xor(&self, val: i16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicI16`

- <span id="atomici16-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI16`

- <span id="atomici16-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI16`

- <span id="atomici16-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicI16`

- <span id="atomici16-deref-type-target"></span>`type Target = AtomicI16`

- <span id="atomici16-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicI16`

- <span id="atomici16-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI16`

- <span id="atomici16-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicI16`

- <span id="atomici16-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicI16`

- <span id="atomici16-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici16-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI16`

- <span id="atomici16-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici16-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU16`

```rust
struct AtomicU16 {
    inner: core::sync::atomic::AtomicU16,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:407`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L407)*

#### Implementations

- <span id="atomicu16-add"></span>`fn add(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu16-sub"></span>`fn sub(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu16-and"></span>`fn and(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu16-or"></span>`fn or(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu16-xor"></span>`fn xor(&self, val: u16, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicU16`

- <span id="atomicu16-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU16`

- <span id="atomicu16-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU16`

- <span id="atomicu16-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicU16`

- <span id="atomicu16-deref-type-target"></span>`type Target = AtomicU16`

- <span id="atomicu16-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicU16`

- <span id="atomicu16-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU16`

- <span id="atomicu16-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicU16`

- <span id="atomicu16-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicU16`

- <span id="atomicu16-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu16-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU16`

- <span id="atomicu16-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu16-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI32`

```rust
struct AtomicI32 {
    inner: core::sync::atomic::AtomicI32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:410`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L410)*

#### Implementations

- <span id="atomici32-add"></span>`fn add(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici32-sub"></span>`fn sub(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici32-and"></span>`fn and(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici32-or"></span>`fn or(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici32-xor"></span>`fn xor(&self, val: i32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicI32`

- <span id="atomici32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI32`

- <span id="atomici32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI32`

- <span id="atomici32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicI32`

- <span id="atomici32-deref-type-target"></span>`type Target = AtomicI32`

- <span id="atomici32-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicI32`

- <span id="atomici32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI32`

- <span id="atomici32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicI32`

- <span id="atomici32-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicI32`

- <span id="atomici32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI32`

- <span id="atomici32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU32`

```rust
struct AtomicU32 {
    inner: core::sync::atomic::AtomicU32,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:413`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L413)*

#### Implementations

- <span id="atomicu32-add"></span>`fn add(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu32-sub"></span>`fn sub(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu32-and"></span>`fn and(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu32-or"></span>`fn or(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu32-xor"></span>`fn xor(&self, val: u32, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicU32`

- <span id="atomicu32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU32`

- <span id="atomicu32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU32`

- <span id="atomicu32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicU32`

- <span id="atomicu32-deref-type-target"></span>`type Target = AtomicU32`

- <span id="atomicu32-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicU32`

- <span id="atomicu32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU32`

- <span id="atomicu32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicU32`

- <span id="atomicu32-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicU32`

- <span id="atomicu32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU32`

- <span id="atomicu32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicI64`

```rust
struct AtomicI64 {
    inner: core::sync::atomic::AtomicI64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:422`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L422)*

#### Implementations

- <span id="atomici64-add"></span>`fn add(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici64-sub"></span>`fn sub(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici64-and"></span>`fn and(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici64-or"></span>`fn or(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomici64-xor"></span>`fn xor(&self, val: i64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicI64`

- <span id="atomici64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI64`

- <span id="atomici64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI64`

- <span id="atomici64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicI64`

- <span id="atomici64-deref-type-target"></span>`type Target = AtomicI64`

- <span id="atomici64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicI64`

- <span id="atomici64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI64`

- <span id="atomici64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicI64`

- <span id="atomici64-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicI64`

- <span id="atomici64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI64`

- <span id="atomici64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU64`

```rust
struct AtomicU64 {
    inner: core::sync::atomic::AtomicU64,
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:431`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L431)*

#### Implementations

- <span id="atomicu64-add"></span>`fn add(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu64-sub"></span>`fn sub(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu64-and"></span>`fn and(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu64-or"></span>`fn or(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

- <span id="atomicu64-xor"></span>`fn xor(&self, val: u64, order: Ordering)` — [`Ordering`](../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicU64`

- <span id="atomicu64-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU64`

- <span id="atomicu64-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU64`

- <span id="atomicu64-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for AtomicU64`

- <span id="atomicu64-deref-type-target"></span>`type Target = AtomicU64`

- <span id="atomicu64-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for AtomicU64`

- <span id="atomicu64-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU64`

- <span id="atomicu64-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for AtomicU64`

- <span id="atomicu64-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for AtomicU64`

- <span id="atomicu64-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu64-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU64`

- <span id="atomicu64-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu64-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `atomic_int!`

*Defined in [`portable-atomic-1.11.1/src/imp/core_atomic.rs:118-396`](../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/core_atomic.rs#L118-L396)*

