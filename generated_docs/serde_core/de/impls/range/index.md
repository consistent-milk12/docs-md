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

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2524-2527`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2524-L2527)*

#### Trait Implementations

##### `impl Expected for RangeVisitor<Idx>`

- <span id="rangevisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Idx> Visitor for RangeVisitor<Idx>`

- <span id="rangevisitor-visitor-type-value"></span>`type Value = (Idx, Idx)`

- <span id="rangevisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangevisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangevisitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    Start,
    End,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2475-2478`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2475-L2478)*

#### Trait Implementations

##### `impl Deserialize for Field`

- <span id="field-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md#deserializer)

##### `impl DeserializeOwned for Field`

## Constants

### `FIELDS`
```rust
const FIELDS: &[&str];
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2469`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2469)*

