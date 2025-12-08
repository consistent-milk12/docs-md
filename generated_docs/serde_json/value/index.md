*[serde_json](../index.md) / [value](index.md)*

---

# Module `value`

The Value enum, a loosely typed way of representing any valid JSON value.

# Constructing JSON

Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
objects with very natural JSON syntax.

```rust
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```rust
use serde_json::json;

fn random_phone() -> u16 { 0 }

let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][`from_str`](../index.md) function. There is also
[`from_slice`][`from_slice`](../index.md) for parsing from a byte slice `&[u8]` and
[`from_reader`][`from_reader`](../index.md) for parsing from any `io::Read` like a File or
a TCP stream.

```rust
use serde_json::{json, Value, Error};

fn untyped_example() -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

untyped_example().unwrap();
```





## Contents

- [Modules](#modules)
  - [`de`](#de)
  - [`from`](#from)
  - [`index`](#index)
  - [`partial_eq`](#partial_eq)
  - [`ser`](#ser)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Enums](#enums)
  - [`Value`](#value)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`parse_index`](#parse_index)
  - [`to_value`](#to_value)
  - [`from_value`](#from_value)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`de`](#de) | mod |  |
| [`from`](#from) | mod |  |
| [`index`](#index) | mod |  |
| [`partial_eq`](#partial_eq) | mod |  |
| [`ser`](#ser) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`Value`](#value) | enum | Represents any valid JSON value. |
| [`unnamed`](#unnamed) | trait |  |
| [`parse_index`](#parse_index) | fn |  |
| [`to_value`](#to_value) | fn | Convert a `T` into `serde_json::Value` which is an enum that can represent |
| [`from_value`](#from_value) | fn | Interpret a `serde_json::Value` as an instance of type `T`. |

## Modules

- [`de`](de/index.md) - 
- [`from`](from/index.md) - 
- [`index`](index/index.md) - 
- [`partial_eq`](partial_eq/index.md) - 
- [`ser`](ser/index.md) - 

## Structs

### `Serializer`

```rust
struct Serializer;
```

Serializer whose output is a `Value`.

This is the serializer that backs `serde_json::to_value`.
Unlike the main serde_json serializer which goes from some serializable
value of type `T` to JSON text, this one goes from `T` to
`serde_json::Value`.

The `to_value` function is implementable as:

```rust
use serde::Serialize;
use serde_json::{Error, Value};

pub fn to_value<T>(input: T) -> Result<Value, Error>
where
    T: Serialize,
{
    input.serialize(serde_json::value::Serializer)
}
```

#### Trait Implementations

##### `impl Serializer for Serializer`

- <span id="serializer-ok"></span>`type Ok = Value`

- <span id="serializer-error"></span>`type Error = Error`

- <span id="serializer-serializeseq"></span>`type SerializeSeq = SerializeVec`

- <span id="serializer-serializetuple"></span>`type SerializeTuple = SerializeVec`

- <span id="serializer-serializetuplestruct"></span>`type SerializeTupleStruct = SerializeVec`

- <span id="serializer-serializetuplevariant"></span>`type SerializeTupleVariant = SerializeTupleVariant`

- <span id="serializer-serializemap"></span>`type SerializeMap = SerializeMap`

- <span id="serializer-serializestruct"></span>`type SerializeStruct = SerializeMap`

- <span id="serializer-serializestructvariant"></span>`type SerializeStructVariant = SerializeStructVariant`

- <span id="serializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-f32"></span>`fn serialize_f32(self, float: f32) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-f64"></span>`fn serialize_f64(self, float: f64) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-bytes"></span>`fn serialize_bytes(self, value: &[u8]) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-none"></span>`fn serialize_none(self) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- <span id="serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../index.md)

- <span id="serializer-serialize-tuple"></span>`fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../index.md)

- <span id="serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../index.md)

- <span id="serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../index.md)

- <span id="serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../index.md)

- <span id="serializer-serialize-struct"></span>`fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../index.md)

- <span id="serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../index.md)

- <span id="serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="map-clear"></span>`fn clear(&mut self)`

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](../index.md)

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](../index.md)

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../index.md)

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](../index.md)

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](../index.md)

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](../index.md)

- <span id="map-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](../map/index.md)

- <span id="map-len"></span>`fn len(&self) -> usize`

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](../map/index.md)

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](../map/index.md)

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](../map/index.md)

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](../map/index.md)

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](../map/index.md)

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](../map/index.md)

- <span id="map-retain"></span>`fn retain<F>(&mut self, f: F)`

- <span id="map-sort-keys"></span>`fn sort_keys(&mut self)`

#### Trait Implementations

##### `impl Clone for Map<alloc::string::String, crate::value::Value>`

- <span id="map-clone"></span>`fn clone(&self) -> Self`

- <span id="map-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl Debug for Map<alloc::string::String, crate::value::Value>`

- <span id="map-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Map<alloc::string::String, crate::value::Value>`

- <span id="map-default"></span>`fn default() -> Self`

##### `impl<'de> Deserialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for Map<K, V>`

##### `impl<'de> Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-error"></span>`type Error = Error`

- <span id="cratemapmap-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Eq for Map<alloc::string::String, crate::value::Value>`

##### `impl Extend for Map<alloc::string::String, crate::value::Value>`

- <span id="map-extend"></span>`fn extend<T>(&mut self, iter: T)`

##### `impl FromIterator for Map<alloc::string::String, crate::value::Value>`

- <span id="map-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-err"></span>`type Err = Error`

- <span id="cratemapmap-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../index.md)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](../index.md)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](../index.md)

##### `impl<'de> IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl IntoIterator for Map<alloc::string::String, crate::value::Value>`

- <span id="map-item"></span>`type Item = (String, Value)`

- <span id="map-intoiter"></span>`type IntoIter = IntoIter`

- <span id="map-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Map<alloc::string::String, crate::value::Value>`

- <span id="map-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Serialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `Number`

```rust
struct Number {
    n: N,
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- <span id="number-unexpected"></span>`fn unexpected(&self) -> Unexpected<'_>`

#### Trait Implementations

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](../index.md)

##### `impl Debug for Number`

- <span id="number-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](../index.md)

##### `impl<T> DeserializeOwned for Number`

##### `impl<'de> Deserializer for Number`

- <span id="number-error"></span>`type Error = Error`

- <span id="number-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="number-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Display for Number`

- <span id="number-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Number`

##### `impl FromStr for crate::number::Number`

- <span id="cratenumbernumber-err"></span>`type Err = Error`

- <span id="cratenumbernumber-from-str"></span>`fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- <span id="number-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Number`

- <span id="number-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](../index.md)

##### `impl Serialize for Number`

- <span id="number-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl<T> ToString for Number`

- <span id="number-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Value`

```rust
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(alloc::string::String),
    Array(alloc::vec::Vec<Value>),
    Object(Map<alloc::string::String, Value>),
}
```

Represents any valid JSON value.

See the [`serde_json::value` module documentation](self) for usage examples.

#### Variants

- **`Null`**

  Represents a JSON null value.
  
  ```rust
  use serde_json::json;
  
  let v = json!(null);
  ```

- **`Bool`**

  Represents a JSON boolean.
  
  ```rust
  use serde_json::json;
  
  let v = json!(true);
  ```

- **`Number`**

  Represents a JSON number, whether integer or floating point.
  
  ```rust
  use serde_json::json;
  
  let v = json!(12.5);
  ```

- **`String`**

  Represents a JSON string.
  
  ```rust
  use serde_json::json;
  
  let v = json!("a string");
  ```

- **`Array`**

  Represents a JSON array.
  
  ```rust
  use serde_json::json;
  
  let v = json!(["an", "array"]);
  ```

- **`Object`**

  Represents a JSON object.
  
  By default the map is backed by a BTreeMap. Enable the `preserve_order`
  feature of serde_json to use IndexMap instead, which preserves
  entries in the order they are inserted into the map. In particular, this
  allows JSON data to be deserialized into a Value and serialized to a
  string while retaining the order of map keys in the input.
  
  ```rust
  use serde_json::json;
  
  let v = json!({ "an": "object" });
  ```

#### Implementations

- <span id="value-get"></span>`fn get<I: Index>(&self, index: I) -> Option<&Value>` — [`Value`](../index.md)

- <span id="value-get-mut"></span>`fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value>` — [`Value`](../index.md)

- <span id="value-is-object"></span>`fn is_object(&self) -> bool`

- <span id="value-as-object"></span>`fn as_object(&self) -> Option<&Map<String, Value>>` — [`Map`](../index.md), [`Value`](../index.md)

- <span id="value-as-object-mut"></span>`fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>>` — [`Map`](../index.md), [`Value`](../index.md)

- <span id="value-is-array"></span>`fn is_array(&self) -> bool`

- <span id="value-as-array"></span>`fn as_array(&self) -> Option<&Vec<Value>>` — [`Value`](../index.md)

- <span id="value-as-array-mut"></span>`fn as_array_mut(&mut self) -> Option<&mut Vec<Value>>` — [`Value`](../index.md)

- <span id="value-is-string"></span>`fn is_string(&self) -> bool`

- <span id="value-as-str"></span>`fn as_str(&self) -> Option<&str>`

- <span id="value-is-number"></span>`fn is_number(&self) -> bool`

- <span id="value-as-number"></span>`fn as_number(&self) -> Option<&Number>` — [`Number`](../index.md)

- <span id="value-is-i64"></span>`fn is_i64(&self) -> bool`

- <span id="value-is-u64"></span>`fn is_u64(&self) -> bool`

- <span id="value-is-f64"></span>`fn is_f64(&self) -> bool`

- <span id="value-as-i64"></span>`fn as_i64(&self) -> Option<i64>`

- <span id="value-as-u64"></span>`fn as_u64(&self) -> Option<u64>`

- <span id="value-as-f64"></span>`fn as_f64(&self) -> Option<f64>`

- <span id="value-is-boolean"></span>`fn is_boolean(&self) -> bool`

- <span id="value-as-bool"></span>`fn as_bool(&self) -> Option<bool>`

- <span id="value-is-null"></span>`fn is_null(&self) -> bool`

- <span id="value-as-null"></span>`fn as_null(&self) -> Option<()>`

- <span id="value-pointer"></span>`fn pointer(&self, pointer: &str) -> Option<&Value>` — [`Value`](../index.md)

- <span id="value-pointer-mut"></span>`fn pointer_mut(&mut self, pointer: &str) -> Option<&mut Value>` — [`Value`](../index.md)

- <span id="value-take"></span>`fn take(&mut self) -> Value` — [`Value`](../index.md)

- <span id="value-sort-all-objects"></span>`fn sort_all_objects(&mut self)`

#### Trait Implementations

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](../index.md)

##### `impl Debug for Value`

- <span id="value-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Value`

- <span id="value-default"></span>`fn default() -> Value` — [`Value`](../index.md)

##### `impl<'de> Deserialize for crate::value::Value`

- <span id="cratevaluevalue-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](../index.md)

##### `impl<T> DeserializeOwned for Value`

##### `impl<'de> Deserializer for crate::value::Value`

- <span id="cratevaluevalue-error"></span>`type Error = Error`

- <span id="cratevaluevalue-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- <span id="cratevaluevalue-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

##### `impl Display for Value`

- <span id="value-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Value`

##### `impl<T: Into<super::Value>> FromIterator for super::Value`

- <span id="supervalue-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl FromStr for crate::value::Value`

- <span id="cratevaluevalue-err"></span>`type Err = Error`

- <span id="cratevaluevalue-from-str"></span>`fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](../index.md), [`Error`](../index.md)

##### `impl Hash for Value`

- <span id="value-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> Index for super::Value`

- <span id="supervalue-output"></span>`type Output = Value`

- <span id="supervalue-index"></span>`fn index(&self, index: I) -> &Value` — [`Value`](../index.md)

##### `impl<I> IndexMut for super::Value`

- <span id="supervalue-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut Value` — [`Value`](../index.md)

##### `impl<'de> IntoDeserializer for crate::value::Value`

- <span id="cratevaluevalue-deserializer"></span>`type Deserializer = Value`

- <span id="cratevaluevalue-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl PartialEq for Value`

- <span id="value-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](../index.md)

##### `impl Serialize for crate::value::Value`

- <span id="cratevaluevalue-serialize"></span>`fn serialize<S>(&self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Value`

##### `impl<T> ToString for Value`

- <span id="value-to-string"></span>`fn to_string(&self) -> String`

## Traits

## Functions

### `parse_index`

```rust
fn parse_index(s: &str) -> Option<usize>
```

### `to_value`

```rust
fn to_value<T>(value: T) -> Result<Value, crate::error::Error>
where
    T: Serialize
```

Convert a `T` into `serde_json::Value` which is an enum that can represent
any valid JSON data.

# Example

```rust
use serde::Serialize;
use serde_json::json;
use std::error::Error;

#[derive(Serialize)]
struct User {
    fingerprint: String,
    location: String,
}

fn compare_json_values() -> Result<(), Box<dyn Error>> {
    let u = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    // The type of `expected` is `serde_json::Value`
    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let v = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);

    Ok(())
}

compare_json_values().unwrap();
```

# Errors

This conversion can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
use std::collections::BTreeMap;

fn main() {
    // The keys in this map are vectors, not strings.
    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    println!("{}", serde_json::to_value(map).unwrap_err());
}
```

### `from_value`

```rust
fn from_value<T>(value: Value) -> Result<T, crate::error::Error>
where
    T: DeserializeOwned
```

Interpret a `serde_json::Value` as an instance of type `T`.

# Example

```rust
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `serde_json::Value`
    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let u: User = serde_json::from_value(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the Value does not match the
structure expected by `T`, for example if `T` is a struct type but the Value
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

