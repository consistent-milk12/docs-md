*[tracing_core](../index.md) / [subscriber](index.md)*

---

# Module `subscriber`

Collectors collect and record trace data.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Interest`](#interest) | struct | Indicates a [`Subscriber`]'s interest in a particular callsite. |
| [`NoSubscriber`](#nosubscriber) | struct | A no-op [`Subscriber`]. |
| [`InterestKind`](#interestkind) | enum |  |
| [`Subscriber`](#subscriber) | trait | Trait representing the functions required to collect trace data. |

## Structs

### `Interest`

```rust
struct Interest(InterestKind);
```

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:589`](../../../.source_1765633015/tracing-core-0.1.35/src/subscriber.rs#L589)*

Indicates a [`Subscriber`](#subscriber)'s interest in a particular callsite.

`Subscriber`s return an `Interest` from their `register_callsite` methods
in order to determine whether that span should be enabled or disabled.



#### Implementations

- <span id="interest-never"></span>`fn never() -> Self`

  Returns an `Interest` indicating that the subscriber is never interested

  in being notified about a callsite.

  

  If all active subscribers are `never()` interested in a callsite, it will

  be completely disabled unless a new subscriber becomes active.

- <span id="interest-sometimes"></span>`fn sometimes() -> Self`

  Returns an `Interest` indicating the subscriber is sometimes interested

  in being notified about a callsite.

  

  If all active subscribers are `sometimes` or `never` interested in a

  callsite, the currently active subscriber will be asked to filter that

  callsite every time it creates a span. This will be the case until a new

  subscriber expresses that it is `always` interested in the callsite.

- <span id="interest-always"></span>`fn always() -> Self`

  Returns an `Interest` indicating the subscriber is always interested in

  being notified about a callsite.

  

  If any subscriber expresses that it is `always()` interested in a given

  callsite, then the callsite will always be enabled.

- <span id="interest-is-never"></span>`fn is_never(&self) -> bool`

  Returns `true` if the subscriber is never interested in being notified

  about this callsite.

- <span id="interest-is-sometimes"></span>`fn is_sometimes(&self) -> bool`

  Returns `true` if the subscriber is sometimes interested in being notified

  about this callsite.

- <span id="interest-is-always"></span>`fn is_always(&self) -> bool`

  Returns `true` if the subscriber is always interested in being notified

  about this callsite.

- <span id="interest-and"></span>`fn and(self, rhs: Interest) -> Self` — [`Interest`](#interest)

  Returns the common interest between these two Interests.

  

  If both interests are the same, this propagates that interest.

  Otherwise, if they differ, the result must always be

  `Interest::sometimes` --- if the two subscribers differ in opinion, we

  will have to ask the current subscriber what it thinks, no matter what.

#### Trait Implementations

##### `impl Any for Interest`

- <span id="interest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Interest`

- <span id="interest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Interest`

- <span id="interest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Interest`

- <span id="interest-clone"></span>`fn clone(&self) -> Interest` — [`Interest`](#interest)

##### `impl CloneToUninit for Interest`

- <span id="interest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Interest`

- <span id="interest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Interest`

- <span id="interest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Interest`

- <span id="interest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Interest`

- <span id="interest-toowned-type-owned"></span>`type Owned = T`

- <span id="interest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Interest`

- <span id="interest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Interest`

- <span id="interest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NoSubscriber`

```rust
struct NoSubscriber(());
```

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:672`](../../../.source_1765633015/tracing-core-0.1.35/src/subscriber.rs#L672)*

A no-op [`Subscriber`](#subscriber).

[`NoSubscriber`](#nosubscriber) implements the [`Subscriber`](#subscriber) trait by never being enabled,
never being interested in any callsite, and dropping all spans and events.

#### Implementations

- <span id="nosubscriber-new"></span>`const fn new() -> Self`

  Returns a new `NoSubscriber`.

#### Trait Implementations

##### `impl Any for NoSubscriber`

- <span id="nosubscriber-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoSubscriber`

- <span id="nosubscriber-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoSubscriber`

- <span id="nosubscriber-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NoSubscriber`

- <span id="nosubscriber-clone"></span>`fn clone(&self) -> NoSubscriber` — [`NoSubscriber`](#nosubscriber)

##### `impl CloneToUninit for NoSubscriber`

- <span id="nosubscriber-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for NoSubscriber`

##### `impl Debug for NoSubscriber`

- <span id="nosubscriber-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NoSubscriber`

- <span id="nosubscriber-default"></span>`fn default() -> NoSubscriber` — [`NoSubscriber`](#nosubscriber)

##### `impl<T> From for NoSubscriber`

- <span id="nosubscriber-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoSubscriber`

- <span id="nosubscriber-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Subscriber for NoSubscriber`

- <span id="nosubscriber-subscriber-register-callsite"></span>`fn register_callsite(&self, _: &'static Metadata<'static>) -> Interest` — [`Metadata`](../metadata/index.md#metadata), [`Interest`](#interest)

- <span id="nosubscriber-subscriber-new-span"></span>`fn new_span(&self, _: &span::Attributes<'_>) -> span::Id` — [`Attributes`](../span/index.md#attributes), [`Id`](../span/index.md#id)

- <span id="nosubscriber-subscriber-event"></span>`fn event(&self, _event: &Event<'_>)` — [`Event`](../event/index.md#event)

- <span id="nosubscriber-subscriber-record"></span>`fn record(&self, _span: &span::Id, _values: &span::Record<'_>)` — [`Id`](../span/index.md#id), [`Record`](../span/index.md#record)

- <span id="nosubscriber-subscriber-record-follows-from"></span>`fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id)` — [`Id`](../span/index.md#id)

- <span id="nosubscriber-subscriber-enabled"></span>`fn enabled(&self, _metadata: &Metadata<'_>) -> bool` — [`Metadata`](../metadata/index.md#metadata)

- <span id="nosubscriber-subscriber-enter"></span>`fn enter(&self, _span: &span::Id)` — [`Id`](../span/index.md#id)

- <span id="nosubscriber-subscriber-exit"></span>`fn exit(&self, _span: &span::Id)` — [`Id`](../span/index.md#id)

##### `impl ToOwned for NoSubscriber`

- <span id="nosubscriber-toowned-type-owned"></span>`type Owned = T`

- <span id="nosubscriber-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nosubscriber-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NoSubscriber`

- <span id="nosubscriber-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nosubscriber-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoSubscriber`

- <span id="nosubscriber-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nosubscriber-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `InterestKind`

```rust
enum InterestKind {
    Never,
    Sometimes,
    Always,
}
```

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:592-596`](../../../.source_1765633015/tracing-core-0.1.35/src/subscriber.rs#L592-L596)*

#### Trait Implementations

##### `impl Any for InterestKind`

- <span id="interestkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InterestKind`

- <span id="interestkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InterestKind`

- <span id="interestkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for InterestKind`

- <span id="interestkind-clone"></span>`fn clone(&self) -> InterestKind` — [`InterestKind`](#interestkind)

##### `impl CloneToUninit for InterestKind`

- <span id="interestkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for InterestKind`

##### `impl Debug for InterestKind`

- <span id="interestkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InterestKind`

##### `impl<T> From for InterestKind`

- <span id="interestkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InterestKind`

- <span id="interestkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for InterestKind`

- <span id="interestkind-ord-cmp"></span>`fn cmp(&self, other: &InterestKind) -> cmp::Ordering` — [`InterestKind`](#interestkind)

##### `impl PartialEq for InterestKind`

- <span id="interestkind-partialeq-eq"></span>`fn eq(&self, other: &InterestKind) -> bool` — [`InterestKind`](#interestkind)

##### `impl PartialOrd for InterestKind`

- <span id="interestkind-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &InterestKind) -> option::Option<cmp::Ordering>` — [`InterestKind`](#interestkind)

##### `impl StructuralPartialEq for InterestKind`

##### `impl ToOwned for InterestKind`

- <span id="interestkind-toowned-type-owned"></span>`type Owned = T`

- <span id="interestkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interestkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for InterestKind`

- <span id="interestkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interestkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InterestKind`

- <span id="interestkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interestkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Subscriber`

```rust
trait Subscriber: 'static { ... }
```

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:80-499`](../../../.source_1765633015/tracing-core-0.1.35/src/subscriber.rs#L80-L499)*

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

- `fn enabled(&self, metadata: &Metadata<'_>) -> bool`

  Returns true if a span or event with the specified [`metadata`](../metadata/index.md) would be

- `fn new_span(&self, span: &span::Attributes<'_>) -> span::Id`

  Visit the construction of a new span, returning a new [span ID] for the

- `fn record(&self, span: &span::Id, values: &span::Record<'_>)`

  Record a set of values on a span.

- `fn record_follows_from(&self, span: &span::Id, follows: &span::Id)`

  Adds an indication that `span` follows from the span with the id

- `fn event(&self, event: &Event<'_>)`

  Records that an [`Event`](../event/index.md) has occurred.

- `fn enter(&self, span: &span::Id)`

  Records that a span has been entered.

- `fn exit(&self, span: &span::Id)`

  Records that a span has been exited.

#### Provided Methods

- `fn on_register_dispatch(&self, subscriber: &Dispatch)`

  Invoked when this subscriber becomes a [`Dispatch`](../dispatcher/index.md).

- `fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest`

  Registers a new [`callsite`](../callsite/index.md) with this subscriber, returning whether or not

- `fn max_level_hint(&self) -> Option<LevelFilter>`

  Returns the highest [verbosity level][`level`](../../tracing_attributes/attr/kw/index.md) that this `Subscriber` will

- `fn event_enabled(&self, event: &Event<'_>) -> bool`

  Determine if an [`Event`](../event/index.md) should be recorded.

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

- [`NoSubscriber`](#nosubscriber)
- `alloc::boxed::Box<S>`
- `alloc::sync::Arc<S>`

