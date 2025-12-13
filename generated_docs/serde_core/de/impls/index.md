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

##### `impl Any for UnitVisitor`

- <span id="unitvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitVisitor`

- <span id="unitvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitVisitor`

- <span id="unitvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for UnitVisitor`

- <span id="unitvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnitVisitor`

- <span id="unitvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitVisitor`

- <span id="unitvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for UnitVisitor`

- <span id="unitvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitVisitor`

- <span id="unitvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for UnitVisitor`

- <span id="unitvisitor-visitor-type-value"></span>`type Value = ()`

- <span id="unitvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="unitvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `BoolVisitor`

```rust
struct BoolVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:53`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L53)*

#### Trait Implementations

##### `impl Any for BoolVisitor`

- <span id="boolvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoolVisitor`

- <span id="boolvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoolVisitor`

- <span id="boolvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for BoolVisitor`

- <span id="boolvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BoolVisitor`

- <span id="boolvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoolVisitor`

- <span id="boolvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BoolVisitor`

- <span id="boolvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boolvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoolVisitor`

- <span id="boolvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boolvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for BoolVisitor`

- <span id="boolvisitor-visitor-type-value"></span>`type Value = bool`

- <span id="boolvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="boolvisitor-visitor-visit-bool"></span>`fn visit_bool<E>(self, v: bool) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `CharVisitor`

```rust
struct CharVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:537`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L537)*

#### Trait Implementations

##### `impl Any for CharVisitor`

- <span id="charvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CharVisitor`

- <span id="charvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CharVisitor`

- <span id="charvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for CharVisitor`

- <span id="charvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CharVisitor`

- <span id="charvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CharVisitor`

- <span id="charvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for CharVisitor`

- <span id="charvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="charvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CharVisitor`

- <span id="charvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="charvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for CharVisitor`

- <span id="charvisitor-visitor-type-value"></span>`type Value = char`

- <span id="charvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="charvisitor-visitor-visit-char"></span>`fn visit_char<E>(self, v: char) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="charvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `StringVisitor`

```rust
struct StringVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:580`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L580)*

#### Trait Implementations

##### `impl Any for StringVisitor`

- <span id="stringvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StringVisitor`

- <span id="stringvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StringVisitor`

- <span id="stringvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for StringVisitor`

- <span id="stringvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StringVisitor`

- <span id="stringvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StringVisitor`

- <span id="stringvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StringVisitor`

- <span id="stringvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stringvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StringVisitor`

- <span id="stringvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stringvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for StringVisitor`

- <span id="stringvisitor-visitor-type-value"></span>`type Value = String`

- <span id="stringvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `StringInPlaceVisitor<'a>`

```rust
struct StringInPlaceVisitor<'a>(&'a mut String);
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:582`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L582)*

#### Trait Implementations

##### `impl Any for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stringinplacevisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stringinplacevisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for StringInPlaceVisitor<'a>`

- <span id="stringinplacevisitor-visitor-type-value"></span>`type Value = ()`

- <span id="stringinplacevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="stringinplacevisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="stringinplacevisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `StrVisitor`

```rust
struct StrVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:706`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L706)*

#### Trait Implementations

##### `impl Any for StrVisitor`

- <span id="strvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrVisitor`

- <span id="strvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrVisitor`

- <span id="strvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for StrVisitor`

- <span id="strvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StrVisitor`

- <span id="strvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StrVisitor`

- <span id="strvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StrVisitor`

- <span id="strvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StrVisitor`

- <span id="strvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for StrVisitor`

- <span id="strvisitor-visitor-type-value"></span>`type Value = &'a str`

- <span id="strvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="strvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="strvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `BytesVisitor`

```rust
struct BytesVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:741`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L741)*

#### Trait Implementations

##### `impl Any for BytesVisitor`

- <span id="bytesvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BytesVisitor`

- <span id="bytesvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BytesVisitor`

