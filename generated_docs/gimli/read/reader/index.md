*[gimli](../../index.md) / [read](../index.md) / [reader](index.md)*

---

# Module `reader`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReaderOffsetId`](#readeroffsetid) | struct | An identifier for an offset within a section reader. |
| [`ReaderOffset`](#readeroffset) | trait | A trait for offsets with a DWARF section. |
| [`ReaderAddress`](#readeraddress) | trait | A trait for addresses within a DWARF section. |
| [`Reader`](#reader) | trait | A trait for reading the data from a DWARF section. |

## Structs

### `ReaderOffsetId`

```rust
struct ReaderOffsetId(u64);
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:19`](../../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L19)*

An identifier for an offset within a section reader.

This is used for error reporting. The meaning of this value is specific to
each reader implementation. The values should be chosen to be unique amongst
all readers. If values are not unique then errors may point to the wrong reader.

#### Trait Implementations

##### `impl Any for ReaderOffsetId`

- <span id="readeroffsetid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReaderOffsetId`

- <span id="readeroffsetid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReaderOffsetId`

- <span id="readeroffsetid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReaderOffsetId`

- <span id="readeroffsetid-clone"></span>`fn clone(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md#readeroffsetid)

##### `impl CloneToUninit for ReaderOffsetId`

- <span id="readeroffsetid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ReaderOffsetId`

##### `impl Debug for ReaderOffsetId`

- <span id="readeroffsetid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ReaderOffsetId`

##### `impl<T> From for ReaderOffsetId`

- <span id="readeroffsetid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReaderOffsetId`

- <span id="readeroffsetid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ReaderOffsetId`

- <span id="readeroffsetid-partialeq-eq"></span>`fn eq(&self, other: &ReaderOffsetId) -> bool` — [`ReaderOffsetId`](../index.md#readeroffsetid)

##### `impl StructuralPartialEq for ReaderOffsetId`

##### `impl ToOwned for ReaderOffsetId`

- <span id="readeroffsetid-toowned-type-owned"></span>`type Owned = T`

- <span id="readeroffsetid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="readeroffsetid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReaderOffsetId`

- <span id="readeroffsetid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readeroffsetid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReaderOffsetId`

- <span id="readeroffsetid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readeroffsetid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ReaderOffset`

```rust
trait ReaderOffset: Debug + Copy + Eq + Ord + Hash + Add<Output = Self> + AddAssign + Sub<Output = Self> { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:24-52`](../../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L24-L52)*

A trait for offsets with a DWARF section.

This allows consumers to choose a size that is appropriate for their address space.

#### Required Methods

- `fn from_u8(offset: u8) -> Self`

  Convert a u8 to an offset.

- `fn from_u16(offset: u16) -> Self`

  Convert a u16 to an offset.

- `fn from_i16(offset: i16) -> Self`

  Convert an i16 to an offset.

- `fn from_u32(offset: u32) -> Self`

  Convert a u32 to an offset.

- `fn from_u64(offset: u64) -> Result<Self>`

  Convert a u64 to an offset.

- `fn into_u64(self) -> u64`

  Convert an offset to a u64.

- `fn wrapping_add(self, other: Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`.

- `fn checked_sub(self, other: Self) -> Option<Self>`

  Checked subtraction. Computes `self - other`.

#### Implementors

- `u32`
- `u64`
- `usize`

### `ReaderAddress`

```rust
trait ReaderAddress: Sized { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:194-230`](../../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L194-L230)*

A trait for addresses within a DWARF section.

Currently this is a simple extension trait for `u64`, but it may be expanded
in the future to support user-defined address types.

#### Required Methods

- `fn add_sized(self, length: u64, size: u8) -> Result<Self>`

  Add a length to an address of the given size.

- `fn wrapping_add_sized(self, length: u64, size: u8) -> Self`

  Add a length to an address of the given size.

- `fn zeros() -> Self`

  The all-zeros value of an address.

- `fn ones_sized(size: u8) -> Self`

  The all-ones value of an address of the given size.

#### Provided Methods

- `fn min_tombstone(size: u8) -> Self`

  Return the minimum value for a tombstone address.

#### Implementors

- `u64`

### `Reader`

```rust
trait Reader: Debug + Clone { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:285-581`](../../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L285-L581)*

A trait for reading the data from a DWARF section.

All read operations advance the section offset of the reader
unless specified otherwise.

## Choosing a `Reader` Implementation

`gimli` comes with a few different `Reader` implementations and lets you
choose the one that is right for your use case. A `Reader` is essentially a
view into the raw bytes that make up some DWARF, but this view might borrow
the underlying data or use reference counting ownership, and it might be
thread safe or not.

