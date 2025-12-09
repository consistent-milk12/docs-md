*[object](../../index.md) / [read](../index.md) / [elf](index.md)*

---

# Module `elf`

Support for reading ELF files.

Traits are used to abstract over the difference between 32-bit and 64-bit ELF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`ElfFile`](#elffile) implements the [`Object`](crate::read::Object) trait for ELF files.
[`ElfFile`](#elffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit ELF. There are type aliases for these parameters ([`ElfFile32`](#elffile32) and
[`ElfFile64`](#elffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`elf::FileHeader32`](../../elf/index.md)
and [`elf::FileHeader64`](../../elf/index.md).

### Example for low level API
 ```no_run
use object::elf;
use object::read::elf::{FileHeader, Sym};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    let sections = elf.sections(endian, &*data)?;
    let symbols = sections.symbols(endian, &*data, elf::SHT_SYMTAB)?;
    for symbol in symbols.iter() {
        let name = symbol.name(endian, symbols.strings())?;
        println!("{}", String::from_utf8_lossy(name));
    }
  }
    Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`file`](#file)
  - [`segment`](#segment)
  - [`section`](#section)
  - [`symbol`](#symbol)
  - [`relocation`](#relocation)
  - [`comdat`](#comdat)
  - [`dynamic`](#dynamic)
  - [`compression`](#compression)
  - [`note`](#note)
  - [`hash`](#hash)
  - [`version`](#version)
  - [`attributes`](#attributes)
- [Structs](#structs)
  - [`ElfFile`](#elffile)
  - [`ElfSegmentIterator`](#elfsegmentiterator)
  - [`ElfSegment`](#elfsegment)
  - [`SectionTable`](#sectiontable)
  - [`ElfSectionIterator`](#elfsectioniterator)
  - [`ElfSection`](#elfsection)
  - [`SymbolTable`](#symboltable)
  - [`ElfSymbolTable`](#elfsymboltable)
  - [`ElfSymbolIterator`](#elfsymboliterator)
  - [`ElfSymbol`](#elfsymbol)
  - [`RelocationSections`](#relocationsections)
  - [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator)
  - [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator)
  - [`RelrIterator`](#relriterator)
  - [`Crel`](#crel)
  - [`CrelIteratorHeader`](#creliteratorheader)
  - [`CrelIteratorState`](#creliteratorstate)
  - [`CrelIterator`](#creliterator)
  - [`ElfComdatIterator`](#elfcomdatiterator)
  - [`ElfComdat`](#elfcomdat)
  - [`ElfComdatSectionIterator`](#elfcomdatsectioniterator)
  - [`NoteIterator`](#noteiterator)
  - [`Note`](#note)
  - [`GnuPropertyIterator`](#gnupropertyiterator)
  - [`GnuProperty`](#gnuproperty)
  - [`HashTable`](#hashtable)
  - [`GnuHashTable`](#gnuhashtable)
  - [`VersionIndex`](#versionindex)
  - [`Version`](#version)
  - [`VersionTable`](#versiontable)
  - [`VerdefIterator`](#verdefiterator)
  - [`VerdauxIterator`](#verdauxiterator)
  - [`VerneedIterator`](#verneediterator)
  - [`VernauxIterator`](#vernauxiterator)
  - [`AttributesSection`](#attributessection)
  - [`AttributesSubsectionIterator`](#attributessubsectioniterator)
  - [`AttributesSubsection`](#attributessubsection)
  - [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)
  - [`AttributesSubsubsection`](#attributessubsubsection)
  - [`AttributeIndexIterator`](#attributeindexiterator)
  - [`AttributeReader`](#attributereader)
- [Enums](#enums)
  - [`ElfRelocationIterator`](#elfrelocationiterator)
- [Traits](#traits)
  - [`FileHeader`](#fileheader)
  - [`ProgramHeader`](#programheader)
  - [`SectionHeader`](#sectionheader)
  - [`Sym`](#sym)
  - [`Rel`](#rel)
  - [`Rela`](#rela)
  - [`Relr`](#relr)
  - [`Dyn`](#dyn)
  - [`CompressionHeader`](#compressionheader)
  - [`NoteHeader`](#noteheader)
- [Functions](#functions)
  - [`parse_relocation`](#parse_relocation)
- [Type Aliases](#type-aliases)
  - [`ElfFile32`](#elffile32)
  - [`ElfFile64`](#elffile64)
  - [`ElfSegmentIterator32`](#elfsegmentiterator32)
  - [`ElfSegmentIterator64`](#elfsegmentiterator64)
  - [`ElfSegment32`](#elfsegment32)
  - [`ElfSegment64`](#elfsegment64)
  - [`ElfSectionIterator32`](#elfsectioniterator32)
  - [`ElfSectionIterator64`](#elfsectioniterator64)
  - [`ElfSection32`](#elfsection32)
  - [`ElfSection64`](#elfsection64)
  - [`ElfSymbolTable32`](#elfsymboltable32)
  - [`ElfSymbolTable64`](#elfsymboltable64)
  - [`ElfSymbolIterator32`](#elfsymboliterator32)
  - [`ElfSymbolIterator64`](#elfsymboliterator64)
  - [`ElfSymbol32`](#elfsymbol32)
  - [`ElfSymbol64`](#elfsymbol64)
  - [`ElfDynamicRelocationIterator32`](#elfdynamicrelocationiterator32)
  - [`ElfDynamicRelocationIterator64`](#elfdynamicrelocationiterator64)
  - [`ElfSectionRelocationIterator32`](#elfsectionrelocationiterator32)
  - [`ElfSectionRelocationIterator64`](#elfsectionrelocationiterator64)
  - [`ElfComdatIterator32`](#elfcomdatiterator32)
  - [`ElfComdatIterator64`](#elfcomdatiterator64)
  - [`ElfComdat32`](#elfcomdat32)
  - [`ElfComdat64`](#elfcomdat64)
  - [`ElfComdatSectionIterator32`](#elfcomdatsectioniterator32)
  - [`ElfComdatSectionIterator64`](#elfcomdatsectioniterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`file`](#file) | mod |  |
| [`segment`](#segment) | mod |  |
| [`section`](#section) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`comdat`](#comdat) | mod |  |
| [`dynamic`](#dynamic) | mod |  |
| [`compression`](#compression) | mod |  |
| [`note`](#note) | mod |  |
| [`hash`](#hash) | mod |  |
| [`version`](#version) | mod |  |
| [`attributes`](#attributes) | mod |  |
| [`ElfFile`](#elffile) | struct | A partially parsed ELF file. |
| [`ElfSegmentIterator`](#elfsegmentiterator) | struct | An iterator for the segments in an [`ElfFile`]. |
| [`ElfSegment`](#elfsegment) | struct | A segment in an [`ElfFile`]. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an ELF file. |
| [`ElfSectionIterator`](#elfsectioniterator) | struct | An iterator for the sections in an [`ElfFile`]. |
| [`ElfSection`](#elfsection) | struct | A section in an [`ElfFile`]. |
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in an ELF file. |
| [`ElfSymbolTable`](#elfsymboltable) | struct | A symbol table in an [`ElfFile`](super::ElfFile). |
| [`ElfSymbolIterator`](#elfsymboliterator) | struct | An iterator for the symbols in an [`ElfFile`](super::ElfFile). |
| [`ElfSymbol`](#elfsymbol) | struct | A symbol in an [`ElfFile`](super::ElfFile). |
| [`RelocationSections`](#relocationsections) | struct | A mapping from section index to associated relocation sections. |
| [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator) | struct | An iterator for the dynamic relocations in an [`ElfFile`]. |
| [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator) | struct | An iterator for the relocations for an [`ElfSection`](super::ElfSection). |
| [`RelrIterator`](#relriterator) | struct | An iterator over the relative relocations in an ELF `SHT_RELR` section. |
| [`Crel`](#crel) | struct | Compact relocation |
| [`CrelIteratorHeader`](#creliteratorheader) | struct |  |
| [`CrelIteratorState`](#creliteratorstate) | struct |  |
| [`CrelIterator`](#creliterator) | struct | Compact relocation iterator. |
| [`ElfComdatIterator`](#elfcomdatiterator) | struct | An iterator for the COMDAT section groups in an [`ElfFile`]. |
| [`ElfComdat`](#elfcomdat) | struct | A COMDAT section group in an [`ElfFile`]. |
| [`ElfComdatSectionIterator`](#elfcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in an [`ElfFile`]. |
| [`NoteIterator`](#noteiterator) | struct | An iterator over the notes in an ELF section or segment. |
| [`Note`](#note) | struct | A parsed [`NoteHeader`]. |
| [`GnuPropertyIterator`](#gnupropertyiterator) | struct | An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note. |
| [`GnuProperty`](#gnuproperty) | struct | A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note. |
| [`HashTable`](#hashtable) | struct | A SysV symbol hash table in an ELF file. |
| [`GnuHashTable`](#gnuhashtable) | struct | A GNU symbol hash table in an ELF file. |
| [`VersionIndex`](#versionindex) | struct | A version index. |
| [`Version`](#version) | struct | A version definition or requirement. |
| [`VersionTable`](#versiontable) | struct | A table of version definitions and requirements. |
| [`VerdefIterator`](#verdefiterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerdauxIterator`](#verdauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerneedIterator`](#verneediterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section. |
| [`VernauxIterator`](#vernauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section. |
| [`AttributesSection`](#attributessection) | struct | An ELF attributes section. |
| [`AttributesSubsectionIterator`](#attributessubsectioniterator) | struct | An iterator for the subsections in an [`AttributesSection`]. |
| [`AttributesSubsection`](#attributessubsection) | struct | A subsection in an [`AttributesSection`]. |
| [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator) | struct | An iterator for the sub-subsections in an [`AttributesSubsection`]. |
| [`AttributesSubsubsection`](#attributessubsubsection) | struct | A sub-subsection in an [`AttributesSubsection`]. |
| [`AttributeIndexIterator`](#attributeindexiterator) | struct | An iterator over the indices in an [`AttributesSubsubsection`]. |
| [`AttributeReader`](#attributereader) | struct | A parser for the attributes in an [`AttributesSubsubsection`]. |
| [`ElfRelocationIterator`](#elfrelocationiterator) | enum |  |
| [`FileHeader`](#fileheader) | trait | A trait for generic access to [`elf::FileHeader32`] and [`elf::FileHeader64`]. |
| [`ProgramHeader`](#programheader) | trait | A trait for generic access to [`elf::ProgramHeader32`] and [`elf::ProgramHeader64`]. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`elf::SectionHeader32`] and [`elf::SectionHeader64`]. |
| [`Sym`](#sym) | trait | A trait for generic access to [`elf::Sym32`] and [`elf::Sym64`]. |
| [`Rel`](#rel) | trait | A trait for generic access to [`elf::Rel32`] and [`elf::Rel64`]. |
| [`Rela`](#rela) | trait | A trait for generic access to [`elf::Rela32`] and [`elf::Rela64`]. |
| [`Relr`](#relr) | trait | A trait for generic access to [`elf::Relr32`] and [`elf::Relr64`]. |
| [`Dyn`](#dyn) | trait | A trait for generic access to [`elf::Dyn32`] and [`elf::Dyn64`]. |
| [`CompressionHeader`](#compressionheader) | trait | A trait for generic access to [`elf::CompressionHeader32`] and [`elf::CompressionHeader64`]. |
| [`NoteHeader`](#noteheader) | trait | A trait for generic access to [`elf::NoteHeader32`] and [`elf::NoteHeader64`]. |
| [`parse_relocation`](#parse_relocation) | fn |  |
| [`ElfFile32`](#elffile32) | type | A 32-bit ELF object file. |
| [`ElfFile64`](#elffile64) | type | A 64-bit ELF object file. |
| [`ElfSegmentIterator32`](#elfsegmentiterator32) | type | An iterator for the segments in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSegmentIterator64`](#elfsegmentiterator64) | type | An iterator for the segments in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSegment32`](#elfsegment32) | type | A segment in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSegment64`](#elfsegment64) | type | A segment in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSectionIterator32`](#elfsectioniterator32) | type | An iterator for the sections in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSectionIterator64`](#elfsectioniterator64) | type | An iterator for the sections in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSection32`](#elfsection32) | type | A section in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSection64`](#elfsection64) | type | A section in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSymbolTable32`](#elfsymboltable32) | type | A symbol table in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbolTable64`](#elfsymboltable64) | type | A symbol table in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbolIterator32`](#elfsymboliterator32) | type | An iterator for the symbols in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbolIterator64`](#elfsymboliterator64) | type | An iterator for the symbols in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSymbol32`](#elfsymbol32) | type | A symbol in an [`ElfFile32`](super::ElfFile32). |
| [`ElfSymbol64`](#elfsymbol64) | type | A symbol in an [`ElfFile64`](super::ElfFile64). |
| [`ElfDynamicRelocationIterator32`](#elfdynamicrelocationiterator32) | type | An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32). |
| [`ElfDynamicRelocationIterator64`](#elfdynamicrelocationiterator64) | type | An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSectionRelocationIterator32`](#elfsectionrelocationiterator32) | type | An iterator for the relocations for an [`ElfSection32`](super::ElfSection32). |
| [`ElfSectionRelocationIterator64`](#elfsectionrelocationiterator64) | type | An iterator for the relocations for an [`ElfSection64`](super::ElfSection64). |
| [`ElfComdatIterator32`](#elfcomdatiterator32) | type | An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32). |
| [`ElfComdatIterator64`](#elfcomdatiterator64) | type | An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64). |
| [`ElfComdat32`](#elfcomdat32) | type | A COMDAT section group in an [`ElfFile32`](super::ElfFile32). |
| [`ElfComdat64`](#elfcomdat64) | type | A COMDAT section group in an [`ElfFile64`](super::ElfFile64). |
| [`ElfComdatSectionIterator32`](#elfcomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32). |
| [`ElfComdatSectionIterator64`](#elfcomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64). |

## Modules

- [`file`](file/index.md)
- [`segment`](segment/index.md)
- [`section`](section/index.md)
- [`symbol`](symbol/index.md)
- [`relocation`](relocation/index.md)
- [`comdat`](comdat/index.md)
- [`dynamic`](dynamic/index.md)
- [`compression`](compression/index.md)
- [`note`](note/index.md)
- [`hash`](hash/index.md)
- [`version`](version/index.md)
- [`attributes`](attributes/index.md)

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

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="elffile-parse"></span>`fn parse(data: R) -> read::Result<Self>` — [`Result`](../../index.md)

- <span id="elffile-endian"></span>`fn endian(&self) -> <Elf as >::Endian` — [`FileHeader`](#fileheader)

- <span id="elffile-data"></span>`fn data(&self) -> R`

- <span id="elffile-raw-header"></span>`fn raw_header(&self) -> &'data Elf`

- <span id="elffile-raw-segments"></span>`fn raw_segments(&self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](#fileheader)

- <span id="elffile-elf-header"></span>`fn elf_header(&self) -> &'data Elf`

- <span id="elffile-elf-program-headers"></span>`fn elf_program_headers(&self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](#fileheader)

- <span id="elffile-elf-section-table"></span>`fn elf_section_table(&self) -> &SectionTable<'data, Elf, R>` — [`SectionTable`](#sectiontable)

- <span id="elffile-elf-symbol-table"></span>`fn elf_symbol_table(&self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

- <span id="elffile-elf-dynamic-symbol-table"></span>`fn elf_dynamic_symbol_table(&self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

- <span id="elffile-elf-relocation-sections"></span>`fn elf_relocation_sections(&self) -> &RelocationSections` — [`RelocationSections`](#relocationsections)

- <span id="elffile-raw-section-by-name"></span>`fn raw_section_by_name<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

- <span id="elffile-zdebug-section-by-name"></span>`fn zdebug_section_by_name<'file>(self: &'file Self, _section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

#### Trait Implementations

##### `impl<'data, Elf, R> Debug for ElfFile<'data, Elf, R>`

- <span id="elffile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Elf, R> Object for ElfFile<'data, Elf, R>`

- <span id="elffile-segment"></span>`type Segment = ElfSegment<'data, 'file, Elf, R>`

- <span id="elffile-segmentiterator"></span>`type SegmentIterator = ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elffile-section"></span>`type Section = ElfSection<'data, 'file, Elf, R>`

- <span id="elffile-sectioniterator"></span>`type SectionIterator = ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elffile-comdat"></span>`type Comdat = ElfComdat<'data, 'file, Elf, R>`

- <span id="elffile-comdatiterator"></span>`type ComdatIterator = ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elffile-symbol"></span>`type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elffile-symboliterator"></span>`type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elffile-symboltable"></span>`type SymbolTable = ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elffile-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elffile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md)

- <span id="elffile-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="elffile-is-64"></span>`fn is_64(&self) -> bool`

- <span id="elffile-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- <span id="elffile-segments"></span>`fn segments(&self) -> ElfSegmentIterator<'data, '_, Elf, R>` — [`ElfSegmentIterator`](#elfsegmentiterator)

- <span id="elffile-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

- <span id="elffile-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> read::Result<ElfSection<'data, '_, Elf, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`ElfSection`](#elfsection)

- <span id="elffile-sections"></span>`fn sections(&self) -> ElfSectionIterator<'data, '_, Elf, R>` — [`ElfSectionIterator`](#elfsectioniterator)

- <span id="elffile-comdats"></span>`fn comdats(&self) -> ElfComdatIterator<'data, '_, Elf, R>` — [`ElfComdatIterator`](#elfcomdatiterator)

- <span id="elffile-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<ElfSymbol<'data, '_, Elf, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ElfSymbol`](#elfsymbol)

- <span id="elffile-symbols"></span>`fn symbols(&self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](#elfsymboliterator)

- <span id="elffile-symbol-table"></span>`fn symbol_table(&self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](#elfsymboltable)

- <span id="elffile-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](#elfsymboliterator)

- <span id="elffile-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](#elfsymboltable)

- <span id="elffile-dynamic-relocations"></span>`fn dynamic_relocations<'file>(self: &'file Self) -> Option<ElfDynamicRelocationIterator<'data, 'file, Elf, R>>` — [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator)

- <span id="elffile-imports"></span>`fn imports(&self) -> read::Result<Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- <span id="elffile-exports"></span>`fn exports(&self) -> read::Result<Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- <span id="elffile-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="elffile-build-id"></span>`fn build_id(&self) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="elffile-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> read::Result<Option<(&'data [u8], u32)>>` — [`Result`](../../index.md)

- <span id="elffile-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> read::Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../index.md)

- <span id="elffile-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="elffile-entry"></span>`fn entry(&self) -> u64`

- <span id="elffile-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, Elf, R> Sealed for ElfFile<'data, Elf, R>`

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

An iterator for the segments in an [`ElfFile`](#elffile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsegmentiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfsegmentiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-item"></span>`type Item = ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

A segment in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- <span id="elfsegment-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

- <span id="elfsegment-elf-program-header"></span>`fn elf_program_header(&self) -> &'data <Elf as >::ProgramHeader` — [`FileHeader`](#fileheader)

- <span id="elfsegment-bytes"></span>`fn bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSegment for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-address"></span>`fn address(&self) -> u64`

- <span id="elfsegment-size"></span>`fn size(&self) -> u64`

- <span id="elfsegment-align"></span>`fn align(&self) -> u64`

- <span id="elfsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="elfsegment-data"></span>`fn data(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="elfsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="elfsegment-name-bytes"></span>`fn name_bytes(&self) -> read::Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="elfsegment-name"></span>`fn name(&self) -> read::Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="elfsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSegment<'data, 'file, Elf, R>`

### `SectionTable<'data, Elf: FileHeader, R>`

```rust
struct SectionTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    sections: &'data [<Elf as >::SectionHeader],
    strings: crate::read::StringTable<'data, R>,
}
```

The table of section headers in an ELF file.

Also includes the string table used for the section names.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-new"></span>`fn new(sections: &'data [<Elf as >::SectionHeader], strings: StringTable<'data, R>) -> Self` — [`FileHeader`](#fileheader), [`StringTable`](../index.md)

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Elf as >::SectionHeader>` — [`FileHeader`](#fileheader)

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data <Elf as >::SectionHeader)>` — [`SectionIndex`](../../index.md), [`FileHeader`](#fileheader)

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Elf as >::SectionHeader>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="sectiontable-section-by-name"></span>`fn section_by_name(&self, endian: <Elf as >::Endian, name: &[u8]) -> Option<(SectionIndex, &'data <Elf as >::SectionHeader)>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md)

- <span id="sectiontable-section-name"></span>`fn section_name(&self, endian: <Elf as >::Endian, section: &<Elf as >::SectionHeader) -> read::Result<&'data [u8]>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- <span id="sectiontable-strings"></span>`fn strings(&self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<StringTable<'data, R>>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`StringTable`](../index.md)

- <span id="sectiontable-symbols"></span>`fn symbols(&self, endian: <Elf as >::Endian, data: R, sh_type: u32) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`SymbolTable`](#symboltable)

- <span id="sectiontable-symbol-table-by-index"></span>`fn symbol_table_by_index(&self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`SymbolTable`](#symboltable)

- <span id="sectiontable-relocation-sections"></span>`fn relocation_sections(&self, endian: <Elf as >::Endian, symbol_section: SectionIndex) -> read::Result<RelocationSections>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`RelocationSections`](#relocationsections)

- <span id="sectiontable-dynamic"></span>`fn dynamic(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [<Elf as >::Dyn], SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`SectionIndex`](../../index.md)

- <span id="sectiontable-hash-header"></span>`fn hash_header(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Elf as >::Endian>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`HashHeader`](../../elf/index.md)

- <span id="sectiontable-hash"></span>`fn hash(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`HashTable`](#hashtable), [`SectionIndex`](../../index.md)

- <span id="sectiontable-gnu-hash-header"></span>`fn gnu_hash_header(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Elf as >::Endian>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`GnuHashHeader`](../../elf/index.md)

- <span id="sectiontable-gnu-hash"></span>`fn gnu_hash(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`GnuHashTable`](#gnuhashtable), [`SectionIndex`](../../index.md)

- <span id="sectiontable-gnu-versym"></span>`fn gnu_versym(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Elf as >::Endian>], SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`Versym`](../../elf/index.md), [`SectionIndex`](../../index.md)

- <span id="sectiontable-gnu-verdef"></span>`fn gnu_verdef(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`VerdefIterator`](#verdefiterator), [`SectionIndex`](../../index.md)

- <span id="sectiontable-gnu-verneed"></span>`fn gnu_verneed(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`VerneedIterator`](#verneediterator), [`SectionIndex`](../../index.md)

- <span id="sectiontable-versions"></span>`fn versions(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<VersionTable<'data, Elf>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md), [`VersionTable`](#versiontable)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader, R> Clone for SectionTable<'data, Elf, R>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Elf, R>` — [`SectionTable`](#sectiontable)

##### `impl<'data, Elf: marker::Copy + FileHeader, R> Copy for SectionTable<'data, Elf, R>`

##### `impl<'data, Elf: fmt::Debug + FileHeader, R> Debug for SectionTable<'data, Elf, R>`

- <span id="sectiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Elf: FileHeader, R: ReadRef<'data>> Default for SectionTable<'data, Elf, R>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

### `ElfSectionIterator<'data, 'file, Elf, R>`

```rust
struct ElfSectionIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Elf as >::SectionHeader>>,
}
```

An iterator for the sections in an [`ElfFile`](#elffile).

#### Implementations

- <span id="elfsectioniterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> Self` — [`ElfFile`](#elffile)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectioniterator-intoiter"></span>`type IntoIter = I`

- <span id="elfsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-item"></span>`type Item = ElfSection<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ElfSection<'data, 'file, Elf, R>`

```rust
struct ElfSection<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    index: crate::read::SectionIndex,
    section: &'data <Elf as >::SectionHeader,
}
```

A section in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="elfsection-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

- <span id="elfsection-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](#fileheader)

- <span id="elfsection-elf-relocation-section-index"></span>`fn elf_relocation_section_index(&self) -> read::Result<Option<SectionIndex>>` — [`Result`](../../index.md), [`SectionIndex`](../../index.md)

- <span id="elfsection-elf-relocation-section"></span>`fn elf_relocation_section(&self) -> read::Result<Option<&'data <Elf as >::SectionHeader>>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="elfsection-elf-linked-rel"></span>`fn elf_linked_rel(&self) -> read::Result<&'data [<Elf as >::Rel]>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="elfsection-elf-linked-rela"></span>`fn elf_linked_rela(&self) -> read::Result<&'data [<Elf as >::Rela]>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="elfsection-bytes"></span>`fn bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="elfsection-maybe-compressed"></span>`fn maybe_compressed(&self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- <span id="elfsection-maybe-compressed-gnu"></span>`fn maybe_compressed_gnu(&self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectSection for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-relocationiterator"></span>`type RelocationIterator = ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="elfsection-address"></span>`fn address(&self) -> u64`

- <span id="elfsection-size"></span>`fn size(&self) -> u64`

- <span id="elfsection-align"></span>`fn align(&self) -> u64`

- <span id="elfsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="elfsection-data"></span>`fn data(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="elfsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- <span id="elfsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> read::Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- <span id="elfsection-compressed-data"></span>`fn compressed_data(&self) -> read::Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- <span id="elfsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="elfsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- <span id="elfsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> read::Result<Option<&[u8]>>` — [`Result`](../../index.md)

- <span id="elfsection-segment-name"></span>`fn segment_name(&self) -> read::Result<Option<&str>>` — [`Result`](../../index.md)

- <span id="elfsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md)

- <span id="elfsection-relocations"></span>`fn relocations(&self) -> ElfSectionRelocationIterator<'data, 'file, Elf, R>` — [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator)

- <span id="elfsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- <span id="elfsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfSection<'data, 'file, Elf, R>`

### `SymbolTable<'data, Elf: FileHeader, R>`

```rust
struct SymbolTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    section: crate::read::SectionIndex,
    string_section: crate::read::SectionIndex,
    shndx_section: crate::read::SectionIndex,
    symbols: &'data [<Elf as >::Sym],
    strings: crate::read::util::StringTable<'data, R>,
    shndx: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A table of symbol entries in an ELF file.

Also includes the string table used for the symbol names.

Returned by `SectionTable::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: R, sections: &SectionTable<'data, Elf, R>, section_index: SectionIndex, section: &<Elf as >::SectionHeader) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`SectionTable`](#sectiontable), [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`SymbolTable`](#symboltable)

- <span id="symboltable-section"></span>`fn section(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="symboltable-shndx-section"></span>`fn shndx_section(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="symboltable-string-section"></span>`fn string_section(&self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- <span id="symboltable-symbols"></span>`fn symbols(&self) -> &'data [<Elf as >::Sym]` — [`FileHeader`](#fileheader)

- <span id="symboltable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Elf as >::Sym>` — [`FileHeader`](#fileheader)

- <span id="symboltable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SymbolIndex, &'data <Elf as >::Sym)>` — [`SymbolIndex`](../../index.md), [`FileHeader`](#fileheader)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> read::Result<&'data <Elf as >::Sym>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- <span id="symboltable-shndx"></span>`fn shndx(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> Option<u32>` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- <span id="symboltable-symbol-section"></span>`fn symbol_section(&self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym, index: SymbolIndex) -> read::Result<Option<SectionIndex>>` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`SectionIndex`](../../index.md)

- <span id="symboltable-symbol-name"></span>`fn symbol_name(&self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym) -> read::Result<&'data [u8]>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Elf as >::Sym) -> Option<Entry>>(&self, endian: <Elf as >::Endian, f: F) -> SymbolMap<Entry>` — [`FileHeader`](#fileheader), [`SymbolMap`](../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader, R> Clone for SymbolTable<'data, Elf, R>`

- <span id="symboltable-clone"></span>`fn clone(&self) -> SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

##### `impl<'data, Elf: marker::Copy + FileHeader, R> Copy for SymbolTable<'data, Elf, R>`

##### `impl<'data, Elf: fmt::Debug + FileHeader, R> Debug for SymbolTable<'data, Elf, R>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Elf: FileHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Elf, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `ElfSymbolTable<'data, 'file, Elf, R>`

```rust
struct ElfSymbolTable<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
}
```

A symbol table in an [`ElfFile`](super::ElfFile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Clone for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-clone"></span>`fn clone(&self) -> ElfSymbolTable<'data, 'file, Elf, R>` — [`ElfSymbolTable`](#elfsymboltable)

##### `impl<'data, 'file, Elf, R> Copy for ElfSymbolTable<'data, 'file, Elf, R>`

##### `impl<'data, 'file, Elf, R> Debug for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-symbol"></span>`type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymboltable-symboliterator"></span>`type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- <span id="elfsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbolTable<'data, 'file, Elf, R>`

### `ElfSymbolIterator<'data, 'file, Elf, R>`

```rust
struct ElfSymbolIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in an [`ElfFile`](super::ElfFile).

#### Implementations

- <span id="elfsymboliterator-new"></span>`fn new(endian: <Elf as >::Endian, symbols: &'file SymbolTable<'data, Elf, R>) -> Self` — [`FileHeader`](#fileheader), [`SymbolTable`](#symboltable)

#### Trait Implementations

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Debug for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsymboliterator-intoiter"></span>`type IntoIter = I`

- <span id="elfsymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Iterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-item"></span>`type Item = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ElfSymbol<'data, 'file, Elf, R>`

```rust
struct ElfSymbol<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Elf as >::Sym,
}
```

A symbol in an [`ElfFile`](super::ElfFile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="elfsymbol-endian"></span>`fn endian(&self) -> <Elf as >::Endian` — [`FileHeader`](#fileheader)

- <span id="elfsymbol-raw-symbol"></span>`fn raw_symbol(&self) -> &'data <Elf as >::Sym` — [`FileHeader`](#fileheader)

- <span id="elfsymbol-elf-symbol"></span>`fn elf_symbol(&self) -> &'data <Elf as >::Sym` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Clone for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-clone"></span>`fn clone(&self) -> ElfSymbol<'data, 'file, Elf, R>` — [`ElfSymbol`](#elfsymbol)

##### `impl<'data, 'file, Elf, R> Copy for ElfSymbol<'data, 'file, Elf, R>`

##### `impl<'data, 'file, Elf, R> Debug for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> ObjectSymbol for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="elfsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="elfsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- <span id="elfsymbol-address"></span>`fn address(&self) -> u64`

- <span id="elfsymbol-size"></span>`fn size(&self) -> u64`

- <span id="elfsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md)

- <span id="elfsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md)

- <span id="elfsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="elfsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="elfsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="elfsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="elfsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md)

- <span id="elfsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="elfsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="elfsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md), [`SectionIndex`](../../index.md), [`SymbolIndex`](../../index.md)

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbol<'data, 'file, Elf, R>`

### `RelocationSections`

```rust
struct RelocationSections {
    relocations: alloc::vec::Vec<usize>,
}
```

A mapping from section index to associated relocation sections.

#### Implementations

- <span id="relocationsections-parse"></span>`fn parse<'data, Elf: FileHeader, R: ReadRef<'data>>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` — [`FileHeader`](#fileheader), [`SectionTable`](#sectiontable), [`SectionIndex`](../../index.md), [`Result`](../../index.md)

- <span id="relocationsections-get"></span>`fn get(&self, index: SectionIndex) -> Option<SectionIndex>` — [`SectionIndex`](../../index.md)

#### Trait Implementations

##### `impl Debug for RelocationSections`

- <span id="relocationsections-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationSections`

- <span id="relocationsections-default"></span>`fn default() -> RelocationSections` — [`RelocationSections`](#relocationsections)

### `ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

```rust
struct ElfDynamicRelocationIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    section_index: crate::read::SectionIndex,
    file: &'file super::ElfFile<'data, Elf, R>,
    relocations: Option<ElfRelocationIterator<'data, Elf>>,
}
```

An iterator for the dynamic relocations in an [`ElfFile`](#elffile).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current relocation section index.

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfdynamicrelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfdynamicrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-item"></span>`type Item = (u64, Relocation)`

- <span id="elfdynamicrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ElfSectionRelocationIterator<'data, 'file, Elf, R>`

```rust
struct ElfSectionRelocationIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    section_index: crate::read::SectionIndex,
    file: &'file super::ElfFile<'data, Elf, R>,
    relocations: Option<ElfRelocationIterator<'data, Elf>>,
}
```

An iterator for the relocations for an [`ElfSection`](super::ElfSection).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current pointer in the chain of relocation sections.

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectionrelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfsectionrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-item"></span>`type Item = (u64, Relocation)`

- <span id="elfsectionrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RelrIterator<'data, Elf: FileHeader>`

```rust
struct RelrIterator<'data, Elf: FileHeader> {
    offset: <Elf as >::Word,
    bits: <Elf as >::Word,
    count: u8,
    iter: slice::Iter<'data, <Elf as >::Relr>,
    endian: <Elf as >::Endian,
}
```

An iterator over the relative relocations in an ELF `SHT_RELR` section.

Returned by [`SectionHeader::relr`](super::SectionHeader::relr).

#### Implementations

- <span id="relriterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for RelrIterator<'data, Elf>`

- <span id="relriterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for RelrIterator<'data, Elf>`

- <span id="relriterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relriterator-intoiter"></span>`type IntoIter = I`

- <span id="relriterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for RelrIterator<'data, Elf>`

- <span id="relriterator-item"></span>`type Item = <Elf as FileHeader>::Word`

- <span id="relriterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Crel`

```rust
struct Crel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}
```

Compact relocation

The specification has been submited here: <https://groups.google.com/g/generic-abi/c/ppkaxtLb0P0/m/awgqZ_1CBAAJ>.

#### Fields

- **`r_offset`**: `u64`

  Relocation offset.

- **`r_sym`**: `u32`

  Relocation symbol index.

- **`r_type`**: `u32`

  Relocation type.

- **`r_addend`**: `i64`

  Relocation addend.
  
  Only set if `CrelIterator::is_rela()` returns `true`.

#### Implementations

- <span id="crel-symbol"></span>`fn symbol(&self) -> Option<SymbolIndex>` — [`SymbolIndex`](../../index.md)

- <span id="crel-from-rel"></span>`fn from_rel<R: Rel>(r: &R, endian: <R as >::Endian) -> Crel` — [`Rel`](#rel), [`Crel`](#crel)

- <span id="crel-from-rela"></span>`fn from_rela<R: Rela>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` — [`Rela`](#rela), [`Crel`](#crel)

#### Trait Implementations

##### `impl Clone for Crel`

- <span id="crel-clone"></span>`fn clone(&self) -> Crel` — [`Crel`](#crel)

##### `impl Copy for Crel`

##### `impl Debug for Crel`

- <span id="crel-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CrelIteratorHeader`

```rust
struct CrelIteratorHeader {
    count: usize,
    flag_bits: u64,
    shift: u64,
    is_rela: bool,
}
```

#### Fields

- **`count`**: `usize`

  The number of encoded relocations.

- **`flag_bits`**: `u64`

  The number of flag bits each relocation uses.

- **`shift`**: `u64`

  Shift of the relocation value.

- **`is_rela`**: `bool`

  True if the relocation format encodes addend.

#### Trait Implementations

##### `impl Clone for CrelIteratorHeader`

- <span id="creliteratorheader-clone"></span>`fn clone(&self) -> CrelIteratorHeader` — [`CrelIteratorHeader`](relocation/index.md)

##### `impl Debug for CrelIteratorHeader`

- <span id="creliteratorheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CrelIteratorState`

```rust
struct CrelIteratorState {
    index: usize,
    offset: u64,
    addend: i64,
    symidx: u32,
    typ: u32,
}
```

#### Fields

- **`index`**: `usize`

  Index of the current relocation.

- **`offset`**: `u64`

  Offset of the latest relocation.

- **`addend`**: `i64`

  Addend of the latest relocation.

- **`symidx`**: `u32`

  Symbol index of the latest relocation.

- **`typ`**: `u32`

  Type of the latest relocation.

#### Trait Implementations

##### `impl Clone for CrelIteratorState`

- <span id="creliteratorstate-clone"></span>`fn clone(&self) -> CrelIteratorState` — [`CrelIteratorState`](relocation/index.md)

##### `impl Debug for CrelIteratorState`

- <span id="creliteratorstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrelIteratorState`

- <span id="creliteratorstate-default"></span>`fn default() -> CrelIteratorState` — [`CrelIteratorState`](relocation/index.md)

### `CrelIterator<'data>`

```rust
struct CrelIterator<'data> {
    data: crate::read::Bytes<'data>,
    header: CrelIteratorHeader,
    state: CrelIteratorState,
}
```

Compact relocation iterator.

#### Fields

- **`data`**: `crate::read::Bytes<'data>`

  Input stream reader.

- **`header`**: `CrelIteratorHeader`

  Parsed header information.

- **`state`**: `CrelIteratorState`

  State of the iterator.

#### Implementations

- <span id="creliterator-new"></span>`fn new(data: &'data [u8]) -> Result<Self, Error>` — [`Error`](../../index.md)

- <span id="creliterator-is-rela"></span>`fn is_rela(&self) -> bool`

- <span id="creliterator-len"></span>`fn len(&self) -> usize`

- <span id="creliterator-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="creliterator-parse"></span>`fn parse(&mut self) -> read::Result<Crel>` — [`Result`](../../index.md), [`Crel`](#crel)

#### Trait Implementations

##### `impl<'data> Clone for CrelIterator<'data>`

- <span id="creliterator-clone"></span>`fn clone(&self) -> CrelIterator<'data>` — [`CrelIterator`](#creliterator)

##### `impl<'data> Debug for CrelIterator<'data>`

- <span id="creliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CrelIterator<'data>`

- <span id="creliterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="creliterator-intoiter"></span>`type IntoIter = I`

- <span id="creliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data> Iterator for CrelIterator<'data>`

- <span id="creliterator-item"></span>`type Item = Result<Crel, Error>`

- <span id="creliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="creliterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ElfComdatIterator<'data, 'file, Elf, R>`

```rust
struct ElfComdatIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Elf as >::SectionHeader>>,
}
```

An iterator for the COMDAT section groups in an [`ElfFile`](#elffile).

#### Implementations

- <span id="elfcomdatiterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> ElfComdatIterator<'data, 'file, Elf, R>` — [`ElfFile`](#elffile), [`ElfComdatIterator`](#elfcomdatiterator)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-item"></span>`type Item = ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ElfComdat<'data, 'file, Elf, R>`

```rust
struct ElfComdat<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    section: &'data <Elf as >::SectionHeader,
    sections: &'data [crate::endian::U32Bytes<<Elf as >::Endian>],
}
```

A COMDAT section group in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Implementations

- <span id="elfcomdat-parse"></span>`fn parse(file: &'file ElfFile<'data, Elf, R>, section: &'data <Elf as >::SectionHeader) -> Option<ElfComdat<'data, 'file, Elf, R>>` — [`ElfFile`](#elffile), [`FileHeader`](#fileheader), [`ElfComdat`](#elfcomdat)

- <span id="elfcomdat-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

- <span id="elfcomdat-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectComdat for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-sectioniterator"></span>`type SectionIterator = ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- <span id="elfcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- <span id="elfcomdat-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- <span id="elfcomdat-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- <span id="elfcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfComdat<'data, 'file, Elf, R>`

### `ElfComdatSectionIterator<'data, 'file, Elf, R>`

```rust
struct ElfComdatSectionIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    sections: slice::Iter<'data, crate::endian::U32Bytes<<Elf as >::Endian>>,
}
```

An iterator for the sections in a COMDAT section group in an [`ElfFile`](#elffile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatsectioniterator-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-item"></span>`type Item = SectionIndex`

- <span id="elfcomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NoteIterator<'data, Elf>`

```rust
struct NoteIterator<'data, Elf>
where
    Elf: FileHeader {
    endian: <Elf as >::Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

#### Implementations

- <span id="noteiterator-new"></span>`fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- <span id="noteiterator-next"></span>`fn next(&mut self) -> read::Result<Option<Note<'data, Elf>>>` — [`Result`](../../index.md), [`Note`](#note)

- <span id="noteiterator-parse"></span>`fn parse(&mut self) -> read::Result<Note<'data, Elf>>` — [`Result`](../../index.md), [`Note`](#note)

#### Trait Implementations

##### `impl<'data, Elf> Debug for NoteIterator<'data, Elf>`

- <span id="noteiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="noteiterator-intoiter"></span>`type IntoIter = I`

- <span id="noteiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-item"></span>`type Item = Result<Note<'data, Elf>, Error>`

- <span id="noteiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Note<'data, Elf>`

```rust
struct Note<'data, Elf>
where
    Elf: FileHeader {
    header: &'data <Elf as >::NoteHeader,
    name: &'data [u8],
    desc: &'data [u8],
}
```

A parsed [`NoteHeader`](#noteheader).

#### Implementations

- <span id="note-n-type"></span>`fn n_type(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

- <span id="note-n-namesz"></span>`fn n_namesz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

- <span id="note-n-descsz"></span>`fn n_descsz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

- <span id="note-name-bytes"></span>`fn name_bytes(&self) -> &'data [u8]`

- <span id="note-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="note-desc"></span>`fn desc(&self) -> &'data [u8]`

- <span id="note-gnu-properties"></span>`fn gnu_properties(&self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` — [`FileHeader`](#fileheader), [`GnuPropertyIterator`](#gnupropertyiterator)

#### Trait Implementations

##### `impl<'data, Elf> Debug for Note<'data, Elf>`

- <span id="note-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `GnuPropertyIterator<'data, Endian: endian::Endian>`

```rust
struct GnuPropertyIterator<'data, Endian: endian::Endian> {
    endian: Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md) note.

Returned by `Note::gnu_properties`.

#### Implementations

- <span id="gnupropertyiterator-next"></span>`fn next(&mut self) -> read::Result<Option<GnuProperty<'data>>>` — [`Result`](../../index.md), [`GnuProperty`](#gnuproperty)

- <span id="gnupropertyiterator-parse"></span>`fn parse(&mut self) -> read::Result<GnuProperty<'data>>` — [`Result`](../../index.md), [`GnuProperty`](#gnuproperty)

#### Trait Implementations

##### `impl<'data, Endian: fmt::Debug + endian::Endian> Debug for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="gnupropertyiterator-intoiter"></span>`type IntoIter = I`

- <span id="gnupropertyiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Endian: endian::Endian> Iterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-item"></span>`type Item = Result<GnuProperty<'data>, Error>`

- <span id="gnupropertyiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `GnuProperty<'data>`

```rust
struct GnuProperty<'data> {
    pr_type: u32,
    pr_data: &'data [u8],
}
```

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md) note.

#### Implementations

- <span id="gnuproperty-pr-type"></span>`fn pr_type(&self) -> u32`

- <span id="gnuproperty-pr-data"></span>`fn pr_data(&self) -> &'data [u8]`

- <span id="gnuproperty-data-u32"></span>`fn data_u32<E: endian::Endian>(&self, endian: E) -> read::Result<u32>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Debug for GnuProperty<'data>`

- <span id="gnuproperty-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HashTable<'data, Elf: FileHeader>`

```rust
struct HashTable<'data, Elf: FileHeader> {
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    chains: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A SysV symbol hash table in an ELF file.

Returned by [`SectionHeader::hash`](super::SectionHeader::hash).

#### Implementations

- <span id="hashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- <span id="hashtable-symbol-table-length"></span>`fn symbol_table_length(&self) -> u32`

- <span id="hashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- <span id="hashtable-chain"></span>`fn chain(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- <span id="hashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](#fileheader), [`Version`](#version), [`SymbolTable`](#symboltable), [`VersionTable`](#versiontable), [`SymbolIndex`](../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for HashTable<'data, Elf>`

- <span id="hashtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `GnuHashTable<'data, Elf: FileHeader>`

```rust
struct GnuHashTable<'data, Elf: FileHeader> {
    symbol_base: u32,
    bloom_shift: u32,
    bloom_filters: &'data [u8],
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    values: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A GNU symbol hash table in an ELF file.

Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash).

#### Implementations

- <span id="gnuhashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- <span id="gnuhashtable-symbol-base"></span>`fn symbol_base(&self) -> u32`

- <span id="gnuhashtable-symbol-table-length"></span>`fn symbol_table_length(&self, endian: <Elf as >::Endian) -> Option<u32>` — [`FileHeader`](#fileheader)

- <span id="gnuhashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md)

- <span id="gnuhashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](#fileheader), [`Version`](#version), [`SymbolTable`](#symboltable), [`VersionTable`](#versiontable), [`SymbolIndex`](../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

A version index.

#### Implementations

- <span id="versionindex-index"></span>`fn index(&self) -> u16`

- <span id="versionindex-is-local"></span>`fn is_local(&self) -> bool`

- <span id="versionindex-is-global"></span>`fn is_global(&self) -> bool`

- <span id="versionindex-is-hidden"></span>`fn is_hidden(&self) -> bool`

#### Trait Implementations

##### `impl Clone for VersionIndex`

- <span id="versionindex-clone"></span>`fn clone(&self) -> VersionIndex` — [`VersionIndex`](#versionindex)

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- <span id="versionindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VersionIndex`

- <span id="versionindex-default"></span>`fn default() -> VersionIndex` — [`VersionIndex`](#versionindex)

### `Version<'data>`

```rust
struct Version<'data> {
    name: &'data [u8],
    hash: u32,
    valid: bool,
    file: Option<&'data [u8]>,
}
```

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`](../../elf/index.md) and [`elf::SHT_GNU_VERNEED`](../../elf/index.md) sections.

#### Implementations

- <span id="version-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="version-hash"></span>`fn hash(&self) -> u32`

- <span id="version-file"></span>`fn file(&self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for Version<'data>`

- <span id="version-clone"></span>`fn clone(&self) -> Version<'data>` — [`Version`](#version)

##### `impl<'data> Copy for Version<'data>`

##### `impl<'data> Debug for Version<'data>`

- <span id="version-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Default for Version<'data>`

- <span id="version-default"></span>`fn default() -> Version<'data>` — [`Version`](#version)

### `VersionTable<'data, Elf: FileHeader>`

```rust
struct VersionTable<'data, Elf: FileHeader> {
    symbols: &'data [elf::Versym<<Elf as >::Endian>],
    versions: alloc::vec::Vec<Version<'data>>,
}
```

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`](../../elf/index.md), [`elf::SHT_GNU_VERDEF`](../../elf/index.md)
and [`elf::SHT_GNU_VERNEED`](../../elf/index.md) sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

#### Implementations

- <span id="versiontable-parse"></span>`fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](#fileheader), [`Versym`](../../elf/index.md), [`VerdefIterator`](#verdefiterator), [`VerneedIterator`](#verneediterator), [`StringTable`](../index.md), [`Result`](../../index.md)

- <span id="versiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="versiontable-version-index"></span>`fn version_index(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md), [`VersionIndex`](#versionindex)

- <span id="versiontable-version"></span>`fn version(&self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](#versionindex), [`Result`](../../index.md), [`Version`](#version)

- <span id="versiontable-matches"></span>`fn matches(&self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md), [`Version`](#version)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- <span id="versiontable-clone"></span>`fn clone(&self) -> VersionTable<'data, Elf>` — [`VersionTable`](#versiontable)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- <span id="versiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Elf: FileHeader> Default for VersionTable<'data, Elf>`

- <span id="versiontable-default"></span>`fn default() -> Self`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Implementations

- <span id="verdefiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](#fileheader)

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../index.md), [`Verdef`](../../elf/index.md), [`FileHeader`](#fileheader), [`VerdauxIterator`](#verdauxiterator)

- <span id="verdefiterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../index.md), [`Verdef`](../../elf/index.md), [`FileHeader`](#fileheader), [`VerdauxIterator`](#verdauxiterator)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clone"></span>`fn clone(&self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](#verdefiterator)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdefiterator-intoiter"></span>`type IntoIter = I`

- <span id="verdefiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-item"></span>`type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VerdauxIterator<'data, Elf: FileHeader>`

```rust
struct VerdauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Implementations

- <span id="verdauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](#fileheader)

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../index.md), [`Verdaux`](../../elf/index.md), [`FileHeader`](#fileheader)

- <span id="verdauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../index.md), [`Verdaux`](../../elf/index.md), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clone"></span>`fn clone(&self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](#verdauxiterator)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdauxiterator-intoiter"></span>`type IntoIter = I`

- <span id="verdauxiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-item"></span>`type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../elf/index.md) section.

#### Implementations

- <span id="verneediterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](#fileheader)

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../index.md), [`Verneed`](../../elf/index.md), [`FileHeader`](#fileheader), [`VernauxIterator`](#vernauxiterator)

- <span id="verneediterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../index.md), [`Verneed`](../../elf/index.md), [`FileHeader`](#fileheader), [`VernauxIterator`](#vernauxiterator)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clone"></span>`fn clone(&self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](#verneediterator)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- <span id="verneediterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verneediterator-intoiter"></span>`type IntoIter = I`

- <span id="verneediterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-item"></span>`type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VernauxIterator<'data, Elf: FileHeader>`

```rust
struct VernauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`](../../elf/index.md) section.

#### Implementations

- <span id="vernauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](#fileheader)

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../index.md), [`Vernaux`](../../elf/index.md), [`FileHeader`](#fileheader)

- <span id="vernauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../index.md), [`Vernaux`](../../elf/index.md), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clone"></span>`fn clone(&self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](#vernauxiterator)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="vernauxiterator-intoiter"></span>`type IntoIter = I`

- <span id="vernauxiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-item"></span>`type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributesSection<'data, Elf: FileHeader>`

```rust
struct AttributesSection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    version: u8,
    data: crate::read::Bytes<'data>,
}
```

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`](#attributessubsection).

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

#### Implementations

- <span id="attributessection-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md)

- <span id="attributessection-version"></span>`fn version(&self) -> u8`

- <span id="attributessection-subsections"></span>`fn subsections(&self) -> Result<AttributesSubsectionIterator<'data, Elf>>` — [`Result`](../../index.md), [`AttributesSubsectionIterator`](#attributessubsectioniterator)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for AttributesSection<'data, Elf>`

- <span id="attributessection-clone"></span>`fn clone(&self) -> AttributesSection<'data, Elf>` — [`AttributesSection`](#attributessection)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for AttributesSection<'data, Elf>`

- <span id="attributessection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributesSubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the subsections in an [`AttributesSection`](#attributessection).

#### Implementations

- <span id="attributessubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsection<'data, Elf>>>` — [`Result`](../../index.md), [`AttributesSubsection`](#attributessubsection)

- <span id="attributessubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsection<'data, Elf>>` — [`Result`](../../index.md), [`AttributesSubsection`](#attributessubsection)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsectionIterator<'data, Elf>` — [`AttributesSubsectionIterator`](#attributessubsectioniterator)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsectioniterator-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-item"></span>`type Item = Result<AttributesSubsection<'data, Elf>, Error>`

- <span id="attributessubsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributesSubsection<'data, Elf: FileHeader>`

```rust
struct AttributesSubsection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    length: u32,
    vendor: &'data [u8],
    data: crate::read::Bytes<'data>,
}
```

A subsection in an [`AttributesSection`](#attributessection).

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`](#attributessubsubsection).

#### Implementations

- <span id="attributessubsection-length"></span>`fn length(&self) -> u32`

- <span id="attributessubsection-vendor"></span>`fn vendor(&self) -> &'data [u8]`

- <span id="attributessubsection-subsubsections"></span>`fn subsubsections(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-clone"></span>`fn clone(&self) -> AttributesSubsection<'data, Elf>` — [`AttributesSubsection`](#attributessubsection)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributesSubsubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the sub-subsections in an [`AttributesSubsection`](#attributessubsection).

#### Implementations

- <span id="attributessubsubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsubsection<'data>>>` — [`Result`](../../index.md), [`AttributesSubsubsection`](#attributessubsubsection)

- <span id="attributessubsubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsubsection<'data>>` — [`Result`](../../index.md), [`AttributesSubsubsection`](#attributessubsubsection)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsubsectioniterator-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsubsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-item"></span>`type Item = Result<AttributesSubsubsection<'data>, Error>`

- <span id="attributessubsubsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    tag: u8,
    length: u32,
    indices: crate::read::Bytes<'data>,
    data: crate::read::Bytes<'data>,
}
```

A sub-subsection in an [`AttributesSubsection`](#attributessubsection).

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

#### Implementations

- <span id="attributessubsubsection-tag"></span>`fn tag(&self) -> u8`

- <span id="attributessubsubsection-length"></span>`fn length(&self) -> u32`

- <span id="attributessubsubsection-indices-data"></span>`fn indices_data(&self) -> &'data [u8]`

- <span id="attributessubsubsection-indices"></span>`fn indices(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](#attributeindexiterator)

- <span id="attributessubsubsection-attributes-data"></span>`fn attributes_data(&self) -> &'data [u8]`

- <span id="attributessubsubsection-attributes"></span>`fn attributes(&self) -> AttributeReader<'data>` — [`AttributeReader`](#attributereader)

#### Trait Implementations

##### `impl<'data> Clone for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clone"></span>`fn clone(&self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](#attributessubsubsection)

##### `impl<'data> Debug for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributeIndexIterator<'data>`

```rust
struct AttributeIndexIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

An iterator over the indices in an [`AttributesSubsubsection`](#attributessubsubsection).

#### Implementations

- <span id="attributeindexiterator-next"></span>`fn next(&mut self) -> Result<Option<u32>>` — [`Result`](../../index.md)

- <span id="attributeindexiterator-parse"></span>`fn parse(&mut self) -> Result<u32>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-clone"></span>`fn clone(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](#attributeindexiterator)

##### `impl<'data> Debug for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributeindexiterator-intoiter"></span>`type IntoIter = I`

- <span id="attributeindexiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data> Iterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-item"></span>`type Item = Result<u32, Error>`

- <span id="attributeindexiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributeReader<'data>`

```rust
struct AttributeReader<'data> {
    data: crate::read::Bytes<'data>,
}
```

A parser for the attributes in an [`AttributesSubsubsection`](#attributessubsubsection).

The parser relies on the caller to know the format of the data for each attribute tag.

#### Implementations

- <span id="attributereader-read-tag"></span>`fn read_tag(&mut self) -> Result<Option<u64>>` — [`Result`](../../index.md)

- <span id="attributereader-read-integer"></span>`fn read_integer(&mut self) -> Result<u64>` — [`Result`](../../index.md)

- <span id="attributereader-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributeReader<'data>`

- <span id="attributereader-clone"></span>`fn clone(&self) -> AttributeReader<'data>` — [`AttributeReader`](#attributereader)

##### `impl<'data> Debug for AttributeReader<'data>`

- <span id="attributereader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `ElfRelocationIterator<'data, Elf: FileHeader>`

```rust
enum ElfRelocationIterator<'data, Elf: FileHeader> {
    Rel(slice::Iter<'data, <Elf as >::Rel>, <Elf as >::Endian),
    Rela(slice::Iter<'data, <Elf as >::Rela>, <Elf as >::Endian, bool),
    Crel(CrelIterator<'data>),
}
```

#### Implementations

- <span id="elfrelocationiterator-is-rel"></span>`fn is_rel(&self) -> bool`

#### Trait Implementations

##### `impl<I> IntoIterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfrelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-item"></span>`type Item = Crel`

- <span id="elfrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::FileHeader32`](../../elf/index.md) and [`elf::FileHeader64`](../../elf/index.md).

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

- [`FileHeader32`](../../elf/index.md)
- [`FileHeader64`](../../elf/index.md)

### `ProgramHeader`

```rust
trait ProgramHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::ProgramHeader32`](../../elf/index.md) and [`elf::ProgramHeader64`](../../elf/index.md).

#### Associated Types

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn p_type(&self, endian: <Self as >::Endian) -> u32`

- `fn p_flags(&self, endian: <Self as >::Endian) -> u32`

- `fn p_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_vaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_paddr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_filesz(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_memsz(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn p_align(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn file_range(&self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [u8], ()>`

  Return the segment data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [T], ()>`

  Return the segment data as a slice of the given type.

- `fn data_range<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, address: u64, size: u64) -> Result<Option<&'data [u8]>, ()>`

  Return the segment data in the given virtual address range

- `fn dynamic<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [<<Self as >::Elf as FileHeader>::Dyn]>>`

  Return entries in a dynamic segment.

- `fn interpreter<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data [u8]>>`

  Return the data in an interpreter segment.

- `fn notes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the segment data.

#### Implementors

- [`ProgramHeader32`](../../elf/index.md)
- [`ProgramHeader64`](../../elf/index.md)

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::SectionHeader32`](../../elf/index.md) and [`elf::SectionHeader64`](../../elf/index.md).

#### Associated Types

- `type Elf: 1`

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn sh_name(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_type(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_flags(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_addr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_link(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_info(&self, endian: <Self as >::Endian) -> u32`

- `fn sh_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn sh_entsize(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the section name from the string table.

- `fn link(&self, endian: <Self as >::Endian) -> SectionIndex`

  Get the `sh_link` field as a section index.

- `fn has_info_link(&self, endian: <Self as >::Endian) -> bool`

  Return true if the `SHF_INFO_LINK` flag is set.

- `fn info_link(&self, endian: <Self as >::Endian) -> SectionIndex`

  Get the `sh_info` field as a section index.

- `fn file_range(&self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [u8]>`

  Return the section data.

- `fn data_as_array<'data, T: Pod, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<&'data [T]>`

  Return the section data as a slice of the given type.

- `fn strings<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<StringTable<'data, R>>>`

  Return the strings in the section.

- `fn symbols<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, sections: &SectionTable<'data, <Self as >::Elf, R>, section_index: SectionIndex) -> read::Result<Option<SymbolTable<'data, <Self as >::Elf, R>>>`

  Return the symbols in the section.

- `fn rel<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Rel], SectionIndex)>>`

  Return the `Elf::Rel` entries in the section.

- `fn rela<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Rela], SectionIndex)>>`

  Return the `Elf::Rela` entries in the section.

- `fn relr<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<RelrIterator<'data, <Self as >::Elf>>>`

  Return the `Elf::Relr` entries in the section.

- `fn crel<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(CrelIterator<'data>, SectionIndex)>>`

  Return the `Crel` entries in the section.

- `fn dynamic<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [<<Self as >::Elf as FileHeader>::Dyn], SectionIndex)>>`

  Return entries in a dynamic section.

- `fn notes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<NoteIterator<'data, <Self as >::Elf>>>`

  Return a note iterator for the section data.

- `fn group<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(u32, &'data [U32Bytes<<Self as >::Endian>])>>`

  Return the contents of a group section.

- `fn hash_header<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Self as >::Endian>>>`

  Return the header of a SysV hash section.

- `fn hash<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, <Self as >::Elf>, SectionIndex)>>`

  Return the contents of a SysV hash section.

- `fn gnu_hash_header<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Self as >::Endian>>>`

  Return the header of a GNU hash section.

- `fn gnu_hash<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, <Self as >::Elf>, SectionIndex)>>`

  Return the contents of a GNU hash section.

- `fn gnu_versym<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Self as >::Endian>], SectionIndex)>>`

  Return the contents of a `SHT_GNU_VERSYM` section.

- `fn gnu_verdef<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, <Self as >::Elf>, SectionIndex)>>`

  Return an iterator for the entries of a `SHT_GNU_VERDEF` section.

- `fn gnu_verneed<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, <Self as >::Elf>, SectionIndex)>>`

  Return an iterator for the entries of a `SHT_GNU_VERNEED` section.

- `fn gnu_attributes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<AttributesSection<'data, <Self as >::Elf>>>`

  Return the contents of a `SHT_GNU_ATTRIBUTES` section.

- `fn attributes<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<AttributesSection<'data, <Self as >::Elf>>`

  Parse the contents of the section as attributes.

- `fn compression<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> read::Result<Option<(&'data <<Self as >::Elf as FileHeader>::CompressionHeader, u64, u64)>>`

  Parse the compression header if present.

#### Implementors

- [`SectionHeader32`](../../elf/index.md)
- [`SectionHeader64`](../../elf/index.md)

### `Sym`

```rust
trait Sym: Debug + Pod { ... }
```

A trait for generic access to [`elf::Sym32`](../../elf/index.md) and [`elf::Sym64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn st_name(&self, endian: <Self as >::Endian) -> u32`

- `fn st_info(&self) -> u8`

- `fn st_bind(&self) -> u8`

- `fn st_type(&self) -> u8`

- `fn st_other(&self) -> u8`

- `fn st_visibility(&self) -> u8`

- `fn st_shndx(&self, endian: <Self as >::Endian) -> u16`

- `fn st_value(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn st_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the symbol name from the string table.

- `fn is_undefined(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_UNDEF`.

- `fn is_definition(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn is_common(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_COMMON`.

- `fn is_absolute(&self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_ABS`.

- `fn is_local(&self) -> bool`

  Return true if the symbol binding is `STB_LOCAL`.

- `fn is_weak(&self) -> bool`

  Return true if the symbol binding is `STB_WEAK`.

#### Implementors

- [`Sym32`](../../elf/index.md)
- [`Sym64`](../../elf/index.md)

### `Rel`

```rust
trait Rel: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Rel32`](../../elf/index.md) and [`elf::Rel64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

#### Required Methods

- `fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(&self, endian: <Self as >::Endian) -> u32`

- `fn r_type(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn symbol(&self, endian: <Self as >::Endian) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

#### Implementors

- [`Rel32`](../../elf/index.md)
- [`Rel64`](../../elf/index.md)

### `Rela`

```rust
trait Rela: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Rela32`](../../elf/index.md) and [`elf::Rela64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

#### Required Methods

- `fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(&self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(&self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn r_type(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

#### Provided Methods

- `fn symbol(&self, endian: <Self as >::Endian, is_mips64el: bool) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

#### Implementors

- [`Rela32`](../../elf/index.md)
- [`Rela64`](../../elf/index.md)

### `Relr`

```rust
trait Relr: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Relr32`](../../elf/index.md) and [`elf::Relr64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Associated Constants

- `const COUNT: u8`

#### Required Methods

- `fn get(&self, endian: <Self as >::Endian) -> <Self as >::Word`

  Get the relocation entry.

- `fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>`

  Return the offset corresponding to the next bit in the bit mask.

#### Implementors

- [`Relr32`](../../elf/index.md)
- [`Relr64`](../../elf/index.md)

### `Dyn`

```rust
trait Dyn: Debug + Pod { ... }
```

A trait for generic access to [`elf::Dyn32`](../../elf/index.md) and [`elf::Dyn64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn d_tag(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn d_val(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn tag32(&self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the tag to a `u32`.

- `fn val32(&self, endian: <Self as >::Endian) -> Option<u32>`

  Try to convert the value to a `u32`.

- `fn is_string(&self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an offset in the dynamic string table.

- `fn string<'data>(&self, endian: <Self as >::Endian, strings: StringTable<'data>) -> Result<&'data [u8]>`

  Use the value to get a string in a string table.

- `fn is_address(&self, endian: <Self as >::Endian) -> bool`

  Return true if the value is an address.

#### Implementors

- [`Dyn32`](../../elf/index.md)
- [`Dyn64`](../../elf/index.md)

### `CompressionHeader`

```rust
trait CompressionHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::CompressionHeader32`](../../elf/index.md) and [`elf::CompressionHeader64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn ch_type(&self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Implementors

- [`CompressionHeader32`](../../elf/index.md)
- [`CompressionHeader64`](../../elf/index.md)

### `NoteHeader`

```rust
trait NoteHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::NoteHeader32`](../../elf/index.md) and [`elf::NoteHeader64`](../../elf/index.md).

#### Associated Types

- `type Endian: 1`

#### Required Methods

- `fn n_namesz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_type(&self, endian: <Self as >::Endian) -> u32`

#### Implementors

- [`NoteHeader32`](../../elf/index.md)
- [`NoteHeader64`](../../elf/index.md)

## Functions

### `parse_relocation`

```rust
fn parse_relocation<Elf: FileHeader>(header: &Elf, endian: <Elf as >::Endian, reloc: Crel, implicit_addend: bool) -> crate::read::Relocation
```

## Type Aliases

### `ElfFile32<'data, Endian, R>`

```rust
type ElfFile32<'data, Endian, R> = ElfFile<'data, elf::FileHeader32<Endian>, R>;
```

A 32-bit ELF object file.

This is a file that starts with [`elf::FileHeader32`](../../elf/index.md), and corresponds
to [`crate::FileKind::Elf32`](../../index.md).

### `ElfFile64<'data, Endian, R>`

```rust
type ElfFile64<'data, Endian, R> = ElfFile<'data, elf::FileHeader64<Endian>, R>;
```

A 64-bit ELF object file.

This is a file that starts with [`elf::FileHeader64`](../../elf/index.md), and corresponds
to [`crate::FileKind::Elf64`](../../index.md).

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

### `ElfSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator32<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the sections in an [`ElfFile32`](super::ElfFile32).

### `ElfSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator64<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the sections in an [`ElfFile64`](super::ElfFile64).

### `ElfSection32<'data, 'file, Endian, R>`

```rust
type ElfSection32<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A section in an [`ElfFile32`](super::ElfFile32).

### `ElfSection64<'data, 'file, Endian, R>`

```rust
type ElfSection64<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A section in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbolTable32<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable32<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolTable64<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable64<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator32<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator32<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator64<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator64<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbol32<'data, 'file, Endian, R>`

```rust
type ElfSymbol32<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbol64<'data, 'file, Endian, R>`

```rust
type ElfSymbol64<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol in an [`ElfFile64`](super::ElfFile64).

### `ElfDynamicRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator32<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).

### `ElfDynamicRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator64<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).

### `ElfSectionRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator32<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).

### `ElfSectionRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator64<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).

### `ElfComdatIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator32<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator64<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).

### `ElfComdat32<'data, 'file, Endian, R>`

```rust
type ElfComdat32<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdat64<'data, 'file, Endian, R>`

```rust
type ElfComdat64<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A COMDAT section group in an [`ElfFile64`](super::ElfFile64).

### `ElfComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator32<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator64<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).

