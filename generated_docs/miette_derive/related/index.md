*[miette_derive](../index.md) / [related](index.md)*

---

# Module `related`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Related`](#related) | struct |  |

## Structs

### `Related`

```rust
struct Related(syn::Member);
```

#### Implementations

- <span id="related-from-fields"></span>`fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- <span id="related-from-fields-vec"></span>`fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- <span id="related-gen-enum"></span>`fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- <span id="related-gen-struct"></span>`fn gen_struct(&self) -> Option<TokenStream>`

