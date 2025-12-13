# Crate `rayon_core`

Rayon-core houses the core stable APIs of Rayon.

These APIs have been mirrored in the Rayon crate and it is recommended to use these from there.

[`join()`](join/index.md) is used to take two closures and potentially run them in parallel.
  - It will run in parallel if task B gets stolen before task A can finish.
  - It will run sequentially if task A finishes before task B is stolen and can continue on task B.

[`scope()`](scope/index.md) creates a scope in which you can run any number of parallel tasks.
These tasks can spawn nested tasks and scopes, but given the nature of work stealing, the order of execution can not be guaranteed.
The scope will exist until all tasks spawned within the scope have been completed.

[`spawn()`](spawn/index.md) add a task into the 'static' or 'global' scope, or a local scope created by the [`scope()`](scope/index.md) function.

[`ThreadPool`](thread_pool/index.md) can be used to create your own thread pools (using [`ThreadPoolBuilder`](#threadpoolbuilder)) or to customize the global one.
Tasks spawned within the pool (using [`install()`][tpinstall], [`join()`][tpjoin], etc.) will be added to a deque,
where it becomes available for work stealing from other threads in the local thread pool.


# Global fallback when threading is unsupported

Rayon uses `std` APIs for threading, but some targets have incomplete implementations that
always return `Unsupported` errors. The WebAssembly `wasm32-unknown-unknown` and `wasm32-wasi`
targets are notable examples of this. Rather than panicking on the unsupported error when
creating the implicit global thread pool, Rayon configures a fallback mode instead.

This fallback mode mostly functions as if it were using a single-threaded "pool", like setting
`RAYON_NUM_THREADS=1`. For example, `join` will execute its two closures sequentially, since
there is no other thread to share the work. However, since the pool is not running independent
of the main thread, non-blocking calls like `spawn` may not execute at all, unless a lower-
priority call like `broadcast` gives them an opening. The fallback mode does not try to emulate
anything like thread preemption or `async` task switching, but `yield_now` or `yield_local`
can also volunteer execution time.

Explicit `ThreadPoolBuilder` methods always report their error without any fallback.

# Restricting multiple versions

In order to ensure proper coordination between thread pools, and especially
to make sure there's only one global thread pool, `rayon-core` is actively
restricted from building multiple versions of itself into a single target.
You may see a build error like this in violation:

```text
error: native library `rayon-core` is being linked to by more
than one package, and can only be linked to by one package
```

While we strive to keep `rayon-core` semver-compatible, it's still
possible to arrive at this situation if different crates have overly
restrictive tilde or inequality requirements for `rayon-core`.  The
conflicting requirements will need to be resolved before the build will
succeed.

## Contents

- [Modules](#modules)
  - [`private`](#private)
  - [`broadcast`](#broadcast)
  - [`job`](#job)
  - [`join`](#join)
  - [`latch`](#latch)
  - [`registry`](#registry)
  - [`scope`](#scope)
  - [`sleep`](#sleep)
  - [`spawn`](#spawn)
  - [`thread_pool`](#thread-pool)
  - [`unwind`](#unwind)
  - [`compile_fail`](#compile-fail)
- [Structs](#structs)
  - [`BroadcastContext`](#broadcastcontext)
  - [`ThreadBuilder`](#threadbuilder)
  - [`Scope`](#scope)
  - [`ScopeFifo`](#scopefifo)
  - [`ThreadPool`](#threadpool)
  - [`ThreadPoolBuildError`](#threadpoolbuilderror)
  - [`ThreadPoolBuilder`](#threadpoolbuilder)
  - [`Configuration`](#configuration)
  - [`FnContext`](#fncontext)
- [Enums](#enums)
  - [`Yield`](#yield)
  - [`ErrorKind`](#errorkind)
- [Functions](#functions)
  - [`broadcast`](#broadcast)
  - [`spawn_broadcast`](#spawn-broadcast)
  - [`join`](#join)
  - [`join_context`](#join-context)
  - [`in_place_scope`](#in-place-scope)
  - [`scope`](#scope)
  - [`in_place_scope_fifo`](#in-place-scope-fifo)
  - [`scope_fifo`](#scope-fifo)
  - [`spawn`](#spawn)
  - [`spawn_fifo`](#spawn-fifo)
  - [`current_thread_has_pending_tasks`](#current-thread-has-pending-tasks)
  - [`current_thread_index`](#current-thread-index)
  - [`yield_local`](#yield-local)
  - [`yield_now`](#yield-now)
  - [`max_num_threads`](#max-num-threads)
  - [`current_num_threads`](#current-num-threads)
  - [`initialize`](#initialize)
- [Type Aliases](#type-aliases)
  - [`PanicHandler`](#panichandler)
  - [`StartHandler`](#starthandler)
  - [`ExitHandler`](#exithandler)
- [Constants](#constants)
  - [`GLOBAL_POOL_ALREADY_INITIALIZED`](#global-pool-already-initialized)
  - [`CURRENT_THREAD_ALREADY_IN_POOL`](#current-thread-already-in-pool)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | The public parts of this private module are used to create traits that cannot be implemented outside of our own crate. |
| [`broadcast`](#broadcast) | mod |  |
| [`job`](#job) | mod |  |
| [`join`](#join) | mod |  |
| [`latch`](#latch) | mod |  |
| [`registry`](#registry) | mod |  |
| [`scope`](#scope) | mod | Methods for custom fork-join scopes, created by the [`scope()`] and [`in_place_scope()`] functions. |
| [`sleep`](#sleep) | mod | Code that decides when workers should go to sleep. |
| [`spawn`](#spawn) | mod |  |
| [`thread_pool`](#thread-pool) | mod | Contains support for user-managed thread pools, represented by the the [`ThreadPool`] type (see that struct for details). |
| [`unwind`](#unwind) | mod | Package up unwind recovery. |
| [`compile_fail`](#compile-fail) | mod |  |
| [`BroadcastContext`](#broadcastcontext) | struct |  |
| [`ThreadBuilder`](#threadbuilder) | struct |  |
| [`Scope`](#scope) | struct |  |
| [`ScopeFifo`](#scopefifo) | struct |  |
| [`ThreadPool`](#threadpool) | struct |  |
| [`ThreadPoolBuildError`](#threadpoolbuilderror) | struct | Error when initializing a thread pool. |
| [`ThreadPoolBuilder`](#threadpoolbuilder) | struct | Used to create a new [`ThreadPool`] or to configure the global rayon thread pool. |
| [`Configuration`](#configuration) | struct | Contains the rayon thread pool configuration. |
| [`FnContext`](#fncontext) | struct | Provides the calling context to a closure called by `join_context`. |
| [`Yield`](#yield) | enum |  |
| [`ErrorKind`](#errorkind) | enum |  |
| [`broadcast`](#broadcast) | fn |  |
| [`spawn_broadcast`](#spawn-broadcast) | fn |  |
| [`join`](#join) | fn |  |
| [`join_context`](#join-context) | fn |  |
| [`in_place_scope`](#in-place-scope) | fn |  |
| [`scope`](#scope) | fn |  |
| [`in_place_scope_fifo`](#in-place-scope-fifo) | fn |  |
| [`scope_fifo`](#scope-fifo) | fn |  |
| [`spawn`](#spawn) | fn |  |
| [`spawn_fifo`](#spawn-fifo) | fn |  |
| [`current_thread_has_pending_tasks`](#current-thread-has-pending-tasks) | fn |  |
| [`current_thread_index`](#current-thread-index) | fn |  |
| [`yield_local`](#yield-local) | fn |  |
| [`yield_now`](#yield-now) | fn |  |
| [`max_num_threads`](#max-num-threads) | fn | Returns the maximum number of threads that Rayon supports in a single thread pool. |
| [`current_num_threads`](#current-num-threads) | fn | Returns the number of threads in the current registry. |
| [`initialize`](#initialize) | fn | Deprecated in favor of `ThreadPoolBuilder::build_global`. |
| [`PanicHandler`](#panichandler) | type | The type for a panic-handling closure. |
| [`StartHandler`](#starthandler) | type | The type for a closure that gets invoked when a thread starts. |
| [`ExitHandler`](#exithandler) | type | The type for a closure that gets invoked when a thread exits. |
| [`GLOBAL_POOL_ALREADY_INITIALIZED`](#global-pool-already-initialized) | const |  |
| [`CURRENT_THREAD_ALREADY_IN_POOL`](#current-thread-already-in-pool) | const |  |

## Modules

- [`private`](private/index.md) — The public parts of this private module are used to create traits
- [`broadcast`](broadcast/index.md)
- [`job`](job/index.md)
- [`join`](join/index.md)
- [`latch`](latch/index.md)
- [`registry`](registry/index.md)
- [`scope`](scope/index.md) — Methods for custom fork-join scopes, created by the [`scope()`]
- [`sleep`](sleep/index.md) — Code that decides when workers should go to sleep. See README.md
- [`spawn`](spawn/index.md)
- [`thread_pool`](thread_pool/index.md) — Contains support for user-managed thread pools, represented by the
- [`unwind`](unwind/index.md) — Package up unwind recovery. Note that if you are in some sensitive
- [`compile_fail`](compile_fail/index.md)

## Structs

### `BroadcastContext<'a>`

```rust
struct BroadcastContext<'a> {
    worker: &'a crate::registry::WorkerThread,
    _marker: std::marker::PhantomData<&'a mut dyn Fn()>,
}
```

*Defined in [`rayon-core-1.13.0/src/broadcast/mod.rs:45-50`](../../.source_1765521767/rayon-core-1.13.0/src/broadcast/mod.rs#L45-L50)*

Provides context to a closure called by `broadcast`.

#### Fields

- **`_marker`**: `std::marker::PhantomData<&'a mut dyn Fn()>`

  Make sure to prevent auto-traits like `Send` and `Sync`.

#### Implementations

- <span id="broadcastcontext-with"></span>`fn with<R>(f: impl FnOnce(BroadcastContext<'_>) -> R) -> R` — [`BroadcastContext`](broadcast/index.md#broadcastcontext)

- <span id="broadcastcontext-index"></span>`fn index(&self) -> usize`

  Our index amongst the broadcast threads (ranges from `0..self.num_threads()`).

- <span id="broadcastcontext-num-threads"></span>`fn num_threads(&self) -> usize`

  The number of threads receiving the broadcast in the thread pool.

  

  # Future compatibility note

  

  Future versions of Rayon might vary the number of threads over time, but

  this method will always return the number of threads which are actually

  receiving your particular `broadcast` call.

#### Trait Implementations

##### `impl Any for BroadcastContext<'a>`

- <span id="broadcastcontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BroadcastContext<'a>`

- <span id="broadcastcontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BroadcastContext<'a>`

- <span id="broadcastcontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for BroadcastContext<'a>`

- <span id="broadcastcontext-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BroadcastContext<'a>`

- <span id="broadcastcontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BroadcastContext<'a>`

- <span id="broadcastcontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for BroadcastContext<'a>`

- <span id="broadcastcontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="broadcastcontext-pointable-type-init"></span>`type Init = T`

- <span id="broadcastcontext-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="broadcastcontext-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="broadcastcontext-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="broadcastcontext-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for BroadcastContext<'a>`

- <span id="broadcastcontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="broadcastcontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BroadcastContext<'a>`

- <span id="broadcastcontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="broadcastcontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThreadBuilder`

```rust
struct ThreadBuilder {
    name: Option<String>,
    stack_size: Option<usize>,
    worker: crossbeam_deque::Worker<crate::job::JobRef>,
    stealer: crossbeam_deque::Stealer<crate::job::JobRef>,
    registry: std::sync::Arc<Registry>,
    index: usize,
}
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:22-29`](../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L22-L29)*

Thread builder used for customization via `ThreadPoolBuilder::spawn_handler()`.

#### Implementations

- <span id="threadbuilder-index"></span>`fn index(&self) -> usize`

  Gets the index of this thread in the pool, within `0..num_threads`.

- <span id="threadbuilder-name"></span>`fn name(&self) -> Option<&str>`

  Gets the string that was specified by `ThreadPoolBuilder::name()`.

- <span id="threadbuilder-stack-size"></span>`fn stack_size(&self) -> Option<usize>`

  Gets the value that was specified by `ThreadPoolBuilder::stack_size()`.

- <span id="threadbuilder-run"></span>`fn run(self)`

  Executes the main loop for this thread. This will not return until the

  thread pool is dropped.

#### Trait Implementations

##### `impl Any for ThreadBuilder`

- <span id="threadbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadBuilder`

- <span id="threadbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadBuilder`

- <span id="threadbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ThreadBuilder`

- <span id="threadbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ThreadBuilder`

- <span id="threadbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreadBuilder`

- <span id="threadbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ThreadBuilder`

- <span id="threadbuilder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="threadbuilder-pointable-type-init"></span>`type Init = T`

- <span id="threadbuilder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadbuilder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadbuilder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadbuilder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ThreadBuilder`

- <span id="threadbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreadBuilder`

- <span id="threadbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Scope<'scope>`

```rust
struct Scope<'scope> {
    base: ScopeBase<'scope>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:24-26`](../../.source_1765521767/rayon-core-1.13.0/src/scope/mod.rs#L24-L26)*

Represents a fork-join scope which can be used to spawn any number of tasks.
See [`scope()`](scope/index.md) for more information.

#### Implementations

- <span id="scope-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](registry/index.md#workerthread), [`Registry`](registry/index.md#registry)

- <span id="scope-spawn"></span>`fn spawn<BODY>(&self, body: BODY)`

  Spawns a job into the fork-join scope `self`. This job will

  execute sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its

  own reference to the scope `self` as argument. This can be

  used to inject new jobs into `self`.

  

  # Returns

  

  Nothing. The spawned closures cannot pass back values to the

  caller directly, though they can write to local variables on

  the stack (if those variables outlive the scope) or

  communicate through shared channels.

  

  (The intention is to eventually integrate with Rust futures to

  support spawns of functions that compute a value.)

  

  # Examples

  

  ```rust

  use rayon_core as rayon;

  let mut value_a = None;

  let mut value_b = None;

  let mut value_c = None;

  rayon::scope(|s| {

      s.spawn(|s1| {

            // ^ this is the same scope as `s`; this handle `s1`

            //   is intended for use by the spawned task,

            //   since scope handles cannot cross thread boundaries.

  

          value_a = Some(22);

  

          // the scope `s` will not end until all these tasks are done

          s1.spawn(|_| {

              value_b = Some(44);

          });

      });

  

      s.spawn(|_| {

          value_c = Some(66);

      });

  });

  assert_eq!(value_a, Some(22));

  assert_eq!(value_b, Some(44));

  assert_eq!(value_c, Some(66));

  ```

  

  # See also

  

  The [`scope` function] has more extensive documentation about

  task spawning.

- <span id="scope-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

  Spawns a job into every thread of the fork-join scope `self`. This job will

  execute on each thread sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its own reference

  to the scope `self` as argument, as well as a `BroadcastContext`.

#### Trait Implementations

##### `impl Any for Scope<'scope>`

- <span id="scope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Scope<'scope>`

- <span id="scope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Scope<'scope>`

- <span id="scope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Scope<'scope>`

- <span id="scope-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Scope<'scope>`

- <span id="scope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Scope<'scope>`

- <span id="scope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Scope<'scope>`

- <span id="scope-pointable-const-align"></span>`const ALIGN: usize`

- <span id="scope-pointable-type-init"></span>`type Init = T`

- <span id="scope-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scope-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scope-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scope-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Scope<'scope>`

- <span id="scope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Scope<'scope>`

- <span id="scope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScopeFifo<'scope>`

```rust
struct ScopeFifo<'scope> {
    base: ScopeBase<'scope>,
    fifos: Vec<crate::job::JobFifo>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:31-34`](../../.source_1765521767/rayon-core-1.13.0/src/scope/mod.rs#L31-L34)*

Represents a fork-join scope which can be used to spawn any number of tasks.
Those spawned from the same thread are prioritized in relative FIFO order.
See [`scope_fifo()`](scope/index.md) for more information.

#### Implementations

- <span id="scopefifo-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](registry/index.md#workerthread), [`Registry`](registry/index.md#registry)

- <span id="scopefifo-spawn-fifo"></span>`fn spawn_fifo<BODY>(&self, body: BODY)`

  Spawns a job into the fork-join scope `self`. This job will

  execute sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its

  own reference to the scope `self` as argument. This can be

  used to inject new jobs into `self`.

  

  # See also

  

  This method is akin to `Scope::spawn()`, but with a FIFO

  priority.  The [`scope_fifo` function] has more details about

  this distinction.

- <span id="scopefifo-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

  Spawns a job into every thread of the fork-join scope `self`. This job will

  execute on each thread sometime before the fork-join scope completes.  The

  job is specified as a closure, and this closure receives its own reference

  to the scope `self` as argument, as well as a `BroadcastContext`.

#### Trait Implementations

##### `impl Any for ScopeFifo<'scope>`

- <span id="scopefifo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopeFifo<'scope>`

- <span id="scopefifo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopeFifo<'scope>`

- <span id="scopefifo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ScopeFifo<'scope>`

- <span id="scopefifo-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ScopeFifo<'scope>`

- <span id="scopefifo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScopeFifo<'scope>`

- <span id="scopefifo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ScopeFifo<'scope>`

- <span id="scopefifo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="scopefifo-pointable-type-init"></span>`type Init = T`

- <span id="scopefifo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopefifo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopefifo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopefifo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ScopeFifo<'scope>`

- <span id="scopefifo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopefifo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScopeFifo<'scope>`

- <span id="scopefifo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopefifo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThreadPool`

```rust
struct ThreadPool {
    registry: std::sync::Arc<crate::registry::Registry>,
}
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:46-48`](../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L46-L48)*

Represents a user-created [thread pool].

Use a [`ThreadPoolBuilder`](#threadpoolbuilder) to specify the number and/or names of threads
in the pool. After calling `ThreadPoolBuilder::build()`, you can then
execute functions explicitly within this [`ThreadPool`](thread_pool/index.md) using
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

- <span id="threadpool-new"></span>`fn new(configuration: crate::Configuration) -> Result<ThreadPool, Box<dyn Error>>` — [`Configuration`](#configuration), [`ThreadPool`](thread_pool/index.md#threadpool)

  Deprecated in favor of `ThreadPoolBuilder::build`.

- <span id="threadpool-build"></span>`fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](#threadpoolbuilder), [`ThreadPool`](thread_pool/index.md#threadpool), [`ThreadPoolBuildError`](#threadpoolbuilderror)

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

  [`ThreadPoolBuilder`](#threadpoolbuilder) that specifies the number of threads,

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

  key idea is that each thread has its own [`deque`](../crossbeam_deque/deque/index.md) of

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

  

  See also: [the `spawn()` function defined on scopes][`spawn`](spawn/index.md).

- <span id="threadpool-spawn-fifo"></span>`fn spawn_fifo<OP>(&self, op: OP)`

  Spawns an asynchronous task in this thread pool. This task will

  run in the implicit, global scope, which means that it may outlast

  the current stack frame -- therefore, it cannot capture any references

  onto the stack (you will likely need a `move` closure).

  

  See also: [the `spawn_fifo()` function defined on scopes][`spawn_fifo`](spawn/index.md).

- <span id="threadpool-spawn-broadcast"></span>`fn spawn_broadcast<OP>(&self, op: OP)`

  Spawns an asynchronous task on every thread in this thread pool. This task

  will run in the implicit, global scope, which means that it may outlast the

  current stack frame -- therefore, it cannot capture any references onto the

  stack (you will likely need a `move` closure).

- <span id="threadpool-yield-now"></span>`fn yield_now(&self) -> Option<Yield>` — [`Yield`](thread_pool/index.md#yield)

  Cooperatively yields execution to Rayon.

  

  This is similar to the general [`yield_now()`](thread_pool/index.md), but only if the current

  thread is part of *this* thread pool.

  

  Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if

  nothing was available, or `None` if the current thread is not part this pool.

- <span id="threadpool-yield-local"></span>`fn yield_local(&self) -> Option<Yield>` — [`Yield`](thread_pool/index.md#yield)

  Cooperatively yields execution to local Rayon work.

  

  This is similar to the general [`yield_local()`](thread_pool/index.md), but only if the current

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

### `ThreadPoolBuildError`

```rust
struct ThreadPoolBuildError {
    kind: ErrorKind,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:142-144`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L142-L144)*

Error when initializing a thread pool.

#### Implementations

- <span id="threadpoolbuilderror-new"></span>`fn new(kind: ErrorKind) -> ThreadPoolBuildError` — [`ErrorKind`](#errorkind), [`ThreadPoolBuildError`](#threadpoolbuilderror)

- <span id="threadpoolbuilderror-is-unsupported"></span>`fn is_unsupported(&self) -> bool`

#### Trait Implementations

##### `impl Any for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-error-description"></span>`fn description(&self) -> &str`

- <span id="threadpoolbuilderror-error-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl<T> From for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-pointable-const-align"></span>`const ALIGN: usize`

- <span id="threadpoolbuilderror-pointable-type-init"></span>`type Init = T`

- <span id="threadpoolbuilderror-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpoolbuilderror-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpoolbuilderror-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpoolbuilderror-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToString for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadpoolbuilderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadpoolbuilderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThreadPoolBuilder<S>`

```rust
struct ThreadPoolBuilder<S> {
    num_threads: usize,
    use_current_thread: bool,
    panic_handler: Option<Box<dyn Fn(Box<dyn Any + Send>) + Send + Sync>>,
    get_thread_name: Option<Box<dyn FnMut(usize) -> String>>,
    stack_size: Option<usize>,
    start_handler: Option<Box<dyn Fn(usize) + Send + Sync>>,
    exit_handler: Option<Box<dyn Fn(usize) + Send + Sync>>,
    spawn_handler: S,
    breadth_first: bool,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:170-202`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L170-L202)*

Used to create a new [`ThreadPool`](thread_pool/index.md) or to configure the global rayon thread pool.
## Creating a ThreadPool
The following creates a thread pool with 22 threads.

```ignore-wasm
use rayon_core as rayon;
let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();
```

To instead configure the global thread pool, use `build_global()`:

```ignore-wasm
use rayon_core as rayon;
rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
```


#### Fields

- **`num_threads`**: `usize`

  The number of threads in the rayon thread pool.
  If zero will use the RAYON_NUM_THREADS environment variable.
  If RAYON_NUM_THREADS is invalid or zero will use the default.

- **`use_current_thread`**: `bool`

  The thread we're building *from* will also be part of the pool.

- **`panic_handler`**: `Option<Box<dyn Fn(Box<dyn Any + Send>) + Send + Sync>>`

  Custom closure, if any, to handle a panic that we cannot propagate
  anywhere else.

- **`get_thread_name`**: `Option<Box<dyn FnMut(usize) -> String>>`

  Closure to compute the name of a thread.

- **`stack_size`**: `Option<usize>`

  The stack size for the created worker threads

- **`start_handler`**: `Option<Box<dyn Fn(usize) + Send + Sync>>`

  Closure invoked on worker-thread start.

- **`exit_handler`**: `Option<Box<dyn Fn(usize) + Send + Sync>>`

  Closure invoked on worker-thread exit.

- **`spawn_handler`**: `S`

  Closure invoked to spawn threads.

- **`breadth_first`**: `bool`

  If false, worker threads will execute spawned jobs in a
  "depth-first" fashion. If true, they will do a "breadth-first"
  fashion. Depth-first is the default.

#### Implementations

- <span id="threadpoolbuilder-new"></span>`fn new() -> Self`

  Creates and returns a valid rayon thread pool builder, but does not initialize it.

#### Trait Implementations

##### `impl Any for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S> Debug for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ThreadPoolBuilder`

- <span id="threadpoolbuilder-default"></span>`fn default() -> Self`

##### `impl<T> From for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="threadpoolbuilder-pointable-type-init"></span>`type Init = T`

- <span id="threadpoolbuilder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpoolbuilder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpoolbuilder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpoolbuilder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadpoolbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadpoolbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Configuration`

```rust
struct Configuration {
    builder: ThreadPoolBuilder,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:207-209`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L207-L209)*

Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`](#threadpoolbuilder) instead.

#### Implementations

- <span id="configuration-new"></span>`fn new() -> Configuration` — [`Configuration`](#configuration)

  Creates and return a valid rayon thread pool configuration, but does not initialize it.

- <span id="configuration-build"></span>`fn build(self) -> Result<ThreadPool, Box<dyn Error>>` — [`ThreadPool`](thread_pool/index.md#threadpool)

  Deprecated in favor of `ThreadPoolBuilder::build`.

- <span id="configuration-thread-name"></span>`fn thread_name<F>(self, closure: F) -> Self`

  Deprecated in favor of `ThreadPoolBuilder::thread_name`.

- <span id="configuration-num-threads"></span>`fn num_threads(self, num_threads: usize) -> Configuration` — [`Configuration`](#configuration)

  Deprecated in favor of `ThreadPoolBuilder::num_threads`.

- <span id="configuration-panic-handler"></span>`fn panic_handler<H>(self, panic_handler: H) -> Configuration` — [`Configuration`](#configuration)

  Deprecated in favor of `ThreadPoolBuilder::panic_handler`.

- <span id="configuration-stack-size"></span>`fn stack_size(self, stack_size: usize) -> Self`

  Deprecated in favor of `ThreadPoolBuilder::stack_size`.

- <span id="configuration-breadth-first"></span>`fn breadth_first(self) -> Self`

  Deprecated in favor of `ThreadPoolBuilder::breadth_first`.

- <span id="configuration-start-handler"></span>`fn start_handler<H>(self, start_handler: H) -> Configuration` — [`Configuration`](#configuration)

  Deprecated in favor of `ThreadPoolBuilder::start_handler`.

- <span id="configuration-exit-handler"></span>`fn exit_handler<H>(self, exit_handler: H) -> Configuration` — [`Configuration`](#configuration)

  Deprecated in favor of `ThreadPoolBuilder::exit_handler`.

- <span id="configuration-into-builder"></span>`fn into_builder(self) -> ThreadPoolBuilder` — [`ThreadPoolBuilder`](#threadpoolbuilder)

  Returns a ThreadPoolBuilder with identical parameters.

#### Trait Implementations

##### `impl Any for Configuration`

- <span id="configuration-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Configuration`

- <span id="configuration-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Configuration`

- <span id="configuration-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Configuration`

- <span id="configuration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Configuration`

- <span id="configuration-default"></span>`fn default() -> Configuration` — [`Configuration`](#configuration)

##### `impl<T> From for Configuration`

- <span id="configuration-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Configuration`

- <span id="configuration-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Configuration`

- <span id="configuration-pointable-const-align"></span>`const ALIGN: usize`

- <span id="configuration-pointable-type-init"></span>`type Init = T`

- <span id="configuration-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="configuration-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="configuration-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="configuration-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Configuration`

- <span id="configuration-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="configuration-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Configuration`

- <span id="configuration-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="configuration-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FnContext`

```rust
struct FnContext {
    migrated: bool,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:840-845`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L840-L845)*

Provides the calling context to a closure called by `join_context`.

#### Fields

- **`_marker`**: `std::marker::PhantomData<*mut ()>`

  disable `Send` and `Sync`, just for a little future-proofing.

#### Implementations

- <span id="fncontext-new"></span>`fn new(migrated: bool) -> Self`

#### Trait Implementations

##### `impl Any for FnContext`

- <span id="fncontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FnContext`

- <span id="fncontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FnContext`

- <span id="fncontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FnContext`

- <span id="fncontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FnContext`

- <span id="fncontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FnContext`

- <span id="fncontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for FnContext`

- <span id="fncontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fncontext-pointable-type-init"></span>`type Init = T`

- <span id="fncontext-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fncontext-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fncontext-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fncontext-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FnContext`

- <span id="fncontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fncontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FnContext`

- <span id="fncontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fncontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Yield`

```rust
enum Yield {
    Executed,
    Idle,
}
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:497-502`](../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L497-L502)*

Result of [`yield_now()`](thread_pool/index.md) or [`yield_local()`](thread_pool/index.md).

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

- <span id="yield-clone"></span>`fn clone(&self) -> Yield` — [`Yield`](thread_pool/index.md#yield)

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

- <span id="yield-partialeq-eq"></span>`fn eq(&self, other: &Yield) -> bool` — [`Yield`](thread_pool/index.md#yield)

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

### `ErrorKind`

```rust
enum ErrorKind {
    GlobalPoolAlreadyInitialized,
    CurrentThreadAlreadyInPool,
    IOError(io::Error),
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:147-151`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L147-L151)*

#### Trait Implementations

##### `impl Any for ErrorKind`

- <span id="errorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorKind`

- <span id="errorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorKind`

- <span id="errorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ErrorKind`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ErrorKind`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ErrorKind`

- <span id="errorkind-pointable-const-align"></span>`const ALIGN: usize`

- <span id="errorkind-pointable-type-init"></span>`type Init = T`

- <span id="errorkind-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="errorkind-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="errorkind-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="errorkind-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ErrorKind`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `broadcast`

```rust
fn broadcast<OP, R>(op: OP) -> Vec<R>
where
    OP: Fn(BroadcastContext<'_>) -> R + Sync,
    R: Send
```

*Defined in [`rayon-core-1.13.0/src/broadcast/mod.rs:19-26`](../../.source_1765521767/rayon-core-1.13.0/src/broadcast/mod.rs#L19-L26)*

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

*Defined in [`rayon-core-1.13.0/src/broadcast/mod.rs:36-42`](../../.source_1765521767/rayon-core-1.13.0/src/broadcast/mod.rs#L36-L42)*

Spawns an asynchronous task on every thread in this thread pool. This task
will run in the implicit, global scope, which means that it may outlast the
current stack frame -- therefore, it cannot capture any references onto the
stack (you will likely need a `move` closure).

For more information, see the `ThreadPool::spawn_broadcast()` method.


### `join`

```rust
fn join<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
where
    A: FnOnce() -> RA + Send,
    B: FnOnce() -> RB + Send,
    RA: Send,
    RB: Send
```

*Defined in [`rayon-core-1.13.0/src/join/mod.rs:93-106`](../../.source_1765521767/rayon-core-1.13.0/src/join/mod.rs#L93-L106)*

Takes two closures and *potentially* runs them in parallel. It
returns a pair of the results from those closures.

Conceptually, calling `join()` is similar to spawning two threads,
one executing each of the two closures. However, the
implementation is quite different and incurs very low
overhead. The underlying technique is called "work stealing": the
Rayon runtime uses a fixed pool of worker threads and attempts to
only execute code in parallel when there are idle CPUs to handle
it.

When `join` is called from outside the thread pool, the calling
thread will block while the closures execute in the pool.  When
`join` is called within the pool, the calling thread still actively
participates in the thread pool. It will begin by executing closure
A (on the current thread). While it is doing that, it will advertise
closure B as being available for other threads to execute. Once closure A
has completed, the current thread will try to execute closure B;
if however closure B has been stolen, then it will look for other work
while waiting for the thief to fully execute closure B. (This is the
typical work-stealing strategy).

# Examples

This example uses join to perform a quick-sort (note this is not a
particularly optimized implementation: if you **actually** want to
sort for real, you should prefer [the `par_sort` method] offered
by Rayon).

```rust
use rayon_core as rayon;
let mut v = vec![5, 1, 8, 22, 0, 44];
quick_sort(&mut v);
assert_eq!(v, vec![0, 1, 5, 8, 22, 44]);

fn quick_sort<T:PartialOrd+Send>(v: &mut [T]) {
   if v.len() > 1 {
       let mid = partition(v);
       let (lo, hi) = v.split_at_mut(mid);
       rayon::join(|| quick_sort(lo),
                   || quick_sort(hi));
   }
}

// Partition rearranges all items `<=` to the pivot
// item (arbitrary selected to be the last item in the slice)
// to the first half of the slice. It then returns the
// "dividing point" where the pivot is placed.
fn partition<T:PartialOrd+Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
```

# Warning about blocking I/O

The assumption is that the closures given to `join()` are
CPU-bound tasks that do not perform I/O or other blocking
operations. If you do perform I/O, and that I/O should block
(e.g., waiting for a network request), the overall performance may
be poor.  Moreover, if you cause one closure to be blocked waiting
on another (for example, using a channel), that could lead to a
deadlock.

# Panics

No matter what happens, both closures will always be executed.  If
a single closure panics, whether it be the first or second
closure, that panic will be propagated and hence `join()` will
panic with the same panic value. If both closures panic, `join()`
will panic with the panic value from the first closure.

### `join_context`

```rust
fn join_context<A, B, RA, RB>(oper_a: A, oper_b: B) -> (RA, RB)
where
    A: FnOnce(crate::FnContext) -> RA + Send,
    B: FnOnce(crate::FnContext) -> RB + Send,
    RA: Send,
    RB: Send
```

*Defined in [`rayon-core-1.13.0/src/join/mod.rs:115-173`](../../.source_1765521767/rayon-core-1.13.0/src/join/mod.rs#L115-L173)*

Identical to `join`, except that the closures have a parameter
that provides context for the way the closure has been called,
especially indicating whether they're executing on a different
thread than where `join_context` was called.  This will occur if
the second job is stolen by a different thread, or if
`join_context` was called from outside the thread pool to begin
with.

### `in_place_scope`

```rust
fn in_place_scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:398-403`](../../.source_1765521767/rayon-core-1.13.0/src/scope/mod.rs#L398-L403)*

Creates a "fork-join" scope `s` and invokes the closure with a
reference to `s`. This closure can then spawn asynchronous tasks
into `s`. Those tasks may run asynchronously with respect to the
closure; they may themselves spawn additional tasks into `s`. When
the closure returns, it will block until all tasks that have been
spawned into `s` complete.

This is just like `scope()` except the closure runs on the same thread
that calls `in_place_scope()`. Only work that it spawns runs in the
thread pool.

# Panics

If a panic occurs, either in the closure given to `in_place_scope()` or in
any of the spawned jobs, that panic will be propagated and the
call to `in_place_scope()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn()`, it will
execute, even if the spawning task should later panic. `in_place_scope()`
returns once all spawned jobs have completed, and any panics are
propagated at that point.

### `scope`

```rust
fn scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&Scope<'scope>) -> R + Send,
    R: Send
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:277-286`](../../.source_1765521767/rayon-core-1.13.0/src/scope/mod.rs#L277-L286)*

Creates a "fork-join" scope `s` and invokes the closure with a
reference to `s`. This closure can then spawn asynchronous tasks
into `s`. Those tasks may run asynchronously with respect to the
closure; they may themselves spawn additional tasks into `s`. When
the closure returns, it will block until all tasks that have been
spawned into `s` complete.

`scope()` is a more flexible building block compared to `join()`,
since a loop can be used to spawn any number of tasks without
recursing. However, that flexibility comes at a performance price:
tasks spawned using `scope()` must be allocated onto the heap,
whereas `join()` can make exclusive use of the stack. **Prefer
`join()` (or, even better, parallel iterators) where possible.**

# Example

The Rayon `join()` function launches two closures and waits for them
to stop. One could implement `join()` using a scope like so, although
it would be less efficient than the real implementation:

```rust
use rayon_core as rayon;
pub fn join<A,B,RA,RB>(oper_a: A, oper_b: B) -> (RA, RB)
    where A: FnOnce() -> RA + Send,
          B: FnOnce() -> RB + Send,
          RA: Send,
          RB: Send,
{
    let mut result_a: Option<RA> = None;
    let mut result_b: Option<RB> = None;
    rayon::scope(|s| {
        s.spawn(|_| result_a = Some(oper_a()));
        s.spawn(|_| result_b = Some(oper_b()));
    });
    (result_a.unwrap(), result_b.unwrap())
}
```

# A note on threading

The closure given to `scope()` executes in the Rayon thread pool,
as do those given to `spawn()`. This means that you can't access
thread-local variables (well, you can, but they may have
unexpected values).

# Task execution

Task execution potentially starts as soon as `spawn()` is called.
The task will end sometime before `scope()` returns. Note that the
*closure* given to scope may return much earlier. In general
the lifetime of a scope created like `scope(body)` goes something like this:

- Scope begins when `scope(body)` is called
- Scope body `body()` is invoked
    - Scope tasks may be spawned
- Scope body returns
- Scope tasks execute, possibly spawning more tasks
- Once all tasks are done, scope ends and `scope()` returns

To see how and when tasks are joined, consider this example:

```rust
use rayon_core as rayon;
// point start
rayon::scope(|s| {
    s.spawn(|s| { // task s.1
        s.spawn(|s| { // task s.1.1
            rayon::scope(|t| {
                t.spawn(|_| ()); // task t.1
                t.spawn(|_| ()); // task t.2
            });
        });
    });
    s.spawn(|s| { // task s.2
    });
    // point mid
});
// point end
```

The various tasks that are run will execute roughly like so:

```notrust
| (start)
|
| (scope `s` created)
+-----------------------------------------------+ (task s.2)
+-------+ (task s.1)                            |
|       |                                       |
|       +---+ (task s.1.1)                      |
|       |   |                                   |
|       |   | (scope `t` created)               |
|       |   +----------------+ (task t.2)       |
|       |   +---+ (task t.1) |                  |
| (mid) |   |   |            |                  |
:       |   + <-+------------+ (scope `t` ends) |
:       |   |                                   |
|<------+---+-----------------------------------+ (scope `s` ends)
|
| (end)
```

The point here is that everything spawned into scope `s` will
terminate (at latest) at the same point -- right before the
original call to `rayon::scope` returns. This includes new
subtasks created by other subtasks (e.g., task `s.1.1`). If a new
scope is created (such as `t`), the things spawned into that scope
will be joined before that scope returns, which in turn occurs
before the creating task (task `s.1.1` in this case) finishes.

There is no guaranteed order of execution for spawns in a scope,
given that other threads may steal tasks at any time. However, they
are generally prioritized in a LIFO order on the thread from which
they were spawned. So in this example, absent any stealing, we can
expect `s.2` to execute before `s.1`, and `t.2` before `t.1`. Other
threads always steal from the other end of the deque, like FIFO
order.  The idea is that "recent" tasks are most likely to be fresh
in the local CPU's cache, while other threads can steal older
"stale" tasks.  For an alternate approach, consider
[`scope_fifo()`](scope/index.md) instead.

# Accessing stack data

In general, spawned tasks may access stack data in place that
outlives the scope itself. Other data must be fully owned by the
spawned task.

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    s.spawn(|_| {
        // We can access `ok` because outlives the scope `s`.
        println!("ok: {:?}", ok);

        // If we just try to use `bad` here, the closure will borrow `bad`
        // (because we are just printing it out, and that only requires a
        // borrow), which will result in a compilation error. Read on
        // for options.
        // println!("bad: {:?}", bad);
   });
});
```

As the comments example above suggest, to reference `bad` we must
take ownership of it. One way to do this is to detach the closure
from the surrounding stack frame, using the `move` keyword. This
will cause it to take ownership of *all* the variables it touches,
in this case including both `ok` *and* `bad`:

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    s.spawn(move |_| {
        println!("ok: {:?}", ok);
        println!("bad: {:?}", bad);
    });

    // That closure is fine, but now we can't use `ok` anywhere else,
    // since it is owned by the previous task:
    // s.spawn(|_| println!("ok: {:?}", ok));
});
```

While this works, it could be a problem if we want to use `ok` elsewhere.
There are two choices. We can keep the closure as a `move` closure, but
instead of referencing the variable `ok`, we create a shadowed variable that
is a borrow of `ok` and capture *that*:

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    let ok: &Vec<i32> = &ok; // shadow the original `ok`
    s.spawn(move |_| {
        println!("ok: {:?}", ok); // captures the shadowed version
        println!("bad: {:?}", bad);
    });

    // Now we too can use the shadowed `ok`, since `&Vec<i32>` references
    // can be shared freely. Note that we need a `move` closure here though,
    // because otherwise we'd be trying to borrow the shadowed `ok`,
    // and that doesn't outlive `scope`.
    s.spawn(move |_| println!("ok: {:?}", ok));
});
```

Another option is not to use the `move` keyword but instead to take ownership
of individual variables:

```rust
use rayon_core as rayon;
let ok: Vec<i32> = vec![1, 2, 3];
rayon::scope(|s| {
    let bad: Vec<i32> = vec![4, 5, 6];
    s.spawn(|_| {
        // Transfer ownership of `bad` into a local variable (also named `bad`).
        // This will force the closure to take ownership of `bad` from the environment.
        let bad = bad;
        println!("ok: {:?}", ok); // `ok` is only borrowed.
        println!("bad: {:?}", bad); // refers to our local variable, above.
    });

    s.spawn(|_| println!("ok: {:?}", ok)); // we too can borrow `ok`
});
```

# Panics

If a panic occurs, either in the closure given to `scope()` or in
any of the spawned jobs, that panic will be propagated and the
call to `scope()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn()`, it will
execute, even if the spawning task should later panic. `scope()`
returns once all spawned jobs have completed, and any panics are
propagated at that point.

### `in_place_scope_fifo`

```rust
fn in_place_scope_fifo<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&ScopeFifo<'scope>) -> R
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:449-454`](../../.source_1765521767/rayon-core-1.13.0/src/scope/mod.rs#L449-L454)*

Creates a "fork-join" scope `s` with FIFO order, and invokes the
closure with a reference to `s`. This closure can then spawn
asynchronous tasks into `s`. Those tasks may run asynchronously with
respect to the closure; they may themselves spawn additional tasks
into `s`. When the closure returns, it will block until all tasks
that have been spawned into `s` complete.

This is just like `scope_fifo()` except the closure runs on the same thread
that calls `in_place_scope_fifo()`. Only work that it spawns runs in the
thread pool.

# Panics

If a panic occurs, either in the closure given to `in_place_scope_fifo()` or in
any of the spawned jobs, that panic will be propagated and the
call to `in_place_scope_fifo()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn_fifo()`, it will
execute, even if the spawning task should later panic. `in_place_scope_fifo()`
returns once all spawned jobs have completed, and any panics are
propagated at that point.

### `scope_fifo`

```rust
fn scope_fifo<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&ScopeFifo<'scope>) -> R + Send,
    R: Send
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:366-375`](../../.source_1765521767/rayon-core-1.13.0/src/scope/mod.rs#L366-L375)*

Creates a "fork-join" scope `s` with FIFO order, and invokes the
closure with a reference to `s`. This closure can then spawn
asynchronous tasks into `s`. Those tasks may run asynchronously with
respect to the closure; they may themselves spawn additional tasks
into `s`. When the closure returns, it will block until all tasks
that have been spawned into `s` complete.

# Task execution

Tasks in a `scope_fifo()` run similarly to [`scope()`](scope/index.md), but there's a
difference in the order of execution. Consider a similar example:

```rust
use rayon_core as rayon;
// point start
rayon::scope_fifo(|s| {
    s.spawn_fifo(|s| { // task s.1
        s.spawn_fifo(|s| { // task s.1.1
            rayon::scope_fifo(|t| {
                t.spawn_fifo(|_| ()); // task t.1
                t.spawn_fifo(|_| ()); // task t.2
            });
        });
    });
    s.spawn_fifo(|s| { // task s.2
    });
    // point mid
});
// point end
```

The various tasks that are run will execute roughly like so:

```notrust
| (start)
|
| (FIFO scope `s` created)
+--------------------+ (task s.1)
+-------+ (task s.2) |
|       |            +---+ (task s.1.1)
|       |            |   |
|       |            |   | (FIFO scope `t` created)
|       |            |   +----------------+ (task t.1)
|       |            |   +---+ (task t.2) |
| (mid) |            |   |   |            |
:       |            |   + <-+------------+ (scope `t` ends)
:       |            |   |
|<------+------------+---+ (scope `s` ends)
|
| (end)
```

Under `scope_fifo()`, the spawns are prioritized in a FIFO order on
the thread from which they were spawned, as opposed to `scope()`'s
LIFO.  So in this example, we can expect `s.1` to execute before
`s.2`, and `t.1` before `t.2`. Other threads also steal tasks in
FIFO order, as usual. Overall, this has roughly the same order as
the now-deprecated `breadth_first` option, except the effect is
isolated to a particular scope. If spawns are intermingled from any
combination of `scope()` and `scope_fifo()`, or from different
threads, their order is only specified with respect to spawns in the
same scope and thread.

For more details on this design, see Rayon [RFC #1].


# Panics

If a panic occurs, either in the closure given to `scope_fifo()` or
in any of the spawned jobs, that panic will be propagated and the
call to `scope_fifo()` will panic. If multiple panics occurs, it is
non-deterministic which of their panic values will propagate.
Regardless, once a task is spawned using `scope.spawn_fifo()`, it
will execute, even if the spawning task should later panic.
`scope_fifo()` returns once all spawned jobs have completed, and any
panics are propagated at that point.

### `spawn`

```rust
fn spawn<F>(func: F)
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:58-64`](../../.source_1765521767/rayon-core-1.13.0/src/spawn/mod.rs#L58-L64)*

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
consider [`spawn_fifo()`](spawn/index.md) instead.

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

### `spawn_fifo`

```rust
fn spawn_fifo<F>(func: F)
where
    F: FnOnce() + Send + 'static
```

*Defined in [`rayon-core-1.13.0/src/spawn/mod.rs:130-136`](../../.source_1765521767/rayon-core-1.13.0/src/spawn/mod.rs#L130-L136)*

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


### `current_thread_has_pending_tasks`

```rust
fn current_thread_has_pending_tasks() -> Option<bool>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:452-457`](../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L452-L457)*

If called from a Rayon worker thread, indicates whether that
thread's local deque still has pending tasks. Otherwise, returns
`None`. For more information, see [the
`ThreadPool::current_thread_has_pending_tasks()` method][m].


### `current_thread_index`

```rust
fn current_thread_index() -> Option<usize>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:438-443`](../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L438-L443)*

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


### `yield_local`

```rust
fn yield_local() -> Option<Yield>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:488-493`](../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L488-L493)*

Cooperatively yields execution to local Rayon work.

If the current thread is part of a rayon thread pool, this looks for a
single unit of pending work in this thread's queue, then executes it.
Completion of that work might include nested work or further work stealing.

This is similar to [`yield_now()`](thread_pool/index.md), but does not steal from other threads.

Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if
nothing was available, or `None` if this thread is not part of any pool at all.

### `yield_now`

```rust
fn yield_now() -> Option<Yield>
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:471-476`](../../.source_1765521767/rayon-core-1.13.0/src/thread_pool/mod.rs#L471-L476)*

Cooperatively yields execution to Rayon.

If the current thread is part of a rayon thread pool, this looks for a
single unit of pending work in the pool, then executes it. Completion of
that work might include nested work or further work stealing.

This is similar to [`std::thread::yield_now()`](../rayon/index.md), but does not literally make
that call. If you are implementing a polling loop, you may want to also
yield to the OS scheduler yourself if no Rayon work was found.

Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if
nothing was available, or `None` if this thread is not part of any pool at all.

### `max_num_threads`

```rust
fn max_num_threads() -> usize
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:113-116`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L113-L116)*

Returns the maximum number of threads that Rayon supports in a single thread pool.

If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting
the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum.

The value may vary between different targets, and is subject to change in new Rayon versions.

### `current_num_threads`

```rust
fn current_num_threads() -> usize
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:136-138`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L136-L138)*

Returns the number of threads in the current registry. If this
code is executing within a Rayon thread pool, then this will be
the number of threads for the thread pool of the current
thread. Otherwise, it will be the number of threads for the global
thread pool.

This can be useful when trying to judge how many times to split
parallel work (the parallel iterator traits use this value
internally for this purpose).

# Future compatibility note

Note that unless this thread pool was created with a
builder that specifies the number of threads, then this
number may vary over time in future versions (see [the
`num_threads()` method for details][snt]).


### `initialize`

```rust
fn initialize(config: Configuration) -> Result<(), Box<dyn Error>>
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:787-789`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L787-L789)*

Deprecated in favor of `ThreadPoolBuilder::build_global`.

## Type Aliases

### `PanicHandler`

```rust
type PanicHandler = dyn Fn(Box<dyn Any + Send>) + Send + Sync;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:213`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L213)*

The type for a panic-handling closure. Note that this same closure
may be invoked multiple times in parallel.

### `StartHandler`

```rust
type StartHandler = dyn Fn(usize) + Send + Sync;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:218`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L218)*

The type for a closure that gets invoked when a thread starts. The
closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.

### `ExitHandler`

```rust
type ExitHandler = dyn Fn(usize) + Send + Sync;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:223`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L223)*

The type for a closure that gets invoked when a thread exits. The
closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.

## Constants

### `GLOBAL_POOL_ALREADY_INITIALIZED`
```rust
const GLOBAL_POOL_ALREADY_INITIALIZED: &str;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:750-751`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L750-L751)*

### `CURRENT_THREAD_ALREADY_IN_POOL`
```rust
const CURRENT_THREAD_ALREADY_IN_POOL: &str;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:753-754`](../../.source_1765521767/rayon-core-1.13.0/src/lib.rs#L753-L754)*

