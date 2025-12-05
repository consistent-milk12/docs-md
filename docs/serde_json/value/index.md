*[serde_json](../index.md) / [value](index.md)*

---

# Module `value`

The Value enum, a loosely typed way of representing any valid JSON value.

# Constructing JSON

Serde JSON provides a [`json!` macro][macro](#macro) to build `serde_json::Value`
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
[`serde_json::from_str`][from_str](#from-str) function. There is also
[`from_slice`][from_slice](#from-slice) for parsing from a byte slice `&[u8]` and
[`from_reader`][from_reader](#from-reader) for parsing from any `io::Read` like a File or
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

[macro](#macro): crate::json
[from_str](#from-str): crate::de::from_str
[from_slice](#from-slice): crate::de::from_slice
[from_reader](#from-reader): crate::de::from_reader

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

##### `impl Serializer`

- `type Ok = Value`

- `type Error = Error`

- `type SerializeSeq = SerializeVec`

- `type SerializeTuple = SerializeVec`

- `type SerializeTupleStruct = SerializeVec`

- `type SerializeTupleVariant = SerializeTupleVariant`

- `type SerializeMap = SerializeMap`

- `type SerializeStruct = SerializeMap`

- `type SerializeStructVariant = SerializeStructVariant`

- `fn serialize_bool(self: Self, value: bool) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_i8(self: Self, value: i8) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_i16(self: Self, value: i16) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_i32(self: Self, value: i32) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_i64(self: Self, value: i64) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_i128(self: Self, value: i128) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_u8(self: Self, value: u8) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_u16(self: Self, value: u16) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_u32(self: Self, value: u32) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_u64(self: Self, value: u64) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_u128(self: Self, value: u128) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_f32(self: Self, float: f32) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_f64(self: Self, float: f64) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_char(self: Self, value: char) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_str(self: Self, value: &str) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_bytes(self: Self, value: &[u8]) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_unit(self: Self) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_none(self: Self) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_some<T>(self: Self, value: &T) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

- `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../../error/index.md)

- `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../../error/index.md)

- `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../../error/index.md)

- `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../../error/index.md)

- `fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../../error/index.md)

- `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../../error/index.md)

- `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../../error/index.md)

- `fn collect_str<T>(self: Self, value: &T) -> Result<Value>` — [`Result`](../../error/index.md), [`Value`](../../value/index.md)

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

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>` — [`Value`](../../value/index.md)

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>` — [`Value`](../../value/index.md)

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../../value/index.md)

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>` — [`Value`](../../value/index.md)

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>` — [`Value`](../../value/index.md)

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>` — [`Value`](../../value/index.md)

- `fn append(self: &mut Self, other: &mut Self)`

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>` — [`Entry`](../../map/index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](../../map/index.md)

- `fn iter_mut(self: &mut Self) -> IterMut<'_>` — [`IterMut`](../../map/index.md)

- `fn keys(self: &Self) -> Keys<'_>` — [`Keys`](../../map/index.md)

- `fn values(self: &Self) -> Values<'_>` — [`Values`](../../map/index.md)

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>` — [`ValuesMut`](../../map/index.md)

- `fn into_values(self: Self) -> IntoValues` — [`IntoValues`](../../map/index.md)

- `fn retain<F>(self: &mut Self, f: F)`

- `fn sort_keys(self: &mut Self)`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

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

##### `impl Eq`

##### `impl Extend`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl FromIterator`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../../error/index.md)

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Index<Q>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value` — [`Value`](../../value/index.md)

##### `impl IndexMut<Q>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value` — [`Value`](../../value/index.md)

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Map<String, Value>`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl IntoIterator`

- `type Item = (String, Value)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `Number`

```rust
struct Number {
    n: N,
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- `fn unexpected(self: &Self) -> Unexpected<'_>`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Number` — [`Number`](../../number/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](../../number/index.md)

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

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

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Number) -> bool` — [`Number`](../../number/index.md)

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

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

- `fn invalid_type<E>(self: &Self, exp: &dyn Expected) -> E`

- `fn unexpected(self: &Self) -> Unexpected<'_>`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Value` — [`Value`](../../value/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Value` — [`Value`](../../value/index.md)

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](../../value/index.md)

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md)

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl FromIterator<K: Into<alloc::string::String>, V: Into<super::Value>>`

- `fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](../../value/index.md), [`Error`](../../error/index.md)

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Index<I>`

- `type Output = Value`

- `fn index(self: &Self, index: I) -> &Value` — [`Value`](../../value/index.md)

##### `impl IndexMut<I>`

- `fn index_mut(self: &mut Self, index: I) -> &mut Value` — [`Value`](../../value/index.md)

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Value`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Value) -> bool` — [`Value`](../../value/index.md)

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

## Traits

## Functions

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

