*[miette_derive](../index.md) / [diagnostic_source](index.md)*

---

# Module `diagnostic_source`

## Structs

### `DiagnosticSource`

```rust
struct DiagnosticSource(syn::Member);
```

#### Implementations

- `fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- `fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- `fn gen_struct(self: &Self) -> Option<TokenStream>`

