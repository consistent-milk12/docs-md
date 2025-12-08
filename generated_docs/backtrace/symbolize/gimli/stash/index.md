*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [stash](index.md)*

---

# Module `stash`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Stash`](#stash) | struct | A simple arena allocator for byte buffers. |

## Structs

### `Stash`

```rust
struct Stash {
    buffers: core::cell::UnsafeCell<alloc::vec::Vec<alloc::vec::Vec<u8>>>,
    mmaps: core::cell::UnsafeCell<alloc::vec::Vec<self::mmap::Mmap>>,
}
```

A simple arena allocator for byte buffers.

#### Implementations

- <span id="stash-new"></span>`fn new() -> Stash` — [`Stash`](#stash)

- <span id="stash-allocate"></span>`fn allocate(&self, size: usize) -> &mut [u8]`

- <span id="stash-cache-mmap"></span>`fn cache_mmap(&self, map: Mmap) -> &[u8]` — [`Mmap`](../mmap/index.md)

