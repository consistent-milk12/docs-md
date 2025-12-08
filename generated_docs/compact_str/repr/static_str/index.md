*[compact_str](../../index.md) / [repr](../index.md) / [static_str](index.md)*

---

# Module `static_str`

## Structs

### `StaticStr`

```rust
struct StaticStr {
    ptr: ptr::NonNull<u8>,
    len: usize,
    discriminant: [u8; 8],
}
```

A buffer stored on the stack whose size is equal to the stack size of `String`
The last byte is set to 0.

#### Implementations

- `const fn new(text: &'static str) -> Self`

- `const fn get_text(self: &Self) -> &'static str`

- `unsafe fn set_len(self: &mut Self, len: usize)`

#### Trait Implementations

##### `impl Clone for StaticStr`

- `fn clone(self: &Self) -> StaticStr` — [`StaticStr`](#staticstr)

##### `impl Copy for StaticStr`

## Constants

### `DISCRIMINANT_SIZE`

```rust
const DISCRIMINANT_SIZE: usize = 8usize;
```

