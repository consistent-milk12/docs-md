*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`MachOSectionIterator`](#machosectioniterator)
  - [`MachOSection`](#machosection)
  - [`MachOSectionInternal`](#machosectioninternal)
- [Traits](#traits)
  - [`Section`](#section)
- [Type Aliases](#type-aliases)
  - [`MachOSectionIterator32`](#machosectioniterator32)
  - [`MachOSectionIterator64`](#machosectioniterator64)
  - [`MachOSection32`](#machosection32)
  - [`MachOSection64`](#machosection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MachOSectionIterator`](#machosectioniterator) | struct | An iterator for the sections in a [`MachOFile`]. |
| [`MachOSection`](#machosection) | struct | A section in a [`MachOFile`]. |
| [`MachOSectionInternal`](#machosectioninternal) | struct |  |
| [`Section`](#section) | trait | A trait for generic access to [`macho::Section32`] and [`macho::Section64`]. |
| [`MachOSectionIterator32`](#machosectioniterator32) | type | An iterator for the sections in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSectionIterator64`](#machosectioniterator64) | type | An iterator for the sections in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSection32`](#machosection32) | type | A section in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSection64`](#machosection64) | type | A section in a [`MachOFile64`](super::MachOFile64). |

## Structs

### `MachOSectionIterator<'data, 'file, Mach, R>`

```rust
struct MachOSectionIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    iter: slice::Iter<'file, MachOSectionInternal<'data, Mach, R>>,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:22-29`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L22-L29)*

An iterator for the sections in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl Any for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Mach, R> Iterator for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-iterator-type-item"></span>`type Item = MachOSection<'data, 'file, Mach, R>`

- <span id="machosectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSectionIterator<'data, 'file, Mach, R>`

- <span id="machosectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSection<'data, 'file, Mach, R>`

```rust
struct MachOSection<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    internal: MachOSectionInternal<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:68-75`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L68-L75)*

A section in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="machosection-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md#machofile)

  Get the Mach-O file containing this section.

- <span id="machosection-macho-section"></span>`fn macho_section(&self) -> &'data <Mach as >::Section` — [`MachHeader`](../index.md#machheader)

  Get the raw Mach-O section structure.

- <span id="machosection-macho-relocations"></span>`fn macho_relocations(&self) -> Result<&'data [macho::Relocation<<Mach as >::Endian>]>` — [`Result`](../../../index.md#result), [`Relocation`](../../../macho/index.md#relocation), [`MachHeader`](../index.md#machheader)

  Get the raw Mach-O relocation entries.

- <span id="machosection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machosection-maybe-compressed-gnu"></span>`fn maybe_compressed_gnu(&self) -> Result<Option<CompressedFileRange>>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

#### Trait Implementations

##### `impl Any for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Mach, R> ObjectSection for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-objectsection-type-relocationiterator"></span>`type RelocationIterator = MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machosection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

- <span id="machosection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="machosection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="machosection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="machosection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="machosection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machosection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="machosection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="machosection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> read::Result<CompressedData<'data>>` — [`Result`](../../../index.md#result), [`CompressedData`](../../../index.md#compresseddata)

- <span id="machosection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machosection-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="machosection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="machosection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="machosection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md#sectionkind)

- <span id="machosection-objectsection-relocations"></span>`fn relocations(&self) -> MachORelocationIterator<'data, 'file, Mach, R>` — [`MachORelocationIterator`](../index.md#machorelocationiterator)

- <span id="machosection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md#result), [`RelocationMap`](../../../index.md#relocationmap)

- <span id="machosection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md#sectionflags)

##### `impl<Mach, R> Sealed for MachOSection<'data, 'file, Mach, R>`

##### `impl<U> TryFrom for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSection<'data, 'file, Mach, R>`

- <span id="machosection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSectionInternal<'data, Mach: MachHeader, R: ReadRef<'data>>`

```rust
struct MachOSectionInternal<'data, Mach: MachHeader, R: ReadRef<'data>> {
    pub index: crate::read::SectionIndex,
    pub kind: crate::read::SectionKind,
    pub section: &'data <Mach as >::Section,
    pub data: R,
}
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:241-250`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L241-L250)*

#### Fields

- **`data`**: `R`

  The data for the file that contains the section data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Implementations

- <span id="machosectioninternal-parse"></span>`fn parse(index: SectionIndex, section: &'data <Mach as >::Section, data: R) -> Self` — [`SectionIndex`](../../../index.md#sectionindex), [`MachHeader`](../index.md#machheader)

#### Trait Implementations

##### `impl Any for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach: clone::Clone + MachHeader, R: clone::Clone + ReadRef<'data>> Clone for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-clone"></span>`fn clone(&self) -> MachOSectionInternal<'data, Mach, R>` — [`MachOSectionInternal`](#machosectioninternal)

##### `impl CloneToUninit for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Mach: marker::Copy + MachHeader, R: marker::Copy + ReadRef<'data>> Copy for MachOSectionInternal<'data, Mach, R>`

##### `impl<Mach: fmt::Debug + MachHeader, R: fmt::Debug + ReadRef<'data>> Debug for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-toowned-type-owned"></span>`type Owned = T`

- <span id="machosectioninternal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="machosectioninternal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosectioninternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSectionInternal<'data, Mach, R>`

- <span id="machosectioninternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosectioninternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Section`

```rust
trait Section: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:285-354`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L285-L354)*

A trait for generic access to [`macho::Section32`](../../../macho/index.md) and [`macho::Section64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn sectname(&self) -> &[u8; 16]`

- `fn segname(&self) -> &[u8; 16]`

- `fn addr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn size(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn offset(&self, endian: <Self as >::Endian) -> u32`

- `fn align(&self, endian: <Self as >::Endian) -> u32`

- `fn reloff(&self, endian: <Self as >::Endian) -> u32`

- `fn nreloc(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the `sectname` bytes up until the null terminator.

- `fn segment_name(&self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(&self, endian: <Self as >::Endian) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

- `fn relocations<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> Result<&'data [macho::Relocation<<Self as >::Endian>]>`

  Return the relocation array.

#### Implementors

- [`Section32`](../../../macho/index.md#section32)
- [`Section64`](../../../macho/index.md#section64)

## Type Aliases

### `MachOSectionIterator32<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator32<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:15-16`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L15-L16)*

An iterator for the sections in a [`MachOFile32`](super::MachOFile32).

### `MachOSectionIterator64<'data, 'file, Endian, R>`

```rust
type MachOSectionIterator64<'data, 'file, Endian, R> = MachOSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:18-19`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L18-L19)*

An iterator for the sections in a [`MachOFile64`](super::MachOFile64).

### `MachOSection32<'data, 'file, Endian, R>`

```rust
type MachOSection32<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:58-59`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L58-L59)*

A section in a [`MachOFile32`](super::MachOFile32).

### `MachOSection64<'data, 'file, Endian, R>`

```rust
type MachOSection64<'data, 'file, Endian, R> = MachOSection<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/section.rs:61-62`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/section.rs#L61-L62)*

A section in a [`MachOFile64`](super::MachOFile64).

