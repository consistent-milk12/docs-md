*[backtrace](../index.md) / [symbolize](index.md)*

---

# Module `symbolize`

## Modules

- [`gimli`](gimli/index.md) - Support for symbolication using the `gimli` crate on crates.io

## Structs

### `Symbol`

```rust
struct Symbol {
    inner: imp::Symbol<'static>,
}
```

A trait representing the resolution of a symbol in a file.

This trait is yielded as a trait object to the closure given to the
`backtrace::resolve` function, and it is virtually dispatched as it's
unknown which implementation is behind it.

A symbol can give contextual information about a function, for example the
name, filename, line number, precise address, etc. Not all information is
always available in a symbol, however, so all methods return an `Option`.

#### Implementations

- `fn name(self: &Self) -> Option<SymbolName<'_>>` — [`SymbolName`](#symbolname)

- `fn addr(self: &Self) -> Option<*mut c_void>`

- `fn filename_raw(self: &Self) -> Option<BytesOrWideString<'_>>` — [`BytesOrWideString`](../types/index.md)

- `fn colno(self: &Self) -> Option<u32>`

- `fn lineno(self: &Self) -> Option<u32>`

- `fn filename(self: &Self) -> Option<&Path>`

#### Trait Implementations

##### `impl Debug for Symbol`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolName<'a>`

```rust
struct SymbolName<'a> {
    bytes: &'a [u8],
    demangled: Option<rustc_demangle::Demangle<'a>>,
}
```

A wrapper around a symbol name to provide ergonomic accessors to the
demangled name, the raw bytes, the raw string, etc.

#### Implementations

- `fn new(bytes: &'a [u8]) -> SymbolName<'a>` — [`SymbolName`](#symbolname)

- `fn as_str(self: &Self) -> Option<&'a str>`

- `fn as_bytes(self: &Self) -> &'a [u8]`

#### Trait Implementations

##### `impl<'a> Debug for SymbolName<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Display for SymbolName<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for SymbolName<'a>`

- `fn to_string(self: &Self) -> String`

## Enums

### `ResolveWhat<'a>`

```rust
enum ResolveWhat<'a> {
    Address(*mut core::ffi::c_void),
    Frame(&'a super::backtrace::Frame),
}
```

#### Implementations

- `fn address_or_ip(self: &Self) -> *mut c_void`

## Functions

### `resolve`

```rust
fn resolve<F: FnMut(&Symbol)>(addr: *mut core::ffi::c_void, cb: F)
```

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

### `resolve_unsynchronized`

```rust
unsafe fn resolve_unsynchronized<F>(addr: *mut core::ffi::c_void, cb: F)
where
    F: FnMut(&Symbol)
```

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

### `clear_symbol_cache`

```rust
fn clear_symbol_cache()
```

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

