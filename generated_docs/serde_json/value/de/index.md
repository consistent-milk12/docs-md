*[serde_json](../../index.md) / [value](../index.md) / [de](index.md)*

---

# Module `de`

## Contents

- [Structs](#structs)
  - [`EnumDeserializer`](#enumdeserializer)
  - [`VariantDeserializer`](#variantdeserializer)
  - [`SeqDeserializer`](#seqdeserializer)
  - [`MapDeserializer`](#mapdeserializer)
  - [`EnumRefDeserializer`](#enumrefdeserializer)
  - [`VariantRefDeserializer`](#variantrefdeserializer)
  - [`SeqRefDeserializer`](#seqrefdeserializer)
  - [`MapRefDeserializer`](#maprefdeserializer)
  - [`MapKeyDeserializer`](#mapkeydeserializer)
  - [`KeyClassifier`](#keyclassifier)
  - [`BorrowedCowStrDeserializer`](#borrowedcowstrdeserializer)
  - [`UnitOnly`](#unitonly)
- [Enums](#enums)
  - [`KeyClass`](#keyclass)
- [Functions](#functions)
  - [`visit_array`](#visit_array)
  - [`visit_array_ref`](#visit_array_ref)
- [Macros](#macros)
  - [`deserialize_number!`](#deserialize_number)
  - [`deserialize_value_ref_number!`](#deserialize_value_ref_number)
  - [`deserialize_numeric_key!`](#deserialize_numeric_key)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EnumDeserializer`](#enumdeserializer) | struct |  |
| [`VariantDeserializer`](#variantdeserializer) | struct |  |
| [`SeqDeserializer`](#seqdeserializer) | struct |  |
| [`MapDeserializer`](#mapdeserializer) | struct |  |
| [`EnumRefDeserializer`](#enumrefdeserializer) | struct |  |
| [`VariantRefDeserializer`](#variantrefdeserializer) | struct |  |
| [`SeqRefDeserializer`](#seqrefdeserializer) | struct |  |
| [`MapRefDeserializer`](#maprefdeserializer) | struct |  |
| [`MapKeyDeserializer`](#mapkeydeserializer) | struct |  |
| [`KeyClassifier`](#keyclassifier) | struct |  |
| [`BorrowedCowStrDeserializer`](#borrowedcowstrdeserializer) | struct |  |
| [`UnitOnly`](#unitonly) | struct |  |
| [`KeyClass`](#keyclass) | enum |  |
| [`visit_array`](#visit_array) | fn |  |
| [`visit_array_ref`](#visit_array_ref) | fn |  |
| [`deserialize_number!`](#deserialize_number) | macro |  |
| [`deserialize_value_ref_number!`](#deserialize_value_ref_number) | macro |  |
| [`deserialize_numeric_key!`](#deserialize_numeric_key) | macro |  |

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

- <span id="enumdeserializer-error"></span>`type Error = Error`

- <span id="enumdeserializer-variant"></span>`type Variant = VariantDeserializer`

- <span id="enumdeserializer-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, VariantDeserializer), Error>` — [`VariantDeserializer`](#variantdeserializer), [`Error`](../../index.md)

### `VariantDeserializer`

```rust
struct VariantDeserializer {
    value: Option<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> VariantAccess for VariantDeserializer`

- <span id="variantdeserializer-error"></span>`type Error = Error`

- <span id="variantdeserializer-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../index.md)

- <span id="variantdeserializer-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../index.md)

- <span id="variantdeserializer-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="variantdeserializer-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

### `SeqDeserializer`

```rust
struct SeqDeserializer {
    iter: vec::IntoIter<crate::value::Value>,
}
```

#### Implementations

- <span id="seqdeserializer-new"></span>`fn new(vec: Vec<Value>) -> Self` — [`Value`](../../index.md)

#### Trait Implementations

##### `impl<'de> SeqAccess for SeqDeserializer`

- <span id="seqdeserializer-error"></span>`type Error = Error`

- <span id="seqdeserializer-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../index.md)

- <span id="seqdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapDeserializer`

```rust
struct MapDeserializer {
    iter: <crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<crate::value::Value>,
}
```

#### Implementations

- <span id="mapdeserializer-new"></span>`fn new(map: Map<String, Value>) -> Self` — [`Map`](../../index.md), [`Value`](../../index.md)

#### Trait Implementations

##### `impl<'de> MapAccess for MapDeserializer`

- <span id="mapdeserializer-error"></span>`type Error = Error`

- <span id="mapdeserializer-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../index.md)

- <span id="mapdeserializer-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `EnumRefDeserializer<'de>`

```rust
struct EnumRefDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> EnumAccess for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-error"></span>`type Error = Error`

- <span id="enumrefdeserializer-variant"></span>`type Variant = VariantRefDeserializer<'de>`

- <span id="enumrefdeserializer-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, <Self as >::Variant), Error>` — [`Error`](../../index.md)

### `VariantRefDeserializer<'de>`

```rust
struct VariantRefDeserializer<'de> {
    value: Option<&'de crate::value::Value>,
}
```

#### Trait Implementations

##### `impl<'de> VariantAccess for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-error"></span>`type Error = Error`

- <span id="variantrefdeserializer-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../index.md)

- <span id="variantrefdeserializer-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../index.md)

- <span id="variantrefdeserializer-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="variantrefdeserializer-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

### `SeqRefDeserializer<'de>`

```rust
struct SeqRefDeserializer<'de> {
    iter: slice::Iter<'de, crate::value::Value>,
}
```

#### Implementations

- <span id="seqrefdeserializer-new"></span>`fn new(slice: &'de [Value]) -> Self` — [`Value`](../../index.md)

#### Trait Implementations

##### `impl<'de> SeqAccess for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-error"></span>`type Error = Error`

- <span id="seqrefdeserializer-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../index.md)

- <span id="seqrefdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapRefDeserializer<'de>`

```rust
struct MapRefDeserializer<'de> {
    iter: <&'de crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<&'de crate::value::Value>,
}
```

#### Implementations

- <span id="maprefdeserializer-new"></span>`fn new(map: &'de Map<String, Value>) -> Self` — [`Map`](../../index.md), [`Value`](../../index.md)

#### Trait Implementations

##### `impl<'de> MapAccess for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-error"></span>`type Error = Error`

- <span id="maprefdeserializer-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../index.md)

- <span id="maprefdeserializer-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../index.md)

- <span id="maprefdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `MapKeyDeserializer<'de>`

```rust
struct MapKeyDeserializer<'de> {
    key: alloc::borrow::Cow<'de, str>,
}
```

#### Trait Implementations

##### `impl<'de> Deserializer for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-error"></span>`type Error = Error`

- <span id="mapkeydeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="mapkeydeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

### `KeyClassifier`

```rust
struct KeyClassifier;
```

#### Trait Implementations

##### `impl<'de> DeserializeSeed for KeyClassifier`

- <span id="keyclassifier-value"></span>`type Value = KeyClass`

- <span id="keyclassifier-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

##### `impl<'de, T> Expected for KeyClassifier`

- <span id="keyclassifier-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl<'de> Visitor for KeyClassifier`

- <span id="keyclassifier-value"></span>`type Value = KeyClass`

- <span id="keyclassifier-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="keyclassifier-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>`

- <span id="keyclassifier-visit-string"></span>`fn visit_string<E>(self, s: String) -> Result<<Self as >::Value, E>`

### `BorrowedCowStrDeserializer<'de>`

```rust
struct BorrowedCowStrDeserializer<'de> {
    value: alloc::borrow::Cow<'de, str>,
}
```

#### Implementations

- <span id="borrowedcowstrdeserializer-new"></span>`fn new(value: Cow<'de, str>) -> Self`

#### Trait Implementations

##### `impl<'de> Deserializer for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-error"></span>`type Error = Error`

- <span id="borrowedcowstrdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="borrowedcowstrdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="borrowedcowstrdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl<'de> EnumAccess for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-error"></span>`type Error = Error`

- <span id="borrowedcowstrdeserializer-variant"></span>`type Variant = UnitOnly`

- <span id="borrowedcowstrdeserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), Error>` — [`Error`](../../index.md)

### `UnitOnly`

```rust
struct UnitOnly;
```

#### Trait Implementations

##### `impl<'de> VariantAccess for UnitOnly`

- <span id="unitonly-error"></span>`type Error = Error`

- <span id="unitonly-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../index.md)

- <span id="unitonly-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../index.md)

- <span id="unitonly-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

- <span id="unitonly-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../index.md)

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

