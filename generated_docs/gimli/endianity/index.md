*[gimli](../index.md) / [endianity](index.md)*

---

# Module `endianity`

Types for compile-time and run-time endianity.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LittleEndian`](#littleendian) | struct | Little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Big endian byte order. |
| [`RunTimeEndian`](#runtimeendian) | enum | Byte order that is selectable at runtime. |
| [`Endianity`](#endianity) | trait | A trait describing the endianity of some buffer. |
| [`NativeEndian`](#nativeendian) | type | The native endianity for the target platform. |

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

*Defined in [`gimli-0.32.3/src/endianity.rs:206`](../../../.source_1765633015/gimli-0.32.3/src/endianity.rs#L206)*

Little endian byte order.

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

##### `impl Endianity for LittleEndian`

- <span id="littleendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

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

*Defined in [`gimli-0.32.3/src/endianity.rs:224`](../../../.source_1765633015/gimli-0.32.3/src/endianity.rs#L224)*

Big endian byte order.

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

##### `impl Endianity for BigEndian`

- <span id="bigendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

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

## Enums

### `RunTimeEndian`

```rust
enum RunTimeEndian {
    Little,
    Big,
}
```

*Defined in [`gimli-0.32.3/src/endianity.rs:176-181`](../../../.source_1765633015/gimli-0.32.3/src/endianity.rs#L176-L181)*

Byte order that is selectable at runtime.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Any for RunTimeEndian`

- <span id="runtimeendian-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RunTimeEndian`

- <span id="runtimeendian-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RunTimeEndian`

- <span id="runtimeendian-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RunTimeEndian`

- <span id="runtimeendian-clone"></span>`fn clone(&self) -> RunTimeEndian` — [`RunTimeEndian`](../index.md#runtimeendian)

##### `impl CloneToUninit for RunTimeEndian`

- <span id="runtimeendian-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RunTimeEndian`

##### `impl Debug for RunTimeEndian`

- <span id="runtimeendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RunTimeEndian`

- <span id="runtimeendian-default"></span>`fn default() -> RunTimeEndian` — [`RunTimeEndian`](../index.md#runtimeendian)

##### `impl Endianity for RunTimeEndian`

- <span id="runtimeendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for RunTimeEndian`

##### `impl<T> From for RunTimeEndian`

- <span id="runtimeendian-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RunTimeEndian`

- <span id="runtimeendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RunTimeEndian`

- <span id="runtimeendian-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RunTimeEndian`

- <span id="runtimeendian-partialeq-eq"></span>`fn eq(&self, other: &RunTimeEndian) -> bool` — [`RunTimeEndian`](../index.md#runtimeendian)

##### `impl StructuralPartialEq for RunTimeEndian`

##### `impl ToOwned for RunTimeEndian`

- <span id="runtimeendian-toowned-type-owned"></span>`type Owned = T`

- <span id="runtimeendian-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="runtimeendian-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RunTimeEndian`

- <span id="runtimeendian-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="runtimeendian-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RunTimeEndian`

- <span id="runtimeendian-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="runtimeendian-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Endianity`

```rust
trait Endianity: Debug + Default + Clone + Copy + PartialEq + Eq { ... }
```

*Defined in [`gimli-0.32.3/src/endianity.rs:7-172`](../../../.source_1765633015/gimli-0.32.3/src/endianity.rs#L7-L172)*

A trait describing the endianity of some buffer.

#### Required Methods

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

#### Provided Methods

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, buf: &[u8]) -> u16`

  Reads an unsigned 16 bit integer from `buf`.

- `fn read_u32(self, buf: &[u8]) -> u32`

  Reads an unsigned 32 bit integer from `buf`.

- `fn read_u64(self, buf: &[u8]) -> u64`

  Reads an unsigned 64 bit integer from `buf`.

- `fn read_uint(&mut self, buf: &[u8]) -> u64`

  Read an unsigned n-bytes integer u64.

- `fn read_i16(self, buf: &[u8]) -> i16`

  Reads a signed 16 bit integer from `buf`.

- `fn read_i32(self, buf: &[u8]) -> i32`

  Reads a signed 32 bit integer from `buf`.

- `fn read_i64(self, buf: &[u8]) -> i64`

  Reads a signed 64 bit integer from `buf`.

- `fn read_f32(self, buf: &[u8]) -> f32`

  Reads a 32 bit floating point number from `buf`.

- `fn read_f64(self, buf: &[u8]) -> f64`

  Reads a 32 bit floating point number from `buf`.

- `fn write_u16(self, buf: &mut [u8], n: u16)`

  Writes an unsigned 16 bit integer `n` to `buf`.

- `fn write_u32(self, buf: &mut [u8], n: u32)`

  Writes an unsigned 32 bit integer `n` to `buf`.

- `fn write_u64(self, buf: &mut [u8], n: u64)`

  Writes an unsigned 64 bit integer `n` to `buf`.

#### Implementors

- [`BigEndian`](../index.md#bigendian)
- [`LittleEndian`](../index.md#littleendian)
- [`RunTimeEndian`](../index.md#runtimeendian)

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

*Defined in [`gimli-0.32.3/src/endianity.rs:242`](../../../.source_1765633015/gimli-0.32.3/src/endianity.rs#L242)*

The native endianity for the target platform.

