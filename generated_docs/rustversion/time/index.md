*[rustversion](../index.md) / [time](index.md)*

---

# Module `time`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`today`](#today) | fn |  |
| [`try_today`](#try-today) | fn |  |
| [`BASE`](#base) | const |  |
| [`BASE_YEAR`](#base-year) | const |  |
| [`BASE_MONTH`](#base-month) | const |  |
| [`CYCLE`](#cycle) | const |  |
| [`DAYS_BY_MONTH`](#days-by-month) | const |  |

## Functions

### `today`

```rust
fn today() -> crate::date::Date
```

*Defined in [`rustversion-1.0.22/src/time.rs:15-22`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L15-L22)*

### `try_today`

```rust
fn try_today() -> Option<crate::date::Date>
```

*Defined in [`rustversion-1.0.22/src/time.rs:24-51`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L24-L51)*

## Constants

### `BASE`
```rust
const BASE: u64 = 1_456_790_400u64;
```

*Defined in [`rustversion-1.0.22/src/time.rs:6`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L6)*

### `BASE_YEAR`
```rust
const BASE_YEAR: u16 = 2_016u16;
```

*Defined in [`rustversion-1.0.22/src/time.rs:7`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L7)*

### `BASE_MONTH`
```rust
const BASE_MONTH: u8 = 3u8;
```

*Defined in [`rustversion-1.0.22/src/time.rs:8`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L8)*

### `CYCLE`
```rust
const CYCLE: u64 = 1_461u64;
```

*Defined in [`rustversion-1.0.22/src/time.rs:11`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L11)*

### `DAYS_BY_MONTH`
```rust
const DAYS_BY_MONTH: [u8; 12];
```

*Defined in [`rustversion-1.0.22/src/time.rs:13`](../../../.source_1765521767/rustversion-1.0.22/src/time.rs#L13)*

