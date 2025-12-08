*[clap_builder](../../index.md) / [builder](../index.md) / [styled_str](index.md)*

---

# Module `styled_str`

## Structs

### `StyledStr`

```rust
struct StyledStr(String);
```

Terminal-styling container

Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)

# Examples

```rust
use clap_builder as clap;
// `cstr!` converts tags to ANSI codes
let after_help: &'static str = color_print::cstr!(
r#"<bold><underline>Examples</underline></bold>

  <dim>$</dim> <bold>mybin --input file.toml</bold>
"#);

let cmd = clap::Command::new("mybin")
    .after_help(after_help)  // The `&str` gets converted into a `StyledStr`
    // ...
  ;
```

#### Implementations

- `const fn new() -> Self`

- `fn ansi(self: &Self) -> impl std::fmt::Display + '_`

- `fn push_string(self: &mut Self, msg: String)`

- `fn push_str(self: &mut Self, msg: &str)`

- `fn trim_start_lines(self: &mut Self)`

- `fn trim_end(self: &mut Self)`

- `fn replace_newline_var(self: &mut Self)`

- `fn indent(self: &mut Self, initial: &str, trailing: &str)`

- `fn wrap(self: &mut Self, _hard_width: usize)`

- `fn display_width(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn as_styled_str(self: &Self) -> &str`

- `fn iter_text(self: &Self) -> impl Iterator<Item = &str>`

- `fn push_styled(self: &mut Self, other: &Self)`

- `fn write_to(self: &Self, buffer: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for StyledStr`

- `fn clone(self: &Self) -> StyledStr` — [`StyledStr`](../index.md)

##### `impl Debug for StyledStr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StyledStr`

- `fn default() -> StyledStr` — [`StyledStr`](../index.md)

##### `impl Display for StyledStr`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StyledStr`

##### `impl<I> IntoResettable for StyledStr`

- `fn into_resettable(self: Self) -> Resettable<StyledStr>` — [`Resettable`](../index.md), [`StyledStr`](../index.md)

##### `impl Ord for StyledStr`

- `fn cmp(self: &Self, other: &StyledStr) -> $crate::cmp::Ordering` — [`StyledStr`](../index.md)

##### `impl PartialEq for StyledStr`

- `fn eq(self: &Self, other: &StyledStr) -> bool` — [`StyledStr`](../index.md)

##### `impl PartialOrd for StyledStr`

- `fn partial_cmp(self: &Self, other: &StyledStr) -> $crate::option::Option<$crate::cmp::Ordering>` — [`StyledStr`](../index.md)

##### `impl StructuralPartialEq for StyledStr`

##### `impl<T> ToString for StyledStr`

- `fn to_string(self: &Self) -> String`

##### `impl Write for StyledStr`

- `fn write_str(self: &mut Self, s: &str) -> Result<(), std::fmt::Error>`

- `fn write_char(self: &mut Self, c: char) -> Result<(), std::fmt::Error>`

