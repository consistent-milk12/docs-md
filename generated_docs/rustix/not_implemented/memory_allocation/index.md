*[rustix](../../index.md) / [not_implemented](../index.md) / [memory_allocation](index.md)*

---

# Module `memory_allocation`

Memory-allocation functions are out of scope for rustix.

It is possible to implement `malloc`, `free`, and similar functions in
Rust, however rustix itself is focused on syscall-like functions. This
module contains an incomplete list of such functions.

There are several allocator implementations for Rust; one of them is
[dlmalloc]. For a rustix-based implementation, see [rustix-dlmalloc].
Another allocator implementation is [talc].




## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`malloc`](#malloc) | fn | See the [module comment](self). |
| [`realloc`](#realloc) | fn | See the [module comment](self). |
| [`calloc`](#calloc) | fn | See the [module comment](self). |
| [`free`](#free) | fn | See the [module comment](self). |
| [`posix_memalign`](#posix-memalign) | fn | See the [module comment](self). |
| [`aligned_alloc`](#aligned-alloc) | fn | See the [module comment](self). |
| [`malloc_usable_size`](#malloc-usable-size) | fn | See the [module comment](self). |

## Functions

### `malloc`

```rust
fn malloc()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:30`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L30)*

See the [module comment](self).

### `realloc`

```rust
fn realloc()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:31`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L31)*

See the [module comment](self).

### `calloc`

```rust
fn calloc()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:32`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L32)*

See the [module comment](self).

### `free`

```rust
fn free()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:33`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L33)*

See the [module comment](self).

### `posix_memalign`

```rust
fn posix_memalign()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:34`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L34)*

See the [module comment](self).

### `aligned_alloc`

```rust
fn aligned_alloc()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:35`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L35)*

See the [module comment](self).

### `malloc_usable_size`

```rust
fn malloc_usable_size()
```

*Defined in [`rustix-1.1.2/src/not_implemented.rs:36`](../../../../.source_1765633015/rustix-1.1.2/src/not_implemented.rs#L36)*

See the [module comment](self).

