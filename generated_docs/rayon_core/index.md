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
  - [`thread_pool`](#thread_pool)
  - [`unwind`](#unwind)
  - [`compile_fail`](#compile_fail)
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
  - [`spawn_broadcast`](#spawn_broadcast)
  - [`join`](#join)
  - [`join_context`](#join_context)
  - [`in_place_scope`](#in_place_scope)
  - [`scope`](#scope)
  - [`in_place_scope_fifo`](#in_place_scope_fifo)
  - [`scope_fifo`](#scope_fifo)
  - [`spawn`](#spawn)
  - [`spawn_fifo`](#spawn_fifo)
  - [`current_thread_has_pending_tasks`](#current_thread_has_pending_tasks)
  - [`current_thread_index`](#current_thread_index)
  - [`yield_local`](#yield_local)
  - [`yield_now`](#yield_now)
  - [`max_num_threads`](#max_num_threads)
  - [`current_num_threads`](#current_num_threads)
  - [`initialize`](#initialize)
- [Type Aliases](#type-aliases)
  - [`PanicHandler`](#panichandler)
  - [`StartHandler`](#starthandler)
  - [`ExitHandler`](#exithandler)
- [Constants](#constants)
  - [`GLOBAL_POOL_ALREADY_INITIALIZED`](#global_pool_already_initialized)
  - [`CURRENT_THREAD_ALREADY_IN_POOL`](#current_thread_already_in_pool)

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
| [`thread_pool`](#thread_pool) | mod | Contains support for user-managed thread pools, represented by the the [`ThreadPool`] type (see that struct for details). |
| [`unwind`](#unwind) | mod | Package up unwind recovery. |
| [`compile_fail`](#compile_fail) | mod |  |
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
| [`spawn_broadcast`](#spawn_broadcast) | fn |  |
| [`join`](#join) | fn |  |
| [`join_context`](#join_context) | fn |  |
| [`in_place_scope`](#in_place_scope) | fn |  |
| [`scope`](#scope) | fn |  |
| [`in_place_scope_fifo`](#in_place_scope_fifo) | fn |  |
| [`scope_fifo`](#scope_fifo) | fn |  |
| [`spawn`](#spawn) | fn |  |
| [`spawn_fifo`](#spawn_fifo) | fn |  |
| [`current_thread_has_pending_tasks`](#current_thread_has_pending_tasks) | fn |  |
| [`current_thread_index`](#current_thread_index) | fn |  |
| [`yield_local`](#yield_local) | fn |  |
| [`yield_now`](#yield_now) | fn |  |
| [`max_num_threads`](#max_num_threads) | fn | Returns the maximum number of threads that Rayon supports in a single thread pool. |
| [`current_num_threads`](#current_num_threads) | fn | Returns the number of threads in the current registry. |
| [`initialize`](#initialize) | fn | Deprecated in favor of `ThreadPoolBuilder::build_global`. |
| [`PanicHandler`](#panichandler) | type | The type for a panic-handling closure. |
| [`StartHandler`](#starthandler) | type | The type for a closure that gets invoked when a thread starts. |
| [`ExitHandler`](#exithandler) | type | The type for a closure that gets invoked when a thread exits. |
| [`GLOBAL_POOL_ALREADY_INITIALIZED`](#global_pool_already_initialized) | const |  |
| [`CURRENT_THREAD_ALREADY_IN_POOL`](#current_thread_already_in_pool) | const |  |

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

*Defined in [`rayon-core-1.13.0/src/broadcast/mod.rs:45-50`](../../.source_1765210505/rayon-core-1.13.0/src/broadcast/mod.rs#L45-L50)*

Provides context to a closure called by `broadcast`.

#### Fields

- **`_marker`**: `std::marker::PhantomData<&'a mut dyn Fn()>`

  Make sure to prevent auto-traits like `Send` and `Sync`.

#### Implementations

- <span id="broadcastcontext-with"></span>`fn with<R>(f: impl FnOnce(BroadcastContext<'_>) -> R) -> R` — [`BroadcastContext`](broadcast/index.md)

- <span id="broadcastcontext-index"></span>`fn index(&self) -> usize`

- <span id="broadcastcontext-num-threads"></span>`fn num_threads(&self) -> usize`

#### Trait Implementations

##### `impl Debug for BroadcastContext<'a>`

- <span id="broadcastcontext-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for BroadcastContext<'a>`

- <span id="broadcastcontext-const-align"></span>`const ALIGN: usize`

- <span id="broadcastcontext-type-init"></span>`type Init = T`

- <span id="broadcastcontext-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="broadcastcontext-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="broadcastcontext-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="broadcastcontext-drop"></span>`unsafe fn drop(ptr: usize)`

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

*Defined in [`rayon-core-1.13.0/src/registry.rs:22-29`](../../.source_1765210505/rayon-core-1.13.0/src/registry.rs#L22-L29)*

Thread builder used for customization via `ThreadPoolBuilder::spawn_handler()`.

#### Implementations

- <span id="threadbuilder-index"></span>`fn index(&self) -> usize`

- <span id="threadbuilder-name"></span>`fn name(&self) -> Option<&str>`

- <span id="threadbuilder-stack-size"></span>`fn stack_size(&self) -> Option<usize>`

- <span id="threadbuilder-run"></span>`fn run(self)`

#### Trait Implementations

##### `impl Debug for ThreadBuilder`

- <span id="threadbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for ThreadBuilder`

- <span id="threadbuilder-const-align"></span>`const ALIGN: usize`

- <span id="threadbuilder-type-init"></span>`type Init = T`

- <span id="threadbuilder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadbuilder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadbuilder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadbuilder-drop"></span>`unsafe fn drop(ptr: usize)`

### `Scope<'scope>`

```rust
struct Scope<'scope> {
    base: ScopeBase<'scope>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:24-26`](../../.source_1765210505/rayon-core-1.13.0/src/scope/mod.rs#L24-L26)*

Represents a fork-join scope which can be used to spawn any number of tasks.
See [`scope()`](scope/index.md) for more information.

#### Implementations

- <span id="scope-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](registry/index.md), [`Registry`](registry/index.md)

- <span id="scope-spawn"></span>`fn spawn<BODY>(&self, body: BODY)`

- <span id="scope-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

#### Trait Implementations

##### `impl Debug for Scope<'scope>`

- <span id="scope-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for Scope<'scope>`

- <span id="scope-const-align"></span>`const ALIGN: usize`

- <span id="scope-type-init"></span>`type Init = T`

- <span id="scope-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scope-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scope-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scope-drop"></span>`unsafe fn drop(ptr: usize)`

### `ScopeFifo<'scope>`

```rust
struct ScopeFifo<'scope> {
    base: ScopeBase<'scope>,
    fifos: Vec<crate::job::JobFifo>,
}
```

*Defined in [`rayon-core-1.13.0/src/scope/mod.rs:31-34`](../../.source_1765210505/rayon-core-1.13.0/src/scope/mod.rs#L31-L34)*

Represents a fork-join scope which can be used to spawn any number of tasks.
Those spawned from the same thread are prioritized in relative FIFO order.
See [`scope_fifo()`](scope/index.md) for more information.

#### Implementations

- <span id="scopefifo-new"></span>`fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](registry/index.md), [`Registry`](registry/index.md)

- <span id="scopefifo-spawn-fifo"></span>`fn spawn_fifo<BODY>(&self, body: BODY)`

- <span id="scopefifo-spawn-broadcast"></span>`fn spawn_broadcast<BODY>(&self, body: BODY)`

#### Trait Implementations

##### `impl Debug for ScopeFifo<'scope>`

- <span id="scopefifo-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for ScopeFifo<'scope>`

- <span id="scopefifo-const-align"></span>`const ALIGN: usize`

- <span id="scopefifo-type-init"></span>`type Init = T`

- <span id="scopefifo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="scopefifo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="scopefifo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="scopefifo-drop"></span>`unsafe fn drop(ptr: usize)`

### `ThreadPool`

```rust
struct ThreadPool {
    registry: std::sync::Arc<crate::registry::Registry>,
}
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:46-48`](../../.source_1765210505/rayon-core-1.13.0/src/thread_pool/mod.rs#L46-L48)*

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

- <span id="threadpool-new"></span>`fn new(configuration: crate::Configuration) -> Result<ThreadPool, Box<dyn Error>>` — [`Configuration`](#configuration), [`ThreadPool`](thread_pool/index.md)

- <span id="threadpool-build"></span>`fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](#threadpoolbuilder), [`ThreadPool`](thread_pool/index.md), [`ThreadPoolBuildError`](#threadpoolbuilderror)

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

- <span id="threadpool-yield-now"></span>`fn yield_now(&self) -> Option<Yield>` — [`Yield`](thread_pool/index.md)

- <span id="threadpool-yield-local"></span>`fn yield_local(&self) -> Option<Yield>` — [`Yield`](thread_pool/index.md)

#### Trait Implementations

##### `impl Debug for ThreadPool`

- <span id="threadpool-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ThreadPool`

- <span id="threadpool-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for ThreadPool`

- <span id="threadpool-const-align"></span>`const ALIGN: usize`

- <span id="threadpool-type-init"></span>`type Init = T`

- <span id="threadpool-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpool-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpool-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpool-drop"></span>`unsafe fn drop(ptr: usize)`

### `ThreadPoolBuildError`

```rust
struct ThreadPoolBuildError {
    kind: ErrorKind,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:142-144`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L142-L144)*

Error when initializing a thread pool.

#### Implementations

- <span id="threadpoolbuilderror-new"></span>`fn new(kind: ErrorKind) -> ThreadPoolBuildError` — [`ErrorKind`](#errorkind), [`ThreadPoolBuildError`](#threadpoolbuilderror)

- <span id="threadpoolbuilderror-is-unsupported"></span>`fn is_unsupported(&self) -> bool`

#### Trait Implementations

##### `impl Debug for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-description"></span>`fn description(&self) -> &str`

- <span id="threadpoolbuilderror-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl Pointable for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-const-align"></span>`const ALIGN: usize`

- <span id="threadpoolbuilderror-type-init"></span>`type Init = T`

- <span id="threadpoolbuilderror-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpoolbuilderror-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpoolbuilderror-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpoolbuilderror-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToString for ThreadPoolBuildError`

- <span id="threadpoolbuilderror-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`rayon-core-1.13.0/src/lib.rs:170-202`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L170-L202)*

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

#### Trait Implementations

##### `impl<S> Debug for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ThreadPoolBuilder`

- <span id="threadpoolbuilder-default"></span>`fn default() -> Self`

##### `impl<T> Pointable for ThreadPoolBuilder<S>`

- <span id="threadpoolbuilder-const-align"></span>`const ALIGN: usize`

- <span id="threadpoolbuilder-type-init"></span>`type Init = T`

- <span id="threadpoolbuilder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadpoolbuilder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadpoolbuilder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadpoolbuilder-drop"></span>`unsafe fn drop(ptr: usize)`

### `Configuration`

```rust
struct Configuration {
    builder: ThreadPoolBuilder,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:207-209`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L207-L209)*

Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`](#threadpoolbuilder) instead.

#### Implementations

- <span id="configuration-new"></span>`fn new() -> Configuration` — [`Configuration`](#configuration)

- <span id="configuration-build"></span>`fn build(self) -> Result<ThreadPool, Box<dyn Error>>` — [`ThreadPool`](thread_pool/index.md)

- <span id="configuration-thread-name"></span>`fn thread_name<F>(self, closure: F) -> Self`

- <span id="configuration-num-threads"></span>`fn num_threads(self, num_threads: usize) -> Configuration` — [`Configuration`](#configuration)

- <span id="configuration-panic-handler"></span>`fn panic_handler<H>(self, panic_handler: H) -> Configuration` — [`Configuration`](#configuration)

- <span id="configuration-stack-size"></span>`fn stack_size(self, stack_size: usize) -> Self`

- <span id="configuration-breadth-first"></span>`fn breadth_first(self) -> Self`

- <span id="configuration-start-handler"></span>`fn start_handler<H>(self, start_handler: H) -> Configuration` — [`Configuration`](#configuration)

- <span id="configuration-exit-handler"></span>`fn exit_handler<H>(self, exit_handler: H) -> Configuration` — [`Configuration`](#configuration)

- <span id="configuration-into-builder"></span>`fn into_builder(self) -> ThreadPoolBuilder` — [`ThreadPoolBuilder`](#threadpoolbuilder)

#### Trait Implementations

##### `impl Debug for Configuration`

- <span id="configuration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Configuration`

- <span id="configuration-default"></span>`fn default() -> Configuration` — [`Configuration`](#configuration)

##### `impl Pointable for Configuration`

- <span id="configuration-const-align"></span>`const ALIGN: usize`

- <span id="configuration-type-init"></span>`type Init = T`

- <span id="configuration-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="configuration-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="configuration-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="configuration-drop"></span>`unsafe fn drop(ptr: usize)`

### `FnContext`

```rust
struct FnContext {
    migrated: bool,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:840-845`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L840-L845)*

Provides the calling context to a closure called by `join_context`.

#### Fields

- **`_marker`**: `std::marker::PhantomData<*mut ()>`

  disable `Send` and `Sync`, just for a little future-proofing.

#### Implementations

- <span id="fncontext-new"></span>`fn new(migrated: bool) -> Self`

#### Trait Implementations

##### `impl Debug for FnContext`

- <span id="fncontext-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for FnContext`

- <span id="fncontext-const-align"></span>`const ALIGN: usize`

- <span id="fncontext-type-init"></span>`type Init = T`

- <span id="fncontext-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fncontext-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fncontext-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fncontext-drop"></span>`unsafe fn drop(ptr: usize)`

## Enums

### `Yield`

```rust
enum Yield {
    Executed,
    Idle,
}
```

*Defined in [`rayon-core-1.13.0/src/thread_pool/mod.rs:497-502`](../../.source_1765210505/rayon-core-1.13.0/src/thread_pool/mod.rs#L497-L502)*

Result of [`yield_now()`](thread_pool/index.md) or [`yield_local()`](thread_pool/index.md).

#### Variants

- **`Executed`**

  Work was found and executed.

- **`Idle`**

  No available work was found.

#### Trait Implementations

##### `impl Clone for Yield`

- <span id="yield-clone"></span>`fn clone(&self) -> Yield` — [`Yield`](thread_pool/index.md)

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- <span id="yield-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Yield`

##### `impl PartialEq for Yield`

- <span id="yield-eq"></span>`fn eq(&self, other: &Yield) -> bool` — [`Yield`](thread_pool/index.md)

##### `impl Pointable for Yield`

- <span id="yield-const-align"></span>`const ALIGN: usize`

- <span id="yield-type-init"></span>`type Init = T`

- <span id="yield-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="yield-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="yield-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="yield-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Yield`

### `ErrorKind`

```rust
enum ErrorKind {
    GlobalPoolAlreadyInitialized,
    CurrentThreadAlreadyInPool,
    IOError(io::Error),
}
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:147-151`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L147-L151)*

#### Trait Implementations

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Pointable for ErrorKind`

- <span id="errorkind-const-align"></span>`const ALIGN: usize`

- <span id="errorkind-type-init"></span>`type Init = T`

- <span id="errorkind-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="errorkind-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="errorkind-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="errorkind-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

*Defined in [`rayon-core-1.13.0/src/lib.rs:88`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L88)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:88`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L88)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:89`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L89)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:89`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L89)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:91`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L91)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:91`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L91)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:92`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L92)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:92`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L92)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:93`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L93)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:93`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L93)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:94`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L94)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:95`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L95)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:97`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L97)*

*Defined in [`rayon-core-1.13.0/src/lib.rs:97`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L97)*

### `max_num_threads`

```rust
fn max_num_threads() -> usize
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:113-116`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L113-L116)*

Returns the maximum number of threads that Rayon supports in a single thread pool.

If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting
the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum.

The value may vary between different targets, and is subject to change in new Rayon versions.

### `current_num_threads`

```rust
fn current_num_threads() -> usize
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:136-138`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L136-L138)*

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

*Defined in [`rayon-core-1.13.0/src/lib.rs:787-789`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L787-L789)*

Deprecated in favor of `ThreadPoolBuilder::build_global`.

## Type Aliases

### `PanicHandler`

```rust
type PanicHandler = dyn Fn(Box<dyn Any + Send>) + Send + Sync;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:213`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L213)*

The type for a panic-handling closure. Note that this same closure
may be invoked multiple times in parallel.

### `StartHandler`

```rust
type StartHandler = dyn Fn(usize) + Send + Sync;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:218`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L218)*

The type for a closure that gets invoked when a thread starts. The
closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.

### `ExitHandler`

```rust
type ExitHandler = dyn Fn(usize) + Send + Sync;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:223`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L223)*

The type for a closure that gets invoked when a thread exits. The
closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.

## Constants

### `GLOBAL_POOL_ALREADY_INITIALIZED`
```rust
const GLOBAL_POOL_ALREADY_INITIALIZED: &str;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:750-751`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L750-L751)*

### `CURRENT_THREAD_ALREADY_IN_POOL`
```rust
const CURRENT_THREAD_ALREADY_IN_POOL: &str;
```

*Defined in [`rayon-core-1.13.0/src/lib.rs:753-754`](../../.source_1765210505/rayon-core-1.13.0/src/lib.rs#L753-L754)*

