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

*Defined in [`gimli-0.32.3/src/read/reader.rs:19`](../../../../.source_1765900590/gimli-0.32.3/src/read/reader.rs#L19)*

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

*Defined in [`gimli-0.32.3/src/read/reader.rs:24-52`](../../../../.source_1765900590/gimli-0.32.3/src/read/reader.rs#L24-L52)*

A trait for offsets with a DWARF section.

This allows consumers to choose a size that is appropriate for their address space.

#### Required Methods

- `fn ReaderOffset::from_u8(offset: u8) -> Self`

  Convert a u8 to an offset.

- `fn ReaderOffset::from_u16(offset: u16) -> Self`

  Convert a u16 to an offset.

- `fn ReaderOffset::from_i16(offset: i16) -> Self`

  Convert an i16 to an offset.

- `fn ReaderOffset::from_u32(offset: u32) -> Self`

  Convert a u32 to an offset.

- `fn ReaderOffset::from_u64(offset: u64) -> Result<Self>`

  Convert a u64 to an offset.
  
  Returns `Error::UnsupportedOffset` if the value is too large.

- `fn ReaderOffset::into_u64(self) -> u64`

  Convert an offset to a u64.

- `fn ReaderOffset::wrapping_add(self, other: Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`.

- `fn ReaderOffset::checked_sub(self, other: Self) -> Option<Self>`

  Checked subtraction. Computes `self - other`.

#### Implementors

- `u32`
- `u64`
- `usize`

### `ReaderAddress`

```rust
trait ReaderAddress: Sized { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:194-230`](../../../../.source_1765900590/gimli-0.32.3/src/read/reader.rs#L194-L230)*

A trait for addresses within a DWARF section.

Currently this is a simple extension trait for `u64`, but it may be expanded
in the future to support user-defined address types.

#### Required Methods

- `fn ReaderAddress::add_sized(self, length: u64, size: u8) -> Result<Self>`

  Add a length to an address of the given size.
  
  Returns an error for overflow.

- `fn ReaderAddress::wrapping_add_sized(self, length: u64, size: u8) -> Self`

  Add a length to an address of the given size.
  
  Wraps the result to the size of the address to allow for the possibility
  that the length is a negative value.

- `fn ReaderAddress::zeros() -> Self`

  The all-zeros value of an address.

- `fn ReaderAddress::ones_sized(size: u8) -> Self`

  The all-ones value of an address of the given size.

#### Provided Methods

- `fn ReaderAddress::min_tombstone(size: u8) -> Self`

  Return the minimum value for a tombstone address.
  
  A variety of values may be used as tombstones in DWARF data.  DWARF 6 specifies a
  tombstone value of -1, and this is compatible with most sections in earlier DWARF
  versions. However, for .debug_loc and .debug_ranges in DWARF 4 and earlier, the
  tombstone value is -2, because -1 already has a special meaning. -2 has also been
  seen in .debug_line, possibly from a proprietary fork of lld.
  
  So this function returns -2 (cast to an unsigned value), and callers can consider
  addresses greater than or equal to this value to be tombstones.
  
  Prior to the use of -1 or -2 for tombstones, it was common to use 0 or 1.
  Additionally, gold may leave the relocation addend in place. These values are not
  handled by this function, so callers will need to handle them separately if they
  want to.

#### Implementors

- `u64`

### `Reader`

```rust
trait Reader: Debug + Clone { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:285-581`](../../../../.source_1765900590/gimli-0.32.3/src/read/reader.rs#L285-L581)*

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

<details>
<summary><strong>Methods (40)</strong> - click to expand</summary>

**Required:**
- [`Reader::endian`](#fn-readerendian)
- [`Reader::len`](#fn-readerlen)
- [`Reader::empty`](#fn-readerempty)
- [`Reader::truncate`](#fn-readertruncate)
- [`Reader::offset_from`](#fn-readeroffset-from)
- [`Reader::offset_id`](#fn-readeroffset-id)
- [`Reader::lookup_offset_id`](#fn-readerlookup-offset-id)
- [`Reader::find`](#fn-readerfind)
- [`Reader::skip`](#fn-readerskip)
- [`Reader::split`](#fn-readersplit)
- [`Reader::to_slice`](#fn-readerto-slice)
- [`Reader::to_string`](#fn-readerto-string)
- [`Reader::to_string_lossy`](#fn-readerto-string-lossy)
- [`Reader::read_slice`](#fn-readerread-slice)

**Provided:**
- [`Reader::read_u8_array`](#fn-readerread-u8-array)
- [`Reader::is_empty`](#fn-readeris-empty)
- [`Reader::read_u8`](#fn-readerread-u8)
- [`Reader::read_i8`](#fn-readerread-i8)
- [`Reader::read_u16`](#fn-readerread-u16)
- [`Reader::read_i16`](#fn-readerread-i16)
- [`Reader::read_u32`](#fn-readerread-u32)
- [`Reader::read_i32`](#fn-readerread-i32)
- [`Reader::read_u64`](#fn-readerread-u64)
- [`Reader::read_i64`](#fn-readerread-i64)
- [`Reader::read_f32`](#fn-readerread-f32)
- [`Reader::read_f64`](#fn-readerread-f64)
- [`Reader::read_uint`](#fn-readerread-uint)
- [`Reader::read_null_terminated_slice`](#fn-readerread-null-terminated-slice)
- [`Reader::skip_leb128`](#fn-readerskip-leb128)
- [`Reader::read_uleb128`](#fn-readerread-uleb128)
- [`Reader::read_uleb128_u32`](#fn-readerread-uleb128-u32)
- [`Reader::read_uleb128_u16`](#fn-readerread-uleb128-u16)
- [`Reader::read_sleb128`](#fn-readerread-sleb128)
- [`Reader::read_initial_length`](#fn-readerread-initial-length)
- [`Reader::read_address_size`](#fn-readerread-address-size)
- [`Reader::read_address`](#fn-readerread-address)
- [`Reader::read_word`](#fn-readerread-word)
- [`Reader::read_length`](#fn-readerread-length)
- [`Reader::read_offset`](#fn-readerread-offset)
- [`Reader::read_sized_offset`](#fn-readerread-sized-offset)

</details>

#### Associated Types

- `type Endian: 1`

- `type Offset: 1`

#### Required Methods

- `fn Reader::endian(&self) -> <Self as >::Endian`

  Return the endianity of bytes that are read.

- `fn Reader::len(&self) -> <Self as >::Offset`

  Return the number of bytes remaining.

- `fn Reader::empty(&mut self)`

  Set the number of bytes remaining to zero.

- `fn Reader::truncate(&mut self, len: <Self as >::Offset) -> Result<()>`

  Set the number of bytes remaining to the specified length.

- `fn Reader::offset_from(&self, base: &Self) -> <Self as >::Offset`

  Return the offset of this reader's data relative to the start of
  the given base reader's data.
  
  May panic if this reader's data is not contained within the given
  base reader's data.

- `fn Reader::offset_id(&self) -> ReaderOffsetId`

  Return an identifier for the current reader offset.

- `fn Reader::lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`

  Return the offset corresponding to the given `id` if
  it is associated with this reader.

- `fn Reader::find(&self, byte: u8) -> Result<<Self as >::Offset>`

  Find the index of the first occurrence of the given byte.
  The offset of the reader is not changed.

- `fn Reader::skip(&mut self, len: <Self as >::Offset) -> Result<()>`

  Discard the specified number of bytes.

- `fn Reader::split(&mut self, len: <Self as >::Offset) -> Result<Self>`

  Split a reader in two.
  
  A new reader is returned that can be used to read the next
  `len` bytes, and `self` is advanced so that it reads the remainder.

- `fn Reader::to_slice(&self) -> Result<Cow<'_, [u8]>>`

  Return all remaining data as a clone-on-write slice.
  
  The slice will be borrowed where possible, but some readers may
  always return an owned vector.
  
  Does not advance the reader.

- `fn Reader::to_string(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string.
  
  The string will be borrowed where possible, but some readers may
  always return an owned string.
  
  Does not advance the reader.
  
  Returns an error if the data contains invalid characters.

- `fn Reader::to_string_lossy(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string, including invalid characters.
  
  The string will be borrowed where possible, but some readers may
  always return an owned string.
  
  Does not advance the reader.

- `fn Reader::read_slice(&mut self, buf: &mut [u8]) -> Result<()>`

  Read exactly `buf.len()` bytes into `buf`.

#### Provided Methods

- `fn Reader::read_u8_array<A>(&mut self) -> Result<A>`

  Read a u8 array.

- `fn Reader::is_empty(&self) -> bool`

  Return true if the number of bytes remaining is zero.

- `fn Reader::read_u8(&mut self) -> Result<u8>`

  Read a u8.

- `fn Reader::read_i8(&mut self) -> Result<i8>`

  Read an i8.

- `fn Reader::read_u16(&mut self) -> Result<u16>`

  Read a u16.

- `fn Reader::read_i16(&mut self) -> Result<i16>`

  Read an i16.

- `fn Reader::read_u32(&mut self) -> Result<u32>`

  Read a u32.

- `fn Reader::read_i32(&mut self) -> Result<i32>`

  Read an i32.

- `fn Reader::read_u64(&mut self) -> Result<u64>`

  Read a u64.

- `fn Reader::read_i64(&mut self) -> Result<i64>`

  Read an i64.

- `fn Reader::read_f32(&mut self) -> Result<f32>`

  Read a f32.

- `fn Reader::read_f64(&mut self) -> Result<f64>`

  Read a f64.

- `fn Reader::read_uint(&mut self, n: usize) -> Result<u64>`

  Read an unsigned n-bytes integer u64.
  
  ##### Panics
  
  Panics when nbytes < 1 or nbytes > 8

- `fn Reader::read_null_terminated_slice(&mut self) -> Result<Self>`

  Read a null-terminated slice, and return it (excluding the null).

- `fn Reader::skip_leb128(&mut self) -> Result<()>`

  Skip a LEB128 encoded integer.

- `fn Reader::read_uleb128(&mut self) -> Result<u64>`

  Read an unsigned LEB128 encoded integer.

- `fn Reader::read_uleb128_u32(&mut self) -> Result<u32>`

  Read an unsigned LEB128 encoded u32.

- `fn Reader::read_uleb128_u16(&mut self) -> Result<u16>`

  Read an unsigned LEB128 encoded u16.

- `fn Reader::read_sleb128(&mut self) -> Result<i64>`

  Read a signed LEB128 encoded integer.

- `fn Reader::read_initial_length(&mut self) -> Result<(<Self as >::Offset, Format)>`

  Read an initial length field.
  
  This field is encoded as either a 32-bit length or
  a 64-bit length, and the returned `Format` indicates which.

- `fn Reader::read_address_size(&mut self) -> Result<u8>`

  Read a byte and validate it as an address size.

- `fn Reader::read_address(&mut self, address_size: u8) -> Result<u64>`

  Read an address-sized integer, and return it as a `u64`.

- `fn Reader::read_word(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized integer according to the DWARF format.
  
  These are always used to encode section offsets or lengths,
  and so have a type of `Self::Offset`.

- `fn Reader::read_length(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section length according to the DWARF format.

- `fn Reader::read_offset(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section offset according to the DWARF format.

- `fn Reader::read_sized_offset(&mut self, size: u8) -> Result<<Self as >::Offset>`

  Parse a section offset of the given size.
  
  This is used for `DW_FORM_ref_addr` values in DWARF version 2.

#### Implementors

- [`EndianSlice`](../index.md#endianslice)
- [`RelocateReader`](../index.md#relocatereader)

