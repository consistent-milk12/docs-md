*[miniz_oxide](../../index.md) / [inflate](../index.md) / [output_buffer](index.md)*

---

# Module `output_buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OutputBuffer`](#outputbuffer) | struct | A wrapper for the output slice used when decompressing. |
| [`InputWrapper`](#inputwrapper) | struct | A wrapper for the output slice used when decompressing. |

## Structs

### `OutputBuffer<'a>`

```rust
struct OutputBuffer<'a> {
    slice: &'a mut [u8],
    position: usize,
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/output_buffer.rs:6-9`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/output_buffer.rs#L6-L9)*

A wrapper for the output slice used when decompressing.

Using this rather than `Cursor` lets us implement the writing methods directly on
the buffer and lets us use a usize rather than u64 for the position which helps with
performance on 32-bit systems.

#### Implementations

- <span id="outputbuffer-from-slice-and-pos"></span>`fn from_slice_and_pos(slice: &'a mut [u8], position: usize) -> OutputBuffer<'a>` — [`OutputBuffer`](#outputbuffer)

- <span id="outputbuffer-position"></span>`const fn position(&self) -> usize`

- <span id="outputbuffer-set-position"></span>`fn set_position(&mut self, position: usize)`

- <span id="outputbuffer-write-byte"></span>`fn write_byte(&mut self, byte: u8)`

  Write a byte to the current position and increment

  

  Assumes that there is space.

- <span id="outputbuffer-write-slice"></span>`fn write_slice(&mut self, data: &[u8])`

  Write a slice to the current position and increment

  

  Assumes that there is space.

- <span id="outputbuffer-bytes-left"></span>`const fn bytes_left(&self) -> usize`

- <span id="outputbuffer-get-ref"></span>`const fn get_ref(&self) -> &[u8]`

- <span id="outputbuffer-get-mut"></span>`fn get_mut(&mut self) -> &mut [u8]`

#### Trait Implementations

##### `impl Any for OutputBuffer<'a>`

- <span id="outputbuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OutputBuffer<'a>`

- <span id="outputbuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OutputBuffer<'a>`

- <span id="outputbuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for OutputBuffer<'a>`

- <span id="outputbuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OutputBuffer<'a>`

- <span id="outputbuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OutputBuffer<'a>`

- <span id="outputbuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="outputbuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OutputBuffer<'a>`

- <span id="outputbuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="outputbuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InputWrapper<'a>`

```rust
struct InputWrapper<'a> {
    slice: &'a [u8],
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/output_buffer.rs:68-70`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/output_buffer.rs#L68-L70)*

A wrapper for the output slice used when decompressing.

Using this rather than `Cursor` lets us implement the writing methods directly on
the buffer and lets us use a usize rather than u64 for the position which helps with
performance on 32-bit systems.

#### Implementations

- <span id="inputwrapper-as-slice"></span>`const fn as_slice(&self) -> &[u8]`

- <span id="inputwrapper-from-slice"></span>`const fn from_slice(slice: &'a [u8]) -> InputWrapper<'a>` — [`InputWrapper`](#inputwrapper)

- <span id="inputwrapper-advance"></span>`fn advance(&mut self, steps: usize)`

- <span id="inputwrapper-read-byte"></span>`fn read_byte(&mut self) -> Option<u8>`

- <span id="inputwrapper-read-u32-le"></span>`fn read_u32_le(&mut self) -> u32`

- <span id="inputwrapper-bytes-left"></span>`const fn bytes_left(&self) -> usize`

#### Trait Implementations

##### `impl Any for InputWrapper<'a>`

- <span id="inputwrapper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InputWrapper<'a>`

- <span id="inputwrapper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InputWrapper<'a>`

- <span id="inputwrapper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for InputWrapper<'a>`

- <span id="inputwrapper-clone"></span>`fn clone(&self) -> InputWrapper<'a>` — [`InputWrapper`](#inputwrapper)

##### `impl CloneToUninit for InputWrapper<'a>`

- <span id="inputwrapper-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for InputWrapper<'a>`

##### `impl<T> From for InputWrapper<'a>`

- <span id="inputwrapper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InputWrapper<'a>`

- <span id="inputwrapper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InputWrapper<'a>`

- <span id="inputwrapper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inputwrapper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InputWrapper<'a>`

- <span id="inputwrapper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inputwrapper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

