*[clap_builder](../../index.md) / [builder](../index.md) / [debug_asserts](index.md)*

---

# Module `debug_asserts`

## Contents

- [Enums](#enums)
  - [`Flag`](#flag)
- [Functions](#functions)
  - [`assert_app`](#assert-app)
  - [`duplicate_tip`](#duplicate-tip)
  - [`detect_duplicate_flags`](#detect-duplicate-flags)
  - [`find_duplicates`](#find-duplicates)
  - [`assert_app_flags`](#assert-app-flags)
  - [`_verify_positionals`](#verify-positionals)
  - [`assert_arg`](#assert-arg)
  - [`assert_arg_flags`](#assert-arg-flags)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flag`](#flag) | enum |  |
| [`assert_app`](#assert-app) | fn |  |
| [`duplicate_tip`](#duplicate-tip) | fn |  |
| [`detect_duplicate_flags`](#detect-duplicate-flags) | fn |  |
| [`find_duplicates`](#find-duplicates) | fn | Find duplicates in a sorted array. |
| [`assert_app_flags`](#assert-app-flags) | fn |  |
| [`_verify_positionals`](#verify-positionals) | fn |  |
| [`assert_arg`](#assert-arg) | fn |  |
| [`assert_arg_flags`](#assert-arg-flags) | fn |  |

## Enums

### `Flag<'a>`

```rust
enum Flag<'a> {
    Command(String, &'a str),
    Arg(String, &'a str),
}
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:401-404`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L401-L404)*

#### Trait Implementations

##### `impl Any for Flag<'a>`

- <span id="flag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flag<'a>`

- <span id="flag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flag<'a>`

- <span id="flag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Eq for Flag<'a>`

##### `impl<T> From for Flag<'a>`

- <span id="flag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flag<'a>`

- <span id="flag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Flag<'_>`

- <span id="flag-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for Flag<'_>`

- <span id="flag-partialeq-eq"></span>`fn eq(&self, other: &Flag<'_>) -> bool` — [`Flag`](#flag)

##### `impl PartialOrd for Flag<'_>`

- <span id="flag-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Flag<'_>) -> Option<Ordering>` — [`Flag`](#flag)

##### `impl<U> TryFrom for Flag<'a>`

- <span id="flag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flag<'a>`

- <span id="flag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `assert_app`

```rust
fn assert_app(cmd: &crate::Command)
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:11-384`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L11-L384)*

### `duplicate_tip`

```rust
fn duplicate_tip(cmd: &crate::Command, first: &crate::Arg, second: &crate::Arg) -> &'static str
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:386-398`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L386-L398)*

### `detect_duplicate_flags`

```rust
fn detect_duplicate_flags(flags: &[Flag<'_>], short_or_long: &str)
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:435-454`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L435-L454)*

### `find_duplicates`

```rust
fn find_duplicates<T: PartialEq>(slice: &[T]) -> impl Iterator<Item = (&T, &T)>
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:460-468`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L460-L468)*

Find duplicates in a sorted array.

The algorithm is simple: the array is sorted, duplicates
must be placed next to each other, we can check only adjacent elements.

### `assert_app_flags`

```rust
fn assert_app_flags(cmd: &crate::Command)
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:470-491`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L470-L491)*

### `_verify_positionals`

```rust
fn _verify_positionals(cmd: &crate::Command) -> bool
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:494-678`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L494-L678)*

### `assert_arg`

```rust
fn assert_arg(arg: &crate::Arg)
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:680-788`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L680-L788)*

### `assert_arg_flags`

```rust
fn assert_arg_flags(arg: &crate::Arg)
```

*Defined in [`clap_builder-4.5.53/src/builder/debug_asserts.rs:790-818`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/debug_asserts.rs#L790-L818)*

