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













## Contents

- [Modules](#modules)
  - [`lazy`](#lazy)
  - [`callsite`](#callsite)
  - [`dispatcher`](#dispatcher)
  - [`event`](#event)
  - [`field`](#field)
  - [`metadata`](#metadata)
  - [`parent`](#parent)
  - [`span`](#span)
  - [`subscriber`](#subscriber)
  - [`sealed`](#sealed)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Macros](#macros)
  - [`identify_callsite!`](#identify_callsite)
  - [`metadata!`](#metadata)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`lazy`](#lazy) | mod |  |
| [`callsite`](#callsite) | mod | Callsites represent the source locations from which spans or events |
| [`dispatcher`](#dispatcher) | mod | Dispatches trace events to [`Subscriber`]s. |
| [`event`](#event) | mod | Events represent single points in time during the execution of a program. |
| [`field`](#field) | mod | `Span` and `Event` key-value data. |
| [`metadata`](#metadata) | mod | Metadata describing trace data. |
| [`parent`](#parent) | mod |  |
| [`span`](#span) | mod | Spans represent periods of time in the execution of a program. |
| [`subscriber`](#subscriber) | mod | Collectors collect and record trace data. |
| [`sealed`](#sealed) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`identify_callsite!`](#identify_callsite) | macro | Statically constructs an [`Identifier`] for the provided [`Callsite`]. |
| [`metadata!`](#metadata) | macro | Statically constructs new span [metadata]. |

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

- <span id="dispatch-none"></span>`fn none() -> Self`

- <span id="dispatch-new"></span>`fn new<S>(subscriber: S) -> Self`

- <span id="dispatch-registrar"></span>`fn registrar(&self) -> Registrar` — [`Registrar`](dispatcher/index.md)

- <span id="dispatch-downgrade"></span>`fn downgrade(&self) -> WeakDispatch` — [`WeakDispatch`](dispatcher/index.md)

- <span id="dispatch-subscriber"></span>`fn subscriber(&self) -> &dyn Subscriber + Send + Sync` — [`Subscriber`](#subscriber)

- <span id="dispatch-register-callsite"></span>`fn register_callsite(&self, metadata: &'static Metadata<'static>) -> subscriber::Interest` — [`Metadata`](#metadata), [`Interest`](#interest)

- <span id="dispatch-max-level-hint"></span>`fn max_level_hint(&self) -> Option<LevelFilter>` — [`LevelFilter`](#levelfilter)

- <span id="dispatch-new-span"></span>`fn new_span(&self, span: &span::Attributes<'_>) -> span::Id` — [`Attributes`](span/index.md), [`Id`](span/index.md)

- <span id="dispatch-record"></span>`fn record(&self, span: &span::Id, values: &span::Record<'_>)` — [`Id`](span/index.md), [`Record`](span/index.md)

- <span id="dispatch-record-follows-from"></span>`fn record_follows_from(&self, span: &span::Id, follows: &span::Id)` — [`Id`](span/index.md)

- <span id="dispatch-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool` — [`Metadata`](#metadata)

- <span id="dispatch-event"></span>`fn event(&self, event: &Event<'_>)` — [`Event`](#event)

- <span id="dispatch-enter"></span>`fn enter(&self, span: &span::Id)` — [`Id`](span/index.md)

- <span id="dispatch-exit"></span>`fn exit(&self, span: &span::Id)` — [`Id`](span/index.md)

- <span id="dispatch-clone-span"></span>`fn clone_span(&self, id: &span::Id) -> span::Id` — [`Id`](span/index.md)

- <span id="dispatch-drop-span"></span>`fn drop_span(&self, id: span::Id)` — [`Id`](span/index.md)

- <span id="dispatch-try-close"></span>`fn try_close(&self, id: span::Id) -> bool` — [`Id`](span/index.md)

- <span id="dispatch-current-span"></span>`fn current_span(&self) -> span::Current` — [`Current`](span/index.md)

- <span id="dispatch-is"></span>`fn is<T: Any>(&self) -> bool`

- <span id="dispatch-downcast-ref"></span>`fn downcast_ref<T: Any>(&self) -> Option<&T>`

#### Trait Implementations

##### `impl Clone for Dispatch`

- <span id="dispatch-clone"></span>`fn clone(&self) -> Dispatch` — [`Dispatch`](#dispatch)

##### `impl Debug for Dispatch`

- <span id="dispatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dispatch`

- <span id="dispatch-default"></span>`fn default() -> Self`

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

- <span id="event-dispatch"></span>`fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- <span id="event-new"></span>`fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- <span id="event-new-child-of"></span>`fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Id`](span/index.md), [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- <span id="event-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Id`](span/index.md), [`Metadata`](#metadata), [`ValueSet`](field/index.md)

- <span id="event-record"></span>`fn record(&self, visitor: &mut dyn field::Visit)` — [`Visit`](field/index.md)

- <span id="event-fields"></span>`fn fields(&self) -> field::Iter` — [`Iter`](field/index.md)

- <span id="event-metadata"></span>`fn metadata(&self) -> &'static Metadata<'static>` — [`Metadata`](#metadata)

- <span id="event-is-root"></span>`fn is_root(&self) -> bool`

- <span id="event-is-contextual"></span>`fn is_contextual(&self) -> bool`

- <span id="event-parent"></span>`fn parent(&self) -> Option<&Id>` — [`Id`](span/index.md)

#### Trait Implementations

##### `impl<'a> Debug for Event<'a>`

- <span id="event-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="field-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](callsite/index.md)

- <span id="field-name"></span>`fn name(&self) -> &'static str`

- <span id="field-index"></span>`fn index(&self) -> usize`

#### Trait Implementations

##### `impl AsRef for Field`

- <span id="field-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for Field`

- <span id="field-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Field`

- <span id="field-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Field`

- <span id="field-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Field`

##### `impl Hash for Field`

- <span id="field-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for Field`

- <span id="field-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> ToString for Field`

- <span id="field-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="level-error"></span>`const ERROR: Level`

- <span id="level-warn"></span>`const WARN: Level`

- <span id="level-info"></span>`const INFO: Level`

- <span id="level-debug"></span>`const DEBUG: Level`

- <span id="level-trace"></span>`const TRACE: Level`

- <span id="level-as-str"></span>`fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for Level`

- <span id="level-clone"></span>`fn clone(&self) -> Level` — [`Level`](#level)

##### `impl Copy for Level`

##### `impl Debug for Level`

- <span id="level-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Level`

- <span id="level-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl FromStr for Level`

- <span id="level-err"></span>`type Err = ParseLevelError`

- <span id="level-from-str"></span>`fn from_str(s: &str) -> Result<Self, ParseLevelError>` — [`ParseLevelError`](metadata/index.md)

##### `impl Hash for Level`

- <span id="level-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Level`

- <span id="level-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for Level`

- <span id="level-eq"></span>`fn eq(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl PartialOrd for Level`

- <span id="level-partial-cmp"></span>`fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](#levelfilter)

- <span id="level-lt"></span>`fn lt(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- <span id="level-le"></span>`fn le(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- <span id="level-gt"></span>`fn gt(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- <span id="level-ge"></span>`fn ge(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl StructuralPartialEq for Level`

##### `impl<T> ToString for Level`

- <span id="level-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="levelfilter-off"></span>`const OFF: LevelFilter`

- <span id="levelfilter-error"></span>`const ERROR: LevelFilter`

- <span id="levelfilter-warn"></span>`const WARN: LevelFilter`

- <span id="levelfilter-info"></span>`const INFO: LevelFilter`

- <span id="levelfilter-debug"></span>`const DEBUG: LevelFilter`

- <span id="levelfilter-trace"></span>`const TRACE: LevelFilter`

- <span id="levelfilter-from-level"></span>`const fn from_level(level: Level) -> Self` — [`Level`](#level)

- <span id="levelfilter-into-level"></span>`const fn into_level(self) -> Option<Level>` — [`Level`](#level)

- <span id="levelfilter-error-usize"></span>`const ERROR_USIZE: usize`

- <span id="levelfilter-warn-usize"></span>`const WARN_USIZE: usize`

- <span id="levelfilter-info-usize"></span>`const INFO_USIZE: usize`

- <span id="levelfilter-debug-usize"></span>`const DEBUG_USIZE: usize`

- <span id="levelfilter-trace-usize"></span>`const TRACE_USIZE: usize`

- <span id="levelfilter-off-usize"></span>`const OFF_USIZE: usize`

- <span id="levelfilter-current"></span>`fn current() -> Self`

- <span id="levelfilter-set-max"></span>`fn set_max(LevelFilter: LevelFilter)` — [`LevelFilter`](#levelfilter)

#### Trait Implementations

##### `impl Clone for LevelFilter`

- <span id="levelfilter-clone"></span>`fn clone(&self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

##### `impl Copy for LevelFilter`

##### `impl Debug for LevelFilter`

- <span id="levelfilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LevelFilter`

- <span id="levelfilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelFilter`

##### `impl FromStr for LevelFilter`

- <span id="levelfilter-err"></span>`type Err = ParseLevelFilterError`

- <span id="levelfilter-from-str"></span>`fn from_str(from: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for LevelFilter`

- <span id="levelfilter-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LevelFilter`

- <span id="levelfilter-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for LevelFilter`

- <span id="levelfilter-eq"></span>`fn eq(&self, other: &Level) -> bool` — [`Level`](#level)

##### `impl PartialOrd for LevelFilter`

- <span id="levelfilter-partial-cmp"></span>`fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering>` — [`Level`](#level)

- <span id="levelfilter-lt"></span>`fn lt(&self, other: &Level) -> bool` — [`Level`](#level)

- <span id="levelfilter-le"></span>`fn le(&self, other: &Level) -> bool` — [`Level`](#level)

- <span id="levelfilter-gt"></span>`fn gt(&self, other: &Level) -> bool` — [`Level`](#level)

- <span id="levelfilter-ge"></span>`fn ge(&self, other: &Level) -> bool` — [`Level`](#level)

##### `impl StructuralPartialEq for LevelFilter`

##### `impl<T> ToString for LevelFilter`

- <span id="levelfilter-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="metadata-new"></span>`const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self` — [`Level`](#level), [`FieldSet`](field/index.md), [`Kind`](#kind)

- <span id="metadata-fields"></span>`fn fields(&self) -> &field::FieldSet` — [`FieldSet`](field/index.md)

- <span id="metadata-level"></span>`fn level(&self) -> &Level` — [`Level`](#level)

- <span id="metadata-name"></span>`fn name(&self) -> &'static str`

- <span id="metadata-target"></span>`fn target(&self) -> &'a str`

- <span id="metadata-module-path"></span>`fn module_path(&self) -> Option<&'a str>`

- <span id="metadata-file"></span>`fn file(&self) -> Option<&'a str>`

- <span id="metadata-line"></span>`fn line(&self) -> Option<u32>`

- <span id="metadata-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](callsite/index.md)

- <span id="metadata-is-event"></span>`fn is_event(&self) -> bool`

- <span id="metadata-is-span"></span>`fn is_span(&self) -> bool`

#### Trait Implementations

##### `impl Debug for Metadata<'_>`

- <span id="metadata-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Metadata<'_>`

##### `impl PartialEq for Metadata<'_>`

- <span id="metadata-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Kind`

```rust
struct Kind(u8);
```

Indicates whether the callsite is a span or event.

#### Implementations

- <span id="kind-event-bit"></span>`const EVENT_BIT: u8`

- <span id="kind-span-bit"></span>`const SPAN_BIT: u8`

- <span id="kind-hint-bit"></span>`const HINT_BIT: u8`

- <span id="kind-event"></span>`const EVENT: Kind`

- <span id="kind-span"></span>`const SPAN: Kind`

- <span id="kind-hint"></span>`const HINT: Kind`

- <span id="kind-is-span"></span>`fn is_span(&self) -> bool`

- <span id="kind-is-event"></span>`fn is_event(&self) -> bool`

- <span id="kind-is-hint"></span>`fn is_hint(&self) -> bool`

- <span id="kind-hint"></span>`const fn hint(self) -> Self`

#### Trait Implementations

##### `impl Clone for Kind`

- <span id="kind-clone"></span>`fn clone(&self) -> Kind` — [`Kind`](#kind)

##### `impl Debug for Kind`

- <span id="kind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Kind`

##### `impl PartialEq for Kind`

- <span id="kind-eq"></span>`fn eq(&self, other: &Kind) -> bool` — [`Kind`](#kind)

##### `impl StructuralPartialEq for Kind`

### `Interest`

```rust
struct Interest(InterestKind);
```

Indicates a [`Subscriber`](#subscriber)'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their `register_callsite` methods
in order to determine whether that span should be enabled or disabled.



#### Implementations

- <span id="interest-never"></span>`fn never() -> Self`

- <span id="interest-sometimes"></span>`fn sometimes() -> Self`

- <span id="interest-always"></span>`fn always() -> Self`

- <span id="interest-is-never"></span>`fn is_never(&self) -> bool`

- <span id="interest-is-sometimes"></span>`fn is_sometimes(&self) -> bool`

- <span id="interest-is-always"></span>`fn is_always(&self) -> bool`

- <span id="interest-and"></span>`fn and(self, rhs: Interest) -> Self` — [`Interest`](#interest)

#### Trait Implementations

##### `impl Clone for Interest`

- <span id="interest-clone"></span>`fn clone(&self) -> Interest` — [`Interest`](#interest)

##### `impl Debug for Interest`

- <span id="interest-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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



