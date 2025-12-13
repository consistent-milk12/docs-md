*[object](../../index.md) / [read](../index.md) / [util](index.md)*

---

# Module `util`

## Contents

- [Structs](#structs)
  - [`Bytes`](#bytes)
  - [`DebugByte`](#debugbyte)
  - [`DebugLen`](#debuglen)
  - [`ByteString`](#bytestring)
  - [`StringTable`](#stringtable)
- [Functions](#functions)
  - [`debug_list_bytes`](#debug-list-bytes)
  - [`align`](#align)
  - [`data_range`](#data-range)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bytes`](#bytes) | struct | A newtype for byte slices. |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |
| [`ByteString`](#bytestring) | struct | A newtype for byte strings. |
| [`StringTable`](#stringtable) | struct | A table of zero-terminated strings. |
| [`debug_list_bytes`](#debug-list-bytes) | fn |  |
| [`align`](#align) | fn |  |
| [`data_range`](#data-range) | fn |  |

## Structs

### `Bytes<'data>`

```rust
struct Bytes<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:16`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L16)*

A newtype for byte slices.

It has these important features:
- no methods that can panic, such as `Index`
- convenience methods for `Pod` types
- a useful `Debug` implementation

#### Implementations

- <span id="bytes-len"></span>`fn len(&self) -> usize`

  Return the length of the byte slice.

- <span id="bytes-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the byte slice is empty.

- <span id="bytes-skip"></span>`fn skip(&mut self, offset: usize) -> Result<(), ()>`

  Skip over the given number of bytes at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes.

- <span id="bytes-read-bytes"></span>`fn read_bytes(&mut self, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](../index.md#bytes)

  Return a reference to the given number of bytes at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes.

- <span id="bytes-read-bytes-at"></span>`fn read_bytes_at(self, offset: usize, count: usize) -> Result<Bytes<'data>, ()>` — [`Bytes`](../index.md#bytes)

  Return a reference to the given number of bytes at the given offset of the byte slice.

  

  Returns an error if the offset is invalid or there are too few bytes.

- <span id="bytes-read"></span>`fn read<T: Pod>(&mut self) -> Result<&'data T, ()>`

  Return a reference to a `Pod` struct at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes or the slice is incorrectly aligned.

- <span id="bytes-read-at"></span>`fn read_at<T: Pod>(self, offset: usize) -> Result<&'data T, ()>`

  Return a reference to a `Pod` struct at the given offset of the byte slice.

  

  Returns an error if there are too few bytes or the offset is incorrectly aligned.

- <span id="bytes-read-slice"></span>`fn read_slice<T: Pod>(&mut self, count: usize) -> Result<&'data [T], ()>`

  Return a reference to a slice of `Pod` structs at the start of the byte slice.

  

  Modifies the byte slice to start after the bytes.

  

  Returns an error if there are too few bytes or the offset is incorrectly aligned.

- <span id="bytes-read-slice-at"></span>`fn read_slice_at<T: Pod>(self, offset: usize, count: usize) -> Result<&'data [T], ()>`

  Return a reference to a slice of `Pod` structs at the given offset of the byte slice.

  

  Returns an error if there are too few bytes or the offset is incorrectly aligned.

- <span id="bytes-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8], ()>`

  Read a null terminated string.

  

  Does not assume any encoding.

  Reads past the null byte, but doesn't return it.

- <span id="bytes-read-string-at"></span>`fn read_string_at(self, offset: usize) -> Result<&'data [u8], ()>`

  Read a null terminated string at an offset.

  

  Does not assume any encoding. Does not return the null byte.

- <span id="bytes-read-uleb128"></span>`fn read_uleb128(&mut self) -> Result<u64, ()>`

  Read an unsigned LEB128 number.

- <span id="bytes-read-sleb128"></span>`fn read_sleb128(&mut self) -> Result<i64, ()>`

  Read a signed LEB128 number.

#### Trait Implementations

##### `impl Any for Bytes<'data>`

- <span id="bytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bytes<'data>`

- <span id="bytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bytes<'data>`

- <span id="bytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Bytes<'data>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'data>` — [`Bytes`](../index.md#bytes)

##### `impl CloneToUninit for Bytes<'data>`

- <span id="bytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Bytes<'data>`

##### `impl Debug for Bytes<'data>`

- <span id="bytes-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bytes<'data>`

- <span id="bytes-default"></span>`fn default() -> Bytes<'data>` — [`Bytes`](../index.md#bytes)

##### `impl Eq for Bytes<'data>`

##### `impl<T> From for Bytes<'data>`

- <span id="bytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bytes<'data>`

- <span id="bytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Bytes<'data>`

- <span id="bytes-partialeq-eq"></span>`fn eq(&self, other: &Bytes<'data>) -> bool` — [`Bytes`](../index.md#bytes)

##### `impl StructuralPartialEq for Bytes<'data>`

##### `impl ToOwned for Bytes<'data>`

- <span id="bytes-toowned-type-owned"></span>`type Owned = T`

- <span id="bytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Bytes<'data>`

- <span id="bytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bytes<'data>`

- <span id="bytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`object-0.37.3/src/read/util.rs:222`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L222)*

#### Trait Implementations

##### `impl Any for DebugByte`

- <span id="debugbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugByte`

- <span id="debugbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugByte`

- <span id="debugbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugByte`

- <span id="debugbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugByte`

- <span id="debugbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugByte`

- <span id="debugbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugByte`

- <span id="debugbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLen`

```rust
struct DebugLen(usize);
```

*Defined in [`object-0.37.3/src/read/util.rs:230`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L230)*

#### Trait Implementations

##### `impl Any for DebugLen`

- <span id="debuglen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLen`

- <span id="debuglen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLen`

- <span id="debuglen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugLen`

- <span id="debuglen-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugLen`

- <span id="debuglen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLen`

- <span id="debuglen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugLen`

- <span id="debuglen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLen`

- <span id="debuglen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteString<'data>`

```rust
struct ByteString<'data>(&'data [u8]);
```

*Defined in [`object-0.37.3/src/read/util.rs:244`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L244)*

A newtype for byte strings.

For byte slices that are strings of an unknown encoding.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Trait Implementations

##### `impl Any for ByteString<'data>`

- <span id="bytestring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteString<'data>`

- <span id="bytestring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteString<'data>`

- <span id="bytestring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ByteString<'data>`

- <span id="bytestring-clone"></span>`fn clone(&self) -> ByteString<'data>` — [`ByteString`](#bytestring)

##### `impl CloneToUninit for ByteString<'data>`

- <span id="bytestring-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ByteString<'data>`

##### `impl Debug for ByteString<'data>`

- <span id="bytestring-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteString<'data>`

- <span id="bytestring-default"></span>`fn default() -> ByteString<'data>` — [`ByteString`](#bytestring)

##### `impl Eq for ByteString<'data>`

##### `impl<T> From for ByteString<'data>`

- <span id="bytestring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteString<'data>`

- <span id="bytestring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ByteString<'data>`

- <span id="bytestring-partialeq-eq"></span>`fn eq(&self, other: &ByteString<'data>) -> bool` — [`ByteString`](#bytestring)

##### `impl StructuralPartialEq for ByteString<'data>`

##### `impl ToOwned for ByteString<'data>`

- <span id="bytestring-toowned-type-owned"></span>`type Owned = T`

- <span id="bytestring-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bytestring-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByteString<'data>`

- <span id="bytestring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytestring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteString<'data>`

- <span id="bytestring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytestring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StringTable<'data, R>`

```rust
struct StringTable<'data, R>
where
    R: ReadRef<'data> {
    data: Option<R>,
    start: u64,
    end: u64,
    marker: core::marker::PhantomData<&'data ()>,
}
```

*Defined in [`object-0.37.3/src/read/util.rs:274-282`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L274-L282)*

A table of zero-terminated strings.

This is used by most file formats for strings such as section names and symbol names.

#### Implementations

- <span id="stringtable-new"></span>`fn new(data: R, start: u64, end: u64) -> Self`

  Interpret the given data as a string table.

- <span id="stringtable-get"></span>`fn get(&self, offset: u32) -> Result<&'data [u8], ()>`

  Return the string at the given offset.

#### Trait Implementations

##### `impl Any for StringTable<'data, R>`

- <span id="stringtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StringTable<'data, R>`

- <span id="stringtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StringTable<'data, R>`

- <span id="stringtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for StringTable<'data, R>`

- <span id="stringtable-clone"></span>`fn clone(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

##### `impl CloneToUninit for StringTable<'data, R>`

- <span id="stringtable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R> Copy for StringTable<'data, R>`

##### `impl<R> Debug for StringTable<'data, R>`

- <span id="stringtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: ReadRef<'data>> Default for StringTable<'data, R>`

- <span id="stringtable-default"></span>`fn default() -> Self`

##### `impl<T> From for StringTable<'data, R>`

- <span id="stringtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StringTable<'data, R>`

- <span id="stringtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StringTable<'data, R>`

- <span id="stringtable-toowned-type-owned"></span>`type Owned = T`

- <span id="stringtable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stringtable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StringTable<'data, R>`

- <span id="stringtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stringtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StringTable<'data, R>`

- <span id="stringtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stringtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `debug_list_bytes`

```rust
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`object-0.37.3/src/read/util.rs:213-220`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L213-L220)*

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

*Defined in [`object-0.37.3/src/read/util.rs:254-256`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L254-L256)*

### `data_range`

```rust
fn data_range(data: &[u8], data_address: u64, range_address: u64, size: u64) -> Option<&[u8]>
```

*Defined in [`object-0.37.3/src/read/util.rs:259-268`](../../../../.source_1765521767/object-0.37.3/src/read/util.rs#L259-L268)*

