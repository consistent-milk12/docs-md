*[hashbrown](../../../index.md) / [raw](../../index.md) / [alloc](../index.md) / [inner](index.md)*

---

# Module `inner`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`do_alloc`](#do_alloc) | fn |  |

## Functions

### `do_alloc`

```rust
fn do_alloc<A: Allocator>(alloc: &A, layout: crate::alloc::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, ()>
```

