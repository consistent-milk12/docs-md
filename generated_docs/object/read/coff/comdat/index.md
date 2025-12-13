*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [comdat](index.md)*

---

# Module `comdat`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CoffComdatIterator`](#coffcomdatiterator) | struct | An iterator for the COMDAT section groups in a [`CoffFile`]. |
| [`CoffComdat`](#coffcomdat) | struct | A COMDAT section group in a [`CoffFile`]. |
| [`CoffComdatSectionIterator`](#coffcomdatsectioniterator) | struct | An iterator for the sections in a COMDAT section group in a [`CoffFile`]. |
| [`CoffBigComdatIterator`](#coffbigcomdatiterator) | type | An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigComdat`](#coffbigcomdat) | type | A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile). |
| [`CoffBigComdatSectionIterator`](#coffbigcomdatsectioniterator) | type | An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile). |

## Structs

### `CoffComdatIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdatIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    index: crate::read::SymbolIndex,
}
```

*Defined in [`object-0.37.3/src/read/coff/comdat.rs:17-25`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/comdat.rs#L17-L25)*

An iterator for the COMDAT section groups in a [`CoffFile`](../index.md).

#### Implementations

- <span id="coffcomdatiterator-new"></span>`fn new(file: &'file CoffFile<'data, R, Coff>) -> Self` — [`CoffFile`](../index.md#cofffile)

#### Trait Implementations

##### `impl Any for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffcomdatiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffcomdatiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-iterator-type-item"></span>`type Item = CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffcomdatiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffComdatIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffcomdatiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffComdat<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdat<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    symbol_index: crate::read::SymbolIndex,
    symbol: &'data <Coff as >::ImageSymbol,
    selection: u8,
}
```

*Defined in [`object-0.37.3/src/read/coff/comdat.rs:63-73`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/comdat.rs#L63-L73)*

A COMDAT section group in a [`CoffFile`](../index.md).

Most functionality is provided by the [`ObjectComdat`](../../index.md) trait implementation.

#### Implementations

- <span id="coffcomdat-parse"></span>`fn parse(file: &'file CoffFile<'data, R, Coff>, section_symbol: &'data <Coff as >::ImageSymbol, index: SymbolIndex) -> Option<CoffComdat<'data, 'file, R, Coff>>` — [`CoffFile`](../index.md#cofffile), [`CoffHeader`](../index.md#coffheader), [`SymbolIndex`](../../../index.md#symbolindex), [`CoffComdat`](../index.md#coffcomdat)

#### Trait Implementations

##### `impl Any for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> ObjectComdat for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-objectcomdat-type-sectioniterator"></span>`type SectionIterator = CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdat-objectcomdat-kind"></span>`fn kind(&self) -> ComdatKind` — [`ComdatKind`](../../../index.md#comdatkind)

- <span id="coffcomdat-objectcomdat-symbol"></span>`fn symbol(&self) -> SymbolIndex` — [`SymbolIndex`](../../../index.md#symbolindex)

- <span id="coffcomdat-objectcomdat-name-bytes"></span>`fn name_bytes(&self) -> Result<&'data [u8]>` — [`Result`](../../../index.md#result)

- <span id="coffcomdat-objectcomdat-name"></span>`fn name(&self) -> Result<&'data str>` — [`Result`](../../../index.md#result)

- <span id="coffcomdat-objectcomdat-sections"></span>`fn sections(&self) -> <Self as >::SectionIterator` — [`ObjectComdat`](../../index.md#objectcomdat)

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Sealed for CoffComdat<'data, 'file, R, Coff>`

##### `impl<U> TryFrom for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffcomdat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffComdat<'data, 'file, R, Coff>`

- <span id="coffcomdat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffcomdat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CoffComdatSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffComdatSectionIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    section_number: i32,
    index: crate::read::SymbolIndex,
}
```

*Defined in [`object-0.37.3/src/read/coff/comdat.rs:172-181`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/comdat.rs#L172-L181)*

An iterator for the sections in a COMDAT section group in a [`CoffFile`](../index.md).

#### Trait Implementations

##### `impl Any for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ReadRef<'data>, Coff: fmt::Debug + CoffHeader> Debug for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffcomdatsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffcomdatsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-iterator-type-item"></span>`type Item = SectionIndex`

- <span id="coffcomdatsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffcomdatsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffComdatSectionIterator<'data, 'file, R, Coff>`

- <span id="coffcomdatsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffcomdatsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `CoffBigComdatIterator<'data, 'file, R>`

```rust
type CoffBigComdatIterator<'data, 'file, R> = CoffComdatIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/comdat.rs:12-13`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/comdat.rs#L12-L13)*

An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile).

### `CoffBigComdat<'data, 'file, R>`

```rust
type CoffBigComdat<'data, 'file, R> = CoffComdat<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/comdat.rs:56-57`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/comdat.rs#L56-L57)*

A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

Most functionality is provided by the [`ObjectComdat`](../../index.md) trait implementation.

### `CoffBigComdatSectionIterator<'data, 'file, R>`

```rust
type CoffBigComdatSectionIterator<'data, 'file, R> = CoffComdatSectionIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/comdat.rs:167-168`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/comdat.rs#L167-L168)*

An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile).

