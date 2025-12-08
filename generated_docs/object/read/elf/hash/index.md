*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [hash](index.md)*

---

# Module `hash`

## Structs

### `HashTable<'data, Elf: FileHeader>`

```rust
struct HashTable<'data, Elf: FileHeader> {
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    chains: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A SysV symbol hash table in an ELF file.

Returned by [`SectionHeader::hash`](super::SectionHeader::hash).

#### Implementations

- `fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- `fn symbol_table_length(self: &Self) -> u32`

- `fn bucket(self: &Self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- `fn chain(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> SymbolIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- `fn find<R: ReadRef<'data>>(self: &Self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](../index.md), [`Version`](../index.md), [`SymbolTable`](../index.md), [`VersionTable`](../index.md), [`SymbolIndex`](../../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for HashTable<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GnuHashTable<'data, Elf: FileHeader>`

```rust
struct GnuHashTable<'data, Elf: FileHeader> {
    symbol_base: u32,
    bloom_shift: u32,
    bloom_filters: &'data [u8],
    buckets: &'data [crate::endian::U32<<Elf as >::Endian>],
    values: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A GNU symbol hash table in an ELF file.

Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash).

#### Implementations

- `fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- `fn symbol_base(self: &Self) -> u32`

- `fn symbol_table_length(self: &Self, endian: <Elf as >::Endian) -> Option<u32>` — [`FileHeader`](../index.md)

- `fn bucket(self: &Self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- `fn find<R: ReadRef<'data>>(self: &Self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](../index.md), [`Version`](../index.md), [`SymbolTable`](../index.md), [`VersionTable`](../index.md), [`SymbolIndex`](../../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for GnuHashTable<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

