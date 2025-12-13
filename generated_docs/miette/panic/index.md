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

*Defined in [`miette-7.6.0/src/panic.rs:30`](../../../.source_1765633015/miette-7.6.0/src/panic.rs#L30)*

#### Implementations

- <span id="panic-backtrace"></span>`fn backtrace() -> String`

#### Trait Implementations

##### `impl Any for Panic`

- <span id="panic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Panic`

- <span id="panic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Panic`

- <span id="panic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Panic`

- <span id="panic-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for Panic`

- <span id="panic-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md#report)

##### `impl Diagnostic for Panic`

- <span id="panic-diagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

##### `impl Display for Panic`

- <span id="panic-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for Panic`

##### `impl<T> From for Panic`

- <span id="panic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Panic`

- <span id="panic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Panic`

##### `impl ToString for Panic`

- <span id="panic-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for Panic`

##### `impl<U> TryFrom for Panic`

- <span id="panic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="panic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Panic`

- <span id="panic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="panic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `set_panic_hook`

```rust
fn set_panic_hook()
```

*Defined in [`miette-7.6.0/src/panic.rs:8-27`](../../../.source_1765633015/miette-7.6.0/src/panic.rs#L8-L27)*

Tells miette to render panics using its rendering engine.

