*[object](../index.md) / [archive](index.md)*

---

# Module `archive`

Archive definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

## Contents

- [Structs](#structs)
  - [`Header`](#header)
  - [`AixHeader`](#aixheader)
  - [`AixFileHeader`](#aixfileheader)
  - [`AixMemberOffset`](#aixmemberoffset)
- [Constants](#constants)
  - [`MAGIC`](#magic)
  - [`AIX_BIG_MAGIC`](#aix-big-magic)
  - [`THIN_MAGIC`](#thin-magic)
  - [`TERMINATOR`](#terminator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Header`](#header) | struct | The header at the start of an archive member. |
| [`AixHeader`](#aixheader) | struct | The header at the start of an AIX big archive member, without name. |
| [`AixFileHeader`](#aixfileheader) | struct | The AIX big archive's fixed length header at file beginning. |
| [`AixMemberOffset`](#aixmemberoffset) | struct | Offset of a member in an AIX big archive. |
| [`MAGIC`](#magic) | const | File identification bytes stored at the beginning of the file. |
| [`AIX_BIG_MAGIC`](#aix-big-magic) | const | File identification bytes at the beginning of AIX big archive. |
| [`THIN_MAGIC`](#thin-magic) | const | File identification bytes stored at the beginning of a thin archive. |
| [`TERMINATOR`](#terminator) | const | The terminator for each archive member header. |

## Structs

### `Header`

```rust
struct Header {
    pub name: [u8; 16],
    pub date: [u8; 12],
    pub uid: [u8; 6],
    pub gid: [u8; 6],
    pub mode: [u8; 8],
    pub size: [u8; 10],
    pub terminator: [u8; 2],
}
```

*Defined in [`object-0.37.3/src/archive.rs:25-40`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L25-L40)*

The header at the start of an archive member.

#### Fields

- **`name`**: `[u8; 16]`

  The file name.

- **`date`**: `[u8; 12]`

  File modification timestamp in decimal.

- **`uid`**: `[u8; 6]`

  User ID in decimal.

- **`gid`**: `[u8; 6]`

  Group ID in decimal.

- **`mode`**: `[u8; 8]`

  File mode in octal.

- **`size`**: `[u8; 10]`

  File size in decimal.

- **`terminator`**: `[u8; 2]`

  Must be equal to `TERMINATOR`.

#### Trait Implementations

##### `impl Any for Header`

- <span id="header-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Header`

- <span id="header-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Header`

- <span id="header-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Header`

- <span id="header-clone"></span>`fn clone(&self) -> Header` — [`Header`](#header)

##### `impl CloneToUninit for Header`

- <span id="header-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Header`

##### `impl Debug for Header`

- <span id="header-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Header`

- <span id="header-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Header`

- <span id="header-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for Header`

##### `impl ToOwned for Header`

- <span id="header-toowned-type-owned"></span>`type Owned = T`

- <span id="header-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="header-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Header`

- <span id="header-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="header-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Header`

- <span id="header-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="header-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AixHeader`

```rust
struct AixHeader {
    pub size: [u8; 20],
    pub nxtmem: [u8; 20],
    pub prvmem: [u8; 20],
    pub date: [u8; 12],
    pub uid: [u8; 12],
    pub gid: [u8; 12],
    pub mode: [u8; 12],
    pub namlen: [u8; 4],
}
```

*Defined in [`object-0.37.3/src/archive.rs:45-62`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L45-L62)*

The header at the start of an AIX big archive member, without name.

#### Fields

- **`size`**: `[u8; 20]`

  File member size in decimal.

- **`nxtmem`**: `[u8; 20]`

  Next member offset in decimal.

- **`prvmem`**: `[u8; 20]`

  Previous member offset in decimal.

- **`date`**: `[u8; 12]`

  File member date in decimal.

- **`uid`**: `[u8; 12]`

  File member user id in decimal.

- **`gid`**: `[u8; 12]`

  File member group id in decimal.

- **`mode`**: `[u8; 12]`

  File member mode in octal.

- **`namlen`**: `[u8; 4]`

  File member name length in decimal.

#### Trait Implementations

##### `impl Any for AixHeader`

- <span id="aixheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AixHeader`

- <span id="aixheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AixHeader`

- <span id="aixheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AixHeader`

- <span id="aixheader-clone"></span>`fn clone(&self) -> AixHeader` — [`AixHeader`](#aixheader)

##### `impl CloneToUninit for AixHeader`

- <span id="aixheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AixHeader`

##### `impl Debug for AixHeader`

- <span id="aixheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AixHeader`

- <span id="aixheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AixHeader`

- <span id="aixheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for AixHeader`

##### `impl ToOwned for AixHeader`

- <span id="aixheader-toowned-type-owned"></span>`type Owned = T`

- <span id="aixheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="aixheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AixHeader`

- <span id="aixheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aixheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AixHeader`

- <span id="aixheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aixheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AixFileHeader`

```rust
struct AixFileHeader {
    pub magic: [u8; 8],
    pub memoff: [u8; 20],
    pub gstoff: [u8; 20],
    pub gst64off: [u8; 20],
    pub fstmoff: [u8; 20],
    pub lstmoff: [u8; 20],
    pub freeoff: [u8; 20],
}
```

*Defined in [`object-0.37.3/src/archive.rs:67-82`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L67-L82)*

The AIX big archive's fixed length header at file beginning.

#### Fields

- **`magic`**: `[u8; 8]`

  Archive magic string.

- **`memoff`**: `[u8; 20]`

  Offset of member table.

- **`gstoff`**: `[u8; 20]`

  Offset of global symbol table.

- **`gst64off`**: `[u8; 20]`

  Offset of global symbol table for 64-bit objects.

- **`fstmoff`**: `[u8; 20]`

  Offset of first member.

- **`lstmoff`**: `[u8; 20]`

  Offset of last member.

- **`freeoff`**: `[u8; 20]`

  Offset of first member on free list.

#### Trait Implementations

##### `impl Any for AixFileHeader`

- <span id="aixfileheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AixFileHeader`

- <span id="aixfileheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AixFileHeader`

- <span id="aixfileheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AixFileHeader`

- <span id="aixfileheader-clone"></span>`fn clone(&self) -> AixFileHeader` — [`AixFileHeader`](#aixfileheader)

##### `impl CloneToUninit for AixFileHeader`

- <span id="aixfileheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AixFileHeader`

##### `impl Debug for AixFileHeader`

- <span id="aixfileheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AixFileHeader`

- <span id="aixfileheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AixFileHeader`

- <span id="aixfileheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for AixFileHeader`

##### `impl ToOwned for AixFileHeader`

- <span id="aixfileheader-toowned-type-owned"></span>`type Owned = T`

- <span id="aixfileheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="aixfileheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AixFileHeader`

- <span id="aixfileheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aixfileheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AixFileHeader`

- <span id="aixfileheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aixfileheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AixMemberOffset`

```rust
struct AixMemberOffset([u8; 20]);
```

*Defined in [`object-0.37.3/src/archive.rs:89`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L89)*

Offset of a member in an AIX big archive.

This is used in the member index.

#### Trait Implementations

##### `impl Any for AixMemberOffset`

- <span id="aixmemberoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AixMemberOffset`

- <span id="aixmemberoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AixMemberOffset`

- <span id="aixmemberoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AixMemberOffset`

- <span id="aixmemberoffset-clone"></span>`fn clone(&self) -> AixMemberOffset` — [`AixMemberOffset`](#aixmemberoffset)

##### `impl CloneToUninit for AixMemberOffset`

- <span id="aixmemberoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AixMemberOffset`

##### `impl Debug for AixMemberOffset`

- <span id="aixmemberoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AixMemberOffset`

- <span id="aixmemberoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AixMemberOffset`

- <span id="aixmemberoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pod for AixMemberOffset`

##### `impl ToOwned for AixMemberOffset`

- <span id="aixmemberoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="aixmemberoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="aixmemberoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AixMemberOffset`

- <span id="aixmemberoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aixmemberoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AixMemberOffset`

- <span id="aixmemberoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aixmemberoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `MAGIC`
```rust
const MAGIC: [u8; 8];
```

*Defined in [`object-0.37.3/src/archive.rs:9`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L9)*

File identification bytes stored at the beginning of the file.

### `AIX_BIG_MAGIC`
```rust
const AIX_BIG_MAGIC: [u8; 8];
```

*Defined in [`object-0.37.3/src/archive.rs:12`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L12)*

File identification bytes at the beginning of AIX big archive.

### `THIN_MAGIC`
```rust
const THIN_MAGIC: [u8; 8];
```

*Defined in [`object-0.37.3/src/archive.rs:17`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L17)*

File identification bytes stored at the beginning of a thin archive.

A thin archive only contains a symbol table and file names.

### `TERMINATOR`
```rust
const TERMINATOR: [u8; 2];
```

*Defined in [`object-0.37.3/src/archive.rs:20`](../../../.source_1765633015/object-0.37.3/src/archive.rs#L20)*

The terminator for each archive member header.

