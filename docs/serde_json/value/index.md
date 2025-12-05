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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

- `fn serialize_bool(self: Self, value: bool) -> Result<Value>`

- `fn serialize_i8(self: Self, value: i8) -> Result<Value>`

- `fn serialize_i16(self: Self, value: i16) -> Result<Value>`

- `fn serialize_i32(self: Self, value: i32) -> Result<Value>`

- `fn serialize_i64(self: Self, value: i64) -> Result<Value>`

- `fn serialize_i128(self: Self, value: i128) -> Result<Value>`

- `fn serialize_u8(self: Self, value: u8) -> Result<Value>`

- `fn serialize_u16(self: Self, value: u16) -> Result<Value>`

- `fn serialize_u32(self: Self, value: u32) -> Result<Value>`

- `fn serialize_u64(self: Self, value: u64) -> Result<Value>`

- `fn serialize_u128(self: Self, value: u128) -> Result<Value>`

- `fn serialize_f32(self: Self, float: f32) -> Result<Value>`

- `fn serialize_f64(self: Self, float: f64) -> Result<Value>`

- `fn serialize_char(self: Self, value: char) -> Result<Value>`

- `fn serialize_str(self: Self, value: &str) -> Result<Value>`

- `fn serialize_bytes(self: Self, value: &[u8]) -> Result<Value>`

- `fn serialize_unit(self: Self) -> Result<Value>`

- `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<Value>`

- `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>`

- `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<Value>`

- `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>`

- `fn serialize_none(self: Self) -> Result<Value>`

- `fn serialize_some<T>(self: Self, value: &T) -> Result<Value>`

- `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>`

- `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple>`

- `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>`

- `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>`

- `fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap>`

- `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>`

- `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>`

- `fn collect_str<T>(self: Self, value: &T) -> Result<Value>`

### `Map<K, V>`

```rust
struct Map<K, V> {
    // [REDACTED: Private Fields]
}
```

Represents a JSON key/value type.

#### Implementations

- `fn new() -> Self`
  Makes a new empty Map.

- `fn with_capacity(capacity: usize) -> Self`
  Makes a new empty Map with the given initial capacity.

- `fn clear(self: &mut Self)`
  Clears the map, removing all values.

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>`
  Returns a reference to the value corresponding to the key.

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`
  Returns true if the map contains a value for the specified key.

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>`
  Returns a mutable reference to the value corresponding to the key.

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>`
  Returns the key-value pair matching the given key.

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>`
  Inserts a key-value pair into the map.

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>`
  Removes a key from the map, returning the value at the key if the key

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>`
  Removes a key from the map, returning the stored key and value if the

- `fn append(self: &mut Self, other: &mut Self)`
  Moves all elements from other into self, leaving other empty.

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>`
  Gets the given key's corresponding entry in the map for in-place

- `fn len(self: &Self) -> usize`
  Returns the number of elements in the map.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the map contains no elements.

- `fn iter(self: &Self) -> Iter<'_>`
  Gets an iterator over the entries of the map.

- `fn iter_mut(self: &mut Self) -> IterMut<'_>`
  Gets a mutable iterator over the entries of the map.

- `fn keys(self: &Self) -> Keys<'_>`
  Gets an iterator over the keys of the map.

- `fn values(self: &Self) -> Values<'_>`
  Gets an iterator over the values of the map.

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>`
  Gets an iterator over mutable values of the map.

- `fn into_values(self: Self) -> IntoValues`
  Gets an iterator over the values of the map.

- `fn retain<F>(self: &mut Self, f: F)`
  Retains only the elements specified by the predicate.

- `fn sort_keys(self: &mut Self)`
  Sorts this map's entries in-place using `str`'s usual ordering.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Self, Error>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Map<String, Value>`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl IntoIterator`

- `type Item = (String, Value)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Extend`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Index<Q>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value`

##### `impl IndexMut<Q>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `Number`

