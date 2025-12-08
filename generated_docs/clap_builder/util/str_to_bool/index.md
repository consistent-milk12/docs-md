*[clap_builder](../../index.md) / [util](../index.md) / [str_to_bool](index.md)*

---

# Module `str_to_bool`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`str_to_bool`](#str_to_bool) | fn | Converts a string literal representation of truth to true or false. |
| [`TRUE_LITERALS`](#true_literals) | const | True values are `y`, `yes`, `t`, `true`, `on`, and `1`. |
| [`FALSE_LITERALS`](#false_literals) | const | False values are `n`, `no`, `f`, `false`, `off`, and `0`. |

## Functions

### `str_to_bool`

```rust
fn str_to_bool(val: impl AsRef<str>) -> Option<bool>
```

Converts a string literal representation of truth to true or false.

`false` values are `n`, `no`, `f`, `false`, `off`, and `0` (case insensitive).

Any other value will be considered as `true`.

## Constants

### `TRUE_LITERALS`

```rust
const TRUE_LITERALS: [&str; 6];
```

True values are `y`, `yes`, `t`, `true`, `on`, and `1`.

### `FALSE_LITERALS`

```rust
const FALSE_LITERALS: [&str; 6];
```

False values are `n`, `no`, `f`, `false`, `off`, and `0`.

