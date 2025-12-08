*[tracing_core](../index.md) / [subscriber](index.md)*

---

# Module `subscriber`

Collectors collect and record trace data.

## Structs

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

### `NoSubscriber`

```rust
struct NoSubscriber(());
```

A no-op [`Subscriber`](#subscriber).

[`NoSubscriber`](#nosubscriber) implements the [`Subscriber`](#subscriber) trait by never being enabled,
never being interested in any callsite, and dropping all spans and events.

#### Implementations

- `const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NoSubscriber`

- `fn clone(self: &Self) -> NoSubscriber` — [`NoSubscriber`](#nosubscriber)

##### `impl Copy for NoSubscriber`

##### `impl Debug for NoSubscriber`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NoSubscriber`

- `fn default() -> NoSubscriber` — [`NoSubscriber`](#nosubscriber)

##### `impl Subscriber for NoSubscriber`

- `fn register_callsite(self: &Self, _: &'static Metadata<'static>) -> Interest` — [`Metadata`](../metadata/index.md), [`Interest`](#interest)

- `fn new_span(self: &Self, _: &span::Attributes<'_>) -> span::Id` — [`Attributes`](../span/index.md), [`Id`](../span/index.md)

- `fn event(self: &Self, _event: &Event<'_>)` — [`Event`](../event/index.md)

- `fn record(self: &Self, _span: &span::Id, _values: &span::Record<'_>)` — [`Id`](../span/index.md), [`Record`](../span/index.md)

- `fn record_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id)` — [`Id`](../span/index.md)

- `fn enabled(self: &Self, _metadata: &Metadata<'_>) -> bool` — [`Metadata`](../metadata/index.md)

- `fn enter(self: &Self, _span: &span::Id)` — [`Id`](../span/index.md)

- `fn exit(self: &Self, _span: &span::Id)` — [`Id`](../span/index.md)

## Enums

### `InterestKind`

```rust
enum InterestKind {
    Never,
    Sometimes,
    Always,
}
```

#### Trait Implementations

##### `impl Clone for InterestKind`

- `fn clone(self: &Self) -> InterestKind` — [`InterestKind`](#interestkind)

##### `impl Copy for InterestKind`

##### `impl Debug for InterestKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for InterestKind`

##### `impl Ord for InterestKind`

- `fn cmp(self: &Self, other: &InterestKind) -> $crate::cmp::Ordering` — [`InterestKind`](#interestkind)

##### `impl PartialEq for InterestKind`

- `fn eq(self: &Self, other: &InterestKind) -> bool` — [`InterestKind`](#interestkind)

##### `impl PartialOrd for InterestKind`

- `fn partial_cmp(self: &Self, other: &InterestKind) -> $crate::option::Option<$crate::cmp::Ordering>` — [`InterestKind`](#interestkind)

##### `impl StructuralPartialEq for InterestKind`

## Traits

### `Subscriber`

```rust
trait Subscriber: 'static { ... }
```

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
  event may originate, and returns an [`Interest`](#interest) value describing whether or
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

- `event_enabled` is called once before every call to the [`event`](../event/index.md)
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

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

  Invoked when this subscriber becomes a [`Dispatch`](../dispatcher/index.md).

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

  Registers a new [`callsite`](../callsite/index.md) with this subscriber, returning whether or not

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`

  Returns true if a span or event with the specified [`metadata`](../metadata/index.md) would be

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

  Returns the highest [verbosity level][`level`](../../tracing_attributes/attr/kw/index.md) that this `Subscriber` will

- `fn new_span(self: &Self, span: &span::Attributes<'_>) -> span::Id`

  Visit the construction of a new span, returning a new [span ID] for the

- `fn record(self: &Self, span: &span::Id, values: &span::Record<'_>)`

  Record a set of values on a span.

- `fn record_follows_from(self: &Self, span: &span::Id, follows: &span::Id)`

  Adds an indication that `span` follows from the span with the id

- `fn event_enabled(self: &Self, event: &Event<'_>) -> bool`

  Determine if an [`Event`](../event/index.md) should be recorded.

- `fn event(self: &Self, event: &Event<'_>)`

  Records that an [`Event`](../event/index.md) has occurred.

- `fn enter(self: &Self, span: &span::Id)`

  Records that a span has been entered.

- `fn exit(self: &Self, span: &span::Id)`

  Records that a span has been exited.

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id`

  Notifies the subscriber that a [span ID] has been cloned.

- `fn drop_span(self: &Self, _id: span::Id)`

  **This method is deprecated.**

- `fn try_close(self: &Self, id: span::Id) -> bool`

  Notifies the subscriber that a [span ID] has been dropped, and returns

- `fn current_span(self: &Self) -> span::Current`

  Returns a type representing this subscriber's view of the current span.

- `fn downcast_raw(self: &Self, id: TypeId) -> Option<*const ()>`

  If `self` is the same type as the provided `TypeId`, returns an untyped

