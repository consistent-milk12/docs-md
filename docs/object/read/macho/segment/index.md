*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [segment](index.md)*

---

# Module `segment`

## Structs

### `MachOSegmentIterator<'data, 'file, Mach, R>`

```rust
struct MachOSegmentIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    iter: slice::Iter<'file, MachOSegmentInternal<'data, Mach, R>>,
}
```

An iterator for the segments in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSegmentIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- `type Item = MachOSegment<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `MachOSegment<'data, 'file, Mach, R>`

```rust
struct MachOSegment<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    internal: &'file MachOSegmentInternal<'data, Mach, R>,
}
```

A segment in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md)

- `fn macho_segment(self: &Self) -> &'data <Mach as >::Segment` — [`MachHeader`](../index.md)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSegment<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSegment for MachOSegment<'data, 'file, Mach, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSegment<'data, 'file, Mach, R>`

### `MachOSegmentInternal<'data, Mach: MachHeader, R: ReadRef<'data>>`

```rust
struct MachOSegmentInternal<'data, Mach: MachHeader, R: ReadRef<'data>> {
    pub segment: &'data <Mach as >::Segment,
    pub data: R,
}
```

#### Fields

- **`data`**: `R`

  The data for the file that contains the segment data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Trait Implementations

##### `impl<'data, Mach: $crate::clone::Clone + MachHeader, R: $crate::clone::Clone + ReadRef<'data>> Clone for MachOSegmentInternal<'data, Mach, R>`

- `fn clone(self: &Self) -> MachOSegmentInternal<'data, Mach, R>` — [`MachOSegmentInternal`](#machosegmentinternal)

##### `impl<'data, Mach: $crate::marker::Copy + MachHeader, R: $crate::marker::Copy + ReadRef<'data>> Copy for MachOSegmentInternal<'data, Mach, R>`

##### `impl<'data, Mach: $crate::fmt::Debug + MachHeader, R: $crate::fmt::Debug + ReadRef<'data>> Debug for MachOSegmentInternal<'data, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Segment`

```rust
trait Segment: Debug + Pod { ... }
```

A trait for generic access to [`macho::SegmentCommand32`](../../../macho/index.md) and [`macho::SegmentCommand64`](../../../macho/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `type Section: 1`

- `fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>`

- `fn cmd(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn cmdsize(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn segname(self: &Self) -> &[u8; 16]`

- `fn vmaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn vmsize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn fileoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn filesize(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn maxprot(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn initprot(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn nsects(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn name(self: &Self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Get the segment data from the file data.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, section_data: R) -> Result<&'data [<Self as >::Section]>`

  Get the array of sections from the data following the segment command.

## Type Aliases

### `MachOSegmentIterator32<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator32<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the segments in a [`MachOFile32`](super::MachOFile32).

### `MachOSegmentIterator64<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator64<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the segments in a [`MachOFile64`](super::MachOFile64).

### `MachOSegment32<'data, 'file, Endian, R>`

```rust
type MachOSegment32<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A segment in a [`MachOFile32`](super::MachOFile32).

### `MachOSegment64<'data, 'file, Endian, R>`

```rust
type MachOSegment64<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A segment in a [`MachOFile64`](super::MachOFile64).

