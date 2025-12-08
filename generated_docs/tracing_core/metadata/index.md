*[tracing_core](../index.md) / [metadata](index.md)*

---

# Module `metadata`

Metadata describing trace data.

## Structs

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

Metadata describing a [`span`](../span/index.md) or [`event`](../event/index.md).

All spans and events have the following metadata:
- A [`name`](../../serde_derive/internals/name/index.md), represented as a static string.
- A [`target`](../../tracing_attributes/attr/kw/index.md), a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`](../index.md) type for details.
- The names of the [`fields`](../../tracing_attributes/attr/kw/index.md) defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`](../index.md)s when filtering spans and events, and it
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

- `const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self` — [`Level`](../index.md), [`FieldSet`](../field/index.md), [`Kind`](../index.md)

- `fn fields(self: &Self) -> &field::FieldSet` — [`FieldSet`](../field/index.md)

- `fn level(self: &Self) -> &Level` — [`Level`](../index.md)

- `fn name(self: &Self) -> &'static str`

- `fn target(self: &Self) -> &'a str`

- `fn module_path(self: &Self) -> Option<&'a str>`

- `fn file(self: &Self) -> Option<&'a str>`

- `fn line(self: &Self) -> Option<u32>`

- `fn callsite(self: &Self) -> callsite::Identifier` — [`Identifier`](../callsite/index.md)

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

- `fn clone(self: &Self) -> Kind` — [`Kind`](../index.md)

##### `impl Debug for Kind`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Kind`

##### `impl PartialEq for Kind`

- `fn eq(self: &Self, other: &Kind) -> bool` — [`Kind`](../index.md)

##### `impl StructuralPartialEq for Kind`

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

The [`LevelFilter`](../index.md) type is provided to enable filtering traces by
verbosity. `Level`s can be compared against [`LevelFilter`](../index.md)s, and
[`LevelFilter`](../index.md) has a variant for each `Level`, which compares analogously
to that level. In addition, [`LevelFilter`](../index.md) adds a `LevelFilter::OFF`
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

Below is a simple example of how a [`Subscriber`](../index.md) could implement filtering through
a [`LevelFilter`](../index.md). When a span or event is recorded, the `Subscriber::enabled` method
compares the span or event's `Level` against the configured [`LevelFilter`](../index.md).
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

- `fn clone(self: &Self) -> Level` — [`Level`](../index.md)

##### `impl Copy for Level`

##### `impl Debug for Level`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Level`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl FromStr for Level`

- `type Err = ParseLevelError`

- `fn from_str(s: &str) -> Result<Self, ParseLevelError>` — [`ParseLevelError`](#parselevelerror)

##### `impl Hash for Level`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Level`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for Level`

- `fn eq(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](../index.md)

##### `impl PartialOrd for Level`

- `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](../index.md)

- `fn lt(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](../index.md)

- `fn le(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](../index.md)

- `fn gt(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](../index.md)

- `fn ge(self: &Self, other: &LevelFilter) -> bool` — [`LevelFilter`](../index.md)

##### `impl StructuralPartialEq for Level`

##### `impl<T> ToString for Level`

- `fn to_string(self: &Self) -> String`

### `LevelFilter`

```rust
struct LevelFilter(Option<Level>);
```

A filter comparable to a verbosity [`Level`](../index.md).

If a [`Level`](../index.md) is considered less than or equal to a `LevelFilter`, it
should be considered enabled; if greater than the `LevelFilter`, that level
is disabled. See `LevelFilter::current` for more details.

Note that this is essentially identical to the `Level` type, but with the
addition of an `OFF` level that completely disables all trace
instrumentation.

See the documentation for the [`Level`](../index.md) type to see how `Level`s
and `LevelFilter`s interact.


#### Implementations

- `const OFF: LevelFilter`

- `const ERROR: LevelFilter`

- `const WARN: LevelFilter`

- `const INFO: LevelFilter`

- `const DEBUG: LevelFilter`

- `const TRACE: LevelFilter`

- `const fn from_level(level: Level) -> Self` — [`Level`](../index.md)

- `const fn into_level(self: Self) -> Option<Level>` — [`Level`](../index.md)

- `const ERROR_USIZE: usize`

- `const WARN_USIZE: usize`

- `const INFO_USIZE: usize`

- `const DEBUG_USIZE: usize`

- `const TRACE_USIZE: usize`

- `const OFF_USIZE: usize`

- `fn current() -> Self`

- `fn set_max(LevelFilter: LevelFilter)` — [`LevelFilter`](../index.md)

#### Trait Implementations

##### `impl Clone for LevelFilter`

- `fn clone(self: &Self) -> LevelFilter` — [`LevelFilter`](../index.md)

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

- `fn eq(self: &Self, other: &Level) -> bool` — [`Level`](../index.md)

##### `impl PartialOrd for LevelFilter`

- `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>` — [`Level`](../index.md)

- `fn lt(self: &Self, other: &Level) -> bool` — [`Level`](../index.md)

- `fn le(self: &Self, other: &Level) -> bool` — [`Level`](../index.md)

- `fn gt(self: &Self, other: &Level) -> bool` — [`Level`](../index.md)

- `fn ge(self: &Self, other: &Level) -> bool` — [`Level`](../index.md)

##### `impl StructuralPartialEq for LevelFilter`

##### `impl<T> ToString for LevelFilter`

- `fn to_string(self: &Self) -> String`

### `ParseLevelFilterError`

```rust
struct ParseLevelFilterError(());
```

Indicates that a string could not be parsed to a valid level.

#### Trait Implementations

##### `impl Clone for ParseLevelFilterError`

- `fn clone(self: &Self) -> ParseLevelFilterError` — [`ParseLevelFilterError`](#parselevelfiltererror)

##### `impl Debug for ParseLevelFilterError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ParseLevelFilterError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseLevelFilterError`

##### `impl<T> ToString for ParseLevelFilterError`

- `fn to_string(self: &Self) -> String`

### `ParseLevelError`

```rust
struct ParseLevelError {
    _p: (),
}
```

Returned if parsing a `Level` fails.

#### Trait Implementations

##### `impl Debug for ParseLevelError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ParseLevelError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseLevelError`

##### `impl<T> ToString for ParseLevelError`

- `fn to_string(self: &Self) -> String`

## Enums

### `LevelInner`

```rust
enum LevelInner {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
```

#### Variants

- **`Trace`**

  The "trace" level.
  
  Designates very low priority, often extremely verbose, information.

- **`Debug`**

  The "debug" level.
  
  Designates lower priority information.

- **`Info`**

  The "info" level.
  
  Designates useful information.

- **`Warn`**

  The "warn" level.
  
  Designates hazardous situations.

- **`Error`**

  The "error" level.
  
  Designates very serious errors.

#### Trait Implementations

##### `impl Clone for LevelInner`

- `fn clone(self: &Self) -> LevelInner` — [`LevelInner`](#levelinner)

##### `impl Copy for LevelInner`

##### `impl Debug for LevelInner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LevelInner`

##### `impl Hash for LevelInner`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for LevelInner`

- `fn eq(self: &Self, other: &LevelInner) -> bool` — [`LevelInner`](#levelinner)

##### `impl StructuralPartialEq for LevelInner`

## Functions

### `filter_as_usize`

```rust
fn filter_as_usize(x: &Option<Level>) -> usize
```

