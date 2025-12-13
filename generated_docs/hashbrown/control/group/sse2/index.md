*[hashbrown](../../../index.md) / [control](../../index.md) / [group](../index.md) / [sse2](index.md)*

---

# Module `sse2`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Group`](#group) | struct | Abstraction over a group of control tags which can be scanned in parallel. |
| [`BitMaskWord`](#bitmaskword) | type |  |
| [`NonZeroBitMaskWord`](#nonzerobitmaskword) | type |  |
| [`BITMASK_STRIDE`](#bitmask-stride) | const |  |
| [`BITMASK_MASK`](#bitmask-mask) | const |  |
| [`BITMASK_ITER_MASK`](#bitmask-iter-mask) | const |  |

## Structs

### `Group`

```rust
struct Group(x86::__m128i);
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:21`](../../../../../.source_1765521767/hashbrown-0.16.1/src/control/group/sse2.rs#L21)*

Abstraction over a group of control tags which can be scanned in
parallel.

This implementation uses a 128-bit SSE value.

#### Implementations

- <span id="group-const-width"></span>`const WIDTH: usize`

- <span id="group-static-empty"></span>`const fn static_empty() -> &'static [Tag; 16]` — [`Tag`](../../tag/index.md#tag)

  Returns a full group of empty tags, suitable for use as the initial

  value for an empty hash table.

  

  This is guaranteed to be aligned to the group size.

- <span id="group-load"></span>`unsafe fn load(ptr: *const Tag) -> Self` — [`Tag`](../../tag/index.md#tag)

  Loads a group of tags starting at the given address.

- <span id="group-load-aligned"></span>`unsafe fn load_aligned(ptr: *const Tag) -> Self` — [`Tag`](../../tag/index.md#tag)

  Loads a group of tags starting at the given address, which must be

  aligned to `mem::align_of::<Group>()`.

- <span id="group-store-aligned"></span>`unsafe fn store_aligned(self, ptr: *mut Tag)` — [`Tag`](../../tag/index.md#tag)

  Stores the group of tags to the given address, which must be

  aligned to `mem::align_of::<Group>()`.

- <span id="group-match-tag"></span>`fn match_tag(self, tag: Tag) -> BitMask` — [`Tag`](../../tag/index.md#tag), [`BitMask`](../../bitmask/index.md#bitmask)

  Returns a `BitMask` indicating all tags in the group which have

  the given value.

- <span id="group-match-empty"></span>`fn match_empty(self) -> BitMask` — [`BitMask`](../../bitmask/index.md#bitmask)

  Returns a `BitMask` indicating all tags in the group which are

  `EMPTY`.

- <span id="group-match-empty-or-deleted"></span>`fn match_empty_or_deleted(self) -> BitMask` — [`BitMask`](../../bitmask/index.md#bitmask)

  Returns a `BitMask` indicating all tags in the group which are

  `EMPTY` or `DELETED`.

- <span id="group-match-full"></span>`fn match_full(&self) -> BitMask` — [`BitMask`](../../bitmask/index.md#bitmask)

  Returns a `BitMask` indicating all tags in the group which are full.

- <span id="group-convert-special-to-empty-and-full-to-deleted"></span>`fn convert_special_to_empty_and_full_to_deleted(self) -> Self`

  Performs the following transformation on all tags in the group:

  - `EMPTY => EMPTY`

  - `DELETED => EMPTY`

  - `FULL => DELETED`

#### Trait Implementations

##### `impl Any for Group`

- <span id="group-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Group`

- <span id="group-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Group`

- <span id="group-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl CloneToUninit for Group`

- <span id="group-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Group`

##### `impl<T> From for Group`

- <span id="group-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Group`

- <span id="group-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Group`

- <span id="group-toowned-type-owned"></span>`type Owned = T`

- <span id="group-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="group-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Group`

- <span id="group-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="group-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Group`

- <span id="group-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="group-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `BitMaskWord`

```rust
type BitMaskWord = u16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:10`](../../../../../.source_1765521767/hashbrown-0.16.1/src/control/group/sse2.rs#L10)*

### `NonZeroBitMaskWord`

```rust
type NonZeroBitMaskWord = core::num::NonZeroU16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:11`](../../../../../.source_1765521767/hashbrown-0.16.1/src/control/group/sse2.rs#L11)*

## Constants

### `BITMASK_STRIDE`
```rust
const BITMASK_STRIDE: usize = 1usize;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:12`](../../../../../.source_1765521767/hashbrown-0.16.1/src/control/group/sse2.rs#L12)*

### `BITMASK_MASK`
```rust
const BITMASK_MASK: u16 = 65_535u16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:13`](../../../../../.source_1765521767/hashbrown-0.16.1/src/control/group/sse2.rs#L13)*

### `BITMASK_ITER_MASK`
```rust
const BITMASK_ITER_MASK: u16 = 65_535u16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:14`](../../../../../.source_1765521767/hashbrown-0.16.1/src/control/group/sse2.rs#L14)*

