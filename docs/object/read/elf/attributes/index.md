*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [attributes](index.md)*

---

# Module `attributes`

## Structs

### `AttributesSection<'data, Elf: FileHeader>`

```rust
struct AttributesSection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    version: u8,
    data: crate::read::Bytes<'data>,
}
```

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`](../index.md).

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

#### Implementations

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md), [`Result`](../../../index.md)

- `fn version(self: &Self) -> u8`

- `fn subsections(self: &Self) -> Result<AttributesSubsectionIterator<'data, Elf>>` — [`Result`](../../../index.md), [`AttributesSubsectionIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSection<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSection<'data, Elf>` — [`AttributesSection`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSection<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AttributesSubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the subsections in an [`AttributesSection`](../index.md).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<AttributesSubsection<'data, Elf>>>` — [`Result`](../../../index.md), [`AttributesSubsection`](../index.md)

- `fn parse(self: &mut Self) -> Result<AttributesSubsection<'data, Elf>>` — [`Result`](../../../index.md), [`AttributesSubsection`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSubsectionIterator<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSubsectionIterator<'data, Elf>` — [`AttributesSubsectionIterator`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSubsectionIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for AttributesSubsectionIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for AttributesSubsectionIterator<'data, Elf>`

- `type Item = Result<AttributesSubsection<'data, Elf>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributesSubsection<'data, Elf: FileHeader>`

```rust
struct AttributesSubsection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    length: u32,
    vendor: &'data [u8],
    data: crate::read::Bytes<'data>,
}
```

A subsection in an [`AttributesSection`](../index.md).

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`](../index.md).

#### Implementations

- `fn length(self: &Self) -> u32`

- `fn vendor(self: &Self) -> &'data [u8]`

- `fn subsubsections(self: &Self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSubsection<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSubsection<'data, Elf>` — [`AttributesSubsection`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSubsection<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AttributesSubsubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the sub-subsections in an [`AttributesSubsection`](../index.md).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<AttributesSubsubsection<'data>>>` — [`Result`](../../../index.md), [`AttributesSubsubsection`](../index.md)

- `fn parse(self: &mut Self) -> Result<AttributesSubsubsection<'data>>` — [`Result`](../../../index.md), [`AttributesSubsubsection`](../index.md)

#### Trait Implementations

##### `impl<'data, Elf: $crate::clone::Clone + FileHeader> Clone for AttributesSubsubsectionIterator<'data, Elf>`

- `fn clone(self: &Self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](../index.md)

##### `impl<'data, Elf: $crate::fmt::Debug + FileHeader> Debug for AttributesSubsubsectionIterator<'data, Elf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for AttributesSubsubsectionIterator<'data, Elf>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, Elf: FileHeader> Iterator for AttributesSubsubsectionIterator<'data, Elf>`

- `type Item = Result<AttributesSubsubsection<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    tag: u8,
    length: u32,
    indices: crate::read::Bytes<'data>,
    data: crate::read::Bytes<'data>,
}
```

A sub-subsection in an [`AttributesSubsection`](../index.md).

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

#### Implementations

- `fn tag(self: &Self) -> u8`

- `fn length(self: &Self) -> u32`

- `fn indices_data(self: &Self) -> &'data [u8]`

- `fn indices(self: &Self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](../index.md)

- `fn attributes_data(self: &Self) -> &'data [u8]`

- `fn attributes(self: &Self) -> AttributeReader<'data>` — [`AttributeReader`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributesSubsubsection<'data>`

- `fn clone(self: &Self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](../index.md)

##### `impl<'data> Debug for AttributesSubsubsection<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AttributeIndexIterator<'data>`

```rust
struct AttributeIndexIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

An iterator over the indices in an [`AttributesSubsubsection`](../index.md).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<u32>>` — [`Result`](../../../index.md)

- `fn parse(self: &mut Self) -> Result<u32>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributeIndexIterator<'data>`

- `fn clone(self: &Self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](../index.md)

##### `impl<'data> Debug for AttributeIndexIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for AttributeIndexIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for AttributeIndexIterator<'data>`

- `type Item = Result<u32, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `AttributeReader<'data>`

```rust
struct AttributeReader<'data> {
    data: crate::read::Bytes<'data>,
}
```

A parser for the attributes in an [`AttributesSubsubsection`](../index.md).

The parser relies on the caller to know the format of the data for each attribute tag.

#### Implementations

- `fn read_tag(self: &mut Self) -> Result<Option<u64>>` — [`Result`](../../../index.md)

- `fn read_integer(self: &mut Self) -> Result<u64>` — [`Result`](../../../index.md)

- `fn read_string(self: &mut Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

#### Trait Implementations

##### `impl<'data> Clone for AttributeReader<'data>`

- `fn clone(self: &Self) -> AttributeReader<'data>` — [`AttributeReader`](../index.md)

##### `impl<'data> Debug for AttributeReader<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

