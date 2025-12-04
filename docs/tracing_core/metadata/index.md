*[tracing_core](../index.md) / [metadata](index.md)*

---

# Module `metadata`

Metadata describing trace data.

## Structs

### `Metadata<'a>`

```rust
struct Metadata<'a> {
    // [REDACTED: Private Fields]
}
```

Metadata describing a [span](#span)
 or [event](#event)
.

All spans and events have the following metadata:
- A [name](#name)
, represented as a static string.
- A [target](#target)
, a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`](tracing_core/metadata/index.md) type for details.
- The names of the [fields](#fields)
 defined by the span or event.
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

[span](#span)
: super::span
[event](#event)
: super::event
[name](#name)
: Self::name
[target](#target)
: Self::target
[fields](#fields)
: Self::fields
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
APIs][envfilter](#envfilter)
 for performing more sophisticated filtering, such as
enabling different levels based on which module or crate a span or event is
recorded in.






[envfilter](#envfilter)
: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ParseLevelFilterError`

```rust
struct ParseLevelFilterError();
```

Indicates that a string could not be parsed to a valid level.

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

- `fn clone(self: &Self) -> ParseLevelFilterError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

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

### `ParseLevelError`

```rust
struct ParseLevelError {
    // [REDACTED: Private Fields]
}
```

Returned if parsing a `Level` fails.

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

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

