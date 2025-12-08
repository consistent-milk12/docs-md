*[tracing_subscriber](../index.md) / [registry](index.md)*

---

# Module `registry`

Storage for span data shared by multiple [`Layer`](../layer/index.md)s.

## Using the Span Registry

This module provides the [`Registry`](sharded/index.md) type, a [`Subscriber`](../fmt/index.md) implementation
which tracks per-span data and exposes it to [`Layer`](../layer/index.md)s. When a `Registry`
is used as the base `Subscriber` of a `Layer` stack, the
[`layer::Context`][ctx] type will provide methods allowing `Layer`s to
[look up span data][lookup] stored in the registry. While [`Registry`](sharded/index.md) is a
reasonable default for storing spans and events, other stores that implement
[`LookupSpan`](#lookupspan) and [`Subscriber`](../fmt/index.md) themselves (with [`SpanData`](#spandata) implemented
by the per-span data they store) can be used as a drop-in replacement.

For example, we might create a `Registry` and add multiple `Layer`s like so:
```rust
use tracing_subscriber::{registry::Registry, Layer, prelude::*};
use tracing_core::Subscriber;
pub struct FooLayer {}
pub struct BarLayer {}
impl<S: Subscriber> Layer<S> for FooLayer {}
impl<S: Subscriber> Layer<S> for BarLayer {}
impl FooLayer {
fn new() -> Self { Self {} }
}
impl BarLayer {
fn new() -> Self { Self {} }
}

let subscriber = Registry::default()
    .with(FooLayer::new())
    .with(BarLayer::new());
```

