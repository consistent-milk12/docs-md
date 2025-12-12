*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [note](index.md)*

---

# Module `note`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoteIterator`](#noteiterator) | struct | An iterator over the notes in an ELF section or segment. |
| [`Note`](#note) | struct | A parsed [`NoteHeader`]. |
| [`GnuPropertyIterator`](#gnupropertyiterator) | struct | An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note. |
| [`GnuProperty`](#gnuproperty) | struct | A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note. |
| [`NoteHeader`](#noteheader) | trait | A trait for generic access to [`elf::NoteHeader32`] and [`elf::NoteHeader64`]. |

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

*Defined in [`object-0.37.3/src/read/elf/note.rs:17-24`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L17-L24)*

An iterator over the notes in an ELF section or segment.

Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)
and [`SectionHeader::notes`](super::SectionHeader::notes).

#### Implementations

- <span id="noteiterator-new"></span>`fn new(endian: <Elf as >::Endian, align: <Elf as >::Word, data: &'data [u8]) -> read::Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

- <span id="noteiterator-next"></span>`fn next(&mut self) -> read::Result<Option<Note<'data, Elf>>>` — [`Result`](../../../index.md#result), [`Note`](../index.md#note)

- <span id="noteiterator-parse"></span>`fn parse(&mut self) -> read::Result<Note<'data, Elf>>` — [`Result`](../../../index.md#result), [`Note`](../index.md#note)

#### Trait Implementations

##### `impl<Elf> Debug for NoteIterator<'data, Elf>`

- <span id="noteiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="noteiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="noteiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for NoteIterator<'data, Elf>`

- <span id="noteiterator-iterator-type-item"></span>`type Item = Result<Note<'data, Elf>, Error>`

- <span id="noteiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`object-0.37.3/src/read/elf/note.rs:109-116`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L109-L116)*

A parsed [`NoteHeader`](../index.md).

#### Implementations

- <span id="note-n-type"></span>`fn n_type(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md#fileheader)

- <span id="note-n-namesz"></span>`fn n_namesz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md#fileheader)

- <span id="note-n-descsz"></span>`fn n_descsz(&self, endian: <Elf as >::Endian) -> u32` — [`FileHeader`](../index.md#fileheader)

- <span id="note-name-bytes"></span>`fn name_bytes(&self) -> &'data [u8]`

- <span id="note-name"></span>`fn name(&self) -> &'data [u8]`

- <span id="note-desc"></span>`fn desc(&self) -> &'data [u8]`

- <span id="note-gnu-properties"></span>`fn gnu_properties(&self, endian: <Elf as >::Endian) -> Option<GnuPropertyIterator<'data, <Elf as >::Endian>>` — [`FileHeader`](../index.md#fileheader), [`GnuPropertyIterator`](../index.md#gnupropertyiterator)

#### Trait Implementations

##### `impl<Elf> Debug for Note<'data, Elf>`

- <span id="note-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `GnuPropertyIterator<'data, Endian: endian::Endian>`

```rust
struct GnuPropertyIterator<'data, Endian: endian::Endian> {
    endian: Endian,
    align: usize,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/note.rs:235-239`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L235-L239)*

An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md) note.

Returned by `Note::gnu_properties`.

#### Implementations

- <span id="gnupropertyiterator-next"></span>`fn next(&mut self) -> read::Result<Option<GnuProperty<'data>>>` — [`Result`](../../../index.md#result), [`GnuProperty`](../index.md#gnuproperty)

- <span id="gnupropertyiterator-parse"></span>`fn parse(&mut self) -> read::Result<GnuProperty<'data>>` — [`Result`](../../../index.md#result), [`GnuProperty`](../index.md#gnuproperty)

#### Trait Implementations

##### `impl<Endian: fmt::Debug + endian::Endian> Debug for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="gnupropertyiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="gnupropertyiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Endian: endian::Endian> Iterator for GnuPropertyIterator<'data, Endian>`

- <span id="gnupropertyiterator-iterator-type-item"></span>`type Item = Result<GnuProperty<'data>, Error>`

- <span id="gnupropertyiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `GnuProperty<'data>`

```rust
struct GnuProperty<'data> {
    pr_type: u32,
    pr_data: &'data [u8],
}
```

*Defined in [`object-0.37.3/src/read/elf/note.rs:277-280`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L277-L280)*

A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`](../../../elf/index.md) note.

#### Implementations

- <span id="gnuproperty-pr-type"></span>`fn pr_type(&self) -> u32`

- <span id="gnuproperty-pr-data"></span>`fn pr_data(&self) -> &'data [u8]`

- <span id="gnuproperty-data-u32"></span>`fn data_u32<E: endian::Endian>(&self, endian: E) -> read::Result<u32>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Debug for GnuProperty<'data>`

- <span id="gnuproperty-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `NoteHeader`

```rust
trait NoteHeader: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/elf/note.rs:185-191`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/note.rs#L185-L191)*

A trait for generic access to [`elf::NoteHeader32`](../../../elf/index.md) and [`elf::NoteHeader64`](../../../elf/index.md).

#### Associated Types

- `type Endian: 1`

#### Required Methods

- `fn n_namesz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_descsz(&self, endian: <Self as >::Endian) -> u32`

- `fn n_type(&self, endian: <Self as >::Endian) -> u32`

#### Implementors

- [`NoteHeader32`](../../../elf/index.md#noteheader32)
- [`NoteHeader64`](../../../elf/index.md#noteheader64)

