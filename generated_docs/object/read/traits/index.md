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

*Defined in [`object-0.37.3/src/read/traits.rs:580`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L580)*

An iterator for files that don't have dynamic relocations.

#### Trait Implementations

##### `impl Any for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nodynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nodynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="nodynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nodynamicrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoDynamicRelocationIterator`

- <span id="nodynamicrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nodynamicrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Object<'data>`

```rust
trait Object<'data>: read::private::Sealed { ... }
```

*Defined in [`object-0.37.3/src/read/traits.rs:15-335`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L15-L335)*

An object file.

This is the primary trait for the unified read API.

<details>
<summary><strong>Methods (33)</strong> - click to expand</summary>

**Required:**
- [`Object::architecture`](#fn-objectarchitecture)
- [`Object::is_little_endian`](#fn-objectis-little-endian)
- [`Object::is_64`](#fn-objectis-64)
- [`Object::kind`](#fn-objectkind)
- [`Object::segments`](#fn-objectsegments)
- [`Object::section_by_name_bytes`](#fn-objectsection-by-name-bytes)
- [`Object::section_by_index`](#fn-objectsection-by-index)
- [`Object::sections`](#fn-objectsections)
- [`Object::comdats`](#fn-objectcomdats)
- [`Object::symbol_table`](#fn-objectsymbol-table)
- [`Object::symbol_by_index`](#fn-objectsymbol-by-index)
- [`Object::symbols`](#fn-objectsymbols)
- [`Object::dynamic_symbol_table`](#fn-objectdynamic-symbol-table)
- [`Object::dynamic_symbols`](#fn-objectdynamic-symbols)
- [`Object::dynamic_relocations`](#fn-objectdynamic-relocations)
- [`Object::imports`](#fn-objectimports)
- [`Object::exports`](#fn-objectexports)
- [`Object::has_debug_symbols`](#fn-objecthas-debug-symbols)
- [`Object::relative_address_base`](#fn-objectrelative-address-base)
- [`Object::entry`](#fn-objectentry)
- [`Object::flags`](#fn-objectflags)

**Provided:**
- [`Object::sub_architecture`](#fn-objectsub-architecture)
- [`Object::endianness`](#fn-objectendianness)
- [`Object::section_by_name`](#fn-objectsection-by-name)
- [`Object::symbol_by_name`](#fn-objectsymbol-by-name)
- [`Object::symbol_by_name_bytes`](#fn-objectsymbol-by-name-bytes)
- [`Object::symbol_map`](#fn-objectsymbol-map)
- [`Object::object_map`](#fn-objectobject-map)
- [`Object::mach_uuid`](#fn-objectmach-uuid)
- [`Object::build_id`](#fn-objectbuild-id)
- [`Object::gnu_debuglink`](#fn-objectgnu-debuglink)
- [`Object::gnu_debugaltlink`](#fn-objectgnu-debugaltlink)
- [`Object::pdb_info`](#fn-objectpdb-info)

</details>

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

- `fn Object::architecture(&self) -> Architecture`

  Get the architecture type of the file.

- `fn Object::is_little_endian(&self) -> bool`

  Return true if the file is little endian, false if it is big endian.

- `fn Object::is_64(&self) -> bool`

  Return true if the file can contain 64-bit addresses.

- `fn Object::kind(&self) -> ObjectKind`

  Return the kind of this object.

- `fn Object::segments(&self) -> <Self as >::SegmentIterator`

  Get an iterator for the loadable segments in the file.
  
  For ELF, this is program headers with type [`PT_LOAD`](crate::elf::PT_LOAD).
  For Mach-O, this is load commands with type [`LC_SEGMENT`](crate::macho::LC_SEGMENT)
  or [`LC_SEGMENT_64`](crate::macho::LC_SEGMENT_64).
  For PE, this is all sections.

- `fn Object::section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<<Self as >::Section>`

  Like `Self::section_by_name`, but allows names that are not UTF-8.

- `fn Object::section_by_index(&self, index: SectionIndex) -> Result<<Self as >::Section>`

  Get the section at the given index.
  
  The meaning of the index depends on the object file.
  
  For some object files, this requires iterating through all sections.
  
  Returns an error if the index is invalid.

- `fn Object::sections(&self) -> <Self as >::SectionIterator`

  Get an iterator for the sections in the file.

- `fn Object::comdats(&self) -> <Self as >::ComdatIterator`

  Get an iterator for the COMDAT section groups in the file.

- `fn Object::symbol_table(&self) -> Option<<Self as >::SymbolTable>`

  Get the debugging symbol table, if any.

- `fn Object::symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the debugging symbol at the given index.
  
  The meaning of the index depends on the object file.
  
  Returns an error if the index is invalid.

- `fn Object::symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the debugging symbols in the file.
  
  This may skip over symbols that are malformed or unsupported.
  
  For Mach-O files, this does not include STAB entries.

- `fn Object::dynamic_symbol_table(&self) -> Option<<Self as >::SymbolTable>`

  Get the dynamic linking symbol table, if any.
  
  Only ELF has a separate dynamic linking symbol table.
  Consider using `Self::exports` or `Self::imports` instead.

- `fn Object::dynamic_symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the dynamic linking symbols in the file.
  
  This may skip over symbols that are malformed or unsupported.
  
  Only ELF has dynamic linking symbols.
  Other file formats will return an empty iterator.
  Consider using `Self::exports` or `Self::imports` instead.

- `fn Object::dynamic_relocations(&self) -> Option<<Self as >::DynamicRelocationIterator>`

  Get the dynamic relocations for this file.
  
  Symbol indices in these relocations refer to the dynamic symbol table.
  
  Only ELF has dynamic relocations.

- `fn Object::imports(&self) -> Result<Vec<Import<'data>>>`

  Get the imported symbols.

- `fn Object::exports(&self) -> Result<Vec<Export<'data>>>`

  Get the exported symbols that expose both a name and an address.
  
  Some file formats may provide other kinds of symbols that can be retrieved using
  the low level API.

- `fn Object::has_debug_symbols(&self) -> bool`

  Return true if the file contains DWARF debug information sections, false if not.

- `fn Object::relative_address_base(&self) -> u64`

  Get the base address used for relative virtual addresses.
  
  Currently this is only non-zero for PE.

- `fn Object::entry(&self) -> u64`

  Get the virtual address of the entry point of the binary.

- `fn Object::flags(&self) -> FileFlags`

  File flags that are specific to each file format.

#### Provided Methods

- `fn Object::sub_architecture(&self) -> Option<SubArchitecture>`

  Get the sub-architecture type of the file if known.
  
  A value of `None` has a range of meanings: the file supports all
  sub-architectures, the file does not explicitly specify a
  sub-architecture, or the sub-architecture is currently unrecognized.

- `fn Object::endianness(&self) -> Endianness`

  Get the endianness of the file.

- `fn Object::section_by_name(&self, section_name: &str) -> Option<<Self as >::Section>`

  Get the section named `section_name`, if such a section exists.
  
  If `section_name` starts with a '.' then it is treated as a system
  section name, and is compared using the conventions specific to the
  object file format. This includes:
  - if ".debug_str_offsets" is requested for a Mach-O object file, then
    the actual section name that is searched for is "__debug_str_offs".
  - if ".debug_info" is requested for an ELF object file, then
    ".zdebug_info" may be returned (and similarly for other debug
    sections). Similarly, if ".debug_info" is requested for a Mach-O
    object file, then "__zdebug_info" may be returned.
  
  For some object files, multiple segments may contain sections with the
  same name. In this case, the first matching section will be used.
  
  This method skips over sections with invalid names.

- `fn Object::symbol_by_name<'file>(self: &'file Self, symbol_name: &str) -> Option<<Self as >::Symbol>`

  Get the symbol named `symbol_name`, if the symbol exists.

- `fn Object::symbol_by_name_bytes<'file>(self: &'file Self, symbol_name: &[u8]) -> Option<<Self as >::Symbol>`

  Like `Self::symbol_by_name`, but allows names that are not UTF-8.

- `fn Object::symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>`

  Construct a map from addresses to symbol names.
  
  The map will only contain defined text and data symbols.
  The dynamic symbol table will only be used if there are no debugging symbols.

- `fn Object::object_map(&self) -> ObjectMap<'data>`

  Construct a map from addresses to symbol names and object file names.
  
  This is derived from Mach-O STAB entries.

- `fn Object::mach_uuid(&self) -> Result<Option<[u8; 16]>>`

  The UUID from a Mach-O [`LC_UUID`](crate::macho::LC_UUID) load command.

- `fn Object::build_id(&self) -> Result<Option<&'data [u8]>>`

  The build ID from an ELF [`NT_GNU_BUILD_ID`](crate::elf::NT_GNU_BUILD_ID) note.

- `fn Object::gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>`

  The filename and CRC from a `.gnu_debuglink` section.

- `fn Object::gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>`

  The filename and build ID from a `.gnu_debugaltlink` section.

- `fn Object::pdb_info(&self) -> Result<Option<CodeView<'_>>>`

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

*Defined in [`object-0.37.3/src/read/traits.rs:340-374`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L340-L374)*

A loadable segment in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Required Methods

- `fn ObjectSegment::address(&self) -> u64`

  Returns the virtual address of the segment.

- `fn ObjectSegment::size(&self) -> u64`

  Returns the size of the segment in memory.

- `fn ObjectSegment::align(&self) -> u64`

  Returns the alignment of the segment in memory.

- `fn ObjectSegment::file_range(&self) -> (u64, u64)`

  Returns the offset and size of the segment in the file.

- `fn ObjectSegment::data(&self) -> Result<&'data [u8]>`

  Returns a reference to the file contents of the segment.
  
  The length of this data may be different from the size of the
  segment in memory.

- `fn ObjectSegment::data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the segment data in the given range.
  
  Returns `Ok(None)` if the segment does not contain the given range.

- `fn ObjectSegment::name_bytes(&self) -> Result<Option<&[u8]>>`

  Returns the name of the segment.

- `fn ObjectSegment::name(&self) -> Result<Option<&str>>`

  Returns the name of the segment.
  
  Returns an error if the name is not UTF-8.

- `fn ObjectSegment::flags(&self) -> SegmentFlags`

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

*Defined in [`object-0.37.3/src/read/traits.rs:379-462`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L379-L462)*

A section in an [`Object`](../index.md).

This trait is part of the unified read API.

<details>
<summary><strong>Methods (18)</strong> - click to expand</summary>

**Required:**
- [`ObjectSection::index`](#fn-objectsectionindex)
- [`ObjectSection::address`](#fn-objectsectionaddress)
- [`ObjectSection::size`](#fn-objectsectionsize)
- [`ObjectSection::align`](#fn-objectsectionalign)
- [`ObjectSection::file_range`](#fn-objectsectionfile-range)
- [`ObjectSection::data`](#fn-objectsectiondata)
- [`ObjectSection::data_range`](#fn-objectsectiondata-range)
- [`ObjectSection::compressed_file_range`](#fn-objectsectioncompressed-file-range)
- [`ObjectSection::compressed_data`](#fn-objectsectioncompressed-data)
- [`ObjectSection::name_bytes`](#fn-objectsectionname-bytes)
- [`ObjectSection::name`](#fn-objectsectionname)
- [`ObjectSection::segment_name_bytes`](#fn-objectsectionsegment-name-bytes)
- [`ObjectSection::segment_name`](#fn-objectsectionsegment-name)
- [`ObjectSection::kind`](#fn-objectsectionkind)
- [`ObjectSection::relocations`](#fn-objectsectionrelocations)
- [`ObjectSection::relocation_map`](#fn-objectsectionrelocation-map)
- [`ObjectSection::flags`](#fn-objectsectionflags)

**Provided:**
- [`ObjectSection::uncompressed_data`](#fn-objectsectionuncompressed-data)

</details>

#### Associated Types

- `type RelocationIterator: 1`

#### Required Methods

- `fn ObjectSection::index(&self) -> SectionIndex`

  Returns the section index.

- `fn ObjectSection::address(&self) -> u64`

  Returns the address of the section.

- `fn ObjectSection::size(&self) -> u64`

  Returns the size of the section in memory.

- `fn ObjectSection::align(&self) -> u64`

  Returns the alignment of the section in memory.

- `fn ObjectSection::file_range(&self) -> Option<(u64, u64)>`

  Returns offset and size of on-disk segment (if any).

- `fn ObjectSection::data(&self) -> Result<&'data [u8]>`

  Returns the raw contents of the section.
  
  The length of this data may be different from the size of the
  section in memory.
  
  This does not do any decompression.

- `fn ObjectSection::data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`

  Return the raw contents of the section data in the given range.
  
  This does not do any decompression.
  
  Returns `Ok(None)` if the section does not contain the given range.

- `fn ObjectSection::compressed_file_range(&self) -> Result<CompressedFileRange>`

  Returns the potentially compressed file range of the section,
  along with information about the compression.

- `fn ObjectSection::compressed_data(&self) -> Result<CompressedData<'data>>`

  Returns the potentially compressed contents of the section,
  along with information about the compression.

- `fn ObjectSection::name_bytes(&self) -> Result<&'data [u8]>`

  Returns the name of the section.

- `fn ObjectSection::name(&self) -> Result<&'data str>`

  Returns the name of the section.
  
  Returns an error if the name is not UTF-8.

- `fn ObjectSection::segment_name_bytes(&self) -> Result<Option<&[u8]>>`

  Returns the name of the segment for this section.

- `fn ObjectSection::segment_name(&self) -> Result<Option<&str>>`

  Returns the name of the segment for this section.
  
  Returns an error if the name is not UTF-8.

- `fn ObjectSection::kind(&self) -> SectionKind`

  Return the kind of this section.

- `fn ObjectSection::relocations(&self) -> <Self as >::RelocationIterator`

  Get the relocations for this section.

- `fn ObjectSection::relocation_map(&self) -> Result<RelocationMap>`

  Construct a relocation map for this section.

- `fn ObjectSection::flags(&self) -> SectionFlags`

  Section flags that are specific to each file format.

#### Provided Methods

- `fn ObjectSection::uncompressed_data(&self) -> Result<Cow<'data, [u8]>>`

  Returns the uncompressed contents of the section.
  
  The length of this data may be different from the size of the
  section in memory.
  
  If no compression is detected, then returns the data unchanged.
  Returns `Err` if decompression fails.

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

*Defined in [`object-0.37.3/src/read/traits.rs:467-487`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L467-L487)*

A COMDAT section group in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Associated Types

- `type SectionIterator: 1`

#### Required Methods

- `fn ObjectComdat::kind(&self) -> ComdatKind`

  Returns the COMDAT selection kind.

- `fn ObjectComdat::symbol(&self) -> SymbolIndex`

  Returns the index of the symbol used for the name of COMDAT section group.

- `fn ObjectComdat::name_bytes(&self) -> Result<&'data [u8]>`

  Returns the name of the COMDAT section group.

- `fn ObjectComdat::name(&self) -> Result<&'data str>`

  Returns the name of the COMDAT section group.
  
  Returns an error if the name is not UTF-8.

- `fn ObjectComdat::sections(&self) -> <Self as >::SectionIterator`

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

*Defined in [`object-0.37.3/src/read/traits.rs:492-510`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L492-L510)*

A symbol table in an [`Object`](../index.md).

This trait is part of the unified read API.

#### Associated Types

- `type Symbol: 1`

- `type SymbolIterator: 1`

#### Required Methods

- `fn ObjectSymbolTable::symbols(&self) -> <Self as >::SymbolIterator`

  Get an iterator for the symbols in the table.
  
  This may skip over symbols that are malformed or unsupported.

- `fn ObjectSymbolTable::symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>`

  Get the symbol at the given index.
  
  The meaning of the index depends on the object file.
  
  Returns an error if the index is invalid.

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

*Defined in [`object-0.37.3/src/read/traits.rs:515-576`](../../../../.source_1765900590/object-0.37.3/src/read/traits.rs#L515-L576)*

A symbol table entry in an [`Object`](../index.md).

This trait is part of the unified read API.

<details>
<summary><strong>Methods (16)</strong> - click to expand</summary>

**Required:**
- [`ObjectSymbol::index`](#fn-objectsymbolindex)
- [`ObjectSymbol::name_bytes`](#fn-objectsymbolname-bytes)
- [`ObjectSymbol::name`](#fn-objectsymbolname)
- [`ObjectSymbol::address`](#fn-objectsymboladdress)
- [`ObjectSymbol::size`](#fn-objectsymbolsize)
- [`ObjectSymbol::kind`](#fn-objectsymbolkind)
- [`ObjectSymbol::section`](#fn-objectsymbolsection)
- [`ObjectSymbol::is_undefined`](#fn-objectsymbolis-undefined)
- [`ObjectSymbol::is_definition`](#fn-objectsymbolis-definition)
- [`ObjectSymbol::is_common`](#fn-objectsymbolis-common)
- [`ObjectSymbol::is_weak`](#fn-objectsymbolis-weak)
- [`ObjectSymbol::scope`](#fn-objectsymbolscope)
- [`ObjectSymbol::is_global`](#fn-objectsymbolis-global)
- [`ObjectSymbol::is_local`](#fn-objectsymbolis-local)
- [`ObjectSymbol::flags`](#fn-objectsymbolflags)

**Provided:**
- [`ObjectSymbol::section_index`](#fn-objectsymbolsection-index)

</details>

#### Required Methods

- `fn ObjectSymbol::index(&self) -> SymbolIndex`

  The index of the symbol.

- `fn ObjectSymbol::name_bytes(&self) -> Result<&'data [u8]>`

  The name of the symbol.

- `fn ObjectSymbol::name(&self) -> Result<&'data str>`

  The name of the symbol.
  
  Returns an error if the name is not UTF-8.

- `fn ObjectSymbol::address(&self) -> u64`

  The address of the symbol. May be zero if the address is unknown.

- `fn ObjectSymbol::size(&self) -> u64`

  The size of the symbol. May be zero if the size is unknown.

- `fn ObjectSymbol::kind(&self) -> SymbolKind`

  Return the kind of this symbol.

- `fn ObjectSymbol::section(&self) -> SymbolSection`

  Returns the section where the symbol is defined.

- `fn ObjectSymbol::is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn ObjectSymbol::is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object
  that has a known address.
  
  This is primarily used to implement `Object::symbol_map`.

- `fn ObjectSymbol::is_common(&self) -> bool`

  Return true if the symbol is common data.
  
  Note: does not check for [`SymbolSection::Section`](../../index.md) with [`SectionKind::Common`](../../index.md).

- `fn ObjectSymbol::is_weak(&self) -> bool`

  Return true if the symbol is weak.

- `fn ObjectSymbol::scope(&self) -> SymbolScope`

  Returns the symbol scope.

- `fn ObjectSymbol::is_global(&self) -> bool`

  Return true if the symbol visible outside of the compilation unit.
  
  This treats [`SymbolScope::Unknown`](../../index.md) as global.

- `fn ObjectSymbol::is_local(&self) -> bool`

  Return true if the symbol is only visible within the compilation unit.

- `fn ObjectSymbol::flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>`

  Symbol flags that are specific to each file format.

#### Provided Methods

- `fn ObjectSymbol::section_index(&self) -> Option<SectionIndex>`

  Returns the section index for the section containing this symbol.
  
  May return `None` if the symbol is not defined in a section.

#### Implementors

- [`CoffSymbol`](../coff/index.md#coffsymbol)
- [`ElfSymbol`](../elf/index.md#elfsymbol)
- [`MachOSymbol`](../macho/index.md#machosymbol)
- [`Symbol`](../index.md#symbol)
- [`XcoffSymbol`](../xcoff/index.md#xcoffsymbol)

