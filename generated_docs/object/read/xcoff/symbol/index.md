*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Contents

- [Structs](#structs)
  - [`SymbolTable`](#symboltable)
  - [`SymbolIterator`](#symboliterator)
  - [`XcoffSymbolTable`](#xcoffsymboltable)
  - [`XcoffSymbolIterator`](#xcoffsymboliterator)
  - [`XcoffSymbol`](#xcoffsymbol)
- [Traits](#traits)
  - [`Symbol`](#symbol)
  - [`FileAux`](#fileaux)
  - [`CsectAux`](#csectaux)
- [Type Aliases](#type-aliases)
  - [`XcoffSymbolTable32`](#xcoffsymboltable32)
  - [`XcoffSymbolTable64`](#xcoffsymboltable64)
  - [`XcoffSymbolIterator32`](#xcoffsymboliterator32)
  - [`XcoffSymbolIterator64`](#xcoffsymboliterator64)
  - [`XcoffSymbol32`](#xcoffsymbol32)
  - [`XcoffSymbol64`](#xcoffsymbol64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in an XCOFF file. |
| [`SymbolIterator`](#symboliterator) | struct | An iterator for symbol entries in an XCOFF file. |
| [`XcoffSymbolTable`](#xcoffsymboltable) | struct | A symbol table in an [`XcoffFile`]. |
| [`XcoffSymbolIterator`](#xcoffsymboliterator) | struct | An iterator for the symbols in an [`XcoffFile`]. |
| [`XcoffSymbol`](#xcoffsymbol) | struct | A symbol in an [`XcoffFile`]. |
| [`Symbol`](#symbol) | trait | A trait for generic access to [`xcoff::Symbol32`] and [`xcoff::Symbol64`]. |
| [`FileAux`](#fileaux) | trait | A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`]. |
| [`CsectAux`](#csectaux) | trait | A trait for generic access to [`xcoff::CsectAux32`] and [`xcoff::CsectAux64`]. |
| [`XcoffSymbolTable32`](#xcoffsymboltable32) | type | A symbol table in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolTable64`](#xcoffsymboltable64) | type | A symbol table in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbolIterator32`](#xcoffsymboliterator32) | type | An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbolIterator64`](#xcoffsymboliterator64) | type | An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffSymbol32`](#xcoffsymbol32) | type | A symbol in an [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffSymbol64`](#xcoffsymbol64) | type | A symbol in an [`XcoffFile64`](super::XcoffFile64). |

## Structs

### `SymbolTable<'data, Xcoff, R>`

```rust
struct SymbolTable<'data, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'data [xcoff::SymbolBytes],
    strings: crate::read::StringTable<'data, R>,
    header: core::marker::PhantomData<Xcoff>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:23-31`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L23-L31)*

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md#stringtable)

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](../index.md#symboliterator)

- <span id="symboltable-iter-none"></span>`fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](../index.md#symboliterator)

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result)

- <span id="symboltable-symbol-unchecked"></span>`fn symbol_unchecked(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="symboltable-aux-file"></span>`fn aux_file(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="symboltable-aux-csect"></span>`fn aux_csect(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `SymbolIterator<'data, 'table, Xcoff, R>`

```rust
struct SymbolIterator<'data, 'table, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    symbols: &'table SymbolTable<'data, Xcoff, R>,
    index: usize,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:182-189`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L182-L189)*

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<'data, 'table, Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'table, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-type-item"></span>`type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- <span id="symboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSymbolTable<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolTable<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:217-224`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L217-L224)*

A symbol table in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clone"></span>`fn clone(&self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](../index.md#xcoffsymboltable)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md#objectsymboltable)

- <span id="xcoffsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ObjectSymbolTable`](../../index.md#objectsymboltable)

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbolTable<'data, 'file, Xcoff, R>`

### `XcoffSymbolIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbolIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: SymbolIterator<'data, 'file, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:263-270`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L263-L270)*

An iterator for the symbols in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsymboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-type-item"></span>`type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffSymbol<'data, 'file, Xcoff, R>`

```rust
struct XcoffSymbol<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    symbols: &'file SymbolTable<'data, Xcoff, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Xcoff as >::Symbol,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:307-316`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L307-L316)*

A symbol in an [`XcoffFile`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- <span id="xcoffsymbol-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](../index.md#xcofffile)

- <span id="xcoffsymbol-xcoff-symbol"></span>`fn xcoff_symbol(&self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](../index.md#fileheader)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clone"></span>`fn clone(&self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](../index.md#xcoffsymbol)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="xcoffsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="xcoffsymbol-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsymbol-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md#symbolkind)

- <span id="xcoffsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md#symbolsection)

- <span id="xcoffsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="xcoffsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="xcoffsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="xcoffsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="xcoffsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md#symbolscope)

- <span id="xcoffsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="xcoffsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="xcoffsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md#symbolflags), [`SectionIndex`](../../../index.md#sectionindex), [`SymbolIndex`](../../../index.md#symbolindex)

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

## Traits

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:540-593`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L540-L593)*

A trait for generic access to [`xcoff::Symbol32`](../../../xcoff/index.md) and [`xcoff::Symbol64`](../../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn n_value(&self) -> <Self as >::Word`

- `fn n_scnum(&self) -> i16`

- `fn n_type(&self) -> u16`

- `fn n_sclass(&self) -> u8`

- `fn n_numaux(&self) -> u8`

- `fn name_offset(&self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

#### Provided Methods

- `fn section(&self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_null(&self) -> bool`

  Return true if the symbol is a null placeholder.

- `fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- `fn has_aux_file(&self) -> bool`

  Return true if the symbol has file auxiliary entry.

- `fn has_aux_csect(&self) -> bool`

  Return true if the symbol has csect auxiliary entry.

#### Implementors

- [`Symbol32`](../../../xcoff/index.md#symbol32)
- [`Symbol64`](../../../xcoff/index.md#symbol64)

### `FileAux`

```rust
trait FileAux: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:687-720`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L687-L720)*

A trait for generic access to [`xcoff::FileAux32`](../../../xcoff/index.md) and [`xcoff::FileAux64`](../../../xcoff/index.md).

#### Required Methods

- `fn x_fname(&self) -> &[u8; 8]`

- `fn x_ftype(&self) -> u8`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn name_offset(&self) -> Option<u32>`

- `fn fname<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse the x_fname field, which may be an inline string or a string table offset.

#### Implementors

- [`FileAux32`](../../../xcoff/index.md#fileaux32)
- [`FileAux64`](../../../xcoff/index.md#fileaux64)

### `CsectAux`

```rust
trait CsectAux: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:752-768`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L752-L768)*

A trait for generic access to [`xcoff::CsectAux32`](../../../xcoff/index.md) and [`xcoff::CsectAux64`](../../../xcoff/index.md).

#### Required Methods

- `fn x_scnlen(&self) -> u64`

- `fn x_parmhash(&self) -> u32`

- `fn x_snhash(&self) -> u16`

- `fn x_smtyp(&self) -> u8`

- `fn x_smclas(&self) -> u8`

- `fn x_stab(&self) -> Option<u32>`

- `fn x_snstab(&self) -> Option<u16>`

- `fn x_auxtype(&self) -> Option<u8>`

#### Provided Methods

- `fn alignment(&self) -> u8`

- `fn sym_type(&self) -> u8`

#### Implementors

- [`CsectAux32`](../../../xcoff/index.md#csectaux32)
- [`CsectAux64`](../../../xcoff/index.md#csectaux64)

## Type Aliases

### `XcoffSymbolTable32<'data, 'file, R>`

```rust
type XcoffSymbolTable32<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:209-210`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L209-L210)*

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:212-213`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L212-L213)*

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:256-257`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L256-L257)*

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:259-260`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L259-L260)*

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:297-298`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L297-L298)*

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:300-301`](../../../../../.source_1765210505/object-0.37.3/src/read/xcoff/symbol.rs#L300-L301)*

A symbol in an [`XcoffFile64`](super::XcoffFile64).

