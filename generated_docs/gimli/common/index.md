*[gimli](../index.md) / [common](index.md)*

---

# Module `common`

## Contents

- [Structs](#structs)
  - [`Encoding`](#encoding)
  - [`LineEncoding`](#lineencoding)
  - [`Register`](#register)
  - [`DebugAbbrevOffset`](#debugabbrevoffset)
  - [`DebugAddrOffset`](#debugaddroffset)
  - [`DebugAddrBase`](#debugaddrbase)
  - [`DebugAddrIndex`](#debugaddrindex)
  - [`DebugArangesOffset`](#debugarangesoffset)
  - [`DebugInfoOffset`](#debuginfooffset)
  - [`DebugLineOffset`](#debuglineoffset)
  - [`DebugLineStrOffset`](#debuglinestroffset)
  - [`LocationListsOffset`](#locationlistsoffset)
  - [`DebugLocListsBase`](#debugloclistsbase)
  - [`DebugLocListsIndex`](#debugloclistsindex)
  - [`DebugMacinfoOffset`](#debugmacinfooffset)
  - [`DebugMacroOffset`](#debugmacrooffset)
  - [`RawRangeListsOffset`](#rawrangelistsoffset)
  - [`RangeListsOffset`](#rangelistsoffset)
  - [`DebugRngListsBase`](#debugrnglistsbase)
  - [`DebugRngListsIndex`](#debugrnglistsindex)
  - [`DebugStrOffset`](#debugstroffset)
  - [`DebugStrOffsetsBase`](#debugstroffsetsbase)
  - [`DebugStrOffsetsIndex`](#debugstroffsetsindex)
  - [`DebugTypesOffset`](#debugtypesoffset)
  - [`DebugTypeSignature`](#debugtypesignature)
  - [`DebugFrameOffset`](#debugframeoffset)
  - [`EhFrameOffset`](#ehframeoffset)
  - [`DwoId`](#dwoid)
- [Enums](#enums)
  - [`Format`](#format)
  - [`Vendor`](#vendor)
  - [`UnitSectionOffset`](#unitsectionoffset)
  - [`SectionId`](#sectionid)
  - [`DwarfFileType`](#dwarffiletype)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Encoding`](#encoding) | struct | Encoding parameters that are commonly used for multiple DWARF sections. |
| [`LineEncoding`](#lineencoding) | struct | Encoding parameters for a line number program. |
| [`Register`](#register) | struct | A DWARF register number. |
| [`DebugAbbrevOffset`](#debugabbrevoffset) | struct | An offset into the `.debug_abbrev` section. |
| [`DebugAddrOffset`](#debugaddroffset) | struct | An offset into the `.debug_addr` section. |
| [`DebugAddrBase`](#debugaddrbase) | struct | An offset to a set of entries in the `.debug_addr` section. |
| [`DebugAddrIndex`](#debugaddrindex) | struct | An index into a set of addresses in the `.debug_addr` section. |
| [`DebugArangesOffset`](#debugarangesoffset) | struct | An offset into the `.debug_aranges` section. |
| [`DebugInfoOffset`](#debuginfooffset) | struct | An offset into the `.debug_info` section. |
| [`DebugLineOffset`](#debuglineoffset) | struct | An offset into the `.debug_line` section. |
| [`DebugLineStrOffset`](#debuglinestroffset) | struct | An offset into the `.debug_line_str` section. |
| [`LocationListsOffset`](#locationlistsoffset) | struct | An offset into either the `.debug_loc` section or the `.debug_loclists` section, depending on the version of the unit the offset was contained in. |
| [`DebugLocListsBase`](#debugloclistsbase) | struct | An offset to a set of location list offsets in the `.debug_loclists` section. |
| [`DebugLocListsIndex`](#debugloclistsindex) | struct | An index into a set of location list offsets in the `.debug_loclists` section. |
| [`DebugMacinfoOffset`](#debugmacinfooffset) | struct | An offset into the `.debug_macinfo` section. |
| [`DebugMacroOffset`](#debugmacrooffset) | struct | An offset into the `.debug_macro` section. |
| [`RawRangeListsOffset`](#rawrangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section, depending on the version of the unit the offset was contained in. |
| [`RangeListsOffset`](#rangelistsoffset) | struct | An offset into either the `.debug_ranges` section or the `.debug_rnglists` section, depending on the version of the unit the offset was contained in. |
| [`DebugRngListsBase`](#debugrnglistsbase) | struct | An offset to a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugRngListsIndex`](#debugrnglistsindex) | struct | An index into a set of range list offsets in the `.debug_rnglists` section. |
| [`DebugStrOffset`](#debugstroffset) | struct | An offset into the `.debug_str` section. |
| [`DebugStrOffsetsBase`](#debugstroffsetsbase) | struct | An offset to a set of entries in the `.debug_str_offsets` section. |
| [`DebugStrOffsetsIndex`](#debugstroffsetsindex) | struct | An index into a set of entries in the `.debug_str_offsets` section. |
| [`DebugTypesOffset`](#debugtypesoffset) | struct | An offset into the `.debug_types` section. |
| [`DebugTypeSignature`](#debugtypesignature) | struct | A type signature as used in the `.debug_types` section. |
| [`DebugFrameOffset`](#debugframeoffset) | struct | An offset into the `.debug_frame` section. |
| [`EhFrameOffset`](#ehframeoffset) | struct | An offset into the `.eh_frame` section. |
| [`DwoId`](#dwoid) | struct | An optionally-provided implementation-defined compilation unit ID to enable split DWARF and linking a split compilation unit back together. |
| [`Format`](#format) | enum | Whether the format of a compilation unit is 32- or 64-bit. |
| [`Vendor`](#vendor) | enum | Which vendor extensions to support. |
| [`UnitSectionOffset`](#unitsectionoffset) | enum | An offset into the `.debug_info` or `.debug_types` sections. |
| [`SectionId`](#sectionid) | enum | An identifier for a DWARF section. |
| [`DwarfFileType`](#dwarffiletype) | enum | The "type" of file with DWARF debugging information. |

## Structs

### `Encoding`

```rust
struct Encoding {
    pub address_size: u8,
    pub format: Format,
    pub version: u16,
}
```

*Defined in [`gimli-0.32.3/src/common.rs:47-56`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L47-L56)*

Encoding parameters that are commonly used for multiple DWARF sections.

This is intended to be small enough to pass by value.

#### Fields

- **`address_size`**: `u8`

  The size of an address.

- **`format`**: `Format`

  Whether the DWARF format is 32- or 64-bit.

- **`version`**: `u16`

  The DWARF version of the header.

#### Trait Implementations

##### `impl Any for Encoding`

- <span id="encoding-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Encoding`

- <span id="encoding-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Encoding`

- <span id="encoding-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

##### `impl CloneToUninit for Encoding`

- <span id="encoding-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- <span id="encoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl<T> From for Encoding`

- <span id="encoding-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Encoding`

- <span id="encoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Encoding`

- <span id="encoding-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Encoding`

- <span id="encoding-partialeq-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](../index.md#encoding)

##### `impl StructuralPartialEq for Encoding`

##### `impl ToOwned for Encoding`

- <span id="encoding-toowned-type-owned"></span>`type Owned = T`

- <span id="encoding-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="encoding-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Encoding`

- <span id="encoding-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="encoding-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Encoding`

- <span id="encoding-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="encoding-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineEncoding`

```rust
struct LineEncoding {
    pub minimum_instruction_length: u8,
    pub maximum_operations_per_instruction: u8,
    pub default_is_stmt: bool,
    pub line_base: i8,
    pub line_range: u8,
}
```

*Defined in [`gimli-0.32.3/src/common.rs:60-76`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L60-L76)*

Encoding parameters for a line number program.

#### Fields

- **`minimum_instruction_length`**: `u8`

  The size in bytes of the smallest target machine instruction.

- **`maximum_operations_per_instruction`**: `u8`

  The maximum number of individual operations that may be encoded in an
  instruction.

- **`default_is_stmt`**: `bool`

  The initial value of the `is_stmt` register.

- **`line_base`**: `i8`

  The minimum value which a special opcode can add to the line register.

- **`line_range`**: `u8`

  The range of values which a special opcode can add to the line register.

#### Trait Implementations

##### `impl Any for LineEncoding`

- <span id="lineencoding-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineEncoding`

- <span id="lineencoding-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineEncoding`

- <span id="lineencoding-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LineEncoding`

- <span id="lineencoding-clone"></span>`fn clone(&self) -> LineEncoding` — [`LineEncoding`](../index.md#lineencoding)

##### `impl CloneToUninit for LineEncoding`

- <span id="lineencoding-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LineEncoding`

##### `impl Debug for LineEncoding`

- <span id="lineencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LineEncoding`

- <span id="lineencoding-default"></span>`fn default() -> Self`

##### `impl Eq for LineEncoding`

##### `impl<T> From for LineEncoding`

- <span id="lineencoding-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LineEncoding`

- <span id="lineencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LineEncoding`

- <span id="lineencoding-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LineEncoding`

- <span id="lineencoding-partialeq-eq"></span>`fn eq(&self, other: &LineEncoding) -> bool` — [`LineEncoding`](../index.md#lineencoding)

##### `impl StructuralPartialEq for LineEncoding`

##### `impl ToOwned for LineEncoding`

- <span id="lineencoding-toowned-type-owned"></span>`type Owned = T`

- <span id="lineencoding-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lineencoding-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineEncoding`

- <span id="lineencoding-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lineencoding-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineEncoding`

- <span id="lineencoding-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lineencoding-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Register`

```rust
struct Register(u16);
```

*Defined in [`gimli-0.32.3/src/common.rs:96`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L96)*

A DWARF register number.

The meaning of this value is ABI dependent. This is generally encoded as
a ULEB128, but supported architectures need 16 bits at most.

#### Implementations

- <span id="cratecommonregister-from-u64"></span>`fn from_u64(x: u64) -> Result<Register>` — [`Result`](../index.md#result), [`Register`](../index.md#register)

#### Trait Implementations

##### `impl Any for Register`

- <span id="register-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Register`

- <span id="register-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Register`

- <span id="register-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Register`

- <span id="register-clone"></span>`fn clone(&self) -> Register` — [`Register`](../index.md#register)

##### `impl CloneToUninit for Register`

- <span id="register-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Register`

##### `impl Debug for Register`

- <span id="register-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Register`

##### `impl<T> From for Register`

- <span id="register-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Register`

- <span id="register-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Register`

- <span id="register-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Register`

- <span id="register-ord-cmp"></span>`fn cmp(&self, other: &Register) -> cmp::Ordering` — [`Register`](../index.md#register)

##### `impl PartialEq for Register`

- <span id="register-partialeq-eq"></span>`fn eq(&self, other: &Register) -> bool` — [`Register`](../index.md#register)

##### `impl PartialOrd for Register`

- <span id="register-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Register) -> option::Option<cmp::Ordering>` — [`Register`](../index.md#register)

##### `impl StructuralPartialEq for Register`

##### `impl ToOwned for Register`

- <span id="register-toowned-type-owned"></span>`type Owned = T`

- <span id="register-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="register-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Register`

- <span id="register-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="register-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Register`

- <span id="register-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="register-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugAbbrevOffset<T>`

```rust
struct DebugAbbrevOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:100`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L100)*

An offset into the `.debug_abbrev` section.

#### Trait Implementations

##### `impl<T> Any for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-clone"></span>`fn clone(&self) -> DebugAbbrevOffset<T>` — [`DebugAbbrevOffset`](../index.md#debugabbrevoffset)

##### `impl<T> CloneToUninit for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugAbbrevOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAbbrevOffset<T>`

##### `impl<T> From for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugAbbrevOffset<T>) -> bool` — [`DebugAbbrevOffset`](../index.md#debugabbrevoffset)

##### `impl<T> StructuralPartialEq for DebugAbbrevOffset<T>`

##### `impl<T> ToOwned for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugabbrevoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugabbrevoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugabbrevoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugAbbrevOffset<T>`

- <span id="debugabbrevoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugabbrevoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugAddrOffset<T>`

```rust
struct DebugAddrOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:104`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L104)*

An offset into the `.debug_addr` section.

#### Trait Implementations

##### `impl<T> Any for DebugAddrOffset<T>`

- <span id="debugaddroffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAddrOffset<T>`

- <span id="debugaddroffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAddrOffset<T>`

- <span id="debugaddroffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugAddrOffset<T>`

- <span id="debugaddroffset-clone"></span>`fn clone(&self) -> DebugAddrOffset<T>` — [`DebugAddrOffset`](../index.md#debugaddroffset)

##### `impl<T> CloneToUninit for DebugAddrOffset<T>`

- <span id="debugaddroffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugAddrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrOffset<T>`

- <span id="debugaddroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrOffset<T>`

##### `impl<T> From for DebugAddrOffset<T>`

- <span id="debugaddroffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugAddrOffset<T>`

- <span id="debugaddroffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrOffset<T>`

- <span id="debugaddroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrOffset<T>) -> bool` — [`DebugAddrOffset`](../index.md#debugaddroffset)

##### `impl<T> StructuralPartialEq for DebugAddrOffset<T>`

##### `impl<T> ToOwned for DebugAddrOffset<T>`

- <span id="debugaddroffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaddroffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaddroffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugAddrOffset<T>`

- <span id="debugaddroffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaddroffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugAddrOffset<T>`

- <span id="debugaddroffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaddroffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugAddrBase<T>`

```rust
struct DebugAddrBase<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:108`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L108)*

An offset to a set of entries in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T> Any for DebugAddrBase<T>`

- <span id="debugaddrbase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAddrBase<T>`

- <span id="debugaddrbase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAddrBase<T>`

- <span id="debugaddrbase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugAddrBase<T>`

- <span id="debugaddrbase-clone"></span>`fn clone(&self) -> DebugAddrBase<T>` — [`DebugAddrBase`](../index.md#debugaddrbase)

##### `impl<T> CloneToUninit for DebugAddrBase<T>`

- <span id="debugaddrbase-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugAddrBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrBase<T>`

- <span id="debugaddrbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrBase<T>`

##### `impl<T> From for DebugAddrBase<T>`

- <span id="debugaddrbase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugAddrBase<T>`

- <span id="debugaddrbase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrBase<T>`

- <span id="debugaddrbase-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrBase<T>) -> bool` — [`DebugAddrBase`](../index.md#debugaddrbase)

##### `impl<T> StructuralPartialEq for DebugAddrBase<T>`

##### `impl<T> ToOwned for DebugAddrBase<T>`

- <span id="debugaddrbase-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaddrbase-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaddrbase-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugAddrBase<T>`

- <span id="debugaddrbase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaddrbase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugAddrBase<T>`

- <span id="debugaddrbase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaddrbase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugAddrIndex<T>`

```rust
struct DebugAddrIndex<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:112`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L112)*

An index into a set of addresses in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T> Any for DebugAddrIndex<T>`

- <span id="debugaddrindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAddrIndex<T>`

- <span id="debugaddrindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAddrIndex<T>`

- <span id="debugaddrindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugAddrIndex<T>`

- <span id="debugaddrindex-clone"></span>`fn clone(&self) -> DebugAddrIndex<T>` — [`DebugAddrIndex`](../index.md#debugaddrindex)

##### `impl<T> CloneToUninit for DebugAddrIndex<T>`

- <span id="debugaddrindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugAddrIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugAddrIndex<T>`

- <span id="debugaddrindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugAddrIndex<T>`

##### `impl<T> From for DebugAddrIndex<T>`

- <span id="debugaddrindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugAddrIndex<T>`

- <span id="debugaddrindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugAddrIndex<T>`

- <span id="debugaddrindex-partialeq-eq"></span>`fn eq(&self, other: &DebugAddrIndex<T>) -> bool` — [`DebugAddrIndex`](../index.md#debugaddrindex)

##### `impl<T> StructuralPartialEq for DebugAddrIndex<T>`

##### `impl<T> ToOwned for DebugAddrIndex<T>`

- <span id="debugaddrindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaddrindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaddrindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugAddrIndex<T>`

- <span id="debugaddrindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaddrindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugAddrIndex<T>`

- <span id="debugaddrindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaddrindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugArangesOffset<T>`

```rust
struct DebugArangesOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:116`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L116)*

An offset into the `.debug_aranges` section.

#### Trait Implementations

##### `impl<T> Any for DebugArangesOffset<T>`

- <span id="debugarangesoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugArangesOffset<T>`

- <span id="debugarangesoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugArangesOffset<T>`

- <span id="debugarangesoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugArangesOffset<T>`

- <span id="debugarangesoffset-clone"></span>`fn clone(&self) -> DebugArangesOffset<T>` — [`DebugArangesOffset`](../index.md#debugarangesoffset)

##### `impl<T> CloneToUninit for DebugArangesOffset<T>`

- <span id="debugarangesoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugArangesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugArangesOffset<T>`

- <span id="debugarangesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugArangesOffset<T>`

##### `impl<T> From for DebugArangesOffset<T>`

- <span id="debugarangesoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugArangesOffset<T>`

- <span id="debugarangesoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugArangesOffset<T>`

- <span id="debugarangesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugArangesOffset<T>) -> bool` — [`DebugArangesOffset`](../index.md#debugarangesoffset)

##### `impl<T> StructuralPartialEq for DebugArangesOffset<T>`

##### `impl<T> ToOwned for DebugArangesOffset<T>`

- <span id="debugarangesoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugarangesoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugarangesoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugArangesOffset<T>`

- <span id="debugarangesoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugarangesoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugArangesOffset<T>`

- <span id="debugarangesoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugarangesoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugInfoOffset<T>`

```rust
struct DebugInfoOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:120`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L120)*

An offset into the `.debug_info` section.

#### Implementations

- <span id="cratecommondebuginfooffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitOffset`](../index.md#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the .debug_info section.

  Returns `None` if the offset is not within this unit entries.

#### Trait Implementations

##### `impl<T> Any for DebugInfoOffset<T>`

- <span id="debuginfooffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugInfoOffset<T>`

- <span id="debuginfooffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugInfoOffset<T>`

- <span id="debuginfooffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugInfoOffset<T>`

- <span id="debuginfooffset-clone"></span>`fn clone(&self) -> DebugInfoOffset<T>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T> CloneToUninit for DebugInfoOffset<T>`

- <span id="debuginfooffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugInfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugInfoOffset<T>`

- <span id="debuginfooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugInfoOffset<T>`

##### `impl<T> From for DebugInfoOffset<T>`

- <span id="debuginfooffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for DebugInfoOffset<T>`

- <span id="debuginfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for DebugInfoOffset<T>`

- <span id="debuginfooffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::Ord> Ord for DebugInfoOffset<T>`

- <span id="debuginfooffset-ord-cmp"></span>`fn cmp(&self, other: &DebugInfoOffset<T>) -> cmp::Ordering` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugInfoOffset<T>`

- <span id="debuginfooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugInfoOffset<T>) -> bool` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugInfoOffset<T>`

- <span id="debuginfooffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugInfoOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl<T> StructuralPartialEq for DebugInfoOffset<T>`

##### `impl<T> ToOwned for DebugInfoOffset<T>`

- <span id="debuginfooffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debuginfooffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuginfooffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugInfoOffset<T>`

- <span id="debuginfooffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuginfooffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugInfoOffset<T>`

- <span id="debuginfooffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuginfooffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLineOffset<T>`

```rust
struct DebugLineOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:124`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L124)*

An offset into the `.debug_line` section.

#### Trait Implementations

##### `impl<T> Any for DebugLineOffset<T>`

- <span id="debuglineoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLineOffset<T>`

- <span id="debuglineoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLineOffset<T>`

- <span id="debuglineoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugLineOffset<T>`

- <span id="debuglineoffset-clone"></span>`fn clone(&self) -> DebugLineOffset<T>` — [`DebugLineOffset`](../index.md#debuglineoffset)

##### `impl<T> CloneToUninit for DebugLineOffset<T>`

- <span id="debuglineoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugLineOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineOffset<T>`

- <span id="debuglineoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineOffset<T>`

##### `impl<T> From for DebugLineOffset<T>`

- <span id="debuglineoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugLineOffset<T>`

- <span id="debuglineoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineOffset<T>`

- <span id="debuglineoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugLineOffset<T>) -> bool` — [`DebugLineOffset`](../index.md#debuglineoffset)

##### `impl<T> StructuralPartialEq for DebugLineOffset<T>`

##### `impl<T> ToOwned for DebugLineOffset<T>`

- <span id="debuglineoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debuglineoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuglineoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugLineOffset<T>`

- <span id="debuglineoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglineoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugLineOffset<T>`

- <span id="debuglineoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglineoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLineStrOffset<T>`

```rust
struct DebugLineStrOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:128`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L128)*

An offset into the `.debug_line_str` section.

#### Trait Implementations

##### `impl<T> Any for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-clone"></span>`fn clone(&self) -> DebugLineStrOffset<T>` — [`DebugLineStrOffset`](../index.md#debuglinestroffset)

##### `impl<T> CloneToUninit for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugLineStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLineStrOffset<T>`

##### `impl<T> From for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugLineStrOffset<T>) -> bool` — [`DebugLineStrOffset`](../index.md#debuglinestroffset)

##### `impl<T> StructuralPartialEq for DebugLineStrOffset<T>`

##### `impl<T> ToOwned for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debuglinestroffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuglinestroffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglinestroffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugLineStrOffset<T>`

- <span id="debuglinestroffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglinestroffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocationListsOffset<T>`

```rust
struct LocationListsOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:133`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L133)*

An offset into either the `.debug_loc` section or the `.debug_loclists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T> Any for LocationListsOffset<T>`

- <span id="locationlistsoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocationListsOffset<T>`

- <span id="locationlistsoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocationListsOffset<T>`

- <span id="locationlistsoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for LocationListsOffset<T>`

- <span id="locationlistsoffset-clone"></span>`fn clone(&self) -> LocationListsOffset<T>` — [`LocationListsOffset`](../index.md#locationlistsoffset)

##### `impl<T> CloneToUninit for LocationListsOffset<T>`

- <span id="locationlistsoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for LocationListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for LocationListsOffset<T>`

- <span id="locationlistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for LocationListsOffset<T>`

##### `impl<T> From for LocationListsOffset<T>`

- <span id="locationlistsoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for LocationListsOffset<T>`

- <span id="locationlistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for LocationListsOffset<T>`

- <span id="locationlistsoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for LocationListsOffset<T>`

- <span id="locationlistsoffset-partialeq-eq"></span>`fn eq(&self, other: &LocationListsOffset<T>) -> bool` — [`LocationListsOffset`](../index.md#locationlistsoffset)

##### `impl<T> StructuralPartialEq for LocationListsOffset<T>`

##### `impl<T> ToOwned for LocationListsOffset<T>`

- <span id="locationlistsoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="locationlistsoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="locationlistsoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for LocationListsOffset<T>`

- <span id="locationlistsoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locationlistsoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for LocationListsOffset<T>`

- <span id="locationlistsoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locationlistsoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLocListsBase<T>`

```rust
struct DebugLocListsBase<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:137`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L137)*

An offset to a set of location list offsets in the `.debug_loclists` section.

#### Implementations

- <span id="cratecommondebugloclistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugLocListsBase<Offset>` — [`Encoding`](../index.md#encoding), [`DwarfFileType`](../index.md#dwarffiletype), [`DebugLocListsBase`](../index.md#debugloclistsbase)

  Returns a `DebugLocListsBase` with the default value of DW_AT_loclists_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T> Any for DebugLocListsBase<T>`

- <span id="debugloclistsbase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLocListsBase<T>`

- <span id="debugloclistsbase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLocListsBase<T>`

- <span id="debugloclistsbase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugLocListsBase<T>`

- <span id="debugloclistsbase-clone"></span>`fn clone(&self) -> DebugLocListsBase<T>` — [`DebugLocListsBase`](../index.md#debugloclistsbase)

##### `impl<T> CloneToUninit for DebugLocListsBase<T>`

- <span id="debugloclistsbase-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugLocListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsBase<T>`

- <span id="debugloclistsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsBase<T>`

##### `impl<T> From for DebugLocListsBase<T>`

- <span id="debugloclistsbase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugLocListsBase<T>`

- <span id="debugloclistsbase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsBase<T>`

- <span id="debugloclistsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugLocListsBase<T>) -> bool` — [`DebugLocListsBase`](../index.md#debugloclistsbase)

##### `impl<T> StructuralPartialEq for DebugLocListsBase<T>`

##### `impl<T> ToOwned for DebugLocListsBase<T>`

- <span id="debugloclistsbase-toowned-type-owned"></span>`type Owned = T`

- <span id="debugloclistsbase-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugloclistsbase-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugLocListsBase<T>`

- <span id="debugloclistsbase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugloclistsbase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugLocListsBase<T>`

- <span id="debugloclistsbase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugloclistsbase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLocListsIndex<T>`

```rust
struct DebugLocListsIndex<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:141`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L141)*

An index into a set of location list offsets in the `.debug_loclists` section.

#### Trait Implementations

##### `impl<T> Any for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-clone"></span>`fn clone(&self) -> DebugLocListsIndex<T>` — [`DebugLocListsIndex`](../index.md#debugloclistsindex)

##### `impl<T> CloneToUninit for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugLocListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugLocListsIndex<T>`

##### `impl<T> From for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugLocListsIndex<T>) -> bool` — [`DebugLocListsIndex`](../index.md#debugloclistsindex)

##### `impl<T> StructuralPartialEq for DebugLocListsIndex<T>`

##### `impl<T> ToOwned for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugloclistsindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugloclistsindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugloclistsindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugLocListsIndex<T>`

- <span id="debugloclistsindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugloclistsindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugMacinfoOffset<T>`

```rust
struct DebugMacinfoOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:145`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L145)*

An offset into the `.debug_macinfo` section.

#### Trait Implementations

##### `impl<T> Any for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-clone"></span>`fn clone(&self) -> DebugMacinfoOffset<T>` — [`DebugMacinfoOffset`](../index.md#debugmacinfooffset)

##### `impl<T> CloneToUninit for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugMacinfoOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacinfoOffset<T>`

##### `impl<T> From for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugMacinfoOffset<T>) -> bool` — [`DebugMacinfoOffset`](../index.md#debugmacinfooffset)

##### `impl<T> StructuralPartialEq for DebugMacinfoOffset<T>`

##### `impl<T> ToOwned for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugmacinfooffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugmacinfooffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugmacinfooffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugMacinfoOffset<T>`

- <span id="debugmacinfooffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugmacinfooffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugMacroOffset<T>`

```rust
struct DebugMacroOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:149`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L149)*

An offset into the `.debug_macro` section.

#### Trait Implementations

##### `impl<T> Any for DebugMacroOffset<T>`

- <span id="debugmacrooffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugMacroOffset<T>`

- <span id="debugmacrooffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugMacroOffset<T>`

- <span id="debugmacrooffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugMacroOffset<T>`

- <span id="debugmacrooffset-clone"></span>`fn clone(&self) -> DebugMacroOffset<T>` — [`DebugMacroOffset`](../index.md#debugmacrooffset)

##### `impl<T> CloneToUninit for DebugMacroOffset<T>`

- <span id="debugmacrooffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugMacroOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugMacroOffset<T>`

- <span id="debugmacrooffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugMacroOffset<T>`

##### `impl<T> From for DebugMacroOffset<T>`

- <span id="debugmacrooffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for DebugMacroOffset<T>`

- <span id="debugmacrooffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for DebugMacroOffset<T>`

- <span id="debugmacrooffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugMacroOffset<T>`

- <span id="debugmacrooffset-partialeq-eq"></span>`fn eq(&self, other: &DebugMacroOffset<T>) -> bool` — [`DebugMacroOffset`](../index.md#debugmacrooffset)

##### `impl<T> StructuralPartialEq for DebugMacroOffset<T>`

##### `impl<T> ToOwned for DebugMacroOffset<T>`

- <span id="debugmacrooffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugmacrooffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugmacrooffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugMacroOffset<T>`

- <span id="debugmacrooffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugmacrooffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugMacroOffset<T>`

- <span id="debugmacrooffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugmacrooffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRangeListsOffset<T>`

```rust
struct RawRangeListsOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:157`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L157)*

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

If this is from a DWARF 4 DWO file, then it must additionally be offset by the
value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this.

#### Trait Implementations

##### `impl<T> Any for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-clone"></span>`fn clone(&self) -> RawRangeListsOffset<T>` — [`RawRangeListsOffset`](../index.md#rawrangelistsoffset)

##### `impl<T> CloneToUninit for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for RawRangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RawRangeListsOffset<T>`

##### `impl<T> From for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-partialeq-eq"></span>`fn eq(&self, other: &RawRangeListsOffset<T>) -> bool` — [`RawRangeListsOffset`](../index.md#rawrangelistsoffset)

##### `impl<T> StructuralPartialEq for RawRangeListsOffset<T>`

##### `impl<T> ToOwned for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="rawrangelistsoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawrangelistsoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrangelistsoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RawRangeListsOffset<T>`

- <span id="rawrangelistsoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrangelistsoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeListsOffset<T>`

```rust
struct RangeListsOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:162`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L162)*

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T> Any for RangeListsOffset<T>`

- <span id="rangelistsoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeListsOffset<T>`

- <span id="rangelistsoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeListsOffset<T>`

- <span id="rangelistsoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RangeListsOffset<T>`

- <span id="rangelistsoffset-clone"></span>`fn clone(&self) -> RangeListsOffset<T>` — [`RangeListsOffset`](../index.md#rangelistsoffset)

##### `impl<T> CloneToUninit for RangeListsOffset<T>`

- <span id="rangelistsoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for RangeListsOffset<T>`

##### `impl<T: fmt::Debug> Debug for RangeListsOffset<T>`

- <span id="rangelistsoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for RangeListsOffset<T>`

##### `impl<T> From for RangeListsOffset<T>`

- <span id="rangelistsoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for RangeListsOffset<T>`

- <span id="rangelistsoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for RangeListsOffset<T>`

- <span id="rangelistsoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for RangeListsOffset<T>`

- <span id="rangelistsoffset-partialeq-eq"></span>`fn eq(&self, other: &RangeListsOffset<T>) -> bool` — [`RangeListsOffset`](../index.md#rangelistsoffset)

##### `impl<T> StructuralPartialEq for RangeListsOffset<T>`

##### `impl<T> ToOwned for RangeListsOffset<T>`

- <span id="rangelistsoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelistsoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelistsoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RangeListsOffset<T>`

- <span id="rangelistsoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelistsoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RangeListsOffset<T>`

- <span id="rangelistsoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelistsoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugRngListsBase<T>`

```rust
struct DebugRngListsBase<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:166`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L166)*

An offset to a set of range list offsets in the `.debug_rnglists` section.

#### Implementations

- <span id="cratecommondebugrnglistsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugRngListsBase<Offset>` — [`Encoding`](../index.md#encoding), [`DwarfFileType`](../index.md#dwarffiletype), [`DebugRngListsBase`](../index.md#debugrnglistsbase)

  Returns a `DebugRngListsBase` with the default value of DW_AT_rnglists_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T> Any for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-clone"></span>`fn clone(&self) -> DebugRngListsBase<T>` — [`DebugRngListsBase`](../index.md#debugrnglistsbase)

##### `impl<T> CloneToUninit for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugRngListsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsBase<T>`

##### `impl<T> From for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugRngListsBase<T>) -> bool` — [`DebugRngListsBase`](../index.md#debugrnglistsbase)

##### `impl<T> StructuralPartialEq for DebugRngListsBase<T>`

##### `impl<T> ToOwned for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-toowned-type-owned"></span>`type Owned = T`

- <span id="debugrnglistsbase-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugrnglistsbase-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugrnglistsbase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugRngListsBase<T>`

- <span id="debugrnglistsbase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugrnglistsbase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugRngListsIndex<T>`

```rust
struct DebugRngListsIndex<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:170`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L170)*

An index into a set of range list offsets in the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<T> Any for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-clone"></span>`fn clone(&self) -> DebugRngListsIndex<T>` — [`DebugRngListsIndex`](../index.md#debugrnglistsindex)

##### `impl<T> CloneToUninit for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugRngListsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugRngListsIndex<T>`

##### `impl<T> From for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugRngListsIndex<T>) -> bool` — [`DebugRngListsIndex`](../index.md#debugrnglistsindex)

##### `impl<T> StructuralPartialEq for DebugRngListsIndex<T>`

##### `impl<T> ToOwned for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugrnglistsindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugrnglistsindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugrnglistsindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugRngListsIndex<T>`

- <span id="debugrnglistsindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugrnglistsindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugStrOffset<T>`

```rust
struct DebugStrOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:174`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L174)*

An offset into the `.debug_str` section.

#### Trait Implementations

##### `impl<T> Any for DebugStrOffset<T>`

- <span id="debugstroffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStrOffset<T>`

- <span id="debugstroffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStrOffset<T>`

- <span id="debugstroffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugStrOffset<T>`

- <span id="debugstroffset-clone"></span>`fn clone(&self) -> DebugStrOffset<T>` — [`DebugStrOffset`](../index.md#debugstroffset)

##### `impl<T> CloneToUninit for DebugStrOffset<T>`

- <span id="debugstroffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugStrOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffset<T>`

- <span id="debugstroffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffset<T>`

##### `impl<T> From for DebugStrOffset<T>`

- <span id="debugstroffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugStrOffset<T>`

- <span id="debugstroffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffset<T>`

- <span id="debugstroffset-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffset<T>) -> bool` — [`DebugStrOffset`](../index.md#debugstroffset)

##### `impl<T> StructuralPartialEq for DebugStrOffset<T>`

##### `impl<T> ToOwned for DebugStrOffset<T>`

- <span id="debugstroffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstroffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstroffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugStrOffset<T>`

- <span id="debugstroffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstroffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugStrOffset<T>`

- <span id="debugstroffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstroffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugStrOffsetsBase<T>`

```rust
struct DebugStrOffsetsBase<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:178`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L178)*

An offset to a set of entries in the `.debug_str_offsets` section.

#### Implementations

- <span id="cratecommondebugstroffsetsbase-default-for-encoding-and-file"></span>`fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugStrOffsetsBase<Offset>` — [`Encoding`](../index.md#encoding), [`DwarfFileType`](../index.md#dwarffiletype), [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase)

  Returns a `DebugStrOffsetsBase` with the default value of DW_AT_str_offsets_base

  for the given `Encoding` and `DwarfFileType`.

#### Trait Implementations

##### `impl<T> Any for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-clone"></span>`fn clone(&self) -> DebugStrOffsetsBase<T>` — [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase)

##### `impl<T> CloneToUninit for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsBase<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsBase<T>`

##### `impl<T> From for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffsetsBase<T>) -> bool` — [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsBase<T>`

##### `impl<T> ToOwned for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstroffsetsbase-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstroffsetsbase-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstroffsetsbase-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugStrOffsetsBase<T>`

- <span id="debugstroffsetsbase-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstroffsetsbase-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugStrOffsetsIndex<T>`

```rust
struct DebugStrOffsetsIndex<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:182`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L182)*

An index into a set of entries in the `.debug_str_offsets` section.

#### Trait Implementations

##### `impl<T> Any for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-clone"></span>`fn clone(&self) -> DebugStrOffsetsIndex<T>` — [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex)

##### `impl<T> CloneToUninit for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugStrOffsetsIndex<T>`

##### `impl<T: fmt::Debug> Debug for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugStrOffsetsIndex<T>`

##### `impl<T> From for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-partialeq-eq"></span>`fn eq(&self, other: &DebugStrOffsetsIndex<T>) -> bool` — [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsIndex<T>`

##### `impl<T> ToOwned for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstroffsetsindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstroffsetsindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstroffsetsindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugStrOffsetsIndex<T>`

- <span id="debugstroffsetsindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstroffsetsindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugTypesOffset<T>`

```rust
struct DebugTypesOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:186`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L186)*

An offset into the `.debug_types` section.

#### Implementations

- <span id="cratecommondebugtypesoffset-to-unit-offset"></span>`fn to_unit_offset<R>(&self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md#unitheader), [`UnitOffset`](../index.md#unitoffset)

  Convert an offset to be relative to the start of the given unit,

  instead of relative to the start of the .debug_types section.

  Returns `None` if the offset is not within the unit entries.

#### Trait Implementations

##### `impl<T> Any for DebugTypesOffset<T>`

- <span id="debugtypesoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTypesOffset<T>`

- <span id="debugtypesoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTypesOffset<T>`

- <span id="debugtypesoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugTypesOffset<T>`

- <span id="debugtypesoffset-clone"></span>`fn clone(&self) -> DebugTypesOffset<T>` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T> CloneToUninit for DebugTypesOffset<T>`

- <span id="debugtypesoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugTypesOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugTypesOffset<T>`

- <span id="debugtypesoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugTypesOffset<T>`

##### `impl<T> From for DebugTypesOffset<T>`

- <span id="debugtypesoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for DebugTypesOffset<T>`

- <span id="debugtypesoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for DebugTypesOffset<T>`

- <span id="debugtypesoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::Ord> Ord for DebugTypesOffset<T>`

- <span id="debugtypesoffset-ord-cmp"></span>`fn cmp(&self, other: &DebugTypesOffset<T>) -> cmp::Ordering` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T: cmp::PartialEq> PartialEq for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugTypesOffset<T>) -> bool` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for DebugTypesOffset<T>`

- <span id="debugtypesoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &DebugTypesOffset<T>) -> option::Option<cmp::Ordering>` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

##### `impl<T> StructuralPartialEq for DebugTypesOffset<T>`

##### `impl<T> ToOwned for DebugTypesOffset<T>`

- <span id="debugtypesoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugtypesoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugtypesoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugTypesOffset<T>`

- <span id="debugtypesoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugtypesoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugTypesOffset<T>`

- <span id="debugtypesoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugtypesoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugTypeSignature`

```rust
struct DebugTypeSignature(u64);
```

*Defined in [`gimli-0.32.3/src/common.rs:190`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L190)*

A type signature as used in the `.debug_types` section.

#### Trait Implementations

##### `impl Any for DebugTypeSignature`

- <span id="debugtypesignature-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTypeSignature`

- <span id="debugtypesignature-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTypeSignature`

- <span id="debugtypesignature-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DebugTypeSignature`

- <span id="debugtypesignature-clone"></span>`fn clone(&self) -> DebugTypeSignature` — [`DebugTypeSignature`](../index.md#debugtypesignature)

##### `impl CloneToUninit for DebugTypeSignature`

- <span id="debugtypesignature-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DebugTypeSignature`

##### `impl Debug for DebugTypeSignature`

- <span id="debugtypesignature-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DebugTypeSignature`

##### `impl<T> From for DebugTypeSignature`

- <span id="debugtypesignature-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DebugTypeSignature`

- <span id="debugtypesignature-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DebugTypeSignature`

- <span id="debugtypesignature-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DebugTypeSignature`

- <span id="debugtypesignature-partialeq-eq"></span>`fn eq(&self, other: &DebugTypeSignature) -> bool` — [`DebugTypeSignature`](../index.md#debugtypesignature)

##### `impl StructuralPartialEq for DebugTypeSignature`

##### `impl ToOwned for DebugTypeSignature`

- <span id="debugtypesignature-toowned-type-owned"></span>`type Owned = T`

- <span id="debugtypesignature-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugtypesignature-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugTypeSignature`

- <span id="debugtypesignature-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugtypesignature-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugTypeSignature`

- <span id="debugtypesignature-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugtypesignature-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugFrameOffset<T>`

```rust
struct DebugFrameOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:194`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L194)*

An offset into the `.debug_frame` section.

#### Trait Implementations

##### `impl<T> Any for DebugFrameOffset<T>`

- <span id="debugframeoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugFrameOffset<T>`

- <span id="debugframeoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugFrameOffset<T>`

- <span id="debugframeoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DebugFrameOffset<T>`

- <span id="debugframeoffset-clone"></span>`fn clone(&self) -> DebugFrameOffset<T>` — [`DebugFrameOffset`](../index.md#debugframeoffset)

##### `impl<T> CloneToUninit for DebugFrameOffset<T>`

- <span id="debugframeoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DebugFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for DebugFrameOffset<T>`

- <span id="debugframeoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DebugFrameOffset<T>`

##### `impl<T> From for DebugFrameOffset<T>`

- <span id="debugframeoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for DebugFrameOffset<T>`

- <span id="debugframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for DebugFrameOffset<T>`

- <span id="debugframeoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DebugFrameOffset<T>`

- <span id="debugframeoffset-partialeq-eq"></span>`fn eq(&self, other: &DebugFrameOffset<T>) -> bool` — [`DebugFrameOffset`](../index.md#debugframeoffset)

##### `impl<T> StructuralPartialEq for DebugFrameOffset<T>`

##### `impl<T> ToOwned for DebugFrameOffset<T>`

- <span id="debugframeoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="debugframeoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugframeoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DebugFrameOffset<T>`

- <span id="debugframeoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugframeoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DebugFrameOffset<T>`

- <span id="debugframeoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugframeoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> UnwindOffset for crate::common::DebugFrameOffset<T>`

- <span id="cratecommondebugframeoffset-unwindoffset-into"></span>`fn into(self) -> T`

### `EhFrameOffset<T>`

```rust
struct EhFrameOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/common.rs:205`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L205)*

An offset into the `.eh_frame` section.

#### Trait Implementations

##### `impl<T> Any for EhFrameOffset<T>`

- <span id="ehframeoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EhFrameOffset<T>`

- <span id="ehframeoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EhFrameOffset<T>`

- <span id="ehframeoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for EhFrameOffset<T>`

- <span id="ehframeoffset-clone"></span>`fn clone(&self) -> EhFrameOffset<T>` — [`EhFrameOffset`](../index.md#ehframeoffset)

##### `impl<T> CloneToUninit for EhFrameOffset<T>`

- <span id="ehframeoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for EhFrameOffset<T>`

##### `impl<T: fmt::Debug> Debug for EhFrameOffset<T>`

- <span id="ehframeoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for EhFrameOffset<T>`

##### `impl<T> From for EhFrameOffset<T>`

- <span id="ehframeoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for EhFrameOffset<T>`

- <span id="ehframeoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for EhFrameOffset<T>`

- <span id="ehframeoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for EhFrameOffset<T>`

- <span id="ehframeoffset-partialeq-eq"></span>`fn eq(&self, other: &EhFrameOffset<T>) -> bool` — [`EhFrameOffset`](../index.md#ehframeoffset)

##### `impl<T> StructuralPartialEq for EhFrameOffset<T>`

##### `impl<T> ToOwned for EhFrameOffset<T>`

- <span id="ehframeoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="ehframeoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ehframeoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for EhFrameOffset<T>`

- <span id="ehframeoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ehframeoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for EhFrameOffset<T>`

- <span id="ehframeoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ehframeoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> UnwindOffset for crate::common::EhFrameOffset<T>`

- <span id="cratecommonehframeoffset-unwindoffset-into"></span>`fn into(self) -> T`

### `DwoId`

```rust
struct DwoId(u64);
```

*Defined in [`gimli-0.32.3/src/common.rs:384`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L384)*

An optionally-provided implementation-defined compilation unit ID to enable
split DWARF and linking a split compilation unit back together.

#### Trait Implementations

##### `impl Any for DwoId`

- <span id="dwoid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DwoId`

- <span id="dwoid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DwoId`

- <span id="dwoid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DwoId`

- <span id="dwoid-clone"></span>`fn clone(&self) -> DwoId` — [`DwoId`](../index.md#dwoid)

##### `impl CloneToUninit for DwoId`

- <span id="dwoid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DwoId`

##### `impl Debug for DwoId`

- <span id="dwoid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DwoId`

##### `impl<T> From for DwoId`

- <span id="dwoid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DwoId`

- <span id="dwoid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for DwoId`

- <span id="dwoid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DwoId`

- <span id="dwoid-partialeq-eq"></span>`fn eq(&self, other: &DwoId) -> bool` — [`DwoId`](../index.md#dwoid)

##### `impl StructuralPartialEq for DwoId`

##### `impl ToOwned for DwoId`

- <span id="dwoid-toowned-type-owned"></span>`type Owned = T`

- <span id="dwoid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dwoid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DwoId`

- <span id="dwoid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwoid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DwoId`

- <span id="dwoid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwoid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Format`

```rust
enum Format {
    Dwarf64,
    Dwarf32,
}
```

*Defined in [`gimli-0.32.3/src/common.rs:3-8`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L3-L8)*

Whether the format of a compilation unit is 32- or 64-bit.

#### Variants

- **`Dwarf64`**

  64-bit DWARF

- **`Dwarf32`**

  32-bit DWARF

#### Implementations

- <span id="format-initial-length-size"></span>`fn initial_length_size(self) -> u8`

  Return the serialized size of an initial length field for the format.

- <span id="format-word-size"></span>`fn word_size(self) -> u8`

  Return the natural word size for the format

#### Trait Implementations

##### `impl Any for Format`

- <span id="format-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Format`

- <span id="format-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Format`

- <span id="format-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Format`

- <span id="format-clone"></span>`fn clone(&self) -> Format` — [`Format`](../index.md#format)

##### `impl CloneToUninit for Format`

- <span id="format-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Format`

##### `impl Debug for Format`

- <span id="format-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Format`

##### `impl<T> From for Format`

- <span id="format-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Format`

- <span id="format-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Format`

- <span id="format-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Format`

- <span id="format-partialeq-eq"></span>`fn eq(&self, other: &Format) -> bool` — [`Format`](../index.md#format)

##### `impl StructuralPartialEq for Format`

##### `impl ToOwned for Format`

- <span id="format-toowned-type-owned"></span>`type Owned = T`

- <span id="format-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="format-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Format`

- <span id="format-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="format-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Format`

- <span id="format-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="format-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Vendor`

```rust
enum Vendor {
    Default,
    AArch64,
}
```

*Defined in [`gimli-0.32.3/src/common.rs:33-38`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L33-L38)*

Which vendor extensions to support.

#### Variants

- **`Default`**

  A default set of extensions, including some common GNU extensions.

- **`AArch64`**

  AAarch64 extensions.

#### Trait Implementations

##### `impl Any for Vendor`

- <span id="vendor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Vendor`

- <span id="vendor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Vendor`

- <span id="vendor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Vendor`

- <span id="vendor-clone"></span>`fn clone(&self) -> Vendor` — [`Vendor`](../index.md#vendor)

##### `impl CloneToUninit for Vendor`

- <span id="vendor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Vendor`

##### `impl Debug for Vendor`

- <span id="vendor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Vendor`

##### `impl<T> From for Vendor`

- <span id="vendor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Vendor`

- <span id="vendor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Vendor`

- <span id="vendor-partialeq-eq"></span>`fn eq(&self, other: &Vendor) -> bool` — [`Vendor`](../index.md#vendor)

##### `impl StructuralPartialEq for Vendor`

##### `impl ToOwned for Vendor`

- <span id="vendor-toowned-type-owned"></span>`type Owned = T`

- <span id="vendor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="vendor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Vendor`

- <span id="vendor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vendor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Vendor`

- <span id="vendor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vendor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitSectionOffset<T>`

```rust
enum UnitSectionOffset<T> {
    DebugInfoOffset(DebugInfoOffset<T>),
    DebugTypesOffset(DebugTypesOffset<T>),
}
```

*Defined in [`gimli-0.32.3/src/common.rs:216-221`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L216-L221)*

An offset into the `.debug_info` or `.debug_types` sections.

#### Variants

- **`DebugInfoOffset`**

  An offset into the `.debug_info` section.

- **`DebugTypesOffset`**

  An offset into the `.debug_types` section.

#### Implementations

- <span id="unitsectionoffset-as-debug-info-offset"></span>`fn as_debug_info_offset(&self) -> Option<DebugInfoOffset<T>>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

  Returns the `DebugInfoOffset` inside, or `None` otherwise.

- <span id="unitsectionoffset-as-debug-types-offset"></span>`fn as_debug_types_offset(&self) -> Option<DebugTypesOffset<T>>` — [`DebugTypesOffset`](../index.md#debugtypesoffset)

  Returns the `DebugTypesOffset` inside, or `None` otherwise.

#### Trait Implementations

##### `impl<T> Any for UnitSectionOffset<T>`

- <span id="unitsectionoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitSectionOffset<T>`

- <span id="unitsectionoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitSectionOffset<T>`

- <span id="unitsectionoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for UnitSectionOffset<T>`

- <span id="unitsectionoffset-clone"></span>`fn clone(&self) -> UnitSectionOffset<T>` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T> CloneToUninit for UnitSectionOffset<T>`

- <span id="unitsectionoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for UnitSectionOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitSectionOffset<T>`

- <span id="unitsectionoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitSectionOffset<T>`

##### `impl<T> From for UnitSectionOffset<T>`

- <span id="unitsectionoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for UnitSectionOffset<T>`

- <span id="unitsectionoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for UnitSectionOffset<T>`

- <span id="unitsectionoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::Ord> Ord for UnitSectionOffset<T>`

- <span id="unitsectionoffset-ord-cmp"></span>`fn cmp(&self, other: &UnitSectionOffset<T>) -> cmp::Ordering` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partialeq-eq"></span>`fn eq(&self, other: &UnitSectionOffset<T>) -> bool` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitSectionOffset<T>`

- <span id="unitsectionoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitSectionOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitSectionOffset`](../index.md#unitsectionoffset)

##### `impl<T> StructuralPartialEq for UnitSectionOffset<T>`

##### `impl<T> ToOwned for UnitSectionOffset<T>`

- <span id="unitsectionoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="unitsectionoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitsectionoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for UnitSectionOffset<T>`

- <span id="unitsectionoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitsectionoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnitSectionOffset<T>`

- <span id="unitsectionoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitsectionoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionId`

```rust
enum SectionId {
    DebugAbbrev,
    DebugAddr,
    DebugAranges,
    DebugCuIndex,
    DebugFrame,
    EhFrame,
    EhFrameHdr,
    DebugInfo,
    DebugLine,
    DebugLineStr,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugPubNames,
    DebugPubTypes,
    DebugRanges,
    DebugRngLists,
    DebugStr,
    DebugStrOffsets,
    DebugTuIndex,
    DebugTypes,
}
```

*Defined in [`gimli-0.32.3/src/common.rs:257-302`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L257-L302)*

An identifier for a DWARF section.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev` section.

- **`DebugAddr`**

  The `.debug_addr` section.

- **`DebugAranges`**

  The `.debug_aranges` section.

- **`DebugCuIndex`**

  The `.debug_cu_index` section.

- **`DebugFrame`**

  The `.debug_frame` section.

- **`EhFrame`**

  The `.eh_frame` section.

- **`EhFrameHdr`**

  The `.eh_frame_hdr` section.

- **`DebugInfo`**

  The `.debug_info` section.

- **`DebugLine`**

  The `.debug_line` section.

- **`DebugLineStr`**

  The `.debug_line_str` section.

- **`DebugLoc`**

  The `.debug_loc` section.

- **`DebugLocLists`**

  The `.debug_loclists` section.

- **`DebugMacinfo`**

  The `.debug_macinfo` section.

- **`DebugMacro`**

  The `.debug_macro` section.

- **`DebugPubNames`**

  The `.debug_pubnames` section.

- **`DebugPubTypes`**

  The `.debug_pubtypes` section.

- **`DebugRanges`**

  The `.debug_ranges` section.

- **`DebugRngLists`**

  The `.debug_rnglists` section.

- **`DebugStr`**

  The `.debug_str` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets` section.

- **`DebugTuIndex`**

  The `.debug_tu_index` section.

- **`DebugTypes`**

  The `.debug_types` section.

#### Implementations

- <span id="sectionid-name"></span>`fn name(self) -> &'static str`

  Returns the ELF section name for this kind.

- <span id="sectionid-dwo-name"></span>`fn dwo_name(self) -> Option<&'static str>`

  Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

- <span id="sectionid-xcoff-name"></span>`fn xcoff_name(self) -> Option<&'static str>`

  Returns the XCOFF section name for this kind.

- <span id="sectionid-is-string"></span>`fn is_string(self) -> bool`

  Returns true if this is a mergeable string section.

  

  This is useful for determining the correct section flags.

#### Trait Implementations

##### `impl Any for SectionId`

- <span id="sectionid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionId`

- <span id="sectionid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionId`

- <span id="sectionid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](../index.md#sectionid)

##### `impl CloneToUninit for SectionId`

- <span id="sectionid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl<T> From for SectionId`

- <span id="sectionid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SectionId`

- <span id="sectionid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SectionId`

- <span id="sectionid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for SectionId`

- <span id="sectionid-ord-cmp"></span>`fn cmp(&self, other: &SectionId) -> cmp::Ordering` — [`SectionId`](../index.md#sectionid)

##### `impl PartialEq for SectionId`

- <span id="sectionid-partialeq-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](../index.md#sectionid)

##### `impl PartialOrd for SectionId`

- <span id="sectionid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SectionId) -> option::Option<cmp::Ordering>` — [`SectionId`](../index.md#sectionid)

##### `impl StructuralPartialEq for SectionId`

##### `impl ToOwned for SectionId`

- <span id="sectionid-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionId`

- <span id="sectionid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionId`

- <span id="sectionid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DwarfFileType`

```rust
enum DwarfFileType {
    Main,
    Dwo,
}
```

*Defined in [`gimli-0.32.3/src/common.rs:389-395`](../../../.source_1765633015/gimli-0.32.3/src/common.rs#L389-L395)*

The "type" of file with DWARF debugging information. This determines, among other things,
which files DWARF sections should be loaded from.

#### Variants

- **`Main`**

  A normal executable or object file.

- **`Dwo`**

  A .dwo split DWARF file.

#### Trait Implementations

##### `impl Any for DwarfFileType`

- <span id="dwarffiletype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DwarfFileType`

- <span id="dwarffiletype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DwarfFileType`

- <span id="dwarffiletype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DwarfFileType`

- <span id="dwarffiletype-clone"></span>`fn clone(&self) -> DwarfFileType` — [`DwarfFileType`](../index.md#dwarffiletype)

##### `impl CloneToUninit for DwarfFileType`

- <span id="dwarffiletype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DwarfFileType`

##### `impl Debug for DwarfFileType`

- <span id="dwarffiletype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DwarfFileType`

- <span id="dwarffiletype-default"></span>`fn default() -> Self`

##### `impl Eq for DwarfFileType`

##### `impl<T> From for DwarfFileType`

- <span id="dwarffiletype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DwarfFileType`

- <span id="dwarffiletype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DwarfFileType`

- <span id="dwarffiletype-partialeq-eq"></span>`fn eq(&self, other: &DwarfFileType) -> bool` — [`DwarfFileType`](../index.md#dwarffiletype)

##### `impl StructuralPartialEq for DwarfFileType`

##### `impl ToOwned for DwarfFileType`

- <span id="dwarffiletype-toowned-type-owned"></span>`type Owned = T`

- <span id="dwarffiletype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dwarffiletype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DwarfFileType`

- <span id="dwarffiletype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwarffiletype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DwarfFileType`

- <span id="dwarffiletype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwarffiletype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

