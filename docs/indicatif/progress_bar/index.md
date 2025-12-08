*[indicatif](../index.md) / [progress_bar](index.md)*

---

# Module `progress_bar`

## Structs

### `ProgressBar`

```rust
struct ProgressBar {
    state: std::sync::Arc<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Arc<crate::state::AtomicPosition>,
    ticker: std::sync::Arc<std::sync::Mutex<Option<Ticker>>>,
}
```

A progress bar or spinner

The progress bar is an `Arc` around its internal state. When the progress bar is cloned it
just increments the refcount (so the original and its clone share the same state).

#### Implementations

- `fn new(len: u64) -> Self`

- `fn no_length() -> Self`

- `fn hidden() -> Self`

- `fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md)

- `fn style(self: &Self) -> ProgressStyle` — [`ProgressStyle`](../style/index.md)

- `fn with_style(self: Self, style: ProgressStyle) -> Self` — [`ProgressStyle`](../style/index.md)

- `fn with_tab_width(self: Self, tab_width: usize) -> Self`

- `fn with_prefix(self: Self, prefix: impl Into<Cow<'static, str>>) -> Self`

- `fn with_message(self: Self, message: impl Into<Cow<'static, str>>) -> Self`

- `fn with_position(self: Self, pos: u64) -> Self`

- `fn with_elapsed(self: Self, elapsed: Duration) -> Self`

- `fn with_finish(self: Self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](../state/index.md)

- `fn new_spinner() -> Self`

- `fn set_style(self: &Self, style: ProgressStyle)` — [`ProgressStyle`](../style/index.md)

- `fn set_tab_width(self: &Self, tab_width: usize)`

- `fn enable_steady_tick(self: &Self, interval: Duration)`

- `fn disable_steady_tick(self: &Self)`

- `fn stop_and_replace_ticker(self: &Self, interval: Option<Duration>)`

- `fn tick(self: &Self)`

- `fn tick_inner(self: &Self, now: Instant)`

- `fn inc(self: &Self, delta: u64)`

- `fn dec(self: &Self, delta: u64)`

- `fn is_hidden(self: &Self) -> bool`

- `fn is_finished(self: &Self) -> bool`

- `fn println<I: AsRef<str>>(self: &Self, msg: I)`

- `fn update(self: &Self, f: impl FnOnce(&mut ProgressState))` — [`ProgressState`](../state/index.md)

- `fn set_position(self: &Self, pos: u64)`

- `fn unset_length(self: &Self)`

- `fn set_length(self: &Self, len: u64)`

- `fn inc_length(self: &Self, delta: u64)`

- `fn dec_length(self: &Self, delta: u64)`

- `fn set_prefix(self: &Self, prefix: impl Into<Cow<'static, str>>)`

- `fn set_message(self: &Self, msg: impl Into<Cow<'static, str>>)`

- `fn set_elapsed(self: &Self, elapsed: Duration)`

- `fn downgrade(self: &Self) -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

- `fn reset_eta(self: &Self)`

- `fn reset_elapsed(self: &Self)`

- `fn reset(self: &Self)`

- `fn finish(self: &Self)`

- `fn finish_with_message(self: &Self, msg: impl Into<Cow<'static, str>>)`

- `fn finish_and_clear(self: &Self)`

- `fn abandon(self: &Self)`

- `fn abandon_with_message(self: &Self, msg: impl Into<Cow<'static, str>>)`

- `fn finish_using_style(self: &Self)`

- `fn set_draw_target(self: &Self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](../draw_target/index.md)

- `fn force_draw(self: &Self)`

- `fn suspend<F: FnOnce() -> R, R>(self: &Self, f: F) -> R`

- `fn wrap_iter<It: Iterator>(self: &Self, it: It) -> ProgressBarIter<It>` — [`ProgressBarIter`](../iter/index.md)

- `fn wrap_read<R: io::Read>(self: &Self, read: R) -> ProgressBarIter<R>` — [`ProgressBarIter`](../iter/index.md)

- `fn wrap_write<W: io::Write>(self: &Self, write: W) -> ProgressBarIter<W>` — [`ProgressBarIter`](../iter/index.md)

- `fn position(self: &Self) -> u64`

- `fn length(self: &Self) -> Option<u64>`

- `fn eta(self: &Self) -> Duration`

- `fn per_sec(self: &Self) -> f64`

- `fn duration(self: &Self) -> Duration`

- `fn elapsed(self: &Self) -> Duration`

- `fn index(self: &Self) -> Option<usize>`

- `fn message(self: &Self) -> String`

- `fn prefix(self: &Self) -> String`

- `fn state(self: &Self) -> MutexGuard<'_, BarState>` — [`BarState`](../state/index.md)

#### Trait Implementations

##### `impl Clone for ProgressBar`

- `fn clone(self: &Self) -> ProgressBar` — [`ProgressBar`](#progressbar)

##### `impl Debug for ProgressBar`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WeakProgressBar`

```rust
struct WeakProgressBar {
    state: std::sync::Weak<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Weak<crate::state::AtomicPosition>,
    ticker: std::sync::Weak<std::sync::Mutex<Option<Ticker>>>,
}
```

A weak reference to a [`ProgressBar`](#progressbar).

Useful for creating custom steady tick implementations

#### Implementations

- `fn new() -> Self`

- `fn upgrade(self: &Self) -> Option<ProgressBar>` — [`ProgressBar`](#progressbar)

#### Trait Implementations

##### `impl Clone for WeakProgressBar`

- `fn clone(self: &Self) -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

##### `impl Default for WeakProgressBar`

- `fn default() -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

### `Ticker`

```rust
struct Ticker {
    stopping: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    join_handle: Option<thread::JoinHandle<()>>,
}
```

#### Implementations

- `fn new(interval: Duration, bar_state: &Arc<Mutex<BarState>>) -> Self` — [`BarState`](../state/index.md)

- `fn stop(self: &Self)`

#### Trait Implementations

##### `impl Drop for Ticker`

- `fn drop(self: &mut Self)`

### `TickerControl`

```rust
struct TickerControl {
    stopping: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    state: std::sync::Weak<std::sync::Mutex<crate::state::BarState>>,
}
```

#### Implementations

- `fn run(self: &Self, interval: Duration)`

