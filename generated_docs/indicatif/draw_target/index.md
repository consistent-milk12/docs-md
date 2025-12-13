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
  - [`visual_line_count`](#visual-line-count)
- [Constants](#constants)
  - [`MAX_BURST`](#max-burst)

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
| [`visual_line_count`](#visual-line-count) | fn | Calculate the number of visual lines in the given lines, after accounting for line wrapping and non-printable characters. |
| [`MAX_BURST`](#max-burst) | const |  |

## Structs

### `ProgressDrawTarget`

```rust
struct ProgressDrawTarget {
    kind: TargetKind,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:25-27`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L25-L27)*

Target for draw operations

This tells a [`ProgressBar`](crate::ProgressBar) or a
[`MultiProgress`](crate::MultiProgress) object where to paint to.
The draw target is a stateful wrapper over a drawing destination and
internally optimizes how often the state is painted to the output
device.

#### Implementations

- <span id="progressdrawtarget-stdout"></span>`fn stdout() -> Self`

  Draw to a buffered stdout terminal at a max of 20 times a second.

  

  For more information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-stderr"></span>`fn stderr() -> Self`

  Draw to a buffered stderr terminal at a max of 20 times a second.

  

  This is the default draw target for progress bars.  For more

  information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-stdout-with-hz"></span>`fn stdout_with_hz(refresh_rate: u8) -> Self`

  Draw to a buffered stdout terminal at a max of `refresh_rate` times a second.

  

  For more information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-stderr-with-hz"></span>`fn stderr_with_hz(refresh_rate: u8) -> Self`

  Draw to a buffered stderr terminal at a max of `refresh_rate` times a second.

  

  For more information see `ProgressDrawTarget::term`.

- <span id="progressdrawtarget-new-remote"></span>`fn new_remote(state: Arc<RwLock<MultiState>>, idx: usize) -> Self` — [`MultiState`](../multi/index.md#multistate)

- <span id="progressdrawtarget-term"></span>`fn term(term: Term, refresh_rate: u8) -> Self`

  Draw to a terminal, with a specific refresh rate.

  

  Progress bars are by default drawn to terminals however if the

  terminal is not user attended the entire progress bar will be

  hidden.  This is done so that piping to a file will not produce

  useless escape codes in that file.

  

  Will panic if `refresh_rate` is `0`.

- <span id="progressdrawtarget-term-like"></span>`fn term_like(term_like: Box<dyn TermLike>) -> Self` — [`TermLike`](../term_like/index.md#termlike)

  Draw to a boxed object that implements the [`TermLike`](../term_like/index.md) trait.

- <span id="progressdrawtarget-term-like-with-hz"></span>`fn term_like_with_hz(term_like: Box<dyn TermLike>, refresh_rate: u8) -> Self` — [`TermLike`](../term_like/index.md#termlike)

  Draw to a boxed object that implements the [`TermLike`](../term_like/index.md) trait,

  with a specific refresh rate.

- <span id="progressdrawtarget-hidden"></span>`fn hidden() -> Self`

  A hidden draw target.

  

  This forces a progress bar to be not rendered at all.

- <span id="progressdrawtarget-is-hidden"></span>`fn is_hidden(&self) -> bool`

  Returns true if the draw target is hidden.

  

  This is internally used in progress bars to figure out if overhead

  from drawing can be prevented.

- <span id="progressdrawtarget-is-stderr"></span>`fn is_stderr(&self) -> bool`

  This is used in progress bars to determine whether to use stdout or stderr

  for detecting color support.

- <span id="progressdrawtarget-width"></span>`fn width(&self) -> Option<u16>`

  Returns the current width of the draw target.

- <span id="progressdrawtarget-mark-zombie"></span>`fn mark_zombie(&self)`

  Notifies the backing `MultiProgress` (if applicable) that the associated progress bar should

  be marked a zombie.

- <span id="progressdrawtarget-set-move-cursor"></span>`fn set_move_cursor(&mut self, move_cursor: bool)`

  Set whether or not to just move cursor instead of clearing lines

- <span id="progressdrawtarget-drawable"></span>`fn drawable(&mut self, force_draw: bool, now: Instant) -> Option<Drawable<'_>>` — [`Drawable`](#drawable)

  Apply the given draw state (draws it).

- <span id="progressdrawtarget-disconnect"></span>`fn disconnect(&self, now: Instant)`

  Properly disconnects from the draw target

- <span id="progressdrawtarget-remote"></span>`fn remote(&self) -> Option<(&Arc<RwLock<MultiState>>, usize)>` — [`MultiState`](../multi/index.md#multistate)

- <span id="progressdrawtarget-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

#### Trait Implementations

##### `impl Any for ProgressDrawTarget`

- <span id="progressdrawtarget-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressDrawTarget`

- <span id="progressdrawtarget-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressDrawTarget`

- <span id="progressdrawtarget-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ProgressDrawTarget`

- <span id="progressdrawtarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ProgressDrawTarget`

- <span id="progressdrawtarget-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProgressDrawTarget`

- <span id="progressdrawtarget-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ProgressDrawTarget`

- <span id="progressdrawtarget-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressdrawtarget-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProgressDrawTarget`

- <span id="progressdrawtarget-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressdrawtarget-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DrawStateWrapper<'a>`

```rust
struct DrawStateWrapper<'a> {
    state: &'a mut DrawState,
    orphan_lines: Option<&'a mut Vec<LineType>>,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:380-383`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L380-L383)*

#### Implementations

- <span id="drawstatewrapper-for-term"></span>`fn for_term(state: &'a mut DrawState) -> Self` — [`DrawState`](#drawstate)

- <span id="drawstatewrapper-for-multi"></span>`fn for_multi(state: &'a mut DrawState, orphan_lines: &'a mut Vec<LineType>) -> Self` — [`DrawState`](#drawstate), [`LineType`](#linetype)

#### Trait Implementations

##### `impl Any for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Deref for DrawStateWrapper<'_>`

- <span id="drawstatewrapper-deref-type-target"></span>`type Target = DrawState`

- <span id="drawstatewrapper-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for DrawStateWrapper<'_>`

- <span id="drawstatewrapper-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Drop for DrawStateWrapper<'_>`

- <span id="drawstatewrapper-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-receiver-type-target"></span>`type Target = T`

##### `impl<U> TryFrom for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drawstatewrapper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DrawStateWrapper<'a>`

- <span id="drawstatewrapper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drawstatewrapper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RateLimiter`

```rust
struct RateLimiter {
    interval: u16,
    capacity: u8,
    prev: std::time::Instant,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:435-439`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L435-L439)*

#### Implementations

- <span id="ratelimiter-new"></span>`fn new(rate: u8) -> Self`

- <span id="ratelimiter-allow"></span>`fn allow(&mut self, now: Instant) -> bool`

#### Trait Implementations

##### `impl Any for RateLimiter`

- <span id="ratelimiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RateLimiter`

- <span id="ratelimiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RateLimiter`

- <span id="ratelimiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for RateLimiter`

- <span id="ratelimiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RateLimiter`

- <span id="ratelimiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RateLimiter`

- <span id="ratelimiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RateLimiter`

- <span id="ratelimiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ratelimiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RateLimiter`

- <span id="ratelimiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ratelimiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DrawState`

```rust
struct DrawState {
    lines: Vec<LineType>,
    move_cursor: bool,
    alignment: crate::multi::MultiProgressAlignment,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:488-495`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L488-L495)*

The drawn state of an element.

#### Fields

- **`lines`**: `Vec<LineType>`

  The lines to print (can contain ANSI codes)

- **`move_cursor`**: `bool`

  True if we should move the cursor up when possible instead of clearing lines.

- **`alignment`**: `crate::multi::MultiProgressAlignment`

  Controls how the multi progress is aligned if some of its progress bars get removed, default is `Top`

#### Implementations

- <span id="drawstate-draw-to-term"></span>`fn draw_to_term(&mut self, term: &impl TermLike + ?Sized, bar_count: &mut VisualLines) -> io::Result<()>` — [`TermLike`](../term_like/index.md#termlike), [`VisualLines`](#visuallines)

  Draw the current state to the terminal

  We expect a few things:

  - self.lines contains n lines of text/empty then m lines of bars

  - None of those lines contain newlines

- <span id="drawstate-reset"></span>`fn reset(&mut self)`

- <span id="drawstate-visual-line-count"></span>`fn visual_line_count(&self, range: impl SliceIndex<[LineType], Output = [LineType]>, width: usize) -> VisualLines` — [`LineType`](#linetype), [`VisualLines`](#visuallines)

#### Trait Implementations

##### `impl Any for DrawState`

- <span id="drawstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DrawState`

- <span id="drawstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DrawState`

- <span id="drawstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DrawState`

- <span id="drawstate-clone"></span>`fn clone(&self) -> DrawState` — [`DrawState`](#drawstate)

##### `impl CloneToUninit for DrawState`

- <span id="drawstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DrawState`

- <span id="drawstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DrawState`

- <span id="drawstate-default"></span>`fn default() -> DrawState` — [`DrawState`](#drawstate)

##### `impl<T> From for DrawState`

- <span id="drawstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DrawState`

- <span id="drawstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DrawState`

- <span id="drawstate-toowned-type-owned"></span>`type Owned = T`

- <span id="drawstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="drawstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DrawState`

- <span id="drawstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drawstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DrawState`

- <span id="drawstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drawstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VisualLines`

```rust
struct VisualLines(usize);
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:600`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L600)*

#### Implementations

- <span id="visuallines-saturating-add"></span>`fn saturating_add(&self, other: Self) -> Self`

- <span id="visuallines-saturating-sub"></span>`fn saturating_sub(&self, other: Self) -> Self`

- <span id="visuallines-as-usize"></span>`fn as_usize(&self) -> usize`

#### Trait Implementations

##### `impl Add for VisualLines`

- <span id="visuallines-add-type-output"></span>`type Output = VisualLines`

- <span id="visuallines-add"></span>`fn add(self, rhs: Self) -> <Self as >::Output`

##### `impl AddAssign for VisualLines`

- <span id="visuallines-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: Self)`

##### `impl Any for VisualLines`

- <span id="visuallines-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VisualLines`

- <span id="visuallines-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VisualLines`

- <span id="visuallines-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for VisualLines`

- <span id="visuallines-clone"></span>`fn clone(&self) -> VisualLines` — [`VisualLines`](#visuallines)

##### `impl CloneToUninit for VisualLines`

- <span id="visuallines-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for VisualLines`

##### `impl Debug for VisualLines`

- <span id="visuallines-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VisualLines`

- <span id="visuallines-default"></span>`fn default() -> VisualLines` — [`VisualLines`](#visuallines)

##### `impl Eq for VisualLines`

##### `impl<T> From for VisualLines`

- <span id="visuallines-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VisualLines`

- <span id="visuallines-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for VisualLines`

- <span id="visuallines-ord-cmp"></span>`fn cmp(&self, other: &VisualLines) -> cmp::Ordering` — [`VisualLines`](#visuallines)

##### `impl PartialEq for VisualLines`

- <span id="visuallines-partialeq-eq"></span>`fn eq(&self, other: &VisualLines) -> bool` — [`VisualLines`](#visuallines)

##### `impl PartialOrd for VisualLines`

- <span id="visuallines-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &VisualLines) -> option::Option<cmp::Ordering>` — [`VisualLines`](#visuallines)

##### `impl StructuralPartialEq for VisualLines`

##### `impl Sub for VisualLines`

- <span id="visuallines-sub-type-output"></span>`type Output = VisualLines`

- <span id="visuallines-sub"></span>`fn sub(self, rhs: Self) -> <Self as >::Output`

##### `impl ToOwned for VisualLines`

- <span id="visuallines-toowned-type-owned"></span>`type Owned = T`

- <span id="visuallines-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visuallines-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VisualLines`

- <span id="visuallines-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visuallines-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VisualLines`

- <span id="visuallines-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visuallines-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/draw_target.rs:248-266`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L248-L266)*

#### Implementations

- <span id="targetkind-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

  Adjust `last_line_count` such that the next draw operation keeps/clears additional lines

#### Trait Implementations

##### `impl Any for TargetKind`

- <span id="targetkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TargetKind`

- <span id="targetkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TargetKind`

- <span id="targetkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for TargetKind`

- <span id="targetkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TargetKind`

- <span id="targetkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TargetKind`

- <span id="targetkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TargetKind`

- <span id="targetkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="targetkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TargetKind`

- <span id="targetkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="targetkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`indicatif-0.18.3/src/draw_target.rs:288-305`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L288-L305)*

#### Implementations

- <span id="drawable-adjust-last-line-count"></span>`fn adjust_last_line_count(&mut self, adjust: LineAdjust)` — [`LineAdjust`](#lineadjust)

  Adjust `last_line_count` such that the next draw operation keeps/clears additional lines

- <span id="drawable-state"></span>`fn state(&mut self) -> DrawStateWrapper<'_>` — [`DrawStateWrapper`](#drawstatewrapper)

- <span id="drawable-clear"></span>`fn clear(self) -> io::Result<()>`

- <span id="drawable-draw"></span>`fn draw(self) -> io::Result<()>`

- <span id="drawable-width"></span>`fn width(&self) -> Option<u16>`

#### Trait Implementations

##### `impl Any for Drawable<'a>`

- <span id="drawable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drawable<'a>`

- <span id="drawable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drawable<'a>`

- <span id="drawable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Drawable<'a>`

- <span id="drawable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Drawable<'a>`

- <span id="drawable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Drawable<'a>`

- <span id="drawable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drawable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Drawable<'a>`

- <span id="drawable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drawable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineAdjust`

```rust
enum LineAdjust {
    Clear(VisualLines),
    Keep(VisualLines),
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:373-378`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L373-L378)*

#### Variants

- **`Clear`**

  Adds to `last_line_count` so that the next draw also clears those lines

- **`Keep`**

  Subtracts from `last_line_count` so that the next draw retains those lines

#### Trait Implementations

##### `impl Any for LineAdjust`

- <span id="lineadjust-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineAdjust`

- <span id="lineadjust-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineAdjust`

- <span id="lineadjust-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LineAdjust`

- <span id="lineadjust-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineAdjust`

- <span id="lineadjust-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LineAdjust`

- <span id="lineadjust-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lineadjust-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineAdjust`

- <span id="lineadjust-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lineadjust-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineType`

```rust
enum LineType {
    Text(String),
    Bar(String),
    Empty,
}
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:653-657`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L653-L657)*

#### Implementations

- <span id="linetype-wrapped-height"></span>`fn wrapped_height(&self, width: usize) -> VisualLines` — [`VisualLines`](#visuallines)

- <span id="linetype-console-width"></span>`fn console_width(&self) -> usize`

#### Trait Implementations

##### `impl Any for LineType`

- <span id="linetype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for LineType`

- <span id="linetype-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for LineType`

- <span id="linetype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineType`

- <span id="linetype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LineType`

- <span id="linetype-clone"></span>`fn clone(&self) -> LineType` — [`LineType`](#linetype)

##### `impl CloneToUninit for LineType`

- <span id="linetype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LineType`

- <span id="linetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LineType`

- <span id="linetype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineType`

- <span id="linetype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LineType`

- <span id="linetype-partialeq-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl ToOwned for LineType`

- <span id="linetype-toowned-type-owned"></span>`type Owned = T`

- <span id="linetype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linetype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineType`

- <span id="linetype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linetype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineType`

- <span id="linetype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linetype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `visual_line_count`

```rust
fn visual_line_count(lines: &[LineType], width: usize) -> VisualLines
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:646-650`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L646-L650)*

Calculate the number of visual lines in the given lines, after
accounting for line wrapping and non-printable characters.

## Constants

### `MAX_BURST`
```rust
const MAX_BURST: u8 = 20u8;
```

*Defined in [`indicatif-0.18.3/src/draw_target.rs:484`](../../../.source_1765633015/indicatif-0.18.3/src/draw_target.rs#L484)*

