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

- `fn clone(self: &Self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- `fn description(self: &Self) -> &str`

##### `impl PartialEq for Error`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

### `UnitDeserializer<E>`

```rust
struct UnitDeserializer<E> {
    marker: PhantomData<E>,
}
```

A deserializer holding a `()`.

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl<E> Clone for UnitDeserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for UnitDeserializer<E>`

##### `impl<E> Debug for UnitDeserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for UnitDeserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for UnitDeserializer<E>`

- `type Deserializer = UnitDeserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: bool) -> Self`

#### Trait Implementations

##### `impl<E> Clone for BoolDeserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for BoolDeserializer<E>`

##### `impl<E> Debug for BoolDeserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for BoolDeserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for BoolDeserializer<E>`

- `type Deserializer = BoolDeserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: i8) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I8Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for I8Deserializer<E>`

##### `impl<E> Debug for I8Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I8Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I8Deserializer<E>`

- `type Deserializer = I8Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: i16) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I16Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for I16Deserializer<E>`

##### `impl<E> Debug for I16Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I16Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I16Deserializer<E>`

- `type Deserializer = I16Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: i32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I32Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for I32Deserializer<E>`

##### `impl<E> Debug for I32Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I32Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I32Deserializer<E>`

- `type Deserializer = I32Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: i64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I64Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for I64Deserializer<E>`

##### `impl<E> Debug for I64Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I64Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I64Deserializer<E>`

- `type Deserializer = I64Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: i128) -> Self`

#### Trait Implementations

##### `impl<E> Clone for I128Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for I128Deserializer<E>`

##### `impl<E> Debug for I128Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for I128Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for I128Deserializer<E>`

- `type Deserializer = I128Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: isize) -> Self`

#### Trait Implementations

##### `impl<E> Clone for IsizeDeserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for IsizeDeserializer<E>`

##### `impl<E> Debug for IsizeDeserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for IsizeDeserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for IsizeDeserializer<E>`

- `type Deserializer = IsizeDeserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: u8) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U8Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for U8Deserializer<E>`

##### `impl<E> Debug for U8Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U8Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U8Deserializer<E>`

- `type Deserializer = U8Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: u16) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U16Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for U16Deserializer<E>`

##### `impl<E> Debug for U16Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U16Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U16Deserializer<E>`

- `type Deserializer = U16Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: u64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U64Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for U64Deserializer<E>`

##### `impl<E> Debug for U64Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U64Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U64Deserializer<E>`

- `type Deserializer = U64Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: u128) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U128Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for U128Deserializer<E>`

##### `impl<E> Debug for U128Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U128Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for U128Deserializer<E>`

- `type Deserializer = U128Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: usize) -> Self`

#### Trait Implementations

##### `impl<E> Clone for UsizeDeserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for UsizeDeserializer<E>`

##### `impl<E> Debug for UsizeDeserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for UsizeDeserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for UsizeDeserializer<E>`

- `type Deserializer = UsizeDeserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: f32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for F32Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for F32Deserializer<E>`

##### `impl<E> Debug for F32Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for F32Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for F32Deserializer<E>`

- `type Deserializer = F32Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: f64) -> Self`

#### Trait Implementations

##### `impl<E> Clone for F64Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for F64Deserializer<E>`

##### `impl<E> Debug for F64Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for F64Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for F64Deserializer<E>`

- `type Deserializer = F64Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: char) -> Self`

#### Trait Implementations

##### `impl<E> Clone for CharDeserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for CharDeserializer<E>`

##### `impl<E> Debug for CharDeserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for CharDeserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> IntoDeserializer for CharDeserializer<E>`

- `type Deserializer = CharDeserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

### `U32Deserializer<E>`

```rust
struct U32Deserializer<E> {
    value: u32,
    marker: PhantomData<E>,
}
```

A deserializer holding a `u32`.

#### Implementations

- `fn new(value: u32) -> Self`

#### Trait Implementations

##### `impl<E> Clone for U32Deserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Copy for U32Deserializer<E>`

##### `impl<E> Debug for U32Deserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for U32Deserializer<E>`

- `type Error = E`

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

##### `impl<'de, E> EnumAccess for U32Deserializer<E>`

- `type Error = E`

- `type Variant = UnitOnly<E>`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, E> IntoDeserializer for U32Deserializer<E>`

- `type Deserializer = U32Deserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

### `StrDeserializer<'a, E>`

```rust
struct StrDeserializer<'a, E> {
    value: &'a str,
    marker: PhantomData<E>,
}
```

A deserializer holding a `&str`.

#### Implementations

- `fn new(value: &'a str) -> Self`

#### Trait Implementations

##### `impl<'de, E> Clone for StrDeserializer<'de, E>`

