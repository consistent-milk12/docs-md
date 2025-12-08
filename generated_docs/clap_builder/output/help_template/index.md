*[clap_builder](../../index.md) / [output](../index.md) / [help_template](index.md)*

---

# Module `help_template`

## Contents

- [Structs](#structs)
  - [`AutoHelp`](#autohelp)
  - [`HelpTemplate`](#helptemplate)
- [Functions](#functions)
  - [`positional_sort_key`](#positional_sort_key)
  - [`option_sort_key`](#option_sort_key)
  - [`dimensions`](#dimensions)
  - [`should_show_arg`](#should_show_arg)
  - [`should_show_subcommand`](#should_show_subcommand)
- [Type Aliases](#type-aliases)
  - [`ArgSortKey`](#argsortkey)
- [Constants](#constants)
  - [`DEFAULT_TEMPLATE`](#default_template)
  - [`DEFAULT_NO_ARGS_TEMPLATE`](#default_no_args_template)
  - [`SHORT_SIZE`](#short_size)
  - [`NEXT_LINE_INDENT`](#next_line_indent)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AutoHelp`](#autohelp) | struct | `clap` auto-generated help writer |
| [`HelpTemplate`](#helptemplate) | struct | Help template writer |
| [`positional_sort_key`](#positional_sort_key) | fn |  |
| [`option_sort_key`](#option_sort_key) | fn |  |
| [`dimensions`](#dimensions) | fn |  |
| [`should_show_arg`](#should_show_arg) | fn |  |
| [`should_show_subcommand`](#should_show_subcommand) | fn |  |
| [`ArgSortKey`](#argsortkey) | type |  |
| [`DEFAULT_TEMPLATE`](#default_template) | const |  |
| [`DEFAULT_NO_ARGS_TEMPLATE`](#default_no_args_template) | const |  |
| [`SHORT_SIZE`](#short_size) | const |  |
| [`NEXT_LINE_INDENT`](#next_line_indent) | const |  |

## Structs

### `AutoHelp<'cmd, 'writer>`

```rust
struct AutoHelp<'cmd, 'writer> {
    template: HelpTemplate<'cmd, 'writer>,
}
```

`clap` auto-generated help writer

#### Implementations

- <span id="autohelp-new"></span>`fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/index.md), [`Command`](../../index.md), [`Usage`](../usage/index.md)

- <span id="autohelp-write-help"></span>`fn write_help(&mut self)`

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

Help template writer

Wraps a writer stream providing different methods to generate help for `clap` objects.

#### Implementations

- <span id="helptemplate-write-display-name"></span>`fn write_display_name(&mut self)`

- <span id="helptemplate-write-bin-name"></span>`fn write_bin_name(&mut self)`

- <span id="helptemplate-write-version"></span>`fn write_version(&mut self)`

- <span id="helptemplate-write-author"></span>`fn write_author(&mut self, before_new_line: bool, after_new_line: bool)`

- <span id="helptemplate-write-about"></span>`fn write_about(&mut self, before_new_line: bool, after_new_line: bool)`

- <span id="helptemplate-write-before-help"></span>`fn write_before_help(&mut self)`

- <span id="helptemplate-write-after-help"></span>`fn write_after_help(&mut self)`

## Functions

### `positional_sort_key`

```rust
fn positional_sort_key(arg: &crate::builder::Arg) -> (usize, String)
```

### `option_sort_key`

```rust
fn option_sort_key(arg: &crate::builder::Arg) -> (usize, String)
```

### `dimensions`

```rust
fn dimensions() -> (Option<usize>, Option<usize>)
```

### `should_show_arg`

```rust
fn should_show_arg(use_long: bool, arg: &crate::builder::Arg) -> bool
```

### `should_show_subcommand`

```rust
fn should_show_subcommand(subcommand: &crate::builder::Command) -> bool
```

## Type Aliases

### `ArgSortKey`

```rust
type ArgSortKey = fn(&crate::builder::Arg) -> (usize, String);
```

## Constants

### `DEFAULT_TEMPLATE`

```rust
const DEFAULT_TEMPLATE: &str;
```

### `DEFAULT_NO_ARGS_TEMPLATE`

```rust
const DEFAULT_NO_ARGS_TEMPLATE: &str;
```

### `SHORT_SIZE`

```rust
const SHORT_SIZE: usize = 4usize;
```

### `NEXT_LINE_INDENT`

```rust
const NEXT_LINE_INDENT: &str;
```

