*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Structs

### `RelocationSections`

```rust
struct RelocationSections {
    relocations: alloc::vec::Vec<usize>,
}
```

A mapping from section index to associated relocation sections.

#### Implementations

- `fn parse<'data, Elf: FileHeader, R: ReadRef<'data>>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` — [`FileHeader`](../index.md), [`SectionTable`](../index.md), [`SectionIndex`](../../../index.md), [`Result`](../../../index.md)

- `fn get(self: &Self, index: SectionIndex) -> Option<SectionIndex>` — [`SectionIndex`](../../../index.md)

#### Trait Implementations

##### `impl Debug for RelocationSections`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RelocationSections`

- `fn default() -> RelocationSections` — [`RelocationSections`](../index.md)

### `ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

```rust
struct ElfDynamicRelocationIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    section_index: crate::read::SectionIndex,
    file: &'file super::ElfFile<'data, Elf, R>,
    relocations: Option<ElfRelocationIterator<'data, Elf>>,
}
```

An iterator for the dynamic relocations in an [`ElfFile`](../index.md).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current relocation section index.

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ElfSectionRelocationIterator<'data, 'file, Elf, R>`

```rust
struct ElfSectionRelocationIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    section_index: crate::read::SectionIndex,
    file: &'file super::ElfFile<'data, Elf, R>,
    relocations: Option<ElfRelocationIterator<'data, Elf>>,
}
```

An iterator for the relocations for an [`ElfSection`](super::ElfSection).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current pointer in the chain of relocation sections.

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `RelrIterator<'data, Elf: FileHeader>`

```rust
struct RelrIterator<'data, Elf: FileHeader> {
    offset: <Elf as >::Word,
    bits: <Elf as >::Word,
    count: u8,
    iter: slice::Iter<'data, <Elf as >::Relr>,
    endian: <Elf as >::Endian,
}
```

An iterator over the relative relocations in an ELF `SHT_RELR` section.

Returned by [`SectionHeader::relr`](super::SectionHeader::relr).

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` — [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for RelrIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for RelrIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for RelrIterator<'data, Elf>`

- `type Item = <Elf as FileHeader>::Word`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Crel`

```rust
struct Crel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}
```

Compact relocation

The specification has been submited here: <https://groups.google.com/g/generic-abi/c/ppkaxtLb0P0/m/awgqZ_1CBAAJ>.

#### Fields

- **`r_offset`**: `u64`

  Relocation offset.

- **`r_sym`**: `u32`

  Relocation symbol index.

- **`r_type`**: `u32`

  Relocation type.

- **`r_addend`**: `i64`

  Relocation addend.
  
  Only set if `CrelIterator::is_rela()` returns `true`.

#### Implementations

- `fn symbol(self: &Self) -> Option<SymbolIndex>` — [`SymbolIndex`](../../../index.md)

- `fn from_rel<R: Rel>(r: &R, endian: <R as >::Endian) -> Crel` — [`Rel`](../index.md), [`Crel`](../index.md)

- `fn from_rela<R: Rela>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` — [`Rela`](../index.md), [`Crel`](../index.md)

#### Trait Implementations

##### `impl Clone for Crel`

- `fn clone(self: &Self) -> Crel` — [`Crel`](../index.md)

##### `impl Copy for Crel`

##### `impl Debug for Crel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CrelIteratorHeader`

```rust
struct CrelIteratorHeader {
    count: usize,
    flag_bits: u64,
    shift: u64,
    is_rela: bool,
}
```

#### Fields

- **`count`**: `usize`

  The number of encoded relocations.

- **`flag_bits`**: `u64`

  The number of flag bits each relocation uses.

- **`shift`**: `u64`

  Shift of the relocation value.

- **`is_rela`**: `bool`

  True if the relocation format encodes addend.

#### Trait Implementations

##### `impl Clone for CrelIteratorHeader`

- `fn clone(self: &Self) -> CrelIteratorHeader` — [`CrelIteratorHeader`](#creliteratorheader)

##### `impl Debug for CrelIteratorHeader`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CrelIteratorState`

