*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [segment](index.md)*

---

# Module `segment`

## Structs

### `ElfSegmentIterator<'data, 'file, Elf, R>`

```rust
struct ElfSegmentIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: slice::Iter<'data, <Elf as >::ProgramHeader>,
}
```

An iterator for the segments in an [`ElfFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSegmentIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- `type Item = ElfSegment<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSegment<'data, 'file, Elf, R>`

```rust
struct ElfSegment<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    segment: &'data <Elf as >::ProgramHeader,
}
```

A segment in an [`ElfFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md)

- `fn elf_program_header(self: &Self) -> &'data <Elf as >::ProgramHeader` — [`FileHeader`](../index.md)

- `fn bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSegment<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSegment for ElfSegment<'data, 'file, Elf, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> read::Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSegment<'data, 'file, Elf, R>`

## Traits

### `ProgramHeader`

```rust
trait ProgramHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::ProgramHeader32`](../../../elf/index.md) and [`elf::ProgramHeader64`](../../../elf/index.md).

#### Required Methods

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

- `fn p_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn p_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_vaddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_paddr(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_filesz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_memsz(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_align(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn file_range(self: &Self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> Result<&'data [u8], ()>`

  Return the segment data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> Result<&'data [T], ()>`

  Return the segment data as a slice of the given type.

- `fn data_range<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, address: u64, size: u64) -> Result<Option<&'data [u8]>, ()>`

  Return the segment data in the given virtual address range

- `fn dynamic<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [<<Self as >::Elf as FileHeader>::Dyn]>>`

  Return entries in a dynamic segment.

- `fn interpreter<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [u8]>>`

  Return the data in an interpreter segment.

- `fn notes<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the segment data.

## Type Aliases

### `ElfSegmentIterator32<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator32<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the segments in an [`ElfFile32`](super::ElfFile32).

### `ElfSegmentIterator64<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator64<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the segments in an [`ElfFile64`](super::ElfFile64).

### `ElfSegment32<'data, 'file, Endian, R>`

```rust
type ElfSegment32<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A segment in an [`ElfFile32`](super::ElfFile32).

### `ElfSegment64<'data, 'file, Endian, R>`

```rust
type ElfSegment64<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A segment in an [`ElfFile64`](super::ElfFile64).

