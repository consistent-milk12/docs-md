*[miette](../../index.md) / [handlers](../index.md) / [narratable](index.md)*

---

# Module `narratable`

## Structs

### `NarratableReportHandler`

```rust
struct NarratableReportHandler {
    context_lines: usize,
    with_cause_chain: bool,
    footer: Option<String>,
}
```

[`ReportHandler`](../../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- `const fn new() -> Self`

- `const fn with_cause_chain(self: Self) -> Self`

- `const fn without_cause_chain(self: Self) -> Self`

- `fn with_footer(self: Self, footer: String) -> Self`

- `const fn with_context_lines(self: Self, lines: usize) -> Self`

#### Trait Implementations

##### `impl Clone for NarratableReportHandler`

- `fn clone(self: &Self) -> NarratableReportHandler` — [`NarratableReportHandler`](../index.md)

##### `impl Debug for NarratableReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NarratableReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for NarratableReportHandler`

##### `impl ReportHandler for NarratableReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    text: String,
    at_end_of_file: bool,
}
```

#### Implementations

- `fn span_attach(self: &Self, span: &SourceSpan) -> Option<SpanAttach>` — [`SourceSpan`](../../index.md), [`SpanAttach`](#spanattach)

#### Trait Implementations

##### `impl<D> OwoColorize for Line`

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

#### Trait Implementations

##### `impl<D> OwoColorize for SpanAttach`

## Functions

### `safe_get_column`

```rust
fn safe_get_column(text: &str, offset: usize, start: bool) -> usize
```

Returns column at offset, and nearest boundary if offset is in the middle of
the character

