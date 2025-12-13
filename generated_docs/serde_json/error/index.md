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

*Defined in [`serde_json-1.0.145/src/error.rs:17-22`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L17-L22)*

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

- <span id="error-classify"></span>`fn classify(&self) -> Category` — [`Category`](#category)

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

- <span id="error-io-error-kind"></span>`fn io_error_kind(&self) -> Option<ErrorKind>` — [`ErrorKind`](../io/index.md#errorkind)

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

### `ErrorImpl`

```rust
struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}
```

*Defined in [`serde_json-1.0.145/src/error.rs:230-234`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L230-L234)*

#### Trait Implementations

##### `impl Any for ErrorImpl`

- <span id="errorimpl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorImpl`

- <span id="errorimpl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorImpl`

- <span id="errorimpl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for ErrorImpl`

- <span id="errorimpl-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ErrorImpl`

- <span id="errorimpl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorImpl`

- <span id="errorimpl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ErrorImpl`

- <span id="errorimpl-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ErrorImpl`

- <span id="errorimpl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorimpl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorImpl`

- <span id="errorimpl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorimpl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JsonUnexpected<'a>`

```rust
struct JsonUnexpected<'a>(de::Unexpected<'a>);
```

*Defined in [`serde_json-1.0.145/src/error.rs:465`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L465)*

#### Trait Implementations

##### `impl Any for JsonUnexpected<'a>`

- <span id="jsonunexpected-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JsonUnexpected<'a>`

- <span id="jsonunexpected-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JsonUnexpected<'a>`

- <span id="jsonunexpected-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for JsonUnexpected<'a>`

- <span id="jsonunexpected-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for JsonUnexpected<'a>`

- <span id="jsonunexpected-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JsonUnexpected<'a>`

- <span id="jsonunexpected-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for JsonUnexpected<'a>`

- <span id="jsonunexpected-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for JsonUnexpected<'a>`

- <span id="jsonunexpected-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jsonunexpected-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JsonUnexpected<'a>`

- <span id="jsonunexpected-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jsonunexpected-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`serde_json-1.0.145/src/error.rs:166-185`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L166-L185)*

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

##### `impl Any for Category`

- <span id="category-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Category`

- <span id="category-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Category`

- <span id="category-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Category`

- <span id="category-clone"></span>`fn clone(&self) -> Category` — [`Category`](#category)

##### `impl CloneToUninit for Category`

- <span id="category-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Category`

##### `impl Debug for Category`

- <span id="category-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Category`

##### `impl<T> From for Category`

- <span id="category-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Category`

- <span id="category-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Category`

- <span id="category-partialeq-eq"></span>`fn eq(&self, other: &Category) -> bool` — [`Category`](#category)

##### `impl StructuralPartialEq for Category`

##### `impl ToOwned for Category`

- <span id="category-toowned-type-owned"></span>`type Owned = T`

- <span id="category-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="category-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Category`

- <span id="category-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="category-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Category`

- <span id="category-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="category-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`serde_json-1.0.145/src/error.rs:236-311`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L236-L311)*

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

##### `impl Any for ErrorCode`

- <span id="errorcode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorCode`

- <span id="errorcode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorCode`

- <span id="errorcode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for ErrorCode`

- <span id="errorcode-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ErrorCode`

- <span id="errorcode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorCode`

- <span id="errorcode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ErrorCode`

- <span id="errorcode-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ErrorCode`

- <span id="errorcode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorcode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorCode`

- <span id="errorcode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorcode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `make_error`

```rust
fn make_error(msg: alloc::string::String) -> Error
```

*Defined in [`serde_json-1.0.145/src/error.rs:483-492`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L483-L492)*

### `parse_line_col`

```rust
fn parse_line_col(msg: &mut alloc::string::String) -> Option<(usize, usize)>
```

*Defined in [`serde_json-1.0.145/src/error.rs:494-534`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L494-L534)*

### `starts_with_digit`

```rust
fn starts_with_digit(slice: &str) -> bool
```

*Defined in [`serde_json-1.0.145/src/error.rs:536-541`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L536-L541)*

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

*Defined in [`serde_json-1.0.145/src/error.rs:25`](../../../.source_1765521767/serde_json-1.0.145/src/error.rs#L25)*

Alias for a `Result` with the error type `serde_json::Error`.

