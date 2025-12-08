*[backtrace](../index.md) / [capture](index.md)*

---

# Module `capture`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Backtrace`](#backtrace) | struct | Representation of an owned and self-contained backtrace. |
| [`TracePtr`](#traceptr) | struct |  |
| [`BacktraceFrame`](#backtraceframe) | struct | Captured version of a frame in a backtrace. |
| [`BacktraceSymbol`](#backtracesymbol) | struct | Captured version of a symbol in a backtrace. |
| [`Frame`](#frame) | enum |  |
| [`_assert_send_sync`](#_assert_send_sync) | fn |  |

## Structs

### `Backtrace`

```rust
struct Backtrace {
    frames: Box<[BacktraceFrame]>,
}
```

Representation of an owned and self-contained backtrace.

This structure can be used to capture a backtrace at various points in a
program and later used to inspect what the backtrace was at that time.

`Backtrace` supports pretty-printing of backtraces through its `Debug`
implementation.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

#### Implementations

- <span id="backtrace-new"></span>`fn new() -> Backtrace` — [`Backtrace`](../index.md)

- <span id="backtrace-new-unresolved"></span>`fn new_unresolved() -> Backtrace` — [`Backtrace`](../index.md)

- <span id="backtrace-create"></span>`fn create(ip: usize) -> Backtrace` — [`Backtrace`](../index.md)

- <span id="backtrace-frames"></span>`fn frames(&self) -> &[BacktraceFrame]` — [`BacktraceFrame`](../index.md)

- <span id="backtrace-resolve"></span>`fn resolve(&mut self)`

#### Trait Implementations

##### `impl Clone for Backtrace`

- <span id="backtrace-clone"></span>`fn clone(&self) -> Backtrace` — [`Backtrace`](../index.md)

##### `impl Debug for Backtrace`

- <span id="backtrace-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Backtrace`

- <span id="backtrace-default"></span>`fn default() -> Backtrace` — [`Backtrace`](../index.md)

### `TracePtr`

```rust
struct TracePtr(*mut core::ffi::c_void);
```

#### Implementations

- <span id="traceptr-into-void"></span>`fn into_void(self) -> *mut c_void`

#### Trait Implementations

##### `impl Clone for TracePtr`

- <span id="traceptr-clone"></span>`fn clone(&self) -> TracePtr` — [`TracePtr`](#traceptr)

##### `impl Copy for TracePtr`

##### `impl Send for TracePtr`

##### `impl Sync for TracePtr`

### `BacktraceFrame`

```rust
struct BacktraceFrame {
    frame: Frame,
    symbols: Option<Box<[BacktraceSymbol]>>,
}
```

Captured version of a frame in a backtrace.

This type is returned as a list from `Backtrace::frames` and represents one
stack frame in a captured backtrace.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

#### Implementations

- <span id="backtraceframe-ip"></span>`fn ip(&self) -> *mut c_void`

- <span id="backtraceframe-symbol-address"></span>`fn symbol_address(&self) -> *mut c_void`

- <span id="backtraceframe-module-base-address"></span>`fn module_base_address(&self) -> Option<*mut c_void>`

- <span id="backtraceframe-symbols"></span>`fn symbols(&self) -> &[BacktraceSymbol]` — [`BacktraceSymbol`](../index.md)

- <span id="backtraceframe-resolve"></span>`fn resolve(&mut self)`

#### Trait Implementations

##### `impl Clone for BacktraceFrame`

- <span id="backtraceframe-clone"></span>`fn clone(&self) -> BacktraceFrame` — [`BacktraceFrame`](../index.md)

##### `impl Debug for BacktraceFrame`

- <span id="backtraceframe-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BacktraceSymbol`

```rust
struct BacktraceSymbol {
    name: Option<Box<[u8]>>,
    addr: Option<TracePtr>,
    filename: Option<std::path::PathBuf>,
    lineno: Option<u32>,
    colno: Option<u32>,
}
```

Captured version of a symbol in a backtrace.

This type is returned as a list from `BacktraceFrame::symbols` and
represents the metadata for a symbol in a backtrace.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

#### Implementations

- <span id="backtracesymbol-name"></span>`fn name(&self) -> Option<SymbolName<'_>>` — [`SymbolName`](../index.md)

- <span id="backtracesymbol-addr"></span>`fn addr(&self) -> Option<*mut c_void>`

- <span id="backtracesymbol-filename"></span>`fn filename(&self) -> Option<&Path>`

- <span id="backtracesymbol-lineno"></span>`fn lineno(&self) -> Option<u32>`

- <span id="backtracesymbol-colno"></span>`fn colno(&self) -> Option<u32>`

#### Trait Implementations

##### `impl Clone for BacktraceSymbol`

- <span id="backtracesymbol-clone"></span>`fn clone(&self) -> BacktraceSymbol` — [`BacktraceSymbol`](../index.md)

##### `impl Debug for BacktraceSymbol`

- <span id="backtracesymbol-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Frame`

```rust
enum Frame {
    Raw(crate::Frame),
}
```

#### Implementations

- <span id="frame-ip"></span>`fn ip(&self) -> *mut c_void`

- <span id="frame-symbol-address"></span>`fn symbol_address(&self) -> *mut c_void`

- <span id="frame-module-base-address"></span>`fn module_base_address(&self) -> Option<*mut c_void>`

- <span id="frame-resolve-symbols"></span>`fn resolve_symbols(&self) -> Box<[BacktraceSymbol]>` — [`BacktraceSymbol`](../index.md)

#### Trait Implementations

##### `impl Clone for Frame`

- <span id="frame-clone"></span>`fn clone(&self) -> Frame` — [`Frame`](#frame)

## Functions

### `_assert_send_sync`

```rust
fn _assert_send_sync()
```

