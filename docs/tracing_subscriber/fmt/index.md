*[tracing_subscriber](../index.md) / [fmt](index.md)*

---

# Module `fmt`

A `Subscriber` for formatting and logging `tracing` data.

# Overview

`tracing` is a framework for instrumenting Rust programs with context-aware,
structured, event-based diagnostic information. This crate provides an
implementation of the [`Subscriber`](#subscriber) trait that records `tracing`'s `Event`s
and `Span`s by formatting them as text and logging them to stdout.

# Usage

First, add this to your `Cargo.toml` file:

```toml
[dependencies]
tracing-subscriber = "0.3"
```

*Compiler support: [requires `rustc` 1.65+][msrv]*

Add the following to your executable to initialize the default subscriber:
```rust
use tracing_subscriber;

tracing_subscriber::fmt::init();
```

## Filtering Events with Environment Variables

The default subscriber installed by `init` enables you to filter events
at runtime using environment variables (using the [`EnvFilter`](../filter/index.md)).

The filter syntax is a superset of the `env_logger` syntax.

For example:
- Setting `RUST_LOG=debug` enables all `Span`s and `Event`s
  set to the log level `DEBUG` or higher
- Setting `RUST_LOG=my_crate=trace` enables `Span`s and `Event`s
  in `my_crate` at all log levels

**Note**: This should **not** be called by libraries. Libraries should use
`tracing` to publish `tracing` `Event`s.

# Configuration

You can configure a subscriber instead of using the defaults with
the following functions:

### Subscriber

The [`FmtSubscriber`](../index.md) formats and records `tracing` events as line-oriented logs.
You can create one by calling:

```rust
let subscriber = tracing_subscriber::fmt()
    // ... add configuration
    .finish();
```

You can find the configuration methods for [`FmtSubscriber`](../index.md) in
[`SubscriberBuilder`](#subscriberbuilder).

## Formatters

The output format used by the layer and subscriber in this module is
represented by implementing the [`FormatEvent`](format/index.md) trait, and can be
customized. This module provides a number of formatter implementations:

* `format::Full`: The default formatter. This emits human-readable,
  single-line logs for each event that occurs, with the current span context
  displayed before the formatted representation of the event. See
  [here](format::Full#example-output) for sample output.

* `format::Compact`: A variant of the default formatter, optimized for
  short line lengths. Fields from the current span context are appended to
  the fields of the formatted event. See
  [here](format::Compact#example-output) for sample output.

* `format::Pretty`: Emits excessively pretty, multi-line logs, optimized
  for human readability. This is primarily intended to be used in local
  development and debugging, or for command-line applications, where
  automated analysis and compact storage of logs is less of a priority than
  readability and visual appeal. See [here](format::Pretty#example-output)
  for sample output.

* `format::Json`: Outputs newline-delimited JSON logs. This is intended
  for production use with systems where structured logs are consumed as JSON
  by analysis and viewing tools. The JSON output is not optimized for human
  readability. See [here](format::Json#example-output) for sample output.

### Customizing Formatters

The formatting of log lines for spans and events is controlled by two
traits, [`FormatEvent`](format/index.md) and [`FormatFields`](format/index.md). The [`FormatEvent`](format/index.md) trait
determines the overall formatting of the log line, such as what information
from the event's metadata and span context is included and in what order.
The [`FormatFields`](format/index.md) trait determines how fields &mdash; both the event's
fields and fields on spans &mdash; are formatted.

The `fmt::format` module provides several types which implement these traits,
many of which expose additional configuration options to customize their
output. The `format::Format` type implements common configuration used by
all the formatters provided in this crate, and can be used as a builder to
set specific formatting settings. For example:

```rust
use tracing_subscriber::fmt;

// Configure a custom event formatter
let format = fmt::format()
   .with_level(false) // don't include levels in formatted output
   .with_target(false) // don't include targets
   .with_thread_ids(true) // include the thread ID of the current thread
   .with_thread_names(true) // include the name of the current thread
   .compact(); // use the `Compact` formatting style.

// Create a `fmt` subscriber that uses our custom event format, and set it
// as the default.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

However, if a specific output format is needed, other crates can
also implement [`FormatEvent`](format/index.md) and [`FormatFields`](format/index.md). See those traits'
documentation for details on how to implement them.

## Filters

If you want to filter the `tracing` `Events` based on environment
variables, you can use the [`EnvFilter`](../filter/index.md) as follows:

```rust
use tracing_subscriber::EnvFilter;

let filter = EnvFilter::from_default_env();
```

As mentioned above, the [`EnvFilter`](../filter/index.md) allows `Span`s and `Event`s to
be filtered at runtime by setting the `RUST_LOG` environment variable.

You can find the other available [`filter`](../filter/index.md)s in the documentation.

### Using Your Subscriber

Finally, once you have configured your `Subscriber`, you need to
configure your executable to use it.

A subscriber can be installed globally using:
```rust
use tracing;
use tracing_subscriber::FmtSubscriber;

let subscriber = FmtSubscriber::new();

tracing::subscriber::set_global_default(subscriber)
    .map_err(|_err| eprintln!("Unable to set global default subscriber"));
// Note this will only fail if you try to set the global default
// subscriber multiple times
```

### Composing Layers

Composing an [`EnvFilter`](../filter/index.md) `Layer` and a `format `Layer``:

```rust
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;

let fmt_layer = fmt::layer()
    .with_target(false);
let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();

tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .init();
```








## Modules

- [`format`](format/index.md) - Formatters for logging [`tracing`] events.
- [`time`](time/index.md) - Formatters for event timestamps.
- [`writer`](writer/index.md) - Abstractions for creating [`io::Write`] instances.

## Structs

### `FmtContext<'a, S, N>`

```rust
struct FmtContext<'a, S, N> {
    ctx: crate::layer::Context<'a, S>,
    fmt_fields: &'a N,
    event: &'a tracing_core::Event<'a>,
}
```

Provides the current span context to a formatter.

#### Implementations

- `fn visit_spans<E, F>(self: &Self, f: F) -> Result<(), E>`

- `fn metadata(self: &Self, id: &Id) -> Option<&'static Metadata<'static>>`

- `fn span(self: &Self, id: &Id) -> Option<SpanRef<'_, S>>` — [`SpanRef`](../registry/index.md)

- `fn exists(self: &Self, id: &Id) -> bool`

- `fn lookup_current(self: &Self) -> Option<SpanRef<'_, S>>` — [`SpanRef`](../registry/index.md)

- `fn current_span(self: &Self) -> Current`

- `fn parent_span(self: &Self) -> Option<SpanRef<'_, S>>` — [`SpanRef`](../registry/index.md)

- `fn span_scope(self: &Self, id: &Id) -> Option<registry::Scope<'_, S>>` — [`Scope`](../registry/index.md)

- `fn event_scope(self: &Self) -> Option<registry::Scope<'_, S>>` — [`Scope`](../registry/index.md)

- `fn field_format(self: &Self) -> &N`

#### Trait Implementations

##### `impl<S, N> Debug for FmtContext<'_, S, N>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'writer, S, N> FormatFields for FmtContext<'_, S, N>`

- `fn format_fields<R: RecordFields>(self: &Self, writer: format::Writer<'writer>, fields: R) -> fmt::Result` — [`Writer`](format/index.md)

##### `impl<T> Instrument for FmtContext<'a, S, N>`

##### `impl<T> WithSubscriber for FmtContext<'a, S, N>`

### `FormattedFields<E: ?Sized>`

```rust
struct FormattedFields<E: ?Sized> {
    _format_fields: core::marker::PhantomData<fn(E)>,
    was_ansi: bool,
    pub fields: alloc::string::String,
}
```

A formatted representation of a span's fields stored in its [`extensions`](../registry/extensions/index.md).

Because `FormattedFields` is generic over the type of the formatter that
produced it, multiple versions of a span's formatted fields can be stored in
the [`Extensions`][`extensions`](../registry/extensions/index.md) type-map. This means that when multiple
formatters are in use, each can store its own formatted representation
without conflicting.


#### Fields

- **`fields`**: `alloc::string::String`

  The formatted fields of a span.

#### Implementations

- `fn new(fields: String) -> Self`

- `fn as_writer(self: &mut Self) -> format::Writer<'_>` — [`Writer`](format/index.md)

#### Trait Implementations

##### `impl<E: ?Sized> Debug for FormattedFields<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + ?Sized> Default for FormattedFields<E>`

- `fn default() -> FormattedFields<E>` — [`FormattedFields`](fmt_layer/index.md)

##### `impl<E: ?Sized> Deref for FormattedFields<E>`

- `type Target = String`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<E: ?Sized> Display for FormattedFields<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for FormattedFields<E>`

##### `impl<P, T> Receiver for FormattedFields<E>`

- `type Target = T`

##### `impl<T> ToString for FormattedFields<E>`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for FormattedFields<E>`

### `Layer<S, N, E, W>`

```rust
struct Layer<S, N, E, W> {
    make_writer: W,
    fmt_fields: N,
    fmt_event: E,
    fmt_span: format::FmtSpanConfig,
    is_ansi: bool,
    log_internal_errors: bool,
    _inner: core::marker::PhantomData<fn(S)>,
}
```

A [`Layer`](../layer/index.md) that logs formatted representations of `tracing` events.

## Examples

Constructing a layer with the default configuration:

```rust
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::prelude::*;

let subscriber = Registry::default()
    .with(fmt::Layer::default());

tracing::subscriber::set_global_default(subscriber).unwrap();
```

Overriding the layer's behavior:

```rust
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::prelude::*;

let fmt_layer = fmt::layer()
   .with_target(false) // don't include event targets when logging
   .with_level(false); // don't include event levels when logging

let subscriber = Registry::default().with(fmt_layer);
tracing::subscriber::set_global_default(subscriber).unwrap();
```

Setting a custom event formatter:

```rust
use tracing_subscriber::fmt::{self, format, time};
use tracing_subscriber::prelude::*;

let fmt = format().with_timer(time::Uptime::default());
let fmt_layer = fmt::layer()
    .event_format(fmt)
    .with_target(false);
let subscriber = fmt_layer.with_subscriber(tracing_subscriber::registry::Registry::default());
tracing::subscriber::set_global_default(subscriber).unwrap();
```


#### Implementations

- `fn with_writer<W2>(self: Self, make_writer: W2) -> Layer<S, N, E, W2>` — [`Layer`](fmt_layer/index.md)

- `fn writer(self: &Self) -> &W`

- `fn writer_mut(self: &mut Self) -> &mut W`

- `fn set_ansi(self: &mut Self, ansi: bool)`

- `fn set_span_events(self: &mut Self, kind: FmtSpan)` — [`FmtSpan`](format/index.md)

- `fn with_test_writer(self: Self) -> Layer<S, N, E, TestWriter>` — [`Layer`](fmt_layer/index.md), [`TestWriter`](writer/index.md)

- `fn with_ansi(self: Self, ansi: bool) -> Self`

- `fn log_internal_errors(self: Self, log_internal_errors: bool) -> Self`

- `fn map_writer<W2>(self: Self, f: impl FnOnce(W) -> W2) -> Layer<S, N, E, W2>` — [`Layer`](fmt_layer/index.md)

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, N: $crate::fmt::Debug, E: $crate::fmt::Debug, W: $crate::fmt::Debug> Debug for Layer<S, N, E, W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<S> Default for Layer<S>`

- `fn default() -> Self`

##### `impl<T> Instrument for Layer<S, N, E, W>`

##### `impl<S, N, E, W> Layer for Layer<S, N, E, W>`

- `fn on_new_span(self: &Self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record(self: &Self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter(self: &Self, id: &Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit(self: &Self, id: &Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close(self: &Self, id: Id, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_event(self: &Self, event: &Event<'_>, ctx: Context<'_, S>)` — [`Context`](../layer/index.md)

##### `impl<T> WithSubscriber for Layer<S, N, E, W>`

### `TestWriter`

```rust
struct TestWriter {
    use_stderr: bool,
}
```

A writer intended to support [`libtest`'s output capturing][capturing] for use in unit tests.

`TestWriter` is used by `fmt::Subscriber` or `fmt::Layer` to enable capturing support.

`cargo test` can only capture output from the standard library's `print!` and `eprint!`
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

- `fn default() -> TestWriter` — [`TestWriter`](writer/index.md)

##### `impl<T> Instrument for TestWriter`

##### `impl<'a> MakeWriter for TestWriter`

- `type Writer = TestWriter`

- `fn make_writer(self: &'a Self) -> <Self as >::Writer` — [`MakeWriter`](writer/index.md)

##### `impl<'a, M> MakeWriterExt for TestWriter`

##### `impl<T> WithSubscriber for TestWriter`

##### `impl Write for TestWriter`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

### `Subscriber<N, E, F, W>`

```rust
struct Subscriber<N, E, F, W> {
    inner: layer::Layered<F, Formatter<N, E, W>>,
}
```

A `Subscriber` that logs formatted representations of `tracing` events.

This consists of an inner `Formatter` wrapped in a layer that performs filtering.

#### Implementations

- `const DEFAULT_MAX_LEVEL: LevelFilter`

- `fn builder() -> SubscriberBuilder` — [`SubscriberBuilder`](#subscriberbuilder)

- `fn new() -> Self`

#### Trait Implementations

##### `impl<N: $crate::fmt::Debug, E: $crate::fmt::Debug, F: $crate::fmt::Debug, W: $crate::fmt::Debug> Debug for Subscriber<N, E, F, W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Subscriber`

- `fn default() -> Self`

##### `impl<T> Instrument for Subscriber<N, E, F, W>`

##### `impl<'a, N, E, F, W> LookupSpan for Subscriber<N, E, F, W>`

- `type Data = <Layered<F, Layered<Layer<Registry, N, E, W>, Registry>> as LookupSpan>::Data`

- `fn span_data(self: &'a Self, id: &span::Id) -> Option<<Self as >::Data>` — [`LookupSpan`](../registry/index.md)

##### `impl<S> Sealed for Subscriber<N, E, F, W>`

##### `impl<N, E, F, W> Subscriber for Subscriber<N, E, F, W>`

- `fn register_callsite(self: &Self, meta: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, meta: &Metadata<'_>) -> bool`

- `fn new_span(self: &Self, attrs: &span::Attributes<'_>) -> span::Id`

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)`

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`

- `fn event_enabled(self: &Self, event: &Event<'_>) -> bool`

- `fn event(self: &Self, event: &Event<'_>)`

- `fn enter(self: &Self, id: &span::Id)`

- `fn exit(self: &Self, id: &span::Id)`

- `fn current_span(self: &Self) -> span::Current`

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id`

- `fn try_close(self: &Self, id: span::Id) -> bool`

- `fn max_level_hint(self: &Self) -> Option<tracing_core::LevelFilter>`

- `unsafe fn downcast_raw(self: &Self, id: TypeId) -> Option<*const ()>`

##### `impl<S> SubscriberExt for Subscriber<N, E, F, W>`

##### `impl<T> SubscriberInitExt for Subscriber<N, E, F, W>`

##### `impl<T> WithSubscriber for Subscriber<N, E, F, W>`

### `SubscriberBuilder<N, E, F, W>`

```rust
struct SubscriberBuilder<N, E, F, W> {
    filter: F,
    inner: Layer<crate::registry::Registry, N, E, W>,
}
```

Configures and constructs `Subscriber`s.

#### Implementations

- `fn reload_handle(self: &Self) -> crate::reload::Handle<crate::EnvFilter, Formatter<N, E, W>>` — [`Handle`](../reload/index.md), [`EnvFilter`](../filter/index.md), [`Formatter`](#formatter)

#### Trait Implementations

##### `impl<N: $crate::fmt::Debug, E: $crate::fmt::Debug, F: $crate::fmt::Debug, W: $crate::fmt::Debug> Debug for SubscriberBuilder<N, E, F, W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SubscriberBuilder`

- `fn default() -> Self`

##### `impl<T> Instrument for SubscriberBuilder<N, E, F, W>`

##### `impl<T> SubscriberInitExt for SubscriberBuilder<N, E, F, W>`

##### `impl<T> WithSubscriber for SubscriberBuilder<N, E, F, W>`

## Traits

## Functions

### `fmt`

```rust
fn fmt() -> SubscriberBuilder
```

Returns a new [`SubscriberBuilder`](#subscriberbuilder) for configuring a [formatting subscriber].

This is essentially shorthand for [`SubscriberBuilder::default()]`.

# Examples

Using [`init`](#init) to set the default subscriber:

```rust
tracing_subscriber::fmt().init();
```

Configuring the output format:

```rust

tracing_subscriber::fmt()
    // Configure formatting settings.
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    // Set the subscriber as the default.
    .init();
```

[`try_init`](#try-init) returns an error if the default subscriber could not be set:

```rust
use std::error::Error;

fn init_subscriber() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        // Configure the subscriber to emit logs in JSON format.
        .json()
        // Configure the subscriber to flatten event fields in the output JSON objects.
        .flatten_event(true)
        // Set the subscriber as the default, returning an error if this fails.
        .try_init()?;

    Ok(())
}
```

Rather than setting the subscriber as the default, `finish` _returns_ the
constructed subscriber, which may then be passed to other functions:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .compact()
    .finish();

tracing::subscriber::with_default(subscriber, || {
    // the subscriber will only be set as the default
    // inside this closure...
})
```






### `layer`

```rust
fn layer<S>() -> Layer<S>
```

Returns a new [formatting layer] that can be [composed] with other layers to
construct a [`Subscriber`](#subscriber).

This is a shorthand for the equivalent `Layer::default()` function.




### `try_init`

```rust
fn try_init() -> Result<(), alloc::boxed::Box<dyn Error + Send + Sync>>
```

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable],
if one is not already set.

If the `tracing-log` feature is enabled, this will also install
the `LogTracer` to convert `log` records into `tracing` `Event`s.

This is shorthand for

```rust
fn doc() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
tracing_subscriber::fmt().try_init()
}
```


# Errors

Returns an Error if the initialization was unsuccessful,
likely because a global subscriber was already installed by another
call to `try_init`.



### `init`

```rust
fn init()
```

Install a global tracing subscriber that listens for events and
filters based on the value of the [`RUST_LOG` environment variable].

The configuration of the subscriber initialized by this function
depends on what [feature flags](crate#feature-flags) are enabled.

If the `tracing-log` feature is enabled, this will also install
the LogTracer to convert `Log` records into `tracing` `Event`s.

If the `env-filter` feature is enabled, this is shorthand for

```rust
use tracing_subscriber::EnvFilter;
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
```

# Panics
Panics if the initialization was unsuccessful, likely because a
global subscriber was already installed by another call to `try_init`.


## Type Aliases

### `Formatter<N, E, W>`

```rust
type Formatter<N, E, W> = layer::Layered<fmt_layer::Layer<crate::registry::Registry, N, E, W>, crate::registry::Registry>;
```

A `Subscriber` that logs formatted representations of `tracing` events.
This type only logs formatted events; it does not perform any filtering.

