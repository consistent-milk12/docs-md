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

*Defined in [`gimli-0.32.3/src/read/str.rs:12-14`](../../../../.source_1765521767/gimli-0.32.3/src/read/str.rs#L12-L14)*

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- <span id="debugstr-new"></span>`fn new(debug_str_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStr<R>`

- <span id="debugstr-clone"></span>`fn clone(&self) -> DebugStr<R>` — [`DebugStr`](../index.md#debugstr)

##### `impl<R: marker::Copy> Copy for DebugStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugStr<R>`

- <span id="debugstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStr<R>`

- <span id="debugstr-default"></span>`fn default() -> DebugStr<R>` — [`DebugStr`](../index.md#debugstr)

##### `impl<R> Section for DebugStr<R>`

- <span id="debugstr-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugstr-reader"></span>`fn reader(&self) -> &R`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:91-93`](../../../../.source_1765521767/gimli-0.32.3/src/read/str.rs#L91-L93)*

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- <span id="debugstroffsets-get-str-offset"></span>`fn get_str_offset(&self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../../index.md#format), [`DebugStrOffsetsBase`](../../index.md#debugstroffsetsbase), [`Reader`](../index.md#reader), [`DebugStrOffsetsIndex`](../../index.md#debugstroffsetsindex), [`Result`](../../index.md#result), [`DebugStrOffset`](../../index.md#debugstroffset)

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStrOffsets<R>`

- <span id="debugstroffsets-clone"></span>`fn clone(&self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md#debugstroffsets)

##### `impl<R: marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: fmt::Debug> Debug for DebugStrOffsets<R>`

- <span id="debugstroffsets-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStrOffsets<R>`

- <span id="debugstroffsets-default"></span>`fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md#debugstroffsets)

##### `impl<R> Section for DebugStrOffsets<R>`

- <span id="debugstroffsets-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugstroffsets-reader"></span>`fn reader(&self) -> &R`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:184-186`](../../../../.source_1765521767/gimli-0.32.3/src/read/str.rs#L184-L186)*

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- <span id="debuglinestr-new"></span>`fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLineStr<R>`

- <span id="debuglinestr-clone"></span>`fn clone(&self) -> DebugLineStr<R>` — [`DebugLineStr`](../index.md#debuglinestr)

##### `impl<R: marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugLineStr<R>`

- <span id="debuglinestr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLineStr<R>`

- <span id="debuglinestr-default"></span>`fn default() -> DebugLineStr<R>` — [`DebugLineStr`](../index.md#debuglinestr)

##### `impl<R> Section for DebugLineStr<R>`

- <span id="debuglinestr-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debuglinestr-reader"></span>`fn reader(&self) -> &R`

