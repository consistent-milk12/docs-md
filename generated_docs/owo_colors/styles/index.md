*[owo_colors](../index.md) / [styles](index.md)*

---

# Module `styles`

Different display styles (strikethrough, bold, etc.)

## Contents

- [Structs](#structs)
  - [`BoldDisplay`](#bolddisplay)
  - [`DimDisplay`](#dimdisplay)
  - [`ItalicDisplay`](#italicdisplay)
  - [`UnderlineDisplay`](#underlinedisplay)
  - [`BlinkDisplay`](#blinkdisplay)
  - [`BlinkFastDisplay`](#blinkfastdisplay)
  - [`ReversedDisplay`](#reverseddisplay)
  - [`HiddenDisplay`](#hiddendisplay)
  - [`StrikeThroughDisplay`](#strikethroughdisplay)
- [Macros](#macros)
  - [`impl_fmt_for_style!`](#impl-fmt-for-style)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BoldDisplay`](#bolddisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of boldening it. |
| [`DimDisplay`](#dimdisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of dimming it. |
| [`ItalicDisplay`](#italicdisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of italics. |
| [`UnderlineDisplay`](#underlinedisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, while underlining it. |
| [`BlinkDisplay`](#blinkdisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, while blinking. |
| [`BlinkFastDisplay`](#blinkfastdisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of making it blink fast. |
| [`ReversedDisplay`](#reverseddisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of swapping fg and bg colors. |
| [`HiddenDisplay`](#hiddendisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of hiding the text. |
| [`StrikeThroughDisplay`](#strikethroughdisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, crossed out. |
| [`impl_fmt_for_style!`](#impl-fmt-for-style) | macro |  |

## Structs

### `BoldDisplay<'a, T: ?Sized>`

```rust
struct BoldDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:26`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L26)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of boldening it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::bold).

#### Implementations

- <span id="bolddisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_bold() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_bold() {

      "hello".bold().into_styled()

  } else {

      "hello".dimmed().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[1mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for BoldDisplay<'a, T>`

- <span id="bolddisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for BoldDisplay<'a, T>`

- <span id="bolddisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for BoldDisplay<'a, T>`

- <span id="bolddisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoldDisplay<'a, T>`

- <span id="bolddisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for BoldDisplay<'a, T>`

- <span id="bolddisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for BoldDisplay<'a, T>`

- <span id="bolddisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BoldDisplay<'a, T>`

- <span id="bolddisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for BoldDisplay<'a, T>`

- <span id="bolddisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for BoldDisplay<'a, T>`

- <span id="bolddisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for BoldDisplay<'a, T>`

- <span id="bolddisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for BoldDisplay<'a, T>`

- <span id="bolddisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BoldDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for BoldDisplay<'a, T>`

- <span id="bolddisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for BoldDisplay<'a, T>`

- <span id="bolddisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bolddisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for BoldDisplay<'a, T>`

- <span id="bolddisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bolddisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for BoldDisplay<'a, T>`

- <span id="bolddisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for BoldDisplay<'a, T>`

- <span id="bolddisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DimDisplay<'a, T: ?Sized>`

```rust
struct DimDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:66`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L66)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of dimming it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::dimmed).

#### Implementations

- <span id="dimdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_dimmed() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_dimmed() {

      "hello".dimmed().into_styled()

  } else {

      "hello".bold().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[2mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for DimDisplay<'a, T>`

- <span id="dimdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for DimDisplay<'a, T>`

- <span id="dimdisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for DimDisplay<'a, T>`

- <span id="dimdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DimDisplay<'a, T>`

- <span id="dimdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for DimDisplay<'a, T>`

- <span id="dimdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for DimDisplay<'a, T>`

- <span id="dimdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DimDisplay<'a, T>`

- <span id="dimdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DimDisplay<'a, T>`

- <span id="dimdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for DimDisplay<'a, T>`

- <span id="dimdisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for DimDisplay<'a, T>`

- <span id="dimdisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for DimDisplay<'a, T>`

- <span id="dimdisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for DimDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for DimDisplay<'a, T>`

- <span id="dimdisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for DimDisplay<'a, T>`

- <span id="dimdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dimdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DimDisplay<'a, T>`

- <span id="dimdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dimdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for DimDisplay<'a, T>`

- <span id="dimdisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for DimDisplay<'a, T>`

- <span id="dimdisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ItalicDisplay<'a, T: ?Sized>`

```rust
struct ItalicDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:106`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L106)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of italics. Recommended to be constructed using
[`OwoColorize`](OwoColorize::italic).

#### Implementations

- <span id="italicdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_italic() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_italic() {

      "hello".italic().into_styled()

  } else {

      "hello".underline().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[3mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for ItalicDisplay<'a, T>`

- <span id="italicdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for ItalicDisplay<'a, T>`

- <span id="italicdisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for ItalicDisplay<'a, T>`

- <span id="italicdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ItalicDisplay<'a, T>`

- <span id="italicdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for ItalicDisplay<'a, T>`

- <span id="italicdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for ItalicDisplay<'a, T>`

- <span id="italicdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ItalicDisplay<'a, T>`

- <span id="italicdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ItalicDisplay<'a, T>`

- <span id="italicdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for ItalicDisplay<'a, T>`

- <span id="italicdisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for ItalicDisplay<'a, T>`

- <span id="italicdisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for ItalicDisplay<'a, T>`

- <span id="italicdisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ItalicDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for ItalicDisplay<'a, T>`

- <span id="italicdisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for ItalicDisplay<'a, T>`

- <span id="italicdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="italicdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ItalicDisplay<'a, T>`

- <span id="italicdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="italicdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for ItalicDisplay<'a, T>`

- <span id="italicdisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for ItalicDisplay<'a, T>`

- <span id="italicdisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnderlineDisplay<'a, T: ?Sized>`

```rust
struct UnderlineDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:146`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L146)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while underlining it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::underline).

#### Implementations

- <span id="underlinedisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_underline() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_underline() {

      "hello".underline().into_styled()

  } else {

      "hello".italic().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[4mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for UnderlineDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="underlinedisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="underlinedisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkDisplay<'a, T: ?Sized>`

```rust
struct BlinkDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:186`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L186)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while blinking. Recommended to be constructed using
[`OwoColorize`](OwoColorize::blink).

#### Implementations

- <span id="blinkdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_blink() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_blink() {

      "hello".blink().into_styled()

  } else {

      "hello".hidden().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[5mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BlinkDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blinkdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blinkdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkFastDisplay<'a, T: ?Sized>`

```rust
struct BlinkFastDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:225`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L225)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of making it blink fast. Use [`OwoColorize`](OwoColorize::blink_fast)

#### Implementations

- <span id="blinkfastdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_blink_fast() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_blink_fast() {

      "hello".blink_fast().into_styled()

  } else {

      "hello".reversed().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[6mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BlinkFastDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blinkfastdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blinkfastdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReversedDisplay<'a, T: ?Sized>`

```rust
struct ReversedDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:264`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L264)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of swapping fg and bg colors. Use [`OwoColorize`](OwoColorize::reversed)

#### Implementations

- <span id="reverseddisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_reversed() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_reversed() {

      "hello".reversed().into_styled()

  } else {

      "hello".blink_fast().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[7mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ReversedDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reverseddisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reverseddisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HiddenDisplay<'a, T: ?Sized>`

```rust
struct HiddenDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:303`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L303)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of hiding the text. Use [`OwoColorize`](OwoColorize::hidden).

#### Implementations

- <span id="hiddendisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_hidden() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_hidden() {

      "hello".hidden().into_styled()

  } else {

      "hello".blink().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[8mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for HiddenDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hiddendisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hiddendisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StrikeThroughDisplay<'a, T: ?Sized>`

```rust
struct StrikeThroughDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:343`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L343)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
crossed out. Recommended to be constructed using
[`OwoColorize`](OwoColorize::strikethrough).

#### Implementations

- <span id="strikethroughdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_strike_through() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_strike_through() {

      "hello".strikethrough().into_styled()

  } else {

      "hello".hidden().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(styled_str.to_string(), "\x1b[9mhello\x1b[0m");

  ```

#### Trait Implementations

##### `impl<T> Any for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + fmt::Binary> Binary for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Debug> Debug for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for StrikeThroughDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strikethroughdisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strikethroughdisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `impl_fmt_for_style!`

*Defined in [`owo-colors-4.2.3/src/styles.rs:8-20`](../../../.source_1765521767/owo-colors-4.2.3/src/styles.rs#L8-L20)*

