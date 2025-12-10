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

*Defined in [`miette-7.6.0/src/handlers/json.rs:11`](../../../../.source_1765210505/miette-7.6.0/src/handlers/json.rs#L11)*

[`ReportHandler`](../../index.md) that renders JSON output. It's a machine-readable output.

#### Implementations

- <span id="jsonreporthandler-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for JSONReportHandler`

- <span id="jsonreporthandler-clone"></span>`fn clone(&self) -> JSONReportHandler` — [`JSONReportHandler`](../index.md#jsonreporthandler)

##### `impl Debug for JSONReportHandler`

- <span id="jsonreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for JSONReportHandler`

- <span id="jsonreporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for JSONReportHandler`

##### `impl ReportHandler for JSONReportHandler`

- <span id="jsonreporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

### `Escape<'a>`

```rust
struct Escape<'a>(&'a str);
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:27`](../../../../.source_1765210505/miette-7.6.0/src/handlers/json.rs#L27)*

#### Trait Implementations

##### `impl Display for Escape<'_>`

- <span id="escape-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for Escape<'a>`

##### `impl ToString for Escape<'a>`

- <span id="escape-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `escape`

```rust
const fn escape(input: &str) -> Escape<'_>
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:52-54`](../../../../.source_1765210505/miette-7.6.0/src/handlers/json.rs#L52-L54)*

