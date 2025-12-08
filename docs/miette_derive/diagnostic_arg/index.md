*[miette_derive](../index.md) / [diagnostic_arg](index.md)*

---

# Module `diagnostic_arg`

## Enums

### `DiagnosticArg`

```rust
enum DiagnosticArg {
    Transparent,
    Code(crate::code::Code),
    Severity(crate::severity::Severity),
    Help(crate::help::Help),
    Url(crate::url::Url),
    Forward(crate::forward::Forward),
}
```

#### Trait Implementations

##### `impl Parse for DiagnosticArg`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

