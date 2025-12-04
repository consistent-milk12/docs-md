*[serde_json](../index.md) / [error](index.md)*

---

# Module `error`

When serializing or deserializing JSON goes wrong.

## Structs

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

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl Error`

- `fn custom<T: Display>(msg: T) -> Error`

- `fn invalid_type(unexp: de::Unexpected<'_>, exp: &dyn de::Expected) -> Self`

- `fn invalid_value(unexp: de::Unexpected<'_>, exp: &dyn de::Expected) -> Self`

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

- `fn clone(self: &Self) -> Category`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Category) -> bool`

##### `impl StructuralPartialEq`

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

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

Alias for a `Result` with the error type `serde_json::Error`.

