*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [file](index.md)*

---

# Module `file`

## Structs

### `CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffCommon<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    sections: super::SectionTable<'data>,
    symbols: super::SymbolTable<'data, R, Coff>,
    image_base: u64,
}
```

The common parts of `PeFile` and `CoffFile`.

#### Trait Implementations

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffCommon<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffFile<'data, R: ReadRef<'data>, Coff: CoffHeader> {
    header: &'data Coff,
    common: CoffCommon<'data, R, Coff>,
    data: R,
}
```

A COFF object file.

This is a file that starts with [`pe::ImageFileHeader`](../../../pe/index.md), and corresponds
to [`crate::FileKind::Coff`](../../../index.md).

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- `fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn coff_header(self: &Self) -> &'data Coff`

- `fn coff_section_table(self: &Self) -> SectionTable<'data>` — [`SectionTable`](../../pe/index.md)

- `fn coff_symbol_table(self: &Self) -> &SymbolTable<'data, R, Coff>` — [`SymbolTable`](../../pe/index.md)

#### Trait Implementations

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>, Coff: $crate::fmt::Debug + CoffHeader> Debug for CoffFile<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R, Coff> Object for CoffFile<'data, R, Coff>`

- `type Segment = CoffSegment<'data, 'file, R, Coff>`

- `type SegmentIterator = CoffSegmentIterator<'data, 'file, R, Coff>`

- `type Section = CoffSection<'data, 'file, R, Coff>`

- `type SectionIterator = CoffSectionIterator<'data, 'file, R, Coff>`

- `type Comdat = CoffComdat<'data, 'file, R, Coff>`

- `type ComdatIterator = CoffComdatIterator<'data, 'file, R, Coff>`

- `type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- `type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- `type SymbolTable = CoffSymbolTable<'data, 'file, R, Coff>`

- `type DynamicRelocationIterator = NoDynamicRelocationIterator`

- `fn architecture(self: &Self) -> Architecture` — [`Architecture`](../../../index.md)

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md)

- `fn is_little_endian(self: &Self) -> bool`

- `fn is_64(self: &Self) -> bool`

- `fn kind(self: &Self) -> ObjectKind` — [`ObjectKind`](../../../index.md)

- `fn segments(self: &Self) -> CoffSegmentIterator<'data, '_, R, Coff>` — [`CoffSegmentIterator`](../index.md)

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<CoffSection<'data, 'file, R, Coff>>` — [`CoffSection`](../index.md)

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<CoffSection<'data, '_, R, Coff>>` — [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`CoffSection`](../index.md)

- `fn sections(self: &Self) -> CoffSectionIterator<'data, '_, R, Coff>` — [`CoffSectionIterator`](../index.md)

- `fn comdats(self: &Self) -> CoffComdatIterator<'data, '_, R, Coff>` — [`CoffComdatIterator`](../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<CoffSymbol<'data, '_, R, Coff>>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`CoffSymbol`](../index.md)

- `fn symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](../index.md)

- `fn symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](../index.md)

- `fn dynamic_symbols(self: &Self) -> CoffSymbolIterator<'data, '_, R, Coff>` — [`CoffSymbolIterator`](../index.md)

- `fn dynamic_symbol_table(self: &Self) -> Option<CoffSymbolTable<'data, '_, R, Coff>>` — [`CoffSymbolTable`](../index.md)

- `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md)

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md), [`Import`](../../../index.md)

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md), [`Export`](../../../index.md)

- `fn has_debug_symbols(self: &Self) -> bool`

- `fn relative_address_base(self: &Self) -> u64`

- `fn entry(self: &Self) -> u64`

- `fn flags(self: &Self) -> FileFlags` — [`FileFlags`](../../../index.md)

##### `impl<'data, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffFile<'data, R, Coff>`

## Traits

### `CoffHeader`

```rust
trait CoffHeader: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageFileHeader`](../../../pe/index.md) and [`pe::AnonObjectHeaderBigobj`](../../../pe/index.md).

#### Required Methods

- `type ImageSymbol: 1`

- `type ImageSymbolBytes: 2`

- `fn is_type_bigobj() -> bool`

  Return true if this type is [`pe::AnonObjectHeaderBigobj`](../../../pe/index.md).

- `fn machine(self: &Self) -> u16`

- `fn number_of_sections(self: &Self) -> u32`

- `fn pointer_to_symbol_table(self: &Self) -> u32`

- `fn number_of_symbols(self: &Self) -> u32`

- `fn characteristics(self: &Self) -> u16`

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: &mut u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn sections<'data, R: ReadRef<'data>>(self: &Self, data: R, offset: u64) -> read::Result<SectionTable<'data>>`

  Read the section table.

- `fn symbols<'data, R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<SymbolTable<'data, R, Self>>`

  Read the symbol table and string table.

## Functions

### `anon_object_class_id`

```rust
fn anon_object_class_id<'data, R: ReadRef<'data>>(data: R) -> crate::read::Result<pe::ClsId>
```

Read the `class_id` field from a [`pe::AnonObjectHeader`](../../../pe/index.md).

This can be used to determine the format of the header.

## Type Aliases

### `CoffBigFile<'data, R>`

```rust
type CoffBigFile<'data, R> = CoffFile<'data, R, pe::AnonObjectHeaderBigobj>;
```

A COFF bigobj object file with 32-bit section numbers.

This is a file that starts with [`pe::AnonObjectHeaderBigobj`](../../../pe/index.md), and corresponds
to [`crate::FileKind::CoffBig`](../../../index.md).

Most functionality is provided by the [`Object`](../../index.md) trait implementation.

