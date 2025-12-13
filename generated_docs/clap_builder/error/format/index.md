*[clap_builder](../../index.md) / [error](../index.md) / [format](index.md)*

---

# Module `format`

## Contents

- [Structs](#structs)
  - [`KindFormatter`](#kindformatter)
  - [`RichFormatter`](#richformatter)
  - [`Escape`](#escape)
- [Traits](#traits)
  - [`ErrorFormatter`](#errorformatter)
- [Functions](#functions)
  - [`start_error`](#start-error)
  - [`write_dynamic_context`](#write-dynamic-context)
  - [`write_values_list`](#write-values-list)
  - [`format_error_message`](#format-error-message)
  - [`singular_or_plural`](#singular-or-plural)
  - [`put_usage`](#put-usage)
  - [`get_help_flag`](#get-help-flag)
  - [`get_user_help_flag`](#get-user-help-flag)
  - [`try_help`](#try-help)
  - [`did_you_mean`](#did-you-mean)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`KindFormatter`](#kindformatter) | struct | Report [`ErrorKind`] |
| [`RichFormatter`](#richformatter) | struct | Richly formatted error context |
| [`Escape`](#escape) | struct |  |
| [`ErrorFormatter`](#errorformatter) | trait | Defines how to format an error for displaying to the user |
| [`start_error`](#start-error) | fn |  |
| [`write_dynamic_context`](#write-dynamic-context) | fn |  |
| [`write_values_list`](#write-values-list) | fn |  |
| [`format_error_message`](#format-error-message) | fn |  |
| [`singular_or_plural`](#singular-or-plural) | fn | Returns the singular or plural form on the verb to be based on the argument's value. |
| [`put_usage`](#put-usage) | fn |  |
| [`get_help_flag`](#get-help-flag) | fn |  |
| [`get_user_help_flag`](#get-user-help-flag) | fn |  |
| [`try_help`](#try-help) | fn |  |
| [`did_you_mean`](#did-you-mean) | fn |  |

## Structs

### `KindFormatter`

```rust
struct KindFormatter;
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:36`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L36)*

Report [`ErrorKind`](../kind/index.md)

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`](#richformatter).

</div>

#### Trait Implementations

##### `impl Any for KindFormatter`

- <span id="kindformatter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KindFormatter`

- <span id="kindformatter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KindFormatter`

- <span id="kindformatter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl ErrorFormatter for KindFormatter`

- <span id="kindformatter-errorformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](../index.md#error), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

##### `impl<T> From for KindFormatter`

- <span id="kindformatter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for KindFormatter`

- <span id="kindformatter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for KindFormatter`

- <span id="kindformatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kindformatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KindFormatter`

- <span id="kindformatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kindformatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RichFormatter`

```rust
struct RichFormatter;
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:62`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L62)*

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl Any for RichFormatter`

- <span id="richformatter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RichFormatter`

- <span id="richformatter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RichFormatter`

- <span id="richformatter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl ErrorFormatter for RichFormatter`

- <span id="richformatter-errorformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](../index.md#error), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

##### `impl<T> From for RichFormatter`

- <span id="richformatter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RichFormatter`

- <span id="richformatter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RichFormatter`

- <span id="richformatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="richformatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RichFormatter`

- <span id="richformatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="richformatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Escape<'s>`

```rust
struct Escape<'s>(&'s str);
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:495`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L495)*

#### Trait Implementations

##### `impl Any for Escape<'s>`

- <span id="escape-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Escape<'s>`

- <span id="escape-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Escape<'s>`

- <span id="escape-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Escape<'_>`

- <span id="escape-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> From for Escape<'s>`

- <span id="escape-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Escape<'s>`

- <span id="escape-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Escape<'s>`

- <span id="escape-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Escape<'s>`

- <span id="escape-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="escape-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Escape<'s>`

- <span id="escape-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="escape-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ErrorFormatter`

```rust
trait ErrorFormatter: Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:20-23`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L20-L23)*

Defines how to format an error for displaying to the user

#### Required Methods

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

  Stylize the error for the terminal

#### Implementors

- [`KindFormatter`](#kindformatter)
- [`RichFormatter`](#richformatter)

## Functions

### `start_error`

```rust
fn start_error(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles)
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:131-135`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L131-L135)*

### `write_dynamic_context`

```rust
fn write_dynamic_context(error: &crate::error::Error, styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles) -> bool
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:139-370`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L139-L370)*

### `write_values_list`

```rust
fn write_values_list(list_name: &'static str, styled: &mut crate::builder::StyledStr, valid: &anstyle::Style, possible_values: Option<&crate::error::ContextValue>)
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:373-394`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L373-L394)*

### `format_error_message`

```rust
fn format_error_message(message: &str, styles: &crate::builder::Styles, cmd: Option<&crate::builder::Command>, usage: Option<&crate::builder::StyledStr>) -> crate::builder::StyledStr
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:396-412`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L396-L412)*

### `singular_or_plural`

```rust
fn singular_or_plural(n: usize) -> &'static str
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:415-421`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L415-L421)*

Returns the singular or plural form on the verb to be based on the argument's value.

### `put_usage`

```rust
fn put_usage(styled: &mut crate::builder::StyledStr, usage: &crate::builder::StyledStr)
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:423-426`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L423-L426)*

### `get_help_flag`

```rust
fn get_help_flag(cmd: &crate::builder::Command) -> Option<std::borrow::Cow<'static, str>>
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:428-438`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L428-L438)*

### `get_user_help_flag`

```rust
fn get_user_help_flag(cmd: &crate::builder::Command) -> Option<String>
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:440-454`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L440-L454)*

### `try_help`

```rust
fn try_help(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles, help: Option<&str>)
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:456-467`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L456-L467)*

### `did_you_mean`

```rust
fn did_you_mean(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles, context: &str, possibles: &crate::error::ContextValue)
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:470-493`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L470-L493)*

