*[serde_json](../index.md) / [ser](index.md)*

---

# Module `ser`

Serialize a Rust data structure into JSON data.

## Contents

- [Structs](#structs)
  - [`Serializer`](#serializer)
  - [`MapKeySerializer`](#mapkeyserializer)
  - [`CompactFormatter`](#compactformatter)
  - [`PrettyFormatter`](#prettyformatter)
- [Enums](#enums)
  - [`CharEscape`](#charescape)
- [Traits](#traits)
  - [`Formatter`](#formatter)
- [Functions](#functions)
  - [`key_must_be_a_string`](#key-must-be-a-string)
  - [`float_key_must_be_finite`](#float-key-must-be-finite)
  - [`format_escaped_str`](#format-escaped-str)
  - [`format_escaped_str_contents`](#format-escaped-str-contents)
  - [`to_writer`](#to-writer)
  - [`to_writer_pretty`](#to-writer-pretty)
  - [`to_vec`](#to-vec)
  - [`to_vec_pretty`](#to-vec-pretty)
  - [`to_string`](#to-string)
  - [`to_string_pretty`](#to-string-pretty)
  - [`indent`](#indent)
- [Constants](#constants)
  - [`BB`](#bb)
  - [`TT`](#tt)
  - [`NN`](#nn)
  - [`FF`](#ff)
  - [`RR`](#rr)
  - [`QU`](#qu)
  - [`BS`](#bs)
  - [`UU`](#uu)
  - [`__`](#)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Serializer`](#serializer) | struct | A structure for serializing Rust values into JSON. |
| [`MapKeySerializer`](#mapkeyserializer) | struct |  |
| [`CompactFormatter`](#compactformatter) | struct | This structure compacts a JSON value with no extra whitespace. |
| [`PrettyFormatter`](#prettyformatter) | struct | This structure pretty prints a JSON value to make it human readable. |
| [`CharEscape`](#charescape) | enum | Represents a character escape code in a type-safe manner. |
| [`Formatter`](#formatter) | trait | This trait abstracts away serializing the JSON control characters, which allows the user to optionally pretty print the JSON output. |
| [`key_must_be_a_string`](#key-must-be-a-string) | fn |  |
| [`float_key_must_be_finite`](#float-key-must-be-finite) | fn |  |
| [`format_escaped_str`](#format-escaped-str) | fn |  |
| [`format_escaped_str_contents`](#format-escaped-str-contents) | fn |  |
| [`to_writer`](#to-writer) | fn | Serialize the given data structure as JSON into the I/O stream. |
| [`to_writer_pretty`](#to-writer-pretty) | fn | Serialize the given data structure as pretty-printed JSON into the I/O stream. |
| [`to_vec`](#to-vec) | fn | Serialize the given data structure as a JSON byte vector. |
| [`to_vec_pretty`](#to-vec-pretty) | fn | Serialize the given data structure as a pretty-printed JSON byte vector. |
| [`to_string`](#to-string) | fn | Serialize the given data structure as a String of JSON. |
| [`to_string_pretty`](#to-string-pretty) | fn | Serialize the given data structure as a pretty-printed String of JSON. |
| [`indent`](#indent) | fn |  |
| [`BB`](#bb) | const |  |
| [`TT`](#tt) | const |  |
| [`NN`](#nn) | const |  |
| [`FF`](#ff) | const |  |
| [`RR`](#rr) | const |  |
| [`QU`](#qu) | const |  |
| [`BS`](#bs) | const |  |
| [`UU`](#uu) | const |  |
| [`__`](#) | const |  |

## Structs

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    writer: W,
    formatter: F,
}
```

*Defined in [`serde_json-1.0.145/src/ser.rs:17-20`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L17-L20)*

A structure for serializing Rust values into JSON.

#### Implementations

- <span id="serializer-new"></span>`fn new(writer: W) -> Self`

#### Trait Implementations

##### `impl<W, F> Serializer for &'a mut Serializer<W, F>`

- <span id="a-mut-serializer-serializer-type-ok"></span>`type Ok = ()`

- <span id="a-mut-serializer-serializer-type-error"></span>`type Error = Error`

- <span id="a-mut-serializer-serializer-type-serializeseq"></span>`type SerializeSeq = Compound<'a, W, F>`

- <span id="a-mut-serializer-serializer-type-serializetuple"></span>`type SerializeTuple = Compound<'a, W, F>`

- <span id="a-mut-serializer-serializer-type-serializetuplestruct"></span>`type SerializeTupleStruct = Compound<'a, W, F>`

- <span id="a-mut-serializer-serializer-type-serializetuplevariant"></span>`type SerializeTupleVariant = Compound<'a, W, F>`

- <span id="a-mut-serializer-serializer-type-serializemap"></span>`type SerializeMap = Compound<'a, W, F>`

- <span id="a-mut-serializer-serializer-type-serializestruct"></span>`type SerializeStruct = Compound<'a, W, F>`

- <span id="a-mut-serializer-serializer-type-serializestructvariant"></span>`type SerializeStructVariant = Compound<'a, W, F>`

- <span id="a-mut-serializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-f32"></span>`fn serialize_f32(self, value: f32) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-f64"></span>`fn serialize_f64(self, value: f64) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-bytes"></span>`fn serialize_bytes(self, value: &[u8]) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-none"></span>`fn serialize_none(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-tuple"></span>`fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-struct"></span>`fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../error/index.md#result)

- <span id="a-mut-serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

### `MapKeySerializer<'a, W: 'a, F: 'a>`

```rust
struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}
```

*Defined in [`serde_json-1.0.145/src/ser.rs:773-775`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L773-L775)*

#### Trait Implementations

##### `impl<W, F> Serializer for MapKeySerializer<'a, W, F>`

- <span id="mapkeyserializer-serializer-type-ok"></span>`type Ok = ()`

- <span id="mapkeyserializer-serializer-type-error"></span>`type Error = Error`

- <span id="mapkeyserializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serializer-type-serializeseq"></span>`type SerializeSeq = Impossible<(), Error>`

- <span id="mapkeyserializer-serializer-type-serializetuple"></span>`type SerializeTuple = Impossible<(), Error>`

- <span id="mapkeyserializer-serializer-type-serializetuplestruct"></span>`type SerializeTupleStruct = Impossible<(), Error>`

- <span id="mapkeyserializer-serializer-type-serializetuplevariant"></span>`type SerializeTupleVariant = Impossible<(), Error>`

- <span id="mapkeyserializer-serializer-type-serializemap"></span>`type SerializeMap = Impossible<(), Error>`

- <span id="mapkeyserializer-serializer-type-serializestruct"></span>`type SerializeStruct = Impossible<(), Error>`

- <span id="mapkeyserializer-serializer-type-serializestructvariant"></span>`type SerializeStructVariant = Impossible<(), Error>`

- <span id="mapkeyserializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-f32"></span>`fn serialize_f32(self, value: f32) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-f64"></span>`fn serialize_f64(self, value: f64) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-bytes"></span>`fn serialize_bytes(self, _value: &[u8]) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-none"></span>`fn serialize_none(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-seq"></span>`fn serialize_seq(self, _len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-tuple"></span>`fn serialize_tuple(self, _len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-map"></span>`fn serialize_map(self, _len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-struct"></span>`fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../error/index.md#result)

- <span id="mapkeyserializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<()>` — [`Result`](../error/index.md#result)

### `CompactFormatter`

```rust
struct CompactFormatter;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:1939`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L1939)*

This structure compacts a JSON value with no extra whitespace.

#### Trait Implementations

##### `impl Clone for CompactFormatter`

- <span id="compactformatter-clone"></span>`fn clone(&self) -> CompactFormatter` — [`CompactFormatter`](#compactformatter)

##### `impl Debug for CompactFormatter`

- <span id="compactformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CompactFormatter`

- <span id="compactformatter-default"></span>`fn default() -> CompactFormatter` — [`CompactFormatter`](#compactformatter)

##### `impl Formatter for CompactFormatter`

### `PrettyFormatter<'a>`

```rust
struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}
```

*Defined in [`serde_json-1.0.145/src/ser.rs:1945-1949`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L1945-L1949)*

This structure pretty prints a JSON value to make it human readable.

#### Implementations

- <span id="prettyformatter-new"></span>`fn new() -> Self`

- <span id="prettyformatter-with-indent"></span>`fn with_indent(indent: &'a [u8]) -> Self`

#### Trait Implementations

##### `impl Clone for PrettyFormatter<'a>`

- <span id="prettyformatter-clone"></span>`fn clone(&self) -> PrettyFormatter<'a>` — [`PrettyFormatter`](#prettyformatter)

##### `impl Debug for PrettyFormatter<'a>`

- <span id="prettyformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PrettyFormatter<'a>`

- <span id="prettyformatter-default"></span>`fn default() -> Self`

##### `impl Formatter for PrettyFormatter<'a>`

- <span id="prettyformatter-begin-array"></span>`fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-end-array"></span>`fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-begin-array-value"></span>`fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-end-array-value"></span>`fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-begin-object"></span>`fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-end-object"></span>`fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-begin-object-key"></span>`fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-begin-object-value"></span>`fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

- <span id="prettyformatter-end-object-value"></span>`fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>` — [`Result`](../io/index.md#result)

## Enums

### `CharEscape`

```rust
enum CharEscape {
    Quote,
    ReverseSolidus,
    Solidus,
    Backspace,
    FormFeed,
    LineFeed,
    CarriageReturn,
    Tab,
    AsciiControl(u8),
}
```

*Defined in [`serde_json-1.0.145/src/ser.rs:1517-1537`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L1517-L1537)*

Represents a character escape code in a type-safe manner.

#### Variants

- **`Quote`**

  An escaped quote `"`

- **`ReverseSolidus`**

  An escaped reverse solidus `\`

- **`Solidus`**

  An escaped solidus `/`

- **`Backspace`**

  An escaped backspace character (usually escaped as `\b`)

- **`FormFeed`**

  An escaped form feed character (usually escaped as `\f`)

- **`LineFeed`**

  An escaped line feed character (usually escaped as `\n`)

- **`CarriageReturn`**

  An escaped carriage return character (usually escaped as `\r`)

- **`Tab`**

  An escaped tab character (usually escaped as `\t`)

- **`AsciiControl`**

  An escaped ASCII plane control character (usually escaped as
  `\u00XX` where `XX` are two hex characters)

## Traits

### `Formatter`

```rust
trait Formatter { ... }
```

*Defined in [`serde_json-1.0.145/src/ser.rs:1541-1935`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L1541-L1935)*

This trait abstracts away serializing the JSON control characters, which allows the user to
optionally pretty print the JSON output.

#### Provided Methods

- `fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Writes a `null` value to the specified writer.

- `fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>`

  Writes a `true` or `false` value to the specified writer.

- `fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>`

  Writes a floating point value like `-31.26e+12` to the specified writer.

- `fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>`

  Writes a floating point value like `-31.26e+12` to the specified writer.

- `fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>`

  Writes a number that has already been rendered to a string.

- `fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called before each series of `write_string_fragment` and

- `fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called after each series of `write_string_fragment` and

- `fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>`

  Writes a string fragment that doesn't need any escaping to the

- `fn write_char_escape<W>(&mut self, writer: &mut W, char_escape: CharEscape) -> io::Result<()>`

  Writes a character escape code to the specified writer.

- `fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> io::Result<()>`

  Writes the representation of a byte array. Formatters can choose whether

- `fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called before every array.  Writes a `[` to the specified

- `fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called after every array.  Writes a `]` to the specified

- `fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>`

  Called before every array value.  Writes a `,` if needed to

- `fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>`

  Called after every array value.

- `fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called before every object.  Writes a `{` to the specified

- `fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called after every object.  Writes a `}` to the specified

- `fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>`

  Called before every object key.

- `fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>`

  Called after every object key.  A `:` should be written to the

- `fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>`

  Called before every object value.  A `:` should be written to

- `fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>`

  Called after every object value.

- `fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>`

  Writes a raw JSON fragment that doesn't need any escaping to the

#### Implementors

- [`CompactFormatter`](#compactformatter)
- [`PrettyFormatter`](#prettyformatter)

## Functions

### `key_must_be_a_string`

```rust
fn key_must_be_a_string() -> crate::error::Error
```

*Defined in [`serde_json-1.0.145/src/ser.rs:787-789`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L787-L789)*

### `float_key_must_be_finite`

```rust
fn float_key_must_be_finite() -> crate::error::Error
```

*Defined in [`serde_json-1.0.145/src/ser.rs:791-793`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L791-L793)*

### `format_escaped_str`

```rust
fn format_escaped_str<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2069-2077`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2069-L2077)*

### `format_escaped_str_contents`

```rust
fn format_escaped_str_contents<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2079-2133`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2079-L2133)*

### `to_writer`

```rust
fn to_writer<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2177-2184`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2177-L2184)*

Serialize the given data structure as JSON into the I/O stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_writer_pretty`

```rust
fn to_writer_pretty<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2197-2204`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2197-L2204)*

Serialize the given data structure as pretty-printed JSON into the I/O
stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_vec`

```rust
fn to_vec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2213-2220`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2213-L2220)*

Serialize the given data structure as a JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_vec_pretty`

```rust
fn to_vec_pretty<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2229-2236`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2229-L2236)*

Serialize the given data structure as a pretty-printed JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_string`

```rust
fn to_string<T>(value: &T) -> crate::error::Result<alloc::string::String>
where
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2245-2255`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2245-L2255)*

Serialize the given data structure as a String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_string_pretty`

```rust
fn to_string_pretty<T>(value: &T) -> crate::error::Result<alloc::string::String>
where
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2264-2274`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2264-L2274)*

Serialize the given data structure as a pretty-printed String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `indent`

```rust
fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2276-2285`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2276-L2285)*

## Constants

### `BB`
```rust
const BB: u8 = 98u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2135`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2135)*

### `TT`
```rust
const TT: u8 = 116u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2136`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2136)*

### `NN`
```rust
const NN: u8 = 110u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2137`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2137)*

### `FF`
```rust
const FF: u8 = 102u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2138`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2138)*

### `RR`
```rust
const RR: u8 = 114u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2139`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2139)*

### `QU`
```rust
const QU: u8 = 34u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2140`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2140)*

### `BS`
```rust
const BS: u8 = 92u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2141`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2141)*

### `UU`
```rust
const UU: u8 = 117u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2142`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2142)*

### `__`
```rust
const __: u8 = 0u8;
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2143`](../../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2143)*

