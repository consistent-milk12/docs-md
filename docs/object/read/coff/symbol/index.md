*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Structs

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

- `fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../index.md)

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`CoffHeader`](../index.md)

- `fn aux_function(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageAuxSymbolFunction`](../../../pe/index.md)

- `fn aux_section(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageAuxSymbolSection`](../../../pe/index.md)

- `fn aux_weak_external(self: &Self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageAuxSymbolWeak`](../../../pe/index.md)

- `fn aux_file_name(self: &Self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md)

- `fn get<T: Pod>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(self: &Self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../index.md)

#### Trait Implementations

##### `impl<'data, R, Coff> Debug for SymbolTable<'data, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, R: ReadRef<'data>, Coff: CoffHeader> Default for SymbolTable<'data, R, Coff>`

- `fn default() -> Self`

### `SymbolIterator<'data, 'table, R, Coff>`

```rust
struct SymbolIterator<'data, 'table, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    symbols: &'table SymbolTable<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for symbol entries in a COFF or PE file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<'data, 'table, R, Coff> Debug for SymbolIterator<'data, 'table, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'table, R: ReadRef<'data>, Coff: CoffHeader> Iterator for SymbolIterator<'data, 'table, R, Coff>`

- `type Item = (SymbolIndex, &'data <Coff as CoffHeader>::ImageSymbol)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffSymbolTable<'data, 'file, R, Coff>`

```rust
struct CoffSymbolTable<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
}
```

A symbol table in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

#### Trait Implementations

##### `impl<'data, 'file, R, Coff> Clone for CoffSymbolTable<'data, 'file, R, Coff>`

- `fn clone(self: &Self) -> CoffSymbolTable<'data, 'file, R, Coff>` — [`CoffSymbolTable`](../index.md)

##### `impl<'data, 'file, R, Coff> Copy for CoffSymbolTable<'data, 'file, R, Coff>`

##### `impl<'data, 'file, R, Coff> Debug for CoffSymbolTable<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbolTable for CoffSymbolTable<'data, 'file, R, Coff>`

- `type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- `type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ObjectSymbolTable`](../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbolTable<'data, 'file, R, Coff>`

### `CoffSymbolIterator<'data, 'file, R, Coff>`

```rust
struct CoffSymbolIterator<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in a [`CoffFile`](super::CoffFile)
or [`PeFile`](crate::read::pe::PeFile).

#### Implementations

- `fn new(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](../file/index.md)

- `fn empty(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](../file/index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffSymbolIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- `type Item = CoffSymbol<'data, 'file, R, Coff>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `CoffSymbol<'data, 'file, R, Coff>`

```rust
struct CoffSymbol<'data, 'file, R, Coff>
where
    R: ReadRef<'data>,
    Coff: CoffHeader {
    file: &'file super::CoffCommon<'data, R, Coff>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Coff as >::ImageSymbol,
}
```

A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- `fn raw_symbol(self: &Self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](../index.md)

- `fn coff_symbol(self: &Self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R, Coff> Clone for CoffSymbol<'data, 'file, R, Coff>`

- `fn clone(self: &Self) -> CoffSymbol<'data, 'file, R, Coff>` — [`CoffSymbol`](../index.md)

##### `impl<'data, 'file, R, Coff> Copy for CoffSymbol<'data, 'file, R, Coff>`

##### `impl<'data, 'file, R, Coff> Debug for CoffSymbol<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbol for CoffSymbol<'data, 'file, R, Coff>`

- `fn index(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn kind(self: &Self) -> SymbolKind` — [`SymbolKind`](../../../index.md)

- `fn section(self: &Self) -> SymbolSection` — [`SymbolSection`](../../../index.md)

- `fn is_undefined(self: &Self) -> bool`

- `fn is_definition(self: &Self) -> bool`

- `fn is_common(self: &Self) -> bool`

- `fn is_weak(self: &Self) -> bool`

- `fn scope(self: &Self) -> SymbolScope` — [`SymbolScope`](../../../index.md)

- `fn is_global(self: &Self) -> bool`

- `fn is_local(self: &Self) -> bool`

- `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md), [`SectionIndex`](../../../index.md), [`SymbolIndex`](../../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbol<'data, 'file, R, Coff>`

## Traits

### `ImageSymbol`

```rust
trait ImageSymbol: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageSymbol`](../../../pe/index.md) and [`pe::ImageSymbolEx`](../../../pe/index.md).

#### Required Methods

- `fn raw_name(self: &Self) -> &[u8; 8]`

- `fn value(self: &Self) -> u32`

- `fn section_number(self: &Self) -> i32`

- `fn typ(self: &Self) -> u16`

- `fn storage_class(self: &Self) -> u8`

- `fn number_of_aux_symbols(self: &Self) -> u8`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse a COFF symbol name.

- `fn address(self: &Self, image_base: u64, sections: &SectionTable<'_>) -> Result<Option<u64>>`

  Return the symbol address.

- `fn section(self: &Self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_definition(self: &Self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn has_aux_file_name(self: &Self) -> bool`

  Return true if the symbol has an auxiliary file name.

- `fn has_aux_function(self: &Self) -> bool`

  Return true if the symbol has an auxiliary function symbol.

- `fn has_aux_section(self: &Self) -> bool`

  Return true if the symbol has an auxiliary section symbol.

- `fn has_aux_weak_external(self: &Self) -> bool`

  Return true if the symbol has an auxiliary weak external symbol.

- `fn base_type(self: &Self) -> u16`

- `fn derived_type(self: &Self) -> u16`

## Type Aliases

### `CoffBigSymbolTable<'data, 'file, R>`

```rust
type CoffBigSymbolTable<'data, 'file, R> = CoffSymbolTable<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A symbol table in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSymbolIterator<'data, 'file, R>`

```rust
type CoffBigSymbolIterator<'data, 'file, R> = CoffSymbolIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigSymbol<'data, 'file, R>`

```rust
type CoffBigSymbol<'data, 'file, R> = CoffSymbol<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

A symbol in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

