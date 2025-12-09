*[indicatif](../index.md) / [progress_bar](index.md)*

---

# Module `progress_bar`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProgressBar`](#progressbar) | struct | A progress bar or spinner |
| [`WeakProgressBar`](#weakprogressbar) | struct | A weak reference to a [`ProgressBar`]. |
| [`Ticker`](#ticker) | struct |  |
| [`TickerControl`](#tickercontrol) | struct |  |

## Structs

### `ProgressBar`

```rust
struct ProgressBar {
    state: std::sync::Arc<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Arc<crate::state::AtomicPosition>,
    ticker: std::sync::Arc<std::sync::Mutex<Option<Ticker>>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:25-29`](../../../.source_1765210505/indicatif-0.18.3/src/progress_bar.rs#L25-L29)*

A progress bar or spinner

The progress bar is an `Arc` around its internal state. When the progress bar is cloned it
just increments the refcount (so the original and its clone share the same state).

#### Implementations

- <span id="progressbar-new"></span>`fn new(len: u64) -> Self`

- <span id="progressbar-no-length"></span>`fn no_length() -> Self`

- <span id="progressbar-hidden"></span>`fn hidden() -> Self`

- <span id="progressbar-with-draw-target"></span>`fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md)

- <span id="progressbar-style"></span>`fn style(&self) -> ProgressStyle` — [`ProgressStyle`](../style/index.md)

- <span id="progressbar-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](../style/index.md)

- <span id="progressbar-with-tab-width"></span>`fn with_tab_width(self, tab_width: usize) -> Self`

- <span id="progressbar-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbar-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbar-with-position"></span>`fn with_position(self, pos: u64) -> Self`

- <span id="progressbar-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

- <span id="progressbar-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](../state/index.md)

- <span id="progressbar-new-spinner"></span>`fn new_spinner() -> Self`

- <span id="progressbar-set-style"></span>`fn set_style(&self, style: ProgressStyle)` — [`ProgressStyle`](../style/index.md)

- <span id="progressbar-set-tab-width"></span>`fn set_tab_width(&self, tab_width: usize)`

- <span id="progressbar-enable-steady-tick"></span>`fn enable_steady_tick(&self, interval: Duration)`

- <span id="progressbar-disable-steady-tick"></span>`fn disable_steady_tick(&self)`

- <span id="progressbar-stop-and-replace-ticker"></span>`fn stop_and_replace_ticker(&self, interval: Option<Duration>)`

- <span id="progressbar-tick"></span>`fn tick(&self)`

- <span id="progressbar-tick-inner"></span>`fn tick_inner(&self, now: Instant)`

- <span id="progressbar-inc"></span>`fn inc(&self, delta: u64)`

- <span id="progressbar-dec"></span>`fn dec(&self, delta: u64)`

- <span id="progressbar-is-hidden"></span>`fn is_hidden(&self) -> bool`

- <span id="progressbar-is-finished"></span>`fn is_finished(&self) -> bool`

- <span id="progressbar-println"></span>`fn println<I: AsRef<str>>(&self, msg: I)`

- <span id="progressbar-update"></span>`fn update(&self, f: impl FnOnce(&mut ProgressState))` — [`ProgressState`](../state/index.md)

- <span id="progressbar-set-position"></span>`fn set_position(&self, pos: u64)`

- <span id="progressbar-unset-length"></span>`fn unset_length(&self)`

- <span id="progressbar-set-length"></span>`fn set_length(&self, len: u64)`

- <span id="progressbar-inc-length"></span>`fn inc_length(&self, delta: u64)`

- <span id="progressbar-dec-length"></span>`fn dec_length(&self, delta: u64)`

- <span id="progressbar-set-prefix"></span>`fn set_prefix(&self, prefix: impl Into<Cow<'static, str>>)`

- <span id="progressbar-set-message"></span>`fn set_message(&self, msg: impl Into<Cow<'static, str>>)`

- <span id="progressbar-set-elapsed"></span>`fn set_elapsed(&self, elapsed: Duration)`

- <span id="progressbar-downgrade"></span>`fn downgrade(&self) -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

- <span id="progressbar-reset-eta"></span>`fn reset_eta(&self)`

- <span id="progressbar-reset-elapsed"></span>`fn reset_elapsed(&self)`

- <span id="progressbar-reset"></span>`fn reset(&self)`

- <span id="progressbar-finish"></span>`fn finish(&self)`

- <span id="progressbar-finish-with-message"></span>`fn finish_with_message(&self, msg: impl Into<Cow<'static, str>>)`

- <span id="progressbar-finish-and-clear"></span>`fn finish_and_clear(&self)`

- <span id="progressbar-abandon"></span>`fn abandon(&self)`

- <span id="progressbar-abandon-with-message"></span>`fn abandon_with_message(&self, msg: impl Into<Cow<'static, str>>)`

- <span id="progressbar-finish-using-style"></span>`fn finish_using_style(&self)`

- <span id="progressbar-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](../draw_target/index.md)

- <span id="progressbar-force-draw"></span>`fn force_draw(&self)`

- <span id="progressbar-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

- <span id="progressbar-wrap-iter"></span>`fn wrap_iter<It: Iterator>(&self, it: It) -> ProgressBarIter<It>` — [`ProgressBarIter`](../iter/index.md)

- <span id="progressbar-wrap-read"></span>`fn wrap_read<R: io::Read>(&self, read: R) -> ProgressBarIter<R>` — [`ProgressBarIter`](../iter/index.md)

- <span id="progressbar-wrap-write"></span>`fn wrap_write<W: io::Write>(&self, write: W) -> ProgressBarIter<W>` — [`ProgressBarIter`](../iter/index.md)

- <span id="progressbar-position"></span>`fn position(&self) -> u64`

- <span id="progressbar-length"></span>`fn length(&self) -> Option<u64>`

- <span id="progressbar-eta"></span>`fn eta(&self) -> Duration`

- <span id="progressbar-per-sec"></span>`fn per_sec(&self) -> f64`

- <span id="progressbar-duration"></span>`fn duration(&self) -> Duration`

- <span id="progressbar-elapsed"></span>`fn elapsed(&self) -> Duration`

- <span id="progressbar-index"></span>`fn index(&self) -> Option<usize>`

- <span id="progressbar-message"></span>`fn message(&self) -> String`

- <span id="progressbar-prefix"></span>`fn prefix(&self) -> String`

- <span id="progressbar-state"></span>`fn state(&self) -> MutexGuard<'_, BarState>` — [`BarState`](../state/index.md)

#### Trait Implementations

##### `impl Clone for ProgressBar`

- <span id="progressbar-clone"></span>`fn clone(&self) -> ProgressBar` — [`ProgressBar`](#progressbar)

##### `impl Debug for ProgressBar`

- <span id="progressbar-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WeakProgressBar`

```rust
struct WeakProgressBar {
    state: std::sync::Weak<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Weak<crate::state::AtomicPosition>,
    ticker: std::sync::Weak<std::sync::Mutex<Option<Ticker>>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:651-655`](../../../.source_1765210505/indicatif-0.18.3/src/progress_bar.rs#L651-L655)*

A weak reference to a [`ProgressBar`](#progressbar).

Useful for creating custom steady tick implementations

#### Implementations

- <span id="weakprogressbar-new"></span>`fn new() -> Self`

- <span id="weakprogressbar-upgrade"></span>`fn upgrade(&self) -> Option<ProgressBar>` — [`ProgressBar`](#progressbar)

#### Trait Implementations

##### `impl Clone for WeakProgressBar`

- <span id="weakprogressbar-clone"></span>`fn clone(&self) -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

##### `impl Default for WeakProgressBar`

- <span id="weakprogressbar-default"></span>`fn default() -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

### `Ticker`

```rust
struct Ticker {
    stopping: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    join_handle: Option<thread::JoinHandle<()>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:677-680`](../../../.source_1765210505/indicatif-0.18.3/src/progress_bar.rs#L677-L680)*

#### Implementations

- <span id="ticker-new"></span>`fn new(interval: Duration, bar_state: &Arc<Mutex<BarState>>) -> Self` — [`BarState`](../state/index.md)

- <span id="ticker-stop"></span>`fn stop(&self)`

#### Trait Implementations

##### `impl Drop for Ticker`

- <span id="ticker-drop"></span>`fn drop(&mut self)`

### `TickerControl`

```rust
struct TickerControl {
    stopping: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    state: std::sync::Weak<std::sync::Mutex<crate::state::BarState>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:719-722`](../../../.source_1765210505/indicatif-0.18.3/src/progress_bar.rs#L719-L722)*

#### Implementations

- <span id="tickercontrol-run"></span>`fn run(&self, interval: Duration)`

