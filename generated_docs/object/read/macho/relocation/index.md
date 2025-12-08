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

An iterator for the relocations in a [`MachOSection`](super::MachOSection).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machorelocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="machorelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-item"></span>`type Item = (u64, Relocation)`

- <span id="machorelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `MachORelocationIterator32<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator32<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).

### `MachORelocationIterator64<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator64<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).

