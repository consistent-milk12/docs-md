*[serde_json](../index.md) / [ser](index.md)*

---

# Module `ser`

Serialize a Rust data structure into JSON data.

## Structs

### `Serializer<W, F>`

```rust
struct Serializer<W, F> {
    // [REDACTED: Private Fields]
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

### `CompactFormatter`

```rust
struct CompactFormatter;
```

This structure compacts a JSON value with no extra whitespace.

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

##### `impl Clone`

- `fn clone(self: &Self) -> CompactFormatter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Formatter`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> CompactFormatter`

### `PrettyFormatter<'a>`

```rust
struct PrettyFormatter<'a> {
    // [REDACTED: Private Fields]
}
```

This structure pretty prints a JSON value to make it human readable.

#### Implementations

- `fn new() -> Self`
  Construct a pretty printer formatter that defaults to using two spaces for indentation.

- `fn with_indent(indent: &'a [u8]) -> Self`
  Construct a pretty printer formatter that uses the `indent` string for indentation.

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

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> PrettyFormatter<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Formatter<'a>`

- `fn begin_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn end_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn begin_array_value<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>`

- `fn end_array_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

- `fn begin_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn end_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn begin_object_key<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>`

- `fn begin_object_value<W>(self: &mut Self, writer: &mut W) -> io::Result<()>`

- `fn end_object_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>`

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'a>`

- `fn default() -> Self`

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