| Implementation    | Ownership         | Thread Safe | Notes |
|:------------------|:------------------|:------------|:------|
| [`EndianSlice`](./struct.EndianSlice.html)        | Borrowed          | Yes         | Fastest, but requires that all of your code work with borrows. |
| [`EndianRcSlice`](./struct.EndianRcSlice.html)    | Reference counted | No          | Shared ownership via reference counting, which alleviates the borrow restrictions of `EndianSlice` but imposes reference counting increments and decrements. Cannot be sent across threads, because the reference count is not atomic. |
| [`EndianArcSlice`](./struct.EndianArcSlice.html)  | Reference counted | Yes         | The same as `EndianRcSlice`, but uses atomic reference counting, and therefore reference counting operations are slower but `EndianArcSlice`s may be sent across threads. |
| [`EndianReader<T>`](./struct.EndianReader.html)   | Same as `T`       | Same as `T` | Escape hatch for easily defining your own type of `Reader`. |

#### Associated Types

- `type Endian: 1`

- `type Offset: 1`

#### Required Methods

- `fn endian(&self) -> <Self as >::Endian`

  Return the endianity of bytes that are read.

- `fn len(&self) -> <Self as >::Offset`

  Return the number of bytes remaining.

- `fn empty(&mut self)`

  Set the number of bytes remaining to zero.

- `fn truncate(&mut self, len: <Self as >::Offset) -> Result<()>`

  Set the number of bytes remaining to the specified length.

- `fn offset_from(&self, base: &Self) -> <Self as >::Offset`

  Return the offset of this reader's data relative to the start of

- `fn offset_id(&self) -> ReaderOffsetId`

  Return an identifier for the current reader offset.

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`

  Return the offset corresponding to the given `id` if

- `fn find(&self, byte: u8) -> Result<<Self as >::Offset>`

  Find the index of the first occurrence of the given byte.

- `fn skip(&mut self, len: <Self as >::Offset) -> Result<()>`

  Discard the specified number of bytes.

- `fn split(&mut self, len: <Self as >::Offset) -> Result<Self>`

  Split a reader in two.

- `fn to_slice(&self) -> Result<Cow<'_, [u8]>>`

  Return all remaining data as a clone-on-write slice.

- `fn to_string(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string.

- `fn to_string_lossy(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string, including invalid characters.

- `fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>`

  Read exactly `buf.len()` bytes into `buf`.

#### Provided Methods

- `fn read_u8_array<A>(&mut self) -> Result<A>`

  Read a u8 array.

- `fn is_empty(&self) -> bool`

  Return true if the number of bytes remaining is zero.

- `fn read_u8(&mut self) -> Result<u8>`

  Read a u8.

- `fn read_i8(&mut self) -> Result<i8>`

  Read an i8.

- `fn read_u16(&mut self) -> Result<u16>`

  Read a u16.

- `fn read_i16(&mut self) -> Result<i16>`

  Read an i16.

- `fn read_u32(&mut self) -> Result<u32>`

  Read a u32.

- `fn read_i32(&mut self) -> Result<i32>`

  Read an i32.

- `fn read_u64(&mut self) -> Result<u64>`

  Read a u64.

- `fn read_i64(&mut self) -> Result<i64>`

  Read an i64.

- `fn read_f32(&mut self) -> Result<f32>`

  Read a f32.

- `fn read_f64(&mut self) -> Result<f64>`

  Read a f64.

- `fn read_uint(&mut self, n: usize) -> Result<u64>`

  Read an unsigned n-bytes integer u64.

- `fn read_null_terminated_slice(&mut self) -> Result<Self>`

  Read a null-terminated slice, and return it (excluding the null).

- `fn skip_leb128(&mut self) -> Result<()>`

  Skip a LEB128 encoded integer.

- `fn read_uleb128(&mut self) -> Result<u64>`

  Read an unsigned LEB128 encoded integer.

- `fn read_uleb128_u32(&mut self) -> Result<u32>`

  Read an unsigned LEB128 encoded u32.

- `fn read_uleb128_u16(&mut self) -> Result<u16>`

  Read an unsigned LEB128 encoded u16.

- `fn read_sleb128(&mut self) -> Result<i64>`

  Read a signed LEB128 encoded integer.

- `fn read_initial_length(&mut self) -> Result<(<Self as >::Offset, Format)>`

  Read an initial length field.

- `fn read_address_size(&mut self) -> Result<u8>`

  Read a byte and validate it as an address size.

- `fn read_address(&mut self, address_size: u8) -> Result<u64>`

  Read an address-sized integer, and return it as a `u64`.

- `fn read_word(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized integer according to the DWARF format.

- `fn read_length(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section length according to the DWARF format.

- `fn read_offset(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section offset according to the DWARF format.

- `fn read_sized_offset(&mut self, size: u8) -> Result<<Self as >::Offset>`

  Parse a section offset of the given size.

#### Implementors

- [`EndianSlice`](../index.md#endianslice)
- [`RelocateReader`](../index.md#relocatereader)

