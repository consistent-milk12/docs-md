*[serde_json](../index.md) / [number](index.md)*

---

# Module `number`

## Structs

### `Number`

```rust
struct Number {
    n: N,
}
```

Represents a JSON number, whether integer or floating point.

#### Implementations

- `fn is_i64(self: &Self) -> bool`

- `fn is_u64(self: &Self) -> bool`

- `fn is_f64(self: &Self) -> bool`

- `fn as_i64(self: &Self) -> Option<i64>`

- `fn as_u64(self: &Self) -> Option<u64>`

- `fn as_f64(self: &Self) -> Option<f64>`

- `fn from_f64(f: f64) -> Option<Number>` — [`Number`](#number)

- `fn as_i128(self: &Self) -> Option<i128>`

- `fn as_u128(self: &Self) -> Option<u128>`

- `fn from_i128(i: i128) -> Option<Number>` — [`Number`](#number)

- `fn from_u128(i: u128) -> Option<Number>` — [`Number`](#number)

- `fn as_f32(self: &Self) -> Option<f32>`

- `fn from_f32(f: f32) -> Option<Number>` — [`Number`](#number)

#### Trait Implementations

##### `impl Clone for Number`

- `fn clone(self: &Self) -> Number` — [`Number`](#number)

##### `impl Debug for Number`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Deserialize for Number`

- `fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](#number)

##### `impl<T> DeserializeOwned for Number`

##### `impl<'de> Deserializer for Number`

- `type Error = Error`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md)

- `fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer>::Error>`

##### `impl Display for Number`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Number`

##### `impl FromStr for crate::number::Number`

- `type Err = Error`

- `fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Number`

- `fn eq(self: &Self, other: &Number) -> bool` — [`Number`](#number)

##### `impl Serialize for Number`

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl<T> ToString for Number`

- `fn to_string(self: &Self) -> String`

## Enums

### `N`

```rust
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}
```

#### Variants

- **`NegInt`**

  Always less than zero.

- **`Float`**

  Always finite.

#### Trait Implementations

##### `impl Clone for N`

- `fn clone(self: &Self) -> N` — [`N`](#n)

##### `impl Copy for N`

##### `impl Eq for N`

##### `impl Hash for N`

- `fn hash<H: Hasher>(self: &Self, h: &mut H)`

##### `impl PartialEq for N`

- `fn eq(self: &Self, other: &Self) -> bool`

## Macros

### `deserialize_any!`

### `deserialize_number!`

### `impl_from_unsigned!`

### `impl_from_signed!`

