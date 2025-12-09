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
  - [`unit_only`](#unit_only)
  - [`map_as_enum`](#map_as_enum)
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
| [`unit_only`](#unit_only) | fn |  |
| [`map_as_enum`](#map_as_enum) | fn |  |
| [`First`](#first) | type |  |
| [`Second`](#second) | type |  |

## Structs

### `UnitOnly<E>`

```rust
struct UnitOnly<E> {
    marker: PhantomData<E>,
}
```

#### Trait Implementations

##### `impl<'de, E> VariantAccess for UnitOnly<E>`

- <span id="unitonly-error"></span>`type Error = E`

- <span id="unitonly-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md)

- <span id="unitonly-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md)

- <span id="unitonly-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

- <span id="unitonly-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

### `MapAsEnum<A>`

```rust
struct MapAsEnum<A> {
    map: A,
}
```

#### Trait Implementations

##### `impl<'de, A> VariantAccess for MapAsEnum<A>`

- <span id="mapasenum-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapasenum-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md)

- <span id="mapasenum-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md)

- <span id="mapasenum-tuple-variant"></span>`fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

- <span id="mapasenum-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

### `SeedTupleVariant<V>`

```rust
struct SeedTupleVariant<V> {
    len: usize,
    visitor: V,
}
```

#### Trait Implementations

##### `impl<'de, V> DeserializeSeed for SeedTupleVariant<V>`

- <span id="seedtuplevariant-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedtuplevariant-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md)

### `SeedStructVariant<V>`

```rust
struct SeedStructVariant<V> {
    visitor: V,
}
```

#### Trait Implementations

##### `impl<'de, V> DeserializeSeed for SeedStructVariant<V>`

- <span id="seedstructvariant-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedstructvariant-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md)

## Traits

### `Pair`

```rust
trait Pair { ... }
```

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

### `map_as_enum`

```rust
fn map_as_enum<A>(map: A) -> MapAsEnum<A>
```

## Type Aliases

### `First<T>`

```rust
type First<T> = <T as Pair>::First;
```

### `Second<T>`

```rust
type Second<T> = <T as Pair>::Second;
```

