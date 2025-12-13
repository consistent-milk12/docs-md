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

*Defined in [`backtrace-0.3.76/src/print.rs:17-23`](../../../.source_1765521767/backtrace-0.3.76/src/print.rs#L17-L23)*

A formatter for backtraces.

This type can be used to print a backtrace regardless of where the backtrace
itself comes from. If you have a `Backtrace` type then its `Debug`
implementation already uses this printing format.

#### Implementations

- <span id="backtracefmt-new"></span>`fn new(fmt: &'a mut fmt::Formatter<'b>, format: PrintFmt, print_path: &'a mut dyn FnMut(&mut fmt::Formatter<'_>, BytesOrWideString<'_>) -> fmt::Result) -> Self` — [`PrintFmt`](#printfmt), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

  Create a new `BacktraceFmt` which will write output to the provided

  `fmt`.

  

  The `format` argument will control the style in which the backtrace is

  printed, and the `print_path` argument will be used to print the

  `BytesOrWideString` instances of filenames. This type itself doesn't do

  any printing of filenames, but this callback is required to do so.

- <span id="backtracefmt-add-context"></span>`fn add_context(&mut self) -> fmt::Result`

  Prints a preamble for the backtrace about to be printed.

  

  This is required on some platforms for backtraces to be fully

  symbolicated later, and otherwise this should just be the first method

  you call after creating a `BacktraceFmt`.

- <span id="backtracefmt-frame"></span>`fn frame(&mut self) -> BacktraceFrameFmt<'_, 'a, 'b>` — [`BacktraceFrameFmt`](#backtraceframefmt)

  Adds a frame to the backtrace output.

  

  This commit returns an RAII instance of a `BacktraceFrameFmt` which can be used

  to actually print a frame, and on destruction it will increment the

  frame counter.

- <span id="backtracefmt-finish"></span>`fn finish(&mut self) -> fmt::Result`

  Completes the backtrace output.

  

  This is currently a no-op but is added for future compatibility with

  backtrace formats.

- <span id="backtracefmt-message"></span>`fn message(&mut self, msg: &str) -> fmt::Result`

  Inserts a message in the backtrace output.

  

  This allows information to be inserted between frames,

  and won't increment the `frame_index` unlike the `frame`

  method.

- <span id="backtracefmt-formatter"></span>`fn formatter(&mut self) -> &mut fmt::Formatter<'b>`

  Return the inner formatter.

  

  This is used for writing custom information between frames with `write!` and `writeln!`,

  and won't increment the `frame_index` unlike the `frame` method.

#### Trait Implementations

##### `impl Any for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backtracefmt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BacktraceFmt<'a, 'b>`

- <span id="backtracefmt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backtracefmt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BacktraceFrameFmt<'fmt, 'a, 'b>`

```rust
struct BacktraceFrameFmt<'fmt, 'a, 'b> {
    fmt: &'fmt mut BacktraceFmt<'a, 'b>,
    symbol_index: usize,
}
```

*Defined in [`backtrace-0.3.76/src/print.rs:111-114`](../../../.source_1765521767/backtrace-0.3.76/src/print.rs#L111-L114)*

A formatter for just one frame of a backtrace.

This type is created by the `BacktraceFmt::frame` function.

#### Implementations

- <span id="backtraceframefmt-backtrace-frame"></span>`fn backtrace_frame(&mut self, frame: &BacktraceFrame) -> fmt::Result` — [`BacktraceFrame`](../capture/index.md#backtraceframe)

  Prints a `BacktraceFrame` with this frame formatter.

  

  This will recursively print all `BacktraceSymbol` instances within the

  `BacktraceFrame`.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtraceframefmt-backtrace-symbol"></span>`fn backtrace_symbol(&mut self, frame: &BacktraceFrame, symbol: &BacktraceSymbol) -> fmt::Result` — [`BacktraceFrame`](../capture/index.md#backtraceframe), [`BacktraceSymbol`](../capture/index.md#backtracesymbol)

  Prints a `BacktraceSymbol` within a `BacktraceFrame`.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtraceframefmt-symbol"></span>`fn symbol(&mut self, frame: &Frame, symbol: &super::Symbol) -> fmt::Result` — [`Frame`](../backtrace/index.md#frame), [`Symbol`](../symbolize/index.md#symbol)

  Prints a raw traced `Frame` and `Symbol`, typically from within the raw

  callbacks of this crate.

- <span id="backtraceframefmt-print-raw"></span>`fn print_raw(&mut self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>) -> fmt::Result` — [`SymbolName`](../symbolize/index.md#symbolname), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

  Adds a raw frame to the backtrace output.

  

  This method, unlike the previous, takes the raw arguments in case

  they're being source from different locations. Note that this may be

  called multiple times for one frame.

- <span id="backtraceframefmt-print-raw-with-column"></span>`fn print_raw_with_column(&mut self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>, colno: Option<u32>) -> fmt::Result` — [`SymbolName`](../symbolize/index.md#symbolname), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

  Adds a raw frame to the backtrace output, including column information.

  

  This method, like the previous, takes the raw arguments in case

  they're being source from different locations. Note that this may be

  called multiple times for one frame.

- <span id="backtraceframefmt-print-raw-generic"></span>`fn print_raw_generic(&mut self, frame_ip: *mut c_void, symbol_name: Option<SymbolName<'_>>, filename: Option<BytesOrWideString<'_>>, lineno: Option<u32>, colno: Option<u32>) -> fmt::Result` — [`SymbolName`](../symbolize/index.md#symbolname), [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtraceframefmt-print-fileline"></span>`fn print_fileline(&mut self, file: BytesOrWideString<'_>, line: u32, colno: Option<u32>) -> fmt::Result` — [`BytesOrWideString`](../types/index.md#bytesorwidestring)

- <span id="backtraceframefmt-print-raw-fuchsia"></span>`fn print_raw_fuchsia(&mut self, frame_ip: *mut c_void) -> fmt::Result`

#### Trait Implementations

##### `impl Any for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for BacktraceFrameFmt<'_, '_, '_>`

- <span id="backtraceframefmt-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backtraceframefmt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BacktraceFrameFmt<'fmt, 'a, 'b>`

- <span id="backtraceframefmt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backtraceframefmt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `PrintFmt`

```rust
enum PrintFmt {
    Short,
    Full,
}
```

*Defined in [`backtrace-0.3.76/src/print.rs:28-33`](../../../.source_1765521767/backtrace-0.3.76/src/print.rs#L28-L33)*

The styles of printing that we can print

#### Variants

- **`Short`**

  Prints a terser backtrace which ideally only contains relevant information

- **`Full`**

  Prints a backtrace that contains all possible information

#### Trait Implementations

##### `impl Any for PrintFmt`

- <span id="printfmt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrintFmt`

- <span id="printfmt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrintFmt`

- <span id="printfmt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PrintFmt`

- <span id="printfmt-clone"></span>`fn clone(&self) -> PrintFmt` — [`PrintFmt`](#printfmt)

##### `impl CloneToUninit for PrintFmt`

- <span id="printfmt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PrintFmt`

##### `impl Eq for PrintFmt`

##### `impl<T> From for PrintFmt`

- <span id="printfmt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PrintFmt`

- <span id="printfmt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PrintFmt`

- <span id="printfmt-partialeq-eq"></span>`fn eq(&self, other: &PrintFmt) -> bool` — [`PrintFmt`](#printfmt)

##### `impl StructuralPartialEq for PrintFmt`

##### `impl ToOwned for PrintFmt`

- <span id="printfmt-toowned-type-owned"></span>`type Owned = T`

- <span id="printfmt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="printfmt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PrintFmt`

- <span id="printfmt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="printfmt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrintFmt`

- <span id="printfmt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="printfmt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `HEX_WIDTH`
```rust
const HEX_WIDTH: usize = 18usize;
```

*Defined in [`backtrace-0.3.76/src/print.rs:7`](../../../.source_1765521767/backtrace-0.3.76/src/print.rs#L7)*

