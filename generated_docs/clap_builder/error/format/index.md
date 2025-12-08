*[clap_builder](../../index.md) / [error](../index.md) / [format](index.md)*

---

# Module `format`

## Structs

### `KindFormatter`

```rust
struct KindFormatter;
```

Report [`ErrorKind`](../index.md)

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`](../index.md).

</div>

#### Trait Implementations

##### `impl ErrorFormatter for KindFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](../index.md), [`StyledStr`](../../builder/index.md)

### `RichFormatter`

```rust
struct RichFormatter;
```

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl ErrorFormatter for RichFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](../index.md), [`StyledStr`](../../builder/index.md)

### `Escape<'s>`

```rust
struct Escape<'s>(&'s str);
```

#### Trait Implementations

##### `impl Display for Escape<'_>`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> ToString for Escape<'s>`

- `fn to_string(self: &Self) -> String`

## Traits

### `ErrorFormatter`

```rust
trait ErrorFormatter: Sized { ... }
```

Defines how to format an error for displaying to the user

#### Required Methods

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

  Stylize the error for the terminal

## Functions

### `start_error`

```rust
fn start_error(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles)
```

### `write_dynamic_context`

```rust
fn write_dynamic_context(error: &crate::error::Error, styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles) -> bool
```

### `write_values_list`

```rust
fn write_values_list(list_name: &'static str, styled: &mut crate::builder::StyledStr, valid: &anstyle::Style, possible_values: Option<&crate::error::ContextValue>)
```

### `format_error_message`

```rust
fn format_error_message(message: &str, styles: &crate::builder::Styles, cmd: Option<&crate::builder::Command>, usage: Option<&crate::builder::StyledStr>) -> crate::builder::StyledStr
```

### `singular_or_plural`

```rust
fn singular_or_plural(n: usize) -> &'static str
```

Returns the singular or plural form on the verb to be based on the argument's value.

### `put_usage`

```rust
fn put_usage(styled: &mut crate::builder::StyledStr, usage: &crate::builder::StyledStr)
```

### `get_help_flag`

```rust
fn get_help_flag(cmd: &crate::builder::Command) -> Option<std::borrow::Cow<'static, str>>
```

### `get_user_help_flag`

```rust
fn get_user_help_flag(cmd: &crate::builder::Command) -> Option<String>
```

### `try_help`

```rust
fn try_help(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles, help: Option<&str>)
```

### `did_you_mean`

```rust
fn did_you_mean(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles, context: &str, possibles: &crate::error::ContextValue)
```

