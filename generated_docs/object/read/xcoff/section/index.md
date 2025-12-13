*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`XcoffSectionIterator`](#xcoffsectioniterator)
  - [`XcoffSection`](#xcoffsection)
  - [`SectionTable`](#sectiontable)
- [Traits](#traits)
  - [`SectionHeader`](#sectionheader)
- [Type Aliases](#type-aliases)
  - [`XcoffSectionIterator32`](#xcoffsectioniterator32)
  - [`XcoffSectionIterator64`](#xcoffsectioniterator64)
  - [`XcoffSection32`](#xcoffsection32)
  - [`XcoffSection64`](#xcoffsection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffSectionIterator`](#xcoffsectioniterator) | struct | An iterator for the sections in an [`XcoffFile`]. |
| [`XcoffSection`](#xcoffsection) | struct | A section in an [`XcoffFile`]. |
| [`SectionTable`](#sectiontable) | struct | The table of section headers in an XCOFF file. |
| [`SectionHeader`](#sectionheader) | trait | A trait for generic access to [`xcoff::SectionHeader32`] and [`xcoff::SectionHeader64`]. |
| [`XcoffSectionIterator32`](#xcoffsectioniterator32) | type | An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSectionIterator64`](#xcoffsectioniterator64) | type | An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSection32`](#xcoffsection32) | type | A section in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSection64`](#xcoffsection64) | type | A section in an [`XcoffFile64`](super::XcoffFile64). |

## Structs

### `XcoffSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Xcoff as >::SectionHeader>>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:23-30`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L23-L30)*

An iterator for the sections in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl Any for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-iterator-type-item"></span>`type Item = XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffSection<'data, 'file, Xcoff, R>`

```rust
struct XcoffSection<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    section: &'data <Xcoff as >::SectionHeader,
    index: crate::read::SectionIndex,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:59-67`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L59-L67)*

A section in an [`XcoffFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="xcoffsection-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](../index.md#xcofffile)

  Get the XCOFF file containing this section.

- <span id="xcoffsection-xcoff-section"></span>`fn xcoff_section(&self) -> &'data <Xcoff as >::SectionHeader` — [`FileHeader`](../index.md#fileheader)

  Get the raw XCOFF section header.

- <span id="xcoffsection-xcoff-relocations"></span>`fn xcoff_relocations(&self) -> Result<&'data [<Xcoff as >::Rel]>` — [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Get the raw XCOFF relocation entries for this section.

- <span id="xcoffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Any for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> ObjectSection for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

- <span id="xcoffsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="xcoffsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="xcoffsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="xcoffsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md#result), [`CompressedData`](../../../index.md#compresseddata)

- <span id="xcoffsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="xcoffsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md#sectionkind)

- <span id="xcoffsection-objectsection-relocations"></span>`fn relocations(&self) -> <Self as >::RelocationIterator` — [`ObjectSection`](../../index.md#objectsection)

- <span id="xcoffsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md#result), [`RelocationMap`](../../../index.md#relocationmap)

- <span id="xcoffsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md#sectionflags)

- <span id="xcoffsection-objectsection-uncompressed-data"></span>`fn uncompressed_data(&self) -> Result<alloc::borrow::Cow<'data, [u8]>>` — [`Result`](../../../index.md#result)

##### `impl<Xcoff, R> Sealed for XcoffSection<'data, 'file, Xcoff, R>`

##### `impl<U> TryFrom for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSection<'data, 'file, Xcoff, R>`

- <span id="xcoffsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionTable<'data, Xcoff: FileHeader>`

```rust
struct SectionTable<'data, Xcoff: FileHeader> {
    sections: &'data [<Xcoff as >::SectionHeader],
}
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:228-230`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L228-L230)*

The table of section headers in an XCOFF file.

Returned by `FileHeader::sections`.

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<R: ReadRef<'data>>(header: &Xcoff, data: R, offset: &mut u64) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Xcoff as >::SectionHeader>` — [`FileHeader`](../index.md#fileheader)

  Iterate over the section headers.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data <Xcoff as >::SectionHeader>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Return the section header at the given index.

  

  The index is 1-based.

#### Trait Implementations

##### `impl Any for SectionTable<'data, Xcoff>`

- <span id="sectiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionTable<'data, Xcoff>`

- <span id="sectiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionTable<'data, Xcoff>`

- <span id="sectiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff: clone::Clone + FileHeader> Clone for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data, Xcoff>` — [`SectionTable`](../index.md#sectiontable)

##### `impl CloneToUninit for SectionTable<'data, Xcoff>`

- <span id="sectiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Xcoff: marker::Copy + FileHeader> Copy for SectionTable<'data, Xcoff>`

##### `impl<Xcoff: fmt::Debug + FileHeader> Debug for SectionTable<'data, Xcoff>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff> Default for SectionTable<'data, Xcoff>`

- <span id="sectiontable-default"></span>`fn default() -> Self`

##### `impl<T> From for SectionTable<'data, Xcoff>`

- <span id="sectiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionTable<'data, Xcoff>`

- <span id="sectiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SectionTable<'data, Xcoff>`

- <span id="sectiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="sectiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionTable<'data, Xcoff>`

- <span id="sectiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionTable<'data, Xcoff>`

- <span id="sectiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `SectionHeader`

```rust
trait SectionHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:290-335`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L290-L335)*

A trait for generic access to [`xcoff::SectionHeader32`](../../../xcoff/index.md) and [`xcoff::SectionHeader64`](../../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

- `type HalfWord: 1`

- `type Xcoff: 1`

- `type Rel: 1`

#### Required Methods

- `fn s_name(&self) -> &[u8; 8]`

- `fn s_paddr(&self) -> <Self as >::Word`

- `fn s_vaddr(&self) -> <Self as >::Word`

- `fn s_size(&self) -> <Self as >::Word`

- `fn s_scnptr(&self) -> <Self as >::Word`

- `fn s_relptr(&self) -> <Self as >::Word`

- `fn s_lnnoptr(&self) -> <Self as >::Word`

- `fn s_nreloc(&self) -> <Self as >::HalfWord`

- `fn s_nlnno(&self) -> <Self as >::HalfWord`

- `fn s_flags(&self) -> u32`

- `fn relocations<'data, R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [<Self as >::Rel]>`

  Read the relocations.

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the section name.

- `fn file_range(&self) -> Option<(u64, u64)>`

  Return the offset and size of the section in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, data: R) -> result::Result<&'data [u8], ()>`

  Return the section data.

#### Implementors

- [`SectionHeader32`](../../../xcoff/index.md#sectionheader32)
- [`SectionHeader64`](../../../xcoff/index.md#sectionheader64)

## Type Aliases

### `XcoffSectionIterator32<'data, 'file, R>`

```rust
type XcoffSectionIterator32<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:15-16`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L15-L16)*

An iterator for the sections in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSectionIterator64<'data, 'file, R>`

```rust
type XcoffSectionIterator64<'data, 'file, R> = XcoffSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:18-19`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L18-L19)*

An iterator for the sections in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSection32<'data, 'file, R>`

```rust
type XcoffSection32<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:49-50`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L49-L50)*

A section in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSection64<'data, 'file, R>`

```rust
type XcoffSection64<'data, 'file, R> = XcoffSection<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/section.rs:52-53`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/section.rs#L52-L53)*

A section in an [`XcoffFile64`](super::XcoffFile64).

