*[gimli](../../index.md) / [read](../index.md) / [addr](index.md)*

---

# Module `addr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAddr`](#debugaddr) | struct | The raw contents of the `.debug_addr` section. |
| [`AddrHeaderIter`](#addrheaderiter) | struct | An iterator over the headers of a `.debug_addr` section. |
| [`AddrHeader`](#addrheader) | struct | A header for a set of entries in the `.debug_addr` section. |
| [`AddrEntryIter`](#addrentryiter) | struct | An iterator over the addresses from a `.debug_addr` section. |

## Structs

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:6-8`](../../../../.source_1765210505/gimli-0.32.3/src/read/addr.rs#L6-L8)*

The raw contents of the `.debug_addr` section.

#### Implementations

- <span id="debugaddr-get-address"></span>`fn get_address(&self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../../index.md), [`Reader`](../index.md), [`DebugAddrIndex`](../../index.md), [`Result`](../../index.md)

- <span id="debugaddr-headers"></span>`fn headers(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAddr<R>`

- <span id="debugaddr-clone"></span>`fn clone(&self) -> DebugAddr<R>` — [`DebugAddr`](../index.md)

##### `impl<R: marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: fmt::Debug> Debug for DebugAddr<R>`

- <span id="debugaddr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAddr<R>`

- <span id="debugaddr-default"></span>`fn default() -> DebugAddr<R>` — [`DebugAddr`](../index.md)

##### `impl<R> Section for DebugAddr<R>`

- <span id="debugaddr-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md)

- <span id="debugaddr-reader"></span>`fn reader(&self) -> &R`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:82-85`](../../../../.source_1765210505/gimli-0.32.3/src/read/addr.rs#L82-L85)*

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- <span id="addrheaderiter-next"></span>`fn next(&mut self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../../index.md), [`AddrHeader`](../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- <span id="addrheaderiter-clone"></span>`fn clone(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](../index.md)

##### `impl<R: fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- <span id="addrheaderiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`gimli-0.32.3/src/read/addr.rs:122-131`](../../../../.source_1765210505/gimli-0.32.3/src/read/addr.rs#L122-L131)*

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- <span id="addrheader-parse"></span>`fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../../index.md), [`Result`](../../index.md)

- <span id="addrheader-offset"></span>`fn offset(&self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../../index.md)

- <span id="addrheader-length"></span>`fn length(&self) -> Offset`

- <span id="addrheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md)

- <span id="addrheader-entries"></span>`fn entries(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- <span id="addrheader-clone"></span>`fn clone(&self) -> AddrHeader<R, Offset>` — [`AddrHeader`](../index.md)

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- <span id="addrheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- <span id="addrheader-eq"></span>`fn eq(&self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:217-220`](../../../../.source_1765210505/gimli-0.32.3/src/read/addr.rs#L217-L220)*

An iterator over the addresses from a `.debug_addr` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="addrentryiter-next"></span>`fn next(&mut self) -> Result<Option<u64>>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for AddrEntryIter<R>`

- <span id="addrentryiter-clone"></span>`fn clone(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](../index.md)

##### `impl<R: fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- <span id="addrentryiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

