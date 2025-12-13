*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [shiftor](index.md)*

---

# Module `shiftor`

An implementation of the [Shift-Or substring search algorithm][shiftor](#shiftor).


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A forward substring searcher using the Shift-Or algorithm. |
| [`Mask`](#mask) | type | The type of our mask. |

## Structs

### `Finder`

```rust
struct Finder {
    masks: alloc::boxed::Box<[u16; 256]>,
    needle_len: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/shiftor.rs:20-23`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/shiftor.rs#L20-L23)*

A forward substring searcher using the Shift-Or algorithm.

#### Implementations

- <span id="finder-const-max-needle-len"></span>`const MAX_NEEDLE_LEN: usize`

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](#finder)

  Create a new Shift-Or forward searcher for the given `needle`.

  

  The needle may be empty. The empty needle matches at every byte offset.

- <span id="finder-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of the needle given to `Finder::new` in

  the `haystack` given. If no such occurrence exists, then `None` is

  returned.

  

  Unlike most other substring search implementations in this crate, this

  finder does not require passing the needle at search time. A match can

  be determined without the needle at all since the required information

  is already encoded into this finder at construction time.

  

  The maximum value this can return is `haystack.len()`, which can only

  occur when the needle and haystack both have length zero. Otherwise,

  for non-empty haystacks, the maximum value is `haystack.len() - 1`.

#### Trait Implementations

##### `impl Any for Finder`

- <span id="finder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Finder`

- <span id="finder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Finder`

- <span id="finder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Finder`

- <span id="finder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Finder`

- <span id="finder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Finder`

- <span id="finder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Finder`

- <span id="finder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="finder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Finder`

- <span id="finder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="finder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `Mask`

```rust
type Mask = u16;
```

*Defined in [`memchr-2.7.6/src/arch/all/shiftor.rs:16`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/shiftor.rs#L16)*

The type of our mask.

While we don't expose anyway to configure this in the public API, if one
really needs less memory usage or support for longer needles, then it is
suggested to copy the code from this module and modify it to fit your
needs. The code below is written to be correct regardless of whether Mask
is a u8, u16, u32, u64 or u128.

