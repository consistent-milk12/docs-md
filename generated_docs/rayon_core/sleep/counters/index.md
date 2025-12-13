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

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:3-16`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L3-L16)*

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

  Load and return the current value of the various counters.

  This value can then be given to other method which will

  attempt to update the counters via compare-and-swap.

- <span id="atomiccounters-try-exchange"></span>`fn try_exchange(&self, old_value: Counters, new_value: Counters, ordering: Ordering) -> bool` — [`Counters`](#counters)

- <span id="atomiccounters-add-inactive-thread"></span>`fn add_inactive_thread(&self)`

  Adds an inactive thread. This cannot fail.

  

  This should be invoked when a thread enters its idle loop looking

  for work. It is decremented when work is found. Note that it is

  not decremented if the thread transitions from idle to sleepy or sleeping;

  so the number of inactive threads is always greater-than-or-equal

  to the number of sleeping threads.

- <span id="atomiccounters-increment-jobs-event-counter-if"></span>`fn increment_jobs_event_counter_if(&self, increment_when: impl Fn(JobsEventCounter) -> bool) -> Counters` — [`JobsEventCounter`](#jobseventcounter), [`Counters`](#counters)

  Increments the jobs event counter if `increment_when`, when applied to

  the current value, is true. Used to toggle the JEC from even (sleepy) to

  odd (active) or vice versa. Returns the final value of the counters, for

  which `increment_when` is guaranteed to return false.

- <span id="atomiccounters-sub-inactive-thread"></span>`fn sub_inactive_thread(&self) -> usize`

  Subtracts an inactive thread. This cannot fail. It is invoked

  when a thread finds work and hence becomes active. It returns the

  number of sleeping threads to wake up (if any).

  

  See `add_inactive_thread`.

- <span id="atomiccounters-sub-sleeping-thread"></span>`fn sub_sleeping_thread(&self)`

  Subtracts a sleeping thread. This cannot fail, but it is only

  safe to do if you you know the number of sleeping threads is

  non-zero (i.e., because you have just awoken a sleeping

  thread).

- <span id="atomiccounters-try-add-sleeping-thread"></span>`fn try_add_sleeping_thread(&self, old_value: Counters) -> bool` — [`Counters`](#counters)

#### Trait Implementations

##### `impl Any for AtomicCounters`

- <span id="atomiccounters-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicCounters`

- <span id="atomiccounters-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicCounters`

- <span id="atomiccounters-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicCounters`

- <span id="atomiccounters-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicCounters`

- <span id="atomiccounters-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for AtomicCounters`

- <span id="atomiccounters-pointable-const-align"></span>`const ALIGN: usize`

- <span id="atomiccounters-pointable-type-init"></span>`type Init = T`

- <span id="atomiccounters-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="atomiccounters-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomiccounters-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomiccounters-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for AtomicCounters`

- <span id="atomiccounters-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomiccounters-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicCounters`

- <span id="atomiccounters-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomiccounters-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Counters`

```rust
struct Counters {
    word: usize,
}
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:19-21`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L19-L21)*

#### Implementations

