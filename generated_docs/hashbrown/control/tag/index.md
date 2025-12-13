*[hashbrown](../../index.md) / [control](../index.md) / [tag](index.md)*

---

# Module `tag`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Tag`](#tag) | struct | Single tag in a control group. |
| [`TagSliceExt`](#tagsliceext) | trait | Extension trait for slices of tags. |

## Structs

### `Tag`

```rust
struct Tag(u8);
```

*Defined in [`hashbrown-0.16.1/src/control/tag.rs:6`](../../../../.source_1765633015/hashbrown-0.16.1/src/control/tag.rs#L6)*

Single tag in a control group.

#### Implementations

- <span id="tag-const-empty"></span>`const EMPTY: Tag`

- <span id="tag-const-deleted"></span>`const DELETED: Tag`

- <span id="tag-is-full"></span>`const fn is_full(self) -> bool`

  Checks whether a control tag represents a full bucket (top bit is clear).

- <span id="tag-is-special"></span>`const fn is_special(self) -> bool`

  Checks whether a control tag represents a special value (top bit is set).

- <span id="tag-special-is-empty"></span>`const fn special_is_empty(self) -> bool`

  Checks whether a special control value is EMPTY (just check 1 bit).

- <span id="tag-full"></span>`const fn full(hash: u64) -> Tag` — [`Tag`](#tag)

  Creates a control tag representing a full bucket with the given hash.

#### Trait Implementations

##### `impl Any for Tag`

- <span id="tag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tag`

- <span id="tag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tag`

- <span id="tag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Tag`

- <span id="tag-clone"></span>`fn clone(&self) -> Tag` — [`Tag`](#tag)

##### `impl CloneToUninit for Tag`

- <span id="tag-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Tag`

##### `impl Debug for Tag`

- <span id="tag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Tag`

##### `impl<K> Equivalent for Tag`

- <span id="tag-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T> From for Tag`

- <span id="tag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tag`

- <span id="tag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Tag`

- <span id="tag-partialeq-eq"></span>`fn eq(&self, other: &Tag) -> bool` — [`Tag`](#tag)

##### `impl StructuralPartialEq for Tag`

##### `impl ToOwned for Tag`

- <span id="tag-toowned-type-owned"></span>`type Owned = T`

- <span id="tag-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tag-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tag`

- <span id="tag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tag`

- <span id="tag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `TagSliceExt`

```rust
trait TagSliceExt { ... }
```

*Defined in [`hashbrown-0.16.1/src/control/tag.rs:67-76`](../../../../.source_1765633015/hashbrown-0.16.1/src/control/tag.rs#L67-L76)*

Extension trait for slices of tags.

#### Required Methods

- `fn fill_tag(&mut self, tag: Tag)`

  Fills the control with the given tag.

#### Provided Methods

- `fn fill_empty(&mut self)`

  Clears out the control.

#### Implementors

- `[Tag]`

