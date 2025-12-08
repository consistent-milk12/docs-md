# Crate `tracing_subscriber`

Utilities for implementing and composing `tracing` subscribers.

`tracing` is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics. The [`Subscriber`](fmt/index.md) trait
represents the functionality necessary to collect this trace data. This
crate contains tools for composing subscribers out of smaller units of
behaviour, and batteries-included implementations of common subscriber
functionality.

`tracing-subscriber` is intended for use by both `Subscriber` authors and
application authors using `tracing` to instrument their applications.

*Compiler support: [requires `rustc` 1.65+][msrv]*

## `Layer`s and `Filter`s

The most important component of the `tracing-subscriber` API is the
[`Layer`](layer/index.md) trait, which provides a composable abstraction for building
[`Subscriber`](fmt/index.md)s. Like the [`Subscriber`](fmt/index.md) trait, a [`Layer`](layer/index.md) defines a
particular behavior for collecting trace data. Unlike [`Subscriber`](fmt/index.md)s,
which implement a *complete* strategy for how trace data is collected,
[`Layer`](layer/index.md)s provide *modular* implementations of specific behaviors.
Therefore, they can be [composed together] to form a [`Subscriber`](fmt/index.md) which is
capable of recording traces in a variety of ways. See the [`layer` module's
documentation][`layer`](layer/index.md) for details on using [`Layer`](layer/index.md)s.

In addition, the [`Filter`](layer/index.md) trait defines an interface for filtering what
spans and events are recorded by a particular layer. This allows different
[`Layer`](layer/index.md)s to handle separate subsets of the trace data emitted by a
program. See the [documentation on per-layer filtering][plf] for more
information on using [`Filter`](layer/index.md)s.





## Included Subscribers

The following `Subscriber`s are provided for application authors:

- [`fmt`](fmt/index.md) - Formats and logs tracing data (requires the `fmt` feature flag)

## Feature Flags

- `std`: Enables APIs that depend on the Rust standard library
  (enabled by default).
- `alloc`: Depend on `liballoc` (enabled by "std").
- `env-filter`: Enables the [`EnvFilter`](filter/index.md) type, which implements filtering
  similar to the [`env_logger` crate]. **Requires "std"**.
- `fmt`: Enables the [`fmt`](fmt/index.md) module, which provides a subscriber
  implementation for printing formatted representations of trace events.
  Enabled by default. **Requires "registry" and "std"**.
- `ansi`: Enables `fmt` support for ANSI terminal colors. Enabled by
  default.
- `registry`: enables the [`registry`](registry/index.md) module. Enabled by default.
  **Requires "std"**.
- `json`: Enables `fmt` support for JSON output. In JSON output, the ANSI
  feature does nothing. **Requires "fmt" and "std"**.
- `local-time`: Enables local time formatting when using the [`time`
  crate]'s timestamp formatters with the `fmt` subscriber.

### Optional Dependencies

- `tracing-log`: Enables better formatting for events emitted by `log`
  macros in the `fmt` subscriber. Enabled by default.
- [`time`][`time` crate]: Enables support for using the [`time` crate] for timestamp
  formatting in the `fmt` subscriber.
- `smallvec`: Causes the `EnvFilter` type to use the `smallvec` crate (rather
  than `Vec`) as a performance optimization. Enabled by default.
- `parking_lot`: Use the `parking_lot` crate's `RwLock` implementation
  rather than the Rust standard library's implementation.

### `no_std` Support

In embedded systems and other bare-metal applications, `tracing` can be
used without requiring the Rust standard library, although some features are
disabled. Although most of the APIs provided by `tracing-subscriber`, such
as [`fmt`](fmt/index.md) and [`EnvFilter`](filter/index.md), require the standard library, some
functionality, such as the [`Layer`](layer/index.md) trait, can still be used in
`no_std` environments.

The dependency on the standard library is controlled by two crate feature
flags, "std", which enables the dependency on `libstd`, and "alloc", which
enables the dependency on `liballoc` (and is enabled by the "std"
feature). These features are enabled by default, but `no_std` users can
disable them using:

```toml
Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false }
```

Additional APIs are available when `liballoc` is available. To enable
`liballoc` but not `std`, use:

