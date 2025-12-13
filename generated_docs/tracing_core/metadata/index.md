*[tracing_core](../index.md) / [metadata](index.md)*

---

# Module `metadata`

Metadata describing trace data.

## Contents

- [Structs](#structs)
  - [`Metadata`](#metadata)
  - [`Kind`](#kind)
  - [`Level`](#level)
  - [`LevelFilter`](#levelfilter)
  - [`ParseLevelFilterError`](#parselevelfiltererror)
  - [`ParseLevelError`](#parselevelerror)
- [Enums](#enums)
  - [`LevelInner`](#levelinner)
- [Functions](#functions)
  - [`filter_as_usize`](#filter-as-usize)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Metadata`](#metadata) | struct | Metadata describing a [span] or [event]. |
| [`Kind`](#kind) | struct | Indicates whether the callsite is a span or event. |
| [`Level`](#level) | struct | Describes the level of verbosity of a span or event. |
| [`LevelFilter`](#levelfilter) | struct | A filter comparable to a verbosity [`Level`]. |
| [`ParseLevelFilterError`](#parselevelfiltererror) | struct | Indicates that a string could not be parsed to a valid level. |
| [`ParseLevelError`](#parselevelerror) | struct | Returned if parsing a `Level` fails. |
| [`LevelInner`](#levelinner) | enum |  |
| [`filter_as_usize`](#filter-as-usize) | fn |  |

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

*Defined in [`tracing-core-0.1.35/src/metadata.rs:57-86`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L57-L86)*

Metadata describing a [`span`](../span/index.md) or [`event`](../event/index.md).

All spans and events have the following metadata:
- A [`name`](../../serde_derive/internals/name/index.md), represented as a static string.
- A [`target`](../../tracing_attributes/attr/kw/index.md), a string that categorizes part of the system where the span
  or event occurred. The `tracing` macros default to using the module
  path where the span or event originated as the target, but it may be
  overridden.
- A [verbosity level]. This determines how verbose a given span or event
  is, and allows enabling or disabling more verbose diagnostics
  situationally. See the documentation for the [`Level`](#level) type for details.
- The names of the [`fields`](../../tracing_attributes/attr/kw/index.md) defined by the span or event.
- Whether the metadata corresponds to a span or event.

In addition, the following optional metadata describing the source code
location where the span or event originated _may_ be provided:
- The [file name]
- The [line number]
- The [module path]

Metadata is used by [`Subscriber`](../subscriber/index.md)s when filtering spans and events, and it
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

- <span id="metadata-new"></span>`const fn new(name: &'static str, target: &'a str, level: Level, file: Option<&'a str>, line: Option<u32>, module_path: Option<&'a str>, fields: field::FieldSet, kind: Kind) -> Self` — [`Level`](#level), [`FieldSet`](../field/index.md#fieldset), [`Kind`](#kind)

  Construct new metadata for a span or event, with a name, target, level, field

  names, and optional source code location.

- <span id="metadata-fields"></span>`fn fields(&self) -> &field::FieldSet` — [`FieldSet`](../field/index.md#fieldset)

  Returns the names of the fields on the described span or event.

- <span id="metadata-level"></span>`fn level(&self) -> &Level` — [`Level`](#level)

  Returns the level of verbosity of the described span or event.

- <span id="metadata-name"></span>`fn name(&self) -> &'static str`

  Returns the name of the span.

- <span id="metadata-target"></span>`fn target(&self) -> &'a str`

  Returns a string describing the part of the system where the span or

  event that this metadata describes occurred.

  

  Typically, this is the module path, but alternate targets may be set

  when spans or events are constructed.

- <span id="metadata-module-path"></span>`fn module_path(&self) -> Option<&'a str>`

  Returns the path to the Rust module where the span occurred, or

  `None` if the module path is unknown.

- <span id="metadata-file"></span>`fn file(&self) -> Option<&'a str>`

  Returns the name of the source code file where the span

  occurred, or `None` if the file is unknown

- <span id="metadata-line"></span>`fn line(&self) -> Option<u32>`

  Returns the line number in the source code file where the span

  occurred, or `None` if the line number is unknown.

- <span id="metadata-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](../callsite/index.md#identifier)

  Returns an opaque `Identifier` that uniquely identifies the callsite

  this `Metadata` originated from.

- <span id="metadata-is-event"></span>`fn is_event(&self) -> bool`

  Returns true if the callsite kind is `Event`.

- <span id="metadata-is-span"></span>`fn is_span(&self) -> bool`

  Return true if the callsite kind is `Span`.

#### Trait Implementations

##### `impl Any for Metadata<'a>`

- <span id="metadata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Metadata<'a>`

- <span id="metadata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Metadata<'a>`

- <span id="metadata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Metadata<'_>`

- <span id="metadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Metadata<'_>`

##### `impl<T> From for Metadata<'a>`

- <span id="metadata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Metadata<'a>`

- <span id="metadata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Metadata<'_>`

- <span id="metadata-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<U> TryFrom for Metadata<'a>`

- <span id="metadata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="metadata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Metadata<'a>`

- <span id="metadata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="metadata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Kind`

```rust
struct Kind(u8);
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:90`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L90)*

Indicates whether the callsite is a span or event.

#### Implementations

- <span id="kind-const-event-bit"></span>`const EVENT_BIT: u8`

- <span id="kind-const-span-bit"></span>`const SPAN_BIT: u8`

- <span id="kind-const-hint-bit"></span>`const HINT_BIT: u8`

- <span id="kind-const-event"></span>`const EVENT: Kind`

- <span id="kind-const-span"></span>`const SPAN: Kind`

- <span id="kind-const-hint"></span>`const HINT: Kind`

- <span id="kind-is-span"></span>`fn is_span(&self) -> bool`

  Return true if the callsite kind is `Span`

- <span id="kind-is-event"></span>`fn is_event(&self) -> bool`

  Return true if the callsite kind is `Event`

- <span id="kind-is-hint"></span>`fn is_hint(&self) -> bool`

  Return true if the callsite kind is `Hint`

- <span id="kind-hint"></span>`const fn hint(self) -> Self`

  Sets that this `Kind` is a [hint](Self::HINT).

  

  This can be called on [`SPAN`](Self::SPAN) and [`EVENT`](Self::EVENT)

  kinds to construct a hint callsite that also counts as a span or event.

#### Trait Implementations

##### `impl Any for Kind`

- <span id="kind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Kind`

- <span id="kind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Kind`

- <span id="kind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Kind`

- <span id="kind-clone"></span>`fn clone(&self) -> Kind` — [`Kind`](#kind)

##### `impl CloneToUninit for Kind`

- <span id="kind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Kind`

- <span id="kind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Kind`

##### `impl<T> From for Kind`

- <span id="kind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Kind`

- <span id="kind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Kind`

- <span id="kind-partialeq-eq"></span>`fn eq(&self, other: &Kind) -> bool` — [`Kind`](#kind)

##### `impl StructuralPartialEq for Kind`

##### `impl ToOwned for Kind`

- <span id="kind-toowned-type-owned"></span>`type Owned = T`

- <span id="kind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="kind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Kind`

- <span id="kind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Kind`

- <span id="kind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Level`

```rust
struct Level(LevelInner);
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:221`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L221)*

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

Below is a simple example of how a [`Subscriber`](../subscriber/index.md) could implement filtering through
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

- <span id="level-const-error"></span>`const ERROR: Level`

- <span id="level-const-warn"></span>`const WARN: Level`

- <span id="level-const-info"></span>`const INFO: Level`

- <span id="level-const-debug"></span>`const DEBUG: Level`

- <span id="level-const-trace"></span>`const TRACE: Level`

- <span id="level-as-str"></span>`fn as_str(&self) -> &'static str`

  Returns the string representation of the `Level`.

  

  This returns the same string as the `fmt::Display` implementation.

#### Trait Implementations

##### `impl Any for Level`

- <span id="level-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Level`

- <span id="level-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Level`

- <span id="level-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Level`

- <span id="level-clone"></span>`fn clone(&self) -> Level` — [`Level`](#level)

##### `impl CloneToUninit for Level`

- <span id="level-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Level`

##### `impl Debug for Level`

- <span id="level-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Level`

- <span id="level-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl<T> From for Level`

- <span id="level-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for Level`

- <span id="level-fromstr-type-err"></span>`type Err = ParseLevelError`

- <span id="level-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, ParseLevelError>` — [`ParseLevelError`](#parselevelerror)

##### `impl Hash for Level`

- <span id="level-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Level`

- <span id="level-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Level`

- <span id="level-ord-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for Level`

- <span id="level-partialeq-eq"></span>`fn eq(&self, other: &Level) -> bool` — [`Level`](#level)

##### `impl PartialOrd for Level`

- <span id="level-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering>` — [`Level`](#level)

- <span id="level-partialord-lt"></span>`fn lt(&self, other: &Level) -> bool` — [`Level`](#level)

- <span id="level-partialord-le"></span>`fn le(&self, other: &Level) -> bool` — [`Level`](#level)

- <span id="level-partialord-gt"></span>`fn gt(&self, other: &Level) -> bool` — [`Level`](#level)

- <span id="level-partialord-ge"></span>`fn ge(&self, other: &Level) -> bool` — [`Level`](#level)

##### `impl StructuralPartialEq for Level`

##### `impl ToOwned for Level`

- <span id="level-toowned-type-owned"></span>`type Owned = T`

- <span id="level-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="level-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Level`

- <span id="level-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Level`

- <span id="level-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="level-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Level`

- <span id="level-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="level-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LevelFilter`

```rust
struct LevelFilter(Option<Level>);
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:239`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L239)*

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

- <span id="levelfilter-const-off"></span>`const OFF: LevelFilter`

- <span id="levelfilter-const-error"></span>`const ERROR: LevelFilter`

- <span id="levelfilter-const-warn"></span>`const WARN: LevelFilter`

- <span id="levelfilter-const-info"></span>`const INFO: LevelFilter`

- <span id="levelfilter-const-debug"></span>`const DEBUG: LevelFilter`

- <span id="levelfilter-const-trace"></span>`const TRACE: LevelFilter`

- <span id="levelfilter-from-level"></span>`const fn from_level(level: Level) -> Self` — [`Level`](#level)

  Returns a `LevelFilter` that enables spans and events with verbosity up

  to and including `level`.

- <span id="levelfilter-into-level"></span>`const fn into_level(self) -> Option<Level>` — [`Level`](#level)

  Returns the most verbose [`Level`](#level) that this filter accepts, or `None`

  if it is `OFF`.

- <span id="levelfilter-const-error-usize"></span>`const ERROR_USIZE: usize`

- <span id="levelfilter-const-warn-usize"></span>`const WARN_USIZE: usize`

- <span id="levelfilter-const-info-usize"></span>`const INFO_USIZE: usize`

- <span id="levelfilter-const-debug-usize"></span>`const DEBUG_USIZE: usize`

- <span id="levelfilter-const-trace-usize"></span>`const TRACE_USIZE: usize`

- <span id="levelfilter-const-off-usize"></span>`const OFF_USIZE: usize`

- <span id="levelfilter-current"></span>`fn current() -> Self`

  Returns a `LevelFilter` that matches the most verbose [`Level`](#level) that any

  currently active [`Subscriber`](../subscriber/index.md) will enable.

  

  User code should treat this as a *hint*. If a given span or event has a

  level *higher* than the returned `LevelFilter`, it will not be enabled.

  However, if the level is less than or equal to this value, the span or

  event is *not* guaranteed to be enabled; the subscriber will still

  filter each callsite individually.

  

  Therefore, comparing a given span or event's level to the returned

  `LevelFilter` **can** be used for determining if something is

  *disabled*, but **should not** be used for determining if something is

  *enabled*.

  

- <span id="levelfilter-set-max"></span>`fn set_max(LevelFilter: LevelFilter)` — [`LevelFilter`](#levelfilter)

#### Trait Implementations

##### `impl Any for LevelFilter`

- <span id="levelfilter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LevelFilter`

- <span id="levelfilter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LevelFilter`

- <span id="levelfilter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LevelFilter`

- <span id="levelfilter-clone"></span>`fn clone(&self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

##### `impl CloneToUninit for LevelFilter`

- <span id="levelfilter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LevelFilter`

##### `impl Debug for LevelFilter`

- <span id="levelfilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LevelFilter`

- <span id="levelfilter-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelFilter`

##### `impl<T> From for LevelFilter`

- <span id="levelfilter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for LevelFilter`

- <span id="levelfilter-fromstr-type-err"></span>`type Err = ParseLevelFilterError`

- <span id="levelfilter-fromstr-from-str"></span>`fn from_str(from: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for LevelFilter`

- <span id="levelfilter-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LevelFilter`

- <span id="levelfilter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for LevelFilter`

- <span id="levelfilter-ord-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq for LevelFilter`

- <span id="levelfilter-partialeq-eq"></span>`fn eq(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl PartialOrd for Level`

- <span id="level-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](#levelfilter)

- <span id="level-partialord-lt"></span>`fn lt(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- <span id="level-partialord-le"></span>`fn le(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- <span id="level-partialord-gt"></span>`fn gt(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

- <span id="level-partialord-ge"></span>`fn ge(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl StructuralPartialEq for LevelFilter`

##### `impl ToOwned for LevelFilter`

- <span id="levelfilter-toowned-type-owned"></span>`type Owned = T`

- <span id="levelfilter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="levelfilter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for LevelFilter`

- <span id="levelfilter-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for LevelFilter`

- <span id="levelfilter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="levelfilter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LevelFilter`

- <span id="levelfilter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="levelfilter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParseLevelFilterError`

```rust
struct ParseLevelFilterError(());
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:243`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L243)*

Indicates that a string could not be parsed to a valid level.

#### Trait Implementations

##### `impl Any for ParseLevelFilterError`

- <span id="parselevelfiltererror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseLevelFilterError`

- <span id="parselevelfiltererror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseLevelFilterError`

- <span id="parselevelfiltererror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ParseLevelFilterError`

- <span id="parselevelfiltererror-clone"></span>`fn clone(&self) -> ParseLevelFilterError` — [`ParseLevelFilterError`](#parselevelfiltererror)

##### `impl CloneToUninit for ParseLevelFilterError`

- <span id="parselevelfiltererror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ParseLevelFilterError`

- <span id="parselevelfiltererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseLevelFilterError`

- <span id="parselevelfiltererror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseLevelFilterError`

##### `impl<T> From for ParseLevelFilterError`

- <span id="parselevelfiltererror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseLevelFilterError`

- <span id="parselevelfiltererror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ParseLevelFilterError`

- <span id="parselevelfiltererror-toowned-type-owned"></span>`type Owned = T`

- <span id="parselevelfiltererror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parselevelfiltererror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ParseLevelFilterError`

- <span id="parselevelfiltererror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseLevelFilterError`

- <span id="parselevelfiltererror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parselevelfiltererror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseLevelFilterError`

- <span id="parselevelfiltererror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parselevelfiltererror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParseLevelError`

```rust
struct ParseLevelError {
    _p: (),
}
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:805-807`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L805-L807)*

Returned if parsing a `Level` fails.

#### Trait Implementations

##### `impl Any for ParseLevelError`

- <span id="parselevelerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseLevelError`

- <span id="parselevelerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseLevelError`

- <span id="parselevelerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseLevelError`

- <span id="parselevelerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseLevelError`

- <span id="parselevelerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseLevelError`

##### `impl<T> From for ParseLevelError`

- <span id="parselevelerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseLevelError`

- <span id="parselevelerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ParseLevelError`

- <span id="parselevelerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseLevelError`

- <span id="parselevelerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parselevelerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseLevelError`

- <span id="parselevelerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parselevelerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`tracing-core-0.1.35/src/metadata.rs:579-600`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L579-L600)*

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

##### `impl Any for LevelInner`

- <span id="levelinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LevelInner`

- <span id="levelinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LevelInner`

- <span id="levelinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LevelInner`

- <span id="levelinner-clone"></span>`fn clone(&self) -> LevelInner` — [`LevelInner`](#levelinner)

##### `impl CloneToUninit for LevelInner`

- <span id="levelinner-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LevelInner`

##### `impl Debug for LevelInner`

- <span id="levelinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelInner`

##### `impl<T> From for LevelInner`

- <span id="levelinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LevelInner`

- <span id="levelinner-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LevelInner`

- <span id="levelinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LevelInner`

- <span id="levelinner-partialeq-eq"></span>`fn eq(&self, other: &LevelInner) -> bool` — [`LevelInner`](#levelinner)

##### `impl StructuralPartialEq for LevelInner`

##### `impl ToOwned for LevelInner`

- <span id="levelinner-toowned-type-owned"></span>`type Owned = T`

- <span id="levelinner-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="levelinner-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LevelInner`

- <span id="levelinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="levelinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LevelInner`

- <span id="levelinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="levelinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `filter_as_usize`

```rust
fn filter_as_usize(x: &Option<Level>) -> usize
```

*Defined in [`tracing-core-0.1.35/src/metadata.rs:972-977`](../../../.source_1765633015/tracing-core-0.1.35/src/metadata.rs#L972-L977)*

