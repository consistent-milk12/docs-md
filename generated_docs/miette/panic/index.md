*[miette](../index.md) / [panic](index.md)*

---

# Module `panic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Panic`](#panic) | struct |  |
| [`set_panic_hook`](#set_panic_hook) | fn | Tells miette to render panics using its rendering engine. |

## Structs

### `Panic`

```rust
struct Panic(String);
```

#### Implementations

- <span id="panic-backtrace"></span>`fn backtrace() -> String`

#### Trait Implementations

##### `impl Debug for Panic`

- <span id="panic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for Panic`

- <span id="panic-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for Panic`

- <span id="panic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

##### `impl Display for Panic`

- <span id="panic-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for Panic`

##### `impl<D> OwoColorize for Panic`

##### `impl<T> ToString for Panic`

- <span id="panic-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for Panic`

## Functions

### `set_panic_hook`

```rust
fn set_panic_hook()
```

Tells miette to render panics using its rendering engine.

