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
Progress bars are [`Sync`](#sync) and [`Send`](#send) objects which means that they are
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
[dependencies](#dependencies)
indicatif = {version = "*", features = ["rayon"]}
```

And then use it like this:

```rust,ignore
# extern crate rayon;
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

let v: Vec<_> = (0..100000).collect();
let v2: Vec<_> = v.par_iter().progress_count(v.len() as u64).map(|i| i + 1).collect();
assert_eq!(v2[0], 1);
```

Or if you'd like to customize the progress bar:

```rust,ignore
# extern crate rayon;
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
# use indicatif::{ProgressBar, ProgressStyle};
# let bar = ProgressBar::new(0);
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
[`ProgressTracker`](#progresstracker) implementation.

The design of the progress bar can be altered with the integrated
template functionality.  The template can be set by changing a
[`ProgressStyle`](indicatif/style/index.md) and attaching it to the progress bar.

# Human Readable Formatting

There are some formatting wrappers for showing elapsed time and
file sizes for human users:

```rust
# use std::time::Duration;
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

- [`style`](style/index.md) - 

## Structs

### `ProgressDrawTarget`

```rust
struct ProgressDrawTarget {
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
  Draw to a buffered stdout terminal at a max of 20 times a second.

- `fn stderr() -> Self`
  Draw to a buffered stderr terminal at a max of 20 times a second.

- `fn stdout_with_hz(refresh_rate: u8) -> Self`
  Draw to a buffered stdout terminal at a max of `refresh_rate` times a second.

- `fn stderr_with_hz(refresh_rate: u8) -> Self`
  Draw to a buffered stderr terminal at a max of `refresh_rate` times a second.

- `fn term(term: Term, refresh_rate: u8) -> Self`
  Draw to a terminal, with a specific refresh rate.

- `fn term_like(term_like: Box<dyn TermLike>) -> Self`
  Draw to a boxed object that implements the [`TermLike`] trait.

- `fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self`
  Draw to a boxed object that implements the [`TermLike`] trait,

- `fn hidden() -> Self`
  A hidden draw target.

- `fn is_hidden(self: &Self) -> bool`
  Returns true if the draw target is hidden.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

Formats bytes for human readability using ISO/IEC prefixes

# Examples
```rust
# use indicatif::BinaryBytes;
assert_eq!("15 B",     format!("{}", BinaryBytes(15)));
assert_eq!("1.46 KiB", format!("{}", BinaryBytes(1_500)));
assert_eq!("1.43 MiB", format!("{}", BinaryBytes(1_500_000)));
assert_eq!("1.40 GiB", format!("{}", BinaryBytes(1_500_000_000)));
assert_eq!("1.36 TiB", format!("{}", BinaryBytes(1_500_000_000_000)));
assert_eq!("1.33 PiB", format!("{}", BinaryBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

Formats bytes for human readability using SI prefixes

# Examples
```rust
# use indicatif::DecimalBytes;
assert_eq!("15 B",    format!("{}", DecimalBytes(15)));
assert_eq!("1.50 kB", format!("{}", DecimalBytes(1_500)));
assert_eq!("1.50 MB", format!("{}", DecimalBytes(1_500_000)));
assert_eq!("1.50 GB", format!("{}", DecimalBytes(1_500_000_000)));
assert_eq!("1.50 TB", format!("{}", DecimalBytes(1_500_000_000_000)));
assert_eq!("1.50 PB", format!("{}", DecimalBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FormattedDuration`

```rust
struct FormattedDuration(std::time::Duration);
```

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

Formats bytes for human readability

# Examples
```rust
# use indicatif::HumanBytes;
assert_eq!("15 B",     format!("{}", HumanBytes(15)));
assert_eq!("1.46 KiB", format!("{}", HumanBytes(1_500)));
assert_eq!("1.43 MiB", format!("{}", HumanBytes(1_500_000)));
assert_eq!("1.40 GiB", format!("{}", HumanBytes(1_500_000_000)));
assert_eq!("1.36 TiB", format!("{}", HumanBytes(1_500_000_000_000)));
assert_eq!("1.33 PiB", format!("{}", HumanBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HumanCount`

```rust
struct HumanCount(u64);
```

Formats counts for human readability using commas

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ProgressBarIter<T>`

```rust
struct ProgressBarIter<T> {
    pub progress: crate::progress_bar::ProgressBar,
}
```

Wraps an iterator to display its progress.

#### Implementations

- `fn with_style(self: Self, style: ProgressStyle) -> Self`
  Builder-like function for setting underlying progress bar's style.

- `fn with_prefix(self: Self, prefix: impl Into<Cow<'static, str>>) -> Self`
  Builder-like function for setting underlying progress bar's prefix.

- `fn with_message(self: Self, message: impl Into<Cow<'static, str>>) -> Self`
  Builder-like function for setting underlying progress bar's message.

- `fn with_position(self: Self, position: u64) -> Self`
  Builder-like function for setting underlying progress bar's position.

- `fn with_elapsed(self: Self, elapsed: Duration) -> Self`
  Builder-like function for setting underlying progress bar's elapsed time.

- `fn with_finish(self: Self, finish: ProgressFinish) -> Self`
  Builder-like function for setting underlying progress bar's finish behavior.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl BufRead<R: io::BufRead>`

- `fn fill_buf(self: &mut Self) -> io::Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl DoubleEndedIterator<T: DoubleEndedIterator>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<T: ExactSizeIterator>`

- `fn len(self: &Self) -> usize`

##### `impl FusedIterator<T: FusedIterator>`

##### `impl Iterator<S, T: Iterator<Item = S>>`

- `type Item = S`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ProgressIterator<S, T>`

- `fn progress_with(self: Self, progress: ProgressBar) -> ProgressBarIter<T>`

##### `impl Read<R: io::Read>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

- `fn read_vectored(self: &mut Self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- `fn read_to_string(self: &mut Self, buf: &mut String) -> io::Result<usize>`

- `fn read_exact(self: &mut Self, buf: &mut [u8]) -> io::Result<()>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write<W: io::Write>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Seek<S: io::Seek>`

- `fn seek(self: &mut Self, f: io::SeekFrom) -> io::Result<u64>`

- `fn stream_position(self: &mut Self) -> io::Result<u64>`

### `MultiProgress`

```rust
struct MultiProgress {
}
```

Manages multiple progress bars from different threads

#### Implementations

- `fn new() -> Self`
  Creates a new multi progress object.

- `fn with_draw_target(draw_target: ProgressDrawTarget) -> Self`
  Creates a new multi progress object with the given draw target.

- `fn set_draw_target(self: &Self, target: ProgressDrawTarget)`
  Sets a different draw target for the multiprogress bar.

- `fn set_move_cursor(self: &Self, move_cursor: bool)`
  Set whether we should try to move the cursor when possible instead of clearing lines.

- `fn set_alignment(self: &Self, alignment: MultiProgressAlignment)`
  Set alignment flag

- `fn add(self: &Self, pb: ProgressBar) -> ProgressBar`
  Adds a progress bar.

- `fn insert(self: &Self, index: usize, pb: ProgressBar) -> ProgressBar`
  Inserts a progress bar.

- `fn insert_from_back(self: &Self, index: usize, pb: ProgressBar) -> ProgressBar`
  Inserts a progress bar from the back.

- `fn insert_before(self: &Self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar`
  Inserts a progress bar before an existing one.

- `fn insert_after(self: &Self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar`
  Inserts a progress bar after an existing one.

- `fn remove(self: &Self, pb: &ProgressBar)`
  Removes a progress bar.

- `fn println<I: AsRef<str>>(self: &Self, msg: I) -> io::Result<()>`
  Print a log line above all progress bars in the [`MultiProgress`]

- `fn suspend<F: FnOnce() -> R, R>(self: &Self, f: F) -> R`
  Hide all progress bars temporarily, execute `f`, then redraw the [`MultiProgress`]

- `fn clear(self: &Self) -> io::Result<()>`

- `fn is_hidden(self: &Self) -> bool`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> MultiProgress`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `ProgressBar`

```rust
struct ProgressBar {
}
```

A progress bar or spinner

The progress bar is an [`Arc`](#arc) around its internal state. When the progress bar is cloned it
just increments the refcount (so the original and its clone share the same state).

#### Implementations

- `fn new(len: u64) -> Self`
  Creates a new progress bar with a given length

- `fn no_length() -> Self`
  Creates a new progress bar without a specified length

- `fn hidden() -> Self`
  Creates a completely hidden progress bar

- `fn with_draw_target(len: Option<u64>, draw_target: ProgressDrawTarget) -> Self`
  Creates a new progress bar with a given length and draw target

- `fn style(self: &Self) -> ProgressStyle`
  Get a clone of the current progress bar style.

- `fn with_style(self: Self, style: ProgressStyle) -> Self`
  A convenience builder-like function for a progress bar with a given style

- `fn with_tab_width(self: Self, tab_width: usize) -> Self`
  A convenience builder-like function for a progress bar with a given tab width

- `fn with_prefix(self: Self, prefix: impl Into<Cow<'static, str>>) -> Self`
  A convenience builder-like function for a progress bar with a given prefix

- `fn with_message(self: Self, message: impl Into<Cow<'static, str>>) -> Self`
  A convenience builder-like function for a progress bar with a given message

- `fn with_position(self: Self, pos: u64) -> Self`
  A convenience builder-like function for a progress bar with a given position

- `fn with_elapsed(self: Self, elapsed: Duration) -> Self`
  A convenience builder-like function for a progress bar with a given elapsed time

- `fn with_finish(self: Self, finish: ProgressFinish) -> Self`
  Sets the finish behavior for the progress bar

- `fn new_spinner() -> Self`
  Creates a new spinner

- `fn set_style(self: &Self, style: ProgressStyle)`
  Overrides the stored style

- `fn set_tab_width(self: &Self, tab_width: usize)`
  Sets the tab width (default: 8). All tabs will be expanded to this many spaces.

- `fn enable_steady_tick(self: &Self, interval: Duration)`
  Spawns a background thread to tick the progress bar

- `fn disable_steady_tick(self: &Self)`
  Undoes [`ProgressBar::enable_steady_tick()`]

- `fn tick(self: &Self)`
  Manually ticks the spinner or progress bar

- `fn inc(self: &Self, delta: u64)`
  Advances the position of the progress bar by `delta`

- `fn dec(self: &Self, delta: u64)`
  Decrease the position of the progress bar by `delta`

- `fn is_hidden(self: &Self) -> bool`
  A quick convenience check if the progress bar is hidden

- `fn is_finished(self: &Self) -> bool`
  Indicates that the progress bar finished

- `fn println<I: AsRef<str>>(self: &Self, msg: I)`
  Print a log line above the progress bar

- `fn update(self: &Self, f: impl FnOnce(&mut ProgressState))`
  Update the `ProgressBar`'s inner [`ProgressState`]

- `fn set_position(self: &Self, pos: u64)`
  Sets the position of the progress bar

- `fn unset_length(self: &Self)`
  Sets the length of the progress bar to `None`

- `fn set_length(self: &Self, len: u64)`
  Sets the length of the progress bar

- `fn inc_length(self: &Self, delta: u64)`
  Increase the length of the progress bar

- `fn dec_length(self: &Self, delta: u64)`
  Decrease the length of the progress bar

- `fn set_prefix(self: &Self, prefix: impl Into<Cow<'static, str>>)`
  Sets the current prefix of the progress bar

- `fn set_message(self: &Self, msg: impl Into<Cow<'static, str>>)`
  Sets the current message of the progress bar

- `fn set_elapsed(self: &Self, elapsed: Duration)`
  Sets the elapsed time for the progress bar

- `fn downgrade(self: &Self) -> WeakProgressBar`
  Creates a new weak reference to this [`ProgressBar`]

- `fn reset_eta(self: &Self)`
  Resets the ETA calculation

- `fn reset_elapsed(self: &Self)`
  Resets elapsed time and the ETA calculation

- `fn reset(self: &Self)`
  Resets all of the progress bar state

- `fn finish(self: &Self)`
  Finishes the progress bar and leaves the current message

- `fn finish_with_message(self: &Self, msg: impl Into<Cow<'static, str>>)`
  Finishes the progress bar and sets a message

- `fn finish_and_clear(self: &Self)`
  Finishes the progress bar and completely clears it

- `fn abandon(self: &Self)`
  Finishes the progress bar and leaves the current message and progress

- `fn abandon_with_message(self: &Self, msg: impl Into<Cow<'static, str>>)`
  Finishes the progress bar and sets a message, and leaves the current progress

- `fn finish_using_style(self: &Self)`
  Finishes the progress bar using the behavior stored in the [`ProgressStyle`]

- `fn set_draw_target(self: &Self, target: ProgressDrawTarget)`
  Sets a different draw target for the progress bar

- `fn force_draw(self: &Self)`
  Force a redraw of the progress bar to be in sync with its state

- `fn suspend<F: FnOnce() -> R, R>(self: &Self, f: F) -> R`
  Hide the progress bar temporarily, execute `f`, then redraw the progress bar

- `fn wrap_iter<It: Iterator>(self: &Self, it: It) -> ProgressBarIter<It>`
  Wraps an [`Iterator`] with the progress bar

- `fn wrap_read<R: io::Read>(self: &Self, read: R) -> ProgressBarIter<R>`
  Wraps an [`io::Read`] with the progress bar

- `fn wrap_write<W: io::Write>(self: &Self, write: W) -> ProgressBarIter<W>`
  Wraps an [`io::Write`] with the progress bar

- `fn position(self: &Self) -> u64`
  Returns the current position

- `fn length(self: &Self) -> Option<u64>`
  Returns the current length

- `fn eta(self: &Self) -> Duration`
  Returns the current ETA

- `fn per_sec(self: &Self) -> f64`
  Returns the current rate of progress

- `fn duration(self: &Self) -> Duration`
  Returns the current expected duration

- `fn elapsed(self: &Self) -> Duration`
  Returns the current elapsed time

- `fn message(self: &Self) -> String`
  Current message

- `fn prefix(self: &Self) -> String`
  Current prefix

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ProgressBar`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WeakProgressBar`

```rust
struct WeakProgressBar {
}
```

A weak reference to a [`ProgressBar`](#progressbar).

Useful for creating custom steady tick implementations

#### Implementations

- `fn new() -> Self`
  Create a new [`WeakProgressBar`] that returns `None` when [`upgrade()`] is called.

- `fn upgrade(self: &Self) -> Option<ProgressBar>`
  Attempts to upgrade the Weak pointer to a [`ProgressBar`], delaying dropping of the inner

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> WeakProgressBar`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> WeakProgressBar`

### `ProgressState`

```rust
struct ProgressState {
}
```

The state of a progress bar at a moment in time.

#### Implementations

- `fn is_finished(self: &Self) -> bool`
  Indicates that the progress bar finished.

- `fn fraction(self: &Self) -> f32`
  Returns the completion as a floating-point number between 0 and 1

- `fn eta(self: &Self) -> Duration`
  The expected ETA

- `fn duration(self: &Self) -> Duration`
  The expected total duration (that is, elapsed time + expected ETA)

- `fn per_sec(self: &Self) -> f64`
  The number of steps per second

- `fn elapsed(self: &Self) -> Duration`

- `fn pos(self: &Self) -> u64`

- `fn set_pos(self: &mut Self, pos: u64)`

- `fn len(self: &Self) -> Option<u64>`

- `fn set_len(self: &mut Self, len: u64)`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ProgressStyle`

```rust
struct ProgressStyle {
}
```

#### Implementations

- `fn default_bar() -> Self`
  Returns the default progress bar style for bars

- `fn default_spinner() -> Self`
  Returns the default progress bar style for spinners

- `fn with_template(template: &str) -> Result<Self, TemplateError>`
  Sets the template string for the progress bar

- `fn tick_chars(self: Self, s: &str) -> Self`
  Sets the tick character sequence for spinners

- `fn tick_strings(self: Self, s: &[&str]) -> Self`
  Sets the tick string sequence for spinners

- `fn progress_chars(self: Self, s: &str) -> Self`
  Sets the progress characters `(filled, current, to do)`

- `fn with_key<S: ProgressTracker + 'static>(self: Self, key: &'static str, f: S) -> Self`
  Adds a custom key that owns a [`ProgressTracker`] to the template

- `fn template(self: Self, s: &str) -> Result<Self, TemplateError>`
  Sets the template string for the progress bar

- `fn get_tick_str(self: &Self, idx: u64) -> &str`
  Returns the tick string for a given number

- `fn get_final_tick_str(self: &Self) -> &str`
  Returns the tick string for the finished state

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ProgressStyle`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> MultiProgressAlignment`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> MultiProgressAlignment`

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
[`ProgressBar::is_finished`](#is-finished) is false.




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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ProgressFinish`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ProgressFinish`

## Traits