- <span id="bytesvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for BytesVisitor`

- <span id="bytesvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BytesVisitor`

- <span id="bytesvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BytesVisitor`

- <span id="bytesvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BytesVisitor`

- <span id="bytesvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytesvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BytesVisitor`

- <span id="bytesvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytesvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for BytesVisitor`

- <span id="bytesvisitor-visitor-type-value"></span>`type Value = &'a [u8]`

- <span id="bytesvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="bytesvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="bytesvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `CStringVisitor`

```rust
struct CStringVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:777`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L777)*

#### Trait Implementations

##### `impl Any for CStringVisitor`

- <span id="cstringvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CStringVisitor`

- <span id="cstringvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CStringVisitor`

- <span id="cstringvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for CStringVisitor`

- <span id="cstringvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CStringVisitor`

- <span id="cstringvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CStringVisitor`

- <span id="cstringvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for CStringVisitor`

- <span id="cstringvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cstringvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CStringVisitor`

- <span id="cstringvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cstringvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for CStringVisitor`

- <span id="cstringvisitor-visitor-type-value"></span>`type Value = CString`

- <span id="cstringvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="cstringvisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="cstringvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

### `OptionVisitor<T>`

```rust
struct OptionVisitor<T> {
    marker: PhantomData<T>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:870-872`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L870-L872)*

#### Trait Implementations

##### `impl<T> Any for OptionVisitor<T>`

- <span id="optionvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OptionVisitor<T>`

- <span id="optionvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OptionVisitor<T>`

- <span id="optionvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Expected for OptionVisitor<T>`

- <span id="optionvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OptionVisitor<T>`

- <span id="optionvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OptionVisitor<T>`

- <span id="optionvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for OptionVisitor<T>`

- <span id="optionvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="optionvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OptionVisitor<T>`

- <span id="optionvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="optionvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Visitor for OptionVisitor<T>`

- <span id="optionvisitor-visitor-type-value"></span>`type Value = Option<T>`

- <span id="optionvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="optionvisitor-visitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="optionvisitor-visitor-visit-some"></span>`fn visit_some<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md#visitor)

### `PhantomDataVisitor<T: ?Sized>`

```rust
struct PhantomDataVisitor<T: ?Sized> {
    marker: PhantomData<T>,
}
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:938-940`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L938-L940)*

#### Trait Implementations

##### `impl<T> Any for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Expected for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="phantomdatavisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="phantomdatavisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Visitor for PhantomDataVisitor<T>`

- <span id="phantomdatavisitor-visitor-type-value"></span>`type Value = PhantomData<T>`

- <span id="phantomdatavisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="phantomdatavisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

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

##### `impl Any for ArrayVisitor<A>`

- <span id="arrayvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVisitor<A>`

- <span id="arrayvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVisitor<A>`

- <span id="arrayvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for ArrayVisitor<A>`

- <span id="arrayvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArrayVisitor<A>`

- <span id="arrayvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArrayVisitor<A>`

- <span id="arrayvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ArrayVisitor<A>`

- <span id="arrayvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVisitor<A>`

- <span id="arrayvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Visitor for ArrayVisitor<[T; 0]>`

- <span id="arrayvisitor-visitor-type-value"></span>`type Value = [T; 0]`

- <span id="arrayvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayvisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, _: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `ArrayInPlaceVisitor<'a, A: 'a>`

```rust
struct ArrayInPlaceVisitor<'a, A: 'a>(&'a mut A);
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1231`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1231)*

#### Trait Implementations

##### `impl Any for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayinplacevisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayInPlaceVisitor<'a, A>`

- <span id="arrayinplacevisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayinplacevisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Visitor for ArrayInPlaceVisitor<'a, [T; 1]>`

- <span id="arrayinplacevisitor-visitor-type-value"></span>`type Value = ()`

- <span id="arrayinplacevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="arrayinplacevisitor-visitor-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

