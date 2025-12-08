*[miette](../../index.md) / [handlers](../index.md) / [json](index.md)*

---

# Module `json`

## Structs

### `JSONReportHandler`

```rust
struct JSONReportHandler;
```

[`ReportHandler`](../../index.md) that renders JSON output. It's a machine-readable output.

#### Implementations

- `fn render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../../index.md)

- `fn _render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- `fn render_snippets(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, source: &dyn SourceCode) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

#### Trait Implementations

##### `impl Clone for JSONReportHandler`

- `fn clone(self: &Self) -> JSONReportHandler` — [`JSONReportHandler`](../index.md)

##### `impl Debug for JSONReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for JSONReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for JSONReportHandler`

##### `impl ReportHandler for JSONReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md)

### `Escape<'a>`

```rust
struct Escape<'a>(&'a str);
```

#### Trait Implementations

##### `impl Display for Escape<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Escape<'a>`

##### `impl<T> ToString for Escape<'a>`

- `fn to_string(self: &Self) -> String`

## Functions

### `escape`

```rust
const fn escape(input: &str) -> Escape<'_>
```

