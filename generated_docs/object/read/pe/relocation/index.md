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

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:11-13`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/relocation.rs#L11-L13)*

An iterator over the relocation blocks in the `.reloc` section of a PE file.

Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks).

#### Implementations

- <span id="relocationblockiterator-new"></span>`fn new(data: &'data [u8]) -> Self`

  Construct a new iterator from the data of the `.reloc` section.

- <span id="relocationblockiterator-next"></span>`fn next(&mut self) -> Result<Option<RelocationIterator<'data>>>` — [`Result`](../../../index.md#result), [`RelocationIterator`](../index.md#relocationiterator)

  Read the next relocation page.

- <span id="relocationblockiterator-parse"></span>`fn parse(&mut self) -> Result<RelocationIterator<'data>>` — [`Result`](../../../index.md#result), [`RelocationIterator`](../index.md#relocationiterator)

#### Trait Implementations

##### `impl Any for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-clone"></span>`fn clone(&self) -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](../index.md#relocationblockiterator)

##### `impl CloneToUninit for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationBlockIterator<'data>`

##### `impl Debug for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-default"></span>`fn default() -> RelocationBlockIterator<'data>` — [`RelocationBlockIterator`](../index.md#relocationblockiterator)

##### `impl<T> From for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationblockiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relocationblockiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-iterator-type-item"></span>`type Item = Result<RelocationIterator<'data>, Error>`

- <span id="relocationblockiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationblockiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationblockiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationblockiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationBlockIterator<'data>`

- <span id="relocationblockiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationblockiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationIterator<'data>`

```rust
struct RelocationIterator<'data> {
    virtual_address: u32,
    size: u32,
    relocs: slice::Iter<'data, crate::endian::U16<crate::endian::LittleEndian>>,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:68-72`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/relocation.rs#L68-L72)*

An iterator of the relocations in a block in the `.reloc` section of a PE file.

#### Implementations

- <span id="relocationiterator-virtual-address"></span>`fn virtual_address(&self) -> u32`

  Return the virtual address of the page that this block of relocations applies to.

- <span id="relocationiterator-size"></span>`fn size(&self) -> u32`

  Return the size in bytes of this block of relocations.

#### Trait Implementations

##### `impl Any for RelocationIterator<'data>`

- <span id="relocationiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationIterator<'data>`

- <span id="relocationiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationIterator<'data>`

- <span id="relocationiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationIterator<'data>`

- <span id="relocationiterator-clone"></span>`fn clone(&self) -> RelocationIterator<'data>` — [`RelocationIterator`](../index.md#relocationiterator)

##### `impl CloneToUninit for RelocationIterator<'data>`

- <span id="relocationiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RelocationIterator<'data>`

- <span id="relocationiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelocationIterator<'data>`

- <span id="relocationiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RelocationIterator<'data>`

- <span id="relocationiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RelocationIterator<'data>`

- <span id="relocationiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="relocationiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="relocationiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RelocationIterator<'data>`

- <span id="relocationiterator-iterator-type-item"></span>`type Item = Relocation`

- <span id="relocationiterator-iterator-next"></span>`fn next(&mut self) -> Option<Relocation>` — [`Relocation`](../index.md#relocation)

##### `impl ToOwned for RelocationIterator<'data>`

- <span id="relocationiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationIterator<'data>`

- <span id="relocationiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationIterator<'data>`

- <span id="relocationiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Relocation`

```rust
struct Relocation {
    pub virtual_address: u32,
    pub typ: u16,
}
```

*Defined in [`object-0.37.3/src/read/pe/relocation.rs:104-109`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/relocation.rs#L104-L109)*

A relocation in the `.reloc` section of a PE file.

#### Fields

- **`virtual_address`**: `u32`

  The virtual address of the relocation.

- **`typ`**: `u16`

  One of the `pe::IMAGE_REL_BASED_*` constants.

#### Trait Implementations

##### `impl Any for Relocation`

- <span id="relocation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Relocation`

- <span id="relocation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Relocation`

- <span id="relocation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Relocation`

- <span id="relocation-clone"></span>`fn clone(&self) -> Relocation` — [`Relocation`](../index.md#relocation)

##### `impl CloneToUninit for Relocation`

- <span id="relocation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Relocation`

##### `impl Debug for Relocation`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Relocation`

- <span id="relocation-default"></span>`fn default() -> Relocation` — [`Relocation`](../index.md#relocation)

##### `impl<T> From for Relocation`

- <span id="relocation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Relocation`

- <span id="relocation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Relocation`

- <span id="relocation-toowned-type-owned"></span>`type Owned = T`

- <span id="relocation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Relocation`

- <span id="relocation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Relocation`

- <span id="relocation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

