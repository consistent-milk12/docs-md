*[crossbeam_epoch](../index.md) / [default](index.md)*

---

# Module `default`

The default garbage collector.

For each thread, a participant is lazily initialized on its first use, when the current thread
is registered in the default collector.  If initialized, the thread's participant will get
destructed on thread exit, which in turn unregisters the thread.

## Functions

### `collector`

```rust
fn collector() -> &'static crate::collector::Collector
```

### `pin`

```rust
fn pin() -> crate::guard::Guard
```

Pins the current thread.

### `is_pinned`

```rust
fn is_pinned() -> bool
```

Returns `true` if the current thread is pinned.

### `default_collector`

```rust
fn default_collector() -> &'static crate::collector::Collector
```

Returns the default global collector.

### `with_handle`

```rust
fn with_handle<F, R>(f: F) -> R
where
    F: FnMut(&crate::collector::LocalHandle) -> R
```

## Constants

### `HANDLE`

```rust
const HANDLE: $crate::thread::LocalKey<crate::collector::LocalHandle>;
```

The per-thread participant for the default garbage collector.

