*[ryu](../index.md) / [f2s](index.md)*

---

# Module `f2s`

## Structs

### `FloatingDecimal32`

```rust
struct FloatingDecimal32 {
    pub mantissa: u32,
    pub exponent: i32,
}
```

## Functions

### `f2d`

```rust
fn f2d(ieee_mantissa: u32, ieee_exponent: u32) -> FloatingDecimal32
```

## Constants

### `FLOAT_MANTISSA_BITS`

```rust
const FLOAT_MANTISSA_BITS: u32 = 23u32;
```

### `FLOAT_EXPONENT_BITS`

```rust
const FLOAT_EXPONENT_BITS: u32 = 8u32;
```

### `FLOAT_BIAS`

```rust
const FLOAT_BIAS: i32 = 127i32;
```

