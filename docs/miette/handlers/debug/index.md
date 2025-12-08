*[miette](../../index.md) / [handlers](../index.md) / [debug](index.md)*

---

# Module `debug`

## Structs

### `DebugReportHandler`

```rust
struct DebugReportHandler;
```

[`ReportHandler`](../../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- `fn render_report(self: &Self, f: &mut fmt::Formatter<'_>, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../../index.md)

#### Trait Implementations

##### `impl Clone for DebugReportHandler`

- `fn clone(self: &Self) -> DebugReportHandler` — [`DebugReportHandler`](../index.md)

##### `impl Debug for DebugReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DebugReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for DebugReportHandler`

##### `impl ReportHandler for DebugReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md)

