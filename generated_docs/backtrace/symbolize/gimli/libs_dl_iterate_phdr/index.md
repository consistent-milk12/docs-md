*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [libs_dl_iterate_phdr](index.md)*

---

# Module `libs_dl_iterate_phdr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CallbackData`](#callbackdata) | struct |  |
| [`native_libraries`](#native_libraries) | fn |  |
| [`infer_current_exe`](#infer_current_exe) | fn |  |
| [`callback`](#callback) | fn | # Safety |

## Structs

### `CallbackData`

```rust
struct CallbackData {
    libs: alloc::vec::Vec<super::Library>,
    maps: Option<alloc::vec::Vec<parse_running_mmaps::MapsEntry>>,
}
```

## Functions

### `native_libraries`

```rust
fn native_libraries() -> alloc::vec::Vec<super::Library>
```

### `infer_current_exe`

```rust
fn infer_current_exe(maps: &Option<alloc::vec::Vec<parse_running_mmaps::MapsEntry>>, base_addr: usize) -> super::mystd::ffi::OsString
```

### `callback`

```rust
unsafe fn callback(info: *mut libc::dl_phdr_info, _size: libc::size_t, data: *mut libc::c_void) -> libc::c_int
```

# Safety
`info` must be a valid pointer.
`data` must be a valid pointer to `CallbackData`.

