*[gimli](../../index.md) / [read](../index.md) / [rnglists](index.md)*

---

# Module `rnglists`

## Structs

### `DebugRanges<R>`

```rust
struct DebugRanges<R> {
    section: R,
}
```

The raw contents of the `.debug_ranges` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugRanges<R>` — [`DebugRanges`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugRanges<R>`

- `fn clone(self: &Self) -> DebugRanges<R>` — [`DebugRanges`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugRanges<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugRanges<R>`

- `fn default() -> DebugRanges<R>` — [`DebugRanges`](../index.md)

##### `impl<R> Section for DebugRanges<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugRngLists<R>` — [`DebugRngLists`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugRngLists<R>`

- `fn clone(self: &Self) -> DebugRngLists<R>` — [`DebugRngLists`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugRngLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugRngLists<R>`

- `fn default() -> DebugRngLists<R>` — [`DebugRngLists`](../index.md)

##### `impl<R> Section for DebugRngLists<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- `fn ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../../index.md), [`Reader`](../index.md), [`Encoding`](../../index.md), [`DebugAddr`](../index.md), [`DebugAddrBase`](../../index.md), [`Result`](../../index.md), [`RngListIter`](../index.md)

- `fn raw_ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../../index.md), [`Reader`](../index.md), [`Encoding`](../../index.md), [`Result`](../../index.md), [`RawRngListIter`](../index.md)

- `fn get_offset(self: &Self, unit_encoding: Encoding, base: DebugRngListsBase<<R as >::Offset>, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`Encoding`](../../index.md), [`DebugRngListsBase`](../../index.md), [`Reader`](../index.md), [`DebugRngListsIndex`](../../index.md), [`Result`](../../index.md), [`RangeListsOffset`](../../index.md)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>` — [`ReaderOffsetId`](../index.md), [`SectionId`](../../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for RangeLists<R>`

- `fn clone(self: &Self) -> RangeLists<R>` — [`RangeLists`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for RangeLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for RangeLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for RangeLists<R>`

- `fn default() -> RangeLists<R>` — [`RangeLists`](../index.md)

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- `fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../../index.md), [`RangeListsFormat`](#rangelistsformat), [`RawRngListIter`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../../index.md), [`RawRngListEntry`](../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RawRngListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- `fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](../index.md), [`DebugAddr`](../index.md), [`DebugAddrBase`](../../index.md), [`Reader`](../index.md), [`RngListIter`](../index.md)

- `fn get_address(self: &Self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

- `fn next(self: &mut Self) -> Result<Option<Range>>` — [`Result`](../../index.md), [`Range`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RngListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- `fn is_end(self: &Self) -> bool`

- `fn is_base_address(self: &Self, address_size: u8) -> bool`

- `fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../../index.md), [`RawRange`](#rawrange)

#### Trait Implementations

##### `impl Clone for RawRange`

- `fn clone(self: &Self) -> RawRange` — [`RawRange`](#rawrange)

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RawRange`

##### `impl Hash for RawRange`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RawRange`

- `fn eq(self: &Self, other: &RawRange) -> bool` — [`RawRange`](#rawrange)

##### `impl StructuralPartialEq for RawRange`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- `fn add_base_address(self: &mut Self, base_address: u64, address_size: u8)`

#### Trait Implementations

##### `impl Clone for Range`

- `fn clone(self: &Self) -> Range` — [`Range`](../index.md)

##### `impl Copy for Range`

##### `impl Debug for Range`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Range`

##### `impl Hash for Range`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Range`

- `fn cmp(self: &Self, other: &Range) -> $crate::cmp::Ordering` — [`Range`](../index.md)

##### `impl PartialEq for Range`

- `fn eq(self: &Self, other: &Range) -> bool` — [`Range`](../index.md)

##### `impl PartialOrd for Range`

- `fn partial_cmp(self: &Self, other: &Range) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Range`](../index.md)

##### `impl StructuralPartialEq for Range`

## Enums

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Clone for RangeListsFormat`

- `fn clone(self: &Self) -> RangeListsFormat` — [`RangeListsFormat`](#rangelistsformat)

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl PartialEq for RangeListsFormat`

- `fn eq(self: &Self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](#rangelistsformat)

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

- `fn parse<R: Reader<Offset = T>>(input: &mut R, encoding: Encoding, format: RangeListsFormat) -> Result<Option<Self>>` — [`Encoding`](../../index.md), [`RangeListsFormat`](#rangelistsformat), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RawRngListEntry<T>`

- `fn clone(self: &Self) -> RawRngListEntry<T>` — [`RawRngListEntry`](../index.md)

##### `impl<T: $crate::fmt::Debug> Debug for RawRngListEntry<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Type Aliases

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