```rust
struct Number {
    // [REDACTED: Private Fields]
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- `fn is_i64(self: &Self) -> bool`
  Returns true if the `Number` is an integer between `i64::MIN` and

- `fn is_u64(self: &Self) -> bool`
  Returns true if the `Number` is an integer between zero and `u64::MAX`.

- `fn is_f64(self: &Self) -> bool`
  Returns true if the `Number` can be represented by f64.

- `fn as_i64(self: &Self) -> Option<i64>`
  If the `Number` is an integer, represent it as i64 if possible. Returns

- `fn as_u64(self: &Self) -> Option<u64>`
  If the `Number` is an integer, represent it as u64 if possible. Returns

- `fn as_f64(self: &Self) -> Option<f64>`
  Represents the number as f64 if possible. Returns None otherwise.

- `fn from_f64(f: f64) -> Option<Number>`
  Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON

- `fn as_i128(self: &Self) -> Option<i128>`
  If the `Number` is an integer, represent it as i128 if possible. Returns

- `fn as_u128(self: &Self) -> Option<u128>`
  If the `Number` is an integer, represent it as u128 if possible. Returns

- `fn from_i128(i: i128) -> Option<Number>`
  Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or

- `fn from_u128(i: u128) -> Option<Number>`
  Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

#### Trait Implementations

##### `impl From`

- `fn from(u: u32) -> Self`

##### `impl From`

- `fn from(i: i8) -> Self`

##### `impl From`

- `fn from(u: usize) -> Self`

##### `impl From`

- `fn from(u: u8) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(u: u16) -> Self`

##### `impl From`

- `fn from(i: i64) -> Self`

##### `impl From`

- `fn from(u: u64) -> Self`

##### `impl From`

- `fn from(i: i16) -> Self`

##### `impl From`

- `fn from(i: isize) -> Self`

##### `impl From`

- `fn from(i: i32) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Number`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Number) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

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

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

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

- `fn get<I: Index>(self: &Self, index: I) -> Option<&Value>`
  Index into a JSON array or map. A string index can be used to access a

- `fn get_mut<I: Index>(self: &mut Self, index: I) -> Option<&mut Value>`
  Mutably index into a JSON array or map. A string index can be used to

- `fn is_object(self: &Self) -> bool`
  Returns true if the `Value` is an Object. Returns false otherwise.

- `fn as_object(self: &Self) -> Option<&Map<String, Value>>`
  If the `Value` is an Object, returns the associated Map. Returns None

- `fn as_object_mut(self: &mut Self) -> Option<&mut Map<String, Value>>`
  If the `Value` is an Object, returns the associated mutable Map.

- `fn is_array(self: &Self) -> bool`
  Returns true if the `Value` is an Array. Returns false otherwise.

- `fn as_array(self: &Self) -> Option<&Vec<Value>>`
  If the `Value` is an Array, returns the associated vector. Returns None

- `fn as_array_mut(self: &mut Self) -> Option<&mut Vec<Value>>`
  If the `Value` is an Array, returns the associated mutable vector.

- `fn is_string(self: &Self) -> bool`
  Returns true if the `Value` is a String. Returns false otherwise.

- `fn as_str(self: &Self) -> Option<&str>`
  If the `Value` is a String, returns the associated str. Returns None

- `fn is_number(self: &Self) -> bool`
  Returns true if the `Value` is a Number. Returns false otherwise.

- `fn as_number(self: &Self) -> Option<&Number>`
  If the `Value` is a Number, returns the associated [`Number`]. Returns

- `fn is_i64(self: &Self) -> bool`
  Returns true if the `Value` is an integer between `i64::MIN` and

- `fn is_u64(self: &Self) -> bool`
  Returns true if the `Value` is an integer between zero and `u64::MAX`.

- `fn is_f64(self: &Self) -> bool`
  Returns true if the `Value` is a number that can be represented by f64.

- `fn as_i64(self: &Self) -> Option<i64>`
  If the `Value` is an integer, represent it as i64 if possible. Returns

- `fn as_u64(self: &Self) -> Option<u64>`
  If the `Value` is an integer, represent it as u64 if possible. Returns

- `fn as_f64(self: &Self) -> Option<f64>`
  If the `Value` is a number, represent it as f64 if possible. Returns

- `fn is_boolean(self: &Self) -> bool`
  Returns true if the `Value` is a Boolean. Returns false otherwise.

- `fn as_bool(self: &Self) -> Option<bool>`
  If the `Value` is a Boolean, returns the associated bool. Returns None

- `fn is_null(self: &Self) -> bool`
  Returns true if the `Value` is a Null. Returns false otherwise.

- `fn as_null(self: &Self) -> Option<()>`
  If the `Value` is a Null, returns (). Returns None otherwise.

- `fn pointer(self: &Self, pointer: &str) -> Option<&Value>`
  Looks up a value by a JSON Pointer.

- `fn pointer_mut(self: &mut Self, pointer: &str) -> Option<&mut Value>`
  Looks up a value by a JSON Pointer and returns a mutable reference to

- `fn take(self: &mut Self) -> Value`
  Takes the value out of the `Value`, leaving a `Null` in its place.

- `fn sort_all_objects(self: &mut Self)`
  Reorders the entries of all `Value::Object` nested within this JSON

#### Trait Implementations

##### `impl From`

- `fn from((): ()) -> Self`
  Convert `()` to `Value::Null`.

##### `impl From`

- `fn from(n: i64) -> Self`

##### `impl From<T: Clone + Into<super::Value>>`

- `fn from(f: &[T]) -> Self`
  Convert a slice to `Value::Array`.

##### `impl From<T: Into<super::Value>>`

- `fn from(f: Vec<T>) -> Self`
  Convert a `Vec` to `Value::Array`.

##### `impl From<T: Into<super::Value>, const N: usize>`

- `fn from(array: [T; N]) -> Self`

##### `impl From`

- `fn from(n: i32) -> Self`

##### `impl From`

- `fn from(n: i16) -> Self`

##### `impl From`

- `fn from(f: Map<String, Value>) -> Self`
  Convert map (with string keys) to `Value::Object`.

##### `impl From`

- `fn from(f: &str) -> Self`
  Convert string slice to `Value::String`.

##### `impl From`

- `fn from(f: f32) -> Self`
  Convert 32-bit floating point number to `Value::Number`, or

##### `impl From`

- `fn from(n: u64) -> Self`

##### `impl From`

- `fn from(n: usize) -> Self`

##### `impl From`

- `fn from(n: i8) -> Self`

##### `impl From`

- `fn from(f: Number) -> Self`
  Convert `Number` to `Value::Number`.

##### `impl From<'a>`

