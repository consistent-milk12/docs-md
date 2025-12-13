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

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:25-29`](../../../.source_1765521767/indicatif-0.18.3/src/progress_bar.rs#L25-L29)*

A progress bar or spinner

The progress bar is an `Arc` around its internal state. When the progress bar is cloned it
just increments the refcount (so the original and its clone share the same state).

#### Implementations

- <span id="progressbar-new"></span>`fn new(len: u64) -> Self`

  Creates a new progress bar with a given length

  

  This progress bar by default draws directly to stderr, and refreshes a maximum of 20 times

  a second. To change the refresh rate, [`set`](../../hashbrown/set/index.md) the [draw target] to one with a different refresh

  rate.

  

- <span id="progressbar-no-length"></span>`fn no_length() -> Self`

  Creates a new progress bar without a specified length

  

  This progress bar by default draws directly to stderr, and refreshes a maximum of 20 times

  a second. To change the refresh rate, [`set`](../../hashbrown/set/index.md) the [draw target] to one with a different refresh

  rate.

  

- <span id="progressbar-hidden"></span>`fn hidden() -> Self`

  Creates a completely hidden progress bar

  

  This progress bar still responds to API changes but it does not have a length or render in

  any way.

- <span id="progressbar-with-draw-target"></span>`fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

  Creates a new progress bar with a given length and draw target

- <span id="progressbar-style"></span>`fn style(&self) -> ProgressStyle` — [`ProgressStyle`](../style/index.md#progressstyle)

  Get a clone of the current progress bar style.

- <span id="progressbar-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](../style/index.md#progressstyle)

  A convenience builder-like function for a progress bar with a given style

- <span id="progressbar-with-tab-width"></span>`fn with_tab_width(self, tab_width: usize) -> Self`

  A convenience builder-like function for a progress bar with a given tab width

- <span id="progressbar-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

  A convenience builder-like function for a progress bar with a given prefix

  

  For the prefix to be visible, the `{prefix}` placeholder must be present in the template

  (see [`ProgressStyle`](../style/index.md)).

- <span id="progressbar-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

  A convenience builder-like function for a progress bar with a given message

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](../style/index.md)).

- <span id="progressbar-with-position"></span>`fn with_position(self, pos: u64) -> Self`

  A convenience builder-like function for a progress bar with a given position

- <span id="progressbar-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

  A convenience builder-like function for a progress bar with a given elapsed time

