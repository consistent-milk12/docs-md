*[indicatif](../index.md) / [state](index.md)*

---

# Module `state`

## Contents

- [Structs](#structs)
  - [`BarState`](#barstate)
  - [`ProgressState`](#progressstate)
  - [`Estimator`](#estimator)
  - [`AtomicPosition`](#atomicposition)
- [Enums](#enums)
  - [`Reset`](#reset)
  - [`TabExpandedString`](#tabexpandedstring)
  - [`ProgressFinish`](#progressfinish)
  - [`Status`](#status)
- [Functions](#functions)
  - [`estimator_weight`](#estimator_weight)
  - [`duration_to_secs`](#duration_to_secs)
  - [`secs_to_duration`](#secs_to_duration)
- [Constants](#constants)
  - [`INTERVAL`](#interval)
  - [`MAX_BURST`](#max_burst)
  - [`DEFAULT_TAB_WIDTH`](#default_tab_width)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BarState`](#barstate) | struct |  |
| [`ProgressState`](#progressstate) | struct | The state of a progress bar at a moment in time. |
| [`Estimator`](#estimator) | struct | Double-smoothed exponentially weighted estimator |
| [`AtomicPosition`](#atomicposition) | struct |  |
| [`Reset`](#reset) | enum |  |
| [`TabExpandedString`](#tabexpandedstring) | enum |  |
| [`ProgressFinish`](#progressfinish) | enum | Behavior of a progress bar when it is finished |
| [`Status`](#status) | enum |  |
| [`estimator_weight`](#estimator_weight) | fn | Get the appropriate dilution weight for Estimator data given the data's age (in seconds) |
| [`duration_to_secs`](#duration_to_secs) | fn |  |
| [`secs_to_duration`](#secs_to_duration) | fn |  |
| [`INTERVAL`](#interval) | const |  |
| [`MAX_BURST`](#max_burst) | const |  |
| [`DEFAULT_TAB_WIDTH`](#default_tab_width) | const |  |

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

- <span id="barstate-new"></span>`fn new(len: Option<u64>, draw_target: ProgressDrawTarget, pos: Arc<AtomicPosition>) -> Self` — [`ProgressDrawTarget`](../index.md), [`AtomicPosition`](#atomicposition)

- <span id="barstate-finish-using-style"></span>`fn finish_using_style(&mut self, now: Instant, finish: ProgressFinish)` — [`ProgressFinish`](../index.md)

- <span id="barstate-reset"></span>`fn reset(&mut self, now: Instant, mode: Reset)` — [`Reset`](#reset)

- <span id="barstate-update"></span>`fn update(&mut self, now: Instant, f: impl FnOnce(&mut ProgressState), tick: bool)` — [`ProgressState`](../index.md)

- <span id="barstate-unset-length"></span>`fn unset_length(&mut self, now: Instant)`

- <span id="barstate-set-length"></span>`fn set_length(&mut self, now: Instant, len: u64)`

- <span id="barstate-inc-length"></span>`fn inc_length(&mut self, now: Instant, delta: u64)`

- <span id="barstate-dec-length"></span>`fn dec_length(&mut self, now: Instant, delta: u64)`

- <span id="barstate-set-tab-width"></span>`fn set_tab_width(&mut self, tab_width: usize)`

- <span id="barstate-set-style"></span>`fn set_style(&mut self, style: ProgressStyle)` — [`ProgressStyle`](../index.md)

- <span id="barstate-tick"></span>`fn tick(&mut self, now: Instant)`

- <span id="barstate-update-estimate-and-draw"></span>`fn update_estimate_and_draw(&mut self, now: Instant)`

- <span id="barstate-println"></span>`fn println(&mut self, now: Instant, msg: &str)`

- <span id="barstate-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&mut self, now: Instant, f: F) -> R`

- <span id="barstate-draw"></span>`fn draw(&mut self, force_draw: bool, now: Instant) -> io::Result<()>`

#### Trait Implementations

##### `impl Drop for BarState`

- <span id="barstate-drop"></span>`fn drop(&mut self)`

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

- <span id="progressstate-new"></span>`fn new(len: Option<u64>, pos: Arc<AtomicPosition>) -> Self` — [`AtomicPosition`](#atomicposition)

- <span id="progressstate-is-finished"></span>`fn is_finished(&self) -> bool`

- <span id="progressstate-fraction"></span>`fn fraction(&self) -> f32`

- <span id="progressstate-eta"></span>`fn eta(&self) -> Duration`

- <span id="progressstate-duration"></span>`fn duration(&self) -> Duration`

- <span id="progressstate-per-sec"></span>`fn per_sec(&self) -> f64`

- <span id="progressstate-elapsed"></span>`fn elapsed(&self) -> Duration`

- <span id="progressstate-pos"></span>`fn pos(&self) -> u64`

- <span id="progressstate-set-pos"></span>`fn set_pos(&mut self, pos: u64)`

- <span id="progressstate-len"></span>`fn len(&self) -> Option<u64>`

- <span id="progressstate-set-len"></span>`fn set_len(&mut self, len: u64)`

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

- <span id="estimator-new"></span>`fn new(now: Instant) -> Self`

- <span id="estimator-record"></span>`fn record(&mut self, new_steps: u64, now: Instant)`

- <span id="estimator-reset"></span>`fn reset(&mut self, now: Instant)`

- <span id="estimator-steps-per-second"></span>`fn steps_per_second(&self, now: Instant) -> f64`

#### Trait Implementations

##### `impl Debug for Estimator`

- <span id="estimator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="atomicposition-new"></span>`fn new() -> Self`

- <span id="atomicposition-allow"></span>`fn allow(&self, now: Instant) -> bool`

- <span id="atomicposition-reset"></span>`fn reset(&self, now: Instant)`

- <span id="atomicposition-inc"></span>`fn inc(&self, delta: u64)`

- <span id="atomicposition-dec"></span>`fn dec(&self, delta: u64)`

- <span id="atomicposition-set"></span>`fn set(&self, pos: u64)`

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

- <span id="tabexpandedstring-new"></span>`fn new(s: Cow<'static, str>, tab_width: usize) -> Self`

- <span id="tabexpandedstring-expanded"></span>`fn expanded(&self) -> &str`

- <span id="tabexpandedstring-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

#### Trait Implementations

##### `impl Clone for TabExpandedString`

- <span id="tabexpandedstring-clone"></span>`fn clone(&self) -> TabExpandedString` — [`TabExpandedString`](#tabexpandedstring)

##### `impl Debug for TabExpandedString`

- <span id="tabexpandedstring-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TabExpandedString`

##### `impl PartialEq for TabExpandedString`

- <span id="tabexpandedstring-eq"></span>`fn eq(&self, other: &TabExpandedString) -> bool` — [`TabExpandedString`](#tabexpandedstring)

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

- <span id="progressfinish-clone"></span>`fn clone(&self) -> ProgressFinish` — [`ProgressFinish`](../index.md)

##### `impl Debug for ProgressFinish`

- <span id="progressfinish-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ProgressFinish`

- <span id="progressfinish-default"></span>`fn default() -> ProgressFinish` — [`ProgressFinish`](../index.md)

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

- <span id="status-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

