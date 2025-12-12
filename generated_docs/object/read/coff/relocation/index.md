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

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coffrelocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coffrelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- <span id="coffrelocationiterator-iterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="coffrelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `CoffBigRelocationIterator<'data, 'file, R>`

```rust
type CoffBigRelocationIterator<'data, 'file, R> = CoffRelocationIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

*Defined in [`object-0.37.3/src/read/coff/relocation.rs:14-15`](../../../../../.source_1765521767/object-0.37.3/src/read/coff/relocation.rs#L14-L15)*

An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).

