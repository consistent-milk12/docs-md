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
  - [`from_trait`](#from_trait)
  - [`from_reader`](#from_reader)
  - [`from_slice`](#from_slice)
  - [`from_str`](#from_str)
- [Macros](#macros)
  - [`overflow!`](#overflow)
  - [`deserialize_number!`](#deserialize_number)
  - [`if_checking_recursion_limit!`](#if_checking_recursion_limit)
  - [`check_recursion!`](#check_recursion)
  - [`deserialize_numeric_key!`](#deserialize_numeric_key)

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
| [`from_trait`](#from_trait) | fn |  |
| [`from_reader`](#from_reader) | fn | Deserialize an instance of type `T` from an I/O stream of JSON. |
| [`from_slice`](#from_slice) | fn | Deserialize an instance of type `T` from bytes of JSON text. |
| [`from_str`](#from_str) | fn | Deserialize an instance of type `T` from a string of JSON text. |
| [`overflow!`](#overflow) | macro |  |
| [`deserialize_number!`](#deserialize_number) | macro |  |
| [`if_checking_recursion_limit!`](#if_checking_recursion_limit) | macro |  |
| [`check_recursion!`](#check_recursion) | macro |  |
| [`deserialize_numeric_key!`](#deserialize_numeric_key) | macro |  |

## Structs

### `SliceRead<'a>`

```rust
struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:164-170`](../../../.source_1765210505/serde_json-1.0.145/src/read.rs#L164-L170)*

JSON input source that reads from a slice of bytes.

#### Fields

- **`index`**: `usize`

  Index of the *next* byte that will be returned by next() or peek().

#### Implementations

- <span id="sliceread-new"></span>`fn new(slice: &'a [u8]) -> Self`

- <span id="sliceread-position-of-index"></span>`fn position_of_index(&self, i: usize) -> Position` — [`Position`](../read/index.md#position)

- <span id="sliceread-skip-to-escape"></span>`fn skip_to_escape(&mut self, forbid_control_characters: bool)`

- <span id="sliceread-skip-to-escape-slow"></span>`fn skip_to_escape_slow(&mut self)`

- <span id="sliceread-parse-str-bytes"></span>`fn parse_str_bytes<'s, T, F>(self: &'s mut Self, scratch: &'s mut Vec<u8>, validate: bool, result: F) -> Result<Reference<'a, 's, T>>` — [`Result`](../error/index.md#result), [`Reference`](../read/index.md#reference)

#### Trait Implementations

##### `impl Fused for SliceRead<'a>`

##### `impl Read for SliceRead<'a>`

##### `impl Sealed for SliceRead<'a>`

### `StrRead<'a>`

```rust
struct StrRead<'a> {
    delegate: SliceRead<'a>,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:175-179`](../../../.source_1765210505/serde_json-1.0.145/src/read.rs#L175-L179)*

JSON input source that reads from a UTF-8 string.

#### Implementations

- <span id="strread-new"></span>`fn new(s: &'a str) -> Self`

#### Trait Implementations

##### `impl Fused for StrRead<'a>`

##### `impl Read for StrRead<'a>`

##### `impl Sealed for StrRead<'a>`

### `IoRead<R>`

```rust
struct IoRead<R>
where
    R: io::Read {
    iter: crate::iter::LineColIterator<io::Bytes<R>>,
    ch: Option<u8>,
}
```

*Defined in [`serde_json-1.0.145/src/read.rs:149-158`](../../../.source_1765210505/serde_json-1.0.145/src/read.rs#L149-L158)*

JSON input source that reads from a std::io input stream.

#### Fields

- **`ch`**: `Option<u8>`

  Temporary storage of peeked byte.

#### Implementations

- <span id="ioread-new"></span>`fn new(reader: R) -> Self`

#### Trait Implementations

##### `impl<'de, R> Read for IoRead<R>`

##### `impl<R> Sealed for IoRead<R>`

### `Deserializer<R>`

```rust
struct Deserializer<R> {
    read: R,
    scratch: alloc::vec::Vec<u8>,
    remaining_depth: u8,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:31-39`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L31-L39)*

A structure that deserializes JSON into Rust values.

#### Implementations

- <span id="deserializer-new"></span>`fn new(read: R) -> Self`

#### Trait Implementations

##### `impl<'de, R: Read<'de>> Deserializer for &mut Deserializer<R>`

- <span id="mut-deserializer-type-error"></span>`type Error = Error`

- <span id="mut-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mut-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

### `SeqAccess<'a, R: 'a>`

```rust
struct SeqAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:1917-1920`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L1917-L1920)*

#### Implementations

- <span id="seqaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> SeqAccess for SeqAccess<'a, R>`

- <span id="seqaccess-type-error"></span>`type Error = Error`

- <span id="seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>>` — [`Result`](../error/index.md#result)

### `MapAccess<'a, R: 'a>`

```rust
struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    first: bool,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:1970-1973`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L1970-L1973)*

#### Implementations

- <span id="mapaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> MapAccess for MapAccess<'a, R>`

- <span id="mapaccess-type-error"></span>`type Error = Error`

- <span id="mapaccess-next-key-seed"></span>`fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as >::Value>>` — [`Result`](../error/index.md#result)

- <span id="mapaccess-next-value-seed"></span>`fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

### `VariantAccess<'a, R: 'a>`

```rust
struct VariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:2035-2037`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2035-L2037)*

#### Implementations

- <span id="variantaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> EnumAccess for VariantAccess<'a, R>`

- <span id="variantaccess-type-error"></span>`type Error = Error`

- <span id="variantaccess-type-variant"></span>`type Variant = VariantAccess<'a, R>`

- <span id="variantaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../error/index.md#result)

##### `impl<'de, 'a, R: Read<'de> + 'a> VariantAccess for VariantAccess<'a, R>`

- <span id="variantaccess-type-error"></span>`type Error = Error`

- <span id="variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value>` — [`Result`](../error/index.md#result)

- <span id="variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="variantaccess-struct-variant"></span>`fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

### `UnitVariantAccess<'a, R: 'a>`

```rust
struct UnitVariantAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:2088-2090`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2088-L2090)*

#### Implementations

- <span id="unitvariantaccess-new"></span>`fn new(de: &'a mut Deserializer<R>) -> Self` — [`Deserializer`](#deserializer)

#### Trait Implementations

##### `impl<'de, 'a, R: Read<'de> + 'a> EnumAccess for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-type-error"></span>`type Error = Error`

- <span id="unitvariantaccess-type-variant"></span>`type Variant = UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, Self)>` — [`Result`](../error/index.md#result)

##### `impl<'de, 'a, R: Read<'de> + 'a> VariantAccess for UnitVariantAccess<'a, R>`

- <span id="unitvariantaccess-type-error"></span>`type Error = Error`

- <span id="unitvariantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<()>` — [`Result`](../error/index.md#result)

- <span id="unitvariantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value>` — [`Result`](../error/index.md#result)

- <span id="unitvariantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="unitvariantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

### `MapKey<'a, R: 'a>`

```rust
struct MapKey<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:2151-2153`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2151-L2153)*

Only deserialize from this after peeking a '"' byte! Otherwise it may
deserialize invalid JSON successfully.

#### Implementations

- <span id="mapkey-deserialize-number"></span>`fn deserialize_number<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl<'de, 'a, R> Deserializer for MapKey<'a, R>`

- <span id="mapkey-type-error"></span>`type Error = Error`

- <span id="mapkey-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="mapkey-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkey-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

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

*Defined in [`serde_json-1.0.145/src/de.rs:2349-2355`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2349-L2355)*

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

- <span id="streamdeserializer-byte-offset"></span>`fn byte_offset(&self) -> usize`

- <span id="streamdeserializer-peek-end-of-value"></span>`fn peek_end_of_value(&mut self) -> Result<()>` — [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl<'de, R, T> FusedIterator for StreamDeserializer<'de, R, T>`

##### `impl<I> IntoIterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamdeserializer-type-intoiter"></span>`type IntoIter = I`

- <span id="streamdeserializer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'de, R, T> Iterator for StreamDeserializer<'de, R, T>`

- <span id="streamdeserializer-type-item"></span>`type Item = Result<T, Error>`

- <span id="streamdeserializer-next"></span>`fn next(&mut self) -> Option<Result<T>>` — [`Result`](../error/index.md#result)

## Enums

### `ParserNumber`

```rust
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
}
```

*Defined in [`serde_json-1.0.145/src/de.rs:109-115`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L109-L115)*

#### Implementations

- <span id="parsernumber-visit"></span>`fn visit<'de, V>(self, visitor: V) -> Result<<V as >::Value>` — [`Result`](../error/index.md#result)

- <span id="parsernumber-invalid-type"></span>`fn invalid_type(self, exp: &dyn Expected) -> Error` — [`Error`](../error/index.md#error)

## Traits

### `Read<'de>`

```rust
trait Read<'de>: private::Sealed { ... }
```

*Defined in [`serde_json-1.0.145/src/read.rs:28-117`](../../../.source_1765210505/serde_json-1.0.145/src/read.rs#L28-L117)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:2495-2506`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2495-L2506)*

### `from_reader`

```rust
fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned
```

*Defined in [`serde_json-1.0.145/src/de.rs:2612-2618`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2612-L2618)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:2655-2660`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2655-L2660)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:2697-2702`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2697-L2702)*

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

*Defined in [`serde_json-1.0.145/src/de.rs:101-107`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L101-L107)*

### `deserialize_number!`

*Defined in [`serde_json-1.0.145/src/de.rs:1339-1352`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L1339-L1352)*

### `if_checking_recursion_limit!`

*Defined in [`serde_json-1.0.145/src/de.rs:1355-1359`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L1355-L1359)*

### `check_recursion!`

*Defined in [`serde_json-1.0.145/src/de.rs:1370-1385`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L1370-L1385)*

### `deserialize_numeric_key!`

*Defined in [`serde_json-1.0.145/src/de.rs:2155-2187`](../../../.source_1765210505/serde_json-1.0.145/src/de.rs#L2155-L2187)*

