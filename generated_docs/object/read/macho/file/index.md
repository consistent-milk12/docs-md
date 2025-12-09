*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [file](index.md)*

---

# Module `file`

## Contents

- [Structs](#structs)
  - [`MachOFile`](#machofile)
  - [`MachOComdatIterator`](#machocomdatiterator)
  - [`MachOComdat`](#machocomdat)
  - [`MachOComdatSectionIterator`](#machocomdatsectioniterator)
- [Traits](#traits)
  - [`MachHeader`](#machheader)
- [Type Aliases](#type-aliases)
  - [`MachOFile32`](#machofile32)
  - [`MachOFile64`](#machofile64)
  - [`MachOComdatIterator32`](#machocomdatiterator32)
  - [`MachOComdatIterator64`](#machocomdatiterator64)
  - [`MachOComdat32`](#machocomdat32)
  - [`MachOComdat64`](#machocomdat64)
  - [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32)
  - [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MachOFile`](#machofile) | struct | A partially parsed Mach-O file. |
| [`MachOComdatIterator`](#machocomdatiterator) | struct | An iterator for the COMDAT section groups in a [`MachOFile`]. |
| [`MachOComdat`](#machocomdat) | struct | A COMDAT section group in a [`MachOFile`]. |
| [`MachOComdatSectionIterator`](#machocomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`MachOFile`]. |
| [`MachHeader`](#machheader) | trait | A trait for generic access to [`macho::MachHeader32`] and [`macho::MachHeader64`]. |
| [`MachOFile32`](#machofile32) | type | A 32-bit Mach-O object file. |
| [`MachOFile64`](#machofile64) | type | A 64-bit Mach-O object file. |
| [`MachOComdatIterator32`](#machocomdatiterator32) | type | An iterator for the COMDAT section groups in a [`MachOFile64`]. |
| [`MachOComdatIterator64`](#machocomdatiterator64) | type | An iterator for the COMDAT section groups in a [`MachOFile64`]. |
| [`MachOComdat32`](#machocomdat32) | type | A COMDAT section group in a [`MachOFile32`]. |
| [`MachOComdat64`](#machocomdat64) | type | A COMDAT section group in a [`MachOFile64`]. |
| [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`MachOFile32`]. |
| [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`MachOFile64`]. |

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

*Defined in [`object-0.37.3/src/read/macho/file.rs:37-49`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L37-L49)*

A partially parsed Mach-O file.

Most of the functionality of this type is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="machofile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- <span id="machofile-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: Endian>(image: &DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](../index.md), [`Result`](../../../index.md)

- <span id="machofile-section-internal"></span>`fn section_internal(&self, index: SectionIndex) -> Result<&MachOSectionInternal<'data, Mach, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`MachOSectionInternal`](../section/index.md)

- <span id="machofile-endian"></span>`fn endian(&self) -> <Mach as >::Endian` — [`MachHeader`](../index.md)

- <span id="machofile-data"></span>`fn data(&self) -> R`

- <span id="machofile-raw-header"></span>`fn raw_header(&self) -> &'data Mach`

- <span id="machofile-macho-header"></span>`fn macho_header(&self) -> &'data Mach`

- <span id="machofile-macho-load-commands"></span>`fn macho_load_commands(&self) -> Result<LoadCommandIterator<'data, <Mach as >::Endian>>` — [`Result`](../../../index.md), [`LoadCommandIterator`](../index.md), [`MachHeader`](../index.md)

- <span id="machofile-macho-symbol-table"></span>`fn macho_symbol_table(&self) -> &SymbolTable<'data, Mach, R>` — [`SymbolTable`](../index.md)

- <span id="machofile-build-version"></span>`fn build_version(&self) -> Result<Option<&'data macho::BuildVersionCommand<<Mach as >::Endian>>>` — [`Result`](../../../index.md), [`BuildVersionCommand`](../../../macho/index.md), [`MachHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Mach, R> Debug for MachOFile<'data, Mach, R>`

- <span id="machofile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Mach, R> Object for MachOFile<'data, Mach, R>`

- <span id="machofile-type-segment"></span>`type Segment = MachOSegment<'data, 'file, Mach, R>`

- <span id="machofile-type-segmentiterator"></span>`type SegmentIterator = MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-section"></span>`type Section = MachOSection<'data, 'file, Mach, R>`

- <span id="machofile-type-sectioniterator"></span>`type SectionIterator = MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-comdat"></span>`type Comdat = MachOComdat<'data, 'file, Mach, R>`

- <span id="machofile-type-comdatiterator"></span>`type ComdatIterator = MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-symbol"></span>`type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machofile-type-symboliterator"></span>`type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machofile-type-symboltable"></span>`type SymbolTable = MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machofile-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="machofile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md)

- <span id="machofile-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md)

- <span id="machofile-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="machofile-is-64"></span>`fn is_64(&self) -> bool`

- <span id="machofile-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- <span id="machofile-segments"></span>`fn segments(&self) -> MachOSegmentIterator<'data, '_, Mach, R>` — [`MachOSegmentIterator`](../index.md)

- <span id="machofile-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<MachOSection<'data, 'file, Mach, R>>` — [`MachOSection`](../index.md)

- <span id="machofile-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<MachOSection<'data, '_, Mach, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`MachOSection`](../index.md)

- <span id="machofile-sections"></span>`fn sections(&self) -> MachOSectionIterator<'data, '_, Mach, R>` — [`MachOSectionIterator`](../index.md)

- <span id="machofile-comdats"></span>`fn comdats(&self) -> MachOComdatIterator<'data, '_, Mach, R>` — [`MachOComdatIterator`](../index.md)

- <span id="machofile-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<MachOSymbol<'data, '_, Mach, R>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`MachOSymbol`](../index.md)

- <span id="machofile-symbols"></span>`fn symbols(&self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](../index.md)

- <span id="machofile-symbol-table"></span>`fn symbol_table(&self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](../index.md)

- <span id="machofile-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](../index.md)

- <span id="machofile-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](../index.md)

- <span id="machofile-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../../../index.md)

- <span id="machofile-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- <span id="machofile-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- <span id="machofile-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md)

- <span id="machofile-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="machofile-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../../../index.md)

- <span id="machofile-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="machofile-entry"></span>`fn entry(&self) -> u64`

- <span id="machofile-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md)

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

*Defined in [`object-0.37.3/src/read/macho/file.rs:527-534`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L527-L534)*

An iterator for the COMDAT section groups in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machocomdatiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machocomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-type-item"></span>`type Item = MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `MachOComdat<'data, 'file, Mach, R>`

```rust
struct MachOComdat<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:561-568`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L561-L568)*

A COMDAT section group in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectComdat for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-type-sectioniterator"></span>`type SectionIterator = MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md)

- <span id="machocomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- <span id="machocomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="machocomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md)

- <span id="machocomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md)

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

*Defined in [`object-0.37.3/src/read/macho/file.rs:621-628`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L621-L628)*

An iterator for the sections in a COMDAT section group in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machocomdatsectioniterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machocomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-type-item"></span>`type Item = SectionIndex`

- <span id="machocomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `MachHeader`

```rust
trait MachHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:644-723`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L644-L723)*

A trait for generic access to [`macho::MachHeader32`](../../../macho/index.md) and [`macho::MachHeader64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

- `type Segment: 1`

- `type Section: 1`

- `type Nlist: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_big_endian(&self) -> bool`

  Return true if the `magic` field signifies big-endian.

- `fn is_little_endian(&self) -> bool`

  Return true if the `magic` field signifies little-endian.

- `fn magic(&self) -> u32`

- `fn cputype(&self, endian: <Self as >::Endian) -> u32`

- `fn cpusubtype(&self, endian: <Self as >::Endian) -> u32`

- `fn filetype(&self, endian: <Self as >::Endian) -> u32`

- `fn ncmds(&self, endian: <Self as >::Endian) -> u32`

- `fn sizeofcmds(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn endian(&self) -> Result<<Self as >::Endian>`

- `fn load_commands<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<LoadCommandIterator<'data, <Self as >::Endian>>`

- `fn uuid<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<Option<[u8; 16]>>`

  Return the UUID from the `LC_UUID` load command, if one is present.

#### Implementors

- [`MachHeader32`](../../../macho/index.md)
- [`MachHeader64`](../../../macho/index.md)

## Type Aliases

### `MachOFile32<'data, Endian, R>`

```rust
type MachOFile32<'data, Endian, R> = MachOFile<'data, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:24-25`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L24-L25)*

A 32-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader32`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachO32`](../../../index.md).

### `MachOFile64<'data, Endian, R>`

```rust
type MachOFile64<'data, Endian, R> = MachOFile<'data, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:30-31`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L30-L31)*

A 64-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader64`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachO64`](../../../index.md).

### `MachOComdatIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator32<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:517-518`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L517-L518)*

An iterator for the COMDAT section groups in a [`MachOFile64`](../index.md).

### `MachOComdatIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator64<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:520-521`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L520-L521)*

An iterator for the COMDAT section groups in a [`MachOFile64`](../index.md).

### `MachOComdat32<'data, 'file, Endian, R>`

```rust
type MachOComdat32<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:550-551`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L550-L551)*

A COMDAT section group in a [`MachOFile32`](../index.md).

### `MachOComdat64<'data, 'file, Endian, R>`

```rust
type MachOComdat64<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:554-555`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L554-L555)*

A COMDAT section group in a [`MachOFile64`](../index.md).

### `MachOComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator32<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:611-612`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L611-L612)*

An iterator for the sections in a COMDAT section group in a [`MachOFile32`](../index.md).

### `MachOComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator64<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:614-615`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/file.rs#L614-L615)*

An iterator for the sections in a COMDAT section group in a [`MachOFile64`](../index.md).

