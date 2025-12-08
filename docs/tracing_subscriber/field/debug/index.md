*[tracing_subscriber](../../index.md) / [field](../index.md) / [debug](index.md)*

---

# Module `debug`

`MakeVisitor` wrappers for working with `fmt::Debug` fields.

## Structs

### `Alt<V>`

```rust
struct Alt<V>(V);
```

A visitor wrapper that ensures any `fmt::Debug` fields are formatted using
the alternate (`:#`) formatter.

#### Implementations

- `fn new(inner: V) -> Self`

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for Alt<V>`

- `fn clone(self: &Self) -> Alt<V>` — [`Alt`](#alt)

##### `impl<V: $crate::fmt::Debug> Debug for Alt<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Alt<V>`

##### `impl<T, M> MakeExt for Alt<V>`

##### `impl<T, V> MakeVisitor for Alt<V>`

- `type Visitor = Alt<<V as MakeVisitor>::Visitor>`

- `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor` — [`MakeVisitor`](../index.md)

##### `impl<T, M> Sealed for Alt<V>`

##### `impl<V> Visit for Alt<V>`

- `fn record_f64(self: &mut Self, field: &Field, value: f64)`

- `fn record_i64(self: &mut Self, field: &Field, value: i64)`

- `fn record_u64(self: &mut Self, field: &Field, value: u64)`

- `fn record_bool(self: &mut Self, field: &Field, value: bool)`

- `fn record_str(self: &mut Self, field: &Field, value: &str)`

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

##### `impl<V> VisitFmt for Alt<V>`

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

##### `impl<V, O> VisitOutput for Alt<V>`

- `fn finish(self: Self) -> O`

##### `impl<V> VisitWrite for Alt<V>`

- `fn writer(self: &mut Self) -> &mut dyn io::Write`

##### `impl<T> WithSubscriber for Alt<V>`

