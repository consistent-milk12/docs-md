*[serde_core](../../../index.md) / [de](../../index.md) / [impls](../index.md) / [range_from](index.md)*

---

# Module `range_from`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeFromVisitor`](#rangefromvisitor) | struct |  |
| [`Field`](#field) | enum |  |
| [`FIELDS`](#fields) | const |  |

## Structs

### `RangeFromVisitor<Idx>`

```rust
struct RangeFromVisitor<Idx> {
    pub expecting: &'static str,
    pub phantom: PhantomData<Idx>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, Idx> Visitor for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-value"></span>`type Value = Idx`

- <span id="rangefromvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangefromvisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

- <span id="rangefromvisitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md)

## Enums

### `Field`

```rust
enum Field {
    Start,
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

