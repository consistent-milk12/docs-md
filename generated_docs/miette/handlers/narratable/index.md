*[miette](../../index.md) / [handlers](../index.md) / [narratable](index.md)*

---

# Module `narratable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NarratableReportHandler`](#narratablereporthandler) | struct | [`ReportHandler`] that renders plain text and avoids extraneous graphics. |
| [`Line`](#line) | struct |  |
| [`SpanAttach`](#spanattach) | enum |  |
| [`safe_get_column`](#safe-get-column) | fn | Returns column at offset, and nearest boundary if offset is in the middle of the character |

## Structs

### `NarratableReportHandler`

```rust
struct NarratableReportHandler {
    context_lines: usize,
    with_cause_chain: bool,
    footer: Option<String>,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:15-19`](../../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L15-L19)*

[`ReportHandler`](../../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="narratablereporthandler-new"></span>`const fn new() -> Self`

  Create a new [`NarratableReportHandler`](../index.md). There are no customization

  options.

- <span id="narratablereporthandler-with-cause-chain"></span>`const fn with_cause_chain(self) -> Self`

  Include the cause chain of the top-level error in the report, if

  available.

- <span id="narratablereporthandler-without-cause-chain"></span>`const fn without_cause_chain(self) -> Self`

  Do not include the cause chain of the top-level error in the report.

- <span id="narratablereporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

  Set the footer to be displayed at the end of the report.

- <span id="narratablereporthandler-with-context-lines"></span>`const fn with_context_lines(self, lines: usize) -> Self`

  Sets the number of lines of context to show around each error.

#### Trait Implementations

##### `impl Any for NarratableReportHandler`

- <span id="narratablereporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NarratableReportHandler`

- <span id="narratablereporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NarratableReportHandler`

- <span id="narratablereporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NarratableReportHandler`

- <span id="narratablereporthandler-clone"></span>`fn clone(&self) -> NarratableReportHandler` — [`NarratableReportHandler`](../index.md#narratablereporthandler)

##### `impl CloneToUninit for NarratableReportHandler`

- <span id="narratablereporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NarratableReportHandler`

- <span id="narratablereporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NarratableReportHandler`

- <span id="narratablereporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for NarratableReportHandler`

- <span id="narratablereporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NarratableReportHandler`

- <span id="narratablereporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for NarratableReportHandler`

##### `impl ReportHandler for NarratableReportHandler`

- <span id="narratablereporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl ToOwned for NarratableReportHandler`

- <span id="narratablereporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="narratablereporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="narratablereporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NarratableReportHandler`

- <span id="narratablereporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="narratablereporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NarratableReportHandler`

- <span id="narratablereporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="narratablereporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    text: String,
    at_end_of_file: bool,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:360-365`](../../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L360-L365)*

#### Implementations

- <span id="line-span-attach"></span>`fn span_attach(&self, span: &SourceSpan) -> Option<SpanAttach>` — [`SourceSpan`](../../index.md#sourcespan), [`SpanAttach`](#spanattach)

#### Trait Implementations

##### `impl Any for Line`

- <span id="line-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Line`

- <span id="line-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Line`

- <span id="line-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Line`

- <span id="line-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Line`

- <span id="line-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Line`

##### `impl<U> TryFrom for Line`

- <span id="line-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="line-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Line`

- <span id="line-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="line-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `SpanAttach`

```rust
enum SpanAttach {
    Contained {
        col_start: usize,
        col_end: usize,
    },
    Starts {
        col_start: usize,
    },
    Ends {
        col_end: usize,
    },
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:367-371`](../../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L367-L371)*

#### Trait Implementations

##### `impl Any for SpanAttach`

- <span id="spanattach-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpanAttach`

- <span id="spanattach-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpanAttach`

- <span id="spanattach-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SpanAttach`

- <span id="spanattach-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpanAttach`

- <span id="spanattach-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SpanAttach`

##### `impl<U> TryFrom for SpanAttach`

- <span id="spanattach-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="spanattach-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpanAttach`

- <span id="spanattach-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="spanattach-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `safe_get_column`

```rust
fn safe_get_column(text: &str, offset: usize, start: bool) -> usize
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:375-392`](../../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L375-L392)*

Returns column at offset, and nearest boundary if offset is in the middle of
the character

