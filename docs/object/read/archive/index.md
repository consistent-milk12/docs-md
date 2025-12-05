*[object](../../index.md) / [read](../index.md) / [archive](index.md)*

---

# Module `archive`

Support for archive files.

## Example
 ```no_run
use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

/// Reads an archive and displays the name of each member.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let file = object::read::archive::ArchiveFile::parse(&*data)?;
    for member in file.members() {
        let member = member?;
        println!("{}", String::from_utf8_lossy(member.name()));
    }
  }
    Ok(())
}
```

## Structs

### `ArchiveFile<'data, R: ReadRef<'data>>`

```rust
struct ArchiveFile<'data, R: ReadRef<'data>> {
    data: R,
    kind: ArchiveKind,
    members: Members<'data>,
    symbols: (u64, u64),
    names: &'data [u8],
    thin: bool,
}
```

A partially parsed archive file.

#### Implementations

- `fn parse(data: R) -> read::Result<Self>` — [`Result`](../../../read/index.md)

- `fn parse_aixbig(data: R) -> read::Result<Self>` — [`Result`](../../../read/index.md)

- `fn kind(self: &Self) -> ArchiveKind` — [`ArchiveKind`](../../../read/archive/index.md)

- `fn is_thin(self: &Self) -> bool`

- `fn members(self: &Self) -> ArchiveMemberIterator<'data, R>` — [`ArchiveMemberIterator`](../../../read/archive/index.md)

- `fn member(self: &Self, member: ArchiveOffset) -> read::Result<ArchiveMember<'data>>` — [`ArchiveOffset`](../../../read/archive/index.md), [`Result`](../../../read/index.md), [`ArchiveMember`](../../../read/archive/index.md)

- `fn symbols(self: &Self) -> read::Result<Option<ArchiveSymbolIterator<'data>>>` — [`Result`](../../../read/index.md), [`ArchiveSymbolIterator`](../../../read/archive/index.md)

#### Trait Implementations

##### `impl Clone<'data, R: $crate::clone::Clone + ReadRef<'data>>`

- `fn clone(self: &Self) -> ArchiveFile<'data, R>` — [`ArchiveFile`](../../../read/archive/index.md)

##### `impl Copy<'data, R: $crate::marker::Copy + ReadRef<'data>>`

##### `impl Debug<'data, R: $crate::fmt::Debug + ReadRef<'data>>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArchiveMemberIterator<'data, R: ReadRef<'data>>`

```rust
struct ArchiveMemberIterator<'data, R: ReadRef<'data>> {
    data: R,
    members: Members<'data>,
    names: &'data [u8],
    thin: bool,
}
```

An iterator over the members of an archive.

#### Trait Implementations

##### `impl Debug<'data, R: $crate::fmt::Debug + ReadRef<'data>>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'data, R: ReadRef<'data>>`

- `type Item = Result<ArchiveMember<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ArchiveMember<'data>`

```rust
struct ArchiveMember<'data> {
    header: MemberHeader<'data>,
    name: &'data [u8],
    offset: u64,
    size: u64,
}
```

A partially parsed archive member.

#### Implementations

- `fn parse<R: ReadRef<'data>>(data: R, offset: &mut u64, names: &'data [u8], thin: bool) -> read::Result<Self>` — [`Result`](../../../read/index.md)

- `fn parse_aixbig_index<R: ReadRef<'data>>(data: R, index: &archive::AixMemberOffset) -> read::Result<Self>` — [`AixMemberOffset`](../../../archive/index.md), [`Result`](../../../read/index.md)

- `fn parse_aixbig<R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<Self>` — [`Result`](../../../read/index.md)

- `fn header(self: &Self) -> Option<&'data archive::Header>` — [`Header`](../../../archive/index.md)

- `fn aix_header(self: &Self) -> Option<&'data archive::AixHeader>` — [`AixHeader`](../../../archive/index.md)

- `fn name(self: &Self) -> &'data [u8]`

- `fn date(self: &Self) -> Option<u64>`

- `fn uid(self: &Self) -> Option<u64>`

- `fn gid(self: &Self) -> Option<u64>`

- `fn mode(self: &Self) -> Option<u64>`

- `fn size(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn is_thin(self: &Self) -> bool`

- `fn data<R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<&'data [u8]>` — [`Result`](../../../read/index.md)

#### Trait Implementations

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArchiveOffset`

```rust
struct ArchiveOffset(u64);
```

An offset of a member in an archive.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ArchiveOffset` — [`ArchiveOffset`](../../../read/archive/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArchiveSymbolIterator<'data>`

```rust
struct ArchiveSymbolIterator<'data>(SymbolIteratorInternal<'data>);
```

An iterator over the symbols in the archive symbol table.

#### Implementations

- `fn new<R: ReadRef<'data>>(kind: ArchiveKind, data: R, offset: u64, size: u64) -> Result<Self, ()>` — [`ArchiveKind`](../../../read/archive/index.md)

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ArchiveSymbolIterator<'data>` — [`ArchiveSymbolIterator`](../../../read/archive/index.md)

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'data>`

- `type Item = Result<ArchiveSymbol<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `ArchiveSymbol<'data>`

```rust
struct ArchiveSymbol<'data> {
    name: &'data [u8],
    offset: ArchiveOffset,
}
```

A symbol in the archive symbol table.

This is used to find the member containing the symbol.

#### Implementations

- `fn name(self: &Self) -> &'data [u8]`

- `fn offset(self: &Self) -> ArchiveOffset` — [`ArchiveOffset`](../../../read/archive/index.md)

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ArchiveSymbol<'data>` — [`ArchiveSymbol`](../../../read/archive/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `ArchiveKind`

```rust
enum ArchiveKind {
    Unknown,
    Gnu,
    Gnu64,
    Bsd,
    Bsd64,
    Coff,
    AixBig,
}
```

The kind of archive format.

#### Variants

- **`Unknown`**

  There are no special files that indicate the archive format.

- **`Gnu`**

  The GNU (or System V) archive format.

- **`Gnu64`**

  The GNU (or System V) archive format with 64-bit symbol table.

- **`Bsd`**

  The BSD archive format.

- **`Bsd64`**

  The BSD archive format with 64-bit symbol table.
  
  This is used for Darwin.

- **`Coff`**

  The Windows COFF archive format.

- **`AixBig`**

  The AIX big archive format.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ArchiveKind` — [`ArchiveKind`](../../../read/archive/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArchiveKind) -> bool` — [`ArchiveKind`](../../../read/archive/index.md)

##### `impl StructuralPartialEq`

