*[serde_json](../index.md) / [number](index.md)*

---

# Module `number`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Number`](#number) | struct | Represents a JSON number, whether integer or floating point. |
| [`N`](#n) | enum |  |
| [`deserialize_any!`](#deserialize-any) | macro |  |
| [`deserialize_number!`](#deserialize-number) | macro |  |
| [`impl_from_unsigned!`](#impl-from-unsigned) | macro |  |
| [`impl_from_signed!`](#impl-from-signed) | macro |  |

## Structs

### `Number`

```rust
struct Number {
    n: N,
}
```

*Defined in [`serde_json-1.0.145/src/number.rs:22-24`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L22-L24)*

Represents a JSON number, whether integer or floating point.

#### Implementations

- <span id="number-is-i64"></span>`fn is_i64(&self) -> bool`

- <span id="number-is-u64"></span>`fn is_u64(&self) -> bool`

- <span id="number-is-f64"></span>`fn is_f64(&self) -> bool`

- <span id="number-as-i64"></span>`fn as_i64(&self) -> Option<i64>`

- <span id="number-as-u64"></span>`fn as_u64(&self) -> Option<u64>`

- <span id="number-as-f64"></span>`fn as_f64(&self) -> Option<f64>`

- <span id="number-from-f64"></span>`fn from_f64(f: f64) -> Option<Number>` — [`Number`](#number)

- <span id="number-as-i128"></span>`fn as_i128(&self) -> Option<i128>`

- <span id="number-as-u128"></span>`fn as_u128(&self) -> Option<u128>`

- <span id="number-from-i128"></span>`fn from_i128(i: i128) -> Option<Number>` — [`Number`](#number)

- <span id="number-from-u128"></span>`fn from_u128(i: u128) -> Option<Number>` — [`Number`](#number)

- <span id="number-as-f32"></span>`fn as_f32(&self) -> Option<f32>`

- <span id="number-from-f32"></span>`fn from_f32(f: f32) -> Option<Number>` — [`Number`](#number)

#### Trait Implementations

##### `impl Clone for Number`

- <span id="number-clone"></span>`fn clone(&self) -> Number` — [`Number`](#number)

##### `impl Debug for Number`

- <span id="number-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Number`

- <span id="number-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Number, <D as >::Error>` — [`Number`](#number)

##### `impl DeserializeOwned for Number`

##### `impl Deserializer for Number`

- <span id="number-deserializer-type-error"></span>`type Error = Error`

- <span id="number-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../error/index.md#error)

- <span id="number-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="number-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl Display for Number`

- <span id="number-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Number`

##### `impl FromStr for crate::number::Number`

- <span id="cratenumbernumber-fromstr-type-err"></span>`type Err = Error`

- <span id="cratenumbernumber-from-str"></span>`fn from_str(s: &str) -> result::Result<Self, <Self as >::Err>`

##### `impl Hash for Number`

- <span id="number-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Number`

- <span id="number-eq"></span>`fn eq(&self, other: &Number) -> bool` — [`Number`](#number)

##### `impl Serialize for Number`

- <span id="number-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Number`

##### `impl ToString for Number`

- <span id="number-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `N`

```rust
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}
```

*Defined in [`serde_json-1.0.145/src/number.rs:28-34`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L28-L34)*

#### Variants

- **`NegInt`**

  Always less than zero.

- **`Float`**

  Always finite.

#### Trait Implementations

##### `impl Clone for N`

- <span id="n-clone"></span>`fn clone(&self) -> N` — [`N`](#n)

##### `impl Copy for N`

##### `impl Eq for N`

##### `impl Hash for N`

- <span id="n-hash"></span>`fn hash<H: Hasher>(&self, h: &mut H)`

##### `impl PartialEq for N`

- <span id="n-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Macros

### `deserialize_any!`

*Defined in [`serde_json-1.0.145/src/number.rs:532-577`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L532-L577)*

### `deserialize_number!`

*Defined in [`serde_json-1.0.145/src/number.rs:579-597`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L579-L597)*

### `impl_from_unsigned!`

*Defined in [`serde_json-1.0.145/src/number.rs:737-757`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L737-L757)*

### `impl_from_signed!`

*Defined in [`serde_json-1.0.145/src/number.rs:759-785`](../../../.source_1765521767/serde_json-1.0.145/src/number.rs#L759-L785)*

