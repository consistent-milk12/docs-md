*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range_to](index.md)*

---

# Module `range_to`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeToVisitor`](#rangetovisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeToVisitor<Idx>`

```rust
struct RangeToVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for RangeToVisitor<Idx>`

- <span id="rangetovisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, Idx> Visitor for RangeToVisitor<Idx>`

- <span id="rangetovisitor-value"></span>`type Value = Idx`

- <span id="rangetovisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangetovisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

- <span id="rangetovisitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

## Enums

### `Field`

```rust
enum Field {
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

