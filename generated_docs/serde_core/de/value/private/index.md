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

*Defined in [`serde_core-1.0.228/src/de/value.rs:1738-1740`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1738-L1740)*

#### Trait Implementations

##### `impl Any for UnitOnly<E>`

- <span id="unitonly-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitOnly<E>`

- <span id="unitonly-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitOnly<E>`

- <span id="unitonly-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnitOnly<E>`

- <span id="unitonly-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitOnly<E>`

- <span id="unitonly-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for UnitOnly<E>`

- <span id="unitonly-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitonly-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitOnly<E>`

- <span id="unitonly-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitonly-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<E> VariantAccess for UnitOnly<E>`

- <span id="unitonly-variantaccess-type-error"></span>`type Error = E`

- <span id="unitonly-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md#variantaccess)

- <span id="unitonly-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

- <span id="unitonly-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="unitonly-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

### `MapAsEnum<A>`

```rust
struct MapAsEnum<A> {
    map: A,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1796-1798`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1796-L1798)*

#### Trait Implementations

##### `impl Any for MapAsEnum<A>`

- <span id="mapasenum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapAsEnum<A>`

- <span id="mapasenum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapAsEnum<A>`

- <span id="mapasenum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapAsEnum<A>`

- <span id="mapasenum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapAsEnum<A>`

- <span id="mapasenum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MapAsEnum<A>`

- <span id="mapasenum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapasenum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapAsEnum<A>`

- <span id="mapasenum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapasenum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<A> VariantAccess for MapAsEnum<A>`

- <span id="mapasenum-variantaccess-type-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapasenum-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md#variantaccess)

- <span id="mapasenum-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

- <span id="mapasenum-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

- <span id="mapasenum-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md#visitor)

### `SeedTupleVariant<V>`

```rust
struct SeedTupleVariant<V> {
    len: usize,
    visitor: V,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1840-1843`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1840-L1843)*

#### Trait Implementations

##### `impl Any for SeedTupleVariant<V>`

- <span id="seedtuplevariant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeedTupleVariant<V>`

- <span id="seedtuplevariant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeedTupleVariant<V>`

- <span id="seedtuplevariant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V> DeserializeSeed for SeedTupleVariant<V>`

- <span id="seedtuplevariant-deserializeseed-type-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedtuplevariant-deserializeseed-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

##### `impl<T> From for SeedTupleVariant<V>`

- <span id="seedtuplevariant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeedTupleVariant<V>`

- <span id="seedtuplevariant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SeedTupleVariant<V>`

- <span id="seedtuplevariant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seedtuplevariant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeedTupleVariant<V>`

- <span id="seedtuplevariant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seedtuplevariant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SeedStructVariant<V>`

```rust
struct SeedStructVariant<V> {
    visitor: V,
}
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1859-1861`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1859-L1861)*

#### Trait Implementations

##### `impl Any for SeedStructVariant<V>`

- <span id="seedstructvariant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeedStructVariant<V>`

- <span id="seedstructvariant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeedStructVariant<V>`

- <span id="seedstructvariant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V> DeserializeSeed for SeedStructVariant<V>`

- <span id="seedstructvariant-deserializeseed-type-value"></span>`type Value = <V as Visitor>::Value`

- <span id="seedstructvariant-deserializeseed-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md#deserializeseed)

##### `impl<T> From for SeedStructVariant<V>`

- <span id="seedstructvariant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeedStructVariant<V>`

- <span id="seedstructvariant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SeedStructVariant<V>`

- <span id="seedstructvariant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seedstructvariant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeedStructVariant<V>`

- <span id="seedstructvariant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seedstructvariant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Pair`

```rust
trait Pair { ... }
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1879-1883`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1879-L1883)*

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

*Defined in [`serde_core-1.0.228/src/de/value.rs:1742-1749`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1742-L1749)*

### `map_as_enum`

```rust
fn map_as_enum<A>(map: A) -> MapAsEnum<A>
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1800-1802`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1800-L1802)*

## Type Aliases

### `First<T>`

```rust
type First<T> = <T as Pair>::First;
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1893`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1893)*

### `Second<T>`

```rust
type Second<T> = <T as Pair>::Second;
```

*Defined in [`serde_core-1.0.228/src/de/value.rs:1894`](../../../../../.source_1765633015/serde_core-1.0.228/src/de/value.rs#L1894)*

