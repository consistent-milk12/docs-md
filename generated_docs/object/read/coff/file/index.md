*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [file](index.md)*

---

# Module `file`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CoffCommon`](#coffcommon) | struct | The common parts of `PeFile` and `CoffFile`. |
| [`CoffFile`](#cofffile) | struct | A COFF object file. |
| [`CoffHeader`](#coffheader) | trait | A trait for generic access to [`pe::ImageFileHeader`] and [`pe::AnonObjectHeaderBigobj`]. |
| [`anon_object_class_id`](#anon-object-class-id) | fn | Read the `class_id` field from a [`pe::AnonObjectHeader`]. |
| [`CoffBigFile`](#coffbigfile) | type | A COFF bigobj object file with 32-bit section numbers. |

## Structs

### `CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    sections: super::SectionTable<'data>,
    symbols: super::SymbolTable<'data, R, Coff>,
    image_base: u64,
}
```

*Defined in [`object-0.37.3/src/read/coff/file.rs:20-24`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/file.rs#L20-L24)*

The common parts of `PeFile` and `CoffFile`.

#### Trait Implementations

##### `impl Any for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffcommon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffCommon<'data, R, Coff>`

- <span id="coffcommon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffcommon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    header: &'data Coff,
    common: CoffCommon<'data, R, Coff>,
    data: R,
}
```

*Defined in [`object-0.37.3/src/read/coff/file.rs:41-46`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/file.rs#L41-L46)*

A COFF object file.

This is a file that starts with [`pe::ImageFileHeader`](../../../pe/index.md), and corresponds
to [`crate::FileKind::Coff`](../../../index.md).

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="cofffile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the raw COFF file data.

- <span id="cofffile-coff-header"></span>`fn coff_header(&self) -> &'data Coff`

  Get the raw COFF file header.

- <span id="cofffile-coff-section-table"></span>`fn coff_section_table(&self) -> SectionTable<'data>` — [`SectionTable`](../index.md#sectiontable)

  Get the COFF section table.

- <span id="cofffile-coff-symbol-table"></span>`fn coff_symbol_table(&self) -> &SymbolTable<'data, R, Coff>` — [`SymbolTable`](../index.md#symboltable)

  Get the COFF symbol table.

#### Trait Implementations

##### `impl Any for CoffFile<'data, R, Coff>`

- <span id="cofffile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffFile<'data, R, Coff>`

- <span id="cofffile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffFile<'data, R, Coff>`

- <span id="cofffile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffFile<'data, R, Coff>`

- <span id="cofffile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffFile<'data, R, Coff>`

- <span id="cofffile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffFile<'data, R, Coff>`

- <span id="cofffile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Coff> Object for CoffFile<'data, R, Coff>`

- <span id="cofffile-object-type-segment"></span>`type Segment = CoffSegment<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-segmentiterator"></span>`type SegmentIterator = CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-section"></span>`type Section = CoffSection<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-sectioniterator"></span>`type SectionIterator = CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-comdat"></span>`type Comdat = CoffComdat<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-comdatiterator"></span>`type ComdatIterator = CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-symboltable"></span>`type SymbolTable = CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="cofffile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="cofffile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

- <span id="cofffile-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md#subarchitecture)

- <span id="cofffile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="cofffile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="cofffile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md#objectkind)

- <span id="cofffile-object-segments"></span>`fn segments(&self) -> CoffSegmentIterator<'data, '_, R, Coff>` — [`CoffSegmentIterator`](../index.md#coffsegmentiterator)

- <span id="cofffile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<CoffSection<'data, 'file, R, Coff>>` — [`CoffSection`](../index.md#coffsection)

- <span id="cofffile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<CoffSection<'data, '_, R, Coff>>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`CoffSection`](../index.md#coffsection)

- <span id="cofffile-object-sections"></span>`fn sections(&self) -> CoffSectionIterator<'data, '_, R, Coff>` — [`CoffSectionIterator`](../index.md#coffsectioniterator)

- <span id="cofffile-object-comdats"></span>`fn comdats(&self) -> CoffComdatIterator<'data, '_, R, Coff>` — [`CoffComdatIterator`](../index.md#coffcomdatiterator)

- <span id="cofffile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R, Coff>>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`CoffSymbol`](../index.md#coffsymbol)

- <span id="cofffile-object-symbols"></span>`fn symbols(&self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](../index.md#coffsymboliterator)

- <span id="cofffile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](../index.md#coffsymboltable)

- <span id="cofffile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](../index.md#coffsymboliterator)

- <span id="cofffile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](../index.md#coffsymboltable)

- <span id="cofffile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md#nodynamicrelocationiterator)

- <span id="cofffile-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md#result), [`Import`](../../../index.md#import)

- <span id="cofffile-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../../../index.md#export)

- <span id="cofffile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="cofffile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="cofffile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="cofffile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md#fileflags)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffFile<'data, R, Coff>`

##### `impl<U> TryFrom for CoffFile<'data, R, Coff>`

- <span id="cofffile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cofffile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffFile<'data, R, Coff>`

- <span id="cofffile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cofffile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `CoffHeader`

```rust
trait CoffHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/coff/file.rs:293-338`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/file.rs#L293-L338)*

A trait for generic access to [`pe::ImageFileHeader`](../../../pe/index.md) and [`pe::AnonObjectHeaderBigobj`](../../../pe/index.md).

#### Associated Types

- `type ImageSymbol: 1`

- `type ImageSymbolBytes: 2`

#### Required Methods

- `fn is_type_bigobj() -> bool`

  Return true if this type is [`pe::AnonObjectHeaderBigobj`](../../../pe/index.md).

- `fn machine(&self) -> u16`

- `fn number_of_sections(&self) -> u32`

- `fn pointer_to_symbol_table(&self) -> u32`

- `fn number_of_symbols(&self) -> u32`

- `fn characteristics(&self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>`

  Read the file header.

#### Provided Methods

- `fn sections<'data, R: ReadRef<'data>>(&self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<SymbolTable<'data, R, Self>>`

  Read the symbol table and string table.

#### Implementors

- [`AnonObjectHeaderBigobj`](../../../pe/index.md#anonobjectheaderbigobj)
- [`ImageFileHeader`](../../../pe/index.md#imagefileheader)

## Functions

### `anon_object_class_id`

```rust
fn anon_object_class_id<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<pe::ClsId>
```

*Defined in [`object-0.37.3/src/read/coff/file.rs:284-289`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/file.rs#L284-L289)*

Read the `class_id` field from a [`pe::AnonObjectHeader`](../../../pe/index.md).

This can be used to determine the format of the header.

## Type Aliases

### `CoffBigFile<'data, R>`

```rust
type CoffBigFile<'data, R> = CoffFile<'data, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/file.rs:32`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/file.rs#L32)*

A COFF bigobj object file with 32-bit section numbers.

This is a file that starts with [`pe::AnonObjectHeaderBigobj`](../../../pe/index.md), and corresponds
to [`crate::FileKind::CoffBig`](../../../index.md).

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

