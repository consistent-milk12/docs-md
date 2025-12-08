*[miette_derive](../index.md) / [diagnostic](index.md)*

---

# Module `diagnostic`

## Structs

### `DiagnosticDef`

```rust
struct DiagnosticDef {
    pub ident: syn::Ident,
    pub fields: syn::Fields,
    pub args: DiagnosticDefArgs,
}
```

### `DiagnosticConcreteArgs`

```rust
struct DiagnosticConcreteArgs {
    pub code: Option<crate::code::Code>,
    pub severity: Option<crate::severity::Severity>,
    pub help: Option<crate::help::Help>,
    pub labels: Option<crate::label::Labels>,
    pub source_code: Option<crate::source_code::SourceCode>,
    pub url: Option<crate::url::Url>,
    pub forward: Option<crate::forward::Forward>,
    pub related: Option<crate::related::Related>,
    pub diagnostic_source: Option<crate::diagnostic_source::DiagnosticSource>,
}
```

#### Implementations

- `fn for_fields(fields: &syn::Fields) -> Result<Self, syn::Error>`

- `fn add_args(self: &mut Self, attr: &syn::Attribute, args: impl Iterator<Item = DiagnosticArg>, errors: &mut Vec<syn::Error>)` — [`DiagnosticArg`](../diagnostic_arg/index.md)

#### Trait Implementations

##### `impl Default for DiagnosticConcreteArgs`

- `fn default() -> DiagnosticConcreteArgs` — [`DiagnosticConcreteArgs`](#diagnosticconcreteargs)

## Enums

### `Diagnostic`

```rust
enum Diagnostic {
    Struct {
        generics: syn::Generics,
        ident: syn::Ident,
        fields: syn::Fields,
        args: DiagnosticDefArgs,
    },
    Enum {
        ident: syn::Ident,
        generics: syn::Generics,
        variants: Vec<DiagnosticDef>,
    },
}
```

#### Implementations

- `fn from_derive_input(input: DeriveInput) -> Result<Self, syn::Error>`

- `fn gen(self: &Self) -> TokenStream`

### `DiagnosticDefArgs`

```rust
enum DiagnosticDefArgs {
    Transparent(crate::forward::Forward),
    Concrete(Box<DiagnosticConcreteArgs>),
}
```

#### Implementations

- `fn forward_or_override_enum(self: &Self, variant: &syn::Ident, which_fn: WhichFn, f: impl FnMut(&DiagnosticConcreteArgs) -> Option<TokenStream>) -> Option<TokenStream>` — [`WhichFn`](../forward/index.md), [`DiagnosticConcreteArgs`](#diagnosticconcreteargs)

