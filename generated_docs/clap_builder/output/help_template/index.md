*[clap_builder](../../index.md) / [output](../index.md) / [help_template](index.md)*

---

# Module `help_template`

## Structs

### `AutoHelp<'cmd, 'writer>`

```rust
struct AutoHelp<'cmd, 'writer> {
    template: HelpTemplate<'cmd, 'writer>,
}
```

`clap` auto-generated help writer

#### Implementations

- `fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/index.md), [`Command`](../../index.md), [`Usage`](../usage/index.md)

- `fn write_help(self: &mut Self)`

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

- `fn write_all_args(self: &mut Self)`

- `fn write_args(self: &mut Self, args: &[&Arg], _category: &str, sort_key: fn(&crate::builder::Arg) -> (usize, String))` — [`Arg`](../../index.md)

- `fn write_arg(self: &mut Self, arg: &Arg, next_line_help: bool, longest: usize)` — [`Arg`](../../index.md)

- `fn short(self: &mut Self, arg: &Arg)` — [`Arg`](../../index.md)

- `fn long(self: &mut Self, arg: &Arg)` — [`Arg`](../../index.md)

- `fn align_to_about(self: &mut Self, arg: &Arg, next_line_help: bool, longest: usize)` — [`Arg`](../../index.md)

- `fn help(self: &mut Self, arg: Option<&Arg>, about: &StyledStr, spec_vals: &str, next_line_help: bool, longest: usize)` — [`Arg`](../../index.md), [`StyledStr`](../../builder/index.md)

- `fn will_args_wrap(self: &Self, args: &[&Arg], longest: usize) -> bool` — [`Arg`](../../index.md)

- `fn arg_next_line_help(self: &Self, arg: &Arg, spec_vals: &str, longest: usize) -> bool` — [`Arg`](../../index.md)

- `fn spec_vals(self: &Self, a: &Arg) -> String` — [`Arg`](../../index.md)

- `fn get_spaces(self: &Self, n: usize) -> String`

- `fn write_padding(self: &mut Self, amount: usize)`

- `fn use_long_pv(self: &Self, arg: &Arg) -> bool` — [`Arg`](../../index.md)

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

