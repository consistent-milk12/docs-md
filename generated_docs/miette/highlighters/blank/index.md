*[miette](../../index.md) / [highlighters](../index.md) / [blank](index.md)*

---

# Module `blank`

## Structs

### `BlankHighlighter`

```rust
struct BlankHighlighter;
```

The default syntax highlighter. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighter`

- `fn clone(self: &Self) -> BlankHighlighter` — [`BlankHighlighter`](../index.md)

##### `impl Debug for BlankHighlighter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BlankHighlighter`

- `fn default() -> Self`

##### `impl Highlighter for BlankHighlighter`

- `fn start_highlighter_state<'h>(self: &'h Self, _source: &dyn SpanContents<'_>) -> Box<dyn super::HighlighterState>` — [`SpanContents`](../../index.md), [`HighlighterState`](../index.md)

##### `impl<D> OwoColorize for BlankHighlighter`

### `BlankHighlighterState`

```rust
struct BlankHighlighterState;
```

The default highlighter state. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Clone for BlankHighlighterState`

- `fn clone(self: &Self) -> BlankHighlighterState` — [`BlankHighlighterState`](../index.md)

##### `impl Debug for BlankHighlighterState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl HighlighterState for BlankHighlighterState`

- `fn highlight_line<'s>(self: &mut Self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>>`

##### `impl<D> OwoColorize for BlankHighlighterState`

