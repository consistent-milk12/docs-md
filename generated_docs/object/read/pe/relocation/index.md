*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RelocationBlockIterator`](#relocationblockiterator) | struct | An iterator over the relocation blocks in the `.reloc` section of a PE file. |
| [`RelocationIterator`](#relocationiterator) | struct | An iterator of the relocations in a block in the `.reloc` section of a PE file. |
| [`Relocation`](#relocation) | struct | A relocation in the `.reloc` section of a PE file. |

## Structs

### `RelocationBlockIterator<'data>`

```rust
struct RelocationBlockIterator<'data> {
    data: crate::read::Bytes<'data>,
}
```

An iterator over the relocation blocks in the `.reloc` section of a PE file.

Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks).

#### Implementations

- <span id="relocationblockiterator-new"></span>`fn new(data: &'data [u8]) -> Self`

- <span id="relocationblockiterator-next"></span>`fn next(&mut self) -> Result<Option<RelocationIterator<'data>>>` — [`Result`](../../../index.md), [`RelocationIterator`](../index.md)

- <span id="relocationblockiterator-parse"></span>`fn parse(&mut self) -> Result<RelocationIterator<'data>>` — [`Result`](../../../index.md), [`RelocationIterator`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-clone"></span>`fn clone(&self) -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](../index.md)

##### `impl<'data> Copy for RelocationBlockIterator<'data>`

##### `impl<'data> Debug for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data> Default for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-default"></span>`fn default() -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](../index.md)

##### `impl<I> IntoIterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationblockiterator-intoiter"></span>`type IntoIter = I`

- <span id="relocationblockiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data> Iterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-item"></span>`type Item = Result<RelocationIterator<'data>, Error>`

- <span id="relocationblockiterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RelocationIterator<'data>`

```rust
struct RelocationIterator<'data> {
    virtual_address: u32,
    size: u32,
    relocs: slice::Iter<'data, crate::endian::U16<crate::endian::LittleEndian>>,
}
```

An iterator of the relocations in a block in the `.reloc` section of a PE file.

#### Implementations

- <span id="relocationiterator-virtual-address"></span>`fn virtual_address(&self) -> u32`

- <span id="relocationiterator-size"></span>`fn size(&self) -> u32`

#### Trait Implementations

##### `impl<'data> Clone for RelocationIterator<'data>`

- <span id="relocationiterator-clone"></span>`fn clone(&self) -> RelocationIterator<'data>` — [`RelocationIterator`](../index.md)

##### `impl<'data> Debug for RelocationIterator<'data>`

- <span id="relocationiterator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for RelocationIterator<'data>`

- <span id="relocationiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationiterator-intoiter"></span>`type IntoIter = I`

- <span id="relocationiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'data> Iterator for RelocationIterator<'data>`

- <span id="relocationiterator-item"></span>`type Item = Relocation`

- <span id="relocationiterator-next"></span>`fn next(&mut self) -> Option<Relocation>` — [`Relocation`](../index.md)

### `Relocation`

```rust
struct Relocation {
    pub virtual_address: u32,
    pub typ: u16,
}
```

A relocation in the `.reloc` section of a PE file.

#### Fields

- **`virtual_address`**: `u32`

  The virtual address of the relocation.

- **`typ`**: `u16`

  One of the `pe::IMAGE_REL_BASED_*` constants.

#### Trait Implementations

##### `impl Clone for Relocation`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation` — [`Relocation`](../index.md)

##### `impl Copy for Relocation`

##### `impl Debug for Relocation`

- <span id="relocation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Relocation`

- <span id="relocation-default"></span>`fn default() -> Relocation` — [`Relocation`](../index.md)

