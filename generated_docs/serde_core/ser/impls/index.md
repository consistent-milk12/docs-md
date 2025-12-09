*[serde_core](../../index.md) / [ser](../index.md) / [impls](index.md)*

---

# Module `impls`

## Contents

- [Functions](#functions)
  - [`format_u8`](#format_u8)
- [Constants](#constants)
  - [`DEC_DIGITS_LUT`](#dec_digits_lut)
- [Macros](#macros)
  - [`primitive_impl!`](#primitive_impl)
  - [`array_impls!`](#array_impls)
  - [`seq_impl!`](#seq_impl)
  - [`tuple_impls!`](#tuple_impls)
  - [`tuple_impl_body!`](#tuple_impl_body)
  - [`map_impl!`](#map_impl)
  - [`deref_impl!`](#deref_impl)
  - [`nonzero_integers!`](#nonzero_integers)
  - [`serialize_display_bounded_length!`](#serialize_display_bounded_length)
  - [`atomic_impl!`](#atomic_impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`format_u8`](#format_u8) | fn |  |
| [`DEC_DIGITS_LUT`](#dec_digits_lut) | const |  |
| [`primitive_impl!`](#primitive_impl) | macro |  |
| [`array_impls!`](#array_impls) | macro |  |
| [`seq_impl!`](#seq_impl) | macro |  |
| [`tuple_impls!`](#tuple_impls) | macro |  |
| [`tuple_impl_body!`](#tuple_impl_body) | macro |  |
| [`map_impl!`](#map_impl) | macro |  |
| [`deref_impl!`](#deref_impl) | macro |  |
| [`nonzero_integers!`](#nonzero_integers) | macro |  |
| [`serialize_display_bounded_length!`](#serialize_display_bounded_length) | macro | Serialize a value that implements `Display` as a string, when that string is statically known to never have more than a constant `MAX_LEN` bytes. |
| [`atomic_impl!`](#atomic_impl) | macro |  |

## Functions

### `format_u8`

```rust
fn format_u8(n: u8, out: &mut [u8]) -> usize
```

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:769-786`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L769-L786)*

## Constants

### `DEC_DIGITS_LUT`
```rust
const DEC_DIGITS_LUT: &[u8];
```

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:760-765`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L760-L765)*

## Macros

### `primitive_impl!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:7-19`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L7-L19)*

### `array_impls!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:143-164`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L143-L164)*

### `seq_impl!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:188-207`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L188-L207)*

### `tuple_impls!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:362-374`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L362-L374)*

### `tuple_impl_body!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:376-390`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L376-L390)*

### `map_impl!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:424-444`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L424-L444)*

### `deref_impl!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:460-476`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L460-L476)*

### `nonzero_integers!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:570-583`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L570-L583)*

### `serialize_display_bounded_length!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:726-733`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L726-L733)*

Serialize a value that implements `Display` as a string, when that string is
statically known to never have more than a constant `MAX_LEN` bytes.

Panics if the `Display` impl tries to write more than `MAX_LEN` bytes.

### `atomic_impl!`

*Defined in [`serde_core-1.0.228/src/ser/impls.rs:1010-1026`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impls.rs#L1010-L1026)*

