*[clap_derive](../../index.md) / [utils](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SpanError`](#spanerror) | trait |  |
| [`ToTokensError`](#totokenserror) | trait |  |

## Traits

### `SpanError`

```rust
trait SpanError { ... }
```

#### Required Methods

- `fn EXPECTED_Span_OR_ToTokens<D: std::fmt::Display>(&self, msg: D) -> syn::Error`

#### Implementors

- `proc_macro2::Span`

### `ToTokensError`

```rust
trait ToTokensError { ... }
```

#### Required Methods

- `fn EXPECTED_Span_OR_ToTokens<D: std::fmt::Display>(&self, msg: D) -> syn::Error`

#### Implementors

- `T`

