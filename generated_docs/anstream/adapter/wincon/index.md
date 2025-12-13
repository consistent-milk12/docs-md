*[anstream](../../index.md) / [adapter](../index.md) / [wincon](index.md)*

---

# Module `wincon`

## Contents

- [Structs](#structs)
  - [`WinconBytes`](#winconbytes)
  - [`WinconBytesIter`](#winconbytesiter)
  - [`WinconCapture`](#winconcapture)
- [Enums](#enums)
  - [`CsiState`](#csistate)
  - [`ColorTarget`](#colortarget)
- [Functions](#functions)
  - [`next_bytes`](#next-bytes)
  - [`to_ansi_color`](#to-ansi-color)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WinconBytes`](#winconbytes) | struct | Incrementally convert to wincon calls for non-contiguous data |
| [`WinconBytesIter`](#winconbytesiter) | struct | See [`WinconBytes`] |
| [`WinconCapture`](#winconcapture) | struct |  |
| [`CsiState`](#csistate) | enum |  |
| [`ColorTarget`](#colortarget) | enum |  |
| [`next_bytes`](#next-bytes) | fn |  |
| [`to_ansi_color`](#to-ansi-color) | fn |  |

## Structs

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:3-6`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L3-L6)*

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- <span id="winconbytes-new"></span>`fn new() -> Self`

  Initial state

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](#winconbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Any for WinconBytes`

- <span id="winconbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WinconBytes`

- <span id="winconbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WinconBytes`

- <span id="winconbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl CloneToUninit for WinconBytes`

- <span id="winconbytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WinconBytes`

- <span id="winconbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl<T> From for WinconBytes`

- <span id="winconbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WinconBytes`

- <span id="winconbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-partialeq-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](#winconbytes)

##### `impl StructuralPartialEq for WinconBytes`

##### `impl ToOwned for WinconBytes`

- <span id="winconbytes-toowned-type-owned"></span>`type Owned = T`

- <span id="winconbytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="winconbytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WinconBytes`

- <span id="winconbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winconbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WinconBytes`

- <span id="winconbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winconbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:28-32`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L28-L32)*

See [`WinconBytes`](#winconbytes)

#### Trait Implementations

##### `impl Any for WinconBytesIter<'s>`

- <span id="winconbytesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WinconBytesIter<'s>`

- <span id="winconbytesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WinconBytesIter<'s>`

- <span id="winconbytesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for WinconBytesIter<'s>`

- <span id="winconbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WinconBytesIter<'s>`

##### `impl<T> From for WinconBytesIter<'s>`

- <span id="winconbytesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WinconBytesIter<'s>`

- <span id="winconbytesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for WinconBytesIter<'s>`

- <span id="winconbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="winconbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="winconbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- <span id="winconbytesiter-iterator-type-item"></span>`type Item = (Style, String)`

- <span id="winconbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for WinconBytesIter<'s>`

- <span id="winconbytesiter-partialeq-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](#winconbytesiter)

##### `impl StructuralPartialEq for WinconBytesIter<'s>`

##### `impl<U> TryFrom for WinconBytesIter<'s>`

- <span id="winconbytesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winconbytesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WinconBytesIter<'s>`

- <span id="winconbytesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winconbytesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WinconCapture`

```rust
struct WinconCapture {
    style: anstyle::Style,
    printable: String,
    ready: Option<anstyle::Style>,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:68-72`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L68-L72)*

#### Implementations

- <span id="winconcapture-reset"></span>`fn reset(&mut self)`

#### Trait Implementations

##### `impl Any for WinconCapture`

- <span id="winconcapture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WinconCapture`

- <span id="winconcapture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WinconCapture`

- <span id="winconcapture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WinconCapture`

- <span id="winconcapture-clone"></span>`fn clone(&self) -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl CloneToUninit for WinconCapture`

- <span id="winconcapture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WinconCapture`

- <span id="winconcapture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconCapture`

- <span id="winconcapture-default"></span>`fn default() -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Eq for WinconCapture`

##### `impl<T> From for WinconCapture`

- <span id="winconcapture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WinconCapture`

- <span id="winconcapture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WinconCapture`

- <span id="winconcapture-partialeq-eq"></span>`fn eq(&self, other: &WinconCapture) -> bool` — [`WinconCapture`](#winconcapture)

##### `impl Perform for WinconCapture`

- <span id="winconcapture-perform-print"></span>`fn print(&mut self, c: char)`

  Draw a character to the screen and update states.

- <span id="winconcapture-perform-execute"></span>`fn execute(&mut self, byte: u8)`

  Execute a C0 or C1 control function.

- <span id="winconcapture-perform-csi-dispatch"></span>`fn csi_dispatch(&mut self, params: &anstyle_parse::Params, _intermediates: &[u8], ignore: bool, action: u8)`

##### `impl StructuralPartialEq for WinconCapture`

##### `impl ToOwned for WinconCapture`

- <span id="winconcapture-toowned-type-owned"></span>`type Owned = T`

- <span id="winconcapture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="winconcapture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WinconCapture`

- <span id="winconcapture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winconcapture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WinconCapture`

- <span id="winconcapture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winconcapture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `CsiState`

```rust
enum CsiState {
    Normal,
    PrepareCustomColor,
    Ansi256,
    Rgb,
    Underline,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:272-278`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L272-L278)*

#### Trait Implementations

##### `impl Any for CsiState`

- <span id="csistate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CsiState`

- <span id="csistate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CsiState`

- <span id="csistate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CsiState`

- <span id="csistate-clone"></span>`fn clone(&self) -> CsiState` — [`CsiState`](#csistate)

##### `impl CloneToUninit for CsiState`

- <span id="csistate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CsiState`

##### `impl Debug for CsiState`

- <span id="csistate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CsiState`

##### `impl<T> From for CsiState`

- <span id="csistate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CsiState`

- <span id="csistate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CsiState`

- <span id="csistate-partialeq-eq"></span>`fn eq(&self, other: &CsiState) -> bool` — [`CsiState`](#csistate)

##### `impl StructuralPartialEq for CsiState`

##### `impl ToOwned for CsiState`

- <span id="csistate-toowned-type-owned"></span>`type Owned = T`

- <span id="csistate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="csistate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CsiState`

- <span id="csistate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="csistate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CsiState`

- <span id="csistate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="csistate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ColorTarget`

```rust
enum ColorTarget {
    Fg,
    Bg,
    Underline,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:281-285`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L281-L285)*

#### Trait Implementations

##### `impl Any for ColorTarget`

- <span id="colortarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ColorTarget`

- <span id="colortarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ColorTarget`

- <span id="colortarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ColorTarget`

- <span id="colortarget-clone"></span>`fn clone(&self) -> ColorTarget` — [`ColorTarget`](#colortarget)

##### `impl CloneToUninit for ColorTarget`

- <span id="colortarget-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ColorTarget`

##### `impl Debug for ColorTarget`

- <span id="colortarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColorTarget`

##### `impl<T> From for ColorTarget`

- <span id="colortarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ColorTarget`

- <span id="colortarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ColorTarget`

- <span id="colortarget-partialeq-eq"></span>`fn eq(&self, other: &ColorTarget) -> bool` — [`ColorTarget`](#colortarget)

##### `impl StructuralPartialEq for ColorTarget`

##### `impl ToOwned for ColorTarget`

- <span id="colortarget-toowned-type-owned"></span>`type Owned = T`

- <span id="colortarget-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="colortarget-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ColorTarget`

- <span id="colortarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colortarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ColorTarget`

- <span id="colortarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colortarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `next_bytes`

```rust
fn next_bytes(bytes: &mut &[u8], parser: &mut anstyle_parse::Parser, capture: &mut WinconCapture) -> Option<(anstyle::Style, String)>
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:44-65`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L44-L65)*

### `to_ansi_color`

```rust
fn to_ansi_color(digit: u16) -> Option<anstyle::AnsiColor>
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:287-299`](../../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L287-L299)*

