*[object](../index.md) / [archive](index.md)*

---

# Module `archive`

Archive definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Header`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Pod`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> AixHeader`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Pod`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> AixFileHeader`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Pod`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AixMemberOffset`

```rust
struct AixMemberOffset([u8; 20]);
```

Offset of a member in an AIX big archive.

This is used in the member index.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> AixMemberOffset`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Pod`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Constants

### `MAGIC`

```rust
const MAGIC: [u8; 8];
```

File identification bytes stored at the beginning of the file.

### `AIX_BIG_MAGIC`

```rust
const AIX_BIG_MAGIC: [u8; 8];
```

File identification bytes at the beginning of AIX big archive.

### `THIN_MAGIC`

```rust
const THIN_MAGIC: [u8; 8];
```

File identification bytes stored at the beginning of a thin archive.

A thin archive only contains a symbol table and file names.

### `TERMINATOR`

```rust
const TERMINATOR: [u8; 2];
```

The terminator for each archive member header.

