# Crate `anstream`

**Auto-adapting [`stdout`](#stdout) / [`stderr`](#stderr) streams**

*A portmanteau of "ansi stream"*

[`AutoStream`](auto/index.md) always accepts [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
[adapting to the user's terminal's capabilities][AutoStream].

Benefits
- Allows the caller to not be concerned with the terminal's capabilities
- Semver safe way of passing styled text between crates as ANSI escape codes offer more
  compatibility than most crate APIs.

Available styling crates:
- [anstyle](https://docs.rs/anstyle) for minimal runtime styling, designed to go in public APIs
- [owo-colors](https://docs.rs/owo-colors) for feature-rich runtime styling
- [color-print](https://docs.rs/color-print) for feature-rich compile-time styling

# Example

```rust
 #[cfg(feature = "auto")] {
use anstream::println;
use owo_colors::OwoColorize as _;

// Foreground colors
println!("My number is {:#x}!", 10.green());
// Background colors
println!("My number is not {}!", 4.on_red());
}
```

And this will correctly handle piping to a file, etc

## Contents

- [Modules](#modules)
  - [`adapter`](#adapter)
  - [`stream`](#stream)
  - [`auto`](#auto)
  - [`buffer`](#buffer)
  - [`fmt`](#fmt)
  - [`strip`](#strip)
- [Structs](#structs)
  - [`AutoStream`](#autostream)
  - [`StripStream`](#stripstream)
- [Functions](#functions)
  - [`stdout`](#stdout)
  - [`stderr`](#stderr)
- [Type Aliases](#type-aliases)
  - [`Stdout`](#stdout)
  - [`Stderr`](#stderr)
- [Macros](#macros)
  - [`print!`](#print)
  - [`println!`](#println)
  - [`eprint!`](#eprint)
  - [`eprintln!`](#eprintln)
  - [`panic!`](#panic)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`adapter`](#adapter) | mod | Gracefully degrade styled output |
| [`stream`](#stream) | mod | Higher-level traits to describe writeable streams |
| [`auto`](#auto) | mod |  |
| [`buffer`](#buffer) | mod |  |
| [`fmt`](#fmt) | mod |  |
| [`strip`](#strip) | mod |  |
| [`AutoStream`](#autostream) | struct |  |
| [`StripStream`](#stripstream) | struct |  |
| [`stdout`](#stdout) | fn | Create an ANSI escape code compatible stdout |
| [`stderr`](#stderr) | fn | Create an ANSI escape code compatible stderr |
| [`Stdout`](#stdout) | type | An adaptive wrapper around the global standard output stream of the current process |
| [`Stderr`](#stderr) | type | An adaptive wrapper around the global standard error stream of the current process |
| [`print!`](#print) | macro | Prints to [`stdout`][crate::stdout]. |
| [`println!`](#println) | macro | Prints to [`stdout`][crate::stdout], with a newline. |
| [`eprint!`](#eprint) | macro | Prints to [`stderr`][crate::stderr]. |
| [`eprintln!`](#eprintln) | macro | Prints to [`stderr`][crate::stderr], with a newline. |
| [`panic!`](#panic) | macro | Panics the current thread. |

## Modules

- [`adapter`](adapter/index.md) — Gracefully degrade styled output
- [`stream`](stream/index.md) — Higher-level traits to describe writeable streams
- [`auto`](auto/index.md)
- [`buffer`](buffer/index.md)
- [`fmt`](fmt/index.md)
- [`strip`](strip/index.md)

## Structs

### `AutoStream<S: RawStream>`

```rust
struct AutoStream<S: RawStream> {
    inner: StreamInner<S>,
}
```

*Defined in [`anstream-0.6.21/src/auto.rs:19-21`](../../.source_1765633015/anstream-0.6.21/src/auto.rs#L19-L21)*

[`std::io::Write`](../fs_err/index.md) that adapts ANSI escape codes to the underlying `Write`s capabilities

This includes
- Stripping colors for non-terminals
- Respecting env variables like [NO_COLOR](https://no-color.org/) or [CLICOLOR](https://bixense.com/clicolors/)
- *(windows)* Falling back to the wincon API where [ENABLE_VIRTUAL_TERMINAL_PROCESSING](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences) is unsupported

You can customize auto-detection by calling into
[anstyle_query](https://docs.rs/anstyle-query/latest/anstyle_query/)
to get a [`ColorChoice`](#colorchoice) and then calling `AutoStream::new(stream, choice)`.

#### Implementations

- <span id="autostream-new"></span>`fn new(raw: S, choice: ColorChoice) -> Self` — [`ColorChoice`](#colorchoice)

  Runtime control over styling behavior

  

  # Example

  

  ```rust

  #[cfg(feature = "auto")] {

  use std::io::IsTerminal as _;

  // Like `AutoStream::choice` but without `NO_COLOR`, `CLICOLOR_FORCE`, `CI`

  fn choice(raw: &dyn anstream::stream::RawStream) -> anstream::ColorChoice {

      let choice = anstream::ColorChoice::global();

      if choice == anstream::ColorChoice::Auto {

          if raw.is_terminal() && anstyle_query::term_supports_color() {

              anstream::ColorChoice::Always

          } else {

              anstream::ColorChoice::Never

          }

      } else {

          choice

      }

  }

  

  let stream = std::io::stdout();

  let choice = choice(&stream);

  let auto = anstream::AutoStream::new(stream, choice);

  }

  ```

- <span id="autostream-auto"></span>`fn auto(raw: S) -> Self`

  Auto-adapt for the stream's capabilities

- <span id="autostream-choice"></span>`fn choice(raw: &S) -> ColorChoice` — [`ColorChoice`](#colorchoice)

  Report the desired choice for the given stream

- <span id="autostream-always-ansi"></span>`fn always_ansi(raw: S) -> Self`

  Force ANSI escape codes to be passed through as-is, no matter what the inner `Write`

  supports.

- <span id="autostream-always-ansi"></span>`fn always_ansi_(raw: S) -> Self`

- <span id="autostream-always"></span>`fn always(raw: S) -> Self`

  Force color, no matter what the inner `Write` supports.

- <span id="autostream-never"></span>`fn never(raw: S) -> Self`

  Only pass printable data to the inner `Write`.

- <span id="autostream-wincon"></span>`fn wincon(raw: S) -> Result<Self, S>`

- <span id="autostream-into-inner"></span>`fn into_inner(self) -> S`

  Get the wrapped [`RawStream`](stream/index.md)

- <span id="autostream-as-inner"></span>`fn as_inner(&self) -> &S`

  Get the wrapped [`RawStream`](stream/index.md)

- <span id="autostream-is-terminal"></span>`fn is_terminal(&self) -> bool`

  Returns `true` if the descriptor/handle refers to a terminal/tty.

- <span id="autostream-current-choice"></span>`fn current_choice(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

  Prefer `AutoStream::choice`

  

  This doesn't report what is requested but what is currently active.

#### Trait Implementations

##### `impl Any for AutoStream<S>`

- <span id="autostream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AutoStream<S>`

- <span id="autostream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AutoStream<S>`

- <span id="autostream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug + RawStream> Debug for AutoStream<S>`

- <span id="autostream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AutoStream<S>`

- <span id="autostream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AutoStream<S>`

- <span id="autostream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AutoStream<S>`

- <span id="autostream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="autostream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AutoStream<S>`

- <span id="autostream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="autostream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S> Write for AutoStream<S>`

- <span id="autostream-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="autostream-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="autostream-write-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

- <span id="autostream-write-write-all"></span>`fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>`

- <span id="autostream-write-write-fmt"></span>`fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

### `StripStream<S>`

```rust
struct StripStream<S>
where
    S: std::io::Write {
    raw: S,
    state: crate::adapter::StripBytes,
}
```

*Defined in [`anstream-0.6.21/src/strip.rs:7-13`](../../.source_1765633015/anstream-0.6.21/src/strip.rs#L7-L13)*

Only pass printable data to the inner `Write`

#### Implementations

- <span id="stripstream-new"></span>`fn new(raw: S) -> Self`

  Only pass printable data to the inner `Write`

- <span id="stripstream-into-inner"></span>`fn into_inner(self) -> S`

  Get the wrapped [`std::io::Write`](../fs_err/index.md)

- <span id="stripstream-as-inner"></span>`fn as_inner(&self) -> &S`

  Get the wrapped [`std::io::Write`](../fs_err/index.md)

#### Trait Implementations

##### `impl Any for StripStream<S>`

- <span id="stripstream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StripStream<S>`

- <span id="stripstream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StripStream<S>`

- <span id="stripstream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S> Debug for StripStream<S>`

- <span id="stripstream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StripStream<S>`

- <span id="stripstream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StripStream<S>`

- <span id="stripstream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StripStream<S>`

- <span id="stripstream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stripstream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StripStream<S>`

- <span id="stripstream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stripstream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S> Write for StripStream<S>`

- <span id="stripstream-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="stripstream-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="stripstream-write-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

- <span id="stripstream-write-write-all"></span>`fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>`

- <span id="stripstream-write-write-fmt"></span>`fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

## Functions

### `stdout`

```rust
fn stdout() -> Stdout
```

*Defined in [`anstream-0.6.21/src/lib.rs:70-73`](../../.source_1765633015/anstream-0.6.21/src/lib.rs#L70-L73)*

Create an ANSI escape code compatible stdout

**Note:** Call `AutoStream::lock` in loops to avoid the performance hit of acquiring/releasing
from the implicit locking in each [`std::io::Write`](../fs_err/index.md) call

### `stderr`

```rust
fn stderr() -> Stderr
```

*Defined in [`anstream-0.6.21/src/lib.rs:80-83`](../../.source_1765633015/anstream-0.6.21/src/lib.rs#L80-L83)*

Create an ANSI escape code compatible stderr

**Note:** Call `AutoStream::lock` in loops to avoid the performance hit of acquiring/releasing
from the implicit locking in each [`std::io::Write`](../fs_err/index.md) call

## Type Aliases

### `Stdout`

```rust
type Stdout = AutoStream<std::io::Stdout>;
```

*Defined in [`anstream-0.6.21/src/lib.rs:61`](../../.source_1765633015/anstream-0.6.21/src/lib.rs#L61)*

An adaptive wrapper around the global standard output stream of the current process

### `Stderr`

```rust
type Stderr = AutoStream<std::io::Stderr>;
```

*Defined in [`anstream-0.6.21/src/lib.rs:63`](../../.source_1765633015/anstream-0.6.21/src/lib.rs#L63)*

An adaptive wrapper around the global standard error stream of the current process

## Macros

### `print!`

*Defined in [`anstream-0.6.21/src/_macros.rs:61-79`](../../.source_1765633015/anstream-0.6.21/src/_macros.rs#L61-L79)*

Prints to `stdout`.

Equivalent to the [`println!`](#println) macro except that a newline is not printed at
the end of the message.

Note that stdout is frequently line-buffered by default so it may be
necessary to use `std::io::Write::flush()` to ensure the output is emitted
immediately.

**NOTE:** The `print!` macro will lock the standard output on each call. If you call
`print!` within a hot loop, this behavior may be the bottleneck of the loop.
To avoid this, lock stdout with `AutoStream::lock`:
```rust
 #[cfg(feature = "auto")] {
use std::io::Write as _;

let mut lock = anstream::stdout().lock();
write!(lock, "hello world").unwrap();
}
```

Use `print!` only for the primary output of your program. Use
[`eprint!`](#eprint) instead to print error and progress messages.

**NOTE:** Not all `print!` calls will be captured in tests like [`std::print!`](../backtrace/print/index.md)
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stdout` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```rust
 #[cfg(feature = "auto")] {
use std::io::Write as _;
use anstream::print;
use anstream::stdout;

print!("this ");
print!("will ");
print!("be ");
print!("on ");
print!("the ");
print!("same ");
print!("line ");

stdout().flush().unwrap();

print!("this string has a newline, why not choose println! instead?\n");

stdout().flush().unwrap();
}
```

### `println!`

*Defined in [`anstream-0.6.21/src/_macros.rs:130-151`](../../.source_1765633015/anstream-0.6.21/src/_macros.rs#L130-L151)*

Prints to `stdout`, with a newline.

On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
(no additional CARRIAGE RETURN (`\r`/`U+000D`)).

This macro uses the same syntax as [`format!`](../clap_builder/error/format/index.md), but writes to the standard output instead.
See [`std::fmt`](fmt/index.md) for more information.

**NOTE:** The `println!` macro will lock the standard output on each call. If you call
`println!` within a hot loop, this behavior may be the bottleneck of the loop.
To avoid this, lock stdout with `AutoStream::lock`:
```rust
 #[cfg(feature = "auto")] {
use std::io::Write as _;

let mut lock = anstream::stdout().lock();
writeln!(lock, "hello world").unwrap();
}
```

Use `println!` only for the primary output of your program. Use
[`eprintln!`](#eprintln) instead to print error and progress messages.

**NOTE:** Not all `println!` calls will be captured in tests like [`std::println!`](#stdprintln)
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stdout` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```rust
 #[cfg(feature = "auto")] {
use anstream::println;

println!(); // prints just a newline
println!("hello there!");
println!("format {} arguments", "some");
let local_variable = "some";
println!("format {local_variable} arguments");
}
```

### `eprint!`

*Defined in [`anstream-0.6.21/src/_macros.rs:184-202`](../../.source_1765633015/anstream-0.6.21/src/_macros.rs#L184-L202)*

Prints to `stderr`.

Equivalent to the [`print!`](#print) macro, except that output goes to
`stderr` instead of `stdout`. See [`print!`](#print) for
example usage.

Use `eprint!` only for error and progress messages. Use `print!`
instead for the primary output of your program.

**NOTE:** Not all `eprint!` calls will be captured in tests like [`std::eprint!`](#stdeprint)
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stderr` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```rust
 #[cfg(feature = "auto")] {
use anstream::eprint;

eprint!("Error: Could not complete task");
}
```

### `eprintln!`

*Defined in [`anstream-0.6.21/src/_macros.rs:235-256`](../../.source_1765633015/anstream-0.6.21/src/_macros.rs#L235-L256)*

Prints to `stderr`, with a newline.

Equivalent to the [`println!`](#println) macro, except that output goes to
`stderr` instead of `stdout`. See [`println!`](#println) for
example usage.

Use `eprintln!` only for error and progress messages. Use `println!`
instead for the primary output of your program.

**NOTE:** Not all `eprintln!` calls will be captured in tests like [`std::eprintln!`](#stdeprintln)
- Capturing will automatically be activated in test binaries
- Otherwise, only when the `test` feature is enabled

# Panics

Panics if writing to `stderr` fails for any reason **except** broken pipe.

Writing to non-blocking stdout can cause an error, which will lead
this macro to panic.

# Examples

```rust
 #[cfg(feature = "auto")] {
use anstream::eprintln;

eprintln!("Error: Could not complete task");
}
```

### `panic!`

*Defined in [`anstream-0.6.21/src/_macros.rs:334-343`](../../.source_1765633015/anstream-0.6.21/src/_macros.rs#L334-L343)*

Panics the current thread.

This allows a program to terminate immediately and provide feedback
to the caller of the program.

This macro is the perfect way to assert conditions in example code and in
tests. `panic!` is closely tied with the `unwrap` method of both
[`Option`][ounwrap] and [`Result`][runwrap] enums. Both implementations call
`panic!` when they are set to [`None`](#none) or `Err` variants.

When using `panic!()` you can specify a string payload, that is built using
the `format!` syntax. That payload is used when injecting the panic into
the calling Rust thread, causing the thread to panic entirely.

The behavior of the default `std` hook, i.e. the code that runs directly
after the panic is invoked, is to print the message payload to
`stderr` along with the file/line/column information of the `panic!()`
call. You can override the panic hook using `std::panic::set_hook()`.
Inside the hook a panic can be accessed as a `&dyn Any + Send`,
which contains either a `&str` or `String` for regular `panic!()` invocations.
To panic with a value of another other type, `panic_any` can be used.

See also the macro `compile_error!`, for raising errors during compilation.

# When to use `panic!` vs `Result`

The Rust language provides two complementary systems for constructing /
representing, reporting, propagating, reacting to, and discarding errors. These
responsibilities are collectively known as "error handling." `panic!` and
`Result` are similar in that they are each the primary interface of their
respective error handling systems; however, the meaning these interfaces attach
to their errors and the responsibilities they fulfill within their respective
error handling systems differ.

The `panic!` macro is used to construct errors that represent a bug that has
been detected in your program. With `panic!` you provide a message that
describes the bug and the language then constructs an error with that message,
reports it, and propagates it for you.

`Result` on the other hand is used to wrap other types that represent either
the successful result of some computation, `Ok(T)`, or error types that
represent an anticipated runtime failure mode of that computation, `Err(E)`.
`Result` is used alongside user defined types which represent the various
anticipated runtime failure modes that the associated computation could
encounter. `Result` must be propagated manually, often with the help of the
`?` operator and `Try` trait, and they must be reported manually, often with
the help of the `Error` trait.

For more detailed information about error handling check out the [book] or the
`std::result` module docs.








# Current implementation

If the main thread panics it will terminate all your threads and end your
program with code `101`.

# Examples

```should_panic
#![allow(unreachable_code)]
use anstream::panic;
panic!();
panic!("this is a terrible mistake!");
panic!("this is a {} {message}", "fancy", message = "message");
```

