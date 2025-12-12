*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [comdat](index.md)*

---

# Module `comdat`

XCOFF doesn't support the COMDAT section.

## Contents

- [Structs](#structs)
  - [`XcoffComdatIterator`](#xcoffcomdatiterator)
  - [`XcoffComdat`](#xcoffcomdat)
  - [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator)
- [Type Aliases](#type-aliases)
  - [`XcoffComdatIterator32`](#xcoffcomdatiterator32)
  - [`XcoffComdatIterator64`](#xcoffcomdatiterator64)
  - [`XcoffComdat32`](#xcoffcomdat32)
  - [`XcoffComdat64`](#xcoffcomdat64)
  - [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32)
  - [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffComdatIterator`](#xcoffcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`XcoffFile`]. |
| [`XcoffComdat`](#xcoffcomdat) | struct | A COMDAT section group in a [`XcoffFile`]. |
| [`XcoffComdatSectionIterator`](#xcoffcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`XcoffFile`]. |
| [`XcoffComdatIterator32`](#xcoffcomdatiterator32) | type | An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatIterator64`](#xcoffcomdatiterator64) | type | An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdat32`](#xcoffcomdat32) | type | A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdat64`](#xcoffcomdat64) | type | A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |
| [`XcoffComdatSectionIterator32`](#xcoffcomdatsectioniterator32) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32). |
| [`XcoffComdatSectionIterator64`](#xcoffcomdatsectioniterator64) | type | An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64). |

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

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:21-28`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L21-L28)*

An iterator for the COMDAT section groups in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-type-item"></span>`type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:55-62`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L55-L62)*

A COMDAT section group in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md#comdatkind)

- <span id="xcoffcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="xcoffcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="xcoffcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md#objectcomdat)

##### `impl<Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:115-122`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L115-L122)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl<Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatsectioniterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="xcoffcomdatsectioniterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:11-12`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L11-L12)*

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:14-15`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L14-L15)*

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:44-45`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L44-L45)*

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:48-49`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L48-L49)*

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:105-106`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L105-L106)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:108-109`](../../../../../.source_1765521767/object-0.37.3/src/read/xcoff/comdat.rs#L108-L109)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

