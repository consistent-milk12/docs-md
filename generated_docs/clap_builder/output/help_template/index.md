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

- <span id="helptemplate-write-all-args"></span>`fn write_all_args(&mut self)`

- <span id="helptemplate-write-args"></span>`fn write_args(&mut self, args: &[&Arg], _category: &str, sort_key: fn(&crate::builder::Arg) -> (usize, String))` — [`Arg`](../../index.md)

- <span id="helptemplate-write-arg"></span>`fn write_arg(&mut self, arg: &Arg, next_line_help: bool, longest: usize)` — [`Arg`](../../index.md)

- <span id="helptemplate-short"></span>`fn short(&mut self, arg: &Arg)` — [`Arg`](../../index.md)

- <span id="helptemplate-long"></span>`fn long(&mut self, arg: &Arg)` — [`Arg`](../../index.md)

- <span id="helptemplate-align-to-about"></span>`fn align_to_about(&mut self, arg: &Arg, next_line_help: bool, longest: usize)` — [`Arg`](../../index.md)

- <span id="helptemplate-help"></span>`fn help(&mut self, arg: Option<&Arg>, about: &StyledStr, spec_vals: &str, next_line_help: bool, longest: usize)` — [`Arg`](../../index.md), [`StyledStr`](../../builder/index.md)

- <span id="helptemplate-will-args-wrap"></span>`fn will_args_wrap(&self, args: &[&Arg], longest: usize) -> bool` — [`Arg`](../../index.md)

- <span id="helptemplate-arg-next-line-help"></span>`fn arg_next_line_help(&self, arg: &Arg, spec_vals: &str, longest: usize) -> bool` — [`Arg`](../../index.md)

- <span id="helptemplate-spec-vals"></span>`fn spec_vals(&self, a: &Arg) -> String` — [`Arg`](../../index.md)

- <span id="helptemplate-get-spaces"></span>`fn get_spaces(&self, n: usize) -> String`

- <span id="helptemplate-write-padding"></span>`fn write_padding(&mut self, amount: usize)`

- <span id="helptemplate-use-long-pv"></span>`fn use_long_pv(&self, arg: &Arg) -> bool` — [`Arg`](../../index.md)

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

