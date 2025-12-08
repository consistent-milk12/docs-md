*[tracing_core](../index.md) / [parent](index.md)*

---

# Module `parent`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

