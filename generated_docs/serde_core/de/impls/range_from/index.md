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

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2680-2683`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2680-L2683)*

#### Trait Implementations

##### `impl<'de, T> Expected for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, Idx> Visitor for RangeFromVisitor<Idx>`

- <span id="rangefromvisitor-type-value"></span>`type Value = Idx`

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

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2634-2636`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2634-L2636)*

#### Trait Implementations

##### `impl Deserialize for Field`

- <span id="field-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md)

##### `impl DeserializeOwned for Field`

## Constants

### `FIELDS`
```rust
const FIELDS: &[&str];
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2628`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/impls.rs#L2628)*

