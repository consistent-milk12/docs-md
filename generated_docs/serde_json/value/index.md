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

- `type Ok = Value`

- `type Error = Error`

- `type SerializeSeq = SerializeVec`

- `type SerializeTuple = SerializeVec`

- `type SerializeTupleStruct = SerializeVec`

- `type SerializeTupleVariant = SerializeTupleVariant`

- `type SerializeMap = SerializeMap`

- `type SerializeStruct = SerializeMap`

- `type SerializeStructVariant = SerializeStructVariant`

- `fn serialize_bool(self: Self, value: bool) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_i8(self: Self, value: i8) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_i16(self: Self, value: i16) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_i32(self: Self, value: i32) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_i64(self: Self, value: i64) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_i128(self: Self, value: i128) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_u8(self: Self, value: u8) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_u16(self: Self, value: u16) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_u32(self: Self, value: u32) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_u64(self: Self, value: u64) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_u128(self: Self, value: u128) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_f32(self: Self, float: f32) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_f64(self: Self, float: f64) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_char(self: Self, value: char) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_str(self: Self, value: &str) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_bytes(self: Self, value: &[u8]) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_unit(self: Self) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_none(self: Self) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_some<T>(self: Self, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

- `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../index.md)

- `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../index.md)

- `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../index.md)

- `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../index.md)

- `fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../index.md)

- `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../index.md)

- `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../index.md)

- `fn collect_str<T>(self: Self, value: &T) -> Result<Value>` — [`Result`](../index.md), [`Value`](../index.md)

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

Represents a JSON key/value type.

#### Implementations

- `fn new() -> Self`

- `fn with_capacity(capacity: usize) -> Self`

- `fn clear(self: &mut Self)`

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>` — [`Value`](../index.md)

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>` — [`Value`](../index.md)

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../index.md)

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>` — [`Value`](../index.md)

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>` — [`Value`](../index.md)

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>` — [`Value`](../index.md)

- `fn append(self: &mut Self, other: &mut Self)`

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>` — [`Entry`](../map/index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](../map/index.md)

- `fn iter_mut(self: &mut Self) -> IterMut<'_>` — [`IterMut`](../map/index.md)

- `fn keys(self: &Self) -> Keys<'_>` — [`Keys`](../map/index.md)

