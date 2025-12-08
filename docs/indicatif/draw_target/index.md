*[indicatif](../index.md) / [draw_target](index.md)*

---

# Module `draw_target`

## Structs

### `ProgressDrawTarget`

```rust
struct ProgressDrawTarget {
    kind: TargetKind,
}
```

Target for draw operations

This tells a [`ProgressBar`](crate::ProgressBar) or a
[`MultiProgress`](crate::MultiProgress) object where to paint to.
The draw target is a stateful wrapper over a drawing destination and
internally optimizes how often the state is painted to the output
device.

#### Implementations

- `fn stdout() -> Self`

- `fn stderr() -> Self`

- `fn stdout_with_hz(refresh_rate: u8) -> Self`

- `fn stderr_with_hz(refresh_rate: u8) -> Self`

- `fn new_remote(state: Arc<RwLock<MultiState>>, idx: usize) -> Self` — [`MultiState`](../multi/index.md)

- `fn term(term: Term, refresh_rate: u8) -> Self`

- `fn term_like(term_like: Box<dyn TermLike>) -> Self` — [`TermLike`](../term_like/index.md)

- `fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self` — [`TermLike`](../term_like/index.md)

- `fn hidden() -> Self`

- `fn is_hidden(self: &Self) -> bool`

- `fn is_stderr(self: &Self) -> bool`

- `fn width(self: &Self) -> Option<u16>`

- `fn mark_zombie(self: &Self)`

- `fn set_move_cursor(self: &mut Self, move_cursor: bool)`

