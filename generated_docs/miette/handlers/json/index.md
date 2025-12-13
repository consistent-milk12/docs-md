*[miette](../../index.md) / [handlers](../index.md) / [json](index.md)*

---

# Module `json`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`JSONReportHandler`](#jsonreporthandler) | struct | [`ReportHandler`] that renders JSON output. |
| [`Escape`](#escape) | struct |  |
| [`escape`](#escape) | fn |  |

## Structs

### `JSONReportHandler`

```rust
struct JSONReportHandler;
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:11`](../../../../.source_1765521767/miette-7.6.0/src/handlers/json.rs#L11)*

[`ReportHandler`](../../index.md) that renders JSON output. It's a machine-readable output.

#### Implementations

- <span id="jsonreporthandler-new"></span>`const fn new() -> Self`

  Create a new [`JSONReportHandler`](../index.md). There are no customization

  options.

#### Trait Implementations

##### `impl Any for JSONReportHandler`

- <span id="jsonreporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JSONReportHandler`

- <span id="jsonreporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JSONReportHandler`

- <span id="jsonreporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for JSONReportHandler`

- <span id="jsonreporthandler-clone"></span>`fn clone(&self) -> JSONReportHandler` — [`JSONReportHandler`](../index.md#jsonreporthandler)

##### `impl CloneToUninit for JSONReportHandler`

- <span id="jsonreporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for JSONReportHandler`

- <span id="jsonreporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for JSONReportHandler`

- <span id="jsonreporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for JSONReportHandler`

- <span id="jsonreporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JSONReportHandler`

- <span id="jsonreporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for JSONReportHandler`

##### `impl ReportHandler for JSONReportHandler`

- <span id="jsonreporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl ToOwned for JSONReportHandler`

- <span id="jsonreporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="jsonreporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="jsonreporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for JSONReportHandler`

- <span id="jsonreporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jsonreporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JSONReportHandler`

- <span id="jsonreporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jsonreporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Escape<'a>`

```rust
struct Escape<'a>(&'a str);
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:27`](../../../../.source_1765521767/miette-7.6.0/src/handlers/json.rs#L27)*

#### Trait Implementations

##### `impl Any for Escape<'a>`

- <span id="escape-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Escape<'a>`

- <span id="escape-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Escape<'a>`

- <span id="escape-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Escape<'_>`

- <span id="escape-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Escape<'a>`

- <span id="escape-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Escape<'a>`

- <span id="escape-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Escape<'a>`

##### `impl ToString for Escape<'a>`

- <span id="escape-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Escape<'a>`

- <span id="escape-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="escape-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Escape<'a>`

- <span id="escape-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="escape-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `escape`

```rust
const fn escape(input: &str) -> Escape<'_>
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:52-54`](../../../../.source_1765521767/miette-7.6.0/src/handlers/json.rs#L52-L54)*

