*[tracing_core](../index.md) / [event](index.md)*

---

# Module `event`

Events represent single points in time during the execution of a program.

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
- Like spans, `Event`s have structured key-value data known as _[fields]_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.



#### Implementations

- `fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Metadata`](../metadata/index.md), [`ValueSet`](../field/index.md)

- `fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Metadata`](../metadata/index.md), [`ValueSet`](../field/index.md)

- `fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self` — [`Id`](../span/index.md), [`Metadata`](../metadata/index.md), [`ValueSet`](../field/index.md)

- `fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)` — [`Id`](../span/index.md), [`Metadata`](../metadata/index.md), [`ValueSet`](../field/index.md)

- `fn record(self: &Self, visitor: &mut dyn field::Visit)` — [`Visit`](../field/index.md)

- `fn fields(self: &Self) -> field::Iter` — [`Iter`](../field/index.md)

- `fn metadata(self: &Self) -> &'static Metadata<'static>` — [`Metadata`](../metadata/index.md)

- `fn is_root(self: &Self) -> bool`

- `fn is_contextual(self: &Self) -> bool`

- `fn parent(self: &Self) -> Option<&Id>` — [`Id`](../span/index.md)

#### Trait Implementations

##### `impl<'a> Debug for Event<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> NormalizeEvent for tracing_core::Event<'a>`

##### `impl RecordFields for tracing_core::Event<'_>`

- `fn exit(self: &Self, span: &span::Id)` — [`Id`](../span/index.md)

##### `impl<'a> Sealed for tracing_core::Event<'a>`

