*[object](../../index.md) / [read](../index.md) / [any](index.md)*

---

# Module `any`

## Contents

- [Structs](#structs)
  - [`SegmentIterator`](#segmentiterator)
  - [`Segment`](#segment)
  - [`SectionIterator`](#sectioniterator)
  - [`Section`](#section)
  - [`ComdatIterator`](#comdatiterator)
  - [`Comdat`](#comdat)
  - [`ComdatSectionIterator`](#comdatsectioniterator)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`Symbol`](#symbol)
  - [`DynamicRelocationIterator`](#dynamicrelocationiterator)
  - [`SectionRelocationIterator`](#sectionrelocationiterator)
- [Enums](#enums)
  - [`File`](#file)
  - [`SegmentIteratorInternal`](#segmentiteratorinternal)
  - [`SegmentInternal`](#segmentinternal)
  - [`SectionIteratorInternal`](#sectioniteratorinternal)
  - [`SectionInternal`](#sectioninternal)
  - [`ComdatIteratorInternal`](#comdatiteratorinternal)
  - [`ComdatInternal`](#comdatinternal)
  - [`ComdatSectionIteratorInternal`](#comdatsectioniteratorinternal)
  - [`SymbolTableInternal`](#symboltableinternal)
  - [`SymbolIteratorInternal`](#symboliteratorinternal)
  - [`SymbolInternal`](#symbolinternal)
  - [`DynamicRelocationIteratorInternal`](#dynamicrelocationiteratorinternal)
  - [`SectionRelocationIteratorInternal`](#sectionrelocationiteratorinternal)
- [Macros](#macros)
  - [`with_inner!`](#with-inner)
  - [`with_inner_mut!`](#with-inner-mut)
  - [`map_inner!`](#map-inner)
  - [`map_inner_option!`](#map-inner-option)
  - [`map_inner_option_mut!`](#map-inner-option-mut)
  - [`next_inner!`](#next-inner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SegmentIterator`](#segmentiterator) | struct | An iterator for the loadable segments in a [`File`]. |
| [`Segment`](#segment) | struct | A loadable segment in a [`File`]. |
| [`SectionIterator`](#sectioniterator) | struct | An iterator for the sections in a [`File`]. |
| [`Section`](#section) | struct | A section in a [`File`]. |
| [`ComdatIterator`](#comdatiterator) | struct | An iterator for the COMDAT section groups in a [`File`]. |
| [`Comdat`](#comdat) | struct | A COMDAT section group in a [`File`]. |
| [`ComdatSectionIterator`](#comdatsectioniterator) | struct | An iterator for the sections in a [`Comdat`]. |
| [`SymbolTable`](#symboltable) | struct | A symbol table in a [`File`]. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for the symbols in a [`SymbolTable`]. |
| [`Symbol`](#symbol) | struct | An symbol in a [`SymbolTable`]. |
| [`DynamicRelocationIterator`](#dynamicrelocationiterator) | struct | An iterator for the dynamic relocation entries in a [`File`]. |
| [`SectionRelocationIterator`](#sectionrelocationiterator) | struct | An iterator for the relocation entries in a [`Section`]. |
| [`File`](#file) | enum | An object file that can be any supported file format. |
| [`SegmentIteratorInternal`](#segmentiteratorinternal) | enum |  |
| [`SegmentInternal`](#segmentinternal) | enum |  |
| [`SectionIteratorInternal`](#sectioniteratorinternal) | enum |  |
| [`SectionInternal`](#sectioninternal) | enum |  |
| [`ComdatIteratorInternal`](#comdatiteratorinternal) | enum |  |
| [`ComdatInternal`](#comdatinternal) | enum |  |
| [`ComdatSectionIteratorInternal`](#comdatsectioniteratorinternal) | enum |  |
| [`SymbolTableInternal`](#symboltableinternal) | enum |  |
| [`SymbolIteratorInternal`](#symboliteratorinternal) | enum |  |
| [`SymbolInternal`](#symbolinternal) | enum |  |
| [`DynamicRelocationIteratorInternal`](#dynamicrelocationiteratorinternal) | enum |  |
| [`SectionRelocationIteratorInternal`](#sectionrelocationiteratorinternal) | enum |  |
| [`with_inner!`](#with-inner) | macro | Evaluate an expression on the contents of a file format enum. |
| [`with_inner_mut!`](#with-inner-mut) | macro |  |
| [`map_inner!`](#map-inner) | macro | Like `with_inner!`, but wraps the result in another enum. |
| [`map_inner_option!`](#map-inner-option) | macro | Like `map_inner!`, but the result is a Result or Option. |
| [`map_inner_option_mut!`](#map-inner-option-mut) | macro |  |
| [`next_inner!`](#next-inner) | macro | Call `next` for a file format iterator. |

## Structs

### `SegmentIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SegmentIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:532-534`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L532-L534)*

An iterator for the loadable segments in a [`File`](../index.md).

#### Trait Implementations

##### `impl Any for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="segmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="segmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-iterator-type-item"></span>`type Item = Segment<'data, 'file, R>`

- <span id="segmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentIterator<'data, 'file, R>`

- <span id="segmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Segment<'data, 'file, R: ReadRef<'data>>`

```rust
struct Segment<'data, 'file, R: ReadRef<'data>> {
    inner: SegmentInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:574-576`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L574-L576)*

A loadable segment in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../index.md) trait implementation.

#### Trait Implementations

##### `impl Any for Segment<'data, 'file, R>`

- <span id="segment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Segment<'data, 'file, R>`

- <span id="segment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Segment<'data, 'file, R>`

- <span id="segment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Segment<'data, 'file, R>`

- <span id="segment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Segment<'data, 'file, R>`

- <span id="segment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Segment<'data, 'file, R>`

- <span id="segment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSegment for Segment<'data, 'file, R>`

- <span id="segment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="segment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="segment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="segment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="segment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="segment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../index.md#segmentflags)

##### `impl<R: ReadRef<'data>> Sealed for Segment<'data, 'file, R>`

##### `impl<U> TryFrom for Segment<'data, 'file, R>`

- <span id="segment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Segment<'data, 'file, R>`

- <span id="segment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:665-667`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L665-L667)*

An iterator for the sections in a [`File`](../index.md).

#### Trait Implementations

##### `impl Any for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-iterator-type-item"></span>`type Item = Section<'data, 'file, R>`

- <span id="sectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionIterator<'data, 'file, R>`

- <span id="sectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Section<'data, 'file, R: ReadRef<'data>>`

```rust
struct Section<'data, 'file, R: ReadRef<'data>> {
    inner: SectionInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:708-710`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L708-L710)*

A section in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSection`](../index.md) trait implementation.

#### Trait Implementations

##### `impl Any for Section<'data, 'file, R>`

- <span id="section-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Section<'data, 'file, R>`

- <span id="section-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Section<'data, 'file, R>`

- <span id="section-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Section<'data, 'file, R>`

- <span id="section-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Section<'data, 'file, R>`

- <span id="section-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Section<'data, 'file, R>`

- <span id="section-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSection for Section<'data, 'file, R>`

- <span id="section-objectsection-type-relocationiterator"></span>`type RelocationIterator = SectionRelocationIterator<'data, 'file, R>`

- <span id="section-objectsection-index"></span>`fn index(&self) -> SectionIndex` — [`SectionIndex`](../../index.md#sectionindex)

- <span id="section-objectsection-address"></span>`fn address(&self) -> u64`

- <span id="section-objectsection-size"></span>`fn size(&self) -> u64`

- <span id="section-objectsection-align"></span>`fn align(&self) -> u64`

- <span id="section-objectsection-file-range"></span>`fn file_range(&self) -> Option<(u64, u64)>`

- <span id="section-objectsection-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-compressed-file-range"></span>`fn compressed_file_range(&self) -> Result<CompressedFileRange>` — [`Result`](../../index.md#result), [`CompressedFileRange`](../../index.md#compressedfilerange)

- <span id="section-objectsection-compressed-data"></span>`fn compressed_data(&self) -> Result<CompressedData<'data>>` — [`Result`](../../index.md#result), [`CompressedData`](../../index.md#compresseddata)

- <span id="section-objectsection-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-segment-name-bytes"></span>`fn segment_name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-segment-name"></span>`fn segment_name(&self) -> Result<Option<&str>>` — [`Result`](../../index.md#result)

- <span id="section-objectsection-kind"></span>`fn kind(&self) -> SectionKind` — [`SectionKind`](../../index.md#sectionkind)

- <span id="section-objectsection-relocations"></span>`fn relocations(&self) -> SectionRelocationIterator<'data, 'file, R>` — [`SectionRelocationIterator`](../index.md#sectionrelocationiterator)

- <span id="section-objectsection-relocation-map"></span>`fn relocation_map(&self) -> Result<RelocationMap>` — [`Result`](../../index.md#result), [`RelocationMap`](../../index.md#relocationmap)

- <span id="section-objectsection-flags"></span>`fn flags(&self) -> SectionFlags` — [`SectionFlags`](../../index.md#sectionflags)

##### `impl<R: ReadRef<'data>> Sealed for Section<'data, 'file, R>`

##### `impl<U> TryFrom for Section<'data, 'file, R>`

- <span id="section-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="section-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Section<'data, 'file, R>`

- <span id="section-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="section-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:843-845`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L843-L845)*

An iterator for the COMDAT section groups in a [`File`](../index.md).

#### Trait Implementations

##### `impl Any for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-iterator-type-item"></span>`type Item = Comdat<'data, 'file, R>`

- <span id="comdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatIterator<'data, 'file, R>`

- <span id="comdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Comdat<'data, 'file, R: ReadRef<'data>>`

```rust
struct Comdat<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:885-887`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L885-L887)*

A COMDAT section group in a [`File`](../index.md).

Most functionality is provided by the [`ObjectComdat`](../index.md) trait implementation.

#### Trait Implementations

##### `impl Any for Comdat<'data, 'file, R>`

- <span id="comdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Comdat<'data, 'file, R>`

- <span id="comdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Comdat<'data, 'file, R>`

- <span id="comdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Comdat<'data, 'file, R>`

- <span id="comdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Comdat<'data, 'file, R>`

- <span id="comdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Comdat<'data, 'file, R>`

- <span id="comdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectComdat for Comdat<'data, 'file, R>`

- <span id="comdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = ComdatSectionIterator<'data, 'file, R>`

- <span id="comdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../index.md#comdatkind)

- <span id="comdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="comdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="comdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="comdat-objectcomdat-sections"></span>`fn sections(&self) -> ComdatSectionIterator<'data, 'file, R>` — [`ComdatSectionIterator`](../index.md#comdatsectioniterator)

##### `impl<R: ReadRef<'data>> Sealed for Comdat<'data, 'file, R>`

##### `impl<U> TryFrom for Comdat<'data, 'file, R>`

- <span id="comdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Comdat<'data, 'file, R>`

- <span id="comdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatSectionIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct ComdatSectionIterator<'data, 'file, R: ReadRef<'data>> {
    inner: ComdatSectionIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:959-961`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L959-L961)*

An iterator for the sections in a [`Comdat`](../index.md).

#### Trait Implementations

##### `impl Any for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="comdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="comdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="comdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatSectionIterator<'data, 'file, R>`

- <span id="comdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolTable<'data, 'file, R>`

```rust
struct SymbolTable<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolTableInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1001-1006`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1001-L1006)*

A symbol table in a [`File`](../index.md).

Most functionality is provided by the [`ObjectSymbolTable`](../index.md) trait implementation.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, 'file, R>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, 'file, R>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, 'file, R>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolTable<'data, 'file, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolTable<'data, 'file, R>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, 'file, R>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSymbolTable for SymbolTable<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="symboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../index.md#objectsymboltable)

- <span id="symboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`ObjectSymbolTable`](../index.md#objectsymboltable)

##### `impl<R: ReadRef<'data>> Sealed for SymbolTable<'data, 'file, R>`

##### `impl<U> TryFrom for SymbolTable<'data, 'file, R>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, 'file, R>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolIterator<'data, 'file, R>`

```rust
struct SymbolIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1085-1090`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1085-L1090)*

An iterator for the symbols in a [`SymbolTable`](../index.md).

#### Trait Implementations

##### `impl Any for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = Symbol<'data, 'file, R>`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIterator<'data, 'file, R>`

- <span id="symboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Symbol<'data, 'file, R>`

```rust
struct Symbol<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: SymbolInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1165-1170`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1165-L1170)*

An symbol in a [`SymbolTable`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../index.md) trait implementation.

#### Trait Implementations

##### `impl Any for Symbol<'data, 'file, R>`

- <span id="symbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Symbol<'data, 'file, R>`

- <span id="symbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Symbol<'data, 'file, R>`

- <span id="symbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>> Debug for Symbol<'data, 'file, R>`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Symbol<'data, 'file, R>`

- <span id="symbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Symbol<'data, 'file, R>`

- <span id="symbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>> ObjectSymbol for Symbol<'data, 'file, R>`

- <span id="symbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../index.md#symbolindex)

- <span id="symbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../index.md#result)

- <span id="symbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../index.md#result)

- <span id="symbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="symbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="symbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../index.md#symbolkind)

- <span id="symbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../index.md#symbolsection)

- <span id="symbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="symbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="symbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="symbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="symbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../index.md#symbolscope)

- <span id="symbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="symbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="symbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../index.md#symbolflags), [`SectionIndex`](../../index.md#sectionindex), [`SymbolIndex`](../../index.md#symbolindex)

##### `impl<R: ReadRef<'data>> Sealed for Symbol<'data, 'file, R>`

##### `impl<U> TryFrom for Symbol<'data, 'file, R>`

- <span id="symbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Symbol<'data, 'file, R>`

- <span id="symbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DynamicRelocationIterator<'data, 'file, R>`

```rust
struct DynamicRelocationIterator<'data, 'file, R>
where
    R: ReadRef<'data> {
    inner: DynamicRelocationIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1301-1306`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1301-L1306)*

An iterator for the dynamic relocation entries in a [`File`](../index.md).

#### Trait Implementations

##### `impl Any for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="dynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dynamicrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynamicRelocationIterator<'data, 'file, R>`

- <span id="dynamicrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dynamicrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionRelocationIterator<'data, 'file, R: ReadRef<'data>>`

```rust
struct SectionRelocationIterator<'data, 'file, R: ReadRef<'data>> {
    inner: SectionRelocationIteratorInternal<'data, 'file, R>,
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1338-1340`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1338-L1340)*

An iterator for the relocation entries in a [`Section`](../index.md).

#### Trait Implementations

##### `impl Any for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sectionrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sectionrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="sectionrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionRelocationIterator<'data, 'file, R>`

- <span id="sectionrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `File<'data, R: ReadRef<'data>>`

```rust
enum File<'data, R: ReadRef<'data>> {
    Coff(coff::CoffFile<'data, R>),
    CoffBig(coff::CoffBigFile<'data, R>),
    Elf32(elf::ElfFile32<'data, crate::endian::Endianness, R>),
    Elf64(elf::ElfFile64<'data, crate::endian::Endianness, R>),
    MachO32(macho::MachOFile32<'data, crate::endian::Endianness, R>),
    MachO64(macho::MachOFile64<'data, crate::endian::Endianness, R>),
    Pe32(pe::PeFile32<'data, R>),
    Pe64(pe::PeFile64<'data, R>),
    Xcoff32(xcoff::XcoffFile32<'data, R>),
    Xcoff64(xcoff::XcoffFile64<'data, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:213-236`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L213-L236)*

An object file that can be any supported file format.

Most functionality is provided by the [`Object`](../index.md) trait implementation.

#### Implementations

- <span id="file-parse"></span>`fn parse(data: R) -> Result<Self>` — [`Result`](../../index.md#result)

  Parse the raw file data.

- <span id="file-parse-dyld-cache-image"></span>`fn parse_dyld_cache_image<'cache, E: crate::Endian>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` — [`DyldCacheImage`](../macho/index.md#dyldcacheimage), [`Result`](../../index.md#result)

  Parse a Mach-O image from the dyld shared cache.

- <span id="file-format"></span>`fn format(&self) -> BinaryFormat` — [`BinaryFormat`](../../index.md#binaryformat)

  Return the file format.

#### Trait Implementations

##### `impl Any for File<'data, R>`

- <span id="file-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for File<'data, R>`

- <span id="file-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for File<'data, R>`

- <span id="file-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for File<'data, R>`

- <span id="file-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for File<'data, R>`

- <span id="file-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for File<'data, R>`

- <span id="file-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Object for File<'data, R>`

- <span id="file-object-type-segment"></span>`type Segment = Segment<'data, 'file, R>`

- <span id="file-object-type-segmentiterator"></span>`type SegmentIterator = SegmentIterator<'data, 'file, R>`

- <span id="file-object-type-section"></span>`type Section = Section<'data, 'file, R>`

- <span id="file-object-type-sectioniterator"></span>`type SectionIterator = SectionIterator<'data, 'file, R>`

- <span id="file-object-type-comdat"></span>`type Comdat = Comdat<'data, 'file, R>`

- <span id="file-object-type-comdatiterator"></span>`type ComdatIterator = ComdatIterator<'data, 'file, R>`

- <span id="file-object-type-symbol"></span>`type Symbol = Symbol<'data, 'file, R>`

- <span id="file-object-type-symboliterator"></span>`type SymbolIterator = SymbolIterator<'data, 'file, R>`

- <span id="file-object-type-symboltable"></span>`type SymbolTable = SymbolTable<'data, 'file, R>`

- <span id="file-object-type-dynamicrelocationiterator"></span>`type DynamicRelocationIterator = DynamicRelocationIterator<'data, 'file, R>`

- <span id="file-object-architecture"></span>`fn architecture(&self) -> Architecture` — [`Architecture`](../../index.md#architecture)

- <span id="file-object-sub-architecture"></span>`fn sub_architecture(&self) -> Option<SubArchitecture>` — [`SubArchitecture`](../../index.md#subarchitecture)

- <span id="file-object-is-little-endian"></span>`fn is_little_endian(&self) -> bool`

- <span id="file-object-is-64"></span>`fn is_64(&self) -> bool`

- <span id="file-object-kind"></span>`fn kind(&self) -> ObjectKind` — [`ObjectKind`](../../index.md#objectkind)

- <span id="file-object-segments"></span>`fn segments(&self) -> SegmentIterator<'data, '_, R>` — [`SegmentIterator`](../index.md#segmentiterator)

- <span id="file-object-section-by-name-bytes"></span>`fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>` — [`Section`](../index.md#section)

- <span id="file-object-section-by-index"></span>`fn section_by_index(&self, index: SectionIndex) -> Result<Section<'data, '_, R>>` — [`SectionIndex`](../../index.md#sectionindex), [`Result`](../../index.md#result), [`Section`](../index.md#section)

- <span id="file-object-sections"></span>`fn sections(&self) -> SectionIterator<'data, '_, R>` — [`SectionIterator`](../index.md#sectioniterator)

- <span id="file-object-comdats"></span>`fn comdats(&self) -> ComdatIterator<'data, '_, R>` — [`ComdatIterator`](../index.md#comdatiterator)

- <span id="file-object-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<Symbol<'data, '_, R>>` — [`SymbolIndex`](../../index.md#symbolindex), [`Result`](../../index.md#result), [`Symbol`](../index.md#symbol)

- <span id="file-object-symbols"></span>`fn symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](../index.md#symboliterator)

- <span id="file-object-symbol-table"></span>`fn symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](../index.md#symboltable)

- <span id="file-object-dynamic-symbols"></span>`fn dynamic_symbols(&self) -> SymbolIterator<'data, '_, R>` — [`SymbolIterator`](../index.md#symboliterator)

- <span id="file-object-dynamic-symbol-table"></span>`fn dynamic_symbol_table(&self) -> Option<SymbolTable<'data, '_, R>>` — [`SymbolTable`](../index.md#symboltable)

- <span id="file-object-dynamic-relocations"></span>`fn dynamic_relocations(&self) -> Option<DynamicRelocationIterator<'data, '_, R>>` — [`DynamicRelocationIterator`](../index.md#dynamicrelocationiterator)

- <span id="file-object-symbol-map"></span>`fn symbol_map(&self) -> SymbolMap<SymbolMapName<'data>>` — [`SymbolMap`](../../index.md#symbolmap), [`SymbolMapName`](../../index.md#symbolmapname)

- <span id="file-object-object-map"></span>`fn object_map(&self) -> ObjectMap<'data>` — [`ObjectMap`](../../index.md#objectmap)

- <span id="file-object-imports"></span>`fn imports(&self) -> Result<Vec<Import<'data>>>` — [`Result`](../../index.md#result), [`Import`](../../index.md#import)

- <span id="file-object-exports"></span>`fn exports(&self) -> Result<Vec<Export<'data>>>` — [`Result`](../../index.md#result), [`Export`](../../index.md#export)

- <span id="file-object-has-debug-symbols"></span>`fn has_debug_symbols(&self) -> bool`

- <span id="file-object-mach-uuid"></span>`fn mach_uuid(&self) -> Result<Option<[u8; 16]>>` — [`Result`](../../index.md#result)

- <span id="file-object-build-id"></span>`fn build_id(&self) -> Result<Option<&'data [u8]>>` — [`Result`](../../index.md#result)

- <span id="file-object-gnu-debuglink"></span>`fn gnu_debuglink(&self) -> Result<Option<(&'data [u8], u32)>>` — [`Result`](../../index.md#result)

- <span id="file-object-gnu-debugaltlink"></span>`fn gnu_debugaltlink(&self) -> Result<Option<(&'data [u8], &'data [u8])>>` — [`Result`](../../index.md#result)

- <span id="file-object-pdb-info"></span>`fn pdb_info(&self) -> Result<Option<CodeView<'_>>>` — [`Result`](../../index.md#result), [`CodeView`](../../index.md#codeview)

- <span id="file-object-relative-address-base"></span>`fn relative_address_base(&self) -> u64`

- <span id="file-object-entry"></span>`fn entry(&self) -> u64`

- <span id="file-object-flags"></span>`fn flags(&self) -> FileFlags` — [`FileFlags`](../../index.md#fileflags)

##### `impl<R: ReadRef<'data>> Sealed for File<'data, R>`

##### `impl<U> TryFrom for File<'data, R>`

- <span id="file-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for File<'data, R>`

- <span id="file-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegmentIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSegmentIterator<'data, 'file, R>),
    Elf32(elf::ElfSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegmentIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegmentIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegmentIterator32<'data, 'file, R>),
    Pe64(pe::PeSegmentIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegmentIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegmentIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:537-560`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L537-L560)*

#### Trait Implementations

##### `impl Any for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentIteratorInternal<'data, 'file, R>`

- <span id="segmentiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SegmentInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSegment<'data, 'file, R>),
    CoffBig(coff::CoffBigSegment<'data, 'file, R>),
    Elf32(elf::ElfSegment32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSegment64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSegment32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSegment64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSegment32<'data, 'file, R>),
    Pe64(pe::PeSegment64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSegment32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSegment64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:579-602`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L579-L602)*

#### Trait Implementations

##### `impl Any for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentInternal<'data, 'file, R>`

- <span id="segmentinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeSectionIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSectionIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:671-694`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L671-L694)*

#### Trait Implementations

##### `impl Any for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectioniteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionIteratorInternal<'data, 'file, R>`

- <span id="sectioniteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectioniteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffSection<'data, 'file, R>),
    CoffBig(coff::CoffBigSection<'data, 'file, R>),
    Elf32(elf::ElfSection32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSection64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOSection32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOSection64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeSection32<'data, 'file, R>),
    Pe64(pe::PeSection64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffSection32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffSection64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:712-735`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L712-L735)*

#### Trait Implementations

##### `impl Any for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectioninternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionInternal<'data, 'file, R>`

- <span id="sectioninternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectioninternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:848-871`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L848-L871)*

#### Trait Implementations

##### `impl Any for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatIteratorInternal<'data, 'file, R>`

- <span id="comdatiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdat<'data, 'file, R>),
    CoffBig(coff::CoffBigComdat<'data, 'file, R>),
    Elf32(elf::ElfComdat32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdat64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdat32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdat64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdat32<'data, 'file, R>),
    Pe64(pe::PeComdat64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdat32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdat64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:889-912`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L889-L912)*

#### Trait Implementations

##### `impl Any for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatInternal<'data, 'file, R>`

- <span id="comdatinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum ComdatSectionIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffComdatSectionIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigComdatSectionIterator<'data, 'file, R>),
    Elf32(elf::ElfComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachOComdatSectionIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachOComdatSectionIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeComdatSectionIterator32<'data, 'file, R>),
    Pe64(pe::PeComdatSectionIterator64<'data, 'file, R>),
    Xcoff32(xcoff::XcoffComdatSectionIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffComdatSectionIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:964-987`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L964-L987)*

#### Trait Implementations

##### `impl Any for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatsectioniteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatSectionIteratorInternal<'data, 'file, R>`

- <span id="comdatsectioniteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatsectioniteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolTableInternal<'data, 'file, R>`

```rust
enum SymbolTableInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolTable32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolTable64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolTable<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolTable32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolTable64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1009-1055`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1009-L1055)*

#### Trait Implementations

##### `impl Any for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltableinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTableInternal<'data, 'file, R>`

- <span id="symboltableinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltableinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolIteratorInternal<'data, 'file, R>`

```rust
enum SymbolIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbolIterator32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbolIterator64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbolIterator<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbolIterator32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbolIterator64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1093-1149`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1093-L1149)*

#### Trait Implementations

##### `impl Any for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIteratorInternal<'data, 'file, R>`

- <span id="symboliteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolInternal<'data, 'file, R>`

```rust
enum SymbolInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Coff((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    CoffBig((coff::CoffBigSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Elf32((elf::ElfSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    Elf64((elf::ElfSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<R>)),
    MachO32((macho::MachOSymbol32<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    MachO64((macho::MachOSymbol64<'data, 'file, crate::endian::Endianness, R>, core::marker::PhantomData<()>)),
    Pe32((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Pe64((coff::CoffSymbol<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff32((xcoff::XcoffSymbol32<'data, 'file, R>, core::marker::PhantomData<R>)),
    Xcoff64((xcoff::XcoffSymbol64<'data, 'file, R>, core::marker::PhantomData<R>)),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1172-1218`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1172-L1218)*

#### Trait Implementations

##### `impl Any for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolInternal<'data, 'file, R>`

- <span id="symbolinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DynamicRelocationIteratorInternal<'data, 'file, R>`

```rust
enum DynamicRelocationIteratorInternal<'data, 'file, R>
where
    R: ReadRef<'data> {
    Elf32(elf::ElfDynamicRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfDynamicRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    None(core::marker::PhantomData<(&'data (), &'file (), R)>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1309-1320`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1309-L1320)*

#### Trait Implementations

##### `impl Any for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Debug for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dynamicrelocationiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynamicRelocationIteratorInternal<'data, 'file, R>`

- <span id="dynamicrelocationiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dynamicrelocationiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>>`

```rust
enum SectionRelocationIteratorInternal<'data, 'file, R: ReadRef<'data>> {
    Coff(coff::CoffRelocationIterator<'data, 'file, R>),
    CoffBig(coff::CoffBigRelocationIterator<'data, 'file, R>),
    Elf32(elf::ElfSectionRelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    Elf64(elf::ElfSectionRelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    MachO32(macho::MachORelocationIterator32<'data, 'file, crate::endian::Endianness, R>),
    MachO64(macho::MachORelocationIterator64<'data, 'file, crate::endian::Endianness, R>),
    Pe32(pe::PeRelocationIterator<'data, 'file, R>),
    Pe64(pe::PeRelocationIterator<'data, 'file, R>),
    Xcoff32(xcoff::XcoffRelocationIterator32<'data, 'file, R>),
    Xcoff64(xcoff::XcoffRelocationIterator64<'data, 'file, R>),
}
```

*Defined in [`object-0.37.3/src/read/any.rs:1343-1366`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L1343-L1366)*

#### Trait Implementations

##### `impl Any for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionrelocationiteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionRelocationIteratorInternal<'data, 'file, R>`

- <span id="sectionrelocationiteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionrelocationiteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `with_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:30-57`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L30-L57)*

Evaluate an expression on the contents of a file format enum.

This is a hack to avoid virtual calls.

### `with_inner_mut!`

*Defined in [`object-0.37.3/src/read/any.rs:59-86`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L59-L86)*

### `map_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:89-116`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L89-L116)*

Like `with_inner!`, but wraps the result in another enum.

### `map_inner_option!`

*Defined in [`object-0.37.3/src/read/any.rs:119-146`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L119-L146)*

Like `map_inner!`, but the result is a Result or Option.

### `map_inner_option_mut!`

*Defined in [`object-0.37.3/src/read/any.rs:148-175`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L148-L175)*

### `next_inner!`

*Defined in [`object-0.37.3/src/read/any.rs:178-205`](../../../../.source_1765521767/object-0.37.3/src/read/any.rs#L178-L205)*

Call `next` for a file format iterator.

