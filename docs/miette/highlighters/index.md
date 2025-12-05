*[miette](../index.md) / [highlighters](index.md)*

---

# Module `highlighters`

This module provides a trait for creating custom syntax highlighters that
highlight [`Diagnostic`](crate::Diagnostic) source code with ANSI escape
sequences when rendering with the [`GraphicalReportHighlighter`](crate::GraphicalReportHandler).

It also provides built-in highlighter implementations that you can use out of the box.
By default, there are no syntax highlighters exported by miette
(except for the no-op [`BlankHighlighter`](blank/index.md)).
To enable support for specific highlighters, you should enable their associated feature flag.

Currently supported syntax highlighters and their feature flags:
* `syntect-highlighter` - Enables [`syntect`](https://docs.rs/syntect/latest/syntect/) syntax highlighting support via the [`SyntectHighlighter`](#syntecthighlighter)


## Traits

### `Highlighter`

```rust
trait Highlighter { ... }
```

A syntax highlighter for highlighting miette [`SourceCode`](crate::SourceCode) snippets.

#### Required Methods

- `fn start_highlighter_state<'h>(self: &'h Self, source: &dyn SpanContents<'_>) -> Box<dyn HighlighterState>`

   Creates a new [`HighlighterState`](#highlighterstate) to begin parsing and highlighting

### `HighlighterState`

```rust
trait HighlighterState { ... }
```

A stateful highlighter that incrementally highlights lines of a particular
source code.

The [`GraphicalReportHandler`](crate::GraphicalReportHandler)
will create a highlighter state by calling
[`start_highlighter_state`](Highlighter::start_highlighter_state) at the
start of rendering, then it will iteratively call
[`highlight_line`](HighlighterState::highlight_line) to render individual
highlighted lines. This allows [`Highlighter`](#highlighter) implementations to maintain
mutable parsing and highlighting state.

#### Required Methods

- `fn highlight_line<'s>(self: &mut Self, line: &'s str) -> Vec<Styled<&'s str>>`

  Highlight an individual line from the source code by returning a vector of [Styled]

