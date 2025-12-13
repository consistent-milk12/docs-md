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

  Deprecated in favor of `ThreadPoolBuilder::build`.

- <span id="threadpool-build"></span>`fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md#threadpoolbuilder), [`ThreadPool`](#threadpool), [`ThreadPoolBuildError`](../index.md#threadpoolbuilderror)

- <span id="threadpool-install"></span>`fn install<OP, R>(&self, op: OP) -> R`

  Executes `op` within the thread pool. Any attempts to use

  `join`, `scope`, or parallel iterators will then operate

  within that thread pool.

  

  # Warning: thread-local data

  

  Because `op` is executing within the Rayon thread pool,

  thread-local data from the current thread will not be

  accessible.

  

  # Warning: execution order

  

  If the current thread is part of a different thread pool, it will try to

  keep busy while the `op` completes in its target pool, similar to

  calling `ThreadPool::yield_now()` in a loop. Therefore, it may

  potentially schedule other tasks to run on the current thread in the

  meantime. For example

  

  ```ignore-wasm

  use rayon_core as rayon;

  fn main() {

      rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();

      let pool = rayon_core::ThreadPoolBuilder::default().build().unwrap();

      let do_it = || {

          print!("one ");

          pool.install(||{});

          print!("two ");

      };

      rayon::join(|| do_it(), || do_it());

  }

  ```

  

  Since we configured just one thread in the global pool, one might

  expect `do_it()` to run sequentially, producing:

  

  ```ascii

  one two one two

  ```

  

  However each call to `install()` yields implicitly, allowing rayon to

  run multiple instances of `do_it()` concurrently on the single, global

  thread. The following output would be equally valid:

  

  ```ascii

  one one two two

  ```

  

  # Panics

  

  If `op` should panic, that panic will be propagated.

  

  ## Using `install()`

  

  ```ignore-wasm

     use rayon_core as rayon;

     fn main() {

          let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();

          let n = pool.install(|| fib(20));

          println!("{}", n);

     }

  

     fn fib(n: usize) -> usize {

          if n == 0 || n == 1 {

              return n;

          }

          let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`

          return a + b;

      }

  ```

- <span id="threadpool-broadcast"></span>`fn broadcast<OP, R>(&self, op: OP) -> Vec<R>`

  Executes `op` within every thread in the thread pool. Any attempts to use

  `join`, `scope`, or parallel iterators will then operate within that

  thread pool.

  

  Broadcasts are executed on each thread after they have exhausted their

  local work queue, before they attempt work-stealing from other threads.

  The goal of that strategy is to run everywhere in a timely manner

  *without* being too disruptive to current work. There may be alternative

  broadcast styles added in the future for more or less aggressive

  injection, if the need arises.

  

  # Warning: thread-local data

  

  Because `op` is executing within the Rayon thread pool,

  thread-local data from the current thread will not be

  accessible.

  

  # Panics

  

  If `op` should panic on one or more threads, exactly one panic

  will be propagated, only after all threads have completed

  (or panicked) their own `op`.

  

  # Examples

  

  ```ignore-wasm

     use rayon_core as rayon;

     use std::sync::atomic::{AtomicUsize, Ordering};

  

     fn main() {

          let pool = rayon::ThreadPoolBuilder::new().num_threads(5).build().unwrap();

  

          // The argument gives context, including the index of each thread.

          let v: Vec<usize> = pool.broadcast(|ctx| ctx.index() * ctx.index());

          assert_eq!(v, &[0, 1, 4, 9, 16]);

  

          // The closure can reference the local stack

          let count = AtomicUsize::new(0);

          pool.broadcast(|_| count.fetch_add(1, Ordering::Relaxed));

          assert_eq!(count.into_inner(), 5);

     }

  ```

- <span id="threadpool-current-num-threads"></span>`fn current_num_threads(&self) -> usize`

  Returns the (current) number of threads in the thread pool.

  

  # Future compatibility note

  

  Note that unless this thread pool was created with a

  [`ThreadPoolBuilder`](../index.md) that specifies the number of threads,

  then this number may vary over time in future versions (see [the

  `num_threads()` method for details][snt]).

- <span id="threadpool-current-thread-index"></span>`fn current_thread_index(&self) -> Option<usize>`

  If called from a Rayon worker thread in this thread pool,

  returns the index of that thread; if not called from a Rayon

  thread, or called from a Rayon thread that belongs to a

  different thread pool, returns `None`.

  

  The index for a given thread will not change over the thread's

  lifetime. However, multiple threads may share the same index if

  they are in distinct thread pools.

  

  # Future compatibility note

  

  Currently, every thread pool (including the global

  thread pool) has a fixed number of threads, but this may

  change in future Rayon versions (see [the `num_threads()` method

  for details][snt]). In that case, the index for a

  thread would not change during its lifetime, but thread

  indices may wind up being reused if threads are terminated and

  restarted.

- <span id="threadpool-current-thread-has-pending-tasks"></span>`fn current_thread_has_pending_tasks(&self) -> Option<bool>`

  Returns true if the current worker thread currently has "local

  tasks" pending. This can be useful as part of a heuristic for

  deciding whether to spawn a new task or execute code on the

  current thread, particularly in breadth-first

  schedulers. However, keep in mind that this is an inherently

  racy check, as other worker threads may be actively "stealing"

  tasks from our local deque.

  

  **Background:** Rayon's uses a [work-stealing] scheduler. The

  key idea is that each thread has its own [`deque`](../../crossbeam_deque/deque/index.md) of

  tasks. Whenever a new task is spawned -- whether through

  `join()`, `Scope::spawn()`, or some other means -- that new

  task is pushed onto the thread's *local* deque. Worker threads

  have a preference for executing their own tasks; if however

  they run out of tasks, they will go try to "steal" tasks from

  other threads. This function therefore has an inherent race

  with other active worker threads, which may be removing items

  from the local deque.

  

- <span id="threadpool-join"></span>`fn join<A, B, RA, RB>(&self, oper_a: A, oper_b: B) -> (RA, RB)`

  Execute `oper_a` and `oper_b` in the thread pool and return

  the results. Equivalent to `self.install(|| join(oper_a,

  oper_b))`.

- <span id="threadpool-scope"></span>`fn scope<'scope, OP, R>(&self, op: OP) -> R`

  Creates a scope that executes within this thread pool.

  Equivalent to `self.install(|| scope(...))`.

  

  See also: [the `scope()` function].

- <span id="threadpool-scope-fifo"></span>`fn scope_fifo<'scope, OP, R>(&self, op: OP) -> R`

  Creates a scope that executes within this thread pool.

  Spawns from the same thread are prioritized in relative FIFO order.

  Equivalent to `self.install(|| scope_fifo(...))`.

  

  See also: [the `scope_fifo()` function].

- <span id="threadpool-in-place-scope"></span>`fn in_place_scope<'scope, OP, R>(&self, op: OP) -> R`

  Creates a scope that spawns work into this thread pool.

  

  See also: [the `in_place_scope()` function].

- <span id="threadpool-in-place-scope-fifo"></span>`fn in_place_scope_fifo<'scope, OP, R>(&self, op: OP) -> R`

  Creates a scope that spawns work into this thread pool in FIFO order.

  

  See also: [the `in_place_scope_fifo()` function].

- <span id="threadpool-spawn"></span>`fn spawn<OP>(&self, op: OP)`

  Spawns an asynchronous task in this thread pool. This task will

  run in the implicit, global scope, which means that it may outlast

  the current stack frame -- therefore, it cannot capture any references

  onto the stack (you will likely need a `move` closure).

  

  See also: [the `spawn()` function defined on scopes][`spawn`](../spawn/index.md).

- <span id="threadpool-spawn-fifo"></span>`fn spawn_fifo<OP>(&self, op: OP)`

  Spawns an asynchronous task in this thread pool. This task will

  run in the implicit, global scope, which means that it may outlast

  the current stack frame -- therefore, it cannot capture any references

  onto the stack (you will likely need a `move` closure).

  

  See also: [the `spawn_fifo()` function defined on scopes][`spawn_fifo`](../spawn/index.md).

- <span id="threadpool-spawn-broadcast"></span>`fn spawn_broadcast<OP>(&self, op: OP)`

  Spawns an asynchronous task on every thread in this thread pool. This task

  will run in the implicit, global scope, which means that it may outlast the

  current stack frame -- therefore, it cannot capture any references onto the

  stack (you will likely need a `move` closure).

- <span id="threadpool-yield-now"></span>`fn yield_now(&self) -> Option<Yield>` — [`Yield`](#yield)

  Cooperatively yields execution to Rayon.

  

  This is similar to the general [`yield_now()`](#yield-now), but only if the current

  thread is part of *this* thread pool.

  

  Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if

  nothing was available, or `None` if the current thread is not part this pool.

- <span id="threadpool-yield-local"></span>`fn yield_local(&self) -> Option<Yield>` — [`Yield`](#yield)

  Cooperatively yields execution to local Rayon work.

  

  This is similar to the general [`yield_local()`](#yield-local), but only if the current

  thread is part of *this* thread pool.

  

  Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if

  nothing was available, or `None` if the current thread is not part this pool.

#### Trait Implementations

##### `impl Any for ThreadPool`

- <span id="threadpool-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadPool`

- <span id="threadpool-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadPool`

- <span id="threadpool-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ThreadPool`

- <span id="threadpool-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ThreadPool`

- <span id="threadpool-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for ThreadPool`

- <span id="threadpool-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreadPool`

- <span id="threadpool-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ThreadPool`

- <span id="threadpool-pointable-const-align"></span>`const ALIGN: usize`

- <span id="threadpool-pointable-type-init"></span>`type Init = T`

- <span id="threadpool-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpool-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpool-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpool-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ThreadPool`

- <span id="threadpool-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadpool-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreadPool`

- <span id="threadpool-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadpool-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Yield`

- <span id="yield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Yield`

- <span id="yield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Yield`

- <span id="yield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Yield`

- <span id="yield-clone"></span>`fn clone(&self) -> Yield` — [`Yield`](#yield)

##### `impl CloneToUninit for Yield`

- <span id="yield-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- <span id="yield-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Yield`

##### `impl<T> From for Yield`

- <span id="yield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Yield`

- <span id="yield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Yield`

- <span id="yield-partialeq-eq"></span>`fn eq(&self, other: &Yield) -> bool` — [`Yield`](#yield)

##### `impl Pointable for Yield`

- <span id="yield-pointable-const-align"></span>`const ALIGN: usize`

- <span id="yield-pointable-type-init"></span>`type Init = T`

- <span id="yield-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="yield-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="yield-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="yield-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Yield`

##### `impl ToOwned for Yield`

- <span id="yield-toowned-type-owned"></span>`type Owned = T`

- <span id="yield-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="yield-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Yield`

- <span id="yield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="yield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Yield`

- <span id="yield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="yield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

