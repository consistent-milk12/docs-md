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

- `fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/index.md), [`Command`](../../index.md), [`Usage`](../usage/index.md)

- `fn term_w(cmd: &'cmd Command) -> usize` — [`Command`](../../index.md)

- `fn write_templated_help(self: &mut Self, template: &str)`

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

