*[syn](../../index.md) / [ext](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeekFn`](#peekfn) | struct |  |
| [`IdentAny`](#identany) | struct |  |
| [`Sealed`](#sealed) | trait |  |

## Structs

### `PeekFn`

```rust
struct PeekFn;
```

#### Trait Implementations

##### `impl Clone for PeekFn`

- <span id="peekfn-clone"></span>`fn clone(&self) -> Self`

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

- <span id="identany-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../../buffer/index.md)

- <span id="identany-display"></span>`fn display() -> &'static str`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

#### Implementors

- [`Ident`](../../index.md)

