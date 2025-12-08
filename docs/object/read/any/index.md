*[object](../../index.md) / [read](../index.md) / [any](index.md)*

---

# Module `any`

## Structs

### `SegmentIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SegmentIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentIteratorInternal<'data, 'file, R>,
}
```

An iterator for the loadable segments in a [`File`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SegmentIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SegmentIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SegmentIterator<'data, 'file, R>`

- `type Item = Segment<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Segment<'data, 'file, R: ReadRef<'data>>`

```rust
struct Segment<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentInternal<'data, 'file, R>,
}
```

A loadable segment in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Segment<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSegment for Segment<'data, 'file, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Segment<'data, 'file, R>`

### `SectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionIteratorInternal<'data, 'file, R>,
}
```

An iterator for the sections in a [`File`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SectionIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SectionIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SectionIterator<'data, 'file, R>`

- `type Item = Section<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Section<'data, 'file, R: ReadRef<'data>>`

```rust
struct Section<'data, 'file, R: ReadRef<'data>> {
    inner: SectionInternal<'data, 'file, R>,
}
```

A section in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Section<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSection for Section<'data, 'file, R>`

- `type RelocationIterator = SectionRelocationIterator<'data, 'file, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../index.md)

- `fn relocations(self: &Self) -> SectionRelocationIterator<'data, 'file, R>` — [`SectionRelocationIterator`](../index.md)

- `fn relocation_map(self: &Self) -> Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Section<'data, 'file, R>`

### `ComdatIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatIteratorInternal<'data, 'file, R>,
}
```

An iterator for the COMDAT section groups in a [`File`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ComdatIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ComdatIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for ComdatIterator<'data, 'file, R>`

- `type Item = Comdat<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Comdat<'data, 'file, R: ReadRef<'data>>`

```rust
struct Comdat<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatInternal<'data, 'file, R>,
}
```

A COMDAT section group in a [`File`](../index.md).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Comdat<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectComdat for Comdat<'data, 'file, R>`

- `type SectionIterator = ComdatSectionIterator<'data, 'file, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn sections(self: &Self) -> ComdatSectionIterator<'data, 'file, R>` — [`ComdatSectionIterator`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Comdat<'data, 'file, R>`

### `ComdatSectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatSectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatSectionIteratorInternal<'data, 'file, R>,
}
```

An iterator for the sections in a [`Comdat`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ComdatSectionIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for ComdatSectionIterator<'data, 'file, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `SymbolTable<'data, 'file, R>`

```rust
struct SymbolTable<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolTableInternal<'data, 'file, R>,
}
```

A symbol table in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSymbolTable`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolTable<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSymbolTable for SymbolTable<'data, 'file, R>`

- `type Symbol = Symbol<'data, 'file, R>`

- `type SymbolIterator = SymbolIterator<'data, 'file, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for SymbolTable<'data, 'file, R>`

### `SymbolIterator<'data, 'file, R>`

```rust
struct SymbolIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolIteratorInternal<'data, 'file, R>,
}
```

An iterator for the symbols in a [`SymbolTable`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'file, R>`

- `type Item = Symbol<'data, 'file, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Symbol<'data, 'file, R>`

```rust
struct Symbol<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolInternal<'data, 'file, R>,
}
```

