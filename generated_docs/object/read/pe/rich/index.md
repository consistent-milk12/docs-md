*[object](../../../index.md) / [read](../../index.md) / [pe](../index.md) / [rich](index.md)*

---

# Module `rich`

PE rich header handling

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RichHeaderInfo`](#richheaderinfo) | struct | Parsed information about a Rich Header. |
| [`RichHeaderEntry`](#richheaderentry) | struct | A PE rich header entry after it has been unmasked. |
| [`memmem`](#memmem) | fn | Find the offset of the first occurrence of needle in the data. |

## Structs

### `RichHeaderInfo<'data>`

```rust
struct RichHeaderInfo<'data> {
    pub offset: usize,
    pub length: usize,
    pub xor_key: u32,
    masked_entries: &'data [pe::MaskedRichHeaderEntry],
}
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:12-26`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/rich.rs#L12-L26)*

Parsed information about a Rich Header.

#### Fields

- **`offset`**: `usize`

  The offset at which the rich header starts.

- **`length`**: `usize`

  The length (in bytes) of the rich header.
  
  This includes the payload, but also the 16-byte start sequence and the
  8-byte final "Rich" and XOR key.

- **`xor_key`**: `u32`

  The XOR key used to mask the rich header.
  
  Unless the file has been tampered with, it should be equal to a checksum
  of the file header.

#### Implementations

- <span id="richheaderinfo-parse"></span>`fn parse<R: ReadRef<'data>>(data: R, nt_header_offset: u64) -> Option<Self>`

  Try to locate a rich header and its entries in the current PE file.

- <span id="richheaderinfo-unmasked-entries"></span>`fn unmasked_entries(&self) -> impl Iterator<Item = RichHeaderEntry> + 'data` — [`RichHeaderEntry`](../index.md#richheaderentry)

  Returns an iterator over the unmasked entries.

#### Trait Implementations

##### `impl Any for RichHeaderInfo<'data>`

- <span id="richheaderinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RichHeaderInfo<'data>`

- <span id="richheaderinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RichHeaderInfo<'data>`

- <span id="richheaderinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RichHeaderInfo<'data>`

- <span id="richheaderinfo-clone"></span>`fn clone(&self) -> RichHeaderInfo<'data>` — [`RichHeaderInfo`](../index.md#richheaderinfo)

##### `impl CloneToUninit for RichHeaderInfo<'data>`

- <span id="richheaderinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RichHeaderInfo<'data>`

##### `impl Debug for RichHeaderInfo<'data>`

- <span id="richheaderinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RichHeaderInfo<'data>`

- <span id="richheaderinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RichHeaderInfo<'data>`

- <span id="richheaderinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RichHeaderInfo<'data>`

- <span id="richheaderinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="richheaderinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="richheaderinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RichHeaderInfo<'data>`

- <span id="richheaderinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="richheaderinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RichHeaderInfo<'data>`

- <span id="richheaderinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="richheaderinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RichHeaderEntry`

```rust
struct RichHeaderEntry {
    pub comp_id: u32,
    pub count: u32,
}
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:33-38`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/rich.rs#L33-L38)*

A PE rich header entry after it has been unmasked.

See [`pe::MaskedRichHeaderEntry`](../../../pe/index.md).

#### Fields

- **`comp_id`**: `u32`

  ID of the component.

- **`count`**: `u32`

  Number of times this component has been used when building this PE.

#### Trait Implementations

##### `impl Any for RichHeaderEntry`

- <span id="richheaderentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RichHeaderEntry`

- <span id="richheaderentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RichHeaderEntry`

- <span id="richheaderentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RichHeaderEntry`

- <span id="richheaderentry-clone"></span>`fn clone(&self) -> RichHeaderEntry` — [`RichHeaderEntry`](../index.md#richheaderentry)

##### `impl CloneToUninit for RichHeaderEntry`

- <span id="richheaderentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RichHeaderEntry`

##### `impl Debug for RichHeaderEntry`

- <span id="richheaderentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RichHeaderEntry`

- <span id="richheaderentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RichHeaderEntry`

- <span id="richheaderentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RichHeaderEntry`

- <span id="richheaderentry-toowned-type-owned"></span>`type Owned = T`

- <span id="richheaderentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="richheaderentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RichHeaderEntry`

- <span id="richheaderentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="richheaderentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RichHeaderEntry`

- <span id="richheaderentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="richheaderentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `memmem`

```rust
fn memmem(data: &[u8], needle: &[u8], align: usize) -> Option<usize>
```

*Defined in [`object-0.37.3/src/read/pe/rich.rs:84-92`](../../../../../.source_1765633015/object-0.37.3/src/read/pe/rich.rs#L84-L92)*

Find the offset of the first occurrence of needle in the data.

The offset must have the given alignment.

