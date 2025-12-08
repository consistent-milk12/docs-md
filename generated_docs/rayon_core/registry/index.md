*[rayon_core](../index.md) / [registry](index.md)*

---

# Module `registry`

## Contents

- [Structs](#structs)
  - [`ThreadBuilder`](#threadbuilder)
  - [`DefaultSpawn`](#defaultspawn)
  - [`CustomSpawn`](#customspawn)
  - [`Registry`](#registry)
  - [`Terminator`](#terminator)
  - [`RegistryId`](#registryid)
  - [`ThreadInfo`](#threadinfo)
  - [`WorkerThread`](#workerthread)
  - [`XorShift64Star`](#xorshift64star)
- [Traits](#traits)
  - [`ThreadSpawn`](#threadspawn)
- [Functions](#functions)
  - [`global_registry`](#global_registry)
  - [`init_global_registry`](#init_global_registry)
  - [`set_global_registry`](#set_global_registry)
  - [`default_global_registry`](#default_global_registry)
  - [`main_loop`](#main_loop)
  - [`in_worker`](#in_worker)
- [Constants](#constants)
  - [`WORKER_THREAD_STATE`](#worker_thread_state)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ThreadBuilder`](#threadbuilder) | struct | Thread builder used for customization via [`ThreadPoolBuilder::spawn_handler()`]. |
| [`DefaultSpawn`](#defaultspawn) | struct | Spawns a thread in the "normal" way with `std::thread::Builder`. |
| [`CustomSpawn`](#customspawn) | struct | Spawns a thread with a user's custom callback. |
| [`Registry`](#registry) | struct |  |
| [`Terminator`](#terminator) | struct |  |
| [`RegistryId`](#registryid) | struct |  |
| [`ThreadInfo`](#threadinfo) | struct |  |
| [`WorkerThread`](#workerthread) | struct |  |
| [`XorShift64Star`](#xorshift64star) | struct | [xorshift*] is a fast pseudorandom number generator which will |
| [`ThreadSpawn`](#threadspawn) | trait | Generalized trait for spawning a thread in the `Registry`. |
| [`global_registry`](#global_registry) | fn | Starts the worker threads (if that has not already happened). |
| [`init_global_registry`](#init_global_registry) | fn | Starts the worker threads (if that has not already happened) with |
| [`set_global_registry`](#set_global_registry) | fn | Starts the worker threads (if that has not already happened) |
| [`default_global_registry`](#default_global_registry) | fn |  |
| [`main_loop`](#main_loop) | fn |  |
| [`in_worker`](#in_worker) | fn | If already in a worker-thread, just execute `op`. |
| [`WORKER_THREAD_STATE`](#worker_thread_state) | const |  |

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

- <span id="threadbuilder-index"></span>`fn index(&self) -> usize`

- <span id="threadbuilder-name"></span>`fn name(&self) -> Option<&str>`

- <span id="threadbuilder-stack-size"></span>`fn stack_size(&self) -> Option<usize>`

- <span id="threadbuilder-run"></span>`fn run(self)`

#### Trait Implementations

##### `impl Debug for ThreadBuilder`

- <span id="threadbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for ThreadBuilder`

- <span id="threadbuilder-align"></span>`const ALIGN: usize`

- <span id="threadbuilder-init"></span>`type Init = T`

- <span id="threadbuilder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadbuilder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadbuilder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadbuilder-drop"></span>`unsafe fn drop(ptr: usize)`

### `DefaultSpawn`

```rust
struct DefaultSpawn;
```

Spawns a thread in the "normal" way with `std::thread::Builder`.

This type is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Trait Implementations

##### `impl Debug for DefaultSpawn`

- <span id="defaultspawn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DefaultSpawn`

- <span id="defaultspawn-default"></span>`fn default() -> DefaultSpawn` — [`DefaultSpawn`](#defaultspawn)

##### `impl<T> Pointable for DefaultSpawn`

- <span id="defaultspawn-align"></span>`const ALIGN: usize`

- <span id="defaultspawn-init"></span>`type Init = T`

- <span id="defaultspawn-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="defaultspawn-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="defaultspawn-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="defaultspawn-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ThreadSpawn for DefaultSpawn`

- <span id="defaultspawn-spawn"></span>`fn spawn(&mut self, thread: ThreadBuilder) -> io::Result<()>` — [`ThreadBuilder`](../index.md)

### `CustomSpawn<F>`

```rust
struct CustomSpawn<F>(F);
```

Spawns a thread with a user's custom callback.

This type is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Implementations

- <span id="customspawn-new"></span>`fn new(spawn: F) -> Self`

#### Trait Implementations

##### `impl<F: fmt::Debug> Debug for CustomSpawn<F>`

- <span id="customspawn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for CustomSpawn<F>`

- <span id="customspawn-align"></span>`const ALIGN: usize`

- <span id="customspawn-init"></span>`type Init = T`

- <span id="customspawn-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="customspawn-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="customspawn-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="customspawn-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<F> ThreadSpawn for CustomSpawn<F>`

- <span id="customspawn-spawn"></span>`fn spawn(&mut self, thread: ThreadBuilder) -> io::Result<()>` — [`ThreadBuilder`](../index.md)

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

- <span id="registry-new"></span>`fn new<S>(builder: ThreadPoolBuilder<S>) -> Result<Arc<Self>, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md), [`ThreadPoolBuildError`](../index.md)

- <span id="registry-current"></span>`fn current() -> Arc<Registry>` — [`Registry`](#registry)

- <span id="registry-current-num-threads"></span>`fn current_num_threads() -> usize`

- <span id="registry-current-thread"></span>`fn current_thread(&self) -> Option<&WorkerThread>` — [`WorkerThread`](#workerthread)

- <span id="registry-id"></span>`fn id(&self) -> RegistryId` — [`RegistryId`](#registryid)

- <span id="registry-num-threads"></span>`fn num_threads(&self) -> usize`

- <span id="registry-catch-unwind"></span>`fn catch_unwind(&self, f: impl FnOnce())`

- <span id="registry-wait-until-primed"></span>`fn wait_until_primed(&self)`

- <span id="registry-inject-or-push"></span>`fn inject_or_push(&self, job_ref: JobRef)` — [`JobRef`](../job/index.md)

- <span id="registry-inject"></span>`fn inject(&self, injected_job: JobRef)` — [`JobRef`](../job/index.md)

- <span id="registry-has-injected-job"></span>`fn has_injected_job(&self) -> bool`

- <span id="registry-pop-injected-job"></span>`fn pop_injected_job(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

- <span id="registry-inject-broadcast"></span>`fn inject_broadcast(&self, injected_jobs: impl ExactSizeIterator<Item = JobRef>)` — [`JobRef`](../job/index.md)

- <span id="registry-in-worker"></span>`fn in_worker<OP, R>(&self, op: OP) -> R`

- <span id="registry-in-worker-cold"></span>`unsafe fn in_worker_cold<OP, R>(&self, op: OP) -> R`

- <span id="registry-in-worker-cross"></span>`unsafe fn in_worker_cross<OP, R>(&self, current_thread: &WorkerThread, op: OP) -> R` — [`WorkerThread`](#workerthread)

- <span id="registry-increment-terminate-count"></span>`fn increment_terminate_count(&self)`

- <span id="registry-terminate"></span>`fn terminate(&self)`

- <span id="registry-notify-worker-latch-is-set"></span>`fn notify_worker_latch_is_set(&self, target_worker_index: usize)`

#### Trait Implementations

##### `impl<T> Pointable for Registry`

- <span id="registry-align"></span>`const ALIGN: usize`

- <span id="registry-init"></span>`type Init = T`

- <span id="registry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="registry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="registry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="registry-drop"></span>`unsafe fn drop(ptr: usize)`

### `Terminator<'a>`

```rust
struct Terminator<'a>(&'a std::sync::Arc<Registry>);
```

#### Trait Implementations

##### `impl<'a> Drop for Terminator<'a>`

- <span id="terminator-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Terminator<'a>`

- <span id="terminator-align"></span>`const ALIGN: usize`

- <span id="terminator-init"></span>`type Init = T`

- <span id="terminator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="terminator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="terminator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="terminator-drop"></span>`unsafe fn drop(ptr: usize)`

### `RegistryId`

```rust
struct RegistryId {
    addr: usize,
}
```

#### Trait Implementations

##### `impl Clone for RegistryId`

- <span id="registryid-clone"></span>`fn clone(&self) -> RegistryId` — [`RegistryId`](#registryid)

##### `impl Copy for RegistryId`

##### `impl Debug for RegistryId`

- <span id="registryid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RegistryId`

##### `impl Ord for RegistryId`

- <span id="registryid-cmp"></span>`fn cmp(&self, other: &RegistryId) -> cmp::Ordering` — [`RegistryId`](#registryid)

##### `impl PartialEq for RegistryId`

- <span id="registryid-eq"></span>`fn eq(&self, other: &RegistryId) -> bool` — [`RegistryId`](#registryid)

##### `impl PartialOrd for RegistryId`

- <span id="registryid-partial-cmp"></span>`fn partial_cmp(&self, other: &RegistryId) -> option::Option<cmp::Ordering>` — [`RegistryId`](#registryid)

##### `impl<T> Pointable for RegistryId`

- <span id="registryid-align"></span>`const ALIGN: usize`

- <span id="registryid-init"></span>`type Init = T`

- <span id="registryid-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="registryid-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="registryid-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="registryid-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="threadinfo-new"></span>`fn new(stealer: Stealer<JobRef>) -> ThreadInfo` — [`JobRef`](../job/index.md), [`ThreadInfo`](#threadinfo)

#### Trait Implementations

##### `impl<T> Pointable for ThreadInfo`

- <span id="threadinfo-align"></span>`const ALIGN: usize`

- <span id="threadinfo-init"></span>`type Init = T`

- <span id="threadinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadinfo-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="workerthread-current"></span>`fn current() -> *const WorkerThread` — [`WorkerThread`](#workerthread)

- <span id="workerthread-set-current"></span>`unsafe fn set_current(thread: *const WorkerThread)` — [`WorkerThread`](#workerthread)

- <span id="workerthread-registry"></span>`fn registry(&self) -> &Arc<Registry>` — [`Registry`](#registry)

- <span id="workerthread-index"></span>`fn index(&self) -> usize`

- <span id="workerthread-push"></span>`unsafe fn push(&self, job: JobRef)` — [`JobRef`](../job/index.md)

- <span id="workerthread-push-fifo"></span>`unsafe fn push_fifo(&self, job: JobRef)` — [`JobRef`](../job/index.md)

- <span id="workerthread-local-deque-is-empty"></span>`fn local_deque_is_empty(&self) -> bool`

- <span id="workerthread-take-local-job"></span>`fn take_local_job(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

- <span id="workerthread-has-injected-job"></span>`fn has_injected_job(&self) -> bool`

- <span id="workerthread-wait-until"></span>`unsafe fn wait_until<L: AsCoreLatch + ?Sized>(&self, latch: &L)`

- <span id="workerthread-wait-until-cold"></span>`unsafe fn wait_until_cold(&self, latch: &CoreLatch)` — [`CoreLatch`](../latch/index.md)

- <span id="workerthread-wait-until-out-of-work"></span>`unsafe fn wait_until_out_of_work(&self)`

- <span id="workerthread-find-work"></span>`fn find_work(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

- <span id="workerthread-yield-now"></span>`fn yield_now(&self) -> Yield` — [`Yield`](../index.md)

- <span id="workerthread-yield-local"></span>`fn yield_local(&self) -> Yield` — [`Yield`](../index.md)

- <span id="workerthread-execute"></span>`unsafe fn execute(&self, job: JobRef)` — [`JobRef`](../job/index.md)

- <span id="workerthread-steal"></span>`fn steal(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md)

#### Trait Implementations

##### `impl Drop for WorkerThread`

- <span id="workerthread-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for WorkerThread`

- <span id="workerthread-align"></span>`const ALIGN: usize`

- <span id="workerthread-init"></span>`type Init = T`

- <span id="workerthread-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="workerthread-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="workerthread-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="workerthread-drop"></span>`unsafe fn drop(ptr: usize)`

### `XorShift64Star`

```rust
struct XorShift64Star {
    state: std::cell::Cell<u64>,
}
```

[xorshift*] is a fast pseudorandom number generator which will
even tolerate weak seeding, as long as it's not zero.


#### Implementations

- <span id="xorshift64star-new"></span>`fn new() -> Self`

- <span id="xorshift64star-next"></span>`fn next(&self) -> u64`

- <span id="xorshift64star-next-usize"></span>`fn next_usize(&self, n: usize) -> usize`

#### Trait Implementations

##### `impl<T> Pointable for XorShift64Star`

- <span id="xorshift64star-align"></span>`const ALIGN: usize`

- <span id="xorshift64star-init"></span>`type Init = T`

- <span id="xorshift64star-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="xorshift64star-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="xorshift64star-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="xorshift64star-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `ThreadSpawn`

```rust
trait ThreadSpawn { ... }
```

Generalized trait for spawning a thread in the `Registry`.

This trait is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Required Methods

- `fn spawn(&mut self, thread: ThreadBuilder) -> io::Result<()>`

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
const WORKER_THREAD_STATE: thread::LocalKey<std::cell::Cell<*const WorkerThread>>;
```

