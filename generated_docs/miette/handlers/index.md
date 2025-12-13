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

*Defined in [`miette-7.6.0/src/handlers/debug.rs:11`](../../../.source_1765521767/miette-7.6.0/src/handlers/debug.rs#L11)*

[`ReportHandler`](../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="debugreporthandler-new"></span>`const fn new() -> Self`

  Create a new [`NarratableReportHandler`](crate::NarratableReportHandler)

  There are no customization options.

#### Trait Implementations

##### `impl Any for DebugReportHandler`

- <span id="debugreporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugReportHandler`

- <span id="debugreporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugReportHandler`

- <span id="debugreporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DebugReportHandler`

- <span id="debugreporthandler-clone"></span>`fn clone(&self) -> DebugReportHandler` — [`DebugReportHandler`](#debugreporthandler)

##### `impl CloneToUninit for DebugReportHandler`

- <span id="debugreporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DebugReportHandler`

- <span id="debugreporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DebugReportHandler`

- <span id="debugreporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for DebugReportHandler`

- <span id="debugreporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugReportHandler`

- <span id="debugreporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DebugReportHandler`

##### `impl ReportHandler for DebugReportHandler`

- <span id="debugreporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

##### `impl ToOwned for DebugReportHandler`

- <span id="debugreporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="debugreporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugreporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugReportHandler`

- <span id="debugreporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugreporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugReportHandler`

- <span id="debugreporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugreporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:26-42`](../../../.source_1765521767/miette-7.6.0/src/handlers/graphical.rs#L26-L42)*

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

  Create a new `GraphicalReportHandler` with the default

  [`GraphicalTheme`](#graphicaltheme). This will use both unicode characters and colors.

- <span id="graphicalreporthandler-new-themed"></span>`fn new_themed(theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](#graphicaltheme)

  Create a new `GraphicalReportHandler` with a given [`GraphicalTheme`](#graphicaltheme).

- <span id="graphicalreporthandler-tab-width"></span>`fn tab_width(self, width: usize) -> Self`

  Set the displayed tab width in spaces.

- <span id="graphicalreporthandler-with-links"></span>`fn with_links(self, links: bool) -> Self`

  Whether to enable error code linkification using `Diagnostic::url()`.

- <span id="graphicalreporthandler-with-cause-chain"></span>`fn with_cause_chain(self) -> Self`

  Include the cause chain of the top-level error in the graphical output,

  if available.

- <span id="graphicalreporthandler-without-cause-chain"></span>`fn without_cause_chain(self) -> Self`

  Do not include the cause chain of the top-level error in the graphical

  output.

- <span id="graphicalreporthandler-with-primary-span-start"></span>`fn with_primary_span_start(self) -> Self`

  Include the line and column for the the start of the primary span when the

  snippet extends multiple lines

- <span id="graphicalreporthandler-without-primary-span-start"></span>`fn without_primary_span_start(self) -> Self`

  Do not include the line and column for the the start of the primary span

  when the snippet extends multiple lines

- <span id="graphicalreporthandler-with-urls"></span>`fn with_urls(self, urls: bool) -> Self`

  Whether to include `Diagnostic::url()` in the output.

  

  Disabling this is not recommended, but can be useful for more easily

  reproducible tests, as `url(docsrs)` links are version-dependent.

- <span id="graphicalreporthandler-with-theme"></span>`fn with_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](#graphicaltheme)

  Set a theme for this handler.

- <span id="graphicalreporthandler-with-width"></span>`fn with_width(self, width: usize) -> Self`

  Sets the width to wrap the report at.

- <span id="graphicalreporthandler-with-wrap-lines"></span>`fn with_wrap_lines(self, wrap_lines: bool) -> Self`

  Enables or disables wrapping of lines to fit the width.

- <span id="graphicalreporthandler-with-break-words"></span>`fn with_break_words(self, break_words: bool) -> Self`

  Enables or disables breaking of words during wrapping.

- <span id="graphicalreporthandler-with-word-separator"></span>`fn with_word_separator(self, word_separator: textwrap::WordSeparator) -> Self`

  Sets the word separator to use when wrapping.

- <span id="graphicalreporthandler-with-word-splitter"></span>`fn with_word_splitter(self, word_splitter: textwrap::WordSplitter) -> Self`

  Sets the word splitter to use when wrapping.

- <span id="graphicalreporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

  Sets the 'global' footer for this handler.

- <span id="graphicalreporthandler-with-context-lines"></span>`fn with_context_lines(self, lines: usize) -> Self`

  Sets the number of lines of context to show around each error.

- <span id="graphicalreporthandler-with-show-related-as-nested"></span>`fn with_show_related_as_nested(self, show_related_as_nested: bool) -> Self`

  Sets whether to render related errors as nested errors.

- <span id="graphicalreporthandler-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../highlighters/index.md#highlighter)

  Enable syntax highlighting for source code snippets, using the given

  [`Highlighter`](../highlighters/index.md). See the [highlighters](crate::highlighters) crate

  for more details.

- <span id="graphicalreporthandler-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

  Disable syntax highlighting. This uses the

  [`crate::highlighters::BlankHighlighter`](../highlighters/index.md) as a no-op highlighter.

- <span id="graphicalreporthandler-with-link-display-text"></span>`fn with_link_display_text(self, text: impl Into<String>) -> Self`

  Sets the display text for links.

  Miette displays `(link)` if this option is not set.

#### Trait Implementations

##### `impl Any for GraphicalReportHandler`

- <span id="graphicalreporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GraphicalReportHandler`

- <span id="graphicalreporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GraphicalReportHandler`

- <span id="graphicalreporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GraphicalReportHandler`

- <span id="graphicalreporthandler-clone"></span>`fn clone(&self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](#graphicalreporthandler)

##### `impl CloneToUninit for GraphicalReportHandler`

- <span id="graphicalreporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GraphicalReportHandler`

- <span id="graphicalreporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalReportHandler`

- <span id="graphicalreporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for GraphicalReportHandler`

- <span id="graphicalreporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GraphicalReportHandler`

- <span id="graphicalreporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GraphicalReportHandler`

##### `impl ReportHandler for GraphicalReportHandler`

- <span id="graphicalreporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

##### `impl ToOwned for GraphicalReportHandler`

- <span id="graphicalreporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="graphicalreporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="graphicalreporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GraphicalReportHandler`

- <span id="graphicalreporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="graphicalreporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GraphicalReportHandler`

- <span id="graphicalreporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="graphicalreporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    length: usize,
    text: String,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1388-1393`](../../../.source_1765521767/miette-7.6.0/src/handlers/graphical.rs#L1388-L1393)*

#### Implementations

- <span id="line-span-line-only"></span>`fn span_line_only(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-applies"></span>`fn span_applies(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

  Returns whether `span` should be visible on this line, either in the gutter or under the

  text on this line

- <span id="line-span-applies-gutter"></span>`fn span_applies_gutter(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

  Returns whether `span` should be visible on this line in the gutter (so this excludes spans

  that are only visible on this line and do not span multiple lines)

- <span id="line-span-flyby"></span>`fn span_flyby(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-starts"></span>`fn span_starts(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

- <span id="line-span-ends"></span>`fn span_ends(&self, span: &FancySpan) -> bool` — [`FancySpan`](graphical/index.md#fancyspan)

#### Trait Implementations

##### `impl Any for Line`

- <span id="line-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Line`

- <span id="line-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Line`

- <span id="line-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Line`

- <span id="line-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Line`

- <span id="line-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Line`

- <span id="line-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Line`

##### `impl<U> TryFrom for Line`

- <span id="line-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="line-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Line`

- <span id="line-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="line-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FancySpan`

```rust
struct FancySpan {
    label: Option<Vec<String>>,
    span: crate::SourceSpan,
    style: owo_colors::Style,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1453-1460`](../../../.source_1765521767/miette-7.6.0/src/handlers/graphical.rs#L1453-L1460)*

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

##### `impl Any for FancySpan`

- <span id="fancyspan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FancySpan`

- <span id="fancyspan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FancySpan`

- <span id="fancyspan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FancySpan`

- <span id="fancyspan-clone"></span>`fn clone(&self) -> FancySpan` — [`FancySpan`](graphical/index.md#fancyspan)

##### `impl CloneToUninit for FancySpan`

- <span id="fancyspan-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FancySpan`

- <span id="fancyspan-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FancySpan`

- <span id="fancyspan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FancySpan`

- <span id="fancyspan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for FancySpan`

##### `impl PartialEq for FancySpan`

- <span id="fancyspan-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for FancySpan`

- <span id="fancyspan-toowned-type-owned"></span>`type Owned = T`

- <span id="fancyspan-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fancyspan-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FancySpan`

- <span id="fancyspan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fancyspan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FancySpan`

- <span id="fancyspan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fancyspan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JSONReportHandler`

```rust
struct JSONReportHandler;
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:11`](../../../.source_1765521767/miette-7.6.0/src/handlers/json.rs#L11)*

[`ReportHandler`](../index.md) that renders JSON output. It's a machine-readable output.

#### Implementations

- <span id="jsonreporthandler-new"></span>`const fn new() -> Self`

  Create a new [`JSONReportHandler`](#jsonreporthandler). There are no customization

  options.

#### Trait Implementations

##### `impl Any for JSONReportHandler`

- <span id="jsonreporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JSONReportHandler`

- <span id="jsonreporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JSONReportHandler`

- <span id="jsonreporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for JSONReportHandler`

- <span id="jsonreporthandler-clone"></span>`fn clone(&self) -> JSONReportHandler` — [`JSONReportHandler`](#jsonreporthandler)

##### `impl CloneToUninit for JSONReportHandler`

- <span id="jsonreporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for JSONReportHandler`

- <span id="jsonreporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for JSONReportHandler`

- <span id="jsonreporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for JSONReportHandler`

- <span id="jsonreporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JSONReportHandler`

- <span id="jsonreporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for JSONReportHandler`

##### `impl ReportHandler for JSONReportHandler`

- <span id="jsonreporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

##### `impl ToOwned for JSONReportHandler`

- <span id="jsonreporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="jsonreporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="jsonreporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for JSONReportHandler`

- <span id="jsonreporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jsonreporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JSONReportHandler`

- <span id="jsonreporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jsonreporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Escape<'a>`

```rust
struct Escape<'a>(&'a str);
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:27`](../../../.source_1765521767/miette-7.6.0/src/handlers/json.rs#L27)*

#### Trait Implementations

##### `impl Any for Escape<'a>`

- <span id="escape-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Escape<'a>`

- <span id="escape-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Escape<'a>`

- <span id="escape-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Escape<'_>`

- <span id="escape-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Escape<'a>`

- <span id="escape-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Escape<'a>`

- <span id="escape-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Escape<'a>`

##### `impl ToString for Escape<'a>`

- <span id="escape-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Escape<'a>`

- <span id="escape-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="escape-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Escape<'a>`

- <span id="escape-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="escape-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NarratableReportHandler`

```rust
struct NarratableReportHandler {
    context_lines: usize,
    with_cause_chain: bool,
    footer: Option<String>,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:15-19`](../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L15-L19)*

[`ReportHandler`](../index.md) that renders plain text and avoids extraneous graphics.
It's optimized for screen readers and braille users, but is also used in any
non-graphical environments, such as non-TTY output.

#### Implementations

- <span id="narratablereporthandler-new"></span>`const fn new() -> Self`

  Create a new [`NarratableReportHandler`](#narratablereporthandler). There are no customization

  options.

- <span id="narratablereporthandler-with-cause-chain"></span>`const fn with_cause_chain(self) -> Self`

  Include the cause chain of the top-level error in the report, if

  available.

- <span id="narratablereporthandler-without-cause-chain"></span>`const fn without_cause_chain(self) -> Self`

  Do not include the cause chain of the top-level error in the report.

- <span id="narratablereporthandler-with-footer"></span>`fn with_footer(self, footer: String) -> Self`

  Set the footer to be displayed at the end of the report.

- <span id="narratablereporthandler-with-context-lines"></span>`const fn with_context_lines(self, lines: usize) -> Self`

  Sets the number of lines of context to show around each error.

#### Trait Implementations

##### `impl Any for NarratableReportHandler`

- <span id="narratablereporthandler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NarratableReportHandler`

- <span id="narratablereporthandler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NarratableReportHandler`

- <span id="narratablereporthandler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NarratableReportHandler`

- <span id="narratablereporthandler-clone"></span>`fn clone(&self) -> NarratableReportHandler` — [`NarratableReportHandler`](#narratablereporthandler)

##### `impl CloneToUninit for NarratableReportHandler`

- <span id="narratablereporthandler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NarratableReportHandler`

- <span id="narratablereporthandler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NarratableReportHandler`

- <span id="narratablereporthandler-default"></span>`fn default() -> Self`

##### `impl<T> From for NarratableReportHandler`

- <span id="narratablereporthandler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NarratableReportHandler`

- <span id="narratablereporthandler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for NarratableReportHandler`

##### `impl ReportHandler for NarratableReportHandler`

- <span id="narratablereporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../index.md#diagnostic)

##### `impl ToOwned for NarratableReportHandler`

- <span id="narratablereporthandler-toowned-type-owned"></span>`type Owned = T`

- <span id="narratablereporthandler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="narratablereporthandler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NarratableReportHandler`

- <span id="narratablereporthandler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="narratablereporthandler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NarratableReportHandler`

- <span id="narratablereporthandler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="narratablereporthandler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Line`

```rust
struct Line {
    line_number: usize,
    offset: usize,
    text: String,
    at_end_of_file: bool,
}
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:360-365`](../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L360-L365)*

#### Implementations

- <span id="line-span-attach"></span>`fn span_attach(&self, span: &SourceSpan) -> Option<SpanAttach>` — [`SourceSpan`](../index.md#sourcespan), [`SpanAttach`](narratable/index.md#spanattach)

#### Trait Implementations

##### `impl Any for Line`

- <span id="line-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Line`

- <span id="line-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Line`

- <span id="line-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Line`

- <span id="line-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Line`

- <span id="line-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Line`

##### `impl<U> TryFrom for Line`

- <span id="line-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="line-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Line`

- <span id="line-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="line-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GraphicalTheme`

```rust
struct GraphicalTheme {
    pub characters: ThemeCharacters,
    pub styles: ThemeStyles,
}
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:17-22`](../../../.source_1765521767/miette-7.6.0/src/handlers/theme.rs#L17-L22)*

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

  ASCII-art-based graphical drawing, with ANSI styling.

- <span id="graphicaltheme-unicode"></span>`fn unicode() -> Self`

  Graphical theme that draws using both ansi colors and unicode

  characters.

  

  Note that full rgb colors aren't enabled by default because they're

  an accessibility hazard, especially in the context of terminal themes

  that can change the background color and make hardcoded colors illegible.

  Such themes typically remap ansi codes properly, treating them more

  like CSS classes than specific colors.

- <span id="graphicaltheme-unicode-nocolor"></span>`fn unicode_nocolor() -> Self`

  Graphical theme that draws in monochrome, while still using unicode

  characters.

- <span id="graphicaltheme-none"></span>`fn none() -> Self`

  A "basic" graphical theme that skips colors and unicode characters and

  just does monochrome ascii art. If you want a completely non-graphical

  rendering of your [`Diagnostic`](crate::Diagnostic)s, check out

  [`NarratableReportHandler`](crate::NarratableReportHandler), or write

  your own [`ReportHandler`](crate::ReportHandler)

#### Trait Implementations

##### `impl Any for GraphicalTheme`

- <span id="graphicaltheme-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GraphicalTheme`

- <span id="graphicaltheme-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GraphicalTheme`

- <span id="graphicaltheme-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GraphicalTheme`

- <span id="graphicaltheme-clone"></span>`fn clone(&self) -> GraphicalTheme` — [`GraphicalTheme`](#graphicaltheme)

##### `impl CloneToUninit for GraphicalTheme`

- <span id="graphicaltheme-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GraphicalTheme`

- <span id="graphicaltheme-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GraphicalTheme`

- <span id="graphicaltheme-default"></span>`fn default() -> Self`

##### `impl<T> From for GraphicalTheme`

- <span id="graphicaltheme-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GraphicalTheme`

- <span id="graphicaltheme-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GraphicalTheme`

##### `impl ToOwned for GraphicalTheme`

- <span id="graphicaltheme-toowned-type-owned"></span>`type Owned = T`

- <span id="graphicaltheme-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="graphicaltheme-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GraphicalTheme`

- <span id="graphicaltheme-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="graphicaltheme-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GraphicalTheme`

- <span id="graphicaltheme-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="graphicaltheme-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miette-7.6.0/src/handlers/theme.rs:87-103`](../../../.source_1765521767/miette-7.6.0/src/handlers/theme.rs#L87-L103)*

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

  Nice RGB colors.

  [Credit](http://terminal.sexy/#FRUV0NDQFRUVrEFCkKlZ9L91ap-1qnWfdbWq0NDQUFBQrEFCkKlZ9L91ap-1qnWfdbWq9fX1).

- <span id="themestyles-ansi"></span>`fn ansi() -> Self`

  ANSI color-based styles.

- <span id="themestyles-none"></span>`fn none() -> Self`

  No styling. Just regular ol' monochrome.

#### Trait Implementations

##### `impl Any for ThemeStyles`

- <span id="themestyles-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThemeStyles`

- <span id="themestyles-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThemeStyles`

- <span id="themestyles-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ThemeStyles`

- <span id="themestyles-clone"></span>`fn clone(&self) -> ThemeStyles` — [`ThemeStyles`](#themestyles)

##### `impl CloneToUninit for ThemeStyles`

- <span id="themestyles-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ThemeStyles`

- <span id="themestyles-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ThemeStyles`

- <span id="themestyles-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThemeStyles`

- <span id="themestyles-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ThemeStyles`

##### `impl ToOwned for ThemeStyles`

- <span id="themestyles-toowned-type-owned"></span>`type Owned = T`

- <span id="themestyles-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="themestyles-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThemeStyles`

- <span id="themestyles-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="themestyles-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThemeStyles`

- <span id="themestyles-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="themestyles-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miette-7.6.0/src/handlers/theme.rs:167-195`](../../../.source_1765521767/miette-7.6.0/src/handlers/theme.rs#L167-L195)*

Characters to be used when drawing when using
[`GraphicalReportHandler`](crate::GraphicalReportHandler).

#### Implementations

- <span id="themecharacters-unicode"></span>`fn unicode() -> Self`

  Fancy unicode-based graphical elements.

- <span id="themecharacters-emoji"></span>`fn emoji() -> Self`

  Emoji-heavy unicode characters.

- <span id="themecharacters-ascii"></span>`fn ascii() -> Self`

  ASCII-art-based graphical elements. Works well on older terminals.

#### Trait Implementations

##### `impl Any for ThemeCharacters`

- <span id="themecharacters-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThemeCharacters`

- <span id="themecharacters-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThemeCharacters`

- <span id="themecharacters-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ThemeCharacters`

- <span id="themecharacters-clone"></span>`fn clone(&self) -> ThemeCharacters` — [`ThemeCharacters`](#themecharacters)

##### `impl CloneToUninit for ThemeCharacters`

- <span id="themecharacters-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ThemeCharacters`

- <span id="themecharacters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ThemeCharacters`

##### `impl<T> From for ThemeCharacters`

- <span id="themecharacters-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThemeCharacters`

- <span id="themecharacters-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ThemeCharacters`

##### `impl PartialEq for ThemeCharacters`

- <span id="themecharacters-partialeq-eq"></span>`fn eq(&self, other: &ThemeCharacters) -> bool` — [`ThemeCharacters`](#themecharacters)

##### `impl StructuralPartialEq for ThemeCharacters`

##### `impl ToOwned for ThemeCharacters`

- <span id="themecharacters-toowned-type-owned"></span>`type Owned = T`

- <span id="themecharacters-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="themecharacters-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThemeCharacters`

- <span id="themecharacters-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="themecharacters-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThemeCharacters`

- <span id="themecharacters-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="themecharacters-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `LinkStyle`

```rust
enum LinkStyle {
    None,
    Link,
    Text,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:45-49`](../../../.source_1765521767/miette-7.6.0/src/handlers/graphical.rs#L45-L49)*

#### Trait Implementations

##### `impl Any for LinkStyle`

- <span id="linkstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LinkStyle`

- <span id="linkstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LinkStyle`

- <span id="linkstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LinkStyle`

- <span id="linkstyle-clone"></span>`fn clone(&self) -> LinkStyle` — [`LinkStyle`](graphical/index.md#linkstyle)

##### `impl CloneToUninit for LinkStyle`

- <span id="linkstyle-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LinkStyle`

##### `impl Debug for LinkStyle`

- <span id="linkstyle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LinkStyle`

##### `impl<T> From for LinkStyle`

- <span id="linkstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LinkStyle`

- <span id="linkstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LinkStyle`

##### `impl PartialEq for LinkStyle`

- <span id="linkstyle-partialeq-eq"></span>`fn eq(&self, other: &LinkStyle) -> bool` — [`LinkStyle`](graphical/index.md#linkstyle)

##### `impl StructuralPartialEq for LinkStyle`

##### `impl ToOwned for LinkStyle`

- <span id="linkstyle-toowned-type-owned"></span>`type Owned = T`

- <span id="linkstyle-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linkstyle-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LinkStyle`

- <span id="linkstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linkstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LinkStyle`

- <span id="linkstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linkstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LabelRenderMode`

```rust
enum LabelRenderMode {
    SingleLine,
    MultiLineFirst,
    MultiLineRest,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1378-1385`](../../../.source_1765521767/miette-7.6.0/src/handlers/graphical.rs#L1378-L1385)*

#### Variants

- **`SingleLine`**

  we're rendering a single line label (or not rendering in any special way)

- **`MultiLineFirst`**

  we're rendering a multiline label

- **`MultiLineRest`**

  we're rendering the rest of a multiline label

#### Trait Implementations

##### `impl Any for LabelRenderMode`

- <span id="labelrendermode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LabelRenderMode`

- <span id="labelrendermode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LabelRenderMode`

- <span id="labelrendermode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LabelRenderMode`

- <span id="labelrendermode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LabelRenderMode`

- <span id="labelrendermode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LabelRenderMode`

- <span id="labelrendermode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LabelRenderMode`

##### `impl PartialEq for LabelRenderMode`

- <span id="labelrendermode-partialeq-eq"></span>`fn eq(&self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](graphical/index.md#labelrendermode)

##### `impl StructuralPartialEq for LabelRenderMode`

##### `impl<U> TryFrom for LabelRenderMode`

- <span id="labelrendermode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="labelrendermode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LabelRenderMode`

- <span id="labelrendermode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="labelrendermode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:367-371`](../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L367-L371)*

#### Trait Implementations

##### `impl Any for SpanAttach`

- <span id="spanattach-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpanAttach`

- <span id="spanattach-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpanAttach`

- <span id="spanattach-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SpanAttach`

- <span id="spanattach-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpanAttach`

- <span id="spanattach-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SpanAttach`

##### `impl<U> TryFrom for SpanAttach`

- <span id="spanattach-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="spanattach-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpanAttach`

- <span id="spanattach-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="spanattach-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1468-1470`](../../../.source_1765521767/miette-7.6.0/src/handlers/graphical.rs#L1468-L1470)*

### `escape`

```rust
const fn escape(input: &str) -> Escape<'_>
```

*Defined in [`miette-7.6.0/src/handlers/json.rs:52-54`](../../../.source_1765521767/miette-7.6.0/src/handlers/json.rs#L52-L54)*

### `safe_get_column`

```rust
fn safe_get_column(text: &str, offset: usize, start: bool) -> usize
```

*Defined in [`miette-7.6.0/src/handlers/narratable.rs:375-392`](../../../.source_1765521767/miette-7.6.0/src/handlers/narratable.rs#L375-L392)*

Returns column at offset, and nearest boundary if offset is in the middle of
the character

### `style`

```rust
fn style() -> owo_colors::Style
```

*Defined in [`miette-7.6.0/src/handlers/theme.rs:105-107`](../../../.source_1765521767/miette-7.6.0/src/handlers/theme.rs#L105-L107)*

