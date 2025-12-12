# Crate `serde_json`

JSON is a ubiquitous open-standard format that uses human-readable text to
transmit data objects consisting of key-value pairs.

```json
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
```

There are three common ways that you might find yourself needing to work
with JSON data in Rust.

 - **As text data.** An unprocessed string of JSON data that you receive on
   an HTTP endpoint, read from a file, or prepare to send to a remote
   server.
 - **As an untyped or loosely typed representation.** Maybe you want to
   check that some JSON data is valid before passing it on, but without
   knowing the structure of what it contains. Or you want to do very basic
   manipulations like insert a key in a particular spot.
 - **As a strongly typed Rust data structure.** When you expect all or most
   of your data to conform to a particular structure and want to get real
   work done without JSON's loosey-goosey nature tripping you up.

Serde JSON provides efficient, flexible, safe ways of converting data
between each of these representations.

# Operating on untyped JSON values

Any valid JSON data can be manipulated in the following recursive enum
representation. This data structure is [`serde_json::Value`][`value`](value/index.md).

```rust
use serde_json::{Number, Map};

#[allow(dead_code)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][`from_str`](de/index.md) function. There is also [`from_slice`](de/index.md)
for parsing from a byte slice `&[u8]` and [`from_reader`](de/index.md) for parsing from
any `io::Read` like a File or a TCP stream.

```rust
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
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

fn main() {
    untyped_example().unwrap();
}
```

The result of square bracket indexing like `v["name"]` is a borrow of the
data at that index, so the type is `&Value`. A JSON map can be indexed with
string keys, while a JSON array can be indexed with integer keys. If the
type of the data is not right for the type with which it is being indexed,
or if a map does not contain the key being indexed, or if the index into a
vector is out of bounds, the returned element is `Value::Null`.

When a `Value` is printed, it is printed as a JSON string. So in the code
above, the output looks like `Please call "John Doe" at the number "+44
1234567"`. The quotation marks appear because `v["name"]` is a `&Value`
containing a JSON string and its JSON representation is `"John Doe"`.
Printing as a plain string without quotation marks involves converting from
a JSON string to a Rust string with `as_str()` or avoiding the use of
`Value` as described in the following section.

The `Value` representation is sufficient for very basic tasks but can be
tedious to work with for anything more significant. Error handling is
verbose to implement correctly, for example imagine trying to detect the
presence of unrecognized fields in the input data. The compiler is powerless
to help you when you make a mistake, for example imagine typoing `v["name"]`
as `v["nmae"]` in one of the dozens of places it is used in your code.

# Parsing JSON as strongly typed data structures

Serde provides a powerful way of mapping JSON data into Rust data structures
largely automatically.

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
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

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn main() {
    typed_example().unwrap();
}
```

This is the same `serde_json::from_str` function as before, but this time we
assign the return value to a variable of type `Person` so Serde will
automatically interpret the input data as a `Person` and produce informative
error messages if the layout does not conform to what a `Person` is expected
to look like.

Any type that implements Serde's `Deserialize` trait can be deserialized
this way. This includes built-in Rust standard library types like `Vec<T>`
and `HashMap<K, V>`, as well as any structs or enums annotated with
`#[derive(Deserialize)]`.

Once we have `p` of type `Person`, our IDE and the Rust compiler can help us
use it correctly like they do for any other Rust code. The IDE can
autocomplete field names to prevent typos, which was impossible in the
`serde_json::Value` representation. And the Rust compiler can check that
when we write `p.phones[0]`, then `p.phones` is guaranteed to be a
`Vec<String>` so indexing into it makes sense and produces a `String`.

# Constructing JSON values

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

This is amazingly convenient, but we have the problem we had before with
`Value`: the IDE and Rust compiler cannot help us if we get it wrong. Serde
JSON provides a better way of serializing strongly-typed data structures
into JSON text.

# Creating JSON by serializing data structures

