*[backtrace](../index.md) / [symbolize](index.md)*

---

# Module `symbolize`

## Contents

- [Modules](#modules)
  - [`gimli`](#gimli)
- [Structs](#structs)
  - [`Symbol`](#symbol)
  - [`SymbolName`](#symbolname)
- [Enums](#enums)
  - [`ResolveWhat`](#resolvewhat)
- [Functions](#functions)
  - [`resolve`](#resolve)
  - [`resolve_frame`](#resolve-frame)
  - [`adjust_ip`](#adjust-ip)
  - [`resolve_unsynchronized`](#resolve-unsynchronized)
  - [`resolve_frame_unsynchronized`](#resolve-frame-unsynchronized)
  - [`format_symbol_name`](#format-symbol-name)
  - [`clear_symbol_cache`](#clear-symbol-cache)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`gimli`](#gimli) | mod | Support for symbolication using the `gimli` crate on crates.io |
| [`Symbol`](#symbol) | struct | A trait representing the resolution of a symbol in a file. |
| [`SymbolName`](#symbolname) | struct | A wrapper around a symbol name to provide ergonomic accessors to the demangled name, the raw bytes, the raw string, etc. |
| [`ResolveWhat`](#resolvewhat) | enum |  |
| [`resolve`](#resolve) | fn | Resolve an address to a symbol, passing the symbol to the specified closure. |
| [`resolve_frame`](#resolve-frame) | fn | Resolve a previously captured frame to a symbol, passing the symbol to the specified closure. |
| [`adjust_ip`](#adjust-ip) | fn |  |
| [`resolve_unsynchronized`](#resolve-unsynchronized) | fn | Same as `resolve`, only unsafe as it's unsynchronized. |
| [`resolve_frame_unsynchronized`](#resolve-frame-unsynchronized) | fn | Same as `resolve_frame`, only unsafe as it's unsynchronized. |
| [`format_symbol_name`](#format-symbol-name) | fn |  |
| [`clear_symbol_cache`](#clear-symbol-cache) | fn | Attempt to reclaim that cached memory used to symbolicate addresses. |

## Modules

- [`gimli`](gimli/index.md) — Support for symbolication using the `gimli` crate on crates.io

## Structs

### `Symbol`

```rust
struct Symbol {
    inner: imp::Symbol<'static>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:190-195`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L190-L195)*

A trait representing the resolution of a symbol in a file.

This trait is yielded as a trait object to the closure given to the
`backtrace::resolve` function, and it is virtually dispatched as it's
unknown which implementation is behind it.

A symbol can give contextual information about a function, for example the
name, filename, line number, precise address, etc. Not all information is
always available in a symbol, however, so all methods return an `Option`.

#### Implementations

- <span id="symbol-name"></span>`fn name(&self) -> Option<SymbolName<'_>>` — [`SymbolName`](#symbolname)

  Returns the name of this function.

  

  The returned structure can be used to query various properties about the

  symbol name:

  

  * The `Display` implementation will print out the demangled symbol.

  * The raw `str` value of the symbol can be accessed (if it's valid

    utf-8).

  * The raw bytes for the symbol name can be accessed.

- <span id="symbol-addr"></span>`fn addr(&self) -> Option<*mut c_void>`

  Returns the starting address of this function.

- <span id="symbol-filename-raw"></span>`fn filename_raw(&self) -> Option<BytesOrWideString<'_>>` — [`BytesOrWideString`](../types/index.md#bytesorwidestring)

  Returns the raw filename as a slice. This is mainly useful for `no_std`

  environments.

- <span id="symbol-colno"></span>`fn colno(&self) -> Option<u32>`

  Returns the column number for where this symbol is currently executing.

  

  Only gimli currently provides a value here and even then only if `filename`

  returns `Some`, and so it is then consequently subject to similar caveats.

- <span id="symbol-lineno"></span>`fn lineno(&self) -> Option<u32>`

  Returns the line number for where this symbol is currently executing.

  

  This return value is typically `Some` if `filename` returns `Some`, and

  is consequently subject to similar caveats.

- <span id="symbol-filename"></span>`fn filename(&self) -> Option<&Path>`

  Returns the file name where this function was defined.

  

  This is currently only available when libbacktrace or gimli is being

  used (e.g. unix platforms other) and when a binary is compiled with

  debuginfo. If neither of these conditions is met then this will likely

  return `None`.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

#### Trait Implementations

##### `impl Any for Symbol`

- <span id="symbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Symbol`

- <span id="symbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Symbol`

- <span id="symbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Symbol`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Symbol`

- <span id="symbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Symbol`

- <span id="symbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Symbol`

- <span id="symbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Symbol`

- <span id="symbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolName<'a>`

```rust
struct SymbolName<'a> {
    bytes: &'a [u8],
    demangled: Option<rustc_demangle::Demangle<'a>>,
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:300-305`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L300-L305)*

A wrapper around a symbol name to provide ergonomic accessors to the
demangled name, the raw bytes, the raw string, etc.

#### Implementations

- <span id="symbolname-new"></span>`fn new(bytes: &'a [u8]) -> SymbolName<'a>` — [`SymbolName`](#symbolname)

  Creates a new symbol name from the raw underlying bytes.

- <span id="symbolname-as-str"></span>`fn as_str(&self) -> Option<&'a str>`

  Returns the raw (mangled) symbol name as a `str` if the symbol is valid utf-8.

  

  Use the `Display` implementation if you want the demangled version.

- <span id="symbolname-as-bytes"></span>`fn as_bytes(&self) -> &'a [u8]`

  Returns the raw symbol name as a list of bytes

#### Trait Implementations

##### `impl Any for SymbolName<'a>`

- <span id="symbolname-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolName<'a>`

- <span id="symbolname-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolName<'a>`

- <span id="symbolname-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SymbolName<'a>`

- <span id="symbolname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SymbolName<'a>`

- <span id="symbolname-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymbolName<'a>`

- <span id="symbolname-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SymbolName<'a>`

- <span id="symbolname-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for SymbolName<'a>`

- <span id="symbolname-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for SymbolName<'a>`

- <span id="symbolname-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolname-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolName<'a>`

- <span id="symbolname-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolname-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ResolveWhat<'a>`

```rust
enum ResolveWhat<'a> {
    Address(*mut core::ffi::c_void),
    Frame(&'a super::backtrace::Frame),
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:108-111`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L108-L111)*

#### Implementations

- <span id="resolvewhat-address-or-ip"></span>`fn address_or_ip(&self) -> *mut c_void`

#### Trait Implementations

##### `impl Any for ResolveWhat<'a>`

- <span id="resolvewhat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ResolveWhat<'a>`

- <span id="resolvewhat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ResolveWhat<'a>`

- <span id="resolvewhat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ResolveWhat<'a>`

- <span id="resolvewhat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ResolveWhat<'a>`

- <span id="resolvewhat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ResolveWhat<'a>`

- <span id="resolvewhat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resolvewhat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ResolveWhat<'a>`

- <span id="resolvewhat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resolvewhat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `resolve`

```rust
fn resolve<F: FnMut(&Symbol)>(addr: *mut core::ffi::c_void, cb: F)
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:61-64`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L61-L64)*

Resolve an address to a symbol, passing the symbol to the specified
closure.

This function will look up the given address in areas such as the local
symbol table, dynamic symbol table, or DWARF debug info (depending on the
activated implementation) to find symbols to yield.

The closure may not be called if resolution could not be performed, and it
also may be called more than once in the case of inlined functions.

Symbols yielded represent the execution at the specified `addr`, returning
file/line pairs for that address (if available).

Note that if you have a `Frame` then it's recommended to use the
`resolve_frame` function instead of this one.

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
        let ip = frame.ip();

        backtrace::resolve(ip, |symbol| {
            // ...
        });

        false // only look at the top frame
    });
}
```

### `resolve_frame`

```rust
fn resolve_frame<F: FnMut(&Symbol)>(frame: &super::backtrace::Frame, cb: F)
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:103-106`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L103-L106)*

Resolve a previously captured frame to a symbol, passing the symbol to the
specified closure.

This function performs the same function as `resolve` except that it takes a
`Frame` as an argument instead of an address. This can allow some platform
implementations of backtracing to provide more accurate symbol information
or information about inline frames for example. It's recommended to use this
if you can.

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
        backtrace::resolve_frame(frame, |symbol| {
            // ...
        });

        false // only look at the top frame
    });
}
```

### `adjust_ip`

```rust
fn adjust_ip(a: *mut core::ffi::c_void) -> *mut core::ffi::c_void
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:141-147`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L141-L147)*

### `resolve_unsynchronized`

```rust
unsafe fn resolve_unsynchronized<F>(addr: *mut core::ffi::c_void, cb: F)
where
    F: FnMut(&Symbol)
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:158-163`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L158-L163)*

Same as `resolve`, only unsafe as it's unsynchronized.

This function does not have synchronization guarantees but is available when
the `std` feature of this crate isn't compiled in. See the `resolve`
function for more documentation and examples.

# Panics

See information on `resolve` for caveats on `cb` panicking.

### `resolve_frame_unsynchronized`

```rust
unsafe fn resolve_frame_unsynchronized<F>(frame: &super::backtrace::Frame, cb: F)
where
    F: FnMut(&Symbol)
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:174-179`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L174-L179)*

Same as `resolve_frame`, only unsafe as it's unsynchronized.

This function does not have synchronization guarantees but is available
when the `std` feature of this crate isn't compiled in. See the
`resolve_frame` function for more documentation and examples.

# Panics

See information on `resolve_frame` for caveats on `cb` panicking.

### `format_symbol_name`

```rust
fn format_symbol_name(fmt: fn(&str, &mut fmt::Formatter<'_>) -> fmt::Result, bytes: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:344-366`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L344-L366)*

### `clear_symbol_cache`

```rust
fn clear_symbol_cache()
```

*Defined in [`backtrace-0.3.76/src/symbolize/mod.rs:426-431`](../../../.source_1765521767/backtrace-0.3.76/src/symbolize/mod.rs#L426-L431)*

Attempt to reclaim that cached memory used to symbolicate addresses.

This method will attempt to release any global data structures that have
otherwise been cached globally or in the thread which typically represent
parsed DWARF information or similar.

# Caveats

While this function is always available it doesn't actually do anything on
most implementations. Libraries like dbghelp or libbacktrace do not provide
facilities to deallocate state and manage the allocated memory. For now the
`std` feature of this crate is the only feature where this
function has any effect.

