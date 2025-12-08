*[indicatif](../index.md) / [multi](index.md)*

---

# Module `multi`

## Structs

### `MultiProgress`

```rust
struct MultiProgress {
    state: std::sync::Arc<std::sync::RwLock<MultiState>>,
}
```

Manages multiple progress bars from different threads

#### Implementations

- `fn new() -> Self`

- `fn with_draw_target(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md)

- `fn set_draw_target(self: &Self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](../draw_target/index.md)

- `fn set_move_cursor(self: &Self, move_cursor: bool)`

- `fn set_alignment(self: &Self, alignment: MultiProgressAlignment)` — [`MultiProgressAlignment`](#multiprogressalignment)

- `fn add(self: &Self, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md)

- `fn insert(self: &Self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md)

- `fn insert_from_back(self: &Self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md)

- `fn insert_before(self: &Self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md)

- `fn insert_after(self: &Self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md)

- `fn remove(self: &Self, pb: &ProgressBar)` — [`ProgressBar`](../progress_bar/index.md)

- `fn internalize(self: &Self, location: InsertLocation, pb: ProgressBar) -> ProgressBar` — [`InsertLocation`](#insertlocation), [`ProgressBar`](../progress_bar/index.md)

- `fn println<I: AsRef<str>>(self: &Self, msg: I) -> io::Result<()>`

- `fn suspend<F: FnOnce() -> R, R>(self: &Self, f: F) -> R`

- `fn clear(self: &Self) -> io::Result<()>`

- `fn is_hidden(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for MultiProgress`

- `fn clone(self: &Self) -> MultiProgress` — [`MultiProgress`](#multiprogress)

##### `impl Debug for MultiProgress`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MultiProgress`

- `fn default() -> Self`

### `MultiState`

```rust
struct MultiState {
    members: Vec<MultiStateMember>,
    free_set: Vec<usize>,
    ordering: Vec<usize>,
    draw_target: crate::draw_target::ProgressDrawTarget,
    alignment: MultiProgressAlignment,
    orphan_lines: Vec<crate::draw_target::LineType>,
    zombie_lines_count: crate::draw_target::VisualLines,
}
```

#### Fields

- **`members`**: `Vec<MultiStateMember>`

  The collection of states corresponding to progress bars

- **`free_set`**: `Vec<usize>`

  Set of removed bars, should have corresponding members in the `members` vector with a
  `draw_state` of `None`.

- **`ordering`**: `Vec<usize>`

  Indices to the `draw_states` to maintain correct visual order

- **`draw_target`**: `crate::draw_target::ProgressDrawTarget`

  Target for draw operation for MultiProgress

- **`alignment`**: `MultiProgressAlignment`

  Controls how the multi progress is aligned if some of its progress bars get removed, default is `Top`

- **`orphan_lines`**: `Vec<crate::draw_target::LineType>`

  Lines to be drawn above everything else in the MultiProgress. These specifically come from
  calling `ProgressBar::println` on a pb that is connected to a `MultiProgress`.

- **`zombie_lines_count`**: `crate::draw_target::VisualLines`

  The count of currently visible zombie lines.

#### Implementations

- `fn new(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md)

- `fn mark_zombie(self: &mut Self, index: usize)`

- `fn draw(self: &mut Self, force_draw: bool, extra_lines: Option<Vec<LineType>>, now: Instant) -> io::Result<()>` — [`LineType`](../draw_target/index.md)

- `fn println<I: AsRef<str>>(self: &mut Self, msg: I, now: Instant) -> io::Result<()>`

- `fn draw_state(self: &mut Self, idx: usize) -> DrawStateWrapper<'_>` — [`DrawStateWrapper`](../draw_target/index.md)

- `fn is_hidden(self: &Self) -> bool`

- `fn suspend<F: FnOnce() -> R, R>(self: &mut Self, f: F, now: Instant) -> R`

- `fn width(self: &Self) -> Option<u16>`

- `fn insert(self: &mut Self, location: InsertLocation) -> usize` — [`InsertLocation`](#insertlocation)

- `fn clear(self: &mut Self, now: Instant) -> io::Result<()>`

- `fn remove_idx(self: &mut Self, idx: usize)`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for MultiState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MultiStateMember`

```rust
struct MultiStateMember {
    draw_state: Option<crate::draw_target::DrawState>,
    is_zombie: bool,
}
```

#### Fields

- **`draw_state`**: `Option<crate::draw_target::DrawState>`

  Draw state will be `None` for members that haven't been drawn before, or for entries that
  correspond to something in the free set.

- **`is_zombie`**: `bool`

  Whether the corresponding progress bar (more precisely, `BarState`) has been dropped.

#### Trait Implementations

##### `impl Debug for MultiStateMember`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> std::fmt::Result`

##### `impl Default for MultiStateMember`

- `fn default() -> MultiStateMember` — [`MultiStateMember`](#multistatemember)

## Enums

### `MultiProgressAlignment`

```rust
enum MultiProgressAlignment {
    Top,
    Bottom,
}
```

Vertical alignment of a multi progress.

The alignment controls how the multi progress is aligned if some of its progress bars get removed.
E.g. [`Top`](MultiProgressAlignment::Top) alignment (default), when _progress bar 2_ is removed:
```ignore
[0/100] progress bar 1        [0/100] progress bar 1
[0/100] progress bar 2   =>   [0/100] progress bar 3
[0/100] progress bar 3
```

[`Bottom`](MultiProgressAlignment::Bottom) alignment
```ignore
[0/100] progress bar 1
[0/100] progress bar 2   =>   [0/100] progress bar 1
[0/100] progress bar 3        [0/100] progress bar 3
```

#### Trait Implementations

##### `impl Clone for MultiProgressAlignment`

- `fn clone(self: &Self) -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

##### `impl Copy for MultiProgressAlignment`

##### `impl Debug for MultiProgressAlignment`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MultiProgressAlignment`

- `fn default() -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

### `InsertLocation`

```rust
enum InsertLocation {
    End,
    Index(usize),
    IndexFromBack(usize),
    After(usize),
    Before(usize),
}
```