- <span id="progressbar-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](../state/index.md#progressfinish)

  Sets the finish behavior for the progress bar

  

  This behavior is invoked when [`ProgressBar`](#progressbar) or

  [`ProgressBarIter`](../iter/index.md) completes and

  `ProgressBar::is_finished()` is false.

  If you don't want the progress bar to be automatically finished then

  call `with_finish(Abandon)`.

  

  

- <span id="progressbar-new-spinner"></span>`fn new_spinner() -> Self`

  Creates a new spinner

  

  This spinner by default draws directly to stderr. This adds the default spinner style to it.

- <span id="progressbar-set-style"></span>`fn set_style(&self, style: ProgressStyle)` — [`ProgressStyle`](../style/index.md#progressstyle)

  Overrides the stored style

  

  This does not redraw the bar. Call `ProgressBar::tick()` to force it.

- <span id="progressbar-set-tab-width"></span>`fn set_tab_width(&self, tab_width: usize)`

  Sets the tab width (default: 8). All tabs will be expanded to this many spaces.

- <span id="progressbar-enable-steady-tick"></span>`fn enable_steady_tick(&self, interval: Duration)`

  Spawns a background thread to tick the progress bar

  

  When this is enabled a background thread will regularly tick the progress bar in the given

  interval. This is useful to advance progress bars that are very slow by themselves.

  

  When steady ticks are enabled, calling `ProgressBar::tick()` on a progress bar does not

  have any effect.

- <span id="progressbar-disable-steady-tick"></span>`fn disable_steady_tick(&self)`

  Undoes `ProgressBar::enable_steady_tick()`

- <span id="progressbar-stop-and-replace-ticker"></span>`fn stop_and_replace_ticker(&self, interval: Option<Duration>)`

- <span id="progressbar-tick"></span>`fn tick(&self)`

  Manually ticks the spinner or progress bar

  

  This automatically happens on any other change to a progress bar.

- <span id="progressbar-tick-inner"></span>`fn tick_inner(&self, now: Instant)`

- <span id="progressbar-inc"></span>`fn inc(&self, delta: u64)`

  Advances the position of the progress bar by `delta`

- <span id="progressbar-dec"></span>`fn dec(&self, delta: u64)`

  Decrease the position of the progress bar by `delta`

- <span id="progressbar-is-hidden"></span>`fn is_hidden(&self) -> bool`

  A quick convenience check if the progress bar is hidden

- <span id="progressbar-is-finished"></span>`fn is_finished(&self) -> bool`

  Indicates that the progress bar finished

- <span id="progressbar-println"></span>`fn println<I: AsRef<str>>(&self, msg: I)`

  Print a log line above the progress bar

  

  If the progress bar is hidden (e.g. when standard output is not a terminal), `println()`

  will not do anything. If you want to write to the standard output in such cases as well, use

  `ProgressBar::suspend()` instead.

  

  If the progress bar was added to a [`MultiProgress`](../multi/index.md), the log line will be

  printed above all other progress bars.

  

- <span id="progressbar-update"></span>`fn update(&self, f: impl FnOnce(&mut ProgressState))` — [`ProgressState`](../state/index.md#progressstate)

  Update the `ProgressBar`'s inner [`ProgressState`](../state/index.md)

- <span id="progressbar-set-position"></span>`fn set_position(&self, pos: u64)`

  Sets the position of the progress bar

- <span id="progressbar-unset-length"></span>`fn unset_length(&self)`

  Sets the length of the progress bar to `None`

- <span id="progressbar-set-length"></span>`fn set_length(&self, len: u64)`

  Sets the length of the progress bar

- <span id="progressbar-inc-length"></span>`fn inc_length(&self, delta: u64)`

  Increase the length of the progress bar

- <span id="progressbar-dec-length"></span>`fn dec_length(&self, delta: u64)`

  Decrease the length of the progress bar

- <span id="progressbar-set-prefix"></span>`fn set_prefix(&self, prefix: impl Into<Cow<'static, str>>)`

  Sets the current prefix of the progress bar

  

  For the prefix to be visible, the `{prefix}` placeholder must be present in the template

  (see [`ProgressStyle`](../style/index.md)).

- <span id="progressbar-set-message"></span>`fn set_message(&self, msg: impl Into<Cow<'static, str>>)`

  Sets the current message of the progress bar

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](../style/index.md)).

- <span id="progressbar-set-elapsed"></span>`fn set_elapsed(&self, elapsed: Duration)`

  Sets the elapsed time for the progress bar

- <span id="progressbar-downgrade"></span>`fn downgrade(&self) -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

  Creates a new weak reference to this [`ProgressBar`](#progressbar)

- <span id="progressbar-reset-eta"></span>`fn reset_eta(&self)`

  Resets the ETA calculation

  

  This can be useful if the progress bars made a large jump or was paused for a prolonged

  time.

- <span id="progressbar-reset-elapsed"></span>`fn reset_elapsed(&self)`

  Resets elapsed time and the ETA calculation

- <span id="progressbar-reset"></span>`fn reset(&self)`

  Resets all of the progress bar state

- <span id="progressbar-finish"></span>`fn finish(&self)`

  Finishes the progress bar and leaves the current message

- <span id="progressbar-finish-with-message"></span>`fn finish_with_message(&self, msg: impl Into<Cow<'static, str>>)`

  Finishes the progress bar and sets a message

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](../style/index.md)).

- <span id="progressbar-finish-and-clear"></span>`fn finish_and_clear(&self)`

  Finishes the progress bar and completely clears it

- <span id="progressbar-abandon"></span>`fn abandon(&self)`

  Finishes the progress bar and leaves the current message and progress

- <span id="progressbar-abandon-with-message"></span>`fn abandon_with_message(&self, msg: impl Into<Cow<'static, str>>)`

  Finishes the progress bar and sets a message, and leaves the current progress

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](../style/index.md)).

- <span id="progressbar-finish-using-style"></span>`fn finish_using_style(&self)`

  Finishes the progress bar using the behavior stored in the [`ProgressStyle`](../style/index.md)

  

  See `ProgressBar::with_finish()`.

- <span id="progressbar-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

  Sets a different draw target for the progress bar

  

  This can be used to draw the progress bar to stderr (this is the default):

  

  ```rust,no_run

  use indicatif::{ProgressBar, ProgressDrawTarget};

  let pb = ProgressBar::new(100);

  pb.set_draw_target(ProgressDrawTarget::stderr());

  ```

  

  **Note:** Calling this method on a [`ProgressBar`](#progressbar) linked with a [`MultiProgress`](../multi/index.md) (after

  running `MultiProgress::add()`) will unlink this progress bar. If you don't want this

  behavior, call `MultiProgress::set_draw_target()` instead.

  

  Use `ProgressBar::with_draw_target()` to set the draw target during creation.

  

  

