*[compact_str](../../index.md) / [repr](../index.md) / [static_str](index.md)*

---

# Module `static_str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StaticStr`](#staticstr) | struct | A buffer stored on the stack whose size is equal to the stack size of `String` The last byte is set to 0. |
| [`DISCRIMINANT_SIZE`](#discriminant_size) | const |  |

## Structs

### `StaticStr`

```rust
struct StaticStr {
    ptr: ptr::NonNull<u8>,
    len: usize,
    discriminant: [u8; 8],
}
```

*Defined in [`compact_str-0.9.0/src/repr/static_str.rs:11-16`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/static_str.rs#L11-L16)*

A buffer stored on the stack whose size is equal to the stack size of `String`
The last byte is set to 0.

#### Implementations

- <span id="staticstr-new"></span>`const fn new(text: &'static str) -> Self`

- <span id="staticstr-get-text"></span>`const fn get_text(&self) -> &'static str`

- <span id="staticstr-set-len"></span>`unsafe fn set_len(&mut self, len: usize)`

#### Trait Implementations

##### `impl Clone for StaticStr`

- <span id="staticstr-clone"></span>`fn clone(&self) -> StaticStr` — [`StaticStr`](#staticstr)

##### `impl Copy for StaticStr`

## Constants

### `DISCRIMINANT_SIZE`
```rust
const DISCRIMINANT_SIZE: usize = 8usize;
```

*Defined in [`compact_str-0.9.0/src/repr/static_str.rs:5`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/static_str.rs#L5)*

