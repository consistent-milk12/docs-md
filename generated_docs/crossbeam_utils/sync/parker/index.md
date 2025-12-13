*[crossbeam_utils](../../index.md) / [sync](../index.md) / [parker](index.md)*

---

# Module `parker`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parker`](#parker) | struct | A thread parking primitive. |
| [`Unparker`](#unparker) | struct | Unparks a thread parked by the associated [`Parker`]. |
| [`Inner`](#inner) | struct |  |
| [`EMPTY`](#empty) | const |  |
| [`PARKED`](#parked) | const |  |
| [`NOTIFIED`](#notified) | const |  |

## Structs

### `Parker`

```rust
struct Parker {
    unparker: Unparker,
    _marker: std::marker::PhantomData<*const ()>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:53-56`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L53-L56)*

A thread parking primitive.

Conceptually, each `Parker` has an associated token which is initially not present:

* The `park` method blocks the current thread unless or until the token is available, at
  which point it automatically consumes the token.

* The `park_timeout` and `park_deadline` methods work the same as `park`, but block for
  a specified maximum time.

* The `unpark` method atomically makes the token available if it wasn't already. Because the
  token is initially absent, `unpark` followed by `park` will result in the second call
  returning immediately.

In other words, each `Parker` acts a bit like a spinlock that can be locked and unlocked using
`park` and `unpark`.

# Examples

```rust
use std::thread;
use std::time::Duration;
use crossbeam_utils::sync::Parker;

let p = Parker::new();
let u = p.unparker().clone();

// Make the token available.
u.unpark();
// Wakes up immediately and consumes the token.
p.park();

thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    u.unpark();
});

// Wakes up when `u.unpark()` provides the token.
p.park();
std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371
```





#### Implementations

- <span id="parker-new"></span>`fn new() -> Parker` — [`Parker`](#parker)

  Creates a new `Parker`.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  ```

- <span id="parker-park"></span>`fn park(&self)`

  Blocks the current thread until the token is made available.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  let u = p.unparker().clone();

  

  // Make the token available.

  u.unpark();

  

  // Wakes up immediately and consumes the token.

  p.park();

  ```

- <span id="parker-park-timeout"></span>`fn park_timeout(&self, timeout: Duration)`

  Blocks the current thread until the token is made available, but only for a limited time.

  

  # Examples

  

  ```rust

  use std::time::Duration;

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  

  // Waits for the token to become available, but will not wait longer than 500 ms.

  p.park_timeout(Duration::from_millis(500));

  ```

- <span id="parker-park-deadline"></span>`fn park_deadline(&self, deadline: Instant)`

  Blocks the current thread until the token is made available, or until a certain deadline.

  

  # Examples

  

  ```rust

  use std::time::{Duration, Instant};

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  let deadline = Instant::now() + Duration::from_millis(500);

  

  // Waits for the token to become available, but will not wait longer than 500 ms.

  p.park_deadline(deadline);

  ```

- <span id="parker-unparker"></span>`fn unparker(&self) -> &Unparker` — [`Unparker`](#unparker)

  Returns a reference to an associated [`Unparker`](#unparker).

  

  The returned [`Unparker`](#unparker) doesn't have to be used by reference - it can also be cloned.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  let u = p.unparker().clone();

  

  // Make the token available.

  u.unpark();

  // Wakes up immediately and consumes the token.

  p.park();

  ```

  

- <span id="parker-into-raw"></span>`fn into_raw(this: Parker) -> *const ()` — [`Parker`](#parker)

  Converts a `Parker` into a raw pointer.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  let raw = Parker::into_raw(p);

  let _ = unsafe { Parker::from_raw(raw) };

  ```

- <span id="parker-from-raw"></span>`unsafe fn from_raw(ptr: *const ()) -> Parker` — [`Parker`](#parker)

  Converts a raw pointer into a `Parker`.

  

  # Safety

  

  This method is safe to use only with pointers returned by `Parker::into_raw`.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  let raw = Parker::into_raw(p);

  let p = unsafe { Parker::from_raw(raw) };

  ```

#### Trait Implementations

##### `impl Any for Parker`

- <span id="parker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parker`

- <span id="parker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parker`

- <span id="parker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Parker`

- <span id="parker-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parker`

- <span id="parker-default"></span>`fn default() -> Self`

##### `impl<T> From for Parker`

- <span id="parker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parker`

- <span id="parker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Send for Parker`

##### `impl<U> TryFrom for Parker`

- <span id="parker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parker`

- <span id="parker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unparker`

```rust
struct Unparker {
    inner: std::sync::Arc<Inner>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:217-219`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L217-L219)*

Unparks a thread parked by the associated [`Parker`](#parker).

#### Implementations

- <span id="unparker-unpark"></span>`fn unpark(&self)`

  Atomically makes the token available if it is not already.

  

  This method will wake up the thread blocked on `park` or `park_timeout`, if there is

  any.

  

  # Examples

  

  ```rust

  use std::thread;

  use std::time::Duration;

  use crossbeam_utils::sync::Parker;

  

  let p = Parker::new();

  let u = p.unparker().clone();

  

  thread::spawn(move || {

      thread::sleep(Duration::from_millis(500));

      u.unpark();

  });

  

  // Wakes up when `u.unpark()` provides the token.

  p.park();

  std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371

  ```

  

- <span id="unparker-into-raw"></span>`fn into_raw(this: Unparker) -> *const ()` — [`Unparker`](#unparker)

  Converts an `Unparker` into a raw pointer.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::{Parker, Unparker};

  

  let p = Parker::new();

  let u = p.unparker().clone();

  let raw = Unparker::into_raw(u);

  let _ = unsafe { Unparker::from_raw(raw) };

  ```

- <span id="unparker-from-raw"></span>`unsafe fn from_raw(ptr: *const ()) -> Unparker` — [`Unparker`](#unparker)

  Converts a raw pointer into an `Unparker`.

  

  # Safety

  

  This method is safe to use only with pointers returned by `Unparker::into_raw`.

  

  # Examples

  

  ```rust

  use crossbeam_utils::sync::{Parker, Unparker};

  

  let p = Parker::new();

  let u = p.unparker().clone();

  

  let raw = Unparker::into_raw(u);

  let u = unsafe { Unparker::from_raw(raw) };

  ```

#### Trait Implementations

##### `impl Any for Unparker`

- <span id="unparker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unparker`

- <span id="unparker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unparker`

- <span id="unparker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Unparker`

- <span id="unparker-clone"></span>`fn clone(&self) -> Unparker` — [`Unparker`](#unparker)

##### `impl CloneToUninit for Unparker`

- <span id="unparker-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Unparker`

- <span id="unparker-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Unparker`

- <span id="unparker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unparker`

- <span id="unparker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Send for Unparker`

##### `impl Sync for Unparker`

##### `impl ToOwned for Unparker`

- <span id="unparker-toowned-type-owned"></span>`type Owned = T`

- <span id="unparker-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unparker-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unparker`

- <span id="unparker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unparker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unparker`

- <span id="unparker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unparker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Inner`

```rust
struct Inner {
    state: core::sync::atomic::AtomicUsize,
    lock: std::sync::Mutex<()>,
    cvar: std::sync::Condvar,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:314-318`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L314-L318)*

#### Implementations

- <span id="inner-park"></span>`fn park(&self, deadline: Option<Instant>)`

- <span id="inner-unpark"></span>`fn unpark(&self)`

#### Trait Implementations

##### `impl Any for Inner`

- <span id="inner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inner`

- <span id="inner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inner`

- <span id="inner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Inner`

- <span id="inner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Inner`

- <span id="inner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Inner`

- <span id="inner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inner`

- <span id="inner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `EMPTY`
```rust
const EMPTY: usize = 0usize;
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:310`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L310)*

### `PARKED`
```rust
const PARKED: usize = 1usize;
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:311`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L311)*

### `NOTIFIED`
```rust
const NOTIFIED: usize = 2usize;
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:312`](../../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L312)*

