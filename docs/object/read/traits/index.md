*[object](../../index.md) / [read](../index.md) / [traits](index.md)*

---

# Module `traits`

## Structs

### `NoDynamicRelocationIterator`

```rust
struct NoDynamicRelocationIterator;
```

An iterator for files that don't have dynamic relocations.

#### Trait Implementations

##### `impl Debug for NoDynamicRelocationIterator`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for NoDynamicRelocationIterator`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for NoDynamicRelocationIterator`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `Object<'data>`

```rust
trait Object<'data>: read::private::Sealed { ... }
```

An object file.

This is the primary trait for the unified read API.

#### Required Methods

- `type Segment: 1`

- `type SegmentIterator: 1`

- `type Section: 1`

- `type SectionIterator: 1`

- `type Comdat: 1`

- `type ComdatIterator: 1`

- `type Symbol: 1`

- `type SymbolIterator: 1`

- `type SymbolTable: 1`

- `type DynamicRelocationIterator: 1`

- `fn architecture(self: &Self) -> Architecture`

  Get the architecture type of the file.

- `fn sub_architecture(self: &Self) -> Option<SubArchitecture>`

  Get the sub-architecture type of the file if known.

- `fn endianness(self: &Self) -> Endianness`

  Get the endianness of the file.

- `fn is_little_endian(self: &Self) -> bool`

  Return true if the file is little endian, false if it is big endian.

- `fn is_64(self: &Self) -> bool`

  Return true if the file can contain 64-bit addresses.

- `fn kind(self: &Self) -> ObjectKind`

  Return the kind of this object.

- `fn segments(self: &Self) -> <Self as >::SegmentIterator`

  Get an iterator for the loadable segments in the file.

- `fn section_by_name(self: &Self, section_name: &str) -> Option<<Self as >::Section>`

  Get the section named `section_name`, if such a section exists.

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<<Self as >::Section>`

  Like `Self::section_by_name`, but allows names that are not UTF-8.

- `fn section_by_index(self: &Self, index: SectionIndex) -> Result<<Self as >::Section>`

  Get the section at the given index.

- `fn sections(self: &Self) -> <Self as >::SectionIterator`

  Get an iterator for the sections in the file.

- `fn comdats(self: &Self) -> <Self as >::ComdatIterator`

  Get an iterator for the COMDAT section groups in the file.

- `fn symbol_table(self: &Self) -> Option<<Self as >::SymbolTable>`

  Get the debugging symbol table, if any.

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the debugging symbol at the given index.

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator`

  Get an iterator for the debugging symbols in the file.

- `fn symbol_by_name<'file>(self: &'file Self, symbol_name: &str) -> Option<<Self as >::Symbol>`

  Get the symbol named `symbol_name`, if the symbol exists.

- `fn symbol_by_name_bytes<'file>(self: &'file Self, symbol_name: &[u8]) -> Option<<Self as >::Symbol>`

  Like `Self::symbol_by_name`, but allows names that are not UTF-8.

- `fn dynamic_symbol_table(self: &Self) -> Option<<Self as >::SymbolTable>`

  Get the dynamic linking symbol table, if any.

- `fn dynamic_symbols(self: &Self) -> <Self as >::SymbolIterator`

  Get an iterator for the dynamic linking symbols in the file.

- `fn dynamic_relocations(self: &Self) -> Option<<Self as >::DynamicRelocationIterator>`

  Get the dynamic relocations for this file.

- `fn symbol_map(self: &Self) -> SymbolMap<SymbolMapName<'data>>`

  Construct a map from addresses to symbol names.

- `fn object_map(self: &Self) -> ObjectMap<'data>`

  Construct a map from addresses to symbol names and object file names.

- `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`

  Get the imported symbols.

- `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`

  Get the exported symbols that expose both a name and an address.

- `fn has_debug_symbols(self: &Self) -> bool`

  Return true if the file contains DWARF debug information sections, false if not.

- `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>`

  The UUID from a Mach-O [`LC_UUID`](crate::macho::LC_UUID) load command.

- `fn build_id(self: &Self) -> Result<Option<&'data [u8]>>`

  The build ID from an ELF [`NT_GNU_BUILD_ID`](crate::elf::NT_GNU_BUILD_ID) note.

- `fn gnu_debuglink(self: &Self) -> Result<Option<(&'data [u8], u32)>>`

  The filename and CRC from a `.gnu_debuglink` section.

- `fn gnu_debugaltlink(self: &Self) -> Result<Option<(&'data [u8], &'data [u8])>>`

  The filename and build ID from a `.gnu_debugaltlink` section.

- `fn pdb_info(self: &Self) -> Result<Option<CodeView<'_>>>`

  The filename and GUID from the PE CodeView section.

- `fn relative_address_base(self: &Self) -> u64`

  Get the base address used for relative virtual addresses.

- `fn entry(self: &Self) -> u64`

  Get the virtual address of the entry point of the binary.

- `fn flags(self: &Self) -> FileFlags`

  File flags that are specific to each file format.

