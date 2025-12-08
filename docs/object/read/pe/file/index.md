*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [file](index.md)*

---

# Module `file`

## Structs

### `PeFile<'data, Pe, R>`

```rust
struct PeFile<'data, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    dos_header: &'data pe::ImageDosHeader,
    nt_headers: &'data Pe,
    data_directories: super::DataDirectories<'data>,
    common: crate::read::coff::CoffCommon<'data, R>,
    data: R,
}
```

A PE image file.

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn data(self: &Self) -> R`

- `fn dos_header(self: &Self) -> &'data pe::ImageDosHeader` — [`ImageDosHeader`](../../../pe/index.md)

- `fn nt_headers(self: &Self) -> &'data Pe`

- `fn rich_header_info(self: &Self) -> Option<RichHeaderInfo<'_>>` — [`RichHeaderInfo`](../index.md)

- `fn section_table(self: &Self) -> SectionTable<'data>` — [`SectionTable`](../../coff/index.md)

- `fn data_directories(self: &Self) -> DataDirectories<'data>` — [`DataDirectories`](../index.md)

- `fn data_directory(self: &Self, id: usize) -> Option<&'data pe::ImageDataDirectory>` — [`ImageDataDirectory`](../../../pe/index.md)

- `fn export_table(self: &Self) -> Result<Option<ExportTable<'data>>>` — [`Result`](../../../index.md), [`ExportTable`](../index.md)

- `fn import_table(self: &Self) -> Result<Option<ImportTable<'data>>>` — [`Result`](../../../index.md), [`ImportTable`](../index.md)

- `fn section_alignment(self: &Self) -> u64`

#### Trait Implementations

##### `impl<'data, Pe, R> Debug for PeFile<'data, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Pe, R> Object for PeFile<'data, Pe, R>`

- `type Segment = PeSegment<'data, 'file, Pe, R>`

- `type SegmentIterator = PeSegmentIterator<'data, 'file, Pe, R>`

- `type Section = PeSection<'data, 'file, Pe, R>`

- `type SectionIterator = PeSectionIterator<'data, 'file, Pe, R>`

- `type Comdat = PeComdat<'data, 'file, Pe, R>`

- `type ComdatIterator = PeComdatIterator<'data, 'file, Pe, R>`

- `type Symbol = CoffSymbol<'data, 'file, R>`

- `type SymbolIterator = CoffSymbolIterator<'data, 'file, R>`

- `type SymbolTable = CoffSymbolTable<'data, 'file, R>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- `fn segments(self: &Self) -> PeSegmentIterator<'data, '_, Pe, R>` — [`PeSegmentIterator`](../index.md)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<PeSection<'data, 'file, Pe, R>>` — [`PeSection`](../index.md)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<PeSection<'data, '_, Pe, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`PeSection`](../index.md)

- `fn sections(self: &Self) -> PeSectionIterator<'data, '_, Pe, R>` — [`PeSectionIterator`](../index.md)

- `fn comdats(self: &Self) -> PeComdatIterator<'data, '_, Pe, R>` — [`PeComdatIterator`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`CoffSymbol`](../../coff/index.md)

- `fn symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../../coff/index.md)

- `fn symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../../coff/index.md)

- `fn dynamic_symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R>` — [`CoffSymbolIterator`](../../coff/index.md)

- `fn dynamic_symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R>>` — [`CoffSymbolTable`](../../coff/index.md)

- `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- `fn pdb_info(self: &Self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../../index.md), [`CodeView`](../../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../../index.md)

##### `impl<'data, Pe, R> Sealed for PeFile<'data, Pe, R>`

### `PeComdatIterator<'data, 'file, Pe, R>`

```rust
struct PeComdatIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

An iterator for the COMDAT section groups in a [`PeFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdatIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeComdatIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeComdatIterator<'data, 'file, Pe, R>`

- `type Item = PeComdat<'data, 'file, Pe, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `PeComdat<'data, 'file, Pe, R>`

```rust
struct PeComdat<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

A COMDAT section group in a [`PeFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdat<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectComdat for PeComdat<'data, 'file, Pe, R>`

- `type SectionIterator = PeComdatSectionIterator<'data, 'file, Pe, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeComdat<'data, 'file, Pe, R>`

### `PeComdatSectionIterator<'data, 'file, Pe, R>`

```rust
struct PeComdatSectionIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file PeFile<'data, Pe, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`PeFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeComdatSectionIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeComdatSectionIterator<'data, 'file, Pe, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `ImageNtHeaders`

```rust
trait ImageNtHeaders: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageNtHeaders32`](../../../pe/index.md) and [`pe::ImageNtHeaders64`](../../../pe/index.md).

#### Required Methods