An symbol in a [`SymbolTable`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>> Debug for Symbol<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>> ObjectSymbol for Symbol<'data, 'file, R>`

- `fn index(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn kind(self: &Self) -> SymbolKind` — [`SymbolKind`](../../index.md)

- `fn section(self: &Self) -> SymbolSection` — [`SymbolSection`](../../index.md)

- `fn is_undefined(self: &Self) -> bool`

- `fn is_definition(self: &Self) -> bool`

- `fn is_common(self: &Self) -> bool`

- `fn is_weak(self: &Self) -> bool`

- `fn scope(self: &Self) -> SymbolScope` — [`SymbolScope`](../../index.md)

- `fn is_global(self: &Self) -> bool`

- `fn is_local(self: &Self) -> bool`

- `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md), [`SectionIndex`](../../index.md), [`SymbolIndex`](../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>> Sealed for Symbol<'data, 'file, R>`

### `DynamicRelocationIterator<'data, 'file, R>`

```rust
struct DynamicRelocationIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: DynamicRelocationIteratorInternal<'data, 'file, R>,
}
```

An iterator for the dynamic relocation entries in a [`File`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for DynamicRelocationIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for DynamicRelocationIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for DynamicRelocationIterator<'data, 'file, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `SectionRelocationIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionRelocationIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionRelocationIteratorInternal<'data, 'file, R>,
}
```

An iterator for the relocation entries in a [`Section`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SectionRelocationIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>> Iterator for SectionRelocationIterator<'data, 'file, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Enums

### `File<'data, R: ReadRef<'data>>`

```rust
enum File<'data, R: ReadRef<'data>> {
    Coff(coff::CoffFile<'data, R>),
    CoffBig(coff::CoffBigFile<'data, R>),
    Elf32(elf::ElfFile32<'data, crate::endian::Endianness, R>),
    Elf64(elf::ElfFile64<'data, crate::endian::Endianness, R>),
    MachO32(macho::MachOFile32<'data, crate::endian::Endianness, R>),
    MachO64(macho::MachOFile64<'data, crate::endian::Endianness, R>),
    Pe32(pe::PeFile32<'data, R>),
    Pe64(pe::PeFile64<'data, R>),
    Xcoff32(xcoff::XcoffFile32<'data, R>),
    Xcoff64(xcoff::XcoffFile64<'data, R>),
}
```

An object file that can be any supported file format.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn parse_dyld_cache_image<'cache, E: crate::Endian>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](../macho/index.md), [`Result`](../../index.md)

- `fn format(self: &Self) -> BinaryFormat` — [`BinaryFormat`](../../index.md)

#### Trait Implementations

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>> Debug for File<'data, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R> Object for File<'data, R>`

- `type Segment = Segment<'data, 'file, R>`

- `type SegmentIterator = SegmentIterator<'data, 'file, R>`

- `type Section = Section<'data, 'file, R>`

- `type SectionIterator = SectionIterator<'data, 'file, R>`

- `type Comdat = Comdat<'data, 'file, R>`

- `type ComdatIterator = ComdatIterator<'data, 'file, R>`

- `type Symbol = Symbol<'data, 'file, R>`

- `type SymbolIterator = SymbolIterator<'data, 'file, R>`

- `type SymbolTable = SymbolTable<'data, 'file, R>`

- `type DynamicRelocationIterator = DynamicRelocationIterator<'data, 'file, R>`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- `fn segments(self: &Self) -> SegmentIterator<'data, '_, R>` — [`SegmentIterator`](../index.md)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>` — [`Section`](../index.md)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<Section<'data, '_, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`Section`](../index.md)

- `fn sections(self: &Self) -> SectionIterator<'data, '_, R>` — [`SectionIterator`](../index.md)

- `fn comdats(self: &Self) -> ComdatIterator<'data, '_, R>` — [`ComdatIterator`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<Symbol<'data, '_, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`Symbol`](../index.md)

- `fn symbols(self: &Self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](../index.md)

- `fn symbol_table(self: &Self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](../index.md)

- `fn dynamic_symbols(self: &Self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](../index.md)

- `fn dynamic_symbol_table(self: &Self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](../index.md)

- `fn dynamic_relocations(self: &Self) -> Option<DynamicRelocationIterator<'data, '_, R>>` — [`DynamicRelocationIterator`](../index.md)

- `fn symbol_map(self: &Self) -> SymbolMap<SymbolMapName<'data>>` — [`SymbolMap`](../../index.md), [`SymbolMapName`](../../index.md)

- `fn object_map(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](../../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>` — [`Result`](../../index.md)

- `fn build_id(self: &Self) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn gnu_debuglink(self: &Self) -> Result<Option<(&'data [u8], u32)>>` — [`Result`](../../index.md)

- `fn gnu_debugaltlink(self: &Self) -> Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../index.md)

- `fn pdb_info(self: &Self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../index.md), [`CodeView`](../../index.md)

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, R: ReadRef<'data>> Sealed for File<'data, R>`

### `SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegmentIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSegmentIterator<'data, 'file, R>),
    Elf32(elf::ElfSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegmentIterator32<'data, 'file, R>),
    Pe64(pe::PeSegmentIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegmentIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegmentIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SegmentIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SegmentInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegment<'data, 'file, R>),
    CoffBig(coff::CoffBigSegment<'data, 'file, R>),
    Elf32(elf::ElfSegment32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegment64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegment32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegment64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegment32<'data, 'file, R>),
    Pe64(pe::PeSegment64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegment32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegment64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SegmentInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeSectionIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSectionIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SectionIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SectionInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSection<'data, 'file, R>),
    CoffBig(coff::CoffBigSection<'data, 'file, R>),
    Elf32(elf::ElfSection32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSection64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSection32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSection64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSection32<'data, 'file, R>),
    Pe64(pe::PeSection64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSection32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSection64<'data, 'file, R>),
}
```

### `ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ComdatIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ComdatInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdat<'data, 'file, R>),
    CoffBig(coff::CoffBigComdat<'data, 'file, R>),
    Elf32(elf::ElfComdat32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdat64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdat32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdat64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdat32<'data, 'file, R>),
    Pe64(pe::PeComdat64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdat32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdat64<'data, 'file, R>),
}
```

### `ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatSectionIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatSectionIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SymbolTableInternal<'data, 'file, R>`

```rust
enum SymbolTableInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolTable32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolTable64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolTableInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SymbolIteratorInternal<'data, 'file, R>`

```rust
enum SymbolIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolIterator32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolIterator64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for SymbolIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SymbolInternal<'data, 'file, R>`

```rust
enum SymbolInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbol32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbol64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

### `DynamicRelocationIteratorInternal<'data, 'file, R>`

```rust
enum DynamicRelocationIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Elf32(elf::ElfDynamicRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfDynamicRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    None(core::marker::PhantomData<(&'data (), &'file (), R)>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R> Debug for DynamicRelocationIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffRelocationIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigRelocationIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachORelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachORelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeRelocationIterator<'data, 'file, R>),
    Pe64(pe::PeRelocationIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffRelocationIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffRelocationIterator64<'data, 'file, R>),
}
```

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIteratorInternal<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Macros

### `with_inner!`

Evaluate an expression on the contents of a file format enum.

This is a hack to avoid virtual calls.

### `with_inner_mut!`

### `map_inner!`

Like `with_inner!`, but wraps the result in another enum.

### `map_inner_option!`

Like `map_inner!`, but the result is a Result or Option.

### `map_inner_option_mut!`

### `next_inner!`

Call `next` for a file format iterator.

