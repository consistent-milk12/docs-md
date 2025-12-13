*[gimli](../../index.md) / [read](../index.md) / [lists](index.md)*

---

# Module `lists`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ListsHeader`](#listsheader) | struct |  |
| [`parse_header`](#parse-header) | fn |  |

## Structs

### `ListsHeader`

```rust
struct ListsHeader {
    encoding: crate::common::Encoding,
    offset_entry_count: u32,
}
```

*Defined in [`gimli-0.32.3/src/read/lists.rs:5-9`](../../../../.source_1765633015/gimli-0.32.3/src/read/lists.rs#L5-L9)*

#### Implementations

- <span id="listsheader-size"></span>`fn size(self) -> u8`

  Return the serialized size of the table header.

- <span id="listsheader-size-for-encoding"></span>`fn size_for_encoding(encoding: Encoding) -> u8` — [`Encoding`](../../index.md#encoding)

  Return the serialized size of the table header.

#### Trait Implementations

##### `impl Any for ListsHeader`

- <span id="listsheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ListsHeader`

- <span id="listsheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ListsHeader`

- <span id="listsheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ListsHeader`

- <span id="listsheader-clone"></span>`fn clone(&self) -> ListsHeader` — [`ListsHeader`](#listsheader)

##### `impl CloneToUninit for ListsHeader`

- <span id="listsheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ListsHeader`

##### `impl Debug for ListsHeader`

- <span id="listsheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ListsHeader`

- <span id="listsheader-default"></span>`fn default() -> Self`

##### `impl<T> From for ListsHeader`

- <span id="listsheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ListsHeader`

- <span id="listsheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ListsHeader`

- <span id="listsheader-toowned-type-owned"></span>`type Owned = T`

- <span id="listsheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="listsheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ListsHeader`

- <span id="listsheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="listsheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ListsHeader`

- <span id="listsheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="listsheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_header`

```rust
fn parse_header<R: Reader>(input: &mut R) -> crate::read::Result<ListsHeader>
```

*Defined in [`gimli-0.32.3/src/read/lists.rs:43-68`](../../../../.source_1765633015/gimli-0.32.3/src/read/lists.rs#L43-L68)*

