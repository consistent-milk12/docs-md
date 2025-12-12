*[object](../../index.md) / [read](../index.md) / [traits](index.md)*

---

# Module `traits`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoDynamicRelocationIterator`](#nodynamicrelocationiterator) | struct | An iterator for files that don't have dynamic relocations. |
| [`Object`](#object) | trait | An object file. |
| [`ObjectSegment`](#objectsegment) | trait | A loadable segment in an [`Object`]. |
| [`ObjectSection`](#objectsection) | trait | A section in an [`Object`]. |
| [`ObjectComdat`](#objectcomdat) | trait | A COMDAT section group in an [`Object`]. |
| [`ObjectSymbolTable`](#objectsymboltable) | trait | A symbol table in an [`Object`]. |
| [`ObjectSymbol`](#objectsymbol) | trait | A symbol table entry in an [`Object`]. |

## Structs

### `NoDynamicRelocationIterator`

```rust
struct NoDynamicRelocationIterator;
```

*Defined in [`object-0.37.3/src/read/traits.rs:580`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L580)*

An iterator for files that don't have dynamic relocations.

#### Trait Implementations

##### `impl Debug for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nodynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nodynamicrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="nodynamicrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `Object<'data>`

```rust
trait Object<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:15-335`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L15-L335)*

An object file.

This is the primary trait for the unified read API.

#### Associated Types

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

#### Required Methods

- `fn architecture(&self) -> Architecture`

  Get the architecture type of the file.

- `fn is_little_endian(&self) -> bool`

  Return true if the file is little endian, false if it is big endian.

- `fn is_64(&self) -> bool`

  Return true if the file can contain 64-bit addresses.

- `fn kind(&self) -> ObjectKind`

  Return the kind of this object.

- `fn segments(&self) -> <Self as >::SegmentIterator`

  Get an iterator for the loadable segments in the file.

- `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<<Self as >::Section>`

  Like `Self::section_by_name`, but allows names that are not UTF-8.

- `fn section_by_index(&self, index: SectionIndex) -> Result<<Self as >::Section>`

  Get the section at the given index.

- `fn sections(&self) -> <Self as >::SectionIterator`

  Get an iterator for the sections in the file.

- `fn comdats(&self) -> <Self as >::ComdatIterator`

  Get an iterator for the COMDAT section groups in the file.

- `fn symbol_table(&self) -> Option<<Self as >::SymbolTable>`

  Get the debugging symbol table, if any.

- `fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the debugging symbol at the given index.

- `fn symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the debugging symbols in the file.

- `fn dynamic_symbol_table(&self) -> Option<<Self as >::SymbolTable>`

  Get the dynamic linking symbol table, if any.

- `fn dynamic_symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the dynamic linking symbols in the file.

- `fn dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>`

  Get the dynamic relocations for this file.

- `fn imports(&self) -> Result<Vec<Import<'data>>>`

  Get the imported symbols.

- `fn exports(&self) -> Result<Vec<Export<'data>>>`

  Get the exported symbols that expose both a name and an address.

- `fn has_debug_symbols(&self) -> bool`

  Return true if the file contains DWARF debug information sections, false if not.

- `fn relative_address_base(&self) -> u64`

  Get the base address used for relative virtual addresses.

- `fn entry(&self) -> u64`

  Get the virtual address of the entry point of the binary.

- `fn flags(&self) -> FileFlags`

  File flags that are specific to each file format.

#### Provided Methods

- `fn sub_architecture(&self) -> Option<SubArchitecture>`

  Get the sub-architecture type of the file if known.

- `fn endianness(&self) -> Endianness`

  Get the endianness of the file.

- `fn section_by_name(&self, section_name: &str) -> Option<<Self as >::Section>`

  Get the section named `section_name`, if such a section exists.

- `fn symbol_by_name<'file>(self: &'file Self, symbol_name: &str) -> Option<<Self as >::Symbol>`

  Get the symbol named `symbol_name`, if the symbol exists.

- `fn symbol_by_name_bytes<'file>(self: &'file Self, symbol_name: &[u8]) -> Option<<Self as >::Symbol>`

  Like `Self::symbol_by_name`, but allows names that are not UTF-8.

- `fn symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>`

  Construct a map from addresses to symbol names.

- `fn object_map(&self) -> ObjectMap<'data>`

  Construct a map from addresses to symbol names and object file names.

- `fn mach_uuid(&self) -> Result<Option<[u8; 16]>>`

  The UUID from a Mach-O [`LC_UUID`](crate::macho::LC_UUID) load command.

- `fn build_id(&self) -> Result<Option<&'data [u8]>>`

  The build ID from an ELF [`NT_GNU_BUILD_ID`](crate::elf::NT_GNU_BUILD_ID) note.

- `fn gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>`

  The filename and CRC from a `.gnu_debuglink` section.

- `fn gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>`

  The filename and build ID from a `.gnu_debugaltlink` section.

- `fn pdb_info(&self) -> Result<Option<CodeView<'_>>>`

  The filename and GUID from the PE CodeView section.

#### Implementors

- [`CoffFile`](../coff/index.md#cofffile)
- [`ElfFile`](../elf/index.md#elffile)
- [`File`](../index.md#file)
- [`MachOFile`](../macho/index.md#machofile)
- [`PeFile`](../pe/index.md#pefile)
- [`XcoffFile`](../xcoff/index.md#xcofffile)

### `ObjectSegment<'data>`

```rust
trait ObjectSegment<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:340-374`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L340-L374)*

A loadable segment in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `fn address(&self) -> u64`

  Returns the virtual address of the segment.

- `fn size(&self) -> u64`

  Returns the size of the segment in memory.

- `fn align(&self) -> u64`

  Returns the alignment of the segment in memory.

- `fn file_range(&self) -> (u64, u64)`

  Returns the offset and size of the segment in the file.

- `fn data(&self) -> Result<&'data [u8]>`

  Returns a reference to the file contents of the segment.

- `fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the segment data in the given range.

- `fn name_bytes(&self) -> Result<Option<&[u8]>>`

  Returns the name of the segment.

- `fn name(&self) -> Result<Option<&str>>`

  Returns the name of the segment.

- `fn flags(&self) -> SegmentFlags`

  Return the flags of segment.

#### Implementors

- [`CoffSegment`](../coff/index.md#coffsegment)
- [`ElfSegment`](../elf/index.md#elfsegment)
- [`MachOSegment`](../macho/index.md#machosegment)
- [`PeSegment`](../pe/index.md#pesegment)
- [`Segment`](../index.md#segment)
- [`XcoffSegment`](../xcoff/index.md#xcoffsegment)

### `ObjectSection<'data>`

```rust
trait ObjectSection<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:379-462`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L379-L462)*

A section in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Associated Types

- `type RelocationIterator: 1`

#### Required Methods

- `fn index(&self) -> SectionIndex`

  Returns the section index.

- `fn address(&self) -> u64`

  Returns the address of the section.

- `fn size(&self) -> u64`

  Returns the size of the section in memory.

- `fn align(&self) -> u64`

  Returns the alignment of the section in memory.

- `fn file_range(&self) -> Option<(u64, u64)>`

  Returns offset and size of on-disk segment (if any).

- `fn data(&self) -> Result<&'data [u8]>`

  Returns the raw contents of the section.

- `fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the raw contents of the section data in the given range.

- `fn compressed_file_range(&self) -> Result<CompressedFileRange>`

  Returns the potentially compressed file range of the section,

- `fn compressed_data(&self) -> Result<CompressedData<'data>>`

  Returns the potentially compressed contents of the section,

- `fn name_bytes(&self) -> Result<&'data [u8]>`

  Returns the name of the section.

- `fn name(&self) -> Result<&'data str>`

  Returns the name of the section.

- `fn segment_name_bytes(&self) -> Result<Option<&[u8]>>`

  Returns the name of the segment for this section.

- `fn segment_name(&self) -> Result<Option<&str>>`

  Returns the name of the segment for this section.

- `fn kind(&self) -> SectionKind`

  Return the kind of this section.

- `fn relocations(&self) -> <Self as >::RelocationIterator`

  Get the relocations for this section.

- `fn relocation_map(&self) -> Result<RelocationMap>`

  Construct a relocation map for this section.

- `fn flags(&self) -> SectionFlags`

  Section flags that are specific to each file format.

#### Provided Methods

- `fn uncompressed_data(&self) -> Result<Cow<'data, [u8]>>`

  Returns the uncompressed contents of the section.

#### Implementors

- [`CoffSection`](../coff/index.md#coffsection)
- [`ElfSection`](../elf/index.md#elfsection)
- [`MachOSection`](../macho/index.md#machosection)
- [`PeSection`](../pe/index.md#pesection)
- [`Section`](../index.md#section)
- [`XcoffSection`](../xcoff/index.md#xcoffsection)

### `ObjectComdat<'data>`

```rust
trait ObjectComdat<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:467-487`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L467-L487)*

A COMDAT section group in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Associated Types

- `type SectionIterator: 1`

#### Required Methods

- `fn kind(&self) -> ComdatKind`

  Returns the COMDAT selection kind.

- `fn symbol(&self) -> SymbolIndex`

  Returns the index of the symbol used for the name of COMDAT section group.

- `fn name_bytes(&self) -> Result<&'data [u8]>`

  Returns the name of the COMDAT section group.

- `fn name(&self) -> Result<&'data str>`

  Returns the name of the COMDAT section group.

- `fn sections(&self) -> <Self as >::SectionIterator`

  Get the sections in this section group.

#### Implementors

- [`CoffComdat`](../coff/index.md#coffcomdat)
- [`Comdat`](../index.md#comdat)
- [`ElfComdat`](../elf/index.md#elfcomdat)
- [`MachOComdat`](../macho/index.md#machocomdat)
- [`PeComdat`](../pe/index.md#pecomdat)
- [`XcoffComdat`](../xcoff/index.md#xcoffcomdat)

### `ObjectSymbolTable<'data>`

```rust
trait ObjectSymbolTable<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:492-510`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L492-L510)*

A symbol table in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Associated Types

- `type Symbol: 1`

- `type SymbolIterator: 1`

#### Required Methods

- `fn symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the symbols in the table.

- `fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the symbol at the given index.

#### Implementors

- [`CoffSymbolTable`](../coff/index.md#coffsymboltable)
- [`ElfSymbolTable`](../elf/index.md#elfsymboltable)
- [`MachOSymbolTable`](../macho/index.md#machosymboltable)
- [`SymbolTable`](../index.md#symboltable)
- [`XcoffSymbolTable`](../xcoff/index.md#xcoffsymboltable)

### `ObjectSymbol<'data>`

```rust
trait ObjectSymbol<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:515-576`](../../../../.source_1765210505/object-0.37.3/src/read/traits.rs#L515-L576)*

A symbol table entry in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `fn index(&self) -> SymbolIndex`

  The index of the symbol.

- `fn name_bytes(&self) -> Result<&'data [u8]>`

  The name of the symbol.

- `fn name(&self) -> Result<&'data str>`

  The name of the symbol.

- `fn address(&self) -> u64`

  The address of the symbol. May be zero if the address is unknown.

- `fn size(&self) -> u64`

  The size of the symbol. May be zero if the size is unknown.

- `fn kind(&self) -> SymbolKind`

  Return the kind of this symbol.

- `fn section(&self) -> SymbolSection`

  Returns the section where the symbol is defined.

- `fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object

- `fn is_common(&self) -> bool`

  Return true if the symbol is common data.

- `fn is_weak(&self) -> bool`

  Return true if the symbol is weak.

- `fn scope(&self) -> SymbolScope`

  Returns the symbol scope.

- `fn is_global(&self) -> bool`

  Return true if the symbol visible outside of the compilation unit.

- `fn is_local(&self) -> bool`

  Return true if the symbol is only visible within the compilation unit.

- `fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>`

  Symbol flags that are specific to each file format.

#### Provided Methods

- `fn section_index(&self) -> Option<SectionIndex>`

  Returns the section index for the section containing this symbol.

#### Implementors

- [`CoffSymbol`](../coff/index.md#coffsymbol)
- [`ElfSymbol`](../elf/index.md#elfsymbol)
- [`MachOSymbol`](../macho/index.md#machosymbol)
- [`Symbol`](../index.md#symbol)
- [`XcoffSymbol`](../xcoff/index.md#xcoffsymbol)

