*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [file](index.md)*

---

# Module `file`

## Structs

### `MachOFile<'data, Mach, R>`

```rust
struct MachOFile<'data, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    endian: <Mach as >::Endian,
    data: R,
    header_offset: u64,
    header: &'data Mach,
    segments: alloc::vec::Vec<super::MachOSegmentInternal<'data, Mach, R>>,
    sections: alloc::vec::Vec<super::MachOSectionInternal<'data, Mach, R>>,
    symbols: super::SymbolTable<'data, Mach, R>,
}
```

A partially parsed Mach-O file.

Most of the functionality of this type is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn parse_dyld_cache_image<'cache, E: Endian>(image: &DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](../index.md), [`Result`](../../../index.md)

- `fn section_internal(self: &Self, index: SectionIndex) -> Result<&MachOSectionInternal<'data, Mach, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`MachOSectionInternal`](../section/index.md)

- `fn endian(self: &Self) -> <Mach as >::Endian` — [`MachHeader`](../index.md)

- `fn data(self: &Self) -> R`

- `fn raw_header(self: &Self) -> &'data Mach`

- `fn macho_header(self: &Self) -> &'data Mach`

- `fn macho_load_commands(self: &Self) -> Result<LoadCommandIterator<'data, <Mach as >::Endian>>` — [`Result`](../../../index.md), [`LoadCommandIterator`](../index.md), [`MachHeader`](../index.md)

- `fn macho_symbol_table(self: &Self) -> &SymbolTable<'data, Mach, R>` — [`SymbolTable`](../index.md)

- `fn build_version(self: &Self) -> Result<Option<&'data macho::BuildVersionCommand<<Mach as >::Endian>>>` — [`Result`](../../../index.md), [`BuildVersionCommand`](../../../macho/index.md), [`MachHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Mach, R> Debug for MachOFile<'data, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Mach, R> Object for MachOFile<'data, Mach, R>`

- `type Segment = MachOSegment<'data, 'file, Mach, R>`

- `type SegmentIterator = MachOSegmentIterator<'data, 'file, Mach, R>`

- `type Section = MachOSection<'data, 'file, Mach, R>`

- `type SectionIterator = MachOSectionIterator<'data, 'file, Mach, R>`

- `type Comdat = MachOComdat<'data, 'file, Mach, R>`

- `type ComdatIterator = MachOComdatIterator<'data, 'file, Mach, R>`

- `type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- `type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- `type SymbolTable = MachOSymbolTable<'data, 'file, Mach, R>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- `fn segments(self: &Self) -> MachOSegmentIterator<'data, '_, Mach, R>` — [`MachOSegmentIterator`](../index.md)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<MachOSection<'data, 'file, Mach, R>>` — [`MachOSection`](../index.md)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<MachOSection<'data, '_, Mach, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`MachOSection`](../index.md)

- `fn sections(self: &Self) -> MachOSectionIterator<'data, '_, Mach, R>` — [`MachOSectionIterator`](../index.md)

- `fn comdats(self: &Self) -> MachOComdatIterator<'data, '_, Mach, R>` — [`MachOComdatIterator`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<MachOSymbol<'data, '_, Mach, R>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`MachOSymbol`](../index.md)

- `fn symbols(self: &Self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](../index.md)

- `fn symbol_table(self: &Self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](../index.md)

- `fn dynamic_symbols(self: &Self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](../index.md)

- `fn dynamic_symbol_table(self: &Self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](../index.md)

- `fn object_map(self: &Self) -> ObjectMap<'data>` — [`ObjectMap`](../../../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>` — [`Result`](../../../index.md)

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../../index.md)

##### `impl<'data, Mach, R> Sealed for MachOFile<'data, Mach, R>`

### `MachOComdatIterator<'data, 'file, Mach, R>`

```rust
struct MachOComdatIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

An iterator for the COMDAT section groups in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for MachOComdatIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatIterator<'data, 'file, Mach, R>`

- `type Item = MachOComdat<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `MachOComdat<'data, 'file, Mach, R>`

```rust
struct MachOComdat<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

A COMDAT section group in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdat<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectComdat for MachOComdat<'data, 'file, Mach, R>`

- `type SectionIterator = MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md)

##### `impl<'data, 'file, Mach, R> Sealed for MachOComdat<'data, 'file, Mach, R>`

### `MachOComdatSectionIterator<'data, 'file, Mach, R>`

```rust
struct MachOComdatSectionIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `MachHeader`

```rust
trait MachHeader: Debug + Pod { ... }
```

A trait for generic access to [`macho::MachHeader32`](../../../macho/index.md) and [`macho::MachHeader64`](../../../macho/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `type Segment: 1`

- `type Section: 1`

- `type Nlist: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_big_endian(self: &Self) -> bool`

  Return true if the `magic` field signifies big-endian.

- `fn is_little_endian(self: &Self) -> bool`

  Return true if the `magic` field signifies little-endian.

- `fn magic(self: &Self) -> u32`

- `fn cputype(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn cpusubtype(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn filetype(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn ncmds(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn sizeofcmds(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(self: &Self) -> bool`

- `fn endian(self: &Self) -> Result<<Self as >::Endian>`

- `fn load_commands<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<LoadCommandIterator<'data, <Self as >::Endian>>`

- `fn uuid<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<Option<[u8; 16]>>`

  Return the UUID from the `LC_UUID` load command, if one is present.

## Type Aliases

### `MachOFile32<'data, Endian, R>`

```rust
type MachOFile32<'data, Endian, R> = MachOFile<'data, macho::MachHeader32<Endian>, R>;
```

A 32-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader32`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachO32`](../../../index.md).

### `MachOFile64<'data, Endian, R>`

```rust
type MachOFile64<'data, Endian, R> = MachOFile<'data, macho::MachHeader64<Endian>, R>;
```

A 64-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader64`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachO64`](../../../index.md).

### `MachOComdatIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator32<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the COMDAT section groups in a [`MachOFile64`](../index.md).

### `MachOComdatIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator64<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the COMDAT section groups in a [`MachOFile64`](../index.md).

### `MachOComdat32<'data, 'file, Endian, R>`

```rust
type MachOComdat32<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A COMDAT section group in a [`MachOFile32`](../index.md).

### `MachOComdat64<'data, 'file, Endian, R>`

```rust
type MachOComdat64<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A COMDAT section group in a [`MachOFile64`](../index.md).

### `MachOComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator32<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in a [`MachOFile32`](../index.md).

### `MachOComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator64<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in a [`MachOFile64`](../index.md).

