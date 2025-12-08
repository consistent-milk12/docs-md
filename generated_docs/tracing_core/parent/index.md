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

