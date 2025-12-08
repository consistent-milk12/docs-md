*[clap_builder](../../index.md) / [builder](../index.md) / [styled_str](index.md)*

---

# Module `styled_str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StyledStr`](#styledstr) | struct | Terminal-styling container |

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

- <span id="styledstr-new"></span>`const fn new() -> Self`

- <span id="styledstr-ansi"></span>`fn ansi(&self) -> impl std::fmt::Display + '_`

- <span id="styledstr-push-string"></span>`fn push_string(&mut self, msg: String)`

- <span id="styledstr-push-str"></span>`fn push_str(&mut self, msg: &str)`

- <span id="styledstr-trim-start-lines"></span>`fn trim_start_lines(&mut self)`

- <span id="styledstr-trim-end"></span>`fn trim_end(&mut self)`

- <span id="styledstr-replace-newline-var"></span>`fn replace_newline_var(&mut self)`

- <span id="styledstr-indent"></span>`fn indent(&mut self, initial: &str, trailing: &str)`

- <span id="styledstr-wrap"></span>`fn wrap(&mut self, _hard_width: usize)`

- <span id="styledstr-display-width"></span>`fn display_width(&self) -> usize`

- <span id="styledstr-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="styledstr-as-styled-str"></span>`fn as_styled_str(&self) -> &str`

- <span id="styledstr-iter-text"></span>`fn iter_text(&self) -> impl Iterator<Item = &str>`

- <span id="styledstr-push-styled"></span>`fn push_styled(&mut self, other: &Self)`

- <span id="styledstr-write-to"></span>`fn write_to(&self, buffer: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for StyledStr`

- <span id="styledstr-clone"></span>`fn clone(&self) -> StyledStr` — [`StyledStr`](../index.md)

##### `impl Debug for StyledStr`

- <span id="styledstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyledStr`

- <span id="styledstr-default"></span>`fn default() -> StyledStr` — [`StyledStr`](../index.md)

##### `impl Display for StyledStr`

- <span id="styledstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StyledStr`

##### `impl<I> IntoResettable for StyledStr`

- <span id="styledstr-into-resettable"></span>`fn into_resettable(self) -> Resettable<StyledStr>` — [`Resettable`](../index.md), [`StyledStr`](../index.md)

##### `impl Ord for StyledStr`

- <span id="styledstr-cmp"></span>`fn cmp(&self, other: &StyledStr) -> cmp::Ordering` — [`StyledStr`](../index.md)

##### `impl PartialEq for StyledStr`

- <span id="styledstr-eq"></span>`fn eq(&self, other: &StyledStr) -> bool` — [`StyledStr`](../index.md)

##### `impl PartialOrd for StyledStr`

- <span id="styledstr-partial-cmp"></span>`fn partial_cmp(&self, other: &StyledStr) -> option::Option<cmp::Ordering>` — [`StyledStr`](../index.md)

##### `impl StructuralPartialEq for StyledStr`

##### `impl<T> ToString for StyledStr`

- <span id="styledstr-to-string"></span>`fn to_string(&self) -> String`

##### `impl Write for StyledStr`

- <span id="styledstr-write-str"></span>`fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error>`

- <span id="styledstr-write-char"></span>`fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error>`

