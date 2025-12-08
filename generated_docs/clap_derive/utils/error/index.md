*[clap_derive](../../index.md) / [utils](../index.md) / [error](index.md)*

---

# Module `error`

## Traits

### `SpanError`

```rust
trait SpanError { ... }
```

#### Required Methods

- `fn EXPECTED_Span_OR_ToTokens<D: std::fmt::Display>(self: &Self, msg: D) -> syn::Error`

### `ToTokensError`

```rust
trait ToTokensError { ... }
```

#### Required Methods

- `fn EXPECTED_Span_OR_ToTokens<D: std::fmt::Display>(self: &Self, msg: D) -> syn::Error`

