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

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:14-16`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L14-L16)*

The raw contents of the `.debug_ranges` section.

#### Implementations

- <span id="debugranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugRanges<R>`

- <span id="debugranges-clone"></span>`fn clone(&self) -> DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

##### `impl<R: marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: fmt::Debug> Debug for DebugRanges<R>`

- <span id="debugranges-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRanges<R>`

- <span id="debugranges-default"></span>`fn default() -> DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

##### `impl<R> Section for DebugRanges<R>`

- <span id="debugranges-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugranges-reader"></span>`fn reader(&self) -> &R`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:74-76`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L74-L76)*

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- <span id="debugrnglists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugRngLists<R>`

- <span id="debugrnglists-clone"></span>`fn clone(&self) -> DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

##### `impl<R: marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugRngLists<R>`

- <span id="debugrnglists-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRngLists<R>`

- <span id="debugrnglists-default"></span>`fn default() -> DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

##### `impl<R> Section for DebugRngLists<R>`

- <span id="debugrnglists-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugrnglists-reader"></span>`fn reader(&self) -> &R`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:158-161`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L158-L161)*

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- <span id="rangelists-new"></span>`fn new(debug_ranges: DebugRanges<R>, debug_rnglists: DebugRngLists<R>) -> RangeLists<R>` — [`DebugRanges`](../index.md#debugranges), [`DebugRngLists`](../index.md#debugrnglists), [`RangeLists`](../index.md#rangelists)

- <span id="rangelists-debug-ranges"></span>`fn debug_ranges(&self) -> &DebugRanges<R>` — [`DebugRanges`](../index.md#debugranges)

- <span id="rangelists-set-debug-ranges"></span>`fn set_debug_ranges(&mut self, debug_ranges: DebugRanges<R>)` — [`DebugRanges`](../index.md#debugranges)

- <span id="rangelists-debug-rnglists"></span>`fn debug_rnglists(&self) -> &DebugRngLists<R>` — [`DebugRngLists`](../index.md#debugrnglists)

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for RangeLists<R>`

- <span id="rangelists-clone"></span>`fn clone(&self) -> RangeLists<R>` — [`RangeLists`](../index.md#rangelists)

##### `impl<R: marker::Copy> Copy for RangeLists<R>`

##### `impl<R: fmt::Debug> Debug for RangeLists<R>`

- <span id="rangelists-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for RangeLists<R>`

- <span id="rangelists-default"></span>`fn default() -> RangeLists<R>` — [`RangeLists`](../index.md#rangelists)

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:306-310`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L306-L310)*

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- <span id="rawrnglistiter-new"></span>`fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../../index.md#encoding), [`RangeListsFormat`](#rangelistsformat), [`RawRngListIter`](../index.md#rawrnglistiter)

- <span id="rawrnglistiter-next"></span>`fn next(&mut self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`RawRngListEntry`](../index.md#rawrnglistentry), [`Reader`](../index.md#reader)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RawRngListIter<R>`

- <span id="rawrnglistiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:473-478`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L473-L478)*

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="rnglistiter-new"></span>`fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](../index.md#rawrnglistiter), [`DebugAddr`](../index.md#debugaddr), [`DebugAddrBase`](../../index.md#debugaddrbase), [`Reader`](../index.md#reader), [`RngListIter`](../index.md#rnglistiter)

- <span id="rnglistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

- <span id="rnglistiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../../index.md#result), [`Range`](../index.md#range)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RngListIter<R>`

- <span id="rnglistiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:598-604`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L598-L604)*

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="rawrange-is-end"></span>`fn is_end(&self) -> bool`

- <span id="rawrange-is-base-address"></span>`fn is_base_address(&self, address_size: u8) -> bool`

- <span id="rawrange-parse"></span>`fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../../index.md#result), [`RawRange`](#rawrange)

#### Trait Implementations

##### `impl Clone for RawRange`

- <span id="rawrange-clone"></span>`fn clone(&self) -> RawRange` — [`RawRange`](#rawrange)

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- <span id="rawrange-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RawRange`

##### `impl Hash for RawRange`

- <span id="rawrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RawRange`

- <span id="rawrange-eq"></span>`fn eq(&self, other: &RawRange) -> bool` — [`RawRange`](#rawrange)

##### `impl StructuralPartialEq for RawRange`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:634-640`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L634-L640)*

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="range-add-base-address"></span>`fn add_base_address(&mut self, base_address: u64, address_size: u8)`

#### Trait Implementations

##### `impl Clone for Range`

- <span id="range-clone"></span>`fn clone(&self) -> Range` — [`Range`](../index.md#range)

##### `impl Copy for Range`

##### `impl Debug for Range`

- <span id="range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Range`

##### `impl Hash for Range`

- <span id="range-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Range`

- <span id="range-cmp"></span>`fn cmp(&self, other: &Range) -> cmp::Ordering` — [`Range`](../index.md#range)

##### `impl PartialEq for Range`

- <span id="range-eq"></span>`fn eq(&self, other: &Range) -> bool` — [`Range`](../index.md#range)

##### `impl PartialOrd for Range`

- <span id="range-partial-cmp"></span>`fn partial_cmp(&self, other: &Range) -> option::Option<cmp::Ordering>` — [`Range`](../index.md#range)

##### `impl StructuralPartialEq for Range`

## Enums

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:294-299`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L294-L299)*

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Clone for RangeListsFormat`

- <span id="rangelistsformat-clone"></span>`fn clone(&self) -> RangeListsFormat` — [`RangeListsFormat`](#rangelistsformat)

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- <span id="rangelistsformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl PartialEq for RangeListsFormat`

- <span id="rangelistsformat-eq"></span>`fn eq(&self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](#rangelistsformat)

##### `impl StructuralPartialEq for RangeListsFormat`

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

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:314-367`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L314-L367)*

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

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RawRngListEntry<T>`

- <span id="rawrnglistentry-clone"></span>`fn clone(&self) -> RawRngListEntry<T>` — [`RawRngListEntry`](../index.md#rawrnglistentry)

##### `impl<T: fmt::Debug> Debug for RawRngListEntry<T>`

- <span id="rawrnglistentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:133`](../../../../.source_1765521767/gimli-0.32.3/src/read/rnglists.rs#L133)*

