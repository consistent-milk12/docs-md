*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [file](index.md)*

---

# Module `file`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffFile`](#xcofffile) | struct | A partially parsed XCOFF file. |
| [`FileHeader`](#fileheader) | trait | A trait for generic access to [`xcoff::FileHeader32`] and [`xcoff::FileHeader64`]. |
| [`AuxHeader`](#auxheader) | trait | A trait for generic access to [`xcoff::AuxHeader32`] and [`xcoff::AuxHeader64`]. |
| [`XcoffFile32`](#xcofffile32) | type | A 32-bit XCOFF object file. |
| [`XcoffFile64`](#xcofffile64) | type | A 64-bit XCOFF object file. |

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

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="xcofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- <span id="xcofffile-data"></span>`fn data(&self) -> R`

- <span id="xcofffile-raw-header"></span>`fn raw_header(&self) -> &'data Xcoff`

- <span id="xcofffile-xcoff-header"></span>`fn xcoff_header(&self) -> &'data Xcoff`

- <span id="xcofffile-xcoff-aux-header"></span>`fn xcoff_aux_header(&self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](../index.md)

- <span id="xcofffile-xcoff-section-table"></span>`fn xcoff_section_table(&self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](../index.md)

- <span id="xcofffile-xcoff-symbol-table"></span>`fn xcoff_symbol_table(&self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](../index.md)

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Xcoff, R> Object for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-segment"></span>`type Segment = XcoffSegment<'data, 'file, Xcoff, R>`

- <span id="xcofffile-segmentiterator"></span>`type SegmentIterator = XcoffSegmentIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-section"></span>`type Section = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcofffile-sectioniterator"></span>`type SectionIterator = XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-comdat"></span>`type Comdat = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcofffile-comdatiterator"></span>`type ComdatIterator = XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcofffile-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcofffile-symboltable"></span>`type SymbolTable = XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcofffile-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="xcofffile-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md)

- <span id="xcofffile-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="xcofffile-is-64"></span>`fn is_64(&self) -> bool`

- <span id="xcofffile-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- <span id="xcofffile-segments"></span>`fn segments(&self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](../index.md)

- <span id="xcofffile-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](../index.md)

- <span id="xcofffile-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`XcoffSection`](../index.md)

- <span id="xcofffile-sections"></span>`fn sections(&self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](../index.md)

- <span id="xcofffile-comdats"></span>`fn comdats(&self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](../index.md)

- <span id="xcofffile-symbol-table"></span>`fn symbol_table(&self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](../index.md)

- <span id="xcofffile-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`XcoffSymbol`](../index.md)

- <span id="xcofffile-symbols"></span>`fn symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](../index.md)

- <span id="xcofffile-dynamic-symbol-table"></span>`fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](../index.md)

- <span id="xcofffile-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](../index.md)

- <span id="xcofffile-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../../index.md)

- <span id="xcofffile-imports"></span>`fn imports(&self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- <span id="xcofffile-exports"></span>`fn exports(&self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- <span id="xcofffile-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="xcofffile-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="xcofffile-entry"></span>`fn entry(&self) -> u64`

- <span id="xcofffile-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md)

##### `impl<'data, Xcoff, R> Sealed for XcoffFile<'data, Xcoff, R>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::FileHeader32`](../../../xcoff/index.md) and [`xcoff::FileHeader64`](../../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `type AuxHeader: 1`

- `type SectionHeader: 1`

- `type Symbol: 1`

- `type FileAux: 1`

- `type CsectAux: 1`

- `type Rel: 1`

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn f_magic(&self) -> u16`

- `fn f_nscns(&self) -> u16`

- `fn f_timdat(&self) -> u32`

- `fn f_symptr(&self) -> <Self as >::Word`

- `fn f_nsyms(&self) -> u32`

- `fn f_opthdr(&self) -> u16`

- `fn f_flags(&self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn aux_header<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<Option<&'data <Self as >::AuxHeader>>`

  Read the auxiliary file header.

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<SectionTable<'data, Self>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> Result<SymbolTable<'data, Self, R>>`

  Return the symbol table.

### `AuxHeader`

```rust
trait AuxHeader: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::AuxHeader32`](../../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

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

## Type Aliases

### `XcoffFile32<'data, R>`

```rust
type XcoffFile32<'data, R> = XcoffFile<'data, xcoff::FileHeader32, R>;
```

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`](../../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff32`](../../../index.md).

### `XcoffFile64<'data, R>`

```rust
type XcoffFile64<'data, R> = XcoffFile<'data, xcoff::FileHeader64, R>;
```

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`](../../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff64`](../../../index.md).

