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

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:21-28`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L21-L28)*

An iterator for the COMDAT section groups in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-type-item"></span>`type Item = XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffcomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffComdatIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffcomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffComdat<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdat<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:55-62`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L55-L62)*

A COMDAT section group in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Xcoff, R> ObjectComdat for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md#comdatkind)

- <span id="xcoffcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="xcoffcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="xcoffcomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="xcoffcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md#objectcomdat)

##### `impl<Xcoff, R> Sealed for XcoffComdat<'data, 'file, Xcoff, R>`

##### `impl<U> TryFrom for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffcomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffComdat<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffcomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffComdatSectionIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:115-122`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L115-L122)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile`](../index.md).

This is a stub that doesn't implement any functionality.

#### Trait Implementations

##### `impl Any for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="xcoffcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffcomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffComdatSectionIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffcomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffcomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `XcoffComdatIterator32<'data, 'file, R>`

```rust
type XcoffComdatIterator32<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:11-12`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L11-L12)*

An iterator for the COMDAT section groups in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatIterator64<'data, 'file, R>`

```rust
type XcoffComdatIterator64<'data, 'file, R> = XcoffComdatIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:14-15`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L14-L15)*

An iterator for the COMDAT section groups in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdat32<'data, 'file, R>`

```rust
type XcoffComdat32<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:44-45`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L44-L45)*

A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdat64<'data, 'file, R>`

```rust
type XcoffComdat64<'data, 'file, R> = XcoffComdat<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:48-49`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L48-L49)*

A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

### `XcoffComdatSectionIterator32<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator32<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:105-106`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L105-L106)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile32`](super::XcoffFile32).

### `XcoffComdatSectionIterator64<'data, 'file, R>`

```rust
type XcoffComdatSectionIterator64<'data, 'file, R> = XcoffComdatSectionIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/comdat.rs:108-109`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/comdat.rs#L108-L109)*

An iterator for the sections in a COMDAT section group in a [`XcoffFile64`](super::XcoffFile64).

