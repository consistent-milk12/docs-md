*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [segment](index.md)*

---

# Module `segment`

## Contents

- [Structs](#structs)
  - [`MachOSegmentIterator`](#machosegmentiterator)
  - [`MachOSegment`](#machosegment)
  - [`MachOSegmentInternal`](#machosegmentinternal)
- [Traits](#traits)
  - [`Segment`](#segment)
- [Type Aliases](#type-aliases)
  - [`MachOSegmentIterator32`](#machosegmentiterator32)
  - [`MachOSegmentIterator64`](#machosegmentiterator64)
  - [`MachOSegment32`](#machosegment32)
  - [`MachOSegment64`](#machosegment64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MachOSegmentIterator`](#machosegmentiterator) | struct | An iterator for the segments in a [`MachOFile`]. |
| [`MachOSegment`](#machosegment) | struct | A segment in a [`MachOFile`]. |
| [`MachOSegmentInternal`](#machosegmentinternal) | struct |  |
| [`Segment`](#segment) | trait | A trait for generic access to [`macho::SegmentCommand32`] and [`macho::SegmentCommand64`]. |
| [`MachOSegmentIterator32`](#machosegmentiterator32) | type | An iterator for the segments in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSegmentIterator64`](#machosegmentiterator64) | type | An iterator for the segments in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSegment32`](#machosegment32) | type | A segment in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSegment64`](#machosegment64) | type | A segment in a [`MachOFile64`](super::MachOFile64). |

## Structs

### `MachOSegmentIterator<'data, 'file, Mach, R>`

```rust
struct MachOSegmentIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    iter: slice::Iter<'file, MachOSegmentInternal<'data, Mach, R>>,
}
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:20-27`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L20-L27)*

An iterator for the segments in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl Any for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosegmentiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosegmentiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Mach, R> Iterator for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-iterator-type-item"></span>`type Item = MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosegmentiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSegmentIterator<'data, 'file, Mach, R>`

- <span id="machosegmentiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosegmentiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSegment<'data, 'file, Mach, R>`

```rust
struct MachOSegment<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    internal: &'file MachOSegmentInternal<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:55-62`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L55-L62)*

A segment in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- <span id="machosegment-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md#machofile)

  Get the Mach-O file containing this segment.

- <span id="machosegment-macho-segment"></span>`fn macho_segment(&self) -> &'data <Mach as >::Segment` — [`MachHeader`](../index.md#machheader)

  Get the raw Mach-O segment structure.

- <span id="machosegment-bytes"></span>`fn bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Any for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Mach, R> ObjectSegment for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-objectsegment-address"></span>`fn address(&self) -> u64`

- <span id="machosegment-objectsegment-size"></span>`fn size(&self) -> u64`

- <span id="machosegment-objectsegment-align"></span>`fn align(&self) -> u64`

- <span id="machosegment-objectsegment-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="machosegment-objectsegment-data"></span>`fn data(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machosegment-objectsegment-data-range"></span>`fn data_range(&self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md#result)

- <span id="machosegment-objectsegment-name-bytes"></span>`fn name_bytes(&self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md#result)

- <span id="machosegment-objectsegment-name"></span>`fn name(&self) -> Result<Option<&str>>` — [`Result`](../../../index.md#result)

- <span id="machosegment-objectsegment-flags"></span>`fn flags(&self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md#segmentflags)

##### `impl<Mach, R> Sealed for MachOSegment<'data, 'file, Mach, R>`

##### `impl<U> TryFrom for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosegment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSegment<'data, 'file, Mach, R>`

- <span id="machosegment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosegment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSegmentInternal<'data, Mach: MachHeader, R: ReadRef<'data>>`

```rust
struct MachOSegmentInternal<'data, Mach: MachHeader, R: ReadRef<'data>> {
    pub segment: &'data <Mach as >::Segment,
    pub data: R,
}
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:161-168`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L161-L168)*

#### Fields

- **`data`**: `R`

  The data for the file that contains the segment data.
  
  This is required for dyld caches, where this may be a different subcache
  from the file containing the Mach-O load commands.

#### Trait Implementations

##### `impl Any for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach: clone::Clone + MachHeader, R: clone::Clone + ReadRef<'data>> Clone for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-clone"></span>`fn clone(&self) -> MachOSegmentInternal<'data, Mach, R>` — [`MachOSegmentInternal`](#machosegmentinternal)

##### `impl CloneToUninit for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Mach: marker::Copy + MachHeader, R: marker::Copy + ReadRef<'data>> Copy for MachOSegmentInternal<'data, Mach, R>`

##### `impl<Mach: fmt::Debug + MachHeader, R: fmt::Debug + ReadRef<'data>> Debug for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-toowned-type-owned"></span>`type Owned = T`

- <span id="machosegmentinternal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="machosegmentinternal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosegmentinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSegmentInternal<'data, Mach, R>`

- <span id="machosegmentinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosegmentinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Segment`

```rust
trait Segment: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:172-229`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L172-L229)*

A trait for generic access to [`macho::SegmentCommand32`](../../../macho/index.md) and [`macho::SegmentCommand64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

- `type Section: 1`

#### Required Methods

- `fn from_command(command: LoadCommandData<'_, <Self as >::Endian>) -> Result<Option<(&Self, &[u8])>>`

- `fn cmd(&self, endian: <Self as >::Endian) -> u32`

- `fn cmdsize(&self, endian: <Self as >::Endian) -> u32`

- `fn segname(&self) -> &[u8; 16]`

- `fn vmaddr(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn vmsize(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn fileoff(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn filesize(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn maxprot(&self, endian: <Self as >::Endian) -> u32`

- `fn initprot(&self, endian: <Self as >::Endian) -> u32`

- `fn nsects(&self, endian: <Self as >::Endian) -> u32`

- `fn flags(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn name(&self) -> &[u8]`

  Return the `segname` bytes up until the null terminator.

- `fn file_range(&self, endian: <Self as >::Endian) -> (u64, u64)`

  Return the offset and size of the segment in the file.

- `fn data<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, data: R) -> result::Result<&'data [u8], ()>`

  Get the segment data from the file data.

- `fn sections<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, section_data: R) -> Result<&'data [<Self as >::Section]>`

  Get the array of sections from the data following the segment command.

#### Implementors

- [`SegmentCommand32`](../../../macho/index.md#segmentcommand32)
- [`SegmentCommand64`](../../../macho/index.md#segmentcommand64)

## Type Aliases

### `MachOSegmentIterator32<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator32<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:12-13`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L12-L13)*

An iterator for the segments in a [`MachOFile32`](super::MachOFile32).

### `MachOSegmentIterator64<'data, 'file, Endian, R>`

```rust
type MachOSegmentIterator64<'data, 'file, Endian, R> = MachOSegmentIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:15-16`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L15-L16)*

An iterator for the segments in a [`MachOFile64`](super::MachOFile64).

### `MachOSegment32<'data, 'file, Endian, R>`

```rust
type MachOSegment32<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:45-46`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L45-L46)*

A segment in a [`MachOFile32`](super::MachOFile32).

### `MachOSegment64<'data, 'file, Endian, R>`

```rust
type MachOSegment64<'data, 'file, Endian, R> = MachOSegment<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/segment.rs:48-49`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/segment.rs#L48-L49)*

A segment in a [`MachOFile64`](super::MachOFile64).

