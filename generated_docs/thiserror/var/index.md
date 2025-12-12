*[thiserror](../index.md) / [var](index.md)*

---

# Module `var`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Var`](#var) | struct |  |

## Structs

### `Var<'a, T: ?Sized>`

```rust
struct Var<'a, T: ?Sized>(&'a T);
```

*Defined in [`thiserror-2.0.17/src/var.rs:3`](../../../.source_1765210505/thiserror-2.0.17/src/var.rs#L3)*

#### Trait Implementations

##### `impl<T: Pointer + ?Sized> Pointer for Var<'a, T>`

- <span id="var-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

