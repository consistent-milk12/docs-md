*[backtrace](../../../index.md) / [backtrace](../../index.md) / [libunwind](../index.md) / [uw](index.md)*

---

# Module `uw`

Unwind library interface used for backtraces

Note that dead code is allowed as here are just bindings
iOS doesn't use all of them it but adding more
platform-specific configs pollutes the code too much

## Enums

### `_Unwind_Reason_Code`

```rust
enum _Unwind_Reason_Code {
    _URC_NO_REASON,
    _URC_FOREIGN_EXCEPTION_CAUGHT,
    _URC_FATAL_PHASE2_ERROR,
    _URC_FATAL_PHASE1_ERROR,
    _URC_NORMAL_STOP,
    _URC_END_OF_STACK,
    _URC_HANDLER_FOUND,
    _URC_INSTALL_CONTEXT,
    _URC_CONTINUE_UNWIND,
    _URC_FAILURE,
}
```

### `_Unwind_Context`

```rust
enum _Unwind_Context {
}
```

## Functions

### `_Unwind_Backtrace`

```rust
unsafe fn _Unwind_Backtrace(trace: fn(*mut _Unwind_Context, *mut core::ffi::c_void) -> _Unwind_Reason_Code, trace_argument: *mut c_void) -> _Unwind_Reason_Code
```

### `_Unwind_GetIP`

```rust
unsafe fn _Unwind_GetIP(ctx: *mut _Unwind_Context) -> libc::uintptr_t
```

### `_Unwind_FindEnclosingFunction`

```rust
unsafe fn _Unwind_FindEnclosingFunction(pc: *mut c_void) -> *mut c_void
```

### `get_sp`

```rust
unsafe fn get_sp(ctx: *mut _Unwind_Context) -> libc::uintptr_t
```

## Type Aliases

### `_Unwind_Trace_Fn`

```rust
type _Unwind_Trace_Fn = fn(*mut _Unwind_Context, *mut core::ffi::c_void) -> _Unwind_Reason_Code;
```

