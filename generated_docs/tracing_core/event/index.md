*[tracing_core](../index.md) / [event](index.md)*

---

# Module `event`

Events represent single points in time during the execution of a program.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Event`](#event) | struct | `Event`s represent single points in time where something occurred during the |

## Structs

### `Event<'a>`

```rust
struct Event<'a> {
    fields: &'a field::ValueSet<'a>,
    metadata: &'static crate::Metadata<'static>,
    parent: crate::parent::Parent,
}
```

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

- <span id="event-dispatch"></span>`fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Metadata`](../index.md), [`ValueSet`](../field/index.md)

- <span id="event-new"></span>`fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Metadata`](../index.md), [`ValueSet`](../field/index.md)

- <span id="event-new-child-of"></span>`fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Id`](../span/index.md), [`Metadata`](../index.md), [`ValueSet`](../field/index.md)

- <span id="event-child-of"></span>`fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Id`](../span/index.md), [`Metadata`](../index.md), [`ValueSet`](../field/index.md)

- <span id="event-record"></span>`fn record(&self, visitor: &mut dyn field::Visit)` — [`Visit`](../field/index.md)

- <span id="event-fields"></span>`fn fields(&self) -> field::Iter` — [`Iter`](../field/index.md)

- <span id="event-metadata"></span>`fn metadata(&self) -> &'static Metadata<'static>` — [`Metadata`](../index.md)

- <span id="event-is-root"></span>`fn is_root(&self) -> bool`

- <span id="event-is-contextual"></span>`fn is_contextual(&self) -> bool`

- <span id="event-parent"></span>`fn parent(&self) -> Option<&Id>` — [`Id`](../span/index.md)

#### Trait Implementations

##### `impl<'a> Debug for Event<'a>`

- <span id="event-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

