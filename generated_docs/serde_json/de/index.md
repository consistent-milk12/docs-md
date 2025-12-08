*[serde_json](../index.md) / [de](index.md)*

---

# Module `de`

Deserialize JSON data to a Rust data structure.

## Structs

### `SliceRead<'a>`

```rust
struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}
```

JSON input source that reads from a slice of bytes.

#### Fields

- **`index`**: `usize`

  Index of the *next* byte that will be returned by next() or peek().

#### Implementations

- `fn new(slice: &'a [u8]) -> Self`

- `fn position_of_index(self: &Self, i: usize) -> Position` — [`Position`](../read/index.md)

- `fn skip_to_escape(self: &mut Self, forbid_control_characters: bool)`

- `fn skip_to_escape_slow(self: &mut Self)`

- `fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<Reference<'a, 's, T>>` — [`Result`](../index.md), [`Reference`](../read/index.md)

#### Trait Implementations

##### `impl<'a> Fused for SliceRead<'a>`

##### `impl<'a> Read for SliceRead<'a>`

##### `impl<'a> Sealed for SliceRead<'a>`

### `StrRead<'a>`

```rust
struct StrRead<'a> {
    delegate: SliceRead<'a>,
}
```

JSON input source that reads from a UTF-8 string.

#### Implementations

- `fn new(s: &'a str) -> Self`

#### Trait Implementations

##### `impl<'a> Fused for StrRead<'a>`

##### `impl<'a> Read for StrRead<'a>`

##### `impl<'a> Sealed for StrRead<'a>`

### `IoRead<R>`

```rust
struct IoRead<R>
where
    R: io::Read {
    iter: crate::iter::LineColIterator<io::Bytes<R>>,
    ch: Option<u8>,
}
```

JSON input source that reads from a std::io input stream.

#### Fields

- **`ch`**: `Option<u8>`

  Temporary storage of peeked byte.

#### Implementations

- `fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<T>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<'de, R> Read for IoRead<R>`

##### `impl<R> Sealed for IoRead<R>`

### `Deserializer<R>`

```rust
struct Deserializer<R> {
    read: R,
    scratch: alloc::vec::Vec<u8>,
    remaining_depth: u8,
}
```

A structure that deserializes JSON into Rust values.

#### Implementations

- `fn end(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn into_iter<T>(self: Self) -> StreamDeserializer<'de, R, T>` — [`StreamDeserializer`](../index.md)

- `fn peek(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../index.md)

- `fn peek_or_null(self: &mut Self) -> Result<u8>` — [`Result`](../index.md)

- `fn eat_char(self: &mut Self)`

- `fn next_char(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../index.md)

- `fn next_char_or_null(self: &mut Self) -> Result<u8>` — [`Result`](../index.md)

- `fn error(self: &Self, reason: ErrorCode) -> Error` — [`ErrorCode`](../error/index.md), [`Error`](../index.md)

- `fn peek_error(self: &Self, reason: ErrorCode) -> Error` — [`ErrorCode`](../error/index.md), [`Error`](../index.md)

