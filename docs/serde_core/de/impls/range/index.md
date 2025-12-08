*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range](index.md)*

---

# Module `range`

## Structs

### `RangeVisitor<Idx>`

```rust
struct RangeVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for RangeVisitor<Idx>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, Idx> Visitor for RangeVisitor<Idx>`

- `type Value = (Idx, Idx)`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

- `fn visit_map<A>(self: Self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

## Enums

### `Field`

```rust
enum Field {
    Start,
    End,
}
```

#### Trait Implementations

##### `impl<'de> Deserialize for Field`

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md)

##### `impl<T> DeserializeOwned for Field`

## Constants

### `FIELDS`

```rust
const FIELDS: &[&str];
```

