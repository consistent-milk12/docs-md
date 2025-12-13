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

*Defined in [`serde_json-1.0.145/src/de.rs:31-39`](../../.source_1765633015/serde_json-1.0.145/src/de.rs#L31-L39)*

A structure that deserializes JSON into Rust values.

#### Implementations

- <span id="deserializer-new"></span>`fn new(read: R) -> Self`

  Create a JSON deserializer from one of the possible serde_json input

  sources.

  

  When reading from a source against which short reads are not efficient, such

  as a `File`, you will want to apply your own buffering because serde_json

  will not buffer the input. See `std::io::BufReader`.

  

  Typically it is more convenient to use one of these methods instead:

  

    - Deserializer::from_str

    - Deserializer::from_slice

    - Deserializer::from_reader

#### Trait Implementations

##### `impl Any for Deserializer<R>`

- <span id="deserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Deserializer<R>`

- <span id="deserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Deserializer<R>`

- <span id="deserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: Read<'de>> Deserializer for &mut Deserializer<R>`

- <span id="mut-deserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

  Parses a JSON string as bytes. Note that this function does not check

  whether the bytes represent a valid UTF-8 string.

  

  The relevant part of the JSON specification is Section 8.2 of [RFC

  7159]:

  

  > When all the strings represented in a JSON text are composed entirely

  > of Unicode characters (however escaped), then that JSON text is

  > interoperable in the sense that all software implementations that

  > parse it will agree on the contents of names and of string values in

  > objects and arrays.

  >

  > However, the ABNF in this specification allows member names and string

  > values to contain bit sequences that cannot encode Unicode characters;

  > for example, "\uDEAD" (a single unpaired UTF-16 surrogate). Instances

  > of this have been observed, for example, when a library truncates a

  > UTF-16 string without checking whether the truncation split a

  > surrogate pair.  The behavior of software that receives JSON texts

  > containing such values is unpredictable; for example, implementations

  > might return different values for the length of a string value or even

  > suffer fatal runtime exceptions.

  

  The behavior of serde_json is specified to fail on non-UTF-8 strings

  when deserializing into Rust UTF-8 string types such as String, and

  succeed with the bytes representing the [WTF-8] encoding of code points

  when deserializing using this method.

  

  Escape sequences are processed as usual, and for `\uXXXX` escapes it is

  still checked if the hex number represents a valid Unicode code point.

  

  # Examples

  

  You can use this to parse JSON strings containing invalid UTF-8 bytes,

  or unpaired surrogates.

  

  ```rust

  use serde_bytes::ByteBuf;

  

  fn look_at_bytes() -> Result<(), serde_json::Error> {

      let json_data = b"\"some bytes: \xe5\x00\xe5\"";

      let bytes: ByteBuf = serde_json::from_slice(json_data)?;

  

      assert_eq!(b'\xe5', bytes[12]);

      assert_eq!(b'\0', bytes[13]);

      assert_eq!(b'\xe5', bytes[14]);

  

      Ok(())

  }

  

  look_at_bytes().unwrap();

  ```

  

  Backslash escape sequences like `\n` are still interpreted and required

  to be valid. `\u` escape sequences are required to represent a valid

  Unicode code point or lone surrogate.

  

  ```rust

  use serde_bytes::ByteBuf;

  

  fn look_at_bytes() -> Result<(), serde_json::Error> {

      let json_data = b"\"lone surrogate: \\uD801\"";

      let bytes: ByteBuf = serde_json::from_slice(json_data)?;

      let expected = b"lone surrogate: \xED\xA0\x81";

      assert_eq!(expected, bytes.as_slice());

      Ok(())

  }

  

  look_at_bytes();

  ```

- <span id="mut-deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

  Parses a `null` as a None, and any other values as a `Some(...)`.

- <span id="mut-deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

  Parses a newtype struct as the underlying value.

- <span id="mut-deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

  Parses an enum as an object like `{"$KEY":$VALUE}`, where $VALUE is either a straight

  value, a `[..]`, or a `{..}`.

- <span id="mut-deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](error/index.md#result)

##### `impl<T> From for Deserializer<R>`

- <span id="deserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Deserializer<R>`

- <span id="deserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Deserializer<R>`

- <span id="deserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Deserializer<R>`

- <span id="deserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`serde_json-1.0.145/src/de.rs:2349-2355`](../../.source_1765633015/serde_json-1.0.145/src/de.rs#L2349-L2355)*

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

  Create a JSON stream deserializer from one of the possible serde_json

  input sources.

  

  Typically it is more convenient to use one of these methods instead:

  

    - Deserializer::from_str(...).into_iter()

    - Deserializer::from_slice(...).into_iter()

    - Deserializer::from_reader(...).into_iter()

- <span id="streamdeserializer-byte-offset"></span>`fn byte_offset(&self) -> usize`

  Returns the number of bytes so far deserialized into a successful `T`.

  

  If a stream deserializer returns an EOF error, new data can be joined to

  `old_data[stream.byte_offset()..]` to try again.

  

  ```rust

  let data = b"[0] [1] [";

  

  let de = serde_json::Deserializer::from_slice(data);

  let mut stream = de.into_iter::<Vec<i32>>();

  assert_eq!(0, stream.byte_offset());

  

  println!("{:?}", stream.next()); // [0]

  assert_eq!(3, stream.byte_offset());

  

  println!("{:?}", stream.next()); // [1]

  assert_eq!(7, stream.byte_offset());

  

  println!("{:?}", stream.next()); // error

  assert_eq!(8, stream.byte_offset());

  

  // If err.is_eof(), can join the remaining data to new data and continue.

  let remaining = &data[stream.byte_offset()..];

  ```

  

  *Note:* In the future this method may be changed to return the number of

  bytes so far deserialized into a successful T *or* syntactically valid

  JSON skipped over due to a type error. See [serde-rs/json#70] for an

  example illustrating this.

- <span id="streamdeserializer-peek-end-of-value"></span>`fn peek_end_of_value(&mut self) -> Result<()>` — [`Result`](error/index.md#result)

#### Trait Implementations

##### `impl<T> Any for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<R, T> FusedIterator for StreamDeserializer<'de, R, T>`

##### `impl<T, U> Into for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamdeserializer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamdeserializer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R, T> Iterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-iterator-type-item"></span>`type Item = Result<T, Error>`

- <span id="streamdeserializer-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](error/index.md#result)

##### `impl<T, U> TryFrom for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Error`

```rust
struct Error {
    err: alloc::boxed::Box<ErrorImpl>,
}
```

*Defined in [`serde_json-1.0.145/src/error.rs:17-22`](../../.source_1765633015/serde_json-1.0.145/src/error.rs#L17-L22)*

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

#### Fields

- **`err`**: `alloc::boxed::Box<ErrorImpl>`

  This `Box` allows us to keep the size of `Error` as small as possible. A
  larger `Error` type was substantially slower due to all the functions
  that pass around `Result<T, Error>`.

#### Implementations

- <span id="error-line"></span>`fn line(&self) -> usize`

  One-based line number at which the error was detected.

  

  Characters in the first line of the input (before the first newline

  character) are in line 1.

- <span id="error-column"></span>`fn column(&self) -> usize`

  One-based column number at which the error was detected.

  

  The first character in the input and any characters immediately

  following a newline character are in column 1.

  

  Note that errors may occur in column 0, for example if a read from an

  I/O stream fails immediately following a previously read newline

  character.

- <span id="error-classify"></span>`fn classify(&self) -> Category` — [`Category`](error/index.md#category)

  Categorizes the cause of this error.

  

  - `Category::Io` - failure to read or write bytes on an I/O stream

  - `Category::Syntax` - input that is not syntactically valid JSON

  - `Category::Data` - input data that is semantically incorrect

  - `Category::Eof` - unexpected end of the input data

- <span id="error-is-io"></span>`fn is_io(&self) -> bool`

  Returns true if this error was caused by a failure to read or write

  bytes on an I/O stream.

- <span id="error-is-syntax"></span>`fn is_syntax(&self) -> bool`

  Returns true if this error was caused by input that was not

  syntactically valid JSON.

- <span id="error-is-data"></span>`fn is_data(&self) -> bool`

  Returns true if this error was caused by input data that was

  semantically incorrect.

  

  For example, JSON containing a number is semantically incorrect when the

  type being deserialized into holds a String.

- <span id="error-is-eof"></span>`fn is_eof(&self) -> bool`

  Returns true if this error was caused by prematurely reaching the end of

  the input data.

  

  Callers that process streaming input may be interested in retrying the

  deserialization once more data is available.

- <span id="error-io-error-kind"></span>`fn io_error_kind(&self) -> Option<ErrorKind>` — [`ErrorKind`](io/index.md#errorkind)

  The kind reported by the underlying standard library I/O error, if this

  error was caused by a failure to read or write bytes on an I/O stream.

  

  # Example

  

  ```rust

  use serde_json::Value;

  use std::io::{self, ErrorKind, Read};

  use std::process;

  

  struct ReaderThatWillTimeOut<'a>(&'a [u8]);

  

  impl<'a> Read for ReaderThatWillTimeOut<'a> {

      fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

          if self.0.is_empty() {

              Err(io::Error::new(ErrorKind::TimedOut, "timed out"))

          } else {

              self.0.read(buf)

          }

      }

  }

  

  fn main() {

      let reader = ReaderThatWillTimeOut(br#" {"k": "#);

  

      let _: Value = match serde_json::from_reader(reader) {

          Ok(value) => value,

          Err(error) => {

              if error.io_error_kind() == Some(ErrorKind::TimedOut) {

                  // Maybe this application needs to retry certain kinds of errors.

  

                  return;

              } else {

                  eprintln!("error: {}", error);

                  process::exit(1);

              }

          }

      };

  }

  ```

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoDeserializer for Map<alloc::string::String, crate::value::Value>`

- <span id="map-intodeserializer-type-deserializer"></span>`type Deserializer = Map<String, Value>`

- <span id="map-intodeserializer-into-deserializer"></span>`fn into_deserializer(self) -> <Self as >::Deserializer`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    writer: W,
    formatter: F,
}
```

*Defined in [`serde_json-1.0.145/src/ser.rs:17-20`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L17-L20)*

A structure for serializing Rust values into JSON.

#### Implementations

- <span id="serializer-new"></span>`fn new(writer: W) -> Self`

  Creates a new JSON serializer.

#### Trait Implementations

##### `impl Any for Serializer<W, F>`

- <span id="serializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Serializer<W, F>`

- <span id="serializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Serializer<W, F>`

- <span id="serializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Serializer<W, F>`

- <span id="serializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Serializer<W, F>`

- <span id="serializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

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

- <span id="a-mut-serializer-serializer-serialize-bool"></span>`fn serialize_bool(self, value: bool) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-i8"></span>`fn serialize_i8(self, value: i8) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-i16"></span>`fn serialize_i16(self, value: i16) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-i32"></span>`fn serialize_i32(self, value: i32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-i64"></span>`fn serialize_i64(self, value: i64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-i128"></span>`fn serialize_i128(self, value: i128) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-u8"></span>`fn serialize_u8(self, value: u8) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-u16"></span>`fn serialize_u16(self, value: u16) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-u32"></span>`fn serialize_u32(self, value: u32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-u64"></span>`fn serialize_u64(self, value: u64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-u128"></span>`fn serialize_u128(self, value: u128) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-f32"></span>`fn serialize_f32(self, value: f32) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-f64"></span>`fn serialize_f64(self, value: f64) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-char"></span>`fn serialize_char(self, value: char) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-str"></span>`fn serialize_str(self, value: &str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-bytes"></span>`fn serialize_bytes(self, value: &[u8]) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-unit"></span>`fn serialize_unit(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-unit-struct"></span>`fn serialize_unit_struct(self, _name: &'static str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-unit-variant"></span>`fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-newtype-struct"></span>`fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

  Serialize newtypes without an object wrapper.

- <span id="a-mut-serializer-serializer-serialize-newtype-variant"></span>`fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-none"></span>`fn serialize_none(self) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-some"></span>`fn serialize_some<T>(self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-seq"></span>`fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-tuple"></span>`fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-tuple-struct"></span>`fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-tuple-variant"></span>`fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-map"></span>`fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-struct"></span>`fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-serialize-struct-variant"></span>`fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant>` — [`Result`](error/index.md#result)

- <span id="a-mut-serializer-serializer-collect-str"></span>`fn collect_str<T>(self, value: &T) -> Result<()>` — [`Result`](error/index.md#result)

##### `impl<U> TryFrom for Serializer<W, F>`

- <span id="serializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="serializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Serializer<W, F>`

- <span id="serializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="serializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Map<K, V>`

```rust
struct Map<K, V> {
    map: alloc::collections::BTreeMap<K, V>,
}
```

*Defined in [`serde_json-1.0.145/src/map.rs:29-31`](../../.source_1765633015/serde_json-1.0.145/src/map.rs#L29-L31)*

Represents a JSON key/value type.

#### Implementations

- <span id="map-new"></span>`fn new() -> Self`

  Makes a new empty Map.

- <span id="map-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Makes a new empty Map with the given initial capacity.

- <span id="map-clear"></span>`fn clear(&mut self)`

  Clears the map, removing all values.

- <span id="map-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&Value>` — [`Value`](value/index.md#value)

  Returns a reference to the value corresponding to the key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

  Returns true if the map contains a value for the specified key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>` — [`Value`](value/index.md#value)

  Returns a mutable reference to the value corresponding to the key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-get-key-value"></span>`fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>` — [`Value`](value/index.md#value)

  Returns the key-value pair matching the given key.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

- <span id="map-insert"></span>`fn insert(&mut self, k: String, v: Value) -> Option<Value>` — [`Value`](value/index.md#value)

  Inserts a key-value pair into the map.

  

  If the map did not have this key present, `None` is returned.

  

  If the map did have this key present, the value is updated, and the old

  value is returned.

- <span id="map-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<Value>` — [`Value`](value/index.md#value)

  Removes a key from the map, returning the value at the key if the key

  was previously in the map.

  

  The key may be any borrowed form of the map's key type, but the ordering

  on the borrowed form *must* match the ordering on the key type.

  

  If serde_json's "preserve_order" is enabled, `.remove(key)` is

  equivalent to `.swap_remove(key)`, replacing this

  entry's position with the last element. If you need to preserve the

  relative order of the keys in the map, use

  `.shift_remove(key)` instead.

- <span id="map-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>` — [`Value`](value/index.md#value)

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

- <span id="map-entry"></span>`fn entry<S>(&mut self, key: S) -> Entry<'_>` — [`Entry`](map/index.md#entry)

  Gets the given key's corresponding entry in the map for in-place

  manipulation.

- <span id="map-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the map.

- <span id="map-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if the map contains no elements.

- <span id="map-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](map/index.md#iter)

  Gets an iterator over the entries of the map.

- <span id="map-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_>` — [`IterMut`](map/index.md#itermut)

  Gets a mutable iterator over the entries of the map.

- <span id="map-keys"></span>`fn keys(&self) -> Keys<'_>` — [`Keys`](map/index.md#keys)

  Gets an iterator over the keys of the map.

- <span id="map-values"></span>`fn values(&self) -> Values<'_>` — [`Values`](map/index.md#values)

  Gets an iterator over the values of the map.

- <span id="map-values-mut"></span>`fn values_mut(&mut self) -> ValuesMut<'_>` — [`ValuesMut`](map/index.md#valuesmut)

  Gets an iterator over mutable values of the map.

- <span id="map-into-values"></span>`fn into_values(self) -> IntoValues` — [`IntoValues`](map/index.md#intovalues)

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

- <span id="cratemapmap-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, Error>` — [`Error`](error/index.md#error)

##### `impl Hash for Map<alloc::string::String, crate::value::Value>`

- <span id="map-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Q> Index for Map<alloc::string::String, crate::value::Value>`

- <span id="map-index-type-output"></span>`type Output = Value`

- <span id="map-index"></span>`fn index(&self, index: &Q) -> &Value` — [`Value`](value/index.md#value)

##### `impl<Q> IndexMut for Map<alloc::string::String, crate::value::Value>`

- <span id="map-indexmut-index-mut"></span>`fn index_mut(&mut self, index: &Q) -> &mut Value` — [`Value`](value/index.md#value)

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

*Defined in [`serde_json-1.0.145/src/number.rs:22-24`](../../.source_1765633015/serde_json-1.0.145/src/number.rs#L22-L24)*

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

- <span id="number-from-f64"></span>`fn from_f64(f: f64) -> Option<Number>` — [`Number`](number/index.md#number)

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

- <span id="number-from-i128"></span>`fn from_i128(i: i128) -> Option<Number>` — [`Number`](number/index.md#number)

  Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or

  larger than u64::MAX can only be represented in `Number` if serde_json's

  "arbitrary_precision" feature is enabled.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_i128(256).is_some());

  ```

- <span id="number-from-u128"></span>`fn from_u128(i: u128) -> Option<Number>` — [`Number`](number/index.md#number)

  Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

  be represented in `Number` if serde_json's "arbitrary_precision" feature

  is enabled.

  

  ```rust

  use serde_json::Number;

  

  assert!(Number::from_u128(256).is_some());

  ```

- <span id="number-as-f32"></span>`fn as_f32(&self) -> Option<f32>`

- <span id="number-from-f32"></span>`fn from_f32(f: f32) -> Option<Number>` — [`Number`](number/index.md#number)

#### Trait Implementations

##### `impl Any for Number`

- <span id="number-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Number`

- <span id="number-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Number`

- <span id="number-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](number/index.md#number)

##### `impl CloneToUninit for Number`

- <span id="number-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Number`

- <span id="number-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](number/index.md#number)

##### `impl DeserializeOwned for Number`

##### `impl Deserializer for Number`

- <span id="number-deserializer-type-error"></span>`type Error = Error`

- <span id="number-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="number-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

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

- <span id="number-partialeq-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](number/index.md#number)

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

*Defined in [`serde_json-1.0.145/src/value/mod.rs:116-176`](../../.source_1765633015/serde_json-1.0.145/src/value/mod.rs#L116-L176)*

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

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](value/index.md#value)

##### `impl CloneToUninit for Value`

- <span id="value-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Value`

- <span id="value-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Value`

- <span id="value-default"></span>`fn default() -> Value` — [`Value`](value/index.md#value)

##### `impl Deserialize for crate::value::Value`

- <span id="cratevaluevalue-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>` — [`Value`](value/index.md#value)

##### `impl DeserializeOwned for Value`

##### `impl Deserializer for crate::value::Value`

- <span id="cratevaluevalue-deserializer-type-error"></span>`type Error = Error`

- <span id="cratevaluevalue-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

- <span id="cratevaluevalue-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](error/index.md#error)

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

- <span id="cratevaluevalue-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Value, Error>` — [`Value`](value/index.md#value), [`Error`](error/index.md#error)

##### `impl Hash for Value`

- <span id="value-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> Index for super::Value`

- <span id="supervalue-index-type-output"></span>`type Output = Value`

- <span id="supervalue-index"></span>`fn index(&self, index: I) -> &Value` — [`Value`](value/index.md#value)

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

- <span id="supervalue-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut Value` — [`Value`](value/index.md#value)

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

- <span id="value-partialeq-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](value/index.md#value)

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

## Functions

### `from_reader`

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned
```

*Defined in [`serde_json-1.0.145/src/de.rs:2612-2618`](../../.source_1765633015/serde_json-1.0.145/src/de.rs#L2612-L2618)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:2655-2660`](../../.source_1765633015/serde_json-1.0.145/src/de.rs#L2655-L2660)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:2697-2702`](../../.source_1765633015/serde_json-1.0.145/src/de.rs#L2697-L2702)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2245-2255`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L2245-L2255)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2264-2274`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L2264-L2274)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2213-2220`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L2213-L2220)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2229-2236`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L2229-L2236)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2177-2184`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L2177-L2184)*

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

*Defined in [`serde_json-1.0.145/src/ser.rs:2197-2204`](../../.source_1765633015/serde_json-1.0.145/src/ser.rs#L2197-L2204)*

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

*Defined in [`serde_json-1.0.145/src/value/mod.rs:1037-1042`](../../.source_1765633015/serde_json-1.0.145/src/value/mod.rs#L1037-L1042)*

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

*Defined in [`serde_json-1.0.145/src/value/mod.rs:995-1000`](../../.source_1765633015/serde_json-1.0.145/src/value/mod.rs#L995-L1000)*

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

*Defined in [`serde_json-1.0.145/src/error.rs:25`](../../.source_1765633015/serde_json-1.0.145/src/error.rs#L25)*

Alias for a `Result` with the error type `serde_json::Error`.

## Macros

### `tri!`

*Defined in [`serde_json-1.0.145/src/lib.rs:410-417`](../../.source_1765633015/serde_json-1.0.145/src/lib.rs#L410-L417)*

### `json!`

*Defined in [`serde_json-1.0.145/src/macros.rs:54-59`](../../.source_1765633015/serde_json-1.0.145/src/macros.rs#L54-L59)*

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

