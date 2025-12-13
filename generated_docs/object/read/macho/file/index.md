*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [file](index.md)*

---

# Module `file`

## Contents

- [Structs](#structs)
  - [`MachOFile`](#machofile)
  - [`MachOComdatIterator`](#machocomdatiterator)
  - [`MachOComdat`](#machocomdat)
  - [`MachOComdatSectionIterator`](#machocomdatsectioniterator)
- [Traits](#traits)
  - [`MachHeader`](#machheader)
- [Type Aliases](#type-aliases)
  - [`MachOFile32`](#machofile32)
  - [`MachOFile64`](#machofile64)
  - [`MachOComdatIterator32`](#machocomdatiterator32)
  - [`MachOComdatIterator64`](#machocomdatiterator64)
  - [`MachOComdat32`](#machocomdat32)
  - [`MachOComdat64`](#machocomdat64)
  - [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32)
  - [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MachOFile`](#machofile) | struct | A partially parsed Mach-O file. |
| [`MachOComdatIterator`](#machocomdatiterator) | struct | An iterator for the COMDAT section groups in a [`MachOFile`]. |
| [`MachOComdat`](#machocomdat) | struct | A COMDAT section group in a [`MachOFile`]. |
| [`MachOComdatSectionIterator`](#machocomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`MachOFile`]. |
| [`MachHeader`](#machheader) | trait | A trait for generic access to [`macho::MachHeader32`] and [`macho::MachHeader64`]. |
| [`MachOFile32`](#machofile32) | type | A 32-bit Mach-O object file. |
| [`MachOFile64`](#machofile64) | type | A 64-bit Mach-O object file. |
| [`MachOComdatIterator32`](#machocomdatiterator32) | type | An iterator for the COMDAT section groups in a [`MachOFile64`]. |
| [`MachOComdatIterator64`](#machocomdatiterator64) | type | An iterator for the COMDAT section groups in a [`MachOFile64`]. |
| [`MachOComdat32`](#machocomdat32) | type | A COMDAT section group in a [`MachOFile32`]. |
| [`MachOComdat64`](#machocomdat64) | type | A COMDAT section group in a [`MachOFile64`]. |
| [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`MachOFile32`]. |
| [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`MachOFile64`]. |

## Structs

### `MachOFile<'data, Mach, R>`

```rust
struct MachOFile<'data, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    endian: <Mach as >::Endian,
    data: R,
    header_offset: u64,
    header: &'data Mach,
    segments: alloc::vec::Vec<super::MachOSegmentInternal<'data, Mach, R>>,
    sections: alloc::vec::Vec<super::MachOSectionInternal<'data, Mach, R>>,
    symbols: super::SymbolTable<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:37-49`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L37-L49)*

A partially parsed Mach-O file.

Most of the functionality of this type is provided by the [`Object`](../../index.md) trait implementation.

#### Implementations

- <span id="machofile-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the raw Mach-O file data.

- <span id="machofile-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: Endian>(image: &DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](../index.md#dyldcacheimage), [`Result`](../../../index.md#result)

  Parse the Mach-O file for the given image from the dyld shared cache.

  This will read different sections from different subcaches, if necessary.

- <span id="machofile-section-internal"></span>`fn section_internal(&self, index: SectionIndex) -> Result<&MachOSectionInternal<'data, Mach, R>>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`MachOSectionInternal`](../section/index.md#machosectioninternal)

  Return the section at the given index.

- <span id="machofile-endian"></span>`fn endian(&self) -> <Mach as >::Endian` — [`MachHeader`](../index.md#machheader)

  Returns the endianness.

- <span id="machofile-data"></span>`fn data(&self) -> R`

  Returns the raw data.

- <span id="machofile-raw-header"></span>`fn raw_header(&self) -> &'data Mach`

  Returns the raw Mach-O file header.

- <span id="machofile-macho-header"></span>`fn macho_header(&self) -> &'data Mach`

  Get the raw Mach-O file header.

- <span id="machofile-macho-load-commands"></span>`fn macho_load_commands(&self) -> Result<LoadCommandIterator<'data, <Mach as >::Endian>>` — [`Result`](../../../index.md#result), [`LoadCommandIterator`](../index.md#loadcommanditerator), [`MachHeader`](../index.md#machheader)

  Get the Mach-O load commands.

- <span id="machofile-macho-symbol-table"></span>`fn macho_symbol_table(&self) -> &SymbolTable<'data, Mach, R>` — [`SymbolTable`](../index.md#symboltable)

  Get the Mach-O symbol table.

  

  Returns an empty symbol table if the file has no symbol table.

- <span id="machofile-build-version"></span>`fn build_version(&self) -> Result<Option<&'data macho::BuildVersionCommand<<Mach as >::Endian>>>` — [`Result`](../../../index.md#result), [`BuildVersionCommand`](../../../macho/index.md#buildversioncommand), [`MachHeader`](../index.md#machheader)

  Return the `LC_BUILD_VERSION` load command if present.

#### Trait Implementations

##### `impl Any for MachOFile<'data, Mach, R>`

- <span id="machofile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOFile<'data, Mach, R>`

- <span id="machofile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOFile<'data, Mach, R>`

- <span id="machofile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOFile<'data, Mach, R>`

- <span id="machofile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOFile<'data, Mach, R>`

- <span id="machofile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOFile<'data, Mach, R>`

- <span id="machofile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Mach, R> Object for MachOFile<'data, Mach, R>`

- <span id="machofile-object-type-segment"></span>`type Segment = MachOSegment<'data, 'file, Mach, R>`

- <span id="machofile-object-type-segmentiterator"></span>`type SegmentIterator = MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machofile-object-type-section"></span>`type Section = MachOSection<'data, 'file, Mach, R>`

- <span id="machofile-object-type-sectioniterator"></span>`type SectionIterator = MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machofile-object-type-comdat"></span>`type Comdat = MachOComdat<'data, 'file, Mach, R>`

- <span id="machofile-object-type-comdatiterator"></span>`type ComdatIterator = MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machofile-object-type-symbol"></span>`type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machofile-object-type-symboliterator"></span>`type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machofile-object-type-symboltable"></span>`type SymbolTable = MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machofile-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = NoDynamicRelocationIterator`

- <span id="machofile-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../../index.md#architecture)

- <span id="machofile-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../../index.md#subarchitecture)

- <span id="machofile-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="machofile-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="machofile-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../../index.md#objectkind)

- <span id="machofile-object-segments"></span>`fn segments(&self) -> MachOSegmentIterator<'data, '_, Mach, R>` — [`MachOSegmentIterator`](../index.md#machosegmentiterator)

- <span id="machofile-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<MachOSection<'data, 'file, Mach, R>>` — [`MachOSection`](../index.md#machosection)

- <span id="machofile-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<MachOSection<'data, '_, Mach, R>>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`MachOSection`](../index.md#machosection)

- <span id="machofile-object-sections"></span>`fn sections(&self) -> MachOSectionIterator<'data, '_, Mach, R>` — [`MachOSectionIterator`](../index.md#machosectioniterator)

- <span id="machofile-object-comdats"></span>`fn comdats(&self) -> MachOComdatIterator<'data, '_, Mach, R>` — [`MachOComdatIterator`](../index.md#machocomdatiterator)

- <span id="machofile-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<MachOSymbol<'data, '_, Mach, R>>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`MachOSymbol`](../index.md#machosymbol)

- <span id="machofile-object-symbols"></span>`fn symbols(&self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](../index.md#machosymboliterator)

- <span id="machofile-object-symbol-table"></span>`fn symbol_table(&self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](../index.md#machosymboltable)

- <span id="machofile-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> MachOSymbolIterator<'data, '_, Mach, R>` — [`MachOSymbolIterator`](../index.md#machosymboliterator)

- <span id="machofile-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<MachOSymbolTable<'data, '_, Mach, R>>` — [`MachOSymbolTable`](../index.md#machosymboltable)

- <span id="machofile-object-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../../../index.md#objectmap)

- <span id="machofile-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../../index.md#result), [`Import`](../../../index.md#import)

- <span id="machofile-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../../index.md#result), [`Export`](../../../index.md#export)

- <span id="machofile-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<NoDynamicRelocationIterator>` — [`NoDynamicRelocationIterator`](../../index.md#nodynamicrelocationiterator)

- <span id="machofile-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="machofile-object-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../../../index.md#result)

- <span id="machofile-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="machofile-object-entry"></span>`fn entry(&self) -> u64`

- <span id="machofile-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../../index.md#fileflags)

##### `impl<Mach, R> Sealed for MachOFile<'data, Mach, R>`

##### `impl<U> TryFrom for MachOFile<'data, Mach, R>`

- <span id="machofile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machofile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOFile<'data, Mach, R>`

- <span id="machofile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machofile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOComdatIterator<'data, 'file, Mach, R>`

```rust
struct MachOComdatIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:527-534`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L527-L534)*

An iterator for the COMDAT section groups in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machocomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machocomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Mach, R> Iterator for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-iterator-type-item"></span>`type Item = MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machocomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOComdatIterator<'data, 'file, Mach, R>`

- <span id="machocomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machocomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOComdat<'data, 'file, Mach, R>`

```rust
struct MachOComdat<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:561-568`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L561-L568)*

A COMDAT section group in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Mach, R> ObjectComdat for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md#comdatkind)

- <span id="machocomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="machocomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machocomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="machocomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md#objectcomdat)

##### `impl<Mach, R> Sealed for MachOComdat<'data, 'file, Mach, R>`

##### `impl<U> TryFrom for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machocomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOComdat<'data, 'file, Mach, R>`

- <span id="machocomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machocomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOComdatSectionIterator<'data, 'file, Mach, R>`

```rust
struct MachOComdatSectionIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:621-628`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L621-L628)*

An iterator for the sections in a COMDAT section group in a [`MachOFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machocomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machocomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Mach, R> Iterator for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="machocomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machocomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOComdatSectionIterator<'data, 'file, Mach, R>`

- <span id="machocomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machocomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `MachHeader`

```rust
trait MachHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:644-723`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L644-L723)*

A trait for generic access to [`macho::MachHeader32`](../../../macho/index.md) and [`macho::MachHeader64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

- `type Segment: 1`

- `type Section: 1`

- `type Nlist: 1`

#### Required Methods

- `fn is_type_64(&self) -> bool`

  Return true if this type is a 64-bit header.

- `fn is_big_endian(&self) -> bool`

  Return true if the `magic` field signifies big-endian.

- `fn is_little_endian(&self) -> bool`

  Return true if the `magic` field signifies little-endian.

- `fn magic(&self) -> u32`

- `fn cputype(&self, endian: <Self as >::Endian) -> u32`

- `fn cpusubtype(&self, endian: <Self as >::Endian) -> u32`

- `fn filetype(&self, endian: <Self as >::Endian) -> u32`

- `fn ncmds(&self, endian: <Self as >::Endian) -> u32`

- `fn sizeofcmds(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn parse<'data, R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<&'data Self>`

  Read the file header.

- `fn is_supported(&self) -> bool`

- `fn endian(&self) -> Result<<Self as >::Endian>`

- `fn load_commands<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<LoadCommandIterator<'data, <Self as >::Endian>>`

- `fn uuid<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R, header_offset: u64) -> Result<Option<[u8; 16]>>`

  Return the UUID from the `LC_UUID` load command, if one is present.

#### Implementors

- [`MachHeader32`](../../../macho/index.md#machheader32)
- [`MachHeader64`](../../../macho/index.md#machheader64)

## Type Aliases

### `MachOFile32<'data, Endian, R>`

```rust
type MachOFile32<'data, Endian, R> = MachOFile<'data, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:24-25`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L24-L25)*

A 32-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader32`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachO32`](../../../index.md).

### `MachOFile64<'data, Endian, R>`

```rust
type MachOFile64<'data, Endian, R> = MachOFile<'data, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:30-31`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L30-L31)*

A 64-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader64`](../../../macho/index.md), and corresponds
to [`crate::FileKind::MachO64`](../../../index.md).

### `MachOComdatIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator32<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:517-518`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L517-L518)*

An iterator for the COMDAT section groups in a [`MachOFile64`](../index.md).

### `MachOComdatIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatIterator64<'data, 'file, Endian, R> = MachOComdatIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:520-521`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L520-L521)*

An iterator for the COMDAT section groups in a [`MachOFile64`](../index.md).

### `MachOComdat32<'data, 'file, Endian, R>`

```rust
type MachOComdat32<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:550-551`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L550-L551)*

A COMDAT section group in a [`MachOFile32`](../index.md).

### `MachOComdat64<'data, 'file, Endian, R>`

```rust
type MachOComdat64<'data, 'file, Endian, R> = MachOComdat<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:554-555`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L554-L555)*

A COMDAT section group in a [`MachOFile64`](../index.md).

### `MachOComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator32<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:611-612`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L611-L612)*

An iterator for the sections in a COMDAT section group in a [`MachOFile32`](../index.md).

### `MachOComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOComdatSectionIterator64<'data, 'file, Endian, R> = MachOComdatSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/file.rs:614-615`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/file.rs#L614-L615)*

An iterator for the sections in a COMDAT section group in a [`MachOFile64`](../index.md).

