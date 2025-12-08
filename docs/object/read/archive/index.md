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

- `fn parse(data: R) -> read::Result<Self>` — [`Result`](../../index.md)

- `fn parse_aixbig(data: R) -> read::Result<Self>` — [`Result`](../../index.md)

- `fn kind(self: &Self) -> ArchiveKind` — [`ArchiveKind`](#archivekind)

- `fn is_thin(self: &Self) -> bool`

- `fn members(self: &Self) -> ArchiveMemberIterator<'data, R>` — [`ArchiveMemberIterator`](#archivememberiterator)

- `fn member(self: &Self, member: ArchiveOffset) -> read::Result<ArchiveMember<'data>>` — [`ArchiveOffset`](#archiveoffset), [`Result`](../../index.md), [`ArchiveMember`](#archivemember)

- `fn symbols(self: &Self) -> read::Result<Option<ArchiveSymbolIterator<'data>>>` — [`Result`](../../index.md), [`ArchiveSymbolIterator`](#archivesymboliterator)

#### Trait Implementations

##### `impl<'data, R: $crate::clone::Clone + ReadRef<'data>> Clone for ArchiveFile<'data, R>`

- `fn clone(self: &Self) -> ArchiveFile<'data, R>` — [`ArchiveFile`](#archivefile)

##### `impl<'data, R: $crate::marker::Copy + ReadRef<'data>> Copy for ArchiveFile<'data, R>`

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ArchiveFile<'data, R>`

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

##### `impl<'data, R: $crate::fmt::Debug + ReadRef<'data>> Debug for ArchiveMemberIterator<'data, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ArchiveMemberIterator<'data, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, R: ReadRef<'data>> Iterator for ArchiveMemberIterator<'data, R>`

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

- `fn parse<R: ReadRef<'data>>(data: R, offset: &mut u64, names: &'data [u8], thin: bool) -> read::Result<Self>` — [`Result`](../../index.md)

- `fn parse_aixbig_index<R: ReadRef<'data>>(data: R, index: &archive::AixMemberOffset) -> read::Result<Self>` — [`AixMemberOffset`](../../archive/index.md), [`Result`](../../index.md)

- `fn parse_aixbig<R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<Self>` — [`Result`](../../index.md)

- `fn header(self: &Self) -> Option<&'data archive::Header>` — [`Header`](../../archive/index.md)

- `fn aix_header(self: &Self) -> Option<&'data archive::AixHeader>` — [`AixHeader`](../../archive/index.md)

- `fn name(self: &Self) -> &'data [u8]`

- `fn date(self: &Self) -> Option<u64>`

- `fn uid(self: &Self) -> Option<u64>`

- `fn gid(self: &Self) -> Option<u64>`

- `fn mode(self: &Self) -> Option<u64>`

- `fn size(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn is_thin(self: &Self) -> bool`

- `fn data<R: ReadRef<'data>>(self: &Self, data: R) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Debug for ArchiveMember<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArchiveOffset`

```rust
struct ArchiveOffset(u64);
```

An offset of a member in an archive.

#### Trait Implementations

##### `impl Clone for ArchiveOffset`

- `fn clone(self: &Self) -> ArchiveOffset` — [`ArchiveOffset`](#archiveoffset)

##### `impl Copy for ArchiveOffset`

##### `impl Debug for ArchiveOffset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArchiveSymbolIterator<'data>`

```rust
struct ArchiveSymbolIterator<'data>(SymbolIteratorInternal<'data>);
```

An iterator over the symbols in the archive symbol table.

#### Implementations

- `fn new<R: ReadRef<'data>>(kind: ArchiveKind, data: R, offset: u64, size: u64) -> Result<Self, ()>` — [`ArchiveKind`](#archivekind)

#### Trait Implementations

##### `impl<'data> Clone for ArchiveSymbolIterator<'data>`

- `fn clone(self: &Self) -> ArchiveSymbolIterator<'data>` — [`ArchiveSymbolIterator`](#archivesymboliterator)

##### `impl<'data> Debug for ArchiveSymbolIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ArchiveSymbolIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for ArchiveSymbolIterator<'data>`

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

- `fn offset(self: &Self) -> ArchiveOffset` — [`ArchiveOffset`](#archiveoffset)

#### Trait Implementations

##### `impl<'data> Clone for ArchiveSymbol<'data>`

- `fn clone(self: &Self) -> ArchiveSymbol<'data>` — [`ArchiveSymbol`](#archivesymbol)

##### `impl<'data> Copy for ArchiveSymbol<'data>`

##### `impl<'data> Debug for ArchiveSymbol<'data>`

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

##### `impl Clone for ArchiveKind`

- `fn clone(self: &Self) -> ArchiveKind` — [`ArchiveKind`](#archivekind)

##### `impl Copy for ArchiveKind`

##### `impl Debug for ArchiveKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ArchiveKind`

##### `impl Hash for ArchiveKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ArchiveKind`

- `fn eq(self: &Self, other: &ArchiveKind) -> bool` — [`ArchiveKind`](#archivekind)

##### `impl StructuralPartialEq for ArchiveKind`

### `Members<'data>`

```rust
enum Members<'data> {
    Common {
        offset: u64,
        end_offset: u64,
    },
    AixBig {
        index: &'data [archive::AixMemberOffset],
    },
}
```

The list of members in the archive.

#### Trait Implementations

##### `impl<'data> Clone for Members<'data>`

- `fn clone(self: &Self) -> Members<'data>` — [`Members`](#members)

##### `impl<'data> Copy for Members<'data>`

##### `impl<'data> Debug for Members<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MemberHeader<'data>`

```rust
enum MemberHeader<'data> {
    Common(&'data archive::Header),
    AixBig(&'data archive::AixHeader),
}
```

An archive member header.

#### Variants

- **`Common`**

  Common header used by many formats.

- **`AixBig`**

  AIX big archive header

#### Trait Implementations

##### `impl<'data> Clone for MemberHeader<'data>`

- `fn clone(self: &Self) -> MemberHeader<'data>` — [`MemberHeader`](#memberheader)

##### `impl<'data> Copy for MemberHeader<'data>`

##### `impl<'data> Debug for MemberHeader<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SymbolIteratorInternal<'data>`

```rust
enum SymbolIteratorInternal<'data> {
    None,
    Gnu {
        offsets: slice::Iter<'data, crate::endian::U32Bytes<crate::endian::BigEndian>>,
        names: crate::read::Bytes<'data>,
    },
    Gnu64 {
        offsets: slice::Iter<'data, crate::endian::U64Bytes<crate::endian::BigEndian>>,
        names: crate::read::Bytes<'data>,
    },
    Bsd {
        offsets: slice::Iter<'data, [crate::endian::U32Bytes<crate::endian::LittleEndian>; 2]>,
        names: crate::read::Bytes<'data>,
    },
    Bsd64 {
        offsets: slice::Iter<'data, [crate::endian::U64Bytes<crate::endian::LittleEndian>; 2]>,
        names: crate::read::Bytes<'data>,
    },
    Coff {
        members: &'data [crate::endian::U32Bytes<crate::endian::LittleEndian>],
        indices: slice::Iter<'data, crate::endian::U16Bytes<crate::endian::LittleEndian>>,
        names: crate::read::Bytes<'data>,
    },
}
```

#### Variants

- **`None`**

  There is no symbol table.

- **`Gnu`**

  A GNU symbol table.
  
  Contains:
  - the number of symbols as a 32-bit big-endian integer
  - the offsets of the member headers as 32-bit big-endian integers
  - the symbol names as null-terminated strings

- **`Gnu64`**

  A GNU 64-bit symbol table
  
  Contains:
  - the number of symbols as a 64-bit big-endian integer
  - the offsets of the member headers as 64-bit big-endian integers
  - the symbol names as null-terminated strings

- **`Bsd`**

  A BSD symbol table.
  
  Contains:
  - the size in bytes of the offsets array as a 32-bit little-endian integer
  - the offsets array, for which each entry is a pair of 32-bit little-endian integers
    for the offset of the member header and the offset of the symbol name
  - the size in bytes of the symbol names as a 32-bit little-endian integer
  - the symbol names as null-terminated strings

- **`Bsd64`**

  A BSD 64-bit symbol table.
  
  Contains:
  - the size in bytes of the offsets array as a 64-bit little-endian integer
  - the offsets array, for which each entry is a pair of 64-bit little-endian integers
    for the offset of the member header and the offset of the symbol name
  - the size in bytes of the symbol names as a 64-bit little-endian integer
  - the symbol names as null-terminated strings

- **`Coff`**

  A Windows COFF symbol table.
  
  Contains:
  - the number of members as a 32-bit little-endian integer
  - the offsets of the member headers as 32-bit little-endian integers
  - the number of symbols as a 32-bit little-endian integer
  - the member index for each symbol as a 16-bit little-endian integer
  - the symbol names as null-terminated strings in lexical order

#### Trait Implementations

##### `impl<'data> Clone for SymbolIteratorInternal<'data>`

- `fn clone(self: &Self) -> SymbolIteratorInternal<'data>` — [`SymbolIteratorInternal`](#symboliteratorinternal)

##### `impl<'data> Debug for SymbolIteratorInternal<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `parse_u64_digits`

```rust
fn parse_u64_digits(digits: &[u8], radix: u32) -> Option<u64>
```

### `parse_sysv_extended_name`

```rust
fn parse_sysv_extended_name<'data>(digits: &[u8], names: &'data [u8]) -> Result<&'data [u8], ()>
```

Digits are a decimal offset into the extended name table.
Name is terminated by "/\n" (for GNU) or a null byte (for COFF).

### `parse_bsd_extended_name`

```rust
fn parse_bsd_extended_name<'data, R: ReadRef<'data>>(digits: &[u8], data: R, offset: &mut u64, size: &mut u64) -> Result<&'data [u8], ()>
```

Digits are a decimal length of the extended name, which is contained
in `data` at `offset`.
Modifies `offset` and `size` to start after the extended name.

