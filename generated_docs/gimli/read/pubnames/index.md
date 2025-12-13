*[gimli](../../index.md) / [read](../index.md) / [pubnames](index.md)*

---

# Module `pubnames`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PubNamesEntry`](#pubnamesentry) | struct | A single parsed pubname. |
| [`DebugPubNames`](#debugpubnames) | struct | The `DebugPubNames` struct represents the DWARF public names information found in the `.debug_pubnames` section. |
| [`PubNamesEntryIter`](#pubnamesentryiter) | struct | An iterator over the pubnames from a `.debug_pubnames` section. |

## Structs

### `PubNamesEntry<R: Reader>`

```rust
struct PubNamesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

*Defined in [`gimli-0.32.3/src/read/pubnames.rs:8-12`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubnames.rs#L8-L12)*

A single parsed pubname.

#### Implementations

- <span id="pubnamesentry-name"></span>`fn name(&self) -> &R`

  Returns the name this entry refers to.

- <span id="pubnamesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains this name.

- <span id="pubnamesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  has this name.

#### Trait Implementations

##### `impl Any for PubNamesEntry<R>`

- <span id="pubnamesentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubNamesEntry<R>`

- <span id="pubnamesentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubNamesEntry<R>`

- <span id="pubnamesentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntry<R>`

- <span id="pubnamesentry-clone"></span>`fn clone(&self) -> PubNamesEntry<R>` — [`PubNamesEntry`](../index.md#pubnamesentry)

##### `impl CloneToUninit for PubNamesEntry<R>`

- <span id="pubnamesentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- <span id="pubnamesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubNamesEntry<R>`

- <span id="pubnamesentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubNamesEntry<R>`

- <span id="pubnamesentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- <span id="pubnamesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`DebugInfoOffset`](../../index.md#debuginfooffset)

##### `impl ToOwned for PubNamesEntry<R>`

- <span id="pubnamesentry-toowned-type-owned"></span>`type Owned = T`

- <span id="pubnamesentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubnamesentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubNamesEntry<R>`

- <span id="pubnamesentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubnamesentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubNamesEntry<R>`

- <span id="pubnamesentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubnamesentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubnames.rs:50`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubnames.rs#L50)*

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- <span id="debugpubnames-new"></span>`fn new(debug_pubnames_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubNames` instance from the data in the `.debug_pubnames`

  section.

  

  It is the caller's responsibility to read the `.debug_pubnames` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubNames, LittleEndian};

  

  let buf = [];

  let read_debug_pubnames_section_somehow = || &buf;

  let debug_pubnames =

      DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugPubNames<R>`

- <span id="debugpubnames-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugPubNames<R>`

- <span id="debugpubnames-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugPubNames<R>`

- <span id="debugpubnames-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugPubNames<R>`

- <span id="debugpubnames-clone"></span>`fn clone(&self) -> DebugPubNames<R>` — [`DebugPubNames`](../index.md#debugpubnames)

##### `impl CloneToUninit for DebugPubNames<R>`

- <span id="debugpubnames-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubNames<R>`

- <span id="debugpubnames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugPubNames<R>`

- <span id="debugpubnames-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugPubNames<R>`

- <span id="debugpubnames-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> Section for DebugPubNames<R>`

- <span id="debugpubnames-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugpubnames-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugPubNames<R>`

- <span id="debugpubnames-toowned-type-owned"></span>`type Owned = T`

- <span id="debugpubnames-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugpubnames-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugPubNames<R>`

- <span id="debugpubnames-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugpubnames-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugPubNames<R>`

- <span id="debugpubnames-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugpubnames-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubnames.rs:118`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubnames.rs#L118)*

An iterator over the pubnames from a `.debug_pubnames` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="pubnamesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../../index.md#result), [`PubNamesEntry`](../index.md#pubnamesentry)

  Advance the iterator and return the next pubname.

  

  Returns the newly parsed pubname as `Ok(Some(pubname))`. Returns

  `Ok(None)` when iteration is complete and all pubnames have already been

  parsed and yielded. If an error occurs while parsing the next pubname,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl Any for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clone"></span>`fn clone(&self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](../index.md#pubnamesentryiter)

##### `impl CloneToUninit for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="pubnamesentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubnamesentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubnamesentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubnamesentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

