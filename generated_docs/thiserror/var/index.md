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

#### Trait Implementations

##### `impl<'a, T: Pointer + ?Sized> Pointer for Var<'a, T>`

- <span id="var-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

