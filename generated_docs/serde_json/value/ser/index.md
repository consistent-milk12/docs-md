*[serde_json](../../index.md) / [value](../index.md) / [ser](index.md)*

---

# Module `ser`

## Contents

- [Structs](#structs)
  - [`Serializer`](#serializer)
  - [`SerializeVec`](#serializevec)
  - [`SerializeTupleVariant`](#serializetuplevariant)
  - [`SerializeStructVariant`](#serializestructvariant)
  - [`MapKeySerializer`](#mapkeyserializer)
- [Enums](#enums)
  - [`SerializeMap`](#serializemap)
- [Functions](#functions)
  - [`key_must_be_a_string`](#key-must-be-a-string)
  - [`float_key_must_be_finite`](#float-key-must-be-finite)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Serializer`](#serializer) | struct | Serializer whose output is a `Value`. |
| [`SerializeVec`](#serializevec) | struct |  |
| [`SerializeTupleVariant`](#serializetuplevariant) | struct |  |
| [`SerializeStructVariant`](#serializestructvariant) | struct |  |
| [`MapKeySerializer`](#mapkeyserializer) | struct |  |
| [`SerializeMap`](#serializemap) | enum |  |
| [`key_must_be_a_string`](#key-must-be-a-string) | fn |  |
| [`float_key_must_be_finite`](#float-key-must-be-finite) | fn |  |

## Structs

### `Serializer`

```rust
struct Serializer;
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:58`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L58)*

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

- <span id="serializer-serializer-type-ok"></span>`type Ok = Value`

- <span id="serializer-serializer-type-error"></span>`type Error = Error`

- <span id="serializer-serializer-type-serializeseq"></span>`type SerializeSeq = SerializeVec`

- <span id="serializer-serializer-type-serializetuple"></span>`type SerializeTuple = SerializeVec`

- <span id="serializer-serializer-type-serializetuplestruct"></span>`type SerializeTupleStruct = SerializeVec`

- <span id="serializer-serializer-type-serializetuplevariant"></span>`type SerializeTupleVariant = SerializeTupleVariant`

- <span id="serializer-serializer-type-serializemap"></span>`type SerializeMap = SerializeMap`

- <span id="serializer-serializer-type-serializestruct"></span>`type SerializeStruct = SerializeMap`

- <span id="serializer-serializer-type-serializestructvariant"></span>`type SerializeStructVariant = SerializeStructVariant`

- <span id="serializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-f32"></span>`fn serialize_f32(self, float: f32) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-f64"></span>`fn serialize_f64(self, float: f64) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-bytes"></span>`fn serialize_bytes(self, value: &[u8]) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-none"></span>`fn serialize_none(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

- <span id="serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../../error/index.md#result)

- <span id="serializer-serialize-tuple"></span>`fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../../error/index.md#result)

- <span id="serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../../error/index.md#result)

- <span id="serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../../error/index.md#result)

- <span id="serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../../error/index.md#result)

- <span id="serializer-serialize-struct"></span>`fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../../error/index.md#result)

- <span id="serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../../error/index.md#result)

- <span id="serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

### `SerializeVec`

```rust
struct SerializeVec {
    vec: alloc::vec::Vec<crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:302-304`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L302-L304)*

#### Trait Implementations

##### `impl SerializeSeq for SerializeVec`

- <span id="serializevec-serializeseq-type-ok"></span>`type Ok = Value`

- <span id="serializevec-serializeseq-type-error"></span>`type Error = Error`

- <span id="serializevec-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializevec-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

##### `impl SerializeTuple for SerializeVec`

- <span id="serializevec-serializetuple-type-ok"></span>`type Ok = Value`

- <span id="serializevec-serializetuple-type-error"></span>`type Error = Error`

- <span id="serializevec-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializevec-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

##### `impl SerializeTupleStruct for SerializeVec`

- <span id="serializevec-serializetuplestruct-type-ok"></span>`type Ok = Value`

- <span id="serializevec-serializetuplestruct-type-error"></span>`type Error = Error`

- <span id="serializevec-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializevec-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

### `SerializeTupleVariant`

```rust
struct SerializeTupleVariant {
    name: alloc::string::String,
    vec: alloc::vec::Vec<crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:306-309`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L306-L309)*

#### Trait Implementations

##### `impl SerializeTupleVariant for SerializeTupleVariant`

- <span id="serializetuplevariant-serializetuplevariant-type-ok"></span>`type Ok = Value`

- <span id="serializetuplevariant-serializetuplevariant-type-error"></span>`type Error = Error`

- <span id="serializetuplevariant-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializetuplevariant-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

### `SerializeStructVariant`

```rust
struct SerializeStructVariant {
    name: alloc::string::String,
    map: crate::map::Map<alloc::string::String, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:322-325`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L322-L325)*

#### Trait Implementations

##### `impl SerializeStructVariant for SerializeStructVariant`

- <span id="serializestructvariant-serializestructvariant-type-ok"></span>`type Ok = Value`

- <span id="serializestructvariant-serializestructvariant-type-error"></span>`type Error = Error`

- <span id="serializestructvariant-serialize-field"></span>`fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializestructvariant-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

### `MapKeySerializer`

```rust
struct MapKeySerializer;
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:448`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L448)*

#### Trait Implementations

##### `impl Serializer for MapKeySerializer`

- <span id="mapkeyserializer-serializer-type-ok"></span>`type Ok = String`

- <span id="mapkeyserializer-serializer-type-error"></span>`type Error = Error`

- <span id="mapkeyserializer-serializer-type-serializeseq"></span>`type SerializeSeq = Impossible<String, Error>`

- <span id="mapkeyserializer-serializer-type-serializetuple"></span>`type SerializeTuple = Impossible<String, Error>`

- <span id="mapkeyserializer-serializer-type-serializetuplestruct"></span>`type SerializeTupleStruct = Impossible<String, Error>`

- <span id="mapkeyserializer-serializer-type-serializetuplevariant"></span>`type SerializeTupleVariant = Impossible<String, Error>`

- <span id="mapkeyserializer-serializer-type-serializemap"></span>`type SerializeMap = Impossible<String, Error>`

- <span id="mapkeyserializer-serializer-type-serializestruct"></span>`type SerializeStruct = Impossible<String, Error>`

- <span id="mapkeyserializer-serializer-type-serializestructvariant"></span>`type SerializeStructVariant = Impossible<String, Error>`

- <span id="mapkeyserializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-f32"></span>`fn serialize_f32(self, value: f32) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-f64"></span>`fn serialize_f64(self, value: f64) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-bytes"></span>`fn serialize_bytes(self, _value: &[u8]) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-none"></span>`fn serialize_none(self) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-some"></span>`fn serialize_some<T>(self, _value: &T) -> Result<String>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-seq"></span>`fn serialize_seq(self, _len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-tuple"></span>`fn serialize_tuple(self, _len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-map"></span>`fn serialize_map(self, _len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-struct"></span>`fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../../error/index.md#result)

- <span id="mapkeyserializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<String>` — [`Result`](../../error/index.md#result)

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

*Defined in [`serde_json-1.0.145/src/value/ser.rs:311-320`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L311-L320)*

#### Trait Implementations

##### `impl SerializeMap for SerializeMap`

- <span id="serializemap-serializemap-type-ok"></span>`type Ok = Value`

- <span id="serializemap-serializemap-type-error"></span>`type Error = Error`

- <span id="serializemap-serialize-key"></span>`fn serialize_key<T>(&mut self, key: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializemap-serialize-value"></span>`fn serialize_value<T>(&mut self, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializemap-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

##### `impl SerializeStruct for SerializeMap`

- <span id="serializemap-serializestruct-type-ok"></span>`type Ok = Value`

- <span id="serializemap-serializestruct-type-error"></span>`type Error = Error`

- <span id="serializemap-serialize-field"></span>`fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="serializemap-end"></span>`fn end(self) -> Result<Value>` — [`Result`](../../error/index.md#result), [`Value`](../index.md#value)

## Functions

### `key_must_be_a_string`

```rust
fn key_must_be_a_string() -> crate::error::Error
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:450-452`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L450-L452)*

### `float_key_must_be_finite`

```rust
fn float_key_must_be_finite() -> crate::error::Error
```

*Defined in [`serde_json-1.0.145/src/value/ser.rs:454-456`](../../../../.source_1765210505/serde_json-1.0.145/src/value/ser.rs#L454-L456)*

