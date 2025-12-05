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

- `fn position_of_index(self: &Self, i: usize) -> Position` — [`Position`](../../read/index.md)

- `fn skip_to_escape(self: &mut Self, forbid_control_characters: bool)`

- `fn skip_to_escape_slow(self: &mut Self)`

- `fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<Reference<'a, 's, T>>` — [`Result`](../../error/index.md), [`Reference`](../../read/index.md)

#### Trait Implementations

##### `impl Fused<'a>`

##### `impl Read<'a>`

##### `impl Sealed<'a>`

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

##### `impl Fused<'a>`

##### `impl Read<'a>`

##### `impl Sealed<'a>`

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

- `fn new(reader: R) -> Self`

#### Trait Implementations

##### `impl Read<'de, R>`

##### `impl Sealed<R>`

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

- `fn end(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn into_iter<T>(self: Self) -> StreamDeserializer<'de, R, T>` — [`StreamDeserializer`](../../de/index.md)

- `fn peek(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../../error/index.md)

- `fn peek_or_null(self: &mut Self) -> Result<u8>` — [`Result`](../../error/index.md)

- `fn eat_char(self: &mut Self)`

- `fn next_char(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../../error/index.md)

- `fn next_char_or_null(self: &mut Self) -> Result<u8>` — [`Result`](../../error/index.md)

- `fn error(self: &Self, reason: ErrorCode) -> Error` — [`ErrorCode`](../../error/index.md), [`Error`](../../error/index.md)

- `fn peek_error(self: &Self, reason: ErrorCode) -> Error` — [`ErrorCode`](../../error/index.md), [`Error`](../../error/index.md)

- `fn parse_whitespace(self: &mut Self) -> Result<Option<u8>>` — [`Result`](../../error/index.md)

- `fn peek_invalid_type(self: &mut Self, exp: &dyn Expected) -> Error` — [`Error`](../../error/index.md)

- `fn deserialize_number<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md)

- `fn do_deserialize_i128<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md)

- `fn do_deserialize_u128<'any, V>(self: &mut Self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../../error/index.md)

- `fn scan_integer128(self: &mut Self, buf: &mut String) -> Result<()>` — [`Result`](../../error/index.md)

- `fn fix_position(self: &Self, err: Error) -> Error` — [`Error`](../../error/index.md)

- `fn parse_ident(self: &mut Self, ident: &[u8]) -> Result<()>` — [`Result`](../../error/index.md)

- `fn parse_integer(self: &mut Self, positive: bool) -> Result<ParserNumber>` — [`Result`](../../error/index.md), [`ParserNumber`](../../de/index.md)

- `fn parse_number(self: &mut Self, positive: bool, significand: u64) -> Result<ParserNumber>` — [`Result`](../../error/index.md), [`ParserNumber`](../../de/index.md)

- `fn parse_decimal(self: &mut Self, positive: bool, significand: u64, exponent_before_decimal_point: i32) -> Result<f64>` — [`Result`](../../error/index.md)

- `fn parse_exponent(self: &mut Self, positive: bool, significand: u64, starting_exp: i32) -> Result<f64>` — [`Result`](../../error/index.md)

- `fn f64_from_parts(self: &mut Self, positive: bool, significand: u64, exponent: i32) -> Result<f64>` — [`Result`](../../error/index.md)

- `fn parse_long_integer(self: &mut Self, positive: bool, significand: u64) -> Result<f64>` — [`Result`](../../error/index.md)

- `fn parse_decimal_overflow(self: &mut Self, positive: bool, significand: u64, exponent: i32) -> Result<f64>` — [`Result`](../../error/index.md)

- `fn parse_exponent_overflow(self: &mut Self, positive: bool, zero_significand: bool, positive_exp: bool) -> Result<f64>` — [`Result`](../../error/index.md)

- `fn parse_any_signed_number(self: &mut Self) -> Result<ParserNumber>` — [`Result`](../../error/index.md), [`ParserNumber`](../../de/index.md)

- `fn parse_any_number(self: &mut Self, positive: bool) -> Result<ParserNumber>` — [`Result`](../../error/index.md), [`ParserNumber`](../../de/index.md)

- `fn parse_object_colon(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn end_seq(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn end_map(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn ignore_value(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn ignore_integer(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn ignore_decimal(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

- `fn ignore_exponent(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

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

- `fn peek_end_of_value(self: &mut Self) -> Result<()>` — [`Result`](../../error/index.md)

#### Trait Implementations

##### `impl FusedIterator<'de, R, T>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'de, R, T>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<Result<T>>` — [`Result`](../../error/index.md)

## Traits

## Functions

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
as a [`File`](#file), you will want to apply your own buffering because serde_json
will not buffer the input. See `std::io::BufReader`.

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`](#deserializer).

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`](#from-str)
or [`from_slice`](#from-slice) on it. See [issue #160].

[issue #160]: https://github.com/serde-rs/json/issues/160

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
    // The type of `j` is `&[u8](#u8)`
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

