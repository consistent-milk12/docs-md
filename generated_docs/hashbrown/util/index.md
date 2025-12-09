*[hashbrown](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`cold_path`](#cold_path) | fn |  |
| [`likely`](#likely) | fn |  |
| [`unlikely`](#unlikely) | fn |  |
| [`invalid_mut`](#invalid_mut) | fn |  |

## Functions

### `cold_path`

```rust
fn cold_path()
```

*Defined in [`hashbrown-0.16.1/src/util.rs:8`](../../../.source_1765210505/hashbrown-0.16.1/src/util.rs#L8)*

### `likely`

```rust
fn likely(b: bool) -> bool
```

*Defined in [`hashbrown-0.16.1/src/util.rs:12-19`](../../../.source_1765210505/hashbrown-0.16.1/src/util.rs#L12-L19)*

### `unlikely`

```rust
fn unlikely(b: bool) -> bool
```

*Defined in [`hashbrown-0.16.1/src/util.rs:23-30`](../../../.source_1765210505/hashbrown-0.16.1/src/util.rs#L23-L30)*

### `invalid_mut`

```rust
fn invalid_mut<T>(addr: usize) -> *mut T
```

*Defined in [`hashbrown-0.16.1/src/util.rs:36-38`](../../../.source_1765210505/hashbrown-0.16.1/src/util.rs#L36-L38)*

