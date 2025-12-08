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

An iterator for the COMDAT section groups in an [`ElfFile`](../index.md).

#### Implementations

- <span id="elfcomdatiterator-new"></span>`fn new(file: &'file ElfFile<'data, Elf, R>) -> ElfComdatIterator<'data, 'file, Elf, R>` — [`ElfFile`](../index.md), [`ElfComdatIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatiterator-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-item"></span>`type Item = ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

A COMDAT section group in an [`ElfFile`](../index.md).

Most functionality is provided by the [`ObjectComdat`](../../index.md) trait implementation.

#### Implementations

- <span id="elfcomdat-parse"></span>`fn parse(file: &'file ElfFile<'data, Elf, R>, section: &'data <Elf as >::SectionHeader) -> Option<ElfComdat<'data, 'file, Elf, R>>` — [`ElfFile`](../index.md), [`FileHeader`](../index.md), [`ElfComdat`](../index.md)

- <span id="elfcomdat-elf-file"></span>`fn elf_file(&self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md)

- <span id="elfcomdat-elf-section-header"></span>`fn elf_section_header(&self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectComdat for ElfComdat<'data, 'file, Elf, R>`

- <span id="elfcomdat-sectioniterator"></span>`type SectionIterator = ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md)

- <span id="elfcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- <span id="elfcomdat-name-bytes"></span>`fn name_bytes(&self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- <span id="elfcomdat-name"></span>`fn name(&self) -> read::Result<&'data str>` — [`Result`](../../../index.md)

- <span id="elfcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md)

##### `impl<'data, 'file, Elf, R> Sealed for ElfComdat<'data, 'file, Elf, R>`

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

An iterator for the sections in a COMDAT section group in an [`ElfFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="elfcomdatsectioniterator-intoiter"></span>`type IntoIter = I`

- <span id="elfcomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- <span id="elfcomdatsectioniterator-item"></span>`type Item = SectionIndex`

- <span id="elfcomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `ElfComdatIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator32<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the COMDAT section groups in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatIterator64<'data, 'file, Endian, R> = ElfComdatIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the COMDAT section groups in an [`ElfFile64`](super::ElfFile64).

### `ElfComdat32<'data, 'file, Endian, R>`

```rust
type ElfComdat32<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader32<Endian>, R>;
```

A COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdat64<'data, 'file, Endian, R>`

```rust
type ElfComdat64<'data, 'file, Endian, R> = ElfComdat<'data, 'file, elf::FileHeader64<Endian>, R>;
```

A COMDAT section group in an [`ElfFile64`](super::ElfFile64).

### `ElfComdatSectionIterator32<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator32<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader32<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in an [`ElfFile32`](super::ElfFile32).

### `ElfComdatSectionIterator64<'data, 'file, Endian, R>`

```rust
type ElfComdatSectionIterator64<'data, 'file, Endian, R> = ElfComdatSectionIterator<'data, 'file, elf::FileHeader64<Endian>, R>;
```

An iterator for the sections in a COMDAT section group in an [`ElfFile64`](super::ElfFile64).

