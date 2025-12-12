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

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLoc<R>`

- <span id="debugloc-clone"></span>`fn clone(&self) -> DebugLoc<R>` — [`DebugLoc`](../index.md#debugloc)

##### `impl<R: marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: fmt::Debug> Debug for DebugLoc<R>`

- <span id="debugloc-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLoc<R>`

- <span id="debugloc-default"></span>`fn default() -> DebugLoc<R>` — [`DebugLoc`](../index.md#debugloc)

##### `impl<R> Section for DebugLoc<R>`

- <span id="debugloc-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugloc-reader"></span>`fn reader(&self) -> &R`

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

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLocLists<R>`

- <span id="debugloclists-clone"></span>`fn clone(&self) -> DebugLocLists<R>` — [`DebugLocLists`](../index.md#debugloclists)

##### `impl<R: marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugLocLists<R>`

- <span id="debugloclists-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLocLists<R>`

- <span id="debugloclists-default"></span>`fn default() -> DebugLocLists<R>` — [`DebugLocLists`](../index.md#debugloclists)

##### `impl<R> Section for DebugLocLists<R>`

- <span id="debugloclists-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugloclists-reader"></span>`fn reader(&self) -> &R`

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

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for LocationLists<R>`

- <span id="locationlists-clone"></span>`fn clone(&self) -> LocationLists<R>` — [`LocationLists`](../index.md#locationlists)

##### `impl<R: marker::Copy> Copy for LocationLists<R>`

##### `impl<R: fmt::Debug> Debug for LocationLists<R>`

- <span id="locationlists-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for LocationLists<R>`

- <span id="locationlists-default"></span>`fn default() -> LocationLists<R>` — [`LocationLists`](../index.md#locationlists)

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

- <span id="rawloclistiter-next"></span>`fn next(&mut self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../../index.md#result), [`RawLocListEntry`](../index.md#rawloclistentry)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListIter<R>`

- <span id="rawloclistiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="loclistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="loclistiter-next"></span>`fn next(&mut self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../../index.md#result), [`LocationListEntry`](../index.md#locationlistentry)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for LocListIter<R>`

- <span id="loclistiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl<R: clone::Clone + Reader> Clone for LocationListEntry<R>`

- <span id="locationlistentry-clone"></span>`fn clone(&self) -> LocationListEntry<R>` — [`LocationListEntry`](../index.md#locationlistentry)

##### `impl<R: marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: fmt::Debug + Reader> Debug for LocationListEntry<R>`

- <span id="locationlistentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<R: hash::Hash + Reader> Hash for LocationListEntry<R>`

- <span id="locationlistentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- <span id="locationlistentry-eq"></span>`fn eq(&self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](../index.md#locationlistentry)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

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

##### `impl Clone for LocListsFormat`

- <span id="loclistsformat-clone"></span>`fn clone(&self) -> LocListsFormat` — [`LocListsFormat`](#loclistsformat)

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- <span id="loclistsformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl PartialEq for LocListsFormat`

- <span id="loclistsformat-eq"></span>`fn eq(&self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](#loclistsformat)

##### `impl StructuralPartialEq for LocListsFormat`

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

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for RawLocListEntry<R>`

- <span id="rawloclistentry-clone"></span>`fn clone(&self) -> RawLocListEntry<R>` — [`RawLocListEntry`](../index.md#rawloclistentry)

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- <span id="rawloclistentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

