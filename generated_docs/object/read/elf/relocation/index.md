*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Contents

- [Structs](#structs)
  - [`RelocationSections`](#relocationsections)
  - [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator)
  - [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator)
  - [`RelrIterator`](#relriterator)
  - [`Crel`](#crel)
  - [`CrelIteratorHeader`](#creliteratorheader)
  - [`CrelIteratorState`](#creliteratorstate)
  - [`CrelIterator`](#creliterator)
- [Enums](#enums)
  - [`ElfRelocationIterator`](#elfrelocationiterator)
- [Traits](#traits)
  - [`Rel`](#rel)
  - [`Rela`](#rela)
  - [`Relr`](#relr)
- [Functions](#functions)
  - [`parse_relocation`](#parse_relocation)
- [Type Aliases](#type-aliases)
  - [`ElfDynamicRelocationIterator32`](#elfdynamicrelocationiterator32)
  - [`ElfDynamicRelocationIterator64`](#elfdynamicrelocationiterator64)
  - [`ElfSectionRelocationIterator32`](#elfsectionrelocationiterator32)
  - [`ElfSectionRelocationIterator64`](#elfsectionrelocationiterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RelocationSections`](#relocationsections) | struct | A mapping from section index to associated relocation sections. |
| [`ElfDynamicRelocationIterator`](#elfdynamicrelocationiterator) | struct | An iterator for the dynamic relocations in an [`ElfFile`]. |
| [`ElfSectionRelocationIterator`](#elfsectionrelocationiterator) | struct | An iterator for the relocations for an [`ElfSection`](super::ElfSection). |
| [`RelrIterator`](#relriterator) | struct | An iterator over the relative relocations in an ELF `SHT_RELR` section. |
| [`Crel`](#crel) | struct | Compact relocation |
| [`CrelIteratorHeader`](#creliteratorheader) | struct |  |
| [`CrelIteratorState`](#creliteratorstate) | struct |  |
| [`CrelIterator`](#creliterator) | struct | Compact relocation iterator. |
| [`ElfRelocationIterator`](#elfrelocationiterator) | enum |  |
| [`Rel`](#rel) | trait | A trait for generic access to [`elf::Rel32`] and [`elf::Rel64`]. |
| [`Rela`](#rela) | trait | A trait for generic access to [`elf::Rela32`] and [`elf::Rela64`]. |
| [`Relr`](#relr) | trait | A trait for generic access to [`elf::Relr32`] and [`elf::Relr64`]. |
| [`parse_relocation`](#parse_relocation) | fn |  |
| [`ElfDynamicRelocationIterator32`](#elfdynamicrelocationiterator32) | type | An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32). |
| [`ElfDynamicRelocationIterator64`](#elfdynamicrelocationiterator64) | type | An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64). |
| [`ElfSectionRelocationIterator32`](#elfsectionrelocationiterator32) | type | An iterator for the relocations for an [`ElfSection32`](super::ElfSection32). |
| [`ElfSectionRelocationIterator64`](#elfsectionrelocationiterator64) | type | An iterator for the relocations for an [`ElfSection64`](super::ElfSection64). |

## Structs

### `RelocationSections`

```rust
struct RelocationSections {
    relocations: alloc::vec::Vec<usize>,
}
```

A mapping from section index to associated relocation sections.

#### Implementations

- <span id="relocationsections-parse"></span>`fn parse<'data, Elf: FileHeader, R: ReadRef<'data>>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` — [`FileHeader`](../index.md), [`SectionTable`](../index.md), [`SectionIndex`](../../../index.md), [`Result`](../../../index.md)

- <span id="relocationsections-get"></span>`fn get(&self, index: SectionIndex) -> Option<SectionIndex>` — [`SectionIndex`](../../../index.md)

#### Trait Implementations

##### `impl Debug for RelocationSections`

- <span id="relocationsections-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationSections`

- <span id="relocationsections-default"></span>`fn default() -> RelocationSections` — [`RelocationSections`](../index.md)

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

- <span id="elfdynamicrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfdynamicrelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfdynamicrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-item"></span>`type Item = (u64, Relocation)`

- <span id="elfdynamicrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="elfsectionrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectionrelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfsectionrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-item"></span>`type Item = (u64, Relocation)`

- <span id="elfsectionrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="relriterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` — [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: fmt::Debug + FileHeader> Debug for RelrIterator<'data, Elf>`

- <span id="relriterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for RelrIterator<'data, Elf>`

- <span id="relriterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relriterator-intoiter"></span>`type IntoIter = I`

- <span id="relriterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for RelrIterator<'data, Elf>`

- <span id="relriterator-item"></span>`type Item = <Elf as FileHeader>::Word`

- <span id="relriterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="crel-symbol"></span>`fn symbol(&self) -> Option<SymbolIndex>` — [`SymbolIndex`](../../../index.md)

- <span id="crel-from-rel"></span>`fn from_rel<R: Rel>(r: &R, endian: <R as >::Endian) -> Crel` — [`Rel`](../index.md), [`Crel`](../index.md)

- <span id="crel-from-rela"></span>`fn from_rela<R: Rela>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` — [`Rela`](../index.md), [`Crel`](../index.md)

#### Trait Implementations

##### `impl Clone for Crel`

- <span id="crel-clone"></span>`fn clone(&self) -> Crel` — [`Crel`](../index.md)

##### `impl Copy for Crel`

##### `impl Debug for Crel`

- <span id="crel-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="creliteratorheader-clone"></span>`fn clone(&self) -> CrelIteratorHeader` — [`CrelIteratorHeader`](#creliteratorheader)

##### `impl Debug for CrelIteratorHeader`

- <span id="creliteratorheader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="creliteratorstate-clone"></span>`fn clone(&self) -> CrelIteratorState` — [`CrelIteratorState`](#creliteratorstate)

##### `impl Debug for CrelIteratorState`

- <span id="creliteratorstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrelIteratorState`

- <span id="creliteratorstate-default"></span>`fn default() -> CrelIteratorState` — [`CrelIteratorState`](#creliteratorstate)

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

- <span id="creliterator-new"></span>`fn new(data: &'data [u8]) -> Result<Self, Error>` — [`Error`](../../../index.md)

- <span id="creliterator-is-rela"></span>`fn is_rela(&self) -> bool`

- <span id="creliterator-len"></span>`fn len(&self) -> usize`

- <span id="creliterator-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="creliterator-parse"></span>`fn parse(&mut self) -> read::Result<Crel>` — [`Result`](../../../index.md), [`Crel`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for CrelIterator<'data>`

- <span id="creliterator-clone"></span>`fn clone(&self) -> CrelIterator<'data>` — [`CrelIterator`](../index.md)

##### `impl<'data> Debug for CrelIterator<'data>`

- <span id="creliterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CrelIterator<'data>`

- <span id="creliterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="creliterator-intoiter"></span>`type IntoIter = I`

- <span id="creliterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data> Iterator for CrelIterator<'data>`

- <span id="creliterator-item"></span>`type Item = Result<Crel, Error>`

- <span id="creliterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="creliterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="elfrelocationiterator-is-rel"></span>`fn is_rel(&self) -> bool`

#### Trait Implementations

##### `impl<I> IntoIterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfrelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-item"></span>`type Item = Crel`

- <span id="elfrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- `fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(&self, endian: <Self as >::Endian) -> u32`

- `fn r_type(&self, endian: <Self as >::Endian) -> u32`

- `fn symbol(&self, endian: <Self as >::Endian) -> Option<SymbolIndex>`

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

- `fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(&self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(&self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn r_type(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn symbol(&self, endian: <Self as >::Endian, is_mips64el: bool) -> Option<SymbolIndex>`

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

- `fn get(&self, endian: <Self as >::Endian) -> <Self as >::Word`

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

