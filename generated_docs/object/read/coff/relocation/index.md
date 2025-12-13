*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CoffRelocationIterator`](#coffrelocationiterator) | struct | An iterator for the relocations in a [`CoffSection`](super::CoffSection). |
| [`CoffBigRelocationIterator`](#coffbigrelocationiterator) | type | An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection). |

## Structs

### `CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageRelocation>,
}
```

*Defined in [`object-0.37.3/src/read/coff/relocation.rs:18-26`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/relocation.rs#L18-L26)*

An iterator for the relocations in a [`CoffSection`](super::CoffSection).

#### Trait Implementations

##### `impl Any for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="coffrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coffrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coffrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `CoffBigRelocationIterator<'data, 'file, R>`

```rust
type CoffBigRelocationIterator<'data, 'file, R> = CoffRelocationIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/relocation.rs:14-15`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/relocation.rs#L14-L15)*

An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).

