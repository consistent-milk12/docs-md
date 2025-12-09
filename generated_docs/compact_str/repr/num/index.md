*[compact_str](../../index.md) / [repr](../index.md) / [num](index.md)*

---

# Module `num`

Implementations for efficiently converting a number into a [`Repr`](../index.md)

Adapted from the implementation in the `std` library at
<https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L188-L266>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NumChars`](#numchars) | trait | All of these `num_chars(...)` methods are kind of crazy, but they are necessary. |
| [`DEC_DIGITS_LUT`](#dec_digits_lut) | const |  |
| [`impl_IntoRepr!`](#impl_intorepr) | macro | Defines the implementation of [`IntoRepr`] for integer types |
| [`impl_NonZero_IntoRepr!`](#impl_nonzero_intorepr) | macro | Defines the implementation of [`IntoRepr`] for NonZero integer types |

## Traits

### `NumChars`

```rust
trait NumChars { ... }
```

All of these `num_chars(...)` methods are kind of crazy, but they are necessary.

An alternate way to calculate the number of digits in a value is to do:
```rust
let val = 42;
let num_digits = ((val as f32).log10().floor()) as usize + 1;
assert_eq!(num_digits, 2);
```
But there are two problems with this approach:
1. floating point math is slow
2. results are dependent on floating point precision, which is too inaccurate for larger values

For example, consider this relatively large value...

```rust
let val = 9999995;
let num_digits = ((val as f32).log10().floor()) as usize + 1;

// this is wrong! There are only 7 digits in this number!
assert_eq!(num_digits, 8);
```

you can use `f64` to get better precision, e.g.

```rust
let val = 9999995;
let num_digits = ((val as f64).log10().floor()) as usize + 1;

// the precision is enough to get the correct value
assert_eq!(num_digits, 7);
```

...but still not precise enough!

```rust
let val: u64 = 9999999999999999999;
let num_digits = ((val as f64).log10().floor()) as usize + 1;

// this is wrong! the number is only 19 digits but the formula returns 20
assert_eq!(num_digits, 20);
```

#### Required Methods

- `fn num_chars(val: Self) -> usize`

#### Implementors

- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Constants

### `DEC_DIGITS_LUT`

```rust
const DEC_DIGITS_LUT: &[u8];
```

## Macros

### `impl_IntoRepr!`

Defines the implementation of [`IntoRepr`](../traits/index.md) for integer types

### `impl_NonZero_IntoRepr!`

Defines the implementation of [`IntoRepr`](../traits/index.md) for NonZero integer types

