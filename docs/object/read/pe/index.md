*[object](../../index.md) / [read](../index.md) / [pe](index.md)*

---

# Module `pe`

Support for reading PE files.

Traits are used to abstract over the difference between PE32 and PE32+.
The primary trait for this is [`ImageNtHeaders`](file/index.md).

## High level API

[`PeFile`](file/index.md) implements the [`Object`](crate::read::Object) trait for
PE files. [`PeFile`](file/index.md) is parameterised by [`ImageNtHeaders`](file/index.md) to allow
reading both PE32 and PE32+. There are type aliases for these parameters
([`PeFile32`](file/index.md) and [`PeFile64`](file/index.md)).

## Low level API

The [`ImageNtHeaders`](file/index.md) trait can be directly used to parse both
`pe::ImageNtHeaders32` and `pe::ImageNtHeaders64`.

### Example for low level API
 ```no_run
use object::pe;
use object::read::pe::ImageNtHeaders;
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let dos_header = pe::ImageDosHeader::parse(&*data)?;
    let mut offset = dos_header.nt_headers_offset().into();
    let (nt_headers, data_directories) = pe::ImageNtHeaders64::parse(&*data, &mut offset)?;
    let sections = nt_headers.sections(&*data, offset)?;
    let symbols = nt_headers.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
  }
    Ok(())
}
```

## Structs

### `SectionTable<'data>`

```rust
struct SectionTable<'data> {
    sections: &'data [pe::ImageSectionHeader],
}
```

The table of section headers in a COFF or PE file.

Returned by `CoffHeader::sections` and
[`ImageNtHeaders::sections`](crate::read::pe::ImageNtHeaders::sections).

#### Implementations

- `fn parse<Coff: CoffHeader, R: ReadRef<'data>>(header: &Coff, data: R, offset: u64) -> Result<Self>` — [`Result`](../../../read/index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, pe::ImageSectionHeader>` — [`ImageSectionHeader`](../../../pe/index.md)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SectionIndex, &'data pe::ImageSectionHeader)>` — [`SectionIndex`](../../../read/index.md), [`ImageSectionHeader`](../../../pe/index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn section(self: &Self, index: SectionIndex) -> read::Result<&'data pe::ImageSectionHeader>` — [`SectionIndex`](../../../read/index.md), [`Result`](../../../read/index.md), [`ImageSectionHeader`](../../../pe/index.md)

- `fn section_by_name<R: ReadRef<'data>>(self: &Self, strings: StringTable<'data, R>, name: &[u8]) -> Option<(SectionIndex, &'data pe::ImageSectionHeader)>` — [`StringTable`](../../../read/util/index.md), [`SectionIndex`](../../../read/index.md), [`ImageSectionHeader`](../../../pe/index.md)

- `fn max_section_file_offset(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone<'data>`

- `fn clone(self: &Self) -> SectionTable<'data>` — [`SectionTable`](../../../read/coff/section/index.md)

##### `impl Copy<'data>`

##### `impl Debug<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data>`

- `fn default() -> SectionTable<'data>` — [`SectionTable`](../../../read/coff/section/index.md)

### `SymbolTable<'data, R, Coff>`

```rust
struct SymbolTable<'data, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    symbols: &'data [<Coff as >::ImageSymbolBytes],
    strings: crate::read::util::StringTable<'data, R>,
}
```

A table of symbol entries in a COFF or PE file.

Also includes the string table used for the symbol names.

Returned by `CoffHeader::symbols` and
[`ImageNtHeaders::symbols`](crate::read::pe::ImageNtHeaders::symbols).

#### Implementations

- `fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../../read/index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../../../read/util/index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../../../read/coff/symbol/index.md)

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../../read/index.md), [`Result`](../../../read/index.md), [`CoffHeader`](../../../read/coff/file/index.md)

- `fn aux_function(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../../read/index.md), [`Result`](../../../read/index.md), [`ImageAuxSymbolFunction`](../../../pe/index.md)

- `fn aux_section(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../../read/index.md), [`Result`](../../../read/index.md), [`ImageAuxSymbolSection`](../../../pe/index.md)

- `fn aux_weak_external(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../../read/index.md), [`Result`](../../../read/index.md), [`ImageAuxSymbolWeak`](../../../pe/index.md)

- `fn aux_file_name(self: &Self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../../read/index.md), [`Result`](../../../read/index.md)

- `fn get<T: Pod>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../read/index.md), [`Result`](../../../read/index.md)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(self: &Self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../read/index.md)

#### Trait Implementations

##### `impl Debug<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'data, R: ReadRef<'data>, Coff: CoffHeader>`

- `fn default() -> Self`

