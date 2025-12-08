*[serde_json](../index.md) / [error](index.md)*

---

# Module `error`

When serializing or deserializing JSON goes wrong.

## Structs

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

- `fn classify(self: &Self) -> Category` — [`Category`](#category)

- `fn is_io(self: &Self) -> bool`

- `fn is_syntax(self: &Self) -> bool`

- `fn is_data(self: &Self) -> bool`

- `fn is_eof(self: &Self) -> bool`

- `fn io_error_kind(self: &Self) -> Option<ErrorKind>` — [`ErrorKind`](../io/index.md)

#### Trait Implementations

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

### `ErrorImpl`

```rust
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}
```

#### Trait Implementations

##### `impl Display for ErrorImpl`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ErrorImpl`

- `fn to_string(self: &Self) -> String`

### `JsonUnexpected<'a>`

```rust
struct JsonUnexpected<'a>(de::Unexpected<'a>);
```

#### Trait Implementations

##### `impl<'a> Display for JsonUnexpected<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for JsonUnexpected<'a>`

- `fn to_string(self: &Self) -> String`

## Enums

### `Category`

```rust
enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}
```

Categorizes the cause of a `serde_json::Error`.

#### Variants

- **`Io`**

  The error was caused by a failure to read or write bytes on an I/O
  stream.

- **`Syntax`**

  The error was caused by input that was not syntactically valid JSON.

- **`Data`**

  The error was caused by input data that was semantically incorrect.
  
  For example, JSON containing a number is semantically incorrect when the
  type being deserialized into holds a String.

- **`Eof`**

  The error was caused by prematurely reaching the end of the input data.
  
  Callers that process streaming input may be interested in retrying the
  deserialization once more data is available.

#### Trait Implementations

##### `impl Clone for Category`

- `fn clone(self: &Self) -> Category` — [`Category`](#category)

##### `impl Copy for Category`

##### `impl Debug for Category`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Category`

##### `impl PartialEq for Category`

- `fn eq(self: &Self, other: &Category) -> bool` — [`Category`](#category)

##### `impl StructuralPartialEq for Category`

### `ErrorCode`

```rust
enum ErrorCode {
    Message(alloc::boxed::Box<str>),
    Io(io::Error),
    EofWhileParsingList,
    EofWhileParsingObject,
    EofWhileParsingString,
    EofWhileParsingValue,
    ExpectedColon,
    ExpectedListCommaOrEnd,
    ExpectedObjectCommaOrEnd,
    ExpectedSomeIdent,
    ExpectedSomeValue,
    ExpectedDoubleQuote,
    InvalidEscape,
    InvalidNumber,
    NumberOutOfRange,
    InvalidUnicodeCodePoint,
    ControlCharacterWhileParsingString,
    KeyMustBeAString,
    ExpectedNumericKey,
    FloatKeyMustBeFinite,
    LoneLeadingSurrogateInHexEscape,
    TrailingComma,
    TrailingCharacters,
    UnexpectedEndOfHexEscape,
    RecursionLimitExceeded,
}
```

#### Variants

- **`Message`**

  Catchall for syntax error messages

- **`Io`**

  Some I/O error occurred while serializing or deserializing.

- **`EofWhileParsingList`**

  EOF while parsing a list.

- **`EofWhileParsingObject`**

  EOF while parsing an object.

- **`EofWhileParsingString`**

  EOF while parsing a string.

- **`EofWhileParsingValue`**

  EOF while parsing a JSON value.

- **`ExpectedColon`**

  Expected this character to be a `':'`.

- **`ExpectedListCommaOrEnd`**

  Expected this character to be either a `','` or a `']'`.

- **`ExpectedObjectCommaOrEnd`**

  Expected this character to be either a `','` or a `'}'`.

- **`ExpectedSomeIdent`**

  Expected to parse either a `true`, `false`, or a `null`.

- **`ExpectedSomeValue`**

  Expected this character to start a JSON value.

- **`ExpectedDoubleQuote`**

  Expected this character to be a `"`.

- **`InvalidEscape`**

  Invalid hex escape code.

- **`InvalidNumber`**

  Invalid number.

- **`NumberOutOfRange`**

  Number is bigger than the maximum value of its type.

- **`InvalidUnicodeCodePoint`**

  Invalid unicode code point.

- **`ControlCharacterWhileParsingString`**

  Control character found while parsing a string.

- **`KeyMustBeAString`**

  Object key is not a string.

- **`ExpectedNumericKey`**

  Contents of key were supposed to be a number.

- **`FloatKeyMustBeFinite`**

  Object key is a non-finite float value.

- **`LoneLeadingSurrogateInHexEscape`**

  Lone leading surrogate in hex escape.

- **`TrailingComma`**

  JSON has a comma after the last value in an array or map.

- **`TrailingCharacters`**

  JSON has non-whitespace trailing characters after the value.

- **`UnexpectedEndOfHexEscape`**

  Unexpected end of hex escape.

- **`RecursionLimitExceeded`**

  Encountered nesting of JSON maps and arrays more than 128 layers deep.

#### Trait Implementations

##### `impl Display for ErrorCode`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for ErrorCode`

- `fn to_string(self: &Self) -> String`

## Functions

### `make_error`

```rust
fn make_error(msg: alloc::string::String) -> Error
```

### `parse_line_col`

```rust
fn parse_line_col(msg: &mut alloc::string::String) -> Option<(usize, usize)>
```

### `starts_with_digit`

```rust
fn starts_with_digit(slice: &str) -> bool
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

Alias for a `Result` with the error type `serde_json::Error`.

