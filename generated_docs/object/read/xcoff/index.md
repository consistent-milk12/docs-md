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

## Modules

- [`file`](file/index.md) - 
- [`section`](section/index.md) - 
- [`symbol`](symbol/index.md) - 
- [`relocation`](relocation/index.md) - 
- [`comdat`](comdat/index.md) - XCOFF doesn't support the COMDAT section.
- [`segment`](segment/index.md) - TODO: Support the segment for XCOFF when auxiliary file header and loader section is ready.

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

A partially parsed XCOFF file.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn data(self: &Self) -> R`

- `fn raw_header(self: &Self) -> &'data Xcoff`

- `fn xcoff_header(self: &Self) -> &'data Xcoff`

- `fn xcoff_aux_header(self: &Self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](#fileheader)

- `fn xcoff_section_table(self: &Self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

- `fn xcoff_symbol_table(self: &Self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](#symboltable)

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for XcoffFile<'data, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Xcoff, R> Object for XcoffFile<'data, Xcoff, R>`

- `type Segment = XcoffSegment<'data, 'file, Xcoff, R>`

- `type SegmentIterator = XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `type Section = XcoffSection<'data, 'file, Xcoff, R>`

- `type SectionIterator = XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `type Comdat = XcoffComdat<'data, 'file, Xcoff, R>`

- `type ComdatIterator = XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- `type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `type SymbolTable = XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../index.md)

- `fn segments(self: &Self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](#xcoffsegmentiterator)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](#xcoffsection)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`XcoffSection`](#xcoffsection)

- `fn sections(self: &Self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](#xcoffsectioniterator)

- `fn comdats(self: &Self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](#xcoffcomdatiterator)

- `fn symbol_table(self: &Self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`XcoffSymbol`](#xcoffsymbol)

- `fn symbols(self: &Self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- `fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](#xcoffsymboltable)

- `fn dynamic_symbols(self: &Self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](#xcoffsymboliterator)

- `fn dynamic_relocations(self: &Self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../index.md)

- `fn imports(self: &Self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../index.md), [`Import`](../../index.md)

- `fn exports(self: &Self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../index.md), [`Export`](../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../index.md)

##### `impl<'data, Xcoff, R> Sealed for XcoffFile<'data, Xcoff, R>`

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

An iterator for the sections in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffSection<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A section in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Implementations

- `fn xcoff_file(self: &Self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

- `fn xcoff_section(self: &Self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](#fileheader)

- `fn xcoff_relocations(self: &Self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- `type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../index.md), [`CompressedFileRange`](../../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md), [`CompressedData`](../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../index.md)

- `fn relocations(self: &Self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../index.md)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../index.md), [`RelocationMap`](../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../index.md)

- `fn uncompressed_data(self: &Self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffSection<'data, 'file, Xcoff, R>`

### `SectionTable<'data, Xcoff: FileHeader>`

```rust
struct SectionTable<'data, Xcoff: FileHeader> {
    sections: &'data [<Xcoff as >::SectionHeader],
}
```

The table of section headers in an XCOFF file.

Returned by `FileHeader::sections`.

#### Implementations

- `fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](#fileheader)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, Xcoff: $crate::clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- `fn clone(self: &Self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](#sectiontable)

##### `impl<'data, Xcoff: $crate::marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<'data, Xcoff: $crate::fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Xcoff> Default for SectionTable<'data, Xcoff>`

- `fn default() -> Self`

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

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- `fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../index.md)

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

- `fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](#symboliterator)

- `fn get<T: Pod>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md)

- `fn symbol_unchecked(self: &Self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn aux_file(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn aux_csect(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`FileHeader`](#fileheader)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- `fn default() -> Self`

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

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<'data, 'table, Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'table, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- `type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A symbol table in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `fn clone(self: &Self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](#xcoffsymboltable)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- `type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md), [`Result`](../../index.md), [`ObjectSymbolTable`](../index.md)

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbolTable<'data, 'file, Xcoff, R>`

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

An iterator for the symbols in an [`XcoffFile`](#xcofffile).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A symbol in an [`XcoffFile`](#xcofffile).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Implementations

- `fn xcoff_file(self: &Self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](#xcofffile)

- `fn xcoff_symbol(self: &Self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn clone(self: &Self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](#xcoffsymbol)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

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

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

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

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffComdatIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the COMDAT section groups in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- `type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffSegmentIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegmentIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the segments in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffSegment<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffSegment<'data, 'file, Xcoff, R>`

```rust
struct XcoffSegment<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A loadable section in an [`XcoffFile`](#xcofffile).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSegment<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectSegment for XcoffSegment<'data, 'file, Xcoff, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../index.md)

- `fn data_range(self: &Self, _address: u64, _size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffSegment<'data, 'file, Xcoff, R>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::FileHeader32`](../../xcoff/index.md) and [`xcoff::FileHeader64`](../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `type AuxHeader: 1`

- `type SectionHeader: 1`

- `type Symbol: 1`

- `type FileAux: 1`

- `type CsectAux: 1`

- `type Rel: 1`

- `fn is_type_64(self: &Self) -> bool`

  Return true if this type is a 64-bit header.

- `fn f_magic(self: &Self) -> u16`

- `fn f_nscns(self: &Self) -> u16`

- `fn f_timdat(self: &Self) -> u32`

- `fn f_symptr(self: &Self) -> <Self as >::Word`

- `fn f_nsyms(self: &Self) -> u32`

- `fn f_opthdr(self: &Self) -> u16`

- `fn f_flags(self: &Self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>`

  Read the file header.

- `fn is_supported(self: &Self) -> bool`

- `fn aux_header<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: &mut u64) -> Result<Option<&'data <Self as >::AuxHeader>>`

  Read the auxiliary file header.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: &mut u64) -> Result<SectionTable<'data, Self>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(self: &Self, data: R) -> Result<SymbolTable<'data, Self, R>>`

  Return the symbol table.

### `AuxHeader`

```rust
trait AuxHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::AuxHeader32`](../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `fn o_mflag(self: &Self) -> u16`

- `fn o_vstamp(self: &Self) -> u16`

- `fn o_tsize(self: &Self) -> <Self as >::Word`

- `fn o_dsize(self: &Self) -> <Self as >::Word`

- `fn o_bsize(self: &Self) -> <Self as >::Word`

- `fn o_entry(self: &Self) -> <Self as >::Word`

- `fn o_text_start(self: &Self) -> <Self as >::Word`

- `fn o_data_start(self: &Self) -> <Self as >::Word`

- `fn o_toc(self: &Self) -> <Self as >::Word`

- `fn o_snentry(self: &Self) -> u16`

- `fn o_sntext(self: &Self) -> u16`

- `fn o_sndata(self: &Self) -> u16`

- `fn o_sntoc(self: &Self) -> u16`

- `fn o_snloader(self: &Self) -> u16`

- `fn o_snbss(self: &Self) -> u16`

- `fn o_algntext(self: &Self) -> u16`

- `fn o_algndata(self: &Self) -> u16`

- `fn o_modtype(self: &Self) -> u16`

- `fn o_cpuflag(self: &Self) -> u8`

- `fn o_cputype(self: &Self) -> u8`

- `fn o_maxstack(self: &Self) -> <Self as >::Word`

- `fn o_maxdata(self: &Self) -> <Self as >::Word`

- `fn o_debugger(self: &Self) -> u32`

- `fn o_textpsize(self: &Self) -> u8`

- `fn o_datapsize(self: &Self) -> u8`

- `fn o_stackpsize(self: &Self) -> u8`

- `fn o_flags(self: &Self) -> u8`

- `fn o_sntdata(self: &Self) -> u16`

- `fn o_sntbss(self: &Self) -> u16`

- `fn o_x64flags(self: &Self) -> Option<u16>`

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::SectionHeader32`](../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `type HalfWord: 1`

- `type Xcoff: 1`

- `type Rel: 1`

- `fn s_name(self: &Self) -> &[u8; 8]`

- `fn s_paddr(self: &Self) -> <Self as >::Word`

- `fn s_vaddr(self: &Self) -> <Self as >::Word`

- `fn s_size(self: &Self) -> <Self as >::Word`

- `fn s_scnptr(self: &Self) -> <Self as >::Word`

- `fn s_relptr(self: &Self) -> <Self as >::Word`

- `fn s_lnnoptr(self: &Self) -> <Self as >::Word`

- `fn s_nreloc(self: &Self) -> <Self as >::HalfWord`

- `fn s_nlnno(self: &Self) -> <Self as >::HalfWord`

- `fn s_flags(self: &Self) -> u32`

- `fn name(self: &Self) -> &[u8]`

  Return the section name.

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(self: &Self, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<&'data [<Self as >::Rel]>`

  Read the relocations.

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::Symbol32`](../../xcoff/index.md) and [`xcoff::Symbol64`](../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `fn n_value(self: &Self) -> <Self as >::Word`

- `fn n_scnum(self: &Self) -> i16`

- `fn n_type(self: &Self) -> u16`

- `fn n_sclass(self: &Self) -> u8`

- `fn n_numaux(self: &Self) -> u8`

- `fn name_offset(self: &Self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

- `fn section(self: &Self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_null(self: &Self) -> bool`

  Return true if the symbol is a null placeholder.

- `fn is_undefined(self: &Self) -> bool`

  Return true if the symbol is undefined.

- `fn has_aux_file(self: &Self) -> bool`

  Return true if the symbol has file auxiliary entry.

- `fn has_aux_csect(self: &Self) -> bool`

  Return true if the symbol has csect auxiliary entry.

### `FileAux`

```rust
trait FileAux: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::FileAux32`](../../xcoff/index.md) and [`xcoff::FileAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_fname(self: &Self) -> &[u8; 8]`

- `fn x_ftype(self: &Self) -> u8`

- `fn x_auxtype(self: &Self) -> Option<u8>`

- `fn name_offset(self: &Self) -> Option<u32>`

- `fn fname<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse the x_fname field, which may be an inline string or a string table offset.

### `CsectAux`

```rust
trait CsectAux: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::CsectAux32`](../../xcoff/index.md) and [`xcoff::CsectAux64`](../../xcoff/index.md).

#### Required Methods

- `fn x_scnlen(self: &Self) -> u64`

- `fn x_parmhash(self: &Self) -> u32`

- `fn x_snhash(self: &Self) -> u16`

- `fn x_smtyp(self: &Self) -> u8`

- `fn x_smclas(self: &Self) -> u8`

- `fn x_stab(self: &Self) -> Option<u32>`

- `fn x_snstab(self: &Self) -> Option<u16>`

- `fn x_auxtype(self: &Self) -> Option<u8>`

- `fn alignment(self: &Self) -> u8`

- `fn sym_type(self: &Self) -> u8`

### `Rel`

```rust
trait Rel: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::Rel32`](../../xcoff/index.md) and [`xcoff::Rel64`](../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `fn r_vaddr(self: &Self) -> <Self as >::Word`

- `fn r_symndx(self: &Self) -> u32`

- `fn r_rsize(self: &Self) -> u8`

- `fn r_rtype(self: &Self) -> u8`

- `fn symbol(self: &Self) -> SymbolIndex`

## Type Aliases

### `XcoffFile32<'data, R>`

```rust
type XcoffFile32<'data, R> = XcoffFile<'data, xcoff::FileHeader32, R>;
```

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff32`](../../index.md).

### `XcoffFile64<'data, R>`

```rust
type XcoffFile64<'data, R> = XcoffFile<'data, xcoff::FileHeader64, R>;
```

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`](../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff64`](../../index.md).

### `XcoffSectionIterator32<'data, 'file, R>`

```rust
type XcoffSectionIterator32<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSectionIterator64<'data, 'file, R>`

```rust
type XcoffSectionIterator64<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSection32<'data, 'file, R>`

```rust
type XcoffSection32<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader32, R>;
```

A section in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSection64<'data, 'file, R>`

```rust
type XcoffSection64<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader64, R>;
```

A section in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolTable32<'data, 'file, R>`

```rust
type XcoffSymbolTable32<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>;
```

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

A symbol in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffRelocationIterator32<'data, 'file, R>`

```rust
type XcoffRelocationIterator32<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).

### `XcoffRelocationIterator64<'data, 'file, R>`

```rust
type XcoffRelocationIterator64<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegmentIterator32<'data, 'file, R>`

```rust
type XcoffSegmentIterator32<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegmentIterator64<'data, 'file, R>`

```rust
type XcoffSegmentIterator64<'data, 'file, R> = XcoffSegmentIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSegment32<'data, 'file, R>`

```rust
type XcoffSegment32<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader32, R>;
```

A segment in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSegment64<'data, 'file, R>`

```rust
type XcoffSegment64<'data, 'file, R> = XcoffSegment<'data, 'file, xcoff::FileHeader64, R>;
```

A segment in an [`XcoffFile64`](super::XcoffFile64).

