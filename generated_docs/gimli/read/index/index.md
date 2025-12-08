*[gimli](../../index.md) / [read](../index.md) / [index](index.md)*

---

# Module `index`

## Structs

### `DebugCuIndex<R>`

```rust
struct DebugCuIndex<R> {
    section: R,
}
```

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

#### Implementations

- `fn index(self: Self) -> Result<UnitIndex<R>>` — [`Result`](../../index.md), [`UnitIndex`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugCuIndex<R>`

- `fn clone(self: &Self) -> DebugCuIndex<R>` — [`DebugCuIndex`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugCuIndex<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugCuIndex<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugCuIndex<R>`

- `fn default() -> DebugCuIndex<R>` — [`DebugCuIndex`](../index.md)

##### `impl<R> Section for DebugCuIndex<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugTuIndex<R>`

```rust
struct DebugTuIndex<R> {
    section: R,
}
```

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

#### Implementations

- `fn index(self: Self) -> Result<UnitIndex<R>>` — [`Result`](../../index.md), [`UnitIndex`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugTuIndex<R>`

- `fn clone(self: &Self) -> DebugTuIndex<R>` — [`DebugTuIndex`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugTuIndex<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugTuIndex<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugTuIndex<R>`

- `fn default() -> DebugTuIndex<R>` — [`DebugTuIndex`](../index.md)

##### `impl<R> Section for DebugTuIndex<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

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

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

#### Implementations

- `fn parse(input: R) -> Result<UnitIndex<R>>` — [`Result`](../../index.md), [`UnitIndex`](../index.md)

- `fn find(self: &Self, id: u64) -> Option<u32>`

- `fn sections(self: &Self, row: u32) -> Result<UnitIndexSectionIterator<'_, R>>` — [`Result`](../../index.md), [`UnitIndexSectionIterator`](../index.md)

- `fn version(self: &Self) -> u16`

- `fn section_count(self: &Self) -> u32`

- `fn unit_count(self: &Self) -> u32`

- `fn slot_count(self: &Self) -> u32`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for UnitIndex<R>`

- `fn clone(self: &Self) -> UnitIndex<R>` — [`UnitIndex`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for UnitIndex<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UnitIndexSectionIterator<'index, R: Reader>`

```rust
struct UnitIndexSectionIterator<'index, R: Reader> {
    sections: slice::Iter<'index, IndexSectionId>,
    offsets: R,
    sizes: R,
}
```

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

#### Trait Implementations

##### `impl<'index, R: $crate::clone::Clone + Reader> Clone for UnitIndexSectionIterator<'index, R>`

- `fn clone(self: &Self) -> UnitIndexSectionIterator<'index, R>` — [`UnitIndexSectionIterator`](../index.md)

##### `impl<'index, R: $crate::fmt::Debug + Reader> Debug for UnitIndexSectionIterator<'index, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for UnitIndexSectionIterator<'index, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'index, R: Reader> Iterator for UnitIndexSectionIterator<'index, R>`

- `type Item = UnitIndexSection`

- `fn next(self: &mut Self) -> Option<UnitIndexSection>` — [`UnitIndexSection`](../index.md)

### `UnitIndexSection`

```rust
struct UnitIndexSection {
    pub section: IndexSectionId,
    pub offset: u32,
    pub size: u32,
}
```

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

- `fn clone(self: &Self) -> UnitIndexSection` — [`UnitIndexSection`](../index.md)

##### `impl Copy for UnitIndexSection`

##### `impl Debug for UnitIndexSection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for UnitIndexSection`

##### `impl PartialEq for UnitIndexSection`

- `fn eq(self: &Self, other: &UnitIndexSection) -> bool` — [`UnitIndexSection`](../index.md)

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

- `fn section_id(self: Self) -> SectionId` — [`SectionId`](../../index.md)

- `fn dwo_name(self: Self) -> &'static str`

#### Trait Implementations

##### `impl Clone for IndexSectionId`

- `fn clone(self: &Self) -> IndexSectionId` — [`IndexSectionId`](../index.md)

##### `impl Copy for IndexSectionId`

##### `impl Debug for IndexSectionId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for IndexSectionId`

##### `impl PartialEq for IndexSectionId`

- `fn eq(self: &Self, other: &IndexSectionId) -> bool` — [`IndexSectionId`](../index.md)

##### `impl StructuralPartialEq for IndexSectionId`

## Constants

### `SECTION_COUNT_MAX`

```rust
const SECTION_COUNT_MAX: u8 = 8u8;
```

