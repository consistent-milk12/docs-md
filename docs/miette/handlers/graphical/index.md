*[miette](../../index.md) / [handlers](../index.md) / [graphical](index.md)*

---

# Module `graphical`

## Structs

### `GraphicalReportHandler`

```rust
struct GraphicalReportHandler {
    links: LinkStyle,
    termwidth: usize,
    theme: GraphicalTheme,
    footer: Option<String>,
    context_lines: usize,
    tab_width: usize,
    with_cause_chain: bool,
    wrap_lines: bool,
    break_words: bool,
    with_primary_span_start: bool,
    word_separator: Option<textwrap::WordSeparator>,
    word_splitter: Option<textwrap::WordSplitter>,
    highlighter: crate::highlighters::MietteHighlighter,
    link_display_text: Option<String>,
    show_related_as_nested: bool,
}
```

A [`ReportHandler`](../../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`](../index.md) of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- `fn new() -> Self`

- `fn new_themed(theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../index.md)

- `fn tab_width(self: Self, width: usize) -> Self`

- `fn with_links(self: Self, links: bool) -> Self`

- `fn with_cause_chain(self: Self) -> Self`

- `fn without_cause_chain(self: Self) -> Self`

- `fn with_primary_span_start(self: Self) -> Self`

- `fn without_primary_span_start(self: Self) -> Self`

- `fn with_urls(self: Self, urls: bool) -> Self`

- `fn with_theme(self: Self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../index.md)

- `fn with_width(self: Self, width: usize) -> Self`

- `fn with_wrap_lines(self: Self, wrap_lines: bool) -> Self`

- `fn with_break_words(self: Self, break_words: bool) -> Self`

- `fn with_word_separator(self: Self, word_separator: textwrap::WordSeparator) -> Self`

- `fn with_word_splitter(self: Self, word_splitter: textwrap::WordSplitter) -> Self`

- `fn with_footer(self: Self, footer: String) -> Self`

- `fn with_context_lines(self: Self, lines: usize) -> Self`

- `fn with_show_related_as_nested(self: Self, show_related_as_nested: bool) -> Self`

- `fn with_syntax_highlighting(self: Self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../../highlighters/index.md)

- `fn without_syntax_highlighting(self: Self) -> Self`

- `fn with_link_display_text(self: Self, text: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for GraphicalReportHandler`

- `fn clone(self: &Self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](../index.md)

##### `impl Debug for GraphicalReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for GraphicalReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for GraphicalReportHandler`

##### `impl ReportHandler for GraphicalReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    length: usize,
    text: String,
}
```

#### Implementations

- `fn span_line_only(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- `fn span_applies(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- `fn span_applies_gutter(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- `fn span_flyby(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- `fn span_starts(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- `fn span_ends(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

#### Trait Implementations

##### `impl Debug for Line`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for Line`

### `FancySpan`

```rust
struct FancySpan {
    label: Option<Vec<String>>,
    span: crate::SourceSpan,
    style: owo_colors::Style,
}
```

#### Fields

- **`label`**: `Option<Vec<String>>`

  this is deliberately an option of a vec because I wanted to be very explicit
  that there can also be *no* label. If there is a label, it can have multiple
  lines which is what the vec is for.

#### Implementations

- `fn new(label: Option<String>, span: SourceSpan, style: Style) -> Self` — [`SourceSpan`](../../index.md)

- `fn style(self: &Self) -> Style`

- `fn label(self: &Self) -> Option<String>`

- `fn label_parts(self: &Self) -> Option<Vec<String>>`

- `fn offset(self: &Self) -> usize`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for FancySpan`

- `fn clone(self: &Self) -> FancySpan` — [`FancySpan`](#fancyspan)

##### `impl Debug for FancySpan`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for FancySpan`

##### `impl PartialEq for FancySpan`

- `fn eq(self: &Self, other: &Self) -> bool`

## Enums

### `LinkStyle`

```rust
enum LinkStyle {
    None,
    Link,
    Text,
}
```

#### Trait Implementations

##### `impl Clone for LinkStyle`

- `fn clone(self: &Self) -> LinkStyle` — [`LinkStyle`](#linkstyle)

##### `impl Copy for LinkStyle`

##### `impl Debug for LinkStyle`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LinkStyle`

##### `impl<D> OwoColorize for LinkStyle`

##### `impl PartialEq for LinkStyle`

- `fn eq(self: &Self, other: &LinkStyle) -> bool` — [`LinkStyle`](#linkstyle)

##### `impl StructuralPartialEq for LinkStyle`

### `LabelRenderMode`

```rust
enum LabelRenderMode {
    SingleLine,
    MultiLineFirst,
    MultiLineRest,
}
```

#### Variants

- **`SingleLine`**

  we're rendering a single line label (or not rendering in any special way)

- **`MultiLineFirst`**

  we're rendering a multiline label

- **`MultiLineRest`**

  we're rendering the rest of a multiline label

#### Trait Implementations

##### `impl Debug for LabelRenderMode`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for LabelRenderMode`

##### `impl PartialEq for LabelRenderMode`

- `fn eq(self: &Self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](#labelrendermode)

##### `impl StructuralPartialEq for LabelRenderMode`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

