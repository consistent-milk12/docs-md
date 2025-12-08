*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [version](index.md)*

---

# Module `version`

## Structs

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

A version index.

#### Implementations

- `fn index(self: &Self) -> u16`

- `fn is_local(self: &Self) -> bool`

- `fn is_global(self: &Self) -> bool`

- `fn is_hidden(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for VersionIndex`

- `fn clone(self: &Self) -> VersionIndex` — [`VersionIndex`](../index.md)

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for VersionIndex`

- `fn default() -> VersionIndex` — [`VersionIndex`](../index.md)

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

- `fn name(self: &Self) -> &'data [u8]`

- `fn hash(self: &Self) -> u32`

- `fn file(self: &Self) -> Option<&'data [u8]>`

#### Trait Implementations

##### `impl<'data> Clone for Version<'data>`

- `fn clone(self: &Self) -> Version<'data>` — [`Version`](../index.md)

##### `impl<'data> Copy for Version<'data>`

##### `impl<'data> Debug for Version<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for Version<'data>`

- `fn default() -> Version<'data>` — [`Version`](../index.md)

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

- `fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](../index.md), [`Versym`](../../../elf/index.md), [`VerdefIterator`](../index.md), [`VerneedIterator`](../index.md), [`StringTable`](../../index.md), [`Result`](../../../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn version_index(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md), [`VersionIndex`](../index.md)

- `fn version(self: &Self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](../index.md), [`Result`](../../../index.md), [`Version`](../index.md)

- `fn matches(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md), [`Version`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- `fn clone(self: &Self) -> VersionTable<'data, Elf>` — [`VersionTable`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf: FileHeader> Default for VersionTable<'data, Elf>`

- `fn default() -> Self`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md), [`Verdef`](../../../elf/index.md), [`FileHeader`](../index.md), [`VerdauxIterator`](../index.md)

- `fn parse(self: &mut Self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../../index.md), [`Verdef`](../../../elf/index.md), [`FileHeader`](../index.md), [`VerdauxIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- `fn clone(self: &Self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VerdefIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- `type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md), [`Verdaux`](../../../elf/index.md), [`FileHeader`](../index.md)

- `fn parse(self: &mut Self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../../index.md), [`Verdaux`](../../../elf/index.md), [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- `fn clone(self: &Self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VerdauxIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- `type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md), [`Verneed`](../../../elf/index.md), [`FileHeader`](../index.md), [`VernauxIterator`](../index.md)

- `fn parse(self: &mut Self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../../index.md), [`Verneed`](../../../elf/index.md), [`FileHeader`](../index.md), [`VernauxIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- `fn clone(self: &Self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VerneedIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- `type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md), [`Vernaux`](../../../elf/index.md), [`FileHeader`](../index.md)

- `fn parse(self: &mut Self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../../index.md), [`Vernaux`](../../../elf/index.md), [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- `fn clone(self: &Self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for VernauxIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- `type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

