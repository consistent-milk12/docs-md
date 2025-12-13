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
  - [`estimator_weight`](#estimator-weight)
  - [`duration_to_secs`](#duration-to-secs)
  - [`secs_to_duration`](#secs-to-duration)
- [Constants](#constants)
  - [`INTERVAL`](#interval)
  - [`MAX_BURST`](#max-burst)
  - [`DEFAULT_TAB_WIDTH`](#default-tab-width)

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
| [`estimator_weight`](#estimator-weight) | fn | Get the appropriate dilution weight for Estimator data given the data's age (in seconds) |
| [`duration_to_secs`](#duration-to-secs) | fn |  |
| [`secs_to_duration`](#secs-to-duration) | fn |  |
| [`INTERVAL`](#interval) | const |  |
| [`MAX_BURST`](#max-burst) | const |  |
| [`DEFAULT_TAB_WIDTH`](#default-tab-width) | const |  |

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

*Defined in [`indicatif-0.18.3/src/state.rs:15-21`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L15-L21)*

#### Implementations

- <span id="barstate-new"></span>`fn new(len: Option<u64>, draw_target: ProgressDrawTarget, pos: Arc<AtomicPosition>) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget), [`AtomicPosition`](#atomicposition)

- <span id="barstate-finish-using-style"></span>`fn finish_using_style(&mut self, now: Instant, finish: ProgressFinish)` — [`ProgressFinish`](#progressfinish)

  Finishes the progress bar using the [`ProgressFinish`](#progressfinish) behavior stored

  in the [`ProgressStyle`](../style/index.md).

- <span id="barstate-reset"></span>`fn reset(&mut self, now: Instant, mode: Reset)` — [`Reset`](#reset)

- <span id="barstate-update"></span>`fn update(&mut self, now: Instant, f: impl FnOnce(&mut ProgressState), tick: bool)` — [`ProgressState`](#progressstate)

- <span id="barstate-unset-length"></span>`fn unset_length(&mut self, now: Instant)`

- <span id="barstate-set-length"></span>`fn set_length(&mut self, now: Instant, len: u64)`

- <span id="barstate-inc-length"></span>`fn inc_length(&mut self, now: Instant, delta: u64)`

- <span id="barstate-dec-length"></span>`fn dec_length(&mut self, now: Instant, delta: u64)`

- <span id="barstate-set-tab-width"></span>`fn set_tab_width(&mut self, tab_width: usize)`

- <span id="barstate-set-style"></span>`fn set_style(&mut self, style: ProgressStyle)` — [`ProgressStyle`](../style/index.md#progressstyle)

- <span id="barstate-tick"></span>`fn tick(&mut self, now: Instant)`

- <span id="barstate-update-estimate-and-draw"></span>`fn update_estimate_and_draw(&mut self, now: Instant)`

- <span id="barstate-println"></span>`fn println(&mut self, now: Instant, msg: &str)`

- <span id="barstate-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&mut self, now: Instant, f: F) -> R`

- <span id="barstate-draw"></span>`fn draw(&mut self, force_draw: bool, now: Instant) -> io::Result<()>`

#### Trait Implementations

##### `impl Any for BarState`

- <span id="barstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BarState`

- <span id="barstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BarState`

- <span id="barstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for BarState`

- <span id="barstate-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for BarState`

- <span id="barstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BarState`

- <span id="barstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BarState`

- <span id="barstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="barstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BarState`

- <span id="barstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="barstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/state.rs:242-251`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L242-L251)*

The state of a progress bar at a moment in time.

#### Implementations

- <span id="progressstate-new"></span>`fn new(len: Option<u64>, pos: Arc<AtomicPosition>) -> Self` — [`AtomicPosition`](#atomicposition)

- <span id="progressstate-is-finished"></span>`fn is_finished(&self) -> bool`

  Indicates that the progress bar finished.

- <span id="progressstate-fraction"></span>`fn fraction(&self) -> f32`

  Returns the completion as a floating-point number between 0 and 1

- <span id="progressstate-eta"></span>`fn eta(&self) -> Duration`

  The expected ETA

- <span id="progressstate-duration"></span>`fn duration(&self) -> Duration`

  The expected total duration (that is, elapsed time + expected ETA)

- <span id="progressstate-per-sec"></span>`fn per_sec(&self) -> f64`

  The number of steps per second

- <span id="progressstate-elapsed"></span>`fn elapsed(&self) -> Duration`

- <span id="progressstate-pos"></span>`fn pos(&self) -> u64`

- <span id="progressstate-set-pos"></span>`fn set_pos(&mut self, pos: u64)`

- <span id="progressstate-len"></span>`fn len(&self) -> Option<u64>`

- <span id="progressstate-set-len"></span>`fn set_len(&mut self, len: u64)`

#### Trait Implementations

##### `impl Any for ProgressState`

- <span id="progressstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressState`

- <span id="progressstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressState`

- <span id="progressstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ProgressState`

- <span id="progressstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressState`

- <span id="progressstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ProgressState`

- <span id="progressstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressState`

- <span id="progressstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/state.rs:421-427`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L421-L427)*

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

  Reset the state of the estimator. Once reset, estimates will not depend on any data prior

  to `now`. This does not reset the stored position of the progress bar.

- <span id="estimator-steps-per-second"></span>`fn steps_per_second(&self, now: Instant) -> f64`

  Average time per step in seconds, using double exponential smoothing

#### Trait Implementations

##### `impl Any for Estimator`

- <span id="estimator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Estimator`

- <span id="estimator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Estimator`

- <span id="estimator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Estimator`

- <span id="estimator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Estimator`

- <span id="estimator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Estimator`

- <span id="estimator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Estimator`

- <span id="estimator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="estimator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Estimator`

- <span id="estimator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="estimator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicPosition`

```rust
struct AtomicPosition {
    pos: portable_atomic::AtomicU64,
    capacity: portable_atomic::AtomicU8,
    prev: portable_atomic::AtomicU64,
    start: std::time::Instant,
}
```

*Defined in [`indicatif-0.18.3/src/state.rs:532-537`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L532-L537)*

#### Implementations

- <span id="atomicposition-new"></span>`fn new() -> Self`

- <span id="atomicposition-allow"></span>`fn allow(&self, now: Instant) -> bool`

- <span id="atomicposition-reset"></span>`fn reset(&self, now: Instant)`

- <span id="atomicposition-inc"></span>`fn inc(&self, delta: u64)`

- <span id="atomicposition-dec"></span>`fn dec(&self, delta: u64)`

- <span id="atomicposition-set"></span>`fn set(&self, pos: u64)`

#### Trait Implementations

##### `impl Any for AtomicPosition`

- <span id="atomicposition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicPosition`

- <span id="atomicposition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicPosition`

- <span id="atomicposition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicPosition`

- <span id="atomicposition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicPosition`

- <span id="atomicposition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AtomicPosition`

- <span id="atomicposition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicposition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicPosition`

- <span id="atomicposition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicposition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Reset`

```rust
enum Reset {
    Eta,
    Elapsed,
    All,
}
```

*Defined in [`indicatif-0.18.3/src/state.rs:234-238`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L234-L238)*

#### Trait Implementations

##### `impl Any for Reset`

- <span id="reset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Reset`

- <span id="reset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Reset`

- <span id="reset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Reset`

- <span id="reset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Reset`

- <span id="reset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Reset`

- <span id="reset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Reset`

- <span id="reset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/state.rs:353-360`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L353-L360)*

#### Implementations

- <span id="tabexpandedstring-new"></span>`fn new(s: Cow<'static, str>, tab_width: usize) -> Self`

- <span id="tabexpandedstring-expanded"></span>`fn expanded(&self) -> &str`

- <span id="tabexpandedstring-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

#### Trait Implementations

##### `impl Any for TabExpandedString`

- <span id="tabexpandedstring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TabExpandedString`

- <span id="tabexpandedstring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TabExpandedString`

- <span id="tabexpandedstring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TabExpandedString`

- <span id="tabexpandedstring-clone"></span>`fn clone(&self) -> TabExpandedString` — [`TabExpandedString`](#tabexpandedstring)

##### `impl CloneToUninit for TabExpandedString`

- <span id="tabexpandedstring-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TabExpandedString`

- <span id="tabexpandedstring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TabExpandedString`

##### `impl<T> From for TabExpandedString`

- <span id="tabexpandedstring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TabExpandedString`

- <span id="tabexpandedstring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TabExpandedString`

- <span id="tabexpandedstring-partialeq-eq"></span>`fn eq(&self, other: &TabExpandedString) -> bool` — [`TabExpandedString`](#tabexpandedstring)

##### `impl StructuralPartialEq for TabExpandedString`

##### `impl ToOwned for TabExpandedString`

- <span id="tabexpandedstring-toowned-type-owned"></span>`type Owned = T`

- <span id="tabexpandedstring-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tabexpandedstring-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TabExpandedString`

- <span id="tabexpandedstring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tabexpandedstring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TabExpandedString`

- <span id="tabexpandedstring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tabexpandedstring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/state.rs:615-637`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L615-L637)*

Behavior of a progress bar when it is finished

This is invoked when a [`ProgressBar`](../progress_bar/index.md) or [`ProgressBarIter`](../iter/index.md) completes and
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

##### `impl Any for ProgressFinish`

- <span id="progressfinish-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressFinish`

- <span id="progressfinish-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressFinish`

- <span id="progressfinish-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProgressFinish`

- <span id="progressfinish-clone"></span>`fn clone(&self) -> ProgressFinish` — [`ProgressFinish`](#progressfinish)

##### `impl CloneToUninit for ProgressFinish`

- <span id="progressfinish-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ProgressFinish`

- <span id="progressfinish-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ProgressFinish`

- <span id="progressfinish-default"></span>`fn default() -> ProgressFinish` — [`ProgressFinish`](#progressfinish)

##### `impl<T> From for ProgressFinish`

- <span id="progressfinish-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressFinish`

- <span id="progressfinish-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ProgressFinish`

- <span id="progressfinish-toowned-type-owned"></span>`type Owned = T`

- <span id="progressfinish-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="progressfinish-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ProgressFinish`

- <span id="progressfinish-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressfinish-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressFinish`

- <span id="progressfinish-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressfinish-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Status`

```rust
enum Status {
    InProgress,
    DoneVisible,
    DoneHidden,
}
```

*Defined in [`indicatif-0.18.3/src/state.rs:679-683`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L679-L683)*

#### Trait Implementations

##### `impl Any for Status`

- <span id="status-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Status`

- <span id="status-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Status`

- <span id="status-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Status`

- <span id="status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Status`

- <span id="status-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Status`

- <span id="status-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Status`

- <span id="status-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="status-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Status`

- <span id="status-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="status-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `estimator_weight`

```rust
fn estimator_weight(age: f64) -> f64
```

*Defined in [`indicatif-0.18.3/src/state.rs:663-666`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L663-L666)*

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

*Defined in [`indicatif-0.18.3/src/state.rs:668-670`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L668-L670)*

### `secs_to_duration`

```rust
fn secs_to_duration(s: f64) -> std::time::Duration
```

*Defined in [`indicatif-0.18.3/src/state.rs:672-676`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L672-L676)*

## Constants

### `INTERVAL`
```rust
const INTERVAL: u64 = 1_000_000u64;
```

*Defined in [`indicatif-0.18.3/src/state.rs:603`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L603)*

### `MAX_BURST`
```rust
const MAX_BURST: u8 = 10u8;
```

*Defined in [`indicatif-0.18.3/src/state.rs:604`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L604)*

### `DEFAULT_TAB_WIDTH`
```rust
const DEFAULT_TAB_WIDTH: usize = 8usize;
```

*Defined in [`indicatif-0.18.3/src/state.rs:685`](../../../.source_1765521767/indicatif-0.18.3/src/state.rs#L685)*

