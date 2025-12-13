*[clap_builder](../../index.md) / [output](../index.md) / [help_template](index.md)*

---

# Module `help_template`

## Contents

- [Structs](#structs)
  - [`AutoHelp`](#autohelp)
  - [`HelpTemplate`](#helptemplate)
- [Functions](#functions)
  - [`positional_sort_key`](#positional-sort-key)
  - [`option_sort_key`](#option-sort-key)
  - [`dimensions`](#dimensions)
  - [`should_show_arg`](#should-show-arg)
  - [`should_show_subcommand`](#should-show-subcommand)
- [Type Aliases](#type-aliases)
  - [`ArgSortKey`](#argsortkey)
- [Constants](#constants)
  - [`DEFAULT_TEMPLATE`](#default-template)
  - [`DEFAULT_NO_ARGS_TEMPLATE`](#default-no-args-template)
  - [`SHORT_SIZE`](#short-size)
  - [`NEXT_LINE_INDENT`](#next-line-indent)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AutoHelp`](#autohelp) | struct | `clap` auto-generated help writer |
| [`HelpTemplate`](#helptemplate) | struct | Help template writer |
| [`positional_sort_key`](#positional-sort-key) | fn |  |
| [`option_sort_key`](#option-sort-key) | fn |  |
| [`dimensions`](#dimensions) | fn |  |
| [`should_show_arg`](#should-show-arg) | fn |  |
| [`should_show_subcommand`](#should-show-subcommand) | fn |  |
| [`ArgSortKey`](#argsortkey) | type |  |
| [`DEFAULT_TEMPLATE`](#default-template) | const |  |
| [`DEFAULT_NO_ARGS_TEMPLATE`](#default-no-args-template) | const |  |
| [`SHORT_SIZE`](#short-size) | const |  |
| [`NEXT_LINE_INDENT`](#next-line-indent) | const |  |

## Structs

### `AutoHelp<'cmd, 'writer>`

```rust
struct AutoHelp<'cmd, 'writer> {
    template: HelpTemplate<'cmd, 'writer>,
}
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:25-27`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L25-L27)*

`clap` auto-generated help writer

#### Implementations

- <span id="autohelp-new"></span>`fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/styled_str/index.md#styledstr), [`Command`](../../builder/command/index.md#command), [`Usage`](../usage/index.md#usage)

  Create a new `HelpTemplate` instance.

- <span id="autohelp-write-help"></span>`fn write_help(&mut self)`

#### Trait Implementations

##### `impl Any for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="autohelp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AutoHelp<'cmd, 'writer>`

- <span id="autohelp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="autohelp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HelpTemplate<'cmd, 'writer>`

```rust
struct HelpTemplate<'cmd, 'writer> {
    writer: &'writer mut crate::builder::StyledStr,
    cmd: &'cmd crate::builder::Command,
    styles: &'cmd crate::builder::Styles,
    usage: &'cmd self::usage::Usage<'cmd>,
    next_line_help: bool,
    term_w: usize,
    use_long: bool,
}
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:82-90`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L82-L90)*

Help template writer

Wraps a writer stream providing different methods to generate help for `clap` objects.

#### Implementations

- <span id="helptemplate-new"></span>`fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/styled_str/index.md#styledstr), [`Command`](../../builder/command/index.md#command), [`Usage`](../usage/index.md#usage)

  Create a new `HelpTemplate` instance.

- <span id="helptemplate-term-w"></span>`fn term_w(cmd: &'cmd Command) -> usize` — [`Command`](../../builder/command/index.md#command)

- <span id="helptemplate-write-templated-help"></span>`fn write_templated_help(&mut self, template: &str)`

  Write help to stream for the parser in the format defined by the template.

  

  For details about the template language see `Command::help_template`.

#### Trait Implementations

##### `impl Any for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="helptemplate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HelpTemplate<'cmd, 'writer>`

- <span id="helptemplate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="helptemplate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `positional_sort_key`

```rust
fn positional_sort_key(arg: &crate::builder::Arg) -> (usize, String)
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1083-1085`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1083-L1085)*

### `option_sort_key`

```rust
fn option_sort_key(arg: &crate::builder::Arg) -> (usize, String)
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1087-1108`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1087-L1108)*

### `dimensions`

```rust
fn dimensions() -> (Option<usize>, Option<usize>)
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1110-1118`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1110-L1118)*

### `should_show_arg`

```rust
fn should_show_arg(use_long: bool, arg: &crate::builder::Arg) -> bool
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1127-1139`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1127-L1139)*

### `should_show_subcommand`

```rust
fn should_show_subcommand(subcommand: &crate::builder::Command) -> bool
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1141-1143`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1141-L1143)*

## Type Aliases

### `ArgSortKey`

```rust
type ArgSortKey = fn(&crate::builder::Arg) -> (usize, String);
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1081`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1081)*

## Constants

### `DEFAULT_TEMPLATE`
```rust
const DEFAULT_TEMPLATE: &str;
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:65-70`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L65-L70)*

### `DEFAULT_NO_ARGS_TEMPLATE`
```rust
const DEFAULT_NO_ARGS_TEMPLATE: &str;
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:72-75`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L72-L75)*

### `SHORT_SIZE`
```rust
const SHORT_SIZE: usize = 4usize;
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:77`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L77)*

### `NEXT_LINE_INDENT`
```rust
const NEXT_LINE_INDENT: &str;
```

*Defined in [`clap_builder-4.5.53/src/output/help_template.rs:1079`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/help_template.rs#L1079)*

