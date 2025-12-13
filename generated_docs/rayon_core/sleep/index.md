*[rayon_core](../index.md) / [sleep](index.md)*

---

# Module `sleep`

Code that decides when workers should go to sleep. See README.md
for an overview.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`counters`](#counters) | mod |  |
| [`Sleep`](#sleep) | struct | The `Sleep` struct is embedded into each registry. |
| [`IdleState`](#idlestate) | struct | An instance of this struct is created when a thread becomes idle. |
| [`WorkerSleepState`](#workersleepstate) | struct | The "sleep state" for an individual worker. |
| [`ROUNDS_UNTIL_SLEEPY`](#rounds-until-sleepy) | const |  |
| [`ROUNDS_UNTIL_SLEEPING`](#rounds-until-sleeping) | const |  |

## Modules

- [`counters`](counters/index.md)

## Structs

### `Sleep`

```rust
struct Sleep {
    worker_sleep_states: Vec<crossbeam_utils::CachePadded<WorkerSleepState>>,
    counters: self::counters::AtomicCounters,
}
```

*Defined in [`rayon-core-1.13.0/src/sleep/mod.rs:21-27`](../../../.source_1765633015/rayon-core-1.13.0/src/sleep/mod.rs#L21-L27)*

The `Sleep` struct is embedded into each registry. It governs the waking and sleeping
of workers. It has callbacks that are invoked periodically at significant events,
such as when workers are looping and looking for work, when latches are set, or when
jobs are published, and it either blocks threads or wakes them in response to these
events. See the `README.md` in this module for more details.

`README.md` README.md

#### Fields

- **`worker_sleep_states`**: `Vec<crossbeam_utils::CachePadded<WorkerSleepState>>`

  One "sleep state" per worker. Used to track if a worker is sleeping and to have
  them block.

#### Implementations

- <span id="sleep-new"></span>`fn new(n_threads: usize) -> Sleep` — [`Sleep`](#sleep)

- <span id="sleep-start-looking"></span>`fn start_looking(&self, worker_index: usize) -> IdleState` — [`IdleState`](#idlestate)

- <span id="sleep-work-found"></span>`fn work_found(&self)`

- <span id="sleep-no-work-found"></span>`fn no_work_found(&self, idle_state: &mut IdleState, latch: &CoreLatch, has_injected_jobs: impl FnOnce() -> bool)` — [`IdleState`](#idlestate), [`CoreLatch`](../latch/index.md#corelatch)

- <span id="sleep-announce-sleepy"></span>`fn announce_sleepy(&self) -> JobsEventCounter` — [`JobsEventCounter`](counters/index.md#jobseventcounter)

- <span id="sleep-sleep"></span>`fn sleep(&self, idle_state: &mut IdleState, latch: &CoreLatch, has_injected_jobs: impl FnOnce() -> bool)` — [`IdleState`](#idlestate), [`CoreLatch`](../latch/index.md#corelatch)

- <span id="sleep-notify-worker-latch-is-set"></span>`fn notify_worker_latch_is_set(&self, target_worker_index: usize)`

  Notify the given thread that it should wake up (if it is

  sleeping).  When this method is invoked, we typically know the

  thread is asleep, though in rare cases it could have been

  awoken by (e.g.) new work having been posted.

- <span id="sleep-new-injected-jobs"></span>`fn new_injected_jobs(&self, num_jobs: u32, queue_was_empty: bool)`

  Signals that `num_jobs` new jobs were injected into the thread

  pool from outside. This function will ensure that there are

  threads available to process them, waking threads from sleep

  if necessary.

  

  # Parameters

  

  - `num_jobs` -- lower bound on number of jobs available for stealing.

    We'll try to get at least one thread per job.

- <span id="sleep-new-internal-jobs"></span>`fn new_internal_jobs(&self, num_jobs: u32, queue_was_empty: bool)`

  Signals that `num_jobs` new jobs were pushed onto a thread's

  local deque. This function will try to ensure that there are

  threads available to process them, waking threads from sleep

  if necessary. However, this is not guaranteed: under certain

  race conditions, the function may fail to wake any new

  threads; in that case the existing thread should eventually

  pop the job.

  

  # Parameters

  

  - `num_jobs` -- lower bound on number of jobs available for stealing.

    We'll try to get at least one thread per job.

- <span id="sleep-new-jobs"></span>`fn new_jobs(&self, num_jobs: u32, queue_was_empty: bool)`

  Common helper for `new_injected_jobs` and `new_internal_jobs`.

- <span id="sleep-wake-any-threads"></span>`fn wake_any_threads(&self, num_to_wake: u32)`

- <span id="sleep-wake-specific-thread"></span>`fn wake_specific_thread(&self, index: usize) -> bool`

#### Trait Implementations

##### `impl Any for Sleep`

- <span id="sleep-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Sleep`

- <span id="sleep-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Sleep`

- <span id="sleep-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Sleep`

- <span id="sleep-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Sleep`

- <span id="sleep-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Sleep`

- <span id="sleep-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sleep-pointable-type-init"></span>`type Init = T`

- <span id="sleep-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sleep-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sleep-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sleep-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Sleep`

- <span id="sleep-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sleep-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Sleep`

- <span id="sleep-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sleep-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IdleState`

```rust
struct IdleState {
    worker_index: usize,
    rounds: u32,
    jobs_counter: self::counters::JobsEventCounter,
}
```

*Defined in [`rayon-core-1.13.0/src/sleep/mod.rs:34-44`](../../../.source_1765633015/rayon-core-1.13.0/src/sleep/mod.rs#L34-L44)*

An instance of this struct is created when a thread becomes idle.
It is consumed when the thread finds work, and passed by `&mut`
reference for operations that preserve the idle state. (In other
words, producing one of these structs is evidence the thread is
idle.) It tracks state such as how long the thread has been idle.

#### Fields

- **`worker_index`**: `usize`

  What is worker index of the idle thread?

- **`rounds`**: `u32`

  How many rounds have we been circling without sleeping?

- **`jobs_counter`**: `self::counters::JobsEventCounter`

  Once we become sleepy, what was the sleepy counter value?
  Set to `INVALID_SLEEPY_COUNTER` otherwise.

#### Implementations

- <span id="idlestate-wake-fully"></span>`fn wake_fully(&mut self)`

- <span id="idlestate-wake-partly"></span>`fn wake_partly(&mut self)`

#### Trait Implementations

##### `impl Any for IdleState`

- <span id="idlestate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IdleState`

- <span id="idlestate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IdleState`

- <span id="idlestate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IdleState`

- <span id="idlestate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IdleState`

- <span id="idlestate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for IdleState`

- <span id="idlestate-pointable-const-align"></span>`const ALIGN: usize`

- <span id="idlestate-pointable-type-init"></span>`type Init = T`

- <span id="idlestate-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="idlestate-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="idlestate-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="idlestate-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for IdleState`

- <span id="idlestate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="idlestate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IdleState`

- <span id="idlestate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="idlestate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WorkerSleepState`

```rust
struct WorkerSleepState {
    is_blocked: crate::sync::Mutex<bool>,
    condvar: crate::sync::Condvar,
}
```

*Defined in [`rayon-core-1.13.0/src/sleep/mod.rs:48-54`](../../../.source_1765633015/rayon-core-1.13.0/src/sleep/mod.rs#L48-L54)*

The "sleep state" for an individual worker.

#### Fields

- **`is_blocked`**: `crate::sync::Mutex<bool>`

  Set to true when the worker goes to sleep; set to false when
  the worker is notified or when it wakes.

#### Trait Implementations

##### `impl Any for WorkerSleepState`

- <span id="workersleepstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WorkerSleepState`

- <span id="workersleepstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WorkerSleepState`

- <span id="workersleepstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for WorkerSleepState`

- <span id="workersleepstate-default"></span>`fn default() -> WorkerSleepState` — [`WorkerSleepState`](#workersleepstate)

##### `impl<T> From for WorkerSleepState`

- <span id="workersleepstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WorkerSleepState`

- <span id="workersleepstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for WorkerSleepState`

- <span id="workersleepstate-pointable-const-align"></span>`const ALIGN: usize`

- <span id="workersleepstate-pointable-type-init"></span>`type Init = T`

- <span id="workersleepstate-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="workersleepstate-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="workersleepstate-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="workersleepstate-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WorkerSleepState`

- <span id="workersleepstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="workersleepstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WorkerSleepState`

- <span id="workersleepstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="workersleepstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `ROUNDS_UNTIL_SLEEPY`
```rust
const ROUNDS_UNTIL_SLEEPY: u32 = 32u32;
```

*Defined in [`rayon-core-1.13.0/src/sleep/mod.rs:56`](../../../.source_1765633015/rayon-core-1.13.0/src/sleep/mod.rs#L56)*

### `ROUNDS_UNTIL_SLEEPING`
```rust
const ROUNDS_UNTIL_SLEEPING: u32 = 33u32;
```

*Defined in [`rayon-core-1.13.0/src/sleep/mod.rs:57`](../../../.source_1765633015/rayon-core-1.13.0/src/sleep/mod.rs#L57)*

