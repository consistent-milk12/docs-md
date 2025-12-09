*[gimli](../../index.md) / [read](../index.md) / [dwarf](index.md)*

---

# Module `dwarf`

## Contents

- [Structs](#structs)
  - [`DwarfSections`](#dwarfsections)
  - [`Dwarf`](#dwarf)
  - [`DwarfPackageSections`](#dwarfpackagesections)
  - [`DwarfPackage`](#dwarfpackage)
  - [`Unit`](#unit)
  - [`UnitRef`](#unitref)
  - [`RangeIter`](#rangeiter)
- [Enums](#enums)
  - [`RangeIterInner`](#rangeiterinner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DwarfSections`](#dwarfsections) | struct | All of the commonly used DWARF sections. |
| [`Dwarf`](#dwarf) | struct | All of the commonly used DWARF sections, and other common information. |
| [`DwarfPackageSections`](#dwarfpackagesections) | struct | The sections from a `.dwp` file. |
| [`DwarfPackage`](#dwarfpackage) | struct | The sections from a `.dwp` file, with parsed indices. |
| [`Unit`](#unit) | struct | All of the commonly used information for a unit in the `.debug_info` or `.debug_types` |
| [`UnitRef`](#unitref) | struct | A reference to a `Unit` and its associated `Dwarf`. |
| [`RangeIter`](#rangeiter) | struct | An iterator for the address ranges of a `DebuggingInformationEntry`. |
| [`RangeIterInner`](#rangeiterinner) | enum |  |

## Structs

### `DwarfSections<T>`

```rust
struct DwarfSections<T> {
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_addr: crate::read::DebugAddr<T>,
    pub debug_aranges: crate::read::DebugAranges<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_line_str: crate::read::DebugLineStr<T>,
    pub debug_macinfo: crate::read::DebugMacinfo<T>,
    pub debug_macro: crate::read::DebugMacro<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_types: crate::read::DebugTypes<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_ranges: crate::read::DebugRanges<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
}
```

All of the commonly used DWARF sections.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `Dwarf` that references the data in `self`.
If `T` does implement `Reader`, then use `Dwarf` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;
// Create references to the DWARF sections.
let dwarf: gimli::Dwarf<_> = dwarf_sections.borrow(|section| {
    gimli::EndianSlice::new(&section, gimli::LittleEndian)
});
unreachable!()
}
```

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<T>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<T>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<T>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<T>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<T>`

  The `.debug_macro` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc` section.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists` section.

- **`debug_ranges`**: `crate::read::DebugRanges<T>`

  The `.debug_ranges` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists` section.

#### Implementations

- <span id="dwarfsections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

- <span id="dwarfsections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](../index.md)

- <span id="dwarfsections-borrow-with-sup"></span>`fn borrow_with_sup<'a, F, R>(self: &'a Self, sup: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](../index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for DwarfSections<T>`

- <span id="dwarfsections-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfSections<T>`

- <span id="dwarfsections-default"></span>`fn default() -> DwarfSections<T>` — [`DwarfSections`](../index.md)

### `Dwarf<R>`

```rust
struct Dwarf<R> {
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_addr: crate::read::DebugAddr<R>,
    pub debug_aranges: crate::read::DebugAranges<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_line_str: crate::read::DebugLineStr<R>,
    pub debug_macinfo: crate::read::DebugMacinfo<R>,
    pub debug_macro: crate::read::DebugMacro<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub locations: crate::read::LocationLists<R>,
    pub ranges: crate::read::RangeLists<R>,
    pub file_type: crate::common::DwarfFileType,
    pub sup: Option<alloc::sync::Arc<Dwarf<R>>>,
    pub abbreviations_cache: crate::read::AbbreviationsCache,
}
```

All of the commonly used DWARF sections, and other common information.

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<R>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<R>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<R>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<R>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<R>`

  The `.debug_macro` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types` section.

- **`locations`**: `crate::read::LocationLists<R>`

  The location lists in the `.debug_loc` and `.debug_loclists` sections.

- **`ranges`**: `crate::read::RangeLists<R>`

  The range lists in the `.debug_ranges` and `.debug_rnglists` sections.

- **`file_type`**: `crate::common::DwarfFileType`

  The type of this file.

- **`sup`**: `Option<alloc::sync::Arc<Dwarf<R>>>`

  The DWARF sections for a supplementary object file.

- **`abbreviations_cache`**: `crate::read::AbbreviationsCache`

  A cache of previously parsed abbreviations for units in this file.

#### Implementations

- <span id="dwarf-make-dwo"></span>`fn make_dwo(&mut self, parent: &Dwarf<R>)` — [`Dwarf`](../index.md)

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for Dwarf<R>`

- <span id="dwarf-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for Dwarf<R>`

- <span id="dwarf-default"></span>`fn default() -> Dwarf<R>` — [`Dwarf`](../index.md)

### `DwarfPackageSections<T>`

```rust
struct DwarfPackageSections<T> {
    pub cu_index: crate::read::DebugCuIndex<T>,
    pub tu_index: crate::read::DebugTuIndex<T>,
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
    pub debug_types: crate::read::DebugTypes<T>,
}
```

The sections from a `.dwp` file.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `DwarfPackage` that references the data in `self`.
If `T` does implement `Reader`, then use `DwarfPackage` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwp_sections: gimli::DwarfPackageSections<Vec<u8>> = gimli::DwarfPackageSections::load(loader)?;
// Create references to the DWARF sections.
let dwp: gimli::DwarfPackage<_> = dwp_sections.borrow(
    |section| gimli::EndianSlice::new(&section, gimli::LittleEndian),
    gimli::EndianSlice::new(&[], gimli::LittleEndian),
)?;
unreachable!()
}
```

#### Fields

- **`cu_index`**: `crate::read::DebugCuIndex<T>`

  The `.debug_cu_index` section.

- **`tu_index`**: `crate::read::DebugTuIndex<T>`

  The `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

#### Implementations

- <span id="dwarfpackagesections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

- <span id="dwarfpackagesections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F, empty: R) -> Result<DwarfPackage<R>>` — [`Result`](../../index.md), [`DwarfPackage`](../index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-default"></span>`fn default() -> DwarfPackageSections<T>` — [`DwarfPackageSections`](../index.md)

### `DwarfPackage<R: Reader>`

```rust
struct DwarfPackage<R: Reader> {
    pub cu_index: crate::read::UnitIndex<R>,
    pub tu_index: crate::read::UnitIndex<R>,
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_loc: crate::read::DebugLoc<R>,
    pub debug_loclists: crate::read::DebugLocLists<R>,
    pub debug_rnglists: crate::read::DebugRngLists<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub empty: R,
}
```

The sections from a `.dwp` file, with parsed indices.

#### Fields

- **`cu_index`**: `crate::read::UnitIndex<R>`

  The compilation unit index in the `.debug_cu_index` section.

- **`tu_index`**: `crate::read::UnitIndex<R>`

  The type unit index in the `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<R>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<R>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<R>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`empty`**: `R`

  An empty section.
  
  Used when creating `Dwarf<R>`.

#### Implementations

- <span id="dwarfpackage-load"></span>`fn load<F, E>(section: F, empty: R) -> core::result::Result<Self, E>`

- <span id="dwarfpackage-from-sections"></span>`fn from_sections(sections: DwarfPackageSections<R>, empty: R) -> Result<Self>` — [`DwarfPackageSections`](../index.md), [`Result`](../../index.md)

- <span id="dwarfpackage-find-cu"></span>`fn find_cu(&self, id: DwoId, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DwoId`](../../index.md), [`Dwarf`](../index.md), [`Result`](../../index.md)

- <span id="dwarfpackage-find-tu"></span>`fn find_tu(&self, signature: DebugTypeSignature, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DebugTypeSignature`](../../index.md), [`Dwarf`](../index.md), [`Result`](../../index.md)

- <span id="dwarfpackage-cu-sections"></span>`fn cu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](../index.md), [`Result`](../../index.md)

- <span id="dwarfpackage-tu-sections"></span>`fn tu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](../index.md), [`Result`](../../index.md)

- <span id="dwarfpackage-sections"></span>`fn sections(&self, sections: UnitIndexSectionIterator<'_, R>, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`UnitIndexSectionIterator`](../index.md), [`Dwarf`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for DwarfPackage<R>`

- <span id="dwarfpackage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Unit<R, Offset>`

```rust
struct Unit<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub header: crate::read::UnitHeader<R, Offset>,
    pub abbreviations: alloc::sync::Arc<crate::read::Abbreviations>,
    pub name: Option<R>,
    pub comp_dir: Option<R>,
    pub low_pc: u64,
    pub str_offsets_base: crate::common::DebugStrOffsetsBase<Offset>,
    pub addr_base: crate::common::DebugAddrBase<Offset>,
    pub loclists_base: crate::common::DebugLocListsBase<Offset>,
    pub rnglists_base: crate::common::DebugRngListsBase<Offset>,
    pub line_program: Option<crate::read::IncompleteLineProgram<R, Offset>>,
    pub dwo_id: Option<crate::common::DwoId>,
}
```

All of the commonly used information for a unit in the `.debug_info` or `.debug_types`
sections.

#### Fields

- **`header`**: `crate::read::UnitHeader<R, Offset>`

  The header of the unit.

- **`abbreviations`**: `alloc::sync::Arc<crate::read::Abbreviations>`

  The parsed abbreviations for the unit.

- **`name`**: `Option<R>`

  The `DW_AT_name` attribute of the unit.

- **`comp_dir`**: `Option<R>`

  The `DW_AT_comp_dir` attribute of the unit.

- **`low_pc`**: `u64`

  The `DW_AT_low_pc` attribute of the unit. Defaults to 0.

- **`str_offsets_base`**: `crate::common::DebugStrOffsetsBase<Offset>`

  The `DW_AT_str_offsets_base` attribute of the unit. Defaults to 0.

- **`addr_base`**: `crate::common::DebugAddrBase<Offset>`

  The `DW_AT_addr_base` attribute of the unit. Defaults to 0.

- **`loclists_base`**: `crate::common::DebugLocListsBase<Offset>`

  The `DW_AT_loclists_base` attribute of the unit. Defaults to 0.

- **`rnglists_base`**: `crate::common::DebugRngListsBase<Offset>`

  The `DW_AT_rnglists_base` attribute of the unit. Defaults to 0.

- **`line_program`**: `Option<crate::read::IncompleteLineProgram<R, Offset>>`

  The line number program of the unit.

- **`dwo_id`**: `Option<crate::common::DwoId>`

  The DWO ID of a skeleton unit or split compilation unit.

#### Implementations

- <span id="unit-new"></span>`fn new(dwarf: &Dwarf<R>, header: UnitHeader<R>) -> Result<Self>` — [`Dwarf`](../index.md), [`UnitHeader`](../index.md), [`Result`](../../index.md)

- <span id="unit-new-with-abbreviations"></span>`fn new_with_abbreviations(dwarf: &Dwarf<R>, header: UnitHeader<R>, abbreviations: Arc<Abbreviations>) -> Result<Self>` — [`Dwarf`](../index.md), [`UnitHeader`](../index.md), [`Abbreviations`](../index.md), [`Result`](../../index.md)

- <span id="unit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, dwarf: &'a Dwarf<R>) -> UnitRef<'a, R>` — [`Dwarf`](../index.md), [`UnitRef`](../index.md)

- <span id="unit-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md)

- <span id="unit-entry"></span>`fn entry(&self, offset: UnitOffset<<R as >::Offset>) -> Result<DebuggingInformationEntry<'_, '_, R>>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`DebuggingInformationEntry`](../index.md)

- <span id="unit-entries"></span>`fn entries(&self) -> EntriesCursor<'_, '_, R>` — [`EntriesCursor`](../index.md)

- <span id="unit-entries-at-offset"></span>`fn entries_at_offset(&self, offset: UnitOffset<<R as >::Offset>) -> Result<EntriesCursor<'_, '_, R>>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`EntriesCursor`](../index.md)

- <span id="unit-entries-tree"></span>`fn entries_tree(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesTree<'_, '_, R>>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`EntriesTree`](../index.md)

- <span id="unit-entries-raw"></span>`fn entries_raw(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesRaw<'_, '_, R>>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`EntriesRaw`](../index.md)

- <span id="unit-copy-relocated-attributes"></span>`fn copy_relocated_attributes(&mut self, other: &Unit<R>)` — [`Unit`](../index.md)

- <span id="unit-dwo-name"></span>`fn dwo_name(&self) -> Result<Option<AttributeValue<R>>>` — [`Result`](../../index.md), [`AttributeValue`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Debug for Unit<R, Offset>`

- <span id="unit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnitRef<'a, R: Reader>`

```rust
struct UnitRef<'a, R: Reader> {
    pub dwarf: &'a Dwarf<R>,
    pub unit: &'a Unit<R>,
}
```

A reference to a `Unit` and its associated `Dwarf`.

These often need to be passed around together, so this struct makes that easier.

It implements `Deref` to `Unit`, so you can use it as if it were a `Unit`.
It also implements methods that correspond to methods on `Dwarf` that take a `Unit`.

#### Fields

- **`dwarf`**: `&'a Dwarf<R>`

  The `Dwarf` that contains the unit.

- **`unit`**: `&'a Unit<R>`

  The `Unit` being referenced.

#### Implementations

- <span id="unitref-new"></span>`fn new(dwarf: &'a Dwarf<R>, unit: &'a Unit<R>) -> Self` — [`Dwarf`](../index.md), [`Unit`](../index.md)

- <span id="unitref-string-offset"></span>`fn string_offset(&self, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`DebugStrOffsetsIndex`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`DebugStrOffset`](../../index.md)

- <span id="unitref-string"></span>`fn string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

- <span id="unitref-line-string"></span>`fn line_string(&self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugLineStrOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

- <span id="unitref-sup-string"></span>`fn sup_string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

- <span id="unitref-attr-string"></span>`fn attr_string(&self, attr: AttributeValue<R>) -> Result<R>` — [`AttributeValue`](../index.md), [`Result`](../../index.md)

- <span id="unitref-address"></span>`fn address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

- <span id="unitref-attr-address"></span>`fn attr_address(&self, attr: AttributeValue<R>) -> Result<Option<u64>>` — [`AttributeValue`](../index.md), [`Result`](../../index.md)

- <span id="unitref-ranges-offset-from-raw"></span>`fn ranges_offset_from_raw(&self, offset: RawRangeListsOffset<<R as >::Offset>) -> RangeListsOffset<<R as >::Offset>` — [`RawRangeListsOffset`](../../index.md), [`Reader`](../index.md), [`RangeListsOffset`](../../index.md)

- <span id="unitref-ranges-offset"></span>`fn ranges_offset(&self, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`DebugRngListsIndex`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`RangeListsOffset`](../../index.md)

- <span id="unitref-ranges"></span>`fn ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`RngListIter`](../index.md)

- <span id="unitref-raw-ranges"></span>`fn raw_ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`RawRngListIter`](../index.md)

- <span id="unitref-attr-ranges-offset"></span>`fn attr_ranges_offset(&self, attr: AttributeValue<R>) -> Result<Option<RangeListsOffset<<R as >::Offset>>>` — [`AttributeValue`](../index.md), [`Result`](../../index.md), [`RangeListsOffset`](../../index.md), [`Reader`](../index.md)

- <span id="unitref-attr-ranges"></span>`fn attr_ranges(&self, attr: AttributeValue<R>) -> Result<Option<RngListIter<R>>>` — [`AttributeValue`](../index.md), [`Result`](../../index.md), [`RngListIter`](../index.md)

- <span id="unitref-die-ranges"></span>`fn die_ranges(&self, entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<RangeIter<R>>` — [`DebuggingInformationEntry`](../index.md), [`Result`](../../index.md), [`RangeIter`](../index.md)

- <span id="unitref-unit-ranges"></span>`fn unit_ranges(&self) -> Result<RangeIter<R>>` — [`Result`](../../index.md), [`RangeIter`](../index.md)

- <span id="unitref-locations-offset"></span>`fn locations_offset(&self, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` — [`DebugLocListsIndex`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`LocationListsOffset`](../../index.md)

- <span id="unitref-locations"></span>`fn locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`LocListIter`](../index.md)

- <span id="unitref-raw-locations"></span>`fn raw_locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`RawLocListIter`](../index.md)

- <span id="unitref-attr-locations-offset"></span>`fn attr_locations_offset(&self, attr: AttributeValue<R>) -> Result<Option<LocationListsOffset<<R as >::Offset>>>` — [`AttributeValue`](../index.md), [`Result`](../../index.md), [`LocationListsOffset`](../../index.md), [`Reader`](../index.md)

- <span id="unitref-attr-locations"></span>`fn attr_locations(&self, attr: AttributeValue<R>) -> Result<Option<LocListIter<R>>>` — [`AttributeValue`](../index.md), [`Result`](../../index.md), [`LocListIter`](../index.md)

- <span id="unitref-macinfo"></span>`fn macinfo(&self, offset: DebugMacinfoOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacinfoOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`MacroIter`](../index.md)

- <span id="unitref-macros"></span>`fn macros(&self, offset: DebugMacroOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacroOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`MacroIter`](../index.md)

#### Trait Implementations

##### `impl<'a, R: Reader> Clone for UnitRef<'a, R>`

- <span id="unitref-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, R: Reader> Copy for UnitRef<'a, R>`

##### `impl<'a, R: fmt::Debug + Reader> Debug for UnitRef<'a, R>`

- <span id="unitref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, R: Reader> Deref for UnitRef<'a, R>`

- <span id="unitref-target"></span>`type Target = Unit<R>`

- <span id="unitref-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for UnitRef<'a, R>`

- <span id="unitref-target"></span>`type Target = T`

### `RangeIter<R: Reader>`

```rust
struct RangeIter<R: Reader>(RangeIterInner<R>);
```

An iterator for the address ranges of a `DebuggingInformationEntry`.

Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`.

#### Implementations

- <span id="rangeiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../../index.md), [`Range`](../index.md)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RangeIter<R>`

- <span id="rangeiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Default for RangeIter<R>`

- <span id="rangeiter-default"></span>`fn default() -> Self`

## Enums

### `RangeIterInner<R: Reader>`

```rust
enum RangeIterInner<R: Reader> {
    Single(Option<crate::read::Range>),
    List(crate::read::RngListIter<R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RangeIterInner<R>`

- <span id="rangeiterinner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

