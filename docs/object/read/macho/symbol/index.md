*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [symbol](index.md)*

---

# Module `symbol`

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

A table of symbol entries in a Mach-O file.

Also includes the string table used for the symbol names.

Returned by `macho::SymtabCommand::symbols`.

#### Implementations

- `fn new(symbols: &'data [<Mach as >::Nlist], strings: StringTable<'data, R>) -> Self` — [`MachHeader`](../index.md), [`StringTable`](../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Mach as >::Nlist>` — [`MachHeader`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn symbol(self: &Self, index: SymbolIndex) -> Result<&'data <Mach as >::Nlist>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`MachHeader`](../index.md)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Mach as >::Nlist) -> Option<Entry>>(self: &Self, f: F) -> SymbolMap<Entry>` — [`SymbolMap`](../../../index.md)

- `fn object_map(self: &Self, endian: <Mach as >::Endian) -> ObjectMap<'data>` — [`MachHeader`](../index.md), [`ObjectMap`](../../../index.md)

#### Trait Implementations

##### `impl<'data, Mach: $crate::clone::Clone + MachHeader, R> Clone for SymbolTable<'data, Mach, R>`

- `fn clone(self: &Self) -> SymbolTable<'data, Mach, R>` — [`SymbolTable`](../index.md)

##### `impl<'data, Mach: $crate::marker::Copy + MachHeader, R> Copy for SymbolTable<'data, Mach, R>`

##### `impl<'data, Mach: $crate::fmt::Debug + MachHeader, R> Debug for SymbolTable<'data, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Mach: MachHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Mach, R>`

- `fn default() -> Self`

### `MachOSymbolTable<'data, 'file, Mach, R>`

```rust
struct MachOSymbolTable<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
}
```

A symbol table in a [`MachOFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbolTable<'data, 'file, Mach, R>`

- `fn clone(self: &Self) -> MachOSymbolTable<'data, 'file, Mach, R>` — [`MachOSymbolTable`](../index.md)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbolTable<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolTable<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbolTable for MachOSymbolTable<'data, 'file, Mach, R>`

- `type Symbol = MachOSymbol<'data, 'file, Mach, R>`

- `type SymbolIterator = MachOSymbolIterator<'data, 'file, Mach, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ObjectSymbolTable`](../../index.md)

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

An iterator for the symbols in a [`MachOFile`](../index.md).

#### Implementations

- `fn new(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](../index.md)

- `fn empty(file: &'file MachOFile<'data, Mach, R>) -> Self` — [`MachOFile`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbolIterator<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachOSymbolIterator<'data, 'file, Mach, R>`

- `type Item = MachOSymbol<'data, 'file, Mach, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A symbol in a [`MachOFile`](../index.md).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- `fn new(file: &'file MachOFile<'data, Mach, R>, index: SymbolIndex, nlist: &'data <Mach as >::Nlist) -> Option<Self>` — [`MachOFile`](../index.md), [`SymbolIndex`](../../../index.md), [`MachHeader`](../index.md)

- `fn macho_file(self: &Self) -> &'file MachOFile<'data, Mach, R>` — [`MachOFile`](../index.md)

- `fn macho_symbol(self: &Self) -> &'data <Mach as >::Nlist` — [`MachHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Clone for MachOSymbol<'data, 'file, Mach, R>`

- `fn clone(self: &Self) -> MachOSymbol<'data, 'file, Mach, R>` — [`MachOSymbol`](../index.md)

##### `impl<'data, 'file, Mach, R> Copy for MachOSymbol<'data, 'file, Mach, R>`

##### `impl<'data, 'file, Mach, R> Debug for MachOSymbol<'data, 'file, Mach, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Mach, R> ObjectSymbol for MachOSymbol<'data, 'file, Mach, R>`

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

##### `impl<'data, 'file, Mach, R> Sealed for MachOSymbol<'data, 'file, Mach, R>`

## Traits

### `Nlist`

```rust
trait Nlist: Debug + Pod { ... }
```

A trait for generic access to [`macho::Nlist32`](../../../macho/index.md) and [`macho::Nlist64`](../../../macho/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn n_strx(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_type(self: &Self) -> u8`

- `fn n_sect(self: &Self) -> u8`

- `fn n_desc(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn n_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> Result<&'data [u8]>`

- `fn is_stab(self: &Self) -> bool`

  Return true if this is a STAB symbol.

- `fn is_undefined(self: &Self) -> bool`

  Return true if this is an undefined symbol.

- `fn is_definition(self: &Self) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn library_ordinal(self: &Self, endian: <Self as >::Endian) -> u8`

  Return the library ordinal.

## Type Aliases

### `MachOSymbolTable32<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable32<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A symbol table in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolTable64<'data, 'file, Endian, R>`

```rust
type MachOSymbolTable64<'data, 'file, Endian, R> = MachOSymbolTable<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A symbol table in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbolIterator32<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator32<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the symbols in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbolIterator64<'data, 'file, Endian, R>`

```rust
type MachOSymbolIterator64<'data, 'file, Endian, R> = MachOSymbolIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the symbols in a [`MachOFile64`](super::MachOFile64).

### `MachOSymbol32<'data, 'file, Endian, R>`

```rust
type MachOSymbol32<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader32<Endian>, R>;
```

A symbol in a [`MachOFile32`](super::MachOFile32).

### `MachOSymbol64<'data, 'file, Endian, R>`

```rust
type MachOSymbol64<'data, 'file, Endian, R> = MachOSymbol<'data, 'file, macho::MachHeader64<Endian>, R>;
```

A symbol in a [`MachOFile64`](super::MachOFile64).