- `fn clone(self: &Self) -> Self`

##### `impl<'de, E> Copy for StrDeserializer<'de, E>`

##### `impl<'a, E> Debug for StrDeserializer<'a, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, 'a, E> Deserializer for StrDeserializer<'a, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, 'a, E> EnumAccess for StrDeserializer<'a, E>`

- `type Error = E`

- `type Variant = UnitOnly<E>`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, 'a, E> IntoDeserializer for StrDeserializer<'a, E>`

- `type Deserializer = StrDeserializer<'a, E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: &'de str) -> BorrowedStrDeserializer<'de, E>` — [`BorrowedStrDeserializer`](#borrowedstrdeserializer)

#### Trait Implementations

##### `impl<'de, E> Clone for BorrowedStrDeserializer<'de, E>`

- `fn clone(self: &Self) -> Self`

##### `impl<'de, E> Copy for BorrowedStrDeserializer<'de, E>`

##### `impl<'de, E> Debug for BorrowedStrDeserializer<'de, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for BorrowedStrDeserializer<'de, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, E> EnumAccess for BorrowedStrDeserializer<'de, E>`

- `type Error = E`

- `type Variant = UnitOnly<E>`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, E> IntoDeserializer for BorrowedStrDeserializer<'de, E>`

- `type Deserializer = BorrowedStrDeserializer<'de, E>`

- `fn into_deserializer(self: Self) -> Self`

### `StringDeserializer<E>`

```rust
struct StringDeserializer<E> {
    value: String,
    marker: PhantomData<E>,
}
```

A deserializer holding a `String`.

#### Implementations

- `fn new(value: String) -> Self`

#### Trait Implementations

##### `impl<E> Clone for StringDeserializer<E>`

- `fn clone(self: &Self) -> Self`

##### `impl<E> Debug for StringDeserializer<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for StringDeserializer<E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, E> EnumAccess for StringDeserializer<E>`

- `type Error = E`

- `type Variant = UnitOnly<E>`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, E> IntoDeserializer for StringDeserializer<E>`

- `type Deserializer = StringDeserializer<E>`

- `fn into_deserializer(self: Self) -> Self`

### `CowStrDeserializer<'a, E>`

```rust
struct CowStrDeserializer<'a, E> {
    value: Cow<'a, str>,
    marker: PhantomData<E>,
}
```

A deserializer holding a `Cow<str>`.

#### Implementations

- `fn new(value: Cow<'a, str>) -> Self`

#### Trait Implementations

##### `impl<'a, E> Clone for CowStrDeserializer<'a, E>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, E> Debug for CowStrDeserializer<'a, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, 'a, E> Deserializer for CowStrDeserializer<'a, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, 'a, E> EnumAccess for CowStrDeserializer<'a, E>`

- `type Error = E`

- `type Variant = UnitOnly<E>`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, 'a, E> IntoDeserializer for CowStrDeserializer<'a, E>`

- `type Deserializer = CowStrDeserializer<'a, E>`

- `fn into_deserializer(self: Self) -> Self`

### `BytesDeserializer<'a, E>`

```rust
struct BytesDeserializer<'a, E> {
    value: &'a [u8],
    marker: PhantomData<E>,
}
```

A deserializer holding a `&[u8]`. Always calls `Visitor::visit_bytes`.

#### Implementations

- `fn new(value: &'a [u8]) -> Self`

#### Trait Implementations

##### `impl<'a, E> Clone for BytesDeserializer<'a, E>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, E> Copy for BytesDeserializer<'a, E>`

##### `impl<'a, E> Debug for BytesDeserializer<'a, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, 'a, E> Deserializer for BytesDeserializer<'a, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, 'a, E> IntoDeserializer for BytesDeserializer<'a, E>`

- `type Deserializer = BytesDeserializer<'a, E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(value: &'de [u8]) -> Self`

#### Trait Implementations

##### `impl<'de, E> Clone for BorrowedBytesDeserializer<'de, E>`

- `fn clone(self: &Self) -> Self`

##### `impl<'de, E> Copy for BorrowedBytesDeserializer<'de, E>`

##### `impl<'de, E> Debug for BorrowedBytesDeserializer<'de, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, E> Deserializer for BorrowedBytesDeserializer<'de, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, E> IntoDeserializer for BorrowedBytesDeserializer<'de, E>`

- `type Deserializer = BorrowedBytesDeserializer<'de, E>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, E: $crate::clone::Clone> Clone for SeqDeserializer<I, E>`

