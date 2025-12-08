*[miette_derive](../index.md) / [code](index.md)*

---

# Module `code`

## Structs

### `Code`

```rust
struct Code(String);
```

#### Implementations

- `fn gen_enum(variants: &[DiagnosticDef]) -> Option<TokenStream>` — [`DiagnosticDef`](../diagnostic/index.md)

- `fn gen_struct(self: &Self) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Debug for Code`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Parse for Code`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

