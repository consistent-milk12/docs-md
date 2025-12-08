*[gimli](../index.md) / [common](index.md)*

---

# Module `common`

## Structs

### `Encoding`

```rust
struct Encoding {
    pub address_size: u8,
    pub format: Format,
    pub version: u16,
}
```

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

##### `impl Clone for Encoding`

- `fn clone(self: &Self) -> Encoding` — [`Encoding`](../index.md)

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Encoding`

##### `impl Hash for Encoding`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Encoding`

- `fn eq(self: &Self, other: &Encoding) -> bool` — [`Encoding`](../index.md)

##### `impl StructuralPartialEq for Encoding`

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

##### `impl Clone for LineEncoding`

- `fn clone(self: &Self) -> LineEncoding` — [`LineEncoding`](../index.md)

##### `impl Copy for LineEncoding`

##### `impl Debug for LineEncoding`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LineEncoding`

- `fn default() -> Self`

##### `impl Eq for LineEncoding`

##### `impl Hash for LineEncoding`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for LineEncoding`

- `fn eq(self: &Self, other: &LineEncoding) -> bool` — [`LineEncoding`](../index.md)

##### `impl StructuralPartialEq for LineEncoding`

### `Register`

```rust
struct Register(u16);
```

A DWARF register number.

The meaning of this value is ABI dependent. This is generally encoded as
a ULEB128, but supported architectures need 16 bits at most.

#### Implementations

- `fn from_u64(x: u64) -> Result<Register>` — [`Result`](../index.md), [`Register`](../index.md)

#### Trait Implementations

##### `impl Clone for Register`

- `fn clone(self: &Self) -> Register` — [`Register`](../index.md)

##### `impl Copy for Register`

##### `impl Debug for Register`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Register`

##### `impl Hash for Register`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Register`

- `fn cmp(self: &Self, other: &Register) -> $crate::cmp::Ordering` — [`Register`](../index.md)

##### `impl PartialEq for Register`

- `fn eq(self: &Self, other: &Register) -> bool` — [`Register`](../index.md)

##### `impl PartialOrd for Register`

- `fn partial_cmp(self: &Self, other: &Register) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Register`](../index.md)

##### `impl StructuralPartialEq for Register`

### `DebugAbbrevOffset<T>`

```rust
struct DebugAbbrevOffset<T>(T);
```

An offset into the `.debug_abbrev` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugAbbrevOffset<T>`

- `fn clone(self: &Self) -> DebugAbbrevOffset<T>` — [`DebugAbbrevOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugAbbrevOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugAbbrevOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugAbbrevOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for DebugAbbrevOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugAbbrevOffset<T>`

- `fn eq(self: &Self, other: &DebugAbbrevOffset<T>) -> bool` — [`DebugAbbrevOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugAbbrevOffset<T>`

### `DebugAddrOffset<T>`

```rust
struct DebugAddrOffset<T>(T);
```

An offset into the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugAddrOffset<T>`

