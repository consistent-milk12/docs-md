*[once_cell](../../index.md) / [imp](../index.md) / [strict](index.md)*

---

# Module `strict`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`addr`](#addr) | fn |  |
| [`with_addr`](#with-addr) | fn |  |
| [`map_addr`](#map-addr) | fn |  |

## Functions

### `addr`

```rust
fn addr<T>(ptr: *mut T) -> usize
where
    T: Sized
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:248-256`](../../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L248-L256)*

### `with_addr`

```rust
fn with_addr<T>(ptr: *mut T, addr: usize) -> *mut T
where
    T: Sized
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:260-277`](../../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L260-L277)*

### `map_addr`

```rust
fn map_addr<T>(ptr: *mut T, f: impl FnOnce(usize) -> usize) -> *mut T
where
    T: Sized
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:281-286`](../../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L281-L286)*

