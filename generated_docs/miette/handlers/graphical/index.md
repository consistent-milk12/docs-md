*[miette](../../index.md) / [handlers](../index.md) / [graphical](index.md)*

---

# Module `graphical`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GraphicalReportHandler`](#graphicalreporthandler) | struct | A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a |
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

A [`ReportHandler`](../../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`](../index.md) of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- <span id="graphicalreporthandler-render-report"></span>`fn render_report(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../../index.md)

- <span id="graphicalreporthandler-render-report-inner"></span>`fn render_report_inner(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- <span id="graphicalreporthandler-render-header"></span>`fn render_header(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, is_nested: bool) -> fmt::Result` — [`Diagnostic`](../../index.md)

- <span id="graphicalreporthandler-render-causes"></span>`fn render_causes(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- <span id="graphicalreporthandler-render-footer"></span>`fn render_footer(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../../index.md)

- <span id="graphicalreporthandler-render-related"></span>`fn render_related(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- <span id="graphicalreporthandler-render-snippets"></span>`fn render_snippets(&self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, opt_source: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- <span id="graphicalreporthandler-render-context"></span>`fn render_context(&self, f: &mut impl fmt::Write, source: &dyn SourceCode, context: &LabeledSpan, labels: &[LabeledSpan]) -> fmt::Result` — [`SourceCode`](../../index.md), [`LabeledSpan`](../../index.md)

- <span id="graphicalreporthandler-render-multi-line-end"></span>`fn render_multi_line_end(&self, f: &mut impl fmt::Write, labels: &[FancySpan], max_gutter: usize, linum_width: usize, line: &Line, label: &FancySpan) -> fmt::Result` — [`FancySpan`](#fancyspan), [`Line`](#line)

- <span id="graphicalreporthandler-render-line-gutter"></span>`fn render_line_gutter(&self, f: &mut impl fmt::Write, max_gutter: usize, line: &Line, highlights: &[FancySpan]) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan)

- <span id="graphicalreporthandler-render-highlight-gutter"></span>`fn render_highlight_gutter(&self, f: &mut impl fmt::Write, max_gutter: usize, line: &Line, highlights: &[FancySpan], render_mode: LabelRenderMode) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan), [`LabelRenderMode`](#labelrendermode)

- <span id="graphicalreporthandler-wrap"></span>`fn wrap(&self, text: &str, opts: textwrap::Options<'_>) -> String`

- <span id="graphicalreporthandler-write-linum"></span>`fn write_linum(&self, f: &mut impl fmt::Write, width: usize, linum: usize) -> fmt::Result`

- <span id="graphicalreporthandler-write-no-linum"></span>`fn write_no_linum(&self, f: &mut impl fmt::Write, width: usize) -> fmt::Result`

- <span id="graphicalreporthandler-line-visual-char-width"></span>`fn line_visual_char_width<'a>(&self, text: &'a str) -> impl Iterator<Item = usize> + 'a`

- <span id="graphicalreporthandler-visual-offset"></span>`fn visual_offset(&self, line: &Line, offset: usize, start: bool) -> usize` — [`Line`](#line)

- <span id="graphicalreporthandler-render-line-text"></span>`fn render_line_text(&self, f: &mut impl fmt::Write, text: &str) -> fmt::Result`

- <span id="graphicalreporthandler-render-single-line-highlights"></span>`fn render_single_line_highlights(&self, f: &mut impl fmt::Write, line: &Line, linum_width: usize, max_gutter: usize, single_liners: &[&FancySpan], all_highlights: &[FancySpan]) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan)

- <span id="graphicalreporthandler-write-label-text"></span>`fn write_label_text(&self, f: &mut impl fmt::Write, line: &Line, linum_width: usize, max_gutter: usize, all_highlights: &[FancySpan], chars: &ThemeCharacters, vbar_offsets: &[(&&FancySpan, usize)], hl: &&FancySpan, label: &str, render_mode: LabelRenderMode) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan), [`ThemeCharacters`](../index.md), [`LabelRenderMode`](#labelrendermode)

- <span id="graphicalreporthandler-render-multi-line-end-single"></span>`fn render_multi_line_end_single(&self, f: &mut impl fmt::Write, label: &str, style: Style, render_mode: LabelRenderMode) -> fmt::Result` — [`LabelRenderMode`](#labelrendermode)

- <span id="graphicalreporthandler-get-lines"></span>`fn get_lines<'a>(self: &'a Self, source: &'a dyn SourceCode, context_span: &'a SourceSpan) -> Result<(Box<dyn SpanContents<'a>>, Vec<Line>), fmt::Error>` — [`SourceCode`](../../index.md), [`SourceSpan`](../../index.md), [`SpanContents`](../../index.md), [`Line`](#line)

#### Trait Implementations

##### `impl Clone for GraphicalReportHandler`

- <span id="graphicalreporthandler-clone"></span>`fn clone(&self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](../index.md)

##### `impl Debug for GraphicalReportHandler`

- <span id="graphicalreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalReportHandler`

- <span id="graphicalreporthandler-default"></span>`fn default() -> Self`

##### `impl<D> OwoColorize for GraphicalReportHandler`

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

##### `impl<D> OwoColorize for FancySpan`

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

#### Trait Implementations

##### `impl Clone for LinkStyle`

- <span id="linkstyle-clone"></span>`fn clone(&self) -> LinkStyle` — [`LinkStyle`](#linkstyle)

##### `impl Copy for LinkStyle`

##### `impl Debug for LinkStyle`

- <span id="linkstyle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LinkStyle`

##### `impl<D> OwoColorize for LinkStyle`

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

##### `impl<D> OwoColorize for LabelRenderMode`

##### `impl PartialEq for LabelRenderMode`

- <span id="labelrendermode-eq"></span>`fn eq(&self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](#labelrendermode)

##### `impl StructuralPartialEq for LabelRenderMode`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

