*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [stash](index.md)*

---

# Module `stash`

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

- `fn new() -> Stash` — [`Stash`](#stash)

- `fn allocate(self: &Self, size: usize) -> &mut [u8]`

- `fn cache_mmap(self: &Self, map: Mmap) -> &[u8]` — [`Mmap`](../mmap/index.md)

