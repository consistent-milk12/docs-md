*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [version](index.md)*

---

# Module `version`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VersionIndex`](#versionindex) | struct | A version index. |
| [`Version`](#version) | struct | A version definition or requirement. |
| [`VersionTable`](#versiontable) | struct | A table of version definitions and requirements. |
| [`VerdefIterator`](#verdefiterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerdauxIterator`](#verdauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerneedIterator`](#verneediterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section. |
| [`VernauxIterator`](#vernauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section. |

## Structs

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

A version index.

#### Implementations

- <span id="versionindex-index"></span>`fn index(&self) -> u16`

- <span id="versionindex-is-local"></span>`fn is_local(&self) -> bool`

- <span id="versionindex-is-global"></span>`fn is_global(&self) -> bool`

- <span id="versionindex-is-hidden"></span>`fn is_hidden(&self) -> bool`

#### Trait Implementations

##### `impl Clone for VersionIndex`

- <span id="versionindex-clone"></span>`fn clone(&self) -> VersionIndex` — [`VersionIndex`](../index.md)

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- <span id="versionindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VersionIndex`

- <span id="versionindex-default"></span>`fn default() -> VersionIndex` — [`VersionIndex`](../index.md)

### `Version<'data>`

```rust
struct Version<'data> {
    name: &'data [u8],
    hash: u32,
    valid: bool,
    file: Option<&'data [u8]>,
}
```

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) and [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) sections.

#### Implementations

- <span id="version-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="version-hash"></span>`fn hash(&self) -> u32`

- <span id="version-file"></span>`fn file(&self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for Version<'data>`

- <span id="version-clone"></span>`fn clone(&self) -> Version<'data>` — [`Version`](../index.md)

##### `impl<'data> Copy for Version<'data>`

##### `impl<'data> Debug for Version<'data>`

- <span id="version-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Default for Version<'data>`

- <span id="version-default"></span>`fn default() -> Version<'data>` — [`Version`](../index.md)

### `VersionTable<'data, Elf: FileHeader>`

```rust
struct VersionTable<'data, Elf: FileHeader> {
    symbols: &'data [elf::Versym<<Elf as >::Endian>],
    versions: alloc::vec::Vec<Version<'data>>,
}
```

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`](../../../elf/index.md), [`elf::SHT_GNU_VERDEF`](../../../elf/index.md)
and [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

#### Implementations

- <span id="versiontable-parse"></span>`fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](../index.md), [`Versym`](../../../elf/index.md), [`VerdefIterator`](../index.md), [`VerneedIterator`](../index.md), [`StringTable`](../../index.md), [`Result`](../../../index.md)

- <span id="versiontable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="versiontable-version-index"></span>`fn version_index(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md), [`VersionIndex`](../index.md)

- <span id="versiontable-version"></span>`fn version(&self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](../index.md), [`Result`](../../../index.md), [`Version`](../index.md)

- <span id="versiontable-matches"></span>`fn matches(&self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md), [`Version`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- <span id="versiontable-clone"></span>`fn clone(&self) -> VersionTable<'data, Elf>` — [`VersionTable`](../index.md)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- <span id="versiontable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Elf: FileHeader> Default for VersionTable<'data, Elf>`

- <span id="versiontable-default"></span>`fn default() -> Self`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- <span id="verdefiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md)

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md), [`Verdef`](../../../elf/index.md), [`FileHeader`](../index.md), [`VerdauxIterator`](../index.md)

- <span id="verdefiterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../../index.md), [`Verdef`](../../../elf/index.md), [`FileHeader`](../index.md), [`VerdauxIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clone"></span>`fn clone(&self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](../index.md)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdefiterator-intoiter"></span>`type IntoIter = I`

- <span id="verdefiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-item"></span>`type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VerdauxIterator<'data, Elf: FileHeader>`

```rust
struct VerdauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- <span id="verdauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md)

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md), [`Verdaux`](../../../elf/index.md), [`FileHeader`](../index.md)

- <span id="verdauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../../index.md), [`Verdaux`](../../../elf/index.md), [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clone"></span>`fn clone(&self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](../index.md)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdauxiterator-intoiter"></span>`type IntoIter = I`

- <span id="verdauxiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-item"></span>`type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- <span id="verneediterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md)

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md), [`Verneed`](../../../elf/index.md), [`FileHeader`](../index.md), [`VernauxIterator`](../index.md)

- <span id="verneediterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../../index.md), [`Verneed`](../../../elf/index.md), [`FileHeader`](../index.md), [`VernauxIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clone"></span>`fn clone(&self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](../index.md)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- <span id="verneediterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verneediterator-intoiter"></span>`type IntoIter = I`

- <span id="verneediterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-item"></span>`type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VernauxIterator<'data, Elf: FileHeader>`

```rust
struct VernauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- <span id="vernauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md)

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md), [`Vernaux`](../../../elf/index.md), [`FileHeader`](../index.md)

- <span id="vernauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../../index.md), [`Vernaux`](../../../elf/index.md), [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clone"></span>`fn clone(&self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](../index.md)

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="vernauxiterator-intoiter"></span>`type IntoIter = I`

- <span id="vernauxiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-item"></span>`type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

