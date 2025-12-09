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

#### Associated Types

- `type Inner: 1`

#### Required Methods

- `fn style(&self) -> &crate::Style`

- `fn inner(&self) -> &<Self as >::Inner`

#### Implementors

- [`Styled`](../../index.md)
- `&T`