```rust
struct CrelIteratorState {
    index: usize,
    offset: u64,
    addend: i64,
    symidx: u32,
    typ: u32,
}
```

#### Fields

- **`index`**: `usize`

  Index of the current relocation.

- **`offset`**: `u64`

  Offset of the latest relocation.

- **`addend`**: `i64`

  Addend of the latest relocation.

- **`symidx`**: `u32`

  Symbol index of the latest relocation.

- **`typ`**: `u32`

  Type of the latest relocation.

#### Trait Implementations

##### `impl Clone for CrelIteratorState`

- `fn clone(self: &Self) -> CrelIteratorState` — [`CrelIteratorState`](#creliteratorstate)

##### `impl Debug for CrelIteratorState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for CrelIteratorState`

- `fn default() -> CrelIteratorState` — [`CrelIteratorState`](#creliteratorstate)

### `CrelIterator<'data>`

```rust
struct CrelIterator<'data> {
    data: crate::read::Bytes<'data>,
    header: CrelIteratorHeader,
    state: CrelIteratorState,
}
```

Compact relocation iterator.

#### Fields

- **`data`**: `crate::read::Bytes<'data>`

  Input stream reader.

- **`header`**: `CrelIteratorHeader`

  Parsed header information.

- **`state`**: `CrelIteratorState`

  State of the iterator.

#### Implementations

- `fn new(data: &'data [u8]) -> Result<Self, Error>` — [`Error`](../../../index.md)

- `fn is_rela(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn parse(self: &mut Self) -> read::Result<Crel>` — [`Result`](../../../index.md), [`Crel`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for CrelIterator<'data>`

- `fn clone(self: &Self) -> CrelIterator<'data>` — [`CrelIterator`](../index.md)

##### `impl<'data> Debug for CrelIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CrelIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for CrelIterator<'data>`

- `type Item = Result<Crel, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Enums

### `ElfRelocationIterator<'data, Elf: FileHeader>`

```rust
enum ElfRelocationIterator<'data, Elf: FileHeader> {
    Rel(slice::Iter<'data, <Elf as >::Rel>, <Elf as >::Endian),
    Rela(slice::Iter<'data, <Elf as >::Rela>, <Elf as >::Endian, bool),
    Crel(CrelIterator<'data>),
}
```

#### Implementations

- `fn is_rel(self: &Self) -> bool`

#### Trait Implementations

##### `impl<I> IntoIterator for ElfRelocationIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for ElfRelocationIterator<'data, Elf>`

- `type Item = Crel`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `Rel`

```rust
trait Rel: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Rel32`](../../../elf/index.md) and [`elf::Rel64`](../../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn symbol(self: &Self, endian: <Self as >::Endian) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

### `Rela`

```rust
trait Rela: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Rela32`](../../../elf/index.md) and [`elf::Rela64`](../../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

- `fn r_offset(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(self: &Self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn r_type(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn symbol(self: &Self, endian: <Self as >::Endian, is_mips64el: bool) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

### `Relr`

```rust
trait Relr: Debug + Pod + Clone { ... }
```

A trait for generic access to [`elf::Relr32`](../../../elf/index.md) and [`elf::Relr64`](../../../elf/index.md).

#### Required Methods

- `type Word: 1`

- `type Endian: 1`

- `const COUNT: u8`

- `fn get(self: &Self, endian: <Self as >::Endian) -> <Self as >::Word`

  Get the relocation entry.

- `fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>`

  Return the offset corresponding to the next bit in the bit mask.

## Functions

### `parse_relocation`

```rust
fn parse_relocation<Elf: FileHeader>(header: &Elf, endian: <Elf as >::Endian, reloc: Crel, implicit_addend: bool) -> crate::read::Relocation
```

## Type Aliases

### `ElfDynamicRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator32<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).

### `ElfDynamicRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator64<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).

### `ElfSectionRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator32<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).

### `ElfSectionRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator64<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).

