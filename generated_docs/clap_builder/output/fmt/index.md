*[clap_builder](../../index.md) / [output](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Colorizer`](#colorizer) | struct |  |
| [`Stream`](#stream) | enum |  |

## Structs

### `Colorizer`

```rust
struct Colorizer {
    stream: Stream,
    color_when: crate::util::color::ColorChoice,
    content: crate::builder::StyledStr,
}
```

*Defined in [`clap_builder-4.5.53/src/output/fmt.rs:11-16`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/fmt.rs#L11-L16)*

#### Implementations

- <span id="colorizer-new"></span>`fn new(stream: Stream, color_when: ColorChoice) -> Self` — [`Stream`](#stream), [`ColorChoice`](../../util/color/index.md#colorchoice)

- <span id="colorizer-with-content"></span>`fn with_content(self, content: StyledStr) -> Self` — [`StyledStr`](../../builder/styled_str/index.md#styledstr)

#### Trait Implementations

##### `impl Any for Colorizer`

- <span id="colorizer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Colorizer`

- <span id="colorizer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Colorizer`

- <span id="colorizer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Colorizer`

- <span id="colorizer-clone"></span>`fn clone(&self) -> Colorizer` — [`Colorizer`](#colorizer)

##### `impl CloneToUninit for Colorizer`

- <span id="colorizer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Colorizer`

- <span id="colorizer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Colorizer`

- <span id="colorizer-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> From for Colorizer`

- <span id="colorizer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Colorizer`

- <span id="colorizer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Colorizer`

- <span id="colorizer-toowned-type-owned"></span>`type Owned = T`

- <span id="colorizer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="colorizer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Colorizer`

- <span id="colorizer-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Colorizer`

- <span id="colorizer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colorizer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Colorizer`

- <span id="colorizer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colorizer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Stream`

```rust
enum Stream {
    Stdout,
    Stderr,
}
```

*Defined in [`clap_builder-4.5.53/src/output/fmt.rs:5-8`](../../../../.source_1765521767/clap_builder-4.5.53/src/output/fmt.rs#L5-L8)*

#### Trait Implementations

##### `impl Any for Stream`

- <span id="stream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Stream`

- <span id="stream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Stream`

- <span id="stream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Stream`

- <span id="stream-clone"></span>`fn clone(&self) -> Stream` — [`Stream`](#stream)

##### `impl CloneToUninit for Stream`

- <span id="stream-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- <span id="stream-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Stream`

##### `impl<T> From for Stream`

- <span id="stream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Stream`

- <span id="stream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Stream`

- <span id="stream-partialeq-eq"></span>`fn eq(&self, other: &Stream) -> bool` — [`Stream`](#stream)

##### `impl StructuralPartialEq for Stream`

##### `impl ToOwned for Stream`

- <span id="stream-toowned-type-owned"></span>`type Owned = T`

- <span id="stream-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stream-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Stream`

- <span id="stream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Stream`

- <span id="stream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

