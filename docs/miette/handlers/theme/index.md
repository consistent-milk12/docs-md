*[miette](../../index.md) / [handlers](../index.md) / [theme](index.md)*

---

# Module `theme`

## Structs

### `GraphicalTheme`

```rust
struct GraphicalTheme {
    pub characters: ThemeCharacters,
    pub styles: ThemeStyles,
}
```

Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler) to
render fancy [`Diagnostic`](crate::Diagnostic) reports.

A theme consists of two things: the set of characters to be used for drawing,
and the
[`owo_colors::Style`](https://docs.rs/owo-colors/latest/owo_colors/struct.Style.html)s to be used to paint various items.

You can create your own custom graphical theme using this type, or you can use
one of the predefined ones using the methods below.

#### Fields

- **`characters`**: `ThemeCharacters`

  Characters to be used for drawing.

- **`styles`**: `ThemeStyles`

  Styles to be used for painting.

#### Implementations

- `fn ascii() -> Self`

- `fn unicode() -> Self`

- `fn unicode_nocolor() -> Self`

- `fn none() -> Self`

#### Trait Implementations

##### `impl Clone for GraphicalTheme`

- `fn clone(self: &Self) -> GraphicalTheme` — [`GraphicalTheme`](../index.md)

##### `impl Debug for GraphicalTheme`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for GraphicalTheme`

- `fn default() -> Self`

##### `impl<D> OwoColorize for GraphicalTheme`

### `ThemeStyles`

```rust
struct ThemeStyles {
    pub error: owo_colors::Style,
    pub warning: owo_colors::Style,
    pub advice: owo_colors::Style,
    pub help: owo_colors::Style,
    pub link: owo_colors::Style,
    pub linum: owo_colors::Style,
    pub highlights: Vec<owo_colors::Style>,
}
```

Styles for various parts of graphical rendering for the
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Fields

- **`error`**: `owo_colors::Style`

  Style to apply to things highlighted as "error".

- **`warning`**: `owo_colors::Style`

  Style to apply to things highlighted as "warning".

- **`advice`**: `owo_colors::Style`

  Style to apply to things highlighted as "advice".

- **`help`**: `owo_colors::Style`

  Style to apply to the help text.

- **`link`**: `owo_colors::Style`

  Style to apply to filenames/links/URLs.

- **`linum`**: `owo_colors::Style`

  Style to apply to line numbers.

- **`highlights`**: `Vec<owo_colors::Style>`

  Styles to cycle through (using `.iter().cycle()`), to render the lines
  and text for diagnostic highlights.

#### Implementations

- `fn rgb() -> Self`

- `fn ansi() -> Self`

- `fn none() -> Self`

#### Trait Implementations

##### `impl Clone for ThemeStyles`

- `fn clone(self: &Self) -> ThemeStyles` — [`ThemeStyles`](../index.md)

##### `impl Debug for ThemeStyles`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for ThemeStyles`

### `ThemeCharacters`

```rust
struct ThemeCharacters {
    pub hbar: char,
    pub vbar: char,
    pub xbar: char,
    pub vbar_break: char,
    pub uarrow: char,
    pub rarrow: char,
    pub ltop: char,
    pub mtop: char,
    pub rtop: char,
    pub lbot: char,
    pub rbot: char,
    pub mbot: char,
    pub lbox: char,
    pub rbox: char,
    pub lcross: char,
    pub rcross: char,
    pub underbar: char,
    pub underline: char,
    pub error: String,
    pub warning: String,
    pub advice: String,
}
```

Characters to be used when drawing when using
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Implementations

- `fn unicode() -> Self`

- `fn emoji() -> Self`

- `fn ascii() -> Self`

#### Trait Implementations

##### `impl Clone for ThemeCharacters`

- `fn clone(self: &Self) -> ThemeCharacters` — [`ThemeCharacters`](../index.md)

##### `impl Debug for ThemeCharacters`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ThemeCharacters`

##### `impl<D> OwoColorize for ThemeCharacters`

##### `impl PartialEq for ThemeCharacters`

- `fn eq(self: &Self, other: &ThemeCharacters) -> bool` — [`ThemeCharacters`](../index.md)

##### `impl StructuralPartialEq for ThemeCharacters`

## Functions

### `style`

```rust
fn style() -> owo_colors::Style
```

