*[backtrace](../../../index.md) / [backtrace](../../index.md) / [libunwind](../index.md) / [uw](index.md)*

---

# Module `uw`

Unwind library interface used for backtraces

Note that dead code is allowed as here are just bindings
iOS doesn't use all of them it but adding more
platform-specific configs pollutes the code too much

## Contents

- [Enums](#enums)
  - [`_Unwind_Reason_Code`](#unwind-reason-code)
  - [`_Unwind_Context`](#unwind-context)
- [Functions](#functions)
  - [`_Unwind_Backtrace`](#unwind-backtrace)
  - [`_Unwind_GetIP`](#unwind-getip)
  - [`_Unwind_FindEnclosingFunction`](#unwind-findenclosingfunction)
  - [`get_sp`](#get-sp)
- [Type Aliases](#type-aliases)
  - [`_Unwind_Trace_Fn`](#unwind-trace-fn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`_Unwind_Reason_Code`](#unwind-reason-code) | enum |  |
| [`_Unwind_Context`](#unwind-context) | enum |  |
| [`_Unwind_Backtrace`](#unwind-backtrace) | fn |  |
| [`_Unwind_GetIP`](#unwind-getip) | fn |  |
| [`_Unwind_FindEnclosingFunction`](#unwind-findenclosingfunction) | fn |  |
| [`get_sp`](#get-sp) | fn |  |
| [`_Unwind_Trace_Fn`](#unwind-trace-fn) | type |  |

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

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:155-166`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L155-L166)*

### `_Unwind_Context`

```rust
enum _Unwind_Context {
}
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:168`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L168)*

## Functions

### `_Unwind_Backtrace`

```rust
unsafe fn _Unwind_Backtrace(trace: fn(*mut _Unwind_Context, *mut core::ffi::c_void) -> _Unwind_Reason_Code, trace_argument: *mut c_void) -> _Unwind_Reason_Code
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:174-177`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L174-L177)*

### `_Unwind_GetIP`

```rust
unsafe fn _Unwind_GetIP(ctx: *mut _Unwind_Context) -> libc::uintptr_t
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:192`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L192)*

### `_Unwind_FindEnclosingFunction`

```rust
unsafe fn _Unwind_FindEnclosingFunction(pc: *mut c_void) -> *mut c_void
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:193`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L193)*

### `get_sp`

```rust
unsafe fn get_sp(ctx: *mut _Unwind_Context) -> libc::uintptr_t
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:202`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L202)*

## Type Aliases

### `_Unwind_Trace_Fn`

```rust
type _Unwind_Trace_Fn = fn(*mut _Unwind_Context, *mut core::ffi::c_void) -> _Unwind_Reason_Code;
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:170-171`](../../../../../.source_1765521767/backtrace-0.3.76/src/backtrace/libunwind.rs#L170-L171)*

