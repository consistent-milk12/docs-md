*[rayon_core](../../index.md) / [sleep](../index.md) / [counters](index.md)*

---

# Module `counters`

## Contents

- [Structs](#structs)
  - [`AtomicCounters`](#atomiccounters)
  - [`Counters`](#counters)
  - [`JobsEventCounter`](#jobseventcounter)
- [Functions](#functions)
  - [`select_thread`](#select-thread)
  - [`select_jec`](#select-jec)
- [Constants](#constants)
  - [`THREADS_BITS`](#threads-bits)
  - [`SLEEPING_SHIFT`](#sleeping-shift)
  - [`INACTIVE_SHIFT`](#inactive-shift)
  - [`JEC_SHIFT`](#jec-shift)
  - [`THREADS_MAX`](#threads-max)
  - [`ONE_SLEEPING`](#one-sleeping)
  - [`ONE_INACTIVE`](#one-inactive)
  - [`ONE_JEC`](#one-jec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AtomicCounters`](#atomiccounters) | struct |  |
| [`Counters`](#counters) | struct |  |
| [`JobsEventCounter`](#jobseventcounter) | struct | A value read from the **Jobs Event Counter**. |
| [`select_thread`](#select-thread) | fn |  |
| [`select_jec`](#select-jec) | fn |  |
| [`THREADS_BITS`](#threads-bits) | const | Number of bits used for the thread counters. |
| [`SLEEPING_SHIFT`](#sleeping-shift) | const | Bits to shift to select the sleeping threads (used with `select_bits`). |
| [`INACTIVE_SHIFT`](#inactive-shift) | const | Bits to shift to select the inactive threads (used with `select_bits`). |
| [`JEC_SHIFT`](#jec-shift) | const | Bits to shift to select the JEC (use JOBS_BITS). |
| [`THREADS_MAX`](#threads-max) | const | Max value for the thread counters. |
| [`ONE_SLEEPING`](#one-sleeping) | const | Constant that can be added to add one sleeping thread. |
| [`ONE_INACTIVE`](#one-inactive) | const | Constant that can be added to add one inactive thread. |
| [`ONE_JEC`](#one-jec) | const | Constant that can be added to add one to the JEC. |

## Structs

### `AtomicCounters`

```rust
struct AtomicCounters {
    value: std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:3-16`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L3-L16)*

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

##### `impl Pointable for AtomicCounters`

- <span id="atomiccounters-pointable-const-align"></span>`const ALIGN: usize`

- <span id="atomiccounters-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:19-21`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L19-L21)*

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

##### `impl Pointable for Counters`

- <span id="counters-pointable-const-align"></span>`const ALIGN: usize`

- <span id="counters-pointable-type-init"></span>`type Init = T`

- <span id="counters-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="counters-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="counters-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="counters-drop"></span>`unsafe fn drop(ptr: usize)`

### `JobsEventCounter`

```rust
struct JobsEventCounter(usize);
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:27`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L27)*

A value read from the **Jobs Event Counter**.
See the [`README.md`](README.md) for more
coverage of how the jobs event counter works.

#### Implementations

- <span id="jobseventcounter-const-dummy"></span>`const DUMMY: JobsEventCounter`

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

##### `impl Pointable for JobsEventCounter`

- <span id="jobseventcounter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobseventcounter-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:210-212`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L210-L212)*

### `select_jec`

```rust
fn select_jec(word: usize) -> usize
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:215-217`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L215-L217)*

## Constants

### `THREADS_BITS`
```rust
const THREADS_BITS: usize = 16usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:57`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L57)*

Number of bits used for the thread counters.

### `SLEEPING_SHIFT`
```rust
const SLEEPING_SHIFT: usize = 0usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:65`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L65)*

Bits to shift to select the sleeping threads
(used with `select_bits`).

### `INACTIVE_SHIFT`
```rust
const INACTIVE_SHIFT: usize = 16usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:70`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L70)*

Bits to shift to select the inactive threads
(used with `select_bits`).

### `JEC_SHIFT`
```rust
const JEC_SHIFT: usize = 32usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:74`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L74)*

Bits to shift to select the JEC
(use JOBS_BITS).

### `THREADS_MAX`
```rust
const THREADS_MAX: usize = 65_535usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:77`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L77)*

Max value for the thread counters.

### `ONE_SLEEPING`
```rust
const ONE_SLEEPING: usize = 1usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:80`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L80)*

Constant that can be added to add one sleeping thread.

### `ONE_INACTIVE`
```rust
const ONE_INACTIVE: usize = 65_536usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:84`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L84)*

Constant that can be added to add one inactive thread.
An inactive thread is either idle, sleepy, or sleeping.

### `ONE_JEC`
```rust
const ONE_JEC: usize = 4_294_967_296usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:87`](../../../../.source_1765521767/rayon-core-1.13.0/src/sleep/counters.rs#L87)*

Constant that can be added to add one to the JEC.

