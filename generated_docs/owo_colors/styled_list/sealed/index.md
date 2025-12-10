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

*Defined in [`owo-colors-4.2.3/src/styled_list.rs:12-17`](../../../../.source_1765210505/owo-colors-4.2.3/src/styled_list.rs#L12-L17)*

#### Associated Types

- `type Inner: 1`

#### Required Methods

- `fn style(&self) -> &crate::Style`

- `fn inner(&self) -> &<Self as >::Inner`

#### Implementors

- [`Styled`](../../index.md#styled)
- `&T`

