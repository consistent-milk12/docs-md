*[owo_colors](../../index.md) / [colors](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Rgb`](#rgb) | struct | Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color) or [`OwoColorize::on_color`](OwoColorize::on_color) |

## Structs

### `Rgb`

```rust
struct Rgb(u8, u8, u8);
```

*Defined in [`owo-colors-4.2.3/src/colors/dynamic.rs:10`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/dynamic.rs#L10)*

Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for Rgb`

- <span id="rgb-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rgb`

- <span id="rgb-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rgb`

- <span id="rgb-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Rgb`

- <span id="rgb-clone"></span>`fn clone(&self) -> Rgb` — [`Rgb`](#rgb)

##### `impl CloneToUninit for Rgb`

- <span id="rgb-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Rgb`

##### `impl Debug for Rgb`

- <span id="rgb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for Rgb`

- <span id="rgb-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Rgb`

##### `impl<T> From for Rgb`

- <span id="rgb-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Rgb`

- <span id="rgb-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Rgb`

##### `impl PartialEq for Rgb`

- <span id="rgb-partialeq-eq"></span>`fn eq(&self, other: &Rgb) -> bool` — [`Rgb`](#rgb)

##### `impl StructuralPartialEq for Rgb`

##### `impl<U> TryFrom for Rgb`

- <span id="rgb-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rgb-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rgb`

- <span id="rgb-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rgb-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

