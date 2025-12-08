*[quote](../index.md) / [to_tokens](index.md)*

---

# Module `to_tokens`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ToTokens`](#totokens) | trait | Types that can be interpolated inside a `quote!` invocation. |

## Traits

### `ToTokens`

```rust
trait ToTokens { ... }
```

Types that can be interpolated inside a `quote!` invocation.

#### Required Methods

- `fn to_tokens(&self, tokens: &mut TokenStream)`

  Write `self` to the given `TokenStream`.

- `fn to_token_stream(&self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.

- `fn into_token_stream(self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.

