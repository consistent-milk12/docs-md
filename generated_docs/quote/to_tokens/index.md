*[quote](../index.md) / [to_tokens](index.md)*

---

# Module `to_tokens`

## Traits

### `ToTokens`

```rust
trait ToTokens { ... }
```

Types that can be interpolated inside a `quote!` invocation.

#### Required Methods

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

  Write `self` to the given `TokenStream`.

- `fn to_token_stream(self: &Self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.

- `fn into_token_stream(self: Self) -> TokenStream`

  Convert `self` directly into a `TokenStream` object.