- <span id="counters-new"></span>`fn new(word: usize) -> Counters` — [`Counters`](#counters)

- <span id="counters-increment-jobs-counter"></span>`fn increment_jobs_counter(self) -> Counters` — [`Counters`](#counters)

- <span id="counters-jobs-counter"></span>`fn jobs_counter(self) -> JobsEventCounter` — [`JobsEventCounter`](#jobseventcounter)

- <span id="counters-inactive-threads"></span>`fn inactive_threads(self) -> usize`

  The number of threads that are not actively

  executing work. They may be idle, sleepy, or asleep.

- <span id="counters-awake-but-idle-threads"></span>`fn awake_but_idle_threads(self) -> usize`

- <span id="counters-sleeping-threads"></span>`fn sleeping_threads(self) -> usize`

#### Trait Implementations

##### `impl Any for Counters`

- <span id="counters-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Counters`

- <span id="counters-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Counters`

- <span id="counters-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Counters`

- <span id="counters-clone"></span>`fn clone(&self) -> Counters` — [`Counters`](#counters)

##### `impl CloneToUninit for Counters`

- <span id="counters-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Counters`

##### `impl Debug for Counters`

- <span id="counters-debug-fmt"></span>`fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> From for Counters`

- <span id="counters-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Counters`

- <span id="counters-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Counters`

- <span id="counters-pointable-const-align"></span>`const ALIGN: usize`

- <span id="counters-pointable-type-init"></span>`type Init = T`

- <span id="counters-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="counters-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="counters-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="counters-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Counters`

- <span id="counters-toowned-type-owned"></span>`type Owned = T`

- <span id="counters-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="counters-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Counters`

- <span id="counters-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="counters-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Counters`

- <span id="counters-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="counters-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JobsEventCounter`

```rust
struct JobsEventCounter(usize);
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:27`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L27)*

A value read from the **Jobs Event Counter**.
See the [`README.md`](README.md) for more
coverage of how the jobs event counter works.

#### Implementations

- <span id="jobseventcounter-const-dummy"></span>`const DUMMY: JobsEventCounter`

- <span id="jobseventcounter-as-usize"></span>`fn as_usize(self) -> usize`

- <span id="jobseventcounter-is-sleepy"></span>`fn is_sleepy(self) -> bool`

  The JEC "is sleepy" if the last thread to increment it was in the

  process of becoming sleepy. This is indicated by its value being *even*.

  When new jobs are posted, they check if the JEC is sleepy, and if so

  they incremented it.

- <span id="jobseventcounter-is-active"></span>`fn is_active(self) -> bool`

  The JEC "is active" if the last thread to increment it was posting new

  work. This is indicated by its value being *odd*. When threads get

  sleepy, they will check if the JEC is active, and increment it.

#### Trait Implementations

##### `impl Any for JobsEventCounter`

- <span id="jobseventcounter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JobsEventCounter`

- <span id="jobseventcounter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JobsEventCounter`

- <span id="jobseventcounter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for JobsEventCounter`

- <span id="jobseventcounter-clone"></span>`fn clone(&self) -> JobsEventCounter` — [`JobsEventCounter`](#jobseventcounter)

##### `impl CloneToUninit for JobsEventCounter`

- <span id="jobseventcounter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for JobsEventCounter`

##### `impl Debug for JobsEventCounter`

- <span id="jobseventcounter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for JobsEventCounter`

- <span id="jobseventcounter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JobsEventCounter`

- <span id="jobseventcounter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for JobsEventCounter`

- <span id="jobseventcounter-partialeq-eq"></span>`fn eq(&self, other: &JobsEventCounter) -> bool` — [`JobsEventCounter`](#jobseventcounter)

##### `impl PartialOrd for JobsEventCounter`

- <span id="jobseventcounter-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &JobsEventCounter) -> option::Option<cmp::Ordering>` — [`JobsEventCounter`](#jobseventcounter)

##### `impl Pointable for JobsEventCounter`

- <span id="jobseventcounter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="jobseventcounter-pointable-type-init"></span>`type Init = T`

- <span id="jobseventcounter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="jobseventcounter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="jobseventcounter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="jobseventcounter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for JobsEventCounter`

##### `impl ToOwned for JobsEventCounter`

- <span id="jobseventcounter-toowned-type-owned"></span>`type Owned = T`

- <span id="jobseventcounter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="jobseventcounter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for JobsEventCounter`

- <span id="jobseventcounter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jobseventcounter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JobsEventCounter`

- <span id="jobseventcounter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jobseventcounter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `select_thread`

```rust
fn select_thread(word: usize, shift: usize) -> usize
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:210-212`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L210-L212)*

### `select_jec`

```rust
fn select_jec(word: usize) -> usize
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:215-217`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L215-L217)*

## Constants

### `THREADS_BITS`
```rust
const THREADS_BITS: usize = 16usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:57`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L57)*

Number of bits used for the thread counters.

### `SLEEPING_SHIFT`
```rust
const SLEEPING_SHIFT: usize = 0usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:65`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L65)*

Bits to shift to select the sleeping threads
(used with `select_bits`).

### `INACTIVE_SHIFT`
```rust
const INACTIVE_SHIFT: usize = 16usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:70`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L70)*

Bits to shift to select the inactive threads
(used with `select_bits`).

### `JEC_SHIFT`
```rust
const JEC_SHIFT: usize = 32usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:74`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L74)*

Bits to shift to select the JEC
(use JOBS_BITS).

### `THREADS_MAX`
```rust
const THREADS_MAX: usize = 65_535usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:77`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L77)*

Max value for the thread counters.

### `ONE_SLEEPING`
```rust
const ONE_SLEEPING: usize = 1usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:80`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L80)*

Constant that can be added to add one sleeping thread.

### `ONE_INACTIVE`
```rust
const ONE_INACTIVE: usize = 65_536usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:84`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L84)*

Constant that can be added to add one inactive thread.
An inactive thread is either idle, sleepy, or sleeping.

### `ONE_JEC`
```rust
const ONE_JEC: usize = 4_294_967_296usize;
```

*Defined in [`rayon-core-1.13.0/src/sleep/counters.rs:87`](../../../../.source_1765633015/rayon-core-1.13.0/src/sleep/counters.rs#L87)*

Constant that can be added to add one to the JEC.