- `fn clone(self: &Self) -> DebugAddrOffset<T>` — [`DebugAddrOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugAddrOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugAddrOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugAddrOffset<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugAddrOffset<T>`

- `fn eq(self: &Self, other: &DebugAddrOffset<T>) -> bool` — [`DebugAddrOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugAddrOffset<T>`

### `DebugAddrBase<T>`

```rust
struct DebugAddrBase<T>(T);
```

An offset to a set of entries in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugAddrBase<T>`

- `fn clone(self: &Self) -> DebugAddrBase<T>` — [`DebugAddrBase`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugAddrBase<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugAddrBase<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugAddrBase<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugAddrBase<T>`

- `fn eq(self: &Self, other: &DebugAddrBase<T>) -> bool` — [`DebugAddrBase`](../index.md)

##### `impl<T> StructuralPartialEq for DebugAddrBase<T>`

### `DebugAddrIndex<T>`

```rust
struct DebugAddrIndex<T>(T);
```

An index into a set of addresses in the `.debug_addr` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugAddrIndex<T>`

- `fn clone(self: &Self) -> DebugAddrIndex<T>` — [`DebugAddrIndex`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugAddrIndex<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugAddrIndex<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugAddrIndex<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugAddrIndex<T>`

- `fn eq(self: &Self, other: &DebugAddrIndex<T>) -> bool` — [`DebugAddrIndex`](../index.md)

##### `impl<T> StructuralPartialEq for DebugAddrIndex<T>`

### `DebugArangesOffset<T>`

```rust
struct DebugArangesOffset<T>(T);
```

An offset into the `.debug_aranges` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugArangesOffset<T>`

- `fn clone(self: &Self) -> DebugArangesOffset<T>` — [`DebugArangesOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugArangesOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugArangesOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugArangesOffset<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugArangesOffset<T>`

- `fn eq(self: &Self, other: &DebugArangesOffset<T>) -> bool` — [`DebugArangesOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugArangesOffset<T>`

### `DebugInfoOffset<T>`

```rust
struct DebugInfoOffset<T>(T);
```

An offset into the `.debug_info` section.

#### Implementations

- `fn to_unit_offset<R>(self: &Self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md), [`UnitOffset`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugInfoOffset<T>`

- `fn clone(self: &Self) -> DebugInfoOffset<T>` — [`DebugInfoOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugInfoOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugInfoOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugInfoOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for DebugInfoOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::Ord> Ord for DebugInfoOffset<T>`

- `fn cmp(self: &Self, other: &DebugInfoOffset<T>) -> $crate::cmp::Ordering` — [`DebugInfoOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugInfoOffset<T>`

- `fn eq(self: &Self, other: &DebugInfoOffset<T>) -> bool` — [`DebugInfoOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialOrd> PartialOrd for DebugInfoOffset<T>`

- `fn partial_cmp(self: &Self, other: &DebugInfoOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DebugInfoOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugInfoOffset<T>`

### `DebugLineOffset<T>`

```rust
struct DebugLineOffset<T>(T);
```

An offset into the `.debug_line` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugLineOffset<T>`

- `fn clone(self: &Self) -> DebugLineOffset<T>` — [`DebugLineOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugLineOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugLineOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugLineOffset<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugLineOffset<T>`

- `fn eq(self: &Self, other: &DebugLineOffset<T>) -> bool` — [`DebugLineOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugLineOffset<T>`

### `DebugLineStrOffset<T>`

```rust
struct DebugLineStrOffset<T>(T);
```

An offset into the `.debug_line_str` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugLineStrOffset<T>`

- `fn clone(self: &Self) -> DebugLineStrOffset<T>` — [`DebugLineStrOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugLineStrOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugLineStrOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugLineStrOffset<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugLineStrOffset<T>`

- `fn eq(self: &Self, other: &DebugLineStrOffset<T>) -> bool` — [`DebugLineStrOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugLineStrOffset<T>`

### `LocationListsOffset<T>`

```rust
struct LocationListsOffset<T>(T);
```

An offset into either the `.debug_loc` section or the `.debug_loclists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for LocationListsOffset<T>`

- `fn clone(self: &Self) -> LocationListsOffset<T>` — [`LocationListsOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for LocationListsOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for LocationListsOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for LocationListsOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for LocationListsOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for LocationListsOffset<T>`

- `fn eq(self: &Self, other: &LocationListsOffset<T>) -> bool` — [`LocationListsOffset`](../index.md)

##### `impl<T> StructuralPartialEq for LocationListsOffset<T>`

### `DebugLocListsBase<T>`

```rust
struct DebugLocListsBase<T>(T);
```

An offset to a set of location list offsets in the `.debug_loclists` section.

#### Implementations

- `fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugLocListsBase<Offset>` — [`Encoding`](../index.md), [`DwarfFileType`](../index.md), [`DebugLocListsBase`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugLocListsBase<T>`

- `fn clone(self: &Self) -> DebugLocListsBase<T>` — [`DebugLocListsBase`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugLocListsBase<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugLocListsBase<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugLocListsBase<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugLocListsBase<T>`

- `fn eq(self: &Self, other: &DebugLocListsBase<T>) -> bool` — [`DebugLocListsBase`](../index.md)

##### `impl<T> StructuralPartialEq for DebugLocListsBase<T>`

### `DebugLocListsIndex<T>`

```rust
struct DebugLocListsIndex<T>(T);
```

An index into a set of location list offsets in the `.debug_loclists` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugLocListsIndex<T>`

- `fn clone(self: &Self) -> DebugLocListsIndex<T>` — [`DebugLocListsIndex`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugLocListsIndex<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugLocListsIndex<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugLocListsIndex<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugLocListsIndex<T>`

- `fn eq(self: &Self, other: &DebugLocListsIndex<T>) -> bool` — [`DebugLocListsIndex`](../index.md)

##### `impl<T> StructuralPartialEq for DebugLocListsIndex<T>`

### `DebugMacinfoOffset<T>`

```rust
struct DebugMacinfoOffset<T>(T);
```

An offset into the `.debug_macinfo` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugMacinfoOffset<T>`

- `fn clone(self: &Self) -> DebugMacinfoOffset<T>` — [`DebugMacinfoOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugMacinfoOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugMacinfoOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugMacinfoOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for DebugMacinfoOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugMacinfoOffset<T>`

- `fn eq(self: &Self, other: &DebugMacinfoOffset<T>) -> bool` — [`DebugMacinfoOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugMacinfoOffset<T>`

### `DebugMacroOffset<T>`

```rust
struct DebugMacroOffset<T>(T);
```

An offset into the `.debug_macro` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugMacroOffset<T>`

- `fn clone(self: &Self) -> DebugMacroOffset<T>` — [`DebugMacroOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugMacroOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugMacroOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugMacroOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for DebugMacroOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugMacroOffset<T>`

- `fn eq(self: &Self, other: &DebugMacroOffset<T>) -> bool` — [`DebugMacroOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugMacroOffset<T>`

### `RawRangeListsOffset<T>`

```rust
struct RawRangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

If this is from a DWARF 4 DWO file, then it must additionally be offset by the
value of `DW_AT_GNU_ranges_base`. You can use `Dwarf::ranges_offset_from_raw` to do this.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RawRangeListsOffset<T>`

- `fn clone(self: &Self) -> RawRangeListsOffset<T>` — [`RawRangeListsOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for RawRangeListsOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for RawRangeListsOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for RawRangeListsOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for RawRangeListsOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for RawRangeListsOffset<T>`

- `fn eq(self: &Self, other: &RawRangeListsOffset<T>) -> bool` — [`RawRangeListsOffset`](../index.md)

##### `impl<T> StructuralPartialEq for RawRangeListsOffset<T>`

### `RangeListsOffset<T>`

```rust
struct RangeListsOffset<T>(T);
```

An offset into either the `.debug_ranges` section or the `.debug_rnglists` section,
depending on the version of the unit the offset was contained in.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RangeListsOffset<T>`

- `fn clone(self: &Self) -> RangeListsOffset<T>` — [`RangeListsOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for RangeListsOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for RangeListsOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for RangeListsOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for RangeListsOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for RangeListsOffset<T>`

- `fn eq(self: &Self, other: &RangeListsOffset<T>) -> bool` — [`RangeListsOffset`](../index.md)

##### `impl<T> StructuralPartialEq for RangeListsOffset<T>`

### `DebugRngListsBase<T>`

```rust
struct DebugRngListsBase<T>(T);
```

An offset to a set of range list offsets in the `.debug_rnglists` section.

#### Implementations

- `fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugRngListsBase<Offset>` — [`Encoding`](../index.md), [`DwarfFileType`](../index.md), [`DebugRngListsBase`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugRngListsBase<T>`

- `fn clone(self: &Self) -> DebugRngListsBase<T>` — [`DebugRngListsBase`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugRngListsBase<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugRngListsBase<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugRngListsBase<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugRngListsBase<T>`

- `fn eq(self: &Self, other: &DebugRngListsBase<T>) -> bool` — [`DebugRngListsBase`](../index.md)

##### `impl<T> StructuralPartialEq for DebugRngListsBase<T>`

### `DebugRngListsIndex<T>`

```rust
struct DebugRngListsIndex<T>(T);
```

An index into a set of range list offsets in the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugRngListsIndex<T>`

- `fn clone(self: &Self) -> DebugRngListsIndex<T>` — [`DebugRngListsIndex`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugRngListsIndex<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugRngListsIndex<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugRngListsIndex<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugRngListsIndex<T>`

- `fn eq(self: &Self, other: &DebugRngListsIndex<T>) -> bool` — [`DebugRngListsIndex`](../index.md)

##### `impl<T> StructuralPartialEq for DebugRngListsIndex<T>`

### `DebugStrOffset<T>`

```rust
struct DebugStrOffset<T>(T);
```

An offset into the `.debug_str` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugStrOffset<T>`

- `fn clone(self: &Self) -> DebugStrOffset<T>` — [`DebugStrOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugStrOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugStrOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugStrOffset<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugStrOffset<T>`

- `fn eq(self: &Self, other: &DebugStrOffset<T>) -> bool` — [`DebugStrOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugStrOffset<T>`

### `DebugStrOffsetsBase<T>`

```rust
struct DebugStrOffsetsBase<T>(T);
```

An offset to a set of entries in the `.debug_str_offsets` section.

#### Implementations

- `fn default_for_encoding_and_file(encoding: Encoding, file_type: DwarfFileType) -> DebugStrOffsetsBase<Offset>` — [`Encoding`](../index.md), [`DwarfFileType`](../index.md), [`DebugStrOffsetsBase`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugStrOffsetsBase<T>`

- `fn clone(self: &Self) -> DebugStrOffsetsBase<T>` — [`DebugStrOffsetsBase`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugStrOffsetsBase<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugStrOffsetsBase<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugStrOffsetsBase<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugStrOffsetsBase<T>`

- `fn eq(self: &Self, other: &DebugStrOffsetsBase<T>) -> bool` — [`DebugStrOffsetsBase`](../index.md)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsBase<T>`

### `DebugStrOffsetsIndex<T>`

```rust
struct DebugStrOffsetsIndex<T>(T);
```

An index into a set of entries in the `.debug_str_offsets` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugStrOffsetsIndex<T>`

- `fn clone(self: &Self) -> DebugStrOffsetsIndex<T>` — [`DebugStrOffsetsIndex`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugStrOffsetsIndex<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugStrOffsetsIndex<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugStrOffsetsIndex<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugStrOffsetsIndex<T>`

- `fn eq(self: &Self, other: &DebugStrOffsetsIndex<T>) -> bool` — [`DebugStrOffsetsIndex`](../index.md)

##### `impl<T> StructuralPartialEq for DebugStrOffsetsIndex<T>`

### `DebugTypesOffset<T>`

```rust
struct DebugTypesOffset<T>(T);
```

An offset into the `.debug_types` section.

#### Implementations

- `fn to_unit_offset<R>(self: &Self, unit: &UnitHeader<R>) -> Option<UnitOffset<T>>` — [`UnitHeader`](../read/index.md), [`UnitOffset`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugTypesOffset<T>`

- `fn clone(self: &Self) -> DebugTypesOffset<T>` — [`DebugTypesOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugTypesOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugTypesOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugTypesOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for DebugTypesOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::Ord> Ord for DebugTypesOffset<T>`

- `fn cmp(self: &Self, other: &DebugTypesOffset<T>) -> $crate::cmp::Ordering` — [`DebugTypesOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugTypesOffset<T>`

- `fn eq(self: &Self, other: &DebugTypesOffset<T>) -> bool` — [`DebugTypesOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialOrd> PartialOrd for DebugTypesOffset<T>`

- `fn partial_cmp(self: &Self, other: &DebugTypesOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`DebugTypesOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugTypesOffset<T>`

### `DebugTypeSignature`

```rust
struct DebugTypeSignature(u64);
```

A type signature as used in the `.debug_types` section.

#### Trait Implementations

##### `impl Clone for DebugTypeSignature`

- `fn clone(self: &Self) -> DebugTypeSignature` — [`DebugTypeSignature`](../index.md)

##### `impl Copy for DebugTypeSignature`

##### `impl Debug for DebugTypeSignature`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for DebugTypeSignature`

##### `impl Hash for DebugTypeSignature`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for DebugTypeSignature`

- `fn eq(self: &Self, other: &DebugTypeSignature) -> bool` — [`DebugTypeSignature`](../index.md)

##### `impl StructuralPartialEq for DebugTypeSignature`

### `DebugFrameOffset<T>`

```rust
struct DebugFrameOffset<T>(T);
```

An offset into the `.debug_frame` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DebugFrameOffset<T>`

- `fn clone(self: &Self) -> DebugFrameOffset<T>` — [`DebugFrameOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for DebugFrameOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DebugFrameOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DebugFrameOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for DebugFrameOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DebugFrameOffset<T>`

- `fn eq(self: &Self, other: &DebugFrameOffset<T>) -> bool` — [`DebugFrameOffset`](../index.md)

##### `impl<T> StructuralPartialEq for DebugFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::DebugFrameOffset<T>`

- `fn into(self: Self) -> T`

### `EhFrameOffset<T>`

```rust
struct EhFrameOffset<T>(T);
```

An offset into the `.eh_frame` section.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for EhFrameOffset<T>`

- `fn clone(self: &Self) -> EhFrameOffset<T>` — [`EhFrameOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for EhFrameOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for EhFrameOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for EhFrameOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for EhFrameOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for EhFrameOffset<T>`

- `fn eq(self: &Self, other: &EhFrameOffset<T>) -> bool` — [`EhFrameOffset`](../index.md)

##### `impl<T> StructuralPartialEq for EhFrameOffset<T>`

##### `impl<T> UnwindOffset for crate::common::EhFrameOffset<T>`

- `fn into(self: Self) -> T`

### `DwoId`

```rust
struct DwoId(u64);
```

An optionally-provided implementation-defined compilation unit ID to enable
split DWARF and linking a split compilation unit back together.

#### Trait Implementations

##### `impl Clone for DwoId`

- `fn clone(self: &Self) -> DwoId` — [`DwoId`](../index.md)

##### `impl Copy for DwoId`

##### `impl Debug for DwoId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for DwoId`

##### `impl Hash for DwoId`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for DwoId`

- `fn eq(self: &Self, other: &DwoId) -> bool` — [`DwoId`](../index.md)

##### `impl StructuralPartialEq for DwoId`

## Enums

### `Format`

```rust
enum Format {
    Dwarf64,
    Dwarf32,
}
```

Whether the format of a compilation unit is 32- or 64-bit.

#### Variants

- **`Dwarf64`**

  64-bit DWARF

- **`Dwarf32`**

  32-bit DWARF

#### Implementations

- `fn initial_length_size(self: Self) -> u8`

- `fn word_size(self: Self) -> u8`

#### Trait Implementations

##### `impl Clone for Format`

- `fn clone(self: &Self) -> Format` — [`Format`](../index.md)

##### `impl Copy for Format`

##### `impl Debug for Format`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Format`

##### `impl Hash for Format`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Format`

- `fn eq(self: &Self, other: &Format) -> bool` — [`Format`](../index.md)

##### `impl StructuralPartialEq for Format`

### `Vendor`

```rust
enum Vendor {
    Default,
    AArch64,
}
```

Which vendor extensions to support.

#### Variants

- **`Default`**

  A default set of extensions, including some common GNU extensions.

- **`AArch64`**

  AAarch64 extensions.

#### Trait Implementations

##### `impl Clone for Vendor`

- `fn clone(self: &Self) -> Vendor` — [`Vendor`](../index.md)

##### `impl Copy for Vendor`

##### `impl Debug for Vendor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Vendor`

##### `impl PartialEq for Vendor`

- `fn eq(self: &Self, other: &Vendor) -> bool` — [`Vendor`](../index.md)

##### `impl StructuralPartialEq for Vendor`

### `UnitSectionOffset<T>`

```rust
enum UnitSectionOffset<T> {
    DebugInfoOffset(DebugInfoOffset<T>),
    DebugTypesOffset(DebugTypesOffset<T>),
}
```

An offset into the `.debug_info` or `.debug_types` sections.

#### Variants

- **`DebugInfoOffset`**

  An offset into the `.debug_info` section.

- **`DebugTypesOffset`**

  An offset into the `.debug_types` section.

#### Implementations

- `fn to_unit_offset<R>(self: &Self, unit: &Unit<R>) -> Option<UnitOffset<T>>` — [`Unit`](../read/index.md), [`UnitOffset`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for UnitSectionOffset<T>`

- `fn clone(self: &Self) -> UnitSectionOffset<T>` — [`UnitSectionOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for UnitSectionOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for UnitSectionOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for UnitSectionOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for UnitSectionOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::Ord> Ord for UnitSectionOffset<T>`

- `fn cmp(self: &Self, other: &UnitSectionOffset<T>) -> $crate::cmp::Ordering` — [`UnitSectionOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for UnitSectionOffset<T>`

- `fn eq(self: &Self, other: &UnitSectionOffset<T>) -> bool` — [`UnitSectionOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialOrd> PartialOrd for UnitSectionOffset<T>`

- `fn partial_cmp(self: &Self, other: &UnitSectionOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`UnitSectionOffset`](../index.md)

##### `impl<T> StructuralPartialEq for UnitSectionOffset<T>`

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

- `fn name(self: Self) -> &'static str`

- `fn dwo_name(self: Self) -> Option<&'static str>`

- `fn xcoff_name(self: Self) -> Option<&'static str>`

- `fn is_string(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for SectionId`

- `fn clone(self: &Self) -> SectionId` — [`SectionId`](../index.md)

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SectionId`

##### `impl Hash for SectionId`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for SectionId`

- `fn cmp(self: &Self, other: &SectionId) -> $crate::cmp::Ordering` — [`SectionId`](../index.md)

##### `impl PartialEq for SectionId`

- `fn eq(self: &Self, other: &SectionId) -> bool` — [`SectionId`](../index.md)

##### `impl PartialOrd for SectionId`

- `fn partial_cmp(self: &Self, other: &SectionId) -> $crate::option::Option<$crate::cmp::Ordering>` — [`SectionId`](../index.md)

##### `impl StructuralPartialEq for SectionId`

### `DwarfFileType`

```rust
enum DwarfFileType {
    Main,
    Dwo,
}
```

The "type" of file with DWARF debugging information. This determines, among other things,
which files DWARF sections should be loaded from.

#### Variants

- **`Main`**

  A normal executable or object file.

- **`Dwo`**

  A .dwo split DWARF file.

#### Trait Implementations

##### `impl Clone for DwarfFileType`

- `fn clone(self: &Self) -> DwarfFileType` — [`DwarfFileType`](../index.md)

##### `impl Copy for DwarfFileType`

##### `impl Debug for DwarfFileType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DwarfFileType`

- `fn default() -> Self`

##### `impl Eq for DwarfFileType`

##### `impl PartialEq for DwarfFileType`

- `fn eq(self: &Self, other: &DwarfFileType) -> bool` — [`DwarfFileType`](../index.md)

##### `impl StructuralPartialEq for DwarfFileType`

