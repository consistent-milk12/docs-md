*[tracing_subscriber](../../index.md) / [field](../index.md) / [display](index.md)*

---

# Module `display`

`MakeVisitor` wrappers for working with `fmt::Display` fields.

## Structs

### `Messages<V>`

```rust
struct Messages<V>(V);
```

A visitor wrapper that ensures any strings named "message" are formatted
using `fmt::Display`

#### Implementations

- `fn new(inner: V) -> Self`

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for Messages<V>`

- `fn clone(self: &Self) -> Messages<V>` — [`Messages`](#messages)

##### `impl<V: $crate::fmt::Debug> Debug for Messages<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Messages<V>`

##### `impl<T, M> MakeExt for Messages<V>`

##### `impl<T, V> MakeVisitor for Messages<V>`

- `type Visitor = Messages<<V as MakeVisitor>::Visitor>`

- `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor` — [`MakeVisitor`](../index.md)

##### `impl<T, M> Sealed for Messages<V>`

##### `impl<V> Visit for Messages<V>`

- `fn record_f64(self: &mut Self, field: &Field, value: f64)`

- `fn record_i64(self: &mut Self, field: &Field, value: i64)`

- `fn record_u64(self: &mut Self, field: &Field, value: u64)`

- `fn record_bool(self: &mut Self, field: &Field, value: bool)`

- `fn record_str(self: &mut Self, field: &Field, value: &str)`

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

##### `impl<V> VisitFmt for Messages<V>`

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

##### `impl<V, O> VisitOutput for Messages<V>`

- `fn finish(self: Self) -> O`

##### `impl<V> VisitWrite for Messages<V>`

- `fn writer(self: &mut Self) -> &mut dyn io::Write`

##### `impl<T> WithSubscriber for Messages<V>`

