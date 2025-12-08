*[miette_derive](../index.md) / [utils](index.md)*

---

# Module `utils`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MemberOrString`](#memberorstring) | enum |  |
| [`gen_all_variants_with`](#gen_all_variants_with) | fn |  |
| [`gen_unused_pat`](#gen_unused_pat) | fn |  |
| [`gen_fields_pat`](#gen_fields_pat) | fn | Goes in the slot `let Self #pat = self;` or `match self { Self #pat => ... |
| [`display_pat_members`](#display_pat_members) | fn | The returned tokens go in the slot `let Self #pat = self;` or `match self { |

## Enums

### `MemberOrString`

```rust
enum MemberOrString {
    Member(syn::Member),
    String(syn::LitStr),
}
```

#### Trait Implementations

##### `impl Parse for MemberOrString`

- <span id="memberorstring-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

##### `impl<T> Spanned for MemberOrString`

- <span id="memberorstring-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for MemberOrString`

- <span id="memberorstring-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Functions

### `gen_all_variants_with`

```rust
fn gen_all_variants_with(variants: &[crate::diagnostic::DiagnosticDef], which_fn: crate::forward::WhichFn, f: impl FnMut(&syn::Ident, &syn::Fields, &crate::diagnostic::DiagnosticConcreteArgs) -> Option<proc_macro2::TokenStream>) -> Option<proc_macro2::TokenStream>
```

### `gen_unused_pat`

```rust
fn gen_unused_pat(fields: &syn::Fields) -> proc_macro2::TokenStream
```

### `gen_fields_pat`

```rust
fn gen_fields_pat(fields: &syn::Fields) -> proc_macro2::TokenStream
```

Goes in the slot `let Self #pat = self;` or `match self { Self #pat => ...
}`.

### `display_pat_members`

```rust
fn display_pat_members(fields: &syn::Fields) -> (proc_macro2::TokenStream, std::collections::HashSet<syn::Member>)
```

The returned tokens go in the slot `let Self #pat = self;` or `match self {
Self #pat => ... }`. The members can be passed to
`Display::expand_shorthand[_cloned]`.

