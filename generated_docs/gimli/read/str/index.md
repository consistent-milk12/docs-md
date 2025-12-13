*[gimli](../../index.md) / [read](../index.md) / [str](index.md)*

---

# Module `str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugStr`](#debugstr) | struct | The `DebugStr` struct represents the DWARF strings found in the `.debug_str` section. |
| [`DebugStrOffsets`](#debugstroffsets) | struct | The raw contents of the `.debug_str_offsets` section. |
| [`DebugLineStr`](#debuglinestr) | struct | The `DebugLineStr` struct represents the DWARF strings found in the `.debug_line_str` section. |

## Structs

### `DebugStr<R>`

```rust
struct DebugStr<R> {
    debug_str_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:12-14`](../../../../.source_1765633015/gimli-0.32.3/src/read/str.rs#L12-L14)*

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- <span id="debugstr-new"></span>`fn new(debug_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugStr` instance from the data in the `.debug_str`

  section.

  

  It is the caller's responsibility to read the `.debug_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_str_section_somehow = || &buf;

  let debug_str = DebugStr::new(read_debug_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugStr<R>`

- <span id="debugstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStr<R>`

- <span id="debugstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStr<R>`

- <span id="debugstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugStr<R>`

- <span id="debugstr-clone"></span>`fn clone(&self) -> DebugStr<R>` — [`DebugStr`](../index.md#debugstr)

##### `impl CloneToUninit for DebugStr<R>`

- <span id="debugstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugStr<R>`

- <span id="debugstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStr<R>`

- <span id="debugstr-default"></span>`fn default() -> DebugStr<R>` — [`DebugStr`](../index.md#debugstr)

##### `impl<T> From for DebugStr<R>`

- <span id="debugstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugStr<R>`

- <span id="debugstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugStr<R>`

- <span id="debugstr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugstr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugStr<R>`

- <span id="debugstr-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugStr<R>`

- <span id="debugstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugStr<R>`

- <span id="debugstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:91-93`](../../../../.source_1765633015/gimli-0.32.3/src/read/str.rs#L91-L93)*

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- <span id="debugstroffsets-get-str-offset"></span>`fn get_str_offset(&self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../../index.md#format), [`DebugStrOffsetsBase`](../../index.md#debugstroffsetsbase), [`Reader`](../index.md#reader), [`DebugStrOffsetsIndex`](../../index.md#debugstroffsetsindex), [`Result`](../../index.md#result), [`DebugStrOffset`](../../index.md#debugstroffset)

  Returns the `.debug_str` offset at the given `base` and `index`.

  

  A set of entries in the `.debug_str_offsets` section consists of a header

  followed by a series of string table offsets.

  

  The `base` must be the `DW_AT_str_offsets_base` value from the compilation unit DIE.

  This is an offset that points to the first entry following the header.

  

  The `index` is the value of a `DW_FORM_strx` attribute.

  

  The `format` must be the DWARF format of the compilation unit. This format must

  match the header. However, note that we do not parse the header to validate this,

  since locating the header is unreliable, and the GNU extensions do not emit it.

#### Trait Implementations

##### `impl Any for DebugStrOffsets<R>`

- <span id="debugstroffsets-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStrOffsets<R>`

- <span id="debugstroffsets-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStrOffsets<R>`

- <span id="debugstroffsets-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugStrOffsets<R>`

- <span id="debugstroffsets-clone"></span>`fn clone(&self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md#debugstroffsets)

##### `impl CloneToUninit for DebugStrOffsets<R>`

- <span id="debugstroffsets-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: fmt::Debug> Debug for DebugStrOffsets<R>`

- <span id="debugstroffsets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStrOffsets<R>`

- <span id="debugstroffsets-default"></span>`fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md#debugstroffsets)

##### `impl<T> From for DebugStrOffsets<R>`

- <span id="debugstroffsets-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugStrOffsets<R>`

- <span id="debugstroffsets-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugStrOffsets<R>`

- <span id="debugstroffsets-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugstroffsets-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugStrOffsets<R>`

- <span id="debugstroffsets-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstroffsets-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstroffsets-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugStrOffsets<R>`

- <span id="debugstroffsets-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstroffsets-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugStrOffsets<R>`

- <span id="debugstroffsets-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstroffsets-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:184-186`](../../../../.source_1765633015/gimli-0.32.3/src/read/str.rs#L184-L186)*

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- <span id="debuglinestr-new"></span>`fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLineStr` instance from the data in the `.debug_line_str`

  section.

  

  It is the caller's responsibility to read the `.debug_line_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLineStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_str_section_somehow = || &buf;

  let debug_str = DebugLineStr::new(read_debug_line_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLineStr<R>`

- <span id="debuglinestr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLineStr<R>`

- <span id="debuglinestr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLineStr<R>`

- <span id="debuglinestr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLineStr<R>`

- <span id="debuglinestr-clone"></span>`fn clone(&self) -> DebugLineStr<R>` — [`DebugLineStr`](../index.md#debuglinestr)

##### `impl CloneToUninit for DebugLineStr<R>`

- <span id="debuglinestr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugLineStr<R>`

- <span id="debuglinestr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLineStr<R>`

- <span id="debuglinestr-default"></span>`fn default() -> DebugLineStr<R>` — [`DebugLineStr`](../index.md#debuglinestr)

##### `impl<T> From for DebugLineStr<R>`

- <span id="debuglinestr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLineStr<R>`

- <span id="debuglinestr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLineStr<R>`

- <span id="debuglinestr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debuglinestr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLineStr<R>`

- <span id="debuglinestr-toowned-type-owned"></span>`type Owned = T`

- <span id="debuglinestr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuglinestr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLineStr<R>`

- <span id="debuglinestr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglinestr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLineStr<R>`

- <span id="debuglinestr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglinestr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

