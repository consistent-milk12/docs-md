*[gimli](../../index.md) / [read](../index.md) / [endian_slice](index.md)*

---

# Module `endian_slice`

Working with byte slices that have an associated endianity.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EndianSlice`](#endianslice) | struct | A `&[u8]` slice with endianity metadata. |
| [`DebugBytes`](#debugbytes) | struct |  |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |

## Structs

### `EndianSlice<'input, Endian>`

```rust
struct EndianSlice<'input, Endian>
where
    Endian: Endianity {
    slice: &'input [u8],
    endian: Endian,
}
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:18-24`](../../../../.source_1765521767/gimli-0.32.3/src/read/endian_slice.rs#L18-L24)*

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- <span id="endianslice-new"></span>`fn new(slice: &'input [u8], endian: Endian) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md#endianslice)

  Construct a new `EndianSlice` with the given slice and endianity.

- <span id="endianslice-slice"></span>`fn slice(&self) -> &'input [u8]`

  Return a reference to the raw slice.

- <span id="endianslice-split-at"></span>`fn split_at(&self, idx: usize) -> (EndianSlice<'input, Endian>, EndianSlice<'input, Endian>)` — [`EndianSlice`](../index.md#endianslice)

  Split the slice in two at the given index, resulting in the tuple where

  the first item has range [0, idx), and the second has range [idx,

  len). Panics if the index is out of bounds.

- <span id="endianslice-find"></span>`fn find(&self, byte: u8) -> Option<usize>`

  Find the first occurrence of a byte in the slice, and return its index.

- <span id="endianslice-offset-from"></span>`fn offset_from(&self, base: EndianSlice<'input, Endian>) -> usize` — [`EndianSlice`](../index.md#endianslice)

  Return the offset of the start of the slice relative to the start

  of the given slice.

- <span id="endianslice-to-string"></span>`fn to_string(&self) -> Result<&'input str>` — [`Result`](../../index.md#result)

  Converts the slice to a string using `str::from_utf8`.

  

  Returns an error if the slice contains invalid characters.

- <span id="endianslice-to-string-lossy"></span>`fn to_string_lossy(&self) -> Cow<'input, str>`

  Converts the slice to a string, including invalid characters,

  using `String::from_utf8_lossy`.

- <span id="endianslice-read-slice"></span>`fn read_slice(&mut self, len: usize) -> Result<&'input [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl Any for EndianSlice<'input, Endian>`

- <span id="endianslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EndianSlice<'input, Endian>`

- <span id="endianslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EndianSlice<'input, Endian>`

- <span id="endianslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Endian> Clone for EndianSlice<'input, Endian>`

- <span id="endianslice-clone"></span>`fn clone(&self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md#endianslice)

##### `impl CloneToUninit for EndianSlice<'input, Endian>`

- <span id="endianslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- <span id="endianslice-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<Endian> Default for EndianSlice<'input, Endian>`

- <span id="endianslice-default"></span>`fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md#endianslice)

##### `impl<Endian> Deref for EndianSlice<'input, Endian>`

- <span id="endianslice-deref-type-target"></span>`type Target = [u8]`

- <span id="endianslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<T> From for EndianSlice<'input, Endian>`

- <span id="endianslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<Endian> Hash for EndianSlice<'input, Endian>`

- <span id="endianslice-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for EndianSlice<'input, Endian>`

- <span id="endianslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Endian> PartialEq for EndianSlice<'input, Endian>`

- <span id="endianslice-partialeq-eq"></span>`fn eq(&self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](../index.md#endianslice)

##### `impl<Endian> Reader for EndianSlice<'input, Endian>`

- <span id="endianslice-reader-type-endian"></span>`type Endian = Endian`

- <span id="endianslice-reader-type-offset"></span>`type Offset = usize`

- <span id="endianslice-reader-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianslice-reader-len"></span>`fn len(&self) -> usize`

- <span id="endianslice-reader-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="endianslice-reader-empty"></span>`fn empty(&mut self)`

- <span id="endianslice-reader-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> usize`

- <span id="endianslice-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md#readeroffsetid)

- <span id="endianslice-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md#readeroffsetid), [`Reader`](../index.md#reader)

- <span id="endianslice-reader-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="endianslice-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md#result)

##### `impl Receiver for EndianSlice<'input, Endian>`

- <span id="endianslice-receiver-type-target"></span>`type Target = T`

##### `impl<Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

##### `impl ToOwned for EndianSlice<'input, Endian>`

- <span id="endianslice-toowned-type-owned"></span>`type Owned = T`

- <span id="endianslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="endianslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EndianSlice<'input, Endian>`

- <span id="endianslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="endianslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EndianSlice<'input, Endian>`

- <span id="endianslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="endianslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:190`](../../../../.source_1765521767/gimli-0.32.3/src/read/endian_slice.rs#L190)*

#### Trait Implementations

##### `impl Any for DebugBytes<'input>`

- <span id="debugbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugBytes<'input>`

- <span id="debugbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugBytes<'input>`

- <span id="debugbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugBytes<'input>`

- <span id="debugbytes-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<T> From for DebugBytes<'input>`

- <span id="debugbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugBytes<'input>`

- <span id="debugbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugBytes<'input>`

- <span id="debugbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugBytes<'input>`

- <span id="debugbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:203`](../../../../.source_1765521767/gimli-0.32.3/src/read/endian_slice.rs#L203)*

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

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:211`](../../../../.source_1765521767/gimli-0.32.3/src/read/endian_slice.rs#L211)*

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

