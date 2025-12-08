*[gimli](../../index.md) / [read](../index.md) / [str](index.md)*

---

# Module `str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugStr`](#debugstr) | struct | The `DebugStr` struct represents the DWARF strings |
| [`DebugStrOffsets`](#debugstroffsets) | struct | The raw contents of the `.debug_str_offsets` section. |
| [`DebugLineStr`](#debuglinestr) | struct | The `DebugLineStr` struct represents the DWARF strings |

## Structs

### `DebugStr<R>`

```rust
struct DebugStr<R> {
    debug_str_section: R,
}
```

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- <span id="debugstr-get-str"></span>`fn get_str(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStr<R>`

- <span id="debugstr-clone"></span>`fn clone(&self) -> DebugStr<R>` — [`DebugStr`](../index.md)

##### `impl<R: marker::Copy> Copy for DebugStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugStr<R>`

- <span id="debugstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStr<R>`

- <span id="debugstr-default"></span>`fn default() -> DebugStr<R>` — [`DebugStr`](../index.md)

##### `impl<R> Section for DebugStr<R>`

- <span id="debugstr-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md)

- <span id="debugstr-reader"></span>`fn reader(&self) -> &R`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- <span id="debugstroffsets-get-str-offset"></span>`fn get_str_offset(&self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../../index.md), [`DebugStrOffsetsBase`](../../index.md), [`Reader`](../index.md), [`DebugStrOffsetsIndex`](../../index.md), [`Result`](../../index.md), [`DebugStrOffset`](../../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStrOffsets<R>`

- <span id="debugstroffsets-clone"></span>`fn clone(&self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md)

##### `impl<R: marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: fmt::Debug> Debug for DebugStrOffsets<R>`

- <span id="debugstroffsets-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStrOffsets<R>`

- <span id="debugstroffsets-default"></span>`fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md)

##### `impl<R> Section for DebugStrOffsets<R>`

- <span id="debugstroffsets-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md)

- <span id="debugstroffsets-reader"></span>`fn reader(&self) -> &R`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- <span id="debuglinestr-get-str"></span>`fn get_str(&self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugLineStrOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLineStr<R>`

- <span id="debuglinestr-clone"></span>`fn clone(&self) -> DebugLineStr<R>` — [`DebugLineStr`](../index.md)

##### `impl<R: marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugLineStr<R>`

- <span id="debuglinestr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLineStr<R>`

- <span id="debuglinestr-default"></span>`fn default() -> DebugLineStr<R>` — [`DebugLineStr`](../index.md)

##### `impl<R> Section for DebugLineStr<R>`

- <span id="debuglinestr-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md)

- <span id="debuglinestr-reader"></span>`fn reader(&self) -> &R`

