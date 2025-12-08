*[rayon_core](../index.md) / [registry](index.md)*

---

# Module `registry`

## Structs

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

### `DefaultSpawn`

```rust
struct DefaultSpawn;
```

Spawns a thread in the "normal" way with `std::thread::Builder`.

This type is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Trait Implementations

##### `impl Debug for DefaultSpawn`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DefaultSpawn`

- `fn default() -> DefaultSpawn` — [`DefaultSpawn`](#defaultspawn)

##### `impl<T> Pointable for DefaultSpawn`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl ThreadSpawn for DefaultSpawn`

- `fn spawn(self: &mut Self, thread: ThreadBuilder) -> io::Result<()>` — [`ThreadBuilder`](../index.md)

### `CustomSpawn<F>`

```rust
struct CustomSpawn<F>(F);
```

Spawns a thread with a user's custom callback.

This type is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Implementations

- `fn new(spawn: F) -> Self`

#### Trait Implementations

##### `impl<F: $crate::fmt::Debug> Debug for CustomSpawn<F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Pointable for CustomSpawn<F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<F> ThreadSpawn for CustomSpawn<F>`

- `fn spawn(self: &mut Self, thread: ThreadBuilder) -> io::Result<()>` — [`ThreadBuilder`](../index.md)

### `Registry`

```rust
struct Registry {
    thread_infos: Vec<ThreadInfo>,
    sleep: crate::sleep::Sleep,
    injected_jobs: crossbeam_deque::Injector<crate::job::JobRef>,
    broadcasts: crate::sync::Mutex<Vec<crossbeam_deque::Worker<crate::job::JobRef>>>,
    panic_handler: Option<Box<dyn Fn(Box<dyn Any + Send>) + Send + Sync>>,
    start_handler: Option<Box<dyn Fn(usize) + Send + Sync>>,
    exit_handler: Option<Box<dyn Fn(usize) + Send + Sync>>,
    terminate_count: std::sync::atomic::AtomicUsize,
}
```

#### Implementations

- `fn new<S>(builder: ThreadPoolBuilder<S>) -> Result<Arc<Self>, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md), [`ThreadPoolBuildError`](../index.md)

- `fn current() -> Arc<Registry>` — [`Registry`](#registry)

- `fn current_num_threads() -> usize`

- `fn current_thread(self: &Self) -> Option<&WorkerThread>` — [`WorkerThread`](#workerthread)

- `fn id(self: &Self) -> RegistryId` — [`RegistryId`](#registryid)

- `fn num_threads(self: &Self) -> usize`

- `fn catch_unwind(self: &Self, f: impl FnOnce())`

- `fn wait_until_primed(self: &Self)`

- `fn inject_or_push(self: &Self, job_ref: JobRef)` — [`JobRef`](../job/index.md)

- `fn inject(self: &Self, injected_job: JobRef)` — [`JobRef`](../job/index.md)

- `fn has_injected_job(self: &Self) -> bool`

- `fn pop_injected_job(self: &Self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

- `fn inject_broadcast(self: &Self, injected_jobs: impl ExactSizeIterator<Item = JobRef>)` — [`JobRef`](../job/index.md)

- `fn in_worker<OP, R>(self: &Self, op: OP) -> R`

- `unsafe fn in_worker_cold<OP, R>(self: &Self, op: OP) -> R`

- `unsafe fn in_worker_cross<OP, R>(self: &Self, current_thread: &WorkerThread, op: OP) -> R` — [`WorkerThread`](#workerthread)

- `fn increment_terminate_count(self: &Self)`

- `fn terminate(self: &Self)`

- `fn notify_worker_latch_is_set(self: &Self, target_worker_index: usize)`

#### Trait Implementations

##### `impl<T> Pointable for Registry`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Terminator<'a>`

```rust
struct Terminator<'a>(&'a std::sync::Arc<Registry>);
```

#### Trait Implementations

##### `impl<'a> Drop for Terminator<'a>`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Terminator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RegistryId`

```rust
struct RegistryId {
    addr: usize,
}
```

#### Trait Implementations

##### `impl Clone for RegistryId`

- `fn clone(self: &Self) -> RegistryId` — [`RegistryId`](#registryid)

##### `impl Copy for RegistryId`

##### `impl Debug for RegistryId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RegistryId`

##### `impl Ord for RegistryId`

- `fn cmp(self: &Self, other: &RegistryId) -> $crate::cmp::Ordering` — [`RegistryId`](#registryid)

##### `impl PartialEq for RegistryId`

- `fn eq(self: &Self, other: &RegistryId) -> bool` — [`RegistryId`](#registryid)

##### `impl PartialOrd for RegistryId`

- `fn partial_cmp(self: &Self, other: &RegistryId) -> $crate::option::Option<$crate::cmp::Ordering>` — [`RegistryId`](#registryid)

##### `impl<T> Pointable for RegistryId`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for RegistryId`

### `ThreadInfo`

```rust
struct ThreadInfo {
    primed: crate::latch::LockLatch,
    stopped: crate::latch::LockLatch,
    terminate: crate::latch::OnceLatch,
    stealer: crossbeam_deque::Stealer<crate::job::JobRef>,
}
```

#### Fields

- **`primed`**: `crate::latch::LockLatch`

  Latch set once thread has started and we are entering into the
  main loop. Used to wait for worker threads to become primed,
  primarily of interest for benchmarking.

- **`stopped`**: `crate::latch::LockLatch`

  Latch is set once worker thread has completed. Used to wait
  until workers have stopped; only used for tests.

- **`terminate`**: `crate::latch::OnceLatch`

  The latch used to signal that terminated has been requested.
  This latch is *set* by the `terminate` method on the
  `Registry`, once the registry's main "terminate" counter
  reaches zero.

- **`stealer`**: `crossbeam_deque::Stealer<crate::job::JobRef>`

  the "stealer" half of the worker's deque

#### Implementations

- `fn new(stealer: Stealer<JobRef>) -> ThreadInfo` — [`JobRef`](../job/index.md), [`ThreadInfo`](#threadinfo)

#### Trait Implementations

##### `impl<T> Pointable for ThreadInfo`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WorkerThread`

```rust
struct WorkerThread {
    worker: crossbeam_deque::Worker<crate::job::JobRef>,
    stealer: crossbeam_deque::Stealer<crate::job::JobRef>,
    fifo: crate::job::JobFifo,
    index: usize,
    rng: XorShift64Star,
    registry: std::sync::Arc<Registry>,
}
```

#### Fields

- **`worker`**: `crossbeam_deque::Worker<crate::job::JobRef>`

  the "worker" half of our local deque

- **`stealer`**: `crossbeam_deque::Stealer<crate::job::JobRef>`

  the "stealer" half of the worker's broadcast deque

- **`fifo`**: `crate::job::JobFifo`

  local queue used for `spawn_fifo` indirection

- **`rng`**: `XorShift64Star`

  A weak random number generator.

#### Implementations

- `fn current() -> *const WorkerThread` — [`WorkerThread`](#workerthread)

- `unsafe fn set_current(thread: *const WorkerThread)` — [`WorkerThread`](#workerthread)

- `fn registry(self: &Self) -> &Arc<Registry>` — [`Registry`](#registry)

- `fn index(self: &Self) -> usize`

- `unsafe fn push(self: &Self, job: JobRef)` — [`JobRef`](../job/index.md)

- `unsafe fn push_fifo(self: &Self, job: JobRef)` — [`JobRef`](../job/index.md)

- `fn local_deque_is_empty(self: &Self) -> bool`

- `fn take_local_job(self: &Self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

- `fn has_injected_job(self: &Self) -> bool`

- `unsafe fn wait_until<L: AsCoreLatch + ?Sized>(self: &Self, latch: &L)`

- `unsafe fn wait_until_cold(self: &Self, latch: &CoreLatch)` — [`CoreLatch`](../latch/index.md)

- `unsafe fn wait_until_out_of_work(self: &Self)`

- `fn find_work(self: &Self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

- `fn yield_now(self: &Self) -> Yield` — [`Yield`](../index.md)

- `fn yield_local(self: &Self) -> Yield` — [`Yield`](../index.md)

- `unsafe fn execute(self: &Self, job: JobRef)` — [`JobRef`](../job/index.md)

- `fn steal(self: &Self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

#### Trait Implementations

##### `impl Drop for WorkerThread`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for WorkerThread`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `XorShift64Star`

```rust
struct XorShift64Star {
    state: std::cell::Cell<u64>,
}
```

[xorshift*] is a fast pseudorandom number generator which will
even tolerate weak seeding, as long as it's not zero.


#### Implementations

- `fn new() -> Self`

- `fn next(self: &Self) -> u64`

- `fn next_usize(self: &Self, n: usize) -> usize`

#### Trait Implementations

##### `impl<T> Pointable for XorShift64Star`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `ThreadSpawn`

```rust
trait ThreadSpawn { ... }
```

Generalized trait for spawning a thread in the `Registry`.

This trait is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Required Methods

- `fn spawn(self: &mut Self, thread: ThreadBuilder) -> io::Result<()>`

  Spawn a thread with the `ThreadBuilder` parameters, and then

## Functions

### `global_registry`

```rust
fn global_registry() -> &'static std::sync::Arc<Registry>
```

Starts the worker threads (if that has not already happened). If
initialization has not already occurred, use the default
configuration.

### `init_global_registry`

```rust
fn init_global_registry<S>(builder: crate::ThreadPoolBuilder<S>) -> Result<&'static std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
where
    S: ThreadSpawn
```

Starts the worker threads (if that has not already happened) with
the given builder.

### `set_global_registry`

```rust
fn set_global_registry<F>(registry: F) -> Result<&'static std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
where
    F: FnOnce() -> Result<std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
```

Starts the worker threads (if that has not already happened)
by creating a registry with the given callback.

### `default_global_registry`

```rust
fn default_global_registry() -> Result<std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
```

### `main_loop`

```rust
unsafe fn main_loop(thread: ThreadBuilder)
```

### `in_worker`

```rust
fn in_worker<OP, R>(op: OP) -> R
where
    OP: FnOnce(&WorkerThread, bool) -> R + Send,
    R: Send
```

If already in a worker-thread, just execute `op`.  Otherwise,
execute `op` in the default thread pool. Either way, block until
`op` completes and return its return value. If `op` panics, that
panic will be propagated as well.  The second argument indicates
`true` if injection was performed, `false` if executed directly.

## Constants

### `WORKER_THREAD_STATE`

```rust
const WORKER_THREAD_STATE: $crate::thread::LocalKey<std::cell::Cell<*const WorkerThread>>;
```

