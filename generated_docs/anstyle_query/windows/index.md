*[anstyle_query](../index.md) / [windows](index.md)*

---

# Module `windows`

Windows-specific style queries

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`windows_console`](#windows-console) | mod |  |
| [`enable_ansi_colors`](#enable-ansi-colors) | fn | Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences)) |

## Modules

- [`windows_console`](windows_console/index.md)

## Functions

### `enable_ansi_colors`

```rust
fn enable_ansi_colors() -> Option<bool>
```

*Defined in [`anstyle-query-1.1.5/src/windows.rs:71-73`](../../../.source_1765521767/anstyle-query-1.1.5/src/windows.rs#L71-L73)*

Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences))

For non-windows systems, returns `None`

