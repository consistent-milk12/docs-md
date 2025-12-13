*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XcoffRelocationIterator`](#xcoffrelocationiterator) | struct | An iterator for the relocations in an [`XcoffSection`](super::XcoffSection). |
| [`Rel`](#rel) | trait | A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`]. |
| [`XcoffRelocationIterator32`](#xcoffrelocationiterator32) | type | An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32). |
| [`XcoffRelocationIterator64`](#xcoffrelocationiterator64) | type | An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64). |

## Structs

### `XcoffRelocationIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffRelocationIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    relocations: slice::Iter<'data, <<Xcoff as FileHeader>::SectionHeader as SectionHeader>::Rel>,
}
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:23-32`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L23-L32)*

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

#### Trait Implementations

##### `impl Any for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Xcoff, R> Debug for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="xcoffrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="xcoffrelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Xcoff, R> Iterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="xcoffrelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xcoffrelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- <span id="xcoffrelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xcoffrelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Rel`

```rust
trait Rel: Debug + Pod { ... }
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:88-98`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L88-L98)*

A trait for generic access to [`xcoff::Rel32`](../../../xcoff/index.md) and [`xcoff::Rel64`](../../../xcoff/index.md).

#### Associated Types

- `type Word: 1`

#### Required Methods

- `fn r_vaddr(&self) -> <Self as >::Word`

- `fn r_symndx(&self) -> u32`

- `fn r_rsize(&self) -> u8`

- `fn r_rtype(&self) -> u8`

#### Provided Methods

- `fn symbol(&self) -> SymbolIndex`

#### Implementors

- [`Rel32`](../../../xcoff/index.md#rel32)
- [`Rel64`](../../../xcoff/index.md#rel64)

## Type Aliases

### `XcoffRelocationIterator32<'data, 'file, R>`

```rust
type XcoffRelocationIterator32<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:16-17`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L16-L17)*

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).

### `XcoffRelocationIterator64<'data, 'file, R>`

```rust
type XcoffRelocationIterator64<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>;
```

*Defined in [`object-0.37.3/src/read/xcoff/relocation.rs:19-20`](../../../../../.source_1765633015/object-0.37.3/src/read/xcoff/relocation.rs#L19-L20)*

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

