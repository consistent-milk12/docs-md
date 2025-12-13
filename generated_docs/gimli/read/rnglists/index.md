*[gimli](../../index.md) / [read](../index.md) / [rnglists](index.md)*

---

# Module `rnglists`

## Contents

- [Structs](#structs)
  - [`DebugRanges`](#debugranges)
  - [`DebugRngLists`](#debugrnglists)
  - [`RangeLists`](#rangelists)
  - [`RawRngListIter`](#rawrnglistiter)
  - [`RngListIter`](#rnglistiter)
  - [`RawRange`](#rawrange)
  - [`Range`](#range)
- [Enums](#enums)
  - [`RangeListsFormat`](#rangelistsformat)
  - [`RawRngListEntry`](#rawrnglistentry)
- [Type Aliases](#type-aliases)
  - [`RngListsHeader`](#rnglistsheader)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugRanges`](#debugranges) | struct | The raw contents of the `.debug_ranges` section. |
| [`DebugRngLists`](#debugrnglists) | struct | The `DebugRngLists` struct represents the contents of the `.debug_rnglists` section. |
| [`RangeLists`](#rangelists) | struct | The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections. |
| [`RawRngListIter`](#rawrnglistiter) | struct | A raw iterator over an address range list. |
| [`RngListIter`](#rnglistiter) | struct | An iterator over an address range list. |
| [`RawRange`](#rawrange) | struct | A raw address range from the `.debug_ranges` section. |
| [`Range`](#range) | struct | An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections. |
| [`RangeListsFormat`](#rangelistsformat) | enum |  |
| [`RawRngListEntry`](#rawrnglistentry) | enum | A raw entry in .debug_rnglists |
| [`RngListsHeader`](#rnglistsheader) | type |  |

## Structs

### `DebugRanges<R>`

```rust
struct DebugRanges<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:14-16`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L14-L16)*

The raw contents of the `.debug_ranges` section.

#### Implementations

- <span id="debugranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRanges` instance from the data in the `.debug_ranges`

  section.

  

  It is the caller's responsibility to read the `.debug_ranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRanges, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_ranges_section_somehow = || &buf;

  let debug_ranges = DebugRanges::new(read_debug_ranges_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugRanges<R>`

- <span id="debugranges-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugRanges<R>`

- <span id="debugranges-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugRanges<R>`

- <span id="debugranges-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugRanges<R>`

- <span id="debugranges-clone"></span>`fn clone(&self) -> DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

##### `impl CloneToUninit for DebugRanges<R>`

- <span id="debugranges-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: fmt::Debug> Debug for DebugRanges<R>`

- <span id="debugranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRanges<R>`

- <span id="debugranges-default"></span>`fn default() -> DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

##### `impl<T> From for DebugRanges<R>`

- <span id="debugranges-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugRanges<R>`

- <span id="debugranges-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugRanges<R>`

- <span id="debugranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugranges-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugRanges<R>`

- <span id="debugranges-toowned-type-owned"></span>`type Owned = T`

- <span id="debugranges-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugranges-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugRanges<R>`

- <span id="debugranges-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugranges-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugRanges<R>`

- <span id="debugranges-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugranges-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:74-76`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L74-L76)*

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- <span id="debugrnglists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRngLists` instance from the data in the

  `.debug_rnglists` section.

  

  It is the caller's responsibility to read the `.debug_rnglists`

  section and present it as a `&[u8]` slice. That means using some ELF

  loader on Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRngLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_rnglists_section_somehow = || &buf;

  let debug_rnglists =

      DebugRngLists::new(read_debug_rnglists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugRngLists<R>`

- <span id="debugrnglists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugRngLists<R>`

- <span id="debugrnglists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugRngLists<R>`

- <span id="debugrnglists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugRngLists<R>`

- <span id="debugrnglists-clone"></span>`fn clone(&self) -> DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

##### `impl CloneToUninit for DebugRngLists<R>`

- <span id="debugrnglists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugRngLists<R>`

- <span id="debugrnglists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRngLists<R>`

- <span id="debugrnglists-default"></span>`fn default() -> DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

##### `impl<T> From for DebugRngLists<R>`

- <span id="debugrnglists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugRngLists<R>`

- <span id="debugrnglists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugRngLists<R>`

- <span id="debugrnglists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugrnglists-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugRngLists<R>`

- <span id="debugrnglists-toowned-type-owned"></span>`type Owned = T`

- <span id="debugrnglists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugrnglists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugRngLists<R>`

- <span id="debugrnglists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugrnglists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugRngLists<R>`

- <span id="debugrnglists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugrnglists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:158-161`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L158-L161)*

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- <span id="rangelists-new"></span>`fn new(debug_ranges: DebugRanges<R>, debug_rnglists: DebugRngLists<R>) -> RangeLists<R>` — [`DebugRanges`](../index.md#debugranges), [`DebugRngLists`](../index.md#debugrnglists), [`RangeLists`](../index.md#rangelists)

  Construct a new `RangeLists` instance from the data in the `.debug_ranges` and

  `.debug_rnglists` sections.

- <span id="rangelists-debug-ranges"></span>`fn debug_ranges(&self) -> &DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

  Return the `.debug_ranges` section.

- <span id="rangelists-set-debug-ranges"></span>`fn set_debug_ranges(&mut self, debug_ranges: DebugRanges<R>)` — [`DebugRanges`](../index.md#debugranges)

  Replace the `.debug_ranges` section.

  

  This is useful for `.dwo` files when using the GNU split-dwarf extension to DWARF 4.

- <span id="rangelists-debug-rnglists"></span>`fn debug_rnglists(&self) -> &DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

  Return the `.debug_rnglists` section.

#### Trait Implementations

##### `impl Any for RangeLists<R>`

- <span id="rangelists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeLists<R>`

- <span id="rangelists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeLists<R>`

- <span id="rangelists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for RangeLists<R>`

- <span id="rangelists-clone"></span>`fn clone(&self) -> RangeLists<R>` — [`RangeLists`](../index.md#rangelists)

##### `impl CloneToUninit for RangeLists<R>`

- <span id="rangelists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for RangeLists<R>`

##### `impl<R: fmt::Debug> Debug for RangeLists<R>`

- <span id="rangelists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for RangeLists<R>`

- <span id="rangelists-default"></span>`fn default() -> RangeLists<R>` — [`RangeLists`](../index.md#rangelists)

##### `impl<T> From for RangeLists<R>`

- <span id="rangelists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeLists<R>`

- <span id="rangelists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RangeLists<R>`

- <span id="rangelists-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RangeLists<R>`

- <span id="rangelists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeLists<R>`

- <span id="rangelists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:306-310`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L306-L310)*

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- <span id="rawrnglistiter-new"></span>`fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../../index.md#encoding), [`RangeListsFormat`](#rangelistsformat), [`RawRngListIter`](../index.md#rawrnglistiter)

  Construct a `RawRngListIter`.

- <span id="rawrnglistiter-next"></span>`fn next(&mut self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`RawRngListEntry`](../index.md#rawrnglistentry), [`Reader`](../index.md#reader)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl Any for RawRngListIter<R>`

- <span id="rawrnglistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRngListIter<R>`

- <span id="rawrnglistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRngListIter<R>`

- <span id="rawrnglistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RawRngListIter<R>`

- <span id="rawrnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawRngListIter<R>`

- <span id="rawrnglistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawRngListIter<R>`

- <span id="rawrnglistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawRngListIter<R>`

- <span id="rawrnglistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrnglistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawRngListIter<R>`

- <span id="rawrnglistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrnglistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:473-478`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L473-L478)*

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="rnglistiter-new"></span>`fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](../index.md#rawrnglistiter), [`DebugAddr`](../index.md#debugaddr), [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`RngListIter`](../index.md#rnglistiter)

  Construct a `RngListIter`.

- <span id="rnglistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="rnglistiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../../index.md#result), [`Range`](../index.md#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl Any for RngListIter<R>`

- <span id="rnglistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RngListIter<R>`

- <span id="rnglistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RngListIter<R>`

- <span id="rnglistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RngListIter<R>`

- <span id="rnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RngListIter<R>`

- <span id="rnglistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RngListIter<R>`

- <span id="rnglistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RngListIter<R>`

- <span id="rnglistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rnglistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RngListIter<R>`

- <span id="rnglistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rnglistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:598-604`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L598-L604)*

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="rawrange-is-end"></span>`fn is_end(&self) -> bool`

  Check if this is a range end entry.

- <span id="rawrange-is-base-address"></span>`fn is_base_address(&self, address_size: u8) -> bool`

  Check if this is a base address selection entry.

  

  A base address selection entry changes the base address that subsequent

  range entries are relative to.

- <span id="rawrange-parse"></span>`fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../../index.md#result), [`RawRange`](#rawrange)

  Parse an address range entry from `.debug_ranges` or `.debug_loc`.

#### Trait Implementations

##### `impl Any for RawRange`

- <span id="rawrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRange`

- <span id="rawrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRange`

- <span id="rawrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RawRange`

- <span id="rawrange-clone"></span>`fn clone(&self) -> RawRange` — [`RawRange`](#rawrange)

##### `impl CloneToUninit for RawRange`

- <span id="rawrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- <span id="rawrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RawRange`

##### `impl<T> From for RawRange`

- <span id="rawrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RawRange`

- <span id="rawrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RawRange`

- <span id="rawrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RawRange`

- <span id="rawrange-partialeq-eq"></span>`fn eq(&self, other: &RawRange) -> bool` — [`RawRange`](#rawrange)

##### `impl StructuralPartialEq for RawRange`

##### `impl ToOwned for RawRange`

- <span id="rawrange-toowned-type-owned"></span>`type Owned = T`

- <span id="rawrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RawRange`

- <span id="rawrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawRange`

- <span id="rawrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:634-640`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L634-L640)*

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="range-add-base-address"></span>`fn add_base_address(&mut self, base_address: u64, address_size: u8)`

  Add a base address to this range.

#### Trait Implementations

##### `impl Any for Range`

- <span id="range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Range`

- <span id="range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Range`

- <span id="range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Range`

- <span id="range-clone"></span>`fn clone(&self) -> Range` — [`Range`](../index.md#range)

##### `impl CloneToUninit for Range`

- <span id="range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Range`

##### `impl Debug for Range`

- <span id="range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Range`

##### `impl<T> From for Range`

- <span id="range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Range`

- <span id="range-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Range`

- <span id="range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Range`

- <span id="range-ord-cmp"></span>`fn cmp(&self, other: &Range) -> cmp::Ordering` — [`Range`](../index.md#range)

##### `impl PartialEq for Range`

- <span id="range-partialeq-eq"></span>`fn eq(&self, other: &Range) -> bool` — [`Range`](../index.md#range)

##### `impl PartialOrd for Range`

- <span id="range-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Range) -> option::Option<cmp::Ordering>` — [`Range`](../index.md#range)

##### `impl StructuralPartialEq for Range`

##### `impl ToOwned for Range`

- <span id="range-toowned-type-owned"></span>`type Owned = T`

- <span id="range-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="range-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Range`

- <span id="range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Range`

- <span id="range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:294-299`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L294-L299)*

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Any for RangeListsFormat`

- <span id="rangelistsformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeListsFormat`

- <span id="rangelistsformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeListsFormat`

- <span id="rangelistsformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RangeListsFormat`

- <span id="rangelistsformat-clone"></span>`fn clone(&self) -> RangeListsFormat` — [`RangeListsFormat`](#rangelistsformat)

##### `impl CloneToUninit for RangeListsFormat`

- <span id="rangelistsformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- <span id="rangelistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl<T> From for RangeListsFormat`

- <span id="rangelistsformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeListsFormat`

- <span id="rangelistsformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RangeListsFormat`

- <span id="rangelistsformat-partialeq-eq"></span>`fn eq(&self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](#rangelistsformat)

##### `impl StructuralPartialEq for RangeListsFormat`

##### `impl ToOwned for RangeListsFormat`

- <span id="rangelistsformat-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelistsformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelistsformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RangeListsFormat`

- <span id="rangelistsformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelistsformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeListsFormat`

- <span id="rangelistsformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelistsformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRngListEntry<T>`

```rust
enum RawRngListEntry<T> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<T>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<T>,
        end: crate::common::DebugAddrIndex<T>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<T>,
        length: u64,
    },
    OffsetPair {
        begin: u64,
        end: u64,
    },
    StartEnd {
        begin: u64,
        end: u64,
    },
    StartLength {
        begin: u64,
        length: u64,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:314-367`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L314-L367)*

A raw entry in .debug_rnglists

#### Variants

- **`AddressOrOffsetPair`**

  A range from DWARF version <= 4.

- **`BaseAddress`**

  DW_RLE_base_address

- **`BaseAddressx`**

  DW_RLE_base_addressx

- **`StartxEndx`**

  DW_RLE_startx_endx

- **`StartxLength`**

  DW_RLE_startx_length

- **`OffsetPair`**

  DW_RLE_offset_pair

- **`StartEnd`**

  DW_RLE_start_end

- **`StartLength`**

  DW_RLE_start_length

#### Implementations

- <span id="rawrnglistentry-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, encoding: Encoding, format: RangeListsFormat) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`RangeListsFormat`](#rangelistsformat), [`Result`](../../index.md#result)

  Parse a range entry from `.debug_rnglists`

#### Trait Implementations

##### `impl<T> Any for RawRngListEntry<T>`

- <span id="rawrnglistentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRngListEntry<T>`

- <span id="rawrnglistentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRngListEntry<T>`

- <span id="rawrnglistentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RawRngListEntry<T>`

- <span id="rawrnglistentry-clone"></span>`fn clone(&self) -> RawRngListEntry<T>` — [`RawRngListEntry`](../index.md#rawrnglistentry)

##### `impl<T> CloneToUninit for RawRngListEntry<T>`

- <span id="rawrnglistentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for RawRngListEntry<T>`

- <span id="rawrnglistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawRngListEntry<T>`

- <span id="rawrnglistentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RawRngListEntry<T>`

- <span id="rawrnglistentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToOwned for RawRngListEntry<T>`

- <span id="rawrnglistentry-toowned-type-owned"></span>`type Owned = T`

- <span id="rawrnglistentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawrnglistentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RawRngListEntry<T>`

- <span id="rawrnglistentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrnglistentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RawRngListEntry<T>`

- <span id="rawrnglistentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrnglistentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:133`](../../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L133)*

