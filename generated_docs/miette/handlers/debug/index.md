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

*Defined in [`miette-7.6.0/src/handlers/debug.rs:11`](../../../../.source_1765210505/miette-7.6.0/src/handlers/debug.rs#L11)*

[`ReportHandler`](../../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="debugreporthandler-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for DebugReportHandler`

- <span id="debugreporthandler-clone"></span>`fn clone(&self) -> DebugReportHandler` — [`DebugReportHandler`](../index.md#debugreporthandler)

##### `impl Debug for DebugReportHandler`

- <span id="debugreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DebugReportHandler`

- <span id="debugreporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for DebugReportHandler`

##### `impl ReportHandler for DebugReportHandler`

- <span id="debugreporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

