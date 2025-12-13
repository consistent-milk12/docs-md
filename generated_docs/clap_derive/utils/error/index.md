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

*Defined in [`clap_derive-4.5.49/src/utils/error.rs:1-4`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/error.rs#L1-L4)*

#### Required Methods

- `fn EXPECTED_Span_OR_ToTokens<D: std::fmt::Display>(&self, msg: D) -> syn::Error`

#### Implementors

- `proc_macro2::Span`

### `ToTokensError`

```rust
trait ToTokensError { ... }
```

*Defined in [`clap_derive-4.5.49/src/utils/error.rs:6-9`](../../../../.source_1765633015/clap_derive-4.5.49/src/utils/error.rs#L6-L9)*

#### Required Methods

- `fn EXPECTED_Span_OR_ToTokens<D: std::fmt::Display>(&self, msg: D) -> syn::Error`

#### Implementors

- `T`

