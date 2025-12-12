# Crate `tracing_core`

Core primitives for `tracing`.

`tracing` is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate defines the core
primitives of `tracing`.

This crate provides:

* [`span::Id`](span/index.md) identifies a span within the execution of a program.

* [`Event`](event/index.md) represents a single event within a trace.

* [`Subscriber`](subscriber/index.md), the trait implemented to collect trace data.

* [`Metadata`](metadata/index.md) and [`Callsite`](callsite/index.md) provide information describing spans and
  `Event`s.

* [`Field`](field/index.md), [`FieldSet`](field/index.md), [`Value`](field/index.md), and [`ValueSet`](field/index.md) represent the
  structured data attached to a span.

* [`Dispatch`](dispatcher/index.md) allows spans and events to be dispatched to `Subscriber`s.

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
  - [`Dispatch`](#dispatch)
  - [`Event`](#event)
  - [`Field`](#field)
  - [`Level`](#level)
  - [`LevelFilter`](#levelfilter)
  - [`Metadata`](#metadata)
  - [`Kind`](#kind)
  - [`Interest`](#interest)
- [Traits](#traits)
  - [`Callsite`](#callsite)
  - [`Subscriber`](#subscriber)
- [Macros](#macros)
  - [`identify_callsite!`](#identify-callsite)
  - [`metadata!`](#metadata)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`lazy`](#lazy) | mod |  |
| [`callsite`](#callsite) | mod | Callsites represent the source locations from which spans or events originate. |
| [`dispatcher`](#dispatcher) | mod | Dispatches trace events to [`Subscriber`]s. |
| [`event`](#event) | mod | Events represent single points in time during the execution of a program. |
| [`field`](#field) | mod | `Span` and `Event` key-value data. |
| [`metadata`](#metadata) | mod | Metadata describing trace data. |
| [`parent`](#parent) | mod |  |
| [`span`](#span) | mod | Spans represent periods of time in the execution of a program. |
| [`subscriber`](#subscriber) | mod | Collectors collect and record trace data. |
| [`sealed`](#sealed) | mod |  |
| [`Dispatch`](#dispatch) | struct |  |
| [`Event`](#event) | struct |  |
| [`Field`](#field) | struct |  |
| [`Level`](#level) | struct |  |
| [`LevelFilter`](#levelfilter) | struct |  |
| [`Metadata`](#metadata) | struct |  |
| [`Kind`](#kind) | struct |  |
| [`Interest`](#interest) | struct |  |
| [`Callsite`](#callsite) | trait |  |
| [`Subscriber`](#subscriber) | trait |  |
| [`identify_callsite!`](#identify-callsite) | macro | Statically constructs an [`Identifier`] for the provided [`Callsite`]. |
| [`metadata!`](#metadata) | macro | Statically constructs new span [metadata]. |

## Modules

- [`lazy`](lazy/index.md)
- [`callsite`](callsite/index.md) — Callsites represent the source locations from which spans or events
- [`dispatcher`](dispatcher/index.md) — Dispatches trace events to [`Subscriber`]s.
- [`event`](event/index.md) — Events represent single points in time during the execution of a program.
- [`field`](field/index.md) — `Span` and `Event` key-value data.
- [`metadata`](metadata/index.md) — Metadata describing trace data.
- [`parent`](parent/index.md)
- [`span`](span/index.md) — Spans represent periods of time in the execution of a program.
- [`subscriber`](subscriber/index.md) — Collectors collect and record trace data.
- [`sealed`](sealed/index.md)

## Structs

### `Dispatch`

```rust
struct Dispatch {
    subscriber: Kind<alloc::sync::Arc<dyn Subscriber + Send + Sync>>,
}
```

*Defined in [`tracing-core-0.1.35/src/dispatcher.rs:149-151`](../../.source_1765521767/tracing-core-0.1.35/src/dispatcher.rs#L149-L151)*

`Dispatch` trace data to a [`Subscriber`](subscriber/index.md).

#### Implementations

- <span id="dispatch-none"></span>`fn none() -> Self`

- <span id="dispatch-new"></span>`fn new<S>(subscriber: S) -> Self`

- <span id="dispatch-registrar"></span>`fn registrar(&self) -> Registrar` — [`Registrar`](dispatcher/index.md#registrar)

- <span id="dispatch-downgrade"></span>`fn downgrade(&self) -> WeakDispatch` — [`WeakDispatch`](dispatcher/index.md#weakdispatch)

- <span id="dispatch-subscriber"></span>`fn subscriber(&self) -> &dyn Subscriber + Send + Sync` — [`Subscriber`](subscriber/index.md#subscriber)

- <span id="dispatch-register-callsite"></span>`fn register_callsite(&self, metadata: &'static Metadata<'static>) -> subscriber::Interest` — [`Metadata`](metadata/index.md#metadata), [`Interest`](subscriber/index.md#interest)

- <span id="dispatch-max-level-hint"></span>`fn max_level_hint(&self) -> Option<LevelFilter>` — [`LevelFilter`](metadata/index.md#levelfilter)

- <span id="dispatch-new-span"></span>`fn new_span(&self, span: &span::Attributes<'_>) -> span::Id` — [`Attributes`](span/index.md#attributes), [`Id`](span/index.md#id)

- <span id="dispatch-record"></span>`fn record(&self, span: &span::Id, values: &span::Record<'_>)` — [`Id`](span/index.md#id), [`Record`](span/index.md#record)

- <span id="dispatch-record-follows-from"></span>`fn record_follows_from(&self, span: &span::Id, follows: &span::Id)` — [`Id`](span/index.md#id)

- <span id="dispatch-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool` — [`Metadata`](metadata/index.md#metadata)

- <span id="dispatch-event"></span>`fn event(&self, event: &Event<'_>)` — [`Event`](event/index.md#event)

- <span id="dispatch-enter"></span>`fn enter(&self, span: &span::Id)` — [`Id`](span/index.md#id)

- <span id="dispatch-exit"></span>`fn exit(&self, span: &span::Id)` — [`Id`](span/index.md#id)

- <span id="dispatch-clone-span"></span>`fn clone_span(&self, id: &span::Id) -> span::Id` — [`Id`](span/index.md#id)

- <span id="dispatch-drop-span"></span>`fn drop_span(&self, id: span::Id)` — [`Id`](span/index.md#id)

- <span id="dispatch-try-close"></span>`fn try_close(&self, id: span::Id) -> bool` — [`Id`](span/index.md#id)

- <span id="dispatch-current-span"></span>`fn current_span(&self) -> span::Current` — [`Current`](span/index.md#current)

- <span id="dispatch-is"></span>`fn is<T: Any>(&self) -> bool`

- <span id="dispatch-downcast-ref"></span>`fn downcast_ref<T: Any>(&self) -> Option<&T>`

#### Trait Implementations

##### `impl Clone for Dispatch`

- <span id="dispatch-clone"></span>`fn clone(&self) -> Dispatch` — [`Dispatch`](dispatcher/index.md#dispatch)

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

*Defined in [`tracing-core-0.1.35/src/event.rs:23-27`](../../.source_1765521767/tracing-core-0.1.35/src/event.rs#L23-L27)*

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

- <span id="event-dispatch"></span>`fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Metadata`](metadata/index.md#metadata), [`ValueSet`](field/index.md#valueset)

- <span id="event-new"></span>`fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Metadata`](metadata/index.md#metadata), [`ValueSet`](field/index.md#valueset)

- <span id="event-new-child-of"></span>`fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Id`](span/index.md#id), [`Metadata`](metadata/index.md#metadata), [`ValueSet`](field/index.md#valueset)

- <span id="event-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Id`](span/index.md#id), [`Metadata`](metadata/index.md#metadata), [`ValueSet`](field/index.md#valueset)

- <span id="event-record"></span>`fn record(&self, visitor: &mut dyn field::Visit)` — [`Visit`](field/index.md#visit)

- <span id="event-fields"></span>`fn fields(&self) -> field::Iter` — [`Iter`](field/index.md#iter)

- <span id="event-metadata"></span>`fn metadata(&self) -> &'static Metadata<'static>` — [`Metadata`](metadata/index.md#metadata)

- <span id="event-is-root"></span>`fn is_root(&self) -> bool`

- <span id="event-is-contextual"></span>`fn is_contextual(&self) -> bool`

- <span id="event-parent"></span>`fn parent(&self) -> Option<&Id>` — [`Id`](span/index.md#id)

#### Trait Implementations

##### `impl Debug for Event<'a>`

- <span id="event-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Field`

```rust
struct Field {
    i: usize,
    fields: FieldSet,
}
```

*Defined in [`tracing-core-0.1.35/src/field.rs:134-137`](../../.source_1765521767/tracing-core-0.1.35/src/field.rs#L134-L137)*

An opaque key allowing _O_(1) access to a field in a `Span`'s key-value
data.

As keys are defined by the _metadata_ of a span, rather than by an
individual instance of a span, a key may be used to access the same field
across all instances of a given span with the same metadata. Thus, when a
subscriber observes a new span, it need only access a field by name _once_,
and use the key for that name for all other accesses.

#### Implementations

- <span id="field-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](callsite/index.md#identifier)

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

##### `impl ToString for Field`

- <span id="field-to-string"></span>`fn to_string(&self) -> String`

### `Level`

```rust
struct Level(LevelInner);
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:221`](../../.source_1765521767/tracing-core-0.1.35/src/metadata.rs#L221)*

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

The [`LevelFilter`](metadata/index.md) type is provided to enable filtering traces by
verbosity. `Level`s can be compared against [`LevelFilter`](metadata/index.md)s, and
[`LevelFilter`](metadata/index.md) has a variant for each `Level`, which compares analogously
to that level. In addition, [`LevelFilter`](metadata/index.md) adds a `LevelFilter::OFF`
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

Below is a simple example of how a [`Subscriber`](subscriber/index.md) could implement filtering through
a [`LevelFilter`](metadata/index.md). When a span or event is recorded, the `Subscriber::enabled` method
compares the span or event's `Level` against the configured [`LevelFilter`](metadata/index.md).
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

- <span id="level-const-error"></span>`const ERROR: Level`

- <span id="level-const-warn"></span>`const WARN: Level`

- <span id="level-const-info"></span>`const INFO: Level`

- <span id="level-const-debug"></span>`const DEBUG: Level`

- <span id="level-const-trace"></span>`const TRACE: Level`

- <span id="level-as-str"></span>`fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for Level`

- <span id="level-clone"></span>`fn clone(&self) -> Level` — [`Level`](metadata/index.md#level)

##### `impl Copy for Level`

##### `impl Debug for Level`

- <span id="level-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Level`

- <span id="level-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl FromStr for Level`

- <span id="level-fromstr-type-err"></span>`type Err = ParseLevelError`

- <span id="level-from-str"></span>`fn from_str(s: &str) -> Result<Self, ParseLevelError>` — [`ParseLevelError`](metadata/index.md#parselevelerror)

##### `impl Hash for Level`

- <span id="level-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Level`

- <span id="level-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for Level`

- <span id="level-eq"></span>`fn eq(&self, other: &Level) -> bool` — [`Level`](metadata/index.md#level)

##### `impl PartialOrd for Level`

- <span id="level-partial-cmp"></span>`fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering>` — [`Level`](metadata/index.md#level)

- <span id="level-lt"></span>`fn lt(&self, other: &Level) -> bool` — [`Level`](metadata/index.md#level)

- <span id="level-le"></span>`fn le(&self, other: &Level) -> bool` — [`Level`](metadata/index.md#level)

- <span id="level-gt"></span>`fn gt(&self, other: &Level) -> bool` — [`Level`](metadata/index.md#level)

- <span id="level-ge"></span>`fn ge(&self, other: &Level) -> bool` — [`Level`](metadata/index.md#level)

##### `impl StructuralPartialEq for Level`

##### `impl ToString for Level`

- <span id="level-to-string"></span>`fn to_string(&self) -> String`

### `LevelFilter`

```rust
struct LevelFilter(Option<Level>);
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:239`](../../.source_1765521767/tracing-core-0.1.35/src/metadata.rs#L239)*

A filter comparable to a verbosity [`Level`](metadata/index.md).

If a [`Level`](metadata/index.md) is considered less than or equal to a `LevelFilter`, it
should be considered enabled; if greater than the `LevelFilter`, that level
is disabled. See `LevelFilter::current` for more details.

Note that this is essentially identical to the `Level` type, but with the
addition of an `OFF` level that completely disables all trace
instrumentation.

See the documentation for the [`Level`](metadata/index.md) type to see how `Level`s
and `LevelFilter`s interact.


#### Implementations

- <span id="levelfilter-const-off"></span>`const OFF: LevelFilter`

- <span id="levelfilter-const-error"></span>`const ERROR: LevelFilter`

- <span id="levelfilter-const-warn"></span>`const WARN: LevelFilter`

- <span id="levelfilter-const-info"></span>`const INFO: LevelFilter`

- <span id="levelfilter-const-debug"></span>`const DEBUG: LevelFilter`

- <span id="levelfilter-const-trace"></span>`const TRACE: LevelFilter`

- <span id="levelfilter-from-level"></span>`const fn from_level(level: Level) -> Self` — [`Level`](metadata/index.md#level)

- <span id="levelfilter-into-level"></span>`const fn into_level(self) -> Option<Level>` — [`Level`](metadata/index.md#level)

- <span id="levelfilter-const-error-usize"></span>`const ERROR_USIZE: usize`

- <span id="levelfilter-const-warn-usize"></span>`const WARN_USIZE: usize`

- <span id="levelfilter-const-info-usize"></span>`const INFO_USIZE: usize`

- <span id="levelfilter-const-debug-usize"></span>`const DEBUG_USIZE: usize`

- <span id="levelfilter-const-trace-usize"></span>`const TRACE_USIZE: usize`

- <span id="levelfilter-const-off-usize"></span>`const OFF_USIZE: usize`

- <span id="levelfilter-current"></span>`fn current() -> Self`

- <span id="levelfilter-set-max"></span>`fn set_max(LevelFilter: LevelFilter)` — [`LevelFilter`](metadata/index.md#levelfilter)

#### Trait Implementations

##### `impl Clone for LevelFilter`

- <span id="levelfilter-clone"></span>`fn clone(&self) -> LevelFilter` — [`LevelFilter`](metadata/index.md#levelfilter)

##### `impl Copy for LevelFilter`

##### `impl Debug for LevelFilter`

- <span id="levelfilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LevelFilter`

- <span id="levelfilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelFilter`

##### `impl FromStr for LevelFilter`

- <span id="levelfilter-fromstr-type-err"></span>`type Err = ParseLevelFilterError`

- <span id="levelfilter-from-str"></span>`fn from_str(from: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for LevelFilter`

- <span id="levelfilter-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LevelFilter`

- <span id="levelfilter-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for LevelFilter`

- <span id="levelfilter-eq"></span>`fn eq(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](metadata/index.md#levelfilter)

##### `impl PartialOrd for Level`

- <span id="level-partial-cmp"></span>`fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](metadata/index.md#levelfilter)

- <span id="level-lt"></span>`fn lt(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](metadata/index.md#levelfilter)

- <span id="level-le"></span>`fn le(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](metadata/index.md#levelfilter)

- <span id="level-gt"></span>`fn gt(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](metadata/index.md#levelfilter)

- <span id="level-ge"></span>`fn ge(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](metadata/index.md#levelfilter)

##### `impl StructuralPartialEq for LevelFilter`

##### `impl ToString for LevelFilter`

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

*Defined in [`tracing-core-0.1.35/src/metadata.rs:57-86`](../../.source_1765521767/tracing-core-0.1.35/src/metadata.rs#L57-L86)*

Metadata describing a [`span`](span/index.md) or [`event`](event/index.md).

All spans and events have the following metadata:
- A [`name`](../serde_derive/internals/name/index.md), represented as a static string.
- A [`target`](../tracing_attributes/attr/kw/index.md), a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`](metadata/index.md) type for details.
- The names of the [`fields`](../tracing_attributes/attr/kw/index.md) defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`](subscriber/index.md)s when filtering spans and events, and it
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

- <span id="metadata-new"></span>`const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self` — [`Level`](metadata/index.md#level), [`FieldSet`](field/index.md#fieldset), [`Kind`](metadata/index.md#kind)

- <span id="metadata-fields"></span>`fn fields(&self) -> &field::FieldSet` — [`FieldSet`](field/index.md#fieldset)

- <span id="metadata-level"></span>`fn level(&self) -> &Level` — [`Level`](metadata/index.md#level)

- <span id="metadata-name"></span>`fn name(&self) -> &'static str`

- <span id="metadata-target"></span>`fn target(&self) -> &'a str`

- <span id="metadata-module-path"></span>`fn module_path(&self) -> Option<&'a str>`

- <span id="metadata-file"></span>`fn file(&self) -> Option<&'a str>`

- <span id="metadata-line"></span>`fn line(&self) -> Option<u32>`

- <span id="metadata-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](callsite/index.md#identifier)

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

*Defined in [`tracing-core-0.1.35/src/metadata.rs:90`](../../.source_1765521767/tracing-core-0.1.35/src/metadata.rs#L90)*

Indicates whether the callsite is a span or event.

#### Implementations

- <span id="kind-const-event-bit"></span>`const EVENT_BIT: u8`

- <span id="kind-const-span-bit"></span>`const SPAN_BIT: u8`

- <span id="kind-const-hint-bit"></span>`const HINT_BIT: u8`

- <span id="kind-const-event"></span>`const EVENT: Kind`

- <span id="kind-const-span"></span>`const SPAN: Kind`

- <span id="kind-const-hint"></span>`const HINT: Kind`

- <span id="kind-is-span"></span>`fn is_span(&self) -> bool`

- <span id="kind-is-event"></span>`fn is_event(&self) -> bool`

- <span id="kind-is-hint"></span>`fn is_hint(&self) -> bool`

- <span id="kind-hint"></span>`const fn hint(self) -> Self`

#### Trait Implementations

##### `impl Clone for Kind`

- <span id="kind-clone"></span>`fn clone(&self) -> Kind` — [`Kind`](metadata/index.md#kind)

##### `impl Debug for Kind`

- <span id="kind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Kind`

##### `impl PartialEq for Kind`

- <span id="kind-eq"></span>`fn eq(&self, other: &Kind) -> bool` — [`Kind`](metadata/index.md#kind)

##### `impl StructuralPartialEq for Kind`

### `Interest`

```rust
struct Interest(InterestKind);
```

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:589`](../../.source_1765521767/tracing-core-0.1.35/src/subscriber.rs#L589)*

Indicates a [`Subscriber`](subscriber/index.md)'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their `register_callsite` methods
in order to determine whether that span should be enabled or disabled.



#### Implementations

- <span id="interest-never"></span>`fn never() -> Self`

- <span id="interest-sometimes"></span>`fn sometimes() -> Self`

- <span id="interest-always"></span>`fn always() -> Self`

- <span id="interest-is-never"></span>`fn is_never(&self) -> bool`

- <span id="interest-is-sometimes"></span>`fn is_sometimes(&self) -> bool`

- <span id="interest-is-always"></span>`fn is_always(&self) -> bool`

- <span id="interest-and"></span>`fn and(self, rhs: Interest) -> Self` — [`Interest`](subscriber/index.md#interest)

#### Trait Implementations

##### `impl Clone for Interest`

- <span id="interest-clone"></span>`fn clone(&self) -> Interest` — [`Interest`](subscriber/index.md#interest)

##### `impl Debug for Interest`

- <span id="interest-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Callsite`

```rust
trait Callsite: Sync { ... }
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:125-170`](../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L125-L170)*

Trait implemented by callsites.

These functions are only intended to be called by the callsite registry, which
correctly handles determining the common interest between all subscribers.

See the [module-level documentation](crate::callsite) for details on
callsites.

#### Required Methods

- `fn set_interest(&self, interest: Interest)`

  Sets the [`Interest`](subscriber/index.md) for this callsite.

- `fn metadata(&self) -> &Metadata<'_>`

  Returns the [`metadata`](metadata/index.md) associated with the callsite.

#### Implementors

- [`DefaultCallsite`](callsite/index.md#defaultcallsite)

### `Subscriber`

```rust
trait Subscriber: 'static { ... }
```

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:80-499`](../../.source_1765521767/tracing-core-0.1.35/src/subscriber.rs#L80-L499)*

Trait representing the functions required to collect trace data.

Crates that provide implementations of methods for collecting or recording
trace data should implement the `Subscriber` interface. This trait is
intended to represent fundamental primitives for collecting trace events and
spans — other libraries may offer utility functions and types to make
subscriber implementations more modular or improve the ergonomics of writing
subscribers.

A subscriber is responsible for the following:
- Registering new spans as they are created, and providing them with span
  IDs. Implicitly, this means the subscriber may determine the strategy for
  determining span equality.
- Recording the attachment of field values and follows-from annotations to
  spans.
- Filtering spans and events, and determining when those filters must be
  invalidated.
- Observing spans as they are entered, exited, and closed, and events as
  they occur.

When a span is entered or exited, the subscriber is provided only with the
[ID] with which it tagged that span when it was created. This means
that it is up to the subscriber to determine whether and how span _data_ —
the fields and metadata describing the span — should be stored. The
`new_span` function is called when a new span is created, and at that
point, the subscriber _may_ choose to store the associated data if it will
be referenced again. However, if the data has already been recorded and will
not be needed by the implementations of `enter` and `exit`, the subscriber
may freely discard that data without allocating space to store it.

## Overriding default impls

Some trait methods on `Subscriber` have default implementations, either in
order to reduce the surface area of implementing `Subscriber`, or for
backward-compatibility reasons. However, many subscribers will likely want
to override these default implementations.

The following methods are likely of interest:

- `register_callsite` is called once for each callsite from which a span
  event may originate, and returns an [`Interest`](subscriber/index.md) value describing whether or
  not the subscriber wishes to see events or spans from that callsite. By
  default, it calls `enabled`, and returns `Interest::always()` if
  `enabled` returns true, or `Interest::never()` if enabled returns false.
  However, if the subscriber's interest can change dynamically at runtime,
  it may want to override this function to return `Interest::sometimes()`.
  Additionally, subscribers which wish to perform a behaviour once for each
  callsite, such as allocating storage for data related to that callsite,
  can perform it in `register_callsite`.

  See also the [documentation on the callsite registry][cs-reg] for details
  on `register_callsite`.

- `event_enabled` is called once before every call to the [`event`](event/index.md)
  method. This can be used to implement filtering on events once their field
  values are known, but before any processing is done in the `event` method.
- `clone_span` is called every time a span ID is cloned, and `try_close`
  is called when a span ID is dropped. By default, these functions do
  nothing. However, they can be used to implement reference counting for
  spans, allowing subscribers to free storage for span data and to determine
  when a span has _closed_ permanently (rather than being exited).
  Subscribers which store per-span data or which need to track span closures
  should override these functions together.










#### Required Methods

- `fn enabled(&self, metadata: &Metadata<'_>) -> bool`

  Returns true if a span or event with the specified [`metadata`](metadata/index.md) would be

- `fn new_span(&self, span: &span::Attributes<'_>) -> span::Id`

  Visit the construction of a new span, returning a new [span ID] for the

- `fn record(&self, span: &span::Id, values: &span::Record<'_>)`

  Record a set of values on a span.

- `fn record_follows_from(&self, span: &span::Id, follows: &span::Id)`

  Adds an indication that `span` follows from the span with the id

- `fn event(&self, event: &Event<'_>)`

  Records that an [`Event`](event/index.md) has occurred.

- `fn enter(&self, span: &span::Id)`

  Records that a span has been entered.

- `fn exit(&self, span: &span::Id)`

  Records that a span has been exited.

#### Provided Methods

- `fn on_register_dispatch(&self, subscriber: &Dispatch)`

  Invoked when this subscriber becomes a [`Dispatch`](dispatcher/index.md).

- `fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest`

  Registers a new [`callsite`](callsite/index.md) with this subscriber, returning whether or not

- `fn max_level_hint(&self) -> Option<LevelFilter>`

  Returns the highest [verbosity level][`level`](../tracing_attributes/attr/kw/index.md) that this `Subscriber` will

- `fn event_enabled(&self, event: &Event<'_>) -> bool`

  Determine if an [`Event`](event/index.md) should be recorded.

- `fn clone_span(&self, id: &span::Id) -> span::Id`

  Notifies the subscriber that a [span ID] has been cloned.

- `fn drop_span(&self, _id: span::Id)`

  **This method is deprecated.**

- `fn try_close(&self, id: span::Id) -> bool`

  Notifies the subscriber that a [span ID] has been dropped, and returns

- `fn current_span(&self) -> span::Current`

  Returns a type representing this subscriber's view of the current span.

- `fn downcast_raw(&self, id: TypeId) -> Option<*const ()>`

  If `self` is the same type as the provided `TypeId`, returns an untyped

#### Implementors

- [`NoSubscriber`](subscriber/index.md#nosubscriber)
- `alloc::boxed::Box<S>`
- `alloc::sync::Arc<S>`

## Macros

### `identify_callsite!`

*Defined in [`tracing-core-0.1.35/src/lib.rs:192-196`](../../.source_1765521767/tracing-core-0.1.35/src/lib.rs#L192-L196)*

Statically constructs an [`Identifier`](callsite/index.md) for the provided [`Callsite`](callsite/index.md).

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

*Defined in [`tracing-core-0.1.35/src/lib.rs:230-267`](../../.source_1765521767/tracing-core-0.1.35/src/lib.rs#L230-L267)*

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



