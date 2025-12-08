*[serde_core](../../index.md) / [de](../index.md) / [impls](index.md)*

---

# Module `impls`

## Contents

- [Modules](#modules)
  - [`range`](#range)
  - [`range_from`](#range_from)
  - [`range_to`](#range_to)
- [Structs](#structs)
  - [`UnitVisitor`](#unitvisitor)
  - [`BoolVisitor`](#boolvisitor)
  - [`CharVisitor`](#charvisitor)
  - [`StringVisitor`](#stringvisitor)
  - [`StringInPlaceVisitor`](#stringinplacevisitor)
  - [`StrVisitor`](#strvisitor)
  - [`BytesVisitor`](#bytesvisitor)
  - [`CStringVisitor`](#cstringvisitor)
  - [`OptionVisitor`](#optionvisitor)
  - [`PhantomDataVisitor`](#phantomdatavisitor)
  - [`ArrayVisitor`](#arrayvisitor)
  - [`ArrayInPlaceVisitor`](#arrayinplacevisitor)
  - [`PathVisitor`](#pathvisitor)
  - [`PathBufVisitor`](#pathbufvisitor)
  - [`OsStringVisitor`](#osstringvisitor)
  - [`FromStrVisitor`](#fromstrvisitor)
- [Enums](#enums)
  - [`OsStringKind`](#osstringkind)
- [Functions](#functions)
  - [`nop_reserve`](#nop_reserve)
- [Macros](#macros)
  - [`impl_deserialize_num!`](#impl_deserialize_num)
  - [`num_self!`](#num_self)
  - [`num_as_self!`](#num_as_self)
  - [`num_as_copysign_self!`](#num_as_copysign_self)
  - [`int_to_int!`](#int_to_int)
  - [`int_to_uint!`](#int_to_uint)
  - [`uint_to_self!`](#uint_to_self)
  - [`num_128!`](#num_128)
  - [`forwarded_impl!`](#forwarded_impl)
  - [`seq_impl!`](#seq_impl)
  - [`array_impls!`](#array_impls)
  - [`tuple_impls!`](#tuple_impls)
  - [`tuple_impl_body!`](#tuple_impl_body)
  - [`map_impl!`](#map_impl)
  - [`parse_ip_impl!`](#parse_ip_impl)
  - [`variant_identifier!`](#variant_identifier)
  - [`deserialize_enum!`](#deserialize_enum)
  - [`parse_socket_impl!`](#parse_socket_impl)
  - [`box_forwarded_impl!`](#box_forwarded_impl)
  - [`atomic_impl!`](#atomic_impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`range`](#range) | mod |  |
| [`range_from`](#range_from) | mod |  |
| [`range_to`](#range_to) | mod |  |
| [`UnitVisitor`](#unitvisitor) | struct |  |
| [`BoolVisitor`](#boolvisitor) | struct |  |
| [`CharVisitor`](#charvisitor) | struct |  |
| [`StringVisitor`](#stringvisitor) | struct |  |
| [`StringInPlaceVisitor`](#stringinplacevisitor) | struct |  |
| [`StrVisitor`](#strvisitor) | struct |  |
| [`BytesVisitor`](#bytesvisitor) | struct |  |
| [`CStringVisitor`](#cstringvisitor) | struct |  |
| [`OptionVisitor`](#optionvisitor) | struct |  |
| [`PhantomDataVisitor`](#phantomdatavisitor) | struct |  |
| [`ArrayVisitor`](#arrayvisitor) | struct |  |
| [`ArrayInPlaceVisitor`](#arrayinplacevisitor) | struct |  |
| [`PathVisitor`](#pathvisitor) | struct |  |
| [`PathBufVisitor`](#pathbufvisitor) | struct |  |
| [`OsStringVisitor`](#osstringvisitor) | struct |  |
| [`FromStrVisitor`](#fromstrvisitor) | struct |  |
| [`OsStringKind`](#osstringkind) | enum |  |
| [`nop_reserve`](#nop_reserve) | fn |  |
| [`impl_deserialize_num!`](#impl_deserialize_num) | macro |  |
| [`num_self!`](#num_self) | macro |  |
| [`num_as_self!`](#num_as_self) | macro |  |
| [`num_as_copysign_self!`](#num_as_copysign_self) | macro |  |
| [`int_to_int!`](#int_to_int) | macro |  |
| [`int_to_uint!`](#int_to_uint) | macro |  |
| [`uint_to_self!`](#uint_to_self) | macro |  |
| [`num_128!`](#num_128) | macro |  |
| [`forwarded_impl!`](#forwarded_impl) | macro |  |
| [`seq_impl!`](#seq_impl) | macro |  |
| [`array_impls!`](#array_impls) | macro |  |
| [`tuple_impls!`](#tuple_impls) | macro |  |
| [`tuple_impl_body!`](#tuple_impl_body) | macro |  |
| [`map_impl!`](#map_impl) | macro |  |
| [`parse_ip_impl!`](#parse_ip_impl) | macro |  |
| [`variant_identifier!`](#variant_identifier) | macro |  |
| [`deserialize_enum!`](#deserialize_enum) | macro |  |
| [`parse_socket_impl!`](#parse_socket_impl) | macro |  |
| [`box_forwarded_impl!`](#box_forwarded_impl) | macro |  |
| [`atomic_impl!`](#atomic_impl) | macro |  |

## Modules

- [`range`](range/index.md) - 
- [`range_from`](range_from/index.md) - 
- [`range_to`](range_to/index.md) - 

## Structs

### `UnitVisitor`

```rust
struct UnitVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for UnitVisitor`

- <span id="unitvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for UnitVisitor`

- <span id="unitvisitor-value"></span>`type Value = ()`

- <span id="unitvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="unitvisitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `BoolVisitor`

```rust
struct BoolVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for BoolVisitor`

- <span id="boolvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for BoolVisitor`

- <span id="boolvisitor-value"></span>`type Value = bool`

- <span id="boolvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="boolvisitor-visit-bool"></span>`fn visit_bool<E>(self, v: bool) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `CharVisitor`

```rust
struct CharVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for CharVisitor`

- <span id="charvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for CharVisitor`

- <span id="charvisitor-value"></span>`type Value = char`

- <span id="charvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="charvisitor-visit-char"></span>`fn visit_char<E>(self, v: char) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="charvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `StringVisitor`

```rust
struct StringVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for StringVisitor`

- <span id="stringvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for StringVisitor`

- <span id="stringvisitor-value"></span>`type Value = String`

- <span id="stringvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="stringvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md), [`Visitor`](../index.md)

- <span id="stringvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="stringvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md), [`Visitor`](../index.md)

### `StringInPlaceVisitor<'a>`

```rust
struct StringInPlaceVisitor<'a>(&'a mut String);
```

#### Trait Implementations

##### `impl<'de, T> Expected for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, 'de> Visitor for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-value"></span>`type Value = ()`

- <span id="stringinplacevisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringinplacevisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="stringinplacevisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md), [`Visitor`](../index.md)

- <span id="stringinplacevisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="stringinplacevisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md), [`Visitor`](../index.md)

### `StrVisitor`

```rust
struct StrVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for StrVisitor`

- <span id="strvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Visitor for StrVisitor`

- <span id="strvisitor-value"></span>`type Value = &'a str`

- <span id="strvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="strvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="strvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `BytesVisitor`

```rust
struct BytesVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for BytesVisitor`

- <span id="bytesvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Visitor for BytesVisitor`

- <span id="bytesvisitor-value"></span>`type Value = &'a [u8]`

- <span id="bytesvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="bytesvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="bytesvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `CStringVisitor`

```rust
struct CStringVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for CStringVisitor`

- <span id="cstringvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for CStringVisitor`

- <span id="cstringvisitor-value"></span>`type Value = CString`

- <span id="cstringvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="cstringvisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

- <span id="cstringvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="cstringvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md), [`Visitor`](../index.md)

- <span id="cstringvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="cstringvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md), [`Visitor`](../index.md)

### `OptionVisitor<T>`

```rust
struct OptionVisitor<T> {
    marker: PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for OptionVisitor<T>`

- <span id="optionvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for OptionVisitor<T>`

- <span id="optionvisitor-value"></span>`type Value = Option<T>`

- <span id="optionvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionvisitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="optionvisitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="optionvisitor-visit-some"></span>`fn visit_some<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md)

### `PhantomDataVisitor<T: ?Sized>`

```rust
struct PhantomDataVisitor<T: ?Sized> {
    marker: PhantomData<T>,
}
```

#### Trait Implementations

##### `impl<'de, T> Expected for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-value"></span>`type Value = PhantomData<T>`

- <span id="phantomdatavisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="phantomdatavisitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `ArrayVisitor<A>`

```rust
struct ArrayVisitor<A> {
    marker: PhantomData<A>,
}
```

#### Implementations

- <span id="arrayvisitor-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<'de, T> Expected for ArrayVisitor<A>`

- <span id="arrayvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for ArrayVisitor<[T; 1]>`

- <span id="arrayvisitor-value"></span>`type Value = [T; 1]`

- <span id="arrayvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayvisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

### `ArrayInPlaceVisitor<'a, A: 'a>`

```rust
struct ArrayInPlaceVisitor<'a, A: 'a>(&'a mut A);
```

#### Trait Implementations

##### `impl<'de, T> Expected for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, 'de, T> Visitor for ArrayInPlaceVisitor<'a, [T; 2]>`

- <span id="arrayinplacevisitor-value"></span>`type Value = ()`

- <span id="arrayinplacevisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayinplacevisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

### `PathVisitor`

```rust
struct PathVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for PathVisitor`

- <span id="pathvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Visitor for PathVisitor`

- <span id="pathvisitor-value"></span>`type Value = &'a Path`

- <span id="pathvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="pathvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

### `PathBufVisitor`

```rust
struct PathBufVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for PathBufVisitor`

- <span id="pathbufvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for PathBufVisitor`

- <span id="pathbufvisitor-value"></span>`type Value = PathBuf`

- <span id="pathbufvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathbufvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="pathbufvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md), [`Visitor`](../index.md)

- <span id="pathbufvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

- <span id="pathbufvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md), [`Visitor`](../index.md)

### `OsStringVisitor`

```rust
struct OsStringVisitor;
```

#### Trait Implementations

##### `impl<'de, T> Expected for OsStringVisitor`

- <span id="osstringvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de> Visitor for OsStringVisitor`

- <span id="osstringvisitor-value"></span>`type Value = OsString`

- <span id="osstringvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="osstringvisitor-visit-enum"></span>`fn visit_enum<A>(self, data: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md)

### `FromStrVisitor<T>`

```rust
struct FromStrVisitor<T> {
    expecting: &'static str,
    ty: PhantomData<T>,
}
```

#### Implementations

- <span id="fromstrvisitor-new"></span>`fn new(expecting: &'static str) -> Self`

#### Trait Implementations

##### `impl<'de, T> Expected for FromStrVisitor<T>`

- <span id="fromstrvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'de, T> Visitor for FromStrVisitor<T>`

- <span id="fromstrvisitor-value"></span>`type Value = T`

- <span id="fromstrvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="fromstrvisitor-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md)

## Enums

### `OsStringKind`

```rust
enum OsStringKind {
    Unix,
    Windows,
}
```

#### Trait Implementations

##### `impl<'de> Deserialize for OsStringKind`

- <span id="osstringkind-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../../index.md)

##### `impl<T> DeserializeOwned for OsStringKind`

## Functions

### `nop_reserve`

```rust
fn nop_reserve<T>(_seq: T, _n: usize)
```

## Macros

### `impl_deserialize_num!`

### `num_self!`

### `num_as_self!`

### `num_as_copysign_self!`

### `int_to_int!`

### `int_to_uint!`

### `uint_to_self!`

### `num_128!`

### `forwarded_impl!`

### `seq_impl!`

### `array_impls!`

### `tuple_impls!`

### `tuple_impl_body!`

### `map_impl!`

### `parse_ip_impl!`

### `variant_identifier!`

### `deserialize_enum!`

### `parse_socket_impl!`

### `box_forwarded_impl!`

### `atomic_impl!`

