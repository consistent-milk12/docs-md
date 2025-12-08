*[compact_str](../../index.md) / [repr](../index.md) / [inline](index.md)*

---

# Module `inline`

## Structs

### `InlineBuffer`

```rust
struct InlineBuffer([u8; 24]);
```

A buffer stored on the stack whose size is equal to the stack size of `String`

#### Implementations

- `unsafe fn new(text: &str) -> Self`

- `const fn new_const(text: &str) -> Self`

- `const fn empty() -> Self`

- `unsafe fn set_len(self: &mut Self, len: usize)`

