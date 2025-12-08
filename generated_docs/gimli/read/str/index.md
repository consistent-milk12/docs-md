*[gimli](../../index.md) / [read](../index.md) / [str](index.md)*

---

# Module `str`

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

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugStr<R>` — [`DebugStr`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugStr<R>`

- `fn clone(self: &Self) -> DebugStr<R>` — [`DebugStr`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugStr<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugStr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugStr<R>`

- `fn default() -> DebugStr<R>` — [`DebugStr`](../index.md)

##### `impl<R> Section for DebugStr<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- `fn get_str_offset(self: &Self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../../index.md), [`DebugStrOffsetsBase`](../../index.md), [`Reader`](../index.md), [`DebugStrOffsetsIndex`](../../index.md), [`Result`](../../index.md), [`DebugStrOffset`](../../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugStrOffsets<R>`

- `fn clone(self: &Self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugStrOffsets<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugStrOffsets<R>`

- `fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](../index.md)

##### `impl<R> Section for DebugStrOffsets<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- `fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLineStr<R>`

- `fn clone(self: &Self) -> DebugLineStr<R>` — [`DebugLineStr`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLineStr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLineStr<R>`

- `fn default() -> DebugLineStr<R>` — [`DebugLineStr`](../index.md)

##### `impl<R> Section for DebugLineStr<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

