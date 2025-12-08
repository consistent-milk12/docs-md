*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [symbol](index.md)*

---

# Module `symbol`

## Structs

### `SymbolTable<'data, Elf: FileHeader, R>`

```rust
struct SymbolTable<'data, Elf: FileHeader, R>
where
    R: ReadRef<'data> {
    section: crate::read::SectionIndex,
    string_section: crate::read::SectionIndex,
    shndx_section: crate::read::SectionIndex,
    symbols: &'data [<Elf as >::Sym],
    strings: crate::read::util::StringTable<'data, R>,
    shndx: &'data [crate::endian::U32<<Elf as >::Endian>],
}
```

A table of symbol entries in an ELF file.

Also includes the string table used for the symbol names.

Returned by `SectionTable::symbols`.

#### Implementations

- `fn parse(endian: <Elf as >::Endian, data: R, sections: &SectionTable<'data, Elf, R>, section_index: SectionIndex, section: &<Elf as >::SectionHeader) -> read::Result<SymbolTable<'data, Elf, R>>` — [`FileHeader`](../index.md), [`SectionTable`](../index.md), [`SectionIndex`](../../../index.md), [`Result`](../../../index.md), [`SymbolTable`](../index.md)

- `fn section(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn shndx_section(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn string_section(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn strings(self: &Self) -> StringTable<'data, R>` — [`StringTable`](../../index.md)

- `fn symbols(self: &Self) -> &'data [<Elf as >::Sym]` — [`FileHeader`](../index.md)

- `fn iter(self: &Self) -> slice::Iter<'data, <Elf as >::Sym>` — [`FileHeader`](../index.md)

- `fn enumerate(self: &Self) -> impl Iterator<Item = (SymbolIndex, &'data <Elf as >::Sym)>` — [`SymbolIndex`](../../../index.md), [`FileHeader`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn symbol(self: &Self, index: SymbolIndex) -> read::Result<&'data <Elf as >::Sym>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`FileHeader`](../index.md)

- `fn shndx(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> Option<u32>` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md)

- `fn symbol_section(self: &Self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym, index: SymbolIndex) -> read::Result<Option<SectionIndex>>` — [`FileHeader`](../index.md), [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`SectionIndex`](../../../index.md)

