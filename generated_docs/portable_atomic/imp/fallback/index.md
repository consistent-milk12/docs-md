*[portable_atomic](../../index.md) / [imp](../index.md) / [fallback](index.md)*

---

# Module `fallback`

## Contents

- [Modules](#modules)
  - [`utils`](#utils)
  - [`seq_lock`](#seq-lock)
- [Structs](#structs)
  - [`AtomicI128`](#atomici128)
  - [`AtomicU128`](#atomicu128)
- [Functions](#functions)
  - [`lock`](#lock)
- [Macros](#macros)
  - [`atomic!`](#atomic)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`utils`](#utils) | mod |  |
| [`seq_lock`](#seq-lock) | mod |  |
| [`AtomicI128`](#atomici128) | struct |  |
| [`AtomicU128`](#atomicu128) | struct |  |
| [`lock`](#lock) | fn |  |
| [`atomic!`](#atomic) | macro |  |

## Modules

- [`utils`](utils/index.md)
- [`seq_lock`](seq_lock/index.md)

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: UnsafeCell<i128>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:451`](../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L451)*

#### Implementations

- <span id="atomici128-const-len"></span>`const LEN: usize`

- <span id="atomici128-chunks"></span>`unsafe fn chunks(&self) -> &[core::sync::atomic::AtomicUsize; 2]`

- <span id="atomici128-optimistic-read"></span>`fn optimistic_read(&self) -> i128`

- <span id="atomici128-read"></span>`fn read(&self, _guard: &SeqLockWriteGuard<'static>) -> i128` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

- <span id="atomici128-write"></span>`fn write(&self, val: i128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

#### Trait Implementations

##### `impl Any for AtomicI128`

- <span id="atomici128-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI128`

- <span id="atomici128-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI128`

- <span id="atomici128-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicI128`

- <span id="atomici128-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI128`

- <span id="atomici128-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for AtomicI128`

##### `impl<U> TryFrom for AtomicI128`

- <span id="atomici128-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici128-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI128`

- <span id="atomici128-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici128-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: UnsafeCell<u128>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:452`](../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L452)*

#### Implementations

- <span id="atomicu128-const-len"></span>`const LEN: usize`

- <span id="atomicu128-chunks"></span>`unsafe fn chunks(&self) -> &[core::sync::atomic::AtomicUsize; 2]`

- <span id="atomicu128-optimistic-read"></span>`fn optimistic_read(&self) -> u128`

- <span id="atomicu128-read"></span>`fn read(&self, _guard: &SeqLockWriteGuard<'static>) -> u128` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

- <span id="atomicu128-write"></span>`fn write(&self, val: u128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md#seqlockwriteguard)

#### Trait Implementations

##### `impl Any for AtomicU128`

- <span id="atomicu128-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU128`

- <span id="atomicu128-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU128`

- <span id="atomicu128-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicU128`

- <span id="atomicu128-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU128`

- <span id="atomicu128-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for AtomicU128`

##### `impl<U> TryFrom for AtomicU128`

- <span id="atomicu128-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu128-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU128`

- <span id="atomicu128-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu128-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `lock`

```rust
fn lock(addr: usize) -> &'static self::seq_lock::SeqLock
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:138-155`](../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L138-L155)*

## Macros

### `atomic!`

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/mod.rs:157-414`](../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/mod.rs#L157-L414)*

