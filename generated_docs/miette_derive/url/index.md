*[miette_derive](../index.md) / [url](index.md)*

---

# Module `url`

## Enums

### `Url`

```rust
enum Url {
    Display(crate::fmt::Display),
    DocsRs,
}
```

#### Implementations

- `fn gen_enum(enum_name: &syn::Ident, variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- `fn gen_struct(self: &Self, struct_name: &syn::Ident, fields: &Fields) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Parse for Url`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

