*[rayon_core](../index.md) / [broadcast](index.md)*

---

# Module `broadcast`

## Structs

### `BroadcastContext<'a>`

```rust
struct BroadcastContext<'a> {
    worker: &'a crate::registry::WorkerThread,
    _marker: std::marker::PhantomData<&'a mut dyn Fn()>,
}
```

Provides context to a closure called by `broadcast`.

#### Fields

- **`_marker`**: `std::marker::PhantomData<&'a mut dyn Fn()>`

  Make sure to prevent auto-traits like `Send` and `Sync`.

#### Implementations

- `fn with<R>(f: impl FnOnce(BroadcastContext<'_>) -> R) -> R` — [`BroadcastContext`](#broadcastcontext)

- `fn index(self: &Self) -> usize`

- `fn num_threads(self: &Self) -> usize`

#### Trait Implementations

##### `impl<'a> Debug for BroadcastContext<'a>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for BroadcastContext<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `broadcast`

```rust
fn broadcast<OP, R>(op: OP) -> Vec<R>
where
    OP: Fn(BroadcastContext<'_>) -> R + Sync,
    R: Send
```

Executes `op` within every thread in the current thread pool. If this is
called from a non-Rayon thread, it will execute in the global thread pool.
Any attempts to use `join`, `scope`, or parallel iterators will then operate
within that thread pool. When the call has completed on each thread, returns
a vector containing all of their return values.

For more information, see the `ThreadPool::broadcast()` method.


### `spawn_broadcast`

```rust
fn spawn_broadcast<OP>(op: OP)
where
    OP: Fn(BroadcastContext<'_>) + Send + Sync + 'static
```

Spawns an asynchronous task on every thread in this thread pool. This task
will run in the implicit, global scope, which means that it may outlast the
current stack frame -- therefore, it cannot capture any references onto the
stack (you will likely need a `move` closure).

For more information, see the `ThreadPool::spawn_broadcast()` method.


### `broadcast_in`

```rust
unsafe fn broadcast_in<OP, R>(op: OP, registry: &std::sync::Arc<crate::registry::Registry>) -> Vec<R>
where
    OP: Fn(BroadcastContext<'_>) -> R + Sync,
    R: Send
```

Execute `op` on every thread in the pool. It will be executed on each
thread when they have nothing else to do locally, before they try to
steal work from other threads. This function will not return until all
threads have completed the `op`.

Unsafe because `registry` must not yet have terminated.

### `spawn_broadcast_in`

```rust
unsafe fn spawn_broadcast_in<OP>(op: OP, registry: &std::sync::Arc<crate::registry::Registry>)
where
    OP: Fn(BroadcastContext<'_>) + Send + Sync + 'static
```

Execute `op` on every thread in the pool. It will be executed on each
thread when they have nothing else to do locally, before they try to
steal work from other threads. This function returns immediately after
injecting the jobs.

Unsafe because `registry` must not yet have terminated.

