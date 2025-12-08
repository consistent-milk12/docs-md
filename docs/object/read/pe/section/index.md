*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [section](index.md)*

---

# Module `section`

## Structs

### `PeSegmentIterator<'data, 'file, Pe, R>`

```rust
struct PeSegmentIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    iter: slice::Iter<'data, pe::ImageSectionHeader>,
}
```

An iterator for the loadable sections in a [`PeFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegmentIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeSegmentIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSegmentIterator<'data, 'file, Pe, R>`

- `type Item = PeSegment<'data, 'file, Pe, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `PeSegment<'data, 'file, Pe, R>`

```rust
struct PeSegment<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    section: &'data pe::ImageSectionHeader,
}
```

A loadable section in a [`PeFile`](../index.md).

Most functionality is provided by the [`ObjectSegment`](../../index.md) trait implementation.

#### Implementations

- `fn pe_file(self: &Self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](../index.md)

- `fn pe_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSegment<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSegment for PeSegment<'data, 'file, Pe, R>`

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> (u64, u64)`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn flags(self: &Self) -> SegmentFlags` — [`SegmentFlags`](../../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeSegment<'data, 'file, Pe, R>`

### `PeSectionIterator<'data, 'file, Pe, R>`

```rust
struct PeSectionIterator<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    iter: iter::Enumerate<slice::Iter<'data, pe::ImageSectionHeader>>,
}
```

An iterator for the sections in a [`PeFile`](../index.md).

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSectionIterator<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeSectionIterator<'data, 'file, Pe, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Pe, R> Iterator for PeSectionIterator<'data, 'file, Pe, R>`

- `type Item = PeSection<'data, 'file, Pe, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `PeSection<'data, 'file, Pe, R>`

```rust
struct PeSection<'data, 'file, Pe, R>
where
    Pe: ImageNtHeaders,
    R: ReadRef<'data> {
    file: &'file super::PeFile<'data, Pe, R>,
    index: crate::read::SectionIndex,
    section: &'data pe::ImageSectionHeader,
}
```

A section in a [`PeFile`](../index.md).

Most functionality is provided by the [`ObjectSection`](../../index.md) trait implementation.

#### Implementations

- `fn pe_file(self: &Self) -> &'file PeFile<'data, Pe, R>` — [`PeFile`](../index.md)

- `fn pe_section(self: &Self) -> &'data pe::ImageSectionHeader` — [`ImageSectionHeader`](../../../pe/index.md)

#### Trait Implementations

##### `impl<'data, 'file, Pe, R> Debug for PeSection<'data, 'file, Pe, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Pe, R> ObjectSection for PeSection<'data, 'file, Pe, R>`

- `type RelocationIterator = PeRelocationIterator<'data, 'file, R>`

- `fn index(self: &Self) -> SectionIndex` — [`SectionIndex`](../../../index.md)

- `fn address(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn align(self: &Self) -> u64`

- `fn file_range(self: &Self) -> Option<(u64, u64)>`

- `fn data(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>` — [`Result`](../../../index.md)

- `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>` — [`Result`](../../../index.md), [`CompressedFileRange`](../../../index.md)

- `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>` — [`Result`](../../../index.md), [`CompressedData`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>` — [`Result`](../../../index.md)

- `fn segment_name(self: &Self) -> Result<Option<&str>>` — [`Result`](../../../index.md)

- `fn kind(self: &Self) -> SectionKind` — [`SectionKind`](../../../index.md)

- `fn relocations(self: &Self) -> PeRelocationIterator<'data, 'file, R>` — [`PeRelocationIterator`](../index.md)

- `fn relocation_map(self: &Self) -> read::Result<RelocationMap>` — [`Result`](../../../index.md), [`RelocationMap`](../../../index.md)

- `fn flags(self: &Self) -> SectionFlags` — [`SectionFlags`](../../../index.md)

##### `impl<'data, 'file, Pe, R> Sealed for PeSection<'data, 'file, Pe, R>`

### `PeRelocationIterator<'data, 'file, R>`

```rust
struct PeRelocationIterator<'data, 'file, R>(core::marker::PhantomData<(&'data (), &'file (), R)>);
```

An iterator for the relocations in an [`PeSection`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, R: $crate::fmt::Debug> Debug for PeRelocationIterator<'data, 'file, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PeRelocationIterator<'data, 'file, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R> Iterator for PeRelocationIterator<'data, 'file, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Type Aliases

### `PeSegmentIterator32<'data, 'file, R>`

```rust
type PeSegmentIterator32<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the loadable sections in a [`PeFile32`](super::PeFile32).

### `PeSegmentIterator64<'data, 'file, R>`

```rust
type PeSegmentIterator64<'data, 'file, R> = PeSegmentIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the loadable sections in a [`PeFile64`](super::PeFile64).

### `PeSegment32<'data, 'file, R>`

```rust
type PeSegment32<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders32, R>;
```

A loadable section in a [`PeFile32`](super::PeFile32).

### `PeSegment64<'data, 'file, R>`

```rust
type PeSegment64<'data, 'file, R> = PeSegment<'data, 'file, pe::ImageNtHeaders64, R>;
```

A loadable section in a [`PeFile64`](super::PeFile64).

### `PeSectionIterator32<'data, 'file, R>`

```rust
type PeSectionIterator32<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders32, R>;
```

An iterator for the sections in a [`PeFile32`](super::PeFile32).

### `PeSectionIterator64<'data, 'file, R>`

```rust
type PeSectionIterator64<'data, 'file, R> = PeSectionIterator<'data, 'file, pe::ImageNtHeaders64, R>;
```

An iterator for the sections in a [`PeFile64`](super::PeFile64).

### `PeSection32<'data, 'file, R>`

```rust
type PeSection32<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders32, R>;
```

A section in a [`PeFile32`](super::PeFile32).

### `PeSection64<'data, 'file, R>`

```rust
type PeSection64<'data, 'file, R> = PeSection<'data, 'file, pe::ImageNtHeaders64, R>;
```

A section in a [`PeFile64`](super::PeFile64).

