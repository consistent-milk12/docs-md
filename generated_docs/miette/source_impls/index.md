*[miette](../index.md) / [source_impls](index.md)*

---

# Module `source_impls`

Default trait implementations for [`SourceCode`](../index.md).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`context_info`](#context_info) | fn |  |

## Functions

### `context_info`

```rust
fn context_info<'a>(input: &'a [u8], span: &crate::SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<crate::MietteSpanContents<'a>, crate::MietteError>
```

