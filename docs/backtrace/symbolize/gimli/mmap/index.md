*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [mmap](index.md)*

---

# Module `mmap`

## Structs

### `Mmap`

```rust
struct Mmap {
    ptr: *mut libc::c_void,
    len: usize,
}
```

#### Implementations

- `unsafe fn map(file: &File, len: usize, offset: u64) -> Option<Mmap>` — [`Mmap`](#mmap)

#### Trait Implementations

##### `impl Deref for Mmap`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &[u8]`

##### `impl Drop for Mmap`

- `fn drop(self: &mut Self)`

##### `impl<P, T> Receiver for Mmap`

- `type Target = T`

