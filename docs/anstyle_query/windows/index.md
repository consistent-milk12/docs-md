*[anstyle_query](../index.md) / [windows](index.md)*

---

# Module `windows`

Windows-specific style queries

## Functions

### `enable_ansi_colors`

```rust
fn enable_ansi_colors() -> Option<bool>
```

Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences))

For non-windows systems, returns `None`