- <span id="progressbar-force-draw"></span>`fn force_draw(&self)`

  Force a redraw of the progress bar to be in sync with its state

  

  For performance reasons the progress bar is not redrawn on each state update.

  This is normally not an issue, since new updates will eventually trigger rendering.

  

  For slow running tasks it is recommended to rely on `ProgressBar::enable_steady_tick()`

  to ensure continued rendering of the progress bar.

- <span id="progressbar-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

  Hide the progress bar temporarily, execute `f`, then redraw the progress bar

  

  Useful for external code that writes to the standard output.

  

  If the progress bar was added to a [`MultiProgress`](../multi/index.md), it will suspend the entire [`MultiProgress`](../multi/index.md).

  

  **Note:** The internal lock is held while `f` is executed. Other threads trying to print

  anything on the progress bar will be blocked until `f` finishes.

  Therefore, it is recommended to avoid long-running operations in `f`.

  

  ```rust,no_run

  use indicatif::ProgressBar;

  let mut pb = ProgressBar::new(3);

  pb.suspend(|| {

      println!("Log message");

  })

  ```

- <span id="progressbar-wrap-iter"></span>`fn wrap_iter<It: Iterator>(&self, it: It) -> ProgressBarIter<It>` — [`ProgressBarIter`](../iter/index.md#progressbariter)

  Wraps an [`Iterator`](../../cargo_docs_md/index.md) with the progress bar

  

  ```rust,no_run

  use indicatif::ProgressBar;

  let v = vec![1, 2, 3];

  let pb = ProgressBar::new(3);

  for item in pb.wrap_iter(v.iter()) {

      // ...

  }

  ```

- <span id="progressbar-wrap-read"></span>`fn wrap_read<R: io::Read>(&self, read: R) -> ProgressBarIter<R>` — [`ProgressBarIter`](../iter/index.md#progressbariter)

  Wraps an [`io::Read`](../../fs_err/index.md) with the progress bar

  

  ```rust,no_run

  use std::fs::File;

  use std::io;

  use indicatif::ProgressBar;

  fn test () -> io::Result<()> {

  let source = File::open("work.txt")?;

  let mut target = File::create("done.txt")?;

  let pb = ProgressBar::new(source.metadata()?.len());

  io::copy(&mut pb.wrap_read(source), &mut target);

  Ok(())

  }

  ```

- <span id="progressbar-wrap-write"></span>`fn wrap_write<W: io::Write>(&self, write: W) -> ProgressBarIter<W>` — [`ProgressBarIter`](../iter/index.md#progressbariter)

  Wraps an [`io::Write`](../../fs_err/index.md) with the progress bar

  

  ```rust,no_run

  use std::fs::File;

  use std::io;

  use indicatif::ProgressBar;

  fn test () -> io::Result<()> {

  let mut source = File::open("work.txt")?;

  let target = File::create("done.txt")?;

  let pb = ProgressBar::new(source.metadata()?.len());

  io::copy(&mut source, &mut pb.wrap_write(target));

  Ok(())

  }

  ```

- <span id="progressbar-position"></span>`fn position(&self) -> u64`

  Returns the current position

- <span id="progressbar-length"></span>`fn length(&self) -> Option<u64>`

  Returns the current length

- <span id="progressbar-eta"></span>`fn eta(&self) -> Duration`

  Returns the current ETA

- <span id="progressbar-per-sec"></span>`fn per_sec(&self) -> f64`

  Returns the current rate of progress

- <span id="progressbar-duration"></span>`fn duration(&self) -> Duration`

  Returns the current expected duration

- <span id="progressbar-elapsed"></span>`fn elapsed(&self) -> Duration`

  Returns the current elapsed time

- <span id="progressbar-index"></span>`fn index(&self) -> Option<usize>`

  Index in the `MultiState`

- <span id="progressbar-message"></span>`fn message(&self) -> String`

  Current message

- <span id="progressbar-prefix"></span>`fn prefix(&self) -> String`

  Current prefix

- <span id="progressbar-state"></span>`fn state(&self) -> MutexGuard<'_, BarState>` — [`BarState`](../state/index.md#barstate)

#### Trait Implementations

##### `impl Any for ProgressBar`

- <span id="progressbar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressBar`

- <span id="progressbar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressBar`

- <span id="progressbar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProgressBar`

- <span id="progressbar-clone"></span>`fn clone(&self) -> ProgressBar` — [`ProgressBar`](#progressbar)

##### `impl CloneToUninit for ProgressBar`

- <span id="progressbar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ProgressBar`

- <span id="progressbar-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ProgressBar`

- <span id="progressbar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressBar`

- <span id="progressbar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ProgressBar`

- <span id="progressbar-toowned-type-owned"></span>`type Owned = T`

- <span id="progressbar-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="progressbar-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ProgressBar`

- <span id="progressbar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressbar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressBar`

- <span id="progressbar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressbar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WeakProgressBar`

```rust
struct WeakProgressBar {
    state: std::sync::Weak<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Weak<crate::state::AtomicPosition>,
    ticker: std::sync::Weak<std::sync::Mutex<Option<Ticker>>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:651-655`](../../../.source_1765521767/indicatif-0.18.3/src/progress_bar.rs#L651-L655)*

A weak reference to a [`ProgressBar`](#progressbar).

Useful for creating custom steady tick implementations

#### Implementations

- <span id="weakprogressbar-new"></span>`fn new() -> Self`

  Create a new [`WeakProgressBar`](#weakprogressbar) that returns `None` when `upgrade()` is called.

- <span id="weakprogressbar-upgrade"></span>`fn upgrade(&self) -> Option<ProgressBar>` — [`ProgressBar`](#progressbar)

  Attempts to upgrade the Weak pointer to a [`ProgressBar`](#progressbar), delaying dropping of the inner

  value if successful. Returns [`None`](#none) if the inner value has since been dropped.

#### Trait Implementations

##### `impl Any for WeakProgressBar`

- <span id="weakprogressbar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WeakProgressBar`

- <span id="weakprogressbar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WeakProgressBar`

- <span id="weakprogressbar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WeakProgressBar`

- <span id="weakprogressbar-clone"></span>`fn clone(&self) -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

##### `impl CloneToUninit for WeakProgressBar`

- <span id="weakprogressbar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for WeakProgressBar`

- <span id="weakprogressbar-default"></span>`fn default() -> WeakProgressBar` — [`WeakProgressBar`](#weakprogressbar)

##### `impl<T> From for WeakProgressBar`

- <span id="weakprogressbar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WeakProgressBar`

- <span id="weakprogressbar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WeakProgressBar`

- <span id="weakprogressbar-toowned-type-owned"></span>`type Owned = T`

- <span id="weakprogressbar-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="weakprogressbar-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WeakProgressBar`

- <span id="weakprogressbar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="weakprogressbar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WeakProgressBar`

- <span id="weakprogressbar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="weakprogressbar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ticker`

```rust
struct Ticker {
    stopping: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    join_handle: Option<thread::JoinHandle<()>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:677-680`](../../../.source_1765521767/indicatif-0.18.3/src/progress_bar.rs#L677-L680)*

#### Implementations

- <span id="ticker-new"></span>`fn new(interval: Duration, bar_state: &Arc<Mutex<BarState>>) -> Self` — [`BarState`](../state/index.md#barstate)

- <span id="ticker-stop"></span>`fn stop(&self)`

#### Trait Implementations

##### `impl Any for Ticker`

- <span id="ticker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ticker`

- <span id="ticker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ticker`

- <span id="ticker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for Ticker`

- <span id="ticker-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Ticker`

- <span id="ticker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ticker`

- <span id="ticker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Ticker`

- <span id="ticker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ticker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ticker`

- <span id="ticker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ticker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TickerControl`

```rust
struct TickerControl {
    stopping: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    state: std::sync::Weak<std::sync::Mutex<crate::state::BarState>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:719-722`](../../../.source_1765521767/indicatif-0.18.3/src/progress_bar.rs#L719-L722)*

#### Implementations

- <span id="tickercontrol-run"></span>`fn run(&self, interval: Duration)`

#### Trait Implementations

##### `impl Any for TickerControl`

- <span id="tickercontrol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TickerControl`

- <span id="tickercontrol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TickerControl`

- <span id="tickercontrol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TickerControl`

- <span id="tickercontrol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TickerControl`

- <span id="tickercontrol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TickerControl`

- <span id="tickercontrol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tickercontrol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TickerControl`

- <span id="tickercontrol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tickercontrol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

