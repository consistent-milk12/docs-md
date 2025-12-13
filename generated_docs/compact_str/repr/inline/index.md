*[compact_str](../../index.md) / [repr](../index.md) / [inline](index.md)*

---

# Module `inline`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InlineBuffer`](#inlinebuffer) | struct | A buffer stored on the stack whose size is equal to the stack size of `String` |

## Structs

### `InlineBuffer`

```rust
struct InlineBuffer([u8; 24]);
```

*Defined in [`compact_str-0.9.0/src/repr/inline.rs:8`](../../../../.source_1765521767/compact_str-0.9.0/src/repr/inline.rs#L8)*

A buffer stored on the stack whose size is equal to the stack size of `String`

#### Implementations

- <span id="inlinebuffer-new"></span>`unsafe fn new(text: &str) -> Self`

  Construct a new `InlineString`. A string that lives in a small buffer on the stack

  

  SAFETY:

  * The caller must guarantee that the length of `text` is less than [`MAX_SIZE`](../index.md)

- <span id="inlinebuffer-new-const"></span>`const fn new_const(text: &str) -> Self`

- <span id="inlinebuffer-empty"></span>`const fn empty() -> Self`

  Returns an empty [`InlineBuffer`](#inlinebuffer)

- <span id="inlinebuffer-set-len"></span>`unsafe fn set_len(&mut self, len: usize)`

  Set's the length of the content for this [`InlineBuffer`](#inlinebuffer)

  

  # SAFETY:

  * The caller must guarantee that `len` bytes in the buffer are valid UTF-8

#### Trait Implementations

##### `impl Any for InlineBuffer`

- <span id="inlinebuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InlineBuffer`

- <span id="inlinebuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InlineBuffer`

- <span id="inlinebuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for InlineBuffer`

- <span id="inlinebuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InlineBuffer`

- <span id="inlinebuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InlineBuffer`

- <span id="inlinebuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inlinebuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InlineBuffer`

- <span id="inlinebuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inlinebuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

