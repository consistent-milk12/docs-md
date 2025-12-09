*[miette_derive](../index.md) / [diagnostic](index.md)*

---

# Module `diagnostic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticDef`](#diagnosticdef) | struct |  |
| [`DiagnosticConcreteArgs`](#diagnosticconcreteargs) | struct |  |
| [`Diagnostic`](#diagnostic) | enum |  |
| [`DiagnosticDefArgs`](#diagnosticdefargs) | enum |  |

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

- <span id="diagnosticconcreteargs-for-fields"></span>`fn for_fields(fields: &syn::Fields) -> Result<Self, syn::Error>`

- <span id="diagnosticconcreteargs-add-args"></span>`fn add_args(&mut self, attr: &syn::Attribute, args: impl Iterator<Item = DiagnosticArg>, errors: &mut Vec<syn::Error>)` — [`DiagnosticArg`](../diagnostic_arg/index.md)

#### Trait Implementations

##### `impl Default for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-default"></span>`fn default() -> DiagnosticConcreteArgs` — [`DiagnosticConcreteArgs`](#diagnosticconcreteargs)

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

- <span id="diagnostic-from-derive-input"></span>`fn from_derive_input(input: DeriveInput) -> Result<Self, syn::Error>`

- <span id="diagnostic-gen"></span>`fn gen(&self) -> TokenStream`

### `DiagnosticDefArgs`

```rust
enum DiagnosticDefArgs {
    Transparent(crate::forward::Forward),
    Concrete(Box<DiagnosticConcreteArgs>),
}
```

#### Implementations

- <span id="diagnosticdefargs-parse"></span>`fn parse(_ident: &syn::Ident, fields: &syn::Fields, attrs: &[&syn::Attribute], allow_transparent: bool) -> syn::Result<Self>`

