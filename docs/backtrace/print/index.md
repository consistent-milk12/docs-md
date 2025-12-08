*[backtrace](../index.md) / [print](index.md)*

---

# Module `print`

## Structs

### `BacktraceFmt<'a, 'b>`

```rust
struct BacktraceFmt<'a, 'b> {
    fmt: &'a mut fmt::Formatter<'b>,
    frame_index: usize,
    format: PrintFmt,
    print_path: &'a mut dyn FnMut(&mut fmt::Formatter<'_>, super::BytesOrWideString<'_>) -> fmt::Result,
}
```

A formatter for backtraces.

This type can be used to print a backtrace regardless of where the backtrace
itself comes from. If you have a `Backtrace` type then its `Debug`
implementation already uses this printing format.

#### Implementations

- `fn new(fmt: &'a mut fmt::Formatter<'b>, format: PrintFmt, print_path: &'a mut dyn FnMut(&mut fmt::Formatter<'_>, BytesOrWideString<'_>) -> fmt::Result) -> Self` — [`PrintFmt`](../index.md), [`BytesOrWideString`](../index.md)

- `fn add_context(self: &mut Self) -> fmt::Result`

- `fn frame(self: &mut Self) -> BacktraceFrameFmt<'_, 'a, 'b>` — [`BacktraceFrameFmt`](../index.md)

- `fn finish(self: &mut Self) -> fmt::Result`

- `fn message(self: &mut Self, msg: &str) -> fmt::Result`

- `fn formatter(self: &mut Self) -> &mut fmt::Formatter<'b>`

### `BacktraceFrameFmt<'fmt, 'a, 'b>`

```rust
struct BacktraceFrameFmt<'fmt, 'a, 'b> {
    fmt: &'fmt mut BacktraceFmt<'a, 'b>,
    symbol_index: usize,
}
```

A formatter for just one frame of a backtrace.

This type is created by the `BacktraceFmt::frame` function.

#### Implementations

- `fn backtrace_frame(self: &mut Self, frame: &BacktraceFrame) -> fmt::Result` — [`BacktraceFrame`](../index.md)

- `fn backtrace_symbol(self: &mut Self, frame: &BacktraceFrame, symbol: &BacktraceSymbol) -> fmt::Result` — [`BacktraceFrame`](../index.md), [`BacktraceSymbol`](../index.md)

- `fn symbol(self: &mut Self, frame: &Frame, symbol: &super::Symbol) -> fmt::Result` — [`Frame`](../index.md), [`Symbol`](../index.md)

- `fn print_raw(self: &mut Self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>) -> fmt::Result` — [`SymbolName`](../index.md), [`BytesOrWideString`](../index.md)

- `fn print_raw_with_column(self: &mut Self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>, colno: Option<u32>) -> fmt::Result` — [`SymbolName`](../index.md), [`BytesOrWideString`](../index.md)

- `fn print_raw_generic(self: &mut Self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>, colno: Option<u32>) -> fmt::Result` — [`SymbolName`](../index.md), [`BytesOrWideString`](../index.md)

- `fn print_fileline(self: &mut Self, file: BytesOrWideString<'_>, line: u32, colno: Option<u32>) -> fmt::Result` — [`BytesOrWideString`](../index.md)

- `fn print_raw_fuchsia(self: &mut Self, frame_ip: *mut c_void) -> fmt::Result`

#### Trait Implementations

##### `impl Drop for BacktraceFrameFmt<'_, '_, '_>`

- `fn drop(self: &mut Self)`

## Enums

### `PrintFmt`

```rust
enum PrintFmt {
    Short,
    Full,
}
```

The styles of printing that we can print

#### Variants

- **`Short`**

  Prints a terser backtrace which ideally only contains relevant information

- **`Full`**

  Prints a backtrace that contains all possible information

#### Trait Implementations

##### `impl Clone for PrintFmt`

- `fn clone(self: &Self) -> PrintFmt` — [`PrintFmt`](../index.md)

##### `impl Copy for PrintFmt`

##### `impl Eq for PrintFmt`

##### `impl PartialEq for PrintFmt`

- `fn eq(self: &Self, other: &PrintFmt) -> bool` — [`PrintFmt`](../index.md)

##### `impl StructuralPartialEq for PrintFmt`

## Constants

### `HEX_WIDTH`

```rust
const HEX_WIDTH: usize = 18usize;
```

