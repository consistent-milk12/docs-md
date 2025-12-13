*[gimli](../../index.md) / [read](../index.md) / [pubtypes](index.md)*

---

# Module `pubtypes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PubTypesEntry`](#pubtypesentry) | struct | A single parsed pubtype. |
| [`DebugPubTypes`](#debugpubtypes) | struct | The `DebugPubTypes` struct represents the DWARF public types information found in the `.debug_info` section. |
| [`PubTypesEntryIter`](#pubtypesentryiter) | struct | An iterator over the pubtypes from a `.debug_pubtypes` section. |

## Structs

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:8-12`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubtypes.rs#L8-L12)*

A single parsed pubtype.

#### Implementations

- <span id="pubtypesentry-name"></span>`fn name(&self) -> &R`

  Returns the name of the type this entry refers to.

- <span id="pubtypesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains the type with this name.

- <span id="pubtypesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  the type with this name.

#### Trait Implementations

##### `impl Any for PubTypesEntry<R>`

- <span id="pubtypesentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubTypesEntry<R>`

- <span id="pubtypesentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubTypesEntry<R>`

- <span id="pubtypesentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntry<R>`

- <span id="pubtypesentry-clone"></span>`fn clone(&self) -> PubTypesEntry<R>` — [`PubTypesEntry`](../index.md#pubtypesentry)

##### `impl CloneToUninit for PubTypesEntry<R>`

- <span id="pubtypesentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- <span id="pubtypesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubTypesEntry<R>`

- <span id="pubtypesentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubTypesEntry<R>`

- <span id="pubtypesentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- <span id="pubtypesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`DebugInfoOffset`](../../index.md#debuginfooffset)

##### `impl ToOwned for PubTypesEntry<R>`

- <span id="pubtypesentry-toowned-type-owned"></span>`type Owned = T`

- <span id="pubtypesentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubtypesentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubTypesEntry<R>`

- <span id="pubtypesentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubtypesentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubTypesEntry<R>`

- <span id="pubtypesentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubtypesentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:50`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubtypes.rs#L50)*

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- <span id="debugpubtypes-new"></span>`fn new(debug_pubtypes_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubTypes` instance from the data in the `.debug_pubtypes`

  section.

  

  It is the caller's responsibility to read the `.debug_pubtypes` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubTypes, LittleEndian};

  

  let buf = [];

  let read_debug_pubtypes_somehow = || &buf;

  let debug_pubtypes =

      DebugPubTypes::new(read_debug_pubtypes_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugPubTypes<R>`

- <span id="debugpubtypes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugPubTypes<R>`

- <span id="debugpubtypes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugPubTypes<R>`

- <span id="debugpubtypes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugPubTypes<R>`

- <span id="debugpubtypes-clone"></span>`fn clone(&self) -> DebugPubTypes<R>` — [`DebugPubTypes`](../index.md#debugpubtypes)

##### `impl CloneToUninit for DebugPubTypes<R>`

- <span id="debugpubtypes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- <span id="debugpubtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugPubTypes<R>`

- <span id="debugpubtypes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugPubTypes<R>`

- <span id="debugpubtypes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- <span id="debugpubtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugpubtypes-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugPubTypes<R>`

- <span id="debugpubtypes-toowned-type-owned"></span>`type Owned = T`

- <span id="debugpubtypes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugpubtypes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugPubTypes<R>`

- <span id="debugpubtypes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugpubtypes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugPubTypes<R>`

- <span id="debugpubtypes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugpubtypes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:118`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubtypes.rs#L118)*

An iterator over the pubtypes from a `.debug_pubtypes` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="pubtypesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../../index.md#result), [`PubTypesEntry`](../index.md#pubtypesentry)

  Advance the iterator and return the next pubtype.

  

  Returns the newly parsed pubtype as `Ok(Some(pubtype))`. Returns

  `Ok(None)` when iteration is complete and all pubtypes have already been

  parsed and yielded. If an error occurs while parsing the next pubtype,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl Any for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clone"></span>`fn clone(&self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](../index.md#pubtypesentryiter)

##### `impl CloneToUninit for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="pubtypesentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubtypesentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubtypesentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubtypesentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

