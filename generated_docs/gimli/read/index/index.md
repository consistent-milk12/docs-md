*[gimli](../../index.md) / [read](../index.md) / [index](index.md)*

---

# Module `index`

## Contents

- [Structs](#structs)
  - [`DebugCuIndex`](#debugcuindex)
  - [`DebugTuIndex`](#debugtuindex)
  - [`UnitIndex`](#unitindex)
  - [`UnitIndexSectionIterator`](#unitindexsectioniterator)
  - [`UnitIndexSection`](#unitindexsection)
- [Enums](#enums)
  - [`IndexSectionId`](#indexsectionid)
- [Constants](#constants)
  - [`SECTION_COUNT_MAX`](#section-count-max)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugCuIndex`](#debugcuindex) | struct | The data in the `.debug_cu_index` section of a `.dwp` file. |
| [`DebugTuIndex`](#debugtuindex) | struct | The data in the `.debug_tu_index` section of a `.dwp` file. |
| [`UnitIndex`](#unitindex) | struct | The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`. |
| [`UnitIndexSectionIterator`](#unitindexsectioniterator) | struct | An iterator over the section offsets and sizes for a row in a `UnitIndex`. |
| [`UnitIndexSection`](#unitindexsection) | struct | Information about a unit's contribution to a section in a `.dwp` file. |
| [`IndexSectionId`](#indexsectionid) | enum | Section kinds which are permitted in a `.dwp` index. |
| [`SECTION_COUNT_MAX`](#section-count-max) | const |  |

## Structs

### `DebugCuIndex<R>`

```rust
struct DebugCuIndex<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:12-14`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L12-L14)*

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

#### Implementations

- <span id="debugcuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugCuIndex<R>`

- <span id="debugcuindex-clone"></span>`fn clone(&self) -> DebugCuIndex<R>` — [`DebugCuIndex`](../index.md#debugcuindex)

##### `impl<R: marker::Copy> Copy for DebugCuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugCuIndex<R>`

- <span id="debugcuindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugCuIndex<R>`

- <span id="debugcuindex-default"></span>`fn default() -> DebugCuIndex<R>` — [`DebugCuIndex`](../index.md#debugcuindex)

##### `impl<R> Section for DebugCuIndex<R>`

- <span id="debugcuindex-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugcuindex-reader"></span>`fn reader(&self) -> &R`

### `DebugTuIndex<R>`

```rust
struct DebugTuIndex<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:68-70`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L68-L70)*

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

#### Implementations

- <span id="debugtuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugTuIndex<R>`

- <span id="debugtuindex-clone"></span>`fn clone(&self) -> DebugTuIndex<R>` — [`DebugTuIndex`](../index.md#debugtuindex)

##### `impl<R: marker::Copy> Copy for DebugTuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugTuIndex<R>`

- <span id="debugtuindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTuIndex<R>`

- <span id="debugtuindex-default"></span>`fn default() -> DebugTuIndex<R>` — [`DebugTuIndex`](../index.md#debugtuindex)

##### `impl<R> Section for DebugTuIndex<R>`

- <span id="debugtuindex-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugtuindex-reader"></span>`fn reader(&self) -> &R`

### `UnitIndex<R: Reader>`

```rust
struct UnitIndex<R: Reader> {
    version: u16,
    section_count: u32,
    unit_count: u32,
    slot_count: u32,
    hash_ids: R,
    hash_rows: R,
    sections: [IndexSectionId; 8],
    offsets: R,
    sizes: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:124-135`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L124-L135)*

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

#### Implementations

- <span id="unitindex-parse"></span>`fn parse(input: R) -> Result<UnitIndex<R>>` — [`Result`](../../index.md#result), [`UnitIndex`](../index.md#unitindex)

- <span id="unitindex-find"></span>`fn find(&self, id: u64) -> Option<u32>`

- <span id="unitindex-sections"></span>`fn sections(&self, row: u32) -> Result<UnitIndexSectionIterator<'_, R>>` — [`Result`](../../index.md#result), [`UnitIndexSectionIterator`](../index.md#unitindexsectioniterator)

- <span id="unitindex-version"></span>`fn version(&self) -> u16`

- <span id="unitindex-section-count"></span>`fn section_count(&self) -> u32`

- <span id="unitindex-unit-count"></span>`fn unit_count(&self) -> u32`

- <span id="unitindex-slot-count"></span>`fn slot_count(&self) -> u32`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for UnitIndex<R>`

- <span id="unitindex-clone"></span>`fn clone(&self) -> UnitIndex<R>` — [`UnitIndex`](../index.md#unitindex)

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndex<R>`

- <span id="unitindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnitIndexSectionIterator<'index, R: Reader>`

```rust
struct UnitIndexSectionIterator<'index, R: Reader> {
    sections: slice::Iter<'index, IndexSectionId>,
    offsets: R,
    sizes: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:307-311`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L307-L311)*

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-clone"></span>`fn clone(&self) -> UnitIndexSectionIterator<'index, R>` — [`UnitIndexSectionIterator`](../index.md#unitindexsectioniterator)

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unitindexsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unitindexsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-iterator-type-item"></span>`type Item = UnitIndexSection`

- <span id="unitindexsectioniterator-next"></span>`fn next(&mut self) -> Option<UnitIndexSection>` — [`UnitIndexSection`](../index.md#unitindexsection)

### `UnitIndexSection`

```rust
struct UnitIndexSection {
    pub section: IndexSectionId,
    pub offset: u32,
    pub size: u32,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:331-338`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L331-L338)*

Information about a unit's contribution to a section in a `.dwp` file.

#### Fields

- **`section`**: `IndexSectionId`

  The section kind.

- **`offset`**: `u32`

  The base offset of the unit's contribution to the section.

- **`size`**: `u32`

  The size of the unit's contribution to the section.

#### Trait Implementations

##### `impl Clone for UnitIndexSection`

- <span id="unitindexsection-clone"></span>`fn clone(&self) -> UnitIndexSection` — [`UnitIndexSection`](../index.md#unitindexsection)

##### `impl Copy for UnitIndexSection`

##### `impl Debug for UnitIndexSection`

- <span id="unitindexsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnitIndexSection`

##### `impl PartialEq for UnitIndexSection`

- <span id="unitindexsection-eq"></span>`fn eq(&self, other: &UnitIndexSection) -> bool` — [`UnitIndexSection`](../index.md#unitindexsection)

##### `impl StructuralPartialEq for UnitIndexSection`

## Enums

### `IndexSectionId`

```rust
enum IndexSectionId {
    DebugAbbrev,
    DebugInfo,
    DebugLine,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugRngLists,
    DebugStrOffsets,
    DebugTypes,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:342-363`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L342-L363)*

Section kinds which are permitted in a `.dwp` index.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev.dwo` section.

- **`DebugInfo`**

  The `.debug_info.dwo` section.

- **`DebugLine`**

  The `.debug_line.dwo` section.

- **`DebugLoc`**

  The `.debug_loc.dwo` section.

- **`DebugLocLists`**

  The `.debug_loclists.dwo` section.

- **`DebugMacinfo`**

  The `.debug_macinfo.dwo` section.

- **`DebugMacro`**

  The `.debug_macro.dwo` section.

- **`DebugRngLists`**

  The `.debug_rnglists.dwo` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets.dwo` section.

- **`DebugTypes`**

  The `.debug_types.dwo` section.

#### Implementations

- <span id="indexsectionid-section-id"></span>`fn section_id(self) -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="indexsectionid-dwo-name"></span>`fn dwo_name(self) -> &'static str`

#### Trait Implementations

##### `impl Clone for IndexSectionId`

- <span id="indexsectionid-clone"></span>`fn clone(&self) -> IndexSectionId` — [`IndexSectionId`](../index.md#indexsectionid)

##### `impl Copy for IndexSectionId`

##### `impl Debug for IndexSectionId`

- <span id="indexsectionid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IndexSectionId`

##### `impl PartialEq for IndexSectionId`

- <span id="indexsectionid-eq"></span>`fn eq(&self, other: &IndexSectionId) -> bool` — [`IndexSectionId`](../index.md#indexsectionid)

##### `impl StructuralPartialEq for IndexSectionId`

## Constants

### `SECTION_COUNT_MAX`
```rust
const SECTION_COUNT_MAX: u8 = 8u8;
```

*Defined in [`gimli-0.32.3/src/read/index.rs:120`](../../../../.source_1765210505/gimli-0.32.3/src/read/index.rs#L120)*