- `fn from(f: Cow<'a, str>) -> Self`
  Convert copy-on-write string to `Value::String`.

##### `impl From`

- `fn from(n: u8) -> Self`

##### `impl From`

- `fn from(f: bool) -> Self`
  Convert boolean to `Value::Bool`.

##### `impl From`

- `fn from(f: f64) -> Self`
  Convert 64-bit floating point number to `Value::Number`, or

##### `impl From`

- `fn from(n: isize) -> Self`

##### `impl From`

- `fn from(n: u16) -> Self`

##### `impl From<T>`

- `fn from(opt: Option<T>) -> Self`

##### `impl From`

- `fn from(n: u32) -> Self`

##### `impl From`

- `fn from(f: String) -> Self`
  Convert `String` to `Value::String`.

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator<T: Into<super::Value>>`

- `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`
  Create a `Value::Array` by collecting an iterator of array elements.

##### `impl FromIterator<K: Into<alloc::string::String>, V: Into<super::Value>>`

- `fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self`
  Create a `Value::Object` by collecting an iterator of key-value pairs.

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Value, Error>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Value`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Value`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`
  Display a JSON value as a string.

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Index<I>`

- `type Output = Value`

- `fn index(self: &Self, index: I) -> &Value`
  Index into a `serde_json::Value` using the syntax `value[0]` or

##### `impl IndexMut<I>`

- `fn index_mut(self: &mut Self, index: I) -> &mut Value`
  Write into a `serde_json::Value` using the syntax `value[0] = ...` or

##### `impl PartialEq`

- `fn eq(self: &Self, other: &bool) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &usize) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i64) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Value) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i32) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u64) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &isize) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u8) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u16) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i8) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &f64) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i16) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &f32) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u32) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Value`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

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

