*[serde_json](../../index.md) / [value](../index.md) / [ser](index.md)*

---

# Module `ser`

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

- `fn serialize_bool(self: Self, value: bool) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_i8(self: Self, value: i8) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_i16(self: Self, value: i16) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_i32(self: Self, value: i32) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_i64(self: Self, value: i64) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_i128(self: Self, value: i128) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_u8(self: Self, value: u8) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_u16(self: Self, value: u16) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_u32(self: Self, value: u32) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_u64(self: Self, value: u64) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_u128(self: Self, value: u128) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_f32(self: Self, float: f32) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_f64(self: Self, float: f64) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_char(self: Self, value: char) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_str(self: Self, value: &str) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_bytes(self: Self, value: &[u8]) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_unit(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_none(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_some<T>(self: Self, value: &T) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

- `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../../index.md)

- `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../../index.md)

- `fn serialize_tuple_struct(self: Self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../../index.md)

- `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../../index.md)

- `fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../../index.md)

- `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../../index.md)

- `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../../index.md)

- `fn collect_str<T>(self: Self, value: &T) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

### `SerializeVec`

```rust
struct SerializeVec {
    vec: alloc::vec::Vec<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl SerializeSeq for SerializeVec`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_element<T>(self: &mut Self, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

##### `impl SerializeTuple for SerializeVec`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_element<T>(self: &mut Self, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

##### `impl SerializeTupleStruct for SerializeVec`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

### `SerializeTupleVariant`

```rust
struct SerializeTupleVariant {
    name: alloc::string::String,
    vec: alloc::vec::Vec<crate::value::Value>,
}
```

#### Trait Implementations

##### `impl SerializeTupleVariant for SerializeTupleVariant`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

### `SerializeStructVariant`

```rust
struct SerializeStructVariant {
    name: alloc::string::String,
    map: crate::map::Map<alloc::string::String, crate::value::Value>,
}
```

#### Trait Implementations

##### `impl SerializeStructVariant for SerializeStructVariant`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, key: &'static str, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

### `MapKeySerializer`

```rust
struct MapKeySerializer;
```

#### Trait Implementations

##### `impl Serializer for MapKeySerializer`

- `type Ok = String`

- `type Error = Error`

- `type SerializeSeq = Impossible<String, Error>`

- `type SerializeTuple = Impossible<String, Error>`

- `type SerializeTupleStruct = Impossible<String, Error>`

- `type SerializeTupleVariant = Impossible<String, Error>`

- `type SerializeMap = Impossible<String, Error>`

- `type SerializeStruct = Impossible<String, Error>`

- `type SerializeStructVariant = Impossible<String, Error>`

- `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_bool(self: Self, value: bool) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_i8(self: Self, value: i8) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_i16(self: Self, value: i16) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_i32(self: Self, value: i32) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_i64(self: Self, value: i64) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_i128(self: Self, value: i128) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_u8(self: Self, value: u8) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_u16(self: Self, value: u16) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_u32(self: Self, value: u32) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_u64(self: Self, value: u64) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_u128(self: Self, value: u128) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_f32(self: Self, value: f32) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_f64(self: Self, value: f64) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_char(self: Self, value: char) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_str(self: Self, value: &str) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_bytes(self: Self, _value: &[u8]) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_unit(self: Self) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_none(self: Self) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_some<T>(self: Self, _value: &T) -> Result<String>` — [`Result`](../../index.md)

- `fn serialize_seq(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../../index.md)

- `fn serialize_tuple(self: Self, _len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../../index.md)

- `fn serialize_tuple_struct(self: Self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../../index.md)

- `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../../index.md)

- `fn serialize_map(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../../index.md)

- `fn serialize_struct(self: Self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../../index.md)

- `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../../index.md)

- `fn collect_str<T>(self: Self, value: &T) -> Result<String>` — [`Result`](../../index.md)

## Enums

### `SerializeMap`

```rust
enum SerializeMap {
    Map {
        map: crate::map::Map<alloc::string::String, crate::value::Value>,
        next_key: Option<alloc::string::String>,
    },
}
```

#### Trait Implementations

##### `impl SerializeMap for SerializeMap`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_key<T>(self: &mut Self, key: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn serialize_value<T>(self: &mut Self, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

##### `impl SerializeStruct for SerializeMap`

- `type Ok = Value`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, key: &'static str, value: &T) -> Result<()>` — [`Result`](../../index.md)

- `fn end(self: Self) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../../index.md)

## Functions

### `key_must_be_a_string`

```rust
fn key_must_be_a_string() -> crate::error::Error
```

### `float_key_must_be_finite`

```rust
fn float_key_must_be_finite() -> crate::error::Error
```

