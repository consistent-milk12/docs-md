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
  - [`parse_u64_digits`](#parse-u64-digits)
  - [`parse_sysv_extended_name`](#parse-sysv-extended-name)
  - [`parse_bsd_extended_name`](#parse-bsd-extended-name)

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
| [`parse_u64_digits`](#parse-u64-digits) | fn |  |
| [`parse_sysv_extended_name`](#parse-sysv-extended-name) | fn | Digits are a decimal offset into the extended name table. |
| [`parse_bsd_extended_name`](#parse-bsd-extended-name) | fn | Digits are a decimal length of the extended name, which is contained in `data` at `offset`. |

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

*Defined in [`object-0.37.3/src/read/archive.rs:66-73`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L66-L73)*

A partially parsed archive file.

#### Implementations

- <span id="archivefile-parse"></span>`fn parse(data: R) -> read::Result<Self>` — [`Result`](../../index.md#result)

  Parse the archive header and special members.

- <span id="archivefile-parse-aixbig"></span>`fn parse_aixbig(data: R) -> read::Result<Self>` — [`Result`](../../index.md#result)

- <span id="archivefile-kind"></span>`fn kind(&self) -> ArchiveKind` — [`ArchiveKind`](#archivekind)

  Return the archive format.

- <span id="archivefile-is-thin"></span>`fn is_thin(&self) -> bool`

  Return true if the archive is a thin archive.

- <span id="archivefile-members"></span>`fn members(&self) -> ArchiveMemberIterator<'data, R>` — [`ArchiveMemberIterator`](#archivememberiterator)

  Iterate over the members of the archive.

  

  This does not return special members.

- <span id="archivefile-member"></span>`fn member(&self, member: ArchiveOffset) -> read::Result<ArchiveMember<'data>>` — [`ArchiveOffset`](#archiveoffset), [`Result`](../../index.md#result), [`ArchiveMember`](#archivemember)

  Return the member at the given offset.

- <span id="archivefile-symbols"></span>`fn symbols(&self) -> read::Result<Option<ArchiveSymbolIterator<'data>>>` — [`Result`](../../index.md#result), [`ArchiveSymbolIterator`](#archivesymboliterator)

  Iterate over the symbols in the archive.

#### Trait Implementations

##### `impl Any for ArchiveFile<'data, R>`

- <span id="archivefile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveFile<'data, R>`

- <span id="archivefile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveFile<'data, R>`

- <span id="archivefile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + ReadRef<'data>> Clone for ArchiveFile<'data, R>`

- <span id="archivefile-clone"></span>`fn clone(&self) -> ArchiveFile<'data, R>` — [`ArchiveFile`](#archivefile)

##### `impl CloneToUninit for ArchiveFile<'data, R>`

- <span id="archivefile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + ReadRef<'data>> Copy for ArchiveFile<'data, R>`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ArchiveFile<'data, R>`

- <span id="archivefile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArchiveFile<'data, R>`

- <span id="archivefile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArchiveFile<'data, R>`

- <span id="archivefile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArchiveFile<'data, R>`

- <span id="archivefile-toowned-type-owned"></span>`type Owned = T`

- <span id="archivefile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="archivefile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArchiveFile<'data, R>`

- <span id="archivefile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archivefile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveFile<'data, R>`

- <span id="archivefile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archivefile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArchiveMemberIterator<'data, R: ReadRef<'data>>`

```rust
struct ArchiveMemberIterator<'data, R: ReadRef<'data>> {
    data: R,
    members: Members<'data>,
    names: &'data [u8],
    thin: bool,
}
```

*Defined in [`object-0.37.3/src/read/archive.rs:323-328`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L323-L328)*

An iterator over the members of an archive.

#### Trait Implementations

##### `impl Any for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>> Debug for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="archivememberiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="archivememberiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>> Iterator for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-iterator-type-item"></span>`type Item = Result<ArchiveMember<'data>, Error>`

- <span id="archivememberiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archivememberiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveMemberIterator<'data, R>`

- <span id="archivememberiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archivememberiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArchiveMember<'data>`

```rust
struct ArchiveMember<'data> {
    header: MemberHeader<'data>,
    name: &'data [u8],
    offset: u64,
    size: u64,
}
```

*Defined in [`object-0.37.3/src/read/archive.rs:374-380`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L374-L380)*

A partially parsed archive member.

#### Implementations

- <span id="archivemember-parse"></span>`fn parse<R: ReadRef<'data>>(data: R, offset: &mut u64, names: &'data [u8], thin: bool) -> read::Result<Self>` — [`Result`](../../index.md#result)

  Parse the member header, name, and file data in an archive with the common format.

  

  This reads the extended name (if any) and adjusts the file size.

- <span id="archivemember-parse-aixbig-index"></span>`fn parse_aixbig_index<R: ReadRef<'data>>(data: R, index: &archive::AixMemberOffset) -> read::Result<Self>` — [`AixMemberOffset`](../../archive/index.md#aixmemberoffset), [`Result`](../../index.md#result)

  Parse a member index entry in an AIX big archive,

  and then parse the member header, name, and file data.

- <span id="archivemember-parse-aixbig"></span>`fn parse_aixbig<R: ReadRef<'data>>(data: R, offset: u64) -> read::Result<Self>` — [`Result`](../../index.md#result)

  Parse the member header, name, and file data in an AIX big archive.

- <span id="archivemember-header"></span>`fn header(&self) -> Option<&'data archive::Header>` — [`Header`](../../archive/index.md#header)

  Return the raw header that is common to many archive formats.

  

  Returns `None` if this archive does not use the common header format.

- <span id="archivemember-aix-header"></span>`fn aix_header(&self) -> Option<&'data archive::AixHeader>` — [`AixHeader`](../../archive/index.md#aixheader)

  Return the raw header for AIX big archives.

  

  Returns `None` if this is not an AIX big archive.

- <span id="archivemember-name"></span>`fn name(&self) -> &'data [u8]`

  Return the parsed file name.

  

  This may be an extended file name.

- <span id="archivemember-date"></span>`fn date(&self) -> Option<u64>`

  Parse the file modification timestamp from the header.

- <span id="archivemember-uid"></span>`fn uid(&self) -> Option<u64>`

  Parse the user ID from the header.

- <span id="archivemember-gid"></span>`fn gid(&self) -> Option<u64>`

  Parse the group ID from the header.

- <span id="archivemember-mode"></span>`fn mode(&self) -> Option<u64>`

  Parse the file mode from the header.

- <span id="archivemember-size"></span>`fn size(&self) -> u64`

  Return the size of the file data.

- <span id="archivemember-file-range"></span>`fn file_range(&self) -> (u64, u64)`

  Return the offset and size of the file data.

- <span id="archivemember-is-thin"></span>`fn is_thin(&self) -> bool`

  Return true if the member is a thin member.

  

  Thin members have no file data.

- <span id="archivemember-data"></span>`fn data<R: ReadRef<'data>>(&self, data: R) -> read::Result<&'data [u8]>` — [`Result`](../../index.md#result)

  Return the file data.

  

  This is an empty slice for thin members.

#### Trait Implementations

##### `impl Any for ArchiveMember<'data>`

- <span id="archivemember-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveMember<'data>`

- <span id="archivemember-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveMember<'data>`

- <span id="archivemember-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ArchiveMember<'data>`

- <span id="archivemember-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArchiveMember<'data>`

- <span id="archivemember-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArchiveMember<'data>`

- <span id="archivemember-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ArchiveMember<'data>`

- <span id="archivemember-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archivemember-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveMember<'data>`

- <span id="archivemember-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archivemember-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArchiveOffset`

```rust
struct ArchiveOffset(u64);
```

*Defined in [`object-0.37.3/src/read/archive.rs:598`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L598)*

An offset of a member in an archive.

#### Trait Implementations

##### `impl Any for ArchiveOffset`

- <span id="archiveoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveOffset`

- <span id="archiveoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveOffset`

- <span id="archiveoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArchiveOffset`

- <span id="archiveoffset-clone"></span>`fn clone(&self) -> ArchiveOffset` — [`ArchiveOffset`](#archiveoffset)

##### `impl CloneToUninit for ArchiveOffset`

- <span id="archiveoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ArchiveOffset`

##### `impl Debug for ArchiveOffset`

- <span id="archiveoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArchiveOffset`

- <span id="archiveoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArchiveOffset`

- <span id="archiveoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArchiveOffset`

- <span id="archiveoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="archiveoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="archiveoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArchiveOffset`

- <span id="archiveoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archiveoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveOffset`

- <span id="archiveoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archiveoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArchiveSymbolIterator<'data>`

```rust
struct ArchiveSymbolIterator<'data>(SymbolIteratorInternal<'data>);
```

*Defined in [`object-0.37.3/src/read/archive.rs:602`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L602)*

An iterator over the symbols in the archive symbol table.

#### Implementations

- <span id="archivesymboliterator-new"></span>`fn new<R: ReadRef<'data>>(kind: ArchiveKind, data: R, offset: u64, size: u64) -> Result<Self, ()>` — [`ArchiveKind`](#archivekind)

#### Trait Implementations

##### `impl Any for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-clone"></span>`fn clone(&self) -> ArchiveSymbolIterator<'data>` — [`ArchiveSymbolIterator`](#archivesymboliterator)

##### `impl CloneToUninit for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="archivesymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="archivesymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-iterator-type-item"></span>`type Item = Result<ArchiveSymbol<'data>, Error>`

- <span id="archivesymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="archivesymboliterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-toowned-type-owned"></span>`type Owned = T`

- <span id="archivesymboliterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="archivesymboliterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archivesymboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveSymbolIterator<'data>`

- <span id="archivesymboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archivesymboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArchiveSymbol<'data>`

```rust
struct ArchiveSymbol<'data> {
    name: &'data [u8],
    offset: ArchiveOffset,
}
```

*Defined in [`object-0.37.3/src/read/archive.rs:825-828`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L825-L828)*

A symbol in the archive symbol table.

This is used to find the member containing the symbol.

#### Implementations

- <span id="archivesymbol-name"></span>`fn name(&self) -> &'data [u8]`

  Return the symbol name.

- <span id="archivesymbol-offset"></span>`fn offset(&self) -> ArchiveOffset` — [`ArchiveOffset`](#archiveoffset)

  Return the offset of the header for the member containing the symbol.

#### Trait Implementations

##### `impl Any for ArchiveSymbol<'data>`

- <span id="archivesymbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveSymbol<'data>`

- <span id="archivesymbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveSymbol<'data>`

- <span id="archivesymbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArchiveSymbol<'data>`

- <span id="archivesymbol-clone"></span>`fn clone(&self) -> ArchiveSymbol<'data>` — [`ArchiveSymbol`](#archivesymbol)

##### `impl CloneToUninit for ArchiveSymbol<'data>`

- <span id="archivesymbol-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ArchiveSymbol<'data>`

##### `impl Debug for ArchiveSymbol<'data>`

- <span id="archivesymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArchiveSymbol<'data>`

- <span id="archivesymbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArchiveSymbol<'data>`

- <span id="archivesymbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArchiveSymbol<'data>`

- <span id="archivesymbol-toowned-type-owned"></span>`type Owned = T`

- <span id="archivesymbol-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="archivesymbol-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArchiveSymbol<'data>`

- <span id="archivesymbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archivesymbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveSymbol<'data>`

- <span id="archivesymbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archivesymbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/archive.rs:33-50`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L33-L50)*

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

##### `impl Any for ArchiveKind`

- <span id="archivekind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArchiveKind`

- <span id="archivekind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArchiveKind`

- <span id="archivekind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArchiveKind`

- <span id="archivekind-clone"></span>`fn clone(&self) -> ArchiveKind` — [`ArchiveKind`](#archivekind)

##### `impl CloneToUninit for ArchiveKind`

- <span id="archivekind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ArchiveKind`

##### `impl Debug for ArchiveKind`

- <span id="archivekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArchiveKind`

##### `impl<T> From for ArchiveKind`

- <span id="archivekind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ArchiveKind`

- <span id="archivekind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ArchiveKind`

- <span id="archivekind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArchiveKind`

- <span id="archivekind-partialeq-eq"></span>`fn eq(&self, other: &ArchiveKind) -> bool` — [`ArchiveKind`](#archivekind)

##### `impl StructuralPartialEq for ArchiveKind`

##### `impl ToOwned for ArchiveKind`

- <span id="archivekind-toowned-type-owned"></span>`type Owned = T`

- <span id="archivekind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="archivekind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArchiveKind`

- <span id="archivekind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="archivekind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArchiveKind`

- <span id="archivekind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="archivekind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/archive.rs:54-62`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L54-L62)*

The list of members in the archive.

#### Trait Implementations

##### `impl Any for Members<'data>`

- <span id="members-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Members<'data>`

- <span id="members-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Members<'data>`

- <span id="members-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Members<'data>`

- <span id="members-clone"></span>`fn clone(&self) -> Members<'data>` — [`Members`](#members)

##### `impl CloneToUninit for Members<'data>`

- <span id="members-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Members<'data>`

##### `impl Debug for Members<'data>`

- <span id="members-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Members<'data>`

- <span id="members-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Members<'data>`

- <span id="members-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Members<'data>`

- <span id="members-toowned-type-owned"></span>`type Owned = T`

- <span id="members-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="members-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Members<'data>`

- <span id="members-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="members-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Members<'data>`

- <span id="members-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="members-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MemberHeader<'data>`

```rust
enum MemberHeader<'data> {
    Common(&'data archive::Header),
    AixBig(&'data archive::AixHeader),
}
```

*Defined in [`object-0.37.3/src/read/archive.rs:365-370`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L365-L370)*

An archive member header.

#### Variants

- **`Common`**

  Common header used by many formats.

- **`AixBig`**

  AIX big archive header

#### Trait Implementations

##### `impl Any for MemberHeader<'data>`

- <span id="memberheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MemberHeader<'data>`

- <span id="memberheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MemberHeader<'data>`

- <span id="memberheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MemberHeader<'data>`

- <span id="memberheader-clone"></span>`fn clone(&self) -> MemberHeader<'data>` — [`MemberHeader`](#memberheader)

##### `impl CloneToUninit for MemberHeader<'data>`

- <span id="memberheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MemberHeader<'data>`

##### `impl Debug for MemberHeader<'data>`

- <span id="memberheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MemberHeader<'data>`

- <span id="memberheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MemberHeader<'data>`

- <span id="memberheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MemberHeader<'data>`

- <span id="memberheader-toowned-type-owned"></span>`type Owned = T`

- <span id="memberheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memberheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MemberHeader<'data>`

- <span id="memberheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memberheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MemberHeader<'data>`

- <span id="memberheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memberheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/archive.rs:605-665`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L605-L665)*

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

##### `impl Any for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-clone"></span>`fn clone(&self) -> SymbolIteratorInternal<'data>` — [`SymbolIteratorInternal`](#symboliteratorinternal)

##### `impl CloneToUninit for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-toowned-type-owned"></span>`type Owned = T`

- <span id="symboliteratorinternal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symboliteratorinternal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliteratorinternal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIteratorInternal<'data>`

- <span id="symboliteratorinternal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliteratorinternal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_u64_digits`

```rust
fn parse_u64_digits(digits: &[u8], radix: u32) -> Option<u64>
```

*Defined in [`object-0.37.3/src/read/archive.rs:845-861`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L845-L861)*

### `parse_sysv_extended_name`

```rust
fn parse_sysv_extended_name<'data>(digits: &[u8], names: &'data [u8]) -> Result<&'data [u8], ()>
```

*Defined in [`object-0.37.3/src/read/archive.rs:865-879`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L865-L879)*

Digits are a decimal offset into the extended name table.
Name is terminated by "/\n" (for GNU) or a null byte (for COFF).

### `parse_bsd_extended_name`

```rust
fn parse_bsd_extended_name<'data, R: ReadRef<'data>>(digits: &[u8], data: R, offset: &mut u64, size: &mut u64) -> Result<&'data [u8], ()>
```

*Defined in [`object-0.37.3/src/read/archive.rs:884-898`](../../../../.source_1765521767/object-0.37.3/src/read/archive.rs#L884-L898)*

Digits are a decimal length of the extended name, which is contained
in `data` at `offset`.
Modifies `offset` and `size` to start after the extended name.

