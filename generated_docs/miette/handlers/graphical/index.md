*[miette](../../index.md) / [handlers](../index.md) / [graphical](index.md)*

---

# Module `graphical`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GraphicalReportHandler`](#graphicalreporthandler) | struct | A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a quasi-graphical way, using terminal colors, unicode drawing characters, and other such things. |
| [`Line`](#line) | struct |  |
| [`FancySpan`](#fancyspan) | struct |  |
| [`LinkStyle`](#linkstyle) | enum |  |
| [`LabelRenderMode`](#labelrendermode) | enum |  |
| [`split_label`](#split_label) | fn |  |

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:26-42`](../../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L26-L42)*

A [`ReportHandler`](../../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`](../index.md) of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- <span id="graphicalreporthandler-new"></span>`fn new() -> Self`

- <span id="graphicalreporthandler-new-themed"></span>`fn new_themed(theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../index.md)

- <span id="graphicalreporthandler-tab-width"></span>`fn tab_width(self, width: usize) -> Self`

- <span id="graphicalreporthandler-with-links"></span>`fn with_links(self, links: bool) -> Self`

- <span id="graphicalreporthandler-with-cause-chain"></span>`fn with_cause_chain(self) -> Self`

- <span id="graphicalreporthandler-without-cause-chain"></span>`fn without_cause_chain(self) -> Self`

- <span id="graphicalreporthandler-with-primary-span-start"></span>`fn with_primary_span_start(self) -> Self`

- <span id="graphicalreporthandler-without-primary-span-start"></span>`fn without_primary_span_start(self) -> Self`

- <span id="graphicalreporthandler-with-urls"></span>`fn with_urls(self, urls: bool) -> Self`

- <span id="graphicalreporthandler-with-theme"></span>`fn with_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../index.md)

- <span id="graphicalreporthandler-with-width"></span>`fn with_width(self, width: usize) -> Self`

- <span id="graphicalreporthandler-with-wrap-lines"></span>`fn with_wrap_lines(self, wrap_lines: bool) -> Self`

- <span id="graphicalreporthandler-with-break-words"></span>`fn with_break_words(self, break_words: bool) -> Self`

- <span id="graphicalreporthandler-with-word-separator"></span>`fn with_word_separator(self, word_separator: textwrap::WordSeparator) -> Self`

- <span id="graphicalreporthandler-with-word-splitter"></span>`fn with_word_splitter(self, word_splitter: textwrap::WordSplitter) -> Self`

- <span id="graphicalreporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

- <span id="graphicalreporthandler-with-context-lines"></span>`fn with_context_lines(self, lines: usize) -> Self`

- <span id="graphicalreporthandler-with-show-related-as-nested"></span>`fn with_show_related_as_nested(self, show_related_as_nested: bool) -> Self`

- <span id="graphicalreporthandler-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../../highlighters/index.md)

- <span id="graphicalreporthandler-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

- <span id="graphicalreporthandler-with-link-display-text"></span>`fn with_link_display_text(self, text: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for GraphicalReportHandler`

- <span id="graphicalreporthandler-clone"></span>`fn clone(&self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](../index.md)

##### `impl Debug for GraphicalReportHandler`

- <span id="graphicalreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalReportHandler`

- <span id="graphicalreporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for GraphicalReportHandler`

##### `impl ReportHandler for GraphicalReportHandler`

- <span id="graphicalreporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    length: usize,
    text: String,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1388-1393`](../../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1388-L1393)*

#### Implementations

- <span id="line-span-line-only"></span>`fn span_line_only(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-applies"></span>`fn span_applies(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-applies-gutter"></span>`fn span_applies_gutter(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-flyby"></span>`fn span_flyby(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-starts"></span>`fn span_starts(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-ends"></span>`fn span_ends(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

#### Trait Implementations

##### `impl Debug for Line`

- <span id="line-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for Line`

### `FancySpan`

```rust
struct FancySpan {
    label: Option<Vec<String>>,
    span: crate::SourceSpan,
    style: owo_colors::Style,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1453-1460`](../../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1453-L1460)*

#### Fields

- **`label`**: `Option<Vec<String>>`

  this is deliberately an option of a vec because I wanted to be very explicit
  that there can also be *no* label. If there is a label, it can have multiple
  lines which is what the vec is for.

#### Implementations

- <span id="fancyspan-new"></span>`fn new(label: Option<String>, span: SourceSpan, style: Style) -> Self` — [`SourceSpan`](../../index.md)

- <span id="fancyspan-style"></span>`fn style(&self) -> Style`

- <span id="fancyspan-label"></span>`fn label(&self) -> Option<String>`

- <span id="fancyspan-label-parts"></span>`fn label_parts(&self) -> Option<Vec<String>>`

- <span id="fancyspan-offset"></span>`fn offset(&self) -> usize`

- <span id="fancyspan-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for FancySpan`

- <span id="fancyspan-clone"></span>`fn clone(&self) -> FancySpan` — [`FancySpan`](#fancyspan)

##### `impl Debug for FancySpan`

- <span id="fancyspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for FancySpan`

##### `impl PartialEq for FancySpan`

- <span id="fancyspan-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Enums

### `LinkStyle`

```rust
enum LinkStyle {
    None,
    Link,
    Text,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:45-49`](../../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L45-L49)*

#### Trait Implementations

##### `impl Clone for LinkStyle`

- <span id="linkstyle-clone"></span>`fn clone(&self) -> LinkStyle` — [`LinkStyle`](#linkstyle)

##### `impl Copy for LinkStyle`

##### `impl Debug for LinkStyle`

- <span id="linkstyle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LinkStyle`

##### `impl OwoColorize for LinkStyle`

##### `impl PartialEq for LinkStyle`

- <span id="linkstyle-eq"></span>`fn eq(&self, other: &LinkStyle) -> bool` — [`LinkStyle`](#linkstyle)

##### `impl StructuralPartialEq for LinkStyle`

### `LabelRenderMode`

```rust
enum LabelRenderMode {
    SingleLine,
    MultiLineFirst,
    MultiLineRest,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1378-1385`](../../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1378-L1385)*

#### Variants

- **`SingleLine`**

  we're rendering a single line label (or not rendering in any special way)

- **`MultiLineFirst`**

  we're rendering a multiline label

- **`MultiLineRest`**

  we're rendering the rest of a multiline label

#### Trait Implementations

##### `impl Debug for LabelRenderMode`

- <span id="labelrendermode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for LabelRenderMode`

##### `impl PartialEq for LabelRenderMode`

- <span id="labelrendermode-eq"></span>`fn eq(&self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](#labelrendermode)

##### `impl StructuralPartialEq for LabelRenderMode`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1468-1470`](../../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1468-L1470)*

