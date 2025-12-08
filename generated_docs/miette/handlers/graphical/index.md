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

- `fn render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../../index.md)

- `fn render_report_inner(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- `fn render_header(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, is_nested: bool) -> fmt::Result` — [`Diagnostic`](../../index.md)

- `fn render_causes(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- `fn render_footer(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../../index.md)

- `fn render_related(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- `fn render_snippets(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, opt_source: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../../index.md), [`SourceCode`](../../index.md)

- `fn render_context(self: &Self, f: &mut impl fmt::Write, source: &dyn SourceCode, context: &LabeledSpan, labels: &[LabeledSpan]) -> fmt::Result` — [`SourceCode`](../../index.md), [`LabeledSpan`](../../index.md)

- `fn render_multi_line_end(self: &Self, f: &mut impl fmt::Write, labels: &[FancySpan], max_gutter: usize, linum_width: usize, line: &Line, label: &FancySpan) -> fmt::Result` — [`FancySpan`](#fancyspan), [`Line`](#line)

- `fn render_line_gutter(self: &Self, f: &mut impl fmt::Write, max_gutter: usize, line: &Line, highlights: &[FancySpan]) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan)

- `fn render_highlight_gutter(self: &Self, f: &mut impl fmt::Write, max_gutter: usize, line: &Line, highlights: &[FancySpan], render_mode: LabelRenderMode) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan), [`LabelRenderMode`](#labelrendermode)

- `fn wrap(self: &Self, text: &str, opts: textwrap::Options<'_>) -> String`

- `fn write_linum(self: &Self, f: &mut impl fmt::Write, width: usize, linum: usize) -> fmt::Result`

- `fn write_no_linum(self: &Self, f: &mut impl fmt::Write, width: usize) -> fmt::Result`

- `fn line_visual_char_width<'a>(self: &Self, text: &'a str) -> impl Iterator<Item = usize> + 'a`

- `fn visual_offset(self: &Self, line: &Line, offset: usize, start: bool) -> usize` — [`Line`](#line)

- `fn render_line_text(self: &Self, f: &mut impl fmt::Write, text: &str) -> fmt::Result`

- `fn render_single_line_highlights(self: &Self, f: &mut impl fmt::Write, line: &Line, linum_width: usize, max_gutter: usize, single_liners: &[&FancySpan], all_highlights: &[FancySpan]) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan)

- `fn write_label_text(self: &Self, f: &mut impl fmt::Write, line: &Line, linum_width: usize, max_gutter: usize, all_highlights: &[FancySpan], chars: &ThemeCharacters, vbar_offsets: &[(&&FancySpan, usize)], hl: &&FancySpan, label: &str, render_mode: LabelRenderMode) -> fmt::Result` — [`Line`](#line), [`FancySpan`](#fancyspan), [`ThemeCharacters`](../index.md), [`LabelRenderMode`](#labelrendermode)

- `fn render_multi_line_end_single(self: &Self, f: &mut impl fmt::Write, label: &str, style: Style, render_mode: LabelRenderMode) -> fmt::Result` — [`LabelRenderMode`](#labelrendermode)

- `fn get_lines<'a>(self: &'a Self, source: &'a dyn SourceCode, context_span: &'a SourceSpan) -> Result<(Box<dyn SpanContents<'a>>, Vec<Line>), fmt::Error>` — [`SourceCode`](../../index.md), [`SourceSpan`](../../index.md), [`SpanContents`](../../index.md), [`Line`](#line)

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

