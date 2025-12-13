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

*Defined in [`object-0.37.3/src/read/elf/hash.rs:13-16`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/hash.rs#L13-L16)*

A SysV symbol hash table in an ELF file.

Returned by [`SectionHeader::hash`](super::SectionHeader::hash).

#### Implementations

- <span id="hashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

  Parse a SysV hash table.

  

  `data` should be from an [`elf::SHT_HASH`](../../../elf/index.md) section, or from a

  segment pointed to via the [`elf::DT_HASH`](../../../elf/index.md) entry.

  

  The header is read at offset 0 in the given `data`.

- <span id="hashtable-symbol-table-length"></span>`fn symbol_table_length(&self) -> u32`

  Return the symbol table length.

- <span id="hashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="hashtable-chain"></span>`fn chain(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> SymbolIndex` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="hashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](../index.md#fileheader), [`Version`](../index.md#version), [`SymbolTable`](../index.md#symboltable), [`VersionTable`](../index.md#versiontable), [`SymbolIndex`](../../../index.md#symbolindex)

  Use the hash table to find the symbol table entry with the given name, hash and version.

#### Trait Implementations

##### `impl Any for HashTable<'data, Elf>`

- <span id="hashtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HashTable<'data, Elf>`

- <span id="hashtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HashTable<'data, Elf>`

- <span id="hashtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for HashTable<'data, Elf>`

- <span id="hashtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HashTable<'data, Elf>`

- <span id="hashtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HashTable<'data, Elf>`

- <span id="hashtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HashTable<'data, Elf>`

- <span id="hashtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hashtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HashTable<'data, Elf>`

- <span id="hashtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hashtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/hash.rs:86-92`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/hash.rs#L86-L92)*

A GNU symbol hash table in an ELF file.

Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash).

#### Implementations

- <span id="gnuhashtable-parse"></span>`fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

  Parse a GNU hash table.

  

  `data` should be from an [`elf::SHT_GNU_HASH`](../../../elf/index.md) section, or from a

  segment pointed to via the [`elf::DT_GNU_HASH`](../../../elf/index.md) entry.

  

  The header is read at offset 0 in the given `data`.

  

  The header does not contain a length field, and so all of `data`

  will be used as the hash table values. It does not matter if this

  is longer than needed, and this will often the case when accessing

  the hash table via the [`elf::DT_GNU_HASH`](../../../elf/index.md) entry.

- <span id="gnuhashtable-symbol-base"></span>`fn symbol_base(&self) -> u32`

  Return the symbol table index of the first symbol in the hash table.

- <span id="gnuhashtable-symbol-table-length"></span>`fn symbol_table_length(&self, endian: <Elf as >::Endian) -> Option<u32>` — [`FileHeader`](../index.md#fileheader)

  Determine the symbol table length by finding the last entry in the hash table.

  

  Returns `None` if the hash table is empty or invalid.

- <span id="gnuhashtable-bucket"></span>`fn bucket(&self, endian: <Elf as >::Endian, hash: u32) -> SymbolIndex` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="gnuhashtable-find"></span>`fn find<R: ReadRef<'data>>(&self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version<'_>>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` — [`FileHeader`](../index.md#fileheader), [`Version`](../index.md#version), [`SymbolTable`](../index.md#symboltable), [`VersionTable`](../index.md#versiontable), [`SymbolIndex`](../../../index.md#symbolindex)

  Use the hash table to find the symbol table entry with the given name, hash, and version.

#### Trait Implementations

##### `impl Any for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gnuhashtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GnuHashTable<'data, Elf>`

- <span id="gnuhashtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gnuhashtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

