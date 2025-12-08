# Crate `tracing_core`

Core primitives for `tracing`.

`tracing` is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate defines the core
primitives of `tracing`.

This crate provides:

* [`span::Id`](span/index.md) identifies a span within the execution of a program.

* [`Event`](#event) represents a single event within a trace.

* [`Subscriber`](#subscriber), the trait implemented to collect trace data.

* [`Metadata`](#metadata) and [`Callsite`](#callsite) provide information describing spans and
  `Event`s.

* [`Field`](#field), [`FieldSet`](field/index.md), [`Value`](field/index.md), and [`ValueSet`](field/index.md) represent the
  structured data attached to a span.

* [`Dispatch`](#dispatch) allows spans and events to be dispatched to `Subscriber`s.

In addition, it defines the global callsite registry and per-thread current
dispatcher which other components of the tracing system rely on.

*Compiler support: [requires `rustc` 1.65+][msrv]*

## Usage

Application authors will typically not use this crate directly. Instead,
they will use the `tracing` crate, which provides a much more
fully-featured API. However, this crate's API will change very infrequently,
so it may be used when dependencies must be very stable.

`Subscriber` implementations may depend on `tracing-core` rather than
`tracing`, as the additional APIs provided by `tracing` are primarily useful
for instrumenting libraries and applications, and are generally not
necessary for `Subscriber` implementations.

The `tokio-rs/tracing` repository contains less stable crates designed to
be used with the `tracing` ecosystem. It includes a collection of
`Subscriber` implementations, as well as utility and adapter crates.

## Crate Feature Flags

The following crate [feature flags] are available:

* `std`: Depend on the Rust standard library (enabled by default).

  `no_std` users may disable this feature with `default-features = false`:

  ```toml
  [dependencies]
  tracing-core = { version = "0.1.22", default-features = false }
  ```

  **Note**:`tracing-core`'s `no_std` support requires `liballoc`.

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables support for recording [field values] using the
  `valuable` crate.

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

- [`lazy`](lazy/index.md) - 
- [`callsite`](callsite/index.md) - Callsites represent the source locations from which spans or events
- [`dispatcher`](dispatcher/index.md) - Dispatches trace events to [`Subscriber`]s.
- [`event`](event/index.md) - Events represent single points in time during the execution of a program.
- [`field`](field/index.md) - `Span` and `Event` key-value data.
- [`metadata`](metadata/index.md) - Metadata describing trace data.
- [`parent`](parent/index.md) - 
- [`span`](span/index.md) - Spans represent periods of time in the execution of a program.
- [`subscriber`](subscriber/index.md) - Collectors collect and record trace data.
- [`sealed`](sealed/index.md) - 

## Structs

### `Dispatch`

```rust
struct Dispatch {
    subscriber: Kind<alloc::sync::Arc<dyn Subscriber + Send + Sync>>,
}
```

`Dispatch` trace data to a [`Subscriber`](#subscriber).

#### Implementations

- `fn none() -> Self`

- `fn new<S>(subscriber: S) -> Self`

- `fn registrar(self: &Self) -> Registrar` — [`Registrar`](dispatcher/index.md)

- `fn downgrade(self: &Self) -> WeakDispatch` — [`WeakDispatch`](dispatcher/index.md)

- `fn subscriber(self: &Self) -> &dyn Subscriber + Send + Sync` — [`Subscriber`](#subscriber)

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> subscriber::Interest` — [`Metadata`](#metadata), [`Interest`](#interest)

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>` — [`LevelFilter`](#levelfilter)

- `fn new_span(self: &Self, span: &span::Attributes<'_>) -> span::Id` — [`Attributes`](span/index.md), [`Id`](span/index.md)

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)` — [`Id`](span/index.md), [`Record`](span/index.md)

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)` — [`Id`](span/index.md)

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool` — [`Metadata`](#metadata)

- `fn event(self: &Self, event: &Event<'_>)` — [`Event`](#event)

- `fn enter(self: &Self, span: &span::Id)` — [`Id`](span/index.md)

- `fn exit(self: &Self, span: &span::Id)` — [`Id`](span/index.md)

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id` — [`Id`](span/index.md)

- `fn drop_span(self: &Self, id: span::Id)` — [`Id`](span/index.md)

- `fn try_close(self: &Self, id: span::Id) -> bool` — [`Id`](span/index.md)

- `fn current_span(self: &Self) -> span::Current` — [`Current`](span/index.md)

- `fn is<T: Any>(self: &Self) -> bool`

- `fn downcast_ref<T: Any>(self: &Self) -> Option<&T>`

#### Trait Implementations

##### `impl Clone for Dispatch`

- `fn clone(self: &Self) -> Dispatch` — [`Dispatch`](#dispatch)

##### `impl Debug for Dispatch`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dispatch`

- `fn default() -> Self`

### `Event<'a>`

```rust
struct Event<'a> {
    fields: &'a field::ValueSet<'a>,
    metadata: &'static crate::Metadata<'static>,
    parent: crate::parent::Parent,
}
```

`Event`s represent single points in time where something occurred during the
execution of a program.

An `Event` can be compared to a log record in unstructured logging, but with
two key differences:
- `Event`s exist _within the context of a [`span`](span/index.md)_. Unlike log lines, they
  may be located within the trace tree, allowing visibility into the
  _temporal_ context in which the event occurred, as well as the source
  code location.
- Like spans, `Event`s have structured key-value data known as _[`fields`](../tracing_attributes/attr/kw/index.md)_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.



#### Implementations

- `fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- `fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- `fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Id`](span/index.md), [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- `fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Id`](span/index.md), [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- `fn record(self: &Self, visitor: &mut dyn field::Visit)` — [`Visit`](field/index.md)

- `fn fields(self: &Self) -> field::Iter` — [`Iter`](field/index.md)

- `fn metadata(self: &Self) -> &'static Metadata<'static>` — [`Metadata`](#metadata)

- `fn is_root(self: &Self) -> bool`

- `fn is_contextual(self: &Self) -> bool`

- `fn parent(self: &Self) -> Option<&Id>` — [`Id`](span/index.md)

#### Trait Implementations

##### `impl<'a> Debug for Event<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Field`

```rust
struct Field {
    i: usize,
    fields: FieldSet,
}
```

An opaque key allowing _O_(1) access to a field in a `Span`'s key-value
data.

As keys are defined by the _metadata_ of a span, rather than by an
individual instance of a span, a key may be used to access the same field
across all instances of a given span with the same metadata. Thus, when a
subscriber observes a new span, it need only access a field by name _once_,
and use the key for that name for all other accesses.

#### Implementations

- `fn callsite(self: &Self) -> callsite::Identifier` — [`Identifier`](callsite/index.md)

- `fn name(self: &Self) -> &'static str`

- `fn index(self: &Self) -> usize`

#### Trait Implementations

##### `impl AsRef for Field`

- `fn as_ref(self: &Self) -> &str`

##### `impl Clone for Field`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for Field`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Field`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Field`

##### `impl Hash for Field`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for Field`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> ToString for Field`

- `fn to_string(self: &Self) -> String`

### `Level`

```rust
struct Level(LevelInner);
```

Describes the level of verbosity of a span or event.

# Comparing Levels

`Level` implements the `PartialOrd` and `Ord` traits, allowing two
`Level`s to be compared to determine which is considered more or less
verbose. Levels which are more verbose are considered "greater than" levels
which are less verbose, with `Level::ERROR` considered the lowest, and
`Level::TRACE` considered the highest.

For example:
```rust
use tracing_core::Level;

assert!(Level::TRACE > Level::DEBUG);
assert!(Level::ERROR < Level::WARN);
assert!(Level::INFO <= Level::DEBUG);
assert_eq!(Level::TRACE, Level::TRACE);
```

# Filtering

`Level`s are typically used to implement filtering that determines which
spans and events are enabled. Depending on the use case, more or less
verbose diagnostics may be desired. For example, when running in
development, `DEBUG`-level traces may be enabled by default. When running in
production, only `INFO`-level and lower traces might be enabled. Libraries
may include very verbose diagnostics at the `DEBUG` and/or `TRACE` levels.
Applications using those libraries typically chose to ignore those traces. However, when
debugging an issue involving said libraries, it may be useful to temporarily
enable the more verbose traces.

The [`LevelFilter`](#levelfilter) type is provided to enable filtering traces by
verbosity. `Level`s can be compared against [`LevelFilter`](#levelfilter)s, and
[`LevelFilter`](#levelfilter) has a variant for each `Level`, which compares analogously
to that level. In addition, [`LevelFilter`](#levelfilter) adds a `LevelFilter::OFF`
variant, which is considered "less verbose" than every other `Level`. This is
intended to allow filters to completely disable tracing in a particular context.

For example:
```rust
use tracing_core::{Level, LevelFilter};

assert!(LevelFilter::OFF < Level::TRACE);
assert!(LevelFilter::TRACE > Level::DEBUG);
assert!(LevelFilter::ERROR < Level::WARN);
assert!(LevelFilter::INFO <= Level::DEBUG);
assert!(LevelFilter::INFO >= Level::INFO);
```

## Examples

Below is a simple example of how a [`Subscriber`](#subscriber) could implement filtering through
a [`LevelFilter`](#levelfilter). When a span or event is recorded, the `Subscriber::enabled` method
compares the span or event's `Level` against the configured [`LevelFilter`](#levelfilter).
The optional `Subscriber::max_level_hint` method can also be implemented to allow spans
and events above a maximum verbosity level to be skipped more efficiently,
often improving performance in short-lived programs.

```rust
use tracing_core::{span, Event, Level, LevelFilter, Subscriber, Metadata};
use tracing_core::span::{Id, Record, Current};

#[derive(Debug)]
pub struct MySubscriber {
    /// The most verbose level that this subscriber will enable.
    max_level: LevelFilter,

    // ...
}

impl MySubscriber {
    /// Returns a new `MySubscriber` which will record spans and events up to
    /// `max_level`.
    pub fn with_max_level(max_level: LevelFilter) -> Self {
        Self {
            max_level,
            // ...
        }
    }
}
impl Subscriber for MySubscriber {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        // A span or event is enabled if it is at or below the configured
        // maximum level.
        meta.level() <= &self.max_level
    }

    // This optional method returns the most verbose level that this
    // subscriber will enable. Although implementing this method is not
    // *required*, it permits additional optimizations when it is provided,
    // allowing spans and events above the max level to be skipped
    // more efficiently.
    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(self.max_level)
    }

    // Implement the rest of the subscriber...
    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        // ...
        drop(span); Id::from_u64(1)
    }

    fn event(&self, event: &Event<'_>) {
        // ...
        drop(event);
    }

    // ...
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}
    fn record(&self, _: &Id, _: &Record<'_>) {}
    fn record_follows_from(&self, _: &Id, _: &Id) {}
}
```

It is worth noting that the `tracing-subscriber` crate provides [additional
APIs][envfilter] for performing more sophisticated filtering, such as
enabling different levels based on which module or crate a span or event is
recorded in.








#### Implementations

- `const ERROR: Level`

- `const WARN: Level`

- `const INFO: Level`

- `const DEBUG: Level`

- `const TRACE: Level`

- `fn as_str(self: &Self) -> &'static str`

#### Trait Implementations

##### `impl Clone for Level`

- `fn clone(self: &Self) -> Level` — [`Level`](#level)

##### `impl Copy for Level`

##### `impl Debug for Level`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Level`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl FromStr for Level`

- `type Err = ParseLevelError`

- `fn from_str(s: &str) -> Result<Self, ParseLevelError>` — [`ParseLevelError`](metadata/index.md)

##### `impl Hash for Level`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Level`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for Level`

- `fn eq(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl PartialOrd for Level`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](#levelfilter)

- `fn lt(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- `fn le(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- `fn gt(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- `fn ge(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl StructuralPartialEq for Level`

##### `impl<T> ToString for Level`

- `fn to_string(self: &Self) -> String`

### `LevelFilter`

```rust
struct LevelFilter(Option<Level>);
```

A filter comparable to a verbosity [`Level`](#level).

If a [`Level`](#level) is considered less than or equal to a `LevelFilter`, it
should be considered enabled; if greater than the `LevelFilter`, that level
is disabled. See `LevelFilter::current` for more details.

Note that this is essentially identical to the `Level` type, but with the
addition of an `OFF` level that completely disables all trace
instrumentation.

See the documentation for the [`Level`](#level) type to see how `Level`s
and `LevelFilter`s interact.


#### Implementations

- `const OFF: LevelFilter`

- `const ERROR: LevelFilter`

- `const WARN: LevelFilter`

- `const INFO: LevelFilter`

- `const DEBUG: LevelFilter`

- `const TRACE: LevelFilter`

- `const fn from_level(level: Level) -> Self` — [`Level`](#level)

- `const fn into_level(self: Self) -> Option<Level>` — [`Level`](#level)

- `const ERROR_USIZE: usize`

- `const WARN_USIZE: usize`

- `const INFO_USIZE: usize`

- `const DEBUG_USIZE: usize`

- `const TRACE_USIZE: usize`

- `const OFF_USIZE: usize`

- `fn current() -> Self`

- `fn set_max(LevelFilter: LevelFilter)` — [`LevelFilter`](#levelfilter)

#### Trait Implementations

##### `impl Clone for LevelFilter`

- `fn clone(self: &Self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

##### `impl Copy for LevelFilter`

##### `impl Debug for LevelFilter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LevelFilter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelFilter`

##### `impl FromStr for LevelFilter`

- `type Err = ParseLevelFilterError`

- `fn from_str(from: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for LevelFilter`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for LevelFilter`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for LevelFilter`

- `fn eq(self: &Self, other: &Level) -> bool` — [`Level`](#level)

##### `impl PartialOrd for LevelFilter`

- `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>` — [`Level`](#level)

- `fn lt(self: &Self, other: &Level) -> bool` — [`Level`](#level)

- `fn le(self: &Self, other: &Level) -> bool` — [`Level`](#level)

- `fn gt(self: &Self, other: &Level) -> bool` — [`Level`](#level)

- `fn ge(self: &Self, other: &Level) -> bool` — [`Level`](#level)

##### `impl StructuralPartialEq for LevelFilter`

##### `impl<T> ToString for LevelFilter`

- `fn to_string(self: &Self) -> String`

### `Metadata<'a>`

```rust
struct Metadata<'a> {
    name: &'static str,
    target: &'a str,
    level: Level,
    module_path: Option<&'a str>,
    file: Option<&'a str>,
    line: Option<u32>,
    fields: field::FieldSet,
    kind: Kind,
}
```

Metadata describing a [`span`](span/index.md) or [`event`](event/index.md).

All spans and events have the following metadata:
- A [`name`](../serde_derive/internals/name/index.md), represented as a static string.
- A [`target`](../tracing_attributes/attr/kw/index.md), a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`](#level) type for details.
- The names of the [`fields`](../tracing_attributes/attr/kw/index.md) defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`](#subscriber)s when filtering spans and events, and it
may also be used as part of their data payload.

When created by the `event!` or `span!` macro, the metadata describing a
particular event or span is constructed statically and exists as a single
static instance. Thus, the overhead of creating the metadata is
_significantly_ lower than that of creating the actual span. Therefore,
filtering is based on metadata, rather than on the constructed span.

## Equality

In well-behaved applications, two `Metadata` with equal
[callsite identifiers] will be equal in all other ways (i.e., have the same
`name`, `target`, etc.). Consequently, in release builds, `Metadata::eq`
*only* checks that its arguments have equal callsites. However, the equality
of `Metadata`'s other fields is checked in debug builds.












#### Fields

- **`name`**: `&'static str`

  The name of the span described by this metadata.

- **`target`**: `&'a str`

  The part of the system that the span that this metadata describes
  occurred in.

- **`level`**: `Level`

  The level of verbosity of the described span.

- **`module_path`**: `Option<&'a str>`

  The name of the Rust module where the span occurred, or `None` if this
  could not be determined.

- **`file`**: `Option<&'a str>`

  The name of the source code file where the span occurred, or `None` if
  this could not be determined.

- **`line`**: `Option<u32>`

  The line number in the source code file where the span occurred, or
  `None` if this could not be determined.

- **`fields`**: `field::FieldSet`

  The names of the key-value fields attached to the described span or
  event.

- **`kind`**: `Kind`

  The kind of the callsite.

#### Implementations

- `const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self` — [`Level`](#level), [`FieldSet`](field/index.md), [`Kind`](#kind)

- `fn fields(self: &Self) -> &field::FieldSet` — [`FieldSet`](field/index.md)

- `fn level(self: &Self) -> &Level` — [`Level`](#level)

- `fn name(self: &Self) -> &'static str`

- `fn target(self: &Self) -> &'a str`

- `fn module_path(self: &Self) -> Option<&'a str>`

- `fn file(self: &Self) -> Option<&'a str>`

- `fn line(self: &Self) -> Option<u32>`

- `fn callsite(self: &Self) -> callsite::Identifier` — [`Identifier`](callsite/index.md)

- `fn is_event(self: &Self) -> bool`

- `fn is_span(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for Metadata<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Metadata<'_>`

##### `impl PartialEq for Metadata<'_>`

- `fn eq(self: &Self, other: &Self) -> bool`

### `Kind`

```rust
struct Kind(u8);
```

Indicates whether the callsite is a span or event.

#### Implementations

- `const EVENT_BIT: u8`

- `const SPAN_BIT: u8`

- `const HINT_BIT: u8`

- `const EVENT: Kind`

- `const SPAN: Kind`

- `const HINT: Kind`

- `fn is_span(self: &Self) -> bool`

- `fn is_event(self: &Self) -> bool`

- `fn is_hint(self: &Self) -> bool`

- `const fn hint(self: Self) -> Self`

#### Trait Implementations

##### `impl Clone for Kind`

- `fn clone(self: &Self) -> Kind` — [`Kind`](#kind)

##### `impl Debug for Kind`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Kind`

##### `impl PartialEq for Kind`

- `fn eq(self: &Self, other: &Kind) -> bool` — [`Kind`](#kind)

##### `impl StructuralPartialEq for Kind`

### `Interest`

```rust
struct Interest(InterestKind);
```

Indicates a [`Subscriber`](#subscriber)'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their `register_callsite` methods
in order to determine whether that span should be enabled or disabled.



#### Implementations

- `fn never() -> Self`

- `fn sometimes() -> Self`

- `fn always() -> Self`

- `fn is_never(self: &Self) -> bool`

- `fn is_sometimes(self: &Self) -> bool`

- `fn is_always(self: &Self) -> bool`

- `fn and(self: Self, rhs: Interest) -> Self` — [`Interest`](#interest)

#### Trait Implementations

##### `impl Clone for Interest`

- `fn clone(self: &Self) -> Interest` — [`Interest`](#interest)

##### `impl Debug for Interest`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

## Macros

### `identify_callsite!`

Statically constructs an [`Identifier`](callsite/index.md) for the provided [`Callsite`](#callsite).

This may be used in contexts such as static initializers.

For example:
```rust
use tracing_core::{callsite, identify_callsite};
use tracing_core::{Metadata, subscriber::Interest};
fn main() {
pub struct MyCallsite {
   // ...
}
impl callsite::Callsite for MyCallsite {
fn set_interest(&self, _: Interest) { unimplemented!() }
fn metadata(&self) -> &Metadata { unimplemented!() }
    // ...
}

static CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static CALLSITE_ID: callsite::Identifier = identify_callsite!(&CALLSITE);
}
```



### `metadata!`

Statically constructs new span [`metadata`](metadata/index.md).

/// For example:
```rust
use tracing_core::{callsite::Callsite, subscriber::Interest};
use tracing_core::metadata;
use tracing_core::metadata::{Kind, Level, Metadata};
fn main() {
pub struct MyCallsite { }
impl Callsite for MyCallsite {
fn set_interest(&self, _: Interest) { unimplemented!() }
fn metadata(&self) -> &Metadata { unimplemented!() }
}

static FOO_CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static FOO_METADATA: Metadata = metadata!{
    name: "foo",
    target: module_path!(),
    level: Level::DEBUG,
    fields: &["bar", "baz"],
    callsite: &FOO_CALLSITE,
    kind: Kind::SPAN,
};
}
```



