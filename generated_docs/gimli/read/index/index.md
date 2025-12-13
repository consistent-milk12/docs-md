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

*Defined in [`gimli-0.32.3/src/read/index.rs:12-14`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L12-L14)*

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

#### Implementations

- <span id="debugcuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugCuIndex` instance from the data in the `.debug_cu_index`

  section.

#### Trait Implementations

##### `impl Any for DebugCuIndex<R>`

- <span id="debugcuindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugCuIndex<R>`

- <span id="debugcuindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugCuIndex<R>`

- <span id="debugcuindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugCuIndex<R>`

- <span id="debugcuindex-clone"></span>`fn clone(&self) -> DebugCuIndex<R>` — [`DebugCuIndex`](../index.md#debugcuindex)

##### `impl CloneToUninit for DebugCuIndex<R>`

- <span id="debugcuindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugCuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugCuIndex<R>`

- <span id="debugcuindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugCuIndex<R>`

- <span id="debugcuindex-default"></span>`fn default() -> DebugCuIndex<R>` — [`DebugCuIndex`](../index.md#debugcuindex)

##### `impl<T> From for DebugCuIndex<R>`

- <span id="debugcuindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugCuIndex<R>`

- <span id="debugcuindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugCuIndex<R>`

- <span id="debugcuindex-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugcuindex-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugCuIndex<R>`

- <span id="debugcuindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugcuindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugcuindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugCuIndex<R>`

- <span id="debugcuindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugcuindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugCuIndex<R>`

- <span id="debugcuindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugcuindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugTuIndex<R>`

```rust
struct DebugTuIndex<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:68-70`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L68-L70)*

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

#### Implementations

- <span id="debugtuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugTuIndex` instance from the data in the `.debug_tu_index`

  section.

#### Trait Implementations

##### `impl Any for DebugTuIndex<R>`

- <span id="debugtuindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTuIndex<R>`

- <span id="debugtuindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTuIndex<R>`

- <span id="debugtuindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugTuIndex<R>`

- <span id="debugtuindex-clone"></span>`fn clone(&self) -> DebugTuIndex<R>` — [`DebugTuIndex`](../index.md#debugtuindex)

##### `impl CloneToUninit for DebugTuIndex<R>`

- <span id="debugtuindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugTuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugTuIndex<R>`

- <span id="debugtuindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTuIndex<R>`

- <span id="debugtuindex-default"></span>`fn default() -> DebugTuIndex<R>` — [`DebugTuIndex`](../index.md#debugtuindex)

##### `impl<T> From for DebugTuIndex<R>`

- <span id="debugtuindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugTuIndex<R>`

- <span id="debugtuindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugTuIndex<R>`

- <span id="debugtuindex-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugtuindex-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugTuIndex<R>`

- <span id="debugtuindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugtuindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugtuindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugTuIndex<R>`

- <span id="debugtuindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugtuindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugTuIndex<R>`

- <span id="debugtuindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugtuindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`gimli-0.32.3/src/read/index.rs:124-135`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L124-L135)*

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

#### Implementations

- <span id="unitindex-parse"></span>`fn parse(input: R) -> Result<UnitIndex<R>>` — [`Result`](../../index.md#result), [`UnitIndex`](../index.md#unitindex)

- <span id="unitindex-find"></span>`fn find(&self, id: u64) -> Option<u32>`

  Find `id` in the index hash table, and return the row index.

  

  `id` may be a compilation unit ID if this index is from `.debug_cu_index`,

  or a type signature if this index is from `.debug_tu_index`.

- <span id="unitindex-sections"></span>`fn sections(&self, row: u32) -> Result<UnitIndexSectionIterator<'_, R>>` — [`Result`](../../index.md#result), [`UnitIndexSectionIterator`](../index.md#unitindexsectioniterator)

  Return the section offsets and sizes for the given row index.

- <span id="unitindex-version"></span>`fn version(&self) -> u16`

  Return the version.

  

  Defaults to 0 for empty sections.

- <span id="unitindex-section-count"></span>`fn section_count(&self) -> u32`

  Return the number of sections.

- <span id="unitindex-unit-count"></span>`fn unit_count(&self) -> u32`

  Return the number of units.

- <span id="unitindex-slot-count"></span>`fn slot_count(&self) -> u32`

  Return the number of slots.

#### Trait Implementations

##### `impl Any for UnitIndex<R>`

- <span id="unitindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitIndex<R>`

- <span id="unitindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitIndex<R>`

- <span id="unitindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for UnitIndex<R>`

- <span id="unitindex-clone"></span>`fn clone(&self) -> UnitIndex<R>` — [`UnitIndex`](../index.md#unitindex)

##### `impl CloneToUninit for UnitIndex<R>`

- <span id="unitindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndex<R>`

- <span id="unitindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnitIndex<R>`

- <span id="unitindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitIndex<R>`

- <span id="unitindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for UnitIndex<R>`

- <span id="unitindex-toowned-type-owned"></span>`type Owned = T`

- <span id="unitindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitIndex<R>`

- <span id="unitindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitIndex<R>`

- <span id="unitindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitIndexSectionIterator<'index, R: Reader>`

```rust
struct UnitIndexSectionIterator<'index, R: Reader> {
    sections: slice::Iter<'index, IndexSectionId>,
    offsets: R,
    sizes: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:307-311`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L307-L311)*

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

#### Trait Implementations

##### `impl Any for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-clone"></span>`fn clone(&self) -> UnitIndexSectionIterator<'index, R>` — [`UnitIndexSectionIterator`](../index.md#unitindexsectioniterator)

##### `impl CloneToUninit for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unitindexsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unitindexsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-iterator-type-item"></span>`type Item = UnitIndexSection`

- <span id="unitindexsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<UnitIndexSection>` — [`UnitIndexSection`](../index.md#unitindexsection)

##### `impl ToOwned for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-toowned-type-owned"></span>`type Owned = T`

- <span id="unitindexsectioniterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitindexsectioniterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitindexsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitindexsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitIndexSection`

```rust
struct UnitIndexSection {
    pub section: IndexSectionId,
    pub offset: u32,
    pub size: u32,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:331-338`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L331-L338)*

Information about a unit's contribution to a section in a `.dwp` file.

#### Fields

- **`section`**: `IndexSectionId`

  The section kind.

- **`offset`**: `u32`

  The base offset of the unit's contribution to the section.

- **`size`**: `u32`

  The size of the unit's contribution to the section.

#### Trait Implementations

##### `impl Any for UnitIndexSection`

- <span id="unitindexsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitIndexSection`

- <span id="unitindexsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitIndexSection`

- <span id="unitindexsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for UnitIndexSection`

- <span id="unitindexsection-clone"></span>`fn clone(&self) -> UnitIndexSection` — [`UnitIndexSection`](../index.md#unitindexsection)

##### `impl CloneToUninit for UnitIndexSection`

- <span id="unitindexsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for UnitIndexSection`

##### `impl Debug for UnitIndexSection`

- <span id="unitindexsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnitIndexSection`

##### `impl<T> From for UnitIndexSection`

- <span id="unitindexsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitIndexSection`

- <span id="unitindexsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for UnitIndexSection`

- <span id="unitindexsection-partialeq-eq"></span>`fn eq(&self, other: &UnitIndexSection) -> bool` — [`UnitIndexSection`](../index.md#unitindexsection)

##### `impl StructuralPartialEq for UnitIndexSection`

##### `impl ToOwned for UnitIndexSection`

- <span id="unitindexsection-toowned-type-owned"></span>`type Owned = T`

- <span id="unitindexsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitindexsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitIndexSection`

- <span id="unitindexsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitindexsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitIndexSection`

- <span id="unitindexsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitindexsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`gimli-0.32.3/src/read/index.rs:342-363`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L342-L363)*

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

  Returns the corresponding `SectionId`.

- <span id="indexsectionid-dwo-name"></span>`fn dwo_name(self) -> &'static str`

  Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

#### Trait Implementations

##### `impl Any for IndexSectionId`

- <span id="indexsectionid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IndexSectionId`

- <span id="indexsectionid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IndexSectionId`

- <span id="indexsectionid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for IndexSectionId`

- <span id="indexsectionid-clone"></span>`fn clone(&self) -> IndexSectionId` — [`IndexSectionId`](../index.md#indexsectionid)

##### `impl CloneToUninit for IndexSectionId`

- <span id="indexsectionid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for IndexSectionId`

##### `impl Debug for IndexSectionId`

- <span id="indexsectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IndexSectionId`

##### `impl<T> From for IndexSectionId`

- <span id="indexsectionid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IndexSectionId`

- <span id="indexsectionid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for IndexSectionId`

- <span id="indexsectionid-partialeq-eq"></span>`fn eq(&self, other: &IndexSectionId) -> bool` — [`IndexSectionId`](../index.md#indexsectionid)

##### `impl StructuralPartialEq for IndexSectionId`

##### `impl ToOwned for IndexSectionId`

- <span id="indexsectionid-toowned-type-owned"></span>`type Owned = T`

- <span id="indexsectionid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="indexsectionid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IndexSectionId`

- <span id="indexsectionid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="indexsectionid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IndexSectionId`

- <span id="indexsectionid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="indexsectionid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `SECTION_COUNT_MAX`
```rust
const SECTION_COUNT_MAX: u8 = 8u8;
```

*Defined in [`gimli-0.32.3/src/read/index.rs:120`](../../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L120)*

