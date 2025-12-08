*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [file](index.md)*

---

# Module `file`

## Structs

### `ElfFile<'data, Elf, R>`

```rust
struct ElfFile<'data, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    data: R,
    header: &'data Elf,
    segments: &'data [<Elf as >::ProgramHeader],
    sections: super::SectionTable<'data, Elf, R>,
    relocations: super::RelocationSections,
    symbols: super::SymbolTable<'data, Elf, R>,
    dynamic_symbols: super::SymbolTable<'data, Elf, R>,
}
```

A partially parsed ELF file.

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> read::Result<Self>` — [`Result`](../../../index.md)

- `fn endian(self: &Self) -> <Elf as >::Endian` — [`FileHeader`](../index.md)

- `fn data(self: &Self) -> R`

- `fn raw_header(self: &Self) -> &'data Elf`

- `fn raw_segments(self: &Self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](../index.md)

- `fn elf_header(self: &Self) -> &'data Elf`

- `fn elf_program_headers(self: &Self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](../index.md)

- `fn elf_section_table(self: &Self) -> &SectionTable<'data, Elf, R>` — [`SectionTable`](../index.md)

- `fn elf_symbol_table(self: &Self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](../index.md)

- `fn elf_dynamic_symbol_table(self: &Self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](../index.md)

- `fn elf_relocation_sections(self: &Self) -> &RelocationSections` — [`RelocationSections`](../index.md)

- `fn raw_section_by_name<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](../index.md)

- `fn zdebug_section_by_name<'file>(self: &'file Self, _section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf, R> Debug for ElfFile<'data, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf, R> Object for ElfFile<'data, Elf, R>`

- `type Segment = ElfSegment<'data, 'file, Elf, R>`

- `type SegmentIterator = ElfSegmentIterator<'data, 'file, Elf, R>`

- `type Section = ElfSection<'data, 'file, Elf, R>`

- `type SectionIterator = ElfSectionIterator<'data, 'file, Elf, R>`

- `type Comdat = ElfComdat<'data, 'file, Elf, R>`

- `type ComdatIterator = ElfComdatIterator<'data, 'file, Elf, R>`

- `type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- `type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- `type SymbolTable = ElfSymbolTable<'data, 'file, Elf, R>`

- `type DynamicRelocationIterator = ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- `fn segments(self: &Self) -> ElfSegmentIterator<'data, '_, Elf, R>` — [`ElfSegmentIterator`](../index.md)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](../index.md)

- `fn section_by_index(self: &Self, index: SectionIndex) -> read::Result<ElfSection<'data, '_, Elf, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`ElfSection`](../index.md)

- `fn sections(self: &Self) -> ElfSectionIterator<'data, '_, Elf, R>` — [`ElfSectionIterator`](../index.md)

- `fn comdats(self: &Self) -> ElfComdatIterator<'data, '_, Elf, R>` — [`ElfComdatIterator`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<ElfSymbol<'data, '_, Elf, R>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ElfSymbol`](../index.md)

- `fn symbols(self: &Self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](../index.md)

- `fn symbol_table(self: &Self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](../index.md)

- `fn dynamic_symbols(self: &Self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](../index.md)

- `fn dynamic_symbol_table(self: &Self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](../index.md)

- `fn dynamic_relocations<'file>(self: &'file Self) -> Option<ElfDynamicRelocationIterator<'data, 'file, Elf, R>>` — [`ElfDynamicRelocationIterator`](../index.md)

- `fn imports(self: &Self) -> read::Result<Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- `fn exports(self: &Self) -> read::Result<Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn build_id(self: &Self) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn gnu_debuglink(self: &Self) -> read::Result<Option<(&'data [u8], u32)>>` — [`Result`](../../../index.md)

- `fn gnu_debugaltlink(self: &Self) -> read::Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../../index.md)

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../../index.md)

##### `impl<'data, Elf, R> Sealed for ElfFile<'data, Elf, R>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::FileHeader32`](../../../elf/index.md) and [`elf::FileHeader64`](../../../elf/index.md).

#### Required Methods

- `type Word: 3`

- `type Sword: 1`

- `type Endian: 1`

- `type ProgramHeader: 1`

- `type SectionHeader: 1`

- `type CompressionHeader: 1`

- `type NoteHeader: 1`

- `type Dyn: 1`

- `type Sym: 1`

- `type Rel: 1`

- `type Rela: 2`

- `type Relr: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_type_64_sized() -> bool`

  Return true if this type is a 64-bit header.

- `fn e_ident(self: &Self) -> &elf::Ident`

- `fn e_type(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_machine(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_version(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_entry(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_phoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_shoff(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_flags(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn e_ehsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_phnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shentsize(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shnum(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn e_shstrndx(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(self: &Self) -> bool`

  Check that the ident field in the file header is a supported format.

- `fn is_class_32(self: &Self) -> bool`

- `fn is_class_64(self: &Self) -> bool`

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_big_endian(self: &Self) -> bool`

- `fn endian(self: &Self) -> read::Result<<Self as >::Endian>`

- `fn section_0<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data <Self as >::SectionHeader>>`

  Return the first section header, if present.

- `fn phnum<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<usize>`

  Return the `e_phnum` field of the header. Handles extended values.

- `fn shnum<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<usize>`

  Return the `e_shnum` field of the header. Handles extended values.

- `fn shstrndx<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<u32>`

  Return the `e_shstrndx` field of the header. Handles extended values.

- `fn program_headers<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [<Self as >::ProgramHeader]>`

  Return the slice of program headers.

- `fn section_headers<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [<Self as >::SectionHeader]>`

  Return the slice of section headers.

- `fn section_strings_index<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<SectionIndex>`

  Get the section index of the section header string table.

- `fn section_strings<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R, sections: &[<Self as >::SectionHeader]) -> read::Result<StringTable<'data, R>>`

  Return the string table for the section headers.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, data: R) -> read::Result<SectionTable<'data, Self, R>>`

  Return the section table.

- `fn is_mips64el(self: &Self, endian: <Self as >::Endian) -> bool`

  Returns whether this is a mips64el elf file.

## Type Aliases

### `ElfFile32<'data, Endian, R>`

```rust
type ElfFile32<'data, Endian, R> = ElfFile<'data, elf::FileHeader32<Endian>, R>;
```

A 32-bit ELF object file.

This is a file that starts with [`elf::FileHeader32`](../../../elf/index.md), and corresponds
to [`crate::FileKind::Elf32`](../../../index.md).

### `ElfFile64<'data, Endian, R>`

```rust
type ElfFile64<'data, Endian, R> = ElfFile<'data, elf::FileHeader64<Endian>, R>;
```

A 64-bit ELF object file.

This is a file that starts with [`elf::FileHeader64`](../../../elf/index.md), and corresponds
to [`crate::FileKind::Elf64`](../../../index.md).

