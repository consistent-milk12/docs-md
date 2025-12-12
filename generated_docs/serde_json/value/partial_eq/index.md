*[serde_json](../../index.md) / [value](../index.md) / [partial_eq](index.md)*

---

# Module `partial_eq`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`eq_i64`](#eq-i64) | fn |  |
| [`eq_u64`](#eq-u64) | fn |  |
| [`eq_f32`](#eq-f32) | fn |  |
| [`eq_f64`](#eq-f64) | fn |  |
| [`eq_bool`](#eq-bool) | fn |  |
| [`eq_str`](#eq-str) | fn |  |
| [`partialeq_numeric!`](#partialeq-numeric) | macro |  |

## Functions

### `eq_i64`

```rust
fn eq_i64(value: &super::Value, other: i64) -> bool
```

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:4-6`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L4-L6)*

### `eq_u64`

```rust
fn eq_u64(value: &super::Value, other: u64) -> bool
```

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:8-10`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L8-L10)*

### `eq_f32`

```rust
fn eq_f32(value: &super::Value, other: f32) -> bool
```

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:12-17`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L12-L17)*

### `eq_f64`

```rust
fn eq_f64(value: &super::Value, other: f64) -> bool
```

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:19-21`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L19-L21)*

### `eq_bool`

```rust
fn eq_bool(value: &super::Value, other: bool) -> bool
```

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:23-25`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L23-L25)*

### `eq_str`

```rust
fn eq_str(value: &super::Value, other: &str) -> bool
```

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:27-29`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L27-L29)*

## Macros

### `partialeq_numeric!`

*Defined in [`serde_json-1.0.145/src/value/partial_eq.rs:67-95`](../../../../.source_1765210505/serde_json-1.0.145/src/value/partial_eq.rs#L67-L95)*

