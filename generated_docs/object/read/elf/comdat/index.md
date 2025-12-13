*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [comdat](index.md)*

---

# Module `comdat`

## Contents

- [Structs](#structs)
  - [`ElfComdatIterator`](#elfcomdatiterator)
  - [`ElfComdat`](#elfcomdat)
  - [`ElfComdatSectionIterator`](#elfcomdatsectioniterator)
- [Type Aliases](#type-aliases)
  - [`ElfComdatIterator32`](#elfcomdatiterator32)
  - [`ElfComdatIterator64`](#elfcomdatiterator64)
  - [`ElfComdat32`](#elfcomdat32)
  - [`ElfComdat64`](#elfcomdat64)
  - [`ElfComdatSectionIterator32`](#elfcomdatsectioniterator32)
  - [`ElfComdatSectionIterator64`](#elfcomdatsectioniterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ElfComdatIterator`](#elfcomdatiterator) | struct | An iterator for the COMDAT section groups in an [`ElfFile`]. |
| [`ElfComdat`](#elfcomdat) | struct | A COMDAT section group in an [`ElfFile`]. |
| [`ElfComdatSectionIterator`](#elfcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in an [`ElfFile`]. |
| [`ElfComdatIterator32`](#elfcomdatiterator32) | type | An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32). |
| [`ElfComdatIterator64`](#elfcomdatiterator64) | type | An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64). |
| [`ElfComdat32`](#elfcomdat32) | type | A COMDAT section group in an [`ElfFile32`](super::ElfFile32). |
| [`ElfComdat64`](#elfcomdat64) | type | A COMDAT section group in an [`ElfFile64`](super::ElfFile64). |
| [`ElfComdatSectionIterator32`](#elfcomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32). |
| [`ElfComdatSectionIterator64`](#elfcomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64). |

## Structs

### `ElfComdatIterator<'data, 'file, Elf, R>`

```rust
struct ElfComdatIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    iter: iter::Enumerate<slice::Iter<'data, <Elf as >::SectionHeader>>,
}
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:19-26`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L19-L26)*

An iterator for the COMDAT section groups in an [`ElfFile`](../index.md).

#### Implementations

- <span id="elfcomdatiterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> ElfComdatIterator<'data, 'file, Elf, R>` — [`ElfFile`](../index.md#elffile), [`ElfComdatIterator`](../index.md#elfcomdatiterator)

#### Trait Implementations

##### `impl Any for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-iterator-type-item"></span>`type Item = ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfcomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfcomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ElfComdat<'data, 'file, Elf, R>`

```rust
struct ElfComdat<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    section: &'data <Elf as >::SectionHeader,
    sections: &'data [crate::endian::U32Bytes<<Elf as >::Endian>],
}
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:70-78`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L70-L78)*

A COMDAT section group in an [`ElfFile`](../index.md).

Most functionality is provided by the [`ObjectComdat`](../../index.md) trait implementation.

#### Implementations

- <span id="elfcomdat-parse"></span>`fn parse(file: &'file ElfFile<'data, Elf, R>, section: &'data <Elf as >::SectionHeader) -> Option<ElfComdat<'data, 'file, Elf, R>>` — [`ElfFile`](../index.md#elffile), [`FileHeader`](../index.md#fileheader), [`ElfComdat`](../index.md#elfcomdat)

- <span id="elfcomdat-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md#elffile)

  Get the ELF file containing this COMDAT section group.

- <span id="elfcomdat-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](../index.md#fileheader)

  Get the raw ELF section header for the COMDAT section group.

#### Trait Implementations

##### `impl Any for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Elf, R> ObjectComdat for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md#comdatkind)

- <span id="elfcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="elfcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="elfcomdat-objectcomdat-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="elfcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md#objectcomdat)

##### `impl<Elf, R> Sealed for ElfComdat<'data, 'file, Elf, R>`

##### `impl<U> TryFrom for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfcomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfcomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ElfComdatSectionIterator<'data, 'file, Elf, R>`

```rust
struct ElfComdatSectionIterator<'data, 'file, Elf, R>
where
    Elf: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::ElfFile<'data, Elf, R>,
    sections: slice::Iter<'data, crate::endian::U32Bytes<<Elf as >::Endian>>,
}
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:166-173`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L166-L173)*

An iterator for the sections in a COMDAT section group in an [`ElfFile`](../index.md).

#### Trait Implementations

##### `impl Any for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf, R> Debug for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf, R> Iterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="elfcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="elfcomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="elfcomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `ElfComdatIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator32<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:11-12`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L11-L12)*

An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator64<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:14-15`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L14-L15)*

An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).

### `ElfComdat32<'data, 'file, Endian, R>`

```rust
type ElfComdat32<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:60-61`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L60-L61)*

A COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdat64<'data, 'file, Endian, R>`

```rust
type ElfComdat64<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:63-64`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L63-L64)*

A COMDAT section group in an [`ElfFile64`](super::ElfFile64).

### `ElfComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator32<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:158-159`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L158-L159)*

An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator64<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/elf/comdat.rs:161-162`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/comdat.rs#L161-L162)*

An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).

