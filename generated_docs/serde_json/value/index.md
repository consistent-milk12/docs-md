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
[`serde_json::from_str`][`from_str`](../de/index.md) function. There is also
[`from_slice`][`from_slice`](../de/index.md) for parsing from a byte slice `&[u8]` and
[`from_reader`][`from_reader`](../de/index.md) for parsing from any `io::Read` like a File or
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
  - [`partial_eq`](#partial-eq)
  - [`ser`](#ser)
- [Structs](#structs)
  - [`Serializer`](#serializer)
  - [`Map`](#map)
  - [`Number`](#number)
- [Enums](#enums)
  - [`Value`](#value)
- [Traits](#traits)
  - [`Index`](#index)
- [Functions](#functions)
  - [`parse_index`](#parse-index)
  - [`to_value`](#to-value)
  - [`from_value`](#from-value)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`de`](#de) | mod |  |
| [`from`](#from) | mod |  |
| [`index`](#index) | mod |  |
| [`partial_eq`](#partial-eq) | mod |  |
| [`ser`](#ser) | mod |  |
| [`Serializer`](#serializer) | struct |  |
| [`Map`](#map) | struct |  |
| [`Number`](#number) | struct |  |
| [`Value`](#value) | enum | Represents any valid JSON value. |
| [`Index`](#index) | trait |  |
| [`parse_index`](#parse-index) | fn |  |
| [`to_value`](#to-value) | fn | Convert a `T` into `serde_json::Value` which is an enum that can represent any valid JSON data. |
| [`from_value`](#from-value) | fn | Interpret a `serde_json::Value` as an instance of type `T`. |

## Modules

- [`de`](de/index.md)
- [`from`](from/index.md)
- [`index`](index/index.md)
- [`partial_eq`](partial_eq/index.md)
- [`ser`](ser/index.md)

## Structs

### `Serializer`

```rust
struct Serializer;
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:58`](../../../.source_1765521767/serde_json-1.0.145/src/value/ser.rs#L58)*

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

##### `impl Any for Serializer`

- <span id="serializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Serializer`

- <span id="serializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Serializer`

- <span id="serializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Serializer`

- <span id="serializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Serializer`

- <span id="serializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Serializer for Serializer`

- <span id="serializer-serializer-type-ok"></span>`type Ok = Value`

- <span id="serializer-serializer-type-error"></span>`type Error = Error`

- <span id="serializer-serializer-type-serializeseq"></span>`type SerializeSeq = SerializeVec`

- <span id="serializer-serializer-type-serializetuple"></span>`type SerializeTuple = SerializeVec`

- <span id="serializer-serializer-type-serializetuplestruct"></span>`type SerializeTupleStruct = SerializeVec`

- <span id="serializer-serializer-type-serializetuplevariant"></span>`type SerializeTupleVariant = SerializeTupleVariant`

- <span id="serializer-serializer-type-serializemap"></span>`type SerializeMap = SerializeMap`

- <span id="serializer-serializer-type-serializestruct"></span>`type SerializeStruct = SerializeMap`

- <span id="serializer-serializer-type-serializestructvariant"></span>`type SerializeStructVariant = SerializeStructVariant`

- <span id="serializer-serializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-f32"></span>`fn serialize_f32(self, float: f32) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-f64"></span>`fn serialize_f64(self, float: f64) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-bytes"></span>`fn serialize_bytes(self, value: &[u8]) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-none"></span>`fn serialize_none(self) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

- <span id="serializer-serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-serialize-tuple"></span>`fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-serialize-struct"></span>`fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../error/index.md#result)

- <span id="serializer-serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<Value>` — [`Result`](../error/index.md#result), [`Value`](#value)

##### `impl<U> TryFrom for Serializer`

- <span id="serializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="serializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Serializer`

- <span id="serializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="serializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:29-31`](../../../.source_1765521767/serde_json-1.0.145/src/map.rs#L29-L31)*

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

  Makes a new empty Map.

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Makes a new empty Map with the given initial capacity.

- <span id="map-clear"></span>`fn clear(&mut self)`

  Clears the map, removing all values.

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](#value)

  Returns a reference to the value corresponding to the key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

  Returns true if the map contains a value for the specified key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](#value)

  Returns a mutable reference to the value corresponding to the key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](#value)

  Returns the key-value pair matching the given key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](#value)

  Inserts a key-value pair into the map.

  

  If the map did not have this key present, `None` is returned.

  

  If the map did have this key present, the value is updated, and the old

  value is returned.

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](#value)

  Removes a key from the map, returning the value at the key if the key

  was previously in the map.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

  

  If serde_json's "preserve_order" is enabled, `.remove(key)` is

  equivalent to `.swap_remove(key)`, replacing this

  entry's position with the last element. If you need to preserve the

  relative order of the keys in the map, use

  `.shift_remove(key)` instead.

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](#value)

  Removes a key from the map, returning the stored key and value if the

  key was previously in the map.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

  

  If serde_json's "preserve_order" is enabled, `.remove_entry(key)` is

  equivalent to `.swap_remove_entry(key)`,

  replacing this entry's position with the last element. If you need to

  preserve the relative order of the keys in the map, use

  `.shift_remove_entry(key)` instead.

- <span id="map-append"></span>`fn append(&mut self, other: &mut Self)`

  Moves all elements from other into self, leaving other empty.

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](../map/index.md#entry)

  Gets the given key's corresponding entry in the map for in-place

  manipulation.

- <span id="map-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the map.

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if the map contains no elements.

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](../map/index.md#iter)

  Gets an iterator over the entries of the map.

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](../map/index.md#itermut)

  Gets a mutable iterator over the entries of the map.

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](../map/index.md#keys)

  Gets an iterator over the keys of the map.

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](../map/index.md#values)

  Gets an iterator over the values of the map.

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](../map/index.md#valuesmut)

  Gets an iterator over mutable values of the map.

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](../map/index.md#intovalues)

  Gets an iterator over the values of the map.

- <span id="map-retain"></span>`fn retain<F>(&mut self, f: F)`

  Retains only the elements specified by the predicate.

  

  In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)`

  returns `false`.

- <span id="map-sort-keys"></span>`fn sort_keys(&mut self)`

  Sorts this map's entries in-place using `str`'s usual ordering.

  

  If serde_json's "preserve_order" feature is not enabled, this method

  does no work because all JSON maps are always kept in a sorted state.

  

  If serde_json's "preserve_order" feature is enabled, this method

  destroys the original source order or insertion order of this map in

  favor of an alphanumerical order that matches how a BTreeMap with the

  same contents would be ordered. This takes **O(n log n + c)** time where

  _n_ is the length of the map and _c_ is the capacity.

  

  Other maps nested within the values of this map are not sorted. If you

  need the entire data structure to be sorted at all levels, you must also

  call

  <code>map.[values_mut]\().for_each([Value::sort_all_objects])</code>.

#### Trait Implementations

##### `impl Any for Map<K, V>`

- <span id="map-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Map<K, V>`

- <span id="map-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Map<K, V>`

- <span id="map-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Map<alloc::string::String, crate::value::Value>`

- <span id="map-clone"></span>`fn clone(&self) -> Self`

- <span id="map-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl CloneToUninit for Map<K, V>`

- <span id="map-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Map<alloc::string::String, crate::value::Value>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Map<alloc::string::String, crate::value::Value>`

- <span id="map-default"></span>`fn default() -> Self`

##### `impl Deserialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Map<K, V>`

##### `impl Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-deserializer-type-error"></span>`type Error = Error`

- <span id="cratemapmap-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

- <span id="cratemapmap-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="cratemapmap-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Eq for Map<alloc::string::String, crate::value::Value>`

##### `impl Extend for Map<alloc::string::String, crate::value::Value>`

- <span id="map-extend"></span>`fn extend<T>(&mut self, iter: T)`

##### `impl<T> From for Map<K, V>`

- <span id="map-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Map<alloc::string::String, crate::value::Value>`

- <span id="map-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-fromstr-type-err"></span>`type Err = Error`

- <span id="cratemapmap-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../error/index.md#error)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-type-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](#value)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-indexmut-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](#value)

##### `impl<U> Into for Map<K, V>`

- <span id="map-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-intodeserializer-type-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl IntoIterator for &'a Map<alloc::string::String, crate::value::Value>`

- <span id="a-map-intoiterator-type-item"></span>`type Item = (&'a String, &'a Value)`

- <span id="a-map-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-map-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Map<alloc::string::String, crate::value::Value>`

- <span id="map-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Serialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl ToOwned for Map<K, V>`

- <span id="map-toowned-type-owned"></span>`type Owned = T`

- <span id="map-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="map-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Map<K, V>`

- <span id="map-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="map-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Map<K, V>`

- <span id="map-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="map-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Number`

```rust
struct Number {
    n: N,
}
```

*Defined in [`serde_json-1.0.145/src/number.rs:22-24`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L22-L24)*

Represents a JSON number, whether integer or floating point.

#### Implementations

- <span id="number-is-i64"></span>`fn is_i64(&self) -> bool`

  Returns true if the `Number` is an integer between `i64::MIN` and

  `i64::MAX`.

  

  For any Number on which `is_i64` returns true, `as_i64` is guaranteed to

  return the integer value.

- <span id="number-is-u64"></span>`fn is_u64(&self) -> bool`

  Returns true if the `Number` is an integer between zero and `u64::MAX`.

  

  For any Number on which `is_u64` returns true, `as_u64` is guaranteed to

  return the integer value.

- <span id="number-is-f64"></span>`fn is_f64(&self) -> bool`

  Returns true if the `Number` can be represented by f64.

  

  For any Number on which `is_f64` returns true, `as_f64` is guaranteed to

  return the floating point value.

  

  Currently this function returns true if and only if both `is_i64` and

  `is_u64` return false but this is not a guarantee in the future.

- <span id="number-as-i64"></span>`fn as_i64(&self) -> Option<i64>`

  If the `Number` is an integer, represent it as i64 if possible. Returns

  None otherwise.

- <span id="number-as-u64"></span>`fn as_u64(&self) -> Option<u64>`

  If the `Number` is an integer, represent it as u64 if possible. Returns

  None otherwise.

- <span id="number-as-f64"></span>`fn as_f64(&self) -> Option<f64>`

  Represents the number as f64 if possible. Returns None otherwise.

- <span id="number-from-f64"></span>`fn from_f64(f: f64) -> Option<Number>` — [`Number`](../number/index.md#number)

  Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON

  numbers.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_f64(256.0).is_some());

  

  assert!(Number::from_f64(f64::NAN).is_none());

  ```

- <span id="number-as-i128"></span>`fn as_i128(&self) -> Option<i128>`

  If the `Number` is an integer, represent it as i128 if possible. Returns

  None otherwise.

- <span id="number-as-u128"></span>`fn as_u128(&self) -> Option<u128>`

  If the `Number` is an integer, represent it as u128 if possible. Returns

  None otherwise.

- <span id="number-from-i128"></span>`fn from_i128(i: i128) -> Option<Number>` — [`Number`](../number/index.md#number)

  Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or

  larger than u64::MAX can only be represented in `Number` if serde_json's

  "arbitrary_precision" feature is enabled.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_i128(256).is_some());

  ```

- <span id="number-from-u128"></span>`fn from_u128(i: u128) -> Option<Number>` — [`Number`](../number/index.md#number)

  Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

  be represented in `Number` if serde_json's "arbitrary_precision" feature

  is enabled.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_u128(256).is_some());

  ```

- <span id="number-as-f32"></span>`fn as_f32(&self) -> Option<f32>`

- <span id="number-from-f32"></span>`fn from_f32(f: f32) -> Option<Number>` — [`Number`](../number/index.md#number)

#### Trait Implementations

##### `impl Any for Number`

- <span id="number-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Number`

- <span id="number-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Number`

- <span id="number-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](../number/index.md#number)

##### `impl CloneToUninit for Number`

- <span id="number-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Number`

- <span id="number-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](../number/index.md#number)

##### `impl DeserializeOwned for Number`

##### `impl Deserializer for Number`

- <span id="number-deserializer-type-error"></span>`type Error = Error`

- <span id="number-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Display for Number`

- <span id="number-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Number`

##### `impl<T> From for Number`

- <span id="number-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for crate::number::Number`

- <span id="cratenumbernumber-fromstr-type-err"></span>`type Err = Error`

- <span id="cratenumbernumber-fromstr-from-str"></span>`fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- <span id="number-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Number`

- <span id="number-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Number`

- <span id="number-partialeq-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](../number/index.md#number)

##### `impl Serialize for Number`

- <span id="number-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl ToOwned for Number`

- <span id="number-toowned-type-owned"></span>`type Owned = T`

- <span id="number-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="number-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Number`

- <span id="number-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Number`

- <span id="number-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="number-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Number`

- <span id="number-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="number-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`serde_json-1.0.145/src/value/mod.rs:116-176`](../../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L116-L176)*

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

- <span id="cratevaluevalue-invalid-type"></span>`fn invalid_type<E>(&self, exp: &dyn Expected) -> E`

- <span id="cratevaluevalue-unexpected"></span>`fn unexpected(&self) -> Unexpected<'_>`

#### Trait Implementations

##### `impl Any for Value`

- <span id="value-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Value`

- <span id="value-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Value`

- <span id="value-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](#value)

##### `impl CloneToUninit for Value`

- <span id="value-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Value`

- <span id="value-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Value`

- <span id="value-default"></span>`fn default() -> Value` — [`Value`](#value)

##### `impl Deserialize for crate::value::Value`

- <span id="cratevaluevalue-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](#value)

##### `impl DeserializeOwned for Value`

##### `impl Deserializer for crate::value::Value`

- <span id="cratevaluevalue-deserializer-type-error"></span>`type Error = Error`

- <span id="cratevaluevalue-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

##### `impl Display for Value`

- <span id="value-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Display a JSON value as a string.

  

  ```rust

  use serde_json::json;

  

  let json = json!({ "city": "London", "street": "10 Downing Street" });

  

  // Compact format:

  //

  // {"city":"London","street":"10 Downing Street"}

  let compact = format!("{}", json);

  assert_eq!(compact,

      "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");

  

  // Pretty format:

  //

  // {

  //   "city": "London",

  //   "street": "10 Downing Street"

  // }

  let pretty = format!("{:#}", json);

  assert_eq!(pretty,

      "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");

  ```

##### `impl Eq for Value`

##### `impl<T> From for Value`

- <span id="value-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Into<super::Value>> FromIterator for super::Value`

- <span id="supervalue-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

  Create a `Value::Array` by collecting an iterator of array elements.

  

  # Examples

  

  ```rust

  use serde_json::Value;

  

  let v = std::iter::repeat(42).take(5);

  let x: Value = v.collect();

  ```

  

  ```rust

  use serde_json::Value;

  

  let v: Vec<_> = vec!["lorem", "ipsum", "dolor"];

  let x: Value = v.into_iter().collect();

  ```

  

  ```rust

  use std::iter::FromIterator;

  use serde_json::Value;

  

  let x: Value = Value::from_iter(vec!["lorem", "ipsum", "dolor"]);

  ```

##### `impl FromStr for crate::value::Value`

- <span id="cratevaluevalue-fromstr-type-err"></span>`type Err = Error`

- <span id="cratevaluevalue-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](#value), [`Error`](../error/index.md#error)

##### `impl Hash for Value`

- <span id="value-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> Index for super::Value`

- <span id="supervalue-index-type-output"></span>`type Output = Value`

- <span id="supervalue-index"></span>`fn index(&self, index: I) -> &Value` — [`Value`](#value)

  Index into a `serde_json::Value` using the syntax `value[0]` or

  `value["k"]`.

  

  Returns `Value::Null` if the type of `self` does not match the type of

  the index, for example if the index is a string and `self` is an array

  or a number. Also returns `Value::Null` if the given key does not exist

  in the map or the given index is not within the bounds of the array.

  

  For retrieving deeply nested values, you should have a look at the

  `Value::pointer` method.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  let data = json!({

      "x": {

          "y": ["z", "zz"]

      }

  });

  

  assert_eq!(data["x"]["y"], json!(["z", "zz"]));

  assert_eq!(data["x"]["y"][0], json!("z"));

  

  assert_eq!(data["a"], json!(null)); // returns null for undefined values

  assert_eq!(data["a"]["b"], json!(null)); // does not panic

  ```

##### `impl<I> IndexMut for super::Value`

- <span id="supervalue-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut Value` — [`Value`](#value)

  Write into a `serde_json::Value` using the syntax `value[0] = ...` or

  `value["k"] = ...`.

  

  If the index is a number, the value must be an array of length bigger

  than the index. Indexing into a value that is not an array or an array

  that is too small will panic.

  

  If the index is a string, the value must be an object or null which is

  treated like an empty object. If the key is not already present in the

  object, it will be inserted with a value of null. Indexing into a value

  that is neither an object nor null will panic.

  

  # Examples

  

  ```rust

  use serde_json::json;

  

  let mut data = json!({ "x": 0 });

  

  // replace an existing key

  data["x"] = json!(1);

  

  // insert a new key

  data["y"] = json!([false, false, false]);

  

  // replace an array value

  data["y"][0] = json!(true);

  

  // inserted a deeply nested key

  data["a"]["b"]["c"]["d"] = json!(true);

  

  println!("{}", data);

  ```

##### `impl<U> Into for Value`

- <span id="value-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoDeserializer for crate::value::Value`

- <span id="cratevaluevalue-intodeserializer-type-deserializer"></span>`type Deserializer = Value`

- <span id="cratevaluevalue-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl PartialEq for Value`

- <span id="value-partialeq-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](#value)

##### `impl Serialize for crate::value::Value`

- <span id="cratevaluevalue-serialize"></span>`fn serialize<S>(&self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Value`

##### `impl ToOwned for Value`

- <span id="value-toowned-type-owned"></span>`type Owned = T`

- <span id="value-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="value-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Value`

- <span id="value-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Value`

- <span id="value-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="value-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Value`

- <span id="value-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="value-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Index`

```rust
trait Index: private::Sealed { ... }
```

*Defined in [`serde_json-1.0.145/src/value/index.rs:37-52`](../../../.source_1765521767/serde_json-1.0.145/src/value/index.rs#L37-L52)*

A type that can be used to index into a `serde_json::Value`.

The [`get`](#get) and `get_mut` methods of `Value` accept any type that
implements `Index`, as does the [square-bracket indexing operator]. This
trait is implemented for strings which are used as the index into a JSON
map, and for `usize` which is used as the index into a JSON array.



This trait is sealed and cannot be implemented for types outside of
`serde_json`.

# Examples

```rust
use serde_json::json;

let data = json!({ "inner": [1, 2, 3] });

// Data is a JSON map so it can be indexed with a string.
let inner = &data["inner"];

// Inner is a JSON array so it can be indexed with an integer.
let first = &inner[0];

assert_eq!(first, 1);
```

#### Implementors

- `&T`
- `alloc::string::String`
- `str`
- `usize`

## Functions

### `parse_index`

```rust
fn parse_index(s: &str) -> Option<usize>
```

*Defined in [`serde_json-1.0.145/src/value/mod.rs:259-264`](../../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L259-L264)*

### `to_value`

```rust
fn to_value<T>(value: T) -> Result<Value, crate::error::Error>
where
    T: Serialize
```

*Defined in [`serde_json-1.0.145/src/value/mod.rs:995-1000`](../../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L995-L1000)*

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

*Defined in [`serde_json-1.0.145/src/value/mod.rs:1037-1042`](../../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L1037-L1042)*

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

