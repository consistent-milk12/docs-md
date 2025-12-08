*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range](index.md)*

---

# Module `range`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeVisitor`](#rangevisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

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

- <span id="rangevisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, Idx> Visitor for RangeVisitor<Idx>`

- <span id="rangevisitor-value"></span>`type Value = (Idx, Idx)`

- <span id="rangevisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangevisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

- <span id="rangevisitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

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

- <span id="field-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../../index.md)

##### `impl<T> DeserializeOwned for Field`

## Constants

### `FIELDS`

```rust
const FIELDS: &[&str];
```

