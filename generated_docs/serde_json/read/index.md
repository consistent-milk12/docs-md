*[serde_json](../index.md) / [read](index.md)*

---

# Module `read`

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Position`](#position)
  - [`IoRead`](#ioread)
  - [`SliceRead`](#sliceread)
  - [`StrRead`](#strread)
- [Enums](#enums)
  - [`Reference`](#reference)
- [Traits](#traits)
  - [`Read`](#read)
  - [`Fused`](#fused)
- [Functions](#functions)
  - [`is_escape`](#is-escape)
  - [`next_or_eof`](#next-or-eof)
  - [`peek_or_eof`](#peek-or-eof)
  - [`error`](#error)
  - [`as_str`](#as-str)
  - [`parse_escape`](#parse-escape)
  - [`parse_unicode_escape`](#parse-unicode-escape)
  - [`push_wtf8_codepoint`](#push-wtf8-codepoint)
  - [`ignore_escape`](#ignore-escape)
  - [`decode_hex_val_slow`](#decode-hex-val-slow)
  - [`build_hex_table`](#build-hex-table)
  - [`decode_four_hex_digits`](#decode-four-hex-digits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Position`](#position) | struct |  |
| [`IoRead`](#ioread) | struct | JSON input source that reads from a std::io input stream. |
| [`SliceRead`](#sliceread) | struct | JSON input source that reads from a slice of bytes. |
| [`StrRead`](#strread) | struct | JSON input source that reads from a UTF-8 string. |
| [`Reference`](#reference) | enum |  |
| [`Read`](#read) | trait | Trait used by the deserializer for iterating over input. |
| [`Fused`](#fused) | trait | Marker for whether StreamDeserializer can implement FusedIterator. |
| [`is_escape`](#is-escape) | fn |  |
| [`next_or_eof`](#next-or-eof) | fn |  |
| [`peek_or_eof`](#peek-or-eof) | fn |  |
| [`error`](#error) | fn |  |
| [`as_str`](#as-str) | fn |  |
| [`parse_escape`](#parse-escape) | fn | Parses a JSON escape sequence and appends it into the scratch space. |
| [`parse_unicode_escape`](#parse-unicode-escape) | fn | Parses a JSON \u escape and appends it into the scratch space. |
| [`push_wtf8_codepoint`](#push-wtf8-codepoint) | fn | Adds a WTF-8 codepoint to the end of the buffer. |
| [`ignore_escape`](#ignore-escape) | fn | Parses a JSON escape sequence and discards the value. |
| [`decode_hex_val_slow`](#decode-hex-val-slow) | fn |  |
| [`build_hex_table`](#build-hex-table) | fn |  |
| [`decode_four_hex_digits`](#decode-four-hex-digits) | fn |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Position`

```rust
struct Position {
    pub line: usize,
    pub column: usize,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:119-122`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L119-L122)*

#### Trait Implementations

##### `impl Any for Position`

- <span id="position-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Position`

- <span id="position-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Position`

- <span id="position-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Position`

- <span id="position-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Position`

- <span id="position-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Position`

- <span id="position-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="position-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Position`

- <span id="position-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="position-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IoRead<R>`

```rust
struct IoRead<R>
where
    R: io::Read {
    iter: crate::iter::LineColIterator<io::Bytes<R>>,
    ch: Option<u8>,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:149-158`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L149-L158)*

JSON input source that reads from a std::io input stream.

#### Fields

- **`ch`**: `Option<u8>`

  Temporary storage of peeked byte.

#### Implementations

- <span id="ioread-new"></span>`fn new(reader: R) -> Self`

  Create a JSON input source to read from a std::io input stream.

  

  When reading from a source against which short reads are not efficient, such

  as a `File`, you will want to apply your own buffering because serde_json

  will not buffer the input. See `std::io::BufReader`.

#### Trait Implementations

##### `impl Any for IoRead<R>`

- <span id="ioread-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IoRead<R>`

- <span id="ioread-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IoRead<R>`

- <span id="ioread-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IoRead<R>`

- <span id="ioread-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IoRead<R>`

- <span id="ioread-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Read for IoRead<R>`

##### `impl<R> Sealed for IoRead<R>`

##### `impl<U> TryFrom for IoRead<R>`

- <span id="ioread-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ioread-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IoRead<R>`

- <span id="ioread-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ioread-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SliceRead<'a>`

```rust
struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:164-170`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L164-L170)*

JSON input source that reads from a slice of bytes.

#### Fields

- **`index`**: `usize`

  Index of the *next* byte that will be returned by next() or peek().

#### Implementations

- <span id="sliceread-new"></span>`fn new(slice: &'a [u8]) -> Self`

  Create a JSON input source to read from a slice of bytes.

- <span id="sliceread-position-of-index"></span>`fn position_of_index(&self, i: usize) -> Position` — [`Position`](#position)

- <span id="sliceread-skip-to-escape"></span>`fn skip_to_escape(&mut self, forbid_control_characters: bool)`

- <span id="sliceread-skip-to-escape-slow"></span>`fn skip_to_escape_slow(&mut self)`

- <span id="sliceread-parse-str-bytes"></span>`fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<Reference<'a, 's, T>>` — [`Result`](../error/index.md#result), [`Reference`](#reference)

  The big optimization here over IoRead is that if the string contains no

  backslash escape sequences, the returned &str is a slice of the raw JSON

  data so we avoid copying into the scratch space.

#### Trait Implementations

##### `impl Any for SliceRead<'a>`

- <span id="sliceread-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SliceRead<'a>`

- <span id="sliceread-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SliceRead<'a>`

- <span id="sliceread-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SliceRead<'a>`

- <span id="sliceread-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Fused for SliceRead<'a>`

##### `impl<U> Into for SliceRead<'a>`

- <span id="sliceread-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Read for SliceRead<'a>`

##### `impl Sealed for SliceRead<'a>`

##### `impl<U> TryFrom for SliceRead<'a>`

- <span id="sliceread-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sliceread-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SliceRead<'a>`

- <span id="sliceread-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sliceread-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StrRead<'a>`

```rust
struct StrRead<'a> {
    delegate: SliceRead<'a>,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:175-179`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L175-L179)*

JSON input source that reads from a UTF-8 string.

#### Implementations

- <span id="strread-new"></span>`fn new(s: &'a str) -> Self`

  Create a JSON input source to read from a UTF-8 string.

#### Trait Implementations

##### `impl Any for StrRead<'a>`

- <span id="strread-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrRead<'a>`

- <span id="strread-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrRead<'a>`

- <span id="strread-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StrRead<'a>`

- <span id="strread-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Fused for StrRead<'a>`

##### `impl<U> Into for StrRead<'a>`

- <span id="strread-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Read for StrRead<'a>`

##### `impl Sealed for StrRead<'a>`

##### `impl<U> TryFrom for StrRead<'a>`

- <span id="strread-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strread-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StrRead<'a>`

- <span id="strread-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strread-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Reference<'b, 'c, T>`

```rust
enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static {
    Borrowed(&'b T),
    Copied(&'c T),
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:124-130`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L124-L130)*

#### Trait Implementations

##### `impl<T> Any for Reference<'b, 'c, T>`

- <span id="reference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Reference<'b, 'c, T>`

- <span id="reference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Reference<'b, 'c, T>`

- <span id="reference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Deref for Reference<'b, 'c, T>`

- <span id="reference-deref-type-target"></span>`type Target = T`

- <span id="reference-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for Reference<'b, 'c, T>`

- <span id="reference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Reference<'b, 'c, T>`

- <span id="reference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for Reference<'b, 'c, T>`

- <span id="reference-receiver-type-target"></span>`type Target = T`

##### `impl<T, U> TryFrom for Reference<'b, 'c, T>`

- <span id="reference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Reference<'b, 'c, T>`

- <span id="reference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Read<'de>`

```rust
trait Read<'de>: private::Sealed { ... }
```

*Defined in [`serde_json-1.0.145/src/read.rs:28-117`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L28-L117)*

Trait used by the deserializer for iterating over input. This is manually
"specialized" for iterating over `&[u8]`. Once feature(specialization) is
stable we can use actual specialization.

This trait is sealed and cannot be implemented for types outside of
`serde_json`.

#### Implementors

- [`IoRead`](#ioread)
- [`SliceRead`](#sliceread)
- [`StrRead`](#strread)
- `&mut R`

### `Fused`

```rust
trait Fused: private::Sealed { ... }
```

*Defined in [`serde_json-1.0.145/src/read.rs:832`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L832)*

Marker for whether StreamDeserializer can implement FusedIterator.

#### Implementors

- [`SliceRead`](#sliceread)
- [`StrRead`](#strread)

## Functions

### `is_escape`

```rust
fn is_escape(ch: u8, including_control_characters: bool) -> bool
```

*Defined in [`serde_json-1.0.145/src/read.rs:836-838`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L836-L838)*

### `next_or_eof`

```rust
fn next_or_eof<'de, R>(read: &mut R) -> crate::error::Result<u8>
where
    R: ?Sized + Read<'de>
```

*Defined in [`serde_json-1.0.145/src/read.rs:840-848`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L840-L848)*

### `peek_or_eof`

```rust
fn peek_or_eof<'de, R>(read: &mut R) -> crate::error::Result<u8>
where
    R: ?Sized + Read<'de>
```

*Defined in [`serde_json-1.0.145/src/read.rs:850-858`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L850-L858)*

### `error`

```rust
fn error<'de, R, T>(read: &R, reason: crate::error::ErrorCode) -> crate::error::Result<T>
where
    R: ?Sized + Read<'de>
```

*Defined in [`serde_json-1.0.145/src/read.rs:860-866`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L860-L866)*

### `as_str`

```rust
fn as_str<'de, 's, R: Read<'de>>(read: &R, slice: &'s [u8]) -> crate::error::Result<&'s str>
```

*Defined in [`serde_json-1.0.145/src/read.rs:868-870`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L868-L870)*

### `parse_escape`

```rust
fn parse_escape<'de, R: Read<'de>>(read: &mut R, validate: bool, scratch: &mut alloc::vec::Vec<u8>) -> crate::error::Result<()>
```

*Defined in [`serde_json-1.0.145/src/read.rs:874-895`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L874-L895)*

Parses a JSON escape sequence and appends it into the scratch space. Assumes
the previous byte read was a backslash.

### `parse_unicode_escape`

```rust
fn parse_unicode_escape<'de, R: Read<'de>>(read: &mut R, validate: bool, scratch: &mut alloc::vec::Vec<u8>) -> crate::error::Result<()>
```

*Defined in [`serde_json-1.0.145/src/read.rs:900-973`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L900-L973)*

Parses a JSON \u escape and appends it into the scratch space. Assumes `\u`
has just been read.

### `push_wtf8_codepoint`

```rust
fn push_wtf8_codepoint(n: u32, scratch: &mut alloc::vec::Vec<u8>)
```

*Defined in [`serde_json-1.0.145/src/read.rs:978-1021`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L978-L1021)*

Adds a WTF-8 codepoint to the end of the buffer. This is a more efficient
implementation of String::push. The codepoint may be a surrogate.

### `ignore_escape`

```rust
fn ignore_escape<'de, R>(read: &mut R) -> crate::error::Result<()>
where
    R: ?Sized + Read<'de>
```

*Defined in [`serde_json-1.0.145/src/read.rs:1025-1048`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L1025-L1048)*

Parses a JSON escape sequence and discards the value. Assumes the previous
byte read was a backslash.

### `decode_hex_val_slow`

```rust
const fn decode_hex_val_slow(val: u8) -> Option<u8>
```

*Defined in [`serde_json-1.0.145/src/read.rs:1050-1057`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L1050-L1057)*

### `build_hex_table`

```rust
const fn build_hex_table(shift: usize) -> [i16; 256]
```

*Defined in [`serde_json-1.0.145/src/read.rs:1059-1070`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L1059-L1070)*

### `decode_four_hex_digits`

```rust
fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16>
```

*Defined in [`serde_json-1.0.145/src/read.rs:1075-1089`](../../../.source_1765521767/serde_json-1.0.145/src/read.rs#L1075-L1089)*

