*[serde_core](../../../index.md) / [de](../../index.md) / [value](../index.md) / [private](index.md)*

---

# Module `private`

## Contents

- [Structs](#structs)
  - [`UnitOnly`](#unitonly)
  - [`MapAsEnum`](#mapasenum)
  - [`SeedTupleVariant`](#seedtuplevariant)
  - [`SeedStructVariant`](#seedstructvariant)
- [Traits](#traits)
  - [`Pair`](#pair)
- [Functions](#functions)
  - [`unit_only`](#unit-only)
  - [`map_as_enum`](#map-as-enum)
- [Type Aliases](#type-aliases)
  - [`First`](#first)
  - [`Second`](#second)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UnitOnly`](#unitonly) | struct |  |
| [`MapAsEnum`](#mapasenum) | struct |  |
| [`SeedTupleVariant`](#seedtuplevariant) | struct |  |
| [`SeedStructVariant`](#seedstructvariant) | struct |  |
| [`Pair`](#pair) | trait | Avoid having to restate the generic types on `MapDeserializer`. |
| [`unit_only`](#unit-only) | fn |  |
| [`map_as_enum`](#map-as-enum) | fn |  |
| [`First`](#first) | type |  |
| [`Second`](#second) | type |  |

## Structs

### `UnitOnly<E>`

```rust
struct UnitOnly<E> {
    marker: PhantomData<E>,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1738-1740`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1738-L1740)*

#### Trait Implementations

##### `impl<E> VariantAccess for UnitOnly<E>`

- <span id="unitonly-variantaccess-type-error"></span>`type Error = E`

- <span id="unitonly-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md#variantaccess)

- <span id="unitonly-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

- <span id="unitonly-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="unitonly-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

### `MapAsEnum<A>`

```rust
struct MapAsEnum<A> {
    map: A,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1796-1798`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1796-L1798)*

#### Trait Implementations

##### `impl<A> VariantAccess for MapAsEnum<A>`

- <span id="mapasenum-variantaccess-type-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapasenum-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md#variantaccess)

- <span id="mapasenum-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

- <span id="mapasenum-tuple-variant"></span>`fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="mapasenum-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

### `SeedTupleVariant<V>`

```rust
struct SeedTupleVariant<V> {
    len: usize,
    visitor: V,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1840-1843`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1840-L1843)*

#### Trait Implementations

##### `impl<V> DeserializeSeed for SeedTupleVariant<V>`

- <span id="seedtuplevariant-deserializeseed-type-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedtuplevariant-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

### `SeedStructVariant<V>`

```rust
struct SeedStructVariant<V> {
    visitor: V,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1859-1861`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1859-L1861)*

#### Trait Implementations

##### `impl<V> DeserializeSeed for SeedStructVariant<V>`

- <span id="seedstructvariant-deserializeseed-type-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedstructvariant-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

## Traits

### `Pair`

```rust
trait Pair { ... }
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1879-1883`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1879-L1883)*

Avoid having to restate the generic types on `MapDeserializer`. The
`Iterator::Item` contains enough information to figure out K and V.

#### Associated Types

- `type First`

- `type Second`

#### Required Methods

- `fn split(self) -> (<Self as >::First, <Self as >::Second)`

#### Implementors

- `(A, B)`

## Functions

### `unit_only`

```rust
fn unit_only<T, E>(t: T) -> (T, UnitOnly<E>)
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1742-1749`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1742-L1749)*

### `map_as_enum`

```rust
fn map_as_enum<A>(map: A) -> MapAsEnum<A>
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1800-1802`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1800-L1802)*

## Type Aliases

### `First<T>`

```rust
type First<T> = <T as Pair>::First;
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1893`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1893)*

### `Second<T>`

```rust
type Second<T> = <T as Pair>::Second;
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1894`](../../../../../.source_1765210505/serde_core-1.0.228/src/de/value.rs#L1894)*

