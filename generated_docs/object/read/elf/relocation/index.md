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
  - [`parse_relocation`](#parse-relocation)
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
| [`parse_relocation`](#parse-relocation) | fn |  |
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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:18-20`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L18-L20)*

A mapping from section index to associated relocation sections.

#### Implementations

- <span id="relocationsections-parse"></span>`fn parse<'data, Elf: FileHeader, R: ReadRef<'data>>(endian: <Elf as >::Endian, sections: &SectionTable<'data, Elf, R>, symbol_section: SectionIndex) -> read::Result<Self>` — [`FileHeader`](../index.md#fileheader), [`SectionTable`](../index.md#sectiontable), [`SectionIndex`](../../../index.md#sectionindex), [`Result`](../../../index.md#result)

  Create a new mapping using the section table.

  

  Skips relocation sections that do not use the given symbol table section.

- <span id="relocationsections-get"></span>`fn get(&self, index: SectionIndex) -> Option<SectionIndex>` — [`SectionIndex`](../../../index.md#sectionindex)

  Given a section index, return the section index of the associated relocation section.

  

  This may also be called with a relocation section index, and it will return the

  next associated relocation section.

#### Trait Implementations

##### `impl Any for RelocationSections`

- <span id="relocationsections-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationSections`

- <span id="relocationsections-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationSections`

- <span id="relocationsections-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RelocationSections`

- <span id="relocationsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationSections`

- <span id="relocationsections-default"></span>`fn default() -> RelocationSections` — [`RelocationSections`](../index.md#relocationsections)

##### `impl<T> From for RelocationSections`

- <span id="relocationsections-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationSections`

- <span id="relocationsections-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RelocationSections`

- <span id="relocationsections-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationsections-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationSections`

- <span id="relocationsections-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationsections-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:123-132`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L123-L132)*

An iterator for the dynamic relocations in an [`ElfFile`](../index.md).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current relocation section index.

#### Trait Implementations

##### `impl Any for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfdynamicrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfdynamicrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="elfdynamicrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfdynamicrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfDynamicRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfdynamicrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfdynamicrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:207-216`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L207-L216)*

An iterator for the relocations for an [`ElfSection`](super::ElfSection).

#### Fields

- **`section_index`**: `crate::read::SectionIndex`

  The current pointer in the chain of relocation sections.

#### Trait Implementations

##### `impl Any for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfsectionrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfsectionrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="elfsectionrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfsectionrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfSectionRelocationIterator<'data, 'file, Elf, R>`

- <span id="elfsectionrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfsectionrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:681-687`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L681-L687)*

An iterator over the relative relocations in an ELF `SHT_RELR` section.

Returned by [`SectionHeader::relr`](super::SectionHeader::relr).

#### Implementations

- <span id="relriterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [<Elf as >::Relr]) -> Self` — [`FileHeader`](../index.md#fileheader)

  Create a new iterator given the `SHT_RELR` section data.

#### Trait Implementations

##### `impl Any for RelrIterator<'data, Elf>`

- <span id="relriterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelrIterator<'data, Elf>`

- <span id="relriterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelrIterator<'data, Elf>`

- <span id="relriterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for RelrIterator<'data, Elf>`

- <span id="relriterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelrIterator<'data, Elf>`

- <span id="relriterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelrIterator<'data, Elf>`

- <span id="relriterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RelrIterator<'data, Elf>`

- <span id="relriterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relriterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relriterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for RelrIterator<'data, Elf>`

- <span id="relriterator-iterator-type-item"></span>`type Item = <Elf as FileHeader>::Word`

- <span id="relriterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for RelrIterator<'data, Elf>`

- <span id="relriterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relriterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelrIterator<'data, Elf>`

- <span id="relriterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relriterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Crel`

```rust
struct Crel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:792-803`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L792-L803)*

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

- <span id="crel-symbol"></span>`fn symbol(&self) -> Option<SymbolIndex>` — [`SymbolIndex`](../../../index.md#symbolindex)

  Get the symbol index referenced by the relocation.

  

  Returns `None` for the null symbol index.

- <span id="crel-from-rel"></span>`fn from_rel<R: Rel>(r: &R, endian: <R as >::Endian) -> Crel` — [`Rel`](../index.md#rel), [`Crel`](../index.md#crel)

  Build Crel type from Rel.

- <span id="crel-from-rela"></span>`fn from_rela<R: Rela>(r: &R, endian: <R as >::Endian, is_mips64el: bool) -> Crel` — [`Rela`](../index.md#rela), [`Crel`](../index.md#crel)

  Build Crel type from Rela.

#### Trait Implementations

##### `impl Any for Crel`

- <span id="crel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Crel`

- <span id="crel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Crel`

- <span id="crel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Crel`

- <span id="crel-clone"></span>`fn clone(&self) -> Crel` — [`Crel`](../index.md#crel)

##### `impl CloneToUninit for Crel`

- <span id="crel-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Crel`

##### `impl Debug for Crel`

- <span id="crel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Crel`

- <span id="crel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Crel`

- <span id="crel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Crel`

- <span id="crel-toowned-type-owned"></span>`type Owned = T`

- <span id="crel-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="crel-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Crel`

- <span id="crel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="crel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Crel`

- <span id="crel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="crel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CrelIteratorHeader`

```rust
struct CrelIteratorHeader {
    count: usize,
    flag_bits: u64,
    shift: u64,
    is_rela: bool,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:839-848`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L839-L848)*

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

##### `impl Any for CrelIteratorHeader`

- <span id="creliteratorheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrelIteratorHeader`

- <span id="creliteratorheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrelIteratorHeader`

- <span id="creliteratorheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrelIteratorHeader`

- <span id="creliteratorheader-clone"></span>`fn clone(&self) -> CrelIteratorHeader` — [`CrelIteratorHeader`](#creliteratorheader)

##### `impl CloneToUninit for CrelIteratorHeader`

- <span id="creliteratorheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrelIteratorHeader`

- <span id="creliteratorheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CrelIteratorHeader`

- <span id="creliteratorheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CrelIteratorHeader`

- <span id="creliteratorheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CrelIteratorHeader`

- <span id="creliteratorheader-toowned-type-owned"></span>`type Owned = T`

- <span id="creliteratorheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="creliteratorheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CrelIteratorHeader`

- <span id="creliteratorheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="creliteratorheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrelIteratorHeader`

- <span id="creliteratorheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="creliteratorheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:851-862`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L851-L862)*

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

##### `impl Any for CrelIteratorState`

- <span id="creliteratorstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrelIteratorState`

- <span id="creliteratorstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrelIteratorState`

- <span id="creliteratorstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrelIteratorState`

- <span id="creliteratorstate-clone"></span>`fn clone(&self) -> CrelIteratorState` — [`CrelIteratorState`](#creliteratorstate)

##### `impl CloneToUninit for CrelIteratorState`

- <span id="creliteratorstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrelIteratorState`

- <span id="creliteratorstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrelIteratorState`

- <span id="creliteratorstate-default"></span>`fn default() -> CrelIteratorState` — [`CrelIteratorState`](#creliteratorstate)

##### `impl<T> From for CrelIteratorState`

- <span id="creliteratorstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CrelIteratorState`

- <span id="creliteratorstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CrelIteratorState`

- <span id="creliteratorstate-toowned-type-owned"></span>`type Owned = T`

- <span id="creliteratorstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="creliteratorstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CrelIteratorState`

- <span id="creliteratorstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="creliteratorstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrelIteratorState`

- <span id="creliteratorstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="creliteratorstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CrelIterator<'data>`

```rust
struct CrelIterator<'data> {
    data: crate::read::Bytes<'data>,
    header: CrelIteratorHeader,
    state: CrelIteratorState,
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:866-873`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L866-L873)*

Compact relocation iterator.

#### Fields

- **`data`**: `crate::read::Bytes<'data>`

  Input stream reader.

- **`header`**: `CrelIteratorHeader`

  Parsed header information.

- **`state`**: `CrelIteratorState`

  State of the iterator.

#### Implementations

- <span id="creliterator-new"></span>`fn new(data: &'data [u8]) -> Result<Self, Error>` — [`Error`](../../../index.md#error)

  Create a new CREL relocation iterator.

- <span id="creliterator-is-rela"></span>`fn is_rela(&self) -> bool`

  True if the encoded relocations have addend.

- <span id="creliterator-len"></span>`fn len(&self) -> usize`

  Return the number of encoded relocations.

- <span id="creliterator-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if there are no more relocations to parse.

- <span id="creliterator-parse"></span>`fn parse(&mut self) -> read::Result<Crel>` — [`Result`](../../../index.md#result), [`Crel`](../index.md#crel)

#### Trait Implementations

##### `impl Any for CrelIterator<'data>`

- <span id="creliterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrelIterator<'data>`

- <span id="creliterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrelIterator<'data>`

- <span id="creliterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CrelIterator<'data>`

- <span id="creliterator-clone"></span>`fn clone(&self) -> CrelIterator<'data>` — [`CrelIterator`](../index.md#creliterator)

##### `impl CloneToUninit for CrelIterator<'data>`

- <span id="creliterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CrelIterator<'data>`

- <span id="creliterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CrelIterator<'data>`

- <span id="creliterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CrelIterator<'data>`

- <span id="creliterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CrelIterator<'data>`

- <span id="creliterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="creliterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="creliterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CrelIterator<'data>`

- <span id="creliterator-iterator-type-item"></span>`type Item = Result<Crel, Error>`

- <span id="creliterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="creliterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for CrelIterator<'data>`

- <span id="creliterator-toowned-type-owned"></span>`type Owned = T`

- <span id="creliterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="creliterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CrelIterator<'data>`

- <span id="creliterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="creliterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrelIterator<'data>`

- <span id="creliterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="creliterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ElfRelocationIterator<'data, Elf: FileHeader>`

```rust
enum ElfRelocationIterator<'data, Elf: FileHeader> {
    Rel(slice::Iter<'data, <Elf as >::Rel>, <Elf as >::Endian),
    Rela(slice::Iter<'data, <Elf as >::Rela>, <Elf as >::Endian, bool),
    Crel(CrelIterator<'data>),
}
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:83-87`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L83-L87)*

#### Implementations

- <span id="elfrelocationiterator-is-rel"></span>`fn is_rel(&self) -> bool`

#### Trait Implementations

##### `impl Any for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-iterator-type-item"></span>`type Item = Crel`

- <span id="elfrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfRelocationIterator<'data, Elf>`

- <span id="elfrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Rel`

```rust
trait Rel: Debug + Pod + Clone { ... }
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:514-535`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L514-L535)*

A trait for generic access to [`elf::Rel32`](../../../elf/index.md) and [`elf::Rel64`](../../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

#### Required Methods

- `fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_sym(&self, endian: <Self as >::Endian) -> u32`

- `fn r_type(&self, endian: <Self as >::Endian) -> u32`

#### Provided Methods

- `fn symbol(&self, endian: <Self as >::Endian) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

#### Implementors

- [`Rel32`](../../../elf/index.md#rel32)
- [`Rel64`](../../../elf/index.md#rel64)

### `Rela`

```rust
trait Rela: Debug + Pod + Clone { ... }
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:591-613`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L591-L613)*

A trait for generic access to [`elf::Rela32`](../../../elf/index.md) and [`elf::Rela64`](../../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Sword: 1`

- `type Endian: 1`

#### Required Methods

- `fn r_offset(&self, endian: <Self as >::Endian) -> <Self as >::Word`

- `fn r_info(&self, endian: <Self as >::Endian, is_mips64el: bool) -> <Self as >::Word`

- `fn r_addend(&self, endian: <Self as >::Endian) -> <Self as >::Sword`

- `fn r_sym(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

- `fn r_type(&self, endian: <Self as >::Endian, is_mips64el: bool) -> u32`

#### Provided Methods

- `fn symbol(&self, endian: <Self as >::Endian, is_mips64el: bool) -> Option<SymbolIndex>`

  Get the symbol index referenced by the relocation.

#### Implementors

- [`Rela32`](../../../elf/index.md#rela32)
- [`Rela64`](../../../elf/index.md#rela64)

### `Relr`

```rust
trait Relr: Debug + Pod + Clone { ... }
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:727-746`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L727-L746)*

A trait for generic access to [`elf::Relr32`](../../../elf/index.md) and [`elf::Relr64`](../../../elf/index.md).

#### Associated Types

- `type Word: 1`

- `type Endian: 1`

#### Associated Constants

- `const COUNT: u8`

#### Required Methods

- `fn get(&self, endian: <Self as >::Endian) -> <Self as >::Word`

  Get the relocation entry.

- `fn next(offset: &mut <Self as >::Word, bits: &mut <Self as >::Word) -> Option<<Self as >::Word>`

  Return the offset corresponding to the next bit in the bit mask.

#### Implementors

- [`Relr32`](../../../elf/index.md#relr32)
- [`Relr64`](../../../elf/index.md#relr64)

## Functions

### `parse_relocation`

```rust
fn parse_relocation<Elf: FileHeader>(header: &Elf, endian: <Elf as >::Endian, reloc: Crel, implicit_addend: bool) -> crate::read::Relocation
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:278-510`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L278-L510)*

## Type Aliases

### `ElfDynamicRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator32<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:116-117`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L116-L117)*

An iterator for the dynamic relocations in an [`ElfFile32`](super::ElfFile32).

### `ElfDynamicRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfDynamicRelocationIterator64<'data, 'file, Endian, R> = ElfDynamicRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:119-120`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L119-L120)*

An iterator for the dynamic relocations in an [`ElfFile64`](super::ElfFile64).

### `ElfSectionRelocationIterator32<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator32<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:200-201`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L200-L201)*

An iterator for the relocations for an [`ElfSection32`](super::ElfSection32).

### `ElfSectionRelocationIterator64<'data, 'file, Endian, R>`

```rust
type ElfSectionRelocationIterator64<'data, 'file, Endian, R> = ElfSectionRelocationIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/relocation.rs:203-204`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/relocation.rs#L203-L204)*

An iterator for the relocations for an [`ElfSection64`](super::ElfSection64).

