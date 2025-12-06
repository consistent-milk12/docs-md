*[rustix](../../index.md) / [not_implemented](../index.md) / [memory_allocation](index.md)*

---

# Module `memory_allocation`

Memory-allocation functions are out of scope for rustix.

It is possible to implement `malloc`, `free`, and similar functions in
Rust, however rustix itself is focused on syscall-like functions. This
module contains an incomplete list of such functions.

There are several allocator implementations for Rust; one of them is
[dlmalloc](#dlmalloc). For a rustix-based implementation, see [rustix-dlmalloc].
Another allocator implementation is [talc](#talc).




## Functions

### `malloc`

```rust
fn malloc()
```

See the [module comment](self).

### `realloc`

```rust
fn realloc()
```

See the [module comment](self).

### `calloc`

```rust
fn calloc()
```

See the [module comment](self).

### `free`

```rust
fn free()
```

See the [module comment](self).

### `posix_memalign`

```rust
fn posix_memalign()
```

See the [module comment](self).

### `aligned_alloc`

```rust
fn aligned_alloc()
```

See the [module comment](self).

### `malloc_usable_size`

```rust
fn malloc_usable_size()
```

See the [module comment](self).

