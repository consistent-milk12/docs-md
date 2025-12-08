# Crate `rayon_core`

Rayon-core houses the core stable APIs of Rayon.

These APIs have been mirrored in the Rayon crate and it is recommended to use these from there.

[`join()`](#join) is used to take two closures and potentially run them in parallel.
  - It will run in parallel if task B gets stolen before task A can finish.
  - It will run sequentially if task A finishes before task B is stolen and can continue on task B.

[`scope()`](#scope) creates a scope in which you can run any number of parallel tasks.
These tasks can spawn nested tasks and scopes, but given the nature of work stealing, the order of execution can not be guaranteed.
The scope will exist until all tasks spawned within the scope have been completed.

[`spawn()`](#spawn) add a task into the 'static' or 'global' scope, or a local scope created by the [`scope()`](#scope) function.

[`ThreadPool`](#threadpool) can be used to create your own thread pools (using [`ThreadPoolBuilder`](#threadpoolbuilder)) or to customize the global one.
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

## Modules

- [`private`](private/index.md) - The public parts of this private module are used to create traits
- [`broadcast`](broadcast/index.md) - 
- [`job`](job/index.md) - 
- [`join`](join/index.md) - 
- [`latch`](latch/index.md) - 
- [`registry`](registry/index.md) - 
- [`scope`](scope/index.md) - Methods for custom fork-join scopes, created by the [`scope()`]
- [`sleep`](sleep/index.md) - Code that decides when workers should go to sleep. See README.md
- [`spawn`](spawn/index.md) - 
- [`thread_pool`](thread_pool/index.md) - Contains support for user-managed thread pools, represented by the
- [`unwind`](unwind/index.md) - Package up unwind recovery. Note that if you are in some sensitive
- [`compile_fail`](compile_fail/index.md) - 

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

Thread builder used for customization via `ThreadPoolBuilder::spawn_handler()`.

#### Implementations

- `fn index(self: &Self) -> usize`

- `fn name(self: &Self) -> Option<&str>`

- `fn stack_size(self: &Self) -> Option<usize>`

- `fn run(self: Self)`

#### Trait Implementations

##### `impl Debug for ThreadBuilder`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for ThreadBuilder`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Scope<'scope>`

```rust
struct Scope<'scope> {
    base: ScopeBase<'scope>,
}
```

Represents a fork-join scope which can be used to spawn any number of tasks.
See [`scope()`](#scope) for more information.

#### Implementations

- `fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](registry/index.md), [`Registry`](registry/index.md)

- `fn spawn<BODY>(self: &Self, body: BODY)`

- `fn spawn_broadcast<BODY>(self: &Self, body: BODY)`

#### Trait Implementations

##### `impl<'scope> Debug for Scope<'scope>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for Scope<'scope>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ScopeFifo<'scope>`

```rust
struct ScopeFifo<'scope> {
    base: ScopeBase<'scope>,
    fifos: Vec<crate::job::JobFifo>,
}
```

Represents a fork-join scope which can be used to spawn any number of tasks.
Those spawned from the same thread are prioritized in relative FIFO order.
See [`scope_fifo()`](#scope-fifo) for more information.

#### Implementations

- `fn new(owner: Option<&WorkerThread>, registry: Option<&Arc<Registry>>) -> Self` — [`WorkerThread`](registry/index.md), [`Registry`](registry/index.md)

- `fn spawn_fifo<BODY>(self: &Self, body: BODY)`

- `fn spawn_broadcast<BODY>(self: &Self, body: BODY)`

#### Trait Implementations

##### `impl<'scope> Debug for ScopeFifo<'scope>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for ScopeFifo<'scope>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ThreadPool`

```rust
struct ThreadPool {
    registry: std::sync::Arc<crate::registry::Registry>,
}
```

Represents a user-created [thread pool].

Use a [`ThreadPoolBuilder`](#threadpoolbuilder) to specify the number and/or names of threads
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

- `fn new(configuration: crate::Configuration) -> Result<ThreadPool, Box<dyn Error>>` — [`Configuration`](#configuration), [`ThreadPool`](#threadpool)

- `fn build<S>(builder: ThreadPoolBuilder<S>) -> Result<ThreadPool, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](#threadpoolbuilder), [`ThreadPool`](#threadpool), [`ThreadPoolBuildError`](#threadpoolbuilderror)

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

- `fn yield_now(self: &Self) -> Option<Yield>` — [`Yield`](#yield)

- `fn yield_local(self: &Self) -> Option<Yield>` — [`Yield`](#yield)

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

### `ThreadPoolBuildError`

```rust
struct ThreadPoolBuildError {
    kind: ErrorKind,
}
```

Error when initializing a thread pool.

#### Implementations

- `fn new(kind: ErrorKind) -> ThreadPoolBuildError` — [`ErrorKind`](#errorkind), [`ThreadPoolBuildError`](#threadpoolbuilderror)

- `fn is_unsupported(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for ThreadPoolBuildError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ThreadPoolBuildError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ThreadPoolBuildError`

- `fn description(self: &Self) -> &str`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl<T> Pointable for ThreadPoolBuildError`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> ToString for ThreadPoolBuildError`

- `fn to_string(self: &Self) -> String`

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

Used to create a new [`ThreadPool`](#threadpool) or to configure the global rayon thread pool.
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

- `fn build_scoped<W, F, R>(self: Self, wrapper: W, with_pool: F) -> Result<R, ThreadPoolBuildError>` — [`ThreadPoolBuildError`](#threadpoolbuilderror)

#### Trait Implementations

##### `impl<S> Debug for ThreadPoolBuilder<S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ThreadPoolBuilder`

- `fn default() -> Self`

##### `impl<T> Pointable for ThreadPoolBuilder<S>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Configuration`

```rust
struct Configuration {
    builder: ThreadPoolBuilder,
}
```

Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`](#threadpoolbuilder) instead.

#### Implementations

- `fn new() -> Configuration` — [`Configuration`](#configuration)

- `fn build(self: Self) -> Result<ThreadPool, Box<dyn Error>>` — [`ThreadPool`](#threadpool)

- `fn thread_name<F>(self: Self, closure: F) -> Self`

- `fn num_threads(self: Self, num_threads: usize) -> Configuration` — [`Configuration`](#configuration)

- `fn panic_handler<H>(self: Self, panic_handler: H) -> Configuration` — [`Configuration`](#configuration)

- `fn stack_size(self: Self, stack_size: usize) -> Self`

- `fn breadth_first(self: Self) -> Self`

- `fn start_handler<H>(self: Self, start_handler: H) -> Configuration` — [`Configuration`](#configuration)

- `fn exit_handler<H>(self: Self, exit_handler: H) -> Configuration` — [`Configuration`](#configuration)

- `fn into_builder(self: Self) -> ThreadPoolBuilder` — [`ThreadPoolBuilder`](#threadpoolbuilder)

#### Trait Implementations

##### `impl Debug for Configuration`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Configuration`

- `fn default() -> Configuration` — [`Configuration`](#configuration)

##### `impl<T> Pointable for Configuration`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FnContext`

```rust
struct FnContext {
    migrated: bool,
    _marker: std::marker::PhantomData<*mut ()>,
}
```

Provides the calling context to a closure called by `join_context`.

#### Fields

- **`_marker`**: `std::marker::PhantomData<*mut ()>`

  disable `Send` and `Sync`, just for a little future-proofing.

#### Implementations

- `fn migrated(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for FnContext`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Pointable for FnContext`

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

Result of [`yield_now()`](#yield-now) or [`yield_local()`](#yield-local).

#### Variants

- **`Executed`**

  Work was found and executed.

- **`Idle`**

  No available work was found.

#### Trait Implementations

##### `impl Clone for Yield`

- `fn clone(self: &Self) -> Yield` — [`Yield`](#yield)

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Yield`

##### `impl PartialEq for Yield`

- `fn eq(self: &Self, other: &Yield) -> bool` — [`Yield`](#yield)

##### `impl<T> Pointable for Yield`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Yield`

### `ErrorKind`

```rust
enum ErrorKind {
    GlobalPoolAlreadyInitialized,
    CurrentThreadAlreadyInPool,
    IOError(io::Error),
}
```

#### Trait Implementations

##### `impl Debug for ErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Pointable for ErrorKind`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `max_num_threads`

```rust
fn max_num_threads() -> usize
```

Returns the maximum number of threads that Rayon supports in a single thread pool.

If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting
the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum.

The value may vary between different targets, and is subject to change in new Rayon versions.

### `current_num_threads`

```rust
fn current_num_threads() -> usize
```

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

Deprecated in favor of `ThreadPoolBuilder::build_global`.

## Type Aliases

### `PanicHandler`

```rust
type PanicHandler = dyn Fn(Box<dyn Any + Send>) + Send + Sync;
```

The type for a panic-handling closure. Note that this same closure
may be invoked multiple times in parallel.

### `StartHandler`

```rust
type StartHandler = dyn Fn(usize) + Send + Sync;
```

The type for a closure that gets invoked when a thread starts. The
closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.

### `ExitHandler`

```rust
type ExitHandler = dyn Fn(usize) + Send + Sync;
```

The type for a closure that gets invoked when a thread exits. The
closure is passed the index of the thread on which it is invoked.
Note that this same closure may be invoked multiple times in parallel.

## Constants

### `GLOBAL_POOL_ALREADY_INITIALIZED`

```rust
const GLOBAL_POOL_ALREADY_INITIALIZED: &str;
```

### `CURRENT_THREAD_ALREADY_IN_POOL`

```rust
const CURRENT_THREAD_ALREADY_IN_POOL: &str;
```

