*[serde_json](../../index.md) / [value](../index.md) / [partial_eq](index.md)*

---

# Module `partial_eq`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`eq_i64`](#eq_i64) | fn |  |
| [`eq_u64`](#eq_u64) | fn |  |
| [`eq_f32`](#eq_f32) | fn |  |
| [`eq_f64`](#eq_f64) | fn |  |
| [`eq_bool`](#eq_bool) | fn |  |
| [`eq_str`](#eq_str) | fn |  |
| [`partialeq_numeric!`](#partialeq_numeric) | macro |  |

## Functions

### `eq_i64`

```rust
fn eq_i64(value: &super::Value, other: i64) -> bool
```

### `eq_u64`

```rust
fn eq_u64(value: &super::Value, other: u64) -> bool
```

### `eq_f32`

```rust
fn eq_f32(value: &super::Value, other: f32) -> bool
```

### `eq_f64`

```rust
fn eq_f64(value: &super::Value, other: f64) -> bool
```

### `eq_bool`

```rust
fn eq_bool(value: &super::Value, other: bool) -> bool
```

### `eq_str`

```rust
fn eq_str(value: &super::Value, other: &str) -> bool
```

## Macros

### `partialeq_numeric!`

