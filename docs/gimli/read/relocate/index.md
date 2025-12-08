*[gimli](../../index.md) / [read](../index.md) / [relocate](index.md)*

---

# Module `relocate`

## Structs

### `RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>>`

```rust
struct RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>> {
    section: R,
    reader: R,
    relocate: T,
}
```

A `Reader` which applies relocations to addresses and offsets.

This is useful for reading sections which contain relocations,
such as those in a relocatable object file.
It is generally not used for reading sections in an executable file.

#### Implementations

- `fn new(section: R, relocate: T) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader<Offset = usize>, T: $crate::clone::Clone + Relocate<<R as >::Offset>> Clone for RelocateReader<R, T>`

- `fn clone(self: &Self) -> RelocateReader<R, T>` — [`RelocateReader`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader<Offset = usize>, T: $crate::fmt::Debug + Relocate<<R as >::Offset>> Debug for RelocateReader<R, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, T> Reader for RelocateReader<R, T>`

- `type Endian = <R as Reader>::Endian`

- `type Offset = <R as Reader>::Offset`

- `fn read_address(self: &mut Self, address_size: u8) -> Result<u64>` — [`Result`](../../index.md)

- `fn read_offset(self: &mut Self, format: Format) -> Result<<R as >::Offset>` — [`Format`](../../index.md), [`Result`](../../index.md), [`Reader`](../index.md)

- `fn read_sized_offset(self: &mut Self, size: u8) -> Result<<R as >::Offset>` — [`Result`](../../index.md), [`Reader`](../index.md)

- `fn split(self: &mut Self, len: <Self as >::Offset) -> Result<Self>` — [`Reader`](../index.md), [`Result`](../../index.md)

- `fn endian(self: &Self) -> <Self as >::Endian` — [`Reader`](../index.md)

- `fn len(self: &Self) -> <Self as >::Offset` — [`Reader`](../index.md)

- `fn empty(self: &mut Self)`

- `fn truncate(self: &mut Self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](../index.md), [`Result`](../../index.md)

- `fn offset_from(self: &Self, base: &Self) -> <Self as >::Offset` — [`Reader`](../index.md)

- `fn offset_id(self: &Self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md), [`Reader`](../index.md)

- `fn find(self: &Self, byte: u8) -> Result<<Self as >::Offset>` — [`Result`](../../index.md), [`Reader`](../index.md)

- `fn skip(self: &mut Self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](../index.md), [`Result`](../../index.md)

- `fn to_slice(self: &Self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md)

- `fn to_string(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md)

- `fn to_string_lossy(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md)

- `fn read_slice(self: &mut Self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md)

## Traits

### `Relocate<T: ReaderOffset>`

```rust
trait Relocate<T: ReaderOffset> { ... }
```

Trait for relocating addresses and offsets while reading a section.

#### Required Methods

- `fn relocate_address(self: &Self, offset: T, value: u64) -> Result<u64>`

  Relocate an address which was read from the given section offset.

- `fn relocate_offset(self: &Self, offset: T, value: T) -> Result<T>`

  Relocate a value which was read from the given section offset.

