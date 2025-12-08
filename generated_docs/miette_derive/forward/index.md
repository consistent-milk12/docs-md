*[miette_derive](../index.md) / [forward](index.md)*

---

# Module `forward`

## Enums

### `Forward`

```rust
enum Forward {
    Unnamed(usize),
    Named(syn::Ident),
}
```

#### Implementations

- `fn for_transparent_field(fields: &syn::Fields) -> syn::Result<Self>`

- `fn gen_struct_method(self: &Self, which_fn: WhichFn) -> TokenStream` — [`WhichFn`](#whichfn)

- `fn gen_enum_match_arm(self: &Self, variant: &syn::Ident, which_fn: WhichFn) -> TokenStream` — [`WhichFn`](#whichfn)

#### Trait Implementations

##### `impl Parse for Forward`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

### `WhichFn`

```rust
enum WhichFn {
    Code,
    Help,
    Url,
    Severity,
    Labels,
    SourceCode,
    Related,
    DiagnosticSource,
}
```

#### Implementations

- `fn method_call(self: &Self) -> TokenStream`

- `fn signature(self: &Self) -> TokenStream`

- `fn catchall_arm(self: &Self) -> TokenStream`

#### Trait Implementations

##### `impl Clone for WhichFn`

- `fn clone(self: &Self) -> WhichFn` — [`WhichFn`](#whichfn)

##### `impl Copy for WhichFn`

