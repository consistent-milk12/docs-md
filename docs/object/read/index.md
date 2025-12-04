*[object](../index.md) / [read](index.md)*

---

# Module `read`

Interface for reading object files.

## Unified read API

The [`Object`](../index.md) trait provides a unified read API for accessing common features of
object files, such as sections and symbols. There is an implementation of this
trait for [`File`](../index.md), which allows reading any file format, as well as implementations
for each file format:
[`ElfFile`](elf::ElfFile), [`MachOFile`](macho::MachOFile), [`CoffFile`](coff::CoffFile),
[`PeFile`](pe::PeFile), [`WasmFile`](wasm::WasmFile), [`XcoffFile`](xcoff::XcoffFile).

## Low level read API

The submodules for each file format define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

See the [submodules](#modules) for examples of the low level read API.

## Naming Convention

Types that form part of the unified API for a file format are prefixed with the
name of the file format.

## Example for unified read API
 ```no_run
use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(all(feature = "read", feature = "std"))] {
    let data = fs::read("path/to/binary")?;
    let file = object::File::parse(&*data)?;
    for section in file.sections() {
        println!("{}", section.name()?);
    }
#   }
    Ok(())
}
```

## Modules

- [`archive`](archive/index.md) - Support for archive files.
- [`coff`](coff/index.md) - Support for reading Windows COFF files.
- [`elf`](elf/index.md) - Support for reading ELF files.
- [`macho`](macho/index.md) - Support for reading Mach-O files.
- [`pe`](pe/index.md) - Support for reading PE files.
- [`xcoff`](xcoff/index.md) - Support for reading AIX XCOFF files.

## Structs

### `Error`

```rust
struct Error();
```

The error type used within the read module.

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

- `fn clone(self: &Self) -> Error`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SectionIndex`

```rust
struct SectionIndex(usize);
```

The index used to identify a section in a file.

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

- `fn clone(self: &Self) -> SectionIndex`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SectionIndex) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SymbolIndex`

```rust
struct SymbolIndex(usize);
```

The index used to identify a symbol in a symbol table.

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

- `fn clone(self: &Self) -> SymbolIndex`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SymbolIndex) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SymbolMap<T: SymbolMapEntry>`

```rust
struct SymbolMap<T: SymbolMapEntry> {
    // [REDACTED: Private Fields]
}
```

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](#symbolmapname).

Returned by `Object::symbol_map`.

#### Implementations

- `fn new(symbols: Vec<T>) -> Self`
  Construct a new symbol map.

- `fn get(self: &Self, address: u64) -> Option<&T>`
  Get the symbol before the given address.

- `fn symbols(self: &Self) -> &[T]`
  Get all symbols in the map.

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

##### `impl Clone<T: $crate::clone::Clone + SymbolMapEntry>`

- `fn clone(self: &Self) -> SymbolMap<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Debug<T: $crate::fmt::Debug + SymbolMapEntry>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<T: $crate::default::Default + SymbolMapEntry>`

- `fn default() -> SymbolMap<T>`

### `SymbolMapName<'data>`

```rust
struct SymbolMapName<'data> {
    // [REDACTED: Private Fields]
}
```

