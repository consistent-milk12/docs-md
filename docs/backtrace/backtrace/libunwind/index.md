*[backtrace](../../index.md) / [backtrace](../index.md) / [libunwind](index.md)*

---

# Module `libunwind`

Backtrace support using libunwind/gcc_s/etc APIs.

This module contains the ability to unwind the stack using libunwind-style
APIs. Note that there's a whole bunch of implementations of the
libunwind-like API, and this is just trying to be compatible with most of
them all at once instead of being picky.

The libunwind API is powered by `_Unwind_Backtrace` and is in practice very
reliable at generating a backtrace. It's not entirely clear how it does it
(frame pointers? eh_frame info? both?) but it seems to work!

Most of the complexity of this module is handling the various platform
differences across libunwind implementations. Otherwise this is a pretty
straightforward Rust binding to the libunwind APIs.

This is the default unwinding API for all non-Windows platforms currently.

## Modules

- [`uw`](uw/index.md) - Unwind library interface used for backtraces

## Structs

### `Bomb`

```rust
struct Bomb {
    enabled: bool,
}
```

#### Trait Implementations

##### `impl Drop for Bomb`

- `fn drop(self: &mut Self)`

## Enums

### `Frame`

```rust
enum Frame {
    Raw(*mut uw::_Unwind_Context),
    Cloned {
        ip: *mut core::ffi::c_void,
        sp: *mut core::ffi::c_void,
        symbol_address: *mut core::ffi::c_void,
    },
}
```

#### Implementations

- `fn ip(self: &Self) -> *mut c_void`

- `fn sp(self: &Self) -> *mut c_void`

- `fn symbol_address(self: &Self) -> *mut c_void`

- `fn module_base_address(self: &Self) -> Option<*mut c_void>`

#### Trait Implementations

##### `impl Clone for Frame`

- `fn clone(self: &Self) -> Frame` — [`Frame`](#frame)

##### `impl Send for Frame`

##### `impl Sync for Frame`

## Functions

### `trace`

```rust
unsafe fn trace(cb: &mut dyn FnMut(&super::Frame) -> bool)
```

