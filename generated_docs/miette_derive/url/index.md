*[miette_derive](../index.md) / [url](index.md)*

---

# Module `url`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Url`](#url) | enum |  |

## Enums

### `Url`

```rust
enum Url {
    Display(crate::fmt::Display),
    DocsRs,
}
```

#### Implementations

- <span id="url-gen-enum"></span>`fn gen_enum(enum_name: &syn::Ident, variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- <span id="url-gen-struct"></span>`fn gen_struct(&self, struct_name: &syn::Ident, fields: &Fields) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Parse for Url`

- <span id="url-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

