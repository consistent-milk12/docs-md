*[miette_derive](../index.md) / [diagnostic_arg](index.md)*

---

# Module `diagnostic_arg`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticArg`](#diagnosticarg) | enum |  |

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

*Defined in [`miette-derive-7.6.0/src/diagnostic_arg.rs:9-16`](../../../.source_1765210505/miette-derive-7.6.0/src/diagnostic_arg.rs#L9-L16)*

#### Trait Implementations

##### `impl Parse for DiagnosticArg`

- <span id="diagnosticarg-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

