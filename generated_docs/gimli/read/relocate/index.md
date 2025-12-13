*[gimli](../../index.md) / [read](../index.md) / [relocate](index.md)*

---

# Module `relocate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RelocateReader`](#relocatereader) | struct | A `Reader` which applies relocations to addresses and offsets. |
| [`Relocate`](#relocate) | trait | Trait for relocating addresses and offsets while reading a section. |

## Structs

### `RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>>`

```rust
struct RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>> {
    section: R,
    reader: R,
    relocate: T,
}
```

*Defined in [`gimli-0.32.3/src/read/relocate.rs:23-27`](../../../../.source_1765521767/gimli-0.32.3/src/read/relocate.rs#L23-L27)*

A `Reader` which applies relocations to addresses and offsets.

This is useful for reading sections which contain relocations,
such as those in a relocatable object file.
It is generally not used for reading sections in an executable file.

#### Implementations

- <span id="relocatereader-new"></span>`fn new(section: R, relocate: T) -> Self`

  Create a new `RelocateReader` which applies relocations to the given section reader.

#### Trait Implementations

##### `impl<T> Any for RelocateReader<R, T>`

- <span id="relocatereader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocateReader<R, T>`

- <span id="relocatereader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocateReader<R, T>`

- <span id="relocatereader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader<Offset = usize>, T: clone::Clone + Relocate<<R as >::Offset>> Clone for RelocateReader<R, T>`

- <span id="relocatereader-clone"></span>`fn clone(&self) -> RelocateReader<R, T>` — [`RelocateReader`](../index.md#relocatereader)

##### `impl<T> CloneToUninit for RelocateReader<R, T>`

- <span id="relocatereader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader<Offset = usize>, T: fmt::Debug + Relocate<<R as >::Offset>> Debug for RelocateReader<R, T>`

- <span id="relocatereader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelocateReader<R, T>`

- <span id="relocatereader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RelocateReader<R, T>`

- <span id="relocatereader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, T> Reader for RelocateReader<R, T>`

- <span id="relocatereader-reader-type-endian"></span>`type Endian = <R as Reader>::Endian`

- <span id="relocatereader-reader-type-offset"></span>`type Offset = <R as Reader>::Offset`

- <span id="relocatereader-reader-read-address"></span>`fn read_address(&mut self, address_size: u8) -> Result<u64>` — [`Result`](../../index.md#result)

- <span id="relocatereader-reader-read-offset"></span>`fn read_offset(&mut self, format: Format) -> Result<<R as >::Offset>` — [`Format`](../../index.md#format), [`Result`](../../index.md#result), [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-read-sized-offset"></span>`fn read_sized_offset(&mut self, size: u8) -> Result<<R as >::Offset>` — [`Result`](../../index.md#result), [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-split"></span>`fn split(&mut self, len: <Self as >::Offset) -> Result<Self>` — [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="relocatereader-reader-endian"></span>`fn endian(&self) -> <Self as >::Endian` — [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-len"></span>`fn len(&self) -> <Self as >::Offset` — [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-empty"></span>`fn empty(&mut self)`

- <span id="relocatereader-reader-truncate"></span>`fn truncate(&mut self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="relocatereader-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> <Self as >::Offset` — [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](../index.md#readeroffsetid)

- <span id="relocatereader-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](../index.md#readeroffsetid), [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-find"></span>`fn find(&self, byte: u8) -> Result<<Self as >::Offset>` — [`Result`](../../index.md#result), [`Reader`](../index.md#reader)

- <span id="relocatereader-reader-skip"></span>`fn skip(&mut self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="relocatereader-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../../index.md#result)

- <span id="relocatereader-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="relocatereader-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../../index.md#result)

- <span id="relocatereader-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../../index.md#result)

##### `impl<T> ToOwned for RelocateReader<R, T>`

- <span id="relocatereader-toowned-type-owned"></span>`type Owned = T`

- <span id="relocatereader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocatereader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RelocateReader<R, T>`

- <span id="relocatereader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocatereader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RelocateReader<R, T>`

- <span id="relocatereader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocatereader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Relocate<T: ReaderOffset>`

```rust
trait Relocate<T: ReaderOffset> { ... }
```

*Defined in [`gimli-0.32.3/src/read/relocate.rs:9-15`](../../../../.source_1765521767/gimli-0.32.3/src/read/relocate.rs#L9-L15)*

Trait for relocating addresses and offsets while reading a section.

#### Required Methods

- `fn relocate_address(&self, offset: T, value: u64) -> Result<u64>`

  Relocate an address which was read from the given section offset.

- `fn relocate_offset(&self, offset: T, value: T) -> Result<T>`

  Relocate a value which was read from the given section offset.

