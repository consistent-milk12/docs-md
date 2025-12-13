# Crate `indicatif`

indicatif is a library for Rust that helps you build command line
interfaces that report progress to users.  It comes with various
tools and utilities for formatting anything that indicates progress.

Platform support:

* Linux
* macOS
* Windows (colors require Windows 10)

Best paired with other libraries in the family:

* [console](https://docs.rs/console)
* [dialoguer](https://docs.rs/dialoguer)

# Crate Contents

* **Progress bars**
  * [`ProgressBar`](#progressbar) for bars and spinners
  * [`MultiProgress`](#multiprogress) for multiple bars
* **Data Formatting**
  * [`HumanBytes`](#humanbytes) for formatting bytes
  * [`DecimalBytes`](#decimalbytes) for formatting bytes using SI prefixes
  * [`BinaryBytes`](#binarybytes) for formatting bytes using ISO/IEC prefixes
  * [`HumanDuration`](#humanduration) for formatting durations
  * [`HumanCount`](#humancount) for formatting large counts
  * [`HumanFloatCount`](#humanfloatcount) for formatting large float counts

# Progress Bars and Spinners

indicatif comes with a [`ProgressBar`](progress_bar/index.md) type that supports both bounded
progress bar uses as well as unbounded "spinner" type progress reports.
Progress bars are [`Sync`](../miniz_oxide/index.md) and `Send` objects which means that they are
internally locked and can be passed from thread to thread.

Additionally a [`MultiProgress`](multi/index.md) utility is provided that can manage
rendering multiple progress bars at once (eg: from multiple threads).

To whet your appetite, this is what this can look like:

<img src="https://github.com/console-rs/indicatif/raw/main/screenshots/yarn.gif?raw=true" width="60%">

Progress bars are manually advanced and by default draw to stderr.
When you are done, the progress bar can be finished either visibly
(eg: the progress bar stays on the screen) or cleared (the progress
bar will be removed).

```rust
use indicatif::ProgressBar;

let bar = ProgressBar::new(1000);
for _ in 0..1000 {
    bar.inc(1);
    // ...
}
bar.finish();
```

Spinners can be manually advanced with [`tick`](ProgressBar::tick), or you can set them up
to spin automatically with [`enable_steady_tick`](ProgressBar::enable_steady_tick):

```rust
use std::time::Duration;
use indicatif::ProgressBar;

let bar = ProgressBar::new_spinner();
bar.enable_steady_tick(Duration::from_millis(100));
// ... do some work
bar.finish();
```

General progress bar behaviors:

* if a non terminal is detected the progress bar will be completely
  hidden.  This makes piping programs to logfiles make sense out of
  the box.
* a progress bar only starts drawing when [`set_message`](ProgressBar::set_message),
  [`inc`](ProgressBar::inc), [`set_position`](ProgressBar::set_position)
  or [`tick`](ProgressBar::tick) are called. In some situations you
  might have to call [`tick`](ProgressBar::tick) once to draw it.
* progress bars should be explicitly finished to reset the rendering
  for others.  Either by also clearing them or by replacing them with
  a new message / retaining the current message.
* the default template renders neither message nor prefix.

# Iterators

Similar to [tqdm](https://github.com/tqdm/tqdm), progress bars can be
associated with an iterator. For example:

```rust
use indicatif::ProgressIterator;

for _ in (0..1000).progress() {
    // ...
}
```

See the [`ProgressIterator`](#progressiterator) trait for more
methods to configure the number of elements in the iterator or change
the progress bar style. Indicatif also has optional support for parallel
iterators with [Rayon](https://github.com/rayon-rs/rayon). In your
`Cargo.toml`, use the "rayon" feature:

```toml
[dependencies]
indicatif = {version = "*", features = ["rayon"]}
```

And then use it like this:

```rust,ignore
extern crate rayon;
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

let v: Vec<_> = (0..100000).collect();
let v2: Vec<_> = v.par_iter().progress_count(v.len() as u64).map(|i| i + 1).collect();
assert_eq!(v2[0], 1);
```

Or if you'd like to customize the progress bar:

```rust,ignore
extern crate rayon;
use indicatif::{ProgressBar, ParallelProgressIterator, ProgressStyle};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

// Alternatively, use `ProgressBar::new().with_style()`
let style = ProgressStyle::default_bar();
let v: Vec<_> = (0..100000).collect();
let v2: Vec<_> = v.par_iter().progress_with_style(style).map(|i| i + 1).collect();
assert_eq!(v2[0], 1);
```

# Templates

Progress bars can be styled with simple format strings similar to the
ones in Rust itself.  The format for a placeholder is `{key:options}`
where the `options` part is optional.  If provided the format is this:

```text
<^>             for an optional alignment specification (left, center and right respectively)
WIDTH           an optional width as positive integer
!               an optional exclamation mark to enable truncation
.STYLE          an optional dot separated style string
/STYLE          an optional dot separated alternative style string
```

For the style component see [`Style::from_dotted_str`](https://docs.rs/console/0.7.5/console/struct.Style.html#method.from_dotted_str)
for more information. Indicatif uses the `console` base crate for all
colorization and formatting options.

Some examples for templates:

```text
[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}
```

This sets a progress bar that is 40 characters wide and has cyan
as primary style color and blue as alternative style color.
Alternative styles are currently only used for progress bars.

Example configuration:

```rust
use indicatif::{ProgressBar, ProgressStyle};
let bar = ProgressBar::new(0);
bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));
```

The following keys exist:

* `bar`: renders a progress bar. By default 20 characters wide.  The
  style string is used to color the elapsed part, the alternative
  style is used for the bar that is yet to render.
* `wide_bar`: like `bar` but always fills the remaining space. It should not be used with `wide_msg`.
* `spinner`: renders the spinner (current tick string). Note that spinners do not automatically tick by default. You either
  need to call `enable_steady_tick` or manually call `tick`.
* `prefix`: renders the prefix set on the progress bar.
* `msg`: renders the currently set message on the progress bar.
* `wide_msg`: like `msg` but always fills the remaining space and truncates. It should not be used
  with `wide_bar`.
* `pos`: renders the current position of the bar as integer
* `human_pos`: renders the current position of the bar as an integer, with commas as the
  thousands separator.
* `len`: renders the amount of work to be done as an integer
* `human_len`: renders the total length of the bar as an integer, with commas as the thousands
  separator.
* `percent`: renders the current position of the bar as a percentage of the total length (as an integer).
* `percent_precise`: renders the current position of the bar as a percentage of the total length (with 3 fraction digits).
* `bytes`: renders the current position of the bar as bytes (alias of `binary_bytes`).
* `total_bytes`: renders the total length of the bar as bytes (alias of `binary_total_bytes`).
* `decimal_bytes`: renders the current position of the bar as bytes using
  power-of-10 units, i.e. `MB`, `kB`, etc.
* `decimal_total_bytes`: renders the total length of the bar as bytes using
  power-of-10 units, i.e. `MB`, `kB`, etc.
* `binary_bytes`: renders the current position of the bar as bytes using
  power-of-two units, i.e. `MiB`, `KiB`, etc.
* `binary_total_bytes`: renders the total length of the bar as bytes using
  power-of-two units, i.e. `MiB`, `KiB`, etc.
* `elapsed_precise`: renders the elapsed time as `HH:MM:SS`.
* `elapsed`: renders the elapsed time as `42s`, `1m` etc.
* `per_sec`: renders the speed in steps per second.
* `bytes_per_sec`: renders the speed in bytes per second (alias of `binary_bytes_per_sec`).
* `decimal_bytes_per_sec`: renders the speed in bytes per second using
  power-of-10 units, i.e. `MB`, `kB`, etc.
* `binary_bytes_per_sec`: renders the speed in bytes per second using
  power-of-two units, i.e. `MiB`, `KiB`, etc.
* `eta_precise`: the remaining time (like `elapsed_precise`).
* `eta`: the remaining time (like `elapsed`).
* `duration_precise`: the extrapolated total duration (like `elapsed_precise`).
* `duration`: the extrapolated total duration time (like `elapsed`).

If the list above does not contain the value you need, consider creating a custom
`ProgressTracker` implementation.

The design of the progress bar can be altered with the integrated
template functionality.  The template can be set by changing a
[`ProgressStyle`](style/index.md) and attaching it to the progress bar.

# Human Readable Formatting

There are some formatting wrappers for showing elapsed time and
file sizes for human users:

```rust
use std::time::Duration;
use indicatif::{HumanBytes, HumanCount, HumanDuration, HumanFloatCount};

assert_eq!("3.00 MiB", HumanBytes(3*1024*1024).to_string());
assert_eq!("8 seconds", HumanDuration(Duration::from_secs(8)).to_string());
assert_eq!("33,857,009", HumanCount(33857009).to_string());
assert_eq!("33,857,009.1235", HumanFloatCount(33857009.123456).to_string());
```

# Feature Flags

* `rayon`: adds rayon support
* `improved_unicode`: adds improved unicode support (graphemes, better width calculation)

## Contents

- [Modules](#modules)
  - [`draw_target`](#draw-target)
  - [`format`](#format)
  - [`iter`](#iter)
  - [`multi`](#multi)
  - [`progress_bar`](#progress-bar)
  - [`state`](#state)
  - [`style`](#style)
  - [`term_like`](#term-like)
- [Structs](#structs)
  - [`ProgressDrawTarget`](#progressdrawtarget)
  - [`BinaryBytes`](#binarybytes)
  - [`DecimalBytes`](#decimalbytes)
  - [`FormattedDuration`](#formattedduration)
  - [`HumanBytes`](#humanbytes)
  - [`HumanCount`](#humancount)
  - [`HumanDuration`](#humanduration)
  - [`HumanFloatCount`](#humanfloatcount)
  - [`ProgressBarIter`](#progressbariter)
  - [`MultiProgress`](#multiprogress)
  - [`ProgressBar`](#progressbar)
  - [`WeakProgressBar`](#weakprogressbar)
  - [`ProgressState`](#progressstate)
  - [`ProgressStyle`](#progressstyle)
- [Enums](#enums)
  - [`MultiProgressAlignment`](#multiprogressalignment)
  - [`ProgressFinish`](#progressfinish)
- [Traits](#traits)
  - [`ProgressIterator`](#progressiterator)
  - [`TermLike`](#termlike)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`draw_target`](#draw-target) | mod |  |
| [`format`](#format) | mod |  |
| [`iter`](#iter) | mod |  |
| [`multi`](#multi) | mod |  |
| [`progress_bar`](#progress-bar) | mod |  |
| [`state`](#state) | mod |  |
| [`style`](#style) | mod |  |
| [`term_like`](#term-like) | mod |  |
| [`ProgressDrawTarget`](#progressdrawtarget) | struct |  |
| [`BinaryBytes`](#binarybytes) | struct |  |
| [`DecimalBytes`](#decimalbytes) | struct |  |
| [`FormattedDuration`](#formattedduration) | struct |  |
| [`HumanBytes`](#humanbytes) | struct |  |
| [`HumanCount`](#humancount) | struct |  |
| [`HumanDuration`](#humanduration) | struct |  |
| [`HumanFloatCount`](#humanfloatcount) | struct |  |
| [`ProgressBarIter`](#progressbariter) | struct |  |
| [`MultiProgress`](#multiprogress) | struct |  |
| [`ProgressBar`](#progressbar) | struct |  |
| [`WeakProgressBar`](#weakprogressbar) | struct |  |
| [`ProgressState`](#progressstate) | struct |  |
| [`ProgressStyle`](#progressstyle) | struct |  |
| [`MultiProgressAlignment`](#multiprogressalignment) | enum |  |
| [`ProgressFinish`](#progressfinish) | enum |  |
| [`ProgressIterator`](#progressiterator) | trait |  |
| [`TermLike`](#termlike) | trait |  |

## Modules

- [`draw_target`](draw_target/index.md)
- [`format`](format/index.md)
- [`iter`](iter/index.md)
- [`multi`](multi/index.md)
- [`progress_bar`](progress_bar/index.md)
- [`state`](state/index.md)
- [`style`](style/index.md)
- [`term_like`](term_like/index.md)

## Structs

### `ProgressDrawTarget`

```rust
struct ProgressDrawTarget {
    kind: TargetKind,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:25-27`](../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L25-L27)*

Target for draw operations

This tells a [`ProgressBar`](crate::ProgressBar) or a
[`MultiProgress`](crate::MultiProgress) object where to paint to.
The draw target is a stateful wrapper over a drawing destination and
internally optimizes how often the state is painted to the output
device.

#### Implementations

- <span id="progressdrawtarget-stdout"></span>`fn stdout() -> Self`

  Draw to a buffered stdout terminal at a max of 20 times a second.

  

  For more information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-stderr"></span>`fn stderr() -> Self`

  Draw to a buffered stderr terminal at a max of 20 times a second.

  

  This is the default draw target for progress bars.  For more

  information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-stdout-with-hz"></span>`fn stdout_with_hz(refresh_rate: u8) -> Self`

  Draw to a buffered stdout terminal at a max of `refresh_rate` times a second.

  

  For more information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-stderr-with-hz"></span>`fn stderr_with_hz(refresh_rate: u8) -> Self`

  Draw to a buffered stderr terminal at a max of `refresh_rate` times a second.

  

  For more information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-new-remote"></span>`fn new_remote(state: Arc<RwLock<MultiState>>, idx: usize) -> Self` — [`MultiState`](multi/index.md#multistate)

- <span id="progressdrawtarget-term"></span>`fn term(term: Term, refresh_rate: u8) -> Self`

  Draw to a terminal, with a specific refresh rate.

  

  Progress bars are by default drawn to terminals however if the

  terminal is not user attended the entire progress bar will be

  hidden.  This is done so that piping to a file will not produce

  useless escape codes in that file.

  

  Will panic if `refresh_rate` is `0`.

- <span id="progressdrawtarget-term-like"></span>`fn term_like(term_like: Box<dyn TermLike>) -> Self` — [`TermLike`](term_like/index.md#termlike)

  Draw to a boxed object that implements the [`TermLike`](term_like/index.md) trait.

- <span id="progressdrawtarget-term-like-with-hz"></span>`fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self` — [`TermLike`](term_like/index.md#termlike)

  Draw to a boxed object that implements the [`TermLike`](term_like/index.md) trait,

  with a specific refresh rate.

- <span id="progressdrawtarget-hidden"></span>`fn hidden() -> Self`

  A hidden draw target.

  

  This forces a progress bar to be not rendered at all.

- <span id="progressdrawtarget-is-hidden"></span>`fn is_hidden(&self) -> bool`

  Returns true if the draw target is hidden.

  

  This is internally used in progress bars to figure out if overhead

  from drawing can be prevented.

- <span id="progressdrawtarget-is-stderr"></span>`fn is_stderr(&self) -> bool`

  This is used in progress bars to determine whether to use stdout or stderr

  for detecting color support.

- <span id="progressdrawtarget-width"></span>`fn width(&self) -> Option<u16>`

  Returns the current width of the draw target.

- <span id="progressdrawtarget-mark-zombie"></span>`fn mark_zombie(&self)`

  Notifies the backing `MultiProgress` (if applicable) that the associated progress bar should

  be marked a zombie.

- <span id="progressdrawtarget-set-move-cursor"></span>`fn set_move_cursor(&mut self, move_cursor: bool)`

  Set whether or not to just move cursor instead of clearing lines

- <span id="progressdrawtarget-drawable"></span>`fn drawable(&mut self, force_draw: bool, now: Instant) -> Option<Drawable<'_>>` — [`Drawable`](draw_target/index.md#drawable)

  Apply the given draw state (draws it).

- <span id="progressdrawtarget-disconnect"></span>`fn disconnect(&self, now: Instant)`

  Properly disconnects from the draw target

- <span id="progressdrawtarget-remote"></span>`fn remote(&self) -> Option<(&Arc<RwLock<MultiState>>, usize)>` — [`MultiState`](multi/index.md#multistate)

- <span id="progressdrawtarget-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](draw_target/index.md#lineadjust)

#### Trait Implementations

##### `impl Any for ProgressDrawTarget`

- <span id="progressdrawtarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressDrawTarget`

- <span id="progressdrawtarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressDrawTarget`

- <span id="progressdrawtarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ProgressDrawTarget`

- <span id="progressdrawtarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ProgressDrawTarget`

- <span id="progressdrawtarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressDrawTarget`

- <span id="progressdrawtarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ProgressDrawTarget`

- <span id="progressdrawtarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressdrawtarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressDrawTarget`

- <span id="progressdrawtarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressdrawtarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:64`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L64)*

Formats bytes for human readability using ISO/IEC prefixes

# Examples
```rust
use indicatif::BinaryBytes;
assert_eq!("15 B",     format!("{}", BinaryBytes(15)));
assert_eq!("1.46 KiB", format!("{}", BinaryBytes(1_500)));
assert_eq!("1.43 MiB", format!("{}", BinaryBytes(1_500_000)));
assert_eq!("1.40 GiB", format!("{}", BinaryBytes(1_500_000_000)));
assert_eq!("1.36 TiB", format!("{}", BinaryBytes(1_500_000_000_000)));
assert_eq!("1.33 PiB", format!("{}", BinaryBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl Any for BinaryBytes`

- <span id="binarybytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BinaryBytes`

- <span id="binarybytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BinaryBytes`

- <span id="binarybytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for BinaryBytes`

- <span id="binarybytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BinaryBytes`

- <span id="binarybytes-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BinaryBytes`

- <span id="binarybytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BinaryBytes`

- <span id="binarybytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for BinaryBytes`

- <span id="binarybytes-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BinaryBytes`

- <span id="binarybytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="binarybytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BinaryBytes`

- <span id="binarybytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="binarybytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:49`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L49)*

Formats bytes for human readability using SI prefixes

# Examples
```rust
use indicatif::DecimalBytes;
assert_eq!("15 B",    format!("{}", DecimalBytes(15)));
assert_eq!("1.50 kB", format!("{}", DecimalBytes(1_500)));
assert_eq!("1.50 MB", format!("{}", DecimalBytes(1_500_000)));
assert_eq!("1.50 GB", format!("{}", DecimalBytes(1_500_000_000)));
assert_eq!("1.50 TB", format!("{}", DecimalBytes(1_500_000_000_000)));
assert_eq!("1.50 PB", format!("{}", DecimalBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl Any for DecimalBytes`

- <span id="decimalbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DecimalBytes`

- <span id="decimalbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DecimalBytes`

- <span id="decimalbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DecimalBytes`

- <span id="decimalbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecimalBytes`

- <span id="decimalbytes-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DecimalBytes`

- <span id="decimalbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DecimalBytes`

- <span id="decimalbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for DecimalBytes`

- <span id="decimalbytes-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for DecimalBytes`

- <span id="decimalbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decimalbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DecimalBytes`

- <span id="decimalbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decimalbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FormattedDuration`

```rust
struct FormattedDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:15`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L15)*

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl Any for FormattedDuration`

- <span id="formattedduration-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FormattedDuration`

- <span id="formattedduration-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FormattedDuration`

- <span id="formattedduration-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FormattedDuration`

- <span id="formattedduration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FormattedDuration`

- <span id="formattedduration-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FormattedDuration`

- <span id="formattedduration-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FormattedDuration`

- <span id="formattedduration-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for FormattedDuration`

- <span id="formattedduration-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FormattedDuration`

- <span id="formattedduration-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="formattedduration-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FormattedDuration`

- <span id="formattedduration-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="formattedduration-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:34`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L34)*

Formats bytes for human readability

# Examples
```rust
use indicatif::HumanBytes;
assert_eq!("15 B",     format!("{}", HumanBytes(15)));
assert_eq!("1.46 KiB", format!("{}", HumanBytes(1_500)));
assert_eq!("1.43 MiB", format!("{}", HumanBytes(1_500_000)));
assert_eq!("1.40 GiB", format!("{}", HumanBytes(1_500_000_000)));
assert_eq!("1.36 TiB", format!("{}", HumanBytes(1_500_000_000_000)));
assert_eq!("1.33 PiB", format!("{}", HumanBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl Any for HumanBytes`

- <span id="humanbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanBytes`

- <span id="humanbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanBytes`

- <span id="humanbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanBytes`

- <span id="humanbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanBytes`

- <span id="humanbytes-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanBytes`

- <span id="humanbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanBytes`

- <span id="humanbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanBytes`

- <span id="humanbytes-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanBytes`

- <span id="humanbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humanbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanBytes`

- <span id="humanbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humanbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanCount`

```rust
struct HumanCount(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:68`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L68)*

Formats counts for human readability using commas

#### Trait Implementations

##### `impl Any for HumanCount`

- <span id="humancount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanCount`

- <span id="humancount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanCount`

- <span id="humancount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanCount`

- <span id="humancount-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanCount`

- <span id="humancount-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanCount`

- <span id="humancount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanCount`

- <span id="humancount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanCount`

- <span id="humancount-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanCount`

- <span id="humancount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humancount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanCount`

- <span id="humancount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humancount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:19`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L19)*

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl Any for HumanDuration`

- <span id="humanduration-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanDuration`

- <span id="humanduration-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanDuration`

- <span id="humanduration-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanDuration`

- <span id="humanduration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanDuration`

- <span id="humanduration-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanDuration`

- <span id="humanduration-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanDuration`

- <span id="humanduration-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanDuration`

- <span id="humanduration-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanDuration`

- <span id="humanduration-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humanduration-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanDuration`

- <span id="humanduration-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humanduration-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:72`](../../.source_1765633015/indicatif-0.18.3/src/format.rs#L72)*

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl Any for HumanFloatCount`

- <span id="humanfloatcount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanFloatCount`

- <span id="humanfloatcount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanFloatCount`

- <span id="humanfloatcount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanFloatCount`

- <span id="humanfloatcount-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanFloatCount`

- <span id="humanfloatcount-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanFloatCount`

- <span id="humanfloatcount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanFloatCount`

- <span id="humanfloatcount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanFloatCount`

- <span id="humanfloatcount-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanFloatCount`

- <span id="humanfloatcount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humanfloatcount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanFloatCount`

- <span id="humanfloatcount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humanfloatcount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ProgressBarIter<T>`

```rust
struct ProgressBarIter<T> {
    it: T,
    pub progress: crate::progress_bar::ProgressBar,
}
```

*Defined in [`indicatif-0.18.3/src/iter.rs:62-65`](../../.source_1765633015/indicatif-0.18.3/src/iter.rs#L62-L65)*

Wraps an iterator to display its progress.

#### Implementations

- <span id="progressbariter-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](style/index.md#progressstyle)

  Builder-like function for setting underlying progress bar's style.

  

  See `ProgressBar::with_style()`.

- <span id="progressbariter-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

  Builder-like function for setting underlying progress bar's prefix.

  

  See `ProgressBar::with_prefix()`.

- <span id="progressbariter-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

  Builder-like function for setting underlying progress bar's message.

  

  See `ProgressBar::with_message()`.

- <span id="progressbariter-with-position"></span>`fn with_position(self, position: u64) -> Self`

  Builder-like function for setting underlying progress bar's position.

  

  See `ProgressBar::with_position()`.

- <span id="progressbariter-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

  Builder-like function for setting underlying progress bar's elapsed time.

  

  See `ProgressBar::with_elapsed()`.

- <span id="progressbariter-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](state/index.md#progressfinish)

  Builder-like function for setting underlying progress bar's finish behavior.

  

  See `ProgressBar::with_finish()`.

#### Trait Implementations

##### `impl<T> Any for ProgressBarIter<T>`

- <span id="progressbariter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressBarIter<T>`

- <span id="progressbariter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressBarIter<T>`

- <span id="progressbariter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: io::BufRead> BufRead for ProgressBarIter<R>`

- <span id="progressbariter-bufread-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="progressbariter-bufread-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<T: fmt::Debug> Debug for ProgressBarIter<T>`

- <span id="progressbariter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: DoubleEndedIterator> DoubleEndedIterator for ProgressBarIter<T>`

- <span id="progressbariter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: ExactSizeIterator> ExactSizeIterator for ProgressBarIter<T>`

- <span id="progressbariter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ProgressBarIter<T>`

- <span id="progressbariter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: FusedIterator> FusedIterator for ProgressBarIter<T>`

##### `impl<T, U> Into for ProgressBarIter<T>`

- <span id="progressbariter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ProgressBarIter<T>`

- <span id="progressbariter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="progressbariter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="progressbariter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Iterator<Item = S>> Iterator for ProgressBarIter<T>`

- <span id="progressbariter-iterator-type-item"></span>`type Item = S`

- <span id="progressbariter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ProgressIterator for ProgressBarIter<T>`

- <span id="progressbariter-progressiterator-progress-with"></span>`fn progress_with(self, progress: ProgressBar) -> ProgressBarIter<T>` — [`ProgressBar`](progress_bar/index.md#progressbar), [`ProgressBarIter`](iter/index.md#progressbariter)

##### `impl<R: io::Read> Read for ProgressBarIter<R>`

- <span id="progressbariter-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

- <span id="progressbariter-read-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- <span id="progressbariter-read-read-to-string"></span>`fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>`

- <span id="progressbariter-read-read-exact"></span>`fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>`

##### `impl<S: io::Seek> Seek for ProgressBarIter<S>`

- <span id="progressbariter-seek"></span>`fn seek(&mut self, f: io::SeekFrom) -> io::Result<u64>`

- <span id="progressbariter-seek-stream-position"></span>`fn stream_position(&mut self) -> io::Result<u64>`

##### `impl<T, U> TryFrom for ProgressBarIter<T>`

- <span id="progressbariter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressbariter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ProgressBarIter<T>`

- <span id="progressbariter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressbariter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<W: io::Write> Write for ProgressBarIter<W>`

- <span id="progressbariter-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="progressbariter-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- <span id="progressbariter-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `MultiProgress`

```rust
struct MultiProgress {
    state: std::sync::Arc<std::sync::RwLock<MultiState>>,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:18-20`](../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L18-L20)*

Manages multiple progress bars from different threads

#### Implementations

- <span id="multiprogress-new"></span>`fn new() -> Self`

  Creates a new multi progress object.

  

  Progress bars added to this object by default draw directly to stderr, and refresh

  a maximum of 15 times a second. To change the refresh rate [`set`](../hashbrown/set/index.md) the [draw target] to

  one with a different refresh rate.

  

- <span id="multiprogress-with-draw-target"></span>`fn with_draw_target(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

  Creates a new multi progress object with the given draw target.

- <span id="multiprogress-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

  Sets a different draw target for the multiprogress bar.

  

  Use `MultiProgress::with_draw_target` to set the draw target during creation.

- <span id="multiprogress-set-move-cursor"></span>`fn set_move_cursor(&self, move_cursor: bool)`

  Set whether we should try to move the cursor when possible instead of clearing lines.

  

  This can reduce flickering, but do not enable it if you intend to change the number of

  progress bars.

- <span id="multiprogress-set-alignment"></span>`fn set_alignment(&self, alignment: MultiProgressAlignment)` — [`MultiProgressAlignment`](multi/index.md#multiprogressalignment)

  Set alignment flag

- <span id="multiprogress-add"></span>`fn add(&self, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Adds a progress bar.

  

  The progress bar added will have the draw target changed to a

  remote draw target that is intercepted by the multi progress

  object overriding custom [`ProgressDrawTarget`](draw_target/index.md) settings.

  

  The progress bar will be positioned below all other bars currently

  in the [`MultiProgress`](multi/index.md).

  

  Adding a progress bar that is already a member of the [`MultiProgress`](multi/index.md)

  will have no effect.

- <span id="multiprogress-insert"></span>`fn insert(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Inserts a progress bar.

  

  The progress bar inserted at position `index` will have the draw

  target changed to a remote draw target that is intercepted by the

  multi progress object overriding custom [`ProgressDrawTarget`](draw_target/index.md) settings.

  

  If `index >= MultiProgressState::objects.len()`, the progress bar

  is added to the end of the list.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](multi/index.md)

  will have no effect.

- <span id="multiprogress-insert-from-back"></span>`fn insert_from_back(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Inserts a progress bar from the back.

  

  The progress bar inserted at position `MultiProgressState::objects.len() - index`

  will have the draw target changed to a remote draw target that is

  intercepted by the multi progress object overriding custom

  [`ProgressDrawTarget`](draw_target/index.md) settings.

  

  If `index >= MultiProgressState::objects.len()`, the progress bar

  is added to the start of the list.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](multi/index.md)

  will have no effect.

- <span id="multiprogress-insert-before"></span>`fn insert_before(&self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Inserts a progress bar before an existing one.

  

  The progress bar added will have the draw target changed to a

  remote draw target that is intercepted by the multi progress

  object overriding custom [`ProgressDrawTarget`](draw_target/index.md) settings.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](multi/index.md)

  will have no effect.

- <span id="multiprogress-insert-after"></span>`fn insert_after(&self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Inserts a progress bar after an existing one.

  

  The progress bar added will have the draw target changed to a

  remote draw target that is intercepted by the multi progress

  object overriding custom [`ProgressDrawTarget`](draw_target/index.md) settings.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](multi/index.md)

  will have no effect.

- <span id="multiprogress-remove"></span>`fn remove(&self, pb: &ProgressBar)` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Removes a progress bar.

  

  The progress bar is removed only if it was previously inserted or added

  by the methods `MultiProgress::insert` or `MultiProgress::add`.

  If the passed progress bar does not satisfy the condition above,

  the `remove` method does nothing.

- <span id="multiprogress-internalize"></span>`fn internalize(&self, location: InsertLocation, pb: ProgressBar) -> ProgressBar` — [`InsertLocation`](multi/index.md#insertlocation), [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-println"></span>`fn println<I: AsRef<str>>(&self, msg: I) -> io::Result<()>`

  Print a log line above all progress bars in the [`MultiProgress`](multi/index.md)

  

  If the draw target is hidden (e.g. when standard output is not a terminal), `println()`

  will not do anything.

- <span id="multiprogress-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

  Hide all progress bars temporarily, execute `f`, then redraw the [`MultiProgress`](multi/index.md)

  

  Executes 'f' even if the draw target is hidden.

  

  Useful for external code that writes to the standard output.

  

  **Note:** The internal lock is held while `f` is executed. Other threads trying to print

  anything on the progress bar will be blocked until `f` finishes.

  Therefore, it is recommended to avoid long-running operations in `f`.

- <span id="multiprogress-clear"></span>`fn clear(&self) -> io::Result<()>`

- <span id="multiprogress-is-hidden"></span>`fn is_hidden(&self) -> bool`

#### Trait Implementations

##### `impl Any for MultiProgress`

- <span id="multiprogress-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProgress`

- <span id="multiprogress-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProgress`

- <span id="multiprogress-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MultiProgress`

- <span id="multiprogress-clone"></span>`fn clone(&self) -> MultiProgress` — [`MultiProgress`](multi/index.md#multiprogress)

##### `impl CloneToUninit for MultiProgress`

- <span id="multiprogress-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MultiProgress`

- <span id="multiprogress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgress`

- <span id="multiprogress-default"></span>`fn default() -> Self`

##### `impl<T> From for MultiProgress`

- <span id="multiprogress-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProgress`

- <span id="multiprogress-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MultiProgress`

- <span id="multiprogress-toowned-type-owned"></span>`type Owned = T`

- <span id="multiprogress-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiprogress-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProgress`

- <span id="multiprogress-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiprogress-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProgress`

- <span id="multiprogress-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiprogress-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ProgressBar`

```rust
struct ProgressBar {
    state: std::sync::Arc<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Arc<crate::state::AtomicPosition>,
    ticker: std::sync::Arc<std::sync::Mutex<Option<Ticker>>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:25-29`](../../.source_1765633015/indicatif-0.18.3/src/progress_bar.rs#L25-L29)*

A progress bar or spinner

The progress bar is an `Arc` around its internal state. When the progress bar is cloned it
just increments the refcount (so the original and its clone share the same state).

#### Implementations

- <span id="progressbar-new"></span>`fn new(len: u64) -> Self`

  Creates a new progress bar with a given length

  

  This progress bar by default draws directly to stderr, and refreshes a maximum of 20 times

  a second. To change the refresh rate, [`set`](../hashbrown/set/index.md) the [draw target] to one with a different refresh

  rate.

  

- <span id="progressbar-no-length"></span>`fn no_length() -> Self`

  Creates a new progress bar without a specified length

  

  This progress bar by default draws directly to stderr, and refreshes a maximum of 20 times

  a second. To change the refresh rate, [`set`](../hashbrown/set/index.md) the [draw target] to one with a different refresh

  rate.

  

- <span id="progressbar-hidden"></span>`fn hidden() -> Self`

  Creates a completely hidden progress bar

  

  This progress bar still responds to API changes but it does not have a length or render in

  any way.

- <span id="progressbar-with-draw-target"></span>`fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

  Creates a new progress bar with a given length and draw target

- <span id="progressbar-style"></span>`fn style(&self) -> ProgressStyle` — [`ProgressStyle`](style/index.md#progressstyle)

  Get a clone of the current progress bar style.

- <span id="progressbar-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](style/index.md#progressstyle)

  A convenience builder-like function for a progress bar with a given style

- <span id="progressbar-with-tab-width"></span>`fn with_tab_width(self, tab_width: usize) -> Self`

  A convenience builder-like function for a progress bar with a given tab width

- <span id="progressbar-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

  A convenience builder-like function for a progress bar with a given prefix

  

  For the prefix to be visible, the `{prefix}` placeholder must be present in the template

  (see [`ProgressStyle`](style/index.md)).

- <span id="progressbar-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

  A convenience builder-like function for a progress bar with a given message

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](style/index.md)).

- <span id="progressbar-with-position"></span>`fn with_position(self, pos: u64) -> Self`

  A convenience builder-like function for a progress bar with a given position

- <span id="progressbar-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

  A convenience builder-like function for a progress bar with a given elapsed time

- <span id="progressbar-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](state/index.md#progressfinish)

  Sets the finish behavior for the progress bar

  

  This behavior is invoked when [`ProgressBar`](progress_bar/index.md) or

  [`ProgressBarIter`](iter/index.md) completes and

  `ProgressBar::is_finished()` is false.

  If you don't want the progress bar to be automatically finished then

  call `with_finish(Abandon)`.

  

  

- <span id="progressbar-new-spinner"></span>`fn new_spinner() -> Self`

  Creates a new spinner

  

  This spinner by default draws directly to stderr. This adds the default spinner style to it.

- <span id="progressbar-set-style"></span>`fn set_style(&self, style: ProgressStyle)` — [`ProgressStyle`](style/index.md#progressstyle)

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

  

  If the progress bar was added to a [`MultiProgress`](multi/index.md), the log line will be

  printed above all other progress bars.

  

- <span id="progressbar-update"></span>`fn update(&self, f: impl FnOnce(&mut ProgressState))` — [`ProgressState`](state/index.md#progressstate)

  Update the `ProgressBar`'s inner [`ProgressState`](state/index.md)

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

  (see [`ProgressStyle`](style/index.md)).

- <span id="progressbar-set-message"></span>`fn set_message(&self, msg: impl Into<Cow<'static, str>>)`

  Sets the current message of the progress bar

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](style/index.md)).

- <span id="progressbar-set-elapsed"></span>`fn set_elapsed(&self, elapsed: Duration)`

  Sets the elapsed time for the progress bar

- <span id="progressbar-downgrade"></span>`fn downgrade(&self) -> WeakProgressBar` — [`WeakProgressBar`](progress_bar/index.md#weakprogressbar)

  Creates a new weak reference to this [`ProgressBar`](progress_bar/index.md)

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

  [`ProgressStyle`](style/index.md)).

- <span id="progressbar-finish-and-clear"></span>`fn finish_and_clear(&self)`

  Finishes the progress bar and completely clears it

- <span id="progressbar-abandon"></span>`fn abandon(&self)`

  Finishes the progress bar and leaves the current message and progress

- <span id="progressbar-abandon-with-message"></span>`fn abandon_with_message(&self, msg: impl Into<Cow<'static, str>>)`

  Finishes the progress bar and sets a message, and leaves the current progress

  

  For the message to be visible, the `{msg}` placeholder must be present in the template (see

  [`ProgressStyle`](style/index.md)).

- <span id="progressbar-finish-using-style"></span>`fn finish_using_style(&self)`

  Finishes the progress bar using the behavior stored in the [`ProgressStyle`](style/index.md)

  

  See `ProgressBar::with_finish()`.

- <span id="progressbar-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

  Sets a different draw target for the progress bar

  

  This can be used to draw the progress bar to stderr (this is the default):

  

  ```rust,no_run

  use indicatif::{ProgressBar, ProgressDrawTarget};

  let pb = ProgressBar::new(100);

  pb.set_draw_target(ProgressDrawTarget::stderr());

  ```

  

  **Note:** Calling this method on a [`ProgressBar`](progress_bar/index.md) linked with a [`MultiProgress`](multi/index.md) (after

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

  

  If the progress bar was added to a [`MultiProgress`](multi/index.md), it will suspend the entire [`MultiProgress`](multi/index.md).

  

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

- <span id="progressbar-wrap-iter"></span>`fn wrap_iter<It: Iterator>(&self, it: It) -> ProgressBarIter<It>` — [`ProgressBarIter`](iter/index.md#progressbariter)

  Wraps an [`Iterator`](../cargo_docs_md/index.md) with the progress bar

  

  ```rust,no_run

  use indicatif::ProgressBar;

  let v = vec![1, 2, 3];

  let pb = ProgressBar::new(3);

  for item in pb.wrap_iter(v.iter()) {

      // ...

  }

  ```

- <span id="progressbar-wrap-read"></span>`fn wrap_read<R: io::Read>(&self, read: R) -> ProgressBarIter<R>` — [`ProgressBarIter`](iter/index.md#progressbariter)

  Wraps an [`io::Read`](../fs_err/index.md) with the progress bar

  

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

- <span id="progressbar-wrap-write"></span>`fn wrap_write<W: io::Write>(&self, write: W) -> ProgressBarIter<W>` — [`ProgressBarIter`](iter/index.md#progressbariter)

  Wraps an [`io::Write`](../fs_err/index.md) with the progress bar

  

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

- <span id="progressbar-state"></span>`fn state(&self) -> MutexGuard<'_, BarState>` — [`BarState`](state/index.md#barstate)

#### Trait Implementations

##### `impl Any for ProgressBar`

- <span id="progressbar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressBar`

- <span id="progressbar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressBar`

- <span id="progressbar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProgressBar`

- <span id="progressbar-clone"></span>`fn clone(&self) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

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

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:651-655`](../../.source_1765633015/indicatif-0.18.3/src/progress_bar.rs#L651-L655)*

A weak reference to a [`ProgressBar`](progress_bar/index.md).

Useful for creating custom steady tick implementations

#### Implementations

- <span id="weakprogressbar-new"></span>`fn new() -> Self`

  Create a new [`WeakProgressBar`](progress_bar/index.md) that returns `None` when `upgrade()` is called.

- <span id="weakprogressbar-upgrade"></span>`fn upgrade(&self) -> Option<ProgressBar>` — [`ProgressBar`](progress_bar/index.md#progressbar)

  Attempts to upgrade the Weak pointer to a [`ProgressBar`](progress_bar/index.md), delaying dropping of the inner

  value if successful. Returns [`None`](#none) if the inner value has since been dropped.

#### Trait Implementations

##### `impl Any for WeakProgressBar`

- <span id="weakprogressbar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WeakProgressBar`

- <span id="weakprogressbar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WeakProgressBar`

- <span id="weakprogressbar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WeakProgressBar`

- <span id="weakprogressbar-clone"></span>`fn clone(&self) -> WeakProgressBar` — [`WeakProgressBar`](progress_bar/index.md#weakprogressbar)

##### `impl CloneToUninit for WeakProgressBar`

- <span id="weakprogressbar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for WeakProgressBar`

- <span id="weakprogressbar-default"></span>`fn default() -> WeakProgressBar` — [`WeakProgressBar`](progress_bar/index.md#weakprogressbar)

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

*Defined in [`indicatif-0.18.3/src/state.rs:242-251`](../../.source_1765633015/indicatif-0.18.3/src/state.rs#L242-L251)*

The state of a progress bar at a moment in time.

#### Implementations

- <span id="progressstate-new"></span>`fn new(len: Option<u64>, pos: Arc<AtomicPosition>) -> Self` — [`AtomicPosition`](state/index.md#atomicposition)

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

### `ProgressStyle`

```rust
struct ProgressStyle {
    tick_strings: Vec<Box<str>>,
    progress_chars: Vec<Box<str>>,
    template: Template,
    char_width: usize,
    tab_width: usize,
    format_map: std::collections::HashMap<&'static str, Box<dyn ProgressTracker>>,
}
```

*Defined in [`indicatif-0.18.3/src/style.rs:23-31`](../../.source_1765633015/indicatif-0.18.3/src/style.rs#L23-L31)*

#### Implementations

- <span id="progressstyle-default-bar"></span>`fn default_bar() -> Self`

  Returns the default progress bar style for bars

- <span id="progressstyle-default-spinner"></span>`fn default_spinner() -> Self`

  Returns the default progress bar style for spinners

- <span id="progressstyle-with-template"></span>`fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](style/index.md#templateerror)

  Sets the template string for the progress bar

  

  Review the [list of template keys](../index.html#templates) for more information.

- <span id="progressstyle-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

- <span id="progressstyle-set-for-stderr"></span>`fn set_for_stderr(&mut self)`

  Specifies that the progress bar is intended to be printed to stderr

  

  The progress bar will determine whether to enable/disable colors based on stderr

  instead of stdout. Under the hood, this uses [`console::colors_enabled_stderr`](../console/utils/index.md).

- <span id="progressstyle-new"></span>`fn new(template: Template) -> Self` — [`Template`](style/index.md#template)

- <span id="progressstyle-tick-chars"></span>`fn tick_chars(self, s: &str) -> Self`

  Sets the tick character sequence for spinners

  

  Note that the last character is used as the [final tick string][Self::get_final_tick_str()].

  At least two characters are required to provide a non-final and final state.

- <span id="progressstyle-tick-strings"></span>`fn tick_strings(self, s: &[&str]) -> Self`

  Sets the tick string sequence for spinners

  

  Note that the last string is used as the [final tick string][Self::get_final_tick_str()].

  At least two strings are required to provide a non-final and final state.

- <span id="progressstyle-progress-chars"></span>`fn progress_chars(self, s: &str) -> Self`

  Sets the progress characters `(filled, current, to do)`

  

  You can pass more than three for a more detailed display.

  All passed grapheme clusters need to be of equal width.

- <span id="progressstyle-with-key"></span>`fn with_key<S: ProgressTracker + 'static>(self, key: &'static str, f: S) -> Self`

  Adds a custom key that owns a [`ProgressTracker`](style/index.md) to the template

- <span id="progressstyle-template"></span>`fn template(self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](style/index.md#templateerror)

  Sets the template string for the progress bar

  

  Review the [list of template keys](../index.html#templates) for more information.

- <span id="progressstyle-current-tick-str"></span>`fn current_tick_str(&self, state: &ProgressState) -> &str` — [`ProgressState`](state/index.md#progressstate)

- <span id="progressstyle-get-tick-str"></span>`fn get_tick_str(&self, idx: u64) -> &str`

  Returns the tick string for a given number

- <span id="progressstyle-get-final-tick-str"></span>`fn get_final_tick_str(&self) -> &str`

  Returns the tick string for the finished state

- <span id="progressstyle-format-bar"></span>`fn format_bar(&self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](style/index.md#bardisplay)

- <span id="progressstyle-format-state"></span>`fn format_state(&self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](state/index.md#progressstate), [`LineType`](draw_target/index.md#linetype)

- <span id="progressstyle-push-line"></span>`fn push_line(&self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](draw_target/index.md#linetype), [`ProgressState`](state/index.md#progressstate), [`WideElement`](style/index.md#wideelement)

  This is used exclusively to add the bars built above to the lines to print

#### Trait Implementations

##### `impl Any for ProgressStyle`

- <span id="progressstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressStyle`

- <span id="progressstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressStyle`

- <span id="progressstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ProgressStyle`

- <span id="progressstyle-clone"></span>`fn clone(&self) -> ProgressStyle` — [`ProgressStyle`](style/index.md#progressstyle)

##### `impl CloneToUninit for ProgressStyle`

- <span id="progressstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for ProgressStyle`

- <span id="progressstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressStyle`

- <span id="progressstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ProgressStyle`

- <span id="progressstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="progressstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="progressstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ProgressStyle`

- <span id="progressstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressStyle`

- <span id="progressstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MultiProgressAlignment`

```rust
enum MultiProgressAlignment {
    Top,
    Bottom,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:505-509`](../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L505-L509)*

Vertical alignment of a multi progress.

The alignment controls how the multi progress is aligned if some of its progress bars get removed.
E.g. [`Top`](MultiProgressAlignment::Top) alignment (default), when _progress bar 2_ is removed:
```ignore
[0/100] progress bar 1        [0/100] progress bar 1
[0/100] progress bar 2   =>   [0/100] progress bar 3
[0/100] progress bar 3
```

[`Bottom`](MultiProgressAlignment::Bottom) alignment
```ignore
[0/100] progress bar 1
[0/100] progress bar 2   =>   [0/100] progress bar 1
[0/100] progress bar 3        [0/100] progress bar 3
```

#### Trait Implementations

##### `impl Any for MultiProgressAlignment`

- <span id="multiprogressalignment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProgressAlignment`

- <span id="multiprogressalignment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProgressAlignment`

- <span id="multiprogressalignment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MultiProgressAlignment`

- <span id="multiprogressalignment-clone"></span>`fn clone(&self) -> MultiProgressAlignment` — [`MultiProgressAlignment`](multi/index.md#multiprogressalignment)

##### `impl CloneToUninit for MultiProgressAlignment`

- <span id="multiprogressalignment-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MultiProgressAlignment`

##### `impl Debug for MultiProgressAlignment`

- <span id="multiprogressalignment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgressAlignment`

- <span id="multiprogressalignment-default"></span>`fn default() -> MultiProgressAlignment` — [`MultiProgressAlignment`](multi/index.md#multiprogressalignment)

##### `impl<T> From for MultiProgressAlignment`

- <span id="multiprogressalignment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProgressAlignment`

- <span id="multiprogressalignment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MultiProgressAlignment`

- <span id="multiprogressalignment-toowned-type-owned"></span>`type Owned = T`

- <span id="multiprogressalignment-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiprogressalignment-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProgressAlignment`

- <span id="multiprogressalignment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiprogressalignment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProgressAlignment`

- <span id="multiprogressalignment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiprogressalignment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/state.rs:615-637`](../../.source_1765633015/indicatif-0.18.3/src/state.rs#L615-L637)*

Behavior of a progress bar when it is finished

This is invoked when a [`ProgressBar`](progress_bar/index.md) or [`ProgressBarIter`](iter/index.md) completes and
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

- <span id="progressfinish-clone"></span>`fn clone(&self) -> ProgressFinish` — [`ProgressFinish`](state/index.md#progressfinish)

##### `impl CloneToUninit for ProgressFinish`

- <span id="progressfinish-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ProgressFinish`

- <span id="progressfinish-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ProgressFinish`

- <span id="progressfinish-default"></span>`fn default() -> ProgressFinish` — [`ProgressFinish`](state/index.md#progressfinish)

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

## Traits

### `ProgressIterator`

```rust
trait ProgressIterator
where
    Self: Sized + Iterator { ... }
```

*Defined in [`indicatif-0.18.3/src/iter.rs:18-58`](../../.source_1765633015/indicatif-0.18.3/src/iter.rs#L18-L58)*

Wraps an iterator to display its progress.

#### Required Methods

- `fn progress_with(self, progress: ProgressBar) -> ProgressBarIter<Self>`

  Wrap an iterator with a custom progress bar.

#### Provided Methods

- `fn try_progress(self) -> Option<ProgressBarIter<Self>>`

  Wrap an iterator with default styling. Uses `Iterator::size_hint()` to get length.

- `fn progress(self) -> ProgressBarIter<Self>`

  Wrap an iterator with default styling.

- `fn progress_count(self, len: u64) -> ProgressBarIter<Self>`

  Wrap an iterator with an explicit element count.

- `fn progress_with_style(self, style: crate::ProgressStyle) -> ProgressBarIter<Self>`

  Wrap an iterator with a progress bar and style it.

#### Implementors

- `T`

### `TermLike`

```rust
trait TermLike: Debug + Send + Sync { ... }
```

*Defined in [`indicatif-0.18.3/src/term_like.rs:11-37`](../../.source_1765633015/indicatif-0.18.3/src/term_like.rs#L11-L37)*

A trait for minimal terminal-like behavior.

Anything that implements this trait can be used a draw target via `ProgressDrawTarget::term_like`.


#### Required Methods

- `fn width(&self) -> u16`

  Return the terminal width

- `fn move_cursor_up(&self, n: usize) -> io::Result<()>`

  Move the cursor up by `n` lines

- `fn move_cursor_down(&self, n: usize) -> io::Result<()>`

  Move the cursor down by `n` lines

- `fn move_cursor_right(&self, n: usize) -> io::Result<()>`

  Move the cursor right by `n` chars

- `fn move_cursor_left(&self, n: usize) -> io::Result<()>`

  Move the cursor left by `n` chars

- `fn write_line(&self, s: &str) -> io::Result<()>`

  Write a string and add a newline.

- `fn write_str(&self, s: &str) -> io::Result<()>`

  Write a string

- `fn clear_line(&self) -> io::Result<()>`

  Clear the current line and reset the cursor to beginning of the line

- `fn flush(&self) -> io::Result<()>`

#### Provided Methods

- `fn height(&self) -> u16`

  Return the terminal height

#### Implementors

- `console::Term`

