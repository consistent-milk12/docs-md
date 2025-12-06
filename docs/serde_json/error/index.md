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

- `fn io_error_kind(self: &Self) -> Option<ErrorKind>`

#### Trait Implementations

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- `fn custom<T: Display>(msg: T) -> Error` — [`Error`](#error)

- `fn invalid_type(unexp: de::Unexpected<'_>, exp: &dyn de::Expected) -> Self`

- `fn invalid_value(unexp: de::Unexpected<'_>, exp: &dyn de::Expected) -> Self`

##### `impl<T> ToString for Error`

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

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

Alias for a `Result` with the error type `serde_json::Error`.

