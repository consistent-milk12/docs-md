*[object](../../index.md) / [read](../index.md) / [pe](index.md)*

---

# Module `pe`

Support for reading PE files.

Traits are used to abstract over the difference between PE32 and PE32+.
The primary trait for this is [`ImageNtHeaders`](#imagentheaders).

## High level API

[`PeFile`](#pefile) implements the [`Object`](crate::read::Object) trait for
PE files. [`PeFile`](#pefile) is parameterised by [`ImageNtHeaders`](#imagentheaders) to allow
reading both PE32 and PE32+. There are type aliases for these parameters
([`PeFile32`](#pefile32) and [`PeFile64`](#pefile64)).

## Low level API

The [`ImageNtHeaders`](#imagentheaders) trait can be directly used to parse both
[`pe::ImageNtHeaders32`](#imagentheaders32) and [`pe::ImageNtHeaders64`](#imagentheaders64).

### Example for low level API
 ```no_run
use object::pe;
use object::read::pe::ImageNtHeaders;
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let dos_header = pe::ImageDosHeader::parse(&*data)?;
    let mut offset = dos_header.nt_headers_offset().into();
    let (nt_headers, data_directories) = pe::ImageNtHeaders64::parse(&*data, &mut offset)?;
    let sections = nt_headers.sections(&*data, offset)?;
    let symbols = nt_headers.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
#   }
    Ok(())
}
```

## Structs

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    // [REDACTED: Private Fields]
}
```

The table of section headers in a COFF or PE file.

Returned by [`CoffHeader::sections`](#sections) and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- `fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>`
  Parse the section table.

- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageSectionHeader>`
  Iterate over the section headers.

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>`
  Iterate over the section headers and their indices.

- `fn is_empty(self: &Self) -> bool`
  Return true if the section table is empty.

- `fn len(self: &Self) -> usize`
  The number of section headers.

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>`
  Return the section header at the given index.

- `fn section_by_name<R: ReadRef<'data>>(self: &Self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>`
  Return the section header with the given name.

- `fn max_section_file_offset(self: &Self) -> u64`
  Compute the maximum file offset used by sections.

- `fn pe_file_range_at(self: &Self, va: u32) -> Option<(u32, u32)>`
  Return the file offset of the given virtual address, and the size up

- `fn pe_data_at<R: ReadRef<'data>>(self: &Self, data: R, va: u32) -> Option<&'data [u8]>`
  Return the data starting at the given virtual address, up to the end of the

- `fn pe_data_containing<R: ReadRef<'data>>(self: &Self, data: R, va: u32) -> Option<(&'data [u8], u32)>`
  Return the data of the section that contains the given virtual address in a PE file.

- `fn section_containing(self: &Self, va: u32) -> Option<&'data ImageSectionHeader>`
  Return the section that contains a given virtual address.

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

- `fn clone(self: &Self) -> SectionTable<'data>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'data>`

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

- `fn default() -> SectionTable<'data>`

### `SymbolTable<'data, R, Coff>`

```rust
struct SymbolTable<'data, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    // [REDACTED: Private Fields]
}
```

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by [`CoffHeader::symbols`](#symbols) and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

#### Implementations

- `fn parse(header: &Coff, data: R) -> Result<Self>`
  Read the symbol table.

- `fn strings(self: &Self) -> StringTable<'data, R>`
  Return the string table used for the symbol names.

- `fn is_empty(self: &Self) -> bool`
  Return true if the symbol table is empty.

- `fn len(self: &Self) -> usize`
  The number of symbol table entries.

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>`
  Iterate over the symbols.

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>`
  Return the symbol table entry at the given index.

- `fn aux_function(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>`
  Return the auxiliary function symbol for the symbol table entry at the given index.

- `fn aux_section(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>`
  Return the auxiliary section symbol for the symbol table entry at the given index.

- `fn aux_weak_external(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>`
  Return the auxiliary weak external symbol for the symbol table entry at the given index.

- `fn aux_file_name(self: &Self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>`
  Return the auxiliary file name for the symbol table entry at the given index.

- `fn get<T: Pod>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>`
  Return the symbol table entry or auxiliary record at the given index and offset.

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(self: &Self, f: F) -> SymbolMap<Entry>`
  Construct a map from addresses to a user-defined map entry.

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

##### `impl Debug<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data, R: ReadRef<'data>, Coff: CoffHeader>`

- `fn default() -> Self`

