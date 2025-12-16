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

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:589`](../../../.source_1765900590/tracing-core-0.1.35/src/subscriber.rs#L589)*

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

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:672`](../../../.source_1765900590/tracing-core-0.1.35/src/subscriber.rs#L672)*

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

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:592-596`](../../../.source_1765900590/tracing-core-0.1.35/src/subscriber.rs#L592-L596)*

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

*Defined in [`tracing-core-0.1.35/src/subscriber.rs:80-499`](../../../.source_1765900590/tracing-core-0.1.35/src/subscriber.rs#L80-L499)*

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










<details>
<summary><strong>Methods (16)</strong> - click to expand</summary>

**Required:**
- [`Subscriber::enabled`](#fn-subscriberenabled)
- [`Subscriber::new_span`](#fn-subscribernew-span)
- [`Subscriber::record`](#fn-subscriberrecord)
- [`Subscriber::record_follows_from`](#fn-subscriberrecord-follows-from)
- [`Subscriber::event`](#fn-subscriberevent)
- [`Subscriber::enter`](#fn-subscriberenter)
- [`Subscriber::exit`](#fn-subscriberexit)

**Provided:**
- [`Subscriber::on_register_dispatch`](#fn-subscriberon-register-dispatch)
- [`Subscriber::register_callsite`](#fn-subscriberregister-callsite)
- [`Subscriber::max_level_hint`](#fn-subscribermax-level-hint)
- [`Subscriber::event_enabled`](#fn-subscriberevent-enabled)
- [`Subscriber::clone_span`](#fn-subscriberclone-span)
- [`Subscriber::drop_span`](#fn-subscriberdrop-span)
- [`Subscriber::try_close`](#fn-subscribertry-close)
- [`Subscriber::current_span`](#fn-subscribercurrent-span)
- [`Subscriber::downcast_raw`](#fn-subscriberdowncast-raw)

</details>

#### Required Methods

- `fn Subscriber::enabled(&self, metadata: &Metadata<'_>) -> bool`

  Returns true if a span or event with the specified [`metadata`](../metadata/index.md) would be
  recorded.
  
  By default, it is assumed that this filter needs only be evaluated once
  for each callsite, so it is called by `register_callsite` when each
  callsite is registered. The result is used to determine if the subscriber
  is always [interested] or never interested in that callsite. This is intended
  primarily as an optimization, so that expensive filters (such as those
  involving string search, et cetera) need not be re-evaluated.
  
  However, if the subscriber's interest in a particular span or event may
  change, or depends on contexts only determined dynamically at runtime,
  then the `register_callsite` method should be overridden to return
  `Interest::sometimes`. In that case, this function will be called every
  time that span or event occurs.
  
  
  

- `fn Subscriber::new_span(&self, span: &span::Attributes<'_>) -> span::Id`

  Visit the construction of a new span, returning a new [span ID] for the
  span being constructed.
  
  The provided [`Attributes`](../span/index.md) contains any field values that were provided
  when the span was created. The subscriber may pass a [`visitor`](../../regex_syntax/ast/visitor/index.md) to the
  `Attributes`' [`record` method] to record these values.
  
  IDs are used to uniquely identify spans and events within the context of a
  subscriber, so span equality will be based on the returned ID. Thus, if
  the subscriber wishes for all spans with the same metadata to be
  considered equal, it should return the same ID every time it is given a
  particular set of metadata. Similarly, if it wishes for two separate
  instances of a span with the same metadata to *not* be equal, it should
  return a distinct ID every time this function is called, regardless of
  the metadata.
  
  Note that the subscriber is free to assign span IDs based on whatever
  scheme it sees fit. Any guarantees about uniqueness, ordering, or ID
  reuse are left up to the subscriber implementation to determine.
  
  
  

- `fn Subscriber::record(&self, span: &span::Id, values: &span::Record<'_>)`

  Record a set of values on a span.
  
  This method will be invoked when value is recorded on a span.
  Recording multiple values for the same field is possible,
  but the actual behaviour is defined by the subscriber implementation.
  
  Keep in mind that a span might not provide a value
  for each field it declares.
  
  The subscriber is expected to provide a [`visitor`](../../regex_syntax/ast/visitor/index.md) to the `Record`'s
  [`record` method] in order to record the added values.
  
  ##### Example
   "foo = 3" will be recorded when `record` is called on the
  `Attributes` passed to `new_span`.
  Since values are not provided for the `bar` and `baz` fields,
  the span's `Metadata` will indicate that it _has_ those fields,
  but values for them won't be recorded at this time.
  
  ```rust,ignore
  use tracing::span;
  
  let mut span = span!("my_span", foo = 3, bar, baz);
  
  // `Subscriber::record` will be called with a `Record`
  // containing "bar = false"
  span.record("bar", &false);
  
  // `Subscriber::record` will be called with a `Record`
  // containing "baz = "a string""
  span.record("baz", &"a string");
  ```
  
  

- `fn Subscriber::record_follows_from(&self, span: &span::Id, follows: &span::Id)`

  Adds an indication that `span` follows from the span with the id
  `follows`.
  
  This relationship differs somewhat from the parent-child relationship: a
  span may have any number of prior spans, rather than a single one; and
  spans are not considered to be executing _inside_ of the spans they
  follow from. This means that a span may close even if subsequent spans
  that follow from it are still open, and time spent inside of a
  subsequent span should not be included in the time its precedents were
  executing. This is used to model causal relationships such as when a
  single future spawns several related background tasks, et cetera.
  
  If the subscriber has spans corresponding to the given IDs, it should
  record this relationship in whatever way it deems necessary. Otherwise,
  if one or both of the given span IDs do not correspond to spans that the
  subscriber knows about, or if a cyclical relationship would be created
  (i.e., some span _a_ which proceeds some other span _b_ may not also
  follow from _b_), it may silently do nothing.

- `fn Subscriber::event(&self, event: &Event<'_>)`

  Records that an [`Event`](../event/index.md) has occurred.
  
  This method will be invoked when an Event is constructed by
  the `Event`'s [`dispatch` method]. For example, this happens internally
  when an event macro from `tracing` is called.
  
  The key difference between this method and `record` is that `record` is
  called when a value is recorded for a field defined by a span,
  while `event` is called when a new event occurs.
  
  The provided `Event` struct contains any field values attached to the
  event. The subscriber may pass a [`visitor`](../../regex_syntax/ast/visitor/index.md) to the `Event`'s
  [`record` method] to record these values.
  
  
  

- `fn Subscriber::enter(&self, span: &span::Id)`

  Records that a span has been entered.
  
  When entering a span, this method is called to notify the subscriber
  that the span has been entered. The subscriber is provided with the
  [span ID] of the entered span, and should update any internal state
  tracking the current span accordingly.

- `fn Subscriber::exit(&self, span: &span::Id)`

  Records that a span has been exited.
  
  When exiting a span, this method is called to notify the subscriber
  that the span has been exited. The subscriber is provided with the
  [span ID] of the exited span, and should update any internal state
  tracking the current span accordingly.
  
  Exiting a span does not imply that the span will not be re-entered.

#### Provided Methods

- `fn Subscriber::on_register_dispatch(&self, subscriber: &Dispatch)`

  Invoked when this subscriber becomes a [`Dispatch`](../dispatcher/index.md).
  
  ###### Avoiding Memory Leaks
  
  `Subscriber`s should not store their own [`Dispatch`](../dispatcher/index.md). Because the
  `Dispatch` owns the `Subscriber`, storing the `Dispatch` within the
  `Subscriber` will create a reference count cycle, preventing the `Dispatch`
  from ever being dropped.
  
  Instead, when it is necessary to store a cyclical reference to the
  `Dispatch` within a `Subscriber`, use `Dispatch::downgrade` to convert a
  `Dispatch` into a [`WeakDispatch`](../dispatcher/index.md). This type is analogous to
  `std::sync::Weak`, and does not create a reference count cycle. A
  [`WeakDispatch`](../dispatcher/index.md) can be stored within a `Subscriber` without causing a
  memory leak, and can be [upgraded] into a `Dispatch` temporarily when
  the `Dispatch` must be accessed by the `Subscriber`.
  

- `fn Subscriber::register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest`

  Registers a new [`callsite`](../callsite/index.md) with this subscriber, returning whether or not
  the subscriber is interested in being notified about the callsite.
  
  By default, this function assumes that the subscriber's [`filter`](../../rayon/iter/filter/index.md)
  represents an unchanging view of its interest in the callsite. However,
  if this is not the case, subscribers may override this function to
  indicate different interests, or to implement behaviour that should run
  once for every callsite.
  
  This function is guaranteed to be called at least once per callsite on
  every active subscriber. The subscriber may store the keys to fields it
  cares about in order to reduce the cost of accessing fields by name,
  preallocate storage for that callsite, or perform any other actions it
  wishes to perform once for each callsite.
  
  The subscriber should then return an [`Interest`](#interest), indicating
  whether it is interested in being notified about that callsite in the
  future. This may be `Always` indicating that the subscriber always
  wishes to be notified about the callsite, and its filter need not be
  re-evaluated; `Sometimes`, indicating that the subscriber may sometimes
  care about the callsite but not always (such as when sampling), or
  `Never`, indicating that the subscriber never wishes to be notified about
  that callsite. If all active subscribers return `Never`, a callsite will
  never be enabled unless a new subscriber expresses interest in it.
  
  `Subscriber`s which require their filters to be run every time an event
  occurs or a span is entered/exited should return `Interest::sometimes`.
  If a subscriber returns `Interest::sometimes`, then its `enabled` method
  will be called every time an event or span is created from that callsite.
  
  For example, suppose a sampling subscriber is implemented by
  incrementing a counter every time `enabled` is called and only returning
  `true` when the counter is divisible by a specified sampling rate. If
  that subscriber returns `Interest::always` from `register_callsite`, then
  the filter will not be re-evaluated once it has been applied to a given
  set of metadata. Thus, the counter will not be incremented, and the span
  or event that corresponds to the metadata will never be `enabled`.
  
  `Subscriber`s that need to change their filters occasionally should call
  [`rebuild_interest_cache`](../callsite/index.md) to re-evaluate `register_callsite` for all
  callsites.
  
  Similarly, if a `Subscriber` has a filtering strategy that can be
  changed dynamically at runtime, it would need to re-evaluate that filter
  if the cached results have changed.
  
  A subscriber which manages fanout to multiple other subscribers
  should proxy this decision to all of its child subscribers,
  returning `Interest::never` only if _all_ such children return
  `Interest::never`. If the set of subscribers to which spans are
  broadcast may change dynamically, the subscriber should also never
  return `Interest::Never`, as a new subscriber may be added that _is_
  interested.
  
  See the [documentation on the callsite registry][cs-reg] for more
  details on how and when the `register_callsite` method is called.
  
  ##### Notes
  This function may be called again when a new subscriber is created or
  when the registry is invalidated.
  
  If a subscriber returns `Interest::never` for a particular callsite, it
  _may_ still see spans and events originating from that callsite, if
  another subscriber expressed interest in it.
  
  
  
  
  

- `fn Subscriber::max_level_hint(&self) -> Option<LevelFilter>`

  Returns the highest [verbosity level][`level`](../../tracing_attributes/attr/kw/index.md) that this `Subscriber` will
  enable, or `None`, if the subscriber does not implement level-based
  filtering or chooses not to implement this method.
  
  If this method returns a [`Level`][`level`](../../tracing_attributes/attr/kw/index.md), it will be used as a hint to
  determine the most verbose level that will be enabled. This will allow
  spans and events which are more verbose than that level to be skipped
  more efficiently. Subscribers which perform filtering are strongly
  encouraged to provide an implementation of this method.
  
  If the maximum level the subscriber will enable can change over the
  course of its lifetime, it is free to return a different value from
  multiple invocations of this method. However, note that changes in the
  maximum level will **only** be reflected after the callsite [`Interest`](#interest)
  cache is rebuilt, by calling the [`callsite::rebuild_interest_cache`][rebuild]
  function. Therefore, if the subscriber will change the value returned by
  this method, it is responsible for ensuring that
  [`rebuild_interest_cache`][rebuild] is called after the value of the max
  level changes.
  

- `fn Subscriber::event_enabled(&self, event: &Event<'_>) -> bool`

  Determine if an [`Event`](../event/index.md) should be recorded.
  
  By default, this returns `true` and `Subscriber`s can filter events in
  `event` without any penalty. However, when `event` is
  more complicated, this can be used to determine if `event` should be
  called at all, separating out the decision from the processing.

- `fn Subscriber::clone_span(&self, id: &span::Id) -> span::Id`

  Notifies the subscriber that a [span ID] has been cloned.
  
  This function is guaranteed to only be called with span IDs that were
  returned by this subscriber's `new_span` function.
  
  Note that the default implementation of this function this is just the
  identity function, passing through the identifier. However, it can be
  used in conjunction with `try_close` to track the number of handles
  capable of `enter`ing a span. When all the handles have been dropped
  (i.e., `try_close` has been called one more time than `clone_span` for a
  given ID), the subscriber may assume that the span will not be entered
  again. It is then free to deallocate storage for data associated with
  that span, write data from that span to IO, and so on.
  
  For more unsafe situations, however, if `id` is itself a pointer of some
  kind this can be used as a hook to "clone" the pointer, depending on
  what that means for the specified pointer.
  

- `fn Subscriber::drop_span(&self, _id: span::Id)`

  **This method is deprecated.**
  
  Using `drop_span` may result in subscribers composed using
  `tracing-subscriber` crate's `Layer` trait from observing close events.
  Use `try_close` instead.
  
  The default implementation of this function does nothing.

- `fn Subscriber::try_close(&self, id: span::Id) -> bool`

  Notifies the subscriber that a [span ID] has been dropped, and returns
  `true` if there are now 0 IDs that refer to that span.
  
  Higher-level libraries providing functionality for composing multiple
  subscriber implementations may use this return value to notify any
  "layered" subscribers that this subscriber considers the span closed.
  
  The default implementation of this method calls the subscriber's
  `drop_span` method and returns `false`. This means that, unless the
  subscriber overrides the default implementation, close notifications
  will never be sent to any layered subscribers. In general, if the
  subscriber tracks reference counts, this method should be implemented,
  rather than `drop_span`.
  
  This function is guaranteed to only be called with span IDs that were
  returned by this subscriber's `new_span` function.
  
  It's guaranteed that if this function has been called once more than the
  number of times `clone_span` was called with the same `id`, then no more
  handles that can enter the span with that `id` exist. This means that it
  can be used in conjunction with `clone_span` to track the number of
  handles capable of `enter`ing a span. When all the handles have been
  dropped (i.e., `try_close` has been called one more time than
  `clone_span` for a given ID), the subscriber may assume that the span
  will not be entered again, and should return `true`. It is then free to
  deallocate storage for data associated with that span, write data from
  that span to IO, and so on.
  
  **Note**: since this function is called when spans are dropped,
  implementations should ensure that they are unwind-safe. Panicking from
  inside of a `try_close` function may cause a double panic, if the span
  was dropped due to a thread unwinding.
  
  

- `fn Subscriber::current_span(&self) -> span::Current`

  Returns a type representing this subscriber's view of the current span.
  
  If subscribers track a current span, they should override this function
  to return `Current::new` if the thread from which this method is
  called is inside a span, or `Current::none` if the thread is not
  inside a span.
  
  By default, this returns a value indicating that the subscriber
  does **not** track what span is current. If the subscriber does not
  implement a current span, it should not override this method.
  

- `fn Subscriber::downcast_raw(&self, id: TypeId) -> Option<*const ()>`

  If `self` is the same type as the provided `TypeId`, returns an untyped
  `*const` pointer to that type. Otherwise, returns `None`.
  
  If you wish to downcast a `Subscriber`, it is strongly advised to use
  the safe API provided by `downcast_ref` instead.
  
  This API is required for `downcast_raw` to be a trait method; a method
  signature like `downcast_ref` (with a generic type parameter) is not
  object-safe, and thus cannot be a trait method for `Subscriber`. This
  means that if we only exposed `downcast_ref`, `Subscriber`
  implementations could not override the downcasting behavior
  
  This method may be overridden by "fan out" or "chained" subscriber
  implementations which consist of multiple composed types. Such
  subscribers might allow `downcast_raw` by returning references to those
  component if they contain components with the given `TypeId`.
  
  ##### Safety
  
  The `downcast_ref` method expects that the pointer returned by
  `downcast_raw` is non-null and points to a valid instance of the type
  with the provided `TypeId`. Failure to ensure this will result in
  undefined behaviour, so implementing `downcast_raw` is unsafe.

#### Implementors

- [`NoSubscriber`](#nosubscriber)
- `alloc::boxed::Box<S>`
- `alloc::sync::Arc<S>`

