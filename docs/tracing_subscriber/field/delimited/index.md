*[tracing_subscriber](../../index.md) / [field](../index.md) / [delimited](index.md)*

---

# Module `delimited`

A `MakeVisitor` wrapper that separates formatted fields with a delimiter.

## Structs

### `Delimited<D, V>`

```rust
struct Delimited<D, V> {
    delimiter: D,
    inner: V,
}
```

A `MakeVisitor` wrapper that wraps a visitor that writes formatted output so
that a delimiter is inserted between writing formatted field values.

#### Implementations

- `fn new(delimiter: D, inner: V) -> Self`

#### Trait Implementations

##### `impl<D: $crate::clone::Clone, V: $crate::clone::Clone> Clone for Delimited<D, V>`

- `fn clone(self: &Self) -> Delimited<D, V>` — [`Delimited`](#delimited)

##### `impl<D: $crate::fmt::Debug, V: $crate::fmt::Debug> Debug for Delimited<D, V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for Delimited<D, V>`

##### `impl<T, M> MakeExt for Delimited<D, V>`

##### `impl<D, V, T> MakeVisitor for Delimited<D, V>`

- `type Visitor = VisitDelimited<D, <V as MakeVisitor>::Visitor>`

- `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor` — [`MakeVisitor`](../index.md)

##### `impl<T, M> Sealed for Delimited<D, V>`

##### `impl<T> WithSubscriber for Delimited<D, V>`

### `VisitDelimited<D, V>`

```rust
struct VisitDelimited<D, V> {
    delimiter: D,
    seen: bool,
    inner: V,
    err: fmt::Result,
}
```

A visitor wrapper that inserts a delimiter after the wrapped visitor formats
a field value.

#### Implementations

- `fn new(delimiter: D, inner: V) -> Self`

- `fn delimit(self: &mut Self)`

#### Trait Implementations

##### `impl<D: $crate::fmt::Debug, V: $crate::fmt::Debug> Debug for VisitDelimited<D, V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for VisitDelimited<D, V>`

##### `impl<D, V> Visit for VisitDelimited<D, V>`

- `fn record_i64(self: &mut Self, field: &Field, value: i64)`

- `fn record_u64(self: &mut Self, field: &Field, value: u64)`

- `fn record_bool(self: &mut Self, field: &Field, value: bool)`

- `fn record_str(self: &mut Self, field: &Field, value: &str)`

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

##### `impl<D, V> VisitFmt for VisitDelimited<D, V>`

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

##### `impl<D, V> VisitOutput for VisitDelimited<D, V>`

- `fn finish(self: Self) -> fmt::Result`

##### `impl<T> WithSubscriber for VisitDelimited<D, V>`

