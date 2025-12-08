*[rayon_core](../index.md) / [thread_pool](index.md)*

---

# Module `thread_pool`

Contains support for user-managed thread pools, represented by the
the [`ThreadPool`](../index.md) type (see that struct for details).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ThreadPool`](#threadpool) | struct | Represents a user-created [thread pool]. |
| [`Yield`](#yield) | enum | Result of [`yield_now()`] or [`yield_local()`]. |
| [`current_thread_index`](#current_thread_index) | fn | If called from a Rayon worker thread, returns the index of that |
| [`current_thread_has_pending_tasks`](#current_thread_has_pending_tasks) | fn | If called from a Rayon worker thread, indicates whether that |
| [`yield_now`](#yield_now) | fn | Cooperatively yields execution to Rayon. |
| [`yield_local`](#yield_local) | fn | Cooperatively yields execution to local Rayon work. |

## Structs

### `ThreadPool`

```rust
struct ThreadPool {
    registry: std::sync::Arc<crate::registry::Registry>,
}
```

Represents a user-created [thread pool].

Use a [`ThreadPoolBuilder`](../index.md) to specify the number and/or names of threads
in the pool. After calling `ThreadPoolBuilder::build()`, you can then
execute functions explicitly within this [`ThreadPool`](../index.md) using
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

- <span id="threadpool-new"></span>`fn new(configuration: crate::Configuration) -> Result<ThreadPool, Box<dyn Error>>` — [`Configuration`](../index.md), [`ThreadPool`](../index.md)

- <span id="threadpool-build"></span>`fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md), [`ThreadPool`](../index.md), [`ThreadPoolBuildError`](../index.md)

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

- <span id="threadpool-yield-now"></span>`fn yield_now(&self) -> Option<Yield>` — [`Yield`](../index.md)

- <span id="threadpool-yield-local"></span>`fn yield_local(&self) -> Option<Yield>` — [`Yield`](../index.md)

#### Trait Implementations

##### `impl Debug for ThreadPool`

- <span id="threadpool-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ThreadPool`

- <span id="threadpool-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for ThreadPool`

- <span id="threadpool-align"></span>`const ALIGN: usize`

- <span id="threadpool-init"></span>`type Init = T`

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

Result of [`yield_now()`](../index.md) or [`yield_local()`](../index.md).

#### Variants

- **`Executed`**

  Work was found and executed.

- **`Idle`**

  No available work was found.

#### Trait Implementations

##### `impl Clone for Yield`

- <span id="yield-clone"></span>`fn clone(&self) -> Yield` — [`Yield`](../index.md)

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- <span id="yield-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Yield`

##### `impl PartialEq for Yield`

- <span id="yield-eq"></span>`fn eq(&self, other: &Yield) -> bool` — [`Yield`](../index.md)

##### `impl<T> Pointable for Yield`

- <span id="yield-align"></span>`const ALIGN: usize`

- <span id="yield-init"></span>`type Init = T`

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

If called from a Rayon worker thread, indicates whether that
thread's local deque still has pending tasks. Otherwise, returns
`None`. For more information, see [the
`ThreadPool::current_thread_has_pending_tasks()` method][m].


### `yield_now`

```rust
fn yield_now() -> Option<Yield>
```

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

Cooperatively yields execution to local Rayon work.

If the current thread is part of a rayon thread pool, this looks for a
single unit of pending work in this thread's queue, then executes it.
Completion of that work might include nested work or further work stealing.

This is similar to [`yield_now()`](../index.md), but does not steal from other threads.

Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if
nothing was available, or `None` if this thread is not part of any pool at all.