- `fn parse_whitespace(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../index.md)

- `fn peek_invalid_type(self: &mut Self, exp: &dyn Expected) -> Error` — [`Error`](../index.md)

- `fn deserialize_number<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn do_deserialize_i128<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn do_deserialize_u128<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn scan_integer128(self: &mut Self, buf: &mut String) -> Result<()>` — [`Result`](../index.md)

- `fn fix_position(self: &Self, err: Error) -> Error` — [`Error`](../index.md)

- `fn parse_ident(self: &mut Self, ident: &[u8]) -> Result<()>` — [`Result`](../index.md)

- `fn parse_integer(self: &mut Self, positive: bool) -> Result<ParserNumber>` — [`Result`](../index.md), [`ParserNumber`](#parsernumber)

- `fn parse_number(self: &mut Self, positive: bool, significand: u64) -> Result<ParserNumber>` — [`Result`](../index.md), [`ParserNumber`](#parsernumber)

- `fn parse_decimal(self: &mut Self, positive: bool, significand: u64, exponent_before_decimal_point: i32) -> Result<f64>` — [`Result`](../index.md)

- `fn parse_exponent(self: &mut Self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64>` — [`Result`](../index.md)

- `fn f64_from_parts(self: &mut Self, positive: bool, significand: u64, exponent: i32) -> Result<f64>` — [`Result`](../index.md)

- `fn parse_long_integer(self: &mut Self, positive: bool, significand: u64) -> Result<f64>` — [`Result`](../index.md)

- `fn parse_decimal_overflow(self: &mut Self, positive: bool, significand: u64, exponent: i32) -> Result<f64>` — [`Result`](../index.md)

- `fn parse_exponent_overflow(self: &mut Self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64>` — [`Result`](../index.md)

- `fn parse_any_signed_number(self: &mut Self) -> Result<ParserNumber>` — [`Result`](../index.md), [`ParserNumber`](#parsernumber)

- `fn parse_any_number(self: &mut Self, positive: bool) -> Result<ParserNumber>` — [`Result`](../index.md), [`ParserNumber`](#parsernumber)

- `fn parse_object_colon(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn end_seq(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn end_map(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn ignore_value(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn ignore_integer(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn ignore_decimal(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn ignore_exponent(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

### `SeqAccess<'a, R: 'a>`

```rust
struct SeqAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
```

#### Implementations

- `fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](../index.md)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> SeqAccess for SeqAccess<'a, R>`

- `type Error = Error`

- `fn next_element_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>>` — [`Result`](../index.md)

### `MapAccess<'a, R: 'a>`

```rust
struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
```

#### Implementations

- `fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](../index.md)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> MapAccess for MapAccess<'a, R>`

- `type Error = Error`

- `fn next_key_seed<K>(self: &mut Self, seed: K) -> Result<Option<<K as >::Value>>` — [`Result`](../index.md)

- `fn next_value_seed<V>(self: &mut Self, seed: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

### `VariantAccess<'a, R: 'a>`

```rust
struct VariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

#### Implementations

- `fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](../index.md)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> EnumAccess for VariantAccess<'a, R>`

- `type Error = Error`

- `type Variant = VariantAccess<'a, R>`

- `fn variant_seed<V>(self: Self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../index.md)

##### `impl<'de, 'a, R: Read<'de> + 'a> VariantAccess for VariantAccess<'a, R>`

- `type Error = Error`

- `fn unit_variant(self: Self) -> Result<()>` — [`Result`](../index.md)

- `fn newtype_variant_seed<T>(self: Self, seed: T) -> Result<<T as >::Value>` — [`Result`](../index.md)

- `fn tuple_variant<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn struct_variant<V>(self: Self, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

### `UnitVariantAccess<'a, R: 'a>`

```rust
struct UnitVariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

#### Implementations

- `fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](../index.md)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> EnumAccess for UnitVariantAccess<'a, R>`

- `type Error = Error`

- `type Variant = UnitVariantAccess<'a, R>`

- `fn variant_seed<V>(self: Self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../index.md)

##### `impl<'de, 'a, R: Read<'de> + 'a> VariantAccess for UnitVariantAccess<'a, R>`

- `type Error = Error`

- `fn unit_variant(self: Self) -> Result<()>` — [`Result`](../index.md)

- `fn newtype_variant_seed<T>(self: Self, _seed: T) -> Result<<T as >::Value>` — [`Result`](../index.md)

- `fn tuple_variant<V>(self: Self, _len: usize, _visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn struct_variant<V>(self: Self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

### `MapKey<'a, R: 'a>`

```rust
struct MapKey<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

Only deserialize from this after peeking a '"' byte! Otherwise it may
deserialize invalid JSON successfully.

#### Implementations

- `fn deserialize_number<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<'de, 'a, R> Deserializer for MapKey<'a, R>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

### `StreamDeserializer<'de, R, T>`

```rust
struct StreamDeserializer<'de, R, T> {
    de: Deserializer<R>,
    offset: usize,
    failed: bool,
    output: core::marker::PhantomData<T>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
```

Iterator that deserializes a stream into multiple JSON values.

A stream deserializer can be created from any JSON deserializer using the
`Deserializer::into_iter` method.

The data can consist of any JSON value. Values need to be a self-delineating value e.g.
arrays, objects, or strings, or be followed by whitespace or a self-delineating value.

```rust
use serde_json::{Deserializer, Value};

fn main() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";

    let stream = Deserializer::from_str(data).into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }
}
```

#### Implementations

- `fn new(read: R) -> Self`

- `fn byte_offset(self: &Self) -> usize`

- `fn peek_end_of_value(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<'de, R, T> FusedIterator for StreamDeserializer<'de, R, T>`

##### `impl<I> IntoIterator for StreamDeserializer<'de, R, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'de, R, T> Iterator for StreamDeserializer<'de, R, T>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<Result<T>>` — [`Result`](../index.md)

## Enums

### `ParserNumber`

```rust
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
}
```

#### Implementations

- `fn visit<'de, V>(self: Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../index.md)

- `fn invalid_type(self: Self, exp: &dyn Expected) -> Error` — [`Error`](../index.md)

## Traits

## Functions

### `from_trait`

```rust
fn from_trait<'de, R, T>(read: R) -> crate::error::Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>
```

### `from_reader`

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned
```

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a `File`, you will want to apply your own buffering because serde_json
will not buffer the input. See `std::io::BufReader`.

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`](../index.md).

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`](../index.md)
or [`from_slice`](../index.md) on it. See [issue #160].


# Example

Reading the contents of a file.

```rust
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
}
fn fake_main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}
```

Reading from a persistent socket connection.

```rust
use serde::Deserialize;

use std::error::Error;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_stream(stream: &mut BufReader<TcpStream>) -> Result<User, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = User::deserialize(&mut de)?;

    Ok(u)
}

fn main() {
}
fn fake_main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for tcp_stream in listener.incoming() {
        let mut buffered = BufReader::new(tcp_stream.unwrap());
        println!("{:#?}", read_user_from_stream(&mut buffered));
    }
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

### `from_slice`

```rust
fn from_slice<'a, T>(v: &'a [u8]) -> crate::error::Result<T>
where
    T: de::Deserialize<'a>
```

Deserialize an instance of type `T` from bytes of JSON text.

# Example

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&[u8]`
    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_slice(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

### `from_str`

```rust
fn from_str<'a, T>(s: &'a str) -> crate::error::Result<T>
where
    T: de::Deserialize<'a>
```

Deserialize an instance of type `T` from a string of JSON text.

# Example

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

## Macros

### `overflow!`

### `deserialize_number!`

### `if_checking_recursion_limit!`

### `check_recursion!`

### `deserialize_numeric_key!`

