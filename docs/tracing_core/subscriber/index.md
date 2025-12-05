*[tracing_core](../index.md) / [subscriber](index.md)*

---

# Module `subscriber`

Collectors collect and record trace data.

## Structs

### `Interest`

```rust
struct Interest();
```

Indicates a [`Subscriber`](#subscriber)'s interest in a particular callsite.

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

### `NoSubscriber`

```rust
struct NoSubscriber();
```

A no-op [`Subscriber`](#subscriber).

[`NoSubscriber`](#nosubscriber) implements the [`Subscriber`](#subscriber) trait by never being enabled,
never being interested in any callsite, and dropping all spans and events.

#### Implementations

- `const fn new() -> Self`
  Returns a new `NoSubscriber`.

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

- `fn clone(self: &Self) -> NoSubscriber`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Subscriber`

- `fn register_callsite(self: &Self, _: &'static Metadata<'static>) -> Interest`

- `fn new_span(self: &Self, _: &span::Attributes<'_>) -> span::Id`

- `fn event(self: &Self, _event: &Event<'_>)`

- `fn record(self: &Self, _span: &span::Id, _values: &span::Record<'_>)`

- `fn record_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id)`

- `fn enabled(self: &Self, _metadata: &Metadata<'_>) -> bool`

- `fn enter(self: &Self, _span: &span::Id)`

- `fn exit(self: &Self, _span: &span::Id)`

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

##### `impl Default`

- `fn default() -> NoSubscriber`

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
[`new_span`](#new-span) function is called when a new span is created, and at that
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

- [`register_callsite`](#register-callsite) is called once for each callsite from which a span
  event may originate, and returns an [`Interest`](#interest) value describing whether or
  not the subscriber wishes to see events or spans from that callsite. By
  default, it calls [`enabled`](#enabled), and returns `Interest::always()` if
  `enabled` returns true, or `Interest::never()` if enabled returns false.
  However, if the subscriber's interest can change dynamically at runtime,
  it may want to override this function to return `Interest::sometimes()`.
  Additionally, subscribers which wish to perform a behaviour once for each
  callsite, such as allocating storage for data related to that callsite,
  can perform it in `register_callsite`.

  See also the [documentation on the callsite registry][cs-reg] for details
  on [`register_callsite`](#register-callsite).

- [`event_enabled`](#event-enabled) is called once before every call to the [`event`](../event/index.md)
  method. This can be used to implement filtering on events once their field
  values are known, but before any processing is done in the `event` method.
- [`clone_span`](#clone-span) is called every time a span ID is cloned, and [`try_close`](#try-close)
  is called when a span ID is dropped. By default, these functions do
  nothing. However, they can be used to implement reference counting for
  spans, allowing subscribers to free storage for span data and to determine
  when a span has _closed_ permanently (rather than being exited).
  Subscribers which store per-span data or which need to track span closures
  should override these functions together.

[ID]: super::span::Id





[cs-reg]: crate::callsite#registering-callsites



#### Required Methods

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

  Invoked when this subscriber becomes a [`Dispatch`](../dispatcher/index.md).

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

  Registers a new [callsite](#callsite) with this subscriber, returning whether or not

- `fn enabled(self: &Self, metadata: &Metadata<'_>) -> bool`

  Returns true if a span or event with the specified [metadata](#metadata) would be

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

  Returns the highest [verbosity level][level](#level) that this `Subscriber` will

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

