*[miette_derive](../index.md) / [related](index.md)*

---

# Module `related`

## Structs

### `Related`

```rust
struct Related(syn::Member);
```

#### Implementations

- `fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- `fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- `fn gen_struct(self: &Self) -> Option<TokenStream>`

