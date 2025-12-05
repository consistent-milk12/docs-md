# Crate `anstream`

**Auto-adapting [`stdout`](#stdout) / [`stderr`](#stderr) streams**

*A portmanteau of "ansi stream"*

[`AutoStream`](#autostream) always accepts [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
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

## Modules

- [`adapter`](adapter/index.md) - Gracefully degrade styled output
- [`stream`](stream/index.md) - Higher-level traits to describe writeable streams

## Structs

### `AutoStream<S: RawStream>`

```rust
struct AutoStream<S: RawStream> {
    // [REDACTED: Private Fields]
}
```

`std::io::Write` that adapts ANSI escape codes to the underlying `Write`s capabilities

This includes
- Stripping colors for non-terminals
- Respecting env variables like [NO_COLOR](https://no-color.org/) or [CLICOLOR](https://bixense.com/clicolors/)
- *(windows)* Falling back to the wincon API where [ENABLE_VIRTUAL_TERMINAL_PROCESSING](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences) is unsupported

You can customize auto-detection by calling into
[anstyle_query](https://docs.rs/anstyle-query/latest/anstyle_query/)
to get a [`ColorChoice`](#colorchoice) and then calling `AutoStream::new(stream, choice)`.

#### Implementations

- `fn new(raw: S, choice: ColorChoice) -> Self`
  Runtime control over styling behavior

- `fn auto(raw: S) -> Self`
  Auto-adapt for the stream's capabilities

- `fn choice(raw: &S) -> ColorChoice`
  Report the desired choice for the given stream

- `fn always_ansi(raw: S) -> Self`
  Force ANSI escape codes to be passed through as-is, no matter what the inner `Write`

- `fn always(raw: S) -> Self`
  Force color, no matter what the inner `Write` supports.

- `fn never(raw: S) -> Self`
  Only pass printable data to the inner `Write`.

- `fn into_inner(self: Self) -> S`
  Get the wrapped [`RawStream`]

- `fn as_inner(self: &Self) -> &S`
  Get the wrapped [`RawStream`]

- `fn is_terminal(self: &Self) -> bool`
  Returns `true` if the descriptor/handle refers to a terminal/tty.

- `fn current_choice(self: &Self) -> ColorChoice`
  Prefer [`AutoStream::choice`]

- `fn lock(self: Self) -> AutoStream<std::io::StderrLock<'static>>`
  Get exclusive access to the `AutoStream`

- `fn lock(self: Self) -> AutoStream<std::io::StdoutLock<'static>>`
  Get exclusive access to the `AutoStream`

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

##### `impl Write<S>`

- `fn write(self: &mut Self, buf: &[u8]) -> std::io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- `fn flush(self: &mut Self) -> std::io::Result<()>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> std::io::Result<()>`

- `fn write_fmt(self: &mut Self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

##### `impl Debug<S: $crate::fmt::Debug + RawStream>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StripStream<S>`

```rust
struct StripStream<S>
where
    S: std::io::Write {
    // [REDACTED: Private Fields]
}
```

Only pass printable data to the inner `Write`

#### Implementations

- `fn lock(self: Self) -> StripStream<std::io::StdoutLock<'static>>`
  Get exclusive access to the `StripStream`

- `fn is_terminal(self: &Self) -> bool`
  Returns `true` if the descriptor/handle refers to a terminal/tty.

- `fn new(raw: S) -> Self`
  Only pass printable data to the inner `Write`

- `fn into_inner(self: Self) -> S`
  Get the wrapped [`std::io::Write`]

- `fn as_inner(self: &Self) -> &S`
  Get the wrapped [`std::io::Write`]

- `fn lock(self: Self) -> StripStream<std::io::StderrLock<'static>>`
  Get exclusive access to the `StripStream`

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

##### `impl Write<S>`

- `fn write(self: &mut Self, buf: &[u8]) -> std::io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- `fn flush(self: &mut Self) -> std::io::Result<()>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> std::io::Result<()>`

- `fn write_fmt(self: &mut Self, args: std::fmt::Arguments<'_>) -> std::io::Result<()>`

##### `impl Debug<S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `stdout`

```rust
fn stdout() -> Stdout
```

Create an ANSI escape code compatible stdout

**Note:** Call `AutoStream::lock` in loops to avoid the performance hit of acquiring/releasing
from the implicit locking in each `std::io::Write` call

### `stderr`

```rust
fn stderr() -> Stderr
```

Create an ANSI escape code compatible stderr

**Note:** Call `AutoStream::lock` in loops to avoid the performance hit of acquiring/releasing
from the implicit locking in each `std::io::Write` call

## Type Aliases

### `Stdout`

```rust
type Stdout = AutoStream<std::io::Stdout>;
```

An adaptive wrapper around the global standard output stream of the current process

### `Stderr`

```rust
type Stderr = AutoStream<std::io::Stderr>;
```

An adaptive wrapper around the global standard error stream of the current process

## Macros

### `print!`

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

**NOTE:** Not all `print!` calls will be captured in tests like `std::print!`
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

Prints to `stdout`, with a newline.

On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
(no additional CARRIAGE RETURN (`\r`/`U+000D`)).

This macro uses the same syntax as [`format!`](#format), but writes to the standard output instead.
See `std::fmt` for more information.

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

**NOTE:** Not all `println!` calls will be captured in tests like `std::println!`
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

Prints to `stderr`.

Equivalent to the [`print!`](#print) macro, except that output goes to
`stderr` instead of `stdout`. See [`print!`](#print) for
example usage.

Use `eprint!` only for error and progress messages. Use `print!`
instead for the primary output of your program.

**NOTE:** Not all `eprint!` calls will be captured in tests like `std::eprint!`
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

Prints to `stderr`, with a newline.

Equivalent to the [`println!`](#println) macro, except that output goes to
`stderr` instead of `stdout`. See [`println!`](#println) for
example usage.

Use `eprintln!` only for error and progress messages. Use `println!`
instead for the primary output of your program.

**NOTE:** Not all `eprintln!` calls will be captured in tests like `std::eprintln!`
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

Panics the current thread.

This allows a program to terminate immediately and provide feedback
to the caller of the program.

This macro is the perfect way to assert conditions in example code and in
tests. `panic!` is closely tied with the `unwrap` method of both
[`Option`][ounwrap](#ounwrap) and [`Result`][runwrap](#runwrap) enums. Both implementations call
`panic!` when they are set to [`None`](#none) or [`Err`](#err) variants.

When using `panic!()` you can specify a string payload, that is built using
the [`format!`](#format) syntax. That payload is used when injecting the panic into
the calling Rust thread, causing the thread to panic entirely.

The behavior of the default `std` hook, i.e. the code that runs directly
after the panic is invoked, is to print the message payload to
`stderr` along with the file/line/column information of the `panic!()`
call. You can override the panic hook using `std::panic::set_hook()`.
Inside the hook a panic can be accessed as a `&dyn Any + Send`,
which contains either a `&str` or `String` for regular `panic!()` invocations.
To panic with a value of another other type, [`panic_any`](#panic-any) can be used.

See also the macro [`compile_error!`](#compile-error), for raising errors during compilation.

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

For more detailed information about error handling check out the [book](#book) or the
`std::result` module docs.

[ounwrap](#ounwrap): Option::unwrap
[runwrap](#runwrap): Result::unwrap




[book](#book): ../book/ch09-00-error-handling.html

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

