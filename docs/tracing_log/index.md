# Crate `tracing_log`

Adapters for connecting unstructured log records from the `log` crate into
the `tracing` ecosystem.

# Overview

[`tracing`](#tracing) is a framework for instrumenting Rust programs with context-aware,
structured, event-based diagnostic information. This crate provides
compatibility layers for using `tracing` alongside the logging facade provided
by the [`log`](#log) crate.

This crate provides:

- [`AsTrace`](#astrace) and [`AsLog`](#aslog) traits for converting between `tracing` and `log` types.
- [`LogTracer`](log_tracer/index.md), a `log::Log` implementation that consumes `log::Record`s
  and outputs them as `tracing::Event`.

*Compiler support: [requires `rustc` 1.56+][msrv]*

# Usage

## Convert log records to tracing `Event`s

To convert `log::Record`s as `tracing::Event`s, set `LogTracer` as the default
logger by calling its `init` or `init_with_filter` methods.

```rust
use std::error::Error;
use tracing_log::LogTracer;
use log;

fn main() -> Result<(), Box<Error>> {
LogTracer::init()?;

// will be available for Subscribers as a tracing Event
log::trace!("an example trace log");
Ok(())
}
```

This conversion does not convert unstructured data in log records (such as
values passed as format arguments to the `log!` macro) to structured
`tracing` fields. However, it *does* attach these new events to to the
span that was currently executing when the record was logged. This is the
primary use-case for this library: making it possible to locate the log
records emitted by dependencies which use `log` within the context of a
trace.

## Convert tracing `Event`s to logs

Enabling the ["log" and "log-always" feature flags][flags] on the `tracing`
crate will cause all `tracing` spans and events to emit `log::Record`s as
they occur.

## Caution: Mixing both conversions

Note that `log::Logger` implementations that convert log records to trace events
should not be used with `Subscriber`s that convert trace events _back_ into
`log` records, as doing so will result in the event recursing between the subscriber
and the logger forever (or, in real life, probably overflowing the call stack).

If the logging of trace events generated from log records produced by the
`log` crate is desired, either the `log` crate should not be used to
implement this logging, or an additional layer of filtering will be
required to avoid infinitely converting between `Event` and `log::Record`.

## Feature Flags

* `std`: enables features that require the Rust standard library (on by default)
* `log-tracer`: enables the `LogTracer` type (on by default)
* `interest-cache`: makes it possible to configure an interest cache for
  logs emitted through the `log` crate (see `Builder::with_interest_cache`); requires `std`

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.56. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.









## Modules

- [`log_tracer`](log_tracer/index.md) - An adapter for converting [`log`] records into `tracing` `Event`s.

## Structs

### `LogTracer`

```rust
struct LogTracer {
    ignore_crates: Box<[String]>,
}
```

A simple "logger" that converts all log records into `tracing` `Event`s.

#### Implementations

- `fn builder() -> Builder` — [`Builder`](log_tracer/index.md)

- `fn new() -> Self`

- `fn init_with_filter(level: log::LevelFilter) -> Result<(), SetLoggerError>`

- `fn init() -> Result<(), SetLoggerError>`

#### Trait Implementations

##### `impl Debug for LogTracer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LogTracer`

- `fn default() -> Self`

##### `impl Log for LogTracer`

- `fn enabled(self: &Self, metadata: &log::Metadata<'_>) -> bool`

- `fn log(self: &Self, record: &log::Record<'_>)`

- `fn flush(self: &Self)`

## Traits

### `AsLog`

```rust
trait AsLog: crate::sealed::Sealed { ... }
```

Trait implemented for `tracing` types that can be converted to a `log`
equivalent.

#### Required Methods

- `type Log`

- `fn as_log(self: &Self) -> <Self as >::Log`

  Returns the `log` equivalent of `self`.

### `AsTrace`

```rust
trait AsTrace: crate::sealed::Sealed { ... }
```

Trait implemented for `log` types that can be converted to a `tracing`
equivalent.

#### Required Methods

- `type Trace`

- `fn as_trace(self: &Self) -> <Self as >::Trace`

  Returns the `tracing` equivalent of `self`.

### `NormalizeEvent<'a>`

```rust
trait NormalizeEvent<'a>: crate::sealed::Sealed { ... }
```

Extends log `Event`s to provide complete `Metadata`.

In `tracing-log`, an `Event` produced by a log (through [`AsTrace`](#astrace)) has an hard coded
"log" target and no `file`, `line`, or `module_path` attributes. This happens because `Event`
requires its `Metadata` to be `'static`, while `log::Record`s provide them with a generic
lifetime.

However, these values are stored in the `Event`'s fields and
the `normalized_metadata` method allows to build a new `Metadata`
that only lives as long as its source `Event`, but provides complete
data.

It can typically be used by `Subscriber`s when processing an `Event`,
to allow accessing its complete metadata in a consistent way,
regardless of the source of its source.


#### Required Methods

- `fn normalized_metadata(self: &'a Self) -> Option<Metadata<'a>>`

  If this `Event` comes from a `log`, this method provides a new

- `fn is_log(self: &Self) -> bool`

  Returns whether this `Event` represents a log (from the `log` crate)

## Functions

### `format_trace`

```rust
fn format_trace(record: &log::Record<'_>) -> io::Result<()>
```

Format a log record as a trace event in the current span.

