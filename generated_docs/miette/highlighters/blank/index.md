*[miette](../../index.md) / [highlighters](../index.md) / [blank](index.md)*

---

# Module `blank`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BlankHighlighter`](#blankhighlighter) | struct | The default syntax highlighter. |
| [`BlankHighlighterState`](#blankhighlighterstate) | struct | The default highlighter state. |

## Structs

### `BlankHighlighter`

```rust
struct BlankHighlighter;
```

*Defined in [`miette-7.6.0/src/highlighters/blank.rs:10`](../../../../.source_1765633015/miette-7.6.0/src/highlighters/blank.rs#L10)*

The default syntax highlighter. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Any for BlankHighlighter`

- <span id="blankhighlighter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlankHighlighter`

- <span id="blankhighlighter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlankHighlighter`

- <span id="blankhighlighter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BlankHighlighter`

- <span id="blankhighlighter-clone"></span>`fn clone(&self) -> BlankHighlighter` — [`BlankHighlighter`](../index.md#blankhighlighter)

##### `impl CloneToUninit for BlankHighlighter`

- <span id="blankhighlighter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BlankHighlighter`

- <span id="blankhighlighter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BlankHighlighter`

- <span id="blankhighlighter-default"></span>`fn default() -> Self`

##### `impl<T> From for BlankHighlighter`

- <span id="blankhighlighter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Highlighter for BlankHighlighter`

- <span id="blankhighlighter-highlighter-start-highlighter-state"></span>`fn start_highlighter_state<'h>(self: &'h Self, _source: &dyn SpanContents<'_>) -> Box<dyn super::HighlighterState>` — [`SpanContents`](../../index.md#spancontents), [`HighlighterState`](../index.md#highlighterstate)

##### `impl<U> Into for BlankHighlighter`

- <span id="blankhighlighter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BlankHighlighter`

##### `impl ToOwned for BlankHighlighter`

- <span id="blankhighlighter-toowned-type-owned"></span>`type Owned = T`

- <span id="blankhighlighter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="blankhighlighter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BlankHighlighter`

- <span id="blankhighlighter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blankhighlighter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlankHighlighter`

- <span id="blankhighlighter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blankhighlighter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BlankHighlighterState`

```rust
struct BlankHighlighterState;
```

*Defined in [`miette-7.6.0/src/highlighters/blank.rs:30`](../../../../.source_1765633015/miette-7.6.0/src/highlighters/blank.rs#L30)*

The default highlighter state. It applies `Style::default()` to input text.
This is used by default when no syntax highlighting features are enabled.

#### Trait Implementations

##### `impl Any for BlankHighlighterState`

- <span id="blankhighlighterstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlankHighlighterState`

- <span id="blankhighlighterstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlankHighlighterState`

- <span id="blankhighlighterstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BlankHighlighterState`

- <span id="blankhighlighterstate-clone"></span>`fn clone(&self) -> BlankHighlighterState` — [`BlankHighlighterState`](../index.md#blankhighlighterstate)

##### `impl CloneToUninit for BlankHighlighterState`

- <span id="blankhighlighterstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BlankHighlighterState`

- <span id="blankhighlighterstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BlankHighlighterState`

- <span id="blankhighlighterstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl HighlighterState for BlankHighlighterState`

- <span id="blankhighlighterstate-highlighterstate-highlight-line"></span>`fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>>`

##### `impl<U> Into for BlankHighlighterState`

- <span id="blankhighlighterstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BlankHighlighterState`

##### `impl ToOwned for BlankHighlighterState`

- <span id="blankhighlighterstate-toowned-type-owned"></span>`type Owned = T`

- <span id="blankhighlighterstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="blankhighlighterstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BlankHighlighterState`

- <span id="blankhighlighterstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blankhighlighterstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlankHighlighterState`

- <span id="blankhighlighterstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blankhighlighterstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

