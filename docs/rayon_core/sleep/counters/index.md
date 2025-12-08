*[rayon_core](../../index.md) / [sleep](../index.md) / [counters](index.md)*

---

# Module `counters`

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

- `fn new() -> AtomicCounters` — [`AtomicCounters`](#atomiccounters)

- `fn load(self: &Self, ordering: Ordering) -> Counters` — [`Counters`](#counters)

- `fn try_exchange(self: &Self, old_value: Counters, new_value: Counters, ordering: Ordering) -> bool` — [`Counters`](#counters)

- `fn add_inactive_thread(self: &Self)`

- `fn increment_jobs_event_counter_if(self: &Self, increment_when: impl Fn(JobsEventCounter) -> bool) -> Counters` — [`JobsEventCounter`](#jobseventcounter), [`Counters`](#counters)

- `fn sub_inactive_thread(self: &Self) -> usize`

- `fn sub_sleeping_thread(self: &Self)`

- `fn try_add_sleeping_thread(self: &Self, old_value: Counters) -> bool` — [`Counters`](#counters)

#### Trait Implementations

##### `impl<T> Pointable for AtomicCounters`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Counters`

```rust
struct Counters {
    word: usize,
}
```

#### Implementations

- `fn new(word: usize) -> Counters` — [`Counters`](#counters)

- `fn increment_jobs_counter(self: Self) -> Counters` — [`Counters`](#counters)

- `fn jobs_counter(self: Self) -> JobsEventCounter` — [`JobsEventCounter`](#jobseventcounter)

- `fn inactive_threads(self: Self) -> usize`

- `fn awake_but_idle_threads(self: Self) -> usize`

- `fn sleeping_threads(self: Self) -> usize`

#### Trait Implementations

##### `impl Clone for Counters`

- `fn clone(self: &Self) -> Counters` — [`Counters`](#counters)

##### `impl Copy for Counters`

##### `impl Debug for Counters`

- `fn fmt(self: &Self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> Pointable for Counters`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `JobsEventCounter`

```rust
struct JobsEventCounter(usize);
```

A value read from the **Jobs Event Counter**.
See the [`README.md`](README.md) for more
coverage of how the jobs event counter works.

#### Implementations

- `const DUMMY: JobsEventCounter`

- `fn as_usize(self: Self) -> usize`

- `fn is_sleepy(self: Self) -> bool`

- `fn is_active(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for JobsEventCounter`

- `fn clone(self: &Self) -> JobsEventCounter` — [`JobsEventCounter`](#jobseventcounter)

##### `impl Copy for JobsEventCounter`

##### `impl Debug for JobsEventCounter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for JobsEventCounter`

- `fn eq(self: &Self, other: &JobsEventCounter) -> bool` — [`JobsEventCounter`](#jobseventcounter)

##### `impl PartialOrd for JobsEventCounter`

- `fn partial_cmp(self: &Self, other: &JobsEventCounter) -> $crate::option::Option<$crate::cmp::Ordering>` — [`JobsEventCounter`](#jobseventcounter)

##### `impl<T> Pointable for JobsEventCounter`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

