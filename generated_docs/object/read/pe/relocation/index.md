*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [relocation](index.md)*

---

# Module `relocation`

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

- `fn new(data: &'data [u8]) -> Self`

- `fn next(self: &mut Self) -> Result<Option<RelocationIterator<'data>>>` — [`Result`](../../../index.md), [`RelocationIterator`](../index.md)

- `fn parse(self: &mut Self) -> Result<RelocationIterator<'data>>` — [`Result`](../../../index.md), [`RelocationIterator`](../index.md)

#### Trait Implementations

##### `impl<'data> Clone for RelocationBlockIterator<'data>`

- `fn clone(self: &Self) -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](../index.md)

##### `impl<'data> Copy for RelocationBlockIterator<'data>`

##### `impl<'data> Debug for RelocationBlockIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data> Default for RelocationBlockIterator<'data>`

- `fn default() -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](../index.md)

##### `impl<I> IntoIterator for RelocationBlockIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for RelocationBlockIterator<'data>`

- `type Item = Result<RelocationIterator<'data>, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn virtual_address(self: &Self) -> u32`

- `fn size(self: &Self) -> u32`

#### Trait Implementations

##### `impl<'data> Clone for RelocationIterator<'data>`

- `fn clone(self: &Self) -> RelocationIterator<'data>` — [`RelocationIterator`](../index.md)

##### `impl<'data> Debug for RelocationIterator<'data>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for RelocationIterator<'data>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data> Iterator for RelocationIterator<'data>`

- `type Item = Relocation`

- `fn next(self: &mut Self) -> Option<Relocation>` — [`Relocation`](../index.md)

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

- `fn clone(self: &Self) -> Relocation` — [`Relocation`](../index.md)

##### `impl Copy for Relocation`

##### `impl Debug for Relocation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Relocation`

- `fn default() -> Relocation` — [`Relocation`](../index.md)

