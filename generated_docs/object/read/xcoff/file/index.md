*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [file](index.md)*

---

# Module `file`

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

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn data(self: &Self) -> R`

- `fn raw_header(self: &Self) -> &'data Xcoff`

- `fn xcoff_header(self: &Self) -> &'data Xcoff`

- `fn xcoff_aux_header(self: &Self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](../index.md)

- `fn xcoff_section_table(self: &Self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](../index.md)

- `fn xcoff_symbol_table(self: &Self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](../index.md)

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

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- `fn segments(self: &Self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](../index.md)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](../index.md)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`XcoffSection`](../index.md)

- `fn sections(self: &Self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](../index.md)

- `fn comdats(self: &Self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](../index.md)

- `fn symbol_table(self: &Self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`XcoffSymbol`](../index.md)

- `fn symbols(self: &Self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](../index.md)

- `fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](../index.md)

- `fn dynamic_symbols(self: &Self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](../index.md)

- `fn dynamic_relocations(self: &Self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../../index.md)

- `fn imports(self: &Self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- `fn exports(self: &Self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../../index.md)

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

A trait for generic access to [`xcoff::AuxHeader32`](../../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../../xcoff/index.md).

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

