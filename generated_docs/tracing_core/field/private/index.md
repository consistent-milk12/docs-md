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

Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility.

#### Implementors

- `[(&'a Field, Option<&'a dyn Value>); N]`

