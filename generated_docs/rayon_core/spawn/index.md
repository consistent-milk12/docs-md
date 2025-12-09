*[rayon_core](../index.md) / [spawn](index.md)*

---

# Module `spawn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`spawn`](#spawn) | fn | Puts the task into the Rayon thread pool's job queue in the "static" or "global" scope. |
| [`spawn_in`](#spawn_in) | fn | Spawns an asynchronous job in `registry.` |
| [`spawn_job`](#spawn_job) | fn |  |
| [`spawn_fifo`](#spawn_fifo) | fn | Fires off a task into the Rayon thread pool in the "static" or "global" scope. |
| [`spawn_fifo_in`](#spawn_fifo_in) | fn | Spawns an asynchronous FIFO job in `registry.` |

## Functions

### `spawn`

```rust
fn spawn<F>(func: F)
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:58-64`](../../../.source_1765210505/rayon-core-1.13.0/src/spawn/mod.rs#L58-L64)*

Puts the task into the Rayon thread pool's job queue in the "static"
or "global" scope. Just like a standard thread, this task is not
tied to the current stack frame, and hence it cannot hold any
references other than those with `'static` lifetime. If you want
to spawn a task that references stack data, use [the `scope()`
function] to create a scope.

Since tasks spawned with this function cannot hold references into
the enclosing stack frame, you almost certainly want to use a
`move` closure as their argument (otherwise, the closure will
typically hold references to any variables from the enclosing
function that you happen to use).

This API assumes that the closure is executed purely for its
side-effects (i.e., it might send messages, modify data protected
by a mutex, or some such thing).

There is no guaranteed order of execution for spawns, given that
other threads may steal tasks at any time. However, they are
generally prioritized in a LIFO order on the thread from which
they were spawned. Other threads always steal from the other end of
the deque, like FIFO order.  The idea is that "recent" tasks are
most likely to be fresh in the local CPU's cache, while other
threads can steal older "stale" tasks.  For an alternate approach,
consider [`spawn_fifo()`](#spawn-fifo) instead.

# Panic handling

If this closure should panic, the resulting panic will be
propagated to the panic handler registered in the `ThreadPoolBuilder`,
if any.  See `ThreadPoolBuilder::panic_handler()` for more
details.

# Examples

This code creates a Rayon task that increments a global counter.

```rust
use rayon_core as rayon;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static GLOBAL_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

rayon::spawn(move || {
    GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
});
```

### `spawn_in`

```rust
unsafe fn spawn_in<F>(func: F, registry: &std::sync::Arc<crate::registry::Registry>)
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:69-82`](../../../.source_1765210505/rayon-core-1.13.0/src/spawn/mod.rs#L69-L82)*

Spawns an asynchronous job in `registry.`

Unsafe because `registry` must not yet have terminated.

### `spawn_job`

```rust
unsafe fn spawn_job<F>(func: F, registry: &std::sync::Arc<crate::registry::Registry>) -> JobRef
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:84-100`](../../../.source_1765210505/rayon-core-1.13.0/src/spawn/mod.rs#L84-L100)*

### `spawn_fifo`

```rust
fn spawn_fifo<F>(func: F)
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:130-136`](../../../.source_1765210505/rayon-core-1.13.0/src/spawn/mod.rs#L130-L136)*

Fires off a task into the Rayon thread pool in the "static" or
"global" scope.  Just like a standard thread, this task is not
tied to the current stack frame, and hence it cannot hold any
references other than those with `'static` lifetime. If you want
to spawn a task that references stack data, use [the `scope_fifo()`
function] to create a scope.

The behavior is essentially the same as [the `spawn`
function], except that calls from the same thread
will be prioritized in FIFO order. This is similar to the now-
deprecated `breadth_first` option, except the effect is isolated
to relative `spawn_fifo` calls, not all thread-pool tasks.

For more details on this design, see Rayon [RFC #1].




# Panic handling

If this closure should panic, the resulting panic will be
propagated to the panic handler registered in the `ThreadPoolBuilder`,
if any.  See `ThreadPoolBuilder::panic_handler()` for more
details.


### `spawn_fifo_in`

```rust
unsafe fn spawn_fifo_in<F>(func: F, registry: &std::sync::Arc<crate::registry::Registry>)
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:141-160`](../../../.source_1765210505/rayon-core-1.13.0/src/spawn/mod.rs#L141-L160)*

Spawns an asynchronous FIFO job in `registry.`

Unsafe because `registry` must not yet have terminated.

