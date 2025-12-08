*[gimli](../../index.md) / [read](../index.md) / [aranges](index.md)*

---

# Module `aranges`

## Structs

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugAranges<R>`

- `fn clone(self: &Self) -> DebugAranges<R>` — [`DebugAranges`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugAranges<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugAranges<R>`

- `fn default() -> DebugAranges<R>` — [`DebugAranges`](../index.md)

##### `impl<R> Section for DebugAranges<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../../index.md), [`ArangeHeader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- `fn clone(self: &Self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArangeHeader<R, Offset>`

```rust
struct ArangeHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugArangesOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    debug_info_offset: crate::common::DebugInfoOffset<Offset>,
    entries: R,
}
```

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- `fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../../index.md), [`Result`](../../index.md)

- `fn offset(self: &Self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../../index.md)

- `fn length(self: &Self) -> Offset`

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../../index.md)

- `fn debug_info_offset(self: &Self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../../index.md)

- `fn entries(self: &Self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- `fn clone(self: &Self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](../index.md)

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- `fn eq(self: &Self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the aranges from a `.debug_aranges` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md), [`ArangeEntry`](../index.md)

- `fn next_raw(self: &mut Self) -> Result<Option<ArangeEntry>>` — [`Result`](../../index.md), [`ArangeEntry`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- `fn clone(self: &Self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

A single parsed arange.

#### Implementations

- `fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../../index.md), [`Result`](../../index.md)

- `fn address(self: &Self) -> u64`

- `fn length(self: &Self) -> u64`

- `fn range(self: &Self) -> Range` — [`Range`](../index.md)

#### Trait Implementations

##### `impl Clone for ArangeEntry`

- `fn clone(self: &Self) -> ArangeEntry` — [`ArangeEntry`](../index.md)

##### `impl Debug for ArangeEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl Ord for ArangeEntry`

- `fn cmp(self: &Self, other: &ArangeEntry) -> $crate::cmp::Ordering` — [`ArangeEntry`](../index.md)

##### `impl PartialEq for ArangeEntry`

- `fn eq(self: &Self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](../index.md)

##### `impl PartialOrd for ArangeEntry`

- `fn partial_cmp(self: &Self, other: &ArangeEntry) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ArangeEntry`](../index.md)

##### `impl StructuralPartialEq for ArangeEntry`

