*[backtrace](../../../index.md) / [backtrace](../../index.md) / [libunwind](../index.md) / [uw](index.md)*

---

# Module `uw`

Unwind library interface used for backtraces

Note that dead code is allowed as here are just bindings
iOS doesn't use all of them it but adding more
platform-specific configs pollutes the code too much

## Contents

- [Enums](#enums)
  - [`_Unwind_Reason_Code`](#_unwind_reason_code)
  - [`_Unwind_Context`](#_unwind_context)
- [Functions](#functions)
  - [`_Unwind_Backtrace`](#_unwind_backtrace)
  - [`_Unwind_GetIP`](#_unwind_getip)
  - [`_Unwind_FindEnclosingFunction`](#_unwind_findenclosingfunction)
  - [`get_sp`](#get_sp)
- [Type Aliases](#type-aliases)
  - [`_Unwind_Trace_Fn`](#_unwind_trace_fn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`_Unwind_Reason_Code`](#_unwind_reason_code) | enum |  |
| [`_Unwind_Context`](#_unwind_context) | enum |  |
| [`_Unwind_Backtrace`](#_unwind_backtrace) | fn |  |
| [`_Unwind_GetIP`](#_unwind_getip) | fn |  |
| [`_Unwind_FindEnclosingFunction`](#_unwind_findenclosingfunction) | fn |  |
| [`get_sp`](#get_sp) | fn |  |
| [`_Unwind_Trace_Fn`](#_unwind_trace_fn) | type |  |

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

