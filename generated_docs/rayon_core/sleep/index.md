*[rayon_core](../index.md) / [sleep](index.md)*

---

# Module `sleep`

Code that decides when workers should go to sleep. See README.md
for an overview.

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

- `fn new(n_threads: usize) -> Sleep` — [`Sleep`](#sleep)

- `fn start_looking(self: &Self, worker_index: usize) -> IdleState` — [`IdleState`](#idlestate)

- `fn work_found(self: &Self)`

- `fn no_work_found(self: &Self, idle_state: &mut IdleState, latch: &CoreLatch, has_injected_jobs: impl FnOnce() -> bool)` — [`IdleState`](#idlestate), [`CoreLatch`](../latch/index.md)

- `fn announce_sleepy(self: &Self) -> JobsEventCounter` — [`JobsEventCounter`](counters/index.md)

- `fn sleep(self: &Self, idle_state: &mut IdleState, latch: &CoreLatch, has_injected_jobs: impl FnOnce() -> bool)` — [`IdleState`](#idlestate), [`CoreLatch`](../latch/index.md)

- `fn notify_worker_latch_is_set(self: &Self, target_worker_index: usize)`

- `fn new_injected_jobs(self: &Self, num_jobs: u32, queue_was_empty: bool)`

- `fn new_internal_jobs(self: &Self, num_jobs: u32, queue_was_empty: bool)`

- `fn new_jobs(self: &Self, num_jobs: u32, queue_was_empty: bool)`

- `fn wake_any_threads(self: &Self, num_to_wake: u32)`

- `fn wake_specific_thread(self: &Self, index: usize) -> bool`

#### Trait Implementations

##### `impl<T> Pointable for Sleep`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn wake_fully(self: &mut Self)`

- `fn wake_partly(self: &mut Self)`

#### Trait Implementations

##### `impl<T> Pointable for IdleState`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn default() -> WorkerSleepState` — [`WorkerSleepState`](#workersleepstate)

##### `impl<T> Pointable for WorkerSleepState`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Constants

### `ROUNDS_UNTIL_SLEEPY`

```rust
const ROUNDS_UNTIL_SLEEPY: u32 = 32u32;
```

### `ROUNDS_UNTIL_SLEEPING`

```rust
const ROUNDS_UNTIL_SLEEPING: u32 = 33u32;
```