```toml
Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
```

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables support for serializing values recorded using the
  `valuable` crate as structured JSON in the `format::Json` formatter.

#### Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
env variable when running `cargo` commands:

```shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```
Alternatively, the following can be added to the `.cargo/config` file in a
project to automatically enable the cfg flag for that project:

```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```



## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.65. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.













## Modules

- [`field`](field/index.md) - Utilities for working with [fields] and [field visitors].
- [`filter`](filter/index.md) - [`Layer`]s that control which spans and events are enabled by the wrapped
- [`prelude`](prelude/index.md) - The `tracing-subscriber` prelude.
- [`registry`](registry/index.md) - Storage for span data shared by multiple [`Layer`]s.
- [`layer`](layer/index.md) -  The [`Layer`] trait, a composable abstraction for building [`Subscriber`]s.
- [`util`](util/index.md) - Extension traits and other utilities to make working with subscribers more
- [`reload`](reload/index.md) - Wrapper for a `Layer` to allow it to be dynamically reloaded.
- [`fmt`](fmt/index.md) - A `Subscriber` for formatting and logging `tracing` data.

## Structs

### `FmtSubscriber<N, E, F, W>`

```rust
struct FmtSubscriber<N, E, F, W> {
    inner: layer::Layered<F, Formatter<N, E, W>>,
}
```

A `Subscriber` that logs formatted representations of `tracing` events.

This consists of an inner `Formatter` wrapped in a layer that performs filtering.

#### Implementations

- `const DEFAULT_MAX_LEVEL: LevelFilter`

- `fn builder() -> SubscriberBuilder` — [`SubscriberBuilder`](fmt/index.md)

- `fn new() -> Self`

#### Trait Implementations

##### `impl<N: $crate::fmt::Debug, E: $crate::fmt::Debug, F: $crate::fmt::Debug, W: $crate::fmt::Debug> Debug for Subscriber<N, E, F, W>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Subscriber`

- `fn default() -> Self`

##### `impl<T> Instrument for Subscriber<N, E, F, W>`

##### `impl<'a, N, E, F, W> LookupSpan for Subscriber<N, E, F, W>`

- `type Data = <Layered<F, Layered<Layer<Registry, N, E, W>, Registry>> as LookupSpan>::Data`

- `fn span_data(self: &'a Self, id: &span::Id) -> Option<<Self as >::Data>` — [`LookupSpan`](registry/index.md)

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

### `EnvFilter`

```rust
struct EnvFilter {
    statics: crate::filter::directive::DirectiveSet<crate::filter::directive::StaticDirective>,
    dynamics: crate::filter::directive::DirectiveSet<Directive>,
    has_dynamics: bool,
    by_id: std::sync::RwLock<std::collections::HashMap<span::Id, MatchSet<field::SpanMatch>>>,
    by_cs: std::sync::RwLock<std::collections::HashMap<callsite::Identifier, MatchSet<field::CallsiteMatch>>>,
    scope: thread_local::ThreadLocal<core::cell::RefCell<alloc::vec::Vec<crate::filter::LevelFilter>>>,
    regex: bool,
}
```

A [`Layer`](layer/index.md) which filters spans and events based on a set of filter
directives.

