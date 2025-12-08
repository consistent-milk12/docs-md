*[miette](../index.md) / [handler](index.md)*

---

# Module `handler`

## Modules

- [`syscall`](syscall/index.md) - 

## Structs

### `MietteHandlerOpts`

```rust
struct MietteHandlerOpts {
    linkify: Option<bool>,
    width: Option<usize>,
    theme: Option<crate::GraphicalTheme>,
    force_graphical: Option<bool>,
    force_narrated: Option<bool>,
    rgb_colors: RgbColors,
    color: Option<bool>,
    unicode: Option<bool>,
    footer: Option<String>,
    context_lines: Option<usize>,
    tab_width: Option<usize>,
    with_cause_chain: Option<bool>,
    break_words: Option<bool>,
    wrap_lines: Option<bool>,
    word_separator: Option<textwrap::WordSeparator>,
    word_splitter: Option<textwrap::WordSplitter>,
    highlighter: Option<crate::highlighters::MietteHighlighter>,
    show_related_as_nested: Option<bool>,
}
```

Create a custom [`MietteHandler`](../index.md) from options.

## Example

```no_run
miette::set_hook(Box::new(|_| {
    Box::new(miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(false)
        .context_lines(3)
        .build())
}))
.unwrap();
```

#### Implementations

- `fn new() -> Self`

- `fn terminal_links(self: Self, linkify: bool) -> Self`

- `fn graphical_theme(self: Self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../handlers/index.md)

- `fn with_syntax_highlighting(self: Self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../highlighters/index.md)

- `fn without_syntax_highlighting(self: Self) -> Self`

- `fn width(self: Self, width: usize) -> Self`

- `fn wrap_lines(self: Self, wrap_lines: bool) -> Self`

- `fn break_words(self: Self, break_words: bool) -> Self`

- `fn word_separator(self: Self, word_separator: textwrap::WordSeparator) -> Self`

- `fn word_splitter(self: Self, word_splitter: textwrap::WordSplitter) -> Self`

- `fn with_cause_chain(self: Self) -> Self`

- `fn without_cause_chain(self: Self) -> Self`

- `fn show_related_errors_as_siblings(self: Self) -> Self`

- `fn show_related_errors_as_nested(self: Self) -> Self`

- `fn color(self: Self, color: bool) -> Self`

- `fn rgb_colors(self: Self, color: RgbColors) -> Self` — [`RgbColors`](../index.md)

- `fn unicode(self: Self, unicode: bool) -> Self`

- `fn force_graphical(self: Self, force: bool) -> Self`

- `fn force_narrated(self: Self, force: bool) -> Self`

- `fn footer(self: Self, footer: String) -> Self`

- `fn context_lines(self: Self, context_lines: usize) -> Self`

- `fn tab_width(self: Self, width: usize) -> Self`

- `fn build(self: Self) -> MietteHandler` — [`MietteHandler`](../index.md)

- `fn is_graphical(self: &Self) -> bool`

- `fn use_links(self: &Self) -> bool`

- `fn get_width(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for MietteHandlerOpts`

- `fn clone(self: &Self) -> MietteHandlerOpts` — [`MietteHandlerOpts`](../index.md)

##### `impl Debug for MietteHandlerOpts`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MietteHandlerOpts`

- `fn default() -> MietteHandlerOpts` — [`MietteHandlerOpts`](../index.md)

##### `impl<D> OwoColorize for MietteHandlerOpts`

### `MietteHandler`

```rust
struct MietteHandler {
    inner: Box<dyn ReportHandler + Send + Sync>,
}
```

A [`ReportHandler`](../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using
`GraphicalReportHandler::new_themed()` and handing it a [`GraphicalTheme`](../handlers/index.md) of
your own creation (or using one of its own defaults).

See [`set_hook`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Default for MietteHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for MietteHandler`

##### `impl ReportHandler for MietteHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md)

## Enums

### `RgbColors`

```rust
enum RgbColors {
    Always,
    Preferred,
    Never,
}
```

Settings to control the color format used for graphical rendering.

#### Variants

- **`Always`**

  Use RGB colors even if the terminal does not support them

- **`Preferred`**

  Use RGB colors instead of ANSI if the terminal supports RGB

- **`Never`**

  Always use ANSI, regardless of terminal support for RGB

#### Trait Implementations

##### `impl Clone for RgbColors`

- `fn clone(self: &Self) -> RgbColors` — [`RgbColors`](../index.md)

##### `impl Copy for RgbColors`

##### `impl Debug for RgbColors`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RgbColors`

- `fn default() -> RgbColors` — [`RgbColors`](../index.md)

##### `impl Eq for RgbColors`

##### `impl<D> OwoColorize for RgbColors`

##### `impl PartialEq for RgbColors`

- `fn eq(self: &Self, other: &RgbColors) -> bool` — [`RgbColors`](../index.md)

##### `impl StructuralPartialEq for RgbColors`

### `HighlighterOption`

```rust
enum HighlighterOption {
    Disable,
    EnableCustom(crate::highlighters::MietteHighlighter),
}
```

#### Implementations

- `fn select(color: Option<bool>, highlighter: Option<MietteHighlighter>, supports_color: bool) -> HighlighterOption` — [`MietteHighlighter`](../highlighters/index.md), [`HighlighterOption`](#highlighteroption)

#### Trait Implementations

##### `impl Default for HighlighterOption`

- `fn default() -> Self`

##### `impl<D> OwoColorize for HighlighterOption`

