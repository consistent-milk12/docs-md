*[serde_json](../index.md) / [ser](index.md)*

---

# Module `ser`

Serialize a Rust data structure into JSON data.

## Structs

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    writer: W,
    formatter: F,
}
```

A structure for serializing Rust values into JSON.

#### Implementations

- `fn with_formatter(writer: W, formatter: F) -> Self`

- `fn into_inner(self: Self) -> W`

### `MapKeySerializer<'a, W: 'a, F: 'a>`

```rust
struct MapKeySerializer<'a, W: 'a, F: 'a> {
    ser: &'a mut Serializer<W, F>,
}
```

#### Trait Implementations

##### `impl<'a, W, F> Serializer for MapKeySerializer<'a, W, F>`

- `type Ok = ()`

- `type Error = Error`

- `fn serialize_str(self: Self, value: &str) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_unit_variant(self: Self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_newtype_struct<T>(self: Self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](../error/index.md)

- `type SerializeSeq = Impossible<(), Error>`

- `type SerializeTuple = Impossible<(), Error>`

- `type SerializeTupleStruct = Impossible<(), Error>`

- `type SerializeTupleVariant = Impossible<(), Error>`

- `type SerializeMap = Impossible<(), Error>`

- `type SerializeStruct = Impossible<(), Error>`

- `type SerializeStructVariant = Impossible<(), Error>`

- `fn serialize_bool(self: Self, value: bool) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_i8(self: Self, value: i8) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_i16(self: Self, value: i16) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_i32(self: Self, value: i32) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_i64(self: Self, value: i64) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_i128(self: Self, value: i128) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_u8(self: Self, value: u8) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_u16(self: Self, value: u16) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_u32(self: Self, value: u32) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_u64(self: Self, value: u64) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_u128(self: Self, value: u128) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_f32(self: Self, value: f32) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_f64(self: Self, value: f64) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_char(self: Self, value: char) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_bytes(self: Self, _value: &[u8]) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_unit(self: Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_unit_struct(self: Self, _name: &'static str) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_newtype_variant<T>(self: Self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_none(self: Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_some<T>(self: Self, value: &T) -> Result<()>` — [`Result`](../error/index.md)

- `fn serialize_seq(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](../error/index.md)

- `fn serialize_tuple(self: Self, _len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](../error/index.md)

- `fn serialize_tuple_struct(self: Self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](../error/index.md)

- `fn serialize_tuple_variant(self: Self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](../error/index.md)

- `fn serialize_map(self: Self, _len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](../error/index.md)

- `fn serialize_struct(self: Self, _name: &'static str, _len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](../error/index.md)

- `fn serialize_struct_variant(self: Self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](../error/index.md)

- `fn collect_str<T>(self: Self, value: &T) -> Result<()>` — [`Result`](../error/index.md)

### `CompactFormatter`

```rust
struct CompactFormatter;
```

This structure compacts a JSON value with no extra whitespace.

#### Trait Implementations

##### `impl Clone for CompactFormatter`

- `fn clone(self: &Self) -> CompactFormatter` — [`CompactFormatter`](#compactformatter)

##### `impl Debug for CompactFormatter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for CompactFormatter`

- `fn default() -> CompactFormatter` — [`CompactFormatter`](#compactformatter)

##### `impl Formatter for CompactFormatter`

### `PrettyFormatter<'a>`

```rust
struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}
```

This structure pretty prints a JSON value to make it human readable.

#### Implementations

- `fn new() -> Self`

- `fn with_indent(indent: &'a [u8]) -> Self`

#### Trait Implementations

##### `impl<'a> Clone for PrettyFormatter<'a>`

- `fn clone(self: &Self) -> PrettyFormatter<'a>` — [`PrettyFormatter`](#prettyformatter)

##### `impl<'a> Debug for PrettyFormatter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> Default for PrettyFormatter<'a>`

- `fn default() -> Self`

##### `impl<'a> Formatter for PrettyFormatter<'a>`

- `fn begin_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn end_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn begin_array_value<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>`

- `fn end_array_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

- `fn begin_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn end_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn begin_object_key<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>`

- `fn begin_object_value<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn end_object_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

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

This trait abstracts away serializing the JSON control characters, which allows the user to
optionally pretty print the JSON output.

#### Required Methods

- `fn write_null<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Writes a `null` value to the specified writer.

- `fn write_bool<W>(self: &mut Self, writer: &mut W, value: bool) -> io::Result<()>`

  Writes a `true` or `false` value to the specified writer.

- `fn write_i8<W>(self: &mut Self, writer: &mut W, value: i8) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i16<W>(self: &mut Self, writer: &mut W, value: i16) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i32<W>(self: &mut Self, writer: &mut W, value: i32) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i64<W>(self: &mut Self, writer: &mut W, value: i64) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_i128<W>(self: &mut Self, writer: &mut W, value: i128) -> io::Result<()>`

  Writes an integer value like `-123` to the specified writer.

- `fn write_u8<W>(self: &mut Self, writer: &mut W, value: u8) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u16<W>(self: &mut Self, writer: &mut W, value: u16) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u32<W>(self: &mut Self, writer: &mut W, value: u32) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u64<W>(self: &mut Self, writer: &mut W, value: u64) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_u128<W>(self: &mut Self, writer: &mut W, value: u128) -> io::Result<()>`

  Writes an integer value like `123` to the specified writer.

- `fn write_f32<W>(self: &mut Self, writer: &mut W, value: f32) -> io::Result<()>`

  Writes a floating point value like `-31.26e+12` to the specified writer.

- `fn write_f64<W>(self: &mut Self, writer: &mut W, value: f64) -> io::Result<()>`

  Writes a floating point value like `-31.26e+12` to the specified writer.

- `fn write_number_str<W>(self: &mut Self, writer: &mut W, value: &str) -> io::Result<()>`

  Writes a number that has already been rendered to a string.

- `fn begin_string<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called before each series of `write_string_fragment` and

- `fn end_string<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called after each series of `write_string_fragment` and

- `fn write_string_fragment<W>(self: &mut Self, writer: &mut W, fragment: &str) -> io::Result<()>`

  Writes a string fragment that doesn't need any escaping to the

- `fn write_char_escape<W>(self: &mut Self, writer: &mut W, char_escape: CharEscape) -> io::Result<()>`

  Writes a character escape code to the specified writer.

- `fn write_byte_array<W>(self: &mut Self, writer: &mut W, value: &[u8]) -> io::Result<()>`

  Writes the representation of a byte array. Formatters can choose whether

- `fn begin_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called before every array.  Writes a `[` to the specified

- `fn end_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called after every array.  Writes a `]` to the specified

- `fn begin_array_value<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>`

  Called before every array value.  Writes a `,` if needed to

- `fn end_array_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

  Called after every array value.

- `fn begin_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called before every object.  Writes a `{` to the specified

- `fn end_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called after every object.  Writes a `}` to the specified

- `fn begin_object_key<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>`

  Called before every object key.

- `fn end_object_key<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

  Called after every object key.  A `:` should be written to the

- `fn begin_object_value<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

  Called before every object value.  A `:` should be written to

- `fn end_object_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

  Called after every object value.

- `fn write_raw_fragment<W>(self: &mut Self, writer: &mut W, fragment: &str) -> io::Result<()>`

  Writes a raw JSON fragment that doesn't need any escaping to the

## Functions

### `key_must_be_a_string`

```rust
fn key_must_be_a_string() -> crate::error::Error
```

### `float_key_must_be_finite`

```rust
fn float_key_must_be_finite() -> crate::error::Error
```

### `format_escaped_str`

```rust
fn format_escaped_str<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter
```

### `format_escaped_str_contents`

```rust
fn format_escaped_str_contents<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter
```

### `to_writer`

```rust
fn to_writer<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize
```

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

## Constants

### `BB`

```rust
const BB: u8 = 98u8;
```

### `TT`

```rust
const TT: u8 = 116u8;
```

### `NN`

```rust
const NN: u8 = 110u8;
```

### `FF`

```rust
const FF: u8 = 102u8;
```

### `RR`

```rust
const RR: u8 = 114u8;
```

### `QU`

```rust
const QU: u8 = 34u8;
```

### `BS`

```rust
const BS: u8 = 92u8;
```

### `UU`

```rust
const UU: u8 = 117u8;
```

### `__`

```rust
const __: u8 = 0u8;
```

