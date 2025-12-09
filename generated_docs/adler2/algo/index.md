*[adler2](../index.md) / [algo](index.md)*

---

# Module `algo`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`U32X4`](#u32x4) | struct |  |

## Structs

### `U32X4`

```rust
struct U32X4([u32; 4]);
```

*Defined in [`adler2-2.0.1/src/algo.rs:112`](../../../.source_1765210505/adler2-2.0.1/src/algo.rs#L112)*

#### Implementations

- <span id="u32x4-from"></span>`fn from(bytes: &[u8]) -> Self`

#### Trait Implementations

##### `impl AddAssign for U32X4`

- <span id="u32x4-add-assign"></span>`fn add_assign(&mut self, other: Self)`

##### `impl Clone for U32X4`

- <span id="u32x4-clone"></span>`fn clone(&self) -> U32X4` — [`U32X4`](#u32x4)

##### `impl Copy for U32X4`

##### `impl MulAssign for U32X4`

- <span id="u32x4-mul-assign"></span>`fn mul_assign(&mut self, rhs: u32)`

##### `impl RemAssign for U32X4`

- <span id="u32x4-rem-assign"></span>`fn rem_assign(&mut self, quotient: u32)`