- `type ImageOptionalHeader: 1`

- `type ImageThunkData: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_valid_optional_magic(self: &Self) -> bool`

  Return true if the magic field in the optional header is valid.

- `fn signature(self: &Self) -> u32`

  Return the signature

- `fn file_header(self: &Self) -> &pe::ImageFileHeader`

  Return the file header.

- `fn optional_header(self: &Self) -> &<Self as >::ImageOptionalHeader`

  Return the optional header.

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<(&'data Self, DataDirectories<'data>)>`

  Read the NT headers, including the data directories.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<SymbolTable<'data, R>>`

  Read the COFF symbol table and string table.

### `ImageOptionalHeader`

```rust
trait ImageOptionalHeader: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageOptionalHeader32`](../../../pe/index.md) and [`pe::ImageOptionalHeader64`](../../../pe/index.md).

#### Required Methods

- `fn magic(self: &Self) -> u16`

- `fn major_linker_version(self: &Self) -> u8`

- `fn minor_linker_version(self: &Self) -> u8`

- `fn size_of_code(self: &Self) -> u32`

- `fn size_of_initialized_data(self: &Self) -> u32`

- `fn size_of_uninitialized_data(self: &Self) -> u32`

- `fn address_of_entry_point(self: &Self) -> u32`

- `fn base_of_code(self: &Self) -> u32`

- `fn base_of_data(self: &Self) -> Option<u32>`

- `fn image_base(self: &Self) -> u64`

- `fn section_alignment(self: &Self) -> u32`

- `fn file_alignment(self: &Self) -> u32`

- `fn major_operating_system_version(self: &Self) -> u16`

- `fn minor_operating_system_version(self: &Self) -> u16`

- `fn major_image_version(self: &Self) -> u16`

- `fn minor_image_version(self: &Self) -> u16`

- `fn major_subsystem_version(self: &Self) -> u16`

- `fn minor_subsystem_version(self: &Self) -> u16`

- `fn win32_version_value(self: &Self) -> u32`

- `fn size_of_image(self: &Self) -> u32`

- `fn size_of_headers(self: &Self) -> u32`

- `fn check_sum(self: &Self) -> u32`

- `fn subsystem(self: &Self) -> u16`

- `fn dll_characteristics(self: &Self) -> u16`

- `fn size_of_stack_reserve(self: &Self) -> u64`

- `fn size_of_stack_commit(self: &Self) -> u64`

- `fn size_of_heap_reserve(self: &Self) -> u64`

- `fn size_of_heap_commit(self: &Self) -> u64`

- `fn loader_flags(self: &Self) -> u32`

- `fn number_of_rva_and_sizes(self: &Self) -> u32`

## Functions

### `optional_header_magic`

```rust
fn optional_header_magic<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<u16>
```

Find the optional header and read its `magic` field.

It can be useful to know this magic value before trying to
fully parse the NT headers.

## Type Aliases

### `PeFile32<'data, R>`

```rust
type PeFile32<'data, R> = PeFile<'data, pe::ImageNtHeaders32, R>;
```

A PE32 (32-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders32`](../../../pe/index.md), and corresponds
to [`crate::FileKind::Pe32`](../../../index.md).

### `PeFile64<'data, R>`

```rust
type PeFile64<'data, R> = PeFile<'data, pe::ImageNtHeaders64, R>;
```

A PE32+ (64-bit) image file.

This is a file that starts with [`pe::ImageNtHeaders64`](../../../pe/index.md), and corresponds
to [`crate::FileKind::Pe64`](../../../index.md).

### `PeComdatIterator32<'data, 'file, R>`

```rust
type PeComdatIterator32<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the COMDAT section groups in a [`PeFile32`](../index.md).

### `PeComdatIterator64<'data, 'file, R>`

```rust
type PeComdatIterator64<'data, 'file, R> = PeComdatIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the COMDAT section groups in a [`PeFile64`](../index.md).

### `PeComdat32<'data, 'file, R>`

```rust
type PeComdat32<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders32, R>;
```

A COMDAT section group in a [`PeFile32`](../index.md).

### `PeComdat64<'data, 'file, R>`

```rust
type PeComdat64<'data, 'file, R> = PeComdat<'data, 'file, pe::ImageNtHeaders64, R>;
```

A COMDAT section group in a [`PeFile64`](../index.md).

### `PeComdatSectionIterator32<'data, 'file, R>`

```rust
type PeComdatSectionIterator32<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the sections in a COMDAT section group in a [`PeFile32`](../index.md).

### `PeComdatSectionIterator64<'data, 'file, R>`

```rust
type PeComdatSectionIterator64<'data, 'file, R> = PeComdatSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the sections in a COMDAT section group in a [`PeFile64`](../index.md).

