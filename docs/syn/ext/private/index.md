*[syn](../../index.md) / [ext](../index.md) / [private](index.md)*

---

# Module `private`

## Structs

### `PeekFn`

```rust
struct PeekFn;
```

#### Trait Implementations

##### `impl Clone for PeekFn`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for PeekFn`

##### `impl Peek for private::PeekFn`

##### `impl Sealed for private::PeekFn`

### `IdentAny`

```rust
struct IdentAny;
```

#### Trait Implementations

##### `impl<T> Sealed for IdentAny`

##### `impl<T> Token for IdentAny`

- `fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../../buffer/index.md)

- `fn display() -> &'static str`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

