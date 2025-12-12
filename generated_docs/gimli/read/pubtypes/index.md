*[gimli](../../index.md) / [read](../index.md) / [pubtypes](index.md)*

---

# Module `pubtypes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PubTypesEntry`](#pubtypesentry) | struct | A single parsed pubtype. |
| [`DebugPubTypes`](#debugpubtypes) | struct | The `DebugPubTypes` struct represents the DWARF public types information found in the `.debug_info` section. |
| [`PubTypesEntryIter`](#pubtypesentryiter) | struct | An iterator over the pubtypes from a `.debug_pubtypes` section. |

## Structs

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:8-12`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubtypes.rs#L8-L12)*

A single parsed pubtype.

#### Implementations

- <span id="pubtypesentry-name"></span>`fn name(&self) -> &R`

- <span id="pubtypesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

- <span id="pubtypesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntry<R>`

- <span id="pubtypesentry-clone"></span>`fn clone(&self) -> PubTypesEntry<R>` — [`PubTypesEntry`](../index.md#pubtypesentry)

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- <span id="pubtypesentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- <span id="pubtypesentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`DebugInfoOffset`](../../index.md#debuginfooffset)

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:50`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubtypes.rs#L50)*

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- <span id="debugpubtypes-new"></span>`fn new(debug_pubtypes_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugPubTypes<R>`

- <span id="debugpubtypes-clone"></span>`fn clone(&self) -> DebugPubTypes<R>` — [`DebugPubTypes`](../index.md#debugpubtypes)

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- <span id="debugpubtypes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- <span id="debugpubtypes-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugpubtypes-reader"></span>`fn reader(&self) -> &R`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:118`](../../../../.source_1765521767/gimli-0.32.3/src/read/pubtypes.rs#L118)*

An iterator over the pubtypes from a `.debug_pubtypes` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="pubtypesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../../index.md#result), [`PubTypesEntry`](../index.md#pubtypesentry)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clone"></span>`fn clone(&self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](../index.md#pubtypesentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

