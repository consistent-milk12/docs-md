*[backtrace](../index.md) / [backtrace](index.md)*

---

# Module `backtrace`

## Modules

- [`libunwind`](libunwind/index.md) - Backtrace support using libunwind/gcc_s/etc APIs.

## Structs

### `Frame`

```rust
struct Frame {
    inner: self::libunwind::Frame,
}
```

A trait representing one frame of a backtrace, yielded to the `trace`
function of this crate.

The tracing function's closure will be yielded frames, and the frame is
virtually dispatched as the underlying implementation is not always known
until runtime.

#### Implementations

- `fn ip(self: &Self) -> *mut c_void`

- `fn sp(self: &Self) -> *mut c_void`

- `fn symbol_address(self: &Self) -> *mut c_void`

- `fn module_base_address(self: &Self) -> Option<*mut c_void>`

#### Trait Implementations

##### `impl Clone for Frame`

- `fn clone(self: &Self) -> Frame` — [`Frame`](#frame)

##### `impl Debug for Frame`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `trace`

```rust
fn trace<F: FnMut(&Frame) -> bool>(cb: F)
```

Inspects the current call-stack, passing all active frames into the closure
provided to calculate a stack trace.

This function is the workhorse of this library in calculating the stack
traces for a program. The given closure `cb` is yielded instances of a
`Frame` which represent information about that call frame on the stack. The
closure is yielded frames in a top-down fashion (most recently called
functions first).

The closure's return value is an indication of whether the backtrace should
continue. A return value of `false` will terminate the backtrace and return
immediately.

Once a `Frame` is acquired you will likely want to call `backtrace::resolve`
to convert the `ip` (instruction pointer) or symbol address to a `Symbol`
through which the name and/or filename/line number can be learned.

Note that this is a relatively low-level function and if you'd like to, for
example, capture a backtrace to be inspected later, then the `Backtrace`
type may be more appropriate.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

# Panics

This function strives to never panic, but if the `cb` provided panics then
some platforms will force a double panic to abort the process. Some
platforms use a C library which internally uses callbacks which cannot be
unwound through, so panicking from `cb` may trigger a process abort.

# Example

```rust
extern crate backtrace;

fn main() {
    backtrace::trace(|frame| {
        // ...

        true // continue the backtrace
    });
}
```

### `trace_unsynchronized`

```rust
unsafe fn trace_unsynchronized<F: FnMut(&Frame) -> bool>(cb: F)
```

Same as `trace`, only unsafe as it's unsynchronized.

This function does not have synchronization guarantees but is available
when the `std` feature of this crate isn't compiled in. See the `trace`
function for more documentation and examples.

# Panics

See information on `trace` for caveats on `cb` panicking.

