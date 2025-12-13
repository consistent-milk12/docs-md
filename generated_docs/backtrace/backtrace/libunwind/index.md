*[backtrace](../../index.md) / [backtrace](../index.md) / [libunwind](index.md)*

---

# Module `libunwind`

Backtrace support using libunwind/gcc_s/etc APIs.

This module contains the ability to unwind the stack using libunwind-style
APIs. Note that there's a whole bunch of implementations of the
libunwind-like API, and this is just trying to be compatible with most of
them all at once instead of being picky.

The libunwind API is powered by `_Unwind_Backtrace` and is in practice very
reliable at generating a backtrace. It's not entirely clear how it does it
(frame pointers? eh_frame info? both?) but it seems to work!

Most of the complexity of this module is handling the various platform
differences across libunwind implementations. Otherwise this is a pretty
straightforward Rust binding to the libunwind APIs.

This is the default unwinding API for all non-Windows platforms currently.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`uw`](#uw) | mod | Unwind library interface used for backtraces |
| [`Bomb`](#bomb) | struct |  |
| [`Frame`](#frame) | enum |  |
| [`trace`](#trace) | fn |  |

## Modules

- [`uw`](uw/index.md) — Unwind library interface used for backtraces

## Structs

### `Bomb`

```rust
struct Bomb {
    enabled: bool,
}
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:102-104`](../../../../.source_1765633015/backtrace-0.3.76/src/backtrace/libunwind.rs#L102-L104)*

#### Trait Implementations

##### `impl Any for Bomb`

- <span id="bomb-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bomb`

- <span id="bomb-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bomb`

- <span id="bomb-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for Bomb`

- <span id="bomb-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Bomb`

- <span id="bomb-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bomb`

- <span id="bomb-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Bomb`

- <span id="bomb-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bomb-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bomb`

- <span id="bomb-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bomb-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Frame`

```rust
enum Frame {
    Raw(*mut uw::_Unwind_Context),
    Cloned {
        ip: *mut core::ffi::c_void,
        sp: *mut core::ffi::c_void,
        symbol_address: *mut core::ffi::c_void,
    },
}
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:21-28`](../../../../.source_1765633015/backtrace-0.3.76/src/backtrace/libunwind.rs#L21-L28)*

#### Implementations

- <span id="frame-ip"></span>`fn ip(&self) -> *mut c_void`

- <span id="frame-sp"></span>`fn sp(&self) -> *mut c_void`

- <span id="frame-symbol-address"></span>`fn symbol_address(&self) -> *mut c_void`

- <span id="frame-module-base-address"></span>`fn module_base_address(&self) -> Option<*mut c_void>`

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

##### `impl Send for Frame`

##### `impl Sync for Frame`

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
unsafe fn trace(cb: &mut dyn FnMut(&super::Frame) -> bool)
```

*Defined in [`backtrace-0.3.76/src/backtrace/libunwind.rs:115-139`](../../../../.source_1765633015/backtrace-0.3.76/src/backtrace/libunwind.rs#L115-L139)*

