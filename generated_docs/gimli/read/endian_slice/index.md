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

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- <span id="endianslice-range"></span>`fn range(&self, idx: Range<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

- <span id="endianslice-range-from"></span>`fn range_from(&self, idx: RangeFrom<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

- <span id="endianslice-range-to"></span>`fn range_to(&self, idx: RangeTo<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

#### Trait Implementations

##### `impl<'input, Endian> Clone for EndianSlice<'input, Endian>`

- <span id="endianslice-clone"></span>`fn clone(&self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

##### `impl<'input, Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<'input, Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- <span id="endianslice-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<'input, Endian> Default for EndianSlice<'input, Endian>`

- <span id="endianslice-default"></span>`fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](../index.md)

##### `impl<'input, Endian> Deref for EndianSlice<'input, Endian>`

- <span id="endianslice-target"></span>`type Target = [u8]`

- <span id="endianslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<'input, Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<'input, Endian> Hash for EndianSlice<'input, Endian>`

- <span id="endianslice-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<'input, Endian> PartialEq for EndianSlice<'input, Endian>`

- <span id="endianslice-eq"></span>`fn eq(&self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](../index.md)

##### `impl<'input, Endian> Reader for EndianSlice<'input, Endian>`

- <span id="endianslice-endian"></span>`type Endian = Endian`

- <span id="endianslice-offset"></span>`type Offset = usize`

- <span id="endianslice-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianslice-len"></span>`fn len(&self) -> usize`

- <span id="endianslice-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="endianslice-empty"></span>`fn empty(&mut self)`

- <span id="endianslice-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md)

- <span id="endianslice-offset-from"></span>`fn offset_from(&self, base: &Self) -> usize`

- <span id="endianslice-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md)

- <span id="endianslice-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md), [`Reader`](../index.md)

- <span id="endianslice-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../../index.md)

- <span id="endianslice-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../../index.md)

- <span id="endianslice-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../../index.md)

- <span id="endianslice-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md)

- <span id="endianslice-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md)

- <span id="endianslice-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md)

- <span id="endianslice-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md)

##### `impl<P, T> Receiver for EndianSlice<'input, Endian>`

- <span id="endianslice-target"></span>`type Target = T`

##### `impl<'input, Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

#### Trait Implementations

##### `impl<'input> Debug for DebugBytes<'input>`

- <span id="debugbytes-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- <span id="debuglen-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

