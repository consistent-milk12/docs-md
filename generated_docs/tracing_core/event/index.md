*[tracing_core](../index.md) / [event](index.md)*

---

# Module `event`

Events represent single points in time during the execution of a program.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Event`](#event) | struct | `Event`s represent single points in time where something occurred during the execution of a program. |

## Structs

### `Event<'a>`

```rust
struct Event<'a> {
    fields: &'a field::ValueSet<'a>,
    metadata: &'static crate::Metadata<'static>,
    parent: crate::parent::Parent,
}
```

*Defined in [`tracing-core-0.1.35/src/event.rs:23-27`](../../../.source_1765521767/tracing-core-0.1.35/src/event.rs#L23-L27)*

`Event`s represent single points in time where something occurred during the
execution of a program.

An `Event` can be compared to a log record in unstructured logging, but with
two key differences:
- `Event`s exist _within the context of a [`span`](../span/index.md)_. Unlike log lines, they
  may be located within the trace tree, allowing visibility into the
  _temporal_ context in which the event occurred, as well as the source
  code location.
- Like spans, `Event`s have structured key-value data known as _[`fields`](../../tracing_attributes/attr/kw/index.md)_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.



#### Implementations

- <span id="event-dispatch"></span>`fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Metadata`](../metadata/index.md#metadata), [`ValueSet`](../field/index.md#valueset)

  Constructs a new `Event` with the specified metadata and set of values,

  and observes it with the current subscriber.

- <span id="event-new"></span>`fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Metadata`](../metadata/index.md#metadata), [`ValueSet`](../field/index.md#valueset)

  Returns a new `Event` in the current span, with the specified metadata

  and set of values.

- <span id="event-new-child-of"></span>`fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Id`](../span/index.md#id), [`Metadata`](../metadata/index.md#metadata), [`ValueSet`](../field/index.md#valueset)

  Returns a new `Event` as a child of the specified span, with the

  provided metadata and set of values.

- <span id="event-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Id`](../span/index.md#id), [`Metadata`](../metadata/index.md#metadata), [`ValueSet`](../field/index.md#valueset)

  Constructs a new `Event` with the specified metadata and set of values,

  and observes it with the current subscriber and an explicit parent.

- <span id="event-record"></span>`fn record(&self, visitor: &mut dyn field::Visit)` — [`Visit`](../field/index.md#visit)

  Visits all the fields on this `Event` with the specified [`visitor`](../../regex_syntax/ast/visitor/index.md).

- <span id="event-fields"></span>`fn fields(&self) -> field::Iter` — [`Iter`](../field/index.md#iter)

  Returns an iterator over the set of values on this `Event`.

- <span id="event-metadata"></span>`fn metadata(&self) -> &'static Metadata<'static>` — [`Metadata`](../metadata/index.md#metadata)

  Returns [`metadata`](../metadata/index.md) describing this `Event`.

- <span id="event-is-root"></span>`fn is_root(&self) -> bool`

  Returns true if the new event should be a root.

- <span id="event-is-contextual"></span>`fn is_contextual(&self) -> bool`

  Returns true if the new event's parent should be determined based on the

  current context.

  

  If this is true and the current thread is currently inside a span, then

  that span should be the new event's parent. Otherwise, if the current

  thread is _not_ inside a span, then the new event will be the root of its

  own trace tree.

- <span id="event-parent"></span>`fn parent(&self) -> Option<&Id>` — [`Id`](../span/index.md#id)

  Returns the new event's explicitly-specified parent, if there is one.

  

  Otherwise (if the new event is a root or is a child of the current span),

  returns `None`.

#### Trait Implementations

##### `impl Any for Event<'a>`

- <span id="event-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Event<'a>`

- <span id="event-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Event<'a>`

- <span id="event-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Event<'a>`

- <span id="event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Event<'a>`

- <span id="event-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Event<'a>`

- <span id="event-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Event<'a>`

- <span id="event-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="event-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Event<'a>`

- <span id="event-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="event-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

