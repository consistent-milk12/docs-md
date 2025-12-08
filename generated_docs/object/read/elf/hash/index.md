*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [hash](index.md)*

---

# Module `hash`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HashTable`](#hashtable) | struct | A SysV symbol hash table in an ELF file. |
| [`GnuHashTable`](#gnuhashtable) | struct | A GNU symbol hash table in an ELF file. |

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

- <span id="hashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- <span id="hashtable-symbol-table-length"></span>`fn symbol_table_length(&self) -> u32`

- <span id="hashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- <span id="hashtable-chain"></span>`fn chain(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> SymbolIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- <span id="hashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](../index.md), [`Version`](../index.md), [`SymbolTable`](../index.md), [`VersionTable`](../index.md), [`SymbolIndex`](../../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for HashTable<'data, Elf>`

- <span id="hashtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="gnuhashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- <span id="gnuhashtable-symbol-base"></span>`fn symbol_base(&self) -> u32`

- <span id="gnuhashtable-symbol-table-length"></span>`fn symbol_table_length(&self, endian: <Elf as >::Endian) -> Option<u32>` — [`FileHeader`](../index.md)

- <span id="gnuhashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- <span id="gnuhashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](../index.md), [`Version`](../index.md), [`SymbolTable`](../index.md), [`VersionTable`](../index.md), [`SymbolIndex`](../../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