If a type implementing `Layer` depends on the functionality of a `Registry`
implementation, it should bound its `Subscriber` type parameter with the
[`LookupSpan`](#lookupspan) trait, like so:

```rust
use tracing_subscriber::{registry, Layer};
use tracing_core::Subscriber;

pub struct MyLayer {
    // ...
}

impl<S> Layer<S> for MyLayer
where
    S: Subscriber + for<'a> registry::LookupSpan<'a>,
{
    // ...
}
```
When this bound is added, the `Layer` implementation will be guaranteed
access to the [`Context`][ctx] methods, such as [`Context::span`][lookup], that
require the root subscriber to be a registry.





## Structs

### `Extensions<'a>`

```rust
struct Extensions<'a> {
    inner: std::sync::RwLockReadGuard<'a, ExtensionsInner>,
}
```

An immutable, read-only reference to a Span's extensions.

#### Implementations

- `fn new(inner: RwLockReadGuard<'a, ExtensionsInner>) -> Self` — [`ExtensionsInner`](extensions/index.md)

- `fn get<T: 'static>(self: &Self) -> Option<&T>`

#### Trait Implementations

##### `impl<'a> Debug for Extensions<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Extensions<'a>`

##### `impl<T> WithSubscriber for Extensions<'a>`

### `ExtensionsMut<'a>`

```rust
struct ExtensionsMut<'a> {
    inner: std::sync::RwLockWriteGuard<'a, ExtensionsInner>,
}
```

An mutable reference to a Span's extensions.

#### Implementations

- `fn new(inner: RwLockWriteGuard<'a, ExtensionsInner>) -> Self` — [`ExtensionsInner`](extensions/index.md)

- `fn insert<T: Send + Sync + 'static>(self: &mut Self, val: T)`

- `fn replace<T: Send + Sync + 'static>(self: &mut Self, val: T) -> Option<T>`

- `fn get_mut<T: 'static>(self: &mut Self) -> Option<&mut T>`

- `fn remove<T: Send + Sync + 'static>(self: &mut Self) -> Option<T>`

#### Trait Implementations

##### `impl<'a> Debug for ExtensionsMut<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for ExtensionsMut<'a>`

##### `impl<T> WithSubscriber for ExtensionsMut<'a>`

### `Data<'a>`

```rust
struct Data<'a> {
    inner: sharded_slab::pool::Ref<'a, DataInner>,
}
```

Span data stored in a [`Registry`](sharded/index.md).

The registry stores well-known data defined by tracing: span relationships,
metadata and reference counts. Additional user-defined data provided by
[`Layer`s], such as formatted fields, metrics, or distributed traces should
be stored in the [`extensions`](extensions/index.md) typemap.



#### Fields

- **`inner`**: `sharded_slab::pool::Ref<'a, DataInner>`

  Immutable reference to the pooled `DataInner` entry.

#### Trait Implementations

##### `impl<'a> Debug for Data<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Data<'a>`

##### `impl<'a> SpanData for Data<'a>`

- `fn id(self: &Self) -> Id`

- `fn metadata(self: &Self) -> &'static Metadata<'static>`

- `fn parent(self: &Self) -> Option<&Id>`

- `fn extensions(self: &Self) -> Extensions<'_>` — [`Extensions`](extensions/index.md)

- `fn extensions_mut(self: &Self) -> ExtensionsMut<'_>` — [`ExtensionsMut`](extensions/index.md)

- `fn is_enabled_for(self: &Self, filter: FilterId) -> bool` — [`FilterId`](../filter/index.md)

##### `impl<T> WithSubscriber for Data<'a>`

### `Registry`

```rust
struct Registry {
    spans: sharded_slab::Pool<DataInner>,
    current_spans: thread_local::ThreadLocal<core::cell::RefCell<super::stack::SpanStack>>,
    next_filter_id: u8,
}
```

A shared, reusable store for spans.

A `Registry` is a [`Subscriber`](../fmt/index.md) around which multiple [`Layer`](../layer/index.md)s
implementing various behaviors may be [added]. Unlike other types
implementing `Subscriber`, `Registry` does not actually record traces itself:
instead, it collects and stores span data that is exposed to any [`Layer`](../layer/index.md)s
wrapping it through implementations of the [`LookupSpan`](#lookupspan) trait.
The `Registry` is responsible for storing span metadata, recording
relationships between spans, and tracking which spans are active and which
are closed. In addition, it provides a mechanism for [`Layer`](../layer/index.md)s to store
user-defined per-span data, called [`extensions`](extensions/index.md), in the registry. This
allows [`Layer`](../layer/index.md)-specific data to benefit from the `Registry`'s
high-performance concurrent storage.

This registry is implemented using a [lock-free sharded slab][slab], and is
highly optimized for concurrent access.

# Span ID Generation

Span IDs are not globally unique, but the registry ensures that
no two currently active spans have the same ID within a process.

One of the primary responsibilities of the registry is to generate [span
IDs]. Therefore, it's important for other code that interacts with the
registry, such as [`Layer`](../layer/index.md)s, to understand the guarantees of the
span IDs that are generated.

The registry's span IDs are guaranteed to be unique **at a given point
in time**. This means that an active span will never be assigned the
same ID as another **currently active** span. However, the registry
**will** eventually reuse the IDs of [closed] spans, although an ID
will never be reassigned immediately after a span has closed.

Spans are not [considered closed] by the `Registry` until *every*
[`Span`](#span) reference with that ID has been dropped.

Thus: span IDs generated by the registry should be considered unique
only at a given point in time, and only relative to other spans
generated by the same process. Two spans with the same ID will not exist
in the same process concurrently. However, if historical span data is
being stored, the same ID may occur for multiple spans times in that
data. If spans must be uniquely identified in historical data, the user
code storing this data must assign its own unique identifiers to those
spans. A counter is generally sufficient for this.

Similarly, span IDs generated by the registry are not unique outside of
a given process. Distributed tracing systems may require identifiers
that are unique across multiple processes on multiple machines (for
example, [OpenTelemetry's `SpanId`s and `TraceId`s][ot]). `tracing` span
IDs generated by the registry should **not** be used for this purpose.
Instead, code which integrates with a distributed tracing system should
generate and propagate its own IDs according to the rules specified by
the distributed tracing system. These IDs can be associated with
`tracing` spans using [`fields`](../macros/index.md) and/or [stored span data].












#### Implementations

- `fn get(self: &Self, id: &Id) -> Option<Ref<'_, DataInner>>` — [`DataInner`](sharded/index.md)

- `fn start_close(self: &Self, id: Id) -> CloseGuard<'_>` — [`CloseGuard`](sharded/index.md)

- `fn has_per_layer_filters(self: &Self) -> bool`

- `fn span_stack(self: &Self) -> cell::Ref<'_, SpanStack>` — [`SpanStack`](stack/index.md)

#### Trait Implementations

##### `impl Debug for Registry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Registry`

- `fn default() -> Self`

##### `impl<T> Instrument for Registry`

##### `impl<'a> LookupSpan for Registry`

- `type Data = Data<'a>`

- `fn span_data(self: &'a Self, id: &Id) -> Option<<Self as >::Data>` — [`LookupSpan`](#lookupspan)

- `fn register_filter(self: &mut Self) -> FilterId` — [`FilterId`](../filter/index.md)

##### `impl<S> Sealed for Registry`

##### `impl Subscriber for Registry`

- `fn register_callsite(self: &Self, _: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, _: &Metadata<'_>) -> bool`

- `fn new_span(self: &Self, attrs: &span::Attributes<'_>) -> span::Id`

- `fn record(self: &Self, _: &span::Id, _: &span::Record<'_>)`

- `fn record_follows_from(self: &Self, _span: &span::Id, _follows: &span::Id)`

- `fn event_enabled(self: &Self, _event: &Event<'_>) -> bool`

- `fn event(self: &Self, _: &Event<'_>)`

- `fn enter(self: &Self, id: &span::Id)`

- `fn exit(self: &Self, id: &span::Id)`

- `fn clone_span(self: &Self, id: &span::Id) -> span::Id`

- `fn current_span(self: &Self) -> Current`

- `fn try_close(self: &Self, id: span::Id) -> bool`

##### `impl<S> SubscriberExt for Registry`

##### `impl<T> SubscriberInitExt for Registry`

##### `impl<T> WithSubscriber for Registry`

### `SpanRef<'a, R: LookupSpan<'a>>`

```rust
struct SpanRef<'a, R: LookupSpan<'a>> {
    registry: &'a R,
    data: <R as >::Data,
    filter: crate::filter::FilterId,
}
```

A reference to [span data] and the associated [registry](#registry).

This type implements all the same methods as [`SpanData`](#spandata), and provides
additional methods for querying the registry based on values from the span.


#### Implementations

- `fn id(self: &Self) -> Id`

- `fn metadata(self: &Self) -> &'static Metadata<'static>`

- `fn name(self: &Self) -> &'static str`

- `fn fields(self: &Self) -> &FieldSet`

- `fn parent(self: &Self) -> Option<Self>`

- `fn scope(self: &Self) -> Scope<'a, R>` — [`Scope`](#scope)

- `fn extensions(self: &Self) -> Extensions<'_>` — [`Extensions`](extensions/index.md)

- `fn extensions_mut(self: &Self) -> ExtensionsMut<'_>` — [`ExtensionsMut`](extensions/index.md)

- `fn try_with_filter(self: Self, filter: FilterId) -> Option<Self>` — [`FilterId`](../filter/index.md)

- `fn is_enabled_for(self: &Self, filter: FilterId) -> bool` — [`FilterId`](../filter/index.md)

- `fn with_filter(self: Self, filter: FilterId) -> Self` — [`FilterId`](../filter/index.md)

#### Trait Implementations

##### `impl<'a, R: $crate::fmt::Debug + LookupSpan<'a>> Debug for SpanRef<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for SpanRef<'a, R>`

##### `impl<T> WithSubscriber for SpanRef<'a, R>`

### `Scope<'a, R>`

```rust
struct Scope<'a, R> {
    registry: &'a R,
    next: Option<tracing_core::span::Id>,
    filter: crate::filter::FilterId,
}
```

An iterator over the parents of a span, ordered from leaf to root.

This is returned by the `SpanRef::scope` method.

#### Implementations

- `fn from_root(self: Self) -> ScopeFromRoot<'a, R>` — [`ScopeFromRoot`](#scopefromroot)

#### Trait Implementations

##### `impl<'a, R: $crate::fmt::Debug> Debug for Scope<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Scope<'a, R>`

##### `impl<I> IntoIterator for Scope<'a, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, R> Iterator for Scope<'a, R>`

- `type Item = SpanRef<'a, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T> WithSubscriber for Scope<'a, R>`

### `ScopeFromRoot<'a, R>`

```rust
struct ScopeFromRoot<'a, R>
where
    R: LookupSpan<'a> {
    spans: iter::Rev<smallvec::IntoIter<[SpanRef<'a, R>; 16]>>,
}
```

An iterator over the parents of a span, ordered from root to leaf.

This is returned by the `Scope::from_root` method.

#### Trait Implementations

##### `impl<'a, R> Debug for ScopeFromRoot<'a, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for ScopeFromRoot<'a, R>`

##### `impl<I> IntoIterator for ScopeFromRoot<'a, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, R> Iterator for ScopeFromRoot<'a, R>`

- `type Item = SpanRef<'a, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> WithSubscriber for ScopeFromRoot<'a, R>`

## Traits

### `LookupSpan<'a>`

```rust
trait LookupSpan<'a> { ... }
```

Provides access to stored span data.

Subscribers which store span data and associate it with span IDs should
implement this trait; if they do, any [`Layer`](../layer/index.md)s wrapping them can look up
metadata via the [`Context`](../layer/index.md) type's `span()` method.




#### Required Methods

- `type Data: 1`

- `fn span_data(self: &'a Self, id: &Id) -> Option<<Self as >::Data>`

  Returns the [`SpanData`](#spandata) for a given `Id`, if it exists.

- `fn span(self: &'a Self, id: &Id) -> Option<SpanRef<'a, Self>>`

  Returns a [`SpanRef`](#spanref) for the span with the given `Id`, if it exists.

- `fn register_filter(self: &mut Self) -> FilterId`

  Registers a [`Filter`](../layer/index.md) for [per-layer filtering] with this

### `SpanData<'a>`

```rust
trait SpanData<'a> { ... }
```

A stored representation of data associated with a span.

#### Required Methods

- `fn id(self: &Self) -> Id`

  Returns this span's ID.

- `fn metadata(self: &Self) -> &'static Metadata<'static>`

  Returns a reference to the span's `Metadata`.

- `fn parent(self: &Self) -> Option<&Id>`

  Returns a reference to the ID

- `fn extensions(self: &Self) -> Extensions<'_>`

  Returns a reference to this span's `Extensions`.

- `fn extensions_mut(self: &Self) -> ExtensionsMut<'_>`

  Returns a mutable reference to this span's `Extensions`.

- `fn is_enabled_for(self: &Self, filter: FilterId) -> bool`

  Returns `true` if this span is enabled for the [per-layer filter][plf]

