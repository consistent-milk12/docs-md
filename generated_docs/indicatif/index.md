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
  - [`draw_target`](#draw_target)
  - [`format`](#format)
  - [`iter`](#iter)
  - [`multi`](#multi)
  - [`progress_bar`](#progress_bar)
  - [`state`](#state)
  - [`style`](#style)
  - [`term_like`](#term_like)
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
| [`draw_target`](#draw_target) | mod |  |
| [`format`](#format) | mod |  |
| [`iter`](#iter) | mod |  |
| [`multi`](#multi) | mod |  |
| [`progress_bar`](#progress_bar) | mod |  |
| [`state`](#state) | mod |  |
| [`style`](#style) | mod |  |
| [`term_like`](#term_like) | mod |  |
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

*Defined in [`indicatif-0.18.3/src/draw_target.rs:25-27`](../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L25-L27)*

Target for draw operations

This tells a [`ProgressBar`](crate::ProgressBar) or a
[`MultiProgress`](crate::MultiProgress) object where to paint to.
The draw target is a stateful wrapper over a drawing destination and
internally optimizes how often the state is painted to the output
device.

#### Implementations

- <span id="progressdrawtarget-stdout"></span>`fn stdout() -> Self`

- <span id="progressdrawtarget-stderr"></span>`fn stderr() -> Self`

- <span id="progressdrawtarget-stdout-with-hz"></span>`fn stdout_with_hz(refresh_rate: u8) -> Self`

- <span id="progressdrawtarget-stderr-with-hz"></span>`fn stderr_with_hz(refresh_rate: u8) -> Self`

- <span id="progressdrawtarget-new-remote"></span>`fn new_remote(state: Arc<RwLock<MultiState>>, idx: usize) -> Self` — [`MultiState`](multi/index.md#multistate)

- <span id="progressdrawtarget-term"></span>`fn term(term: Term, refresh_rate: u8) -> Self`

- <span id="progressdrawtarget-term-like"></span>`fn term_like(term_like: Box<dyn TermLike>) -> Self` — [`TermLike`](term_like/index.md#termlike)

- <span id="progressdrawtarget-term-like-with-hz"></span>`fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self` — [`TermLike`](term_like/index.md#termlike)

- <span id="progressdrawtarget-hidden"></span>`fn hidden() -> Self`

- <span id="progressdrawtarget-is-hidden"></span>`fn is_hidden(&self) -> bool`

- <span id="progressdrawtarget-is-stderr"></span>`fn is_stderr(&self) -> bool`

- <span id="progressdrawtarget-width"></span>`fn width(&self) -> Option<u16>`

- <span id="progressdrawtarget-mark-zombie"></span>`fn mark_zombie(&self)`

- <span id="progressdrawtarget-set-move-cursor"></span>`fn set_move_cursor(&mut self, move_cursor: bool)`

- <span id="progressdrawtarget-drawable"></span>`fn drawable(&mut self, force_draw: bool, now: Instant) -> Option<Drawable<'_>>` — [`Drawable`](draw_target/index.md#drawable)

- <span id="progressdrawtarget-disconnect"></span>`fn disconnect(&self, now: Instant)`

- <span id="progressdrawtarget-remote"></span>`fn remote(&self) -> Option<(&Arc<RwLock<MultiState>>, usize)>` — [`MultiState`](multi/index.md#multistate)

- <span id="progressdrawtarget-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](draw_target/index.md#lineadjust)

#### Trait Implementations

##### `impl Debug for ProgressDrawTarget`

- <span id="progressdrawtarget-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:64`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L64)*

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

- <span id="binarybytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BinaryBytes`

- <span id="binarybytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for BinaryBytes`

- <span id="binarybytes-to-string"></span>`fn to_string(&self) -> String`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:49`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L49)*

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

- <span id="decimalbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecimalBytes`

- <span id="decimalbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for DecimalBytes`

- <span id="decimalbytes-to-string"></span>`fn to_string(&self) -> String`

### `FormattedDuration`

```rust
struct FormattedDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:15`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L15)*

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl Debug for FormattedDuration`

- <span id="formattedduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FormattedDuration`

- <span id="formattedduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for FormattedDuration`

- <span id="formattedduration-to-string"></span>`fn to_string(&self) -> String`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:34`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L34)*

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

- <span id="humanbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanBytes`

- <span id="humanbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanBytes`

- <span id="humanbytes-to-string"></span>`fn to_string(&self) -> String`

### `HumanCount`

```rust
struct HumanCount(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:68`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L68)*

Formats counts for human readability using commas

#### Trait Implementations

##### `impl Debug for HumanCount`

- <span id="humancount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanCount`

- <span id="humancount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanCount`

- <span id="humancount-to-string"></span>`fn to_string(&self) -> String`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:19`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L19)*

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl Debug for HumanDuration`

- <span id="humanduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanDuration`

- <span id="humanduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanDuration`

- <span id="humanduration-to-string"></span>`fn to_string(&self) -> String`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:72`](../../.source_1765210505/indicatif-0.18.3/src/format.rs#L72)*

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl Debug for HumanFloatCount`

- <span id="humanfloatcount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanFloatCount`

- <span id="humanfloatcount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanFloatCount`

- <span id="humanfloatcount-to-string"></span>`fn to_string(&self) -> String`

### `ProgressBarIter<T>`

```rust
struct ProgressBarIter<T> {
    it: T,
    pub progress: crate::progress_bar::ProgressBar,
}
```

*Defined in [`indicatif-0.18.3/src/iter.rs:62-65`](../../.source_1765210505/indicatif-0.18.3/src/iter.rs#L62-L65)*

Wraps an iterator to display its progress.

#### Implementations

- <span id="progressbariter-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](style/index.md#progressstyle)

- <span id="progressbariter-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbariter-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbariter-with-position"></span>`fn with_position(self, position: u64) -> Self`

- <span id="progressbariter-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

- <span id="progressbariter-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](state/index.md#progressfinish)

#### Trait Implementations

##### `impl<R: io::BufRead> BufRead for ProgressBarIter<R>`

- <span id="progressbariter-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="progressbariter-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<T: fmt::Debug> Debug for ProgressBarIter<T>`

- <span id="progressbariter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: DoubleEndedIterator> DoubleEndedIterator for ProgressBarIter<T>`

- <span id="progressbariter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: ExactSizeIterator> ExactSizeIterator for ProgressBarIter<T>`

- <span id="progressbariter-len"></span>`fn len(&self) -> usize`

##### `impl<T: FusedIterator> FusedIterator for ProgressBarIter<T>`

##### `impl<I> IntoIterator for ProgressBarIter<T>`

- <span id="progressbariter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="progressbariter-type-intoiter"></span>`type IntoIter = I`

- <span id="progressbariter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<S, T: Iterator<Item = S>> Iterator for ProgressBarIter<T>`

- <span id="progressbariter-type-item"></span>`type Item = S`

- <span id="progressbariter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<S, T> ProgressIterator for ProgressBarIter<T>`

- <span id="progressbariter-progress-with"></span>`fn progress_with(self, progress: ProgressBar) -> ProgressBarIter<T>` — [`ProgressBar`](progress_bar/index.md#progressbar), [`ProgressBarIter`](iter/index.md#progressbariter)

##### `impl<R: io::Read> Read for ProgressBarIter<R>`

- <span id="progressbariter-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

- <span id="progressbariter-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- <span id="progressbariter-read-to-string"></span>`fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>`

- <span id="progressbariter-read-exact"></span>`fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>`

##### `impl<S: io::Seek> Seek for ProgressBarIter<S>`

- <span id="progressbariter-seek"></span>`fn seek(&mut self, f: io::SeekFrom) -> io::Result<u64>`

- <span id="progressbariter-stream-position"></span>`fn stream_position(&mut self) -> io::Result<u64>`

##### `impl<W: io::Write> Write for ProgressBarIter<W>`

- <span id="progressbariter-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="progressbariter-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- <span id="progressbariter-flush"></span>`fn flush(&mut self) -> io::Result<()>`

### `MultiProgress`

```rust
struct MultiProgress {
    state: std::sync::Arc<std::sync::RwLock<MultiState>>,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:18-20`](../../.source_1765210505/indicatif-0.18.3/src/multi.rs#L18-L20)*

Manages multiple progress bars from different threads

#### Implementations

- <span id="multiprogress-new"></span>`fn new() -> Self`

- <span id="multiprogress-with-draw-target"></span>`fn with_draw_target(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

- <span id="multiprogress-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

- <span id="multiprogress-set-move-cursor"></span>`fn set_move_cursor(&self, move_cursor: bool)`

- <span id="multiprogress-set-alignment"></span>`fn set_alignment(&self, alignment: MultiProgressAlignment)` — [`MultiProgressAlignment`](multi/index.md#multiprogressalignment)

- <span id="multiprogress-add"></span>`fn add(&self, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-insert"></span>`fn insert(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-insert-from-back"></span>`fn insert_from_back(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-insert-before"></span>`fn insert_before(&self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-insert-after"></span>`fn insert_after(&self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-remove"></span>`fn remove(&self, pb: &ProgressBar)` — [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-internalize"></span>`fn internalize(&self, location: InsertLocation, pb: ProgressBar) -> ProgressBar` — [`InsertLocation`](multi/index.md#insertlocation), [`ProgressBar`](progress_bar/index.md#progressbar)

- <span id="multiprogress-println"></span>`fn println<I: AsRef<str>>(&self, msg: I) -> io::Result<()>`

- <span id="multiprogress-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

- <span id="multiprogress-clear"></span>`fn clear(&self) -> io::Result<()>`

- <span id="multiprogress-is-hidden"></span>`fn is_hidden(&self) -> bool`

#### Trait Implementations

##### `impl Clone for MultiProgress`

- <span id="multiprogress-clone"></span>`fn clone(&self) -> MultiProgress` — [`MultiProgress`](multi/index.md#multiprogress)

##### `impl Debug for MultiProgress`

- <span id="multiprogress-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgress`

- <span id="multiprogress-default"></span>`fn default() -> Self`

### `ProgressBar`

```rust
struct ProgressBar {
    state: std::sync::Arc<std::sync::Mutex<crate::state::BarState>>,
    pos: std::sync::Arc<crate::state::AtomicPosition>,
    ticker: std::sync::Arc<std::sync::Mutex<Option<Ticker>>>,
}
```

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:25-29`](../../.source_1765210505/indicatif-0.18.3/src/progress_bar.rs#L25-L29)*

A progress bar or spinner

The progress bar is an `Arc` around its internal state. When the progress bar is cloned it
just increments the refcount (so the original and its clone share the same state).

#### Implementations

- <span id="progressbar-new"></span>`fn new(len: u64) -> Self`

- <span id="progressbar-no-length"></span>`fn no_length() -> Self`

- <span id="progressbar-hidden"></span>`fn hidden() -> Self`

- <span id="progressbar-with-draw-target"></span>`fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

- <span id="progressbar-style"></span>`fn style(&self) -> ProgressStyle` — [`ProgressStyle`](style/index.md#progressstyle)

- <span id="progressbar-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](style/index.md#progressstyle)

- <span id="progressbar-with-tab-width"></span>`fn with_tab_width(self, tab_width: usize) -> Self`

- <span id="progressbar-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbar-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbar-with-position"></span>`fn with_position(self, pos: u64) -> Self`

- <span id="progressbar-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

- <span id="progressbar-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](state/index.md#progressfinish)

- <span id="progressbar-new-spinner"></span>`fn new_spinner() -> Self`

- <span id="progressbar-set-style"></span>`fn set_style(&self, style: ProgressStyle)` — [`ProgressStyle`](style/index.md#progressstyle)

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

- <span id="progressbar-update"></span>`fn update(&self, f: impl FnOnce(&mut ProgressState))` — [`ProgressState`](state/index.md#progressstate)

- <span id="progressbar-set-position"></span>`fn set_position(&self, pos: u64)`

- <span id="progressbar-unset-length"></span>`fn unset_length(&self)`

- <span id="progressbar-set-length"></span>`fn set_length(&self, len: u64)`

- <span id="progressbar-inc-length"></span>`fn inc_length(&self, delta: u64)`

- <span id="progressbar-dec-length"></span>`fn dec_length(&self, delta: u64)`

- <span id="progressbar-set-prefix"></span>`fn set_prefix(&self, prefix: impl Into<Cow<'static, str>>)`

- <span id="progressbar-set-message"></span>`fn set_message(&self, msg: impl Into<Cow<'static, str>>)`

- <span id="progressbar-set-elapsed"></span>`fn set_elapsed(&self, elapsed: Duration)`

- <span id="progressbar-downgrade"></span>`fn downgrade(&self) -> WeakProgressBar` — [`WeakProgressBar`](progress_bar/index.md#weakprogressbar)

- <span id="progressbar-reset-eta"></span>`fn reset_eta(&self)`

- <span id="progressbar-reset-elapsed"></span>`fn reset_elapsed(&self)`

- <span id="progressbar-reset"></span>`fn reset(&self)`

- <span id="progressbar-finish"></span>`fn finish(&self)`

- <span id="progressbar-finish-with-message"></span>`fn finish_with_message(&self, msg: impl Into<Cow<'static, str>>)`

- <span id="progressbar-finish-and-clear"></span>`fn finish_and_clear(&self)`

- <span id="progressbar-abandon"></span>`fn abandon(&self)`

- <span id="progressbar-abandon-with-message"></span>`fn abandon_with_message(&self, msg: impl Into<Cow<'static, str>>)`

- <span id="progressbar-finish-using-style"></span>`fn finish_using_style(&self)`

- <span id="progressbar-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](draw_target/index.md#progressdrawtarget)

- <span id="progressbar-force-draw"></span>`fn force_draw(&self)`

- <span id="progressbar-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

- <span id="progressbar-wrap-iter"></span>`fn wrap_iter<It: Iterator>(&self, it: It) -> ProgressBarIter<It>` — [`ProgressBarIter`](iter/index.md#progressbariter)

- <span id="progressbar-wrap-read"></span>`fn wrap_read<R: io::Read>(&self, read: R) -> ProgressBarIter<R>` — [`ProgressBarIter`](iter/index.md#progressbariter)

- <span id="progressbar-wrap-write"></span>`fn wrap_write<W: io::Write>(&self, write: W) -> ProgressBarIter<W>` — [`ProgressBarIter`](iter/index.md#progressbariter)

- <span id="progressbar-position"></span>`fn position(&self) -> u64`

- <span id="progressbar-length"></span>`fn length(&self) -> Option<u64>`

- <span id="progressbar-eta"></span>`fn eta(&self) -> Duration`

- <span id="progressbar-per-sec"></span>`fn per_sec(&self) -> f64`

- <span id="progressbar-duration"></span>`fn duration(&self) -> Duration`

- <span id="progressbar-elapsed"></span>`fn elapsed(&self) -> Duration`

- <span id="progressbar-index"></span>`fn index(&self) -> Option<usize>`

- <span id="progressbar-message"></span>`fn message(&self) -> String`

- <span id="progressbar-prefix"></span>`fn prefix(&self) -> String`

- <span id="progressbar-state"></span>`fn state(&self) -> MutexGuard<'_, BarState>` — [`BarState`](state/index.md#barstate)

#### Trait Implementations

##### `impl Clone for ProgressBar`

- <span id="progressbar-clone"></span>`fn clone(&self) -> ProgressBar` — [`ProgressBar`](progress_bar/index.md#progressbar)

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

*Defined in [`indicatif-0.18.3/src/progress_bar.rs:651-655`](../../.source_1765210505/indicatif-0.18.3/src/progress_bar.rs#L651-L655)*

A weak reference to a [`ProgressBar`](progress_bar/index.md).

Useful for creating custom steady tick implementations

#### Implementations

- <span id="weakprogressbar-new"></span>`fn new() -> Self`

- <span id="weakprogressbar-upgrade"></span>`fn upgrade(&self) -> Option<ProgressBar>` — [`ProgressBar`](progress_bar/index.md#progressbar)

#### Trait Implementations

##### `impl Clone for WeakProgressBar`

- <span id="weakprogressbar-clone"></span>`fn clone(&self) -> WeakProgressBar` — [`WeakProgressBar`](progress_bar/index.md#weakprogressbar)

##### `impl Default for WeakProgressBar`

- <span id="weakprogressbar-default"></span>`fn default() -> WeakProgressBar` — [`WeakProgressBar`](progress_bar/index.md#weakprogressbar)

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

*Defined in [`indicatif-0.18.3/src/state.rs:242-251`](../../.source_1765210505/indicatif-0.18.3/src/state.rs#L242-L251)*

The state of a progress bar at a moment in time.

#### Implementations

- <span id="progressstate-new"></span>`fn new(len: Option<u64>, pos: Arc<AtomicPosition>) -> Self` — [`AtomicPosition`](state/index.md#atomicposition)

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

*Defined in [`indicatif-0.18.3/src/style.rs:23-31`](../../.source_1765210505/indicatif-0.18.3/src/style.rs#L23-L31)*

#### Implementations

- <span id="progressstyle-default-bar"></span>`fn default_bar() -> Self`

- <span id="progressstyle-default-spinner"></span>`fn default_spinner() -> Self`

- <span id="progressstyle-with-template"></span>`fn with_template(template: &str) -> Result<Self, TemplateError>` — [`TemplateError`](style/index.md#templateerror)

- <span id="progressstyle-set-tab-width"></span>`fn set_tab_width(&mut self, new_tab_width: usize)`

- <span id="progressstyle-set-for-stderr"></span>`fn set_for_stderr(&mut self)`

- <span id="progressstyle-new"></span>`fn new(template: Template) -> Self` — [`Template`](style/index.md#template)

- <span id="progressstyle-tick-chars"></span>`fn tick_chars(self, s: &str) -> Self`

- <span id="progressstyle-tick-strings"></span>`fn tick_strings(self, s: &[&str]) -> Self`

- <span id="progressstyle-progress-chars"></span>`fn progress_chars(self, s: &str) -> Self`

- <span id="progressstyle-with-key"></span>`fn with_key<S: ProgressTracker + 'static>(self, key: &'static str, f: S) -> Self`

- <span id="progressstyle-template"></span>`fn template(self, s: &str) -> Result<Self, TemplateError>` — [`TemplateError`](style/index.md#templateerror)

- <span id="progressstyle-current-tick-str"></span>`fn current_tick_str(&self, state: &ProgressState) -> &str` — [`ProgressState`](state/index.md#progressstate)

- <span id="progressstyle-get-tick-str"></span>`fn get_tick_str(&self, idx: u64) -> &str`

- <span id="progressstyle-get-final-tick-str"></span>`fn get_final_tick_str(&self) -> &str`

- <span id="progressstyle-format-bar"></span>`fn format_bar(&self, fract: f32, width: usize, alt_style: Option<&Style>) -> BarDisplay<'_>` — [`BarDisplay`](style/index.md#bardisplay)

- <span id="progressstyle-format-state"></span>`fn format_state(&self, state: &ProgressState, lines: &mut Vec<LineType>, target_width: u16)` — [`ProgressState`](state/index.md#progressstate), [`LineType`](draw_target/index.md#linetype)

- <span id="progressstyle-push-line"></span>`fn push_line(&self, lines: &mut Vec<LineType>, cur: &mut String, state: &ProgressState, buf: &mut String, target_width: u16, wide: &Option<WideElement<'_>>)` — [`LineType`](draw_target/index.md#linetype), [`ProgressState`](state/index.md#progressstate), [`WideElement`](style/index.md#wideelement)

#### Trait Implementations

##### `impl Clone for ProgressStyle`

- <span id="progressstyle-clone"></span>`fn clone(&self) -> ProgressStyle` — [`ProgressStyle`](style/index.md#progressstyle)

## Enums

### `MultiProgressAlignment`

```rust
enum MultiProgressAlignment {
    Top,
    Bottom,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:505-509`](../../.source_1765210505/indicatif-0.18.3/src/multi.rs#L505-L509)*

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

- <span id="multiprogressalignment-clone"></span>`fn clone(&self) -> MultiProgressAlignment` — [`MultiProgressAlignment`](multi/index.md#multiprogressalignment)

##### `impl Copy for MultiProgressAlignment`

##### `impl Debug for MultiProgressAlignment`

- <span id="multiprogressalignment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgressAlignment`

- <span id="multiprogressalignment-default"></span>`fn default() -> MultiProgressAlignment` — [`MultiProgressAlignment`](multi/index.md#multiprogressalignment)

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

*Defined in [`indicatif-0.18.3/src/state.rs:615-637`](../../.source_1765210505/indicatif-0.18.3/src/state.rs#L615-L637)*

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

##### `impl Clone for ProgressFinish`

- <span id="progressfinish-clone"></span>`fn clone(&self) -> ProgressFinish` — [`ProgressFinish`](state/index.md#progressfinish)

##### `impl Debug for ProgressFinish`

- <span id="progressfinish-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ProgressFinish`

- <span id="progressfinish-default"></span>`fn default() -> ProgressFinish` — [`ProgressFinish`](state/index.md#progressfinish)

## Traits

### `ProgressIterator`

```rust
trait ProgressIterator
where
    Self: Sized + Iterator { ... }
```

*Defined in [`indicatif-0.18.3/src/iter.rs:18-58`](../../.source_1765210505/indicatif-0.18.3/src/iter.rs#L18-L58)*

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

*Defined in [`indicatif-0.18.3/src/term_like.rs:11-37`](../../.source_1765210505/indicatif-0.18.3/src/term_like.rs#L11-L37)*

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

