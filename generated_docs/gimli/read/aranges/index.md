*[gimli](../../index.md) / [read](../index.md) / [aranges](index.md)*

---

# Module `aranges`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAranges`](#debugaranges) | struct | The `DebugAranges` struct represents the DWARF address range information found in the `.debug_aranges` section. |
| [`ArangeHeaderIter`](#arangeheaderiter) | struct | An iterator over the headers of a `.debug_aranges` section. |
| [`ArangeHeader`](#arangeheader) | struct | A header for a set of entries in the `.debug_arange` section. |
| [`ArangeEntryIter`](#arangeentryiter) | struct | An iterator over the aranges from a `.debug_aranges` section. |
| [`ArangeEntry`](#arangeentry) | struct | A single parsed arange. |

## Structs

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:10-12`](../../../../.source_1765210505/gimli-0.32.3/src/read/aranges.rs#L10-L12)*

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- <span id="debugaranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAranges<R>`

- <span id="debugaranges-clone"></span>`fn clone(&self) -> DebugAranges<R>` — [`DebugAranges`](../index.md#debugaranges)

##### `impl<R: marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: fmt::Debug> Debug for DebugAranges<R>`

- <span id="debugaranges-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAranges<R>`

- <span id="debugaranges-default"></span>`fn default() -> DebugAranges<R>` — [`DebugAranges`](../index.md#debugaranges)

##### `impl<R> Section for DebugAranges<R>`

- <span id="debugaranges-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugaranges-reader"></span>`fn reader(&self) -> &R`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:91-94`](../../../../.source_1765210505/gimli-0.32.3/src/read/aranges.rs#L91-L94)*

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- <span id="arangeheaderiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../../index.md#result), [`ArangeHeader`](../index.md#arangeheader)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clone"></span>`fn clone(&self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](../index.md#arangeheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ArangeHeader<R, Offset>`

```rust
struct ArangeHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugArangesOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    debug_info_offset: crate::common::DebugInfoOffset<Offset>,
    entries: R,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:131-141`](../../../../.source_1765210505/gimli-0.32.3/src/read/aranges.rs#L131-L141)*

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- <span id="arangeheader-parse"></span>`fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../../index.md#debugarangesoffset), [`Result`](../../index.md#result)

- <span id="arangeheader-offset"></span>`fn offset(&self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../../index.md#debugarangesoffset)

- <span id="arangeheader-length"></span>`fn length(&self) -> Offset`

- <span id="arangeheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

- <span id="arangeheader-debug-info-offset"></span>`fn debug_info_offset(&self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../../index.md#debuginfooffset)

- <span id="arangeheader-entries"></span>`fn entries(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md#arangeentryiter)

#### Trait Implementations

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- <span id="arangeheader-clone"></span>`fn clone(&self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](../index.md#arangeheader)

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- <span id="arangeheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- <span id="arangeheader-eq"></span>`fn eq(&self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](../index.md#arangeheader)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:239-242`](../../../../.source_1765210505/gimli-0.32.3/src/read/aranges.rs#L239-L242)*

An iterator over the aranges from a `.debug_aranges` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="arangeentryiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md#result), [`ArangeEntry`](../index.md#arangeentry)

- <span id="arangeentryiter-next-raw"></span>`fn next_raw(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md#result), [`ArangeEntry`](../index.md#arangeentry)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- <span id="arangeentryiter-clone"></span>`fn clone(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md#arangeentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- <span id="arangeentryiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:318-321`](../../../../.source_1765210505/gimli-0.32.3/src/read/aranges.rs#L318-L321)*

A single parsed arange.

#### Implementations

- <span id="arangeentry-parse"></span>`fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../../index.md#encoding), [`Result`](../../index.md#result)

- <span id="arangeentry-address"></span>`fn address(&self) -> u64`

- <span id="arangeentry-length"></span>`fn length(&self) -> u64`

- <span id="arangeentry-range"></span>`fn range(&self) -> Range` — [`Range`](../index.md#range)

#### Trait Implementations

##### `impl Clone for ArangeEntry`

- <span id="arangeentry-clone"></span>`fn clone(&self) -> ArangeEntry` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl Debug for ArangeEntry`

- <span id="arangeentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl Ord for ArangeEntry`

- <span id="arangeentry-cmp"></span>`fn cmp(&self, other: &ArangeEntry) -> cmp::Ordering` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl PartialEq for ArangeEntry`

- <span id="arangeentry-eq"></span>`fn eq(&self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl PartialOrd for ArangeEntry`

- <span id="arangeentry-partial-cmp"></span>`fn partial_cmp(&self, other: &ArangeEntry) -> option::Option<cmp::Ordering>` — [`ArangeEntry`](../index.md#arangeentry)

##### `impl StructuralPartialEq for ArangeEntry`