`EnvFilter` implements both the [`Layer`](#impl-Layer<S>) and [`Filter`](layer/index.md) traits, so it may
be used for both [global filtering][global] and [per-layer filtering][plf],
respectively. See [the documentation on filtering with `Layer`s][filtering]
for details.

The [`Targets`](filter/targets/index.md) type implements a similar form of filtering, but without the
ability to dynamically enable events based on the current span context, and
without filtering on field values. When these features are not required,
[`Targets`](filter/targets/index.md) provides a lighter-weight alternative to [`EnvFilter`](filter/index.md).

# Directives

A filter consists of one or more comma-separated directives which match on [`Span`](#span)s and `Event`s.
Each directive may have a corresponding maximum verbosity [`level`](filter/level/index.md) which
enables (e.g., _selects for_) spans and events that match. Like `log`,
`tracing` considers less exclusive levels (like `trace` or `info`) to be more
verbose than more exclusive levels (like `error` or `warn`).

The directive syntax is similar to that of `env_logger`'s. At a high level, the syntax for directives
consists of several parts:

```text
target[span{field=value}]=level
```

Each component (`target`, `span`, `field`, `value`, and `level`) will be covered in turn.

- `target` matches the event or span's target. In general, this is the module path and/or crate name.
  Examples of targets `h2`, `tokio::net`, or `tide::server`. For more information on targets,
  please refer to `Metadata`'s documentation.
- `span` matches on the span's name. If a `span` directive is provided alongside a `target`,
  the `span` directive will match on spans _within_ the `target`.
- `field` matches on [`fields`](macros/index.md) within spans. Field names can also be supplied without a `value`
  and will match on any [`Span`](#span) or `Event` that has a field with that name.
  For example: `[span{field=\"value\"}]=debug`, `[{field}]=trace`.
- `value` matches on the value of a span's field. If a value is a numeric literal or a bool,
  it will match _only_ on that value. Otherwise, this filter matches the
  `std::fmt::Debug` output from the value.
- `level` sets a maximum verbosity level accepted by this directive.

When a field value directive (`[{<FIELD NAME>=<FIELD_VALUE>}]=...`) matches a
value's `std::fmt::Debug` output (i.e., the field value in the directive
is not a `bool`, `i64`, `u64`, or `f64` literal), the matched pattern may be
interpreted as either a regular expression or as the precise expected
output of the field's `std::fmt::Debug` implementation. By default, these
filters are interpreted as regular expressions, but this can be disabled
using the `Builder::with_regex` builder method to use precise matching
instead.

When field value filters are interpreted as regular expressions, the
[`regex` crate's regular expression syntax][re-syntax] is supported.

**Note**: When filters are constructed from potentially untrusted inputs,
[disabling regular expression matching](Builder::with_regex) is strongly
recommended.

## Usage Notes

- The portion of the directive which is included within the square brackets is `tracing`-specific.
- Any portion of the directive can be omitted.
    - The sole exception are the `field` and `value` directives. If a `value` is provided,
      a `field` must _also_ be provided. However, the converse does not hold, as fields can
      be matched without a value.
- If only a level is provided, it will set the maximum level for all `Span`s and `Event`s
  that are not enabled by other filters.
- A directive without a level will enable anything that it matches. This is equivalent to `=trace`.
- When a crate has a dash in its name, the default target for events will be the
  crate's module path as it appears in Rust. This means every dash will be replaced
  with an underscore.
- A dash in a target will only appear when being specified explicitly:
  `tracing::info!(target: "target-name", ...);`

## Example Syntax

- `tokio::net=info` will enable all spans or events that:
   - have the `tokio::net` target,
   - at the level `info` or above.
- `warn,tokio::net=info` will enable all spans and events that:
   - are at the level `warn` or above, *or*
   - have the `tokio::net` target at the level `info` or above.
- `my_crate[span_a]=trace` will enable all spans and events that:
   - are within the `span_a` span or named `span_a` _if_ `span_a` has the target `my_crate`,
   - at the level `trace` or above.
- `[span_b{name=\"bob\"}]` will enable all spans or event that:
   - have _any_ target,
   - are inside a span named `span_b`,
   - which has a field named `name` with value `bob`,
   - at _any_ level.

# Examples

Parsing an `EnvFilter` from the [default environment
variable](EnvFilter::from_default_env) (`RUST_LOG`):

```rust
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_default_env())
    .init();
```

Parsing an `EnvFilter` [from a user-provided environment
variable](EnvFilter::from_env):

```rust
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

tracing_subscriber::registry()
    .with(fmt::layer())
    .with(EnvFilter::from_env("MYAPP_LOG"))
    .init();
```

Using `EnvFilter` as a [per-layer filter][plf] to filter only a single

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// Parse an `EnvFilter` configuration from the `RUST_LOG`
// environment variable.
let filter = EnvFilter::from_default_env();

// Apply the filter to this layer *only*.
let filtered_layer = fmt::layer().with_filter(filter);

// Some other layer, whose output we don't want to filter.
let unfiltered_layer = // ...
    # fmt::layer();

tracing_subscriber::registry()
    .with(filtered_layer)
    .with(unfiltered_layer)
    .init();
```rust
Constructing `EnvFilter`s

An `EnvFilter` is be constructed by parsing a string containing one or more
directives. The `EnvFilter::new` constructor parses an `EnvFilter` from a
string, ignoring any invalid directives, while `EnvFilter::try_new`
returns an error if invalid directives are encountered. Similarly, the
`EnvFilter::from_env` and `EnvFilter::try_from_env` constructors parse
an `EnvFilter` from the value of the provided environment variable, with
lossy and strict validation, respectively.

A [builder](EnvFilter::builder) interface is available to set additional
configuration options prior to parsing an `EnvFilter`. See the [`Builder`
type's documentation](Builder) for details on the options that can be
configured using the builder.













#### Implementations

- `const DEFAULT_ENV: &'static str`

- `fn builder() -> Builder` — [`Builder`](filter/env/builder/index.md)

- `fn from_default_env() -> Self`

- `fn from_env<A: AsRef<str>>(env: A) -> Self`

- `fn new<S: AsRef<str>>(directives: S) -> Self`

- `fn try_new<S: AsRef<str>>(dirs: S) -> Result<Self, crate::filter::directive::ParseError>` — [`ParseError`](filter/directive/index.md)

- `fn try_from_default_env() -> Result<Self, FromEnvError>` — [`FromEnvError`](filter/index.md)

- `fn try_from_env<A: AsRef<str>>(env: A) -> Result<Self, FromEnvError>` — [`FromEnvError`](filter/index.md)

- `fn add_directive(self: Self, directive: Directive) -> Self` — [`Directive`](filter/env/directive/index.md)

- `fn enabled<S>(self: &Self, metadata: &Metadata<'_>, _: Context<'_, S>) -> bool` — [`Context`](layer/index.md)

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

- `fn on_new_span<S>(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, _: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_enter<S>(self: &Self, id: &span::Id, _: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_exit<S>(self: &Self, id: &span::Id, _: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_close<S>(self: &Self, id: span::Id, _: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_record<S>(self: &Self, id: &span::Id, values: &span::Record<'_>, _: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn cares_about_span(self: &Self, span: &span::Id) -> bool`

- `fn base_interest(self: &Self) -> Interest`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

#### Trait Implementations

##### `impl Clone for EnvFilter`

- `fn clone(self: &Self) -> EnvFilter` — [`EnvFilter`](filter/index.md)

##### `impl Debug for EnvFilter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for EnvFilter`

- `fn default() -> Self`

##### `impl Display for EnvFilter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<S> Filter for EnvFilter`

- `fn enabled(self: &Self, meta: &Metadata<'_>, ctx: &Context<'_, S>) -> bool` — [`Context`](layer/index.md)

- `fn callsite_enabled(self: &Self, meta: &'static Metadata<'static>) -> Interest`

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_record(self: &Self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_close(self: &Self, id: span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

##### `impl<F, S> FilterExt for EnvFilter`

##### `impl FromStr for EnvFilter`

- `type Err = ParseError`

- `fn from_str(spec: &str) -> Result<Self, <Self as >::Err>`

##### `impl<T> Instrument for EnvFilter`

##### `impl<S: Subscriber> Layer for EnvFilter`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool` — [`Context`](layer/index.md)

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_record(self: &Self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

- `fn on_close(self: &Self, id: span::Id, ctx: Context<'_, S>)` — [`Context`](layer/index.md)

##### `impl<T> ToString for EnvFilter`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for EnvFilter`

### `Registry`

```rust
struct Registry {
    spans: sharded_slab::Pool<DataInner>,
    current_spans: thread_local::ThreadLocal<core::cell::RefCell<super::stack::SpanStack>>,
    next_filter_id: u8,
}
```

A shared, reusable store for spans.

A `Registry` is a [`Subscriber`](fmt/index.md) around which multiple [`Layer`](layer/index.md)s
implementing various behaviors may be [added]. Unlike other types
implementing `Subscriber`, `Registry` does not actually record traces itself:
instead, it collects and stores span data that is exposed to any [`Layer`](layer/index.md)s
wrapping it through implementations of the [`LookupSpan`](registry/index.md) trait.
The `Registry` is responsible for storing span metadata, recording
relationships between spans, and tracking which spans are active and which
are closed. In addition, it provides a mechanism for [`Layer`](layer/index.md)s to store
user-defined per-span data, called [`extensions`](registry/extensions/index.md), in the registry. This
allows [`Layer`](layer/index.md)-specific data to benefit from the `Registry`'s
high-performance concurrent storage.

This registry is implemented using a [lock-free sharded slab][slab], and is
highly optimized for concurrent access.

# Span ID Generation

Span IDs are not globally unique, but the registry ensures that
no two currently active spans have the same ID within a process.

One of the primary responsibilities of the registry is to generate [span
IDs]. Therefore, it's important for other code that interacts with the
registry, such as [`Layer`](layer/index.md)s, to understand the guarantees of the
span IDs that are generated.

The registry's span IDs are guaranteed to be unique **at a given point
in time**. This means that an active span will never be assigned the
same ID as another **currently active** span. However, the registry
**will** eventually reuse the IDs of [closed] spans, although an ID
will never be reassigned immediately after a span has closed.

Spans are not [considered closed] by the `Registry` until *every*
[`Span`](#span) reference with that ID has been dropped.

Thus: span IDs generated by the registry should be considered unique
only at a given point in time, and only relative to other spans
generated by the same process. Two spans with the same ID will not exist
in the same process concurrently. However, if historical span data is
being stored, the same ID may occur for multiple spans times in that
data. If spans must be uniquely identified in historical data, the user
code storing this data must assign its own unique identifiers to those
spans. A counter is generally sufficient for this.

Similarly, span IDs generated by the registry are not unique outside of
a given process. Distributed tracing systems may require identifiers
that are unique across multiple processes on multiple machines (for
example, [OpenTelemetry's `SpanId`s and `TraceId`s][ot]). `tracing` span
IDs generated by the registry should **not** be used for this purpose.
Instead, code which integrates with a distributed tracing system should
generate and propagate its own IDs according to the rules specified by
the distributed tracing system. These IDs can be associated with
`tracing` spans using [`fields`](macros/index.md) and/or [stored span data].












#### Implementations

- `fn get(self: &Self, id: &Id) -> Option<Ref<'_, DataInner>>` — [`DataInner`](registry/sharded/index.md)

- `fn start_close(self: &Self, id: Id) -> CloseGuard<'_>` — [`CloseGuard`](registry/sharded/index.md)

- `fn has_per_layer_filters(self: &Self) -> bool`

- `fn span_stack(self: &Self) -> cell::Ref<'_, SpanStack>` — [`SpanStack`](registry/stack/index.md)

#### Trait Implementations

##### `impl Debug for Registry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Registry`

- `fn default() -> Self`

##### `impl<T> Instrument for Registry`

##### `impl<'a> LookupSpan for Registry`

- `type Data = Data<'a>`

- `fn span_data(self: &'a Self, id: &Id) -> Option<<Self as >::Data>` — [`LookupSpan`](registry/index.md)

- `fn register_filter(self: &mut Self) -> FilterId` — [`FilterId`](filter/index.md)

##### `impl<S> Sealed for Registry`

##### `impl Subscriber for Registry`

- `fn register_callsite(self: &Self, _: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, _: &Metadata<'_>) -> bool`

- `fn new_span(self: &Self, attrs: &span::Attributes<'_>) -> span::Id`

- `fn record(self: &Self, _: &span::Id, _: &span::Record<'_>)`

- `fn record_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id)`

- `fn event_enabled(self: &Self, _event: &Event<'_>) -> bool`

- `fn event(self: &Self, _: &Event<'_>)`

- `fn enter(self: &Self, id: &span::Id)`

- `fn exit(self: &Self, id: &span::Id)`

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id`

- `fn current_span(self: &Self) -> Current`

- `fn try_close(self: &Self, id: span::Id) -> bool`

##### `impl<S> SubscriberExt for Registry`

##### `impl<T> SubscriberInitExt for Registry`

##### `impl<T> WithSubscriber for Registry`

## Traits

## Functions

### `registry`

```rust
fn registry() -> Registry
```

Returns a default [`Registry`](registry/sharded/index.md).

