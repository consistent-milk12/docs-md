*[unicode_ident](../index.md) / [tables](index.md)*

---

# Module `tables`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Align8`](#align8) | struct |  |
| [`Align64`](#align64) | struct |  |
| [`UNICODE_VERSION`](#unicode-version) | const |  |
| [`ASCII_START`](#ascii-start) | const |  |
| [`ASCII_CONTINUE`](#ascii-continue) | const |  |
| [`CHUNK`](#chunk) | const |  |

## Structs

### `Align8<T>`

```rust
struct Align8<T>(T);
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:8`](../../../.source_1765210505/unicode-ident-1.0.22/src/tables.rs#L8)*

### `Align64<T>`

```rust
struct Align64<T>(T);
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:10`](../../../.source_1765210505/unicode-ident-1.0.22/src/tables.rs#L10)*

## Constants

### `UNICODE_VERSION`
```rust
const UNICODE_VERSION: (u8, u8, u8);
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:12`](../../../.source_1765210505/unicode-ident-1.0.22/src/tables.rs#L12)*

### `ASCII_START`
```rust
const ASCII_START: u128 = 10_633_823_810_298_881_996_379_053_697_534_001_152u128;
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:14`](../../../.source_1765210505/unicode-ident-1.0.22/src/tables.rs#L14)*

### `ASCII_CONTINUE`
```rust
const ASCII_CONTINUE: u128 = 10_633_823_849_912_963_253_799_171_395_480_977_408u128;
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:15`](../../../.source_1765210505/unicode-ident-1.0.22/src/tables.rs#L15)*

### `CHUNK`
```rust
const CHUNK: usize = 64usize;
```

*Defined in [`unicode-ident-1.0.22/src/tables.rs:17`](../../../.source_1765210505/unicode-ident-1.0.22/src/tables.rs#L17)*

