*[miette](../index.md) / [handlers](index.md)*

---

# Module `handlers`

Reporters included with `miette`.

## Modules

- [`debug`](debug/index.md) - 
- [`graphical`](graphical/index.md) - 
- [`json`](json/index.md) - 
- [`narratable`](narratable/index.md) - 
- [`theme`](theme/index.md) - 

## Structs

### `DebugReportHandler`

```rust
struct DebugReportHandler;
```

[`ReportHandler`](../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- `const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for DebugReportHandler`

- `fn clone(self: &Self) -> DebugReportHandler` — [`DebugReportHandler`](#debugreporthandler)

##### `impl Debug for DebugReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DebugReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for DebugReportHandler`

##### `impl ReportHandler for DebugReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md)

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

A [`ReportHandler`](../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`](#graphicaltheme) of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- `fn render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_report_inner(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_header(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, is_nested: bool) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_causes(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_footer(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_related(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_snippets(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, opt_source: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_context(self: &Self, f: &mut impl fmt::Write, source: &dyn SourceCode, context: &LabeledSpan, labels: &[LabeledSpan]) -> fmt::Result` — [`SourceCode`](../index.md), [`LabeledSpan`](../index.md)

- `fn render_multi_line_end(self: &Self, f: &mut impl fmt::Write, labels: &[FancySpan], max_gutter: usize, linum_width: usize, line: &Line, label: &FancySpan) -> fmt::Result` — [`FancySpan`](graphical/index.md), [`Line`](graphical/index.md)

- `fn render_line_gutter(self: &Self, f: &mut impl fmt::Write, max_gutter: usize, line: &Line, highlights: &[FancySpan]) -> fmt::Result` — [`Line`](graphical/index.md), [`FancySpan`](graphical/index.md)

- `fn render_highlight_gutter(self: &Self, f: &mut impl fmt::Write, max_gutter: usize, line: &Line, highlights: &[FancySpan], render_mode: LabelRenderMode) -> fmt::Result` — [`Line`](graphical/index.md), [`FancySpan`](graphical/index.md), [`LabelRenderMode`](graphical/index.md)

- `fn wrap(self: &Self, text: &str, opts: textwrap::Options<'_>) -> String`

- `fn write_linum(self: &Self, f: &mut impl fmt::Write, width: usize, linum: usize) -> fmt::Result`

- `fn write_no_linum(self: &Self, f: &mut impl fmt::Write, width: usize) -> fmt::Result`

- `fn line_visual_char_width<'a>(self: &Self, text: &'a str) -> impl Iterator<Item = usize> + 'a`

- `fn visual_offset(self: &Self, line: &Line, offset: usize, start: bool) -> usize` — [`Line`](graphical/index.md)

- `fn render_line_text(self: &Self, f: &mut impl fmt::Write, text: &str) -> fmt::Result`

- `fn render_single_line_highlights(self: &Self, f: &mut impl fmt::Write, line: &Line, linum_width: usize, max_gutter: usize, single_liners: &[&FancySpan], all_highlights: &[FancySpan]) -> fmt::Result` — [`Line`](graphical/index.md), [`FancySpan`](graphical/index.md)

- `fn write_label_text(self: &Self, f: &mut impl fmt::Write, line: &Line, linum_width: usize, max_gutter: usize, all_highlights: &[FancySpan], chars: &ThemeCharacters, vbar_offsets: &[(&&FancySpan, usize)], hl: &&FancySpan, label: &str, render_mode: LabelRenderMode) -> fmt::Result` — [`Line`](graphical/index.md), [`FancySpan`](graphical/index.md), [`ThemeCharacters`](#themecharacters), [`LabelRenderMode`](graphical/index.md)

- `fn render_multi_line_end_single(self: &Self, f: &mut impl fmt::Write, label: &str, style: Style, render_mode: LabelRenderMode) -> fmt::Result` — [`LabelRenderMode`](graphical/index.md)

- `fn get_lines<'a>(self: &'a Self, source: &'a dyn SourceCode, context_span: &'a SourceSpan) -> Result<(Box<dyn SpanContents<'a>>, Vec<Line>), fmt::Error>` — [`SourceCode`](../index.md), [`SourceSpan`](../index.md), [`SpanContents`](../index.md), [`Line`](graphical/index.md)

#### Trait Implementations

##### `impl Clone for GraphicalReportHandler`

- `fn clone(self: &Self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](#graphicalreporthandler)

##### `impl Debug for GraphicalReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for GraphicalReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for GraphicalReportHandler`

##### `impl ReportHandler for GraphicalReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md)

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

- `fn span_line_only(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md)

- `fn span_applies(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md)

- `fn span_applies_gutter(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md)

- `fn span_flyby(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md)

- `fn span_starts(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md)

- `fn span_ends(self: &Self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md)

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

- `fn new(label: Option<String>, span: SourceSpan, style: Style) -> Self` — [`SourceSpan`](../index.md)

- `fn style(self: &Self) -> Style`

- `fn label(self: &Self) -> Option<String>`

- `fn label_parts(self: &Self) -> Option<Vec<String>>`

- `fn offset(self: &Self) -> usize`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for FancySpan`

- `fn clone(self: &Self) -> FancySpan` — [`FancySpan`](graphical/index.md)

##### `impl Debug for FancySpan`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for FancySpan`

##### `impl PartialEq for FancySpan`

- `fn eq(self: &Self, other: &Self) -> bool`

### `JSONReportHandler`

```rust
struct JSONReportHandler;
```

[`ReportHandler`](../index.md) that renders JSON output. It's a machine-readable output.

#### Implementations

- `fn render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn _render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_snippets(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, source: &dyn SourceCode) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

#### Trait Implementations

##### `impl Clone for JSONReportHandler`

- `fn clone(self: &Self) -> JSONReportHandler` — [`JSONReportHandler`](#jsonreporthandler)

##### `impl Debug for JSONReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for JSONReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for JSONReportHandler`

##### `impl ReportHandler for JSONReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md)

### `Escape<'a>`

```rust
struct Escape<'a>(&'a str);
```

#### Trait Implementations

##### `impl Display for Escape<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Escape<'a>`

##### `impl<T> ToString for Escape<'a>`

- `fn to_string(self: &Self) -> String`

### `NarratableReportHandler`

```rust
struct NarratableReportHandler {
    context_lines: usize,
    with_cause_chain: bool,
    footer: Option<String>,
}
```

[`ReportHandler`](../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- `fn render_report(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_header(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_causes(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_footer(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic) -> fmt::Result` — [`Diagnostic`](../index.md)

- `fn render_related(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, parent_src: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_snippets(self: &Self, f: &mut impl fmt::Write, diagnostic: &dyn Diagnostic, source_code: Option<&dyn SourceCode>) -> fmt::Result` — [`Diagnostic`](../index.md), [`SourceCode`](../index.md)

- `fn render_context(self: &Self, f: &mut impl fmt::Write, source: &dyn SourceCode, context: &LabeledSpan, labels: &[LabeledSpan]) -> fmt::Result` — [`SourceCode`](../index.md), [`LabeledSpan`](../index.md)

- `fn get_lines<'a>(self: &'a Self, source: &'a dyn SourceCode, context_span: &'a SourceSpan) -> Result<(Box<dyn SpanContents<'a>>, Vec<Line>), fmt::Error>` — [`SourceCode`](../index.md), [`SourceSpan`](../index.md), [`SpanContents`](../index.md), [`Line`](narratable/index.md)

#### Trait Implementations

##### `impl Clone for NarratableReportHandler`

- `fn clone(self: &Self) -> NarratableReportHandler` — [`NarratableReportHandler`](#narratablereporthandler)

##### `impl Debug for NarratableReportHandler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NarratableReportHandler`

- `fn default() -> Self`

##### `impl<D> OwoColorize for NarratableReportHandler`

##### `impl ReportHandler for NarratableReportHandler`

- `fn debug(self: &Self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    text: String,
    at_end_of_file: bool,
}
```

#### Implementations

- `fn span_attach(self: &Self, span: &SourceSpan) -> Option<SpanAttach>` — [`SourceSpan`](../index.md), [`SpanAttach`](narratable/index.md)

#### Trait Implementations

##### `impl<D> OwoColorize for Line`

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

- `fn clone(self: &Self) -> GraphicalTheme` — [`GraphicalTheme`](#graphicaltheme)

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

- `fn clone(self: &Self) -> ThemeStyles` — [`ThemeStyles`](#themestyles)

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

- `fn clone(self: &Self) -> ThemeCharacters` — [`ThemeCharacters`](#themecharacters)

##### `impl Debug for ThemeCharacters`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ThemeCharacters`

##### `impl<D> OwoColorize for ThemeCharacters`

##### `impl PartialEq for ThemeCharacters`

- `fn eq(self: &Self, other: &ThemeCharacters) -> bool` — [`ThemeCharacters`](#themecharacters)

##### `impl StructuralPartialEq for ThemeCharacters`

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

- `fn clone(self: &Self) -> LinkStyle` — [`LinkStyle`](graphical/index.md)

##### `impl Copy for LinkStyle`

##### `impl Debug for LinkStyle`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LinkStyle`

##### `impl<D> OwoColorize for LinkStyle`

##### `impl PartialEq for LinkStyle`

- `fn eq(self: &Self, other: &LinkStyle) -> bool` — [`LinkStyle`](graphical/index.md)

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

- `fn eq(self: &Self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](graphical/index.md)

##### `impl StructuralPartialEq for LabelRenderMode`

### `SpanAttach`

```rust
enum SpanAttach {
    Contained {
        col_start: usize,
        col_end: usize,
    },
    Starts {
        col_start: usize,
    },
    Ends {
        col_end: usize,
    },
}
```

#### Trait Implementations

##### `impl<D> OwoColorize for SpanAttach`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

### `escape`

```rust
const fn escape(input: &str) -> Escape<'_>
```

### `safe_get_column`

```rust
fn safe_get_column(text: &str, offset: usize, start: bool) -> usize
```

Returns column at offset, and nearest boundary if offset is in the middle of
the character

### `style`

```rust
fn style() -> owo_colors::Style
```

