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

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:23-29`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L23-L29)*

A table of symbol entries in a Mach-O file.

Also includes the string table used for the symbol names.

Returned by `macho::SymtabCommand::symbols`.

#### Implementations

- <span id="symboltable-new"></span>`fn new(symbols: &'data [<Mach as >::Nlist], strings: StringTable<'data, R>) -> Self` — [`MachHeader`](../index.md#machheader), [`StringTable`](../../index.md#stringtable)

- <span id="symboltable-strings"></span>`fn strings(&self) -> StringTable<'data, R>` — [`StringTable`](../../index.md#stringtable)

- <span id="symboltable-iter"></span>`fn iter(&self) -> slice::Iter<'data, <Mach as >::Nlist>` — [`MachHeader`](../index.md#machheader)

- <span id="symboltable-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="symboltable-len"></span>`fn len(&self) -> usize`

- <span id="symboltable-symbol"></span>`fn symbol(&self, index: SymbolIndex) -> Result<&'data <Mach as >::Nlist>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`MachHeader`](../index.md#machheader)

- <span id="symboltable-map"></span>`fn map<Entry: SymbolMapEntry, F: Fn(&'data <Mach as >::Nlist) -> Option<Entry>>(&self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../index.md#symbolmap)

- <span id="symboltable-object-map"></span>`fn object_map(&self, endian: <Mach as >::Endian) -> ObjectMap<'data>` — [`MachHeader`](../index.md#machheader), [`ObjectMap`](../../../index.md#objectmap)

#### Trait Implementations

##### `impl<'data, Mach: clone::Clone + MachHeader, R> Clone for SymbolTable<'data, Mach, R>`

- <span id="symboltable-clone"></span>`fn clone(&self) -> SymbolTable<'data, Mach, R>` — [`SymbolTable`](../index.md#symboltable)

##### `impl<'data, Mach: marker::Copy + MachHeader, R> Copy for SymbolTable<'data, Mach, R>`

##### `impl<'data, Mach: fmt::Debug + MachHeader, R> Debug for SymbolTable<'data, Mach, R>`

- <span id="symboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, Mach: MachHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Mach, R>`

- <span id="symboltable-default"></span>`fn default() -> Self`

### `MachOSymbolTable<'data, 'file, Mach, R>`

```rust
struct MachOSymbolTable<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
}
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:184-190`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L184-L190)*

A symbol table in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-clone"></span>`fn clone(&self) -> MachOSymbolTable<'data, 'file, Mach, R>` — [`MachOSymbolTable`](../index.md#machosymboltable)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbolTable<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbolTable for MachOSymbolTable<'data, 'file, Mach, R>`

- <span id="machosymboltable-type-symbol"></span>`type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymboltable-type-symboliterator"></span>`type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboltable-symbols"></span>`fn symbols(&self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md#objectsymboltable)

- <span id="machosymboltable-symbol-by-index"></span>`fn symbol_by_index(&self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md#symbolindex), [`Result`](../../../index.md#result), [`ObjectSymbolTable`](../../index.md#objectsymboltable)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSymbolTable<'data, 'file, Mach, R>`

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

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:225-232`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L225-L232)*

An iterator for the symbols in a [`MachOFile`](../index.md).

#### Implementations

- <span id="machosymboliterator-new"></span>`fn new(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](../index.md#machofile)

- <span id="machosymboliterator-empty"></span>`fn empty(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](../index.md#machofile)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machosymboliterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machosymboliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- <span id="machosymboliterator-type-item"></span>`type Item = MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymboliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:294-302`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L294-L302)*

A symbol in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- <span id="machosymbol-new"></span>`fn new(file: &'file MachOFile<'data, Mach, R>, index: SymbolIndex, nlist: &'data <Mach as >::Nlist) -> Option<Self>` — [`MachOFile`](../index.md#machofile), [`SymbolIndex`](../../../index.md#symbolindex), [`MachHeader`](../index.md#machheader)

- <span id="machosymbol-macho-file"></span>`fn macho_file(&self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md#machofile)

- <span id="machosymbol-macho-symbol"></span>`fn macho_symbol(&self) -> &'data <Mach as >::Nlist` — [`MachHeader`](../index.md#machheader)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-clone"></span>`fn clone(&self) -> MachOSymbol<'data, 'file, Mach, R>` — [`MachOSymbol`](../index.md#machosymbol)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbol<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbol for MachOSymbol<'data, 'file, Mach, R>`

- <span id="machosymbol-index"></span>`fn index(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="machosymbol-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="machosymbol-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="machosymbol-address"></span>`fn address(&self) -> u64`

- <span id="machosymbol-size"></span>`fn size(&self) -> u64`

- <span id="machosymbol-kind"></span>`fn kind(&self) -> SymbolKind` — [`SymbolKind`](../../../index.md#symbolkind)

- <span id="machosymbol-section"></span>`fn section(&self) -> SymbolSection` — [`SymbolSection`](../../../index.md#symbolsection)

- <span id="machosymbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

- <span id="machosymbol-is-definition"></span>`fn is_definition(&self) -> bool`

- <span id="machosymbol-is-common"></span>`fn is_common(&self) -> bool`

- <span id="machosymbol-is-weak"></span>`fn is_weak(&self) -> bool`

- <span id="machosymbol-scope"></span>`fn scope(&self) -> SymbolScope` — [`SymbolScope`](../../../index.md#symbolscope)

- <span id="machosymbol-is-global"></span>`fn is_global(&self) -> bool`

- <span id="machosymbol-is-local"></span>`fn is_local(&self) -> bool`

- <span id="machosymbol-flags"></span>`fn flags(&self) -> SymbolFlags<SectionIndex, SymbolIndex>` — [`SymbolFlags`](../../../index.md#symbolflags), [`SectionIndex`](../../../index.md#sectionindex), [`SymbolIndex`](../../../index.md#symbolindex)

##### `impl<'data, 'file, Mach, R> Sealed for MachOSymbol<'data, 'file, Mach, R>`

## Traits

### `Nlist`

```rust
trait Nlist: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:457-504`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L457-L504)*

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

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:176-177`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L176-L177)*

A symbol table in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolTable64<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable64<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:179-180`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L179-L180)*

A symbol table in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolIterator32<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator32<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:218-219`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L218-L219)*

An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolIterator64<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator64<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:221-222`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L221-L222)*

An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbol32<'data, 'file, Endian, R>`

```rust
type MachOSymbol32<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:284-285`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L284-L285)*

A symbol in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbol64<'data, 'file, Endian, R>`

```rust
type MachOSymbol64<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/symbol.rs:287-288`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/symbol.rs#L287-L288)*

A symbol in a [`MachOFile64`](super::MachOFile64).

