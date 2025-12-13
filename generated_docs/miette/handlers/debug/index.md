*[miette](../../index.md) / [handlers](../index.md) / [debug](index.md)*

---

# Module `debug`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugReportHandler`](#debugreporthandler) | struct | [`ReportHandler`] that renders plain text and avoids extraneous graphics. |

## Structs

### `DebugReportHandler`

```rust
struct DebugReportHandler;
```

*Defined in [`miette-7.6.0/src/handlers/debug.rs:11`](../../../../.source_1765633015/miette-7.6.0/src/handlers/debug.rs#L11)*

[`ReportHandler`](../../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="debugreporthandler-new"></span>`const fn new() -> Self`

  Create a new [`NarratableReportHandler`](crate::NarratableReportHandler)

  There are no customization options.

#### Trait Implementations

##### `impl Any for DebugReportHandler`

- <span id="debugreporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugReportHandler`

- <span id="debugreporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugReportHandler`

- <span id="debugreporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DebugReportHandler`

- <span id="debugreporthandler-clone"></span>`fn clone(&self) -> DebugReportHandler` — [`DebugReportHandler`](../index.md#debugreporthandler)

##### `impl CloneToUninit for DebugReportHandler`

- <span id="debugreporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DebugReportHandler`

- <span id="debugreporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DebugReportHandler`

- <span id="debugreporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for DebugReportHandler`

- <span id="debugreporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugReportHandler`

- <span id="debugreporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DebugReportHandler`

##### `impl ReportHandler for DebugReportHandler`

- <span id="debugreporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl ToOwned for DebugReportHandler`

- <span id="debugreporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="debugreporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugreporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugReportHandler`

- <span id="debugreporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugreporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugReportHandler`

- <span id="debugreporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugreporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

