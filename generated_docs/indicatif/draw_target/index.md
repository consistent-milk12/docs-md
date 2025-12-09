*[indicatif](../index.md) / [draw_target](index.md)*

---

# Module `draw_target`

## Contents

- [Structs](#structs)
  - [`ProgressDrawTarget`](#progressdrawtarget)
  - [`DrawStateWrapper`](#drawstatewrapper)
  - [`RateLimiter`](#ratelimiter)
  - [`DrawState`](#drawstate)
  - [`VisualLines`](#visuallines)
- [Enums](#enums)
  - [`TargetKind`](#targetkind)
  - [`Drawable`](#drawable)
  - [`LineAdjust`](#lineadjust)
  - [`LineType`](#linetype)
- [Functions](#functions)
  - [`visual_line_count`](#visual_line_count)
- [Constants](#constants)
  - [`MAX_BURST`](#max_burst)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProgressDrawTarget`](#progressdrawtarget) | struct | Target for draw operations |
| [`DrawStateWrapper`](#drawstatewrapper) | struct |  |
| [`RateLimiter`](#ratelimiter) | struct |  |
| [`DrawState`](#drawstate) | struct | The drawn state of an element. |
| [`VisualLines`](#visuallines) | struct |  |
| [`TargetKind`](#targetkind) | enum |  |
| [`Drawable`](#drawable) | enum |  |
| [`LineAdjust`](#lineadjust) | enum |  |
| [`LineType`](#linetype) | enum |  |
| [`visual_line_count`](#visual_line_count) | fn | Calculate the number of visual lines in the given lines, after accounting for line wrapping and non-printable characters. |
| [`MAX_BURST`](#max_burst) | const |  |

## Structs

### `ProgressDrawTarget`

```rust
struct ProgressDrawTarget {
    kind: TargetKind,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:25-27`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L25-L27)*

Target for draw operations

This tells a [`ProgressBar`](crate::ProgressBar) or a
[`MultiProgress`](crate::MultiProgress) object where to paint to.
The draw target is a stateful wrapper over a drawing destination and
internally optimizes how often the state is painted to the output
device.

#### Implementations

- <span id="progressdrawtarget-stdout"></span>`fn stdout() -> Self`

- <span id="progressdrawtarget-stderr"></span>`fn stderr() -> Self`

- <span id="progressdrawtarget-stdout-with-hz"></span>`fn stdout_with_hz(refresh_rate: u8) -> Self`

- <span id="progressdrawtarget-stderr-with-hz"></span>`fn stderr_with_hz(refresh_rate: u8) -> Self`

- <span id="progressdrawtarget-new-remote"></span>`fn new_remote(state: Arc<RwLock<MultiState>>, idx: usize) -> Self` — [`MultiState`](../multi/index.md)

- <span id="progressdrawtarget-term"></span>`fn term(term: Term, refresh_rate: u8) -> Self`

- <span id="progressdrawtarget-term-like"></span>`fn term_like(term_like: Box<dyn TermLike>) -> Self` — [`TermLike`](../term_like/index.md)

- <span id="progressdrawtarget-term-like-with-hz"></span>`fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self` — [`TermLike`](../term_like/index.md)

- <span id="progressdrawtarget-hidden"></span>`fn hidden() -> Self`

- <span id="progressdrawtarget-is-hidden"></span>`fn is_hidden(&self) -> bool`

- <span id="progressdrawtarget-is-stderr"></span>`fn is_stderr(&self) -> bool`

- <span id="progressdrawtarget-width"></span>`fn width(&self) -> Option<u16>`

- <span id="progressdrawtarget-mark-zombie"></span>`fn mark_zombie(&self)`

- <span id="progressdrawtarget-set-move-cursor"></span>`fn set_move_cursor(&mut self, move_cursor: bool)`

- <span id="progressdrawtarget-drawable"></span>`fn drawable(&mut self, force_draw: bool, now: Instant) -> Option<Drawable<'_>>` — [`Drawable`](#drawable)

- <span id="progressdrawtarget-disconnect"></span>`fn disconnect(&self, now: Instant)`

- <span id="progressdrawtarget-remote"></span>`fn remote(&self) -> Option<(&Arc<RwLock<MultiState>>, usize)>` — [`MultiState`](../multi/index.md)

- <span id="progressdrawtarget-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

#### Trait Implementations

##### `impl Debug for ProgressDrawTarget`

- <span id="progressdrawtarget-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DrawStateWrapper<'a>`

```rust
struct DrawStateWrapper<'a> {
    state: &'a mut DrawState,
    orphan_lines: Option<&'a mut Vec<LineType>>,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:380-383`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L380-L383)*

#### Implementations

- <span id="drawstatewrapper-for-term"></span>`fn for_term(state: &'a mut DrawState) -> Self` — [`DrawState`](#drawstate)

- <span id="drawstatewrapper-for-multi"></span>`fn for_multi(state: &'a mut DrawState, orphan_lines: &'a mut Vec<LineType>) -> Self` — [`DrawState`](#drawstate), [`LineType`](#linetype)

#### Trait Implementations

##### `impl Deref for DrawStateWrapper<'_>`

- <span id="drawstatewrapper-type-target"></span>`type Target = DrawState`

- <span id="drawstatewrapper-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for DrawStateWrapper<'_>`

- <span id="drawstatewrapper-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Drop for DrawStateWrapper<'_>`

- <span id="drawstatewrapper-drop"></span>`fn drop(&mut self)`

##### `impl Receiver for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-type-target"></span>`type Target = T`

### `RateLimiter`

```rust
struct RateLimiter {
    interval: u16,
    capacity: u8,
    prev: std::time::Instant,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:435-439`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L435-L439)*

#### Implementations

- <span id="ratelimiter-new"></span>`fn new(rate: u8) -> Self`

- <span id="ratelimiter-allow"></span>`fn allow(&mut self, now: Instant) -> bool`

#### Trait Implementations

##### `impl Debug for RateLimiter`

- <span id="ratelimiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DrawState`

```rust
struct DrawState {
    lines: Vec<LineType>,
    move_cursor: bool,
    alignment: crate::multi::MultiProgressAlignment,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:488-495`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L488-L495)*

The drawn state of an element.

#### Fields

- **`lines`**: `Vec<LineType>`

  The lines to print (can contain ANSI codes)

- **`move_cursor`**: `bool`

  True if we should move the cursor up when possible instead of clearing lines.

- **`alignment`**: `crate::multi::MultiProgressAlignment`

  Controls how the multi progress is aligned if some of its progress bars get removed, default is `Top`

#### Implementations

- <span id="drawstate-draw-to-term"></span>`fn draw_to_term(&mut self, term: &impl TermLike + ?Sized, bar_count: &mut VisualLines) -> io::Result<()>` — [`TermLike`](../term_like/index.md), [`VisualLines`](#visuallines)

- <span id="drawstate-reset"></span>`fn reset(&mut self)`

- <span id="drawstate-visual-line-count"></span>`fn visual_line_count(&self, range: impl SliceIndex<[LineType], Output = [LineType]>, width: usize) -> VisualLines` — [`LineType`](#linetype), [`VisualLines`](#visuallines)

#### Trait Implementations

##### `impl Clone for DrawState`

- <span id="drawstate-clone"></span>`fn clone(&self) -> DrawState` — [`DrawState`](#drawstate)

##### `impl Debug for DrawState`

- <span id="drawstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DrawState`

- <span id="drawstate-default"></span>`fn default() -> DrawState` — [`DrawState`](#drawstate)

### `VisualLines`

```rust
struct VisualLines(usize);
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:600`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L600)*

#### Implementations

- <span id="visuallines-saturating-add"></span>`fn saturating_add(&self, other: Self) -> Self`

- <span id="visuallines-saturating-sub"></span>`fn saturating_sub(&self, other: Self) -> Self`

- <span id="visuallines-as-usize"></span>`fn as_usize(&self) -> usize`

#### Trait Implementations

##### `impl Add for VisualLines`

- <span id="visuallines-type-output"></span>`type Output = VisualLines`

- <span id="visuallines-add"></span>`fn add(self, rhs: Self) -> <Self as >::Output`

##### `impl AddAssign for VisualLines`

- <span id="visuallines-add-assign"></span>`fn add_assign(&mut self, rhs: Self)`

##### `impl Clone for VisualLines`

- <span id="visuallines-clone"></span>`fn clone(&self) -> VisualLines` — [`VisualLines`](#visuallines)

##### `impl Copy for VisualLines`

##### `impl Debug for VisualLines`

- <span id="visuallines-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VisualLines`

- <span id="visuallines-default"></span>`fn default() -> VisualLines` — [`VisualLines`](#visuallines)

##### `impl Eq for VisualLines`

##### `impl Ord for VisualLines`

- <span id="visuallines-cmp"></span>`fn cmp(&self, other: &VisualLines) -> cmp::Ordering` — [`VisualLines`](#visuallines)

##### `impl PartialEq for VisualLines`

- <span id="visuallines-eq"></span>`fn eq(&self, other: &VisualLines) -> bool` — [`VisualLines`](#visuallines)

##### `impl PartialOrd for VisualLines`

- <span id="visuallines-partial-cmp"></span>`fn partial_cmp(&self, other: &VisualLines) -> option::Option<cmp::Ordering>` — [`VisualLines`](#visuallines)

##### `impl StructuralPartialEq for VisualLines`

##### `impl Sub for VisualLines`

- <span id="visuallines-type-output"></span>`type Output = VisualLines`

- <span id="visuallines-sub"></span>`fn sub(self, rhs: Self) -> <Self as >::Output`

## Enums

### `TargetKind`

```rust
enum TargetKind {
    Term {
        term: console::Term,
        last_line_count: VisualLines,
        rate_limiter: RateLimiter,
        draw_state: DrawState,
    },
    Multi {
        state: std::sync::Arc<std::sync::RwLock<crate::multi::MultiState>>,
        idx: usize,
    },
    Hidden,
    TermLike {
        inner: Box<dyn TermLike>,
        last_line_count: VisualLines,
        rate_limiter: Option<RateLimiter>,
        draw_state: DrawState,
    },
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:248-266`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L248-L266)*

#### Implementations

- <span id="targetkind-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

#### Trait Implementations

##### `impl Debug for TargetKind`

- <span id="targetkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Drawable<'a>`

```rust
enum Drawable<'a> {
    Term {
        term: &'a console::Term,
        last_line_count: &'a mut VisualLines,
        draw_state: &'a mut DrawState,
    },
    Multi {
        state: std::sync::RwLockWriteGuard<'a, crate::multi::MultiState>,
        idx: usize,
        force_draw: bool,
        now: std::time::Instant,
    },
    TermLike {
        term_like: &'a dyn TermLike,
        last_line_count: &'a mut VisualLines,
        draw_state: &'a mut DrawState,
    },
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:288-305`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L288-L305)*

#### Implementations

- <span id="drawable-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

- <span id="drawable-state"></span>`fn state(&mut self) -> DrawStateWrapper<'_>` — [`DrawStateWrapper`](#drawstatewrapper)

- <span id="drawable-clear"></span>`fn clear(self) -> io::Result<()>`

- <span id="drawable-draw"></span>`fn draw(self) -> io::Result<()>`

- <span id="drawable-width"></span>`fn width(&self) -> Option<u16>`

### `LineAdjust`

```rust
enum LineAdjust {
    Clear(VisualLines),
    Keep(VisualLines),
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:373-378`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L373-L378)*

#### Variants

- **`Clear`**

  Adds to `last_line_count` so that the next draw also clears those lines

- **`Keep`**

  Subtracts from `last_line_count` so that the next draw retains those lines

### `LineType`

```rust
enum LineType {
    Text(String),
    Bar(String),
    Empty,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:653-657`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L653-L657)*

#### Implementations

- <span id="linetype-wrapped-height"></span>`fn wrapped_height(&self, width: usize) -> VisualLines` — [`VisualLines`](#visuallines)

- <span id="linetype-console-width"></span>`fn console_width(&self) -> usize`

#### Trait Implementations

##### `impl AsRef for LineType`

- <span id="linetype-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for LineType`

- <span id="linetype-clone"></span>`fn clone(&self) -> LineType` — [`LineType`](#linetype)

##### `impl Debug for LineType`

- <span id="linetype-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for LineType`

- <span id="linetype-eq"></span>`fn eq(&self, other: &str) -> bool`

## Functions

### `visual_line_count`

```rust
fn visual_line_count(lines: &[LineType], width: usize) -> VisualLines
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:646-650`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L646-L650)*

Calculate the number of visual lines in the given lines, after
accounting for line wrapping and non-printable characters.

## Constants

### `MAX_BURST`
```rust
const MAX_BURST: u8 = 20u8;
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:484`](../../../.source_1765210505/indicatif-0.18.3/src/draw_target.rs#L484)*

