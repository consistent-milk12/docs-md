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

- <span id="parker-new"></span>`fn new() -> Parker` — [`Parker`](../index.md)

- <span id="parker-park"></span>`fn park(&self)`

- <span id="parker-park-timeout"></span>`fn park_timeout(&self, timeout: Duration)`

- <span id="parker-park-deadline"></span>`fn park_deadline(&self, deadline: Instant)`

- <span id="parker-unparker"></span>`fn unparker(&self) -> &Unparker` — [`Unparker`](../index.md)

- <span id="parker-into-raw"></span>`fn into_raw(this: Parker) -> *const ()` — [`Parker`](../index.md)

- <span id="parker-from-raw"></span>`unsafe fn from_raw(ptr: *const ()) -> Parker` — [`Parker`](../index.md)

#### Trait Implementations

##### `impl Debug for Parker`

- <span id="parker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parker`

- <span id="parker-default"></span>`fn default() -> Self`

##### `impl Send for Parker`

### `Unparker`

```rust
struct Unparker {
    inner: std::sync::Arc<Inner>,
}
```

Unparks a thread parked by the associated [`Parker`](../index.md).

#### Implementations

- <span id="unparker-unpark"></span>`fn unpark(&self)`

- <span id="unparker-into-raw"></span>`fn into_raw(this: Unparker) -> *const ()` — [`Unparker`](../index.md)

- <span id="unparker-from-raw"></span>`unsafe fn from_raw(ptr: *const ()) -> Unparker` — [`Unparker`](../index.md)

#### Trait Implementations

##### `impl Clone for Unparker`

- <span id="unparker-clone"></span>`fn clone(&self) -> Unparker` — [`Unparker`](../index.md)

##### `impl Debug for Unparker`

- <span id="unparker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Send for Unparker`

##### `impl Sync for Unparker`

### `Inner`

```rust
struct Inner {
    state: core::sync::atomic::AtomicUsize,
    lock: std::sync::Mutex<()>,
    cvar: std::sync::Condvar,
}
```

#### Implementations

- <span id="inner-park"></span>`fn park(&self, deadline: Option<Instant>)`

- <span id="inner-unpark"></span>`fn unpark(&self)`

## Constants

### `EMPTY`

```rust
const EMPTY: usize = 0usize;
```

### `PARKED`

```rust
const PARKED: usize = 1usize;
```

### `NOTIFIED`

```rust
const NOTIFIED: usize = 2usize;
```

