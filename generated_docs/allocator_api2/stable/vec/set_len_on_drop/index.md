*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [set_len_on_drop](index.md)*

---

# Module `set_len_on_drop`

## Structs

### `SetLenOnDrop<'a>`

```rust
struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}
```

#### Implementations

- `fn new(len: &'a mut usize) -> Self`

- `fn increment_len(self: &mut Self, increment: usize)`

#### Trait Implementations

##### `impl Drop for SetLenOnDrop<'_>`

- `fn drop(self: &mut Self)`

