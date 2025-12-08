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

## Contents

- [Structs](#structs)
  - [`ArchiveFile`](#archivefile)
  - [`ArchiveMemberIterator`](#archivememberiterator)
  - [`ArchiveMember`](#archivemember)
  - [`ArchiveOffset`](#archiveoffset)
  - [`ArchiveSymbolIterator`](#archivesymboliterator)
  - [`ArchiveSymbol`](#archivesymbol)
- [Enums](#enums)
  - [`ArchiveKind`](#archivekind)
  - [`Members`](#members)
  - [`MemberHeader`](#memberheader)
  - [`SymbolIteratorInternal`](#symboliteratorinternal)
- [Functions](#functions)
  - [`parse_u64_digits`](#parse_u64_digits)
  - [`parse_sysv_extended_name`](#parse_sysv_extended_name)
  - [`parse_bsd_extended_name`](#parse_bsd_extended_name)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArchiveFile`](#archivefile) | struct | A partially parsed archive file. |
| [`ArchiveMemberIterator`](#archivememberiterator) | struct | An iterator over the members of an archive. |
| [`ArchiveMember`](#archivemember) | struct | A partially parsed archive member. |
| [`ArchiveOffset`](#archiveoffset) | struct | An offset of a member in an archive. |
| [`ArchiveSymbolIterator`](#archivesymboliterator) | struct | An iterator over the symbols in the archive symbol table. |
| [`ArchiveSymbol`](#archivesymbol) | struct | A symbol in the archive symbol table. |
| [`ArchiveKind`](#archivekind) | enum | The kind of archive format. |
| [`Members`](#members) | enum | The list of members in the archive. |
| [`MemberHeader`](#memberheader) | enum | An archive member header. |
| [`SymbolIteratorInternal`](#symboliteratorinternal) | enum |  |
| [`parse_u64_digits`](#parse_u64_digits) | fn |  |
| [`parse_sysv_extended_name`](#parse_sysv_extended_name) | fn | Digits are a decimal offset into the extended name table. |
| [`parse_bsd_extended_name`](#parse_bsd_extended_name) | fn | Digits are a decimal length of the extended name, which is contained |

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

- <span id="archivefile-parse"></span>`fn parse(data: R) -> read::Result<Self>` — [`Result`](../../index.md)

- <span id="archivefile-parse-aixbig"></span>`fn parse_aixbig(data: R) -> read::Result<Self>` — [`Result`](../../index.md)

- <span id="archivefile-kind"></span>`fn kind(&self) -> ArchiveKind` — [`ArchiveKind`](#archivekind)

- <span id="archivefile-is-thin"></span>`fn is_thin(&self) -> bool`

- <span id="archivefile-members"></span>`fn members(&self) -> ArchiveMemberIterator<'data, R>` — [`ArchiveMemberIterator`](#archivememberiterator)

- <span id="archivefile-member"></span>`fn member(&self, member: ArchiveOffset) -> read::Result<ArchiveMember<'data>>` — [`ArchiveOffset`](#archiveoffset), [`Result`](../../index.md), [`ArchiveMember`](#archivemember)

- <span id="archivefile-symbols"></span>`fn symbols(&self) -> read::Result<Option<ArchiveSymbolIterator<'data>>>` — [`Result`](../../index.md), [`ArchiveSymbolIterator`](#archivesymboliterator)

#### Trait Implementations

##### `impl<'data, R: clone::Clone + ReadRef<'data>> Clone for ArchiveFile<'data, R>`

- <span id="archivefile-clone"></span>`fn clone(&self) -> ArchiveFile<'data, R>` — [`ArchiveFile`](#archivefile)

##### `impl<'data, R: marker::Copy + ReadRef<'data>> Copy for ArchiveFile<'data, R>`

##### `impl<'data, R: fmt::Debug + ReadRef<'data>> Debug for ArchiveFile<'data, R>`

- <span id="archivefile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl<'data, R: fmt::Debug + ReadRef<'data>> Debug for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="archivememberiterator-intoiter"></span>`type IntoIter = I`

- <span id="archivememberiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, R: ReadRef<'data>> Iterator for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-item"></span>`type Item = Result<ArchiveMember<'data>, Error>`

- <span id="archivememberiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="archivemember-parse"></span>`fn parse<R: ReadRef<'data>>(data: R, offset: &mut u64, names: &'data [u8], thin: bool) -> read::Result<Self>` — [`Result`](../../index.md)

- <span id="archivemember-parse-aixbig-index"></span>`fn parse_aixbig_index<R: ReadRef<'data>>(data: R, index: &archive::AixMemberOffset) -> read::Result<Self>` — [`AixMemberOffset`](../../archive/index.md), [`Result`](../../index.md)

- <span id="archivemember-parse-aixbig"></span>`fn parse_aixbig<R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<Self>` — [`Result`](../../index.md)

- <span id="archivemember-header"></span>`fn header(&self) -> Option<&'data archive::Header>` — [`Header`](../../archive/index.md)

- <span id="archivemember-aix-header"></span>`fn aix_header(&self) -> Option<&'data archive::AixHeader>` — [`AixHeader`](../../archive/index.md)

- <span id="archivemember-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="archivemember-date"></span>`fn date(&self) -> Option<u64>`

- <span id="archivemember-uid"></span>`fn uid(&self) -> Option<u64>`

- <span id="archivemember-gid"></span>`fn gid(&self) -> Option<u64>`

- <span id="archivemember-mode"></span>`fn mode(&self) -> Option<u64>`

- <span id="archivemember-size"></span>`fn size(&self) -> u64`

- <span id="archivemember-file-range"></span>`fn file_range(&self) -> (u64, u64)`

- <span id="archivemember-is-thin"></span>`fn is_thin(&self) -> bool`

- <span id="archivemember-data"></span>`fn data<R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [u8]>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'data> Debug for ArchiveMember<'data>`

- <span id="archivemember-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ArchiveOffset`

```rust
struct ArchiveOffset(u64);
```

An offset of a member in an archive.

#### Trait Implementations

##### `impl Clone for ArchiveOffset`

- <span id="archiveoffset-clone"></span>`fn clone(&self) -> ArchiveOffset` — [`ArchiveOffset`](#archiveoffset)

##### `impl Copy for ArchiveOffset`

##### `impl Debug for ArchiveOffset`

- <span id="archiveoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ArchiveSymbolIterator<'data>`

```rust
struct ArchiveSymbolIterator<'data>(SymbolIteratorInternal<'data>);
```

An iterator over the symbols in the archive symbol table.

#### Implementations

- <span id="archivesymboliterator-new"></span>`fn new<R: ReadRef<'data>>(kind: ArchiveKind, data: R, offset: u64, size: u64) -> Result<Self, ()>` — [`ArchiveKind`](#archivekind)

#### Trait Implementations

##### `impl<'data> Clone for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-clone"></span>`fn clone(&self) -> ArchiveSymbolIterator<'data>` — [`ArchiveSymbolIterator`](#archivesymboliterator)

##### `impl<'data> Debug for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="archivesymboliterator-intoiter"></span>`type IntoIter = I`

- <span id="archivesymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data> Iterator for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-item"></span>`type Item = Result<ArchiveSymbol<'data>, Error>`

- <span id="archivesymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="archivesymboliterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="archivesymbol-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="archivesymbol-offset"></span>`fn offset(&self) -> ArchiveOffset` — [`ArchiveOffset`](#archiveoffset)

#### Trait Implementations

##### `impl<'data> Clone for ArchiveSymbol<'data>`

- <span id="archivesymbol-clone"></span>`fn clone(&self) -> ArchiveSymbol<'data>` — [`ArchiveSymbol`](#archivesymbol)

##### `impl<'data> Copy for ArchiveSymbol<'data>`

##### `impl<'data> Debug for ArchiveSymbol<'data>`

- <span id="archivesymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="archivekind-clone"></span>`fn clone(&self) -> ArchiveKind` — [`ArchiveKind`](#archivekind)

##### `impl Copy for ArchiveKind`

##### `impl Debug for ArchiveKind`

- <span id="archivekind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArchiveKind`

##### `impl Hash for ArchiveKind`

- <span id="archivekind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ArchiveKind`

- <span id="archivekind-eq"></span>`fn eq(&self, other: &ArchiveKind) -> bool` — [`ArchiveKind`](#archivekind)

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

- <span id="members-clone"></span>`fn clone(&self) -> Members<'data>` — [`Members`](#members)

##### `impl<'data> Copy for Members<'data>`

##### `impl<'data> Debug for Members<'data>`

- <span id="members-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="memberheader-clone"></span>`fn clone(&self) -> MemberHeader<'data>` — [`MemberHeader`](#memberheader)

##### `impl<'data> Copy for MemberHeader<'data>`

##### `impl<'data> Debug for MemberHeader<'data>`

- <span id="memberheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="symboliteratorinternal-clone"></span>`fn clone(&self) -> SymbolIteratorInternal<'data>` — [`SymbolIteratorInternal`](#symboliteratorinternal)

##### `impl<'data> Debug for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

