*[syn](../index.md) / [bigint](index.md)*

---

# Module `bigint`

## Structs

### `BigInt`

```rust
struct BigInt {
    digits: Vec<u8>,
}
```

#### Implementations

- `fn new() -> Self`

- `fn to_string(self: &Self) -> String`

- `fn reserve_two_digits(self: &mut Self)`

#### Trait Implementations

##### `impl AddAssign for BigInt`

- `fn add_assign(self: &mut Self, increment: u8)`

##### `impl MulAssign for BigInt`

- `fn mul_assign(self: &mut Self, base: u8)`

