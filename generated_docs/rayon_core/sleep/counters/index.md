*[rayon_core](../../index.md) / [sleep](../index.md) / [counters](index.md)*

---

# Module `counters`

## Contents

- [Structs](#structs)
  - [`AtomicCounters`](#atomiccounters)
  - [`Counters`](#counters)
  - [`JobsEventCounter`](#jobseventcounter)
- [Functions](#functions)
  - [`select_thread`](#select_thread)
  - [`select_jec`](#select_jec)
- [Constants](#constants)
  - [`THREADS_BITS`](#threads_bits)
  - [`SLEEPING_SHIFT`](#sleeping_shift)
  - [`INACTIVE_SHIFT`](#inactive_shift)
  - [`JEC_SHIFT`](#jec_shift)
  - [`THREADS_MAX`](#threads_max)
  - [`ONE_SLEEPING`](#one_sleeping)
  - [`ONE_INACTIVE`](#one_inactive)
  - [`ONE_JEC`](#one_jec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AtomicCounters`](#atomiccounters) | struct |  |
| [`Counters`](#counters) | struct |  |
| [`JobsEventCounter`](#jobseventcounter) | struct | A value read from the **Jobs Event Counter**. |
| [`select_thread`](#select_thread) | fn |  |
| [`select_jec`](#select_jec) | fn |  |
| [`THREADS_BITS`](#threads_bits) | const | Number of bits used for the thread counters. |
| [`SLEEPING_SHIFT`](#sleeping_shift) | const | Bits to shift to select the sleeping threads |
| [`INACTIVE_SHIFT`](#inactive_shift) | const | Bits to shift to select the inactive threads |
| [`JEC_SHIFT`](#jec_shift) | const | Bits to shift to select the JEC |
| [`THREADS_MAX`](#threads_max) | const | Max value for the thread counters. |
| [`ONE_SLEEPING`](#one_sleeping) | const | Constant that can be added to add one sleeping thread. |
| [`ONE_INACTIVE`](#one_inactive) | const | Constant that can be added to add one inactive thread. |
| [`ONE_JEC`](#one_jec) | const | Constant that can be added to add one to the JEC. |

## Structs

### `AtomicCounters`

```rust
struct AtomicCounters {
    value: std::sync::atomic::AtomicUsize,
}
```

#### Fields

- **`value`**: `std::sync::atomic::AtomicUsize`

  Packs together a number of counters. The counters are ordered as
  follows, from least to most significant bits (here, we assuming
  that [`THREADS_BITS`](#threads-bits) is equal to 10):
  
  * Bits 0..10: Stores the number of **sleeping threads**
  * Bits 10..20: Stores the number of **inactive threads**
  * Bits 20..: Stores the **job event counter** (JEC)
  
  This uses 10 bits ([`THREADS_BITS`](#threads-bits)) to encode the number of threads. Note
  that the total number of bits (and hence the number of bits used for the
  JEC) will depend on whether we are using a 32- or 64-bit architecture.

#### Implementations

- <span id="atomiccounters-new"></span>`fn new() -> AtomicCounters` — [`AtomicCounters`](#atomiccounters)

- <span id="atomiccounters-load"></span>`fn load(&self, ordering: Ordering) -> Counters` — [`Counters`](#counters)

- <span id="atomiccounters-try-exchange"></span>`fn try_exchange(&self, old_value: Counters, new_value: Counters, ordering: Ordering) -> bool` — [`Counters`](#counters)

- <span id="atomiccounters-add-inactive-thread"></span>`fn add_inactive_thread(&self)`

- <span id="atomiccounters-increment-jobs-event-counter-if"></span>`fn increment_jobs_event_counter_if(&self, increment_when: impl Fn(JobsEventCounter) -> bool) -> Counters` — [`JobsEventCounter`](#jobseventcounter), [`Counters`](#counters)

- <span id="atomiccounters-sub-inactive-thread"></span>`fn sub_inactive_thread(&self) -> usize`

- <span id="atomiccounters-sub-sleeping-thread"></span>`fn sub_sleeping_thread(&self)`

- <span id="atomiccounters-try-add-sleeping-thread"></span>`fn try_add_sleeping_thread(&self, old_value: Counters) -> bool` — [`Counters`](#counters)

#### Trait Implementations

##### `impl<T> Pointable for AtomicCounters`

- <span id="atomiccounters-align"></span>`const ALIGN: usize`

- <span id="atomiccounters-init"></span>`type Init = T`

- <span id="atomiccounters-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="atomiccounters-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomiccounters-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomiccounters-drop"></span>`unsafe fn drop(ptr: usize)`

### `Counters`

```rust
struct Counters {
    word: usize,
}
```

#### Implementations

- <span id="counters-new"></span>`fn new(word: usize) -> Counters` — [`Counters`](#counters)

- <span id="counters-increment-jobs-counter"></span>`fn increment_jobs_counter(self) -> Counters` — [`Counters`](#counters)

- <span id="counters-jobs-counter"></span>`fn jobs_counter(self) -> JobsEventCounter` — [`JobsEventCounter`](#jobseventcounter)

- <span id="counters-inactive-threads"></span>`fn inactive_threads(self) -> usize`

- <span id="counters-awake-but-idle-threads"></span>`fn awake_but_idle_threads(self) -> usize`

- <span id="counters-sleeping-threads"></span>`fn sleeping_threads(self) -> usize`

#### Trait Implementations

##### `impl Clone for Counters`

- <span id="counters-clone"></span>`fn clone(&self) -> Counters` — [`Counters`](#counters)

##### `impl Copy for Counters`

##### `impl Debug for Counters`

- <span id="counters-fmt"></span>`fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> Pointable for Counters`

- <span id="counters-align"></span>`const ALIGN: usize`

- <span id="counters-init"></span>`type Init = T`

- <span id="counters-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="counters-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="counters-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="counters-drop"></span>`unsafe fn drop(ptr: usize)`

### `JobsEventCounter`

```rust
struct JobsEventCounter(usize);
```

A value read from the **Jobs Event Counter**.
See the [`README.md`](README.md) for more
coverage of how the jobs event counter works.

#### Implementations

- <span id="jobseventcounter-dummy"></span>`const DUMMY: JobsEventCounter`

- <span id="jobseventcounter-as-usize"></span>`fn as_usize(self) -> usize`

- <span id="jobseventcounter-is-sleepy"></span>`fn is_sleepy(self) -> bool`

- <span id="jobseventcounter-is-active"></span>`fn is_active(self) -> bool`

#### Trait Implementations

##### `impl Clone for JobsEventCounter`

- <span id="jobseventcounter-clone"></span>`fn clone(&self) -> JobsEventCounter` — [`JobsEventCounter`](#jobseventcounter)

##### `impl Copy for JobsEventCounter`

##### `impl Debug for JobsEventCounter`

- <span id="jobseventcounter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for JobsEventCounter`

- <span id="jobseventcounter-eq"></span>`fn eq(&self, other: &JobsEventCounter) -> bool` — [`JobsEventCounter`](#jobseventcounter)

##### `impl PartialOrd for JobsEventCounter`

- <span id="jobseventcounter-partial-cmp"></span>`fn partial_cmp(&self, other: &JobsEventCounter) -> option::Option<cmp::Ordering>` — [`JobsEventCounter`](#jobseventcounter)

##### `impl<T> Pointable for JobsEventCounter`

- <span id="jobseventcounter-align"></span>`const ALIGN: usize`

- <span id="jobseventcounter-init"></span>`type Init = T`

- <span id="jobseventcounter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobseventcounter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobseventcounter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobseventcounter-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for JobsEventCounter`

## Functions

### `select_thread`

```rust
fn select_thread(word: usize, shift: usize) -> usize
```

### `select_jec`

```rust
fn select_jec(word: usize) -> usize
```

## Constants

### `THREADS_BITS`

```rust
const THREADS_BITS: usize = 16usize;
```

Number of bits used for the thread counters.

### `SLEEPING_SHIFT`

```rust
const SLEEPING_SHIFT: usize = 0usize;
```

Bits to shift to select the sleeping threads
(used with `select_bits`).

### `INACTIVE_SHIFT`

```rust
const INACTIVE_SHIFT: usize = 16usize;
```

Bits to shift to select the inactive threads
(used with `select_bits`).

### `JEC_SHIFT`

```rust
const JEC_SHIFT: usize = 32usize;
```

Bits to shift to select the JEC
(use JOBS_BITS).

### `THREADS_MAX`

```rust
const THREADS_MAX: usize = 65_535usize;
```

Max value for the thread counters.

### `ONE_SLEEPING`

```rust
const ONE_SLEEPING: usize = 1usize;
```

Constant that can be added to add one sleeping thread.

### `ONE_INACTIVE`

```rust
const ONE_INACTIVE: usize = 65_536usize;
```

Constant that can be added to add one inactive thread.
An inactive thread is either idle, sleepy, or sleeping.

### `ONE_JEC`

```rust
const ONE_JEC: usize = 4_294_967_296usize;
```

Constant that can be added to add one to the JEC.

