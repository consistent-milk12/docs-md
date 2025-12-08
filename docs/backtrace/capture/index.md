*[backtrace](../index.md) / [capture](index.md)*

---

# Module `capture`

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

- `fn new() -> Backtrace` — [`Backtrace`](#backtrace)

- `fn new_unresolved() -> Backtrace` — [`Backtrace`](#backtrace)

- `fn create(ip: usize) -> Backtrace` — [`Backtrace`](#backtrace)

- `fn frames(self: &Self) -> &[BacktraceFrame]` — [`BacktraceFrame`](#backtraceframe)

- `fn resolve(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for Backtrace`

- `fn clone(self: &Self) -> Backtrace` — [`Backtrace`](#backtrace)

##### `impl Debug for Backtrace`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Backtrace`

- `fn default() -> Backtrace` — [`Backtrace`](#backtrace)

### `TracePtr`

```rust
struct TracePtr(*mut core::ffi::c_void);
```

#### Implementations

- `fn into_void(self: Self) -> *mut c_void`

#### Trait Implementations

##### `impl Clone for TracePtr`

- `fn clone(self: &Self) -> TracePtr` — [`TracePtr`](#traceptr)

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

- `fn ip(self: &Self) -> *mut c_void`

- `fn symbol_address(self: &Self) -> *mut c_void`

- `fn module_base_address(self: &Self) -> Option<*mut c_void>`

- `fn symbols(self: &Self) -> &[BacktraceSymbol]` — [`BacktraceSymbol`](#backtracesymbol)

- `fn resolve(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for BacktraceFrame`

- `fn clone(self: &Self) -> BacktraceFrame` — [`BacktraceFrame`](#backtraceframe)

##### `impl Debug for BacktraceFrame`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn name(self: &Self) -> Option<SymbolName<'_>>` — [`SymbolName`](../symbolize/index.md)

- `fn addr(self: &Self) -> Option<*mut c_void>`

- `fn filename(self: &Self) -> Option<&Path>`

- `fn lineno(self: &Self) -> Option<u32>`

- `fn colno(self: &Self) -> Option<u32>`

#### Trait Implementations

##### `impl Clone for BacktraceSymbol`

- `fn clone(self: &Self) -> BacktraceSymbol` — [`BacktraceSymbol`](#backtracesymbol)

##### `impl Debug for BacktraceSymbol`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Frame`

```rust
enum Frame {
    Raw(crate::Frame),
}
```

#### Implementations

- `fn ip(self: &Self) -> *mut c_void`

- `fn symbol_address(self: &Self) -> *mut c_void`

- `fn module_base_address(self: &Self) -> Option<*mut c_void>`

- `fn resolve_symbols(self: &Self) -> Box<[BacktraceSymbol]>` — [`BacktraceSymbol`](#backtracesymbol)

#### Trait Implementations

##### `impl Clone for Frame`

- `fn clone(self: &Self) -> Frame` — [`Frame`](#frame)

## Functions

### `_assert_send_sync`

```rust
fn _assert_send_sync()
```

