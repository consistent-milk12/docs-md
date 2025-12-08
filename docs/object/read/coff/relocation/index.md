*[object](../../../index.md) / [read](../../index.md) / [coff](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Structs

### `CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader>`

```rust
struct CoffRelocationIterator<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> {
    file: &'file super::CoffFile<'data, R, Coff>,
    iter: slice::Iter<'data, pe::ImageRelocation>,
}
```

An iterator for the relocations in a [`CoffSection`](super::CoffSection).

#### Trait Implementations

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Debug for CoffRelocationIterator<'data, 'file, R, Coff>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, R: ReadRef<'data>, Coff: CoffHeader> Iterator for CoffRelocationIterator<'data, 'file, R, Coff>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Type Aliases

### `CoffBigRelocationIterator<'data, 'file, R>`

```rust
type CoffBigRelocationIterator<'data, 'file, R> = CoffRelocationIterator<'data, 'file, R, pe::AnonObjectHeaderBigobj>;
```

An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection).

