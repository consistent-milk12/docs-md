*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [libs_dl_iterate_phdr](index.md)*

---

# Module `libs_dl_iterate_phdr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CallbackData`](#callbackdata) | struct |  |
| [`native_libraries`](#native-libraries) | fn |  |
| [`infer_current_exe`](#infer-current-exe) | fn |  |
| [`callback`](#callback) | fn | # Safety `info` must be a valid pointer. |

## Structs

### `CallbackData`

```rust
struct CallbackData {
    libs: alloc::vec::Vec<super::Library>,
    maps: Option<alloc::vec::Vec<parse_running_mmaps::MapsEntry>>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs:14-17`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs#L14-L17)*

## Functions

### `native_libraries`

```rust
fn native_libraries() -> alloc::vec::Vec<super::Library>
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs:18-30`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs#L18-L30)*

### `infer_current_exe`

```rust
fn infer_current_exe(maps: &Option<alloc::vec::Vec<parse_running_mmaps::MapsEntry>>, base_addr: usize) -> super::mystd::ffi::OsString
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs:32-49`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs#L32-L49)*

### `callback`

```rust
unsafe fn callback(info: *mut libc::dl_phdr_info, _size: libc::size_t, data: *mut libc::c_void) -> libc::c_int
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs:55-122`](../../../../../.source_1765521767/backtrace-0.3.76/src/symbolize/gimli/libs_dl_iterate_phdr.rs#L55-L122)*

# Safety
`info` must be a valid pointer.
`data` must be a valid pointer to `CallbackData`.