### `ObjectSegment<'data>`

```rust
trait ObjectSegment<'data>: read::private::Sealed { ... }
```

A loadable segment in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `fn address(self: &Self) -> u64`

  Returns the virtual address of the segment.

- `fn size(self: &Self) -> u64`

  Returns the size of the segment in memory.

- `fn align(self: &Self) -> u64`

  Returns the alignment of the segment in memory.

- `fn file_range(self: &Self) -> (u64, u64)`

  Returns the offset and size of the segment in the file.

- `fn data(self: &Self) -> Result<&'data [u8]>`

  Returns a reference to the file contents of the segment.

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the segment data in the given range.

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>`

  Returns the name of the segment.

- `fn name(self: &Self) -> Result<Option<&str>>`

  Returns the name of the segment.

- `fn flags(self: &Self) -> SegmentFlags`

  Return the flags of segment.

### `ObjectSection<'data>`

```rust
trait ObjectSection<'data>: read::private::Sealed { ... }
```

A section in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `type RelocationIterator: 1`

- `fn index(self: &Self) -> SectionIndex`

  Returns the section index.

- `fn address(self: &Self) -> u64`

  Returns the address of the section.

- `fn size(self: &Self) -> u64`

  Returns the size of the section in memory.

- `fn align(self: &Self) -> u64`

  Returns the alignment of the section in memory.

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

  Returns offset and size of on-disk segment (if any).

- `fn data(self: &Self) -> Result<&'data [u8]>`

  Returns the raw contents of the section.

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the raw contents of the section data in the given range.

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>`

  Returns the potentially compressed file range of the section,

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>`

  Returns the potentially compressed contents of the section,

- `fn uncompressed_data(self: &Self) -> Result<Cow<'data, [u8]>>`

  Returns the uncompressed contents of the section.

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>`

  Returns the name of the section.

- `fn name(self: &Self) -> Result<&'data str>`

  Returns the name of the section.

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`

  Returns the name of the segment for this section.

- `fn segment_name(self: &Self) -> Result<Option<&str>>`

  Returns the name of the segment for this section.

- `fn kind(self: &Self) -> SectionKind`

  Return the kind of this section.

- `fn relocations(self: &Self) -> <Self as >::RelocationIterator`

  Get the relocations for this section.

- `fn relocation_map(self: &Self) -> Result<RelocationMap>`

  Construct a relocation map for this section.

- `fn flags(self: &Self) -> SectionFlags`

  Section flags that are specific to each file format.

### `ObjectComdat<'data>`

```rust
trait ObjectComdat<'data>: read::private::Sealed { ... }
```

A COMDAT section group in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `type SectionIterator: 1`

- `fn kind(self: &Self) -> ComdatKind`

  Returns the COMDAT selection kind.

- `fn symbol(self: &Self) -> SymbolIndex`

  Returns the index of the symbol used for the name of COMDAT section group.

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>`

  Returns the name of the COMDAT section group.

- `fn name(self: &Self) -> Result<&'data str>`

  Returns the name of the COMDAT section group.

- `fn sections(self: &Self) -> <Self as >::SectionIterator`

  Get the sections in this section group.

### `ObjectSymbolTable<'data>`

```rust
trait ObjectSymbolTable<'data>: read::private::Sealed { ... }
```

A symbol table in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `type Symbol: 1`

- `type SymbolIterator: 1`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator`

  Get an iterator for the symbols in the table.

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the symbol at the given index.

### `ObjectSymbol<'data>`

```rust
trait ObjectSymbol<'data>: read::private::Sealed { ... }
```

A symbol table entry in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `fn index(self: &Self) -> SymbolIndex`

  The index of the symbol.

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>`

  The name of the symbol.

- `fn name(self: &Self) -> Result<&'data str>`

  The name of the symbol.

- `fn address(self: &Self) -> u64`

  The address of the symbol. May be zero if the address is unknown.

- `fn size(self: &Self) -> u64`

  The size of the symbol. May be zero if the size is unknown.

- `fn kind(self: &Self) -> SymbolKind`

  Return the kind of this symbol.

- `fn section(self: &Self) -> SymbolSection`

  Returns the section where the symbol is defined.

- `fn section_index(self: &Self) -> Option<SectionIndex>`

  Returns the section index for the section containing this symbol.

- `fn is_undefined(self: &Self) -> bool`

  Return true if the symbol is undefined.

- `fn is_definition(self: &Self) -> bool`

  Return true if the symbol is a definition of a function or data object

- `fn is_common(self: &Self) -> bool`

  Return true if the symbol is common data.

- `fn is_weak(self: &Self) -> bool`

  Return true if the symbol is weak.

- `fn scope(self: &Self) -> SymbolScope`

  Returns the symbol scope.

- `fn is_global(self: &Self) -> bool`

  Return true if the symbol visible outside of the compilation unit.

- `fn is_local(self: &Self) -> bool`

  Return true if the symbol is only visible within the compilation unit.

- `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>`

  Symbol flags that are specific to each file format.

