*[gimli](../../index.md) / [read](../index.md) / [macros](index.md)*

---

# Module `macros`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugMacinfo`](#debugmacinfo) | struct | The raw contents of the `.debug_macinfo` section. |
| [`DebugMacro`](#debugmacro) | struct | The raw contents of the `.debug_macro` section. |
| [`MacroUnitHeader`](#macrounitheader) | struct |  |
| [`MacroIter`](#macroiter) | struct | Iterator over the entries in the `.debug_macro` section. |
| [`MacroString`](#macrostring) | enum | A string in a macro entry. |
| [`MacroEntry`](#macroentry) | enum | an Entry in the `.debug_macro` section. |

## Structs

### `DebugMacinfo<R>`

```rust
struct DebugMacinfo<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:11-13`](../../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L11-L13)*

The raw contents of the `.debug_macinfo` section.

#### Implementations

- <span id="debugmacinfo-new"></span>`fn new(macinfo_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacinfo` instance from the data in the `.debug_macinfo`

  section.

  

  It is the caller's responsibility to read the `.debug_macinfo` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacinfo, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacinfo::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugMacinfo<R>`

- <span id="debugmacinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugMacinfo<R>`

- <span id="debugmacinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugMacinfo<R>`

- <span id="debugmacinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugMacinfo<R>`

- <span id="debugmacinfo-clone"></span>`fn clone(&self) -> DebugMacinfo<R>` — [`DebugMacinfo`](../index.md#debugmacinfo)

##### `impl CloneToUninit for DebugMacinfo<R>`

- <span id="debugmacinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugMacinfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacinfo<R>`

- <span id="debugmacinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacinfo<R>`

- <span id="debugmacinfo-default"></span>`fn default() -> DebugMacinfo<R>` — [`DebugMacinfo`](../index.md#debugmacinfo)

##### `impl<T> From for DebugMacinfo<R>`

- <span id="debugmacinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugMacinfo<R>`

- <span id="debugmacinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugMacinfo<R>`

- <span id="debugmacinfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugmacinfo-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugMacinfo<R>`

- <span id="debugmacinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="debugmacinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugmacinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugMacinfo<R>`

- <span id="debugmacinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugmacinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugMacinfo<R>`

- <span id="debugmacinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugmacinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugMacro<R>`

```rust
struct DebugMacro<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:104-106`](../../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L104-L106)*

The raw contents of the `.debug_macro` section.

#### Implementations

- <span id="debugmacro-new"></span>`fn new(macro_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacro` instance from the data in the `.debug_macro`

  section.

  

  It is the caller's responsibility to read the `.debug_macro` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacro, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacro::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugMacro<R>`

- <span id="debugmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugMacro<R>`

- <span id="debugmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugMacro<R>`

- <span id="debugmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugMacro<R>`

- <span id="debugmacro-clone"></span>`fn clone(&self) -> DebugMacro<R>` — [`DebugMacro`](../index.md#debugmacro)

##### `impl CloneToUninit for DebugMacro<R>`

- <span id="debugmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugMacro<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacro<R>`

- <span id="debugmacro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacro<R>`

- <span id="debugmacro-default"></span>`fn default() -> DebugMacro<R>` — [`DebugMacro`](../index.md#debugmacro)

##### `impl<T> From for DebugMacro<R>`

- <span id="debugmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugMacro<R>`

- <span id="debugmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugMacro<R>`

- <span id="debugmacro-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugmacro-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugMacro<R>`

- <span id="debugmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="debugmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugMacro<R>`

- <span id="debugmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugMacro<R>`

- <span id="debugmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroUnitHeader<R: Reader>`

```rust
struct MacroUnitHeader<R: Reader> {
    _version: u16,
    flags: u8,
    _debug_line_offset: crate::DebugLineOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:197-202`](../../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L197-L202)*

#### Fields

- **`_version`**: `u16`

  The version of the macro unit header. At the moment only version 5 is defined.

#### Implementations

- <span id="macrounitheader-const-offset-size-flag"></span>`const OFFSET_SIZE_FLAG: u8`

- <span id="macrounitheader-const-debug-line-offset-flag"></span>`const DEBUG_LINE_OFFSET_FLAG: u8`

- <span id="macrounitheader-const-opcode-operands-table-flag"></span>`const OPCODE_OPERANDS_TABLE_FLAG: u8`

- <span id="macrounitheader-parse"></span>`fn parse(input: &mut R) -> Result<Self>` — [`Result`](../../index.md#result)

- <span id="macrounitheader-format"></span>`fn format(&self) -> Format` — [`Format`](../../index.md#format)

#### Trait Implementations

##### `impl Any for MacroUnitHeader<R>`

- <span id="macrounitheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroUnitHeader<R>`

- <span id="macrounitheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroUnitHeader<R>`

- <span id="macrounitheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for MacroUnitHeader<R>`

- <span id="macrounitheader-clone"></span>`fn clone(&self) -> MacroUnitHeader<R>` — [`MacroUnitHeader`](#macrounitheader)

##### `impl CloneToUninit for MacroUnitHeader<R>`

- <span id="macrounitheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for MacroUnitHeader<R>`

- <span id="macrounitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MacroUnitHeader<R>`

- <span id="macrounitheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroUnitHeader<R>`

- <span id="macrounitheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MacroUnitHeader<R>`

- <span id="macrounitheader-toowned-type-owned"></span>`type Owned = T`

- <span id="macrounitheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macrounitheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroUnitHeader<R>`

- <span id="macrounitheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macrounitheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroUnitHeader<R>`

- <span id="macrounitheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macrounitheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroIter<R: Reader>`

```rust
struct MacroIter<R: Reader> {
    input: R,
    format: crate::Format,
    is_macro: bool,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:327-331`](../../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L327-L331)*

Iterator over the entries in the `.debug_macro` section.

#### Implementations

- <span id="macroiter-next"></span>`fn next(&mut self) -> Result<Option<MacroEntry<R>>>` — [`Result`](../../index.md#result), [`MacroEntry`](../index.md#macroentry)

  Advance the iterator to the next entry in the `.debug_macro` section.

#### Trait Implementations

##### `impl Any for MacroIter<R>`

- <span id="macroiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroIter<R>`

- <span id="macroiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroIter<R>`

- <span id="macroiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for MacroIter<R>`

- <span id="macroiter-clone"></span>`fn clone(&self) -> MacroIter<R>` — [`MacroIter`](../index.md#macroiter)

##### `impl CloneToUninit for MacroIter<R>`

- <span id="macroiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for MacroIter<R>`

- <span id="macroiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MacroIter<R>`

- <span id="macroiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroIter<R>`

- <span id="macroiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MacroIter<R>`

- <span id="macroiter-toowned-type-owned"></span>`type Owned = T`

- <span id="macroiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macroiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroIter<R>`

- <span id="macroiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macroiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroIter<R>`

- <span id="macroiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macroiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MacroString<R, Offset>`

```rust
enum MacroString<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Direct(R),
    StringPointer(crate::DebugStrOffset<Offset>),
    IndirectStringPointer(crate::DebugStrOffsetsIndex<Offset>),
    Supplementary(crate::DebugStrOffset<Offset>),
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:244-258`](../../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L244-L258)*

A string in a macro entry.

#### Variants

- **`Direct`**

  The string is directly embedded in the macro entry

- **`StringPointer`**

  The macro refers to a string in the `.debug_str` section using a `DebugStrOffset`.

- **`IndirectStringPointer`**

  The macro contains an index into an array in the `.debug_str_offsets`
  section, which refers to a string in the `.debug_str` section.

- **`Supplementary`**

  The macro refers to a string in the `.debug_str` section in the supplementary object file

#### Implementations

- <span id="macrostring-string"></span>`fn string(&self, unit: UnitRef<'_, R>) -> Result<R>` — [`UnitRef`](../index.md#unitref), [`Result`](../../index.md#result)

  Get the string slice from the macro entry.

#### Trait Implementations

##### `impl Any for MacroString<R, Offset>`

- <span id="macrostring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroString<R, Offset>`

- <span id="macrostring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroString<R, Offset>`

- <span id="macrostring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for MacroString<R, Offset>`

- <span id="macrostring-clone"></span>`fn clone(&self) -> MacroString<R, Offset>` — [`MacroString`](../index.md#macrostring)

##### `impl CloneToUninit for MacroString<R, Offset>`

- <span id="macrostring-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for MacroString<R, Offset>`

- <span id="macrostring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroString<R, Offset>`

##### `impl<T> From for MacroString<R, Offset>`

- <span id="macrostring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroString<R, Offset>`

- <span id="macrostring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for MacroString<R, Offset>`

- <span id="macrostring-partialeq-eq"></span>`fn eq(&self, other: &MacroString<R, Offset>) -> bool` — [`MacroString`](../index.md#macrostring)

##### `impl<R, Offset> StructuralPartialEq for MacroString<R, Offset>`

##### `impl ToOwned for MacroString<R, Offset>`

- <span id="macrostring-toowned-type-owned"></span>`type Owned = T`

- <span id="macrostring-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macrostring-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroString<R, Offset>`

- <span id="macrostring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macrostring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroString<R, Offset>`

- <span id="macrostring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macrostring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroEntry<R, Offset>`

```rust
enum MacroEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Define {
        line: u64,
        text: MacroString<R>,
    },
    Undef {
        line: u64,
        name: MacroString<R>,
    },
    StartFile {
        line: u64,
        file: u64,
    },
    EndFile,
    Import {
        offset: crate::DebugMacroOffset<Offset>,
    },
    ImportSup {
        offset: crate::DebugMacroOffset<Offset>,
    },
    VendorExt {
        numeric: u64,
        string: R,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:277-323`](../../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L277-L323)*

an Entry in the `.debug_macro` section.

#### Variants

- **`Define`**

  A macro definition.

- **`Undef`**

  A macro undefinition.

- **`StartFile`**

  The start of a file.

- **`EndFile`**

  The end of the current included file.

- **`Import`**

  import a macro unit

- **`ImportSup`**

  import a macro unit from the supplementary object file

- **`VendorExt`**

  A vendor-specific extension.

#### Trait Implementations

##### `impl Any for MacroEntry<R, Offset>`

- <span id="macroentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroEntry<R, Offset>`

- <span id="macroentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroEntry<R, Offset>`

- <span id="macroentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for MacroEntry<R, Offset>`

- <span id="macroentry-clone"></span>`fn clone(&self) -> MacroEntry<R, Offset>` — [`MacroEntry`](../index.md#macroentry)

##### `impl CloneToUninit for MacroEntry<R, Offset>`

- <span id="macroentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for MacroEntry<R, Offset>`

- <span id="macroentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroEntry<R, Offset>`

##### `impl<T> From for MacroEntry<R, Offset>`

- <span id="macroentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroEntry<R, Offset>`

- <span id="macroentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for MacroEntry<R, Offset>`

- <span id="macroentry-partialeq-eq"></span>`fn eq(&self, other: &MacroEntry<R, Offset>) -> bool` — [`MacroEntry`](../index.md#macroentry)

##### `impl<R, Offset> StructuralPartialEq for MacroEntry<R, Offset>`

##### `impl ToOwned for MacroEntry<R, Offset>`

- <span id="macroentry-toowned-type-owned"></span>`type Owned = T`

- <span id="macroentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macroentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroEntry<R, Offset>`

- <span id="macroentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macroentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroEntry<R, Offset>`

- <span id="macroentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macroentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

