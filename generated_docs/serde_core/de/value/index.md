*[serde_core](../../index.md) / [de](../index.md) / [value](index.md)*

---

# Module `value`

Building blocks for deserializing basic values using the `IntoDeserializer`
trait.

```edition2021
use serde::de::{value, Deserialize, IntoDeserializer};
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
enum Setting {
    On,
    Off,
}

impl FromStr for Setting {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
```

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Error`](#error)
  - [`UnitDeserializer`](#unitdeserializer)
  - [`BoolDeserializer`](#booldeserializer)
  - [`I8Deserializer`](#i8deserializer)
  - [`I16Deserializer`](#i16deserializer)
  - [`I32Deserializer`](#i32deserializer)
  - [`I64Deserializer`](#i64deserializer)
  - [`I128Deserializer`](#i128deserializer)
  - [`IsizeDeserializer`](#isizedeserializer)
  - [`U8Deserializer`](#u8deserializer)
  - [`U16Deserializer`](#u16deserializer)
  - [`U64Deserializer`](#u64deserializer)
  - [`U128Deserializer`](#u128deserializer)
  - [`UsizeDeserializer`](#usizedeserializer)
  - [`F32Deserializer`](#f32deserializer)
  - [`F64Deserializer`](#f64deserializer)
  - [`CharDeserializer`](#chardeserializer)
  - [`U32Deserializer`](#u32deserializer)
  - [`StrDeserializer`](#strdeserializer)
  - [`BorrowedStrDeserializer`](#borrowedstrdeserializer)
  - [`StringDeserializer`](#stringdeserializer)
  - [`CowStrDeserializer`](#cowstrdeserializer)
  - [`BytesDeserializer`](#bytesdeserializer)
  - [`BorrowedBytesDeserializer`](#borrowedbytesdeserializer)
  - [`SeqDeserializer`](#seqdeserializer)
  - [`ExpectedInSeq`](#expectedinseq)
  - [`SeqAccessDeserializer`](#seqaccessdeserializer)
  - [`MapDeserializer`](#mapdeserializer)
  - [`PairDeserializer`](#pairdeserializer)
  - [`PairVisitor`](#pairvisitor)
  - [`ExpectedInMap`](#expectedinmap)
  - [`MapAccessDeserializer`](#mapaccessdeserializer)
  - [`EnumAccessDeserializer`](#enumaccessdeserializer)
- [Type Aliases](#type-aliases)
  - [`ErrorImpl`](#errorimpl)
- [Macros](#macros)
  - [`impl_copy_clone!`](#impl_copy_clone)
  - [`primitive_deserializer!`](#primitive_deserializer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Error`](#error) | struct | A minimal representation of all possible errors that can occur using the |
| [`UnitDeserializer`](#unitdeserializer) | struct | A deserializer holding a `()`. |
| [`BoolDeserializer`](#booldeserializer) | struct | A deserializer holding |
| [`I8Deserializer`](#i8deserializer) | struct | A deserializer holding |
| [`I16Deserializer`](#i16deserializer) | struct | A deserializer holding |
| [`I32Deserializer`](#i32deserializer) | struct | A deserializer holding |
| [`I64Deserializer`](#i64deserializer) | struct | A deserializer holding |
| [`I128Deserializer`](#i128deserializer) | struct | A deserializer holding |
| [`IsizeDeserializer`](#isizedeserializer) | struct | A deserializer holding |
| [`U8Deserializer`](#u8deserializer) | struct | A deserializer holding |
| [`U16Deserializer`](#u16deserializer) | struct | A deserializer holding |
| [`U64Deserializer`](#u64deserializer) | struct | A deserializer holding |
| [`U128Deserializer`](#u128deserializer) | struct | A deserializer holding |
| [`UsizeDeserializer`](#usizedeserializer) | struct | A deserializer holding |
| [`F32Deserializer`](#f32deserializer) | struct | A deserializer holding |
| [`F64Deserializer`](#f64deserializer) | struct | A deserializer holding |
| [`CharDeserializer`](#chardeserializer) | struct | A deserializer holding |
| [`U32Deserializer`](#u32deserializer) | struct | A deserializer holding a `u32`. |
| [`StrDeserializer`](#strdeserializer) | struct | A deserializer holding a `&str`. |
| [`BorrowedStrDeserializer`](#borrowedstrdeserializer) | struct | A deserializer holding a `&str` with a lifetime tied to another |
| [`StringDeserializer`](#stringdeserializer) | struct | A deserializer holding a `String`. |
| [`CowStrDeserializer`](#cowstrdeserializer) | struct | A deserializer holding a `Cow<str>`. |
| [`BytesDeserializer`](#bytesdeserializer) | struct | A deserializer holding a `&[u8]`. |
| [`BorrowedBytesDeserializer`](#borrowedbytesdeserializer) | struct | A deserializer holding a `&[u8]` with a lifetime tied to another |
| [`SeqDeserializer`](#seqdeserializer) | struct | A deserializer that iterates over a sequence. |
| [`ExpectedInSeq`](#expectedinseq) | struct |  |
| [`SeqAccessDeserializer`](#seqaccessdeserializer) | struct | A deserializer holding a `SeqAccess`. |
| [`MapDeserializer`](#mapdeserializer) | struct | A deserializer that iterates over a map. |
| [`PairDeserializer`](#pairdeserializer) | struct |  |
| [`PairVisitor`](#pairvisitor) | struct |  |
| [`ExpectedInMap`](#expectedinmap) | struct |  |
| [`MapAccessDeserializer`](#mapaccessdeserializer) | struct | A deserializer holding a `MapAccess`. |
| [`EnumAccessDeserializer`](#enumaccessdeserializer) | struct | A deserializer holding an `EnumAccess`. |
| [`ErrorImpl`](#errorimpl) | type |  |
| [`impl_copy_clone!`](#impl_copy_clone) | macro |  |
| [`primitive_deserializer!`](#primitive_deserializer) | macro |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Error`

```rust
struct Error {
    err: Box<str>,
}
```

A minimal representation of all possible errors that can occur using the
`IntoDeserializer` trait.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-custom"></span>`fn custom<T>(msg: T) -> Self`

##### `impl PartialEq for Error`

- <span id="error-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../lib/index.md)

### `UnitDeserializer<E>`

```rust
struct UnitDeserializer<E> {
    marker: PhantomData<E>,
}
```

A deserializer holding a `()`.

#### Implementations

- <span id="unitdeserializer-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<E> Clone for UnitDeserializer<E>`

- <span id="unitdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for UnitDeserializer<E>`

##### `impl<E> Debug for UnitDeserializer<E>`

- <span id="unitdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for UnitDeserializer<E>`

- <span id="unitdeserializer-error"></span>`type Error = E`

- <span id="unitdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="unitdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="unitdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for UnitDeserializer<E>`

- <span id="unitdeserializer-deserializer"></span>`type Deserializer = UnitDeserializer<E>`

- <span id="unitdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BoolDeserializer<E>`

```rust
struct BoolDeserializer<E> {
    value: bool,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `bool`.

#### Implementations

- <span id="booldeserializer-new"></span>`fn new(value: bool) -> Self`

#### Trait Implementations

##### `impl<E> Clone for BoolDeserializer<E>`

- <span id="booldeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for BoolDeserializer<E>`

##### `impl<E> Debug for BoolDeserializer<E>`

- <span id="booldeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for BoolDeserializer<E>`

- <span id="booldeserializer-error"></span>`type Error = E`

- <span id="booldeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="booldeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for BoolDeserializer<E>`

- <span id="booldeserializer-deserializer"></span>`type Deserializer = BoolDeserializer<E>`

- <span id="booldeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I8Deserializer<E>`

```rust
struct I8Deserializer<E> {
    value: i8,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i8`.

#### Implementations

- <span id="i8deserializer-new"></span>`fn new(value: i8) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I8Deserializer<E>`

- <span id="i8deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I8Deserializer<E>`

##### `impl<E> Debug for I8Deserializer<E>`

- <span id="i8deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I8Deserializer<E>`

- <span id="i8deserializer-error"></span>`type Error = E`

- <span id="i8deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i8deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I8Deserializer<E>`

- <span id="i8deserializer-deserializer"></span>`type Deserializer = I8Deserializer<E>`

- <span id="i8deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I16Deserializer<E>`

```rust
struct I16Deserializer<E> {
    value: i16,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i16`.

#### Implementations

- <span id="i16deserializer-new"></span>`fn new(value: i16) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I16Deserializer<E>`

- <span id="i16deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I16Deserializer<E>`

##### `impl<E> Debug for I16Deserializer<E>`

- <span id="i16deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I16Deserializer<E>`

- <span id="i16deserializer-error"></span>`type Error = E`

- <span id="i16deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i16deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I16Deserializer<E>`

- <span id="i16deserializer-deserializer"></span>`type Deserializer = I16Deserializer<E>`

- <span id="i16deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I32Deserializer<E>`

```rust
struct I32Deserializer<E> {
    value: i32,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i32`.

#### Implementations

- <span id="i32deserializer-new"></span>`fn new(value: i32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I32Deserializer<E>`

- <span id="i32deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I32Deserializer<E>`

##### `impl<E> Debug for I32Deserializer<E>`

- <span id="i32deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I32Deserializer<E>`

- <span id="i32deserializer-error"></span>`type Error = E`

- <span id="i32deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i32deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I32Deserializer<E>`

- <span id="i32deserializer-deserializer"></span>`type Deserializer = I32Deserializer<E>`

- <span id="i32deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I64Deserializer<E>`

```rust
struct I64Deserializer<E> {
    value: i64,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i64`.

#### Implementations

- <span id="i64deserializer-new"></span>`fn new(value: i64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I64Deserializer<E>`

- <span id="i64deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I64Deserializer<E>`

##### `impl<E> Debug for I64Deserializer<E>`

- <span id="i64deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I64Deserializer<E>`

- <span id="i64deserializer-error"></span>`type Error = E`

- <span id="i64deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i64deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I64Deserializer<E>`

- <span id="i64deserializer-deserializer"></span>`type Deserializer = I64Deserializer<E>`

- <span id="i64deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `I128Deserializer<E>`

```rust
struct I128Deserializer<E> {
    value: i128,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `i128`.

#### Implementations

- <span id="i128deserializer-new"></span>`fn new(value: i128) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I128Deserializer<E>`

- <span id="i128deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for I128Deserializer<E>`

##### `impl<E> Debug for I128Deserializer<E>`

- <span id="i128deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I128Deserializer<E>`

- <span id="i128deserializer-error"></span>`type Error = E`

- <span id="i128deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="i128deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I128Deserializer<E>`

- <span id="i128deserializer-deserializer"></span>`type Deserializer = I128Deserializer<E>`

- <span id="i128deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `IsizeDeserializer<E>`

```rust
struct IsizeDeserializer<E> {
    value: isize,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `isize`.

#### Implementations

- <span id="isizedeserializer-new"></span>`fn new(value: isize) -> Self`

#### Trait Implementations

##### `impl<E> Clone for IsizeDeserializer<E>`

- <span id="isizedeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for IsizeDeserializer<E>`

##### `impl<E> Debug for IsizeDeserializer<E>`

- <span id="isizedeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for IsizeDeserializer<E>`

- <span id="isizedeserializer-error"></span>`type Error = E`

- <span id="isizedeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="isizedeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for IsizeDeserializer<E>`

- <span id="isizedeserializer-deserializer"></span>`type Deserializer = IsizeDeserializer<E>`

- <span id="isizedeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U8Deserializer<E>`

```rust
struct U8Deserializer<E> {
    value: u8,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u8`.

#### Implementations

- <span id="u8deserializer-new"></span>`fn new(value: u8) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U8Deserializer<E>`

- <span id="u8deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U8Deserializer<E>`

##### `impl<E> Debug for U8Deserializer<E>`

- <span id="u8deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U8Deserializer<E>`

- <span id="u8deserializer-error"></span>`type Error = E`

- <span id="u8deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u8deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U8Deserializer<E>`

- <span id="u8deserializer-deserializer"></span>`type Deserializer = U8Deserializer<E>`

- <span id="u8deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U16Deserializer<E>`

```rust
struct U16Deserializer<E> {
    value: u16,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u16`.

#### Implementations

- <span id="u16deserializer-new"></span>`fn new(value: u16) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U16Deserializer<E>`

- <span id="u16deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U16Deserializer<E>`

##### `impl<E> Debug for U16Deserializer<E>`

- <span id="u16deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U16Deserializer<E>`

- <span id="u16deserializer-error"></span>`type Error = E`

- <span id="u16deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u16deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U16Deserializer<E>`

- <span id="u16deserializer-deserializer"></span>`type Deserializer = U16Deserializer<E>`

- <span id="u16deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U64Deserializer<E>`

```rust
struct U64Deserializer<E> {
    value: u64,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u64`.

#### Implementations

- <span id="u64deserializer-new"></span>`fn new(value: u64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U64Deserializer<E>`

- <span id="u64deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U64Deserializer<E>`

##### `impl<E> Debug for U64Deserializer<E>`

- <span id="u64deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U64Deserializer<E>`

- <span id="u64deserializer-error"></span>`type Error = E`

- <span id="u64deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u64deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U64Deserializer<E>`

- <span id="u64deserializer-deserializer"></span>`type Deserializer = U64Deserializer<E>`

- <span id="u64deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U128Deserializer<E>`

```rust
struct U128Deserializer<E> {
    value: u128,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `u128`.

#### Implementations

- <span id="u128deserializer-new"></span>`fn new(value: u128) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U128Deserializer<E>`

- <span id="u128deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U128Deserializer<E>`

##### `impl<E> Debug for U128Deserializer<E>`

- <span id="u128deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U128Deserializer<E>`

- <span id="u128deserializer-error"></span>`type Error = E`

- <span id="u128deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u128deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U128Deserializer<E>`

- <span id="u128deserializer-deserializer"></span>`type Deserializer = U128Deserializer<E>`

- <span id="u128deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `UsizeDeserializer<E>`

```rust
struct UsizeDeserializer<E> {
    value: usize,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `usize`.

#### Implementations

- <span id="usizedeserializer-new"></span>`fn new(value: usize) -> Self`

#### Trait Implementations

##### `impl<E> Clone for UsizeDeserializer<E>`

- <span id="usizedeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for UsizeDeserializer<E>`

##### `impl<E> Debug for UsizeDeserializer<E>`

- <span id="usizedeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for UsizeDeserializer<E>`

- <span id="usizedeserializer-error"></span>`type Error = E`

- <span id="usizedeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="usizedeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for UsizeDeserializer<E>`

- <span id="usizedeserializer-deserializer"></span>`type Deserializer = UsizeDeserializer<E>`

- <span id="usizedeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `F32Deserializer<E>`

```rust
struct F32Deserializer<E> {
    value: f32,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `f32`.

#### Implementations

- <span id="f32deserializer-new"></span>`fn new(value: f32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for F32Deserializer<E>`

- <span id="f32deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for F32Deserializer<E>`

##### `impl<E> Debug for F32Deserializer<E>`

- <span id="f32deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for F32Deserializer<E>`

- <span id="f32deserializer-error"></span>`type Error = E`

- <span id="f32deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f32deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for F32Deserializer<E>`

- <span id="f32deserializer-deserializer"></span>`type Deserializer = F32Deserializer<E>`

- <span id="f32deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `F64Deserializer<E>`

```rust
struct F64Deserializer<E> {
    value: f64,
    marker: PhantomData<E>,
}
```

A deserializer holding
an `f64`.

#### Implementations

- <span id="f64deserializer-new"></span>`fn new(value: f64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for F64Deserializer<E>`

- <span id="f64deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for F64Deserializer<E>`

##### `impl<E> Debug for F64Deserializer<E>`

- <span id="f64deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for F64Deserializer<E>`

- <span id="f64deserializer-error"></span>`type Error = E`

- <span id="f64deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="f64deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for F64Deserializer<E>`

- <span id="f64deserializer-deserializer"></span>`type Deserializer = F64Deserializer<E>`

- <span id="f64deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `CharDeserializer<E>`

```rust
struct CharDeserializer<E> {
    value: char,
    marker: PhantomData<E>,
}
```

A deserializer holding
a `char`.

#### Implementations

- <span id="chardeserializer-new"></span>`fn new(value: char) -> Self`

#### Trait Implementations

##### `impl<E> Clone for CharDeserializer<E>`

- <span id="chardeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for CharDeserializer<E>`

##### `impl<E> Debug for CharDeserializer<E>`

- <span id="chardeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for CharDeserializer<E>`

- <span id="chardeserializer-error"></span>`type Error = E`

- <span id="chardeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="chardeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for CharDeserializer<E>`

- <span id="chardeserializer-deserializer"></span>`type Deserializer = CharDeserializer<E>`

- <span id="chardeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `U32Deserializer<E>`

```rust
struct U32Deserializer<E> {
    value: u32,
    marker: PhantomData<E>,
}
```

A deserializer holding a `u32`.

#### Implementations

- <span id="u32deserializer-new"></span>`fn new(value: u32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U32Deserializer<E>`

- <span id="u32deserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Copy for U32Deserializer<E>`

##### `impl<E> Debug for U32Deserializer<E>`

- <span id="u32deserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U32Deserializer<E>`

- <span id="u32deserializer-error"></span>`type Error = E`

- <span id="u32deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="u32deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="u32deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> EnumAccess for U32Deserializer<E>`

- <span id="u32deserializer-error"></span>`type Error = E`

- <span id="u32deserializer-variant"></span>`type Variant = UnitOnly<E>`

- <span id="u32deserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, E> IntoDeserializer for U32Deserializer<E>`

- <span id="u32deserializer-deserializer"></span>`type Deserializer = U32Deserializer<E>`

- <span id="u32deserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `StrDeserializer<'a, E>`

```rust
struct StrDeserializer<'a, E> {
    value: &'a str,
    marker: PhantomData<E>,
}
```

A deserializer holding a `&str`.

#### Implementations

- <span id="strdeserializer-new"></span>`fn new(value: &'a str) -> Self`

#### Trait Implementations

##### `impl<'de, E> Clone for StrDeserializer<'de, E>`

- <span id="strdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'de, E> Copy for StrDeserializer<'de, E>`

##### `impl<'a, E> Debug for StrDeserializer<'a, E>`

- <span id="strdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, 'a, E> Deserializer for StrDeserializer<'a, E>`

- <span id="strdeserializer-error"></span>`type Error = E`

- <span id="strdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="strdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="strdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="strdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, 'a, E> EnumAccess for StrDeserializer<'a, E>`

- <span id="strdeserializer-error"></span>`type Error = E`

- <span id="strdeserializer-variant"></span>`type Variant = UnitOnly<E>`

- <span id="strdeserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, 'a, E> IntoDeserializer for StrDeserializer<'a, E>`

- <span id="strdeserializer-deserializer"></span>`type Deserializer = StrDeserializer<'a, E>`

- <span id="strdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BorrowedStrDeserializer<'de, E>`

```rust
struct BorrowedStrDeserializer<'de, E> {
    value: &'de str,
    marker: PhantomData<E>,
}
```

A deserializer holding a `&str` with a lifetime tied to another
deserializer.

#### Implementations

- <span id="borrowedstrdeserializer-new"></span>`fn new(value: &'de str) -> BorrowedStrDeserializer<'de, E>` — [`BorrowedStrDeserializer`](#borrowedstrdeserializer)

#### Trait Implementations

##### `impl<'de, E> Clone for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'de, E> Copy for BorrowedStrDeserializer<'de, E>`

##### `impl<'de, E> Debug for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-error"></span>`type Error = E`

- <span id="borrowedstrdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="borrowedstrdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="borrowedstrdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedstrdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, E> EnumAccess for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-error"></span>`type Error = E`

- <span id="borrowedstrdeserializer-variant"></span>`type Variant = UnitOnly<E>`

- <span id="borrowedstrdeserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, E> IntoDeserializer for BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-deserializer"></span>`type Deserializer = BorrowedStrDeserializer<'de, E>`

- <span id="borrowedstrdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `StringDeserializer<E>`

```rust
struct StringDeserializer<E> {
    value: String,
    marker: PhantomData<E>,
}
```

A deserializer holding a `String`.

#### Implementations

- <span id="stringdeserializer-new"></span>`fn new(value: String) -> Self` — [`String`](../../lib/index.md)

#### Trait Implementations

##### `impl<E> Clone for StringDeserializer<E>`

- <span id="stringdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<E> Debug for StringDeserializer<E>`

- <span id="stringdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for StringDeserializer<E>`

- <span id="stringdeserializer-error"></span>`type Error = E`

- <span id="stringdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="stringdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="stringdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="stringdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, E> EnumAccess for StringDeserializer<E>`

- <span id="stringdeserializer-error"></span>`type Error = E`

- <span id="stringdeserializer-variant"></span>`type Variant = UnitOnly<E>`

- <span id="stringdeserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, E> IntoDeserializer for StringDeserializer<E>`

- <span id="stringdeserializer-deserializer"></span>`type Deserializer = StringDeserializer<E>`

- <span id="stringdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `CowStrDeserializer<'a, E>`

```rust
struct CowStrDeserializer<'a, E> {
    value: Cow<'a, str>,
    marker: PhantomData<E>,
}
```

A deserializer holding a `Cow<str>`.

#### Implementations

- <span id="cowstrdeserializer-new"></span>`fn new(value: Cow<'a, str>) -> Self` — [`Cow`](../../lib/index.md)

#### Trait Implementations

##### `impl<'a, E> Clone for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, E> Debug for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, 'a, E> Deserializer for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-error"></span>`type Error = E`

- <span id="cowstrdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="cowstrdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="cowstrdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="cowstrdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, 'a, E> EnumAccess for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-error"></span>`type Error = E`

- <span id="cowstrdeserializer-variant"></span>`type Variant = UnitOnly<E>`

- <span id="cowstrdeserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, 'a, E> IntoDeserializer for CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-deserializer"></span>`type Deserializer = CowStrDeserializer<'a, E>`

- <span id="cowstrdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BytesDeserializer<'a, E>`

```rust
struct BytesDeserializer<'a, E> {
    value: &'a [u8],
    marker: PhantomData<E>,
}
```

A deserializer holding a `&[u8]`. Always calls `Visitor::visit_bytes`.

#### Implementations

- <span id="bytesdeserializer-new"></span>`fn new(value: &'a [u8]) -> Self`

#### Trait Implementations

##### `impl<'a, E> Clone for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, E> Copy for BytesDeserializer<'a, E>`

##### `impl<'a, E> Debug for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, 'a, E> Deserializer for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-error"></span>`type Error = E`

- <span id="bytesdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="bytesdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="bytesdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, 'a, E> IntoDeserializer for BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-deserializer"></span>`type Deserializer = BytesDeserializer<'a, E>`

- <span id="bytesdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `BorrowedBytesDeserializer<'de, E>`

```rust
struct BorrowedBytesDeserializer<'de, E> {
    value: &'de [u8],
    marker: PhantomData<E>,
}
```

A deserializer holding a `&[u8]` with a lifetime tied to another
deserializer. Always calls `Visitor::visit_borrowed_bytes`.

#### Implementations

- <span id="borrowedbytesdeserializer-new"></span>`fn new(value: &'de [u8]) -> Self`

#### Trait Implementations

##### `impl<'de, E> Clone for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'de, E> Copy for BorrowedBytesDeserializer<'de, E>`

##### `impl<'de, E> Debug for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-error"></span>`type Error = E`

- <span id="borrowedbytesdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="borrowedbytesdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="borrowedbytesdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, E> IntoDeserializer for BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-deserializer"></span>`type Deserializer = BorrowedBytesDeserializer<'de, E>`

- <span id="borrowedbytesdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `SeqDeserializer<I, E>`

```rust
struct SeqDeserializer<I, E> {
    iter: iter::Fuse<I>,
    count: usize,
    marker: PhantomData<E>,
}
```

A deserializer that iterates over a sequence.

#### Implementations

- <span id="seqdeserializer-new"></span>`fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, E: clone::Clone> Clone for SeqDeserializer<I, E>`

- <span id="seqdeserializer-clone"></span>`fn clone(&self) -> SeqDeserializer<I, E>` — [`SeqDeserializer`](#seqdeserializer)

##### `impl<I, E> Debug for SeqDeserializer<I, E>`

- <span id="seqdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, I, T, E> Deserializer for SeqDeserializer<I, E>`

- <span id="seqdeserializer-error"></span>`type Error = E`

- <span id="seqdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="seqdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, I, T, E> IntoDeserializer for SeqDeserializer<I, E>`

- <span id="seqdeserializer-deserializer"></span>`type Deserializer = SeqDeserializer<I, E>`

- <span id="seqdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

##### `impl<'de, I, T, E> SeqAccess for SeqDeserializer<I, E>`

- <span id="seqdeserializer-error"></span>`type Error = E`

- <span id="seqdeserializer-next-element-seed"></span>`fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<<V as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- <span id="seqdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `ExpectedInSeq`

```rust
struct ExpectedInSeq(usize);
```

#### Trait Implementations

##### `impl Expected for ExpectedInSeq`

- <span id="expectedinseq-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SeqAccessDeserializer<A>`

```rust
struct SeqAccessDeserializer<A> {
    seq: A,
}
```

A deserializer holding a `SeqAccess`.

#### Implementations

- <span id="seqaccessdeserializer-new"></span>`fn new(seq: A) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-clone"></span>`fn clone(&self) -> SeqAccessDeserializer<A>` — [`SeqAccessDeserializer`](#seqaccessdeserializer)

##### `impl<A: fmt::Debug> Debug for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, A> Deserializer for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-error"></span>`type Error = <A as SeqAccess>::Error`

- <span id="seqaccessdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="seqaccessdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="seqaccessdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, A> IntoDeserializer for SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-deserializer"></span>`type Deserializer = SeqAccessDeserializer<A>`

- <span id="seqaccessdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `MapDeserializer<'de, I, E>`

```rust
struct MapDeserializer<'de, I, E>
where
    I: Iterator,
    <I as >::Item: private::Pair {
    iter: iter::Fuse<I>,
    value: Option<<<I as >::Item as Pair>::Second>,
    count: usize,
    lifetime: PhantomData<&'de ()>,
    error: PhantomData<E>,
}
```

A deserializer that iterates over a map.

#### Implementations

- <span id="mapdeserializer-end"></span>`fn end(self) -> Result<(), E>`

#### Trait Implementations

##### `impl<'de, I, E> Clone for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'de, I, E> Debug for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, I, E> Deserializer for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-error"></span>`type Error = E`

- <span id="mapdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="mapdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="mapdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="mapdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, I, E> IntoDeserializer for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-deserializer"></span>`type Deserializer = MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

##### `impl<'de, I, E> MapAccess for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-error"></span>`type Error = E`

- <span id="mapdeserializer-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- <span id="mapdeserializer-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- <span id="mapdeserializer-next-entry-seed"></span>`fn next_entry_seed<TK, TV>(&mut self, kseed: TK, vseed: TV) -> Result<Option<(<TK as >::Value, <TV as >::Value)>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- <span id="mapdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

##### `impl<'de, I, E> SeqAccess for MapDeserializer<'de, I, E>`

- <span id="mapdeserializer-error"></span>`type Error = E`

- <span id="mapdeserializer-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- <span id="mapdeserializer-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `PairDeserializer<A, B, E>`

```rust
struct PairDeserializer<A, B, E>(A, B, PhantomData<E>);
```

#### Trait Implementations

##### `impl<'de, A, B, E> Deserializer for PairDeserializer<A, B, E>`

- <span id="pairdeserializer-error"></span>`type Error = E`

- <span id="pairdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="pairdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="pairdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="pairdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

### `PairVisitor<A, B, E>`

```rust
struct PairVisitor<A, B, E>(Option<A>, Option<B>, PhantomData<E>);
```

#### Trait Implementations

##### `impl<'de, A, B, E> SeqAccess for PairVisitor<A, B, E>`

- <span id="pairvisitor-error"></span>`type Error = E`

- <span id="pairvisitor-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- <span id="pairvisitor-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

### `ExpectedInMap`

```rust
struct ExpectedInMap(usize);
```

#### Trait Implementations

##### `impl Expected for ExpectedInMap`

- <span id="expectedinmap-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MapAccessDeserializer<A>`

```rust
struct MapAccessDeserializer<A> {
    map: A,
}
```

A deserializer holding a `MapAccess`.

#### Implementations

- <span id="mapaccessdeserializer-new"></span>`fn new(map: A) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-clone"></span>`fn clone(&self) -> MapAccessDeserializer<A>` — [`MapAccessDeserializer`](#mapaccessdeserializer)

##### `impl<A: fmt::Debug> Debug for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, A> Deserializer for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapaccessdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="mapaccessdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="mapaccessdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="mapaccessdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, A> EnumAccess for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-error"></span>`type Error = <A as MapAccess>::Error`

- <span id="mapaccessdeserializer-variant"></span>`type Variant = MapAsEnum<A>`

- <span id="mapaccessdeserializer-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, A> IntoDeserializer for MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-deserializer"></span>`type Deserializer = MapAccessDeserializer<A>`

- <span id="mapaccessdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

### `EnumAccessDeserializer<A>`

```rust
struct EnumAccessDeserializer<A> {
    access: A,
}
```

A deserializer holding an `EnumAccess`.

#### Implementations

- <span id="enumaccessdeserializer-new"></span>`fn new(access: A) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone> Clone for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-clone"></span>`fn clone(&self) -> EnumAccessDeserializer<A>` — [`EnumAccessDeserializer`](#enumaccessdeserializer)

##### `impl<A: fmt::Debug> Debug for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, A> Deserializer for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-error"></span>`type Error = <A as EnumAccess>::Error`

- <span id="enumaccessdeserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- <span id="enumaccessdeserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

- <span id="enumaccessdeserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../../index.md)

##### `impl<'de, A> IntoDeserializer for EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-deserializer"></span>`type Deserializer = EnumAccessDeserializer<A>`

- <span id="enumaccessdeserializer-into-deserializer"></span>`fn into_deserializer(self) -> Self`

## Type Aliases

### `ErrorImpl`

```rust
type ErrorImpl = Box<str>;
```

## Macros

### `impl_copy_clone!`

### `primitive_deserializer!`

