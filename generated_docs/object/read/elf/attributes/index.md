*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [attributes](index.md)*

---

# Module `attributes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AttributesSection`](#attributessection) | struct | An ELF attributes section. |
| [`AttributesSubsectionIterator`](#attributessubsectioniterator) | struct | An iterator for the subsections in an [`AttributesSection`]. |
| [`AttributesSubsection`](#attributessubsection) | struct | A subsection in an [`AttributesSection`]. |
| [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator) | struct | An iterator for the sub-subsections in an [`AttributesSubsection`]. |
| [`AttributesSubsubsection`](#attributessubsubsection) | struct | A sub-subsection in an [`AttributesSubsection`]. |
| [`AttributeIndexIterator`](#attributeindexiterator) | struct | An iterator over the indices in an [`AttributesSubsubsection`]. |
| [`AttributeReader`](#attributereader) | struct | A parser for the attributes in an [`AttributesSubsubsection`]. |

## Structs

### `AttributesSection<'data, Elf: FileHeader>`

```rust
struct AttributesSection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    version: u8,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:18-22`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L18-L22)*

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`](../index.md).

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

#### Implementations

- <span id="attributessection-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Result`](../../../index.md#result)

- <span id="attributessection-version"></span>`fn version(&self) -> u8`

- <span id="attributessection-subsections"></span>`fn subsections(&self) -> Result<AttributesSubsectionIterator<'data, Elf>>` — [`Result`](../../../index.md#result), [`AttributesSubsectionIterator`](../index.md#attributessubsectioniterator)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSection<'data, Elf>`

- <span id="attributessection-clone"></span>`fn clone(&self) -> AttributesSection<'data, Elf>` — [`AttributesSection`](../index.md#attributessection)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSection<'data, Elf>`

- <span id="attributessection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributesSubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:61-64`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L61-L64)*

An iterator for the subsections in an [`AttributesSection`](../index.md).

#### Implementations

- <span id="attributessubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsection<'data, Elf>>>` — [`Result`](../../../index.md#result), [`AttributesSubsection`](../index.md#attributessubsection)

- <span id="attributessubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsection<'data, Elf>>` — [`Result`](../../../index.md#result), [`AttributesSubsection`](../index.md#attributessubsection)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsectionIterator<'data, Elf>` — [`AttributesSubsectionIterator`](../index.md#attributessubsectioniterator)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for AttributesSubsectionIterator<'data, Elf>`

- <span id="attributessubsectioniterator-iterator-type-item"></span>`type Item = Result<AttributesSubsection<'data, Elf>, Error>`

- <span id="attributessubsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributesSubsection<'data, Elf: FileHeader>`

```rust
struct AttributesSubsection<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    length: u32,
    vendor: &'data [u8],
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:124-129`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L124-L129)*

A subsection in an [`AttributesSection`](../index.md).

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`](../index.md).

#### Implementations

- <span id="attributessubsection-length"></span>`fn length(&self) -> u32`

- <span id="attributessubsection-vendor"></span>`fn vendor(&self) -> &'data [u8]`

- <span id="attributessubsection-subsubsections"></span>`fn subsubsections(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](../index.md#attributessubsubsectioniterator)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-clone"></span>`fn clone(&self) -> AttributesSubsection<'data, Elf>` — [`AttributesSubsection`](../index.md#attributessubsection)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsection<'data, Elf>`

- <span id="attributessubsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributesSubsubsectionIterator<'data, Elf: FileHeader>`

```rust
struct AttributesSubsubsectionIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:153-156`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L153-L156)*

An iterator for the sub-subsections in an [`AttributesSubsection`](../index.md).

#### Implementations

- <span id="attributessubsubsectioniterator-next"></span>`fn next(&mut self) -> Result<Option<AttributesSubsubsection<'data>>>` — [`Result`](../../../index.md#result), [`AttributesSubsubsection`](../index.md#attributessubsubsection)

- <span id="attributessubsubsectioniterator-parse"></span>`fn parse(&mut self) -> Result<AttributesSubsubsection<'data>>` — [`Result`](../../../index.md#result), [`AttributesSubsubsection`](../index.md#attributessubsubsection)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-clone"></span>`fn clone(&self) -> AttributesSubsubsectionIterator<'data, Elf>` — [`AttributesSubsubsectionIterator`](../index.md#attributessubsubsectioniterator)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributessubsubsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributessubsubsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for AttributesSubsubsectionIterator<'data, Elf>`

- <span id="attributessubsubsectioniterator-iterator-type-item"></span>`type Item = Result<AttributesSubsubsection<'data>, Error>`

- <span id="attributessubsubsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributesSubsubsection<'data>`

```rust
struct AttributesSubsubsection<'data> {
    tag: u8,
    length: u32,
    indices: crate::read::Bytes<'data>,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:229-234`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L229-L234)*

A sub-subsection in an [`AttributesSubsection`](../index.md).

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

#### Implementations

- <span id="attributessubsubsection-tag"></span>`fn tag(&self) -> u8`

- <span id="attributessubsubsection-length"></span>`fn length(&self) -> u32`

- <span id="attributessubsubsection-indices-data"></span>`fn indices_data(&self) -> &'data [u8]`

- <span id="attributessubsubsection-indices"></span>`fn indices(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](../index.md#attributeindexiterator)

- <span id="attributessubsubsection-attributes-data"></span>`fn attributes_data(&self) -> &'data [u8]`

- <span id="attributessubsubsection-attributes"></span>`fn attributes(&self) -> AttributeReader<'data>` — [`AttributeReader`](../index.md#attributereader)

#### Trait Implementations

##### `impl Clone for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-clone"></span>`fn clone(&self) -> AttributesSubsubsection<'data>` — [`AttributesSubsubsection`](../index.md#attributessubsubsection)

##### `impl Debug for AttributesSubsubsection<'data>`

- <span id="attributessubsubsection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AttributeIndexIterator<'data>`

```rust
struct AttributeIndexIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:274-276`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L274-L276)*

An iterator over the indices in an [`AttributesSubsubsection`](../index.md).

#### Implementations

- <span id="attributeindexiterator-next"></span>`fn next(&mut self) -> Result<Option<u32>>` — [`Result`](../../../index.md#result)

- <span id="attributeindexiterator-parse"></span>`fn parse(&mut self) -> Result<u32>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Clone for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-clone"></span>`fn clone(&self) -> AttributeIndexIterator<'data>` — [`AttributeIndexIterator`](../index.md#attributeindexiterator)

##### `impl Debug for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="attributeindexiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="attributeindexiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for AttributeIndexIterator<'data>`

- <span id="attributeindexiterator-iterator-type-item"></span>`type Item = Result<u32, Error>`

- <span id="attributeindexiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AttributeReader<'data>`

```rust
struct AttributeReader<'data> {
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/attributes.rs:315-317`](../../../../../.source_1765521767/object-0.37.3/src/read/elf/attributes.rs#L315-L317)*

A parser for the attributes in an [`AttributesSubsubsection`](../index.md).

The parser relies on the caller to know the format of the data for each attribute tag.

#### Implementations

- <span id="attributereader-read-tag"></span>`fn read_tag(&mut self) -> Result<Option<u64>>` — [`Result`](../../../index.md#result)

- <span id="attributereader-read-integer"></span>`fn read_integer(&mut self) -> Result<u64>` — [`Result`](../../../index.md#result)

- <span id="attributereader-read-string"></span>`fn read_string(&mut self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

#### Trait Implementations

##### `impl Clone for AttributeReader<'data>`

- <span id="attributereader-clone"></span>`fn clone(&self) -> AttributeReader<'data>` — [`AttributeReader`](../index.md#attributereader)

##### `impl Debug for AttributeReader<'data>`

- <span id="attributereader-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

