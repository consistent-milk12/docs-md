*[backtrace](../index.md) / [backtrace](index.md)*

---

# Module `backtrace`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`libunwind`](#libunwind) | mod | Backtrace support using libunwind/gcc_s/etc APIs. |
| [`Frame`](#frame) | struct | A trait representing one frame of a backtrace, yielded to the `trace` function of this crate. |
| [`trace`](#trace) | fn | Inspects the current call-stack, passing all active frames into the closure provided to calculate a stack trace. |
| [`trace_unsynchronized`](#trace-unsynchronized) | fn | Same as `trace`, only unsafe as it's unsynchronized. |

## Modules

- [`libunwind`](libunwind/index.md) — Backtrace support using libunwind/gcc_s/etc APIs.

## Structs

### `Frame`

```rust
struct Frame {
    inner: self::libunwind::Frame,
}
```

*Defined in [`backtrace-0.3.76/src/backtrace/mod.rs:76-78`](../../../.source_1765521767/backtrace-0.3.76/src/backtrace/mod.rs#L76-L78)*

A trait representing one frame of a backtrace, yielded to the `trace`
function of this crate.

The tracing function's closure will be yielded frames, and the frame is
virtually dispatched as the underlying implementation is not always known
until runtime.

#### Implementations

- <span id="frame-ip"></span>`fn ip(&self) -> *mut c_void`

  Returns the current instruction pointer of this frame.

  

  This is normally the next instruction to execute in the frame, but not

  all implementations list this with 100% accuracy (but it's generally

  pretty close).

  

  It is recommended to pass this value to `backtrace::resolve` to turn it

  into a symbol name.

- <span id="frame-sp"></span>`fn sp(&self) -> *mut c_void`

  Returns the current stack pointer of this frame.

  

  In the case that a backend cannot recover the stack pointer for this

  frame, a null pointer is returned.

- <span id="frame-symbol-address"></span>`fn symbol_address(&self) -> *mut c_void`

  Returns the starting symbol address of the frame of this function.

  

  This will attempt to rewind the instruction pointer returned by `ip` to

  the start of the function, returning that value. In some cases, however,

  backends will just return `ip` from this function.

  

  The returned value can sometimes be used if `backtrace::resolve` failed

  on the `ip` given above.

- <span id="frame-module-base-address"></span>`fn module_base_address(&self) -> Option<*mut c_void>`

  Returns the base address of the module to which the frame belongs.

#### Trait Implementations

##### `impl Any for Frame`

- <span id="frame-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Frame`

- <span id="frame-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Frame`

- <span id="frame-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Frame`

- <span id="frame-clone"></span>`fn clone(&self) -> Frame` — [`Frame`](#frame)

##### `impl CloneToUninit for Frame`

- <span id="frame-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Frame`

- <span id="frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Frame`

- <span id="frame-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Frame`

- <span id="frame-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Frame`

- <span id="frame-toowned-type-owned"></span>`type Owned = T`

- <span id="frame-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="frame-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Frame`

- <span id="frame-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frame-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Frame`

- <span id="frame-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frame-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `trace`

```rust
fn trace<F: FnMut(&Frame) -> bool>(cb: F)
```

*Defined in [`backtrace-0.3.76/src/backtrace/mod.rs:51-54`](../../../.source_1765521767/backtrace-0.3.76/src/backtrace/mod.rs#L51-L54)*

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

*Defined in [`backtrace-0.3.76/src/backtrace/mod.rs:65-67`](../../../.source_1765521767/backtrace-0.3.76/src/backtrace/mod.rs#L65-L67)*

Same as `trace`, only unsafe as it's unsynchronized.

This function does not have synchronization guarantees but is available
when the `std` feature of this crate isn't compiled in. See the `trace`
function for more documentation and examples.

# Panics

See information on `trace` for caveats on `cb` panicking.