- `fn values(self: &Self) -> Values<'_>` — [`Values`](../map/index.md)

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>` — [`ValuesMut`](../map/index.md)

- `fn into_values(self: Self) -> IntoValues` — [`IntoValues`](../map/index.md)

- `fn retain<F>(self: &mut Self, f: F)`

- `fn sort_keys(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for Map<alloc::string::String, crate::value::Value>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Debug for Map<alloc::string::String, crate::value::Value>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Map<alloc::string::String, crate::value::Value>`

- `fn default() -> Self`

##### `impl<'de> Deserialize for Map<alloc::string::String, crate::value::Value>`

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for Map<K, V>`

##### `impl<'de> Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- `fn deserialize_enum<V>(self: Self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

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

##### `impl Eq for Map<alloc::string::String, crate::value::Value>`

##### `impl Extend for Map<alloc::string::String, crate::value::Value>`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl FromIterator for Map<alloc::string::String, crate::value::Value>`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr for crate::map::Map<alloc::string::String, crate::value::Value>`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../index.md)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value` — [`Value`](../index.md)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value` — [`Value`](../index.md)

##### `impl<'de> IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- `type Deserializer = Map<String, Value>`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl IntoIterator for Map<alloc::string::String, crate::value::Value>`

- `type Item = (String, Value)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl PartialEq for Map<alloc::string::String, crate::value::Value>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Serialize for Map<alloc::string::String, crate::value::Value>`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `Number`

```rust
struct Number {
    n: N,
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- `fn is_i64(self: &Self) -> bool`

- `fn is_u64(self: &Self) -> bool`

- `fn is_f64(self: &Self) -> bool`

- `fn as_i64(self: &Self) -> Option<i64>`

- `fn as_u64(self: &Self) -> Option<u64>`

- `fn as_f64(self: &Self) -> Option<f64>`

- `fn from_f64(f: f64) -> Option<Number>` — [`Number`](../index.md)

- `fn as_i128(self: &Self) -> Option<i128>`

- `fn as_u128(self: &Self) -> Option<u128>`

- `fn from_i128(i: i128) -> Option<Number>` — [`Number`](../index.md)

- `fn from_u128(i: u128) -> Option<Number>` — [`Number`](../index.md)

- `fn as_f32(self: &Self) -> Option<f32>`

- `fn from_f32(f: f32) -> Option<Number>` — [`Number`](../index.md)

#### Trait Implementations

##### `impl Clone for Number`

- `fn clone(self: &Self) -> Number` — [`Number`](../index.md)

##### `impl Debug for Number`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Deserialize for Number`

- `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](../index.md)

##### `impl<T> DeserializeOwned for Number`

##### `impl<'de> Deserializer for Number`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

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

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

##### `impl Display for Number`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Number`

##### `impl FromStr for crate::number::Number`

- `type Err = Error`

- `fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Number`

- `fn eq(self: &Self, other: &Number) -> bool` — [`Number`](../index.md)

##### `impl Serialize for Number`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl<T> ToString for Number`

- `fn to_string(self: &Self) -> String`

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

- `fn get<I: Index>(self: &Self, index: I) -> Option<&Value>` — [`Value`](../index.md)

- `fn get_mut<I: Index>(self: &mut Self, index: I) -> Option<&mut Value>` — [`Value`](../index.md)

- `fn is_object(self: &Self) -> bool`

- `fn as_object(self: &Self) -> Option<&Map<String, Value>>` — [`Map`](../index.md), [`Value`](../index.md)

- `fn as_object_mut(self: &mut Self) -> Option<&mut Map<String, Value>>` — [`Map`](../index.md), [`Value`](../index.md)

- `fn is_array(self: &Self) -> bool`

- `fn as_array(self: &Self) -> Option<&Vec<Value>>` — [`Value`](../index.md)

- `fn as_array_mut(self: &mut Self) -> Option<&mut Vec<Value>>` — [`Value`](../index.md)

- `fn is_string(self: &Self) -> bool`

- `fn as_str(self: &Self) -> Option<&str>`

- `fn is_number(self: &Self) -> bool`

- `fn as_number(self: &Self) -> Option<&Number>` — [`Number`](../index.md)

- `fn is_i64(self: &Self) -> bool`

- `fn is_u64(self: &Self) -> bool`

- `fn is_f64(self: &Self) -> bool`

- `fn as_i64(self: &Self) -> Option<i64>`

- `fn as_u64(self: &Self) -> Option<u64>`

- `fn as_f64(self: &Self) -> Option<f64>`

- `fn is_boolean(self: &Self) -> bool`

- `fn as_bool(self: &Self) -> Option<bool>`

- `fn is_null(self: &Self) -> bool`

- `fn as_null(self: &Self) -> Option<()>`

- `fn pointer(self: &Self, pointer: &str) -> Option<&Value>` — [`Value`](../index.md)

- `fn pointer_mut(self: &mut Self, pointer: &str) -> Option<&mut Value>` — [`Value`](../index.md)

- `fn take(self: &mut Self) -> Value` — [`Value`](../index.md)

- `fn sort_all_objects(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for Value`

- `fn clone(self: &Self) -> Value` — [`Value`](../index.md)

##### `impl Debug for Value`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Value`

- `fn default() -> Value` — [`Value`](../index.md)

##### `impl<'de> Deserialize for crate::value::Value`

- `fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](../index.md)

##### `impl<T> DeserializeOwned for Value`

##### `impl<'de> Deserializer for crate::value::Value`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../index.md)

##### `impl Display for Value`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Value`

##### `impl<T: Into<super::Value>> FromIterator for super::Value`

- `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl FromStr for crate::value::Value`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](../index.md), [`Error`](../index.md)

##### `impl Hash for Value`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> Index for super::Value`

- `type Output = Value`

- `fn index(self: &Self, index: I) -> &Value` — [`Value`](../index.md)

##### `impl<I> IndexMut for super::Value`

- `fn index_mut(self: &mut Self, index: I) -> &mut Value` — [`Value`](../index.md)

##### `impl<'de> IntoDeserializer for crate::value::Value`

- `type Deserializer = Value`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl PartialEq for Value`

- `fn eq(self: &Self, other: &u64) -> bool`

##### `impl Serialize for crate::value::Value`

- `fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Value`

##### `impl<T> ToString for Value`

- `fn to_string(self: &Self) -> String`

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

