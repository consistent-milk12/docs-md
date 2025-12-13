*[object](../index.md) / [endian](index.md)*

---

# Module `endian`

Types for compile-time and run-time endianness.

## Contents

- [Structs](#structs)
  - [`LittleEndian`](#littleendian)
  - [`BigEndian`](#bigendian)
  - [`U16Bytes`](#u16bytes)
  - [`U32Bytes`](#u32bytes)
  - [`U64Bytes`](#u64bytes)
  - [`I16Bytes`](#i16bytes)
  - [`I32Bytes`](#i32bytes)
  - [`I64Bytes`](#i64bytes)
- [Enums](#enums)
  - [`Endianness`](#endianness)
- [Traits](#traits)
  - [`Endian`](#endian)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`U16`](#u16)
  - [`U32`](#u32)
  - [`U64`](#u64)
  - [`I16`](#i16)
  - [`I32`](#i32)
  - [`I64`](#i64)
- [Macros](#macros)
  - [`unsafe_impl_endian_pod!`](#unsafe-impl-endian-pod)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LittleEndian`](#littleendian) | struct | Compile-time little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Compile-time big endian byte order. |
| [`U16Bytes`](#u16bytes) | struct | An unaligned `u16` value with an externally specified endianness of type `E`. |
| [`U32Bytes`](#u32bytes) | struct | An unaligned `u32` value with an externally specified endianness of type `E`. |
| [`U64Bytes`](#u64bytes) | struct | An unaligned `u64` value with an externally specified endianness of type `E`. |
| [`I16Bytes`](#i16bytes) | struct | An unaligned `i16` value with an externally specified endianness of type `E`. |
| [`I32Bytes`](#i32bytes) | struct | An unaligned `i32` value with an externally specified endianness of type `E`. |
| [`I64Bytes`](#i64bytes) | struct | An unaligned `i64` value with an externally specified endianness of type `E`. |
| [`Endianness`](#endianness) | enum | An endianness that is selectable at run-time. |
| [`Endian`](#endian) | trait | A trait for using an endianness specification. |
| [`NativeEndian`](#nativeendian) | type | The native endianness for the target platform. |
| [`U16`](#u16) | type | A `u16` value with an externally specified endianness of type `E`. |
| [`U32`](#u32) | type | A `u32` value with an externally specified endianness of type `E`. |
| [`U64`](#u64) | type | A `u64` value with an externally specified endianness of type `E`. |
| [`I16`](#i16) | type | An `i16` value with an externally specified endianness of type `E`. |
| [`I32`](#i32) | type | An `i32` value with an externally specified endianness of type `E`. |
| [`I64`](#i64) | type | An `i64` value with an externally specified endianness of type `E`. |
| [`unsafe_impl_endian_pod!`](#unsafe-impl-endian-pod) | macro |  |

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

*Defined in [`object-0.37.3/src/endian.rs:317`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L317)*

Compile-time little endian byte order.

#### Trait Implementations

##### `impl Any for LittleEndian`

- <span id="littleendian-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LittleEndian`

- <span id="littleendian-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LittleEndian`

- <span id="littleendian-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](../index.md#littleendian)

##### `impl CloneToUninit for LittleEndian`

- <span id="littleendian-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](../index.md#littleendian)

##### `impl Endian for LittleEndian`

- <span id="littleendian-endian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="littleendian-endian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl<T> From for LittleEndian`

- <span id="littleendian-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LittleEndian`

- <span id="littleendian-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](../index.md#littleendian)

##### `impl StructuralPartialEq for LittleEndian`

##### `impl ToOwned for LittleEndian`

- <span id="littleendian-toowned-type-owned"></span>`type Owned = T`

- <span id="littleendian-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="littleendian-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LittleEndian`

- <span id="littleendian-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="littleendian-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LittleEndian`

- <span id="littleendian-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="littleendian-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BigEndian`

```rust
struct BigEndian;
```

*Defined in [`object-0.37.3/src/endian.rs:344`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L344)*

Compile-time big endian byte order.

#### Trait Implementations

##### `impl Any for BigEndian`

- <span id="bigendian-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BigEndian`

- <span id="bigendian-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BigEndian`

- <span id="bigendian-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](../index.md#bigendian)

##### `impl CloneToUninit for BigEndian`

- <span id="bigendian-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](../index.md#bigendian)

##### `impl Endian for BigEndian`

- <span id="bigendian-endian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="bigendian-endian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for BigEndian`

##### `impl<T> From for BigEndian`

- <span id="bigendian-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for BigEndian`

- <span id="bigendian-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](../index.md#bigendian)

##### `impl StructuralPartialEq for BigEndian`

##### `impl ToOwned for BigEndian`

- <span id="bigendian-toowned-type-owned"></span>`type Owned = T`

- <span id="bigendian-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bigendian-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BigEndian`

- <span id="bigendian-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bigendian-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BigEndian`

- <span id="bigendian-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bigendian-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `U16Bytes<E: Endian>`

```rust
struct U16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

*Defined in [`object-0.37.3/src/endian.rs:620`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L620)*

An unaligned `u16` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u16bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 2]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="u16bytes-new"></span>`fn new(e: E, n: u16) -> Self`

  Construct a new value given a native endian value.

- <span id="u16bytes-get"></span>`fn get(self, e: E) -> u16`

  Return the value as a native endian value.

- <span id="u16bytes-set"></span>`fn set(&mut self, e: E, n: u16)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl Any for U16Bytes<E>`

- <span id="u16bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for U16Bytes<E>`

- <span id="u16bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for U16Bytes<E>`

- <span id="u16bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for U16Bytes<E>`

- <span id="u16bytes-clone"></span>`fn clone(&self) -> U16Bytes<E>` — [`U16Bytes`](../index.md#u16bytes)

##### `impl CloneToUninit for U16Bytes<E>`

- <span id="u16bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for U16Bytes<E>`

##### `impl<E: Endian> Debug for U16Bytes<E>`

- <span id="u16bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U16Bytes<E>`

- <span id="u16bytes-default"></span>`fn default() -> U16Bytes<E>` — [`U16Bytes`](../index.md#u16bytes)

##### `impl<E: cmp::Eq + Endian> Eq for U16Bytes<E>`

##### `impl<T> From for U16Bytes<E>`

- <span id="u16bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<E: hash::Hash + Endian> Hash for U16Bytes<E>`

- <span id="u16bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for U16Bytes<E>`

- <span id="u16bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: cmp::Ord + Endian> Ord for U16Bytes<E>`

- <span id="u16bytes-ord-cmp"></span>`fn cmp(&self, other: &U16Bytes<E>) -> cmp::Ordering` — [`U16Bytes`](../index.md#u16bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U16Bytes<E>`

- <span id="u16bytes-partialeq-eq"></span>`fn eq(&self, other: &U16Bytes<E>) -> bool` — [`U16Bytes`](../index.md#u16bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U16Bytes<E>`

- <span id="u16bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &U16Bytes<E>) -> option::Option<cmp::Ordering>` — [`U16Bytes`](../index.md#u16bytes)

##### `impl<E: Endian> Pod for U16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U16Bytes<E>`

##### `impl ToOwned for U16Bytes<E>`

- <span id="u16bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="u16bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="u16bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for U16Bytes<E>`

- <span id="u16bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="u16bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for U16Bytes<E>`

- <span id="u16bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="u16bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `U32Bytes<E: Endian>`

```rust
struct U32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

*Defined in [`object-0.37.3/src/endian.rs:647`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L647)*

An unaligned `u32` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u32bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 4]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="u32bytes-new"></span>`fn new(e: E, n: u32) -> Self`

  Construct a new value given a native endian value.

- <span id="u32bytes-get"></span>`fn get(self, e: E) -> u32`

  Return the value as a native endian value.

- <span id="u32bytes-set"></span>`fn set(&mut self, e: E, n: u32)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl Any for U32Bytes<E>`

- <span id="u32bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for U32Bytes<E>`

- <span id="u32bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for U32Bytes<E>`

- <span id="u32bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for U32Bytes<E>`

- <span id="u32bytes-clone"></span>`fn clone(&self) -> U32Bytes<E>` — [`U32Bytes`](../index.md#u32bytes)

##### `impl CloneToUninit for U32Bytes<E>`

- <span id="u32bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for U32Bytes<E>`

##### `impl<E: Endian> Debug for U32Bytes<E>`

- <span id="u32bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U32Bytes<E>`

- <span id="u32bytes-default"></span>`fn default() -> U32Bytes<E>` — [`U32Bytes`](../index.md#u32bytes)

##### `impl<E: cmp::Eq + Endian> Eq for U32Bytes<E>`

##### `impl<T> From for U32Bytes<E>`

- <span id="u32bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<E: hash::Hash + Endian> Hash for U32Bytes<E>`

- <span id="u32bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for U32Bytes<E>`

- <span id="u32bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: cmp::Ord + Endian> Ord for U32Bytes<E>`

- <span id="u32bytes-ord-cmp"></span>`fn cmp(&self, other: &U32Bytes<E>) -> cmp::Ordering` — [`U32Bytes`](../index.md#u32bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U32Bytes<E>`

- <span id="u32bytes-partialeq-eq"></span>`fn eq(&self, other: &U32Bytes<E>) -> bool` — [`U32Bytes`](../index.md#u32bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U32Bytes<E>`

- <span id="u32bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &U32Bytes<E>) -> option::Option<cmp::Ordering>` — [`U32Bytes`](../index.md#u32bytes)

##### `impl<E: Endian> Pod for U32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U32Bytes<E>`

##### `impl ToOwned for U32Bytes<E>`

- <span id="u32bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="u32bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="u32bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for U32Bytes<E>`

- <span id="u32bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="u32bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for U32Bytes<E>`

- <span id="u32bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="u32bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `U64Bytes<E: Endian>`

```rust
struct U64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

*Defined in [`object-0.37.3/src/endian.rs:674`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L674)*

An unaligned `u64` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u64bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 8]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="u64bytes-new"></span>`fn new(e: E, n: u64) -> Self`

  Construct a new value given a native endian value.

- <span id="u64bytes-get"></span>`fn get(self, e: E) -> u64`

  Return the value as a native endian value.

- <span id="u64bytes-set"></span>`fn set(&mut self, e: E, n: u64)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl Any for U64Bytes<E>`

- <span id="u64bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for U64Bytes<E>`

- <span id="u64bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for U64Bytes<E>`

- <span id="u64bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for U64Bytes<E>`

- <span id="u64bytes-clone"></span>`fn clone(&self) -> U64Bytes<E>` — [`U64Bytes`](../index.md#u64bytes)

##### `impl CloneToUninit for U64Bytes<E>`

- <span id="u64bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for U64Bytes<E>`

##### `impl<E: Endian> Debug for U64Bytes<E>`

- <span id="u64bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U64Bytes<E>`

- <span id="u64bytes-default"></span>`fn default() -> U64Bytes<E>` — [`U64Bytes`](../index.md#u64bytes)

##### `impl<E: cmp::Eq + Endian> Eq for U64Bytes<E>`

##### `impl<T> From for U64Bytes<E>`

- <span id="u64bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<E: hash::Hash + Endian> Hash for U64Bytes<E>`

- <span id="u64bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for U64Bytes<E>`

- <span id="u64bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: cmp::Ord + Endian> Ord for U64Bytes<E>`

- <span id="u64bytes-ord-cmp"></span>`fn cmp(&self, other: &U64Bytes<E>) -> cmp::Ordering` — [`U64Bytes`](../index.md#u64bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U64Bytes<E>`

- <span id="u64bytes-partialeq-eq"></span>`fn eq(&self, other: &U64Bytes<E>) -> bool` — [`U64Bytes`](../index.md#u64bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U64Bytes<E>`

- <span id="u64bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &U64Bytes<E>) -> option::Option<cmp::Ordering>` — [`U64Bytes`](../index.md#u64bytes)

##### `impl<E: Endian> Pod for U64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U64Bytes<E>`

##### `impl ToOwned for U64Bytes<E>`

- <span id="u64bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="u64bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="u64bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for U64Bytes<E>`

- <span id="u64bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="u64bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for U64Bytes<E>`

- <span id="u64bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="u64bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `I16Bytes<E: Endian>`

```rust
struct I16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

*Defined in [`object-0.37.3/src/endian.rs:701`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L701)*

An unaligned `i16` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i16bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 2]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="i16bytes-new"></span>`fn new(e: E, n: i16) -> Self`

  Construct a new value given a native endian value.

- <span id="i16bytes-get"></span>`fn get(self, e: E) -> i16`

  Return the value as a native endian value.

- <span id="i16bytes-set"></span>`fn set(&mut self, e: E, n: i16)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl Any for I16Bytes<E>`

- <span id="i16bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for I16Bytes<E>`

- <span id="i16bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for I16Bytes<E>`

- <span id="i16bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for I16Bytes<E>`

- <span id="i16bytes-clone"></span>`fn clone(&self) -> I16Bytes<E>` — [`I16Bytes`](../index.md#i16bytes)

##### `impl CloneToUninit for I16Bytes<E>`

- <span id="i16bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for I16Bytes<E>`

##### `impl<E: Endian> Debug for I16Bytes<E>`

- <span id="i16bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I16Bytes<E>`

- <span id="i16bytes-default"></span>`fn default() -> I16Bytes<E>` — [`I16Bytes`](../index.md#i16bytes)

##### `impl<E: cmp::Eq + Endian> Eq for I16Bytes<E>`

##### `impl<T> From for I16Bytes<E>`

- <span id="i16bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<E: hash::Hash + Endian> Hash for I16Bytes<E>`

- <span id="i16bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for I16Bytes<E>`

- <span id="i16bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: cmp::Ord + Endian> Ord for I16Bytes<E>`

- <span id="i16bytes-ord-cmp"></span>`fn cmp(&self, other: &I16Bytes<E>) -> cmp::Ordering` — [`I16Bytes`](../index.md#i16bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I16Bytes<E>`

- <span id="i16bytes-partialeq-eq"></span>`fn eq(&self, other: &I16Bytes<E>) -> bool` — [`I16Bytes`](../index.md#i16bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I16Bytes<E>`

- <span id="i16bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &I16Bytes<E>) -> option::Option<cmp::Ordering>` — [`I16Bytes`](../index.md#i16bytes)

##### `impl<E: Endian> Pod for I16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I16Bytes<E>`

##### `impl ToOwned for I16Bytes<E>`

- <span id="i16bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="i16bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="i16bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for I16Bytes<E>`

- <span id="i16bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="i16bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for I16Bytes<E>`

- <span id="i16bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="i16bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `I32Bytes<E: Endian>`

```rust
struct I32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

*Defined in [`object-0.37.3/src/endian.rs:728`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L728)*

An unaligned `i32` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i32bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 4]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="i32bytes-new"></span>`fn new(e: E, n: i32) -> Self`

  Construct a new value given a native endian value.

- <span id="i32bytes-get"></span>`fn get(self, e: E) -> i32`

  Return the value as a native endian value.

- <span id="i32bytes-set"></span>`fn set(&mut self, e: E, n: i32)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl Any for I32Bytes<E>`

- <span id="i32bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for I32Bytes<E>`

- <span id="i32bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for I32Bytes<E>`

- <span id="i32bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for I32Bytes<E>`

- <span id="i32bytes-clone"></span>`fn clone(&self) -> I32Bytes<E>` — [`I32Bytes`](../index.md#i32bytes)

##### `impl CloneToUninit for I32Bytes<E>`

- <span id="i32bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for I32Bytes<E>`

##### `impl<E: Endian> Debug for I32Bytes<E>`

- <span id="i32bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I32Bytes<E>`

- <span id="i32bytes-default"></span>`fn default() -> I32Bytes<E>` — [`I32Bytes`](../index.md#i32bytes)

##### `impl<E: cmp::Eq + Endian> Eq for I32Bytes<E>`

##### `impl<T> From for I32Bytes<E>`

- <span id="i32bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<E: hash::Hash + Endian> Hash for I32Bytes<E>`

- <span id="i32bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for I32Bytes<E>`

- <span id="i32bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: cmp::Ord + Endian> Ord for I32Bytes<E>`

- <span id="i32bytes-ord-cmp"></span>`fn cmp(&self, other: &I32Bytes<E>) -> cmp::Ordering` — [`I32Bytes`](../index.md#i32bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I32Bytes<E>`

- <span id="i32bytes-partialeq-eq"></span>`fn eq(&self, other: &I32Bytes<E>) -> bool` — [`I32Bytes`](../index.md#i32bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I32Bytes<E>`

- <span id="i32bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &I32Bytes<E>) -> option::Option<cmp::Ordering>` — [`I32Bytes`](../index.md#i32bytes)

##### `impl<E: Endian> Pod for I32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I32Bytes<E>`

##### `impl ToOwned for I32Bytes<E>`

- <span id="i32bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="i32bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="i32bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for I32Bytes<E>`

- <span id="i32bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="i32bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for I32Bytes<E>`

- <span id="i32bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="i32bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `I64Bytes<E: Endian>`

```rust
struct I64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

*Defined in [`object-0.37.3/src/endian.rs:755`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L755)*

An unaligned `i64` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i64bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 8]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="i64bytes-new"></span>`fn new(e: E, n: i64) -> Self`

  Construct a new value given a native endian value.

- <span id="i64bytes-get"></span>`fn get(self, e: E) -> i64`

  Return the value as a native endian value.

- <span id="i64bytes-set"></span>`fn set(&mut self, e: E, n: i64)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl Any for I64Bytes<E>`

- <span id="i64bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for I64Bytes<E>`

- <span id="i64bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for I64Bytes<E>`

- <span id="i64bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + Endian> Clone for I64Bytes<E>`

- <span id="i64bytes-clone"></span>`fn clone(&self) -> I64Bytes<E>` — [`I64Bytes`](../index.md#i64bytes)

##### `impl CloneToUninit for I64Bytes<E>`

- <span id="i64bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: marker::Copy + Endian> Copy for I64Bytes<E>`

##### `impl<E: Endian> Debug for I64Bytes<E>`

- <span id="i64bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I64Bytes<E>`

- <span id="i64bytes-default"></span>`fn default() -> I64Bytes<E>` — [`I64Bytes`](../index.md#i64bytes)

##### `impl<E: cmp::Eq + Endian> Eq for I64Bytes<E>`

##### `impl<T> From for I64Bytes<E>`

- <span id="i64bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<E: hash::Hash + Endian> Hash for I64Bytes<E>`

- <span id="i64bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for I64Bytes<E>`

- <span id="i64bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<E: cmp::Ord + Endian> Ord for I64Bytes<E>`

- <span id="i64bytes-ord-cmp"></span>`fn cmp(&self, other: &I64Bytes<E>) -> cmp::Ordering` — [`I64Bytes`](../index.md#i64bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I64Bytes<E>`

- <span id="i64bytes-partialeq-eq"></span>`fn eq(&self, other: &I64Bytes<E>) -> bool` — [`I64Bytes`](../index.md#i64bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I64Bytes<E>`

- <span id="i64bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &I64Bytes<E>) -> option::Option<cmp::Ordering>` — [`I64Bytes`](../index.md#i64bytes)

##### `impl<E: Endian> Pod for I64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I64Bytes<E>`

##### `impl ToOwned for I64Bytes<E>`

- <span id="i64bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="i64bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="i64bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for I64Bytes<E>`

- <span id="i64bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="i64bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for I64Bytes<E>`

- <span id="i64bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="i64bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Endianness`

```rust
enum Endianness {
    Little,
    Big,
}
```

*Defined in [`object-0.37.3/src/endian.rs:278-283`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L278-L283)*

An endianness that is selectable at run-time.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Any for Endianness`

- <span id="endianness-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Endianness`

- <span id="endianness-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Endianness`

- <span id="endianness-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Endianness`

- <span id="endianness-clone"></span>`fn clone(&self) -> Endianness` — [`Endianness`](../index.md#endianness)

##### `impl CloneToUninit for Endianness`

- <span id="endianness-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Endianness`

##### `impl Debug for Endianness`

- <span id="endianness-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Endianness`

- <span id="endianness-default"></span>`fn default() -> Endianness` — [`Endianness`](../index.md#endianness)

##### `impl Endian for Endianness`

- <span id="endianness-endian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="endianness-endian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for Endianness`

##### `impl<T> From for Endianness`

- <span id="endianness-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Endianness`

- <span id="endianness-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Endianness`

- <span id="endianness-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Endianness`

- <span id="endianness-partialeq-eq"></span>`fn eq(&self, other: &Endianness) -> bool` — [`Endianness`](../index.md#endianness)

##### `impl StructuralPartialEq for Endianness`

##### `impl ToOwned for Endianness`

- <span id="endianness-toowned-type-owned"></span>`type Owned = T`

- <span id="endianness-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="endianness-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Endianness`

- <span id="endianness-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="endianness-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Endianness`

- <span id="endianness-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="endianness-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Endian`

```rust
trait Endian: Debug + Default + Clone + Copy + PartialEq + Eq + 'static { ... }
```

*Defined in [`object-0.37.3/src/endian.rs:13-274`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L13-L274)*

A trait for using an endianness specification.

Provides methods for converting between the specified endianness and
the native endianness of the target machine.

This trait does not require that the endianness is known at compile time.

#### Required Methods

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

#### Provided Methods

- `fn from_little_endian(little_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, n: u16) -> u16`

  Converts an unsigned 16 bit integer to native endian.

- `fn read_u32(self, n: u32) -> u32`

  Converts an unsigned 32 bit integer to native endian.

- `fn read_u64(self, n: u64) -> u64`

  Converts an unsigned 64 bit integer to native endian.

- `fn read_i16(self, n: i16) -> i16`

  Converts a signed 16 bit integer to native endian.

- `fn read_i32(self, n: i32) -> i32`

  Converts a signed 32 bit integer to native endian.

- `fn read_i64(self, n: i64) -> i64`

  Converts a signed 64 bit integer to native endian.

- `fn read_u16_bytes(self, n: [u8; 2]) -> u16`

  Converts an unaligned unsigned 16 bit integer to native endian.

- `fn read_u32_bytes(self, n: [u8; 4]) -> u32`

  Converts an unaligned unsigned 32 bit integer to native endian.

- `fn read_u64_bytes(self, n: [u8; 8]) -> u64`

  Converts an unaligned unsigned 64 bit integer to native endian.

- `fn read_i16_bytes(self, n: [u8; 2]) -> i16`

  Converts an unaligned signed 16 bit integer to native endian.

- `fn read_i32_bytes(self, n: [u8; 4]) -> i32`

  Converts an unaligned signed 32 bit integer to native endian.

- `fn read_i64_bytes(self, n: [u8; 8]) -> i64`

  Converts an unaligned signed 64 bit integer to native endian.

- `fn write_u16(self, n: u16) -> u16`

  Converts an unsigned 16 bit integer from native endian.

- `fn write_u32(self, n: u32) -> u32`

  Converts an unsigned 32 bit integer from native endian.

- `fn write_u64(self, n: u64) -> u64`

  Converts an unsigned 64 bit integer from native endian.

- `fn write_i16(self, n: i16) -> i16`

  Converts a signed 16 bit integer from native endian.

- `fn write_i32(self, n: i32) -> i32`

  Converts a signed 32 bit integer from native endian.

- `fn write_i64(self, n: i64) -> i64`

  Converts a signed 64 bit integer from native endian.

- `fn write_u16_bytes(self, n: u16) -> [u8; 2]`

  Converts an unaligned unsigned 16 bit integer from native endian.

- `fn write_u32_bytes(self, n: u32) -> [u8; 4]`

  Converts an unaligned unsigned 32 bit integer from native endian.

- `fn write_u64_bytes(self, n: u64) -> [u8; 8]`

  Converts an unaligned unsigned 64 bit integer from native endian.

- `fn write_i16_bytes(self, n: i16) -> [u8; 2]`

  Converts an unaligned signed 16 bit integer from native endian.

- `fn write_i32_bytes(self, n: i32) -> [u8; 4]`

  Converts an unaligned signed 32 bit integer from native endian.

- `fn write_i64_bytes(self, n: i64) -> [u8; 8]`

  Converts an unaligned signed 64 bit integer from native endian.

#### Implementors

- [`BigEndian`](../index.md#bigendian)
- [`Endianness`](../index.md#endianness)
- [`LittleEndian`](../index.md#littleendian)

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

*Defined in [`object-0.37.3/src/endian.rs:371`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L371)*

The native endianness for the target platform.

### `U16<E>`

```rust
type U16<E> = U16Bytes<E>;
```

*Defined in [`object-0.37.3/src/endian.rs:595`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L595)*

A `u16` value with an externally specified endianness of type `E`.

### `U32<E>`

```rust
type U32<E> = U32Bytes<E>;
```

*Defined in [`object-0.37.3/src/endian.rs:599`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L599)*

A `u32` value with an externally specified endianness of type `E`.

### `U64<E>`

```rust
type U64<E> = U64Bytes<E>;
```

*Defined in [`object-0.37.3/src/endian.rs:603`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L603)*

A `u64` value with an externally specified endianness of type `E`.

### `I16<E>`

```rust
type I16<E> = I16Bytes<E>;
```

*Defined in [`object-0.37.3/src/endian.rs:607`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L607)*

An `i16` value with an externally specified endianness of type `E`.

### `I32<E>`

```rust
type I32<E> = I32Bytes<E>;
```

*Defined in [`object-0.37.3/src/endian.rs:611`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L611)*

An `i32` value with an externally specified endianness of type `E`.

### `I64<E>`

```rust
type I64<E> = I64Bytes<E>;
```

*Defined in [`object-0.37.3/src/endian.rs:615`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L615)*

An `i64` value with an externally specified endianness of type `E`.

## Macros

### `unsafe_impl_endian_pod!`

*Defined in [`object-0.37.3/src/endian.rs:387-393`](../../../.source_1765521767/object-0.37.3/src/endian.rs#L387-L393)*

