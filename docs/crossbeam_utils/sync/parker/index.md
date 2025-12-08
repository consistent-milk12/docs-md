*[crossbeam_utils](../../index.md) / [sync](../index.md) / [parker](index.md)*

---

# Module `parker`

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

- `fn new() -> Parker` — [`Parker`](#parker)

- `fn park(self: &Self)`

- `fn park_timeout(self: &Self, timeout: Duration)`

- `fn park_deadline(self: &Self, deadline: Instant)`

- `fn unparker(self: &Self) -> &Unparker` — [`Unparker`](#unparker)

- `fn into_raw(this: Parker) -> *const ()` — [`Parker`](#parker)

- `unsafe fn from_raw(ptr: *const ()) -> Parker` — [`Parker`](#parker)

#### Trait Implementations

##### `impl Debug for Parker`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parker`

- `fn default() -> Self`

##### `impl Send for Parker`

### `Unparker`

```rust
struct Unparker {
    inner: std::sync::Arc<Inner>,
}
```

Unparks a thread parked by the associated [`Parker`](#parker).

#### Implementations

- `fn unpark(self: &Self)`

- `fn into_raw(this: Unparker) -> *const ()` — [`Unparker`](#unparker)

- `unsafe fn from_raw(ptr: *const ()) -> Unparker` — [`Unparker`](#unparker)

#### Trait Implementations

##### `impl Clone for Unparker`

- `fn clone(self: &Self) -> Unparker` — [`Unparker`](#unparker)

##### `impl Debug for Unparker`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn park(self: &Self, deadline: Option<Instant>)`

- `fn unpark(self: &Self)`

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

