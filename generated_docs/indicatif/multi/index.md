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

*Defined in [`indicatif-0.18.3/src/multi.rs:18-20`](../../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L18-L20)*

Manages multiple progress bars from different threads

#### Implementations

- <span id="multiprogress-new"></span>`fn new() -> Self`

  Creates a new multi progress object.

  

  Progress bars added to this object by default draw directly to stderr, and refresh

  a maximum of 15 times a second. To change the refresh rate [`set`](../../hashbrown/set/index.md) the [draw target] to

  one with a different refresh rate.

  

- <span id="multiprogress-with-draw-target"></span>`fn with_draw_target(draw_target: ProgressDrawTarget) -> Self` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

  Creates a new multi progress object with the given draw target.

- <span id="multiprogress-set-draw-target"></span>`fn set_draw_target(&self, target: ProgressDrawTarget)` — [`ProgressDrawTarget`](../draw_target/index.md#progressdrawtarget)

  Sets a different draw target for the multiprogress bar.

  

  Use `MultiProgress::with_draw_target` to set the draw target during creation.

- <span id="multiprogress-set-move-cursor"></span>`fn set_move_cursor(&self, move_cursor: bool)`

  Set whether we should try to move the cursor when possible instead of clearing lines.

  

  This can reduce flickering, but do not enable it if you intend to change the number of

  progress bars.

- <span id="multiprogress-set-alignment"></span>`fn set_alignment(&self, alignment: MultiProgressAlignment)` — [`MultiProgressAlignment`](#multiprogressalignment)

  Set alignment flag

- <span id="multiprogress-add"></span>`fn add(&self, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

  Adds a progress bar.

  

  The progress bar added will have the draw target changed to a

  remote draw target that is intercepted by the multi progress

  object overriding custom [`ProgressDrawTarget`](../draw_target/index.md) settings.

  

  The progress bar will be positioned below all other bars currently

  in the [`MultiProgress`](#multiprogress).

  

  Adding a progress bar that is already a member of the [`MultiProgress`](#multiprogress)

  will have no effect.

- <span id="multiprogress-insert"></span>`fn insert(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

  Inserts a progress bar.

  

  The progress bar inserted at position `index` will have the draw

  target changed to a remote draw target that is intercepted by the

  multi progress object overriding custom [`ProgressDrawTarget`](../draw_target/index.md) settings.

  

  If `index >= MultiProgressState::objects.len()`, the progress bar

  is added to the end of the list.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](#multiprogress)

  will have no effect.

- <span id="multiprogress-insert-from-back"></span>`fn insert_from_back(&self, index: usize, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

  Inserts a progress bar from the back.

  

  The progress bar inserted at position `MultiProgressState::objects.len() - index`

  will have the draw target changed to a remote draw target that is

  intercepted by the multi progress object overriding custom

  [`ProgressDrawTarget`](../draw_target/index.md) settings.

  

  If `index >= MultiProgressState::objects.len()`, the progress bar

  is added to the start of the list.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](#multiprogress)

  will have no effect.

- <span id="multiprogress-insert-before"></span>`fn insert_before(&self, before: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

  Inserts a progress bar before an existing one.

  

  The progress bar added will have the draw target changed to a

  remote draw target that is intercepted by the multi progress

  object overriding custom [`ProgressDrawTarget`](../draw_target/index.md) settings.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](#multiprogress)

  will have no effect.

- <span id="multiprogress-insert-after"></span>`fn insert_after(&self, after: &ProgressBar, pb: ProgressBar) -> ProgressBar` — [`ProgressBar`](../progress_bar/index.md#progressbar)

  Inserts a progress bar after an existing one.

  

  The progress bar added will have the draw target changed to a

  remote draw target that is intercepted by the multi progress

  object overriding custom [`ProgressDrawTarget`](../draw_target/index.md) settings.

  

  Inserting a progress bar that is already a member of the [`MultiProgress`](#multiprogress)

  will have no effect.

- <span id="multiprogress-remove"></span>`fn remove(&self, pb: &ProgressBar)` — [`ProgressBar`](../progress_bar/index.md#progressbar)

  Removes a progress bar.

  

  The progress bar is removed only if it was previously inserted or added

  by the methods `MultiProgress::insert` or `MultiProgress::add`.

  If the passed progress bar does not satisfy the condition above,

  the `remove` method does nothing.

- <span id="multiprogress-internalize"></span>`fn internalize(&self, location: InsertLocation, pb: ProgressBar) -> ProgressBar` — [`InsertLocation`](#insertlocation), [`ProgressBar`](../progress_bar/index.md#progressbar)

- <span id="multiprogress-println"></span>`fn println<I: AsRef<str>>(&self, msg: I) -> io::Result<()>`

  Print a log line above all progress bars in the [`MultiProgress`](#multiprogress)

  

  If the draw target is hidden (e.g. when standard output is not a terminal), `println()`

  will not do anything.

- <span id="multiprogress-suspend"></span>`fn suspend<F: FnOnce() -> R, R>(&self, f: F) -> R`

  Hide all progress bars temporarily, execute `f`, then redraw the [`MultiProgress`](#multiprogress)

  

  Executes 'f' even if the draw target is hidden.

  

  Useful for external code that writes to the standard output.

  

  **Note:** The internal lock is held while `f` is executed. Other threads trying to print

  anything on the progress bar will be blocked until `f` finishes.

  Therefore, it is recommended to avoid long-running operations in `f`.

- <span id="multiprogress-clear"></span>`fn clear(&self) -> io::Result<()>`

- <span id="multiprogress-is-hidden"></span>`fn is_hidden(&self) -> bool`

#### Trait Implementations

##### `impl Any for MultiProgress`

- <span id="multiprogress-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProgress`

- <span id="multiprogress-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProgress`

- <span id="multiprogress-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MultiProgress`

- <span id="multiprogress-clone"></span>`fn clone(&self) -> MultiProgress` — [`MultiProgress`](#multiprogress)

##### `impl CloneToUninit for MultiProgress`

- <span id="multiprogress-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MultiProgress`

- <span id="multiprogress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgress`

- <span id="multiprogress-default"></span>`fn default() -> Self`

##### `impl<T> From for MultiProgress`

- <span id="multiprogress-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProgress`

- <span id="multiprogress-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MultiProgress`

- <span id="multiprogress-toowned-type-owned"></span>`type Owned = T`

- <span id="multiprogress-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiprogress-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProgress`

- <span id="multiprogress-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiprogress-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProgress`

- <span id="multiprogress-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiprogress-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/multi.rs:207-224`](../../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L207-L224)*

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

##### `impl Any for MultiState`

- <span id="multistate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiState`

- <span id="multistate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiState`

- <span id="multistate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for MultiState`

- <span id="multistate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MultiState`

- <span id="multistate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiState`

- <span id="multistate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MultiState`

- <span id="multistate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multistate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiState`

- <span id="multistate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multistate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MultiStateMember`

```rust
struct MultiStateMember {
    draw_state: Option<crate::draw_target::DrawState>,
    is_zombie: bool,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:471-477`](../../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L471-L477)*

#### Fields

- **`draw_state`**: `Option<crate::draw_target::DrawState>`

  Draw state will be `None` for members that haven't been drawn before, or for entries that
  correspond to something in the free set.

- **`is_zombie`**: `bool`

  Whether the corresponding progress bar (more precisely, `BarState`) has been dropped.

#### Trait Implementations

##### `impl Any for MultiStateMember`

- <span id="multistatemember-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiStateMember`

- <span id="multistatemember-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiStateMember`

- <span id="multistatemember-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for MultiStateMember`

- <span id="multistatemember-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result`

##### `impl Default for MultiStateMember`

- <span id="multistatemember-default"></span>`fn default() -> MultiStateMember` — [`MultiStateMember`](#multistatemember)

##### `impl<T> From for MultiStateMember`

- <span id="multistatemember-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiStateMember`

- <span id="multistatemember-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MultiStateMember`

- <span id="multistatemember-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multistatemember-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiStateMember`

- <span id="multistatemember-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multistatemember-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MultiProgressAlignment`

```rust
enum MultiProgressAlignment {
    Top,
    Bottom,
}
```

*Defined in [`indicatif-0.18.3/src/multi.rs:505-509`](../../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L505-L509)*

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

##### `impl Any for MultiProgressAlignment`

- <span id="multiprogressalignment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProgressAlignment`

- <span id="multiprogressalignment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProgressAlignment`

- <span id="multiprogressalignment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MultiProgressAlignment`

- <span id="multiprogressalignment-clone"></span>`fn clone(&self) -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

##### `impl CloneToUninit for MultiProgressAlignment`

- <span id="multiprogressalignment-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MultiProgressAlignment`

##### `impl Debug for MultiProgressAlignment`

- <span id="multiprogressalignment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MultiProgressAlignment`

- <span id="multiprogressalignment-default"></span>`fn default() -> MultiProgressAlignment` — [`MultiProgressAlignment`](#multiprogressalignment)

##### `impl<T> From for MultiProgressAlignment`

- <span id="multiprogressalignment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProgressAlignment`

- <span id="multiprogressalignment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MultiProgressAlignment`

- <span id="multiprogressalignment-toowned-type-owned"></span>`type Owned = T`

- <span id="multiprogressalignment-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiprogressalignment-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProgressAlignment`

- <span id="multiprogressalignment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiprogressalignment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProgressAlignment`

- <span id="multiprogressalignment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiprogressalignment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/multi.rs:511-517`](../../../.source_1765633015/indicatif-0.18.3/src/multi.rs#L511-L517)*

#### Trait Implementations

##### `impl Any for InsertLocation`

- <span id="insertlocation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InsertLocation`

- <span id="insertlocation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InsertLocation`

- <span id="insertlocation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for InsertLocation`

- <span id="insertlocation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InsertLocation`

- <span id="insertlocation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InsertLocation`

- <span id="insertlocation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="insertlocation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InsertLocation`

- <span id="insertlocation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="insertlocation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

