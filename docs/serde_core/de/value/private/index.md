*[serde_core](../../../index.md) / [de](../../index.md) / [value](../index.md) / [private](index.md)*

---

# Module `private`

## Structs

### `UnitOnly<E>`

```rust
struct UnitOnly<E> {
    marker: PhantomData<E>,
}
```

#### Trait Implementations

##### `impl<'de, E> VariantAccess for UnitOnly<E>`

- `type Error = E`

- `fn unit_variant(self: Self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md)

- `fn newtype_variant_seed<T>(self: Self, _seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md)

- `fn tuple_variant<V>(self: Self, _len: usize, _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

- `fn struct_variant<V>(self: Self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

### `MapAsEnum<A>`

```rust
struct MapAsEnum<A> {
    map: A,
}
```

#### Trait Implementations

##### `impl<'de, A> VariantAccess for MapAsEnum<A>`

- `type Error = <A as MapAccess>::Error`

- `fn unit_variant(self: Self) -> Result<(), <Self as >::Error>` — [`VariantAccess`](../../index.md)

- `fn newtype_variant_seed<T>(self: Self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../../index.md)

- `fn tuple_variant<V>(self: Self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

- `fn struct_variant<V>(self: Self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../../index.md)

### `SeedTupleVariant<V>`

```rust
struct SeedTupleVariant<V> {
    len: usize,
    visitor: V,
}
```

#### Trait Implementations

##### `impl<'de, V> DeserializeSeed for SeedTupleVariant<V>`

- `type Value = <V as Visitor>::Value`

- `fn deserialize<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md)

### `SeedStructVariant<V>`

```rust
struct SeedStructVariant<V> {
    visitor: V,
}
```

#### Trait Implementations

##### `impl<'de, V> DeserializeSeed for SeedStructVariant<V>`

- `type Value = <V as Visitor>::Value`

- `fn deserialize<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`DeserializeSeed`](../../index.md)

## Traits

### `Pair`

```rust
trait Pair { ... }
```

Avoid having to restate the generic types on `MapDeserializer`. The
`Iterator::Item` contains enough information to figure out K and V.

#### Required Methods

- `type First`

- `type Second`

- `fn split(self: Self) -> (<Self as >::First, <Self as >::Second)`

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

