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

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:20-27`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/relocation.rs#L20-L27)*

An iterator for the relocations in a [`MachOSection`](super::MachOSection).

#### Trait Implementations

##### `impl<'data, 'file, Mach, R> Debug for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="machorelocationiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="machorelocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data, 'file, Mach, R> Iterator for MachORelocationIterator<'data, 'file, Mach, R>`

- <span id="machorelocationiterator-type-item"></span>`type Item = (u64, Relocation)`

- <span id="machorelocationiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Type Aliases

### `MachORelocationIterator32<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator32<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader32<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:13-14`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/relocation.rs#L13-L14)*

An iterator for the relocations in a [`MachOSection32`](super::MachOSection32).

### `MachORelocationIterator64<'data, 'file, Endian, R>`

```rust
type MachORelocationIterator64<'data, 'file, Endian, R> = MachORelocationIterator<'data, 'file, macho::MachHeader64<Endian>, R>;
```

*Defined in [`object-0.37.3/src/read/macho/relocation.rs:16-17`](../../../../../.source_1765210505/object-0.37.3/src/read/macho/relocation.rs#L16-L17)*

An iterator for the relocations in a [`MachOSection64`](super::MachOSection64).

