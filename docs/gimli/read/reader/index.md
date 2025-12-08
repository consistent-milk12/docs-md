*[gimli](../../index.md) / [read](../index.md) / [reader](index.md)*

---

# Module `reader`

## Structs

### `ReaderOffsetId`

```rust
struct ReaderOffsetId(u64);
```

An identifier for an offset within a section reader.

This is used for error reporting. The meaning of this value is specific to
each reader implementation. The values should be chosen to be unique amongst
all readers. If values are not unique then errors may point to the wrong reader.

#### Trait Implementations

##### `impl Clone for ReaderOffsetId`

- `fn clone(self: &Self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md)

##### `impl Copy for ReaderOffsetId`

##### `impl Debug for ReaderOffsetId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ReaderOffsetId`

##### `impl PartialEq for ReaderOffsetId`

- `fn eq(self: &Self, other: &ReaderOffsetId) -> bool` — [`ReaderOffsetId`](../index.md)

##### `impl StructuralPartialEq for ReaderOffsetId`

## Traits

### `ReaderOffset`

```rust
trait ReaderOffset: Debug + Copy + Eq + Ord + Hash + Add<Output = Self> + AddAssign + Sub<Output = Self> { ... }
```

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

- `fn into_u64(self: Self) -> u64`

  Convert an offset to a u64.

- `fn wrapping_add(self: Self, other: Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`.

- `fn checked_sub(self: Self, other: Self) -> Option<Self>`

  Checked subtraction. Computes `self - other`.

### `ReaderAddress`

```rust
trait ReaderAddress: Sized { ... }
```

A trait for addresses within a DWARF section.

Currently this is a simple extension trait for `u64`, but it may be expanded
in the future to support user-defined address types.

#### Required Methods

- `fn add_sized(self: Self, length: u64, size: u8) -> Result<Self>`

  Add a length to an address of the given size.

- `fn wrapping_add_sized(self: Self, length: u64, size: u8) -> Self`

  Add a length to an address of the given size.

- `fn zeros() -> Self`

  The all-zeros value of an address.

- `fn ones_sized(size: u8) -> Self`

  The all-ones value of an address of the given size.

- `fn min_tombstone(size: u8) -> Self`

  Return the minimum value for a tombstone address.

### `Reader`

```rust
trait Reader: Debug + Clone { ... }
```

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

#### Required Methods

- `type Endian: 1`

- `type Offset: 1`

- `fn endian(self: &Self) -> <Self as >::Endian`

  Return the endianity of bytes that are read.

- `fn len(self: &Self) -> <Self as >::Offset`

  Return the number of bytes remaining.

- `fn empty(self: &mut Self)`

  Set the number of bytes remaining to zero.

- `fn truncate(self: &mut Self, len: <Self as >::Offset) -> Result<()>`

  Set the number of bytes remaining to the specified length.

- `fn offset_from(self: &Self, base: &Self) -> <Self as >::Offset`

  Return the offset of this reader's data relative to the start of

- `fn offset_id(self: &Self) -> ReaderOffsetId`

  Return an identifier for the current reader offset.

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`

  Return the offset corresponding to the given `id` if

- `fn find(self: &Self, byte: u8) -> Result<<Self as >::Offset>`

  Find the index of the first occurrence of the given byte.

- `fn skip(self: &mut Self, len: <Self as >::Offset) -> Result<()>`

  Discard the specified number of bytes.

- `fn split(self: &mut Self, len: <Self as >::Offset) -> Result<Self>`

  Split a reader in two.

- `fn to_slice(self: &Self) -> Result<Cow<'_, [u8]>>`

  Return all remaining data as a clone-on-write slice.

- `fn to_string(self: &Self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string.

- `fn to_string_lossy(self: &Self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string, including invalid characters.

- `fn read_slice(self: &mut Self, buf: &mut [u8]) -> Result<()>`

  Read exactly `buf.len()` bytes into `buf`.

- `fn read_u8_array<A>(self: &mut Self) -> Result<A>`

  Read a u8 array.

- `fn is_empty(self: &Self) -> bool`

  Return true if the number of bytes remaining is zero.

- `fn read_u8(self: &mut Self) -> Result<u8>`

  Read a u8.

- `fn read_i8(self: &mut Self) -> Result<i8>`

  Read an i8.

- `fn read_u16(self: &mut Self) -> Result<u16>`

  Read a u16.

- `fn read_i16(self: &mut Self) -> Result<i16>`

  Read an i16.

- `fn read_u32(self: &mut Self) -> Result<u32>`

  Read a u32.

- `fn read_i32(self: &mut Self) -> Result<i32>`

  Read an i32.

- `fn read_u64(self: &mut Self) -> Result<u64>`

  Read a u64.

- `fn read_i64(self: &mut Self) -> Result<i64>`

  Read an i64.

- `fn read_f32(self: &mut Self) -> Result<f32>`

  Read a f32.

- `fn read_f64(self: &mut Self) -> Result<f64>`

  Read a f64.

- `fn read_uint(self: &mut Self, n: usize) -> Result<u64>`

  Read an unsigned n-bytes integer u64.

- `fn read_null_terminated_slice(self: &mut Self) -> Result<Self>`

  Read a null-terminated slice, and return it (excluding the null).

- `fn skip_leb128(self: &mut Self) -> Result<()>`

  Skip a LEB128 encoded integer.

- `fn read_uleb128(self: &mut Self) -> Result<u64>`

  Read an unsigned LEB128 encoded integer.

- `fn read_uleb128_u32(self: &mut Self) -> Result<u32>`

  Read an unsigned LEB128 encoded u32.

- `fn read_uleb128_u16(self: &mut Self) -> Result<u16>`

  Read an unsigned LEB128 encoded u16.

- `fn read_sleb128(self: &mut Self) -> Result<i64>`

  Read a signed LEB128 encoded integer.

- `fn read_initial_length(self: &mut Self) -> Result<(<Self as >::Offset, Format)>`

  Read an initial length field.

- `fn read_address_size(self: &mut Self) -> Result<u8>`

  Read a byte and validate it as an address size.

- `fn read_address(self: &mut Self, address_size: u8) -> Result<u64>`

  Read an address-sized integer, and return it as a `u64`.

- `fn read_word(self: &mut Self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized integer according to the DWARF format.

- `fn read_length(self: &mut Self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section length according to the DWARF format.

- `fn read_offset(self: &mut Self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section offset according to the DWARF format.

- `fn read_sized_offset(self: &mut Self, size: u8) -> Result<<Self as >::Offset>`

  Parse a section offset of the given size.

