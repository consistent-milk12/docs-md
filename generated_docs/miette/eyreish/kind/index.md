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

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:53`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L53)*

#### Implementations

- <span id="adhoc-new"></span>`fn new<M>(self, message: M) -> Report` — [`Report`](../../index.md#report)

#### Trait Implementations

##### `impl OwoColorize for Adhoc`

### `Trait`

```rust
struct Trait;
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:75`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L75)*

#### Implementations

- <span id="trait-new"></span>`fn new<E>(self, error: E) -> Report` — [`Report`](../../index.md#report)

#### Trait Implementations

##### `impl OwoColorize for Trait`

### `Boxed`

```rust
struct Boxed;
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:97`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L97)*

#### Implementations

- <span id="boxed-new"></span>`fn new(self, error: Box<dyn Diagnostic + Send + Sync>) -> Report` — [`Diagnostic`](../../index.md#diagnostic), [`Report`](../../index.md#report)

#### Trait Implementations

##### `impl OwoColorize for Boxed`

## Traits

### `AdhocKind`

```rust
trait AdhocKind: Sized { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:55-60`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L55-L60)*

#### Provided Methods

- `fn miette_kind(&self) -> Adhoc`

#### Implementors

- `&T`

### `TraitKind`

```rust
trait TraitKind: Sized { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:77-82`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L77-L82)*

#### Provided Methods

- `fn miette_kind(&self) -> Trait`

#### Implementors

- `E`

### `BoxedKind`

```rust
trait BoxedKind: Sized { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/kind.rs:99-104`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/kind.rs#L99-L104)*

#### Provided Methods

- `fn miette_kind(&self) -> Boxed`

#### Implementors

- `Box<dyn Diagnostic + Send + Sync>`

