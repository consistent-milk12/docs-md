*[thiserror](../index.md) / [var](index.md)*

---

# Module `var`

## Structs

### `Var<'a, T: ?Sized>`

```rust
struct Var<'a, T: ?Sized>(&'a T);
```

#### Trait Implementations

##### `impl<'a, T: Pointer + ?Sized> Pointer for Var<'a, T>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

