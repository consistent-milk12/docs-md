*[owo_colors](../index.md) / [colors](index.md)*

---

# Module `colors`

Color types for used for being generic over the color

## Contents

- [Modules](#modules)
  - [`ansi_colors`](#ansi-colors)
  - [`css`](#css)
  - [`xterm`](#xterm)
  - [`custom`](#custom)
  - [`dynamic`](#dynamic)
- [Structs](#structs)
  - [`Black`](#black)
  - [`Red`](#red)
  - [`Green`](#green)
  - [`Yellow`](#yellow)
  - [`Blue`](#blue)
  - [`Magenta`](#magenta)
  - [`Cyan`](#cyan)
  - [`White`](#white)
  - [`Default`](#default)
  - [`BrightBlack`](#brightblack)
  - [`BrightRed`](#brightred)
  - [`BrightGreen`](#brightgreen)
  - [`BrightYellow`](#brightyellow)
  - [`BrightBlue`](#brightblue)
  - [`BrightMagenta`](#brightmagenta)
  - [`BrightCyan`](#brightcyan)
  - [`BrightWhite`](#brightwhite)
  - [`CustomColor`](#customcolor)
- [Macros](#macros)
  - [`colors!`](#colors)
  - [`impl_fmt_for!`](#impl-fmt-for)
  - [`impl_fmt_for_dyn!`](#impl-fmt-for-dyn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ansi_colors`](#ansi-colors) | mod |  |
| [`css`](#css) | mod | CSS named colors. |
| [`xterm`](#xterm) | mod | XTerm 256-bit colors. |
| [`custom`](#custom) | mod |  |
| [`dynamic`](#dynamic) | mod |  |
| [`Black`](#black) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Red`](#red) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Green`](#green) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Yellow`](#yellow) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Blue`](#blue) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Magenta`](#magenta) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Cyan`](#cyan) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`White`](#white) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Default`](#default) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightBlack`](#brightblack) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightRed`](#brightred) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightGreen`](#brightgreen) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightYellow`](#brightyellow) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightBlue`](#brightblue) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightMagenta`](#brightmagenta) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightCyan`](#brightcyan) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightWhite`](#brightwhite) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`CustomColor`](#customcolor) | struct |  |
| [`colors!`](#colors) | macro |  |
| [`impl_fmt_for!`](#impl-fmt-for) | macro |  |
| [`impl_fmt_for_dyn!`](#impl-fmt-for-dyn) | macro |  |

## Modules

- [`ansi_colors`](ansi_colors/index.md)
- [`css`](css/index.md) — CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.
- [`xterm`](xterm/index.md) — XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.
- [`custom`](custom/index.md)
- [`dynamic`](dynamic/index.md)

## Structs

### `Black`

```rust
struct Black;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Black`

- <span id="black-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Black`

- <span id="black-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Black`

- <span id="black-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Black`

- <span id="black-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="black-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Black`

- <span id="black-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Black`

- <span id="black-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Black`

##### `impl<U> TryFrom for Black`

- <span id="black-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="black-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Black`

- <span id="black-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="black-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Red`

```rust
struct Red;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Red`

- <span id="red-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Red`

- <span id="red-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Red`

- <span id="red-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Red`

- <span id="red-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="red-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Red`

- <span id="red-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Red`

- <span id="red-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Red`

##### `impl<U> TryFrom for Red`

- <span id="red-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="red-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Red`

- <span id="red-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="red-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Green`

```rust
struct Green;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Green`

- <span id="green-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Green`

- <span id="green-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Green`

- <span id="green-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Green`

- <span id="green-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="green-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Green`

- <span id="green-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Green`

- <span id="green-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Green`

##### `impl<U> TryFrom for Green`

- <span id="green-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="green-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Green`

- <span id="green-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="green-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Yellow`

```rust
struct Yellow;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Yellow`

- <span id="yellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Yellow`

- <span id="yellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Yellow`

- <span id="yellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Yellow`

- <span id="yellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="yellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Yellow`

- <span id="yellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Yellow`

- <span id="yellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Yellow`

##### `impl<U> TryFrom for Yellow`

- <span id="yellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="yellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Yellow`

- <span id="yellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="yellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Blue`

```rust
struct Blue;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Blue`

- <span id="blue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Blue`

- <span id="blue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Blue`

- <span id="blue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Blue`

- <span id="blue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="blue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Blue`

- <span id="blue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Blue`

- <span id="blue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Blue`

##### `impl<U> TryFrom for Blue`

- <span id="blue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Blue`

- <span id="blue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Magenta`

```rust
struct Magenta;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Magenta`

- <span id="magenta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Magenta`

- <span id="magenta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Magenta`

- <span id="magenta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Magenta`

- <span id="magenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="magenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="magenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="magenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Magenta`

- <span id="magenta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Magenta`

- <span id="magenta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Magenta`

##### `impl<U> TryFrom for Magenta`

- <span id="magenta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="magenta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Magenta`

- <span id="magenta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="magenta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cyan`

```rust
struct Cyan;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Cyan`

- <span id="cyan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cyan`

- <span id="cyan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cyan`

- <span id="cyan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Cyan`

- <span id="cyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="cyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Cyan`

- <span id="cyan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cyan`

- <span id="cyan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Cyan`

##### `impl<U> TryFrom for Cyan`

- <span id="cyan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cyan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cyan`

- <span id="cyan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cyan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `White`

```rust
struct White;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for White`

- <span id="white-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for White`

- <span id="white-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for White`

- <span id="white-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for White`

- <span id="white-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="white-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for White`

- <span id="white-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for White`

- <span id="white-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for White`

##### `impl<U> TryFrom for White`

- <span id="white-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="white-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for White`

- <span id="white-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="white-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Default`

```rust
struct Default;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for Default`

- <span id="default-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Default`

- <span id="default-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Default`

- <span id="default-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Default`

- <span id="default-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="default-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="default-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="default-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for Default`

- <span id="default-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Default`

- <span id="default-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Default`

##### `impl<U> TryFrom for Default`

- <span id="default-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="default-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Default`

- <span id="default-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="default-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightBlack`

```rust
struct BrightBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightBlack`

- <span id="brightblack-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightBlack`

- <span id="brightblack-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightBlack`

- <span id="brightblack-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightBlack`

- <span id="brightblack-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightblack-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightblack-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightblack-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightBlack`

- <span id="brightblack-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightBlack`

- <span id="brightblack-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightBlack`

##### `impl<U> TryFrom for BrightBlack`

- <span id="brightblack-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightblack-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightBlack`

- <span id="brightblack-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightblack-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightRed`

```rust
struct BrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightRed`

- <span id="brightred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightRed`

- <span id="brightred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightRed`

- <span id="brightred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightRed`

- <span id="brightred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightRed`

- <span id="brightred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightRed`

- <span id="brightred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightRed`

##### `impl<U> TryFrom for BrightRed`

- <span id="brightred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightRed`

- <span id="brightred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightGreen`

```rust
struct BrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightGreen`

- <span id="brightgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightGreen`

- <span id="brightgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightGreen`

- <span id="brightgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightGreen`

- <span id="brightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightGreen`

- <span id="brightgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightGreen`

- <span id="brightgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightGreen`

##### `impl<U> TryFrom for BrightGreen`

- <span id="brightgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightGreen`

- <span id="brightgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightYellow`

```rust
struct BrightYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightYellow`

- <span id="brightyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightYellow`

- <span id="brightyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightYellow`

- <span id="brightyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightYellow`

- <span id="brightyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightYellow`

- <span id="brightyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightYellow`

- <span id="brightyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightYellow`

##### `impl<U> TryFrom for BrightYellow`

- <span id="brightyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightYellow`

- <span id="brightyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightBlue`

```rust
struct BrightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightBlue`

- <span id="brightblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightBlue`

- <span id="brightblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightBlue`

- <span id="brightblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightBlue`

- <span id="brightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightBlue`

- <span id="brightblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightBlue`

- <span id="brightblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightBlue`

##### `impl<U> TryFrom for BrightBlue`

- <span id="brightblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightBlue`

- <span id="brightblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightMagenta`

```rust
struct BrightMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightMagenta`

- <span id="brightmagenta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightMagenta`

- <span id="brightmagenta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightMagenta`

- <span id="brightmagenta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightMagenta`

- <span id="brightmagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightmagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightmagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightmagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightMagenta`

- <span id="brightmagenta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightMagenta`

- <span id="brightmagenta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightMagenta`

##### `impl<U> TryFrom for BrightMagenta`

- <span id="brightmagenta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightmagenta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightMagenta`

- <span id="brightmagenta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightmagenta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightCyan`

```rust
struct BrightCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightCyan`

- <span id="brightcyan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightCyan`

- <span id="brightcyan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightCyan`

- <span id="brightcyan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightCyan`

- <span id="brightcyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightcyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightcyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightcyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightCyan`

- <span id="brightcyan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightCyan`

- <span id="brightcyan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightCyan`

##### `impl<U> TryFrom for BrightCyan`

- <span id="brightcyan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightcyan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightCyan`

- <span id="brightcyan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightcyan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightWhite`

```rust
struct BrightWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Any for BrightWhite`

- <span id="brightwhite-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightWhite`

- <span id="brightwhite-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightWhite`

- <span id="brightwhite-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightWhite`

- <span id="brightwhite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightwhite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightwhite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightwhite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for BrightWhite`

- <span id="brightwhite-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightWhite`

- <span id="brightwhite-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightWhite`

##### `impl<U> TryFrom for BrightWhite`

- <span id="brightwhite-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightwhite-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightWhite`

- <span id="brightwhite-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightwhite-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CustomColor<const R: u8, const G: u8, const B: u8>`

```rust
struct CustomColor<const R: u8, const G: u8, const B: u8>;
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:83`](../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L83)*

A custom RGB color, determined at compile time

#### Implementations

- <span id="customcolor-const-ansi-fg-u8"></span>`const ANSI_FG_U8: [u8; 19]`

- <span id="customcolor-const-ansi-bg-u8"></span>`const ANSI_BG_U8: [u8; 19]`

- <span id="customcolor-const-raw-ansi-fg-u8"></span>`const RAW_ANSI_FG_U8: [u8; 16]`

- <span id="customcolor-const-raw-ansi-bg-u8"></span>`const RAW_ANSI_BG_U8: [u8; 16]`

#### Trait Implementations

##### `impl Any for CustomColor<R, G, B>`

- <span id="customcolor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CustomColor<R, G, B>`

- <span id="customcolor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CustomColor<R, G, B>`

- <span id="customcolor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CustomColor<R, G, B>`

- <span id="customcolor-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="customcolor-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="customcolor-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="customcolor-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for CustomColor<R, G, B>`

- <span id="customcolor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CustomColor<R, G, B>`

- <span id="customcolor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CustomColor<R, G, B>`

##### `impl<U> TryFrom for CustomColor<R, G, B>`

- <span id="customcolor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="customcolor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CustomColor<R, G, B>`

- <span id="customcolor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="customcolor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `colors!`

*Defined in [`owo-colors-4.2.3/src/colors.rs:5-106`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L5-L106)*

### `impl_fmt_for!`

*Defined in [`owo-colors-4.2.3/src/colors.rs:129-151`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L129-L151)*

### `impl_fmt_for_dyn!`

*Defined in [`owo-colors-4.2.3/src/colors.rs:165-187`](../../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L165-L187)*

