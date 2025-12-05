# Crate `serde_json`

# Serde JSON

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
representation. This data structure is [`serde_json::Value`][value](#value).

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
[`serde_json::from_str`][from_str](#from-str) function. There is also [`from_slice`](de/index.md)
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
a JSON string to a Rust string with [`as_str()`](#as-str) or avoiding the use of
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
[`serde-json-core`](#serde-json-core) crate.

[value](#value): crate::value::Value
[from_str](#from-str): crate::de::from_str
[from_slice](#from-slice): crate::de::from_slice
[from_reader](#from-reader): crate::de::from_reader
[to_string](#to-string): crate::ser::to_string
[to_vec](#to-vec): crate::ser::to_vec
[to_writer](#to-writer): crate::ser::to_writer
[macro](#macro): crate::json


## Modules

- [`de`](de/index.md) - Deserialize JSON data to a Rust data structure.
- [`error`](error/index.md) - When serializing or deserializing JSON goes wrong.
- [`map`](map/index.md) - A map of String to serde_json::Value.
- [`ser`](ser/index.md) - Serialize a Rust data structure into JSON data.
- [`value`](value/index.md) - The Value enum, a loosely typed way of representing any valid JSON value.

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

- `fn end(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn into_iter<T>(self: Self) -> StreamDeserializer<'de, R, T>` — [`StreamDeserializer`](../de/index.md)

- `fn peek(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../error/index.md)

- `fn peek_or_null(self: &mut Self) -> Result<u8>` — [`Result`](../error/index.md)

- `fn eat_char(self: &mut Self)`

- `fn next_char(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../error/index.md)

- `fn next_char_or_null(self: &mut Self) -> Result<u8>` — [`Result`](../error/index.md)

- `fn error(self: &Self, reason: ErrorCode) -> Error` — [`ErrorCode`](../error/index.md), [`Error`](../error/index.md)

- `fn peek_error(self: &Self, reason: ErrorCode) -> Error` — [`ErrorCode`](../error/index.md), [`Error`](../error/index.md)

- `fn parse_whitespace(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../error/index.md)

- `fn peek_invalid_type(self: &mut Self, exp: &dyn Expected) -> Error` — [`Error`](../error/index.md)

- `fn deserialize_number<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md)

- `fn do_deserialize_i128<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md)

- `fn do_deserialize_u128<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md)

- `fn scan_integer128(self: &mut Self, buf: &mut String) -> Result<()>` — [`Result`](../error/index.md)

- `fn fix_position(self: &Self, err: Error) -> Error` — [`Error`](../error/index.md)

- `fn parse_ident(self: &mut Self, ident: &[u8]) -> Result<()>` — [`Result`](../error/index.md)

- `fn parse_integer(self: &mut Self, positive: bool) -> Result<ParserNumber>` — [`Result`](../error/index.md), [`ParserNumber`](../de/index.md)

- `fn parse_number(self: &mut Self, positive: bool, significand: u64) -> Result<ParserNumber>` — [`Result`](../error/index.md), [`ParserNumber`](../de/index.md)

- `fn parse_decimal(self: &mut Self, positive: bool, significand: u64, exponent_before_decimal_point: i32) -> Result<f64>` — [`Result`](../error/index.md)

- `fn parse_exponent(self: &mut Self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64>` — [`Result`](../error/index.md)

- `fn f64_from_parts(self: &mut Self, positive: bool, significand: u64, exponent: i32) -> Result<f64>` — [`Result`](../error/index.md)

- `fn parse_long_integer(self: &mut Self, positive: bool, significand: u64) -> Result<f64>` — [`Result`](../error/index.md)

- `fn parse_decimal_overflow(self: &mut Self, positive: bool, significand: u64, exponent: i32) -> Result<f64>` — [`Result`](../error/index.md)

- `fn parse_exponent_overflow(self: &mut Self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64>` — [`Result`](../error/index.md)

- `fn parse_any_signed_number(self: &mut Self) -> Result<ParserNumber>` — [`Result`](../error/index.md), [`ParserNumber`](../de/index.md)

- `fn parse_any_number(self: &mut Self, positive: bool) -> Result<ParserNumber>` — [`Result`](../error/index.md), [`ParserNumber`](../de/index.md)

- `fn parse_object_colon(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn end_seq(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn end_map(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn ignore_value(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn ignore_integer(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn ignore_decimal(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

- `fn ignore_exponent(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

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

- `fn peek_end_of_value(self: &mut Self) -> Result<()>` — [`Result`](../error/index.md)

#### Trait Implementations

##### `impl FusedIterator<'de, R, T>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'de, R, T>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<Result<T>>` — [`Result`](../error/index.md)

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

- `fn line(self: &Self) -> usize`

- `fn column(self: &Self) -> usize`

- `fn classify(self: &Self) -> Category` — [`Category`](../error/index.md)

- `fn is_io(self: &Self) -> bool`

- `fn is_syntax(self: &Self) -> bool`

- `fn is_data(self: &Self) -> bool`

- `fn is_eof(self: &Self) -> bool`

- `fn io_error_kind(self: &Self) -> Option<ErrorKind>`

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

- `fn custom<T: Display>(msg: T) -> Error` — [`Error`](../error/index.md)

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    writer: W,
    formatter: F,
}
```

A structure for serializing Rust values into JSON.

#### Implementations

- `fn pretty(writer: W) -> Self`

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

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>` — [`Value`](../value/index.md)

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>` — [`Value`](../value/index.md)

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](../value/index.md)

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>` — [`Value`](../value/index.md)

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>` — [`Value`](../value/index.md)

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>` — [`Value`](../value/index.md)

- `fn append(self: &mut Self, other: &mut Self)`

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>` — [`Entry`](../map/index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_>` — [`Iter`](../map/index.md)

- `fn iter_mut(self: &mut Self) -> IterMut<'_>` — [`IterMut`](../map/index.md)

- `fn keys(self: &Self) -> Keys<'_>` — [`Keys`](../map/index.md)

- `fn values(self: &Self) -> Values<'_>` — [`Values`](../map/index.md)

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>` — [`ValuesMut`](../map/index.md)

- `fn into_values(self: Self) -> IntoValues` — [`IntoValues`](../map/index.md)

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

- `fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](../error/index.md)

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Index<Q>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value` — [`Value`](../value/index.md)

##### `impl IndexMut<Q>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value` — [`Value`](../value/index.md)

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

- `fn clone(self: &Self) -> Number` — [`Number`](../number/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](../number/index.md)

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

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

- `fn eq(self: &Self, other: &Number) -> bool` — [`Number`](../number/index.md)

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

- `fn clone(self: &Self) -> Value` — [`Value`](../value/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Value` — [`Value`](../value/index.md)

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](../value/index.md)

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl FromIterator<K: Into<alloc::string::String>, V: Into<super::Value>>`

- `fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](../value/index.md), [`Error`](../error/index.md)

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Index<I>`

- `type Output = Value`

- `fn index(self: &Self, index: I) -> &Value` — [`Value`](../value/index.md)

##### `impl IndexMut<I>`

- `fn index_mut(self: &mut Self, index: I) -> &mut Value` — [`Value`](../value/index.md)

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Value`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Value) -> bool` — [`Value`](../value/index.md)

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

## Functions

## Type Aliases

## Macros

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

