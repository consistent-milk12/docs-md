*[miette](../../index.md) / [eyreish](../index.md) / [kind](index.md)*

---

# Module `kind`

## Structs

### `Adhoc`

```rust
struct Adhoc;
```

#### Implementations

- `fn new<M>(self: Self, message: M) -> Report` — [`Report`](../../index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Adhoc`

### `Trait`

```rust
struct Trait;
```

#### Implementations

- `fn new<E>(self: Self, error: E) -> Report` — [`Report`](../../index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Trait`

### `Boxed`

```rust
struct Boxed;
```

#### Implementations

- `fn new(self: Self, error: Box<dyn Diagnostic + Send + Sync>) -> Report` — [`Diagnostic`](../../index.md), [`Report`](../../index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Boxed`

## Traits

### `AdhocKind`

```rust
trait AdhocKind: Sized { ... }
```

#### Required Methods

- `fn miette_kind(self: &Self) -> Adhoc`

### `TraitKind`

```rust
trait TraitKind: Sized { ... }
```

#### Required Methods

- `fn miette_kind(self: &Self) -> Trait`

### `BoxedKind`

```rust
trait BoxedKind: Sized { ... }
```

#### Required Methods

- `fn miette_kind(self: &Self) -> Boxed`

