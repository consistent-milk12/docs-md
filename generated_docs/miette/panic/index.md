*[miette](../index.md) / [panic](index.md)*

---

# Module `panic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Panic`](#panic) | struct |  |
| [`set_panic_hook`](#set-panic-hook) | fn | Tells miette to render panics using its rendering engine. |

## Structs

### `Panic`

```rust
struct Panic(String);
```

*Defined in [`miette-7.6.0/src/panic.rs:30`](../../../.source_1765521767/miette-7.6.0/src/panic.rs#L30)*

#### Implementations

- <span id="panic-backtrace"></span>`fn backtrace() -> String`

#### Trait Implementations

##### `impl Debug for Panic`

- <span id="panic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for Panic`

- <span id="panic-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md#report)

##### `impl Diagnostic for Panic`

- <span id="panic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

##### `impl Display for Panic`

- <span id="panic-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for Panic`

##### `impl OwoColorize for Panic`

##### `impl ToString for Panic`

- <span id="panic-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for Panic`

## Functions

### `set_panic_hook`

```rust
fn set_panic_hook()
```

*Defined in [`miette-7.6.0/src/panic.rs:8-27`](../../../.source_1765521767/miette-7.6.0/src/panic.rs#L8-L27)*

Tells miette to render panics using its rendering engine.

