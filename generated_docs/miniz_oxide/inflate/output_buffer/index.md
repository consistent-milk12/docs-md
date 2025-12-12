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

- <span id="outputbuffer-write-slice"></span>`fn write_slice(&mut self, data: &[u8])`

- <span id="outputbuffer-bytes-left"></span>`const fn bytes_left(&self) -> usize`

- <span id="outputbuffer-get-ref"></span>`const fn get_ref(&self) -> &[u8]`

- <span id="outputbuffer-get-mut"></span>`fn get_mut(&mut self) -> &mut [u8]`

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

##### `impl Clone for InputWrapper<'a>`

- <span id="inputwrapper-clone"></span>`fn clone(&self) -> InputWrapper<'a>` — [`InputWrapper`](#inputwrapper)

##### `impl Copy for InputWrapper<'a>`

