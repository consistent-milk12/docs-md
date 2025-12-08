*[owo_colors](../../index.md) / [styled_list](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IsStyled`](#isstyled) | trait |  |

## Traits

### `IsStyled`

```rust
trait IsStyled { ... }
```

#### Required Methods

- `type Inner: 1`

- `fn style(&self) -> &crate::Style`

- `fn inner(&self) -> &<Self as >::Inner`

