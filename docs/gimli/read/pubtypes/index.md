*[gimli](../../index.md) / [read](../index.md) / [pubtypes](index.md)*

---

# Module `pubtypes`

## Structs

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubtype.

#### Implementations

- `fn name(self: &Self) -> &R`

- `fn unit_header_offset(self: &Self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md), [`Reader`](../index.md)

- `fn die_offset(self: &Self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubTypesEntry<R>`

- `fn clone(self: &Self) -> PubTypesEntry<R>` — [`PubTypesEntry`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`DebugInfoOffset`](../../index.md)

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- `fn new(debug_pubtypes_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugPubTypes<R>`

- `fn clone(self: &Self) -> DebugPubTypes<R>` — [`DebugPubTypes`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

An iterator over the pubtypes from a `.debug_pubtypes` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../../index.md), [`PubTypesEntry`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- `fn clone(self: &Self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

