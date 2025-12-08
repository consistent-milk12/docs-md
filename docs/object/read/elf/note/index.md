*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [note](index.md)*

---

# Module `note`

## Structs

### `NoteIterator<'data, Elf>`

```rust
struct NoteIterator<'data, Elf>
where
    Elf: FileHeader {
    endian: <Elf as >::Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

#### Implementations

- `fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- `fn next(self: &mut Self) -> read::Result<Option<Note<'data, Elf>>>` — [`Result`](../../../index.md), [`Note`](../index.md)

- `fn parse(self: &mut Self) -> read::Result<Note<'data, Elf>>` — [`Result`](../../../index.md), [`Note`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf> Debug for NoteIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for NoteIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for NoteIterator<'data, Elf>`

- `type Item = Result<Note<'data, Elf>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Note<'data, Elf>`

```rust
struct Note<'data, Elf>
where
    Elf: FileHeader {
    header: &'data <Elf as >::NoteHeader,
    name: &'data [u8],
    desc: &'data [u8],
}
```

A parsed [`NoteHeader`](../index.md).

#### Implementations

- `fn n_type(self: &Self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md)

- `fn n_namesz(self: &Self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md)

- `fn n_descsz(self: &Self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md)

- `fn name_bytes(self: &Self) -> &'data [u8]`

- `fn name(self: &Self) -> &'data [u8]`

- `fn desc(self: &Self) -> &'data [u8]`

- `fn gnu_properties(self: &Self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` — [`FileHeader`](../index.md), [`GnuPropertyIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf> Debug for Note<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `GnuPropertyIterator<'data, Endian: endian::Endian>`

```rust
struct GnuPropertyIterator<'data, Endian: endian::Endian> {
    endian: Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md) note.

Returned by `Note::gnu_properties`.

#### Implementations

- `fn next(self: &mut Self) -> read::Result<Option<GnuProperty<'data>>>` — [`Result`](../../../index.md), [`GnuProperty`](../index.md)

- `fn parse(self: &mut Self) -> read::Result<GnuProperty<'data>>` — [`Result`](../../../index.md), [`GnuProperty`](../index.md)

#### Trait Implementations

##### `impl<'data, Endian: $crate::fmt::Debug + endian::Endian> Debug for GnuPropertyIterator<'data, Endian>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for GnuPropertyIterator<'data, Endian>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Endian: endian::Endian> Iterator for GnuPropertyIterator<'data, Endian>`

- `type Item = Result<GnuProperty<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `GnuProperty<'data>`

```rust
struct GnuProperty<'data> {
    pr_type: u32,
    pr_data: &'data [u8],
}
```

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md) note.

#### Implementations

- `fn pr_type(self: &Self) -> u32`

- `fn pr_data(self: &Self) -> &'data [u8]`

- `fn data_u32<E: endian::Endian>(self: &Self, endian: E) -> read::Result<u32>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data> Debug for GnuProperty<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `NoteHeader`

```rust
trait NoteHeader: Debug + Pod { ... }
```

A trait for generic access to [`elf::NoteHeader32`](../../../elf/index.md) and [`elf::NoteHeader64`](../../../elf/index.md).

#### Required Methods

- `type Endian: 1`

- `fn n_namesz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(self: &Self, endian: <Self as >::Endian) -> u32`

- `fn n_type(self: &Self, endian: <Self as >::Endian) -> u32`

