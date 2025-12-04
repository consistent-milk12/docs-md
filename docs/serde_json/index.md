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
representation. This data structure is [`serde_json::Value`][value](#value)
.

```
# use serde_json::{Number, Map};
#
# #[allow(dead_code)]
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
[`serde_json::from_str`][from_str](#from-str)
 function. There is also [`from_slice`](serde_json/de/index.md)
for parsing from a byte slice `&[u8](#u8)
` and [`from_reader`](serde_json/de/index.md) for parsing from
any `io::Read` like a File or a TCP stream.

```
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
#
# fn main() {
#     untyped_example().unwrap();
# }
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

```
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
#
# fn main() {
#     typed_example().unwrap();
# }
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

Serde JSON provides a [`json!` macro][macro](#macro)
 to build `serde_json::Value`
objects with very natural JSON syntax.

```
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

```
# use serde_json::json;
#
# fn random_phone() -> u16 { 0 }
#
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
[`serde_json::to_string`][to_string](#to-string)
. There is also
[`serde_json::to_vec`][to_vec](#to-vec)
 which serializes to a `Vec<u8>` and
[`serde_json::to_writer`][to_writer](#to-writer)
 which serializes to any `io::Write`
such as a File or a TCP stream.

```
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
#
# fn main() {
#     print_an_address().unwrap();
# }
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
[dependencies](#dependencies)

serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

For JSON support in Serde without a memory allocator, please see the
[`serde-json-core`](#serde-json-core) crate.

[value](#value)
: crate::value::Value
[from_str](#from-str)
: crate::de::from_str
[from_slice](#from-slice)
: crate::de::from_slice
[from_reader](#from-reader)
: crate::de::from_reader
[to_string](#to-string)
: crate::ser::to_string
[to_vec](#to-vec)
: crate::ser::to_vec
[to_writer](#to-writer)
: crate::ser::to_writer
[macro](#macro)
: crate::json


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
}
```

A structure that deserializes JSON into Rust values.

#### Implementations

- `fn from_reader(reader: R) -> Self`
  Creates a JSON deserializer from an `io::Read`.

- `fn from_str(s: &'a str) -> Self`
  Creates a JSON deserializer from a `&str`.

- `fn new(read: R) -> Self`
  Create a JSON deserializer from one of the possible serde_json input

- `fn end(self: &mut Self) -> Result<()>`
  The `Deserializer::end` method should be called after a value has been fully deserialized.

- `fn into_iter<T>(self: Self) -> StreamDeserializer<'de, R, T>`
  Turn a JSON deserializer into an iterator over values of type T.

- `fn from_slice(bytes: &'a [u8]) -> Self`
  Creates a JSON deserializer from a `&[u8]`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `StreamDeserializer<'de, R, T>`

```rust
struct StreamDeserializer<'de, R, T> {
}
```

Iterator that deserializes a stream into multiple JSON values.

A stream deserializer can be created from any JSON deserializer using the
`Deserializer::into_iter` method.

The data can consist of any JSON value. Values need to be a self-delineating value e.g.
arrays, objects, or strings, or be followed by whitespace or a self-delineating value.

```
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
  Create a JSON stream deserializer from one of the possible serde_json

- `fn byte_offset(self: &Self) -> usize`
  Returns the number of bytes so far deserialized into a successful `T`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator<'de, R, T>`

##### `impl Iterator<'de, R, T>`

- `type Item = Result<T, Error>`

- `fn next(self: &mut Self) -> Option<Result<T>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Error`

```rust
struct Error {
}
```

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

#### Implementations

- `fn line(self: &Self) -> usize`
  One-based line number at which the error was detected.

- `fn column(self: &Self) -> usize`
  One-based column number at which the error was detected.

- `fn classify(self: &Self) -> Category`
  Categorizes the cause of this error.

- `fn is_io(self: &Self) -> bool`
  Returns true if this error was caused by a failure to read or write

- `fn is_syntax(self: &Self) -> bool`
  Returns true if this error was caused by input that was not

- `fn is_data(self: &Self) -> bool`
  Returns true if this error was caused by input data that was

- `fn is_eof(self: &Self) -> bool`
  Returns true if this error was caused by prematurely reaching the end of

- `fn io_error_kind(self: &Self) -> Option<ErrorKind>`
  The kind reported by the underlying standard library I/O error, if this

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

- `fn custom<T: Display>(msg: T) -> Error`

##### `impl Error`

- `fn custom<T: Display>(msg: T) -> Error`

- `fn invalid_type(unexp: de::Unexpected<'_>, exp: &dyn de::Expected) -> Self`

- `fn invalid_value(unexp: de::Unexpected<'_>, exp: &dyn de::Expected) -> Self`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
}
```

A structure for serializing Rust values into JSON.

#### Implementations

- `fn pretty(writer: W) -> Self`
  Creates a new JSON pretty print serializer.

- `fn new(writer: W) -> Self`
  Creates a new JSON serializer.

- `fn with_formatter(writer: W, formatter: F) -> Self`
  Creates a new JSON visitor whose output will be written to the writer

- `fn into_inner(self: Self) -> W`
  Unwrap the `Writer` from the `Serializer`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Map<K, V>`

```rust
struct Map<K, V> {
}
```

Represents a JSON key/value type.

#### Implementations

- `fn new() -> Self`
  Makes a new empty Map.

- `fn with_capacity(capacity: usize) -> Self`
  Makes a new empty Map with the given initial capacity.

- `fn clear(self: &mut Self)`
  Clears the map, removing all values.

- `fn get<Q>(self: &Self, key: &Q) -> Option<&Value>`
  Returns a reference to the value corresponding to the key.

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`
  Returns true if the map contains a value for the specified key.

- `fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>`
  Returns a mutable reference to the value corresponding to the key.

- `fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>`
  Returns the key-value pair matching the given key.

- `fn insert(self: &mut Self, k: String, v: Value) -> Option<Value>`
  Inserts a key-value pair into the map.

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>`
  Removes a key from the map, returning the value at the key if the key

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>`
  Removes a key from the map, returning the stored key and value if the

- `fn append(self: &mut Self, other: &mut Self)`
  Moves all elements from other into self, leaving other empty.

- `fn entry<S>(self: &mut Self, key: S) -> Entry<'_>`
  Gets the given key's corresponding entry in the map for in-place

- `fn len(self: &Self) -> usize`
  Returns the number of elements in the map.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the map contains no elements.

- `fn iter(self: &Self) -> Iter<'_>`
  Gets an iterator over the entries of the map.

- `fn iter_mut(self: &mut Self) -> IterMut<'_>`
  Gets a mutable iterator over the entries of the map.

- `fn keys(self: &Self) -> Keys<'_>`
  Gets an iterator over the keys of the map.

- `fn values(self: &Self) -> Values<'_>`
  Gets an iterator over the values of the map.

- `fn values_mut(self: &mut Self) -> ValuesMut<'_>`
  Gets an iterator over mutable values of the map.

- `fn into_values(self: Self) -> IntoValues`
  Gets an iterator over the values of the map.

- `fn retain<F>(self: &mut Self, f: F)`
  Retains only the elements specified by the predicate.

- `fn sort_keys(self: &mut Self)`
  Sorts this map's entries in-place using `str`'s usual ordering.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Self, Error>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Map<String, Value>`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl IntoIterator`

- `type Item = (String, Value)`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Extend`

- `fn extend<T>(self: &mut Self, iter: T)`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Index<Q>`

- `type Output = Value`

- `fn index(self: &Self, index: &Q) -> &Value`

##### `impl IndexMut<Q>`

- `fn index_mut(self: &mut Self, index: &Q) -> &mut Value`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

### `Number`

```rust
struct Number {
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- `fn is_i64(self: &Self) -> bool`
  Returns true if the `Number` is an integer between `i64::MIN` and

- `fn is_u64(self: &Self) -> bool`
  Returns true if the `Number` is an integer between zero and `u64::MAX`.

- `fn is_f64(self: &Self) -> bool`
  Returns true if the `Number` can be represented by f64.

- `fn as_i64(self: &Self) -> Option<i64>`
  If the `Number` is an integer, represent it as i64 if possible. Returns

- `fn as_u64(self: &Self) -> Option<u64>`
  If the `Number` is an integer, represent it as u64 if possible. Returns

- `fn as_f64(self: &Self) -> Option<f64>`
  Represents the number as f64 if possible. Returns None otherwise.

- `fn from_f64(f: f64) -> Option<Number>`
  Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON

- `fn as_i128(self: &Self) -> Option<i128>`
  If the `Number` is an integer, represent it as i128 if possible. Returns

- `fn as_u128(self: &Self) -> Option<u128>`
  If the `Number` is an integer, represent it as u128 if possible. Returns

- `fn from_i128(i: i128) -> Option<Number>`
  Converts an `i128` to a `Number`. Numbers smaller than i64::MIN or

- `fn from_u128(i: u128) -> Option<Number>`
  Converts a `u128` to a `Number`. Numbers greater than u64::MAX can only

#### Trait Implementations

##### `impl From`

- `fn from(u: u32) -> Self`

##### `impl From`

- `fn from(i: i8) -> Self`

##### `impl From`

- `fn from(u: u8) -> Self`

##### `impl From`

- `fn from(u: usize) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(i: isize) -> Self`

##### `impl From`

- `fn from(i: i32) -> Self`

##### `impl From`

- `fn from(i: i64) -> Self`

##### `impl From`

- `fn from(u: u64) -> Self`

##### `impl From`

- `fn from(u: u16) -> Self`

##### `impl From`

- `fn from(i: i16) -> Self`

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Number`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Number) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

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

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

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
  
  ```
  # use serde_json::json;
  #
  let v = json!(null);
  ```

- **`Bool`**

  Represents a JSON boolean.
  
  ```
  # use serde_json::json;
  #
  let v = json!(true);
  ```

- **`Number`**

  Represents a JSON number, whether integer or floating point.
  
  ```
  # use serde_json::json;
  #
  let v = json!(12.5);
  ```

- **`String`**

  Represents a JSON string.
  
  ```
  # use serde_json::json;
  #
  let v = json!("a string");
  ```

- **`Array`**

  Represents a JSON array.
  
  ```
  # use serde_json::json;
  #
  let v = json!(["an", "array"]);
  ```

- **`Object`**

  Represents a JSON object.
  
  By default the map is backed by a BTreeMap. Enable the `preserve_order`
  feature of serde_json to use IndexMap instead, which preserves
  entries in the order they are inserted into the map. In particular, this
  allows JSON data to be deserialized into a Value and serialized to a
  string while retaining the order of map keys in the input.
  
  ```
  # use serde_json::json;
  #
  let v = json!({ "an": "object" });
  ```

#### Implementations

- `fn get<I: Index>(self: &Self, index: I) -> Option<&Value>`
  Index into a JSON array or map. A string index can be used to access a

- `fn get_mut<I: Index>(self: &mut Self, index: I) -> Option<&mut Value>`
  Mutably index into a JSON array or map. A string index can be used to

- `fn is_object(self: &Self) -> bool`
  Returns true if the `Value` is an Object. Returns false otherwise.

- `fn as_object(self: &Self) -> Option<&Map<String, Value>>`
  If the `Value` is an Object, returns the associated Map. Returns None

- `fn as_object_mut(self: &mut Self) -> Option<&mut Map<String, Value>>`
  If the `Value` is an Object, returns the associated mutable Map.

- `fn is_array(self: &Self) -> bool`
  Returns true if the `Value` is an Array. Returns false otherwise.

- `fn as_array(self: &Self) -> Option<&Vec<Value>>`
  If the `Value` is an Array, returns the associated vector. Returns None

- `fn as_array_mut(self: &mut Self) -> Option<&mut Vec<Value>>`
  If the `Value` is an Array, returns the associated mutable vector.

- `fn is_string(self: &Self) -> bool`
  Returns true if the `Value` is a String. Returns false otherwise.

- `fn as_str(self: &Self) -> Option<&str>`
  If the `Value` is a String, returns the associated str. Returns None

- `fn is_number(self: &Self) -> bool`
  Returns true if the `Value` is a Number. Returns false otherwise.

- `fn as_number(self: &Self) -> Option<&Number>`
  If the `Value` is a Number, returns the associated [`Number`]. Returns

- `fn is_i64(self: &Self) -> bool`
  Returns true if the `Value` is an integer between `i64::MIN` and

- `fn is_u64(self: &Self) -> bool`
  Returns true if the `Value` is an integer between zero and `u64::MAX`.

- `fn is_f64(self: &Self) -> bool`
  Returns true if the `Value` is a number that can be represented by f64.

- `fn as_i64(self: &Self) -> Option<i64>`
  If the `Value` is an integer, represent it as i64 if possible. Returns

- `fn as_u64(self: &Self) -> Option<u64>`
  If the `Value` is an integer, represent it as u64 if possible. Returns

- `fn as_f64(self: &Self) -> Option<f64>`
  If the `Value` is a number, represent it as f64 if possible. Returns

- `fn is_boolean(self: &Self) -> bool`
  Returns true if the `Value` is a Boolean. Returns false otherwise.

- `fn as_bool(self: &Self) -> Option<bool>`
  If the `Value` is a Boolean, returns the associated bool. Returns None

- `fn is_null(self: &Self) -> bool`
  Returns true if the `Value` is a Null. Returns false otherwise.

- `fn as_null(self: &Self) -> Option<()>`
  If the `Value` is a Null, returns (). Returns None otherwise.

- `fn pointer(self: &Self, pointer: &str) -> Option<&Value>`
  Looks up a value by a JSON Pointer.

- `fn pointer_mut(self: &mut Self, pointer: &str) -> Option<&mut Value>`
  Looks up a value by a JSON Pointer and returns a mutable reference to

- `fn take(self: &mut Self) -> Value`
  Takes the value out of the `Value`, leaving a `Null` in its place.

- `fn sort_all_objects(self: &mut Self)`
  Reorders the entries of all `Value::Object` nested within this JSON

#### Trait Implementations

##### `impl From`

- `fn from(n: i32) -> Self`

##### `impl From`

- `fn from(f: String) -> Self`
  Convert `String` to `Value::String`.

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(n: isize) -> Self`

##### `impl From`

- `fn from(n: u16) -> Self`

##### `impl From<T: Clone + Into<super::Value>>`

- `fn from(f: &[T]) -> Self`
  Convert a slice to `Value::Array`.

##### `impl From`

- `fn from(n: i64) -> Self`

##### `impl From`

- `fn from(n: u32) -> Self`

##### `impl From`

- `fn from(f: f64) -> Self`
  Convert 64-bit floating point number to `Value::Number`, or

##### `impl From<T>`

- `fn from(opt: Option<T>) -> Self`

##### `impl From`

- `fn from(f: f32) -> Self`
  Convert 32-bit floating point number to `Value::Number`, or

##### `impl From`

- `fn from(n: u8) -> Self`

##### `impl From<'a>`

- `fn from(f: Cow<'a, str>) -> Self`
  Convert copy-on-write string to `Value::String`.

##### `impl From`

- `fn from(n: u64) -> Self`

##### `impl From`

- `fn from(f: bool) -> Self`
  Convert boolean to `Value::Bool`.

##### `impl From`

- `fn from(f: Number) -> Self`
  Convert `Number` to `Value::Number`.

##### `impl From`

- `fn from(n: i8) -> Self`

##### `impl From`

- `fn from(f: Map<String, Value>) -> Self`
  Convert map (with string keys) to `Value::Object`.

##### `impl From<T: Into<super::Value>>`

- `fn from(f: Vec<T>) -> Self`
  Convert a `Vec` to `Value::Array`.

##### `impl From<T: Into<super::Value>, const N: usize>`

- `fn from(array: [T; N]) -> Self`

##### `impl From`

- `fn from((): ()) -> Self`
  Convert `()` to `Value::Null`.

##### `impl From`

- `fn from(n: usize) -> Self`

##### `impl From`

- `fn from(n: i16) -> Self`

##### `impl From`

- `fn from(f: &str) -> Self`
  Convert string slice to `Value::String`.

##### `impl FromIterator<K: Into<alloc::string::String>, V: Into<super::Value>>`

- `fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self`
  Create a `Value::Object` by collecting an iterator of key-value pairs.

##### `impl FromIterator<T: Into<super::Value>>`

- `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`
  Create a `Value::Array` by collecting an iterator of array elements.

##### `impl FromStr`

- `type Err = Error`

- `fn from_str(s: &str) -> Result<Value, Error>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoDeserializer<'de>`

- `type Deserializer = Value`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Value`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`
  Display a JSON value as a string.

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Index<I>`

- `type Output = Value`

- `fn index(self: &Self, index: I) -> &Value`
  Index into a `serde_json::Value` using the syntax `value[0]` or

##### `impl IndexMut<I>`

- `fn index_mut(self: &mut Self, index: I) -> &mut Value`
  Write into a `serde_json::Value` using the syntax `value[0] = ...` or

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i64) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &isize) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &f32) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u16) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i8) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i16) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u64) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u32) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &usize) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &bool) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &f64) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u8) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Value) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &i32) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Value`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>`

##### `impl DeserializeOwned<T>`

##### `impl Deserializer<'de>`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_unit_struct<V>(self: Self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_tuple_struct<V>(self: Self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_struct<V>(self: Self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>`

##### `impl Serialize`

- `fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>`

## Functions

## Type Aliases

## Macros

### `json!`

Construct a `serde_json::Value` from a JSON literal.

```
# use serde_json::json;
#
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

```
# use serde_json::json;
#
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

```
# use serde_json::json;
#
let value = json!([
    "notice",
    "the",
    "trailing",
    "comma -->",
]);
```

