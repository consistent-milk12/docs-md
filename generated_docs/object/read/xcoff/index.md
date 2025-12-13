*[object](../../index.md) / [read](../index.md) / [xcoff](index.md)*

---

# Module `xcoff`

Support for reading AIX XCOFF files.

Traits are used to abstract over the difference between 32-bit and 64-bit XCOFF.
The primary trait for this is [`FileHeader`](#fileheader).

## High level API

[`XcoffFile`](#xcofffile) implements the [`Object`](crate::read::Object) trait for XCOFF files.
[`XcoffFile`](#xcofffile) is parameterised by [`FileHeader`](#fileheader) to allow reading both 32-bit and
64-bit XCOFF. There are type aliases for these parameters ([`XcoffFile32`](#xcofffile32) and
[`XcoffFile64`](#xcofffile64)).

## Low level API

The [`FileHeader`](#fileheader) trait can be directly used to parse both [`xcoff::FileHeader32`](../../xcoff/index.md)
and [`xcoff::FileHeader64`](../../xcoff/index.md).

### Example for low level API
 ```no_run
use object::xcoff;
use object::read::xcoff::{FileHeader, SectionHeader, Symbol};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = xcoff::FileHeader64::parse(&*data, &mut offset)?;
    let aux_header = header.aux_header(&*data, &mut offset)?;
    let sections = header.sections(&*data, &mut offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name()));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

## Contents

- [Modules](#modules)
  - [`file`](#file)
  - [`section`](#section)
  - [`symbol`](#symbol)
  - [`relocation`](#relocation)
  - [`comdat`](#comdat)
  - [`segment`](#segment)
- [Structs](#structs)
  - [`XcoffFile`](#xcofffile)
  - [`XcoffSectionIterator`](#xcoffsectioniterator)
  - [`XcoffSection`](#xcoffsection)
  - [`SectionTable`](#sectiontable)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`XcoffSymbolTable`](#xcoffsymboltable)
  - [`XcoffSymbolIterator`](#xcoffsymboliterator)
  - [`XcoffSymbol`](#xcoffsymbol)
  - [`XcoffRelocationIterator`](#xcoffrelocationiterator)
  - [`XcoffComdatIterator`](#xcoffcomdatiterator)
  - [`XcoffComdat`](#xcoffcomdat)
  - [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator)
  - [`XcoffSegmentIterator`](#xcoffsegmentiterator)
  - [`XcoffSegment`](#xcoffsegment)
- [Traits](#traits)
  - [`FileHeader`](#fileheader)
  - [`AuxHeader`](#auxheader)
  - [`SectionHeader`](#sectionheader)
  - [`Symbol`](#symbol)
  - [`FileAux`](#fileaux)
  - [`CsectAux`](#csectaux)
  - [`Rel`](#rel)
- [Type Aliases](#type-aliases)
  - [`XcoffFile32`](#xcofffile32)
  - [`XcoffFile64`](#xcofffile64)
  - [`XcoffSectionIterator32`](#xcoffsectioniterator32)
  - [`XcoffSectionIterator64`](#xcoffsectioniterator64)
  - [`XcoffSection32`](#xcoffsection32)
  - [`XcoffSection64`](#xcoffsection64)
  - [`XcoffSymbolTable32`](#xcoffsymboltable32)
  - [`XcoffSymbolTable64`](#xcoffsymboltable64)
  - [`XcoffSymbolIterator32`](#xcoffsymboliterator32)
  - [`XcoffSymbolIterator64`](#xcoffsymboliterator64)
  - [`XcoffSymbol32`](#xcoffsymbol32)
  - [`XcoffSymbol64`](#xcoffsymbol64)
  - [`XcoffRelocationIterator32`](#xcoffrelocationiterator32)
  - [`XcoffRelocationIterator64`](#xcoffrelocationiterator64)
  - [`XcoffComdatIterator32`](#xcoffcomdatiterator32)
  - [`XcoffComdatIterator64`](#xcoffcomdatiterator64)
  - [`XcoffComdat32`](#xcoffcomdat32)
  - [`XcoffComdat64`](#xcoffcomdat64)
  - [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32)
  - [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64)
  - [`XcoffSegmentIterator32`](#xcoffsegmentiterator32)
  - [`XcoffSegmentIterator64`](#xcoffsegmentiterator64)
  - [`XcoffSegment32`](#xcoffsegment32)
  - [`XcoffSegment64`](#xcoffsegment64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`file`](#file) | mod |  |
| [`section`](#section) | mod |  |
| [`symbol`](#symbol) | mod |  |
| [`relocation`](#relocation) | mod |  |
| [`comdat`](#comdat) | mod | XCOFF doesn't support the COMDAT section. |
| [`segment`](#segment) | mod | TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready. |
| [`XcoffFile`](#xcofffile) | struct | A partially parsed XCOFF file. |
| [`XcoffSectionIterator`](#xcoffsectioniterator) | struct | An iterator for the sections in an [`XcoffFile`]. |
| [`XcoffSection`](#xcoffsection) | struct | A section in an [`XcoffFile`]. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an XCOFF file. |
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in an XCOFF file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in an XCOFF file. |
| [`XcoffSymbolTable`](#xcoffsymboltable) | struct | A symbol table in an [`XcoffFile`]. |
| [`XcoffSymbolIterator`](#xcoffsymboliterator) | struct | An iterator for the symbols in an [`XcoffFile`]. |
| [`XcoffSymbol`](#xcoffsymbol) | struct | A symbol in an [`XcoffFile`]. |
| [`XcoffRelocationIterator`](#xcoffrelocationiterator) | struct | An iterator for the relocations in an [`XcoffSection`](super::XcoffSection). |
| [`XcoffComdatIterator`](#xcoffcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`XcoffFile`]. |
| [`XcoffComdat`](#xcoffcomdat) | struct | A COMDAT section group in a [`XcoffFile`]. |
| [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`XcoffFile`]. |
| [`XcoffSegmentIterator`](#xcoffsegmentiterator) | struct | An iterator for the segments in an [`XcoffFile`]. |
| [`XcoffSegment`](#xcoffsegment) | struct | A loadable section in an [`XcoffFile`]. |
| [`FileHeader`](#fileheader) | trait | A trait for generic access to [`xcoff::FileHeader32`] and [`xcoff::FileHeader64`]. |
| [`AuxHeader`](#auxheader) | trait | A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`]. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`]. |
| [`Symbol`](#symbol) | trait | A trait for generic access to [`xcoff::Symbol32`] and [`xcoff::Symbol64`]. |
| [`FileAux`](#fileaux) | trait | A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`]. |
| [`CsectAux`](#csectaux) | trait | A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`]. |
| [`Rel`](#rel) | trait | A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`]. |
| [`XcoffFile32`](#xcofffile32) | type | A 32-bit XCOFF object file. |
| [`XcoffFile64`](#xcofffile64) | type | A 64-bit XCOFF object file. |
| [`XcoffSectionIterator32`](#xcoffsectioniterator32) | type | An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSectionIterator64`](#xcoffsectioniterator64) | type | An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSection32`](#xcoffsection32) | type | A section in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSection64`](#xcoffsection64) | type | A section in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolTable32`](#xcoffsymboltable32) | type | A symbol table in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolTable64`](#xcoffsymboltable64) | type | A symbol table in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolIterator32`](#xcoffsymboliterator32) | type | An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolIterator64`](#xcoffsymboliterator64) | type | An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbol32`](#xcoffsymbol32) | type | A symbol in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbol64`](#xcoffsymbol64) | type | A symbol in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffRelocationIterator32`](#xcoffrelocationiterator32) | type | An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32). |
| [`XcoffRelocationIterator64`](#xcoffrelocationiterator64) | type | An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64). |
| [`XcoffComdatIterator32`](#xcoffcomdatiterator32) | type | An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatIterator64`](#xcoffcomdatiterator64) | type | An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdat32`](#xcoffcomdat32) | type | A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdat64`](#xcoffcomdat64) | type | A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegmentIterator32`](#xcoffsegmentiterator32) | type | An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegmentIterator64`](#xcoffsegmentiterator64) | type | An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSegment32`](#xcoffsegment32) | type | A segment in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSegment64`](#xcoffsegment64) | type | A segment in an [`XcoffFile64`](super::XcoffFile64). |

## Modules

- [`file`](file/index.md)
- [`section`](section/index.md)
- [`symbol`](symbol/index.md)
- [`relocation`](relocation/index.md)
- [`comdat`](comdat/index.md) — XCOFF doesn't support the COMDAT section.
- [`segment`](segment/index.md) — TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

## Structs

### `XcoffFile<'data, Xcoff, R>`

```rust
struct XcoffFile<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    data: R,
    header: &'data Xcoff,
    aux_header: Option<&'data <Xcoff as >::AuxHeader>,
    sections: super::SectionTable<'data, Xcoff>,
    symbols: super::SymbolTable<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:35-45`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/file.rs#L35-L45)*

A partially parsed XCOFF file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="xcofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw XCOFF file data.

- <span id="xcofffile-data"></span>`fn data(&self) -> R`

  Returns the raw data.

- <span id="xcofffile-raw-header"></span>`fn raw_header(&self) -> &'data Xcoff`

  Returns the raw XCOFF file header.

- <span id="xcofffile-xcoff-header"></span>`fn xcoff_header(&self) -> &'data Xcoff`

  Get the raw XCOFF file header.

- <span id="xcofffile-xcoff-aux-header"></span>`fn xcoff_aux_header(&self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](#fileheader)

  Get the raw XCOFF auxiliary header.

- <span id="xcofffile-xcoff-section-table"></span>`fn xcoff_section_table(&self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

  Get the XCOFF section table.

- <span id="xcofffile-xcoff-symbol-table"></span>`fn xcoff_symbol_table(&self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](#symboltable)

  Get the XCOFF symbol table.

#### Trait Implementations

##### `impl Any for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> Object for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-object-type-segment"></span>`type Segment = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-segmentiterator"></span>`type SegmentIterator = XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-section"></span>`type Section = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-sectioniterator"></span>`type SectionIterator = XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-comdat"></span>`type Comdat = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-comdatiterator"></span>`type ComdatIterator = XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-symboltable"></span>`type SymbolTable = XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcofffile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="xcofffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="xcofffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="xcofffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="xcofffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="xcofffile-object-segments"></span>`fn segments(&self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](#xcoffsegmentiterator)

- <span id="xcofffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](#xcoffsection)

- <span id="xcofffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`XcoffSection`](#xcoffsection)

- <span id="xcofffile-object-sections"></span>`fn sections(&self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](#xcoffsectioniterator)

- <span id="xcofffile-object-comdats"></span>`fn comdats(&self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](#xcoffcomdatiterator)

- <span id="xcofffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- <span id="xcofffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`XcoffSymbol`](#xcoffsymbol)

- <span id="xcofffile-object-symbols"></span>`fn symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- <span id="xcofffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- <span id="xcofffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- <span id="xcofffile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../index.md#object)

- <span id="xcofffile-object-imports"></span>`fn imports(&self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="xcofffile-object-exports"></span>`fn exports(&self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="xcofffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="xcofffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="xcofffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="xcofffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<Xcoff, R> Sealed for XcoffFile<'data, Xcoff, R>`

##### `impl<U> TryFrom for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcofffile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcofffile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Xcoff as >::SectionHeader>>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:23-30`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L23-L30)*

An iterator for the sections in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl Any for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-iterator-type-item"></span>`type Item = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSection<'data, 'file, Xcoff, R>`

```rust
struct XcoffSection<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    section: &'data <Xcoff as >::SectionHeader,
    index: crate::read::SectionIndex,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:59-67`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L59-L67)*

A section in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- <span id="xcoffsection-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

  Get the XCOFF file containing this section.

- <span id="xcoffsection-xcoff-section"></span>`fn xcoff_section(&self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](#fileheader)

  Get the raw XCOFF section header.

- <span id="xcoffsection-xcoff-relocations"></span>`fn xcoff_relocations(&self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the raw XCOFF relocation entries for this section.

- <span id="xcoffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl Any for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="xcoffsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="xcoffsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="xcoffsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="xcoffsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="xcoffsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="xcoffsection-objectsection-relocations"></span>`fn relocations(&self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../index.md#objectsection)

- <span id="xcoffsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="xcoffsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

- <span id="xcoffsection-objectsection-uncompressed-data"></span>`fn uncompressed_data(&self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../index.md#result)

##### `impl<Xcoff, R> Sealed for XcoffSection<'data, 'file, Xcoff, R>`

##### `impl<U> TryFrom for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionTable<'data, Xcoff: FileHeader>`

```rust
struct SectionTable<'data, Xcoff: FileHeader> {
    sections: &'data [<Xcoff as >::SectionHeader],
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:228-230`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L228-L230)*

The table of section headers in an XCOFF file.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](#fileheader)

  Iterate over the section headers.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Return the section header at the given index.

  

  The index is 1-based.

#### Trait Implementations

##### `impl Any for SectionTable<'data, Xcoff>`

- <span id="sectiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionTable<'data, Xcoff>`

- <span id="sectiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionTable<'data, Xcoff>`

- <span id="sectiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff: clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

##### `impl CloneToUninit for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Xcoff: marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<Xcoff: fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff> Default for SectionTable<'data, Xcoff>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

##### `impl<T> From for SectionTable<'data, Xcoff>`

- <span id="sectiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionTable<'data, Xcoff>`

- <span id="sectiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SectionTable<'data, Xcoff>`

- <span id="sectiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="sectiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionTable<'data, Xcoff>`

- <span id="sectiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionTable<'data, Xcoff>`

- <span id="sectiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolTable<'data, Xcoff, R>`

```rust
struct SymbolTable<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'data [xcoff::SymbolBytes],
    strings: crate::read::StringTable<'data, R>,
    header: core::marker::PhantomData<Xcoff>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:23-31`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L23-L31)*

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the symbol table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

  Iterate over the symbols.

  

  This does not return null symbols.

- <span id="symboltable-iter-none"></span>`fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

  Empty symbol iterator.

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result)

  Return the symbol entry at the given index and offset.

- <span id="symboltable-symbol-unchecked"></span>`fn symbol_unchecked(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the symbol at the given index.

  

  This does not check if the symbol is null, but does check if the index is in bounds.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Get the symbol at the given index.

  

  Returns an error for null symbols and out of bounds indices.

  Note that this is unable to check whether the index is an auxiliary symbol.

- <span id="symboltable-aux-file"></span>`fn aux_file(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Return a file auxiliary symbol.

- <span id="symboltable-aux-csect"></span>`fn aux_csect(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`FileHeader`](#fileheader)

  Return the csect auxiliary symbol.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbol table entries.

  

  This includes auxiliary symbol table entries.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

##### `impl<T> From for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolIterator<'data, 'table, Xcoff, R>`

```rust
struct SymbolIterator<'data, 'table, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'table SymbolTable<'data, Xcoff, R>,
    index: usize,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:182-189`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L182-L189)*

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl Any for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSymbolTable<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolTable<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:217-224`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L217-L224)*

A symbol table in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl Any for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clone"></span>`fn clone(&self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](#xcoffsymboltable)

##### `impl CloneToUninit for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="xcoffsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl ToOwned for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-toowned-type-owned"></span>`type Owned = T`

- <span id="xcoffsymboltable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="xcoffsymboltable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsymboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsymboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSymbolIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: SymbolIterator<'data, 'file, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:263-270`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L263-L270)*

An iterator for the symbols in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl Any for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-iterator-type-item"></span>`type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsymboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsymboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSymbol<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbol<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Xcoff as >::Symbol,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:307-316`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L307-L316)*

A symbol in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- <span id="xcoffsymbol-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

  Get the XCOFF file containing this symbol.

- <span id="xcoffsymbol-xcoff-symbol"></span>`fn xcoff_symbol(&self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](#fileheader)

  Get the raw XCOFF symbol structure.

#### Trait Implementations

##### `impl Any for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clone"></span>`fn clone(&self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](#xcoffsymbol)

##### `impl CloneToUninit for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="xcoffsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsymbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="xcoffsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="xcoffsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="xcoffsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- <span id="xcoffsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="xcoffsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl ToOwned for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-toowned-type-owned"></span>`type Owned = T`

- <span id="xcoffsymbol-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="xcoffsymbol-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsymbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsymbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffRelocationIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffRelocationIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    relocations: slice::Iter<'data, <<Xcoff as FileHeader>::SectionHeader as SectionHeader>::Rel>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:23-32`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L23-L32)*

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

#### Trait Implementations

##### `impl Any for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="xcoffrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffComdatIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:21-28`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L21-L28)*

An iterator for the COMDAT section groups in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-type-item"></span>`type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffcomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffcomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:55-62`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L55-L62)*

A COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="xcoffcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="xcoffcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffcomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="xcoffcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md#objectcomdat)

##### `impl<Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

##### `impl<U> TryFrom for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffcomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffcomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:115-122`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L115-L122)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="xcoffcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffcomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffcomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSegmentIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegmentIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:22-29`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/segment.rs#L22-L29)*

An iterator for the segments in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-type-item"></span>`type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:54-61`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/segment.rs#L54-L61)*

A loadable section in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="xcoffsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-data-range"></span>`fn data_range(&self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="xcoffsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

##### `impl<U> TryFrom for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcoffsegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:306-387`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/file.rs#L306-L387)*

A trait for generic access to [`xcoff::FileHeader32`](../../xcoff/index.md) and [`xcoff::FileHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type AuxHeader: 1`

- `type SectionHeader: 1`

- `type Symbol: 1`

- `type FileAux: 1`

- `type CsectAux: 1`

- `type Rel: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn f_magic(&self) -> u16`

- `fn f_nscns(&self) -> u16`

- `fn f_timdat(&self) -> u32`

- `fn f_symptr(&self) -> <Self as >::Word`

- `fn f_nsyms(&self) -> u32`

- `fn f_opthdr(&self) -> u16`

- `fn f_flags(&self) -> u16`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn aux_header<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<Option<&'data <Self as >::AuxHeader>>`

  Read the auxiliary file header.

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<SectionTable<'data, Self>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> Result<SymbolTable<'data, Self, R>>`

  Return the symbol table.

#### Implementors

- [`FileHeader32`](../../xcoff/index.md#fileheader32)
- [`FileHeader64`](../../xcoff/index.md#fileheader64)

### `AuxHeader`

```rust
trait AuxHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:475-508`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/file.rs#L475-L508)*

A trait for generic access to [`xcoff::AuxHeader32`](../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn o_mflag(&self) -> u16`

- `fn o_vstamp(&self) -> u16`

- `fn o_tsize(&self) -> <Self as >::Word`

- `fn o_dsize(&self) -> <Self as >::Word`

- `fn o_bsize(&self) -> <Self as >::Word`

- `fn o_entry(&self) -> <Self as >::Word`

- `fn o_text_start(&self) -> <Self as >::Word`

- `fn o_data_start(&self) -> <Self as >::Word`

- `fn o_toc(&self) -> <Self as >::Word`

- `fn o_snentry(&self) -> u16`

- `fn o_sntext(&self) -> u16`

- `fn o_sndata(&self) -> u16`

- `fn o_sntoc(&self) -> u16`

- `fn o_snloader(&self) -> u16`

- `fn o_snbss(&self) -> u16`

- `fn o_algntext(&self) -> u16`

- `fn o_algndata(&self) -> u16`

- `fn o_modtype(&self) -> u16`

- `fn o_cpuflag(&self) -> u8`

- `fn o_cputype(&self) -> u8`

- `fn o_maxstack(&self) -> <Self as >::Word`

- `fn o_maxdata(&self) -> <Self as >::Word`

- `fn o_debugger(&self) -> u32`

- `fn o_textpsize(&self) -> u8`

- `fn o_datapsize(&self) -> u8`

- `fn o_stackpsize(&self) -> u8`

- `fn o_flags(&self) -> u8`

- `fn o_sntdata(&self) -> u16`

- `fn o_sntbss(&self) -> u16`

- `fn o_x64flags(&self) -> Option<u16>`

#### Implementors

- [`AuxHeader32`](../../xcoff/index.md#auxheader32)
- [`AuxHeader64`](../../xcoff/index.md#auxheader64)

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:290-335`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L290-L335)*

A trait for generic access to [`xcoff::SectionHeader32`](../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type HalfWord: 1`

- `type Xcoff: 1`

- `type Rel: 1`

#### Required Methods

- `fn s_name(&self) -> &[u8; 8]`

- `fn s_paddr(&self) -> <Self as >::Word`

- `fn s_vaddr(&self) -> <Self as >::Word`

- `fn s_size(&self) -> <Self as >::Word`

- `fn s_scnptr(&self) -> <Self as >::Word`

- `fn s_relptr(&self) -> <Self as >::Word`

- `fn s_lnnoptr(&self) -> <Self as >::Word`

- `fn s_nreloc(&self) -> <Self as >::HalfWord`

- `fn s_nlnno(&self) -> <Self as >::HalfWord`

- `fn s_flags(&self) -> u32`

- `fn relocations<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [<Self as >::Rel]>`

  Read the relocations.

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the section name.

- `fn file_range(&self) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

#### Implementors

- [`SectionHeader32`](../../xcoff/index.md#sectionheader32)
- [`SectionHeader64`](../../xcoff/index.md#sectionheader64)

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:540-593`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L540-L593)*

A trait for generic access to [`xcoff::Symbol32`](../../xcoff/index.md) and [`xcoff::Symbol64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn n_value(&self) -> <Self as >::Word`

- `fn n_scnum(&self) -> i16`

- `fn n_type(&self) -> u16`

- `fn n_sclass(&self) -> u8`

- `fn n_numaux(&self) -> u8`

- `fn name_offset(&self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

#### Provided Methods

- `fn section(&self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_null(&self) -> bool`

  Return true if the symbol is a null placeholder.

- `fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn has_aux_file(&self) -> bool`

  Return true if the symbol has file auxiliary entry.

- `fn has_aux_csect(&self) -> bool`

  Return true if the symbol has csect auxiliary entry.

#### Implementors

- [`Symbol32`](../../xcoff/index.md#symbol32)
- [`Symbol64`](../../xcoff/index.md#symbol64)

### `FileAux`

```rust
trait FileAux: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:687-720`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L687-L720)*

A trait for generic access to [`xcoff::FileAux32`](../../xcoff/index.md) and [`xcoff::FileAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_fname(&self) -> &[u8; 8]`

- `fn x_ftype(&self) -> u8`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn name_offset(&self) -> Option<u32>`

- `fn fname<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse the x_fname field, which may be an inline string or a string table offset.

#### Implementors

- [`FileAux32`](../../xcoff/index.md#fileaux32)
- [`FileAux64`](../../xcoff/index.md#fileaux64)

### `CsectAux`

```rust
trait CsectAux: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:752-768`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L752-L768)*

A trait for generic access to [`xcoff::CsectAux32`](../../xcoff/index.md) and [`xcoff::CsectAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_scnlen(&self) -> u64`

- `fn x_parmhash(&self) -> u32`

- `fn x_snhash(&self) -> u16`

- `fn x_smtyp(&self) -> u8`

- `fn x_smclas(&self) -> u8`

- `fn x_stab(&self) -> Option<u32>`

- `fn x_snstab(&self) -> Option<u16>`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn alignment(&self) -> u8`

- `fn sym_type(&self) -> u8`

#### Implementors

- [`CsectAux32`](../../xcoff/index.md#csectaux32)
- [`CsectAux64`](../../xcoff/index.md#csectaux64)

### `Rel`

```rust
trait Rel: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:88-98`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L88-L98)*

A trait for generic access to [`xcoff::Rel32`](../../xcoff/index.md) and [`xcoff::Rel64`](../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn r_vaddr(&self) -> <Self as >::Word`

- `fn r_symndx(&self) -> u32`

- `fn r_rsize(&self) -> u8`

- `fn r_rtype(&self) -> u8`

#### Provided Methods

- `fn symbol(&self) -> SymbolIndex`

#### Implementors

- [`Rel32`](../../xcoff/index.md#rel32)
- [`Rel64`](../../xcoff/index.md#rel64)

## Type Aliases

### `XcoffFile32<'data, R>`

```rust
type XcoffFile32<'data, R> = XcoffFile<'data, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:24`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/file.rs#L24)*

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff32`](../../index.md).

### `XcoffFile64<'data, R>`

```rust
type XcoffFile64<'data, R> = XcoffFile<'data, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:29`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/file.rs#L29)*

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff64`](../../index.md).

### `XcoffSectionIterator32<'data, 'file, R>`

```rust
type XcoffSectionIterator32<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:15-16`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L15-L16)*

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSectionIterator64<'data, 'file, R>`

```rust
type XcoffSectionIterator64<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:18-19`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L18-L19)*

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSection32<'data, 'file, R>`

```rust
type XcoffSection32<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:49-50`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L49-L50)*

A section in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSection64<'data, 'file, R>`

```rust
type XcoffSection64<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:52-53`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L52-L53)*

A section in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolTable32<'data, 'file, R>`

```rust
type XcoffSymbolTable32<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:209-210`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L209-L210)*

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:212-213`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L212-L213)*

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:256-257`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L256-L257)*

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:259-260`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L259-L260)*

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:297-298`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L297-L298)*

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:300-301`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/symbol.rs#L300-L301)*

A symbol in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffRelocationIterator32<'data, 'file, R>`

```rust
type XcoffRelocationIterator32<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:16-17`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L16-L17)*

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).

### `XcoffRelocationIterator64<'data, 'file, R>`

```rust
type XcoffRelocationIterator64<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:19-20`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L19-L20)*

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:11-12`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L11-L12)*

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:14-15`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L14-L15)*

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:44-45`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L44-L45)*

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:48-49`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L48-L49)*

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:105-106`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L105-L106)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:108-109`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L108-L109)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:12-13`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/segment.rs#L12-L13)*

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:15-16`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/segment.rs#L15-L16)*

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:44-45`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/segment.rs#L44-L45)*

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/segment.rs:47-48`](../../../../.source_1765633015/object-0.37.3/src/read/xcoff/segment.rs#L47-L48)*

A segment in an [`XcoffFile64`](super::XcoffFile64).

