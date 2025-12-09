*[miette](../index.md) / [handler](index.md)*

---

# Module `handler`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`syscall`](#syscall) | mod |  |
| [`MietteHandlerOpts`](#miettehandleropts) | struct | Create a custom [`MietteHandler`] from options. |
| [`MietteHandler`](#miettehandler) | struct | A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a quasi-graphical way, using terminal colors, unicode drawing characters, and other such things. |
| [`RgbColors`](#rgbcolors) | enum | Settings to control the color format used for graphical rendering. |
| [`HighlighterOption`](#highlighteroption) | enum |  |

## Modules

- [`syscall`](syscall/index.md)

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

*Defined in [`miette-7.6.0/src/handler.rs:42-61`](../../../.source_1765210505/miette-7.6.0/src/handler.rs#L42-L61)*

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

- <span id="miettehandleropts-new"></span>`fn new() -> Self`

- <span id="miettehandleropts-terminal-links"></span>`fn terminal_links(self, linkify: bool) -> Self`

- <span id="miettehandleropts-graphical-theme"></span>`fn graphical_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../handlers/index.md)

- <span id="miettehandleropts-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../highlighters/index.md)

- <span id="miettehandleropts-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

- <span id="miettehandleropts-width"></span>`fn width(self, width: usize) -> Self`

- <span id="miettehandleropts-wrap-lines"></span>`fn wrap_lines(self, wrap_lines: bool) -> Self`

- <span id="miettehandleropts-break-words"></span>`fn break_words(self, break_words: bool) -> Self`

- <span id="miettehandleropts-word-separator"></span>`fn word_separator(self, word_separator: textwrap::WordSeparator) -> Self`

- <span id="miettehandleropts-word-splitter"></span>`fn word_splitter(self, word_splitter: textwrap::WordSplitter) -> Self`

- <span id="miettehandleropts-with-cause-chain"></span>`fn with_cause_chain(self) -> Self`

- <span id="miettehandleropts-without-cause-chain"></span>`fn without_cause_chain(self) -> Self`

- <span id="miettehandleropts-show-related-errors-as-siblings"></span>`fn show_related_errors_as_siblings(self) -> Self`

- <span id="miettehandleropts-show-related-errors-as-nested"></span>`fn show_related_errors_as_nested(self) -> Self`

- <span id="miettehandleropts-color"></span>`fn color(self, color: bool) -> Self`

- <span id="miettehandleropts-rgb-colors"></span>`fn rgb_colors(self, color: RgbColors) -> Self` — [`RgbColors`](../index.md)

- <span id="miettehandleropts-unicode"></span>`fn unicode(self, unicode: bool) -> Self`

- <span id="miettehandleropts-force-graphical"></span>`fn force_graphical(self, force: bool) -> Self`

- <span id="miettehandleropts-force-narrated"></span>`fn force_narrated(self, force: bool) -> Self`

- <span id="miettehandleropts-footer"></span>`fn footer(self, footer: String) -> Self`

- <span id="miettehandleropts-context-lines"></span>`fn context_lines(self, context_lines: usize) -> Self`

- <span id="miettehandleropts-tab-width"></span>`fn tab_width(self, width: usize) -> Self`

- <span id="miettehandleropts-build"></span>`fn build(self) -> MietteHandler` — [`MietteHandler`](../index.md)

- <span id="miettehandleropts-is-graphical"></span>`fn is_graphical(&self) -> bool`

- <span id="miettehandleropts-use-links"></span>`fn use_links(&self) -> bool`

- <span id="miettehandleropts-get-width"></span>`fn get_width(&self) -> usize`

#### Trait Implementations

##### `impl Clone for MietteHandlerOpts`

- <span id="miettehandleropts-clone"></span>`fn clone(&self) -> MietteHandlerOpts` — [`MietteHandlerOpts`](../index.md)

##### `impl Debug for MietteHandlerOpts`

- <span id="miettehandleropts-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MietteHandlerOpts`

- <span id="miettehandleropts-default"></span>`fn default() -> MietteHandlerOpts` — [`MietteHandlerOpts`](../index.md)

##### `impl OwoColorize for MietteHandlerOpts`

### `MietteHandler`

```rust
struct MietteHandler {
    inner: Box<dyn ReportHandler + Send + Sync>,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:387-389`](../../../.source_1765210505/miette-7.6.0/src/handler.rs#L387-L389)*

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

- <span id="miettehandler-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Default for MietteHandler`

- <span id="miettehandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for MietteHandler`

##### `impl ReportHandler for MietteHandler`

- <span id="miettehandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md)

## Enums

### `RgbColors`

```rust
enum RgbColors {
    Always,
    Preferred,
    Never,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:15-23`](../../../.source_1765210505/miette-7.6.0/src/handler.rs#L15-L23)*

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

- <span id="rgbcolors-clone"></span>`fn clone(&self) -> RgbColors` — [`RgbColors`](../index.md)

##### `impl Copy for RgbColors`

##### `impl Debug for RgbColors`

- <span id="rgbcolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RgbColors`

- <span id="rgbcolors-default"></span>`fn default() -> RgbColors` — [`RgbColors`](../index.md)

##### `impl Eq for RgbColors`

##### `impl OwoColorize for RgbColors`

##### `impl PartialEq for RgbColors`

- <span id="rgbcolors-eq"></span>`fn eq(&self, other: &RgbColors) -> bool` — [`RgbColors`](../index.md)

##### `impl StructuralPartialEq for RgbColors`

### `HighlighterOption`

```rust
enum HighlighterOption {
    Disable,
    EnableCustom(crate::highlighters::MietteHighlighter),
}
```

*Defined in [`miette-7.6.0/src/handler.rs:414-419`](../../../.source_1765210505/miette-7.6.0/src/handler.rs#L414-L419)*

#### Implementations

- <span id="highlighteroption-select"></span>`fn select(color: Option<bool>, highlighter: Option<MietteHighlighter>, supports_color: bool) -> HighlighterOption` — [`MietteHighlighter`](../highlighters/index.md), [`HighlighterOption`](#highlighteroption)

#### Trait Implementations

##### `impl Default for HighlighterOption`

- <span id="highlighteroption-default"></span>`fn default() -> Self`

##### `impl OwoColorize for HighlighterOption`

