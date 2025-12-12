*[serde_json](../index.md) / [error](index.md)*

---

# Module `error`

When serializing or deserializing JSON goes wrong.

## Contents

- [Structs](#structs)
  - [`Error`](#error)
  - [`ErrorImpl`](#errorimpl)
  - [`JsonUnexpected`](#jsonunexpected)
- [Enums](#enums)
  - [`Category`](#category)
  - [`ErrorCode`](#errorcode)
- [Functions](#functions)
  - [`make_error`](#make-error)
  - [`parse_line_col`](#parse-line-col)
  - [`starts_with_digit`](#starts-with-digit)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | This type represents all possible errors that can occur when serializing or deserializing JSON data. |
| [`ErrorImpl`](#errorimpl) | struct |  |
| [`JsonUnexpected`](#jsonunexpected) | struct |  |
| [`Category`](#category) | enum | Categorizes the cause of a `serde_json::Error`. |
| [`ErrorCode`](#errorcode) | enum |  |
| [`make_error`](#make-error) | fn |  |
| [`parse_line_col`](#parse-line-col) | fn |  |
| [`starts_with_digit`](#starts-with-digit) | fn |  |
| [`Result`](#result) | type | Alias for a `Result` with the error type `serde_json::Error`. |

## Structs

### `Error`

```rust
struct Error {
    err: alloc::boxed::Box<ErrorImpl>,
}
```

*Defined in [`serde_json-1.0.145/src/error.rs:17-22`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L17-L22)*

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

- <span id="error-classify"></span>`fn classify(&self) -> Category` — [`Category`](#category)

- <span id="error-is-io"></span>`fn is_io(&self) -> bool`

- <span id="error-is-syntax"></span>`fn is_syntax(&self) -> bool`

- <span id="error-is-data"></span>`fn is_data(&self) -> bool`

- <span id="error-is-eof"></span>`fn is_eof(&self) -> bool`

- <span id="error-io-error-kind"></span>`fn io_error_kind(&self) -> Option<ErrorKind>` — [`ErrorKind`](../io/index.md#errorkind)

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

### `ErrorImpl`

```rust
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}
```

*Defined in [`serde_json-1.0.145/src/error.rs:230-234`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L230-L234)*

#### Trait Implementations

##### `impl Display for ErrorImpl`

- <span id="errorimpl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ErrorImpl`

- <span id="errorimpl-to-string"></span>`fn to_string(&self) -> String`

### `JsonUnexpected<'a>`

```rust
struct JsonUnexpected<'a>(de::Unexpected<'a>);
```

*Defined in [`serde_json-1.0.145/src/error.rs:465`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L465)*

#### Trait Implementations

##### `impl Display for JsonUnexpected<'a>`

- <span id="jsonunexpected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for JsonUnexpected<'a>`

- <span id="jsonunexpected-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`serde_json-1.0.145/src/error.rs:166-185`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L166-L185)*

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

- <span id="category-clone"></span>`fn clone(&self) -> Category` — [`Category`](#category)

##### `impl Copy for Category`

##### `impl Debug for Category`

- <span id="category-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Category`

##### `impl PartialEq for Category`

- <span id="category-eq"></span>`fn eq(&self, other: &Category) -> bool` — [`Category`](#category)

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

*Defined in [`serde_json-1.0.145/src/error.rs:236-311`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L236-L311)*

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

- <span id="errorcode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ErrorCode`

- <span id="errorcode-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `make_error`

```rust
fn make_error(msg: alloc::string::String) -> Error
```

*Defined in [`serde_json-1.0.145/src/error.rs:483-492`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L483-L492)*

### `parse_line_col`

```rust
fn parse_line_col(msg: &mut alloc::string::String) -> Option<(usize, usize)>
```

*Defined in [`serde_json-1.0.145/src/error.rs:494-534`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L494-L534)*

### `starts_with_digit`

```rust
fn starts_with_digit(slice: &str) -> bool
```

*Defined in [`serde_json-1.0.145/src/error.rs:536-541`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L536-L541)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

*Defined in [`serde_json-1.0.145/src/error.rs:25`](../../../.source_1765210505/serde_json-1.0.145/src/error.rs#L25)*

Alias for a `Result` with the error type `serde_json::Error`.