- `fn symbol_name(self: &Self, endian: <Elf as >::Endian, symbol: &<Elf as >::Sym) -> read::Result<&'data [u8]>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- `fn map<Entry: SymbolMapEntry, F: Fn(&'data <Elf as >::Sym) -> Option<Entry>>(self: &Self, endian: <Elf as >::Endian, f: F) -> SymbolMap<Entry>` — [`FileHeader`](../index.md), [`SymbolMap`](../../../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader, R> Clone for SymbolTable<'data, Elf, R>`

- `fn clone(self: &Self) -> SymbolTable<'data, Elf, R>` — [`SymbolTable`](../index.md)

##### `impl<'data, Elf: $crate::marker::Copy + FileHeader, R> Copy for SymbolTable<'data, Elf, R>`

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader, R> Debug for SymbolTable<'data, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, Elf: FileHeader, R: ReadRef<'data>> Default for SymbolTable<'data, Elf, R>`

- `fn default() -> Self`

### `ElfSymbolTable<'data, 'file, Elf, R>`

```rust
struct ElfSymbolTable<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
}
```

A symbol table in an [`ElfFile`](super::ElfFile).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Clone for ElfSymbolTable<'data, 'file, Elf, R>`

- `fn clone(self: &Self) -> ElfSymbolTable<'data, 'file, Elf, R>` — [`ElfSymbolTable`](../index.md)

##### `impl<'data, 'file, Elf, R> Copy for ElfSymbolTable<'data, 'file, Elf, R>`

##### `impl<'data, 'file, Elf, R> Debug for ElfSymbolTable<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> ObjectSymbolTable for ElfSymbolTable<'data, 'file, Elf, R>`

- `type Symbol = ElfSymbol<'data, 'file, Elf, R>`

- `type SymbolIterator = ElfSymbolIterator<'data, 'file, Elf, R>`

- `fn symbols(self: &Self) -> <Self as >::SymbolIterator` — [`ObjectSymbolTable`](../../index.md)

- `fn symbol_by_index(self: &Self, index: SymbolIndex) -> read::Result<<Self as >::Symbol>` — [`SymbolIndex`](../../../index.md), [`Result`](../../../index.md), [`ObjectSymbolTable`](../../index.md)

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbolTable<'data, 'file, Elf, R>`

### `ElfSymbolIterator<'data, 'file, Elf, R>`

```rust
struct ElfSymbolIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
}
```

An iterator for the symbols in an [`ElfFile`](super::ElfFile).

#### Implementations

- `fn new(endian: <Elf as >::Endian, symbols: &'file SymbolTable<'data, Elf, R>) -> Self` — [`FileHeader`](../index.md), [`SymbolTable`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Debug for ElfSymbolIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Iterator for ElfSymbolIterator<'data, 'file, Elf, R>`

- `type Item = ElfSymbol<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSymbol<'data, 'file, Elf, R>`

```rust
struct ElfSymbol<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    endian: <Elf as >::Endian,
    symbols: &'file SymbolTable<'data, Elf, R>,
    index: crate::read::SymbolIndex,
    symbol: &'data <Elf as >::Sym,
}
```

A symbol in an [`ElfFile`](super::ElfFile).

Most functionality is provided by the [`ObjectSymbol`](../../index.md) trait implementation.

#### Implementations

- `fn endian(self: &Self) -> <Elf as >::Endian` — [`FileHeader`](../index.md)

- `fn raw_symbol(self: &Self) -> &'data <Elf as >::Sym` — [`FileHeader`](../index.md)

- `fn elf_symbol(self: &Self) -> &'data <Elf as >::Sym` — [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Clone for ElfSymbol<'data, 'file, Elf, R>`

- `fn clone(self: &Self) -> ElfSymbol<'data, 'file, Elf, R>` — [`ElfSymbol`](../index.md)

##### `impl<'data, 'file, Elf, R> Copy for ElfSymbol<'data, 'file, Elf, R>`

##### `impl<'data, 'file, Elf, R> Debug for ElfSymbol<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> ObjectSymbol for ElfSymbol<'data, 'file, Elf, R>`

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

##### `impl<'data, 'file, Elf: FileHeader, R: ReadRef<'data>> Sealed for ElfSymbol<'data, 'file, Elf, R>`

## Traits

### `Sym`

```rust
trait Sym: Debug + Pod { ... }
```

A trait for generic access to [`elf::Sym32`](../../../elf/index.md) and [`elf::Sym64`](../../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `fn st_name(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn st_info(self: &Self) -> u8`

- `fn st_bind(self: &Self) -> u8`

- `fn st_type(self: &Self) -> u8`

- `fn st_other(self: &Self) -> u8`

- `fn st_visibility(self: &Self) -> u8`

- `fn st_shndx(self: &Self, endian: <Self as >::Endian) -> u16`

- `fn st_value(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn st_size(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn name<'data, R: ReadRef<'data>>(self: &Self, endian: <Self as >::Endian, strings: StringTable<'data, R>) -> read::Result<&'data [u8]>`

  Parse the symbol name from the string table.

- `fn is_undefined(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_UNDEF`.

- `fn is_definition(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol is a definition of a function or data object.

- `fn is_common(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_COMMON`.

- `fn is_absolute(self: &Self, endian: <Self as >::Endian) -> bool`

  Return true if the symbol section is `SHN_ABS`.

- `fn is_local(self: &Self) -> bool`

  Return true if the symbol binding is `STB_LOCAL`.

- `fn is_weak(self: &Self) -> bool`

  Return true if the symbol binding is `STB_WEAK`.

## Type Aliases

### `ElfSymbolTable32<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable32<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolTable64<'data, 'file, Endian, R>`

```rust
type ElfSymbolTable64<'data, 'file, Endian, R> = ElfSymbolTable<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol table in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator32<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator32<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbolIterator64<'data, 'file, Endian, R>`

```rust
type ElfSymbolIterator64<'data, 'file, Endian, R> = ElfSymbolIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the symbols in an [`ElfFile64`](super::ElfFile64).

### `ElfSymbol32<'data, 'file, Endian, R>`

```rust
type ElfSymbol32<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A symbol in an [`ElfFile32`](super::ElfFile32).

### `ElfSymbol64<'data, 'file, Endian, R>`

```rust
type ElfSymbol64<'data, 'file, Endian, R> = ElfSymbol<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A symbol in an [`ElfFile64`](super::ElfFile64).

