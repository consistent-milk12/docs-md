*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Contents

- [Structs](#structs)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`CoffSymbolTable`](#coffsymboltable)
  - [`CoffSymbolIterator`](#coffsymboliterator)
  - [`CoffSymbol`](#coffsymbol)
- [Traits](#traits)
  - [`ImageSymbol`](#imagesymbol)
- [Type Aliases](#type-aliases)
  - [`CoffBigSymbolTable`](#coffbigsymboltable)
  - [`CoffBigSymbolIterator`](#coffbigsymboliterator)
  - [`CoffBigSymbol`](#coffbigsymbol)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in a COFF or PE file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in a COFF or PE file. |
| [`CoffSymbolTable`](#coffsymboltable) | struct | A symbol table in a [`CoffFile`](super::CoffFile) |
| [`CoffSymbolIterator`](#coffsymboliterator) | struct | An iterator for the symbols in a [`CoffFile`](super::CoffFile) |
| [`CoffSymbol`](#coffsymbol) | struct | A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile). |
| [`ImageSymbol`](#imagesymbol) | trait | A trait for generic access to [`pe::ImageSymbol`] and [`pe::ImageSymbolEx`]. |
| [`CoffBigSymbolTable`](#coffbigsymboltable) | type | A symbol table in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbolIterator`](#coffbigsymboliterator) | type | An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigSymbol`](#coffbigsymbol) | type | A symbol in a [`CoffBigFile`](super::CoffBigFile). |

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

- <span id="symboltable-parse"></span>`fn parse(header: &Coff, data: R) -> Result<Self>` — [`Result`](../../../index.md)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, R, Coff>` — [`SymbolIterator`](../index.md)

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Coff as >::ImageSymbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`CoffHeader`](../index.md)

- <span id="symboltable-aux-function"></span>`fn aux_function(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolFunction>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageAuxSymbolFunction`](../../../pe/index.md)

- <span id="symboltable-aux-section"></span>`fn aux_section(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolSection>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageAuxSymbolSection`](../../../pe/index.md)

- <span id="symboltable-aux-weak-external"></span>`fn aux_weak_external(&self, index: SymbolIndex) -> Result<&'data pe::ImageAuxSymbolWeak>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ImageAuxSymbolWeak`](../../../pe/index.md)

- <span id="symboltable-aux-file-name"></span>`fn aux_file_name(&self, index: SymbolIndex, aux_count: u8) -> Result<&'data [u8]>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md)

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md)

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Coff as >::ImageSymbol) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../index.md)

#### Trait Implementations

##### `impl<'data, R, Coff> Debug for SymbolTable<'data, R, Coff>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, R: ReadRef<'data>, Coff: CoffHeader> Default for SymbolTable<'data, R, Coff>`

- <span id="symboltable-default"></span>`fn default() -> Self`

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

- <span id="symboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, R, Coff>`

- <span id="symboliterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'table, R: ReadRef<'data>, Coff: CoffHeader> Iterator for SymbolIterator<'data, 'table, R, Coff>`

- <span id="symboliterator-item"></span>`type Item = (SymbolIndex, &'data <Coff as CoffHeader>::ImageSymbol)`

- <span id="symboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="coffsymboltable-clone"></span>`fn clone(&self) -> CoffSymbolTable<'data, 'file, R, Coff>` — [`CoffSymbolTable`](../index.md)

##### `impl<'data, 'file, R, Coff> Copy for CoffSymbolTable<'data, 'file, R, Coff>`

##### `impl<'data, 'file, R, Coff> Debug for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbolTable for CoffSymbolTable<'data, 'file, R, Coff>`

- <span id="coffsymboltable-symbol"></span>`type Symbol = CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymboltable-symboliterator"></span>`type SymbolIterator = CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md)

- <span id="coffsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ObjectSymbolTable`](../../index.md)

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

- <span id="coffsymboliterator-new"></span>`fn new(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](../file/index.md)

- <span id="coffsymboliterator-empty"></span>`fn empty(file: &'file CoffCommon<'data, R, Coff>) -> Self` — [`CoffCommon`](../file/index.md)

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffsymboliterator-intoiter"></span>`type IntoIter = I`

- <span id="coffsymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffSymbolIterator<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-item"></span>`type Item = CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="coffsymbol-raw-symbol"></span>`fn raw_symbol(&self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](../index.md)

- <span id="coffsymbol-coff-symbol"></span>`fn coff_symbol(&self) -> &'data <Coff as >::ImageSymbol` — [`CoffHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, R, Coff> Clone for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-clone"></span>`fn clone(&self) -> CoffSymbol<'data, 'file, R, Coff>` — [`CoffSymbol`](../index.md)

##### `impl<'data, 'file, R, Coff> Copy for CoffSymbol<'data, 'file, R, Coff>`

##### `impl<'data, 'file, R, Coff> Debug for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> ObjectSymbol for CoffSymbol<'data, 'file, R, Coff>`

- <span id="coffsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- <span id="coffsymbol-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="coffsymbol-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md)

- <span id="coffsymbol-address"></span>`fn address(&self) -> u64`

- <span id="coffsymbol-size"></span>`fn size(&self) -> u64`

- <span id="coffsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md)

- <span id="coffsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md)

- <span id="coffsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="coffsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="coffsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="coffsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="coffsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md)

- <span id="coffsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="coffsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="coffsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md), [`SectionIndex`](../../../index.md), [`SymbolIndex`](../../../index.md)

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffSymbol<'data, 'file, R, Coff>`

## Traits

### `ImageSymbol`

```rust
trait ImageSymbol: Debug + Pod { ... }
```

A trait for generic access to [`pe::ImageSymbol`](../../../pe/index.md) and [`pe::ImageSymbolEx`](../../../pe/index.md).

#### Required Methods

- `fn raw_name(&self) -> &[u8; 8]`

- `fn value(&self) -> u32`

- `fn section_number(&self) -> i32`

- `fn typ(&self) -> u16`

- `fn storage_class(&self) -> u8`

- `fn number_of_aux_symbols(&self) -> u8`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse a COFF symbol name.

- `fn address(&self, image_base: u64, sections: &SectionTable<'_>) -> Result<Option<u64>>`

  Return the symbol address.

- `fn section(&self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn has_aux_file_name(&self) -> bool`

  Return true if the symbol has an auxiliary file name.

- `fn has_aux_function(&self) -> bool`

  Return true if the symbol has an auxiliary function symbol.

- `fn has_aux_section(&self) -> bool`

  Return true if the symbol has an auxiliary section symbol.

- `fn has_aux_weak_external(&self) -> bool`

  Return true if the symbol has an auxiliary weak external symbol.

- `fn base_type(&self) -> u16`

- `fn derived_type(&self) -> u16`

#### Implementors

- [`ImageSymbolEx`](../../../pe/index.md)
- [`ImageSymbol`](../../../pe/index.md)

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

