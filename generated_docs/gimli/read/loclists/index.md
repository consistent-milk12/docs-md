*[gimli](../../index.md) / [read](../index.md) / [loclists](index.md)*

---

# Module `loclists`

## Contents

- [Structs](#structs)
  - [`DebugLoc`](#debugloc)
  - [`DebugLocLists`](#debugloclists)
  - [`LocationLists`](#locationlists)
  - [`RawLocListIter`](#rawloclistiter)
  - [`LocListIter`](#loclistiter)
  - [`LocationListEntry`](#locationlistentry)
- [Enums](#enums)
  - [`LocListsFormat`](#loclistsformat)
  - [`RawLocListEntry`](#rawloclistentry)
- [Functions](#functions)
  - [`parse_data`](#parse-data)
- [Type Aliases](#type-aliases)
  - [`LocListsHeader`](#loclistsheader)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugLoc`](#debugloc) | struct | The raw contents of the `.debug_loc` section. |
| [`DebugLocLists`](#debugloclists) | struct | The `DebugLocLists` struct represents the DWARF data found in the `.debug_loclists` section. |
| [`LocationLists`](#locationlists) | struct | The DWARF data found in `.debug_loc` and `.debug_loclists` sections. |
| [`RawLocListIter`](#rawloclistiter) | struct | A raw iterator over a location list. |
| [`LocListIter`](#loclistiter) | struct | An iterator over a location list. |
| [`LocationListEntry`](#locationlistentry) | struct | A location list entry from the `.debug_loc` or `.debug_loclists` sections. |
| [`LocListsFormat`](#loclistsformat) | enum |  |
| [`RawLocListEntry`](#rawloclistentry) | enum | A raw entry in .debug_loclists. |
| [`parse_data`](#parse-data) | fn |  |
| [`LocListsHeader`](#loclistsheader) | type |  |

## Structs

### `DebugLoc<R>`

```rust
struct DebugLoc<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:14-16`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L14-L16)*

The raw contents of the `.debug_loc` section.

#### Implementations

- <span id="debugloc-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLoc` instance from the data in the `.debug_loc`

  section.

  

  It is the caller's responsibility to read the `.debug_loc` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLoc, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loc_section_somehow = || &buf;

  let debug_loc = DebugLoc::new(read_debug_loc_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLoc<R>`

- <span id="debugloc-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLoc<R>`

- <span id="debugloc-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLoc<R>`

- <span id="debugloc-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLoc<R>`

- <span id="debugloc-clone"></span>`fn clone(&self) -> DebugLoc<R>` — [`DebugLoc`](../index.md#debugloc)

##### `impl CloneToUninit for DebugLoc<R>`

- <span id="debugloc-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: fmt::Debug> Debug for DebugLoc<R>`

- <span id="debugloc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLoc<R>`

- <span id="debugloc-default"></span>`fn default() -> DebugLoc<R>` — [`DebugLoc`](../index.md#debugloc)

##### `impl<T> From for DebugLoc<R>`

- <span id="debugloc-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLoc<R>`

- <span id="debugloc-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLoc<R>`

- <span id="debugloc-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugloc-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLoc<R>`

- <span id="debugloc-toowned-type-owned"></span>`type Owned = T`

- <span id="debugloc-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugloc-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLoc<R>`

- <span id="debugloc-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugloc-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLoc<R>`

- <span id="debugloc-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugloc-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLocLists<R>`

```rust
struct DebugLocLists<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:74-76`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L74-L76)*

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

#### Implementations

- <span id="debugloclists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLocLists` instance from the data in the `.debug_loclists`

  section.

  

  It is the caller's responsibility to read the `.debug_loclists` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLocLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loclists_section_somehow = || &buf;

  let debug_loclists = DebugLocLists::new(read_debug_loclists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLocLists<R>`

- <span id="debugloclists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLocLists<R>`

- <span id="debugloclists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLocLists<R>`

- <span id="debugloclists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLocLists<R>`

- <span id="debugloclists-clone"></span>`fn clone(&self) -> DebugLocLists<R>` — [`DebugLocLists`](../index.md#debugloclists)

##### `impl CloneToUninit for DebugLocLists<R>`

- <span id="debugloclists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugLocLists<R>`

- <span id="debugloclists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLocLists<R>`

- <span id="debugloclists-default"></span>`fn default() -> DebugLocLists<R>` — [`DebugLocLists`](../index.md#debugloclists)

##### `impl<T> From for DebugLocLists<R>`

- <span id="debugloclists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLocLists<R>`

- <span id="debugloclists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLocLists<R>`

- <span id="debugloclists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugloclists-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLocLists<R>`

- <span id="debugloclists-toowned-type-owned"></span>`type Owned = T`

- <span id="debugloclists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugloclists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLocLists<R>`

- <span id="debugloclists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugloclists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLocLists<R>`

- <span id="debugloclists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugloclists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocationLists<R>`

```rust
struct LocationLists<R> {
    debug_loc: DebugLoc<R>,
    debug_loclists: DebugLocLists<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:156-159`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L156-L159)*

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

#### Implementations

- <span id="locationlists-new"></span>`fn new(debug_loc: DebugLoc<R>, debug_loclists: DebugLocLists<R>) -> LocationLists<R>` — [`DebugLoc`](../index.md#debugloc), [`DebugLocLists`](../index.md#debugloclists), [`LocationLists`](../index.md#locationlists)

  Construct a new `LocationLists` instance from the data in the `.debug_loc` and

  `.debug_loclists` sections.

#### Trait Implementations

##### `impl Any for LocationLists<R>`

- <span id="locationlists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocationLists<R>`

- <span id="locationlists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocationLists<R>`

- <span id="locationlists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for LocationLists<R>`

- <span id="locationlists-clone"></span>`fn clone(&self) -> LocationLists<R>` — [`LocationLists`](../index.md#locationlists)

##### `impl CloneToUninit for LocationLists<R>`

- <span id="locationlists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for LocationLists<R>`

##### `impl<R: fmt::Debug> Debug for LocationLists<R>`

- <span id="locationlists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for LocationLists<R>`

- <span id="locationlists-default"></span>`fn default() -> LocationLists<R>` — [`LocationLists`](../index.md#locationlists)

##### `impl<T> From for LocationLists<R>`

- <span id="locationlists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocationLists<R>`

- <span id="locationlists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LocationLists<R>`

- <span id="locationlists-toowned-type-owned"></span>`type Owned = T`

- <span id="locationlists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="locationlists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocationLists<R>`

- <span id="locationlists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locationlists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocationLists<R>`

- <span id="locationlists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locationlists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawLocListIter<R: Reader>`

```rust
struct RawLocListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: LocListsFormat,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:329-333`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L329-L333)*

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

#### Implementations

- <span id="rawloclistiter-new"></span>`fn new(input: R, encoding: Encoding, format: LocListsFormat) -> RawLocListIter<R>` — [`Encoding`](../../index.md#encoding), [`LocListsFormat`](#loclistsformat), [`RawLocListIter`](../index.md#rawloclistiter)

  Construct a `RawLocListIter`.

- <span id="rawloclistiter-next"></span>`fn next(&mut self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../../index.md#result), [`RawLocListEntry`](../index.md#rawloclistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl Any for RawLocListIter<R>`

- <span id="rawloclistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawLocListIter<R>`

- <span id="rawloclistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawLocListIter<R>`

- <span id="rawloclistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListIter<R>`

- <span id="rawloclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawLocListIter<R>`

- <span id="rawloclistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawLocListIter<R>`

- <span id="rawloclistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawLocListIter<R>`

- <span id="rawloclistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawloclistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawLocListIter<R>`

- <span id="rawloclistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawloclistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocListIter<R: Reader>`

```rust
struct LocListIter<R: Reader> {
    raw: RawLocListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:536-541`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L536-L541)*

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="loclistiter-new"></span>`fn new(raw: RawLocListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> LocListIter<R>` — [`RawLocListIter`](../index.md#rawloclistiter), [`DebugAddr`](../index.md#debugaddr), [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`LocListIter`](../index.md#loclistiter)

  Construct a `LocListIter`.

- <span id="loclistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="loclistiter-next"></span>`fn next(&mut self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../../index.md#result), [`LocationListEntry`](../index.md#locationlistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl Any for LocListIter<R>`

- <span id="loclistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocListIter<R>`

- <span id="loclistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocListIter<R>`

- <span id="loclistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for LocListIter<R>`

- <span id="loclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LocListIter<R>`

- <span id="loclistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocListIter<R>`

- <span id="loclistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LocListIter<R>`

- <span id="loclistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loclistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocListIter<R>`

- <span id="loclistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loclistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocationListEntry<R: Reader>`

```rust
struct LocationListEntry<R: Reader> {
    pub range: crate::read::Range,
    pub data: crate::read::Expression<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:679-685`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L679-L685)*

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

#### Fields

- **`range`**: `crate::read::Range`

  The address range that this location is valid for.

- **`data`**: `crate::read::Expression<R>`

  The data containing a single location description.

#### Trait Implementations

##### `impl Any for LocationListEntry<R>`

- <span id="locationlistentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocationListEntry<R>`

- <span id="locationlistentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocationListEntry<R>`

- <span id="locationlistentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for LocationListEntry<R>`

- <span id="locationlistentry-clone"></span>`fn clone(&self) -> LocationListEntry<R>` — [`LocationListEntry`](../index.md#locationlistentry)

##### `impl CloneToUninit for LocationListEntry<R>`

- <span id="locationlistentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: fmt::Debug + Reader> Debug for LocationListEntry<R>`

- <span id="locationlistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<T> From for LocationListEntry<R>`

- <span id="locationlistentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<R: hash::Hash + Reader> Hash for LocationListEntry<R>`

- <span id="locationlistentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LocationListEntry<R>`

- <span id="locationlistentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- <span id="locationlistentry-partialeq-eq"></span>`fn eq(&self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](../index.md#locationlistentry)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

##### `impl ToOwned for LocationListEntry<R>`

- <span id="locationlistentry-toowned-type-owned"></span>`type Owned = T`

- <span id="locationlistentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="locationlistentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocationListEntry<R>`

- <span id="locationlistentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locationlistentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocationListEntry<R>`

- <span id="locationlistentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locationlistentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `LocListsFormat`

```rust
enum LocListsFormat {
    Bare,
    Lle,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:316-322`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L316-L322)*

#### Variants

- **`Bare`**

  The bare location list format used before DWARF 5.

- **`Lle`**

  The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU
  split dwarf extension.

#### Trait Implementations

##### `impl Any for LocListsFormat`

- <span id="loclistsformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocListsFormat`

- <span id="loclistsformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocListsFormat`

- <span id="loclistsformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LocListsFormat`

- <span id="loclistsformat-clone"></span>`fn clone(&self) -> LocListsFormat` — [`LocListsFormat`](#loclistsformat)

##### `impl CloneToUninit for LocListsFormat`

- <span id="loclistsformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- <span id="loclistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl<T> From for LocListsFormat`

- <span id="loclistsformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocListsFormat`

- <span id="loclistsformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LocListsFormat`

- <span id="loclistsformat-partialeq-eq"></span>`fn eq(&self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](#loclistsformat)

##### `impl StructuralPartialEq for LocListsFormat`

##### `impl ToOwned for LocListsFormat`

- <span id="loclistsformat-toowned-type-owned"></span>`type Owned = T`

- <span id="loclistsformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loclistsformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocListsFormat`

- <span id="loclistsformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loclistsformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocListsFormat`

- <span id="loclistsformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loclistsformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawLocListEntry<R: Reader>`

```rust
enum RawLocListEntry<R: Reader> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<<R as >::Offset>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        end: crate::common::DebugAddrIndex<<R as >::Offset>,
        data: crate::read::Expression<R>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        length: u64,
        data: crate::read::Expression<R>,
    },
    OffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    DefaultLocation {
        data: crate::read::Expression<R>,
    },
    StartEnd {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    StartLength {
        begin: u64,
        length: u64,
        data: crate::read::Expression<R>,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:337-407`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L337-L407)*

A raw entry in .debug_loclists.

#### Variants

- **`AddressOrOffsetPair`**

  A location from DWARF version <= 4.

- **`BaseAddress`**

  DW_LLE_base_address

- **`BaseAddressx`**

  DW_LLE_base_addressx

- **`StartxEndx`**

  DW_LLE_startx_endx

- **`StartxLength`**

  DW_LLE_startx_length

- **`OffsetPair`**

  DW_LLE_offset_pair

- **`DefaultLocation`**

  DW_LLE_default_location

- **`StartEnd`**

  DW_LLE_start_end

- **`StartLength`**

  DW_LLE_start_length

#### Implementations

- <span id="rawloclistentry-parse"></span>`fn parse(input: &mut R, encoding: Encoding, format: LocListsFormat) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`LocListsFormat`](#loclistsformat), [`Result`](../../index.md#result)

  Parse a location list entry from `.debug_loclists`

#### Trait Implementations

##### `impl Any for RawLocListEntry<R>`

- <span id="rawloclistentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawLocListEntry<R>`

- <span id="rawloclistentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawLocListEntry<R>`

- <span id="rawloclistentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for RawLocListEntry<R>`

- <span id="rawloclistentry-clone"></span>`fn clone(&self) -> RawLocListEntry<R>` — [`RawLocListEntry`](../index.md#rawloclistentry)

##### `impl CloneToUninit for RawLocListEntry<R>`

- <span id="rawloclistentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- <span id="rawloclistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawLocListEntry<R>`

- <span id="rawloclistentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawLocListEntry<R>`

- <span id="rawloclistentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RawLocListEntry<R>`

- <span id="rawloclistentry-toowned-type-owned"></span>`type Owned = T`

- <span id="rawloclistentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawloclistentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RawLocListEntry<R>`

- <span id="rawloclistentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawloclistentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawLocListEntry<R>`

- <span id="rawloclistentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawloclistentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_data`

```rust
fn parse_data<R: Reader>(input: &mut R, encoding: crate::common::Encoding) -> crate::read::Result<crate::read::Expression<R>>
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:409-418`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L409-L418)*

## Type Aliases

### `LocListsHeader`

```rust
type LocListsHeader = crate::read::lists::ListsHeader;
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:131`](../../../../.source_1765521767/gimli-0.32.3/src/read/loclists.rs#L131)*

