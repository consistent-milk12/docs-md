*[backtrace](../index.md) / [lock](index.md)*

---

# Module `lock`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LockGuard`](#lockguard) | struct | A "Maybe" LockGuard |
| [`lock`](#lock) | fn | Acquire a partially unsound(!!!) global re-entrant lock over backtrace's internals. |
| [`LOCK_HELD`](#lock-held) | const |  |

## Structs

### `LockGuard`

```rust
struct LockGuard(Option<std::sync::MutexGuard<'static, ()>>);
```

*Defined in [`backtrace-0.3.76/src/lib.rs:145`](../../../.source_1765633015/backtrace-0.3.76/src/lib.rs#L145)*

A "Maybe" LockGuard

#### Trait Implementations

##### `impl Any for LockGuard`

- <span id="lockguard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LockGuard`

- <span id="lockguard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LockGuard`

- <span id="lockguard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for LockGuard`

- <span id="lockguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for LockGuard`

- <span id="lockguard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LockGuard`

- <span id="lockguard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LockGuard`

- <span id="lockguard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lockguard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LockGuard`

- <span id="lockguard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lockguard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `lock`

```rust
fn lock() -> LockGuard
```

*Defined in [`backtrace-0.3.76/src/lib.rs:217-228`](../../../.source_1765633015/backtrace-0.3.76/src/lib.rs#L217-L228)*

Acquire a partially unsound(!!!) global re-entrant lock over
backtrace's internals.

That is, this lock can be acquired as many times as you want
on a single thread without deadlocking, allowing one thread
to acquire exclusive access to the ability to make backtraces.
Calls to this locking function are freely sprinkled in every place
where that needs to be enforced.


# Why

This was first introduced to guard uses of Windows' dbghelp API,
which isn't threadsafe. It's unclear if other things now rely on
this locking.


# How

The basic idea is to have a single global mutex, and a thread_local
boolean saying "yep this is the thread that acquired the mutex".

The first time a thread acquires the lock, it is handed a
`LockGuard(Some(..))` that will actually release the lock on Drop.
All subsequence attempts to lock on the same thread will see
that their thread acquired the lock, and get `LockGuard(None)`
which will do nothing when dropped.


# Safety

As long as you only ever assign the returned LockGuard to a freshly
declared local variable, it will do its job correctly, as the "first"
LockGuard will strictly outlive all subsequent LockGuards and
properly release the lock when the thread is done with backtracing.

However if you ever attempt to store a LockGuard beyond the scope
it was acquired in, it might actually be a `LockGuard(None)` that
doesn't actually hold the lock! In this case another thread might
acquire the lock and you'll get races this system was intended to
avoid!

This is why this is "partially unsound". As a public API this would
be unacceptable, but this is crate-private, and if you use this in
the most obvious and simplistic way it Just Works™.

Note however that std specifically bypasses this lock, and uses
the `*_unsynchronized` backtrace APIs. This is "fine" because
it wraps its own calls to backtrace in a non-reentrant Mutex
that prevents two backtraces from getting interleaved during printing.

## Constants

### `LOCK_HELD`
```rust
const LOCK_HELD: thread::LocalKey<std::cell::Cell<bool>>;
```

*Defined in [`backtrace-0.3.76/src/lib.rs:150`](../../../.source_1765633015/backtrace-0.3.76/src/lib.rs#L150)*