- `fn drawable(self: &mut Self, force_draw: bool, now: Instant) -> Option<Drawable<'_>>` — [`Drawable`](#drawable)

- `fn disconnect(self: &Self, now: Instant)`

- `fn remote(self: &Self) -> Option<(&Arc<RwLock<MultiState>>, usize)>` — [`MultiState`](../multi/index.md)

- `fn adjust_last_line_count(self: &mut Self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

#### Trait Implementations

##### `impl Debug for ProgressDrawTarget`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DrawStateWrapper<'a>`

```rust
struct DrawStateWrapper<'a> {
    state: &'a mut DrawState,
    orphan_lines: Option<&'a mut Vec<LineType>>,
}
```

#### Implementations

- `fn for_term(state: &'a mut DrawState) -> Self` — [`DrawState`](#drawstate)

- `fn for_multi(state: &'a mut DrawState, orphan_lines: &'a mut Vec<LineType>) -> Self` — [`DrawState`](#drawstate), [`LineType`](#linetype)

#### Trait Implementations

##### `impl Deref for DrawStateWrapper<'_>`

- `type Target = DrawState`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for DrawStateWrapper<'_>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Drop for DrawStateWrapper<'_>`

- `fn drop(self: &mut Self)`

##### `impl<P, T> Receiver for DrawStateWrapper<'a>`

- `type Target = T`

### `RateLimiter`

```rust
struct RateLimiter {
    interval: u16,
    capacity: u8,
    prev: std::time::Instant,
}
```

#### Implementations

- `fn new(rate: u8) -> Self`

- `fn allow(self: &mut Self, now: Instant) -> bool`

#### Trait Implementations

##### `impl Debug for RateLimiter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DrawState`

```rust
struct DrawState {
    lines: Vec<LineType>,
    move_cursor: bool,
    alignment: crate::multi::MultiProgressAlignment,
}
```

The drawn state of an element.

#### Fields

- **`lines`**: `Vec<LineType>`

  The lines to print (can contain ANSI codes)

- **`move_cursor`**: `bool`

  True if we should move the cursor up when possible instead of clearing lines.

- **`alignment`**: `crate::multi::MultiProgressAlignment`

  Controls how the multi progress is aligned if some of its progress bars get removed, default is `Top`

#### Implementations

- `fn draw_to_term(self: &mut Self, term: &impl TermLike + ?Sized, bar_count: &mut VisualLines) -> io::Result<()>` — [`TermLike`](../term_like/index.md), [`VisualLines`](#visuallines)

- `fn reset(self: &mut Self)`

- `fn visual_line_count(self: &Self, range: impl SliceIndex<[LineType], Output = [LineType]>, width: usize) -> VisualLines` — [`LineType`](#linetype), [`VisualLines`](#visuallines)

#### Trait Implementations

##### `impl Clone for DrawState`

- `fn clone(self: &Self) -> DrawState` — [`DrawState`](#drawstate)

##### `impl Debug for DrawState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DrawState`

- `fn default() -> DrawState` — [`DrawState`](#drawstate)

### `VisualLines`

```rust
struct VisualLines(usize);
```

#### Implementations

- `fn saturating_add(self: &Self, other: Self) -> Self`

- `fn saturating_sub(self: &Self, other: Self) -> Self`

- `fn as_usize(self: &Self) -> usize`

#### Trait Implementations

##### `impl Add for VisualLines`

- `type Output = VisualLines`

- `fn add(self: Self, rhs: Self) -> <Self as >::Output`

##### `impl AddAssign for VisualLines`

- `fn add_assign(self: &mut Self, rhs: Self)`

##### `impl Clone for VisualLines`

- `fn clone(self: &Self) -> VisualLines` — [`VisualLines`](#visuallines)

##### `impl Copy for VisualLines`

##### `impl Debug for VisualLines`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for VisualLines`

- `fn default() -> VisualLines` — [`VisualLines`](#visuallines)

##### `impl Eq for VisualLines`

##### `impl Ord for VisualLines`

- `fn cmp(self: &Self, other: &VisualLines) -> $crate::cmp::Ordering` — [`VisualLines`](#visuallines)

##### `impl PartialEq for VisualLines`

- `fn eq(self: &Self, other: &VisualLines) -> bool` — [`VisualLines`](#visuallines)

##### `impl PartialOrd for VisualLines`

- `fn partial_cmp(self: &Self, other: &VisualLines) -> $crate::option::Option<$crate::cmp::Ordering>` — [`VisualLines`](#visuallines)

##### `impl StructuralPartialEq for VisualLines`

##### `impl Sub for VisualLines`

- `type Output = VisualLines`

- `fn sub(self: Self, rhs: Self) -> <Self as >::Output`

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

#### Implementations

- `fn adjust_last_line_count(self: &mut Self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

#### Trait Implementations

##### `impl Debug for TargetKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

#### Implementations

- `fn adjust_last_line_count(self: &mut Self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

- `fn state(self: &mut Self) -> DrawStateWrapper<'_>` — [`DrawStateWrapper`](#drawstatewrapper)

- `fn clear(self: Self) -> io::Result<()>`

- `fn draw(self: Self) -> io::Result<()>`

- `fn width(self: &Self) -> Option<u16>`

### `LineAdjust`

```rust
enum LineAdjust {
    Clear(VisualLines),
    Keep(VisualLines),
}
```

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

#### Implementations

- `fn wrapped_height(self: &Self, width: usize) -> VisualLines` — [`VisualLines`](#visuallines)

- `fn console_width(self: &Self) -> usize`

#### Trait Implementations

##### `impl AsRef for LineType`

- `fn as_ref(self: &Self) -> &str`

##### `impl Clone for LineType`

- `fn clone(self: &Self) -> LineType` — [`LineType`](#linetype)

##### `impl Debug for LineType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for LineType`

- `fn eq(self: &Self, other: &str) -> bool`

## Functions

### `visual_line_count`

```rust
fn visual_line_count(lines: &[LineType], width: usize) -> VisualLines
```

Calculate the number of visual lines in the given lines, after
accounting for line wrapping and non-printable characters.

## Constants

### `MAX_BURST`

```rust
const MAX_BURST: u8 = 20u8;
```

