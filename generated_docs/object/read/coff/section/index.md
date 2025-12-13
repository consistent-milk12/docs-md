*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`SectionTable`](#sectiontable)
  - [`CoffSegmentIterator`](#coffsegmentiterator)
  - [`CoffSegment`](#coffsegment)
  - [`CoffSectionIterator`](#coffsectioniterator)
  - [`CoffSection`](#coffsection)
- [Type Aliases](#type-aliases)
  - [`CoffBigSegmentIterator`](#coffbigsegmentiterator)
  - [`CoffBigSegment`](#coffbigsegment)
  - [`CoffBigSectionIterator`](#coffbigsectioniterator)
  - [`CoffBigSection`](#coffbigsection)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SectionTable`](#sectiontable) | struct | The table of section headers in a COFF or PE file. |
| [`CoffSegmentIterator`](#coffsegmentiterator) | struct | An iterator for the loadable sections in a [`CoffFile`]. |
| [`CoffSegment`](#coffsegment) | struct | A loadable section in a [`CoffFile`]. |
| [`CoffSectionIterator`](#coffsectioniterator) | struct | An iterator for the sections in a [`CoffFile`]. |
| [`CoffSection`](#coffsection) | struct | A section in a [`CoffFile`]. |
| [`CoffBigSegmentIterator`](#coffbigsegmentiterator) | type | An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSegment`](#coffbigsegment) | type | A loadable section in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSectionIterator`](#coffbigsectioniterator) | type | An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSection`](#coffbigsection) | type | A section in a [`CoffBigFile`](super::CoffBigFile). |

## Structs

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    sections: &'data [pe::ImageSectionHeader],
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:19-21`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L19-L21)*

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- <span id="sectiontable-parse"></span>`fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the section table.

  

  `data` must be the entire file data.

  `offset` must be after the optional file header.

- <span id="sectiontable-iter"></span>`fn iter(&self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Iterate over the section headers.

  

  Warning: section indices start at 1.

- <span id="sectiontable-enumerate"></span>`fn enumerate(&self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../../index.md#sectionindex), [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Iterate over the section headers and their indices.

- <span id="sectiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the section table is empty.

- <span id="sectiontable-len"></span>`fn len(&self) -> usize`

  The number of section headers.

- <span id="sectiontable-section"></span>`fn section(&self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result), [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Return the section header at the given index.

  

  The index is 1-based.

- <span id="sectiontable-section-by-name"></span>`fn section_by_name<R: ReadRef<'data>>(&self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../../index.md#stringtable), [`SectionIndex`](../../../index.md#sectionindex), [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Return the section header with the given name.

  

  The returned index is 1-based.

  

  Ignores sections with invalid names.

- <span id="sectiontable-max-section-file-offset"></span>`fn max_section_file_offset(&self) -> u64`

  Compute the maximum file offset used by sections.

  

  This will usually match the end of file, unless the PE file has a

  [data overlay](https://security.stackexchange.com/questions/77336/how-is-the-file-overlay-read-by-an-exe-virus)

#### Trait Implementations

##### `impl Any for SectionTable<'data>`

- <span id="sectiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionTable<'data>`

- <span id="sectiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionTable<'data>`

- <span id="sectiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionTable<'data>`

- <span id="sectiontable-clone"></span>`fn clone(&self) -> SectionTable<'data>` — [`SectionTable`](../index.md#sectiontable)

##### `impl CloneToUninit for SectionTable<'data>`

- <span id="sectiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionTable<'data>`

##### `impl Debug for SectionTable<'data>`

- <span id="sectiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionTable<'data>`

- <span id="sectiontable-default"></span>`fn default() -> SectionTable<'data>` — [`SectionTable`](../index.md#sectiontable)

##### `impl<T> From for SectionTable<'data>`

- <span id="sectiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionTable<'data>`

- <span id="sectiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SectionTable<'data>`

- <span id="sectiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="sectiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionTable<'data>`

- <span id="sectiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionTable<'data>`

- <span id="sectiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegmentIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:121-129`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L121-L129)*

An iterator for the loadable sections in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl Any for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-iterator-type-item"></span>`type Item = CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffsegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffSegmentIterator<'data, 'file, R, Coff>`

- <span id="coffsegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffsegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSegment<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section: &'data pe::ImageSectionHeader,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:154-162`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L154-L162)*

A loadable section in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- <span id="coffsegment-coff-file"></span>`fn coff_file(&self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](../index.md#cofffile)

  Get the COFF file containing this segment.

- <span id="coffsegment-coff-section"></span>`fn coff_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Get the raw COFF section header.

- <span id="coffsegment-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Any for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSegment for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="coffsegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="coffsegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="coffsegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="coffsegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="coffsegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="coffsegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="coffsegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="coffsegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md#segmentflags)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSegment<'data, 'file, R, Coff>`

##### `impl<U> TryFrom for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffsegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffSegment<'data, 'file, R, Coff>`

- <span id="coffsegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffsegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:253-261`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L253-L261)*

An iterator for the sections in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl Any for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-iterator-type-item"></span>`type Item = CoffSection<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffSectionIterator<'data, 'file, R, Coff>`

- <span id="coffsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffSection<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:287-296`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L287-L296)*

A section in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="coffsection-coff-file"></span>`fn coff_file(&self) -> &'file CoffFile<'data, R, Coff>` — [`CoffFile`](../index.md#cofffile)

  Get the COFF file containing this section.

- <span id="coffsection-coff-section"></span>`fn coff_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Get the raw COFF section header.

- <span id="coffsection-coff-relocations"></span>`fn coff_relocations(&self) -> Result<&'data [pe::ImageRelocation]>` — [`Result`](../../../index.md#result), [`ImageRelocation`](../../../pe/index.md#imagerelocation)

  Get the raw COFF relocations for this section.

- <span id="coffsection-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Any for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectSection for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-objectsection-type-relocationiterator"></span>`type RelocationIterator = CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffsection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

- <span id="coffsection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="coffsection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="coffsection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="coffsection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="coffsection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="coffsection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="coffsection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="coffsection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md#result), [`CompressedData`](../../../index.md#compresseddata)

- <span id="coffsection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="coffsection-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="coffsection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="coffsection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="coffsection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md#sectionkind)

- <span id="coffsection-objectsection-relocations"></span>`fn relocations(&self) -> CoffRelocationIterator<'data, 'file, R, Coff>` — [`CoffRelocationIterator`](../index.md#coffrelocationiterator)

- <span id="coffsection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md#result), [`RelocationMap`](../../../index.md#relocationmap)

- <span id="coffsection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md#sectionflags)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSection<'data, 'file, R, Coff>`

##### `impl<U> TryFrom for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffSection<'data, 'file, R, Coff>`

- <span id="coffsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `CoffBigSegmentIterator<'data, 'file, R>`

```rust
type CoffBigSegmentIterator<'data, 'file, R> = CoffSegmentIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:116-117`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L116-L117)*

An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSegment<'data, 'file, R>`

```rust
type CoffBigSegment<'data, 'file, R> = CoffSegment<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:147-148`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L147-L148)*

A loadable section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

### `CoffBigSectionIterator<'data, 'file, R>`

```rust
type CoffBigSectionIterator<'data, 'file, R> = CoffSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:248-249`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L248-L249)*

An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSection<'data, 'file, R>`

```rust
type CoffBigSection<'data, 'file, R> = CoffSection<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/section.rs:280-281`](../../../../../.source_1765633015/object-0.37.3/src/read/coff/section.rs#L280-L281)*

A section in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

