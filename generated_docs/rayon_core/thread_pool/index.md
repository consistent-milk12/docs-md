*[rayon_core](../index.md) / [thread_pool](index.md)*

---

# Module `thread_pool`

Contains support for user-managed thread pools, represented by the
the [`ThreadPool`](../index.md) type (see that struct for details).

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

- `fn new(configuration: crate::Configuration) -> Result<ThreadPool, Box<dyn Error>>` — [`Configuration`](../index.md), [`ThreadPool`](../index.md)

- `fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md), [`ThreadPool`](../index.md), [`ThreadPoolBuildError`](../index.md)

- `fn install<OP, R>(self: &Self, op: OP) -> R`

- `fn broadcast<OP, R>(self: &Self, op: OP) -> Vec<R>`

- `fn current_num_threads(self: &Self) -> usize`

- `fn current_thread_index(self: &Self) -> Option<usize>`

- `fn current_thread_has_pending_tasks(self: &Self) -> Option<bool>`

- `fn join<A, B, RA, RB>(self: &Self, oper_a: A, oper_b: B) -> (RA, RB)`

- `fn scope<'scope, OP, R>(self: &Self, op: OP) -> R`

- `fn scope_fifo<'scope, OP, R>(self: &Self, op: OP) -> R`

- `fn in_place_scope<'scope, OP, R>(self: &Self, op: OP) -> R`

- `fn in_place_scope_fifo<'scope, OP, R>(self: &Self, op: OP) -> R`

- `fn spawn<OP>(self: &Self, op: OP)`

- `fn spawn_fifo<OP>(self: &Self, op: OP)`

- `fn spawn_broadcast<OP>(self: &Self, op: OP)`

- `fn yield_now(self: &Self) -> Option<Yield>` — [`Yield`](../index.md)

- `fn yield_local(self: &Self) -> Option<Yield>` — [`Yield`](../index.md)

#### Trait Implementations

##### `impl Debug for ThreadPool`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ThreadPool`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for ThreadPool`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn clone(self: &Self) -> Yield` — [`Yield`](../index.md)

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Yield`

##### `impl PartialEq for Yield`

- `fn eq(self: &Self, other: &Yield) -> bool` — [`Yield`](../index.md)

##### `impl<T> Pointable for Yield`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

