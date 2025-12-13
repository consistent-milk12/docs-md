*[gimli](../../index.md) / [read](../index.md) / [aranges](index.md)*

---

# Module `aranges`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAranges`](#debugaranges) | struct | The `DebugAranges` struct represents the DWARF address range information found in the `.debug_aranges` section. |
| [`ArangeHeaderIter`](#arangeheaderiter) | struct | An iterator over the headers of a `.debug_aranges` section. |
| [`ArangeHeader`](#arangeheader) | struct | A header for a set of entries in the `.debug_arange` section. |
| [`ArangeEntryIter`](#arangeentryiter) | struct | An iterator over the aranges from a `.debug_aranges` section. |
| [`ArangeEntry`](#arangeentry) | struct | A single parsed arange. |

## Structs

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:10-12`](../../../../.source_1765521767/gimli-0.32.3/src/read/aranges.rs#L10-L12)*

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- <span id="debugaranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugAranges` instance from the data in the `.debug_aranges`

  section.

  

  It is the caller's responsibility to read the `.debug_aranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugAranges, LittleEndian};

  

  let buf = [];

  let read_debug_aranges_section = || &buf;

  let debug_aranges =

      DebugAranges::new(read_debug_aranges_section(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugAranges<R>`

- <span id="debugaranges-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAranges<R>`

- <span id="debugaranges-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAranges<R>`

- <span id="debugaranges-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugAranges<R>`

- <span id="debugaranges-clone"></span>`fn clone(&self) -> DebugAranges<R>` — [`DebugAranges`](../index.md#debugaranges)

##### `impl CloneToUninit for DebugAranges<R>`

- <span id="debugaranges-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: fmt::Debug> Debug for DebugAranges<R>`

- <span id="debugaranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAranges<R>`

- <span id="debugaranges-default"></span>`fn default() -> DebugAranges<R>` — [`DebugAranges`](../index.md#debugaranges)

##### `impl<T> From for DebugAranges<R>`

- <span id="debugaranges-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugAranges<R>`

- <span id="debugaranges-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugAranges<R>`

- <span id="debugaranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugaranges-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugAranges<R>`

- <span id="debugaranges-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaranges-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaranges-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugAranges<R>`

- <span id="debugaranges-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaranges-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugAranges<R>`

- <span id="debugaranges-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaranges-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:91-94`](../../../../.source_1765521767/gimli-0.32.3/src/read/aranges.rs#L91-L94)*

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- <span id="arangeheaderiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../../index.md#result), [`ArangeHeader`](../index.md#arangeheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl Any for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clone"></span>`fn clone(&self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](../index.md#arangeheaderiter)

##### `impl CloneToUninit for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeheaderiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeheaderiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeheaderiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeheaderiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeHeader<R, Offset>`

```rust
struct ArangeHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugArangesOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    debug_info_offset: crate::common::DebugInfoOffset<Offset>,
    entries: R,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:131-141`](../../../../.source_1765521767/gimli-0.32.3/src/read/aranges.rs#L131-L141)*

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- <span id="arangeheader-parse"></span>`fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../../index.md#debugarangesoffset), [`Result`](../../index.md#result)

- <span id="arangeheader-offset"></span>`fn offset(&self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../../index.md#debugarangesoffset)

  Return the offset of this header within the `.debug_aranges` section.

- <span id="arangeheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="arangeheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="arangeheader-debug-info-offset"></span>`fn debug_info_offset(&self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset)

  Return the offset into the .debug_info section for this set of arange entries.

- <span id="arangeheader-entries"></span>`fn entries(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md#arangeentryiter)

  Return the arange entries in this set.

#### Trait Implementations

##### `impl Any for ArangeHeader<R, Offset>`

- <span id="arangeheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeHeader<R, Offset>`

- <span id="arangeheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeHeader<R, Offset>`

- <span id="arangeheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- <span id="arangeheader-clone"></span>`fn clone(&self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](../index.md#arangeheader)

##### `impl CloneToUninit for ArangeHeader<R, Offset>`

- <span id="arangeheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- <span id="arangeheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<T> From for ArangeHeader<R, Offset>`

- <span id="arangeheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeHeader<R, Offset>`

- <span id="arangeheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- <span id="arangeheader-partialeq-eq"></span>`fn eq(&self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](../index.md#arangeheader)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

##### `impl ToOwned for ArangeHeader<R, Offset>`

- <span id="arangeheader-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeHeader<R, Offset>`

- <span id="arangeheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeHeader<R, Offset>`

- <span id="arangeheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:239-242`](../../../../.source_1765521767/gimli-0.32.3/src/read/aranges.rs#L239-L242)*

An iterator over the aranges from a `.debug_aranges` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="arangeentryiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md#result), [`ArangeEntry`](../index.md#arangeentry)

  Advance the iterator and return the next arange.

  

  Returns the newly parsed arange as `Ok(Some(arange))`. Returns `Ok(None)`

  when iteration is complete and all aranges have already been parsed and

  yielded. If an error occurs while parsing the next arange, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

- <span id="arangeentryiter-next-raw"></span>`fn next_raw(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md#result), [`ArangeEntry`](../index.md#arangeentry)

  Advance the iterator and return the next arange without validating it.

  

  The returned entry will have `range.end` set to 0.

  This will return tombstone entries as well.

#### Trait Implementations

##### `impl Any for ArangeEntryIter<R>`

- <span id="arangeentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeEntryIter<R>`

- <span id="arangeentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeEntryIter<R>`

- <span id="arangeentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- <span id="arangeentryiter-clone"></span>`fn clone(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md#arangeentryiter)

##### `impl CloneToUninit for ArangeEntryIter<R>`

- <span id="arangeentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- <span id="arangeentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArangeEntryIter<R>`

- <span id="arangeentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeEntryIter<R>`

- <span id="arangeentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArangeEntryIter<R>`

- <span id="arangeentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeEntryIter<R>`

- <span id="arangeentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeEntryIter<R>`

- <span id="arangeentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:318-321`](../../../../.source_1765521767/gimli-0.32.3/src/read/aranges.rs#L318-L321)*

A single parsed arange.

#### Implementations

- <span id="arangeentry-parse"></span>`fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`Result`](../../index.md#result)

  Parse a single arange. Return `None` for the null arange, `Some` for an actual arange.

- <span id="arangeentry-address"></span>`fn address(&self) -> u64`

  Return the beginning address of this arange.

- <span id="arangeentry-length"></span>`fn length(&self) -> u64`

  Return the length of this arange.

- <span id="arangeentry-range"></span>`fn range(&self) -> Range` — [`Range`](../index.md#range)

  Return the range.

#### Trait Implementations

##### `impl Any for ArangeEntry`

- <span id="arangeentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeEntry`

- <span id="arangeentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeEntry`

- <span id="arangeentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArangeEntry`

- <span id="arangeentry-clone"></span>`fn clone(&self) -> ArangeEntry` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl CloneToUninit for ArangeEntry`

- <span id="arangeentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArangeEntry`

- <span id="arangeentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl<T> From for ArangeEntry`

- <span id="arangeentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeEntry`

- <span id="arangeentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ArangeEntry`

- <span id="arangeentry-ord-cmp"></span>`fn cmp(&self, other: &ArangeEntry) -> cmp::Ordering` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl PartialEq for ArangeEntry`

- <span id="arangeentry-partialeq-eq"></span>`fn eq(&self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl PartialOrd for ArangeEntry`

- <span id="arangeentry-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArangeEntry) -> option::Option<cmp::Ordering>` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl StructuralPartialEq for ArangeEntry`

##### `impl ToOwned for ArangeEntry`

- <span id="arangeentry-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeEntry`

- <span id="arangeentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeEntry`

- <span id="arangeentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

