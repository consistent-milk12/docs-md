*[gimli](../../index.md) / [read](../index.md) / [endian_slice](index.md)*

---

# Module `endian_slice`

Working with byte slices that have an associated endianity.

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

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- `fn range(self: &Self, idx: Range<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

- `fn range_from(self: &Self, idx: RangeFrom<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

- `fn range_to(self: &Self, idx: RangeTo<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

#### Trait Implementations

##### `impl<'input, Endian> Clone for EndianSlice<'input, Endian>`

- `fn clone(self: &Self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

##### `impl<'input, Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<'input, Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<'input, Endian> Default for EndianSlice<'input, Endian>`

- `fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

##### `impl<'input, Endian> Deref for EndianSlice<'input, Endian>`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<'input, Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<'input, Endian> Hash for EndianSlice<'input, Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'input, Endian> PartialEq for EndianSlice<'input, Endian>`

- `fn eq(self: &Self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](../index.md)

##### `impl<'input, Endian> Reader for EndianSlice<'input, Endian>`

- `type Endian = Endian`

- `type Offset = usize`

- `fn endian(self: &Self) -> Endian`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn empty(self: &mut Self)`

- `fn truncate(self: &mut Self, len: usize) -> Result<()>` — [`Result`](../../index.md)

- `fn offset_from(self: &Self, base: &Self) -> usize`

- `fn offset_id(self: &Self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md), [`Reader`](../index.md)

- `fn find(self: &Self, byte: u8) -> Result<usize>` — [`Result`](../../index.md)

- `fn skip(self: &mut Self, len: usize) -> Result<()>` — [`Result`](../../index.md)

- `fn split(self: &mut Self, len: usize) -> Result<Self>` — [`Result`](../../index.md)

- `fn to_slice(self: &Self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md)

- `fn to_string(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md)

- `fn to_string_lossy(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md)

- `fn read_slice(self: &mut Self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md)

##### `impl<P, T> Receiver for EndianSlice<'input, Endian>`

- `type Target = T`

##### `impl<'input, Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

#### Trait Implementations

##### `impl<'input> Debug for DebugBytes<'input>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

