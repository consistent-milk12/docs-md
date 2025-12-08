*[serde_core](../../index.md) / [ser](../index.md) / [impls](index.md)*

---

# Module `impls`

## Functions

### `format_u8`

```rust
fn format_u8(n: u8, out: &mut [u8]) -> usize
```

## Constants

### `DEC_DIGITS_LUT`

```rust
const DEC_DIGITS_LUT: &[u8];
```

## Macros

### `primitive_impl!`

### `array_impls!`

### `seq_impl!`

### `tuple_impls!`

### `tuple_impl_body!`

### `map_impl!`

### `deref_impl!`

### `nonzero_integers!`

### `serialize_display_bounded_length!`

Serialize a value that implements `Display` as a string, when that string is
statically known to never have more than a constant `MAX_LEN` bytes.

Panics if the `Display` impl tries to write more than `MAX_LEN` bytes.

### `atomic_impl!`

