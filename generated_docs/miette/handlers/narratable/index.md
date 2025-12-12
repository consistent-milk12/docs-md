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

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:15-19`](../../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L15-L19)*

[`ReportHandler`](../../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="narratablereporthandler-new"></span>`const fn new() -> Self`

- <span id="narratablereporthandler-with-cause-chain"></span>`const fn with_cause_chain(self) -> Self`

- <span id="narratablereporthandler-without-cause-chain"></span>`const fn without_cause_chain(self) -> Self`

- <span id="narratablereporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

- <span id="narratablereporthandler-with-context-lines"></span>`const fn with_context_lines(self, lines: usize) -> Self`

#### Trait Implementations

##### `impl Clone for NarratableReportHandler`

- <span id="narratablereporthandler-clone"></span>`fn clone(&self) -> NarratableReportHandler` — [`NarratableReportHandler`](../index.md#narratablereporthandler)

##### `impl Debug for NarratableReportHandler`

- <span id="narratablereporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NarratableReportHandler`

- <span id="narratablereporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for NarratableReportHandler`

##### `impl ReportHandler for NarratableReportHandler`

- <span id="narratablereporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    text: String,
    at_end_of_file: bool,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:360-365`](../../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L360-L365)*

#### Implementations

- <span id="line-span-attach"></span>`fn span_attach(&self, span: &SourceSpan) -> Option<SpanAttach>` — [`SourceSpan`](../../index.md#sourcespan), [`SpanAttach`](#spanattach)

#### Trait Implementations

##### `impl OwoColorize for Line`

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

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:367-371`](../../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L367-L371)*

#### Trait Implementations

##### `impl OwoColorize for SpanAttach`

## Functions

### `safe_get_column`

```rust
fn safe_get_column(text: &str, offset: usize, start: bool) -> usize
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:375-392`](../../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L375-L392)*

Returns column at offset, and nearest boundary if offset is in the middle of
the character

