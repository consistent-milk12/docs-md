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
  - [`global_registry`](#global-registry)
  - [`init_global_registry`](#init-global-registry)
  - [`set_global_registry`](#set-global-registry)
  - [`default_global_registry`](#default-global-registry)
  - [`main_loop`](#main-loop)
  - [`in_worker`](#in-worker)
- [Constants](#constants)
  - [`WORKER_THREAD_STATE`](#worker-thread-state)

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
| [`XorShift64Star`](#xorshift64star) | struct | [xorshift*] is a fast pseudorandom number generator which will even tolerate weak seeding, as long as it's not zero. |
| [`ThreadSpawn`](#threadspawn) | trait | Generalized trait for spawning a thread in the `Registry`. |
| [`global_registry`](#global-registry) | fn | Starts the worker threads (if that has not already happened). |
| [`init_global_registry`](#init-global-registry) | fn | Starts the worker threads (if that has not already happened) with the given builder. |
| [`set_global_registry`](#set-global-registry) | fn | Starts the worker threads (if that has not already happened) by creating a registry with the given callback. |
| [`default_global_registry`](#default-global-registry) | fn |  |
| [`main_loop`](#main-loop) | fn |  |
| [`in_worker`](#in-worker) | fn | If already in a worker-thread, just execute `op`. |
| [`WORKER_THREAD_STATE`](#worker-thread-state) | const |  |

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

*Defined in [`rayon-core-1.13.0/src/registry.rs:22-29`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L22-L29)*

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

### `DefaultSpawn`

```rust
struct DefaultSpawn;
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:82`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L82)*

Spawns a thread in the "normal" way with `std::thread::Builder`.

This type is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Trait Implementations

##### `impl Any for DefaultSpawn`

- <span id="defaultspawn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DefaultSpawn`

- <span id="defaultspawn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DefaultSpawn`

- <span id="defaultspawn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DefaultSpawn`

- <span id="defaultspawn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DefaultSpawn`

- <span id="defaultspawn-default"></span>`fn default() -> DefaultSpawn` — [`DefaultSpawn`](#defaultspawn)

##### `impl<T> From for DefaultSpawn`

- <span id="defaultspawn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DefaultSpawn`

- <span id="defaultspawn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for DefaultSpawn`

- <span id="defaultspawn-pointable-const-align"></span>`const ALIGN: usize`

- <span id="defaultspawn-pointable-type-init"></span>`type Init = T`

- <span id="defaultspawn-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="defaultspawn-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="defaultspawn-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="defaultspawn-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ThreadSpawn for DefaultSpawn`

- <span id="defaultspawn-threadspawn-spawn"></span>`fn spawn(&mut self, thread: ThreadBuilder) -> io::Result<()>` — [`ThreadBuilder`](#threadbuilder)

##### `impl<U> TryFrom for DefaultSpawn`

- <span id="defaultspawn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="defaultspawn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DefaultSpawn`

- <span id="defaultspawn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="defaultspawn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CustomSpawn<F>`

```rust
struct CustomSpawn<F>(F);
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:105`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L105)*

Spawns a thread with a user's custom callback.

This type is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Implementations

- <span id="customspawn-new"></span>`fn new(spawn: F) -> Self`

#### Trait Implementations

##### `impl Any for CustomSpawn<F>`

- <span id="customspawn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CustomSpawn<F>`

- <span id="customspawn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CustomSpawn<F>`

- <span id="customspawn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: fmt::Debug> Debug for CustomSpawn<F>`

- <span id="customspawn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CustomSpawn<F>`

- <span id="customspawn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CustomSpawn<F>`

- <span id="customspawn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for CustomSpawn<F>`

- <span id="customspawn-pointable-const-align"></span>`const ALIGN: usize`

- <span id="customspawn-pointable-type-init"></span>`type Init = T`

- <span id="customspawn-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="customspawn-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="customspawn-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="customspawn-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<F> ThreadSpawn for CustomSpawn<F>`

- <span id="customspawn-threadspawn-spawn"></span>`fn spawn(&mut self, thread: ThreadBuilder) -> io::Result<()>` — [`ThreadBuilder`](#threadbuilder)

##### `impl<U> TryFrom for CustomSpawn<F>`

- <span id="customspawn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="customspawn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CustomSpawn<F>`

- <span id="customspawn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="customspawn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-core-1.13.0/src/registry.rs:128-151`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L128-L151)*

#### Implementations

- <span id="registry-new"></span>`fn new<S>(builder: ThreadPoolBuilder<S>) -> Result<Arc<Self>, ThreadPoolBuildError>` — [`ThreadPoolBuilder`](../index.md#threadpoolbuilder), [`ThreadPoolBuildError`](../index.md#threadpoolbuilderror)

- <span id="registry-current"></span>`fn current() -> Arc<Registry>` — [`Registry`](#registry)

- <span id="registry-current-num-threads"></span>`fn current_num_threads() -> usize`

  Returns the number of threads in the current registry.  This

  is better than `Registry::current().num_threads()` because it

  avoids incrementing the `Arc`.

- <span id="registry-current-thread"></span>`fn current_thread(&self) -> Option<&WorkerThread>` — [`WorkerThread`](#workerthread)

  Returns the current `WorkerThread` if it's part of this `Registry`.

- <span id="registry-id"></span>`fn id(&self) -> RegistryId` — [`RegistryId`](#registryid)

  Returns an opaque identifier for this registry.

- <span id="registry-num-threads"></span>`fn num_threads(&self) -> usize`

- <span id="registry-catch-unwind"></span>`fn catch_unwind(&self, f: impl FnOnce())`

- <span id="registry-wait-until-primed"></span>`fn wait_until_primed(&self)`

  Waits for the worker threads to get up and running.  This is

  meant to be used for benchmarking purposes, primarily, so that

  you can get more consistent numbers by having everything

  "ready to go".

- <span id="registry-inject-or-push"></span>`fn inject_or_push(&self, job_ref: JobRef)` — [`JobRef`](../job/index.md#jobref)

  Push a job into the given `registry`. If we are running on a

  worker thread for the registry, this will push onto the

  deque. Else, it will inject from the outside (which is slower).

- <span id="registry-inject"></span>`fn inject(&self, injected_job: JobRef)` — [`JobRef`](../job/index.md#jobref)

  Push a job into the "external jobs" queue; it will be taken by

  whatever worker has nothing to do. Use this if you know that

  you are not on a worker of this registry.

- <span id="registry-has-injected-job"></span>`fn has_injected_job(&self) -> bool`

- <span id="registry-pop-injected-job"></span>`fn pop_injected_job(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md#jobref)

- <span id="registry-inject-broadcast"></span>`fn inject_broadcast(&self, injected_jobs: impl ExactSizeIterator<Item = JobRef>)` — [`JobRef`](../job/index.md#jobref)

  Push a job into each thread's own "external jobs" queue; it will be

  executed only on that thread, when it has nothing else to do locally,

  before it tries to steal other work.

  

  **Panics** if not given exactly as many jobs as there are threads.

- <span id="registry-in-worker"></span>`fn in_worker<OP, R>(&self, op: OP) -> R`

  If already in a worker-thread of this registry, just execute `op`.

  Otherwise, inject `op` in this thread pool. Either way, block until `op`

  completes and return its return value. If `op` panics, that panic will

  be propagated as well.  The second argument indicates `true` if injection

  was performed, `false` if executed directly.

- <span id="registry-in-worker-cold"></span>`unsafe fn in_worker_cold<OP, R>(&self, op: OP) -> R`

- <span id="registry-in-worker-cross"></span>`unsafe fn in_worker_cross<OP, R>(&self, current_thread: &WorkerThread, op: OP) -> R` — [`WorkerThread`](#workerthread)

- <span id="registry-increment-terminate-count"></span>`fn increment_terminate_count(&self)`

  Increments the terminate counter. This increment should be

  balanced by a call to `terminate`, which will decrement. This

  is used when spawning asynchronous work, which needs to

  prevent the registry from terminating so long as it is active.

  

  Note that blocking functions such as `join` and `scope` do not

  need to concern themselves with this fn; their context is

  responsible for ensuring the current thread pool will not

  terminate until they return.

  

  The global thread pool always has an outstanding reference

  (the initial one). Custom thread pools have one outstanding

  reference that is dropped when the `ThreadPool` is dropped:

  since installing the thread pool blocks until any joins/scopes

  complete, this ensures that joins/scopes are covered.

  

  The exception is `::spawn()`, which can create a job outside

  of any blocking scope. In that case, the job itself holds a

  terminate count and is responsible for invoking `terminate()`

  when finished.

- <span id="registry-terminate"></span>`fn terminate(&self)`

  Signals that the thread pool which owns this registry has been

  dropped. The worker threads will gradually terminate, once any

  extant work is completed.

- <span id="registry-notify-worker-latch-is-set"></span>`fn notify_worker_latch_is_set(&self, target_worker_index: usize)`

  Notify the worker that the latch they are sleeping on has been "set".

#### Trait Implementations

##### `impl Any for Registry`

- <span id="registry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Registry`

- <span id="registry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Registry`

- <span id="registry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Registry`

- <span id="registry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Registry`

- <span id="registry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Registry`

- <span id="registry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="registry-pointable-type-init"></span>`type Init = T`

- <span id="registry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="registry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="registry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="registry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Registry`

- <span id="registry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="registry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Registry`

- <span id="registry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="registry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Terminator<'a>`

```rust
struct Terminator<'a>(&'a std::sync::Arc<Registry>);
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:230`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L230)*

#### Trait Implementations

##### `impl Any for Terminator<'a>`

- <span id="terminator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Terminator<'a>`

- <span id="terminator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Terminator<'a>`

- <span id="terminator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for Terminator<'a>`

- <span id="terminator-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Terminator<'a>`

- <span id="terminator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Terminator<'a>`

- <span id="terminator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Terminator<'a>`

- <span id="terminator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="terminator-pointable-type-init"></span>`type Init = T`

- <span id="terminator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="terminator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="terminator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="terminator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Terminator<'a>`

- <span id="terminator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="terminator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Terminator<'a>`

- <span id="terminator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="terminator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegistryId`

```rust
struct RegistryId {
    addr: usize,
}
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:609-611`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L609-L611)*

#### Trait Implementations

##### `impl Any for RegistryId`

- <span id="registryid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegistryId`

- <span id="registryid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegistryId`

- <span id="registryid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RegistryId`

- <span id="registryid-clone"></span>`fn clone(&self) -> RegistryId` — [`RegistryId`](#registryid)

##### `impl CloneToUninit for RegistryId`

- <span id="registryid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RegistryId`

##### `impl Debug for RegistryId`

- <span id="registryid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RegistryId`

##### `impl<T> From for RegistryId`

- <span id="registryid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegistryId`

- <span id="registryid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for RegistryId`

- <span id="registryid-ord-cmp"></span>`fn cmp(&self, other: &RegistryId) -> cmp::Ordering` — [`RegistryId`](#registryid)

##### `impl PartialEq for RegistryId`

- <span id="registryid-partialeq-eq"></span>`fn eq(&self, other: &RegistryId) -> bool` — [`RegistryId`](#registryid)

##### `impl PartialOrd for RegistryId`

- <span id="registryid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &RegistryId) -> option::Option<cmp::Ordering>` — [`RegistryId`](#registryid)

##### `impl Pointable for RegistryId`

- <span id="registryid-pointable-const-align"></span>`const ALIGN: usize`

- <span id="registryid-pointable-type-init"></span>`type Init = T`

- <span id="registryid-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="registryid-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="registryid-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="registryid-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for RegistryId`

##### `impl ToOwned for RegistryId`

- <span id="registryid-toowned-type-owned"></span>`type Owned = T`

- <span id="registryid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="registryid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RegistryId`

- <span id="registryid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="registryid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegistryId`

- <span id="registryid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="registryid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThreadInfo`

```rust
struct ThreadInfo {
    primed: crate::latch::LockLatch,
    stopped: crate::latch::LockLatch,
    terminate: crate::latch::OnceLatch,
    stealer: crossbeam_deque::Stealer<crate::job::JobRef>,
}
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:613-631`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L613-L631)*

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

- <span id="threadinfo-new"></span>`fn new(stealer: Stealer<JobRef>) -> ThreadInfo` — [`JobRef`](../job/index.md#jobref), [`ThreadInfo`](#threadinfo)

#### Trait Implementations

##### `impl Any for ThreadInfo`

- <span id="threadinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreadInfo`

- <span id="threadinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreadInfo`

- <span id="threadinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ThreadInfo`

- <span id="threadinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreadInfo`

- <span id="threadinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for ThreadInfo`

- <span id="threadinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="threadinfo-pointable-type-init"></span>`type Init = T`

- <span id="threadinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="threadinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="threadinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="threadinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ThreadInfo`

- <span id="threadinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threadinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreadInfo`

- <span id="threadinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threadinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-core-1.13.0/src/registry.rs:647-663`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L647-L663)*

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

  Gets the `WorkerThread` index for the current thread; returns

  NULL if this is not a worker thread. This pointer is valid

  anywhere on the current thread.

- <span id="workerthread-set-current"></span>`unsafe fn set_current(thread: *const WorkerThread)` — [`WorkerThread`](#workerthread)

  Sets `self` as the worker-thread index for the current thread.

  This is done during worker-thread startup.

- <span id="workerthread-registry"></span>`fn registry(&self) -> &Arc<Registry>` — [`Registry`](#registry)

  Returns the registry that owns this worker thread.

- <span id="workerthread-index"></span>`fn index(&self) -> usize`

  Our index amongst the worker threads (ranges from `0..self.num_threads()`).

- <span id="workerthread-push"></span>`unsafe fn push(&self, job: JobRef)` — [`JobRef`](../job/index.md#jobref)

- <span id="workerthread-push-fifo"></span>`unsafe fn push_fifo(&self, job: JobRef)` — [`JobRef`](../job/index.md#jobref)

- <span id="workerthread-local-deque-is-empty"></span>`fn local_deque_is_empty(&self) -> bool`

- <span id="workerthread-take-local-job"></span>`fn take_local_job(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md#jobref)

  Attempts to obtain a "local" job -- typically this means

  popping from the top of the stack, though if we are configured

  for breadth-first execution, it would mean dequeuing from the

  bottom.

- <span id="workerthread-has-injected-job"></span>`fn has_injected_job(&self) -> bool`

- <span id="workerthread-wait-until"></span>`unsafe fn wait_until<L: AsCoreLatch + ?Sized>(&self, latch: &L)`

  Wait until the latch is set. Try to keep busy by popping and

  stealing tasks as necessary.

- <span id="workerthread-wait-until-cold"></span>`unsafe fn wait_until_cold(&self, latch: &CoreLatch)` — [`CoreLatch`](../latch/index.md#corelatch)

- <span id="workerthread-wait-until-out-of-work"></span>`unsafe fn wait_until_out_of_work(&self)`

- <span id="workerthread-find-work"></span>`fn find_work(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md#jobref)

- <span id="workerthread-yield-now"></span>`fn yield_now(&self) -> Yield` — [`Yield`](../thread_pool/index.md#yield)

- <span id="workerthread-yield-local"></span>`fn yield_local(&self) -> Yield` — [`Yield`](../thread_pool/index.md#yield)

- <span id="workerthread-execute"></span>`unsafe fn execute(&self, job: JobRef)` — [`JobRef`](../job/index.md#jobref)

- <span id="workerthread-steal"></span>`fn steal(&self) -> Option<JobRef>` — [`JobRef`](../job/index.md#jobref)

  Try to steal a single job and return it.

  

  This should only be done as a last resort, when there is no

  local work to do.

#### Trait Implementations

##### `impl Any for WorkerThread`

- <span id="workerthread-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WorkerThread`

- <span id="workerthread-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WorkerThread`

- <span id="workerthread-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for WorkerThread`

- <span id="workerthread-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for WorkerThread`

- <span id="workerthread-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WorkerThread`

- <span id="workerthread-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for WorkerThread`

- <span id="workerthread-pointable-const-align"></span>`const ALIGN: usize`

- <span id="workerthread-pointable-type-init"></span>`type Init = T`

- <span id="workerthread-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="workerthread-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="workerthread-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="workerthread-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WorkerThread`

- <span id="workerthread-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="workerthread-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WorkerThread`

- <span id="workerthread-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="workerthread-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XorShift64Star`

```rust
struct XorShift64Star {
    state: std::cell::Cell<u64>,
}
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:968-970`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L968-L970)*

[xorshift*] is a fast pseudorandom number generator which will
even tolerate weak seeding, as long as it's not zero.


#### Implementations

- <span id="xorshift64star-new"></span>`fn new() -> Self`

- <span id="xorshift64star-next"></span>`fn next(&self) -> u64`

- <span id="xorshift64star-next-usize"></span>`fn next_usize(&self, n: usize) -> usize`

  Return a value from `0..n`.

#### Trait Implementations

##### `impl Any for XorShift64Star`

- <span id="xorshift64star-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XorShift64Star`

- <span id="xorshift64star-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XorShift64Star`

- <span id="xorshift64star-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for XorShift64Star`

- <span id="xorshift64star-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XorShift64Star`

- <span id="xorshift64star-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for XorShift64Star`

- <span id="xorshift64star-pointable-const-align"></span>`const ALIGN: usize`

- <span id="xorshift64star-pointable-type-init"></span>`type Init = T`

- <span id="xorshift64star-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="xorshift64star-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="xorshift64star-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="xorshift64star-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for XorShift64Star`

- <span id="xorshift64star-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xorshift64star-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XorShift64Star`

- <span id="xorshift64star-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xorshift64star-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ThreadSpawn`

```rust
trait ThreadSpawn { ... }
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:69-75`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L69-L75)*

Generalized trait for spawning a thread in the `Registry`.

This trait is pub-in-private -- E0445 forces us to make it public,
but we don't actually want to expose these details in the API.

#### Required Methods

- `fn spawn(&mut self, thread: ThreadBuilder) -> io::Result<()>`

  Spawn a thread with the `ThreadBuilder` parameters, and then

#### Implementors

- [`CustomSpawn`](#customspawn)
- [`DefaultSpawn`](#defaultspawn)

## Functions

### `global_registry`

```rust
fn global_registry() -> &'static std::sync::Arc<Registry>
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:162-172`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L162-L172)*

Starts the worker threads (if that has not already happened). If
initialization has not already occurred, use the default
configuration.

### `init_global_registry`

```rust
fn init_global_registry<S>(builder: crate::ThreadPoolBuilder<S>) -> Result<&'static std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
where
    S: ThreadSpawn
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:176-183`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L176-L183)*

Starts the worker threads (if that has not already happened) with
the given builder.

### `set_global_registry`

```rust
fn set_global_registry<F>(registry: F) -> Result<&'static std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
where
    F: FnOnce() -> Result<std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:187-207`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L187-L207)*

Starts the worker threads (if that has not already happened)
by creating a registry with the given callback.

### `default_global_registry`

```rust
fn default_global_registry() -> Result<std::sync::Arc<Registry>, crate::ThreadPoolBuildError>
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:209-228`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L209-L228)*

### `main_loop`

```rust
unsafe fn main_loop(thread: ThreadBuilder)
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:910-939`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L910-L939)*

### `in_worker`

```rust
fn in_worker<OP, R>(op: OP) -> R
where
    OP: FnOnce(&WorkerThread, bool) -> R + Send,
    R: Send
```

*Defined in [`rayon-core-1.13.0/src/registry.rs:946-962`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L946-L962)*

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

*Defined in [`rayon-core-1.13.0/src/registry.rs:670-672`](../../../.source_1765521767/rayon-core-1.13.0/src/registry.rs#L670-L672)*

