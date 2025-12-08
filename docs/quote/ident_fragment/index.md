*[quote](../index.md) / [ident_fragment](index.md)*

---

# Module `ident_fragment`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format this value as an identifier fragment.

- `fn span(self: &Self) -> Option<Span>`

  Span associated with this `IdentFragment`.

## Macros

### `ident_fragment_display!`

