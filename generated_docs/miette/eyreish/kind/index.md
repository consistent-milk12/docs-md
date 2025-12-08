*[miette](../../index.md) / [eyreish](../index.md) / [kind](index.md)*

---

# Module `kind`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Adhoc`](#adhoc) | struct |  |
| [`Trait`](#trait) | struct |  |
| [`Boxed`](#boxed) | struct |  |
| [`AdhocKind`](#adhockind) | trait |  |
| [`TraitKind`](#traitkind) | trait |  |
| [`BoxedKind`](#boxedkind) | trait |  |

## Structs

### `Adhoc`

```rust
struct Adhoc;
```

#### Implementations

- <span id="adhoc-new"></span>`fn new<M>(self, message: M) -> Report` — [`Report`](../../index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Adhoc`

### `Trait`

```rust
struct Trait;
```

#### Implementations

- <span id="trait-new"></span>`fn new<E>(self, error: E) -> Report` — [`Report`](../../index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Trait`

### `Boxed`

```rust
struct Boxed;
```

#### Implementations

- <span id="boxed-new"></span>`fn new(self, error: Box<dyn Diagnostic + Send + Sync>) -> Report` — [`Diagnostic`](../../index.md), [`Report`](../../index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Boxed`

## Traits

### `AdhocKind`

```rust
trait AdhocKind: Sized { ... }
```

#### Required Methods

- `fn miette_kind(&self) -> Adhoc`

### `TraitKind`

```rust
trait TraitKind: Sized { ... }
```

#### Required Methods

- `fn miette_kind(&self) -> Trait`

### `BoxedKind`

```rust
trait BoxedKind: Sized { ... }
```

#### Required Methods

- `fn miette_kind(&self) -> Boxed`

