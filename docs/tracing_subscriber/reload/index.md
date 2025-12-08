*[tracing_subscriber](../index.md) / [reload](index.md)*

---

# Module `reload`

Wrapper for a `Layer` to allow it to be dynamically reloaded.

This module provides a [`Layer` type] implementing the [`Layer` trait] or [`Filter` trait]
which wraps another type implementing the corresponding trait. This
allows the wrapped type to be replaced with another
instance of that type at runtime.

This can be used in cases where a subset of `Layer` or `Filter` functionality
should be dynamically reconfigured, such as when filtering directives may
change at runtime. Note that this layer introduces a (relatively small)
amount of overhead, and should thus only be used as needed.

# Examples

Reloading a [global filtering](crate::layer#global-filtering) layer:

```rust
use tracing::info;
use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filter = filter::LevelFilter::WARN;
let (filter, reload_handle) = reload::Layer::new(filter);
tracing_subscriber::registry()
  .with(filter)
  .with(fmt::Layer::default())
  .init();

// specifying the Registry type is required
let _: &reload::Handle<filter::LevelFilter, tracing_subscriber::Registry> = &reload_handle;

info!("This will be ignored");
reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO);
info!("This will be logged");
```

Reloading a [`Filtered`](crate::filter::Filtered) layer:

```rust
use tracing::info;
use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filtered_layer = fmt::Layer::default().with_filter(filter::LevelFilter::WARN);
let (filtered_layer, reload_handle) = reload::Layer::new(filtered_layer);

// specifying the Registry type is required
let _: &reload::Handle<filter::Filtered<fmt::Layer<tracing_subscriber::Registry>,
filter::LevelFilter, tracing_subscriber::Registry>,tracing_subscriber::Registry>
= &reload_handle;

tracing_subscriber::registry()
  .with(filtered_layer)
  .init();
info!("This will be ignored");
reload_handle.modify(|layer| *layer.filter_mut() = filter::LevelFilter::INFO);
info!("This will be logged");
```

## Note

The [`Layer`](#layer) implementation is unable to implement downcasting functionality,
so certain [`Layer`](#layer) will fail to downcast if wrapped in a `reload::Layer`.

If you only want to be able to dynamically change the
`Filter` on a layer, prefer wrapping that `Filter` in the `reload::Layer`.




## Structs

### `Layer<L, S>`

```rust
struct Layer<L, S> {
    inner: std::sync::Arc<std::sync::RwLock<L>>,
    _s: std::marker::PhantomData<fn(S)>,
}
```

Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.

#### Implementations

- `fn new(inner: L) -> (Self, Handle<L, S>)` — [`Handle`](#handle)

- `fn handle(self: &Self) -> Handle<L, S>` — [`Handle`](#handle)

#### Trait Implementations

##### `impl<L: $crate::fmt::Debug, S: $crate::fmt::Debug> Debug for Layer<L, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<S, L> Filter for Layer<L, S>`

- `fn callsite_enabled(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, ctx: &layer::Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record(self: &Self, span: &span::Id, values: &span::Record<'_>, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close(self: &Self, id: span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn max_level_hint(self: &Self) -> Option<LevelFilter>`

##### `impl<F, S> FilterExt for Layer<L, S>`

##### `impl<T> Instrument for Layer<L, S>`

##### `impl<L, S> Layer for Layer<L, S>`

- `fn on_register_dispatch(self: &Self, subscriber: &Dispatch)`

- `fn on_layer(self: &mut Self, subscriber: &mut S)`

- `fn register_callsite(self: &Self, metadata: &'static Metadata<'static>) -> Interest`

- `fn enabled(self: &Self, metadata: &Metadata<'_>, ctx: layer::Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn on_new_span(self: &Self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_record(self: &Self, span: &span::Id, values: &span::Record<'_>, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_follows_from(self: &Self, span: &span::Id, follows: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn event_enabled(self: &Self, event: &Event<'_>, ctx: layer::Context<'_, S>) -> bool` — [`Context`](../layer/index.md)

- `fn on_event(self: &Self, event: &Event<'_>, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_enter(self: &Self, id: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_exit(self: &Self, id: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_close(self: &Self, id: span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

- `fn on_id_change(self: &Self, old: &span::Id, new: &span::Id, ctx: layer::Context<'_, S>)` — [`Context`](../layer/index.md)

##### `impl<T> WithSubscriber for Layer<L, S>`

### `Handle<L, S>`

```rust
struct Handle<L, S> {
    inner: std::sync::Weak<std::sync::RwLock<L>>,
    _s: std::marker::PhantomData<fn(S)>,
}
```

Allows reloading the state of an associated [`Layer`](crate::layer::Layer).

#### Implementations

- `fn reload(self: &Self, new_value: impl Into<L>) -> Result<(), Error>` — [`Error`](#error)

- `fn modify(self: &Self, f: impl FnOnce(&mut L)) -> Result<(), Error>` — [`Error`](#error)

- `fn clone_current(self: &Self) -> Option<L>`

- `fn with_current<T>(self: &Self, f: impl FnOnce(&L) -> T) -> Result<T, Error>` — [`Error`](#error)

#### Trait Implementations

##### `impl<L, S> Clone for Handle<L, S>`

- `fn clone(self: &Self) -> Self`

##### `impl<L: $crate::fmt::Debug, S: $crate::fmt::Debug> Debug for Handle<L, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Handle<L, S>`

##### `impl<T> WithSubscriber for Handle<L, S>`

### `Error`

```rust
struct Error {
    kind: ErrorKind,
}
```

Indicates that an error occurred when reloading a layer.

#### Implementations

- `fn poisoned() -> Self`

- `fn is_poisoned(self: &Self) -> bool`

- `fn is_dropped(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

##### `impl<T> Instrument for Error`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

##### `impl<T> WithSubscriber for Error`

