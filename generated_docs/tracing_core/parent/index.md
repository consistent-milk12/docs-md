*[tracing_core](../index.md) / [parent](index.md)*

---

# Module `parent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parent`](#parent) | enum |  |

## Enums

### `Parent`

```rust
enum Parent {
    Root,
    Current,
    Explicit(crate::span::Id),
}
```

*Defined in [`tracing-core-0.1.35/src/parent.rs:4-11`](../../../.source_1765210505/tracing-core-0.1.35/src/parent.rs#L4-L11)*

#### Variants

- **`Root`**

  The new span will be a root span.

- **`Current`**

  The new span will be rooted in the current span.

- **`Explicit`**

  The new span has an explicitly-specified parent.

#### Trait Implementations

##### `impl Debug for Parent`

- <span id="parent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

