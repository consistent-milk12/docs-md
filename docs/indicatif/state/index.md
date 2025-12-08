*[indicatif](../index.md) / [state](index.md)*

---

# Module `state`

## Structs

### `BarState`

```rust
struct BarState {
    draw_target: crate::draw_target::ProgressDrawTarget,
    on_finish: ProgressFinish,
    style: crate::style::ProgressStyle,
    state: ProgressState,
    tab_width: usize,
}
```

#### Implementations

- `fn new(len: Option<u64>, draw_target: ProgressDrawTarget, pos: Arc<AtomicPosition>) -> Self` — [`ProgressDrawTarget`](../index.md), [`AtomicPosition`](#atomicposition)

- `fn finish_using_style(self: &mut Self, now: Instant, finish: ProgressFinish)` — [`ProgressFinish`](../index.md)

- `fn reset(self: &mut Self, now: Instant, mode: Reset)` — [`Reset`](#reset)

- `fn update(self: &mut Self, now: Instant, f: impl FnOnce(&mut ProgressState), tick: bool)` — [`ProgressState`](../index.md)

- `fn unset_length(self: &mut Self, now: Instant)`

- `fn set_length(self: &mut Self, now: Instant, len: u64)`

- `fn inc_length(self: &mut Self, now: Instant, delta: u64)`

- `fn dec_length(self: &mut Self, now: Instant, delta: u64)`

- `fn set_tab_width(self: &mut Self, tab_width: usize)`

- `fn set_style(self: &mut Self, style: ProgressStyle)` — [`ProgressStyle`](../index.md)

- `fn tick(self: &mut Self, now: Instant)`

- `fn update_estimate_and_draw(self: &mut Self, now: Instant)`

- `fn println(self: &mut Self, now: Instant, msg: &str)`

- `fn suspend<F: FnOnce() -> R, R>(self: &mut Self, now: Instant, f: F) -> R`

- `fn draw(self: &mut Self, force_draw: bool, now: Instant) -> io::Result<()>`

#### Trait Implementations

##### `impl Drop for BarState`

- `fn drop(self: &mut Self)`

### `ProgressState`

```rust
struct ProgressState {
    pos: std::sync::Arc<AtomicPosition>,
    len: Option<u64>,
    tick: u64,
    started: std::time::Instant,
    status: Status,
    est: Estimator,
    message: TabExpandedString,
    prefix: TabExpandedString,
}
```

The state of a progress bar at a moment in time.

#### Implementations

- `fn new(len: Option<u64>, pos: Arc<AtomicPosition>) -> Self` — [`AtomicPosition`](#atomicposition)

- `fn is_finished(self: &Self) -> bool`

- `fn fraction(self: &Self) -> f32`

- `fn eta(self: &Self) -> Duration`

- `fn duration(self: &Self) -> Duration`

- `fn per_sec(self: &Self) -> f64`

- `fn elapsed(self: &Self) -> Duration`

- `fn pos(self: &Self) -> u64`

- `fn set_pos(self: &mut Self, pos: u64)`

- `fn len(self: &Self) -> Option<u64>`

- `fn set_len(self: &mut Self, len: u64)`

### `Estimator`

```rust
struct Estimator {
    smoothed_steps_per_sec: f64,
    double_smoothed_steps_per_sec: f64,
    prev_steps: u64,
    prev_time: std::time::Instant,
    start_time: std::time::Instant,
}
```

Double-smoothed exponentially weighted estimator

This uses an exponentially weighted *time-based* estimator, meaning that it exponentially
downweights old data based on its age. The rate at which this occurs is currently a constant
value of 15 seconds for 90% weighting. This means that all data older than 15 seconds has a
collective weight of 0.1 in the estimate, and all data older than 30 seconds has a collective
weight of 0.01, and so on.

The primary value exposed by `Estimator` is `steps_per_second`. This value is doubly-smoothed,
meaning that is the result of using an exponentially weighted estimator (as described above) to
estimate the value of another exponentially weighted estimator, which estimates the value of
the raw data.

The purpose of this extra smoothing step is to reduce instantaneous fluctations in the estimate
when large updates are received. Without this, estimates might have a large spike followed by a
slow asymptotic approach to zero (until the next spike).

#### Implementations

- `fn new(now: Instant) -> Self`

- `fn record(self: &mut Self, new_steps: u64, now: Instant)`

- `fn reset(self: &mut Self, now: Instant)`

- `fn steps_per_second(self: &Self, now: Instant) -> f64`

#### Trait Implementations

##### `impl Debug for Estimator`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AtomicPosition`

```rust
struct AtomicPosition {
    pos: portable_atomic::AtomicU64,
    capacity: portable_atomic::AtomicU8,
    prev: portable_atomic::AtomicU64,
    start: std::time::Instant,
}
```

#### Implementations

- `fn new() -> Self`

- `fn allow(self: &Self, now: Instant) -> bool`

- `fn reset(self: &Self, now: Instant)`

- `fn inc(self: &Self, delta: u64)`

- `fn dec(self: &Self, delta: u64)`

- `fn set(self: &Self, pos: u64)`

## Enums

### `Reset`

```rust
enum Reset {
    Eta,
    Elapsed,
    All,
}
```

### `TabExpandedString`

```rust
enum TabExpandedString {
    NoTabs(std::borrow::Cow<'static, str>),
    WithTabs {
        original: std::borrow::Cow<'static, str>,
        expanded: std::sync::OnceLock<String>,
        tab_width: usize,
    },
}
```

#### Implementations

- `fn new(s: Cow<'static, str>, tab_width: usize) -> Self`

- `fn expanded(self: &Self) -> &str`

- `fn set_tab_width(self: &mut Self, new_tab_width: usize)`

#### Trait Implementations

##### `impl Clone for TabExpandedString`

- `fn clone(self: &Self) -> TabExpandedString` — [`TabExpandedString`](#tabexpandedstring)

##### `impl Debug for TabExpandedString`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TabExpandedString`

##### `impl PartialEq for TabExpandedString`

- `fn eq(self: &Self, other: &TabExpandedString) -> bool` — [`TabExpandedString`](#tabexpandedstring)

##### `impl StructuralPartialEq for TabExpandedString`

### `ProgressFinish`

```rust
enum ProgressFinish {
    AndLeave,
    WithMessage(std::borrow::Cow<'static, str>),
    AndClear,
    Abandon,
    AbandonWithMessage(std::borrow::Cow<'static, str>),
}
```

Behavior of a progress bar when it is finished

This is invoked when a [`ProgressBar`](../index.md) or [`ProgressBarIter`](../index.md) completes and
`ProgressBar::is_finished` is false.




#### Variants

- **`AndLeave`**

  Finishes the progress bar and leaves the current message
  
  Same behavior as calling [`ProgressBar::finish()`](crate::ProgressBar::finish).

- **`WithMessage`**

  Finishes the progress bar and sets a message
  
  Same behavior as calling [`ProgressBar::finish_with_message()`](crate::ProgressBar::finish_with_message).

- **`AndClear`**

  Finishes the progress bar and completely clears it (this is the default)
  
  Same behavior as calling [`ProgressBar::finish_and_clear()`](crate::ProgressBar::finish_and_clear).

- **`Abandon`**

  Finishes the progress bar and leaves the current message and progress
  
  Same behavior as calling [`ProgressBar::abandon()`](crate::ProgressBar::abandon).

- **`AbandonWithMessage`**

  Finishes the progress bar and sets a message, and leaves the current progress
  
  Same behavior as calling [`ProgressBar::abandon_with_message()`](crate::ProgressBar::abandon_with_message).

#### Trait Implementations

##### `impl Clone for ProgressFinish`

- `fn clone(self: &Self) -> ProgressFinish` — [`ProgressFinish`](../index.md)

##### `impl Debug for ProgressFinish`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ProgressFinish`

- `fn default() -> ProgressFinish` — [`ProgressFinish`](../index.md)

### `Status`

```rust
enum Status {
    InProgress,
    DoneVisible,
    DoneHidden,
}
```

#### Trait Implementations

##### `impl Debug for Status`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `estimator_weight`

```rust
fn estimator_weight(age: f64) -> f64
```

Get the appropriate dilution weight for Estimator data given the data's age (in seconds)

Whenever an update occurs, we will create a new estimate using a weight `w_i` like so:

```math
<new estimate> = <previous estimate> * w_i + <new data> * (1 - w_i)
```

In other words, the new estimate is a weighted average of the previous estimate and the new
data. We want to choose weights such that for any set of samples where `t_0, t_1, ...` are
the durations of the samples:

```math
Sum(t_i) = ews ==> Prod(w_i) = 0.1
```

With this constraint it is easy to show that

```math
w_i = 0.1 ^ (t_i / ews)
```

Notice that the constraint implies that estimates are independent of the durations of the
samples, a very useful feature.

### `duration_to_secs`

```rust
fn duration_to_secs(d: std::time::Duration) -> f64
```

### `secs_to_duration`

```rust
fn secs_to_duration(s: f64) -> std::time::Duration
```

## Constants

### `INTERVAL`

```rust
const INTERVAL: u64 = 1_000_000u64;
```

### `MAX_BURST`

```rust
const MAX_BURST: u8 = 10u8;
```

### `DEFAULT_TAB_WIDTH`

```rust
const DEFAULT_TAB_WIDTH: usize = 8usize;
```

