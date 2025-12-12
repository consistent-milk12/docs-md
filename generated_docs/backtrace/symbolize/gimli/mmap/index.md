*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [mmap](index.md)*

---

# Module `mmap`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Mmap`](#mmap) | struct |  |

## Structs

### `Mmap`

```rust
struct Mmap {
    ptr: *mut libc::c_void,
    len: usize,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/mmap_unix.rs:12-15`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/mmap_unix.rs#L12-L15)*

#### Implementations

- <span id="mmap-map"></span>`unsafe fn map(file: &File, len: usize, offset: u64) -> Option<Mmap>` — [`Mmap`](#mmap)

#### Trait Implementations

##### `impl Deref for Mmap`

- <span id="mmap-deref-type-target"></span>`type Target = [u8]`

- <span id="mmap-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Drop for Mmap`

- <span id="mmap-drop"></span>`fn drop(&mut self)`

##### `impl Receiver for Mmap`

- <span id="mmap-receiver-type-target"></span>`type Target = T`

