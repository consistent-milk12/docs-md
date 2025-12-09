*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [set_len_on_drop](index.md)*

---

# Module `set_len_on_drop`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SetLenOnDrop`](#setlenondrop) | struct |  |

## Structs

### `SetLenOnDrop<'a>`

```rust
struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/set_len_on_drop.rs:6-9`](../../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/set_len_on_drop.rs#L6-L9)*

#### Implementations

- <span id="setlenondrop-new"></span>`fn new(len: &'a mut usize) -> Self`

- <span id="setlenondrop-increment-len"></span>`fn increment_len(&mut self, increment: usize)`

#### Trait Implementations

##### `impl Drop for SetLenOnDrop<'_>`

- <span id="setlenondrop-drop"></span>`fn drop(&mut self)`

