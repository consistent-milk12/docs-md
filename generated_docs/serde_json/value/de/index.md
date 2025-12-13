*[serde_json](../../index.md) / [value](../index.md) / [de](index.md)*

---

# Module `de`

## Contents

- [Structs](#structs)
  - [`EnumDeserializer`](#enumdeserializer)
  - [`VariantDeserializer`](#variantdeserializer)
  - [`SeqDeserializer`](#seqdeserializer)
  - [`MapDeserializer`](#mapdeserializer)
  - [`EnumRefDeserializer`](#enumrefdeserializer)
  - [`VariantRefDeserializer`](#variantrefdeserializer)
  - [`SeqRefDeserializer`](#seqrefdeserializer)
  - [`MapRefDeserializer`](#maprefdeserializer)
  - [`MapKeyDeserializer`](#mapkeydeserializer)
  - [`KeyClassifier`](#keyclassifier)
  - [`BorrowedCowStrDeserializer`](#borrowedcowstrdeserializer)
  - [`UnitOnly`](#unitonly)
- [Enums](#enums)
  - [`KeyClass`](#keyclass)
- [Functions](#functions)
  - [`visit_array`](#visit-array)
  - [`visit_array_ref`](#visit-array-ref)
- [Macros](#macros)
  - [`deserialize_number!`](#deserialize-number)
  - [`deserialize_value_ref_number!`](#deserialize-value-ref-number)
  - [`deserialize_numeric_key!`](#deserialize-numeric-key)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EnumDeserializer`](#enumdeserializer) | struct |  |
| [`VariantDeserializer`](#variantdeserializer) | struct |  |
| [`SeqDeserializer`](#seqdeserializer) | struct |  |
| [`MapDeserializer`](#mapdeserializer) | struct |  |
| [`EnumRefDeserializer`](#enumrefdeserializer) | struct |  |
| [`VariantRefDeserializer`](#variantrefdeserializer) | struct |  |
| [`SeqRefDeserializer`](#seqrefdeserializer) | struct |  |
| [`MapRefDeserializer`](#maprefdeserializer) | struct |  |
| [`MapKeyDeserializer`](#mapkeydeserializer) | struct |  |
| [`KeyClassifier`](#keyclassifier) | struct |  |
| [`BorrowedCowStrDeserializer`](#borrowedcowstrdeserializer) | struct |  |
| [`UnitOnly`](#unitonly) | struct |  |
| [`KeyClass`](#keyclass) | enum |  |
| [`visit_array`](#visit-array) | fn |  |
| [`visit_array_ref`](#visit-array-ref) | fn |  |
| [`deserialize_number!`](#deserialize-number) | macro |  |
| [`deserialize_value_ref_number!`](#deserialize-value-ref-number) | macro |  |
| [`deserialize_numeric_key!`](#deserialize-numeric-key) | macro |  |

## Structs

### `EnumDeserializer`

```rust
struct EnumDeserializer {
    variant: alloc::string::String,
    value: Option<crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:514-517`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L514-L517)*

#### Trait Implementations

##### `impl Any for EnumDeserializer`

- <span id="enumdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnumDeserializer`

- <span id="enumdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnumDeserializer`

- <span id="enumdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl EnumAccess for EnumDeserializer`

- <span id="enumdeserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="enumdeserializer-enumaccess-type-variant"></span>`type Variant = VariantDeserializer`

- <span id="enumdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, VariantDeserializer), Error>` — [`VariantDeserializer`](#variantdeserializer), [`Error`](../../error/index.md#error)

##### `impl<T> From for EnumDeserializer`

- <span id="enumdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EnumDeserializer`

- <span id="enumdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EnumDeserializer`

- <span id="enumdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enumdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnumDeserializer`

- <span id="enumdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enumdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VariantDeserializer`

```rust
struct VariantDeserializer {
    value: Option<crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:549-551`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L549-L551)*

#### Trait Implementations

##### `impl Any for VariantDeserializer`

- <span id="variantdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VariantDeserializer`

- <span id="variantdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VariantDeserializer`

- <span id="variantdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for VariantDeserializer`

- <span id="variantdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VariantDeserializer`

- <span id="variantdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VariantDeserializer`

- <span id="variantdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variantdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VariantDeserializer`

- <span id="variantdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variantdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl VariantAccess for VariantDeserializer`

- <span id="variantdeserializer-variantaccess-type-error"></span>`type Error = Error`

- <span id="variantdeserializer-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="variantdeserializer-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantdeserializer-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantdeserializer-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

### `SeqDeserializer`

```rust
struct SeqDeserializer {
    iter: vec::IntoIter<crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:621-623`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L621-L623)*

#### Implementations

- <span id="seqdeserializer-new"></span>`fn new(vec: Vec<Value>) -> Self` — [`Value`](../index.md#value)

#### Trait Implementations

##### `impl Any for SeqDeserializer`

- <span id="seqdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeqDeserializer`

- <span id="seqdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeqDeserializer`

- <span id="seqdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SeqDeserializer`

- <span id="seqdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeqDeserializer`

- <span id="seqdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl SeqAccess for SeqDeserializer`

- <span id="seqdeserializer-seqaccess-type-error"></span>`type Error = Error`

- <span id="seqdeserializer-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="seqdeserializer-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

##### `impl<U> TryFrom for SeqDeserializer`

- <span id="seqdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seqdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeqDeserializer`

- <span id="seqdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seqdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapDeserializer`

```rust
struct MapDeserializer {
    iter: <crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:654-657`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L654-L657)*

#### Implementations

- <span id="mapdeserializer-new"></span>`fn new(map: Map<String, Value>) -> Self` — [`Map`](../../map/index.md#map), [`Value`](../index.md#value)

#### Trait Implementations

##### `impl Any for MapDeserializer`

- <span id="mapdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapDeserializer`

- <span id="mapdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapDeserializer`

- <span id="mapdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapDeserializer`

- <span id="mapdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapDeserializer`

- <span id="mapdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl MapAccess for MapDeserializer`

- <span id="mapdeserializer-mapaccess-type-error"></span>`type Error = Error`

- <span id="mapdeserializer-mapaccess-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapdeserializer-mapaccess-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapdeserializer-mapaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

##### `impl<U> TryFrom for MapDeserializer`

- <span id="mapdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapDeserializer`

- <span id="mapdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EnumRefDeserializer<'de>`

```rust
struct EnumRefDeserializer<'de> {
    variant: &'de str,
    value: Option<&'de crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1040-1043`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1040-L1043)*

#### Trait Implementations

##### `impl Any for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl EnumAccess for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="enumrefdeserializer-enumaccess-type-variant"></span>`type Variant = VariantRefDeserializer<'de>`

- <span id="enumrefdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<V>(self, seed: V) -> Result<(<V as >::Value, <Self as >::Variant), Error>` — [`Error`](../../error/index.md#error)

##### `impl<T> From for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enumrefdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnumRefDeserializer<'de>`

- <span id="enumrefdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enumrefdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VariantRefDeserializer<'de>`

```rust
struct VariantRefDeserializer<'de> {
    value: Option<&'de crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1059-1061`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1059-L1061)*

#### Trait Implementations

##### `impl Any for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variantrefdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variantrefdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl VariantAccess for VariantRefDeserializer<'de>`

- <span id="variantrefdeserializer-variantaccess-type-error"></span>`type Error = Error`

- <span id="variantrefdeserializer-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="variantrefdeserializer-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantrefdeserializer-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="variantrefdeserializer-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

### `SeqRefDeserializer<'de>`

```rust
struct SeqRefDeserializer<'de> {
    iter: slice::Iter<'de, crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1131-1133`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1131-L1133)*

#### Implementations

- <span id="seqrefdeserializer-new"></span>`fn new(slice: &'de [Value]) -> Self` — [`Value`](../index.md#value)

#### Trait Implementations

##### `impl Any for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl SeqAccess for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-seqaccess-type-error"></span>`type Error = Error`

- <span id="seqrefdeserializer-seqaccess-next-element-seed"></span>`fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="seqrefdeserializer-seqaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

##### `impl<U> TryFrom for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seqrefdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeqRefDeserializer<'de>`

- <span id="seqrefdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seqrefdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapRefDeserializer<'de>`

```rust
struct MapRefDeserializer<'de> {
    iter: <&'de crate::map::Map<alloc::string::String, crate::value::Value> as IntoIterator>::IntoIter,
    value: Option<&'de crate::value::Value>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1162-1165`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1162-L1165)*

#### Implementations

- <span id="maprefdeserializer-new"></span>`fn new(map: &'de Map<String, Value>) -> Self` — [`Map`](../../map/index.md#map), [`Value`](../index.md#value)

#### Trait Implementations

##### `impl Any for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl MapAccess for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-mapaccess-type-error"></span>`type Error = Error`

- <span id="maprefdeserializer-mapaccess-next-key-seed"></span>`fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<<T as >::Value>, Error>` — [`Error`](../../error/index.md#error)

- <span id="maprefdeserializer-mapaccess-next-value-seed"></span>`fn next_value_seed<T>(&mut self, seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="maprefdeserializer-mapaccess-size-hint"></span>`fn size_hint(&self) -> Option<usize>`

##### `impl<U> TryFrom for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maprefdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapRefDeserializer<'de>`

- <span id="maprefdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maprefdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapKeyDeserializer<'de>`

```rust
struct MapKeyDeserializer<'de> {
    key: alloc::borrow::Cow<'de, str>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1213-1215`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1213-L1215)*

#### Trait Implementations

##### `impl Any for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deserializer for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="mapkeydeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="mapkeydeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="mapkeydeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl<T> From for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapkeydeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapKeyDeserializer<'de>`

- <span id="mapkeydeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapkeydeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `KeyClassifier`

```rust
struct KeyClassifier;
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1329`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1329)*

#### Trait Implementations

##### `impl Any for KeyClassifier`

- <span id="keyclassifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KeyClassifier`

- <span id="keyclassifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KeyClassifier`

- <span id="keyclassifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl DeserializeSeed for KeyClassifier`

- <span id="keyclassifier-deserializeseed-type-value"></span>`type Value = KeyClass`

- <span id="keyclassifier-deserializeseed-deserialize"></span>`fn deserialize<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

##### `impl Expected for KeyClassifier`

- <span id="keyclassifier-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl<T> From for KeyClassifier`

- <span id="keyclassifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for KeyClassifier`

- <span id="keyclassifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for KeyClassifier`

- <span id="keyclassifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="keyclassifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KeyClassifier`

- <span id="keyclassifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="keyclassifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor for KeyClassifier`

- <span id="keyclassifier-visitor-type-value"></span>`type Value = KeyClass`

- <span id="keyclassifier-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="keyclassifier-visitor-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>`

- <span id="keyclassifier-visitor-visit-string"></span>`fn visit_string<E>(self, s: String) -> Result<<Self as >::Value, E>`

### `BorrowedCowStrDeserializer<'de>`

```rust
struct BorrowedCowStrDeserializer<'de> {
    value: alloc::borrow::Cow<'de, str>,
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1407-1409`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1407-L1409)*

#### Implementations

- <span id="borrowedcowstrdeserializer-new"></span>`fn new(value: Cow<'de, str>) -> Self`

#### Trait Implementations

##### `impl Any for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deserializer for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-deserializer-type-error"></span>`type Error = Error`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-any"></span>`fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-enum"></span>`fn deserialize_enum<V>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-bool"></span>`fn deserialize_bool<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i8"></span>`fn deserialize_i8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i16"></span>`fn deserialize_i16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i32"></span>`fn deserialize_i32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i64"></span>`fn deserialize_i64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-i128"></span>`fn deserialize_i128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u8"></span>`fn deserialize_u8<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u16"></span>`fn deserialize_u16<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u32"></span>`fn deserialize_u32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u64"></span>`fn deserialize_u64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-u128"></span>`fn deserialize_u128<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-f32"></span>`fn deserialize_f32<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-f64"></span>`fn deserialize_f64<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-char"></span>`fn deserialize_char<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-str"></span>`fn deserialize_str<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-string"></span>`fn deserialize_string<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-bytes"></span>`fn deserialize_bytes<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-byte-buf"></span>`fn deserialize_byte_buf<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-option"></span>`fn deserialize_option<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-unit"></span>`fn deserialize_unit<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-unit-struct"></span>`fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-newtype-struct"></span>`fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-seq"></span>`fn deserialize_seq<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-tuple"></span>`fn deserialize_tuple<V>(self, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-tuple-struct"></span>`fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-map"></span>`fn deserialize_map<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-struct"></span>`fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-identifier"></span>`fn deserialize_identifier<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

- <span id="borrowedcowstrdeserializer-deserializer-deserialize-ignored-any"></span>`fn deserialize_ignored_any<V>(self, visitor: V) -> __private::Result<<V as >::Value, <Self as de::Deserializer>::Error>`

##### `impl EnumAccess for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-enumaccess-type-error"></span>`type Error = Error`

- <span id="borrowedcowstrdeserializer-enumaccess-type-variant"></span>`type Variant = UnitOnly`

- <span id="borrowedcowstrdeserializer-enumaccess-variant-seed"></span>`fn variant_seed<T>(self, seed: T) -> Result<(<T as >::Value, <Self as >::Variant), Error>` — [`Error`](../../error/index.md#error)

##### `impl<T> From for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="borrowedcowstrdeserializer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BorrowedCowStrDeserializer<'de>`

- <span id="borrowedcowstrdeserializer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="borrowedcowstrdeserializer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitOnly`

```rust
struct UnitOnly;
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1465`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1465)*

#### Trait Implementations

##### `impl Any for UnitOnly`

- <span id="unitonly-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitOnly`

- <span id="unitonly-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitOnly`

- <span id="unitonly-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UnitOnly`

- <span id="unitonly-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitOnly`

- <span id="unitonly-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for UnitOnly`

- <span id="unitonly-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitonly-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitOnly`

- <span id="unitonly-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitonly-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl VariantAccess for UnitOnly`

- <span id="unitonly-variantaccess-type-error"></span>`type Error = Error`

- <span id="unitonly-variantaccess-unit-variant"></span>`fn unit_variant(self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="unitonly-variantaccess-newtype-variant-seed"></span>`fn newtype_variant_seed<T>(self, _seed: T) -> Result<<T as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="unitonly-variantaccess-tuple-variant"></span>`fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

- <span id="unitonly-variantaccess-struct-variant"></span>`fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<<V as >::Value, Error>` — [`Error`](../../error/index.md#error)

## Enums

### `KeyClass`

```rust
enum KeyClass {
    Map(alloc::string::String),
}
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:1331-1337`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1331-L1337)*

#### Trait Implementations

##### `impl Any for KeyClass`

- <span id="keyclass-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KeyClass`

- <span id="keyclass-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KeyClass`

- <span id="keyclass-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for KeyClass`

- <span id="keyclass-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for KeyClass`

- <span id="keyclass-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for KeyClass`

- <span id="keyclass-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="keyclass-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KeyClass`

- <span id="keyclass-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="keyclass-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `visit_array`

```rust
fn visit_array<'de, V>(array: alloc::vec::Vec<crate::value::Value>, visitor: V) -> Result<<V as >::Value, crate::error::Error>
where
    V: Visitor<'de>
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:195-211`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L195-L211)*

### `visit_array_ref`

```rust
fn visit_array_ref<'de, V>(array: &'de [crate::value::Value], visitor: V) -> Result<<V as >::Value, crate::error::Error>
where
    V: Visitor<'de>
```

*Defined in [`serde_json-1.0.145/src/value/de.rs:731-747`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L731-L747)*

## Macros

### `deserialize_number!`

*Defined in [`serde_json-1.0.145/src/value/de.rs:169-193`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L169-L193)*

### `deserialize_value_ref_number!`

*Defined in [`serde_json-1.0.145/src/value/de.rs:705-729`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L705-L729)*

### `deserialize_numeric_key!`

*Defined in [`serde_json-1.0.145/src/value/de.rs:1217-1243`](../../../../.source_1765633015/serde_json-1.0.145/src/value/de.rs#L1217-L1243)*

