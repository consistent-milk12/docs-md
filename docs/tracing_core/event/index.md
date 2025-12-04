*[tracing_core](../index.md) / [event](index.md)*

---

# Module `event`

Events represent single points in time during the execution of a program.

## Structs

### `Event<'a>`

```rust
struct Event<'a> {
}
```

`Event`s represent single points in time where something occurred during the
execution of a program.

An `Event` can be compared to a log record in unstructured logging, but with
two key differences:
- `Event`s exist _within the context of a [span](#span)
_. Unlike log lines, they
  may be located within the trace tree, allowing visibility into the
  _temporal_ context in which the event occurred, as well as the source
  code location.
- Like spans, `Event`s have structured key-value data known as _[fields](#fields)
_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.

[span](#span)
: super::span
[fields](#fields)
: super::field

#### Implementations

- `fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)`
  Constructs a new `Event` with the specified metadata and set of values,

- `fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self`
  Returns a new `Event` in the current span, with the specified metadata

- `fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self`
  Returns a new `Event` as a child of the specified span, with the

- `fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)`
  Constructs a new `Event` with the specified metadata and set of values,

- `fn record(self: &Self, visitor: &mut dyn field::Visit)`
  Visits all the fields on this `Event` with the specified [visitor].

- `fn fields(self: &Self) -> field::Iter`
  Returns an iterator over the set of values on this `Event`.

- `fn metadata(self: &Self) -> &'static Metadata<'static>`
  Returns [metadata] describing this `Event`.

- `fn is_root(self: &Self) -> bool`
  Returns true if the new event should be a root.

- `fn is_contextual(self: &Self) -> bool`
  Returns true if the new event's parent should be determined based on the

- `fn parent(self: &Self) -> Option<&Id>`
  Returns the new event's explicitly-specified parent, if there is one.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

