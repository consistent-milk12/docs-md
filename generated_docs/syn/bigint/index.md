*[syn](../index.md) / [bigint](index.md)*

---

# Module `bigint`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BigInt`](#bigint) | struct |  |

## Structs

### `BigInt`

```rust
struct BigInt {
    digits: Vec<u8>,
}
```

#### Implementations

- <span id="bigint-new"></span>`fn new() -> Self`

- <span id="bigint-to-string"></span>`fn to_string(&self) -> String`

- <span id="bigint-reserve-two-digits"></span>`fn reserve_two_digits(&mut self)`

#### Trait Implementations

##### `impl AddAssign for BigInt`

- <span id="bigint-add-assign"></span>`fn add_assign(&mut self, increment: u8)`

##### `impl MulAssign for BigInt`

- <span id="bigint-mul-assign"></span>`fn mul_assign(&mut self, base: u8)`

