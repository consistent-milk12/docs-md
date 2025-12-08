*[miette_derive](../index.md) / [forward](index.md)*

---

# Module `forward`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Forward`](#forward) | enum |  |
| [`WhichFn`](#whichfn) | enum |  |

## Enums

### `Forward`

```rust
enum Forward {
    Unnamed(usize),
    Named(syn::Ident),
}
```

#### Implementations

- <span id="forward-for-transparent-field"></span>`fn for_transparent_field(fields: &syn::Fields) -> syn::Result<Self>`

- <span id="forward-gen-struct-method"></span>`fn gen_struct_method(&self, which_fn: WhichFn) -> TokenStream` — [`WhichFn`](#whichfn)

- <span id="forward-gen-enum-match-arm"></span>`fn gen_enum_match_arm(&self, variant: &syn::Ident, which_fn: WhichFn) -> TokenStream` — [`WhichFn`](#whichfn)

#### Trait Implementations

##### `impl Parse for Forward`

- <span id="forward-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

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

- <span id="whichfn-method-call"></span>`fn method_call(&self) -> TokenStream`

- <span id="whichfn-signature"></span>`fn signature(&self) -> TokenStream`

- <span id="whichfn-catchall-arm"></span>`fn catchall_arm(&self) -> TokenStream`

#### Trait Implementations

##### `impl Clone for WhichFn`

- <span id="whichfn-clone"></span>`fn clone(&self) -> WhichFn` — [`WhichFn`](#whichfn)

##### `impl Copy for WhichFn`