### `PathVisitor`

```rust
struct PathVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1785`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1785)*

#### Trait Implementations

##### `impl Any for PathVisitor`

- <span id="pathvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathVisitor`

- <span id="pathvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathVisitor`

- <span id="pathvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for PathVisitor`

- <span id="pathvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PathVisitor`

- <span id="pathvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PathVisitor`

- <span id="pathvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for PathVisitor`

- <span id="pathvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathVisitor`

- <span id="pathvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for PathVisitor`

- <span id="pathvisitor-visitor-type-value"></span>`type Value = &'a Path`

- <span id="pathvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathvisitor-visitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathvisitor-visitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

### `PathBufVisitor`

```rust
struct PathBufVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1824`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1824)*

#### Trait Implementations

##### `impl Any for PathBufVisitor`

- <span id="pathbufvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathBufVisitor`

- <span id="pathbufvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathBufVisitor`

- <span id="pathbufvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for PathBufVisitor`

- <span id="pathbufvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PathBufVisitor`

- <span id="pathbufvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PathBufVisitor`

- <span id="pathbufvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for PathBufVisitor`

- <span id="pathbufvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathbufvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathBufVisitor`

- <span id="pathbufvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathbufvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for PathBufVisitor`

- <span id="pathbufvisitor-visitor-type-value"></span>`type Value = PathBuf`

- <span id="pathbufvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="pathbufvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>` — [`String`](../../lib/index.md#string), [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="pathbufvisitor-visitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>` — [`Vec`](../../lib/index.md#vec), [`Visitor`](../index.md#visitor)

### `OsStringVisitor`

```rust
struct OsStringVisitor;
```

*Defined in [`serde_core-1.0.228/src/de/impls.rs:1898`](../../../../.source_1765521767/serde_core-1.0.228/src/de/impls.rs#L1898)*

#### Trait Implementations

##### `impl Any for OsStringVisitor`

- <span id="osstringvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OsStringVisitor`

- <span id="osstringvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OsStringVisitor`

- <span id="osstringvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Expected for OsStringVisitor`

- <span id="osstringvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OsStringVisitor`

- <span id="osstringvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OsStringVisitor`

- <span id="osstringvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OsStringVisitor`

- <span id="osstringvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="osstringvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OsStringVisitor`

- <span id="osstringvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="osstringvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for OsStringVisitor`

- <span id="osstringvisitor-visitor-type-value"></span>`type Value = OsString`

- <span id="osstringvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="osstringvisitor-visitor-visit-enum"></span>`fn visit_enum<A>(self, data: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

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

##### `impl<T> Any for FromStrVisitor<T>`

- <span id="fromstrvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FromStrVisitor<T>`

- <span id="fromstrvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FromStrVisitor<T>`

- <span id="fromstrvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Expected for FromStrVisitor<T>`

- <span id="fromstrvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FromStrVisitor<T>`

- <span id="fromstrvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for FromStrVisitor<T>`

- <span id="fromstrvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for FromStrVisitor<T>`

- <span id="fromstrvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fromstrvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FromStrVisitor<T>`

- <span id="fromstrvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fromstrvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> Visitor for FromStrVisitor<T>`

- <span id="fromstrvisitor-visitor-type-value"></span>`type Value = T`

- <span id="fromstrvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="fromstrvisitor-visitor-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

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

##### `impl Any for OsStringKind`

- <span id="osstringkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OsStringKind`

- <span id="osstringkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OsStringKind`

- <span id="osstringkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deserialize for OsStringKind`

- <span id="osstringkind-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>` — [`Deserializer`](../index.md#deserializer)

##### `impl DeserializeOwned for OsStringKind`

##### `impl<T> From for OsStringKind`

- <span id="osstringkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OsStringKind`

- <span id="osstringkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OsStringKind`

- <span id="osstringkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="osstringkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OsStringKind`

- <span id="osstringkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="osstringkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

