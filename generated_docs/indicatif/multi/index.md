*[indicatif](../index.md) / [multi](index.md)*

---

# Module `multi`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiProgress`](#multiprogress) | struct | Manages multiple progress bars from different threads |
| [`MultiState`](#multistate) | struct |  |
| [`MultiStateMember`](#multistatemember) | struct |  |
| [`MultiProgressAlignment`](#multiprogressalignment) | enum | Vertical alignment of a multi progress. |
| [`InsertLocation`](#insertlocation) | enum |  |

## Structs

### `MultiProgress`

```rust
struct MultiProgress {
    state: std::sync::Arc<std::sync::RwLock<MultiState>>,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:18-20`](../../../.source_1765521767/indicatif-0.18.3/src/multi.rs#L18-L20)*

Manages multiple progress bars from different threads

#### Implementations

- <span id="multiprogress-new"></span>`fn new() -> Self`

- <span id="multiprogress-with-draw-target"></span>`fn with_draw_target(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

- <span id="multiprogress-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

- <span id="multiprogress-set-move-cursor"></span>`fn set_move_cursor(&self, move_cursor: bool)`

- <span id="multiprogress-set-alignment"></span>`fn set_alignment(&self, alignment: MultiProgressAlignment)` — [`MultiProgressAlignment`](#multiprogressalignment)

- <span id="multiprogress-add"></span>`fn add(&self, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-insert"></span>`fn insert(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-insert-from-back"></span>`fn insert_from_back(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-insert-before"></span>`fn insert_before(&self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-insert-after"></span>`fn insert_after(&self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-remove"></span>`fn remove(&self, pb: &ProgressBar)` — [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-internalize"></span>`fn internalize(&self, location: InsertLocation, pb: ProgressBar) -> ProgressBar` — [`InsertLocation`](#insertlocation), [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-println"></span>`fn println<I: AsRef<str>>(&self, msg: I) -> io::Result<()>`

- <span id="multiprogress-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

- <span id="multiprogress-clear"></span>`fn clear(&self) -> io::Result<()>`

- <span id="multiprogress-is-hidden"></span>`fn is_hidden(&self) -> bool`

#### Trait Implementations

##### `impl Clone for MultiProgress`

- <span id="multiprogress-clone"></span>`fn clone(&self) -> MultiProgress` — [`MultiProgress`](#multiprogress)

##### `impl Debug for MultiProgress`

- <span id="multiprogress-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgress`

- <span id="multiprogress-default"></span>`fn default() -> Self`

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

*Defined in [`indicatif-0.18.3/src/multi.rs:207-224`](../../../.source_1765521767/indicatif-0.18.3/src/multi.rs#L207-L224)*

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

- <span id="multistate-new"></span>`fn new(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

- <span id="multistate-mark-zombie"></span>`fn mark_zombie(&mut self, index: usize)`

- <span id="multistate-draw"></span>`fn draw(&mut self, force_draw: bool, extra_lines: Option<Vec<LineType>>, now: Instant) -> io::Result<()>` — [`LineType`](../draw_target/index.md#linetype)

- <span id="multistate-println"></span>`fn println<I: AsRef<str>>(&mut self, msg: I, now: Instant) -> io::Result<()>`

- <span id="multistate-draw-state"></span>`fn draw_state(&mut self, idx: usize) -> DrawStateWrapper<'_>` — [`DrawStateWrapper`](../draw_target/index.md#drawstatewrapper)

- <span id="multistate-is-hidden"></span>`fn is_hidden(&self) -> bool`

- <span id="multistate-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&mut self, f: F, now: Instant) -> R`

- <span id="multistate-width"></span>`fn width(&self) -> Option<u16>`

- <span id="multistate-insert"></span>`fn insert(&mut self, location: InsertLocation) -> usize` — [`InsertLocation`](#insertlocation)

- <span id="multistate-clear"></span>`fn clear(&mut self, now: Instant) -> io::Result<()>`

- <span id="multistate-remove-idx"></span>`fn remove_idx(&mut self, idx: usize)`

- <span id="multistate-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Debug for MultiState`

- <span id="multistate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MultiStateMember`

```rust
struct MultiStateMember {
    draw_state: Option<crate::draw_target::DrawState>,
    is_zombie: bool,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:471-477`](../../../.source_1765521767/indicatif-0.18.3/src/multi.rs#L471-L477)*

#### Fields

- **`draw_state`**: `Option<crate::draw_target::DrawState>`

  Draw state will be `None` for members that haven't been drawn before, or for entries that
  correspond to something in the free set.

- **`is_zombie`**: `bool`

  Whether the corresponding progress bar (more precisely, `BarState`) has been dropped.

#### Trait Implementations

##### `impl Debug for MultiStateMember`

- <span id="multistatemember-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result`

##### `impl Default for MultiStateMember`

- <span id="multistatemember-default"></span>`fn default() -> MultiStateMember` — [`MultiStateMember`](#multistatemember)

## Enums

### `MultiProgressAlignment`

```rust
enum MultiProgressAlignment {
    Top,
    Bottom,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:505-509`](../../../.source_1765521767/indicatif-0.18.3/src/multi.rs#L505-L509)*

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

- <span id="multiprogressalignment-clone"></span>`fn clone(&self) -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

##### `impl Copy for MultiProgressAlignment`

##### `impl Debug for MultiProgressAlignment`

- <span id="multiprogressalignment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgressAlignment`

- <span id="multiprogressalignment-default"></span>`fn default() -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

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

*Defined in [`indicatif-0.18.3/src/multi.rs:511-517`](../../../.source_1765521767/indicatif-0.18.3/src/multi.rs#L511-L517)*

