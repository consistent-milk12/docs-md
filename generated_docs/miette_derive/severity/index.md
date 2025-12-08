*[miette_derive](../index.md) / [severity](index.md)*

---

# Module `severity`

## Structs

### `Severity`

```rust
struct Severity(syn::Ident);
```

#### Implementations

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- `fn gen_struct(self: &Self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Parse for Severity`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

## Functions

### `get_severity`

```rust
fn get_severity(input: &str, span: proc_macro2::Span) -> syn::Result<String>
```

