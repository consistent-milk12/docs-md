*[serde_core](../../index.md) / [de](../index.md) / [impls](index.md)*

---

# Module `impls`

## Contents

- [Modules](#modules)
  - [`range`](#range)
  - [`range_from`](#range-from)
  - [`range_to`](#range-to)
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
  - [`nop_reserve`](#nop-reserve)
- [Macros](#macros)
  - [`impl_deserialize_num!`](#impl-deserialize-num)
  - [`num_self!`](#num-self)
  - [`num_as_self!`](#num-as-self)
  - [`num_as_copysign_self!`](#num-as-copysign-self)
  - [`int_to_int!`](#int-to-int)
  - [`int_to_uint!`](#int-to-uint)
  - [`uint_to_self!`](#uint-to-self)
  - [`num_128!`](#num-128)
  - [`forwarded_impl!`](#forwarded-impl)
  - [`seq_impl!`](#seq-impl)
  - [`array_impls!`](#array-impls)
  - [`tuple_impls!`](#tuple-impls)
  - [`tuple_impl_body!`](#tuple-impl-body)
  - [`map_impl!`](#map-impl)
  - [`parse_ip_impl!`](#parse-ip-impl)
  - [`variant_identifier!`](#variant-identifier)
  - [`deserialize_enum!`](#deserialize-enum)
  - [`parse_socket_impl!`](#parse-socket-impl)
  - [`box_forwarded_impl!`](#box-forwarded-impl)
  - [`atomic_impl!`](#atomic-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`range`](#range) | mod |  |
| [`range_from`](#range-from) | mod |  |
| [`range_to`](#range-to) | mod |  |
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
| [`nop_reserve`](#nop-reserve) | fn |  |
| [`impl_deserialize_num!`](#impl-deserialize-num) | macro |  |
| [`num_self!`](#num-self) | macro |  |
| [`num_as_self!`](#num-as-self) | macro |  |
| [`num_as_copysign_self!`](#num-as-copysign-self) | macro |  |
| [`int_to_int!`](#int-to-int) | macro |  |
| [`int_to_uint!`](#int-to-uint) | macro |  |
| [`uint_to_self!`](#uint-to-self) | macro |  |
| [`num_128!`](#num-128) | macro |  |
| [`forwarded_impl!`](#forwarded-impl) | macro |  |
| [`seq_impl!`](#seq-impl) | macro |  |
| [`array_impls!`](#array-impls) | macro |  |
| [`tuple_impls!`](#tuple-impls) | macro |  |
| [`tuple_impl_body!`](#tuple-impl-body) | macro |  |
| [`map_impl!`](#map-impl) | macro |  |
| [`parse_ip_impl!`](#parse-ip-impl) | macro |  |
| [`variant_identifier!`](#variant-identifier) | macro |  |
| [`deserialize_enum!`](#deserialize-enum) | macro |  |
| [`parse_socket_impl!`](#parse-socket-impl) | macro |  |
| [`box_forwarded_impl!`](#box-forwarded-impl) | macro |  |
| [`atomic_impl!`](#atomic-impl) | macro |  |

## Modules

- [`range`](range/index.md)
- [`range_from`](range_from/index.md)
- [`range_to`](range_to/index.md)

## Structs

### `UnitVisitor`

```rust
struct UnitVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:14`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L14)*

#### Trait Implementations

##### `impl Expected for UnitVisitor`

- <span id="unitvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for UnitVisitor`

- <span id="unitvisitor-visitor-type-value"></span>`type Value = ()`

- <span id="unitvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="unitvisitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `BoolVisitor`

```rust
struct BoolVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:53`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L53)*

#### Trait Implementations

##### `impl Expected for BoolVisitor`

- <span id="boolvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for BoolVisitor`

- <span id="boolvisitor-visitor-type-value"></span>`type Value = bool`

- <span id="boolvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="boolvisitor-visit-bool"></span>`fn visit_bool<E>(self, v: bool) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `CharVisitor`

```rust
struct CharVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:537`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L537)*

#### Trait Implementations

##### `impl Expected for CharVisitor`

- <span id="charvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for CharVisitor`

- <span id="charvisitor-visitor-type-value"></span>`type Value = char`

- <span id="charvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="charvisitor-visit-char"></span>`fn visit_char<E>(self, v: char) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="charvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `StringVisitor`

```rust
struct StringVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:580`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L580)*

#### Trait Implementations

##### `impl Expected for StringVisitor`

- <span id="stringvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for StringVisitor`

- <span id="stringvisitor-visitor-type-value"></span>`type Value = String`

- <span id="stringvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `StringInPlaceVisitor<'a>`

```rust
struct StringInPlaceVisitor<'a>(&'a mut String);
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:582`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L582)*

#### Trait Implementations

##### `impl Expected for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-visitor-type-value"></span>`type Value = ()`

- <span id="stringinplacevisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringinplacevisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `StrVisitor`

```rust
struct StrVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:706`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L706)*

#### Trait Implementations

##### `impl Expected for StrVisitor`

- <span id="strvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for StrVisitor`

- <span id="strvisitor-visitor-type-value"></span>`type Value = &'a str`

- <span id="strvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="strvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="strvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `BytesVisitor`

```rust
struct BytesVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:741`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L741)*

#### Trait Implementations

##### `impl Expected for BytesVisitor`

- <span id="bytesvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for BytesVisitor`

- <span id="bytesvisitor-visitor-type-value"></span>`type Value = &'a [u8]`

- <span id="bytesvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="bytesvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="bytesvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `CStringVisitor`

```rust
struct CStringVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:777`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L777)*

#### Trait Implementations

##### `impl Expected for CStringVisitor`

- <span id="cstringvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for CStringVisitor`

- <span id="cstringvisitor-visitor-type-value"></span>`type Value = CString`

- <span id="cstringvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="cstringvisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

### `OptionVisitor<T>`

```rust
struct OptionVisitor<T> {
    marker: PhantomData<T>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:870-872`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L870-L872)*

#### Trait Implementations

##### `impl<T> Expected for OptionVisitor<T>`

- <span id="optionvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for OptionVisitor<T>`

- <span id="optionvisitor-visitor-type-value"></span>`type Value = Option<T>`

- <span id="optionvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionvisitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="optionvisitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="optionvisitor-visit-some"></span>`fn visit_some<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md#visitor)

### `PhantomDataVisitor<T: ?Sized>`

```rust
struct PhantomDataVisitor<T: ?Sized> {
    marker: PhantomData<T>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:938-940`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L938-L940)*

#### Trait Implementations

##### `impl<T> Expected for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-visitor-type-value"></span>`type Value = PhantomData<T>`

- <span id="phantomdatavisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="phantomdatavisitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `ArrayVisitor<A>`

```rust
struct ArrayVisitor<A> {
    marker: PhantomData<A>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1228-1230`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1228-L1230)*

#### Implementations

- <span id="arrayvisitor-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Expected for ArrayVisitor<A>`

- <span id="arrayvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for ArrayVisitor<[T; 0]>`

- <span id="arrayvisitor-visitor-type-value"></span>`type Value = [T; 0]`

- <span id="arrayvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayvisitor-visit-seq"></span>`fn visit_seq<A>(self, _: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `ArrayInPlaceVisitor<'a, A: 'a>`

```rust
struct ArrayInPlaceVisitor<'a, A: 'a>(&'a mut A);
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1231`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1231)*

#### Trait Implementations

##### `impl Expected for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for ArrayInPlaceVisitor<'a, [T; 1]>`

- <span id="arrayinplacevisitor-visitor-type-value"></span>`type Value = ()`

- <span id="arrayinplacevisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayinplacevisitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `PathVisitor`

```rust
struct PathVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1785`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1785)*

#### Trait Implementations

##### `impl Expected for PathVisitor`

- <span id="pathvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for PathVisitor`

- <span id="pathvisitor-visitor-type-value"></span>`type Value = &'a Path`

- <span id="pathvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `PathBufVisitor`

```rust
struct PathBufVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1824`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1824)*

#### Trait Implementations

##### `impl Expected for PathBufVisitor`

- <span id="pathbufvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for PathBufVisitor`

- <span id="pathbufvisitor-visitor-type-value"></span>`type Value = PathBuf`

- <span id="pathbufvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathbufvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `OsStringVisitor`

```rust
struct OsStringVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1898`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1898)*

#### Trait Implementations

##### `impl Expected for OsStringVisitor`

- <span id="osstringvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Visitor for OsStringVisitor`

- <span id="osstringvisitor-visitor-type-value"></span>`type Value = OsString`

- <span id="osstringvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="osstringvisitor-visit-enum"></span>`fn visit_enum<A>(self, data: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `FromStrVisitor<T>`

```rust
struct FromStrVisitor<T> {
    expecting: &'static str,
    ty: PhantomData<T>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:3140-3143`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L3140-L3143)*

#### Implementations

- <span id="fromstrvisitor-new"></span>`fn new(expecting: &'static str) -> Self`

#### Trait Implementations

##### `impl<T> Expected for FromStrVisitor<T>`

- <span id="fromstrvisitor-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Visitor for FromStrVisitor<T>`

- <span id="fromstrvisitor-visitor-type-value"></span>`type Value = T`

- <span id="fromstrvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="fromstrvisitor-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

## Enums

### `OsStringKind`

```rust
enum OsStringKind {
    Unix,
    Windows,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1891-1895`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1891-L1895)*

#### Trait Implementations

##### `impl Deserialize for OsStringKind`

- <span id="osstringkind-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../index.md#deserializer)

##### `impl DeserializeOwned for OsStringKind`

## Functions

### `nop_reserve`

```rust
fn nop_reserve<T>(_seq: T, _n: usize)
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1074`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1074)*

## Macros

### `impl_deserialize_num!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:81-152`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L81-L152)*

### `num_self!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:154-186`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L154-L186)*

### `num_as_self!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:188-220`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L188-L220)*

### `num_as_copysign_self!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:222-242`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L222-L242)*

### `int_to_int!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:244-284`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L244-L284)*

### `int_to_uint!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:286-334`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L286-L334)*

### `uint_to_self!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:336-374`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L336-L374)*

### `num_128!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:466-517`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L466-L517)*

### `forwarded_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:841-856`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L841-L856)*

### `seq_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:978-1070`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L978-L1070)*

### `array_impls!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1267-1343`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1267-L1343)*

### `tuple_impls!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1382-1394`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1382-L1394)*

### `tuple_impl_body!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1396-1467`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1396-L1467)*

### `map_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1501-1555`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1501-L1555)*

### `parse_ip_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1576-1591`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1576-L1591)*

### `variant_identifier!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1594-1666`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1594-L1666)*

### `deserialize_enum!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1669-1703`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1669-L1703)*

### `parse_socket_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1731-1749`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1731-L1749)*

### `box_forwarded_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:2042-2061`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L2042-L2061)*

### `atomic_impl!`

*Defined in [`serde_core-1.0.228/src/de/impls.rs:3103-3118`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L3103-L3118)*

