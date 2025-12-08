*[gimli](../../index.md) / [read](../index.md) / [pubnames](index.md)*

---

# Module `pubnames`

## Structs

### `PubNamesEntry<R: Reader>`

```rust
struct PubNamesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubname.

#### Implementations

- `fn name(self: &Self) -> &R`

- `fn unit_header_offset(self: &Self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md), [`Reader`](../index.md)

- `fn die_offset(self: &Self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubNamesEntry<R>`

- `fn clone(self: &Self) -> PubNamesEntry<R>` — [`PubNamesEntry`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`DebugInfoOffset`](../../index.md)

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- `fn items(self: &Self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugPubNames<R>`

- `fn clone(self: &Self) -> DebugPubNames<R>` — [`DebugPubNames`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugPubNames<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> Section for DebugPubNames<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

An iterator over the pubnames from a `.debug_pubnames` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../../index.md), [`PubNamesEntry`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- `fn clone(self: &Self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

