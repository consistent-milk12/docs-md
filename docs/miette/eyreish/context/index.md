*[miette](../../index.md) / [eyreish](../index.md) / [context](index.md)*

---

# Module `context`

## Modules

- [`ext`](ext/index.md) - 
- [`private`](private/index.md) - 

## Structs

### `Quoted<D>`

```rust
struct Quoted<D>(D);
```

#### Trait Implementations

##### `impl<D> Debug for Quoted<D>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Quoted<D>`

##### `impl Write for Quoted<&mut fmt::Formatter<'_>>`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

