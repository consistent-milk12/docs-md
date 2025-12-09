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
[`serde_json::from_str`][from_str](#from-str) function. There is also [`from_slice`](#from-slice)
for parsing from a byte slice `&[u8]` and [`from_reader`](#from-reader) for parsing from
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
[`serde_json::to_string`][to_string](#to-string). There is also
[`serde_json::to_vec`][to_vec](#to-vec) which serializes to a `Vec<u8>` and
[`serde_json::to_writer`][to_writer](#to-writer) which serializes to any `io::Write`
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
  - [`from_reader`](#from_reader)
  - [`from_slice`](#from_slice)
  - [`from_str`](#from_str)
  - [`to_string`](#to_string)
  - [`to_string_pretty`](#to_string_pretty)
  - [`to_vec`](#to_vec)
  - [`to_vec_pretty`](#to_vec_pretty)
  - [`to_writer`](#to_writer)
  - [`to_writer_pretty`](#to_writer_pretty)
  - [`from_value`](#from_value)
  - [`to_value`](#to_value)
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
| [`from_reader`](#from_reader) | fn |  |
| [`from_slice`](#from_slice) | fn |  |
| [`from_str`](#from_str) | fn |  |
| [`to_string`](#to_string) | fn |  |
| [`to_string_pretty`](#to_string_pretty) | fn |  |
| [`to_vec`](#to_vec) | fn |  |
| [`to_vec_pretty`](#to_vec_pretty) | fn |  |
| [`to_writer`](#to_writer) | fn |  |
| [`to_writer_pretty`](#to_writer_pretty) | fn |  |
| [`from_value`](#from_value) | fn |  |
| [`to_value`](#to_value) | fn |  |
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
}
```

A structure that deserializes JSON into Rust values.

#### Implementations

- <span id="deserializer-from-slice"></span>`fn from_slice(bytes: &'a [u8]) -> Self`

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

- <span id="streamdeserializer-new"></span>`fn new(read: R) -> Self`

- <span id="streamdeserializer-byte-offset"></span>`fn byte_offset(&self) -> usize`

- <span id="streamdeserializer-peek-end-of-value"></span>`fn peek_end_of_value(&mut self) -> Result<()>` — [`Result`](#result)

#### Trait Implementations

##### `impl<'de, R, T> FusedIterator for StreamDeserializer<'de, R, T>`

##### `impl<I> IntoIterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamdeserializer-intoiter"></span>`type IntoIter = I`

- <span id="streamdeserializer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'de, R, T> Iterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-item"></span>`type Item = Result<T, Error>`

- <span id="streamdeserializer-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](#result)

### `Error`

```rust
struct Error {
    err: alloc::boxed::Box<ErrorImpl>,
}
```

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

- <span id="error-classify"></span>`fn classify(&self) -> Category` — [`Category`](error/index.md)

- <span id="error-is-io"></span>`fn is_io(&self) -> bool`

- <span id="error-is-syntax"></span>`fn is_syntax(&self) -> bool`

- <span id="error-is-data"></span>`fn is_data(&self) -> bool`

- <span id="error-is-eof"></span>`fn is_eof(&self) -> bool`

- <span id="error-io-error-kind"></span>`fn io_error_kind(&self) -> Option<ErrorKind>` — [`ErrorKind`](io/index.md)

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    writer: W,
    formatter: F,
}
```

A structure for serializing Rust values into JSON.

#### Implementations

- <span id="serializer-pretty"></span>`fn pretty(writer: W) -> Self`

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="map-clear"></span>`fn clear(&mut self)`

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](#value)

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](#value)

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](#value)

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](#value)

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](#value)

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](#value)

- <span id="map-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](map/index.md)

- <span id="map-len"></span>`fn len(&self) -> usize`

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](map/index.md)

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](map/index.md)

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](map/index.md)

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](map/index.md)

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](map/index.md)

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](map/index.md)

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

##### `impl<'de> Deserialize for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for Map<K, V>`

##### `impl<'de> Deserializer for crate::map::Map<alloc::string::String, crate::value::Value>`

- <span id="cratemapmap-error"></span>`type Error = Error`

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

- <span id="cratemapmap-err"></span>`type Err = Error`

- <span id="cratemapmap-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](#error)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](#value)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](#value)

##### `impl<'de> IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl IntoIterator for Map<alloc::string::String, crate::value::Value>`

- <span id="map-item"></span>`type Item = (String, Value)`

- <span id="map-intoiter"></span>`type IntoIter = IntoIter`

- <span id="map-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

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

Represents a JSON number, whether integer or floating point.

#### Implementations

- <span id="number-unexpected"></span>`fn unexpected(&self) -> Unexpected<'_>`

#### Trait Implementations

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](#number)

##### `impl Debug for Number`

- <span id="number-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](#number)

##### `impl<T> DeserializeOwned for Number`

##### `impl<'de> Deserializer for Number`

- <span id="number-error"></span>`type Error = Error`

- <span id="number-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="number-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

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

- <span id="cratenumbernumber-err"></span>`type Err = Error`

- <span id="cratenumbernumber-from-str"></span>`fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- <span id="number-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Number`

- <span id="number-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](#number)

##### `impl Serialize for Number`

- <span id="number-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl<T> ToString for Number`

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

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](#value)

##### `impl Debug for Value`

- <span id="value-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Value`

- <span id="value-default"></span>`fn default() -> Value` — [`Value`](#value)

##### `impl<'de> Deserialize for crate::value::Value`

- <span id="cratevaluevalue-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](#value)

##### `impl<T> DeserializeOwned for Value`

##### `impl<'de> Deserializer for crate::value::Value`

- <span id="cratevaluevalue-error"></span>`type Error = Error`

- <span id="cratevaluevalue-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

- <span id="cratevaluevalue-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](#error)

##### `impl Display for Value`

- <span id="value-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Value`

##### `impl<K: Into<alloc::string::String>, V: Into<super::Value>> FromIterator for super::Value`

- <span id="supervalue-from-iter"></span>`fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self`

##### `impl FromStr for crate::value::Value`

- <span id="cratevaluevalue-err"></span>`type Err = Error`

- <span id="cratevaluevalue-from-str"></span>`fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](#value), [`Error`](#error)

##### `impl Hash for Value`

- <span id="value-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> Index for super::Value`

- <span id="supervalue-output"></span>`type Output = Value`

- <span id="supervalue-index"></span>`fn index(&self, index: I) -> &Value` — [`Value`](#value)

##### `impl<I> IndexMut for super::Value`

- <span id="supervalue-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut Value` — [`Value`](#value)

##### `impl<'de> IntoDeserializer for crate::value::Value`

- <span id="cratevaluevalue-deserializer"></span>`type Deserializer = Value`

- <span id="cratevaluevalue-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl PartialEq for Value`

- <span id="value-eq"></span>`fn eq(&self, other: &i32) -> bool`

##### `impl Serialize for crate::value::Value`

- <span id="cratevaluevalue-serialize"></span>`fn serialize<S>(&self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Value`

##### `impl<T> ToString for Value`

- <span id="value-to-string"></span>`fn to_string(&self) -> String`

## Functions

## Type Aliases

## Macros

### `tri!`

### `json!`

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

