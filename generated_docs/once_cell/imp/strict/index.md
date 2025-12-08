*[once_cell](../../index.md) / [imp](../index.md) / [strict](index.md)*

---

# Module `strict`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`addr`](#addr) | fn |  |
| [`with_addr`](#with_addr) | fn |  |
| [`map_addr`](#map_addr) | fn |  |

## Functions

### `addr`

```rust
fn addr<T>(ptr: *mut T) -> usize
where
    T: Sized
```

### `with_addr`

```rust
fn with_addr<T>(ptr: *mut T, addr: usize) -> *mut T
where
    T: Sized
```

### `map_addr`

```rust
fn map_addr<T>(ptr: *mut T, f: impl FnOnce(usize) -> usize) -> *mut T
where
    T: Sized
```

