# Crate `utf8`

## Structs

### `LossyDecoder<F: FnMut(&str)>`

```rust
struct LossyDecoder<F: FnMut(&str)> {
    // [REDACTED: Private Fields]
}
```

A push-based, lossy decoder for UTF-8.
Errors are replaced with the U+FFFD replacement character.

Users “push” bytes into the decoder, which in turn “pushes” `&str` slices into a callback.

For example, `String::from_utf8_lossy` (but returning `String` instead of `Cow`)
can be rewritten as:

```rust
fn string_from_utf8_lossy(input: &[u8]) -> String {
    let mut string = String::new();
    utf8::LossyDecoder::new(|s| string.push_str(s)).feed(input);
    string
}
```

**Note:** Dropping the decoder signals the end of the input:
If the last input chunk ended with an incomplete byte sequence for a code point,
this is an error and a replacement character is emitted.
Use `std::mem::forget` to inhibit this behavior.

#### Implementations

- `fn new(push_str: F) -> Self`
  Create a new decoder from a callback.

- `fn feed(self: &mut Self, input: &[u8])`
  Feed one chunk of input into the decoder.

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

##### `impl Drop<F: FnMut(&str)>`

- `fn drop(self: &mut Self)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `BufReadDecoder<B: BufRead>`

```rust
struct BufReadDecoder<B: BufRead> {
    // [REDACTED: Private Fields]
}
```

Wraps a `std::io::BufRead` buffered byte stream and decode it as UTF-8.

#### Implementations

- `fn read_to_string_lossy(buf_read: B) -> io::Result<String>`
  This is to `Read::read_to_string` what `String::from_utf8_lossy` is to `String::from_utf8`.

- `fn new(buf_read: B) -> Self`

- `fn next_lossy(self: &mut Self) -> Option<io::Result<&str>>`
  Same as `BufReadDecoder::next_strict`, but replace UTF-8 errors with U+FFFD.

- `fn next_strict(self: &mut Self) -> Option<Result<&str, BufReadDecoderError<'_>>>`
  Decode and consume the next chunk of UTF-8 input.

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

### `Incomplete`

```rust
struct Incomplete {
    pub buffer: [u8; 4],
    pub buffer_len: u8,
}
```

#### Implementations

- `fn empty() -> Self`

- `fn is_empty(self: &Self) -> bool`

- `fn new(bytes: &[u8]) -> Self`

- `fn try_complete<'input>(self: &mut Self, input: &'input [u8]) -> Option<(Result<&str, &[u8]>, &'input [u8])>`
  * `None`: still incomplete, call `try_complete` again with more input.

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

- `fn clone(self: &Self) -> Incomplete`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

## Enums

### `BufReadDecoderError<'a>`

```rust
enum BufReadDecoderError<'a> {
    InvalidByteSequence(&'a [u8]),
    Io(io::Error),
}
```

#### Variants

- **`InvalidByteSequence`**

  Represents one UTF-8 error in the byte stream.
  
  In lossy decoding, each such error should be replaced with U+FFFD.
  (See `BufReadDecoder::next_lossy` and `BufReadDecoderError::lossy`.)

- **`Io`**

  An I/O error from the underlying byte stream

#### Implementations

- `fn lossy(self: Self) -> Result<&'static str, io::Error>`
  Replace UTF-8 errors with U+FFFD

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

##### `impl Display<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error<'a>`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DecodeError<'a>`

```rust
enum DecodeError<'a> {
    Invalid {
        valid_prefix: &'a str,
        invalid_sequence: &'a [u8],
        remaining_input: &'a [u8],
    },
    Incomplete {
        valid_prefix: &'a str,
        incomplete_suffix: Incomplete,
    },
}
```

#### Variants

- **`Invalid`**

  In lossy decoding insert `valid_prefix`, then `"\u{FFFD}"`,
  then call `decode()` again with `remaining_input`.

- **`Incomplete`**

  Call the `incomplete_suffix.try_complete` method with more input when available.
  If no more input is available, this is an invalid byte sequence.

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

- `fn clone(self: &Self) -> DecodeError<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl Display<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error<'a>`

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

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `decode`

```rust
fn decode(input: &[u8]) -> Result<&str, DecodeError<'_>>
```

## Constants

### `REPLACEMENT_CHARACTER`

```rust
const REPLACEMENT_CHARACTER: &'static str;
```

The replacement character, U+FFFD. In lossy decoding, insert it for every decoding error.

