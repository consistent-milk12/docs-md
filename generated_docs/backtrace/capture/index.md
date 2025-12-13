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
| [`_assert_send_sync`](#assert-send-sync) | fn |  |

## Structs

### `Backtrace`

```rust
struct Backtrace {
    frames: Box<[BacktraceFrame]>,
}
```

*Defined in [`backtrace-0.3.76/src/capture.rs:29-32`](../../../.source_1765633015/backtrace-0.3.76/src/capture.rs#L29-L32)*

Representation of an owned and self-contained backtrace.

This structure can be used to capture a backtrace at various points in a
program and later used to inspect what the backtrace was at that time.

`Backtrace` supports pretty-printing of backtraces through its `Debug`
implementation.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

#### Implementations

- <span id="backtrace-new"></span>`fn new() -> Backtrace` — [`Backtrace`](#backtrace)

  Captures a backtrace at the callsite of this function, returning an

  owned representation.

  

  This function is useful for representing a backtrace as an object in

  Rust. This returned value can be sent across threads and printed

  elsewhere, and the purpose of this value is to be entirely self

  contained.

  

  Note that on some platforms acquiring a full backtrace and resolving it

  can be extremely expensive. If the cost is too much for your application

  it's recommended to instead use `Backtrace::new_unresolved()` which

  avoids the symbol resolution step (which typically takes the longest)

  and allows deferring that to a later date.

  

  # Examples

  

  ```rust

  use backtrace::Backtrace;

  

  let current_backtrace = Backtrace::new();

  ```

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtrace-new-unresolved"></span>`fn new_unresolved() -> Backtrace` — [`Backtrace`](#backtrace)

  Similar to `new` except that this does not resolve any symbols, this

  simply captures the backtrace as a list of addresses.

  

  At a later time the `resolve` function can be called to resolve this

  backtrace's symbols into readable names. This function exists because

  the resolution process can sometimes take a significant amount of time

  whereas any one backtrace may only be rarely printed.

  

  # Examples

  

  ```rust

  use backtrace::Backtrace;

  

  let mut current_backtrace = Backtrace::new_unresolved();

  println!("{current_backtrace:?}"); // no symbol names

  current_backtrace.resolve();

  println!("{current_backtrace:?}"); // symbol names now present

  ```

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtrace-create"></span>`fn create(ip: usize) -> Backtrace` — [`Backtrace`](#backtrace)

- <span id="backtrace-frames"></span>`fn frames(&self) -> &[BacktraceFrame]` — [`BacktraceFrame`](#backtraceframe)

  Returns the frames from when this backtrace was captured.

  

  The first entry of this slice is likely the function `Backtrace::new`,

  and the last frame is likely something about how this thread or the main

  function started.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtrace-resolve"></span>`fn resolve(&mut self)`

  If this backtrace was created from `new_unresolved` then this function

  will resolve all addresses in the backtrace to their symbolic names.

  

  If this backtrace has been previously resolved or was created through

  `new`, this function does nothing.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

#### Trait Implementations

##### `impl Any for Backtrace`

- <span id="backtrace-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Backtrace`

- <span id="backtrace-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Backtrace`

- <span id="backtrace-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Backtrace`

- <span id="backtrace-clone"></span>`fn clone(&self) -> Backtrace` — [`Backtrace`](#backtrace)

##### `impl CloneToUninit for Backtrace`

- <span id="backtrace-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Backtrace`

- <span id="backtrace-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Backtrace`

- <span id="backtrace-default"></span>`fn default() -> Backtrace` — [`Backtrace`](#backtrace)

##### `impl<T> From for Backtrace`

- <span id="backtrace-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Backtrace`

- <span id="backtrace-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Backtrace`

- <span id="backtrace-toowned-type-owned"></span>`type Owned = T`

- <span id="backtrace-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="backtrace-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Backtrace`

- <span id="backtrace-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backtrace-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Backtrace`

- <span id="backtrace-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backtrace-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TracePtr`

```rust
struct TracePtr(*mut core::ffi::c_void);
```

*Defined in [`backtrace-0.3.76/src/capture.rs:35`](../../../.source_1765633015/backtrace-0.3.76/src/capture.rs#L35)*

#### Implementations

- <span id="traceptr-into-void"></span>`fn into_void(self) -> *mut c_void`

#### Trait Implementations

##### `impl Any for TracePtr`

- <span id="traceptr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TracePtr`

- <span id="traceptr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TracePtr`

- <span id="traceptr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TracePtr`

- <span id="traceptr-clone"></span>`fn clone(&self) -> TracePtr` — [`TracePtr`](#traceptr)

##### `impl CloneToUninit for TracePtr`

- <span id="traceptr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TracePtr`

##### `impl<T> From for TracePtr`

- <span id="traceptr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TracePtr`

- <span id="traceptr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Send for TracePtr`

##### `impl Sync for TracePtr`

##### `impl ToOwned for TracePtr`

- <span id="traceptr-toowned-type-owned"></span>`type Owned = T`

- <span id="traceptr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traceptr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TracePtr`

- <span id="traceptr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traceptr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TracePtr`

- <span id="traceptr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traceptr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BacktraceFrame`

```rust
struct BacktraceFrame {
    frame: Frame,
    symbols: Option<Box<[BacktraceSymbol]>>,
}
```

*Defined in [`backtrace-0.3.76/src/capture.rs:144-147`](../../../.source_1765633015/backtrace-0.3.76/src/capture.rs#L144-L147)*

Captured version of a frame in a backtrace.

This type is returned as a list from `Backtrace::frames` and represents one
stack frame in a captured backtrace.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

#### Implementations

- <span id="backtraceframe-ip"></span>`fn ip(&self) -> *mut c_void`

  Same as `Frame::ip`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtraceframe-symbol-address"></span>`fn symbol_address(&self) -> *mut c_void`

  Same as `Frame::symbol_address`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtraceframe-module-base-address"></span>`fn module_base_address(&self) -> Option<*mut c_void>`

  Same as `Frame::module_base_address`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtraceframe-symbols"></span>`fn symbols(&self) -> &[BacktraceSymbol]` — [`BacktraceSymbol`](#backtracesymbol)

  Returns the list of symbols that this frame corresponds to.

  

  Normally there is only one symbol per frame, but sometimes if a number

  of functions are inlined into one frame then multiple symbols will be

  returned. The first symbol listed is the "innermost function", whereas

  the last symbol is the outermost (last caller).

  

  Note that if this frame came from an unresolved backtrace then this will

  return an empty list.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtraceframe-resolve"></span>`fn resolve(&mut self)`

  Resolve all addresses in this frame to their symbolic names.

  

  If this frame has been previously resolved, this function does nothing.

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

#### Trait Implementations

##### `impl Any for BacktraceFrame`

- <span id="backtraceframe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BacktraceFrame`

- <span id="backtraceframe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BacktraceFrame`

- <span id="backtraceframe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BacktraceFrame`

- <span id="backtraceframe-clone"></span>`fn clone(&self) -> BacktraceFrame` — [`BacktraceFrame`](#backtraceframe)

##### `impl CloneToUninit for BacktraceFrame`

- <span id="backtraceframe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BacktraceFrame`

- <span id="backtraceframe-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BacktraceFrame`

- <span id="backtraceframe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BacktraceFrame`

- <span id="backtraceframe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BacktraceFrame`

- <span id="backtraceframe-toowned-type-owned"></span>`type Owned = T`

- <span id="backtraceframe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="backtraceframe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BacktraceFrame`

- <span id="backtraceframe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backtraceframe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BacktraceFrame`

- <span id="backtraceframe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backtraceframe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`backtrace-0.3.76/src/capture.rs:222-228`](../../../.source_1765633015/backtrace-0.3.76/src/capture.rs#L222-L228)*

Captured version of a symbol in a backtrace.

This type is returned as a list from `BacktraceFrame::symbols` and
represents the metadata for a symbol in a backtrace.

# Required features

This function requires the `std` feature of the `backtrace` crate to be
enabled, and the `std` feature is enabled by default.

#### Implementations

- <span id="backtracesymbol-name"></span>`fn name(&self) -> Option<SymbolName<'_>>` — [`SymbolName`](../symbolize/index.md#symbolname)

  Same as `Symbol::name`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtracesymbol-addr"></span>`fn addr(&self) -> Option<*mut c_void>`

  Same as `Symbol::addr`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtracesymbol-filename"></span>`fn filename(&self) -> Option<&Path>`

  Same as `Symbol::filename`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtracesymbol-lineno"></span>`fn lineno(&self) -> Option<u32>`

  Same as `Symbol::lineno`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

- <span id="backtracesymbol-colno"></span>`fn colno(&self) -> Option<u32>`

  Same as `Symbol::colno`

  

  # Required features

  

  This function requires the `std` feature of the `backtrace` crate to be

  enabled, and the `std` feature is enabled by default.

#### Trait Implementations

##### `impl Any for BacktraceSymbol`

- <span id="backtracesymbol-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BacktraceSymbol`

- <span id="backtracesymbol-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BacktraceSymbol`

- <span id="backtracesymbol-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BacktraceSymbol`

- <span id="backtracesymbol-clone"></span>`fn clone(&self) -> BacktraceSymbol` — [`BacktraceSymbol`](#backtracesymbol)

##### `impl CloneToUninit for BacktraceSymbol`

- <span id="backtracesymbol-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BacktraceSymbol`

- <span id="backtracesymbol-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BacktraceSymbol`

- <span id="backtracesymbol-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BacktraceSymbol`

- <span id="backtracesymbol-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BacktraceSymbol`

- <span id="backtracesymbol-toowned-type-owned"></span>`type Owned = T`

- <span id="backtracesymbol-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="backtracesymbol-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BacktraceSymbol`

- <span id="backtracesymbol-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backtracesymbol-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BacktraceSymbol`

- <span id="backtracesymbol-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backtracesymbol-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Frame`

```rust
enum Frame {
    Raw(crate::Frame),
}
```

*Defined in [`backtrace-0.3.76/src/capture.rs:150-158`](../../../.source_1765633015/backtrace-0.3.76/src/capture.rs#L150-L158)*

#### Implementations

- <span id="frame-ip"></span>`fn ip(&self) -> *mut c_void`

- <span id="frame-symbol-address"></span>`fn symbol_address(&self) -> *mut c_void`

- <span id="frame-module-base-address"></span>`fn module_base_address(&self) -> Option<*mut c_void>`

- <span id="frame-resolve-symbols"></span>`fn resolve_symbols(&self) -> Box<[BacktraceSymbol]>` — [`BacktraceSymbol`](#backtracesymbol)

  Resolve all addresses in the frame to their symbolic names.

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

### `_assert_send_sync`

```rust
fn _assert_send_sync()
```

*Defined in [`backtrace-0.3.76/src/capture.rs:129-132`](../../../.source_1765633015/backtrace-0.3.76/src/capture.rs#L129-L132)*

