*[gimli](../../index.md) / [read](../index.md) / [pubnames](index.md)*

---

# Module `pubnames`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PubNamesEntry`](#pubnamesentry) | struct | A single parsed pubname. |
| [`DebugPubNames`](#debugpubnames) | struct | The `DebugPubNames` struct represents the DWARF public names information |
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

A single parsed pubname.

#### Implementations

- <span id="pubnamesentry-name"></span>`fn name(&self) -> &R`

- <span id="pubnamesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md), [`Reader`](../index.md)

- <span id="pubnamesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntry<R>`

- <span id="pubnamesentry-clone"></span>`fn clone(&self) -> PubNamesEntry<R>` — [`PubNamesEntry`](../index.md)

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- <span id="pubnamesentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- <span id="pubnamesentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`DebugInfoOffset`](../../index.md)

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- <span id="debugpubnames-new"></span>`fn new(debug_pubnames_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugPubNames<R>`

- <span id="debugpubnames-clone"></span>`fn clone(&self) -> DebugPubNames<R>` — [`DebugPubNames`](../index.md)

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubNames<R>`

- <span id="debugpubnames-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Section for DebugPubNames<R>`

- <span id="debugpubnames-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md)

- <span id="debugpubnames-reader"></span>`fn reader(&self) -> &R`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

An iterator over the pubnames from a `.debug_pubnames` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="pubnamesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../../index.md), [`PubNamesEntry`](../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clone"></span>`fn clone(&self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](../index.md)

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

