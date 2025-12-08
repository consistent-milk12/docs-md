*[miette](../index.md) / [panic](index.md)*

---

# Module `panic`

## Structs

### `Panic`

```rust
struct Panic(String);
```

#### Implementations

- `fn backtrace() -> String`

#### Trait Implementations

##### `impl Debug for Panic`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E> Diag for Panic`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for Panic`

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

##### `impl Display for Panic`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for Panic`

##### `impl<D> OwoColorize for Panic`

##### `impl<T> ToString for Panic`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for Panic`

## Functions

### `set_panic_hook`

```rust
fn set_panic_hook()
```

Tells miette to render panics using its rendering engine.

