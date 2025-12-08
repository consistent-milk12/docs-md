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

indicatif comes with a [`ProgressBar`](#progressbar) type that supports both bounded
progress bar uses as well as unbounded "spinner" type progress reports.
Progress bars are [`Sync`](../miniz_oxide/index.md) and `Send` objects which means that they are
internally locked and can be passed from thread to thread.

Additionally a [`MultiProgress`](#multiprogress) utility is provided that can manage
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
[`ProgressStyle`](#progressstyle) and attaching it to the progress bar.

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

## Modules

- [`draw_target`](draw_target/index.md) - 
- [`format`](format/index.md) - 
- [`iter`](iter/index.md) - 
- [`multi`](multi/index.md) - 
- [`progress_bar`](progress_bar/index.md) - 
- [`state`](state/index.md) - 
- [`style`](style/index.md) - 
- [`term_like`](term_like/index.md) - 

## Structs

### `ProgressDrawTarget`

```rust
struct ProgressDrawTarget {
    kind: TargetKind,
}
```

Target for draw operations

This tells a [`ProgressBar`](crate::ProgressBar) or a
[`MultiProgress`](crate::MultiProgress) object where to paint to.
The draw target is a stateful wrapper over a drawing destination and
internally optimizes how often the state is painted to the output
device.

#### Implementations

- `fn stdout() -> Self`

- `fn stderr() -> Self`

- `fn stdout_with_hz(refresh_rate: u8) -> Self`

- `fn stderr_with_hz(refresh_rate: u8) -> Self`

- `fn new_remote(state: Arc<RwLock<MultiState>>, idx: usize) -> Self` — [`MultiState`](multi/index.md)

- `fn term(term: Term, refresh_rate: u8) -> Self`

- `fn term_like(term_like: Box<dyn TermLike>) -> Self` — [`TermLike`](#termlike)

- `fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self` — [`TermLike`](#termlike)

- `fn hidden() -> Self`

- `fn is_hidden(self: &Self) -> bool`

- `fn is_stderr(self: &Self) -> bool`

- `fn width(self: &Self) -> Option<u16>`

- `fn mark_zombie(self: &Self)`

- `fn set_move_cursor(self: &mut Self, move_cursor: bool)`

- `fn drawable(self: &mut Self, force_draw: bool, now: Instant) -> Option<Drawable<'_>>` — [`Drawable`](draw_target/index.md)

- `fn disconnect(self: &Self, now: Instant)`

- `fn remote(self: &Self) -> Option<(&Arc<RwLock<MultiState>>, usize)>` — [`MultiState`](multi/index.md)

- `fn adjust_last_line_count(self: &mut Self, adjust: LineAdjust)` — [`LineAdjust`](draw_target/index.md)

#### Trait Implementations

##### `impl Debug for ProgressDrawTarget`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

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

##### `impl Debug for BinaryBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BinaryBytes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for BinaryBytes`

- `fn to_string(self: &Self) -> String`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

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

##### `impl Debug for DecimalBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DecimalBytes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for DecimalBytes`

- `fn to_string(self: &Self) -> String`

### `FormattedDuration`

```rust
struct FormattedDuration(std::time::Duration);
```

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl Debug for FormattedDuration`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for FormattedDuration`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for FormattedDuration`

- `fn to_string(self: &Self) -> String`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

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

##### `impl Debug for HumanBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanBytes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanBytes`

- `fn to_string(self: &Self) -> String`

### `HumanCount`

```rust
struct HumanCount(u64);
```

Formats counts for human readability using commas

#### Trait Implementations

##### `impl Debug for HumanCount`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanCount`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanCount`

- `fn to_string(self: &Self) -> String`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl Debug for HumanDuration`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanDuration`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanDuration`

- `fn to_string(self: &Self) -> String`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl Debug for HumanFloatCount`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanFloatCount`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanFloatCount`

- `fn to_string(self: &Self) -> String`

### `ProgressBarIter<T>`

```rust
struct ProgressBarIter<T> {
    it: T,
    pub progress: crate::progress_bar::ProgressBar,
}
```

Wraps an iterator to display its progress.

#### Implementations

- `fn with_style(self: Self, style: ProgressStyle) -> Self` — [`ProgressStyle`](#progressstyle)

- `fn with_prefix(self: Self, prefix: impl Into<Cow<'static, str>>) -> Self`

- `fn with_message(self: Self, message: impl Into<Cow<'static, str>>) -> Self`

- `fn with_position(self: Self, position: u64) -> Self`

- `fn with_elapsed(self: Self, elapsed: Duration) -> Self`

- `fn with_finish(self: Self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](#progressfinish)

#### Trait Implementations

##### `impl<R: io::BufRead> BufRead for ProgressBarIter<R>`

- `fn fill_buf(self: &mut Self) -> io::Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl<T: $crate::fmt::Debug> Debug for ProgressBarIter<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: DoubleEndedIterator> DoubleEndedIterator for ProgressBarIter<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T: ExactSizeIterator> ExactSizeIterator for ProgressBarIter<T>`

- `fn len(self: &Self) -> usize`

##### `impl<T: FusedIterator> FusedIterator for ProgressBarIter<T>`

##### `impl<I> IntoIterator for ProgressBarIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<S, T: Iterator<Item = S>> Iterator for ProgressBarIter<T>`

- `type Item = S`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<S, T> ProgressIterator for ProgressBarIter<T>`

- `fn progress_with(self: Self, progress: ProgressBar) -> ProgressBarIter<T>` — [`ProgressBar`](#progressbar), [`ProgressBarIter`](#progressbariter)

##### `impl<R: io::Read> Read for ProgressBarIter<R>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

- `fn read_vectored(self: &mut Self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- `fn read_to_string(self: &mut Self, buf: &mut String) -> io::Result<usize>`

- `fn read_exact(self: &mut Self, buf: &mut [u8]) -> io::Result<()>`

##### `impl<S: io::Seek> Seek for ProgressBarIter<S>`

- `fn seek(self: &mut Self, f: io::SeekFrom) -> io::Result<u64>`

- `fn stream_position(self: &mut Self) -> io::Result<u64>`

##### `impl<W: io::Write> Write for ProgressBarIter<W>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

### `MultiProgress`

```rust
struct MultiProgress {
    state: std::sync::Arc<std::sync::RwLock<MultiState>>,
}
```

Manages multiple progress bars from different threads

#### Implementations

- `fn new() -> Self`

- `fn with_draw_target(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](#progressdrawtarget)

- `fn set_draw_target(self: &Self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](#progressdrawtarget)

- `fn set_move_cursor(self: &Self, move_cursor: bool)`

- `fn set_alignment(self: &Self, alignment: MultiProgressAlignment)` — [`MultiProgressAlignment`](#multiprogressalignment)

- `fn add(self: &Self, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](#progressbar)

- `fn insert(self: &Self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](#progressbar)

- `fn insert_from_back(self: &Self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](#progressbar)

- `fn insert_before(self: &Self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](#progressbar)

- `fn insert_after(self: &Self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](#progressbar)

- `fn remove(self: &Self, pb: &ProgressBar)` — [`ProgressBar`](#progressbar)

- `fn internalize(self: &Self, location: InsertLocation, pb: ProgressBar) -> ProgressBar` — [`InsertLocation`](multi/index.md), [`ProgressBar`](#progressbar)

- `fn println<I: AsRef<str>>(self: &Self, msg: I) -> io::Result<()>`

- `fn suspend<F: FnOnce() -> R, R>(self: &Self, f: F) -> R`

- `fn clear(self: &Self) -> io::Result<()>`

- `fn is_hidden(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for MultiProgress`

- `fn clone(self: &Self) -> MultiProgress` — [`MultiProgress`](#multiprogress)

##### `impl Debug for MultiProgress`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MultiProgress`

- `fn default() -> Self`

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

- `fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](#progressdrawtarget)

- `fn style(self: &Self) -> ProgressStyle` — [`ProgressStyle`](#progressstyle)

- `fn with_style(self: Self, style: ProgressStyle) -> Self` — [`ProgressStyle`](#progressstyle)

- `fn with_tab_width(self: Self, tab_width: usize) -> Self`

- `fn with_prefix(self: Self, prefix: impl Into<Cow<'static, str>>) -> Self`

- `fn with_message(self: Self, message: impl Into<Cow<'static, str>>) -> Self`

- `fn with_position(self: Self, pos: u64) -> Self`

- `fn with_elapsed(self: Self, elapsed: Duration) -> Self`

- `fn with_finish(self: Self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](#progressfinish)

- `fn new_spinner() -> Self`

- `fn set_style(self: &Self, style: ProgressStyle)` — [`ProgressStyle`](#progressstyle)

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

- `fn update(self: &Self, f: impl FnOnce(&mut ProgressState))` — [`ProgressState`](#progressstate)

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

- `fn set_draw_target(self: &Self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](#progressdrawtarget)

- `fn force_draw(self: &Self)`

- `fn suspend<F: FnOnce() -> R, R>(self: &Self, f: F) -> R`

- `fn wrap_iter<It: Iterator>(self: &Self, it: It) -> ProgressBarIter<It>` — [`ProgressBarIter`](#progressbariter)

- `fn wrap_read<R: io::Read>(self: &Self, read: R) -> ProgressBarIter<R>` — [`ProgressBarIter`](#progressbariter)

- `fn wrap_write<W: io::Write>(self: &Self, write: W) -> ProgressBarIter<W>` — [`ProgressBarIter`](#progressbariter)

- `fn position(self: &Self) -> u64`

- `fn length(self: &Self) -> Option<u64>`

- `fn eta(self: &Self) -> Duration`

- `fn per_sec(self: &Self) -> f64`

- `fn duration(self: &Self) -> Duration`

- `fn elapsed(self: &Self) -> Duration`

- `fn index(self: &Self) -> Option<usize>`

- `fn message(self: &Self) -> String`

- `fn prefix(self: &Self) -> String`

- `fn state(self: &Self) -> MutexGuard<'_, BarState>` — [`BarState`](state/index.md)

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

- `fn new(len: Option<u64>, pos: Arc<AtomicPosition>) -> Self` — [`AtomicPosition`](state/index.md)

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

#### Implementations

- `fn default_bar() -> Self`

- `fn default_spinner() -> Self`

- `fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](style/index.md)

- `fn set_tab_width(self: &mut Self, new_tab_width: usize)`

- `fn set_for_stderr(self: &mut Self)`

- `fn new(template: Template) -> Self` — [`Template`](style/index.md)

- `fn tick_chars(self: Self, s: &str) -> Self`

- `fn tick_strings(self: Self, s: &[&str]) -> Self`

- `fn progress_chars(self: Self, s: &str) -> Self`

- `fn with_key<S: ProgressTracker + 'static>(self: Self, key: &'static str, f: S) -> Self`

- `fn template(self: Self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](style/index.md)

- `fn current_tick_str(self: &Self, state: &ProgressState) -> &str` — [`ProgressState`](#progressstate)

- `fn get_tick_str(self: &Self, idx: u64) -> &str`

- `fn get_final_tick_str(self: &Self) -> &str`

- `fn format_bar(self: &Self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](style/index.md)

- `fn format_state(self: &Self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](#progressstate), [`LineType`](draw_target/index.md)

- `fn push_line(self: &Self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](draw_target/index.md), [`ProgressState`](#progressstate), [`WideElement`](style/index.md)

#### Trait Implementations

##### `impl Clone for ProgressStyle`

- `fn clone(self: &Self) -> ProgressStyle` — [`ProgressStyle`](#progressstyle)

## Enums

### `MultiProgressAlignment`

```rust
enum MultiProgressAlignment {
    Top,
    Bottom,
}
```

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

##### `impl Clone for MultiProgressAlignment`

- `fn clone(self: &Self) -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

##### `impl Copy for MultiProgressAlignment`

##### `impl Debug for MultiProgressAlignment`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MultiProgressAlignment`

- `fn default() -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

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

This is invoked when a [`ProgressBar`](#progressbar) or [`ProgressBarIter`](#progressbariter) completes and
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

- `fn clone(self: &Self) -> ProgressFinish` — [`ProgressFinish`](#progressfinish)

##### `impl Debug for ProgressFinish`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ProgressFinish`

- `fn default() -> ProgressFinish` — [`ProgressFinish`](#progressfinish)

## Traits

