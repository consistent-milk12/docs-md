*[itoa](../index.md) / [udiv128](index.md)*

---

# Module `udiv128`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`u128_mulhi`](#u128-mulhi) | fn | Multiply unsigned 128 bit integers, return upper 128 bits of the result |
| [`udivmod_1e19`](#udivmod-1e19) | fn | Divide `n` by 1e19 and return quotient and remainder |

## Functions

### `u128_mulhi`

```rust
fn u128_mulhi(x: u128, y: u128) -> u128
```

*Defined in [`itoa-1.0.15/src/udiv128.rs:7-22`](../../../.source_1765210505/itoa-1.0.15/src/udiv128.rs#L7-L22)*

Multiply unsigned 128 bit integers, return upper 128 bits of the result

### `udivmod_1e19`

```rust
fn udivmod_1e19(n: u128) -> (u128, u64)
```

*Defined in [`itoa-1.0.15/src/udiv128.rs:34-48`](../../../.source_1765210505/itoa-1.0.15/src/udiv128.rs#L34-L48)*

Divide `n` by 1e19 and return quotient and remainder

Integer division algorithm is based on the following paper:

  T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”
  in Proc. of the SIGPLAN94 Conference on Programming Language Design and
  Implementation, 1994, pp. 61–72


