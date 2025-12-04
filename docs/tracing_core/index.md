# Crate `tracing_core`

Core primitives for `tracing`.

[`tracing`](../tracing/index.md) is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information. This crate defines the core
primitives of `tracing`.

This crate provides:

* [`Id`](tracing_core/span/index.md) identifies a span within the execution of a program.

* [`Event`](tracing_core/event/index.md) represents a single event within a trace.

* [`Subscriber`](tracing_core/subscriber/index.md), the trait implemented to collect trace data.

* [`Metadata`](tracing_core/metadata/index.md) and [`Callsite`](tracing_core/callsite/index.md) provide information describing spans and
  `Event`s.

* [`Field`](tracing_core/field/index.md), [`FieldSet`](tracing_core/field/index.md), [`Value`](tracing_core/field/index.md), and [`ValueSet`](tracing_core/field/index.md) represent the
  structured data attached to a span.

* [`Dispatch`](tracing_core/dispatcher/index.md) allows spans and events to be dispatched to `Subscriber`s.

In addition, it defines the global callsite registry and per-thread current
dispatcher which other components of the tracing system rely on.

*Compiler support: [requires `rustc` 1.65+][msrv](#msrv)*

[msrv](#msrv): #supported-rust-versions

## Usage

Application authors will typically not use this crate directly. Instead,
they will use the [`tracing`](../tracing/index.md) crate, which provides a much more
fully-featured API. However, this crate's API will change very infrequently,
so it may be used when dependencies must be very stable.

`Subscriber` implementations may depend on `tracing-core` rather than
`tracing`, as the additional APIs provided by `tracing` are primarily useful
for instrumenting libraries and applications, and are generally not
necessary for `Subscriber` implementations.

The [`tokio-rs/tracing`](#tokio-rstracing) repository contains less stable crates designed to
be used with the `tracing` ecosystem. It includes a collection of
`Subscriber` implementations, as well as utility and adapter crates.

## Crate Feature Flags

The following crate [feature flags] are available:

* `std`: Depend on the Rust standard library (enabled by default).

  `no_std` users may disable this feature with `default-features = false`:

  ```toml
  [dependencies](#dependencies)
  tracing-core = { version = "0.1.22", default-features = false }
  ```

  **Note**:`tracing-core`'s `no_std` support requires `liballoc`.

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables support for recording [field values] using the
  [`valuable`](#valuable) crate.

#### Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
env variable when running `cargo` commands:

```shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```
Alternatively, the following can be added to the `.cargo/config` file in a
project to automatically enable the cfg flag for that project:

```toml
[build](#build)
rustflags = ["--cfg", "tracing_unstable"]
```

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[field values]: crate::field

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

- [`callsite`](callsite/index.md) - Callsites represent the source locations from which spans or events
- [`dispatcher`](dispatcher/index.md) - Dispatches trace events to [`Subscriber`]s.
- [`event`](event/index.md) - Events represent single points in time during the execution of a program.
- [`field`](field/index.md) - `Span` and `Event` key-value data.
- [`metadata`](metadata/index.md) - Metadata describing trace data.
- [`span`](span/index.md) - Spans represent periods of time in the execution of a program.
- [`subscriber`](subscriber/index.md) - Collectors collect and record trace data.

## Structs

### `Dispatch`

```rust
struct Dispatch {
}
```

`Dispatch` trace data to a [`Subscriber`](tracing_core/subscriber/index.md).

#### Implementations

- `fn none() -> Self`
  Returns a new `Dispatch` that discards events and spans.

- `fn new<S>(subscriber: S) -> Self`
  Returns a `Dispatch` that forwards to the given [`Subscriber`].

- `fn downgrade(self: &Self) -> WeakDispatch`
  Creates a [`WeakDispatch`] from this `Dispatch`.

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> subscriber::Interest`
  Registers a new callsite with this subscriber, returning whether or not

- `fn new_span(self: &Self, span: &span::Attributes<'_>) -> span::Id`
  Record the construction of a new span, returning a new [ID] for the

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)`
  Record a set of values on a span.

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`
  Adds an indication that `span` follows from the span with the id

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`
  Returns true if a span with the specified [metadata] would be

- `fn event(self: &Self, event: &Event<'_>)`
  Records that an [`Event`] has occurred.

- `fn enter(self: &Self, span: &span::Id)`
  Records that a span has been can_enter.

- `fn exit(self: &Self, span: &span::Id)`
  Records that a span has been exited.

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id`
  Notifies the subscriber that a [span ID] has been cloned.

- `fn drop_span(self: &Self, id: span::Id)`
  Notifies the subscriber that a [span ID] has been dropped.

- `fn try_close(self: &Self, id: span::Id) -> bool`
  Notifies the subscriber that a [span ID] has been dropped, and returns

- `fn current_span(self: &Self) -> span::Current`
  Returns a type representing this subscriber's view of the current span.

- `fn is<T: Any>(self: &Self) -> bool`
  Returns `true` if this `Dispatch` forwards to a `Subscriber` of type

- `fn downcast_ref<T: Any>(self: &Self) -> Option<&T>`
  Returns some reference to the `Subscriber` this `Dispatch` forwards to

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<S>`

- `fn from(subscriber: S) -> Self`

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

- `fn clone(self: &Self) -> Dispatch`

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

##### `impl Default`

- `fn default() -> Self`
  Returns the current default dispatcher

### `Event<'a>`

```rust
struct Event<'a> {
}
```

`Event`s represent single points in time where something occurred during the
execution of a program.

An `Event` can be compared to a log record in unstructured logging, but with
two key differences:
- `Event`s exist _within the context of a [span](#span)_. Unlike log lines, they
  may be located within the trace tree, allowing visibility into the
  _temporal_ context in which the event occurred, as well as the source
  code location.
- Like spans, `Event`s have structured key-value data known as _[fields](#fields)_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.

[span](#span): super::span
[fields](#fields): super::field

#### Implementations

- `fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)`
  Constructs a new `Event` with the specified metadata and set of values,

- `fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self`
  Returns a new `Event` in the current span, with the specified metadata

- `fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self`
  Returns a new `Event` as a child of the specified span, with the

- `fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)`
  Constructs a new `Event` with the specified metadata and set of values,

- `fn record(self: &Self, visitor: &mut dyn field::Visit)`
  Visits all the fields on this `Event` with the specified [visitor].

- `fn fields(self: &Self) -> field::Iter`
  Returns an iterator over the set of values on this `Event`.

- `fn metadata(self: &Self) -> &'static Metadata<'static>`
  Returns [metadata] describing this `Event`.

- `fn is_root(self: &Self) -> bool`
  Returns true if the new event should be a root.

- `fn is_contextual(self: &Self) -> bool`
  Returns true if the new event's parent should be determined based on the

- `fn parent(self: &Self) -> Option<&Id>`
  Returns the new event's explicitly-specified parent, if there is one.

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Field`

```rust
struct Field {
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

- `fn callsite(self: &Self) -> callsite::Identifier`
  Returns an [`Identifier`] that uniquely identifies the [`Callsite`]

- `fn name(self: &Self) -> &'static str`
  Returns a string representing the name of the field.

- `fn index(self: &Self) -> usize`
  Returns the index of this field in its [`FieldSet`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

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

### `Level`

```rust
struct Level();
```

Describes the level of verbosity of a span or event.

# Comparing Levels

`Level` implements the [`PartialOrd`](#partialord) and [`Ord`](#ord) traits, allowing two
`Level`s to be compared to determine which is considered more or less
verbose. Levels which are more verbose are considered "greater than" levels
which are less verbose, with [`Level::ERROR`](#error) considered the lowest, and
[`Level::TRACE`](#trace) considered the highest.

For example:
```
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
development, [`DEBUG`](#debug)-level traces may be enabled by default. When running in
production, only [`INFO`](#info)-level and lower traces might be enabled. Libraries
may include very verbose diagnostics at the [`DEBUG`](#debug) and/or [`TRACE`](#trace) levels.
Applications using those libraries typically chose to ignore those traces. However, when
debugging an issue involving said libraries, it may be useful to temporarily
enable the more verbose traces.

The [`LevelFilter`](tracing_core/metadata/index.md) type is provided to enable filtering traces by
verbosity. `Level`s can be compared against [`LevelFilter`](tracing_core/metadata/index.md)s, and
[`LevelFilter`](tracing_core/metadata/index.md) has a variant for each `Level`, which compares analogously
to that level. In addition, [`LevelFilter`](tracing_core/metadata/index.md) adds a [`LevelFilter::OFF`](#off)
variant, which is considered "less verbose" than every other `Level`. This is
intended to allow filters to completely disable tracing in a particular context.

For example:
```
use tracing_core::{Level, LevelFilter};

assert!(LevelFilter::OFF < Level::TRACE);
assert!(LevelFilter::TRACE > Level::DEBUG);
assert!(LevelFilter::ERROR < Level::WARN);
assert!(LevelFilter::INFO <= Level::DEBUG);
assert!(LevelFilter::INFO >= Level::INFO);
```

## Examples

Below is a simple example of how a [`Subscriber`](tracing_core/subscriber/index.md) could implement filtering through
a [`LevelFilter`](tracing_core/metadata/index.md). When a span or event is recorded, the [`Subscriber::enabled`](#enabled) method
compares the span or event's `Level` against the configured [`LevelFilter`](tracing_core/metadata/index.md).
The optional [`Subscriber::max_level_hint`](#max-level-hint) method can also be implemented to allow spans
and events above a maximum verbosity level to be skipped more efficiently,
often improving performance in short-lived programs.

```
use tracing_core::{span, Event, Level, LevelFilter, Subscriber, Metadata};
# use tracing_core::span::{Id, Record, Current};

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
        # drop(span); Id::from_u64(1)
    }

    fn event(&self, event: &Event<'_>) {
        // ...
        # drop(event);
    }

    // ...
    # fn enter(&self, _: &Id) {}
    # fn exit(&self, _: &Id) {}
    # fn record(&self, _: &Id, _: &Record<'_>) {}
    # fn record_follows_from(&self, _: &Id, _: &Id) {}
}
```

It is worth noting that the `tracing-subscriber` crate provides [additional
APIs][envfilter](#envfilter) for performing more sophisticated filtering, such as
enabling different levels based on which module or crate a span or event is
recorded in.






[envfilter](#envfilter): https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html

#### Implementations

- `const ERROR: Level`

- `const WARN: Level`

- `const INFO: Level`

- `const DEBUG: Level`

- `const TRACE: Level`

- `fn as_str(self: &Self) -> &'static str`
  Returns the string representation of the `Level`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = ParseLevelError`

- `fn from_str(s: &str) -> Result<Self, ParseLevelError>`

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

- `fn clone(self: &Self) -> Level`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Level) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LevelFilter) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>`

- `fn lt(self: &Self, other: &LevelFilter) -> bool`

- `fn le(self: &Self, other: &LevelFilter) -> bool`

- `fn gt(self: &Self, other: &LevelFilter) -> bool`

- `fn ge(self: &Self, other: &LevelFilter) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>`

- `fn lt(self: &Self, other: &Level) -> bool`

- `fn le(self: &Self, other: &Level) -> bool`

- `fn gt(self: &Self, other: &Level) -> bool`

- `fn ge(self: &Self, other: &Level) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

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

### `LevelFilter`

```rust
struct LevelFilter();
```

A filter comparable to a verbosity [`Level`](tracing_core/metadata/index.md).

If a [`Level`](tracing_core/metadata/index.md) is considered less than or equal to a `LevelFilter`, it
should be considered enabled; if greater than the `LevelFilter`, that level
is disabled. See [`LevelFilter::current`](#current) for more details.

Note that this is essentially identical to the `Level` type, but with the
addition of an [`OFF`](#off) level that completely disables all trace
instrumentation.

See the documentation for the [`Level`](tracing_core/metadata/index.md) type to see how `Level`s
and `LevelFilter`s interact.


#### Implementations

- `const OFF: LevelFilter`

- `const ERROR: LevelFilter`

- `const WARN: LevelFilter`

- `const INFO: LevelFilter`

- `const DEBUG: LevelFilter`

- `const TRACE: LevelFilter`

- `const fn from_level(level: Level) -> Self`
  Returns a `LevelFilter` that enables spans and events with verbosity up

- `const fn into_level(self: Self) -> Option<Level>`
  Returns the most verbose [`Level`] that this filter accepts, or `None`

- `fn current() -> Self`
  Returns a `LevelFilter` that matches the most verbose [`Level`] that any

#### Trait Implementations

##### `impl From`

- `fn from(level: Option<Level>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(level: Level) -> Self`

##### `impl FromStr`

- `type Err = ParseLevelFilterError`

- `fn from_str(from: &str) -> Result<Self, <Self as >::Err>`

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

- `fn clone(self: &Self) -> LevelFilter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LevelFilter) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Level) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>`

- `fn lt(self: &Self, other: &Level) -> bool`

- `fn le(self: &Self, other: &Level) -> bool`

- `fn gt(self: &Self, other: &Level) -> bool`

- `fn ge(self: &Self, other: &Level) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>`

- `fn lt(self: &Self, other: &LevelFilter) -> bool`

- `fn le(self: &Self, other: &LevelFilter) -> bool`

- `fn gt(self: &Self, other: &LevelFilter) -> bool`

- `fn ge(self: &Self, other: &LevelFilter) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Metadata<'a>`

```rust
struct Metadata<'a> {
}
```

Metadata describing a [span](#span) or [event](#event).

All spans and events have the following metadata:
- A [name](#name), represented as a static string.
- A [target](#target), a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`](tracing_core/metadata/index.md) type for details.
- The names of the [fields](#fields) defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`](tracing_core/subscriber/index.md)s when filtering spans and events, and it
may also be used as part of their data payload.

When created by the `event!` or `span!` macro, the metadata describing a
particular event or span is constructed statically and exists as a single
static instance. Thus, the overhead of creating the metadata is
_significantly_ lower than that of creating the actual span. Therefore,
filtering is based on metadata, rather than on the constructed span.

## Equality

In well-behaved applications, two `Metadata` with equal
[callsite identifiers] will be equal in all other ways (i.e., have the same
`name`, `target`, etc.). Consequently, in release builds, [`Metadata::eq`](#eq)
*only* checks that its arguments have equal callsites. However, the equality
of `Metadata`'s other fields is checked in debug builds.

[span](#span): super::span
[event](#event): super::event
[name](#name): Self::name
[target](#target): Self::target
[fields](#fields): Self::fields
[verbosity level]: Self::level
[file name]: Self::file
[line number]: Self::line
[module path]: Self::module_path

[callsite identifiers]: Self::callsite

#### Implementations

- `const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self`
  Construct new metadata for a span or event, with a name, target, level, field

- `fn fields(self: &Self) -> &field::FieldSet`
  Returns the names of the fields on the described span or event.

- `fn level(self: &Self) -> &Level`
  Returns the level of verbosity of the described span or event.

- `fn name(self: &Self) -> &'static str`
  Returns the name of the span.

- `fn target(self: &Self) -> &'a str`
  Returns a string describing the part of the system where the span or

- `fn module_path(self: &Self) -> Option<&'a str>`
  Returns the path to the Rust module where the span occurred, or

- `fn file(self: &Self) -> Option<&'a str>`
  Returns the name of the source code file where the span

- `fn line(self: &Self) -> Option<u32>`
  Returns the line number in the source code file where the span

- `fn callsite(self: &Self) -> callsite::Identifier`
  Returns an opaque `Identifier` that uniquely identifies the callsite

- `fn is_event(self: &Self) -> bool`
  Returns true if the callsite kind is `Event`.

- `fn is_span(self: &Self) -> bool`
  Return true if the callsite kind is `Span`.

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Kind`

```rust
struct Kind();
```

Indicates whether the callsite is a span or event.

#### Implementations

- `const EVENT: Kind`

- `const SPAN: Kind`

- `const HINT: Kind`

- `fn is_span(self: &Self) -> bool`
  Return true if the callsite kind is `Span`

- `fn is_event(self: &Self) -> bool`
  Return true if the callsite kind is `Event`

- `fn is_hint(self: &Self) -> bool`
  Return true if the callsite kind is `Hint`

- `const fn hint(self: Self) -> Self`
  Sets that this `Kind` is a [hint](Self::HINT).

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

- `fn clone(self: &Self) -> Kind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Kind) -> bool`

##### `impl StructuralPartialEq`

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

### `Interest`

```rust
struct Interest();
```

Indicates a [`Subscriber`](tracing_core/subscriber/index.md)'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their [`register_callsite`](#register-callsite) methods
in order to determine whether that span should be enabled or disabled.



#### Implementations

- `fn never() -> Self`
  Returns an `Interest` indicating that the subscriber is never interested

- `fn sometimes() -> Self`
  Returns an `Interest` indicating the subscriber is sometimes interested

- `fn always() -> Self`
  Returns an `Interest` indicating the subscriber is always interested in

- `fn is_never(self: &Self) -> bool`
  Returns `true` if the subscriber is never interested in being notified

- `fn is_sometimes(self: &Self) -> bool`
  Returns `true` if the subscriber is sometimes interested in being notified

- `fn is_always(self: &Self) -> bool`
  Returns `true` if the subscriber is always interested in being notified

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

- `fn clone(self: &Self) -> Interest`

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

## Traits

## Macros

### `identify_callsite!`

Statically constructs an [`Identifier`](tracing_core/callsite/index.md) for the provided [`Callsite`](tracing_core/callsite/index.md).

This may be used in contexts such as static initializers.

For example:
```rust
use tracing_core::{callsite, identify_callsite};
# use tracing_core::{Metadata, subscriber::Interest};
# fn main() {
pub struct MyCallsite {
   // ...
}
impl callsite::Callsite for MyCallsite {
# fn set_interest(&self, _: Interest) { unimplemented!() }
# fn metadata(&self) -> &Metadata { unimplemented!() }
    // ...
}

static CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static CALLSITE_ID: callsite::Identifier = identify_callsite!(&CALLSITE);
# }
```



### `metadata!`

Statically constructs new span [metadata](#metadata).

/// For example:
```rust
# use tracing_core::{callsite::Callsite, subscriber::Interest};
use tracing_core::metadata;
use tracing_core::metadata::{Kind, Level, Metadata};
# fn main() {
# pub struct MyCallsite { }
# impl Callsite for MyCallsite {
# fn set_interest(&self, _: Interest) { unimplemented!() }
# fn metadata(&self) -> &Metadata { unimplemented!() }
# }
#
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
# }
```

[metadata](#metadata): metadata::Metadata


