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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:23-31`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L23-L31)*

A table of symbol entries in an XCOFF file.

Also includes the string table used for the symbol names.

Returned by `FileHeader::symbols`.

#### Implementations

- <span id="symboltable-parse"></span>`fn parse(header: Xcoff, data: R) -> Result<Self>` — [`Result`](../../../index.md#result)

  Parse the symbol table.

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-iter"></span>`fn iter<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](../index.md#symboliterator)

  Iterate over the symbols.

  

  This does not return null symbols.

- <span id="symboltable-iter-none"></span>`fn iter_none<'table>(self: &'table Self) -> SymbolIterator<'data, 'table, Xcoff, R>` — [`SymbolIterator`](../index.md#symboliterator)

  Empty symbol iterator.

- <span id="symboltable-get"></span>`fn get<T: Pod>(&self, index: SymbolIndex, offset: usize) -> Result<&'data T>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result)

  Return the symbol entry at the given index and offset.

- <span id="symboltable-symbol-unchecked"></span>`fn symbol_unchecked(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Get the symbol at the given index.

  

  This does not check if the symbol is null, but does check if the index is in bounds.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Xcoff as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Get the symbol at the given index.

  

  Returns an error for null symbols and out of bounds indices.

  Note that this is unable to check whether the index is an auxiliary symbol.

- <span id="symboltable-aux-file"></span>`fn aux_file(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::FileAux>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Return a file auxiliary symbol.

- <span id="symboltable-aux-csect"></span>`fn aux_csect(&self, index: SymbolIndex, offset: usize) -> Result<&'data <Xcoff as >::CsectAux>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`FileHeader`](../index.md#fileheader)

  Return the csect auxiliary symbol.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbol table entries.

  

  This includes auxiliary symbol table entries.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> Default for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

##### `impl<T> From for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, Xcoff, R>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:182-189`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L182-L189)*

An iterator for symbol entries in an XCOFF file.

Yields the index and symbol structure for each symbol.

#### Trait Implementations

##### `impl Any for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Iterator for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-iterator-type-item"></span>`type Item = (SymbolIndex, &'data <Xcoff as FileHeader>::Symbol)`

- <span id="symboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolIterator<'data, 'table, Xcoff, R>`

- <span id="symboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:217-224`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L217-L224)*

A symbol table in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl Any for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Clone for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clone"></span>`fn clone(&self) -> XcoffSymbolTable<'data, 'file, Xcoff, R>` — [`XcoffSymbolTable`](../index.md#xcoffsymboltable)

##### `impl CloneToUninit for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Xcoff, R> Copy for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl<Xcoff, R> Debug for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-type-symbol"></span>`type Symbol = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md#objectsymboltable)

- <span id="xcoffsymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ObjectSymbolTable`](../../index.md#objectsymboltable)

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbolTable<'data, 'file, Xcoff, R>`

##### `impl ToOwned for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-toowned-type-owned"></span>`type Owned = T`

- <span id="xcoffsymboltable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="xcoffsymboltable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsymboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSymbolTable<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsymboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:263-270`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L263-L270)*

An iterator for the symbols in an [`XcoffFile`](../index.md).

#### Trait Implementations

##### `impl Any for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Debug for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffsymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffsymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Iterator for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-iterator-type-item"></span>`type Item = XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsymboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSymbolIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffsymboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsymboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:307-316`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L307-L316)*

A symbol in an [`XcoffFile`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- <span id="xcoffsymbol-xcoff-file"></span>`fn xcoff_file(&self) -> &'file XcoffFile<'data, Xcoff, R>` — [`XcoffFile`](../index.md#xcofffile)

  Get the XCOFF file containing this symbol.

- <span id="xcoffsymbol-xcoff-symbol"></span>`fn xcoff_symbol(&self) -> &'data <Xcoff as >::Symbol` — [`FileHeader`](../index.md#fileheader)

  Get the raw XCOFF symbol structure.

#### Trait Implementations

##### `impl Any for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Clone for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clone"></span>`fn clone(&self) -> XcoffSymbol<'data, 'file, Xcoff, R>` — [`XcoffSymbol`](../index.md#xcoffsymbol)

##### `impl CloneToUninit for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Xcoff, R> Copy for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl<Xcoff, R> Debug for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> ObjectSymbol for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="xcoffsymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffsymbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="xcoffsymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="xcoffsymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="xcoffsymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md#symbolkind)

- <span id="xcoffsymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md#symbolsection)

- <span id="xcoffsymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- <span id="xcoffsymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md#symbolscope)

- <span id="xcoffsymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="xcoffsymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md#symbolflags), [`SectionIndex`](../../../index.md#sectionindex), [`SymbolIndex`](../../../index.md#symbolindex)

##### `impl<Xcoff: FileHeader, R: ReadRef<'data>> Sealed for XcoffSymbol<'data, 'file, Xcoff, R>`

##### `impl ToOwned for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-toowned-type-owned"></span>`type Owned = T`

- <span id="xcoffsymbol-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="xcoffsymbol-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffsymbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffSymbol<'data, 'file, Xcoff, R>`

- <span id="xcoffsymbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffsymbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Symbol`

```rust
trait Symbol: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:540-593`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L540-L593)*

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:687-720`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L687-L720)*

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:752-768`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L752-L768)*

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

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:209-210`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L209-L210)*

A symbol table in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolTable64<'data, 'file, R>`

```rust
type XcoffSymbolTable64<'data, 'file, R> = XcoffSymbolTable<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:212-213`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L212-L213)*

A symbol table in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbolIterator32<'data, 'file, R>`

```rust
type XcoffSymbolIterator32<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:256-257`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L256-L257)*

An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbolIterator64<'data, 'file, R>`

```rust
type XcoffSymbolIterator64<'data, 'file, R> = XcoffSymbolIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:259-260`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L259-L260)*

An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64).

### `XcoffSymbol32<'data, 'file, R>`

```rust
type XcoffSymbol32<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:297-298`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L297-L298)*

A symbol in an [`XcoffFile32`](super::XcoffFile32).

### `XcoffSymbol64<'data, 'file, R>`

```rust
type XcoffSymbol64<'data, 'file, R> = XcoffSymbol<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/symbol.rs:300-301`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/symbol.rs#L300-L301)*

A symbol in an [`XcoffFile64`](super::XcoffFile64).

