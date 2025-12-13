*[crossbeam_utils](../../index.md) / [atomic](../index.md) / [seq_lock](index.md)*

---

# Module `seq_lock`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SeqLock`](#seqlock) | struct | A simple stamped lock. |
| [`SeqLockWriteGuard`](#seqlockwriteguard) | struct | An RAII guard that releases the lock and increments the stamp when dropped. |

## Structs

### `SeqLock`

```rust
struct SeqLock {
    state: core::sync::atomic::AtomicUsize,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/seq_lock.rs:7-13`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/seq_lock.rs#L7-L13)*

A simple stamped lock.

#### Fields

- **`state`**: `core::sync::atomic::AtomicUsize`

  The current state of the lock.
  
  All bits except the least significant one hold the current stamp. When locked, the state
  equals 1 and doesn't contain a valid stamp.

#### Implementations

- <span id="seqlock-new"></span>`const fn new() -> Self`

- <span id="seqlock-optimistic-read"></span>`fn optimistic_read(&self) -> Option<usize>`

  If not locked, returns the current stamp.

  

  This method should be called before optimistic reads.

- <span id="seqlock-validate-read"></span>`fn validate_read(&self, stamp: usize) -> bool`

  Returns `true` if the current stamp is equal to `stamp`.

  

  This method should be called after optimistic reads to check whether they are valid. The

  argument `stamp` should correspond to the one returned by method `optimistic_read`.

- <span id="seqlock-write"></span>`fn write(self: &'static Self) -> SeqLockWriteGuard` — [`SeqLockWriteGuard`](#seqlockwriteguard)

  Grabs the lock for writing.

#### Trait Implementations

##### `impl Any for SeqLock`

- <span id="seqlock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeqLock`

- <span id="seqlock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeqLock`

- <span id="seqlock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SeqLock`

- <span id="seqlock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeqLock`

- <span id="seqlock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SeqLock`

- <span id="seqlock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seqlock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeqLock`

- <span id="seqlock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seqlock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SeqLockWriteGuard`

```rust
struct SeqLockWriteGuard {
    lock: &'static SeqLock,
    state: usize,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/atomic/seq_lock.rs:67-73`](../../../../.source_1765633015/crossbeam-utils-0.8.21/src/atomic/seq_lock.rs#L67-L73)*

An RAII guard that releases the lock and increments the stamp when dropped.

#### Fields

- **`lock`**: `&'static SeqLock`

  The parent lock.

- **`state`**: `usize`

  The stamp before locking.

#### Implementations

- <span id="seqlockwriteguard-abort"></span>`fn abort(self)`

  Releases the lock without incrementing the stamp.

#### Trait Implementations

##### `impl Any for SeqLockWriteGuard`

- <span id="seqlockwriteguard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeqLockWriteGuard`

- <span id="seqlockwriteguard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeqLockWriteGuard`

- <span id="seqlockwriteguard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for SeqLockWriteGuard`

- <span id="seqlockwriteguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for SeqLockWriteGuard`

- <span id="seqlockwriteguard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeqLockWriteGuard`

- <span id="seqlockwriteguard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SeqLockWriteGuard`

- <span id="seqlockwriteguard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seqlockwriteguard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeqLockWriteGuard`

- <span id="seqlockwriteguard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seqlockwriteguard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

