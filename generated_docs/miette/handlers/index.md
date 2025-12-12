*[miette](../index.md) / [handlers](index.md)*

---

# Module `handlers`

Reporters included with `miette`.

## Contents

- [Modules](#modules)
  - [`debug`](#debug)
  - [`graphical`](#graphical)
  - [`json`](#json)
  - [`narratable`](#narratable)
  - [`theme`](#theme)
- [Structs](#structs)
  - [`DebugReportHandler`](#debugreporthandler)
  - [`GraphicalReportHandler`](#graphicalreporthandler)
  - [`Line`](#line)
  - [`FancySpan`](#fancyspan)
  - [`JSONReportHandler`](#jsonreporthandler)
  - [`Escape`](#escape)
  - [`NarratableReportHandler`](#narratablereporthandler)
  - [`Line`](#line)
  - [`GraphicalTheme`](#graphicaltheme)
  - [`ThemeStyles`](#themestyles)
  - [`ThemeCharacters`](#themecharacters)
- [Enums](#enums)
  - [`LinkStyle`](#linkstyle)
  - [`LabelRenderMode`](#labelrendermode)
  - [`SpanAttach`](#spanattach)
- [Functions](#functions)
  - [`split_label`](#split-label)
  - [`escape`](#escape)
  - [`safe_get_column`](#safe-get-column)
  - [`style`](#style)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`debug`](#debug) | mod |  |
| [`graphical`](#graphical) | mod |  |
| [`json`](#json) | mod |  |
| [`narratable`](#narratable) | mod |  |
| [`theme`](#theme) | mod |  |
| [`DebugReportHandler`](#debugreporthandler) | struct | [`ReportHandler`] that renders plain text and avoids extraneous graphics. |
| [`GraphicalReportHandler`](#graphicalreporthandler) | struct | A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a quasi-graphical way, using terminal colors, unicode drawing characters, and other such things. |
| [`Line`](#line) | struct |  |
| [`FancySpan`](#fancyspan) | struct |  |
| [`JSONReportHandler`](#jsonreporthandler) | struct | [`ReportHandler`] that renders JSON output. |
| [`Escape`](#escape) | struct |  |
| [`NarratableReportHandler`](#narratablereporthandler) | struct | [`ReportHandler`] that renders plain text and avoids extraneous graphics. |
| [`Line`](#line) | struct |  |
| [`GraphicalTheme`](#graphicaltheme) | struct | Theme used by [`GraphicalReportHandler`](crate::GraphicalReportHandler) to render fancy [`Diagnostic`](crate::Diagnostic) reports. |
| [`ThemeStyles`](#themestyles) | struct | Styles for various parts of graphical rendering for the [`GraphicalReportHandler`](crate::GraphicalReportHandler). |
| [`ThemeCharacters`](#themecharacters) | struct | Characters to be used when drawing when using [`GraphicalReportHandler`](crate::GraphicalReportHandler). |
| [`LinkStyle`](#linkstyle) | enum |  |
| [`LabelRenderMode`](#labelrendermode) | enum |  |
| [`SpanAttach`](#spanattach) | enum |  |
| [`split_label`](#split-label) | fn |  |
| [`escape`](#escape) | fn |  |
| [`safe_get_column`](#safe-get-column) | fn | Returns column at offset, and nearest boundary if offset is in the middle of the character |
| [`style`](#style) | fn |  |

## Modules

- [`debug`](debug/index.md)
- [`graphical`](graphical/index.md)
- [`json`](json/index.md)
- [`narratable`](narratable/index.md)
- [`theme`](theme/index.md)

## Structs

### `DebugReportHandler`

```rust
struct DebugReportHandler;
```

*Defined in [`miette-7.6.0/src/handlers/debug.rs:11`](../../../.source_1765210505/miette-7.6.0/src/handlers/debug.rs#L11)*

[`ReportHandler`](../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="debugreporthandler-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for DebugReportHandler`

- <span id="debugreporthandler-clone"></span>`fn clone(&self) -> DebugReportHandler` — [`DebugReportHandler`](#debugreporthandler)

##### `impl Debug for DebugReportHandler`

- <span id="debugreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DebugReportHandler`

- <span id="debugreporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for DebugReportHandler`

##### `impl ReportHandler for DebugReportHandler`

- <span id="debugreporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:26-42`](../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L26-L42)*

A [`ReportHandler`](../index.md) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`](#graphicaltheme) of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- <span id="graphicalreporthandler-new"></span>`fn new() -> Self`

- <span id="graphicalreporthandler-new-themed"></span>`fn new_themed(theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](#graphicaltheme)

- <span id="graphicalreporthandler-tab-width"></span>`fn tab_width(self, width: usize) -> Self`

- <span id="graphicalreporthandler-with-links"></span>`fn with_links(self, links: bool) -> Self`

- <span id="graphicalreporthandler-with-cause-chain"></span>`fn with_cause_chain(self) -> Self`

- <span id="graphicalreporthandler-without-cause-chain"></span>`fn without_cause_chain(self) -> Self`

- <span id="graphicalreporthandler-with-primary-span-start"></span>`fn with_primary_span_start(self) -> Self`

- <span id="graphicalreporthandler-without-primary-span-start"></span>`fn without_primary_span_start(self) -> Self`

- <span id="graphicalreporthandler-with-urls"></span>`fn with_urls(self, urls: bool) -> Self`

- <span id="graphicalreporthandler-with-theme"></span>`fn with_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](#graphicaltheme)

- <span id="graphicalreporthandler-with-width"></span>`fn with_width(self, width: usize) -> Self`

- <span id="graphicalreporthandler-with-wrap-lines"></span>`fn with_wrap_lines(self, wrap_lines: bool) -> Self`

- <span id="graphicalreporthandler-with-break-words"></span>`fn with_break_words(self, break_words: bool) -> Self`

- <span id="graphicalreporthandler-with-word-separator"></span>`fn with_word_separator(self, word_separator: textwrap::WordSeparator) -> Self`

- <span id="graphicalreporthandler-with-word-splitter"></span>`fn with_word_splitter(self, word_splitter: textwrap::WordSplitter) -> Self`

- <span id="graphicalreporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

- <span id="graphicalreporthandler-with-context-lines"></span>`fn with_context_lines(self, lines: usize) -> Self`

- <span id="graphicalreporthandler-with-show-related-as-nested"></span>`fn with_show_related_as_nested(self, show_related_as_nested: bool) -> Self`

- <span id="graphicalreporthandler-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../highlighters/index.md#highlighter)

- <span id="graphicalreporthandler-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

- <span id="graphicalreporthandler-with-link-display-text"></span>`fn with_link_display_text(self, text: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for GraphicalReportHandler`

- <span id="graphicalreporthandler-clone"></span>`fn clone(&self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](#graphicalreporthandler)

##### `impl Debug for GraphicalReportHandler`

- <span id="graphicalreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalReportHandler`

- <span id="graphicalreporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for GraphicalReportHandler`

##### `impl ReportHandler for GraphicalReportHandler`

- <span id="graphicalreporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    length: usize,
    text: String,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1388-1393`](../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1388-L1393)*

#### Implementations

- <span id="line-span-line-only"></span>`fn span_line_only(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-applies"></span>`fn span_applies(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-applies-gutter"></span>`fn span_applies_gutter(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-flyby"></span>`fn span_flyby(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-starts"></span>`fn span_starts(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-ends"></span>`fn span_ends(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1453-1460`](../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1453-L1460)*

#### Fields

- **`label`**: `Option<Vec<String>>`

  this is deliberately an option of a vec because I wanted to be very explicit
  that there can also be *no* label. If there is a label, it can have multiple
  lines which is what the vec is for.

#### Implementations

- <span id="fancyspan-new"></span>`fn new(label: Option<String>, span: SourceSpan, style: Style) -> Self` — [`SourceSpan`](../index.md#sourcespan)

- <span id="fancyspan-style"></span>`fn style(&self) -> Style`

- <span id="fancyspan-label"></span>`fn label(&self) -> Option<String>`

- <span id="fancyspan-label-parts"></span>`fn label_parts(&self) -> Option<Vec<String>>`

- <span id="fancyspan-offset"></span>`fn offset(&self) -> usize`

- <span id="fancyspan-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for FancySpan`

- <span id="fancyspan-clone"></span>`fn clone(&self) -> FancySpan` — [`FancySpan`](graphical/index.md#fancyspan)

##### `impl Debug for FancySpan`

- <span id="fancyspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for FancySpan`

##### `impl PartialEq for FancySpan`

- <span id="fancyspan-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `JSONReportHandler`

```rust
struct JSONReportHandler;
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:11`](../../../.source_1765210505/miette-7.6.0/src/handlers/json.rs#L11)*

[`ReportHandler`](../index.md) that renders JSON output. It's a machine-readable output.

#### Implementations

- <span id="jsonreporthandler-new"></span>`const fn new() -> Self`

#### Trait Implementations

##### `impl Clone for JSONReportHandler`

- <span id="jsonreporthandler-clone"></span>`fn clone(&self) -> JSONReportHandler` — [`JSONReportHandler`](#jsonreporthandler)

##### `impl Debug for JSONReportHandler`

- <span id="jsonreporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for JSONReportHandler`

- <span id="jsonreporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for JSONReportHandler`

##### `impl ReportHandler for JSONReportHandler`

- <span id="jsonreporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

### `Escape<'a>`

```rust
struct Escape<'a>(&'a str);
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:27`](../../../.source_1765210505/miette-7.6.0/src/handlers/json.rs#L27)*

#### Trait Implementations

##### `impl Display for Escape<'_>`

- <span id="escape-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for Escape<'a>`

##### `impl ToString for Escape<'a>`

- <span id="escape-to-string"></span>`fn to_string(&self) -> String`

### `NarratableReportHandler`

```rust
struct NarratableReportHandler {
    context_lines: usize,
    with_cause_chain: bool,
    footer: Option<String>,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:15-19`](../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L15-L19)*

[`ReportHandler`](../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="narratablereporthandler-new"></span>`const fn new() -> Self`

- <span id="narratablereporthandler-with-cause-chain"></span>`const fn with_cause_chain(self) -> Self`

- <span id="narratablereporthandler-without-cause-chain"></span>`const fn without_cause_chain(self) -> Self`

- <span id="narratablereporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

- <span id="narratablereporthandler-with-context-lines"></span>`const fn with_context_lines(self, lines: usize) -> Self`

#### Trait Implementations

##### `impl Clone for NarratableReportHandler`

- <span id="narratablereporthandler-clone"></span>`fn clone(&self) -> NarratableReportHandler` — [`NarratableReportHandler`](#narratablereporthandler)

##### `impl Debug for NarratableReportHandler`

- <span id="narratablereporthandler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NarratableReportHandler`

- <span id="narratablereporthandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for NarratableReportHandler`

##### `impl ReportHandler for NarratableReportHandler`

- <span id="narratablereporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    text: String,
    at_end_of_file: bool,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:360-365`](../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L360-L365)*

#### Implementations

- <span id="line-span-attach"></span>`fn span_attach(&self, span: &SourceSpan) -> Option<SpanAttach>` — [`SourceSpan`](../index.md#sourcespan), [`SpanAttach`](narratable/index.md#spanattach)

#### Trait Implementations

##### `impl OwoColorize for Line`

### `GraphicalTheme`

```rust
struct GraphicalTheme {
    pub characters: ThemeCharacters,
    pub styles: ThemeStyles,
}
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:17-22`](../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L17-L22)*

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

- <span id="graphicaltheme-clone"></span>`fn clone(&self) -> GraphicalTheme` — [`GraphicalTheme`](#graphicaltheme)

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

*Defined in [`miette-7.6.0/src/handlers/theme.rs:87-103`](../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L87-L103)*

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

- <span id="themestyles-clone"></span>`fn clone(&self) -> ThemeStyles` — [`ThemeStyles`](#themestyles)

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

*Defined in [`miette-7.6.0/src/handlers/theme.rs:167-195`](../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L167-L195)*

Characters to be used when drawing when using
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Implementations

- <span id="themecharacters-unicode"></span>`fn unicode() -> Self`

- <span id="themecharacters-emoji"></span>`fn emoji() -> Self`

- <span id="themecharacters-ascii"></span>`fn ascii() -> Self`

#### Trait Implementations

##### `impl Clone for ThemeCharacters`

- <span id="themecharacters-clone"></span>`fn clone(&self) -> ThemeCharacters` — [`ThemeCharacters`](#themecharacters)

##### `impl Debug for ThemeCharacters`

- <span id="themecharacters-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ThemeCharacters`

##### `impl OwoColorize for ThemeCharacters`

##### `impl PartialEq for ThemeCharacters`

- <span id="themecharacters-eq"></span>`fn eq(&self, other: &ThemeCharacters) -> bool` — [`ThemeCharacters`](#themecharacters)

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:45-49`](../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L45-L49)*

#### Trait Implementations

##### `impl Clone for LinkStyle`

- <span id="linkstyle-clone"></span>`fn clone(&self) -> LinkStyle` — [`LinkStyle`](graphical/index.md#linkstyle)

##### `impl Copy for LinkStyle`

##### `impl Debug for LinkStyle`

- <span id="linkstyle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LinkStyle`

##### `impl OwoColorize for LinkStyle`

##### `impl PartialEq for LinkStyle`

- <span id="linkstyle-eq"></span>`fn eq(&self, other: &LinkStyle) -> bool` — [`LinkStyle`](graphical/index.md#linkstyle)

##### `impl StructuralPartialEq for LinkStyle`

### `LabelRenderMode`

```rust
enum LabelRenderMode {
    SingleLine,
    MultiLineFirst,
    MultiLineRest,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1378-1385`](../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1378-L1385)*

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

- <span id="labelrendermode-eq"></span>`fn eq(&self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](graphical/index.md#labelrendermode)

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

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:367-371`](../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L367-L371)*

#### Trait Implementations

##### `impl OwoColorize for SpanAttach`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1468-1470`](../../../.source_1765210505/miette-7.6.0/src/handlers/graphical.rs#L1468-L1470)*

### `escape`

```rust
const fn escape(input: &str) -> Escape<'_>
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:52-54`](../../../.source_1765210505/miette-7.6.0/src/handlers/json.rs#L52-L54)*

### `safe_get_column`

```rust
fn safe_get_column(text: &str, offset: usize, start: bool) -> usize
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:375-392`](../../../.source_1765210505/miette-7.6.0/src/handlers/narratable.rs#L375-L392)*

Returns column at offset, and nearest boundary if offset is in the middle of
the character

### `style`

```rust
fn style() -> owo_colors::Style
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:105-107`](../../../.source_1765210505/miette-7.6.0/src/handlers/theme.rs#L105-L107)*

