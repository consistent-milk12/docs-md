*[backtrace](../index.md) / [print](index.md)*

---

# Module `print`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BacktraceFmt`](#backtracefmt) | struct | A formatter for backtraces. |
| [`BacktraceFrameFmt`](#backtraceframefmt) | struct | A formatter for just one frame of a backtrace. |
| [`PrintFmt`](#printfmt) | enum | The styles of printing that we can print |
| [`HEX_WIDTH`](#hex-width) | const |  |

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

*Defined in [`backtrace-0.3.76/src/print.rs:17-23`](../../../.source_1765210505/backtrace-0.3.76/src/print.rs#L17-L23)*

A formatter for backtraces.

This type can be used to print a backtrace regardless of where the backtrace
itself comes from. If you have a `Backtrace` type then its `Debug`
implementation already uses this printing format.

#### Implementations

- <span id="backtracefmt-new"></span>`fn new(fmt: &'a mut fmt::Formatter<'b>, format: PrintFmt, print_path: &'a mut dyn FnMut(&mut fmt::Formatter<'_>, BytesOrWideString<'_>) -> fmt::Result) -> Self` — [`PrintFmt`](#printfmt), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtracefmt-add-context"></span>`fn add_context(&mut self) -> fmt::Result`

- <span id="backtracefmt-frame"></span>`fn frame(&mut self) -> BacktraceFrameFmt<'_, 'a, 'b>` — [`BacktraceFrameFmt`](#backtraceframefmt)

- <span id="backtracefmt-finish"></span>`fn finish(&mut self) -> fmt::Result`

- <span id="backtracefmt-message"></span>`fn message(&mut self, msg: &str) -> fmt::Result`

- <span id="backtracefmt-formatter"></span>`fn formatter(&mut self) -> &mut fmt::Formatter<'b>`

### `BacktraceFrameFmt<'fmt, 'a, 'b>`

```rust
struct BacktraceFrameFmt<'fmt, 'a, 'b> {
    fmt: &'fmt mut BacktraceFmt<'a, 'b>,
    symbol_index: usize,
}
```

*Defined in [`backtrace-0.3.76/src/print.rs:111-114`](../../../.source_1765210505/backtrace-0.3.76/src/print.rs#L111-L114)*

A formatter for just one frame of a backtrace.

This type is created by the `BacktraceFmt::frame` function.

#### Implementations

- <span id="backtraceframefmt-backtrace-frame"></span>`fn backtrace_frame(&mut self, frame: &BacktraceFrame) -> fmt::Result` — [`BacktraceFrame`](../capture/index.md#backtraceframe)

- <span id="backtraceframefmt-backtrace-symbol"></span>`fn backtrace_symbol(&mut self, frame: &BacktraceFrame, symbol: &BacktraceSymbol) -> fmt::Result` — [`BacktraceFrame`](../capture/index.md#backtraceframe), [`BacktraceSymbol`](../capture/index.md#backtracesymbol)

- <span id="backtraceframefmt-symbol"></span>`fn symbol(&mut self, frame: &Frame, symbol: &super::Symbol) -> fmt::Result` — [`Frame`](../backtrace/index.md#frame), [`Symbol`](../symbolize/index.md#symbol)

- <span id="backtraceframefmt-print-raw"></span>`fn print_raw(&mut self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>) -> fmt::Result` — [`SymbolName`](../symbolize/index.md#symbolname), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtraceframefmt-print-raw-with-column"></span>`fn print_raw_with_column(&mut self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>, colno: Option<u32>) -> fmt::Result` — [`SymbolName`](../symbolize/index.md#symbolname), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtraceframefmt-print-raw-generic"></span>`fn print_raw_generic(&mut self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>, colno: Option<u32>) -> fmt::Result` — [`SymbolName`](../symbolize/index.md#symbolname), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtraceframefmt-print-fileline"></span>`fn print_fileline(&mut self, file: BytesOrWideString<'_>, line: u32, colno: Option<u32>) -> fmt::Result` — [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtraceframefmt-print-raw-fuchsia"></span>`fn print_raw_fuchsia(&mut self, frame_ip: *mut c_void) -> fmt::Result`

#### Trait Implementations

##### `impl Drop for BacktraceFrameFmt<'_, '_, '_>`

- <span id="backtraceframefmt-drop"></span>`fn drop(&mut self)`

## Enums

### `PrintFmt`

```rust
enum PrintFmt {
    Short,
    Full,
}
```

*Defined in [`backtrace-0.3.76/src/print.rs:28-33`](../../../.source_1765210505/backtrace-0.3.76/src/print.rs#L28-L33)*

The styles of printing that we can print

#### Variants

- **`Short`**

  Prints a terser backtrace which ideally only contains relevant information

- **`Full`**

  Prints a backtrace that contains all possible information

#### Trait Implementations

##### `impl Clone for PrintFmt`

- <span id="printfmt-clone"></span>`fn clone(&self) -> PrintFmt` — [`PrintFmt`](#printfmt)

##### `impl Copy for PrintFmt`

##### `impl Eq for PrintFmt`

##### `impl PartialEq for PrintFmt`

- <span id="printfmt-eq"></span>`fn eq(&self, other: &PrintFmt) -> bool` — [`PrintFmt`](#printfmt)

##### `impl StructuralPartialEq for PrintFmt`

## Constants

### `HEX_WIDTH`
```rust
const HEX_WIDTH: usize = 18usize;
```

*Defined in [`backtrace-0.3.76/src/print.rs:7`](../../../.source_1765210505/backtrace-0.3.76/src/print.rs#L7)*