A data structure can be converted to a JSON string by
[`serde_json::to_string`][`to_string`](ser/index.md). There is also
[`serde_json::to_vec`][`to_vec`](ser/index.md) which serializes to a `Vec<u8>` and
[`serde_json::to_writer`][`to_writer`](ser/index.md) which serializes to any `io::Write`
such as a File or a TCP stream.

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}

fn main() {
    print_an_address().unwrap();
}
```

Any type that implements Serde's `Serialize` trait can be serialized this
way. This includes built-in Rust standard library types like `Vec<T>` and
`HashMap<K, V>`, as well as any structs or enums annotated with
`#[derive(Serialize)]`.

# No-std support

As long as there is a memory allocator, it is possible to use serde_json
without the rest of the Rust standard library. Disable the default "std"
feature and enable the "alloc" feature:

```toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

For JSON support in Serde without a memory allocator, please see the
`serde-json-core` crate.










## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`de`](#de)
  - [`error`](#error)
  - [`map`](#map)
  - [`ser`](#ser)
  - [`value`](#value)
  - [`io`](#io)
  - [`iter`](#iter)
  - [`number`](#number)
  - [`read`](#read)
- [Structs](#structs)
  - [`Deserializer`](#deserializer)
  - [`StreamDeserializer`](#streamdeserializer)
  - [`Error`](#error)
  - [`Serializer`](#serializer)
  - [`Map`](#map)
  - [`Number`](#number)
- [Enums](#enums)
  - [`Value`](#value)
- [Functions](#functions)
  - [`from_reader`](#from-reader)
  - [`from_slice`](#from-slice)
  - [`from_str`](#from-str)
  - [`to_string`](#to-string)
  - [`to_string_pretty`](#to-string-pretty)
  - [`to_vec`](#to-vec)
  - [`to_vec_pretty`](#to-vec-pretty)
  - [`to_writer`](#to-writer)
  - [`to_writer_pretty`](#to-writer-pretty)
  - [`from_value`](#from-value)
  - [`to_value`](#to-value)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`tri!`](#tri)
  - [`json!`](#json)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`de`](#de) | mod | Deserialize JSON data to a Rust data structure. |
| [`error`](#error) | mod | When serializing or deserializing JSON goes wrong. |
| [`map`](#map) | mod | A map of String to serde_json::Value. |
| [`ser`](#ser) | mod | Serialize a Rust data structure into JSON data. |
| [`value`](#value) | mod | The Value enum, a loosely typed way of representing any valid JSON value. |
| [`io`](#io) | mod | A tiny, `no_std`-friendly facade around `std::io`. |
| [`iter`](#iter) | mod |  |
| [`number`](#number) | mod |  |
| [`read`](#read) | mod |  |
| [`Deserializer`](#deserializer) | struct |  |
| [`StreamDeserializer`](#streamdeserializer) | struct |  |
| [`Error`](#error) | struct |  |
| [`Serializer`](#serializer) | struct |  |
| [`Map`](#map) | struct |  |
| [`Number`](#number) | struct |  |
| [`Value`](#value) | enum |  |
| [`from_reader`](#from-reader) | fn |  |
| [`from_slice`](#from-slice) | fn |  |
| [`from_str`](#from-str) | fn |  |
| [`to_string`](#to-string) | fn |  |
| [`to_string_pretty`](#to-string-pretty) | fn |  |
| [`to_vec`](#to-vec) | fn |  |
| [`to_vec_pretty`](#to-vec-pretty) | fn |  |
| [`to_writer`](#to-writer) | fn |  |
| [`to_writer_pretty`](#to-writer-pretty) | fn |  |
| [`from_value`](#from-value) | fn |  |
| [`to_value`](#to-value) | fn |  |
| [`Result`](#result) | type |  |
| [`tri!`](#tri) | macro |  |
| [`json!`](#json) | macro | Construct a `serde_json::Value` from a JSON literal. |

## Modules

- [`macros`](macros/index.md)
- [`de`](de/index.md) — Deserialize JSON data to a Rust data structure.
- [`error`](error/index.md) — When serializing or deserializing JSON goes wrong.
- [`map`](map/index.md) — A map of String to serde_json::Value.
- [`ser`](ser/index.md) — Serialize a Rust data structure into JSON data.
- [`value`](value/index.md) — The Value enum, a loosely typed way of representing any valid JSON value.
- [`io`](io/index.md) — A tiny, `no_std`-friendly facade around `std::io`.
- [`iter`](iter/index.md)
- [`number`](number/index.md)
- [`read`](read/index.md)

## Structs

### `Deserializer<R>`

```rust
struct Deserializer<R> {
    read: R,
    scratch: alloc::vec::Vec<u8>,
    remaining_depth: u8,
    disable_recursion_limit: bool,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:31-39`](../../.source_1765521767/serde_json-1.0.145/src/de.rs#L31-L39)*

A structure that deserializes JSON into Rust values.

#### Implementations

- <span id="deserializer-new"></span>`fn new(read: R) -> Self`

#### Trait Implementations

##### `impl<R: Read<'de>> Deserializer for &mut Deserializer<R>`

- <span id="mut-deserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

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

*Defined in [`serde_json-1.0.145/src/de.rs:2349-2355`](../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2349-L2355)*

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

- <span id="streamdeserializer-new"></span>`fn new(read: R) -> Self`

- <span id="streamdeserializer-byte-offset"></span>`fn byte_offset(&self) -> usize`

- <span id="streamdeserializer-peek-end-of-value"></span>`fn peek_end_of_value(&mut self) -> Result<()>` — [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl<R, T> FusedIterator for StreamDeserializer<'de, R, T>`

##### `impl IntoIterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamdeserializer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamdeserializer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R, T> Iterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-iterator-type-item"></span>`type Item = Result<T, Error>`

- <span id="streamdeserializer-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](error/index.md#result)

### `Error`

```rust
struct Error {
    err: alloc::boxed::Box<ErrorImpl>,
}
```

*Defined in [`serde_json-1.0.145/src/error.rs:17-22`](../../.source_1765521767/serde_json-1.0.145/src/error.rs#L17-L22)*

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

#### Fields

- **`err`**: `alloc::boxed::Box<ErrorImpl>`

  This `Box` allows us to keep the size of `Error` as small as possible. A
  larger `Error` type was substantially slower due to all the functions
  that pass around `Result<T, Error>`.

#### Implementations

- <span id="error-line"></span>`fn line(&self) -> usize`

- <span id="error-column"></span>`fn column(&self) -> usize`

- <span id="error-classify"></span>`fn classify(&self) -> Category` — [`Category`](error/index.md#category)

- <span id="error-is-io"></span>`fn is_io(&self) -> bool`

- <span id="error-is-syntax"></span>`fn is_syntax(&self) -> bool`

- <span id="error-is-data"></span>`fn is_data(&self) -> bool`

- <span id="error-is-eof"></span>`fn is_eof(&self) -> bool`

- <span id="error-io-error-kind"></span>`fn io_error_kind(&self) -> Option<ErrorKind>` — [`ErrorKind`](io/index.md#errorkind)

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-intodeserializer-type-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    writer: W,
    formatter: F,
}
```

*Defined in [`serde_json-1.0.145/src/ser.rs:17-20`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L17-L20)*

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

- <span id="a-mut-serializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-f32"></span>`fn serialize_f32(self, value: f32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-f64"></span>`fn serialize_f64(self, value: f64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-bytes"></span>`fn serialize_bytes(self, value: &[u8]) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-none"></span>`fn serialize_none(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-tuple"></span>`fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-struct"></span>`fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:29-31`](../../.source_1765521767/serde_json-1.0.145/src/map.rs#L29-L31)*

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="map-clear"></span>`fn clear(&mut self)`

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](value/index.md#value)

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](value/index.md#value)

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](value/index.md#value)

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](value/index.md#value)

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](value/index.md#value)

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](value/index.md#value)

- <span id="map-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](map/index.md#entry)

- <span id="map-len"></span>`fn len(&self) -> usize`

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](map/index.md#iter)

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](map/index.md#itermut)

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](map/index.md#keys)

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](map/index.md#values)

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](map/index.md#valuesmut)

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](map/index.md#intovalues)

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

##### `impl Deserialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Map<K, V>`

##### `impl Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-deserializer-type-error"></span>`type Error = Error`

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

- <span id="cratemapmap-fromstr-type-err"></span>`type Err = Error`

- <span id="cratemapmap-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](error/index.md#error)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-type-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](value/index.md#value)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](value/index.md#value)

##### `impl IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-intodeserializer-type-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl IntoIterator for &'a Map<alloc::string::String, crate::value::Value>`

- <span id="a-map-intoiterator-type-item"></span>`type Item = (&'a String, &'a Value)`

- <span id="a-map-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-map-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

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

*Defined in [`serde_json-1.0.145/src/number.rs:22-24`](../../.source_1765521767/serde_json-1.0.145/src/number.rs#L22-L24)*

Represents a JSON number, whether integer or floating point.

#### Implementations

- <span id="number-is-i64"></span>`fn is_i64(&self) -> bool`

- <span id="number-is-u64"></span>`fn is_u64(&self) -> bool`

- <span id="number-is-f64"></span>`fn is_f64(&self) -> bool`

- <span id="number-as-i64"></span>`fn as_i64(&self) -> Option<i64>`

- <span id="number-as-u64"></span>`fn as_u64(&self) -> Option<u64>`

- <span id="number-as-f64"></span>`fn as_f64(&self) -> Option<f64>`

- <span id="number-from-f64"></span>`fn from_f64(f: f64) -> Option<Number>` — [`Number`](number/index.md#number)

- <span id="number-as-i128"></span>`fn as_i128(&self) -> Option<i128>`

- <span id="number-as-u128"></span>`fn as_u128(&self) -> Option<u128>`

- <span id="number-from-i128"></span>`fn from_i128(i: i128) -> Option<Number>` — [`Number`](number/index.md#number)

- <span id="number-from-u128"></span>`fn from_u128(i: u128) -> Option<Number>` — [`Number`](number/index.md#number)

- <span id="number-as-f32"></span>`fn as_f32(&self) -> Option<f32>`

- <span id="number-from-f32"></span>`fn from_f32(f: f32) -> Option<Number>` — [`Number`](number/index.md#number)

#### Trait Implementations

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](number/index.md#number)

##### `impl Debug for Number`

- <span id="number-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](number/index.md#number)

##### `impl DeserializeOwned for Number`

##### `impl Deserializer for Number`

- <span id="number-deserializer-type-error"></span>`type Error = Error`

- <span id="number-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

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

- <span id="cratenumbernumber-fromstr-type-err"></span>`type Err = Error`

- <span id="cratenumbernumber-from-str"></span>`fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- <span id="number-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Number`

- <span id="number-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](number/index.md#number)

##### `impl Serialize for Number`

- <span id="number-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl ToString for Number`

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

*Defined in [`serde_json-1.0.145/src/value/mod.rs:116-176`](../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L116-L176)*

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

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](value/index.md#value)

##### `impl Debug for Value`

- <span id="value-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Value`

- <span id="value-default"></span>`fn default() -> Value` — [`Value`](value/index.md#value)

##### `impl Deserialize for crate::value::Value`

- <span id="cratevaluevalue-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](value/index.md#value)

##### `impl DeserializeOwned for Value`

##### `impl Deserializer for crate::value::Value`

- <span id="cratevaluevalue-deserializer-type-error"></span>`type Error = Error`

- <span id="cratevaluevalue-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

##### `impl Display for Value`

- <span id="value-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Value`

##### `impl<T: Into<super::Value>> FromIterator for super::Value`

- <span id="supervalue-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl FromStr for crate::value::Value`

- <span id="cratevaluevalue-fromstr-type-err"></span>`type Err = Error`

- <span id="cratevaluevalue-from-str"></span>`fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](value/index.md#value), [`Error`](error/index.md#error)

##### `impl Hash for Value`

- <span id="value-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> Index for super::Value`

- <span id="supervalue-index-type-output"></span>`type Output = Value`

- <span id="supervalue-index"></span>`fn index(&self, index: I) -> &Value` — [`Value`](value/index.md#value)

##### `impl<I> IndexMut for super::Value`

- <span id="supervalue-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut Value` — [`Value`](value/index.md#value)

##### `impl IntoDeserializer for crate::value::Value`

- <span id="cratevaluevalue-intodeserializer-type-deserializer"></span>`type Deserializer = Value`

- <span id="cratevaluevalue-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl PartialEq for Value`

- <span id="value-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](value/index.md#value)

##### `impl Serialize for crate::value::Value`

- <span id="cratevaluevalue-serialize"></span>`fn serialize<S>(&self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Value`

##### `impl ToString for Value`

- <span id="value-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `from_reader`

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned
```

*Defined in [`serde_json-1.0.145/src/de.rs:2612-2618`](../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2612-L2618)*

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a `File`, you will want to apply your own buffering because serde_json
will not buffer the input. See `std::io::BufReader`.

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`](de/index.md).

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`](de/index.md)
or [`from_slice`](de/index.md) on it. See [issue #160].


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

*Defined in [`serde_json-1.0.145/src/de.rs:2655-2660`](../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2655-L2660)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:2697-2702`](../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2697-L2702)*

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

### `to_string`

```rust
fn to_string<T>(value: &T) -> crate::error::Result<alloc::string::String>
where
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2245-2255`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2245-L2255)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2264-2274`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2264-L2274)*

Serialize the given data structure as a pretty-printed String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_vec`

```rust
fn to_vec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2213-2220`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2213-L2220)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2229-2236`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2229-L2236)*

Serialize the given data structure as a pretty-printed JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `to_writer`

```rust
fn to_writer<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize
```

*Defined in [`serde_json-1.0.145/src/ser.rs:2177-2184`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2177-L2184)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2197-2204`](../../.source_1765521767/serde_json-1.0.145/src/ser.rs#L2197-L2204)*

Serialize the given data structure as pretty-printed JSON into the I/O
stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

### `from_value`

```rust
fn from_value<T>(value: Value) -> Result<T, crate::error::Error>
where
    T: DeserializeOwned
```

*Defined in [`serde_json-1.0.145/src/value/mod.rs:1037-1042`](../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L1037-L1042)*

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

### `to_value`

```rust
fn to_value<T>(value: T) -> Result<Value, crate::error::Error>
where
    T: Serialize
```

*Defined in [`serde_json-1.0.145/src/value/mod.rs:995-1000`](../../.source_1765521767/serde_json-1.0.145/src/value/mod.rs#L995-L1000)*

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

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

*Defined in [`serde_json-1.0.145/src/error.rs:25`](../../.source_1765521767/serde_json-1.0.145/src/error.rs#L25)*

Alias for a `Result` with the error type `serde_json::Error`.

## Macros

### `tri!`

*Defined in [`serde_json-1.0.145/src/lib.rs:410-417`](../../.source_1765521767/serde_json-1.0.145/src/lib.rs#L410-L417)*

### `json!`

*Defined in [`serde_json-1.0.145/src/macros.rs:54-59`](../../.source_1765521767/serde_json-1.0.145/src/macros.rs#L54-L59)*

Construct a `serde_json::Value` from a JSON literal.

```rust
use serde_json::json;

let value = json!({
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "serde",
            "json"
        ],
        "homepage": null
    }
});
```

Variables or expressions can be interpolated into the JSON literal. Any type
interpolated into an array element or object value must implement Serde's
`Serialize` trait, while any type interpolated into a object key must
implement `Into<String>`. If the `Serialize` implementation of the
interpolated type decides to fail, or if the interpolated type contains a
map with non-string keys, the `json!` macro will panic.

```rust
use serde_json::json;

let code = 200;
let features = vec!["serde", "json"];

let value = json!({
    "code": code,
    "success": code == 200,
    "payload": {
        features[0]: features[1]
    }
});
```

Trailing commas are allowed inside both arrays and objects.

```rust
use serde_json::json;

let value = json!([
    "notice",
    "the",
    "trailing",
    "comma -->",
]);
```

