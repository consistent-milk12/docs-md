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

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2819-2822`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2819-L2822)*

#### Trait Implementations

##### `impl Expected for RangeToVisitor<Idx>`

- <span id="rangetovisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Idx> Visitor for RangeToVisitor<Idx>`

- <span id="rangetovisitor-visitor-type-value"></span>`type Value = Idx`

- <span id="rangetovisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rangetovisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="rangetovisitor-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../../index.md#visitor)

## Enums

### `Field`

```rust
enum Field {
    End,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2773-2775`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2773-L2775)*

#### Trait Implementations

##### `impl Deserialize for Field`

- <span id="field-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md#deserializer)

##### `impl DeserializeOwned for Field`

## Constants

### `FIELDS`
```rust
const FIELDS: &[&str];
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2767`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2767)*

