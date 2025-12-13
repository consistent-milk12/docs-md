*[object](../../../index.md) / [read](../../index.md) / [macho](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MachORelocationIterator`](#machorelocationiterator) | struct | An iterator for the relocations in a [`MachOSection`](super::MachOSection). |
| [`MachORelocationIterator32`](#machorelocationiterator32) | type | An iterator for the relocations in a [`MachOSection32`](super::MachOSection32). |
| [`MachORelocationIterator64`](#machorelocationiterator64) | type | An iterator for the relocations in a [`MachOSection64`](super::MachOSection64). |

## Structs

### `MachORelocationIterator<'data, 'file, Mach, R>`

```rust
struct MachORelocationIterator<'data, 'file, Mach, R>
where
    Mach: MachHeader,
    R: ReadRef<'data> {
    file: &'file super::MachOFile<'data, Mach, R>,
    relocations: slice::Iter<'data, macho::Relocation<<Mach as >::Endian>>,
}
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:20-27`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/relocation.rs#L20-L27)*

An iterator for the relocations in a [`MachOSection`](super::MachOSection).

#### Trait Implementations

##### `impl Any for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Mach, R> Debug for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machorelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machorelocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Mach, R> Iterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="machorelocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="machorelocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="machorelocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `MachORelocationIterator32<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator32<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:13-14`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/relocation.rs#L13-L14)*

An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).

### `MachORelocationIterator64<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator64<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:16-17`](../../../../../.source_1765633015/object-0.37.3/src/read/macho/relocation.rs#L16-L17)*

An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).

