*[miette_derive](../index.md) / [source_code](index.md)*

---

# Module `source_code`

## Structs

### `SourceCode`

```rust
struct SourceCode {
    source_code: syn::Member,
    is_option: bool,
}
```

#### Implementations

- `fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- `fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- `fn gen_struct(self: &Self, fields: &syn::Fields) -> Option<TokenStream>`

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