- `fn clone(self: &Self) -> SeqDeserializer<I, E>` — [`SeqDeserializer`](#seqdeserializer)

##### `impl<I, E> Debug for SeqDeserializer<I, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, I, T, E> Deserializer for SeqDeserializer<I, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, I, T, E> IntoDeserializer for SeqDeserializer<I, E>`

- `type Deserializer = SeqDeserializer<I, E>`

- `fn into_deserializer(self: Self) -> Self`

##### `impl<'de, I, T, E> SeqAccess for SeqDeserializer<I, E>`

- `type Error = E`

- `fn next_element_seed<V>(self: &mut Self, seed: V) -> Result<Option<<V as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

### `SeqAccessDeserializer<A>`

```rust
struct SeqAccessDeserializer<A> {
    seq: A,
}
```

A deserializer holding a `SeqAccess`.

#### Implementations

- `fn new(seq: A) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone> Clone for SeqAccessDeserializer<A>`

- `fn clone(self: &Self) -> SeqAccessDeserializer<A>` — [`SeqAccessDeserializer`](#seqaccessdeserializer)

##### `impl<A: $crate::fmt::Debug> Debug for SeqAccessDeserializer<A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'de, A> Deserializer for SeqAccessDeserializer<A>`

- `type Error = <A as SeqAccess>::Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, A> IntoDeserializer for SeqAccessDeserializer<A>`

- `type Deserializer = SeqAccessDeserializer<A>`

- `fn into_deserializer(self: Self) -> Self`

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

- `fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<'de, I, E> Clone for MapDeserializer<'de, I, E>`

- `fn clone(self: &Self) -> Self`

##### `impl<'de, I, E> Debug for MapDeserializer<'de, I, E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, I, E> Deserializer for MapDeserializer<'de, I, E>`

- `type Error = E`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, I, E> IntoDeserializer for MapDeserializer<'de, I, E>`

- `type Deserializer = MapDeserializer<'de, I, E>`

- `fn into_deserializer(self: Self) -> Self`

##### `impl<'de, I, E> MapAccess for MapDeserializer<'de, I, E>`

- `type Error = E`

- `fn next_key_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- `fn next_value_seed<T>(self: &mut Self, seed: T) -> Result<<T as >::Value, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- `fn next_entry_seed<TK, TV>(self: &mut Self, kseed: TK, vseed: TV) -> Result<Option<(<TK as >::Value, <TV as >::Value)>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

##### `impl<'de, I, E> SeqAccess for MapDeserializer<'de, I, E>`

- `type Error = E`

- `fn next_element_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>` — [`DeserializeSeed`](../index.md)

- `fn size_hint(self: &Self) -> Option<usize>`

### `MapAccessDeserializer<A>`

```rust
struct MapAccessDeserializer<A> {
    map: A,
}
```

A deserializer holding a `MapAccess`.

#### Implementations

- `fn new(map: A) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone> Clone for MapAccessDeserializer<A>`

- `fn clone(self: &Self) -> MapAccessDeserializer<A>` — [`MapAccessDeserializer`](#mapaccessdeserializer)

##### `impl<A: $crate::fmt::Debug> Debug for MapAccessDeserializer<A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'de, A> Deserializer for MapAccessDeserializer<A>`

- `type Error = <A as MapAccess>::Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_enum<V>(self: Self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, A> EnumAccess for MapAccessDeserializer<A>`

- `type Error = <A as MapAccess>::Error`

- `type Variant = MapAsEnum<A>`

- `fn variant_seed<T>(self: Self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), <Self as >::Error>` — [`DeserializeSeed`](../index.md)

##### `impl<'de, A> IntoDeserializer for MapAccessDeserializer<A>`

- `type Deserializer = MapAccessDeserializer<A>`

- `fn into_deserializer(self: Self) -> Self`

### `EnumAccessDeserializer<A>`

```rust
struct EnumAccessDeserializer<A> {
    access: A,
}
```

A deserializer holding an `EnumAccess`.

#### Implementations

- `fn new(access: A) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone> Clone for EnumAccessDeserializer<A>`

- `fn clone(self: &Self) -> EnumAccessDeserializer<A>` — [`EnumAccessDeserializer`](#enumaccessdeserializer)

##### `impl<A: $crate::fmt::Debug> Debug for EnumAccessDeserializer<A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'de, A> Deserializer for EnumAccessDeserializer<A>`

- `type Error = <A as EnumAccess>::Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>` — [`Visitor`](../index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>` — [`Visitor`](../index.md), [`Deserializer`](../index.md)

##### `impl<'de, A> IntoDeserializer for EnumAccessDeserializer<A>`

- `type Deserializer = EnumAccessDeserializer<A>`

- `fn into_deserializer(self: Self) -> Self`

