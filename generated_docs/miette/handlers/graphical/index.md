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
| [`split_label`](#split-label) | fn |  |

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:26-42`](../../../../.source_1765633015/miette-7.6.0/src/handlers/graphical.rs#L26-L42)*

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

  Create a new `GraphicalReportHandler` with the default

  [`GraphicalTheme`](../index.md). This will use both unicode characters and colors.

- <span id="graphicalreporthandler-new-themed"></span>`fn new_themed(theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../index.md#graphicaltheme)

  Create a new `GraphicalReportHandler` with a given [`GraphicalTheme`](../index.md).

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

- <span id="graphicalreporthandler-with-theme"></span>`fn with_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](../index.md#graphicaltheme)

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

- <span id="graphicalreporthandler-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](../../highlighters/index.md#highlighter)

  Enable syntax highlighting for source code snippets, using the given

  [`Highlighter`](../../highlighters/index.md). See the [highlighters](crate::highlighters) crate

  for more details.

- <span id="graphicalreporthandler-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

  Disable syntax highlighting. This uses the

  [`crate::highlighters::BlankHighlighter`](../../highlighters/index.md) as a no-op highlighter.

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

- <span id="graphicalreporthandler-clone"></span>`fn clone(&self) -> GraphicalReportHandler` — [`GraphicalReportHandler`](../index.md#graphicalreporthandler)

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

- <span id="graphicalreporthandler-reporthandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](../../index.md#diagnostic)

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1388-1393`](../../../../.source_1765633015/miette-7.6.0/src/handlers/graphical.rs#L1388-L1393)*

#### Implementations

- <span id="line-span-line-only"></span>`fn span_line_only(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-applies"></span>`fn span_applies(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

  Returns whether `span` should be visible on this line, either in the gutter or under the

  text on this line

- <span id="line-span-applies-gutter"></span>`fn span_applies_gutter(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

  Returns whether `span` should be visible on this line in the gutter (so this excludes spans

  that are only visible on this line and do not span multiple lines)

- <span id="line-span-flyby"></span>`fn span_flyby(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-starts"></span>`fn span_starts(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

- <span id="line-span-ends"></span>`fn span_ends(&self, span: &FancySpan) -> bool` — [`FancySpan`](#fancyspan)

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1453-1460`](../../../../.source_1765633015/miette-7.6.0/src/handlers/graphical.rs#L1453-L1460)*

#### Fields

- **`label`**: `Option<Vec<String>>`

  this is deliberately an option of a vec because I wanted to be very explicit
  that there can also be *no* label. If there is a label, it can have multiple
  lines which is what the vec is for.

#### Implementations

- <span id="fancyspan-new"></span>`fn new(label: Option<String>, span: SourceSpan, style: Style) -> Self` — [`SourceSpan`](../../index.md#sourcespan)

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

- <span id="fancyspan-clone"></span>`fn clone(&self) -> FancySpan` — [`FancySpan`](#fancyspan)

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

## Enums

### `LinkStyle`

```rust
enum LinkStyle {
    None,
    Link,
    Text,
}
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:45-49`](../../../../.source_1765633015/miette-7.6.0/src/handlers/graphical.rs#L45-L49)*

#### Trait Implementations

##### `impl Any for LinkStyle`

- <span id="linkstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LinkStyle`

- <span id="linkstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LinkStyle`

- <span id="linkstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LinkStyle`

- <span id="linkstyle-clone"></span>`fn clone(&self) -> LinkStyle` — [`LinkStyle`](#linkstyle)

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

- <span id="linkstyle-partialeq-eq"></span>`fn eq(&self, other: &LinkStyle) -> bool` — [`LinkStyle`](#linkstyle)

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

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1378-1385`](../../../../.source_1765633015/miette-7.6.0/src/handlers/graphical.rs#L1378-L1385)*

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

- <span id="labelrendermode-partialeq-eq"></span>`fn eq(&self, other: &LabelRenderMode) -> bool` — [`LabelRenderMode`](#labelrendermode)

##### `impl StructuralPartialEq for LabelRenderMode`

##### `impl<U> TryFrom for LabelRenderMode`

- <span id="labelrendermode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="labelrendermode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LabelRenderMode`

- <span id="labelrendermode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="labelrendermode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `split_label`

```rust
fn split_label(v: String) -> Vec<String>
```

*Defined in [`miette-7.6.0/src/handlers/graphical.rs:1468-1470`](../../../../.source_1765633015/miette-7.6.0/src/handlers/graphical.rs#L1468-L1470)*

