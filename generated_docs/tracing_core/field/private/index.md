*[tracing_core](../../index.md) / [field](../index.md) / [private](index.md)*

---

# Module `private`

## Traits

### `ValidLen<'a>`

```rust
trait ValidLen<'a>: Borrow<[(&'a Field, Option<&'a dyn Value>)]> { ... }
```

Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility.

