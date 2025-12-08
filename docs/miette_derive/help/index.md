*[miette_derive](../index.md) / [help](index.md)*

---

# Module `help`

## Enums

### `Help`

```rust
enum Help {
    Display(crate::fmt::Display),
    Field(syn::Member, Box<syn::Type>),
}
```

#### Implementations

- `fn from_fields(fields: &syn::Fields) -> syn::Result<Option<Self>>`

- `fn from_fields_vec(fields: Vec<&syn::Field>) -> syn::Result<Option<Self>>`

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- `fn gen_struct(self: &Self, fields: &Fields) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Parse for Help`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

