*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [comdat](index.md)*

---

# Module `comdat`

XCOFF doesn't support the COMDAT section.

## Structs

### `XcoffComdatIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the COMDAT section groups in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- `type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

A COMDAT section group in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, 'file, Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- `type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `fn kind(self: &Self) -> ComdatKind` — [`ComdatKind`](../../../index.md)

- `fn symbol(self: &Self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md)

- `fn name_bytes(self: &Self) -> Result<&'data [u8]>` — [`Result`](../../../index.md)

- `fn name(self: &Self) -> Result<&'data str>` — [`Result`](../../../index.md)

- `fn sections(self: &Self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md)

##### `impl<'data, 'file, Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- `type Item = SectionIndex`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Type Aliases

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

