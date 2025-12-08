*[tracing_subscriber](../../index.md) / [fmt](../index.md) / [writer](index.md)*

---

# Module `writer`

Abstractions for creating `io::Write` instances.


## Structs

### `TestWriter`

```rust
struct TestWriter {
    use_stderr: bool,
}
```

A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.

`TestWriter` is used by `fmt::Subscriber` or `fmt::Layer` to enable capturing support.

`cargo test` can only capture output from the standard library's `print!` and [`eprint!`](../../../anstream/index.md)
macros. See [`libtest`'s output capturing][capturing] and
[rust-lang/rust#90785](https://github.com/rust-lang/rust/issues/90785) for more details about
output capturing.

Writing to `io::stdout` and `io::stderr` produces the same results as using
[`libtest`'s `--nocapture` option][nocapture] which may make the results look unreadable.








#### Fields

- **`use_stderr`**: `bool`

  Whether or not to use `stderr` instead of the default `stdout` as
  the underlying stream to write to.

#### Implementations

- `fn new() -> Self`

- `fn with_stderr() -> Self`

#### Trait Implementations

##### `impl Debug for TestWriter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for TestWriter`

- `fn default() -> TestWriter` — [`TestWriter`](#testwriter)

##### `impl<T> Instrument for TestWriter`

##### `impl<'a> MakeWriter for TestWriter`

- `type Writer = TestWriter`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for TestWriter`

##### `impl<T> WithSubscriber for TestWriter`

##### `impl Write for TestWriter`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

### `BoxMakeWriter`

```rust
struct BoxMakeWriter {
    inner: alloc::boxed::Box<dyn MakeWriter<'a, Writer = alloc::boxed::Box<dyn Write>> + Send + Sync>,
    name: &'static str,
}
```

A writer that erases the specific `io::Write` and [`MakeWriter`](#makewriter) types being used.

This is useful in cases where the concrete type of the writer cannot be known
until runtime.

# Examples

A function that returns a [`Subscriber`](../index.md) that will write to either stdout or stderr:

```rust
use tracing::Subscriber;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

fn dynamic_writer(use_stderr: bool) -> impl Subscriber {
    let writer = if use_stderr {
        BoxMakeWriter::new(std::io::stderr)
    } else {
        BoxMakeWriter::new(std::io::stdout)
    };

    tracing_subscriber::fmt().with_writer(writer).finish()
}
```



#### Implementations

- `fn new<M>(make_writer: M) -> Self`

#### Trait Implementations

##### `impl Debug for BoxMakeWriter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for BoxMakeWriter`

##### `impl<'a> MakeWriter for BoxMakeWriter`

- `type Writer = Box<dyn Write>`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for BoxMakeWriter`

##### `impl<T> WithSubscriber for BoxMakeWriter`

### `WithMaxLevel<M>`

```rust
struct WithMaxLevel<M> {
    make: M,
    level: tracing_core::Level,
}
```

A [`MakeWriter`](#makewriter) combinator that only returns an enabled [writer](#writer) for spans
and events with metadata at or below a specified verbosity [`Level`](../../index.md).

This is returned by the `MakeWriterExt::with_max_level` method. See the
method documentation for details.



#### Implementations

- `fn new(make: M, level: tracing_core::Level) -> Self`

#### Trait Implementations

##### `impl<M: $crate::clone::Clone> Clone for WithMaxLevel<M>`

- `fn clone(self: &Self) -> WithMaxLevel<M>` — [`WithMaxLevel`](#withmaxlevel)

##### `impl<M: $crate::marker::Copy> Copy for WithMaxLevel<M>`

##### `impl<M: $crate::fmt::Debug> Debug for WithMaxLevel<M>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<M: $crate::cmp::Eq> Eq for WithMaxLevel<M>`

##### `impl<T> Instrument for WithMaxLevel<M>`

##### `impl<'a, M: MakeWriter<'a>> MakeWriter for WithMaxLevel<M>`

- `type Writer = EitherWriter<<M as MakeWriter>::Writer, Sink>`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for WithMaxLevel<M>`

##### `impl<M: $crate::cmp::PartialEq> PartialEq for WithMaxLevel<M>`

- `fn eq(self: &Self, other: &WithMaxLevel<M>) -> bool` — [`WithMaxLevel`](#withmaxlevel)

##### `impl<M> StructuralPartialEq for WithMaxLevel<M>`

##### `impl<T> WithSubscriber for WithMaxLevel<M>`

### `WithMinLevel<M>`

```rust
struct WithMinLevel<M> {
    make: M,
    level: tracing_core::Level,
}
```

A [`MakeWriter`](#makewriter) combinator that only returns an enabled [writer](#writer) for spans
and events with metadata at or above a specified verbosity [`Level`](../../index.md).

This is returned by the `MakeWriterExt::with_min_level` method. See the
method documentation for details.



#### Implementations

- `fn new(make: M, level: tracing_core::Level) -> Self`

#### Trait Implementations

##### `impl<M: $crate::clone::Clone> Clone for WithMinLevel<M>`

- `fn clone(self: &Self) -> WithMinLevel<M>` — [`WithMinLevel`](#withminlevel)

##### `impl<M: $crate::marker::Copy> Copy for WithMinLevel<M>`

##### `impl<M: $crate::fmt::Debug> Debug for WithMinLevel<M>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<M: $crate::cmp::Eq> Eq for WithMinLevel<M>`

##### `impl<T> Instrument for WithMinLevel<M>`

##### `impl<'a, M: MakeWriter<'a>> MakeWriter for WithMinLevel<M>`

- `type Writer = EitherWriter<<M as MakeWriter>::Writer, Sink>`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for WithMinLevel<M>`

##### `impl<M: $crate::cmp::PartialEq> PartialEq for WithMinLevel<M>`

- `fn eq(self: &Self, other: &WithMinLevel<M>) -> bool` — [`WithMinLevel`](#withminlevel)

##### `impl<M> StructuralPartialEq for WithMinLevel<M>`

##### `impl<T> WithSubscriber for WithMinLevel<M>`

### `WithFilter<M, F>`

```rust
struct WithFilter<M, F> {
    make: M,
    filter: F,
}
```

A [`MakeWriter`](#makewriter) combinator that wraps a [`MakeWriter`](#makewriter) with a predicate for
span and event `Metadata`, so that the `MakeWriter::make_writer_for`
method returns [`OptionalWriter::some`][ows] when the predicate returns `true`,
and [`OptionalWriter::none`][own] when the predicate returns `false`.

This is returned by the `MakeWriterExt::with_filter` method. See the
method documentation for details.




#### Implementations

- `fn new(make: M, filter: F) -> Self`

#### Trait Implementations

##### `impl<M: $crate::clone::Clone, F: $crate::clone::Clone> Clone for WithFilter<M, F>`

- `fn clone(self: &Self) -> WithFilter<M, F>` — [`WithFilter`](#withfilter)

##### `impl<M: $crate::marker::Copy, F: $crate::marker::Copy> Copy for WithFilter<M, F>`

##### `impl<M: $crate::fmt::Debug, F: $crate::fmt::Debug> Debug for WithFilter<M, F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<M: $crate::cmp::Eq, F: $crate::cmp::Eq> Eq for WithFilter<M, F>`

##### `impl<T> Instrument for WithFilter<M, F>`

##### `impl<'a, M, F> MakeWriter for WithFilter<M, F>`

- `type Writer = EitherWriter<<M as MakeWriter>::Writer, Sink>`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for WithFilter<M, F>`

##### `impl<M: $crate::cmp::PartialEq, F: $crate::cmp::PartialEq> PartialEq for WithFilter<M, F>`

- `fn eq(self: &Self, other: &WithFilter<M, F>) -> bool` — [`WithFilter`](#withfilter)

##### `impl<M, F> StructuralPartialEq for WithFilter<M, F>`

##### `impl<T> WithSubscriber for WithFilter<M, F>`

### `OrElse<A, B>`

```rust
struct OrElse<A, B> {
    inner: A,
    or_else: B,
}
```

Combines a [`MakeWriter`](#makewriter) that returns an [`OptionalWriter`](#optionalwriter) with another
[`MakeWriter`](#makewriter), so that the second [`MakeWriter`](#makewriter) is used when the first
[`MakeWriter`](#makewriter) returns [`OptionalWriter::none`][own].

This is returned by the [`MakeWriterExt::or_else] method. See the
method documentation for details.


#### Implementations

- `fn new<'a, W>(inner: A, or_else: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for OrElse<A, B>`

- `fn clone(self: &Self) -> OrElse<A, B>` — [`OrElse`](#orelse)

##### `impl<A: $crate::marker::Copy, B: $crate::marker::Copy> Copy for OrElse<A, B>`

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for OrElse<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A: $crate::cmp::Eq, B: $crate::cmp::Eq> Eq for OrElse<A, B>`

##### `impl<T> Instrument for OrElse<A, B>`

##### `impl<'a, A, B, W> MakeWriter for OrElse<A, B>`

- `type Writer = EitherWriter<W, <B as MakeWriter>::Writer>`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for OrElse<A, B>`

##### `impl<A: $crate::cmp::PartialEq, B: $crate::cmp::PartialEq> PartialEq for OrElse<A, B>`

- `fn eq(self: &Self, other: &OrElse<A, B>) -> bool` — [`OrElse`](#orelse)

##### `impl<A, B> StructuralPartialEq for OrElse<A, B>`

##### `impl<T> WithSubscriber for OrElse<A, B>`

### `Tee<A, B>`

```rust
struct Tee<A, B> {
    a: A,
    b: B,
}
```

Combines two types implementing [`MakeWriter`](#makewriter) (or [`std::io::Write`](../../../fs_err/index.md)) to
produce a writer that writes to both [`MakeWriter`](#makewriter)'s returned writers.

This is returned by the `MakeWriterExt::and` method. See the method
documentation for details.

#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for Tee<A, B>`

- `fn clone(self: &Self) -> Tee<A, B>` — [`Tee`](#tee)

##### `impl<A: $crate::marker::Copy, B: $crate::marker::Copy> Copy for Tee<A, B>`

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for Tee<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A: $crate::cmp::Eq, B: $crate::cmp::Eq> Eq for Tee<A, B>`

##### `impl<T> Instrument for Tee<A, B>`

##### `impl<'a, A, B> MakeWriter for Tee<A, B>`

- `type Writer = Tee<<A as MakeWriter>::Writer, <B as MakeWriter>::Writer>`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer` — [`MakeWriter`](#makewriter)

##### `impl<'a, M> MakeWriterExt for Tee<A, B>`

##### `impl<A: $crate::cmp::PartialEq, B: $crate::cmp::PartialEq> PartialEq for Tee<A, B>`

- `fn eq(self: &Self, other: &Tee<A, B>) -> bool` — [`Tee`](#tee)

##### `impl<A, B> StructuralPartialEq for Tee<A, B>`

##### `impl<T> WithSubscriber for Tee<A, B>`

##### `impl<A, B> Write for Tee<A, B>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()>`

- `fn write_fmt(self: &mut Self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>`

### `MutexGuardWriter<'a, W>`

```rust
struct MutexGuardWriter<'a, W>(std::sync::MutexGuard<'a, W>);
```

A type implementing `io::Write` for a `MutexGuard` where the type
inside the `Mutex` implements `io::Write`.

This is used by the [`MakeWriter`](#makewriter) implementation for `Mutex`, because
`MutexGuard` itself will not implement `io::Write` — instead, it
_dereferences_ to a type implementing `io::Write`. Because [`MakeWriter`](#makewriter)
requires the `Writer` type to implement `io::Write`, it's necessary to add
a newtype that forwards the trait implementation.




#### Trait Implementations

##### `impl<'a, W: $crate::fmt::Debug> Debug for MutexGuardWriter<'a, W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for MutexGuardWriter<'a, W>`

##### `impl<T> WithSubscriber for MutexGuardWriter<'a, W>`

##### `impl<W> Write for MutexGuardWriter<'_, W>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()>`

- `fn write_fmt(self: &mut Self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>`

## Enums

### `EitherWriter<A, B>`

```rust
enum EitherWriter<A, B> {
    A(A),
    B(B),
}
```

A [writer](#writer) that is one of two types implementing [`io::Write`](../../../fs_err/index.md).

This may be used by [`MakeWriter`](#makewriter) implementations that may conditionally
return one of two writers.


#### Variants

- **`A`**

  A writer of type `A`.

- **`B`**

  A writer of type `B`.

#### Implementations

- `fn none() -> Self`

- `fn some(t: T) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for EitherWriter<A, B>`

- `fn clone(self: &Self) -> EitherWriter<A, B>` — [`EitherWriter`](#eitherwriter)

##### `impl<A: $crate::marker::Copy, B: $crate::marker::Copy> Copy for EitherWriter<A, B>`

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for EitherWriter<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A: $crate::cmp::Eq, B: $crate::cmp::Eq> Eq for EitherWriter<A, B>`

##### `impl<T> Instrument for EitherWriter<A, B>`

##### `impl<A: $crate::cmp::PartialEq, B: $crate::cmp::PartialEq> PartialEq for EitherWriter<A, B>`

- `fn eq(self: &Self, other: &EitherWriter<A, B>) -> bool` — [`EitherWriter`](#eitherwriter)

##### `impl<A, B> StructuralPartialEq for EitherWriter<A, B>`

##### `impl<T> WithSubscriber for EitherWriter<A, B>`

##### `impl<A, B> Write for EitherWriter<A, B>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()>`

- `fn write_fmt(self: &mut Self, fmt: std::fmt::Arguments<'_>) -> io::Result<()>`

## Traits

### `MakeWriter<'a>`

```rust
trait MakeWriter<'a> { ... }
```

A type that can create `io::Write` instances.

`MakeWriter` is used by `fmt::Layer` or `fmt::Subscriber` to print
formatted text representations of `Event`s.

This trait is already implemented for function pointers and
immutably-borrowing closures that return an instance of `io::Write`, such
as `io::stdout` and `io::stderr`. Additionally, it is implemented for
[`std::sync::Mutex`](../../../serde_core/lib/index.md) when the type inside the mutex implements
`io::Write`.

# Examples

The simplest usage is to pass in a named function that returns a writer. For
example, to log all events to stderr, we could write:
```rust
let subscriber = tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .finish();
drop(subscriber);
```

Any function that returns a writer can be used:

```rust
fn make_my_great_writer() -> impl std::io::Write {
    // ...
    std::io::stdout()
}

let subscriber = tracing_subscriber::fmt()
    .with_writer(make_my_great_writer)
    .finish();
drop(subscriber);
```

A closure can be used to introduce arbitrary logic into how the writer is
created. Consider the (admittedly rather silly) example of sending every 5th
event to stderr, and all other events to stdout:

```rust
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

let n = AtomicUsize::new(0);
let subscriber = tracing_subscriber::fmt()
    .with_writer(move || -> Box<dyn io::Write> {
        if n.fetch_add(1, Relaxed) % 5 == 0 {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
       }
    })
    .finish();
drop(subscriber);
```

A single instance of a type implementing `io::Write` may be used as a
`MakeWriter` by wrapping it in a [`Mutex`](../../../serde_core/lib/index.md). For example, we could
write to a file like so:

```rust
use std::{fs::File, sync::Mutex};

fn docs() -> Result<(), Box<dyn std::error::Error>> {
let log_file = File::create("my_cool_trace.log")?;
let subscriber = tracing_subscriber::fmt()
    .with_writer(Mutex::new(log_file))
    .finish();
drop(subscriber);
Ok(())
}
```











#### Required Methods

- `type Writer: 1`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer`

  Returns an instance of [`Writer`](../format/index.md).

- `fn make_writer_for(self: &'a Self, meta: &Metadata<'_>) -> <Self as >::Writer`

  Returns a [`Writer`](../format/index.md) for writing data from the span or event described

### `MakeWriterExt<'a>`

```rust
trait MakeWriterExt<'a>: MakeWriter<'a> { ... }
```

Extension trait adding combinators for working with types implementing
[`MakeWriter`](#makewriter).

This is not intended to be implemented directly for user-defined
[`MakeWriter`](#makewriter)s; instead, it should be imported when the desired methods are
used.

#### Required Methods

- `fn with_max_level(self: Self, level: tracing_core::Level) -> WithMaxLevel<Self>`

  Wraps `self` and returns a [`MakeWriter`](#makewriter) that will only write output

- `fn with_min_level(self: Self, level: tracing_core::Level) -> WithMinLevel<Self>`

  Wraps `self` and returns a [`MakeWriter`](#makewriter) that will only write output

- `fn with_filter<F>(self: Self, filter: F) -> WithFilter<Self, F>`

  Wraps `self` with a predicate that takes a span or event's `Metadata`

- `fn and<B>(self: Self, other: B) -> Tee<Self, B>`

  Combines `self` with another type implementing [`MakeWriter`](#makewriter), returning

- `fn or_else<W, B>(self: Self, other: B) -> OrElse<Self, B>`

  Combines `self` with another type implementing [`MakeWriter`](#makewriter), returning

## Type Aliases

### `OptionalWriter<T>`

```rust
type OptionalWriter<T> = EitherWriter<T, std::io::Sink>;
```

A [writer](#writer) which may or may not be enabled.

This may be used by [`MakeWriter`](#makewriter) implementations that wish to
conditionally enable or disable the returned writer based on a span or
event's [`Metadata`](../../../tracing_core/metadata/index.md).


