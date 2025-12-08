*[gimli](../../index.md) / [read](../index.md) / [addr](index.md)*

---

# Module `addr`

## Structs

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

The raw contents of the `.debug_addr` section.

#### Implementations

- `fn get_address(self: &Self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../../index.md), [`Reader`](../index.md), [`DebugAddrIndex`](../../index.md), [`Result`](../../index.md)

- `fn headers(self: &Self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugAddr<R>`

- `fn clone(self: &Self) -> DebugAddr<R>` — [`DebugAddr`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugAddr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugAddr<R>`

- `fn default() -> DebugAddr<R>` — [`DebugAddr`](../index.md)

##### `impl<R> Section for DebugAddr<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../../index.md), [`AddrHeader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- `fn clone(self: &Self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AddrHeader<R, Offset>`

```rust
struct AddrHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugAddrOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    entries: R,
}
```

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- `fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../../index.md), [`Result`](../../index.md)

- `fn offset(self: &Self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../../index.md)

- `fn length(self: &Self) -> Offset`

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../../index.md)

- `fn entries(self: &Self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- `fn clone(self: &Self) -> AddrHeader<R, Offset>` — [`AddrHeader`](../index.md)

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- `fn eq(self: &Self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the addresses from a `.debug_addr` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<u64>>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for AddrEntryIter<R>`

- `fn clone(self: &Self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