The type used for entries in a [`SymbolMap`](#symbolmap) that maps from addresses to names.

#### Implementations

- `fn new(address: u64, name: &'data str) -> Self`
  Construct a `SymbolMapName`.

- `fn address(self: &Self) -> u64`
  The symbol address.

- `fn name(self: &Self) -> &'data str`
  The symbol name.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> SymbolMapName<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &SymbolMapName<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

##### `impl SymbolMapEntry<'data>`

- `fn address(self: &Self) -> u64`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ObjectMap<'data>`

```rust
struct ObjectMap<'data> {
    // [REDACTED: Private Fields]
}
```

A map from addresses to symbol names and object files.

This is derived from STAB entries in Mach-O files.

Returned by `Object::object_map`.

#### Implementations

- `fn get(self: &Self, address: u64) -> Option<&ObjectMapEntry<'data>>`
  Get the entry containing the given address.

- `fn symbols(self: &Self) -> &[ObjectMapEntry<'data>]`
  Get all symbols in the map.

- `fn objects(self: &Self) -> &[ObjectMapFile<'data>]`
  Get all objects in the map.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ObjectMap<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data>`

- `fn default() -> ObjectMap<'data>`

### `ObjectMapEntry<'data>`

```rust
struct ObjectMapEntry<'data> {
    // [REDACTED: Private Fields]
}
```

A symbol in an [`ObjectMap`](#objectmap).

#### Implementations

- `fn address(self: &Self) -> u64`
  Get the symbol address.

- `fn size(self: &Self) -> u64`
  Get the symbol size.

- `fn name(self: &Self) -> &'data [u8]`
  Get the symbol name.

- `fn object_index(self: &Self) -> usize`
  Get the index of the object file name.

- `fn object<'a>(self: &Self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>`
  Get the object file name.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ObjectMapEntry<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &ObjectMapEntry<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

##### `impl SymbolMapEntry<'data>`

- `fn address(self: &Self) -> u64`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data>`

- `fn default() -> ObjectMapEntry<'data>`

### `ObjectMapFile<'data>`

```rust
struct ObjectMapFile<'data> {
    // [REDACTED: Private Fields]
}
```

An object file name in an [`ObjectMap`](#objectmap).

#### Implementations

- `fn path(self: &Self) -> &'data [u8]`
  Get the path to the file containing the object.

- `fn member(self: &Self) -> Option<&'data [u8]>`
  If the file is an archive, get the name of the member containing the object.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> ObjectMapFile<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &ObjectMapFile<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Import<'data>`

```rust
struct Import<'data> {
    // [REDACTED: Private Fields]
}
```

An imported symbol.

Returned by `Object::imports`.

#### Implementations

- `fn name(self: &Self) -> &'data [u8]`
  The symbol name.

- `fn library(self: &Self) -> &'data [u8]`
  The name of the library to import the symbol from.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> Import<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &Import<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Export<'data>`

```rust
struct Export<'data> {
    // [REDACTED: Private Fields]
}
```

An exported symbol.

Returned by `Object::exports`.

#### Implementations

- `fn name(self: &Self) -> &'data [u8]`
  The symbol name.

- `fn address(self: &Self) -> u64`
  The virtual address of the symbol.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> Export<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &Export<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CodeView<'data>`

```rust
struct CodeView<'data> {
    // [REDACTED: Private Fields]
}
```

PDB information from the debug directory in a PE file.

#### Implementations

- `fn path(self: &Self) -> &'data [u8]`
  The path to the PDB as stored in CodeView.

- `fn age(self: &Self) -> u32`
  The age of the PDB.

- `fn guid(self: &Self) -> [u8; 16]`
  The GUID of the PDB.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> CodeView<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &CodeView<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Relocation`

```rust
struct Relocation {
    // [REDACTED: Private Fields]
}
```

A relocation entry.

Returned by `Object::dynamic_relocations` or `ObjectSection::relocations`.

#### Implementations

- `fn kind(self: &Self) -> RelocationKind`
  The operation used to calculate the result of the relocation.

- `fn encoding(self: &Self) -> RelocationEncoding`
  Information about how the result of the relocation operation is encoded in the place.

- `fn size(self: &Self) -> u8`
  The size in bits of the place of the relocation.

- `fn target(self: &Self) -> RelocationTarget`
  The target of the relocation.

- `fn addend(self: &Self) -> i64`
  The addend to use in the relocation calculation.

- `fn set_addend(self: &mut Self, addend: i64)`
  Set the addend to use in the relocation calculation.

- `fn has_implicit_addend(self: &Self) -> bool`
  Returns true if there is an implicit addend stored in the data at the offset

- `fn flags(self: &Self) -> RelocationFlags`
  Relocation flags that are specific to each file format.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RelocationMap`

```rust
struct RelocationMap();
```

A map from section offsets to relocation information.

This can be used to apply relocations to a value at a given section offset.
This is intended for use with DWARF in relocatable object files, and only
supports relocations that are used in DWARF.

Returned by `ObjectSection::relocation_map`.

#### Implementations

- `fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>`
  Construct a new relocation map for a section.

- `fn add<'data: 'file, 'file, T>(self: &mut Self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>`
  Add a single relocation to the map.

- `fn relocate(self: &Self, offset: u64, value: u64) -> u64`
  Relocate a value that was read from the section at the given offset.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> RelocationMap`

### `CompressedFileRange`

```rust
struct CompressedFileRange {
    pub format: CompressionFormat,
    pub offset: u64,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}
```

A range in a file that may be compressed.

Returned by `ObjectSection::compressed_file_range`.

#### Fields

- **`format`**: `CompressionFormat`

  The data compression format.

- **`offset`**: `u64`

  The file offset of the compressed data.

- **`compressed_size`**: `u64`

  The compressed data size.

- **`uncompressed_size`**: `u64`

  The uncompressed data size.

#### Implementations

- `fn none(range: Option<(u64, u64)>) -> Self`
  Data that is uncompressed.

- `fn data<'data, R: ReadRef<'data>>(self: Self, file: R) -> Result<CompressedData<'data>>`
  Convert to [`CompressedData`] by reading from the file.

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

- `fn clone(self: &Self) -> CompressedFileRange`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressedFileRange) -> bool`

##### `impl StructuralPartialEq`

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

### `CompressedData<'data>`

```rust
struct CompressedData<'data> {
    pub format: CompressionFormat,
    pub data: &'data [u8],
    pub uncompressed_size: u64,
}
```

Data that may be compressed.

Returned by `ObjectSection::compressed_data`.

#### Fields

- **`format`**: `CompressionFormat`

  The data compression format.

- **`data`**: `&'data [u8]`

  The compressed data.

- **`uncompressed_size`**: `u64`

  The uncompressed data size.

#### Implementations

- `fn none(data: &'data [u8]) -> Self`
  Data that is uncompressed.

- `fn decompress(self: Self) -> Result<Cow<'data, [u8]>>`
  Return the uncompressed data.

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

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> CompressedData<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

##### `impl Eq<'data>`

##### `impl Hash<'data>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'data>`

- `fn eq(self: &Self, other: &CompressedData<'data>) -> bool`

##### `impl StructuralPartialEq<'data>`

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

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `FileKind`

```rust
enum FileKind {
    Archive,
    Coff,
    CoffBig,
    CoffImport,
    DyldCache,
    Elf32,
    Elf64,
    MachO32,
    MachO64,
    MachOFat32,
    MachOFat64,
    Pe32,
    Pe64,
    Xcoff32,
    Xcoff64,
}
```

A file format kind.

#### Variants

- **`Archive`**

  A Unix archive.
  
  See `archive::ArchiveFile`.

- **`Coff`**

  A COFF object file.
  
  See `coff::CoffFile`.

- **`CoffBig`**

  A COFF bigobj object file.
  
  This supports a larger number of sections.
  
  See `coff::CoffBigFile`.

- **`CoffImport`**

  A Windows short import file.
  
  See `coff::ImportFile`.

- **`DyldCache`**

  A dyld cache file containing Mach-O images.
  
  See `macho::DyldCache`

- **`Elf32`**

  A 32-bit ELF file.
  
  See `elf::ElfFile32`.

- **`Elf64`**

  A 64-bit ELF file.
  
  See `elf::ElfFile64`.

- **`MachO32`**

  A 32-bit Mach-O file.
  
  See `macho::MachOFile32`.

- **`MachO64`**

  A 64-bit Mach-O file.
  
  See `macho::MachOFile64`.

- **`MachOFat32`**

  A 32-bit Mach-O fat binary.
  
  See `macho::MachOFatFile32`.

- **`MachOFat64`**

  A 64-bit Mach-O fat binary.
  
  See `macho::MachOFatFile64`.

- **`Pe32`**

  A 32-bit PE file.
  
  See `pe::PeFile32`.

- **`Pe64`**

  A 64-bit PE file.
  
  See `pe::PeFile64`.

- **`Xcoff32`**

  A 32-bit XCOFF file.
  
  See `xcoff::XcoffFile32`.

- **`Xcoff64`**

  A 64-bit XCOFF file.
  
  See `xcoff::XcoffFile64`.

#### Implementations

- `fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>`
  Determine a file kind by parsing the start of the file.

- `fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>`
  Determine a file kind by parsing at the given offset.

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

- `fn clone(self: &Self) -> FileKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &FileKind) -> bool`

##### `impl StructuralPartialEq`

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

### `ObjectKind`

```rust
enum ObjectKind {
    Unknown,
    Relocatable,
    Executable,
    Dynamic,
    Core,
}
```

An object kind.

Returned by `Object::kind`.

#### Variants

- **`Unknown`**

  The object kind is unknown.

- **`Relocatable`**

  Relocatable object.

- **`Executable`**

  Executable.

- **`Dynamic`**

  Dynamic shared object.

- **`Core`**

  Core.

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

- `fn clone(self: &Self) -> ObjectKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ObjectKind) -> bool`

##### `impl StructuralPartialEq`

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

### `SymbolSection`

```rust
enum SymbolSection {
    Unknown,
    None,
    Undefined,
    Absolute,
    Common,
    Section(SectionIndex),
}
```

The section where an [`ObjectSymbol`](../index.md) is defined.

#### Variants

- **`Unknown`**

  The section is unknown.

- **`None`**

  The section is not applicable for this symbol (such as file symbols).

- **`Undefined`**

  The symbol is undefined.

- **`Absolute`**

  The symbol has an absolute value.

- **`Common`**

  The symbol is a zero-initialized symbol that will be combined with duplicate definitions.

- **`Section`**

  The symbol is defined in the given section.

#### Implementations

- `fn index(self: Self) -> Option<SectionIndex>`
  Returns the section index for the section where the symbol is defined.

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

- `fn clone(self: &Self) -> SymbolSection`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SymbolSection) -> bool`

##### `impl StructuralPartialEq`

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

### `RelocationTarget`

```rust
enum RelocationTarget {
    Symbol(SymbolIndex),
    Section(SectionIndex),
    Absolute,
}
```

The target referenced by a [`Relocation`](#relocation).

#### Variants

- **`Symbol`**

  The target is a symbol.

- **`Section`**

  The target is a section.

- **`Absolute`**

  The offset is an absolute address.

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

- `fn clone(self: &Self) -> RelocationTarget`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RelocationTarget) -> bool`

##### `impl StructuralPartialEq`

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

### `CompressionFormat`

```rust
enum CompressionFormat {
    None,
    Unknown,
    Zlib,
    Zstandard,
}
```

A data compression format.

#### Variants

- **`None`**

  The data is uncompressed.

- **`Unknown`**

  The data is compressed, but the compression format is unknown.

- **`Zlib`**

  ZLIB/DEFLATE.
  
  Used for ELF compression and GNU compressed debug information.

- **`Zstandard`**

  Zstandard.
  
  Used for ELF compression.

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

- `fn clone(self: &Self) -> CompressionFormat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressionFormat) -> bool`

##### `impl StructuralPartialEq`

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

## Traits

### `SymbolMapEntry`

```rust
trait SymbolMapEntry { ... }
```

An entry in a [`SymbolMap`](#symbolmap).

#### Required Methods

- `fn address(self: &Self) -> u64`

  The symbol address.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result type used within the read module.

### `NativeFile<'data, R>`

```rust
type NativeFile<'data, R> = elf::ElfFile64<'data, crate::endian::Endianness, R>;
```

The native executable file for the target platform.

