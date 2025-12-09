*[miette](../../../index.md) / [eyreish](../../index.md) / [context](../index.md) / [ext](index.md)*

---

# Module `ext`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Diag`](#diag) | trait |  |

## Traits

### `Diag`

```rust
trait Diag { ... }
```

#### Required Methods

- `fn ext_report<D>(self, msg: D) -> Report`

#### Implementors

- [`BoxedError`](../../wrapper/index.md)
- [`ContextError`](../../error/index.md)
- [`DiagnosticError`](../../into_diagnostic/index.md)
- [`DisplayError`](../../wrapper/index.md)
- [`InstallError`](../../../index.md)
- [`MessageError`](../../wrapper/index.md)
- [`MietteDiagnostic`](../../../index.md)
- [`MietteError`](../../../index.md)
- [`Panic`](../../../panic/index.md)
- [`Report`](../../../index.md)
- [`WithSourceCode`](../../wrapper/index.md)
- `E`

