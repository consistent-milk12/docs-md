*[quote](../index.md) / [ident_fragment](index.md)*

---

# Module `ident_fragment`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IdentFragment`](#identfragment) | trait | Specialized formatting trait used by `format_ident!`. |
| [`ident_fragment_display!`](#ident-fragment-display) | macro |  |

## Traits

### `IdentFragment`

```rust
trait IdentFragment { ... }
```

*Defined in [`quote-1.0.42/src/ident_fragment.rs:13-23`](../../../.source_1765900590/quote-1.0.42/src/ident_fragment.rs#L13-L23)*

Specialized formatting trait used by `format_ident!`.

[`Ident`](../../proc_macro2/imp/index.md) arguments formatted using this trait will have their `r#` prefix
stripped, if present.

See `format_ident!` for more information.


#### Required Methods

- `fn IdentFragment::fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format this value as an identifier fragment.

#### Provided Methods

- `fn IdentFragment::span(&self) -> Option<Span>`

  Span associated with this `IdentFragment`.
  
  If non-`None`, may be inherited by formatted identifiers.

#### Implementors

- `&T`
- `&mut T`
- `String`
- `alloc::borrow::Cow<'_, T>`
- `bool`
- `char`
- `proc_macro2::Ident`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Macros

### `ident_fragment_display!`

*Defined in [`quote-1.0.42/src/ident_fragment.rs:75-85`](../../../.source_1765900590/quote-1.0.42/src/ident_fragment.rs#L75-L85)*

