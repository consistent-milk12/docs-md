*[gimli](../../index.md) / [read](../index.md) / [loclists](index.md)*

---

# Module `loclists`

## Structs

### `DebugLoc<R>`

```rust
struct DebugLoc<R> {
    section: R,
}
```

The raw contents of the `.debug_loc` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugLoc<R>` — [`DebugLoc`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLoc<R>`

- `fn clone(self: &Self) -> DebugLoc<R>` — [`DebugLoc`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLoc<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLoc<R>`

- `fn default() -> DebugLoc<R>` — [`DebugLoc`](../index.md)

##### `impl<R> Section for DebugLoc<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugLocLists<R>`

```rust
struct DebugLocLists<R> {
    section: R,
}
```

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugLocLists<R>` — [`DebugLocLists`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLocLists<R>`

- `fn clone(self: &Self) -> DebugLocLists<R>` — [`DebugLocLists`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLocLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLocLists<R>`

- `fn default() -> DebugLocLists<R>` — [`DebugLocLists`](../index.md)

##### `impl<R> Section for DebugLocLists<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `LocationLists<R>`

```rust
struct LocationLists<R> {
    debug_loc: DebugLoc<R>,
    debug_loclists: DebugLocLists<R>,
}
```

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

#### Implementations

- `fn locations(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../../index.md), [`Reader`](../index.md), [`Encoding`](../../index.md), [`DebugAddr`](../index.md), [`DebugAddrBase`](../../index.md), [`Result`](../../index.md), [`LocListIter`](../index.md)

- `fn locations_dwo(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../../index.md), [`Reader`](../index.md), [`Encoding`](../../index.md), [`DebugAddr`](../index.md), [`DebugAddrBase`](../../index.md), [`Result`](../../index.md), [`LocListIter`](../index.md)

- `fn raw_locations(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../../index.md), [`Reader`](../index.md), [`Encoding`](../../index.md), [`Result`](../../index.md), [`RawLocListIter`](../index.md)

- `fn raw_locations_dwo(self: &Self, offset: LocationListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../../index.md), [`Reader`](../index.md), [`Encoding`](../../index.md), [`Result`](../../index.md), [`RawLocListIter`](../index.md)

- `fn get_offset(self: &Self, unit_encoding: Encoding, base: DebugLocListsBase<<R as >::Offset>, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` — [`Encoding`](../../index.md), [`DebugLocListsBase`](../../index.md), [`Reader`](../index.md), [`DebugLocListsIndex`](../../index.md), [`Result`](../../index.md), [`LocationListsOffset`](../../index.md)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>` — [`ReaderOffsetId`](../index.md), [`SectionId`](../../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for LocationLists<R>`

- `fn clone(self: &Self) -> LocationLists<R>` — [`LocationLists`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for LocationLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for LocationLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for LocationLists<R>`

- `fn default() -> LocationLists<R>` — [`LocationLists`](../index.md)

### `RawLocListIter<R: Reader>`

```rust
struct RawLocListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: LocListsFormat,
}
```

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

#### Implementations

- `fn new(input: R, encoding: Encoding, format: LocListsFormat) -> RawLocListIter<R>` — [`Encoding`](../../index.md), [`LocListsFormat`](#loclistsformat), [`RawLocListIter`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../../index.md), [`RawLocListEntry`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RawLocListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LocListIter<R: Reader>`

```rust
struct LocListIter<R: Reader> {
    raw: RawLocListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

#### Implementations

- `fn new(raw: RawLocListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> LocListIter<R>` — [`RawLocListIter`](../index.md), [`DebugAddr`](../index.md), [`DebugAddrBase`](../../index.md), [`Reader`](../index.md), [`LocListIter`](../index.md)

- `fn get_address(self: &Self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

- `fn next(self: &mut Self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../../index.md), [`LocationListEntry`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for LocListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LocationListEntry<R: Reader>`

```rust
struct LocationListEntry<R: Reader> {
    pub range: crate::read::Range,
    pub data: crate::read::Expression<R>,
}
```

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

#### Fields

- **`range`**: `crate::read::Range`

  The address range that this location is valid for.

- **`data`**: `crate::read::Expression<R>`

  The data containing a single location description.

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for LocationListEntry<R>`

- `fn clone(self: &Self) -> LocationListEntry<R>` — [`LocationListEntry`](../index.md)

##### `impl<R: $crate::marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for LocationListEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<R: $crate::hash::Hash + Reader> Hash for LocationListEntry<R>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- `fn eq(self: &Self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](../index.md)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

## Enums

### `LocListsFormat`

```rust
enum LocListsFormat {
    Bare,
    Lle,
}
```

#### Variants

- **`Bare`**

  The bare location list format used before DWARF 5.

- **`Lle`**

  The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU
  split dwarf extension.

#### Trait Implementations

##### `impl Clone for LocListsFormat`

- `fn clone(self: &Self) -> LocListsFormat` — [`LocListsFormat`](#loclistsformat)

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl PartialEq for LocListsFormat`

- `fn eq(self: &Self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](#loclistsformat)

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

- `fn parse(input: &mut R, encoding: Encoding, format: LocListsFormat) -> Result<Option<Self>>` — [`Encoding`](../../index.md), [`LocListsFormat`](#loclistsformat), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for RawLocListEntry<R>`

- `fn clone(self: &Self) -> RawLocListEntry<R>` — [`RawLocListEntry`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `parse_data`

```rust
fn parse_data<R: Reader>(input: &mut R, encoding: crate::common::Encoding) -> crate::read::Result<crate::read::Expression<R>>
```

## Type Aliases

### `LocListsHeader`

```rust
type LocListsHeader = crate::read::lists::ListsHeader;
```

