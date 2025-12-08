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
| [`ROUNDS_UNTIL_SLEEPY`](#rounds_until_sleepy) | const |  |
| [`ROUNDS_UNTIL_SLEEPING`](#rounds_until_sleeping) | const |  |

## Modules

- [`counters`](counters/index.md) - 

## Structs

### `Sleep`

```rust
struct Sleep {
    worker_sleep_states: Vec<crossbeam_utils::CachePadded<WorkerSleepState>>,
    counters: self::counters::AtomicCounters,
}
```

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

- <span id="sleep-no-work-found"></span>`fn no_work_found(&self, idle_state: &mut IdleState, latch: &CoreLatch, has_injected_jobs: impl FnOnce() -> bool)` — [`IdleState`](#idlestate), [`CoreLatch`](../latch/index.md)

- <span id="sleep-announce-sleepy"></span>`fn announce_sleepy(&self) -> JobsEventCounter` — [`JobsEventCounter`](counters/index.md)

- <span id="sleep-sleep"></span>`fn sleep(&self, idle_state: &mut IdleState, latch: &CoreLatch, has_injected_jobs: impl FnOnce() -> bool)` — [`IdleState`](#idlestate), [`CoreLatch`](../latch/index.md)

- <span id="sleep-notify-worker-latch-is-set"></span>`fn notify_worker_latch_is_set(&self, target_worker_index: usize)`

- <span id="sleep-new-injected-jobs"></span>`fn new_injected_jobs(&self, num_jobs: u32, queue_was_empty: bool)`

- <span id="sleep-new-internal-jobs"></span>`fn new_internal_jobs(&self, num_jobs: u32, queue_was_empty: bool)`

- <span id="sleep-new-jobs"></span>`fn new_jobs(&self, num_jobs: u32, queue_was_empty: bool)`

- <span id="sleep-wake-any-threads"></span>`fn wake_any_threads(&self, num_to_wake: u32)`

- <span id="sleep-wake-specific-thread"></span>`fn wake_specific_thread(&self, index: usize) -> bool`

#### Trait Implementations

##### `impl<T> Pointable for Sleep`

- <span id="sleep-align"></span>`const ALIGN: usize`

- <span id="sleep-init"></span>`type Init = T`

- <span id="sleep-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sleep-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sleep-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sleep-drop"></span>`unsafe fn drop(ptr: usize)`

### `IdleState`

```rust
struct IdleState {
    worker_index: usize,
    rounds: u32,
    jobs_counter: self::counters::JobsEventCounter,
}
```

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

##### `impl<T> Pointable for IdleState`

- <span id="idlestate-align"></span>`const ALIGN: usize`

- <span id="idlestate-init"></span>`type Init = T`

- <span id="idlestate-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="idlestate-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="idlestate-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="idlestate-drop"></span>`unsafe fn drop(ptr: usize)`

### `WorkerSleepState`

```rust
struct WorkerSleepState {
    is_blocked: crate::sync::Mutex<bool>,
    condvar: crate::sync::Condvar,
}
```

The "sleep state" for an individual worker.

#### Fields

- **`is_blocked`**: `crate::sync::Mutex<bool>`

  Set to true when the worker goes to sleep; set to false when
  the worker is notified or when it wakes.

#### Trait Implementations

##### `impl Default for WorkerSleepState`

- <span id="workersleepstate-default"></span>`fn default() -> WorkerSleepState` — [`WorkerSleepState`](#workersleepstate)

##### `impl<T> Pointable for WorkerSleepState`

- <span id="workersleepstate-align"></span>`const ALIGN: usize`

- <span id="workersleepstate-init"></span>`type Init = T`

- <span id="workersleepstate-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="workersleepstate-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="workersleepstate-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="workersleepstate-drop"></span>`unsafe fn drop(ptr: usize)`

## Constants

### `ROUNDS_UNTIL_SLEEPY`

```rust
const ROUNDS_UNTIL_SLEEPY: u32 = 32u32;
```

### `ROUNDS_UNTIL_SLEEPING`

```rust
const ROUNDS_UNTIL_SLEEPING: u32 = 33u32;
```

