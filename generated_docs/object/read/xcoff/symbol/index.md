*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [symbol](index.md)*

---

# Module `symbol`

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

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- `fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../../index.md)

- `fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](../index.md)

- `fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](../index.md)

- `fn get<T: Pod>(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md)

- `fn symbol_unchecked(self: &Self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn aux_file(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn aux_csect(self: &Self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<'data, Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- `fn default() -> Self`

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

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl<'data, 'table, Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'table, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- `type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A symbol table in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `fn clone(self: &Self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](../index.md)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- `type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- `type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ObjectSymbolTable`](../../index.md)

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

An iterator for the symbols in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A symbol in an [`XcoffFile`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- `fn xcoff_file(self: &Self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](../index.md)

- `fn xcoff_symbol(self: &Self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn clone(self: &Self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](../index.md)

##### `impl<'data, 'file, Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

- `fn index(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

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

##### `impl<'data, 'file, Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

## Traits

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::Symbol32`](../../../xcoff/index.md) and [`xcoff::Symbol64`](../../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `fn n_value(self: &Self) -> <Self as >::Word`

- `fn n_scnum(self: &Self) -> i16`

- `fn n_type(self: &Self) -> u16`

- `fn n_sclass(self: &Self) -> u8`

- `fn n_numaux(self: &Self) -> u8`

- `fn name_offset(self: &Self) -> Option<u32>`

- `fn name<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

- `fn section(self: &Self) -> Option<SectionIndex>`

  Return the section index for the symbol.

- `fn is_null(self: &Self) -> bool`

  Return true if the symbol is a null placeholder.

- `fn is_undefined(self: &Self) -> bool`

  Return true if the symbol is undefined.

- `fn has_aux_file(self: &Self) -> bool`

  Return true if the symbol has file auxiliary entry.

- `fn has_aux_csect(self: &Self) -> bool`

  Return true if the symbol has csect auxiliary entry.

### `FileAux`

```rust
trait FileAux: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::FileAux32`](../../../xcoff/index.md) and [`xcoff::FileAux64`](../../../xcoff/index.md).

#### Required Methods

- `fn x_fname(self: &Self) -> &[u8; 8]`

- `fn x_ftype(self: &Self) -> u8`

- `fn x_auxtype(self: &Self) -> Option<u8>`

- `fn name_offset(self: &Self) -> Option<u32>`

- `fn fname<'data, R: ReadRef<'data>>(self: &'data Self, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

  Parse the x_fname field, which may be an inline string or a string table offset.

### `CsectAux`

```rust
trait CsectAux: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::CsectAux32`](../../../xcoff/index.md) and [`xcoff::CsectAux64`](../../../xcoff/index.md).

#### Required Methods

- `fn x_scnlen(self: &Self) -> u64`

- `fn x_parmhash(self: &Self) -> u32`

- `fn x_snhash(self: &Self) -> u16`

- `fn x_smtyp(self: &Self) -> u8`

- `fn x_smclas(self: &Self) -> u8`

- `fn x_stab(self: &Self) -> Option<u32>`

- `fn x_snstab(self: &Self) -> Option<u16>`

- `fn x_auxtype(self: &Self) -> Option<u8>`

- `fn alignment(self: &Self) -> u8`

- `fn sym_type(self: &Self) -> u8`

## Type Aliases

### `XcoffSymbolTable32<'data, 'file, R>`

```rust
type XcoffSymbolTable32<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader32, R>;
```

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

A symbol in an [`XcoffFile64`](super::XcoffFile64).

