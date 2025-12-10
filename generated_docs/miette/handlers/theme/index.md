*[miette](../../index.md) / [handlers](../index.md) / [theme](index.md)*

---

# Module `theme`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GraphicalTheme`](#graphicaltheme) | struct | Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler) to render fancy [`Diagnostic`](crate::Diagnostic) reports. |
| [`ThemeStyles`](#themestyles) | struct | Styles for various parts of graphical rendering for the [`GraphicalReportHandler`](crate::GraphicalReportHandler). |
| [`ThemeCharacters`](#themecharacters) | struct | Characters to be used when drawing when using [`GraphicalReportHandler`](crate::GraphicalReportHandler). |
| [`style`](#style) | fn |  |

## Structs

### `GraphicalTheme`

```rust
struct GraphicalTheme {
    pub characters: ThemeCharacters,
    pub styles: ThemeStyles,
}
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:17-22`](../../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L17-L22)*

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

- <span id="graphicaltheme-ascii"></span>`fn ascii() -> Self`

- <span id="graphicaltheme-unicode"></span>`fn unicode() -> Self`

- <span id="graphicaltheme-unicode-nocolor"></span>`fn unicode_nocolor() -> Self`

- <span id="graphicaltheme-none"></span>`fn none() -> Self`

#### Trait Implementations

##### `impl Clone for GraphicalTheme`

- <span id="graphicaltheme-clone"></span>`fn clone(&self) -> GraphicalTheme` — [`GraphicalTheme`](../index.md#graphicaltheme)

##### `impl Debug for GraphicalTheme`

- <span id="graphicaltheme-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalTheme`

- <span id="graphicaltheme-default"></span>`fn default() -> Self`

##### `impl OwoColorize for GraphicalTheme`

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

*Defined in [`miette-7.6.0/src/handlers/theme.rs:87-103`](../../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L87-L103)*

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

- <span id="themestyles-rgb"></span>`fn rgb() -> Self`

- <span id="themestyles-ansi"></span>`fn ansi() -> Self`

- <span id="themestyles-none"></span>`fn none() -> Self`

#### Trait Implementations

##### `impl Clone for ThemeStyles`

- <span id="themestyles-clone"></span>`fn clone(&self) -> ThemeStyles` — [`ThemeStyles`](../index.md#themestyles)

##### `impl Debug for ThemeStyles`

- <span id="themestyles-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ThemeStyles`

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

*Defined in [`miette-7.6.0/src/handlers/theme.rs:167-195`](../../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L167-L195)*

Characters to be used when drawing when using
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Implementations

- <span id="themecharacters-unicode"></span>`fn unicode() -> Self`

- <span id="themecharacters-emoji"></span>`fn emoji() -> Self`

- <span id="themecharacters-ascii"></span>`fn ascii() -> Self`

#### Trait Implementations

##### `impl Clone for ThemeCharacters`

- <span id="themecharacters-clone"></span>`fn clone(&self) -> ThemeCharacters` — [`ThemeCharacters`](../index.md#themecharacters)

##### `impl Debug for ThemeCharacters`

- <span id="themecharacters-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ThemeCharacters`

##### `impl OwoColorize for ThemeCharacters`

##### `impl PartialEq for ThemeCharacters`

- <span id="themecharacters-eq"></span>`fn eq(&self, other: &ThemeCharacters) -> bool` — [`ThemeCharacters`](../index.md#themecharacters)

##### `impl StructuralPartialEq for ThemeCharacters`

## Functions

### `style`

```rust
fn style() -> owo_colors::Style
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:105-107`](../../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L105-L107)*

