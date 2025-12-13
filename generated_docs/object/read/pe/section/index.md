*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [section](index.md)*

---

# Module `section`

## Contents

- [Structs](#structs)
  - [`PeSegmentIterator`](#pesegmentiterator)
  - [`PeSegment`](#pesegment)
  - [`PeSectionIterator`](#pesectioniterator)
  - [`PeSection`](#pesection)
  - [`PeRelocationIterator`](#perelocationiterator)
- [Type Aliases](#type-aliases)
  - [`PeSegmentIterator32`](#pesegmentiterator32)
  - [`PeSegmentIterator64`](#pesegmentiterator64)
  - [`PeSegment32`](#pesegment32)
  - [`PeSegment64`](#pesegment64)
  - [`PeSectionIterator32`](#pesectioniterator32)
  - [`PeSectionIterator64`](#pesectioniterator64)
  - [`PeSection32`](#pesection32)
  - [`PeSection64`](#pesection64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeSegmentIterator`](#pesegmentiterator) | struct | An iterator for the loadable sections in a [`PeFile`]. |
| [`PeSegment`](#pesegment) | struct | A loadable section in a [`PeFile`]. |
| [`PeSectionIterator`](#pesectioniterator) | struct | An iterator for the sections in a [`PeFile`]. |
| [`PeSection`](#pesection) | struct | A section in a [`PeFile`]. |
| [`PeRelocationIterator`](#perelocationiterator) | struct | An iterator for the relocations in an [`PeSection`]. |
| [`PeSegmentIterator32`](#pesegmentiterator32) | type | An iterator for the loadable sections in a [`PeFile32`](super::PeFile32). |
| [`PeSegmentIterator64`](#pesegmentiterator64) | type | An iterator for the loadable sections in a [`PeFile64`](super::PeFile64). |
| [`PeSegment32`](#pesegment32) | type | A loadable section in a [`PeFile32`](super::PeFile32). |
| [`PeSegment64`](#pesegment64) | type | A loadable section in a [`PeFile64`](super::PeFile64). |
| [`PeSectionIterator32`](#pesectioniterator32) | type | An iterator for the sections in a [`PeFile32`](super::PeFile32). |
| [`PeSectionIterator64`](#pesectioniterator64) | type | An iterator for the sections in a [`PeFile64`](super::PeFile64). |
| [`PeSection32`](#pesection32) | type | A section in a [`PeFile32`](super::PeFile32). |
| [`PeSection64`](#pesection64) | type | A section in a [`PeFile64`](super::PeFile64). |

## Structs

### `PeSegmentIterator<'data, 'file, Pe, R>`

```rust
struct PeSegmentIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:23-30`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L23-L30)*

An iterator for the loadable sections in a [`PeFile`](../index.md).

#### Trait Implementations

##### `impl Any for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pesegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Pe, R> Iterator for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-iterator-type-item"></span>`type Item = PeSegment<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSegmentIterator<'data, 'file, Pe, R>`

- <span id="pesegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeSegment<'data, 'file, Pe, R>`

```rust
struct PeSegment<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    section: &'data pe::ImageSectionHeader,
}
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:58-65`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L58-L65)*

A loadable section in a [`PeFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- <span id="pesegment-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](../index.md#pefile)

  Get the PE file containing this segment.

- <span id="pesegment-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Get the raw PE section header.

#### Trait Implementations

##### `impl Any for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Pe, R> ObjectSegment for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="pesegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="pesegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="pesegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="pesegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="pesegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="pesegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="pesegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="pesegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md#segmentflags)

##### `impl<Pe, R> Sealed for PeSegment<'data, 'file, Pe, R>`

##### `impl<U> TryFrom for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSegment<'data, 'file, Pe, R>`

- <span id="pesegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeSectionIterator<'data, 'file, Pe, R>`

```rust
struct PeSectionIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:162-169`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L162-L169)*

An iterator for the sections in a [`PeFile`](../index.md).

#### Trait Implementations

##### `impl Any for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pesectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pesectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Pe, R> Iterator for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-iterator-type-item"></span>`type Item = PeSection<'data, 'file, Pe, R>`

- <span id="pesectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSectionIterator<'data, 'file, Pe, R>`

- <span id="pesectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeSection<'data, 'file, Pe, R>`

```rust
struct PeSection<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:198-206`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L198-L206)*

A section in a [`PeFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- <span id="pesection-pe-file"></span>`fn pe_file(&self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](../index.md#pefile)

  Get the PE file containing this segment.

- <span id="pesection-pe-section"></span>`fn pe_section(&self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md#imagesectionheader)

  Get the raw PE section header.

#### Trait Implementations

##### `impl Any for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Pe, R> Debug for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Pe, R> ObjectSection for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-objectsection-type-relocationiterator"></span>`type RelocationIterator = PeRelocationIterator<'data, 'file, R>`

- <span id="pesection-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../../index.md#sectionindex)

- <span id="pesection-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="pesection-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="pesection-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="pesection-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="pesection-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="pesection-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="pesection-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md#result), [`CompressedFileRange`](../../../index.md#compressedfilerange)

- <span id="pesection-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md#result), [`CompressedData`](../../../index.md#compresseddata)

- <span id="pesection-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="pesection-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="pesection-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="pesection-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="pesection-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../../index.md#sectionkind)

- <span id="pesection-objectsection-relocations"></span>`fn relocations(&self) -> PeRelocationIterator<'data, 'file, R>` — [`PeRelocationIterator`](../index.md#perelocationiterator)

- <span id="pesection-objectsection-relocation-map"></span>`fn relocation_map(&self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md#result), [`RelocationMap`](../../../index.md#relocationmap)

- <span id="pesection-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../../index.md#sectionflags)

##### `impl<Pe, R> Sealed for PeSection<'data, 'file, Pe, R>`

##### `impl<U> TryFrom for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pesection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeSection<'data, 'file, Pe, R>`

- <span id="pesection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pesection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeRelocationIterator<'data, 'file, R>`

```rust
struct PeRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:466-468`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L466-L468)*

An iterator for the relocations in an [`PeSection`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug> Debug for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="perelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="perelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R> Iterator for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="perelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="perelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeRelocationIterator<'data, 'file, R>`

- <span id="perelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="perelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `PeSegmentIterator32<'data, 'file, R>`

```rust
type PeSegmentIterator32<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:15-16`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L15-L16)*

An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).

### `PeSegmentIterator64<'data, 'file, R>`

```rust
type PeSegmentIterator64<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:18-19`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L18-L19)*

An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).

### `PeSegment32<'data, 'file, R>`

```rust
type PeSegment32<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:48-49`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L48-L49)*

A loadable section in a [`PeFile32`](super::PeFile32).

### `PeSegment64<'data, 'file, R>`

```rust
type PeSegment64<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:51-52`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L51-L52)*

A loadable section in a [`PeFile64`](super::PeFile64).

### `PeSectionIterator32<'data, 'file, R>`

```rust
type PeSectionIterator32<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:154-155`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L154-L155)*

An iterator for the sections in a [`PeFile32`](super::PeFile32).

### `PeSectionIterator64<'data, 'file, R>`

```rust
type PeSectionIterator64<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:157-158`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L157-L158)*

An iterator for the sections in a [`PeFile64`](super::PeFile64).

### `PeSection32<'data, 'file, R>`

```rust
type PeSection32<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders32, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:188-189`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L188-L189)*

A section in a [`PeFile32`](super::PeFile32).

### `PeSection64<'data, 'file, R>`

```rust
type PeSection64<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders64, R>;
```

*Defined in [`object-0.37.3/src/read/pe/section.rs:191-192`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/section.rs#L191-L192)*

A section in a [`PeFile64`](super::PeFile64).

