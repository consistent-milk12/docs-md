*[crossbeam_epoch](../index.md) / [default](index.md)*

---

# Module `default`

The default garbage collector.

For each thread, a participant is lazily initialized on its first use, when the current thread
is registered in the default collector.  If initialized, the thread's participant will get
destructed on thread exit, which in turn unregisters the thread.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`collector`](#collector) | fn |  |
| [`pin`](#pin) | fn | Pins the current thread. |
| [`is_pinned`](#is-pinned) | fn | Returns `true` if the current thread is pinned. |
| [`default_collector`](#default-collector) | fn | Returns the default global collector. |
| [`with_handle`](#with-handle) | fn |  |
| [`HANDLE`](#handle) | const | The per-thread participant for the default garbage collector. |

## Functions

### `collector`

```rust
fn collector() -> &'static crate::collector::Collector
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:13-30`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L13-L30)*

### `pin`

```rust
fn pin() -> crate::guard::Guard
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:39-41`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L39-L41)*

Pins the current thread.

### `is_pinned`

```rust
fn is_pinned() -> bool
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:45-47`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L45-L47)*

Returns `true` if the current thread is pinned.

### `default_collector`

```rust
fn default_collector() -> &'static crate::collector::Collector
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:50-52`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L50-L52)*

Returns the default global collector.

### `with_handle`

```rust
fn with_handle<F, R>(f: F) -> R
where
    F: FnMut(&crate::collector::LocalHandle) -> R
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:55-62`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L55-L62)*

## Constants

### `HANDLE`
```rust
const HANDLE: thread::LocalKey<crate::collector::LocalHandle>;
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:32-35`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L32-L35)*

The per-thread participant for the default garbage collector.

