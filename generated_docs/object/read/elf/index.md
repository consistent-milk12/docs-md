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
  - [`parse_relocation`](#parse-relocation)
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
| [`parse_relocation`](#parse-relocation) | fn |  |
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

*Defined in [`object-0.37.3/src/read/elf/file.rs:38-51`](../../../../.source_1765521767/object-0.37.3/src/read/elf/file.rs#L38-L51)*

A partially parsed ELF file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="elffile-parse"></span>`fn parse(data: R) -> read::Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw ELF file data.

- <span id="elffile-endian"></span>`fn endian(&self) -> <Elf as >::Endian` — [`FileHeader`](#fileheader)

  Returns the endianness.

- <span id="elffile-data"></span>`fn data(&self) -> R`

  Returns the raw data.

- <span id="elffile-raw-header"></span>`fn raw_header(&self) -> &'data Elf`

  Returns the raw ELF file header.

- <span id="elffile-raw-segments"></span>`fn raw_segments(&self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](#fileheader)

  Returns the raw ELF segments.

- <span id="elffile-elf-header"></span>`fn elf_header(&self) -> &'data Elf`

  Get the raw ELF file header.

- <span id="elffile-elf-program-headers"></span>`fn elf_program_headers(&self) -> &'data [<Elf as >::ProgramHeader]` — [`FileHeader`](#fileheader)

  Get the raw ELF program headers.

  

  Returns an empty slice if the file has no program headers.

- <span id="elffile-elf-section-table"></span>`fn elf_section_table(&self) -> &SectionTable<'data, Elf, R>` — [`SectionTable`](#sectiontable)

  Get the ELF section table.

  

  Returns an empty section table if the file has no section headers.

- <span id="elffile-elf-symbol-table"></span>`fn elf_symbol_table(&self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

  Get the ELF symbol table.

  

  Returns an empty symbol table if the file has no symbol table.

- <span id="elffile-elf-dynamic-symbol-table"></span>`fn elf_dynamic_symbol_table(&self) -> &SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

  Get the ELF dynamic symbol table.

  

  Returns an empty symbol table if the file has no dynamic symbol table.

- <span id="elffile-elf-relocation-sections"></span>`fn elf_relocation_sections(&self) -> &RelocationSections` — [`RelocationSections`](#relocationsections)

  Get a mapping for linked relocation sections.

- <span id="elffile-raw-section-by-name"></span>`fn raw_section_by_name<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

- <span id="elffile-zdebug-section-by-name"></span>`fn zdebug_section_by_name<'file>(self: &'file Self, _section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

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

- <span id="elffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="elffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="elffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="elffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="elffile-object-segments"></span>`fn segments(&self) -> ElfSegmentIterator<'data, '_, Elf, R>` — [`ElfSegmentIterator`](#elfsegmentiterator)

- <span id="elffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<ElfSection<'data, 'file, Elf, R>>` — [`ElfSection`](#elfsection)

- <span id="elffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> read::Result<ElfSection<'data, '_, Elf, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`ElfSection`](#elfsection)

- <span id="elffile-object-sections"></span>`fn sections(&self) -> ElfSectionIterator<'data, '_, Elf, R>` — [`ElfSectionIterator`](#elfsectioniterator)

- <span id="elffile-object-comdats"></span>`fn comdats(&self) -> ElfComdatIterator<'data, '_, Elf, R>` — [`ElfComdatIterator`](#elfcomdatiterator)

- <span id="elffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<ElfSymbol<'data, '_, Elf, R>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ElfSymbol`](#elfsymbol)

- <span id="elffile-object-symbols"></span>`fn symbols(&self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](#elfsymboliterator)

- <span id="elffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](#elfsymboltable)

- <span id="elffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> ElfSymbolIterator<'data, '_, Elf, R>` — [`ElfSymbolIterator`](#elfsymboliterator)

- <span id="elffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<ElfSymbolTable<'data, '_, Elf, R>>` — [`ElfSymbolTable`](#elfsymboltable)

- <span id="elffile-object-dynamic-relocations"></span>`fn dynamic_relocations<'file>(self: &'file Self) -> Option<ElfDynamicRelocationIterator<'data, 'file, Elf, R>>` — [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator)

- <span id="elffile-object-imports"></span>`fn imports(&self) -> read::Result<Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="elffile-object-exports"></span>`fn exports(&self) -> read::Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="elffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="elffile-object-build-id"></span>`fn build_id(&self) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="elffile-object-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> read::Result<Option<(&'data [u8], u32)>>` — [`Result`](../../index.md#result)

- <span id="elffile-object-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> read::Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../index.md#result)

- <span id="elffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="elffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="elffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<Elf, R> Sealed for ElfFile<'data, Elf, R>`

##### `impl<U> TryFrom for ElfFile<'data, Elf, R>`

- <span id="elffile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elffile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfFile<'data, Elf, R>`

- <span id="elffile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elffile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/segment.rs:20-27`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L20-L27)*

An iterator for the segments in an [`ElfFile`](#elffile).

#### Trait Implementations

##### `impl Any for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-iterator-type-item"></span>`type Item = ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSegmentIterator<'data, 'file, Elf, R>`

- <span id="elfsegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/segment.rs:60-67`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L60-L67)*

A segment in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Implementations

- <span id="elfsegment-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

  Get the ELF file containing this segment.

- <span id="elfsegment-elf-program-header"></span>`fn elf_program_header(&self) -> &'data <Elf as >::ProgramHeader` — [`FileHeader`](#fileheader)

  Get the raw ELF program header for the segment.

- <span id="elfsegment-bytes"></span>`fn bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl Any for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf, R> ObjectSegment for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="elfsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="elfsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="elfsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="elfsegment-objectsegment-data"></span>`fn data(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="elfsegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="elfsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> read::Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="elfsegment-objectsegment-name"></span>`fn name(&self) -> read::Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="elfsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<Elf, R> Sealed for ElfSegment<'data, 'file, Elf, R>`

##### `impl<U> TryFrom for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSegment<'data, 'file, Elf, R>`

- <span id="elfsegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionTable<'data, Elf: FileHeader, R>`

```rust
struct SectionTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    sections: &'data [<Elf as >::SectionHeader],
    strings: crate::read::StringTable<'data, R>,
}
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:25-31`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L25-L31)*

The table of section headers in an ELF file.

Also includes the string table used for the section names.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-new"></span>`fn new(sections: &'data [<Elf as >::SectionHeader], strings: StringTable<'data, R>) -> Self` — [`FileHeader`](#fileheader), [`StringTable`](../index.md#stringtable)

  Create a new section table.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Elf as >::SectionHeader>` — [`FileHeader`](#fileheader)

  Iterate over the section headers.

  

  This includes the null section at index 0, which you will usually need to skip.

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data <Elf as >::SectionHeader)>` — [`SectionIndex`](../../index.md#sectionindex), [`FileHeader`](#fileheader)

  Iterate over the section headers and their indices.

  

  This includes the null section at index 0, which you will usually need to skip.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Elf as >::SectionHeader>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the section header at the given index.

  

  Returns an error for the null section at index 0.

- <span id="sectiontable-section-by-name"></span>`fn section_by_name(&self, endian: <Elf as >::Endian, name: &[u8]) -> Option<(SectionIndex, &'data <Elf as >::SectionHeader)>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md#sectionindex)

  Return the section header with the given name.

  

  Ignores sections with invalid names.

- <span id="sectiontable-section-name"></span>`fn section_name(&self, endian: <Elf as >::Endian, section: &<Elf as >::SectionHeader) -> read::Result<&'data [u8]>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result)

  Return the section name for the given section header.

- <span id="sectiontable-strings"></span>`fn strings(&self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<StringTable<'data, R>>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`StringTable`](../index.md#stringtable)

  Return the string table at the given section index.

  

  Returns an empty string table if the index is 0.

  Returns an error if the section is not a string table.

- <span id="sectiontable-symbols"></span>`fn symbols(&self, endian: <Elf as >::Endian, data: R, sh_type: u32) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`SymbolTable`](#symboltable)

  Return the symbol table of the given section type.

  

  Returns an empty symbol table if the symbol table does not exist.

- <span id="sectiontable-symbol-table-by-index"></span>`fn symbol_table_by_index(&self, endian: <Elf as >::Endian, data: R, index: SectionIndex) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`SymbolTable`](#symboltable)

  Return the symbol table at the given section index.

  

  Returns an error if the section is not a symbol table.

- <span id="sectiontable-relocation-sections"></span>`fn relocation_sections(&self, endian: <Elf as >::Endian, symbol_section: SectionIndex) -> read::Result<RelocationSections>` — [`FileHeader`](#fileheader), [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`RelocationSections`](#relocationsections)

  Create a mapping from section index to associated relocation sections.

- <span id="sectiontable-dynamic"></span>`fn dynamic(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [<Elf as >::Dyn], SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`SectionIndex`](../../index.md#sectionindex)

  Return the contents of a dynamic section.

  

  Also returns the linked string table index.

  

  Returns `Ok(None)` if there is no `SHT_DYNAMIC` section.

  Returns `Err` for invalid values.

- <span id="sectiontable-hash-header"></span>`fn hash_header(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::HashHeader<<Elf as >::Endian>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`HashHeader`](../../elf/index.md#hashheader)

  Return the header of a SysV hash section.

  

  Returns `Ok(None)` if there is no SysV GNU hash section.

  Returns `Err` for invalid values.

- <span id="sectiontable-hash"></span>`fn hash(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(HashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`HashTable`](#hashtable), [`SectionIndex`](../../index.md#sectionindex)

  Return the contents of a SysV hash section.

  

  Also returns the linked symbol table index.

  

  Returns `Ok(None)` if there is no SysV hash section.

  Returns `Err` for invalid values.

- <span id="sectiontable-gnu-hash-header"></span>`fn gnu_hash_header(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<&'data elf::GnuHashHeader<<Elf as >::Endian>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`GnuHashHeader`](../../elf/index.md#gnuhashheader)

  Return the header of a GNU hash section.

  

  Returns `Ok(None)` if there is no GNU hash section.

  Returns `Err` for invalid values.

- <span id="sectiontable-gnu-hash"></span>`fn gnu_hash(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(GnuHashTable<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`GnuHashTable`](#gnuhashtable), [`SectionIndex`](../../index.md#sectionindex)

  Return the contents of a GNU hash section.

  

  Also returns the linked symbol table index.

  

  Returns `Ok(None)` if there is no GNU hash section.

  Returns `Err` for invalid values.

- <span id="sectiontable-gnu-versym"></span>`fn gnu_versym(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(&'data [elf::Versym<<Elf as >::Endian>], SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`Versym`](../../elf/index.md#versym), [`SectionIndex`](../../index.md#sectionindex)

  Return the contents of a `SHT_GNU_VERSYM` section.

  

  Also returns the linked symbol table index.

  

  Returns `Ok(None)` if there is no `SHT_GNU_VERSYM` section.

  Returns `Err` for invalid values.

- <span id="sectiontable-gnu-verdef"></span>`fn gnu_verdef(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerdefIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`VerdefIterator`](#verdefiterator), [`SectionIndex`](../../index.md#sectionindex)

  Return the contents of a `SHT_GNU_VERDEF` section.

  

  Also returns the linked string table index.

  

  Returns `Ok(None)` if there is no `SHT_GNU_VERDEF` section.

  Returns `Err` for invalid values.

- <span id="sectiontable-gnu-verneed"></span>`fn gnu_verneed(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<(VerneedIterator<'data, Elf>, SectionIndex)>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`VerneedIterator`](#verneediterator), [`SectionIndex`](../../index.md#sectionindex)

  Return the contents of a `SHT_GNU_VERNEED` section.

  

  Also returns the linked string table index.

  

  Returns `Ok(None)` if there is no `SHT_GNU_VERNEED` section.

  Returns `Err` for invalid values.

- <span id="sectiontable-versions"></span>`fn versions(&self, endian: <Elf as >::Endian, data: R) -> read::Result<Option<VersionTable<'data, Elf>>>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result), [`VersionTable`](#versiontable)

  Returns the symbol version table.

  

  Returns `Ok(None)` if there is no `SHT_GNU_VERSYM` section.

  Returns `Err` for invalid values.

#### Trait Implementations

##### `impl Any for SectionTable<'data, Elf, R>`

- <span id="sectiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionTable<'data, Elf, R>`

- <span id="sectiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionTable<'data, Elf, R>`

- <span id="sectiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader, R> Clone for SectionTable<'data, Elf, R>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Elf, R>` — [`SectionTable`](#sectiontable)

##### `impl CloneToUninit for SectionTable<'data, Elf, R>`

- <span id="sectiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: marker::Copy + FileHeader, R> Copy for SectionTable<'data, Elf, R>`

##### `impl<Elf: fmt::Debug + FileHeader, R> Debug for SectionTable<'data, Elf, R>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Default for SectionTable<'data, Elf, R>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

##### `impl<T> From for SectionTable<'data, Elf, R>`

- <span id="sectiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionTable<'data, Elf, R>`

- <span id="sectiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SectionTable<'data, Elf, R>`

- <span id="sectiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="sectiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionTable<'data, Elf, R>`

- <span id="sectiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionTable<'data, Elf, R>`

- <span id="sectiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/section.rs:358-365`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L358-L365)*

An iterator for the sections in an [`ElfFile`](#elffile).

#### Implementations

- <span id="elfsectioniterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> Self` — [`ElfFile`](#elffile)

#### Trait Implementations

##### `impl Any for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-iterator-type-item"></span>`type Item = ElfSection<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSectionIterator<'data, 'file, Elf, R>`

- <span id="elfsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/section.rs:406-414`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L406-L414)*

A section in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="elfsection-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

  Get the ELF file containing this section.

- <span id="elfsection-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](#fileheader)

  Get the raw ELF section header.

- <span id="elfsection-elf-relocation-section-index"></span>`fn elf_relocation_section_index(&self) -> read::Result<Option<SectionIndex>>` — [`Result`](../../index.md#result), [`SectionIndex`](../../index.md#sectionindex)

  Get the index of the relocation section that references this section.

  

  Returns `None` if there are no relocations.

  Returns an error if there are multiple relocation sections that reference this section.

- <span id="elfsection-elf-relocation-section"></span>`fn elf_relocation_section(&self) -> read::Result<Option<&'data <Elf as >::SectionHeader>>` — [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the relocation section that references this section.

  

  Returns `None` if there are no relocations.

  Returns an error if there are multiple relocation sections that reference this section.

- <span id="elfsection-elf-linked-rel"></span>`fn elf_linked_rel(&self) -> read::Result<&'data [<Elf as >::Rel]>` — [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the `Elf::Rel` entries that apply to this section.

  

  Returns an empty slice if there are no relocations.

  Returns an error if there are multiple relocation sections that reference this section.

- <span id="elfsection-elf-linked-rela"></span>`fn elf_linked_rela(&self) -> read::Result<&'data [<Elf as >::Rela]>` — [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the `Elf::Rela` entries that apply to this section.

  

  Returns an empty slice if there are no relocations.

  Returns an error if there are multiple relocation sections that reference this section.

- <span id="elfsection-bytes"></span>`fn bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="elfsection-maybe-compressed"></span>`fn maybe_compressed(&self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="elfsection-maybe-compressed-gnu"></span>`fn maybe_compressed_gnu(&self) -> read::Result<Option<CompressedFileRange>>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

#### Trait Implementations

##### `impl Any for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf, R> ObjectSection for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="elfsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="elfsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="elfsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="elfsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="elfsection-objectsection-data"></span>`fn data(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="elfsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> read::Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="elfsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> read::Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="elfsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> read::Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="elfsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="elfsection-objectsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="elfsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> read::Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="elfsection-objectsection-segment-name"></span>`fn segment_name(&self) -> read::Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="elfsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="elfsection-objectsection-relocations"></span>`fn relocations(&self) -> ElfSectionRelocationIterator<'data, 'file, Elf, R>` — [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator)

- <span id="elfsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="elfsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

##### `impl<Elf, R> Sealed for ElfSection<'data, 'file, Elf, R>`

##### `impl<U> TryFrom for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSection<'data, 'file, Elf, R>`

- <span id="elfsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:24-34`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L24-L34)*

A table of symbol entries in an ELF file.

Also includes the string table used for the symbol names.

Returned by `SectionTable::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: R, sections: &SectionTable<'data, Elf, R>, section_index: SectionIndex, section: &<Elf as >::SectionHeader) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](#fileheader), [`SectionTable`](#sectiontable), [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`SymbolTable`](#symboltable)

  Parse the given symbol table section.

- <span id="symboltable-section"></span>`fn section(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

  Return the section index of this symbol table.

- <span id="symboltable-shndx-section"></span>`fn shndx_section(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

  Return the section index of the shndx table.

- <span id="symboltable-string-section"></span>`fn string_section(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

  Return the section index of the linked string table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-symbols"></span>`fn symbols(&self) -> &'data [<Elf as >::Sym]` — [`FileHeader`](#fileheader)

  Return the symbol table.

- <span id="symboltable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Elf as >::Sym>` — [`FileHeader`](#fileheader)

  Iterate over the symbols.

  

  This includes the null symbol at index 0, which you will usually need to skip.

- <span id="symboltable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SymbolIndex, &'data <Elf as >::Sym)>` — [`SymbolIndex`](../../index.md#symbolindex), [`FileHeader`](#fileheader)

  Iterate over the symbols and their indices.

  

  This includes the null symbol at index 0, which you will usually need to skip.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbols.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> read::Result<&'data <Elf as >::Sym>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the symbol at the given index.

  

  Returns an error for null entry at index 0.

- <span id="symboltable-shndx"></span>`fn shndx(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> Option<u32>` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex)

  Return the extended section index for the given symbol if present.

- <span id="symboltable-symbol-section"></span>`fn symbol_section(&self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym, index: SymbolIndex) -> read::Result<Option<SectionIndex>>` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`SectionIndex`](../../index.md#sectionindex)

  Return the section index for the given symbol.

  

  This uses the extended section index if present.

- <span id="symboltable-symbol-name"></span>`fn symbol_name(&self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym) -> read::Result<&'data [u8]>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result)

  Return the symbol name for the given symbol.

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Elf as >::Sym) -> Option<Entry>>(&self, endian: <Elf as >::Endian, f: F) -> SymbolMap<Entry>` — [`FileHeader`](#fileheader), [`SymbolMap`](../../index.md#symbolmap)

  Construct a map from addresses to a user-defined map entry.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, Elf, R>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, Elf, R>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, Elf, R>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader, R> Clone for SymbolTable<'data, Elf, R>`

- <span id="symboltable-clone"></span>`fn clone(&self) -> SymbolTable<'data, Elf, R>` — [`SymbolTable`](#symboltable)

##### `impl CloneToUninit for SymbolTable<'data, Elf, R>`

- <span id="symboltable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: marker::Copy + FileHeader, R> Copy for SymbolTable<'data, Elf, R>`

##### `impl<Elf: fmt::Debug + FileHeader, R> Debug for SymbolTable<'data, Elf, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Elf, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

##### `impl<T> From for SymbolTable<'data, Elf, R>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, Elf, R>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SymbolTable<'data, Elf, R>`

- <span id="symboltable-toowned-type-owned"></span>`type Owned = T`

- <span id="symboltable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symboltable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolTable<'data, Elf, R>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, Elf, R>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:229-236`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L229-L236)*

A symbol table in an [`ElfFile`](super::ElfFile).

#### Trait Implementations

##### `impl Any for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Clone for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-clone"></span>`fn clone(&self) -> ElfSymbolTable<'data, 'file, Elf, R>` — [`ElfSymbolTable`](#elfsymboltable)

##### `impl CloneToUninit for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf, R> Copy for ElfSymbolTable<'data, 'file, Elf, R>`

##### `impl<Elf, R> Debug for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="elfsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbolTable<'data, 'file, Elf, R>`

##### `impl ToOwned for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-toowned-type-owned"></span>`type Owned = T`

- <span id="elfsymboltable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="elfsymboltable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsymboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSymbolTable<'data, 'file, Elf, R>`

- <span id="elfsymboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsymboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:272-280`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L272-L280)*

An iterator for the symbols in an [`ElfFile`](super::ElfFile).

#### Implementations

- <span id="elfsymboliterator-new"></span>`fn new(endian: <Elf as >::Endian, symbols: &'file SymbolTable<'data, Elf, R>) -> Self` — [`FileHeader`](#fileheader), [`SymbolTable`](#symboltable)

#### Trait Implementations

##### `impl Any for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Debug for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Iterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-iterator-type-item"></span>`type Item = ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsymboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSymbolIterator<'data, 'file, Elf, R>`

- <span id="elfsymboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsymboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:333-342`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L333-L342)*

A symbol in an [`ElfFile`](super::ElfFile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="elfsymbol-endian"></span>`fn endian(&self) -> <Elf as >::Endian` — [`FileHeader`](#fileheader)

  Get the endianness of the ELF file.

- <span id="elfsymbol-raw-symbol"></span>`fn raw_symbol(&self) -> &'data <Elf as >::Sym` — [`FileHeader`](#fileheader)

  Return a reference to the raw symbol structure.

- <span id="elfsymbol-elf-symbol"></span>`fn elf_symbol(&self) -> &'data <Elf as >::Sym` — [`FileHeader`](#fileheader)

  Get the raw ELF symbol structure.

#### Trait Implementations

##### `impl Any for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Clone for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-clone"></span>`fn clone(&self) -> ElfSymbol<'data, 'file, Elf, R>` — [`ElfSymbol`](#elfsymbol)

##### `impl CloneToUninit for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf, R> Copy for ElfSymbol<'data, 'file, Elf, R>`

##### `impl<Elf, R> Debug for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf: FileHeader, R: ReadRef<'data>> ObjectSymbol for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="elfsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="elfsymbol-objectsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="elfsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="elfsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="elfsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="elfsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="elfsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="elfsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="elfsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="elfsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="elfsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl<Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbol<'data, 'file, Elf, R>`

##### `impl ToOwned for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-toowned-type-owned"></span>`type Owned = T`

- <span id="elfsymbol-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="elfsymbol-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsymbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSymbol<'data, 'file, Elf, R>`

- <span id="elfsymbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsymbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationSections`

```rust
struct RelocationSections {
    relocations: alloc::vec::Vec<usize>,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:18-20`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L18-L20)*

A mapping from section index to associated relocation sections.

#### Implementations

- <span id="relocationsections-parse"></span>`fn parse<'data, Elf: FileHeader, R: ReadRef<'data>>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` — [`FileHeader`](#fileheader), [`SectionTable`](#sectiontable), [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result)

  Create a new mapping using the section table.

  

  Skips relocation sections that do not use the given symbol table section.

- <span id="relocationsections-get"></span>`fn get(&self, index: SectionIndex) -> Option<SectionIndex>` — [`SectionIndex`](../../index.md#sectionindex)

  Given a section index, return the section index of the associated relocation section.

  

  This may also be called with a relocation section index, and it will return the

  next associated relocation section.

#### Trait Implementations

##### `impl Any for RelocationSections`

- <span id="relocationsections-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationSections`

- <span id="relocationsections-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationSections`

- <span id="relocationsections-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RelocationSections`

- <span id="relocationsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationSections`

- <span id="relocationsections-default"></span>`fn default() -> RelocationSections` — [`RelocationSections`](#relocationsections)

##### `impl<T> From for RelocationSections`

- <span id="relocationsections-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationSections`

- <span id="relocationsections-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RelocationSections`

- <span id="relocationsections-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationsections-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationSections`

- <span id="relocationsections-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationsections-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:123-132`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L123-L132)*

An iterator for the dynamic relocations in an [`ElfFile`](#elffile).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current relocation section index.

#### Trait Implementations

##### `impl Any for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfdynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfdynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="elfdynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfdynamicrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfdynamicrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:207-216`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L207-L216)*

An iterator for the relocations for an [`ElfSection`](super::ElfSection).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current pointer in the chain of relocation sections.

#### Trait Implementations

##### `impl Any for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectionrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsectionrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="elfsectionrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsectionrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsectionrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:681-687`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L681-L687)*

An iterator over the relative relocations in an ELF `SHT_RELR` section.

Returned by [`SectionHeader::relr`](super::SectionHeader::relr).

#### Implementations

- <span id="relriterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` — [`FileHeader`](#fileheader)

  Create a new iterator given the `SHT_RELR` section data.

#### Trait Implementations

##### `impl Any for RelrIterator<'data, Elf>`

- <span id="relriterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelrIterator<'data, Elf>`

- <span id="relriterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelrIterator<'data, Elf>`

- <span id="relriterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for RelrIterator<'data, Elf>`

- <span id="relriterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelrIterator<'data, Elf>`

- <span id="relriterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelrIterator<'data, Elf>`

- <span id="relriterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RelrIterator<'data, Elf>`

- <span id="relriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relriterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for RelrIterator<'data, Elf>`

- <span id="relriterator-iterator-type-item"></span>`type Item = <Elf as FileHeader>::Word`

- <span id="relriterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for RelrIterator<'data, Elf>`

- <span id="relriterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relriterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelrIterator<'data, Elf>`

- <span id="relriterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relriterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Crel`

```rust
struct Crel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:792-803`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L792-L803)*

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

- <span id="crel-symbol"></span>`fn symbol(&self) -> Option<SymbolIndex>` — [`SymbolIndex`](../../index.md#symbolindex)

  Get the symbol index referenced by the relocation.

  

  Returns `None` for the null symbol index.

- <span id="crel-from-rel"></span>`fn from_rel<R: Rel>(r: &R, endian: <R as >::Endian) -> Crel` — [`Rel`](#rel), [`Crel`](#crel)

  Build Crel type from Rel.

- <span id="crel-from-rela"></span>`fn from_rela<R: Rela>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` — [`Rela`](#rela), [`Crel`](#crel)

  Build Crel type from Rela.

#### Trait Implementations

##### `impl Any for Crel`

- <span id="crel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Crel`

- <span id="crel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Crel`

- <span id="crel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Crel`

- <span id="crel-clone"></span>`fn clone(&self) -> Crel` — [`Crel`](#crel)

##### `impl CloneToUninit for Crel`

- <span id="crel-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Crel`

##### `impl Debug for Crel`

- <span id="crel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Crel`

- <span id="crel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Crel`

- <span id="crel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Crel`

- <span id="crel-toowned-type-owned"></span>`type Owned = T`

- <span id="crel-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="crel-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Crel`

- <span id="crel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="crel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Crel`

- <span id="crel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="crel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CrelIteratorHeader`

```rust
struct CrelIteratorHeader {
    count: usize,
    flag_bits: u64,
    shift: u64,
    is_rela: bool,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:839-848`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L839-L848)*

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

##### `impl Any for CrelIteratorHeader`

- <span id="creliteratorheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrelIteratorHeader`

- <span id="creliteratorheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrelIteratorHeader`

- <span id="creliteratorheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrelIteratorHeader`

- <span id="creliteratorheader-clone"></span>`fn clone(&self) -> CrelIteratorHeader` — [`CrelIteratorHeader`](relocation/index.md#creliteratorheader)

##### `impl CloneToUninit for CrelIteratorHeader`

- <span id="creliteratorheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrelIteratorHeader`

- <span id="creliteratorheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CrelIteratorHeader`

- <span id="creliteratorheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CrelIteratorHeader`

- <span id="creliteratorheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CrelIteratorHeader`

- <span id="creliteratorheader-toowned-type-owned"></span>`type Owned = T`

- <span id="creliteratorheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="creliteratorheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CrelIteratorHeader`

- <span id="creliteratorheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="creliteratorheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrelIteratorHeader`

- <span id="creliteratorheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="creliteratorheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:851-862`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L851-L862)*

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

##### `impl Any for CrelIteratorState`

- <span id="creliteratorstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrelIteratorState`

- <span id="creliteratorstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrelIteratorState`

- <span id="creliteratorstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrelIteratorState`

- <span id="creliteratorstate-clone"></span>`fn clone(&self) -> CrelIteratorState` — [`CrelIteratorState`](relocation/index.md#creliteratorstate)

##### `impl CloneToUninit for CrelIteratorState`

- <span id="creliteratorstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrelIteratorState`

- <span id="creliteratorstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrelIteratorState`

- <span id="creliteratorstate-default"></span>`fn default() -> CrelIteratorState` — [`CrelIteratorState`](relocation/index.md#creliteratorstate)

##### `impl<T> From for CrelIteratorState`

- <span id="creliteratorstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CrelIteratorState`

- <span id="creliteratorstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CrelIteratorState`

- <span id="creliteratorstate-toowned-type-owned"></span>`type Owned = T`

- <span id="creliteratorstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="creliteratorstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CrelIteratorState`

- <span id="creliteratorstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="creliteratorstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrelIteratorState`

- <span id="creliteratorstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="creliteratorstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CrelIterator<'data>`

```rust
struct CrelIterator<'data> {
    data: crate::read::Bytes<'data>,
    header: CrelIteratorHeader,
    state: CrelIteratorState,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:866-873`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L866-L873)*

Compact relocation iterator.

#### Fields

- **`data`**: `crate::read::Bytes<'data>`

  Input stream reader.

- **`header`**: `CrelIteratorHeader`

  Parsed header information.

- **`state`**: `CrelIteratorState`

  State of the iterator.

#### Implementations

- <span id="creliterator-new"></span>`fn new(data: &'data [u8]) -> Result<Self, Error>` — [`Error`](../../index.md#error)

  Create a new CREL relocation iterator.

- <span id="creliterator-is-rela"></span>`fn is_rela(&self) -> bool`

  True if the encoded relocations have addend.

- <span id="creliterator-len"></span>`fn len(&self) -> usize`

  Return the number of encoded relocations.

- <span id="creliterator-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if there are no more relocations to parse.

- <span id="creliterator-parse"></span>`fn parse(&mut self) -> read::Result<Crel>` — [`Result`](../../index.md#result), [`Crel`](#crel)

#### Trait Implementations

##### `impl Any for CrelIterator<'data>`

- <span id="creliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrelIterator<'data>`

- <span id="creliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrelIterator<'data>`

- <span id="creliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrelIterator<'data>`

- <span id="creliterator-clone"></span>`fn clone(&self) -> CrelIterator<'data>` — [`CrelIterator`](#creliterator)

##### `impl CloneToUninit for CrelIterator<'data>`

- <span id="creliterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrelIterator<'data>`

- <span id="creliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CrelIterator<'data>`

- <span id="creliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CrelIterator<'data>`

- <span id="creliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CrelIterator<'data>`

- <span id="creliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="creliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="creliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CrelIterator<'data>`

- <span id="creliterator-iterator-type-item"></span>`type Item = Result<Crel, Error>`

- <span id="creliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="creliterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for CrelIterator<'data>`

- <span id="creliterator-toowned-type-owned"></span>`type Owned = T`

- <span id="creliterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="creliterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CrelIterator<'data>`

- <span id="creliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="creliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrelIterator<'data>`

- <span id="creliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="creliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:19-26`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L19-L26)*

An iterator for the COMDAT section groups in an [`ElfFile`](#elffile).

#### Implementations

- <span id="elfcomdatiterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> ElfComdatIterator<'data, 'file, Elf, R>` — [`ElfFile`](#elffile), [`ElfComdatIterator`](#elfcomdatiterator)

#### Trait Implementations

##### `impl Any for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-iterator-type-item"></span>`type Item = ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfcomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfcomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:70-78`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L70-L78)*

A COMDAT section group in an [`ElfFile`](#elffile).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Implementations

- <span id="elfcomdat-parse"></span>`fn parse(file: &'file ElfFile<'data, Elf, R>, section: &'data <Elf as >::SectionHeader) -> Option<ElfComdat<'data, 'file, Elf, R>>` — [`ElfFile`](#elffile), [`FileHeader`](#fileheader), [`ElfComdat`](#elfcomdat)

- <span id="elfcomdat-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](#elffile)

  Get the ELF file containing this COMDAT section group.

- <span id="elfcomdat-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](#fileheader)

  Get the raw ELF section header for the COMDAT section group.

#### Trait Implementations

##### `impl Any for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf, R> ObjectComdat for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="elfcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="elfcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="elfcomdat-objectcomdat-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="elfcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md#objectcomdat)

##### `impl<Elf, R> Sealed for ElfComdat<'data, 'file, Elf, R>`

##### `impl<U> TryFrom for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfcomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfcomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:166-173`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L166-L173)*

An iterator for the sections in a COMDAT section group in an [`ElfFile`](#elffile).

#### Trait Implementations

##### `impl Any for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="elfcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfcomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfcomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/note.rs:17-24`](../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L17-L24)*

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

#### Implementations

- <span id="noteiterator-new"></span>`fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result)

  An iterator over the notes in an ELF section or segment.

  

  `align` should be from the `p_align` field of the segment,

  or the `sh_addralign` field of the section. Supported values are

  either 4 or 8, but values less than 4 are treated as 4.

  This matches the behaviour of binutils.

  

  Returns `Err` if `align` is invalid.

- <span id="noteiterator-next"></span>`fn next(&mut self) -> read::Result<Option<Note<'data, Elf>>>` — [`Result`](../../index.md#result), [`Note`](#note)

  Returns the next note.

- <span id="noteiterator-parse"></span>`fn parse(&mut self) -> read::Result<Note<'data, Elf>>` — [`Result`](../../index.md#result), [`Note`](#note)

#### Trait Implementations

##### `impl Any for NoteIterator<'data, Elf>`

- <span id="noteiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoteIterator<'data, Elf>`

- <span id="noteiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoteIterator<'data, Elf>`

- <span id="noteiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf> Debug for NoteIterator<'data, Elf>`

- <span id="noteiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NoteIterator<'data, Elf>`

- <span id="noteiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoteIterator<'data, Elf>`

- <span id="noteiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="noteiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="noteiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-iterator-type-item"></span>`type Item = Result<Note<'data, Elf>, Error>`

- <span id="noteiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for NoteIterator<'data, Elf>`

- <span id="noteiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="noteiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoteIterator<'data, Elf>`

- <span id="noteiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="noteiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/note.rs:109-116`](../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L109-L116)*

A parsed [`NoteHeader`](#noteheader).

#### Implementations

- <span id="note-n-type"></span>`fn n_type(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

  Return the `n_type` field of the `NoteHeader`.

  

  The meaning of this field is determined by `name`.

- <span id="note-n-namesz"></span>`fn n_namesz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

  Return the `n_namesz` field of the `NoteHeader`.

- <span id="note-n-descsz"></span>`fn n_descsz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](#fileheader)

  Return the `n_descsz` field of the `NoteHeader`.

- <span id="note-name-bytes"></span>`fn name_bytes(&self) -> &'data [u8]`

  Return the bytes for the name field following the `NoteHeader`.

  

  This field is usually a string including one or more trailing null bytes

  (but it is not required to be).

  

  The length of this field is given by `n_namesz`.

- <span id="note-name"></span>`fn name(&self) -> &'data [u8]`

  Return the bytes for the name field following the `NoteHeader`,

  excluding all trailing null bytes.

- <span id="note-desc"></span>`fn desc(&self) -> &'data [u8]`

  Return the bytes for the desc field following the `NoteHeader`.

  

  The length of this field is given by `n_descsz`. The meaning

  of this field is determined by `name` and `n_type`.

- <span id="note-gnu-properties"></span>`fn gnu_properties(&self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` — [`FileHeader`](#fileheader), [`GnuPropertyIterator`](#gnupropertyiterator)

  Return an iterator for properties if this note's type is [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md).

#### Trait Implementations

##### `impl Any for Note<'data, Elf>`

- <span id="note-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Note<'data, Elf>`

- <span id="note-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Note<'data, Elf>`

- <span id="note-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf> Debug for Note<'data, Elf>`

- <span id="note-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Note<'data, Elf>`

- <span id="note-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Note<'data, Elf>`

- <span id="note-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Note<'data, Elf>`

- <span id="note-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="note-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Note<'data, Elf>`

- <span id="note-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="note-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GnuPropertyIterator<'data, Endian: endian::Endian>`

```rust
struct GnuPropertyIterator<'data, Endian: endian::Endian> {
    endian: Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/note.rs:235-239`](../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L235-L239)*

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md) note.

Returned by `Note::gnu_properties`.

#### Implementations

- <span id="gnupropertyiterator-next"></span>`fn next(&mut self) -> read::Result<Option<GnuProperty<'data>>>` — [`Result`](../../index.md#result), [`GnuProperty`](#gnuproperty)

  Returns the next property.

- <span id="gnupropertyiterator-parse"></span>`fn parse(&mut self) -> read::Result<GnuProperty<'data>>` — [`Result`](../../index.md#result), [`GnuProperty`](#gnuproperty)

#### Trait Implementations

##### `impl Any for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Endian: fmt::Debug + endian::Endian> Debug for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="gnupropertyiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="gnupropertyiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Endian: endian::Endian> Iterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-iterator-type-item"></span>`type Item = Result<GnuProperty<'data>, Error>`

- <span id="gnupropertyiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gnupropertyiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gnupropertyiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GnuProperty<'data>`

```rust
struct GnuProperty<'data> {
    pr_type: u32,
    pr_data: &'data [u8],
}
```

*Defined in [`object-0.37.3/src/read/elf/note.rs:277-280`](../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L277-L280)*

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../elf/index.md) note.

#### Implementations

- <span id="gnuproperty-pr-type"></span>`fn pr_type(&self) -> u32`

  Return the property type.

  

  This is one of the `GNU_PROPERTY_*` constants.

- <span id="gnuproperty-pr-data"></span>`fn pr_data(&self) -> &'data [u8]`

  Return the property data.

- <span id="gnuproperty-data-u32"></span>`fn data_u32<E: endian::Endian>(&self, endian: E) -> read::Result<u32>` — [`Result`](../../index.md#result)

  Parse the property data as an unsigned 32-bit integer.

#### Trait Implementations

##### `impl Any for GnuProperty<'data>`

- <span id="gnuproperty-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GnuProperty<'data>`

- <span id="gnuproperty-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GnuProperty<'data>`

- <span id="gnuproperty-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for GnuProperty<'data>`

- <span id="gnuproperty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GnuProperty<'data>`

- <span id="gnuproperty-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GnuProperty<'data>`

- <span id="gnuproperty-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for GnuProperty<'data>`

- <span id="gnuproperty-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gnuproperty-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GnuProperty<'data>`

- <span id="gnuproperty-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gnuproperty-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HashTable<'data, Elf: FileHeader>`

```rust
struct HashTable<'data, Elf: FileHeader> {
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    chains: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

*Defined in [`object-0.37.3/src/read/elf/hash.rs:13-16`](../../../../.source_1765521767/object-0.37.3/src/read/elf/hash.rs#L13-L16)*

A SysV symbol hash table in an ELF file.

Returned by [`SectionHeader::hash`](super::SectionHeader::hash).

#### Implementations

- <span id="hashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result)

  Parse a SysV hash table.

  

  `data` should be from an [`elf::SHT_HASH`](../../elf/index.md) section, or from a

  segment pointed to via the [`elf::DT_HASH`](../../elf/index.md) entry.

  

  The header is read at offset 0 in the given `data`.

- <span id="hashtable-symbol-table-length"></span>`fn symbol_table_length(&self) -> u32`

  Return the symbol table length.

- <span id="hashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex)

- <span id="hashtable-chain"></span>`fn chain(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex)

- <span id="hashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](#fileheader), [`Version`](#version), [`SymbolTable`](#symboltable), [`VersionTable`](#versiontable), [`SymbolIndex`](../../index.md#symbolindex)

  Use the hash table to find the symbol table entry with the given name, hash and version.

#### Trait Implementations

##### `impl Any for HashTable<'data, Elf>`

- <span id="hashtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HashTable<'data, Elf>`

- <span id="hashtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HashTable<'data, Elf>`

- <span id="hashtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for HashTable<'data, Elf>`

- <span id="hashtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HashTable<'data, Elf>`

- <span id="hashtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HashTable<'data, Elf>`

- <span id="hashtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HashTable<'data, Elf>`

- <span id="hashtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hashtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HashTable<'data, Elf>`

- <span id="hashtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hashtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/hash.rs:86-92`](../../../../.source_1765521767/object-0.37.3/src/read/elf/hash.rs#L86-L92)*

A GNU symbol hash table in an ELF file.

Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash).

#### Implementations

- <span id="gnuhashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result)

  Parse a GNU hash table.

  

  `data` should be from an [`elf::SHT_GNU_HASH`](../../elf/index.md) section, or from a

  segment pointed to via the [`elf::DT_GNU_HASH`](../../elf/index.md) entry.

  

  The header is read at offset 0 in the given `data`.

  

  The header does not contain a length field, and so all of `data`

  will be used as the hash table values. It does not matter if this

  is longer than needed, and this will often the case when accessing

  the hash table via the [`elf::DT_GNU_HASH`](../../elf/index.md) entry.

- <span id="gnuhashtable-symbol-base"></span>`fn symbol_base(&self) -> u32`

  Return the symbol table index of the first symbol in the hash table.

- <span id="gnuhashtable-symbol-table-length"></span>`fn symbol_table_length(&self, endian: <Elf as >::Endian) -> Option<u32>` — [`FileHeader`](#fileheader)

  Determine the symbol table length by finding the last entry in the hash table.

  

  Returns `None` if the hash table is empty or invalid.

- <span id="gnuhashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex)

- <span id="gnuhashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](#fileheader), [`Version`](#version), [`SymbolTable`](#symboltable), [`VersionTable`](#versiontable), [`SymbolIndex`](../../index.md#symbolindex)

  Use the hash table to find the symbol table entry with the given name, hash, and version.

#### Trait Implementations

##### `impl Any for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gnuhashtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gnuhashtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:10`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L10)*

A version index.

#### Implementations

- <span id="versionindex-index"></span>`fn index(&self) -> u16`

  Return the version index.

- <span id="versionindex-is-local"></span>`fn is_local(&self) -> bool`

  Return true if it is the local index.

- <span id="versionindex-is-global"></span>`fn is_global(&self) -> bool`

  Return true if it is the global index.

- <span id="versionindex-is-hidden"></span>`fn is_hidden(&self) -> bool`

  Return the hidden flag.

#### Trait Implementations

##### `impl Any for VersionIndex`

- <span id="versionindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VersionIndex`

- <span id="versionindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VersionIndex`

- <span id="versionindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for VersionIndex`

- <span id="versionindex-clone"></span>`fn clone(&self) -> VersionIndex` — [`VersionIndex`](#versionindex)

##### `impl CloneToUninit for VersionIndex`

- <span id="versionindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- <span id="versionindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VersionIndex`

- <span id="versionindex-default"></span>`fn default() -> VersionIndex` — [`VersionIndex`](#versionindex)

##### `impl<T> From for VersionIndex`

- <span id="versionindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VersionIndex`

- <span id="versionindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for VersionIndex`

- <span id="versionindex-toowned-type-owned"></span>`type Owned = T`

- <span id="versionindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="versionindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VersionIndex`

- <span id="versionindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="versionindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VersionIndex`

- <span id="versionindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="versionindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Version<'data>`

```rust
struct Version<'data> {
    name: &'data [u8],
    hash: u32,
    valid: bool,
    file: Option<&'data [u8]>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:38-44`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L38-L44)*

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`](../../elf/index.md) and [`elf::SHT_GNU_VERNEED`](../../elf/index.md) sections.

#### Implementations

- <span id="version-name"></span>`fn name(&self) -> &'data [u8]`

  Return the version name.

- <span id="version-hash"></span>`fn hash(&self) -> u32`

  Return hash of the version name.

- <span id="version-file"></span>`fn file(&self) -> Option<&'data [u8]>`

  Return the filename of the library containing this version.

  

  This is the `vn_file` field of the associated entry in [`elf::SHT_GNU_VERNEED`](../../elf/index.md).

  or `None` if the version info was parsed from a [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Trait Implementations

##### `impl Any for Version<'data>`

- <span id="version-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Version<'data>`

- <span id="version-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Version<'data>`

- <span id="version-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Version<'data>`

- <span id="version-clone"></span>`fn clone(&self) -> Version<'data>` — [`Version`](#version)

##### `impl CloneToUninit for Version<'data>`

- <span id="version-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Version<'data>`

##### `impl Debug for Version<'data>`

- <span id="version-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Version<'data>`

- <span id="version-default"></span>`fn default() -> Version<'data>` — [`Version`](#version)

##### `impl<T> From for Version<'data>`

- <span id="version-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Version<'data>`

- <span id="version-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Version<'data>`

- <span id="version-toowned-type-owned"></span>`type Owned = T`

- <span id="version-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="version-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Version<'data>`

- <span id="version-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="version-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Version<'data>`

- <span id="version-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="version-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VersionTable<'data, Elf: FileHeader>`

```rust
struct VersionTable<'data, Elf: FileHeader> {
    symbols: &'data [elf::Versym<<Elf as >::Endian>],
    versions: alloc::vec::Vec<Version<'data>>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:75-78`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L75-L78)*

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`](../../elf/index.md), [`elf::SHT_GNU_VERDEF`](../../elf/index.md)
and [`elf::SHT_GNU_VERNEED`](../../elf/index.md) sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

#### Implementations

- <span id="versiontable-parse"></span>`fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](#fileheader), [`Versym`](../../elf/index.md#versym), [`VerdefIterator`](#verdefiterator), [`VerneedIterator`](#verneediterator), [`StringTable`](../index.md#stringtable), [`Result`](../../index.md#result)

  Parse the version sections.

- <span id="versiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the version table is empty.

- <span id="versiontable-version-index"></span>`fn version_index(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex), [`VersionIndex`](#versionindex)

  Return version index for a given symbol index.

- <span id="versiontable-version"></span>`fn version(&self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](#versionindex), [`Result`](../../index.md#result), [`Version`](#version)

  Return version information for a given symbol version index.

  

  Returns `Ok(None)` for local and global versions.

  Returns `Err(_)` if index is invalid.

- <span id="versiontable-matches"></span>`fn matches(&self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](#fileheader), [`SymbolIndex`](../../index.md#symbolindex), [`Version`](#version)

  Return true if the given symbol index satisfies the requirements of `need`.

  

  Returns false for any error.

  

  Note: this function hasn't been fully tested and is likely to be incomplete.

#### Trait Implementations

##### `impl Any for VersionTable<'data, Elf>`

- <span id="versiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VersionTable<'data, Elf>`

- <span id="versiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VersionTable<'data, Elf>`

- <span id="versiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- <span id="versiontable-clone"></span>`fn clone(&self) -> VersionTable<'data, Elf>` — [`VersionTable`](#versiontable)

##### `impl CloneToUninit for VersionTable<'data, Elf>`

- <span id="versiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- <span id="versiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader> Default for VersionTable<'data, Elf>`

- <span id="versiontable-default"></span>`fn default() -> Self`

##### `impl<T> From for VersionTable<'data, Elf>`

- <span id="versiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VersionTable<'data, Elf>`

- <span id="versiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for VersionTable<'data, Elf>`

- <span id="versiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="versiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="versiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VersionTable<'data, Elf>`

- <span id="versiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="versiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VersionTable<'data, Elf>`

- <span id="versiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="versiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:234-237`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L234-L237)*

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Implementations

- <span id="verdefiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](#fileheader)

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../index.md#result), [`Verdef`](../../elf/index.md#verdef), [`FileHeader`](#fileheader), [`VerdauxIterator`](#verdauxiterator)

  Return the next `Verdef` entry.

- <span id="verdefiterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../index.md#result), [`Verdef`](../../elf/index.md#verdef), [`FileHeader`](#fileheader), [`VerdauxIterator`](#verdauxiterator)

#### Trait Implementations

##### `impl Any for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clone"></span>`fn clone(&self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](#verdefiterator)

##### `impl CloneToUninit for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdefiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verdefiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-iterator-type-item"></span>`type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- <span id="verdefiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="verdefiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="verdefiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verdefiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verdefiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerdauxIterator<'data, Elf: FileHeader>`

```rust
struct VerdauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:297-301`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L297-L301)*

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`](../../elf/index.md) section.

#### Implementations

- <span id="verdauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](#fileheader)

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../index.md#result), [`Verdaux`](../../elf/index.md#verdaux), [`FileHeader`](#fileheader)

  Return the next `Verdaux` entry.

- <span id="verdauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../index.md#result), [`Verdaux`](../../elf/index.md#verdaux), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl Any for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clone"></span>`fn clone(&self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](#verdauxiterator)

##### `impl CloneToUninit for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdauxiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verdauxiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-iterator-type-item"></span>`type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="verdauxiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="verdauxiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="verdauxiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verdauxiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verdauxiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:350-353`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L350-L353)*

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../elf/index.md) section.

#### Implementations

- <span id="verneediterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](#fileheader)

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../index.md#result), [`Verneed`](../../elf/index.md#verneed), [`FileHeader`](#fileheader), [`VernauxIterator`](#vernauxiterator)

  Return the next `Verneed` entry.

- <span id="verneediterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../index.md#result), [`Verneed`](../../elf/index.md#verneed), [`FileHeader`](#fileheader), [`VernauxIterator`](#vernauxiterator)

#### Trait Implementations

##### `impl Any for VerneedIterator<'data, Elf>`

- <span id="verneediterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerneedIterator<'data, Elf>`

- <span id="verneediterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerneedIterator<'data, Elf>`

- <span id="verneediterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clone"></span>`fn clone(&self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](#verneediterator)

##### `impl CloneToUninit for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- <span id="verneediterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VerneedIterator<'data, Elf>`

- <span id="verneediterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerneedIterator<'data, Elf>`

- <span id="verneediterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verneediterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verneediterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-iterator-type-item"></span>`type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- <span id="verneediterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VerneedIterator<'data, Elf>`

- <span id="verneediterator-toowned-type-owned"></span>`type Owned = T`

- <span id="verneediterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="verneediterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VerneedIterator<'data, Elf>`

- <span id="verneediterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verneediterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerneedIterator<'data, Elf>`

- <span id="verneediterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verneediterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VernauxIterator<'data, Elf: FileHeader>`

```rust
struct VernauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:426-430`](../../../../.source_1765521767/object-0.37.3/src/read/elf/version.rs#L426-L430)*

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`](../../elf/index.md) section.

#### Implementations

- <span id="vernauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](#fileheader)

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../index.md#result), [`Vernaux`](../../elf/index.md#vernaux), [`FileHeader`](#fileheader)

  Return the next `Vernaux` entry.

- <span id="vernauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../index.md#result), [`Vernaux`](../../elf/index.md#vernaux), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl Any for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clone"></span>`fn clone(&self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](#vernauxiterator)

##### `impl CloneToUninit for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="vernauxiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="vernauxiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-iterator-type-item"></span>`type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="vernauxiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="vernauxiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="vernauxiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vernauxiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vernauxiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSection<'data, Elf: FileHeader>`

```rust
struct AttributesSection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    version: u8,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:18-22`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L18-L22)*

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`](#attributessubsection).

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

#### Implementations

- <span id="attributessection-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](#fileheader), [`Result`](../../index.md#result)

  Parse an ELF attributes section given the section data.

- <span id="attributessection-version"></span>`fn version(&self) -> u8`

  Return the version of the attributes section.

- <span id="attributessection-subsections"></span>`fn subsections(&self) -> Result<AttributesSubsectionIterator<'data, Elf>>` — [`Result`](../../index.md#result), [`AttributesSubsectionIterator`](#attributessubsectioniterator)

  Return an iterator over the subsections.

#### Trait Implementations

##### `impl Any for AttributesSection<'data, Elf>`

- <span id="attributessection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSection<'data, Elf>`

- <span id="attributessection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSection<'data, Elf>`

- <span id="attributessection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSection<'data, Elf>`

- <span id="attributessection-clone"></span>`fn clone(&self) -> AttributesSection<'data, Elf>` — [`AttributesSection`](#attributessection)

##### `impl CloneToUninit for AttributesSection<'data, Elf>`

- <span id="attributessection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSection<'data, Elf>`

- <span id="attributessection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSection<'data, Elf>`

- <span id="attributessection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSection<'data, Elf>`

- <span id="attributessection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributesSection<'data, Elf>`

- <span id="attributessection-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSection<'data, Elf>`

- <span id="attributessection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSection<'data, Elf>`

- <span id="attributessection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:61-64`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L61-L64)*

An iterator for the subsections in an [`AttributesSection`](#attributessection).

#### Implementations

- <span id="attributessubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsection<'data, Elf>>>` — [`Result`](../../index.md#result), [`AttributesSubsection`](#attributessubsection)

  Return the next subsection.

- <span id="attributessubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsection<'data, Elf>>` — [`Result`](../../index.md#result), [`AttributesSubsection`](#attributessubsection)

#### Trait Implementations

##### `impl Any for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsectionIterator<'data, Elf>` — [`AttributesSubsectionIterator`](#attributessubsectioniterator)

##### `impl CloneToUninit for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-iterator-type-item"></span>`type Item = Result<AttributesSubsection<'data, Elf>, Error>`

- <span id="attributessubsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsectioniterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsectioniterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsection<'data, Elf: FileHeader>`

```rust
struct AttributesSubsection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    length: u32,
    vendor: &'data [u8],
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:124-129`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L124-L129)*

A subsection in an [`AttributesSection`](#attributessection).

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`](#attributessubsubsection).

#### Implementations

- <span id="attributessubsection-length"></span>`fn length(&self) -> u32`

  Return the length of the attributes subsection.

- <span id="attributessubsection-vendor"></span>`fn vendor(&self) -> &'data [u8]`

  Return the vendor name of the attributes subsection.

- <span id="attributessubsection-subsubsections"></span>`fn subsubsections(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)

  Return an iterator over the sub-subsections.

#### Trait Implementations

##### `impl Any for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-clone"></span>`fn clone(&self) -> AttributesSubsection<'data, Elf>` — [`AttributesSubsection`](#attributessubsection)

##### `impl CloneToUninit for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:153-156`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L153-L156)*

An iterator for the sub-subsections in an [`AttributesSubsection`](#attributessubsection).

#### Implementations

- <span id="attributessubsubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsubsection<'data>>>` — [`Result`](../../index.md#result), [`AttributesSubsubsection`](#attributessubsubsection)

  Return the next sub-subsection.

- <span id="attributessubsubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsubsection<'data>>` — [`Result`](../../index.md#result), [`AttributesSubsubsection`](#attributessubsubsection)

#### Trait Implementations

##### `impl Any for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator)

##### `impl CloneToUninit for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsubsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsubsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-iterator-type-item"></span>`type Item = Result<AttributesSubsubsection<'data>, Error>`

- <span id="attributessubsubsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsubsectioniterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsubsectioniterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsubsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsubsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    tag: u8,
    length: u32,
    indices: crate::read::Bytes<'data>,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:229-234`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L229-L234)*

A sub-subsection in an [`AttributesSubsection`](#attributessubsection).

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

#### Implementations

- <span id="attributessubsubsection-tag"></span>`fn tag(&self) -> u8`

  Return the tag of the attributes sub-subsection.

- <span id="attributessubsubsection-length"></span>`fn length(&self) -> u32`

  Return the length of the attributes sub-subsection.

- <span id="attributessubsubsection-indices-data"></span>`fn indices_data(&self) -> &'data [u8]`

  Return the data containing the indices.

- <span id="attributessubsubsection-indices"></span>`fn indices(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](#attributeindexiterator)

  Return the indices.

  

  This will be section indices if the tag is `Tag_Section`,

  or symbol indices if the tag is `Tag_Symbol`,

  and otherwise it will be empty.

- <span id="attributessubsubsection-attributes-data"></span>`fn attributes_data(&self) -> &'data [u8]`

  Return the data containing the attributes.

- <span id="attributessubsubsection-attributes"></span>`fn attributes(&self) -> AttributeReader<'data>` — [`AttributeReader`](#attributereader)

  Return a parser for the data containing the attributes.

#### Trait Implementations

##### `impl Any for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clone"></span>`fn clone(&self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](#attributessubsubsection)

##### `impl CloneToUninit for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-toowned-type-owned"></span>`type Owned = T`

- <span id="attributessubsubsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributessubsubsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributessubsubsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributessubsubsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributeIndexIterator<'data>`

```rust
struct AttributeIndexIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:274-276`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L274-L276)*

An iterator over the indices in an [`AttributesSubsubsection`](#attributessubsubsection).

#### Implementations

- <span id="attributeindexiterator-next"></span>`fn next(&mut self) -> Result<Option<u32>>` — [`Result`](../../index.md#result)

  Parse the next index.

- <span id="attributeindexiterator-parse"></span>`fn parse(&mut self) -> Result<u32>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl Any for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-clone"></span>`fn clone(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](#attributeindexiterator)

##### `impl CloneToUninit for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributeindexiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributeindexiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-iterator-type-item"></span>`type Item = Result<u32, Error>`

- <span id="attributeindexiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="attributeindexiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributeindexiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributeindexiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributeindexiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributeReader<'data>`

```rust
struct AttributeReader<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:315-317`](../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L315-L317)*

A parser for the attributes in an [`AttributesSubsubsection`](#attributessubsubsection).

The parser relies on the caller to know the format of the data for each attribute tag.

#### Implementations

- <span id="attributereader-read-tag"></span>`fn read_tag(&mut self) -> Result<Option<u64>>` — [`Result`](../../index.md#result)

  Parse a tag.

- <span id="attributereader-read-integer"></span>`fn read_integer(&mut self) -> Result<u64>` — [`Result`](../../index.md#result)

  Parse an integer value.

- <span id="attributereader-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

  Parse a string value.

#### Trait Implementations

##### `impl Any for AttributeReader<'data>`

- <span id="attributereader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeReader<'data>`

- <span id="attributereader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeReader<'data>`

- <span id="attributereader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributeReader<'data>`

- <span id="attributereader-clone"></span>`fn clone(&self) -> AttributeReader<'data>` — [`AttributeReader`](#attributereader)

##### `impl CloneToUninit for AttributeReader<'data>`

- <span id="attributereader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AttributeReader<'data>`

- <span id="attributereader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttributeReader<'data>`

- <span id="attributereader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributeReader<'data>`

- <span id="attributereader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttributeReader<'data>`

- <span id="attributereader-toowned-type-owned"></span>`type Owned = T`

- <span id="attributereader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributereader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeReader<'data>`

- <span id="attributereader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributereader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeReader<'data>`

- <span id="attributereader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributereader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ElfRelocationIterator<'data, Elf: FileHeader>`

```rust
enum ElfRelocationIterator<'data, Elf: FileHeader> {
    Rel(slice::Iter<'data, <Elf as >::Rel>, <Elf as >::Endian),
    Rela(slice::Iter<'data, <Elf as >::Rela>, <Elf as >::Endian, bool),
    Crel(CrelIterator<'data>),
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:83-87`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L83-L87)*

#### Implementations

- <span id="elfrelocationiterator-is-rel"></span>`fn is_rel(&self) -> bool`

#### Trait Implementations

##### `impl Any for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-iterator-type-item"></span>`type Item = Crel`

- <span id="elfrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/file.rs:530-819`](../../../../.source_1765521767/object-0.37.3/src/read/elf/file.rs#L530-L819)*

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

- [`FileHeader32`](../../elf/index.md#fileheader32)
- [`FileHeader64`](../../elf/index.md#fileheader64)

### `ProgramHeader`

```rust
trait ProgramHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/segment.rs:152-273`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L152-L273)*

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

- [`ProgramHeader32`](../../elf/index.md#programheader32)
- [`ProgramHeader64`](../../elf/index.md#programheader64)

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:686-1170`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L686-L1170)*

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

- [`SectionHeader32`](../../elf/index.md#sectionheader32)
- [`SectionHeader64`](../../elf/index.md#sectionheader64)

### `Sym`

```rust
trait Sym: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:491-554`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L491-L554)*

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

- [`Sym32`](../../elf/index.md#sym32)
- [`Sym64`](../../elf/index.md#sym64)

### `Rel`

```rust
trait Rel: Debug + Pod + Clone { ... }
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:514-535`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L514-L535)*

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

- [`Rel32`](../../elf/index.md#rel32)
- [`Rel64`](../../elf/index.md#rel64)

### `Rela`

```rust
trait Rela: Debug + Pod + Clone { ... }
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:591-613`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L591-L613)*

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

- [`Rela32`](../../elf/index.md#rela32)
- [`Rela64`](../../elf/index.md#rela64)

### `Relr`

```rust
trait Relr: Debug + Pod + Clone { ... }
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:727-746`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L727-L746)*

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

- [`Relr32`](../../elf/index.md#relr32)
- [`Relr64`](../../elf/index.md#relr64)

### `Dyn`

```rust
trait Dyn: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/dynamic.rs:11-87`](../../../../.source_1765521767/object-0.37.3/src/read/elf/dynamic.rs#L11-L87)*

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

- [`Dyn32`](../../elf/index.md#dyn32)
- [`Dyn64`](../../elf/index.md#dyn64)

### `CompressionHeader`

```rust
trait CompressionHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/compression.rs:9-16`](../../../../.source_1765521767/object-0.37.3/src/read/elf/compression.rs#L9-L16)*

A trait for generic access to [`elf::CompressionHeader32`](../../elf/index.md) and [`elf::CompressionHeader64`](../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn ch_type(&self, endian: <Self as >::Endian) -> u32`

- `fn ch_size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn ch_addralign(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Implementors

- [`CompressionHeader32`](../../elf/index.md#compressionheader32)
- [`CompressionHeader64`](../../elf/index.md#compressionheader64)

### `NoteHeader`

```rust
trait NoteHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/note.rs:185-191`](../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L185-L191)*

A trait for generic access to [`elf::NoteHeader32`](../../elf/index.md) and [`elf::NoteHeader64`](../../elf/index.md).

#### Associated Types

- `type Endian: 1`

#### Required Methods

- `fn n_namesz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_type(&self, endian: <Self as >::Endian) -> u32`

#### Implementors

- [`NoteHeader32`](../../elf/index.md#noteheader32)
- [`NoteHeader64`](../../elf/index.md#noteheader64)

## Functions

### `parse_relocation`

```rust
fn parse_relocation<Elf: FileHeader>(header: &Elf, endian: <Elf as >::Endian, reloc: Crel, implicit_addend: bool) -> crate::read::Relocation
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:278-510`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L278-L510)*

## Type Aliases

### `ElfFile32<'data, Endian, R>`

```rust
type ElfFile32<'data, Endian, R> = ElfFile<'data, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/file.rs:25-26`](../../../../.source_1765521767/object-0.37.3/src/read/elf/file.rs#L25-L26)*

A 32-bit ELF object file.

This is a file that starts with [`elf::FileHeader32`](../../elf/index.md), and corresponds
to [`crate::FileKind::Elf32`](../../index.md).

### `ElfFile64<'data, Endian, R>`

```rust
type ElfFile64<'data, Endian, R> = ElfFile<'data, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/file.rs:31-32`](../../../../.source_1765521767/object-0.37.3/src/read/elf/file.rs#L31-L32)*

A 64-bit ELF object file.

This is a file that starts with [`elf::FileHeader64`](../../elf/index.md), and corresponds
to [`crate::FileKind::Elf64`](../../index.md).

### `ElfSegmentIterator32<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator32<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/segment.rs:12-13`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L12-L13)*

An iterator for the segments in an [`ElfFile32`](super::ElfFile32).

### `ElfSegmentIterator64<'data, 'file, Endian, R>`

```rust
type ElfSegmentIterator64<'data, 'file, Endian, R> = ElfSegmentIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/segment.rs:15-16`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L15-L16)*

An iterator for the segments in an [`ElfFile64`](super::ElfFile64).

### `ElfSegment32<'data, 'file, Endian, R>`

```rust
type ElfSegment32<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/segment.rs:50-51`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L50-L51)*

A segment in an [`ElfFile32`](super::ElfFile32).

### `ElfSegment64<'data, 'file, Endian, R>`

```rust
type ElfSegment64<'data, 'file, Endian, R> = ElfSegment<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/segment.rs:53-54`](../../../../.source_1765521767/object-0.37.3/src/read/elf/segment.rs#L53-L54)*

A segment in an [`ElfFile64`](super::ElfFile64).

### `ElfSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator32<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:350-351`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L350-L351)*

An iterator for the sections in an [`ElfFile32`](super::ElfFile32).

### `ElfSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionIterator64<'data, 'file, Endian, R> = ElfSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:353-354`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L353-L354)*

An iterator for the sections in an [`ElfFile64`](super::ElfFile64).

### `ElfSection32<'data, 'file, Endian, R>`

```rust
type ElfSection32<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:396-397`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L396-L397)*

A section in an [`ElfFile32`](super::ElfFile32).

### `ElfSection64<'data, 'file, Endian, R>`

```rust
type ElfSection64<'data, 'file, Endian, R> = ElfSection<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/section.rs:399-400`](../../../../.source_1765521767/object-0.37.3/src/read/elf/section.rs#L399-L400)*

A section in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbolTable32<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable32<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:221-222`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L221-L222)*

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolTable64<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable64<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:224-225`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L224-L225)*

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator32<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator32<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:265-266`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L265-L266)*

An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator64<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator64<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:268-269`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L268-L269)*

An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbol32<'data, 'file, Endian, R>`

```rust
type ElfSymbol32<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:323-324`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L323-L324)*

A symbol in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbol64<'data, 'file, Endian, R>`

```rust
type ElfSymbol64<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/symbol.rs:326-327`](../../../../.source_1765521767/object-0.37.3/src/read/elf/symbol.rs#L326-L327)*

A symbol in an [`ElfFile64`](super::ElfFile64).

### `ElfDynamicRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator32<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:116-117`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L116-L117)*

An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).

### `ElfDynamicRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator64<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:119-120`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L119-L120)*

An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).

### `ElfSectionRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator32<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:200-201`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L200-L201)*

An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).

### `ElfSectionRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator64<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:203-204`](../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L203-L204)*

An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).

### `ElfComdatIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator32<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:11-12`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L11-L12)*

An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator64<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:14-15`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L14-L15)*

An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).

### `ElfComdat32<'data, 'file, Endian, R>`

```rust
type ElfComdat32<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:60-61`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L60-L61)*

A COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdat64<'data, 'file, Endian, R>`

```rust
type ElfComdat64<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:63-64`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L63-L64)*

A COMDAT section group in an [`ElfFile64`](super::ElfFile64).

### `ElfComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator32<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:158-159`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L158-L159)*

An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator64<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:161-162`](../../../../.source_1765521767/object-0.37.3/src/read/elf/comdat.rs#L161-L162)*

An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).

