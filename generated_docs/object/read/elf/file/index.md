*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [file](index.md)*

---

# Module `file`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ElfFile`](#elffile) | struct | A partially parsed ELF file. |
| [`FileHeader`](#fileheader) | trait | A trait for generic access to [`elf::FileHeader32`] and [`elf::FileHeader64`]. |
| [`ElfFile32`](#elffile32) | type | A 32-bit ELF object file. |
| [`ElfFile64`](#elffile64) | type | A 64-bit ELF object file. |

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

*Defined in [`object-0.37.3/src/read/elf/file.rs:38-51`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/file.rs#L38-L51)*

A partially parsed ELF file.

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="elffile-parse"></span>`fn parse(data: R) -> read::Result<Self>` — [`Result`](../../../index.md#result)

  Parse the raw ELF file data.

- <span id="elffile-endian"></span>`fn endian(&self) -> <Elf as >::Endian` — [`FileHeader`](../index.md#fileheader)

  Returns the endianness.

- <span id="elffile-data"></span>`fn data(&self) -> R`

  Returns the raw data.

- <span id="elffile-raw-header"></span>`fn raw_header(&self) -> &'data Elf`

  Returns the raw ELF file header.

- <span id="elffile-raw-segments"></span>`fn raw_segments(&self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](../index.md#fileheader)

  Returns the raw ELF segments.

- <span id="elffile-elf-header"></span>`fn elf_header(&self) -> &'data Elf`

  Get the raw ELF file header.

- <span id="elffile-elf-program-headers"></span>`fn elf_program_headers(&self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](../index.md#fileheader)

  Get the raw ELF program headers.

  

  Returns an empty slice if the file has no program headers.

- <span id="elffile-elf-section-table"></span>`fn elf_section_table(&self) -> &SectionTable<'data, Elf, R>` — [`SectionTable`](../index.md#sectiontable)

  Get the ELF section table.

  

  Returns an empty section table if the file has no section headers.

- <span id="elffile-elf-symbol-table"></span>`fn elf_symbol_table(&self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](../index.md#symboltable)

  Get the ELF symbol table.

  

  Returns an empty symbol table if the file has no symbol table.

- <span id="elffile-elf-dynamic-symbol-table"></span>`fn elf_dynamic_symbol_table(&self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](../index.md#symboltable)

  Get the ELF dynamic symbol table.

  

  Returns an empty symbol table if the file has no dynamic symbol table.

- <span id="elffile-elf-relocation-sections"></span>`fn elf_relocation_sections(&self) -> &RelocationSections` — [`RelocationSections`](../index.md#relocationsections)

  Get a mapping for linked relocation sections.

- <span id="elffile-raw-section-by-name"></span>`fn raw_section_by_name<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](../index.md#elfsection)

- <span id="elffile-zdebug-section-by-name"></span>`fn zdebug_section_by_name<'file>(self: &'file Self, _section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](../index.md#elfsection)

#### Trait Implementations

##### `impl Any for ElfFile<'data, Elf, R>`

- <span id="elffile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfFile<'data, Elf, R>`

- <span id="elffile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfFile<'data, Elf, R>`

- <span id="elffile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfFile<'data, Elf, R>`

- <span id="elffile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfFile<'data, Elf, R>`

- <span id="elffile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfFile<'data, Elf, R>`

- <span id="elffile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf, R> Object for ElfFile<'data, Elf, R>`

- <span id="elffile-object-type-segment"></span>`type Segment = ElfSegment<'data, 'file, Elf, R>`

- <span id="elffile-object-type-segmentiterator"></span>`type SegmentIterator = ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elffile-object-type-section"></span>`type Section = ElfSection<'data, 'file, Elf, R>`

- <span id="elffile-object-type-sectioniterator"></span>`type SectionIterator = ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elffile-object-type-comdat"></span>`type Comdat = ElfComdat<'data, 'file, Elf, R>`

- <span id="elffile-object-type-comdatiterator"></span>`type ComdatIterator = ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elffile-object-type-symbol"></span>`type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elffile-object-type-symboliterator"></span>`type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elffile-object-type-symboltable"></span>`type SymbolTable = ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elffile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

- <span id="elffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="elffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="elffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md#objectkind)

- <span id="elffile-object-segments"></span>`fn segments(&self) -> ElfSegmentIterator<'data, '_, Elf, R>` — [`ElfSegmentIterator`](../index.md#elfsegmentiterator)

- <span id="elffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](../index.md#elfsection)

- <span id="elffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> read::Result<ElfSection<'data, '_, Elf, R>>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`ElfSection`](../index.md#elfsection)

- <span id="elffile-object-sections"></span>`fn sections(&self) -> ElfSectionIterator<'data, '_, Elf, R>` — [`ElfSectionIterator`](../index.md#elfsectioniterator)

- <span id="elffile-object-comdats"></span>`fn comdats(&self) -> ElfComdatIterator<'data, '_, Elf, R>` — [`ElfComdatIterator`](../index.md#elfcomdatiterator)

- <span id="elffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<ElfSymbol<'data, '_, Elf, R>>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ElfSymbol`](../index.md#elfsymbol)

- <span id="elffile-object-symbols"></span>`fn symbols(&self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](../index.md#elfsymboliterator)

- <span id="elffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](../index.md#elfsymboltable)

- <span id="elffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](../index.md#elfsymboliterator)

- <span id="elffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](../index.md#elfsymboltable)

- <span id="elffile-object-dynamic-relocations"></span>`fn dynamic_relocations<'file>(self: &'file Self) -> Option<ElfDynamicRelocationIterator<'data, 'file, Elf, R>>` — [`ElfDynamicRelocationIterator`](../index.md#elfdynamicrelocationiterator)

- <span id="elffile-object-imports"></span>`fn imports(&self) -> read::Result<Vec<Import<'data>>>` — [`Result`](../../../index.md#result), [`Import`](../../../index.md#import)

- <span id="elffile-object-exports"></span>`fn exports(&self) -> read::Result<Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../../../index.md#export)

- <span id="elffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="elffile-object-build-id"></span>`fn build_id(&self) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="elffile-object-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> read::Result<Option<(&'data [u8], u32)>>` — [`Result`](../../../index.md#result)

- <span id="elffile-object-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> read::Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../../index.md#result)

- <span id="elffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="elffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="elffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md#fileflags)

##### `impl<Elf, R> Sealed for ElfFile<'data, Elf, R>`

##### `impl<U> TryFrom for ElfFile<'data, Elf, R>`

- <span id="elffile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elffile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfFile<'data, Elf, R>`

- <span id="elffile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elffile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/file.rs:530-819`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/file.rs#L530-L819)*

A trait for generic access to [`elf::FileHeader32`](../../../elf/index.md) and [`elf::FileHeader64`](../../../elf/index.md).

#### Associated Types

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

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_type_64_sized() -> bool`

  Return true if this type is a 64-bit header.

- `fn e_ident(&self) -> &elf::Ident`

- `fn e_type(&self, endian: <Self as >::Endian) -> u16`

- `fn e_machine(&self, endian: <Self as >::Endian) -> u16`

- `fn e_version(&self, endian: <Self as >::Endian) -> u32`

- `fn e_entry(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_phoff(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_shoff(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn e_flags(&self, endian: <Self as >::Endian) -> u32`

- `fn e_ehsize(&self, endian: <Self as >::Endian) -> u16`

- `fn e_phentsize(&self, endian: <Self as >::Endian) -> u16`

- `fn e_phnum(&self, endian: <Self as >::Endian) -> u16`

- `fn e_shentsize(&self, endian: <Self as >::Endian) -> u16`

- `fn e_shnum(&self, endian: <Self as >::Endian) -> u16`

- `fn e_shstrndx(&self, endian: <Self as >::Endian) -> u16`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

  Check that the ident field in the file header is a supported format.

- `fn is_class_32(&self) -> bool`

- `fn is_class_64(&self) -> bool`

- `fn is_little_endian(&self) -> bool`

- `fn is_big_endian(&self) -> bool`

- `fn endian(&self) -> read::Result<<Self as >::Endian>`

- `fn section_0<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data <Self as >::SectionHeader>>`

  Return the first section header, if present.

- `fn phnum<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<usize>`

  Return the `e_phnum` field of the header. Handles extended values.

- `fn shnum<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<usize>`

  Return the `e_shnum` field of the header. Handles extended values.

- `fn shstrndx<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<u32>`

  Return the `e_shstrndx` field of the header. Handles extended values.

- `fn program_headers<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [<Self as >::ProgramHeader]>`

  Return the slice of program headers.

- `fn section_headers<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [<Self as >::SectionHeader]>`

  Return the slice of section headers.

- `fn section_strings_index<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<SectionIndex>`

  Get the section index of the section header string table.

- `fn section_strings<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, sections: &[<Self as >::SectionHeader]) -> read::Result<StringTable<'data, R>>`

  Return the string table for the section headers.

- `fn sections<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<SectionTable<'data, Self, R>>`

  Return the section table.

- `fn is_mips64el(&self, endian: <Self as >::Endian) -> bool`

  Returns whether this is a mips64el elf file.

#### Implementors

- [`FileHeader32`](../../../elf/index.md#fileheader32)
- [`FileHeader64`](../../../elf/index.md#fileheader64)

## Type Aliases

### `ElfFile32<'data, Endian, R>`

```rust
type ElfFile32<'data, Endian, R> = ElfFile<'data, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/file.rs:25-26`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/file.rs#L25-L26)*

A 32-bit ELF object file.

This is a file that starts with [`elf::FileHeader32`](../../../elf/index.md), and corresponds
to [`crate::FileKind::Elf32`](../../../index.md).

### `ElfFile64<'data, Endian, R>`

```rust
type ElfFile64<'data, Endian, R> = ElfFile<'data, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/file.rs:31-32`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/file.rs#L31-L32)*

A 64-bit ELF object file.

This is a file that starts with [`elf::FileHeader64`](../../../elf/index.md), and corresponds
to [`crate::FileKind::Elf64`](../../../index.md).

