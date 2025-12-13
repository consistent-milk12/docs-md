*[tracing_core](../../index.md) / [field](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValidLen`](#validlen) | trait | Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility. |

## Traits

### `ValidLen<'a>`

```rust
trait ValidLen<'a>: Borrow<[(&'a Field, Option<&'a dyn Value>)]> { ... }
```

*Defined in [`tracing-core-0.1.35/src/field.rs:1152`](../../../../.source_1765633015/tracing-core-0.1.35/src/field.rs#L1152)*

Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility.

#### Implementors

- `[(&'a Field, Option<&'a dyn Value>); N]`

