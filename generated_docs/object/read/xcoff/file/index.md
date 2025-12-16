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

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:35-45`](../../../../../.source_1765900590/object-0.37.3/src/read/xcoff/file.rs#L35-L45)*

A partially parsed XCOFF file.

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="xcofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the raw XCOFF file data.

- <span id="xcofffile-data"></span>`fn data(&self) -> R`

  Returns the raw data.

- <span id="xcofffile-raw-header"></span>`fn raw_header(&self) -> &'data Xcoff`

  Returns the raw XCOFF file header.

- <span id="xcofffile-xcoff-header"></span>`fn xcoff_header(&self) -> &'data Xcoff`

  Get the raw XCOFF file header.

- <span id="xcofffile-xcoff-aux-header"></span>`fn xcoff_aux_header(&self) -> Option<&'data <Xcoff as >::AuxHeader>` — [`FileHeader`](../index.md#fileheader)

  Get the raw XCOFF auxiliary header.

- <span id="xcofffile-xcoff-section-table"></span>`fn xcoff_section_table(&self) -> &SectionTable<'data, Xcoff>` — [`SectionTable`](../index.md#sectiontable)

  Get the XCOFF section table.

- <span id="xcofffile-xcoff-symbol-table"></span>`fn xcoff_symbol_table(&self) -> &SymbolTable<'data, Xcoff, R>` — [`SymbolTable`](../index.md#symboltable)

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

- <span id="xcofffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

- <span id="xcofffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="xcofffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="xcofffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md#objectkind)

- <span id="xcofffile-object-segments"></span>`fn segments(&self) -> XcoffSegmentIterator<'data, '_, Xcoff, R>` — [`XcoffSegmentIterator`](../index.md#xcoffsegmentiterator)

- <span id="xcofffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<XcoffSection<'data, 'file, Xcoff, R>>` — [`XcoffSection`](../index.md#xcoffsection)

- <span id="xcofffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<XcoffSection<'data, '_, Xcoff, R>>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`XcoffSection`](../index.md#xcoffsection)

- <span id="xcofffile-object-sections"></span>`fn sections(&self) -> XcoffSectionIterator<'data, '_, Xcoff, R>` — [`XcoffSectionIterator`](../index.md#xcoffsectioniterator)

- <span id="xcofffile-object-comdats"></span>`fn comdats(&self) -> XcoffComdatIterator<'data, '_, Xcoff, R>` — [`XcoffComdatIterator`](../index.md#xcoffcomdatiterator)

- <span id="xcofffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<XcoffSymbolTable<'data, '_, Xcoff, R>>` — [`XcoffSymbolTable`](../index.md#xcoffsymboltable)

- <span id="xcofffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<XcoffSymbol<'data, '_, Xcoff, R>>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`XcoffSymbol`](../index.md#xcoffsymbol)

- <span id="xcofffile-object-symbols"></span>`fn symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](../index.md#xcoffsymboliterator)

- <span id="xcofffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table<'file>(self: &'file Self) -> Option<XcoffSymbolTable<'data, 'file, Xcoff, R>>` — [`XcoffSymbolTable`](../index.md#xcoffsymboltable)

- <span id="xcofffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> XcoffSymbolIterator<'data, '_, Xcoff, R>` — [`XcoffSymbolIterator`](../index.md#xcoffsymboliterator)

- <span id="xcofffile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>` — [`Object`](../../index.md#object)

- <span id="xcofffile-object-imports"></span>`fn imports(&self) -> Result<alloc::vec::Vec<Import<'data>>>` — [`Result`](../../../index.md#result), [`Import`](../../../index.md#import)

- <span id="xcofffile-object-exports"></span>`fn exports(&self) -> Result<alloc::vec::Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../../../index.md#export)

- <span id="xcofffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="xcofffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="xcofffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="xcofffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md#fileflags)

##### `impl<Xcoff, R> Sealed for XcoffFile<'data, Xcoff, R>`

##### `impl<U> TryFrom for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcofffile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffFile<'data, Xcoff, R>`

- <span id="xcofffile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcofffile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `FileHeader`

```rust
trait FileHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:306-387`](../../../../../.source_1765900590/object-0.37.3/src/read/xcoff/file.rs#L306-L387)*

A trait for generic access to [`xcoff::FileHeader32`](../../../xcoff/index.md) and [`xcoff::FileHeader64`](../../../xcoff/index.md).

<details>
<summary><strong>Methods (13)</strong> - click to expand</summary>

**Required:**
- [`FileHeader::is_type_64`](#fn-fileheaderis-type-64)
- [`FileHeader::f_magic`](#fn-fileheaderf-magic)
- [`FileHeader::f_nscns`](#fn-fileheaderf-nscns)
- [`FileHeader::f_timdat`](#fn-fileheaderf-timdat)
- [`FileHeader::f_symptr`](#fn-fileheaderf-symptr)
- [`FileHeader::f_nsyms`](#fn-fileheaderf-nsyms)
- [`FileHeader::f_opthdr`](#fn-fileheaderf-opthdr)
- [`FileHeader::f_flags`](#fn-fileheaderf-flags)

**Provided:**
- [`FileHeader::parse`](#fn-fileheaderparse)
- [`FileHeader::is_supported`](#fn-fileheaderis-supported)
- [`FileHeader::aux_header`](#fn-fileheaderaux-header)
- [`FileHeader::sections`](#fn-fileheadersections)
- [`FileHeader::symbols`](#fn-fileheadersymbols)

</details>

#### Associated Types

- `type Word: 1`

- `type AuxHeader: 1`

- `type SectionHeader: 1`

- `type Symbol: 1`

- `type FileAux: 1`

- `type CsectAux: 1`

- `type Rel: 1`

#### Required Methods

- `fn FileHeader::is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn FileHeader::f_magic(&self) -> u16`

- `fn FileHeader::f_nscns(&self) -> u16`

- `fn FileHeader::f_timdat(&self) -> u32`

- `fn FileHeader::f_symptr(&self) -> <Self as >::Word`

- `fn FileHeader::f_nsyms(&self) -> u32`

- `fn FileHeader::f_opthdr(&self) -> u16`

- `fn FileHeader::f_flags(&self) -> u16`

#### Provided Methods

- `fn FileHeader::parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> Result<&'data Self>`

  Read the file header.
  
  Also checks that the magic field in the file header is a supported format.

- `fn FileHeader::is_supported(&self) -> bool`

- `fn FileHeader::aux_header<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<Option<&'data <Self as >::AuxHeader>>`

  Read the auxiliary file header.

- `fn FileHeader::sections<'data, R: ReadRef<'data>>(&self, data: R, offset: &mut u64) -> Result<SectionTable<'data, Self>>`

  Read the section table.

- `fn FileHeader::symbols<'data, R: ReadRef<'data>>(&self, data: R) -> Result<SymbolTable<'data, Self, R>>`

  Return the symbol table.

#### Implementors

- [`FileHeader32`](../../../xcoff/index.md#fileheader32)
- [`FileHeader64`](../../../xcoff/index.md#fileheader64)

### `AuxHeader`

```rust
trait AuxHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:475-508`](../../../../../.source_1765900590/object-0.37.3/src/read/xcoff/file.rs#L475-L508)*

A trait for generic access to [`xcoff::AuxHeader32`](../../../xcoff/index.md) and [`xcoff::AuxHeader64`](../../../xcoff/index.md).

<details>
<summary><strong>Methods (30)</strong> - click to expand</summary>

**Required:**
- [`AuxHeader::o_mflag`](#fn-auxheadero-mflag)
- [`AuxHeader::o_vstamp`](#fn-auxheadero-vstamp)
- [`AuxHeader::o_tsize`](#fn-auxheadero-tsize)
- [`AuxHeader::o_dsize`](#fn-auxheadero-dsize)
- [`AuxHeader::o_bsize`](#fn-auxheadero-bsize)
- [`AuxHeader::o_entry`](#fn-auxheadero-entry)
- [`AuxHeader::o_text_start`](#fn-auxheadero-text-start)
- [`AuxHeader::o_data_start`](#fn-auxheadero-data-start)
- [`AuxHeader::o_toc`](#fn-auxheadero-toc)
- [`AuxHeader::o_snentry`](#fn-auxheadero-snentry)
- [`AuxHeader::o_sntext`](#fn-auxheadero-sntext)
- [`AuxHeader::o_sndata`](#fn-auxheadero-sndata)
- [`AuxHeader::o_sntoc`](#fn-auxheadero-sntoc)
- [`AuxHeader::o_snloader`](#fn-auxheadero-snloader)
- [`AuxHeader::o_snbss`](#fn-auxheadero-snbss)
- [`AuxHeader::o_algntext`](#fn-auxheadero-algntext)
- [`AuxHeader::o_algndata`](#fn-auxheadero-algndata)
- [`AuxHeader::o_modtype`](#fn-auxheadero-modtype)
- [`AuxHeader::o_cpuflag`](#fn-auxheadero-cpuflag)
- [`AuxHeader::o_cputype`](#fn-auxheadero-cputype)
- [`AuxHeader::o_maxstack`](#fn-auxheadero-maxstack)
- [`AuxHeader::o_maxdata`](#fn-auxheadero-maxdata)
- [`AuxHeader::o_debugger`](#fn-auxheadero-debugger)
- [`AuxHeader::o_textpsize`](#fn-auxheadero-textpsize)
- [`AuxHeader::o_datapsize`](#fn-auxheadero-datapsize)
- [`AuxHeader::o_stackpsize`](#fn-auxheadero-stackpsize)
- [`AuxHeader::o_flags`](#fn-auxheadero-flags)
- [`AuxHeader::o_sntdata`](#fn-auxheadero-sntdata)
- [`AuxHeader::o_sntbss`](#fn-auxheadero-sntbss)
- [`AuxHeader::o_x64flags`](#fn-auxheadero-x64flags)

</details>

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn AuxHeader::o_mflag(&self) -> u16`

- `fn AuxHeader::o_vstamp(&self) -> u16`

- `fn AuxHeader::o_tsize(&self) -> <Self as >::Word`

- `fn AuxHeader::o_dsize(&self) -> <Self as >::Word`

- `fn AuxHeader::o_bsize(&self) -> <Self as >::Word`

- `fn AuxHeader::o_entry(&self) -> <Self as >::Word`

- `fn AuxHeader::o_text_start(&self) -> <Self as >::Word`

- `fn AuxHeader::o_data_start(&self) -> <Self as >::Word`

- `fn AuxHeader::o_toc(&self) -> <Self as >::Word`

- `fn AuxHeader::o_snentry(&self) -> u16`

- `fn AuxHeader::o_sntext(&self) -> u16`

- `fn AuxHeader::o_sndata(&self) -> u16`

- `fn AuxHeader::o_sntoc(&self) -> u16`

- `fn AuxHeader::o_snloader(&self) -> u16`

- `fn AuxHeader::o_snbss(&self) -> u16`

- `fn AuxHeader::o_algntext(&self) -> u16`

- `fn AuxHeader::o_algndata(&self) -> u16`

- `fn AuxHeader::o_modtype(&self) -> u16`

- `fn AuxHeader::o_cpuflag(&self) -> u8`

- `fn AuxHeader::o_cputype(&self) -> u8`

- `fn AuxHeader::o_maxstack(&self) -> <Self as >::Word`

- `fn AuxHeader::o_maxdata(&self) -> <Self as >::Word`

- `fn AuxHeader::o_debugger(&self) -> u32`

- `fn AuxHeader::o_textpsize(&self) -> u8`

- `fn AuxHeader::o_datapsize(&self) -> u8`

- `fn AuxHeader::o_stackpsize(&self) -> u8`

- `fn AuxHeader::o_flags(&self) -> u8`

- `fn AuxHeader::o_sntdata(&self) -> u16`

- `fn AuxHeader::o_sntbss(&self) -> u16`

- `fn AuxHeader::o_x64flags(&self) -> Option<u16>`

#### Implementors

- [`AuxHeader32`](../../../xcoff/index.md#auxheader32)
- [`AuxHeader64`](../../../xcoff/index.md#auxheader64)

## Type Aliases

### `XcoffFile32<'data, R>`

```rust
type XcoffFile32<'data, R> = XcoffFile<'data, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:24`](../../../../../.source_1765900590/object-0.37.3/src/read/xcoff/file.rs#L24)*

A 32-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader32`](../../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff32`](../../../index.md).

### `XcoffFile64<'data, R>`

```rust
type XcoffFile64<'data, R> = XcoffFile<'data, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/file.rs:29`](../../../../../.source_1765900590/object-0.37.3/src/read/xcoff/file.rs#L29)*

A 64-bit XCOFF object file.

This is a file that starts with [`xcoff::FileHeader64`](../../../xcoff/index.md), and corresponds
to [`crate::FileKind::Xcoff64`](../../../index.md).

