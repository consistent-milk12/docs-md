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

A buffer stored on the stack whose size is equal to the stack size of `String`

#### Implementations

- <span id="inlinebuffer-new"></span>`unsafe fn new(text: &str) -> Self`

- <span id="inlinebuffer-new-const"></span>`const fn new_const(text: &str) -> Self`

- <span id="inlinebuffer-empty"></span>`const fn empty() -> Self`

- <span id="inlinebuffer-set-len"></span>`unsafe fn set_len(&mut self, len: usize)`

