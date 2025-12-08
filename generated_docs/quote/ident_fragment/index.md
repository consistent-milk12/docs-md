*[quote](../index.md) / [ident_fragment](index.md)*

---

# Module `ident_fragment`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IdentFragment`](#identfragment) | trait | Specialized formatting trait used by `format_ident!`. |
| [`ident_fragment_display!`](#ident_fragment_display) | macro |  |

## Traits

### `IdentFragment`

```rust
trait IdentFragment { ... }
```

Specialized formatting trait used by `format_ident!`.

[`Ident`](../../proc_macro2/imp/index.md) arguments formatted using this trait will have their `r#` prefix
stripped, if present.

See `format_ident!` for more information.


#### Required Methods

- `fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format this value as an identifier fragment.

- `fn span(&self) -> Option<Span>`

  Span associated with this `IdentFragment`.

## Macros

### `ident_fragment_display!`

