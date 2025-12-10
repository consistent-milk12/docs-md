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

*Defined in [`syn-2.0.111/src/ext.rs:165`](../../../../.source_1765210505/syn-2.0.111/src/ext.rs#L165)*

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

*Defined in [`syn-2.0.111/src/ext.rs:168`](../../../../.source_1765210505/syn-2.0.111/src/ext.rs#L168)*

#### Trait Implementations

##### `impl Sealed for IdentAny`

##### `impl Token for IdentAny`

- <span id="identany-peek"></span>`fn peek(cursor: Cursor<'_>) -> bool` — [`Cursor`](../../buffer/index.md#cursor)

- <span id="identany-display"></span>`fn display() -> &'static str`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:160`](../../../../.source_1765210505/syn-2.0.111/src/ext.rs#L160)*

#### Implementors

- [`Ident`](../../ident/index.md#ident)

