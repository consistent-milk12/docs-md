*[rayon_core](../index.md) / [thread_pool](index.md)*

---

# Module `thread_pool`

Contains support for user-managed thread pools, represented by the
the [`ThreadPool`](#threadpool) type (see that struct for details).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ThreadPool`](#threadpool) | struct | Represents a user-created [thread pool]. |
| [`Yield`](#yield) | enum | Result of [`yield_now()`] or [`yield_local()`]. |
| [`current_thread_index`](#current-thread-index) | fn | If called from a Rayon worker thread, returns the index of that thread within its current pool; if not called from a Rayon thread, returns `None`. |
| [`current_thread_has_pending_tasks`](#current-thread-has-pending-tasks) | fn | If called from a Rayon worker thread, indicates whether that thread's local deque still has pending tasks. |
| [`yield_now`](#yield-now) | fn | Cooperatively yields execution to Rayon. |
| [`yield_local`](#yield-local) | fn | Cooperatively yields execution to local Rayon work. |

## Structs

### `ThreadPool`

```rust
struct ThreadPool {
    registry: std::sync::Arc<crate::registry::Registry>,
}
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:46-48`](../../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L46-L48)*

Represents a user-created [thread pool].

Use a [`ThreadPoolBuilder`](../index.md) to specify the number and/or names of threads
in the pool. After calling `ThreadPoolBuilder::build()`, you can then
execute functions explicitly within this [`ThreadPool`](#threadpool) using
`ThreadPool::install()`. By contrast, top-level rayon functions
(like `join()`) will execute implicitly within the current thread pool.


## Creating a ThreadPool

```ignore-wasm
use rayon_core as rayon;
let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
```

[`install()`]`ThreadPool::install()` executes a closure in one of the `ThreadPool`'s
threads. In addition, any other rayon operations called inside of `install()` will also
execute in the context of the `ThreadPool`.

When the `ThreadPool` is dropped, that's a signal for the threads it manages to terminate,
they will complete executing any remaining work that you have spawned, and automatically
terminate.




#### Implementations

- <span id="threadpool-new"></span>`fn new(configuration: crate::Configuration) -> Result<ThreadPool, Box<dyn Error>>` — [`Configuration`](../index.md#configuration), [`ThreadPool`](#threadpool)

- <span id="threadpool-build"></span>`fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md#threadpoolbuilder), [`ThreadPool`](#threadpool), [`ThreadPoolBuildError`](../index.md#threadpoolbuilderror)

- <span id="threadpool-install"></span>`fn install<OP, R>(&self, op: OP) -> R`

- <span id="threadpool-broadcast"></span>`fn broadcast<OP, R>(&self, op: OP) -> Vec<R>`

- <span id="threadpool-current-num-threads"></span>`fn current_num_threads(&self) -> usize`

- <span id="threadpool-current-thread-index"></span>`fn current_thread_index(&self) -> Option<usize>`

- <span id="threadpool-current-thread-has-pending-tasks"></span>`fn current_thread_has_pending_tasks(&self) -> Option<bool>`

- <span id="threadpool-join"></span>`fn join<A, B, RA, RB>(&self, oper_a: A, oper_b: B) -> (RA, RB)`

- <span id="threadpool-scope"></span>`fn scope<'scope, OP, R>(&self, op: OP) -> R`

- <span id="threadpool-scope-fifo"></span>`fn scope_fifo<'scope, OP, R>(&self, op: OP) -> R`

- <span id="threadpool-in-place-scope"></span>`fn in_place_scope<'scope, OP, R>(&self, op: OP) -> R`

- <span id="threadpool-in-place-scope-fifo"></span>`fn in_place_scope_fifo<'scope, OP, R>(&self, op: OP) -> R`

- <span id="threadpool-spawn"></span>`fn spawn<OP>(&self, op: OP)`

- <span id="threadpool-spawn-fifo"></span>`fn spawn_fifo<OP>(&self, op: OP)`

- <span id="threadpool-spawn-broadcast"></span>`fn spawn_broadcast<OP>(&self, op: OP)`

- <span id="threadpool-yield-now"></span>`fn yield_now(&self) -> Option<Yield>` — [`Yield`](#yield)

- <span id="threadpool-yield-local"></span>`fn yield_local(&self) -> Option<Yield>` — [`Yield`](#yield)

#### Trait Implementations

##### `impl Debug for ThreadPool`

- <span id="threadpool-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ThreadPool`

- <span id="threadpool-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for ThreadPool`

- <span id="threadpool-pointable-const-align"></span>`const ALIGN: usize`

- <span id="threadpool-pointable-type-init"></span>`type Init = T`

- <span id="threadpool-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpool-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpool-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpool-drop"></span>`unsafe fn drop(ptr: usize)`

## Enums

### `Yield`

```rust
enum Yield {
    Executed,
    Idle,
}
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:497-502`](../../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L497-L502)*

Result of [`yield_now()`](#yield-now) or [`yield_local()`](#yield-local).

#### Variants

- **`Executed`**

  Work was found and executed.

- **`Idle`**

  No available work was found.

#### Trait Implementations

##### `impl Clone for Yield`

- <span id="yield-clone"></span>`fn clone(&self) -> Yield` — [`Yield`](#yield)

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- <span id="yield-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Yield`

##### `impl PartialEq for Yield`

- <span id="yield-eq"></span>`fn eq(&self, other: &Yield) -> bool` — [`Yield`](#yield)

##### `impl Pointable for Yield`

- <span id="yield-pointable-const-align"></span>`const ALIGN: usize`

- <span id="yield-pointable-type-init"></span>`type Init = T`

- <span id="yield-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="yield-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="yield-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="yield-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Yield`

## Functions

### `current_thread_index`

```rust
fn current_thread_index() -> Option<usize>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:438-443`](../../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L438-L443)*

If called from a Rayon worker thread, returns the index of that
thread within its current pool; if not called from a Rayon thread,
returns `None`.

The index for a given thread will not change over the thread's
lifetime. However, multiple threads may share the same index if
they are in distinct thread pools.

See also: [the `ThreadPool::current_thread_index()` method][m].

# Future compatibility note

Currently, every thread pool (including the global
thread pool) has a fixed number of threads, but this may
change in future Rayon versions (see [the `num_threads()` method
for details][snt]). In that case, the index for a
thread would not change during its lifetime, but thread
indices may wind up being reused if threads are terminated and
restarted.


### `current_thread_has_pending_tasks`

```rust
fn current_thread_has_pending_tasks() -> Option<bool>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:452-457`](../../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L452-L457)*

If called from a Rayon worker thread, indicates whether that
thread's local deque still has pending tasks. Otherwise, returns
`None`. For more information, see [the
`ThreadPool::current_thread_has_pending_tasks()` method][m].


### `yield_now`

```rust
fn yield_now() -> Option<Yield>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:471-476`](../../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L471-L476)*

Cooperatively yields execution to Rayon.

If the current thread is part of a rayon thread pool, this looks for a
single unit of pending work in the pool, then executes it. Completion of
that work might include nested work or further work stealing.

This is similar to [`std::thread::yield_now()`](../../rayon/index.md), but does not literally make
that call. If you are implementing a polling loop, you may want to also
yield to the OS scheduler yourself if no Rayon work was found.

Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if
nothing was available, or `None` if this thread is not part of any pool at all.

### `yield_local`

```rust
fn yield_local() -> Option<Yield>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:488-493`](../../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L488-L493)*

Cooperatively yields execution to local Rayon work.

If the current thread is part of a rayon thread pool, this looks for a
single unit of pending work in this thread's queue, then executes it.
Completion of that work might include nested work or further work stealing.

This is similar to [`yield_now()`](#yield-now), but does not steal from other threads.

Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if
nothing was available, or `None` if this thread is not part of any pool at all.

