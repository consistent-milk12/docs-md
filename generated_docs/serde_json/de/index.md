*[serde_json](../index.md) / [de](index.md)*

---

# Module `de`

Deserialize JSON data to a Rust data structure.

## Contents

- [Structs](#structs)
  - [`SliceRead`](#sliceread)
  - [`StrRead`](#strread)
  - [`IoRead`](#ioread)
  - [`Deserializer`](#deserializer)
  - [`SeqAccess`](#seqaccess)
  - [`MapAccess`](#mapaccess)
  - [`VariantAccess`](#variantaccess)
  - [`UnitVariantAccess`](#unitvariantaccess)
  - [`MapKey`](#mapkey)
  - [`StreamDeserializer`](#streamdeserializer)
- [Enums](#enums)
  - [`ParserNumber`](#parsernumber)
- [Traits](#traits)
  - [`Read`](#read)
- [Functions](#functions)
  - [`from_trait`](#from-trait)
  - [`from_reader`](#from-reader)
  - [`from_slice`](#from-slice)
  - [`from_str`](#from-str)
- [Macros](#macros)
  - [`overflow!`](#overflow)
  - [`deserialize_number!`](#deserialize-number)
  - [`if_checking_recursion_limit!`](#if-checking-recursion-limit)
  - [`check_recursion!`](#check-recursion)
  - [`deserialize_numeric_key!`](#deserialize-numeric-key)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SliceRead`](#sliceread) | struct |  |
| [`StrRead`](#strread) | struct |  |
| [`IoRead`](#ioread) | struct |  |
| [`Deserializer`](#deserializer) | struct | A structure that deserializes JSON into Rust values. |
| [`SeqAccess`](#seqaccess) | struct |  |
| [`MapAccess`](#mapaccess) | struct |  |
| [`VariantAccess`](#variantaccess) | struct |  |
| [`UnitVariantAccess`](#unitvariantaccess) | struct |  |
| [`MapKey`](#mapkey) | struct | Only deserialize from this after peeking a '"' byte! Otherwise it may deserialize invalid JSON successfully. |
| [`StreamDeserializer`](#streamdeserializer) | struct | Iterator that deserializes a stream into multiple JSON values. |
| [`ParserNumber`](#parsernumber) | enum |  |
| [`Read`](#read) | trait |  |
| [`from_trait`](#from-trait) | fn |  |
| [`from_reader`](#from-reader) | fn | Deserialize an instance of type `T` from an I/O stream of JSON. |
| [`from_slice`](#from-slice) | fn | Deserialize an instance of type `T` from bytes of JSON text. |
| [`from_str`](#from-str) | fn | Deserialize an instance of type `T` from a string of JSON text. |
| [`overflow!`](#overflow) | macro |  |
| [`deserialize_number!`](#deserialize-number) | macro |  |
| [`if_checking_recursion_limit!`](#if-checking-recursion-limit) | macro |  |
| [`check_recursion!`](#check-recursion) | macro |  |
| [`deserialize_numeric_key!`](#deserialize-numeric-key) | macro |  |

## Structs

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

- <span id="sliceread-position-of-index"></span>`fn position_of_index(&self, i: usize) -> Position` — [`Position`](../read/index.md#position)

- <span id="sliceread-skip-to-escape"></span>`fn skip_to_escape(&mut self, forbid_control_characters: bool)`

- <span id="sliceread-skip-to-escape-slow"></span>`fn skip_to_escape_slow(&mut self)`

- <span id="sliceread-parse-str-bytes"></span>`fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<Reference<'a, 's, T>>` — [`Result`](../error/index.md#result), [`Reference`](../read/index.md#reference)

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

### `Deserializer<R>`

```rust
struct Deserializer<R> {
    read: R,
    scratch: alloc::vec::Vec<u8>,
    remaining_depth: u8,
    disable_recursion_limit: bool,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:31-39`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L31-L39)*

A structure that deserializes JSON into Rust values.

#### Implementations

- <span id="deserializer-new"></span>`fn new(read: R) -> Self`

  Create a JSON deserializer from one of the possible serde_json input

  sources.

  

  When reading from a source against which short reads are not efficient, such

  as a `File`, you will want to apply your own buffering because serde_json

  will not buffer the input. See `std::io::BufReader`.

  

  Typically it is more convenient to use one of these methods instead:

  

    - Deserializer::from_str

    - Deserializer::from_slice

    - Deserializer::from_reader

#### Trait Implementations

##### `impl Any for Deserializer<R>`

- <span id="deserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Deserializer<R>`

- <span id="deserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Deserializer<R>`

- <span id="deserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: Read<'de>> Deserializer for &mut Deserializer<R>`

- <span id="mut-deserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

  Parses a JSON string as bytes. Note that this function does not check

  whether the bytes represent a valid UTF-8 string.

  

  The relevant part of the JSON specification is Section 8.2 of [RFC

  7159]:

  

  > When all the strings represented in a JSON text are composed entirely

  > of Unicode characters (however escaped), then that JSON text is

  > interoperable in the sense that all software implementations that

  > parse it will agree on the contents of names and of string values in

  > objects and arrays.

  >

  > However, the ABNF in this specification allows member names and string

  > values to contain bit sequences that cannot encode Unicode characters;

  > for example, "\uDEAD" (a single unpaired UTF-16 surrogate). Instances

  > of this have been observed, for example, when a library truncates a

  > UTF-16 string without checking whether the truncation split a

  > surrogate pair.  The behavior of software that receives JSON texts

  > containing such values is unpredictable; for example, implementations

  > might return different values for the length of a string value or even

  > suffer fatal runtime exceptions.

  

  The behavior of serde_json is specified to fail on non-UTF-8 strings

  when deserializing into Rust UTF-8 string types such as String, and

  succeed with the bytes representing the [WTF-8] encoding of code points

  when deserializing using this method.

  

  Escape sequences are processed as usual, and for `\uXXXX` escapes it is

  still checked if the hex number represents a valid Unicode code point.

  

  # Examples

  

  You can use this to parse JSON strings containing invalid UTF-8 bytes,

  or unpaired surrogates.

  

  ```rust

  use serde_bytes::ByteBuf;

  

  fn look_at_bytes() -> Result<(), serde_json::Error> {

      let json_data = b"\"some bytes: \xe5\x00\xe5\"";

      let bytes: ByteBuf = serde_json::from_slice(json_data)?;

  

      assert_eq!(b'\xe5', bytes[12]);

      assert_eq!(b'\0', bytes[13]);

      assert_eq!(b'\xe5', bytes[14]);

  

      Ok(())

  }

  

  look_at_bytes().unwrap();

  ```

  

  Backslash escape sequences like `\n` are still interpreted and required

  to be valid. `\u` escape sequences are required to represent a valid

  Unicode code point or lone surrogate.

  

  ```rust

  use serde_bytes::ByteBuf;

  

  fn look_at_bytes() -> Result<(), serde_json::Error> {

      let json_data = b"\"lone surrogate: \\uD801\"";

      let bytes: ByteBuf = serde_json::from_slice(json_data)?;

      let expected = b"lone surrogate: \xED\xA0\x81";

      assert_eq!(expected, bytes.as_slice());

      Ok(())

  }

  

  look_at_bytes();

  ```

- <span id="mut-deserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

  Parses a `null` as a None, and any other values as a `Some(...)`.

- <span id="mut-deserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

  Parses a newtype struct as the underlying value.

- <span id="mut-deserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

  Parses an enum as an object like `{"$KEY":$VALUE}`, where $VALUE is either a straight

  value, a `[..]`, or a `{..}`.

- <span id="mut-deserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

##### `impl<T> From for Deserializer<R>`

- <span id="deserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Deserializer<R>`

- <span id="deserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Deserializer<R>`

- <span id="deserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Deserializer<R>`

- <span id="deserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SeqAccess<'a, R: 'a>`

```rust
struct SeqAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:1917-1920`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L1917-L1920)*

#### Implementations

- <span id="seqaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl Any for SeqAccess<'a, R>`

- <span id="seqaccess-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeqAccess<'a, R>`

- <span id="seqaccess-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeqAccess<'a, R>`

- <span id="seqaccess-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SeqAccess<'a, R>`

- <span id="seqaccess-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeqAccess<'a, R>`

- <span id="seqaccess-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Read<'de> + 'a> SeqAccess for SeqAccess<'a, R>`

- <span id="seqaccess-seqaccess-type-error"></span>`type Error = Error`

- <span id="seqaccess-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>>` — [`Result`](../error/index.md#result)

##### `impl<U> TryFrom for SeqAccess<'a, R>`

- <span id="seqaccess-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seqaccess-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeqAccess<'a, R>`

- <span id="seqaccess-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seqaccess-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapAccess<'a, R: 'a>`

```rust
struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:1970-1973`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L1970-L1973)*

#### Implementations

- <span id="mapaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl Any for MapAccess<'a, R>`

- <span id="mapaccess-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapAccess<'a, R>`

- <span id="mapaccess-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapAccess<'a, R>`

- <span id="mapaccess-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapAccess<'a, R>`

- <span id="mapaccess-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapAccess<'a, R>`

- <span id="mapaccess-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Read<'de> + 'a> MapAccess for MapAccess<'a, R>`

- <span id="mapaccess-mapaccess-type-error"></span>`type Error = Error`

- <span id="mapaccess-mapaccess-next-key-seed"></span>`fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as >::Value>>` — [`Result`](../error/index.md#result)

- <span id="mapaccess-mapaccess-next-value-seed"></span>`fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

##### `impl<U> TryFrom for MapAccess<'a, R>`

- <span id="mapaccess-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapaccess-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapAccess<'a, R>`

- <span id="mapaccess-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapaccess-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VariantAccess<'a, R: 'a>`

```rust
struct VariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:2035-2037`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2035-L2037)*

#### Implementations

- <span id="variantaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl Any for VariantAccess<'a, R>`

- <span id="variantaccess-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VariantAccess<'a, R>`

- <span id="variantaccess-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VariantAccess<'a, R>`

- <span id="variantaccess-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: Read<'de> + 'a> EnumAccess for VariantAccess<'a, R>`

- <span id="variantaccess-enumaccess-type-error"></span>`type Error = Error`

- <span id="variantaccess-enumaccess-type-variant"></span>`type Variant = VariantAccess<'a, R>`

- <span id="variantaccess-enumaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../error/index.md#result)

##### `impl<T> From for VariantAccess<'a, R>`

- <span id="variantaccess-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VariantAccess<'a, R>`

- <span id="variantaccess-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VariantAccess<'a, R>`

- <span id="variantaccess-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variantaccess-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VariantAccess<'a, R>`

- <span id="variantaccess-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variantaccess-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R: Read<'de> + 'a> VariantAccess for VariantAccess<'a, R>`

- <span id="variantaccess-variantaccess-type-error"></span>`type Error = Error`

- <span id="variantaccess-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="variantaccess-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value>` — [`Result`](../error/index.md#result)

- <span id="variantaccess-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="variantaccess-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

### `UnitVariantAccess<'a, R: 'a>`

```rust
struct UnitVariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:2088-2090`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2088-L2090)*

#### Implementations

- <span id="unitvariantaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl Any for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: Read<'de> + 'a> EnumAccess for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-enumaccess-type-error"></span>`type Error = Error`

- <span id="unitvariantaccess-enumaccess-type-variant"></span>`type Variant = UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-enumaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../error/index.md#result)

##### `impl<T> From for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitvariantaccess-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitvariantaccess-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R: Read<'de> + 'a> VariantAccess for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-variantaccess-type-error"></span>`type Error = Error`

- <span id="unitvariantaccess-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="unitvariantaccess-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value>` — [`Result`](../error/index.md#result)

- <span id="unitvariantaccess-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="unitvariantaccess-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

### `MapKey<'a, R: 'a>`

```rust
struct MapKey<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:2151-2153`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2151-L2153)*

Only deserialize from this after peeking a '"' byte! Otherwise it may
deserialize invalid JSON successfully.

#### Implementations

- <span id="mapkey-deserialize-number"></span>`fn deserialize_number<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for MapKey<'a, R>`

- <span id="mapkey-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapKey<'a, R>`

- <span id="mapkey-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapKey<'a, R>`

- <span id="mapkey-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Deserializer for MapKey<'a, R>`

- <span id="mapkey-deserializer-type-error"></span>`type Error = Error`

- <span id="mapkey-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl<T> From for MapKey<'a, R>`

- <span id="mapkey-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapKey<'a, R>`

- <span id="mapkey-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MapKey<'a, R>`

- <span id="mapkey-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapkey-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapKey<'a, R>`

- <span id="mapkey-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapkey-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`serde_json-1.0.145/src/de.rs:2349-2355`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2349-L2355)*

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

- <span id="streamdeserializer-new"></span>`fn new(read: R) -> Self`

  Create a JSON stream deserializer from one of the possible serde_json

  input sources.

  

  Typically it is more convenient to use one of these methods instead:

  

    - Deserializer::from_str(...).into_iter()

    - Deserializer::from_slice(...).into_iter()

    - Deserializer::from_reader(...).into_iter()

- <span id="streamdeserializer-byte-offset"></span>`fn byte_offset(&self) -> usize`

  Returns the number of bytes so far deserialized into a successful `T`.

  

  If a stream deserializer returns an EOF error, new data can be joined to

  `old_data[stream.byte_offset()..]` to try again.

  

  ```rust

  let data = b"[0] [1] [";

  

  let de = serde_json::Deserializer::from_slice(data);

  let mut stream = de.into_iter::<Vec<i32>>();

  assert_eq!(0, stream.byte_offset());

  

  println!("{:?}", stream.next()); // [0]

  assert_eq!(3, stream.byte_offset());

  

  println!("{:?}", stream.next()); // [1]

  assert_eq!(7, stream.byte_offset());

  

  println!("{:?}", stream.next()); // error

  assert_eq!(8, stream.byte_offset());

  

  // If err.is_eof(), can join the remaining data to new data and continue.

  let remaining = &data[stream.byte_offset()..];

  ```

  

  *Note:* In the future this method may be changed to return the number of

  bytes so far deserialized into a successful T *or* syntactically valid

  JSON skipped over due to a type error. See [serde-rs/json#70] for an

  example illustrating this.

- <span id="streamdeserializer-peek-end-of-value"></span>`fn peek_end_of_value(&mut self) -> Result<()>` — [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl<T> Any for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<R, T> FusedIterator for StreamDeserializer<'de, R, T>`

##### `impl<T, U> Into for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamdeserializer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamdeserializer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R, T> Iterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-iterator-type-item"></span>`type Item = Result<T, Error>`

- <span id="streamdeserializer-iterator-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](../error/index.md#result)

##### `impl<T, U> TryFrom for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ParserNumber`

```rust
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:109-115`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L109-L115)*

#### Implementations

- <span id="parsernumber-visit"></span>`fn visit<'de, V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="parsernumber-invalid-type"></span>`fn invalid_type(self, exp: &dyn Expected) -> Error` — [`Error`](../error/index.md#error)

#### Trait Implementations

##### `impl Any for ParserNumber`

- <span id="parsernumber-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParserNumber`

- <span id="parsernumber-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParserNumber`

- <span id="parsernumber-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ParserNumber`

- <span id="parsernumber-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParserNumber`

- <span id="parsernumber-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ParserNumber`

- <span id="parsernumber-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsernumber-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParserNumber`

- <span id="parsernumber-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsernumber-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- [`IoRead`](../read/index.md#ioread)
- [`SliceRead`](../read/index.md#sliceread)
- [`StrRead`](../read/index.md#strread)
- `&mut R`

## Functions

### `from_trait`

```rust
fn from_trait<'de, R, T>(read: R) -> crate::error::Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>
```

*Defined in [`serde_json-1.0.145/src/de.rs:2495-2506`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2495-L2506)*

### `from_reader`

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned
```

*Defined in [`serde_json-1.0.145/src/de.rs:2612-2618`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2612-L2618)*

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a `File`, you will want to apply your own buffering because serde_json
will not buffer the input. See `std::io::BufReader`.

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`](#deserializer).

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`](#from-str)
or [`from_slice`](#from-slice) on it. See [issue #160].


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

*Defined in [`serde_json-1.0.145/src/de.rs:2655-2660`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2655-L2660)*

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
    // The type of `j` is `&[u8]`
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

*Defined in [`serde_json-1.0.145/src/de.rs:2697-2702`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2697-L2702)*

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

## Macros

### `overflow!`

*Defined in [`serde_json-1.0.145/src/de.rs:101-107`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L101-L107)*

### `deserialize_number!`

*Defined in [`serde_json-1.0.145/src/de.rs:1339-1352`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L1339-L1352)*

### `if_checking_recursion_limit!`

*Defined in [`serde_json-1.0.145/src/de.rs:1362-1368`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L1362-L1368)*

### `check_recursion!`

*Defined in [`serde_json-1.0.145/src/de.rs:1370-1385`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L1370-L1385)*

### `deserialize_numeric_key!`

*Defined in [`serde_json-1.0.145/src/de.rs:2155-2187`](../../../.source_1765521767/serde_json-1.0.145/src/de.rs#L2155-L2187)*

