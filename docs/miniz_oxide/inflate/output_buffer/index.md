*[miniz_oxide](../../index.md) / [inflate](../index.md) / [output_buffer](index.md)*

---

# Module `output_buffer`

## Structs

### `OutputBuffer<'a>`

```rust
struct OutputBuffer<'a> {
    slice: &'a mut [u8],
    position: usize,
}
```

A wrapper for the output slice used when decompressing.

Using this rather than `Cursor` lets us implement the writing methods directly on
the buffer and lets us use a usize rather than u64 for the position which helps with
performance on 32-bit systems.

#### Implementations

- `fn from_slice_and_pos(slice: &'a mut [u8], position: usize) -> OutputBuffer<'a>` — [`OutputBuffer`](#outputbuffer)

- `const fn position(self: &Self) -> usize`

- `fn set_position(self: &mut Self, position: usize)`

- `fn write_byte(self: &mut Self, byte: u8)`

- `fn write_slice(self: &mut Self, data: &[u8])`

- `const fn bytes_left(self: &Self) -> usize`

- `const fn get_ref(self: &Self) -> &[u8]`

- `fn get_mut(self: &mut Self) -> &mut [u8]`

### `InputWrapper<'a>`

```rust
struct InputWrapper<'a> {
    slice: &'a [u8],
}
```

A wrapper for the output slice used when decompressing.

Using this rather than `Cursor` lets us implement the writing methods directly on
the buffer and lets us use a usize rather than u64 for the position which helps with
performance on 32-bit systems.

#### Implementations

- `const fn as_slice(self: &Self) -> &[u8]`

- `const fn from_slice(slice: &'a [u8]) -> InputWrapper<'a>` — [`InputWrapper`](#inputwrapper)

- `fn advance(self: &mut Self, steps: usize)`

- `fn read_byte(self: &mut Self) -> Option<u8>`

- `fn read_u32_le(self: &mut Self) -> u32`

- `const fn bytes_left(self: &Self) -> usize`

#### Trait Implementations

##### `impl<'a> Clone for InputWrapper<'a>`

- `fn clone(self: &Self) -> InputWrapper<'a>` — [`InputWrapper`](#inputwrapper)

##### `impl<'a> Copy for InputWrapper<'a>`

