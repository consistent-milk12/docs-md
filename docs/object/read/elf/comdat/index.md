*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [comdat](index.md)*

---

# Module `comdat`

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

- `fn new(file: &'file ElfFile<'data, Elf, R>) -> ElfComdatIterator<'data, 'file, Elf, R>` — [`ElfFile`](../index.md), [`ElfComdatIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdatIterator<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfComdatIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatIterator<'data, 'file, Elf, R>`

- `type Item = ElfComdat<'data, 'file, Elf, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn parse(file: &'file ElfFile<'data, Elf, R>, section: &'data <Elf as >::SectionHeader) -> Option<ElfComdat<'data, 'file, Elf, R>>` — [`ElfFile`](../index.md), [`FileHeader`](../index.md), [`ElfComdat`](../index.md)

- `fn elf_file(self: &Self) -> &'file ElfFile<'data, Elf, R>` — [`ElfFile`](../index.md)

- `fn elf_section_header(self: &Self) -> &'data <Elf as >::SectionHeader` — [`FileHeader`](../index.md)

#### Trait Implementations

##### `impl<'data, 'file, Elf, R> Debug for ElfComdat<'data, 'file, Elf, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Elf, R> ObjectComdat for ElfComdat<'data, 'file, Elf, R>`

- `type SectionIterator = ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- `fn name_bytes(self: &Self) -> read::Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> read::Result<&'data str>` — [`Result`](../../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md)

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Elf, R> Iterator for ElfComdatSectionIterator<'data, 'file, Elf, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

