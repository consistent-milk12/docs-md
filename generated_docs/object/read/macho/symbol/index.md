*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Contents

- [Structs](#structs)
  - [`SymbolTable`](#symboltable)
  - [`MachOSymbolTable`](#machosymboltable)
  - [`MachOSymbolIterator`](#machosymboliterator)
  - [`MachOSymbol`](#machosymbol)
- [Traits](#traits)
  - [`Nlist`](#nlist)
- [Type Aliases](#type-aliases)
  - [`MachOSymbolTable32`](#machosymboltable32)
  - [`MachOSymbolTable64`](#machosymboltable64)
  - [`MachOSymbolIterator32`](#machosymboliterator32)
  - [`MachOSymbolIterator64`](#machosymboliterator64)
  - [`MachOSymbol32`](#machosymbol32)
  - [`MachOSymbol64`](#machosymbol64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SymbolTable`](#symboltable) | struct | A table of symbol entries in a Mach-O file. |
| [`MachOSymbolTable`](#machosymboltable) | struct | A symbol table in a [`MachOFile`]. |
| [`MachOSymbolIterator`](#machosymboliterator) | struct | An iterator for the symbols in a [`MachOFile`]. |
| [`MachOSymbol`](#machosymbol) | struct | A symbol in a [`MachOFile`]. |
| [`Nlist`](#nlist) | trait | A trait for generic access to [`macho::Nlist32`] and [`macho::Nlist64`]. |
| [`MachOSymbolTable32`](#machosymboltable32) | type | A symbol table in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSymbolTable64`](#machosymboltable64) | type | A symbol table in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSymbolIterator32`](#machosymboliterator32) | type | An iterator for the symbols in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSymbolIterator64`](#machosymboliterator64) | type | An iterator for the symbols in a [`MachOFile64`](super::MachOFile64). |
| [`MachOSymbol32`](#machosymbol32) | type | A symbol in a [`MachOFile32`](super::MachOFile32). |
| [`MachOSymbol64`](#machosymbol64) | type | A symbol in a [`MachOFile64`](super::MachOFile64). |

## Structs

### `SymbolTable<'data, Mach: MachHeader, R>`

```rust
struct SymbolTable<'data, Mach: MachHeader, R>
where
    R: ReadRef<'data> {
    symbols: &'data [<Mach as >::Nlist],
    strings: crate::read::util::StringTable<'data, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:23-29`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L23-L29)*

A table of symbol entries in a Mach-O file.

Also includes the string table used for the symbol names.

Returned by `macho::SymtabCommand::symbols`.

#### Implementations

- <span id="symboltable-new"></span>`fn new(symbols: &'data [<Mach as >::Nlist], strings: StringTable<'data, R>) -> Self` — [`MachHeader`](../index.md#machheader), [`StringTable`](../../index.md#stringtable)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md#stringtable)

  Return the string table used for the symbol names.

- <span id="symboltable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Mach as >::Nlist>` — [`MachHeader`](../index.md#machheader)

  Iterate over the symbols.

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the symbol table is empty.

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

  The number of symbols.

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Mach as >::Nlist>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`MachHeader`](../index.md#machheader)

  Return the symbol at the given index.

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Mach as >::Nlist) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../index.md#symbolmap)

  Construct a map from addresses to a user-defined map entry.

- <span id="symboltable-object-map"></span>`fn object_map(&self, endian: <Mach as >::Endian) -> ObjectMap<'data>` — [`MachHeader`](../index.md#machheader), [`ObjectMap`](../../../index.md#objectmap)

  Construct a map from addresses to symbol names and object file names.

#### Trait Implementations

##### `impl Any for SymbolTable<'data, Mach, R>`

- <span id="symboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolTable<'data, Mach, R>`

- <span id="symboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolTable<'data, Mach, R>`

- <span id="symboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach: clone::Clone + MachHeader, R> Clone for SymbolTable<'data, Mach, R>`

- <span id="symboltable-clone"></span>`fn clone(&self) -> SymbolTable<'data, Mach, R>` — [`SymbolTable`](../index.md#symboltable)

##### `impl CloneToUninit for SymbolTable<'data, Mach, R>`

- <span id="symboltable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Mach: marker::Copy + MachHeader, R> Copy for SymbolTable<'data, Mach, R>`

##### `impl<Mach: fmt::Debug + MachHeader, R> Debug for SymbolTable<'data, Mach, R>`

- <span id="symboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Mach: MachHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Mach, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

##### `impl<T> From for SymbolTable<'data, Mach, R>`

- <span id="symboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolTable<'data, Mach, R>`

- <span id="symboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SymbolTable<'data, Mach, R>`

- <span id="symboltable-toowned-type-owned"></span>`type Owned = T`

- <span id="symboltable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symboltable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolTable<'data, Mach, R>`

- <span id="symboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolTable<'data, Mach, R>`

- <span id="symboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSymbolTable<'data, 'file, Mach, R>`

```rust
struct MachOSymbolTable<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:184-190`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L184-L190)*

A symbol table in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl Any for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Clone for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-clone"></span>`fn clone(&self) -> MachOSymbolTable<'data, 'file, Mach, R>` — [`MachOSymbolTable`](../index.md#machosymboltable)

##### `impl CloneToUninit for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Mach, R> Copy for MachOSymbolTable<'data, 'file, Mach, R>`

##### `impl<Mach, R> Debug for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Mach, R> ObjectSymbolTable for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-objectsymboltable-type-symbol"></span>`type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymboltable-objectsymboltable-type-symboliterator"></span>`type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboltable-objectsymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md#objectsymboltable)

- <span id="machosymboltable-objectsymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ObjectSymbolTable`](../../index.md#objectsymboltable)

##### `impl<Mach, R> Sealed for MachOSymbolTable<'data, 'file, Mach, R>`

##### `impl ToOwned for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-toowned-type-owned"></span>`type Owned = T`

- <span id="machosymboltable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="machosymboltable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosymboltable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosymboltable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSymbolIterator<'data, 'file, Mach, R>`

```rust
struct MachOSymbolIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    index: crate::read::SymbolIndex,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:225-232`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L225-L232)*

An iterator for the symbols in a [`MachOFile`](../index.md).

#### Implementations

- <span id="machosymboliterator-new"></span>`fn new(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](../index.md#machofile)

- <span id="machosymboliterator-empty"></span>`fn empty(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](../index.md#machofile)

#### Trait Implementations

##### `impl Any for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosymboliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosymboliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Mach, R> Iterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-iterator-type-item"></span>`type Item = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymboliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosymboliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosymboliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MachOSymbol<'data, 'file, Mach, R>`

```rust
struct MachOSymbol<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    index: crate::read::SymbolIndex,
    nlist: &'data <Mach as >::Nlist,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:294-302`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L294-L302)*

A symbol in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- <span id="machosymbol-new"></span>`fn new(file: &'file MachOFile<'data, Mach, R>, index: SymbolIndex, nlist: &'data <Mach as >::Nlist) -> Option<Self>` — [`MachOFile`](../index.md#machofile), [`SymbolIndex`](../../../index.md#symbolindex), [`MachHeader`](../index.md#machheader)

- <span id="machosymbol-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md#machofile)

  Get the Mach-O file containing this symbol.

- <span id="machosymbol-macho-symbol"></span>`fn macho_symbol(&self) -> &'data <Mach as >::Nlist` — [`MachHeader`](../index.md#machheader)

  Get the raw Mach-O symbol structure.

#### Trait Implementations

##### `impl Any for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Clone for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-clone"></span>`fn clone(&self) -> MachOSymbol<'data, 'file, Mach, R>` — [`MachOSymbol`](../index.md#machosymbol)

##### `impl CloneToUninit for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Mach, R> Copy for MachOSymbol<'data, 'file, Mach, R>`

##### `impl<Mach, R> Debug for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Mach, R> ObjectSymbol for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-objectsymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="machosymbol-objectsymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machosymbol-objectsymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="machosymbol-objectsymbol-address"></span>`fn address(&self) -> u64`

- <span id="machosymbol-objectsymbol-size"></span>`fn size(&self) -> u64`

- <span id="machosymbol-objectsymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md#symbolkind)

- <span id="machosymbol-objectsymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md#symbolsection)

- <span id="machosymbol-objectsymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="machosymbol-objectsymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="machosymbol-objectsymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="machosymbol-objectsymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="machosymbol-objectsymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md#symbolscope)

- <span id="machosymbol-objectsymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="machosymbol-objectsymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="machosymbol-objectsymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md#symbolflags), [`SectionIndex`](../../../index.md#sectionindex), [`SymbolIndex`](../../../index.md#symbolindex)

##### `impl<Mach, R> Sealed for MachOSymbol<'data, 'file, Mach, R>`

##### `impl ToOwned for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-toowned-type-owned"></span>`type Owned = T`

- <span id="machosymbol-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="machosymbol-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machosymbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machosymbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Nlist`

```rust
trait Nlist: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:457-504`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L457-L504)*

A trait for generic access to [`macho::Nlist32`](../../../macho/index.md) and [`macho::Nlist64`](../../../macho/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Required Methods

- `fn n_strx(&self, endian: <Self as >::Endian) -> u32`

- `fn n_type(&self) -> u8`

- `fn n_sect(&self) -> u8`

- `fn n_desc(&self, endian: <Self as >::Endian) -> u16`

- `fn n_value(&self, endian: <Self as >::Endian) -> <Self as >::Word`

#### Provided Methods

- `fn name<'data, R: ReadRef<'data>>(&self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

- `fn is_stab(&self) -> bool`

  Return true if this is a STAB symbol.

- `fn is_undefined(&self) -> bool`

  Return true if this is an undefined symbol.

- `fn is_definition(&self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn library_ordinal(&self, endian: <Self as >::Endian) -> u8`

  Return the library ordinal.

#### Implementors

- [`Nlist32`](../../../macho/index.md#nlist32)
- [`Nlist64`](../../../macho/index.md#nlist64)

## Type Aliases

### `MachOSymbolTable32<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable32<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:176-177`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L176-L177)*

A symbol table in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolTable64<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable64<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:179-180`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L179-L180)*

A symbol table in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolIterator32<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator32<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:218-219`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L218-L219)*

An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolIterator64<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator64<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:221-222`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L221-L222)*

An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbol32<'data, 'file, Endian, R>`

```rust
type MachOSymbol32<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:284-285`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L284-L285)*

A symbol in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbol64<'data, 'file, Endian, R>`

```rust
type MachOSymbol64<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:287-288`](../../../../../.source_1765521767/object-0.37.3/src/read/macho/symbol.rs#L287-L288)*

A symbol in a [`MachOFile64`](super::MachOFile64).

