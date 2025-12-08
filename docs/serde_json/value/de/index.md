*[serde_json](../../index.md) / [value](../index.md) / [de](index.md)*

---

# Module `de`

## Structs

### `EnumDeserializer`

```rust
struct EnumDeserializer {
    variant: alloc::string::String,
    value: Option<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> EnumAccess for EnumDeserializer`

- `type Error = Error`

- `type Variant = VariantDeserializer`

- `fn variant_seed<V>(self: Self, seed: V) -> Result<(<V as >::Value, VariantDeserializer), Error>` — [`VariantDeserializer`](#variantdeserializer), [`Error`](../../error/index.md)

### `VariantDeserializer`

```rust
struct VariantDeserializer {
    value: Option<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> VariantAccess for VariantDeserializer`

- `type Error = Error`

- `fn unit_variant(self: Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn newtype_variant_seed<T>(self: Self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn tuple_variant<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn struct_variant<V>(self: Self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

### `SeqDeserializer`

```rust
struct SeqDeserializer {
    iter: vec::IntoIter<crate::value::Value>,
}
```

#### Implementations

- `fn new(vec: Vec<Value>) -> Self` — [`Value`](../index.md)

#### Trait Implementations

##### `impl<'de> SeqAccess for SeqDeserializer`

- `type Error = Error`

- `fn next_element_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

### `MapDeserializer`

```rust
struct MapDeserializer {
    iter: <crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<crate::value::Value>,
}
```

#### Implementations

- `fn new(map: Map<String, Value>) -> Self` — [`Map`](../../map/index.md), [`Value`](../index.md)

#### Trait Implementations

##### `impl<'de> MapAccess for MapDeserializer`

- `type Error = Error`

- `fn next_key_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md)

- `fn next_value_seed<T>(self: &mut Self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

### `EnumRefDeserializer<'de>`

```rust
struct EnumRefDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> EnumAccess for EnumRefDeserializer<'de>`

- `type Error = Error`

- `type Variant = VariantRefDeserializer<'de>`

- `fn variant_seed<V>(self: Self, seed: V) -> Result<(<V as >::Value, <Self as >::Variant), Error>` — [`Error`](../../error/index.md)

### `VariantRefDeserializer<'de>`

```rust
struct VariantRefDeserializer<'de> {
    value: Option<&'de crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> VariantAccess for VariantRefDeserializer<'de>`

- `type Error = Error`

- `fn unit_variant(self: Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn newtype_variant_seed<T>(self: Self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn tuple_variant<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn struct_variant<V>(self: Self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

### `SeqRefDeserializer<'de>`

```rust
struct SeqRefDeserializer<'de> {
    iter: slice::Iter<'de, crate::value::Value>,
}
```

#### Implementations

- `fn new(slice: &'de [Value]) -> Self` — [`Value`](../index.md)

#### Trait Implementations

##### `impl<'de> SeqAccess for SeqRefDeserializer<'de>`

- `type Error = Error`

- `fn next_element_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

### `MapRefDeserializer<'de>`

```rust
struct MapRefDeserializer<'de> {
    iter: <&'de crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<&'de crate::value::Value>,
}
```

#### Implementations

- `fn new(map: &'de Map<String, Value>) -> Self` — [`Map`](../../map/index.md), [`Value`](../index.md)

#### Trait Implementations

##### `impl<'de> MapAccess for MapRefDeserializer<'de>`

- `type Error = Error`

- `fn next_key_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md)

- `fn next_value_seed<T>(self: &mut Self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

### `MapKeyDeserializer<'de>`

```rust
struct MapKeyDeserializer<'de> {
    key: alloc::borrow::Cow<'de, str>,
}
```

#### Trait Implementations

##### `impl<'de> Deserializer for MapKeyDeserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_newtype_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

### `KeyClassifier`

```rust
struct KeyClassifier;
```

#### Trait Implementations

##### `impl<'de> DeserializeSeed for KeyClassifier`

- `type Value = KeyClass`

- `fn deserialize<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

##### `impl<'de, T> Expected for KeyClassifier`

- `fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl<'de> Visitor for KeyClassifier`

- `type Value = KeyClass`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_str<E>(self: Self, s: &str) -> Result<<Self as >::Value, E>`

- `fn visit_string<E>(self: Self, s: String) -> Result<<Self as >::Value, E>`

### `BorrowedCowStrDeserializer<'de>`

```rust
struct BorrowedCowStrDeserializer<'de> {
    value: alloc::borrow::Cow<'de, str>,
}
```

#### Implementations

- `fn new(value: Cow<'de, str>) -> Self`

#### Trait Implementations

##### `impl<'de> Deserializer for BorrowedCowStrDeserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_enum<V>(self: Self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

##### `impl<'de> EnumAccess for BorrowedCowStrDeserializer<'de>`

- `type Error = Error`

- `type Variant = UnitOnly`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), Error>` — [`Error`](../../error/index.md)

### `UnitOnly`

```rust
struct UnitOnly;
```

#### Trait Implementations

##### `impl<'de> VariantAccess for UnitOnly`

- `type Error = Error`

- `fn unit_variant(self: Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn newtype_variant_seed<T>(self: Self, _seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn tuple_variant<V>(self: Self, _len: usize, _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn struct_variant<V>(self: Self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

## Enums

### `KeyClass`

```rust
enum KeyClass {
    Map(alloc::string::String),
}
```

## Functions

### `visit_array`

```rust
fn visit_array<'de, V>(array: alloc::vec::Vec<crate::value::Value>, visitor: V) -> Result<<V as >::Value, crate::error::Error>
where
    V: Visitor<'de>
```

### `visit_array_ref`

```rust
fn visit_array_ref<'de, V>(array: &'de [crate::value::Value], visitor: V) -> Result<<V as >::Value, crate::error::Error>
where
    V: Visitor<'de>
```

## Macros

### `deserialize_number!`

### `deserialize_value_ref_number!`

### `deserialize_numeric_key!`

