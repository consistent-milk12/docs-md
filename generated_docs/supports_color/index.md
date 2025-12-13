# Crate `supports_color`

Detects whether a terminal supports color, and gives details about that
support. It takes into account the `NO_COLOR` environment variable.

This crate is a Rust port of [@sindresorhus](https://github.com/sindresorhus)'
[NPM package by the same name](https://npm.im/supports-color).

## Example

```rust
use supports_color::Stream;

if let Some(support) = supports_color::on(Stream::Stdout) {
    if support.has_16m {
        println!("16 million (RGB) colors are supported");
    } else if support.has_256 {
        println!("256-bit colors are supported.");
    } else if support.has_basic {
        println!("Only basic ANSI colors are supported.");
    }
} else {
    println!("No color support.");
}
```

## Contents

- [Structs](#structs)
  - [`ColorLevel`](#colorlevel)
- [Enums](#enums)
  - [`Stream`](#stream)
- [Functions](#functions)
  - [`env_force_color`](#env-force-color)
  - [`env_no_color`](#env-no-color)
  - [`as_str`](#as-str)
  - [`translate_level`](#translate-level)
  - [`is_a_tty`](#is-a-tty)
  - [`supports_color`](#supports-color)
  - [`check_ansi_color`](#check-ansi-color)
  - [`check_colorterm_16m`](#check-colorterm-16m)
  - [`check_term_16m`](#check-term-16m)
  - [`check_256_color`](#check-256-color)
  - [`on`](#on)
  - [`on_cached`](#on-cached)
- [Macros](#macros)
  - [`assert_stream_in_bounds!`](#assert-stream-in-bounds)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ColorLevel`](#colorlevel) | struct | Color level support details. |
| [`Stream`](#stream) | enum | possible stream sources |
| [`env_force_color`](#env-force-color) | fn |  |
| [`env_no_color`](#env-no-color) | fn |  |
| [`as_str`](#as-str) | fn |  |
| [`translate_level`](#translate-level) | fn |  |
| [`is_a_tty`](#is-a-tty) | fn |  |
| [`supports_color`](#supports-color) | fn |  |
| [`check_ansi_color`](#check-ansi-color) | fn |  |
| [`check_colorterm_16m`](#check-colorterm-16m) | fn |  |
| [`check_term_16m`](#check-term-16m) | fn |  |
| [`check_256_color`](#check-256-color) | fn |  |
| [`on`](#on) | fn | Returns a [ColorLevel] if a [Stream] supports terminal colors. |
| [`on_cached`](#on-cached) | fn | Returns a [ColorLevel] if a [Stream] supports terminal colors, caching the result to be returned from then on. |
| [`assert_stream_in_bounds!`](#assert-stream-in-bounds) | macro |  |

## Structs

### `ColorLevel`

```rust
struct ColorLevel {
    level: usize,
    pub has_basic: bool,
    pub has_256: bool,
    pub has_16m: bool,
}
```

*Defined in [`supports-color-3.0.2/src/lib.rs:191-199`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L191-L199)*

Color level support details.

This type is returned from [on](#on). See documentation for its fields for more details.

#### Fields

- **`has_basic`**: `bool`

  Basic ANSI colors are supported.

- **`has_256`**: `bool`

  256-bit colors are supported.

- **`has_16m`**: `bool`

  16 million (RGB) colors are supported.

#### Trait Implementations

##### `impl Any for ColorLevel`

- <span id="colorlevel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ColorLevel`

- <span id="colorlevel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ColorLevel`

- <span id="colorlevel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ColorLevel`

- <span id="colorlevel-clone"></span>`fn clone(&self) -> ColorLevel` — [`ColorLevel`](#colorlevel)

##### `impl CloneToUninit for ColorLevel`

- <span id="colorlevel-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ColorLevel`

##### `impl Debug for ColorLevel`

- <span id="colorlevel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColorLevel`

##### `impl<T> From for ColorLevel`

- <span id="colorlevel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ColorLevel`

- <span id="colorlevel-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ColorLevel`

- <span id="colorlevel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ColorLevel`

- <span id="colorlevel-partialeq-eq"></span>`fn eq(&self, other: &ColorLevel) -> bool` — [`ColorLevel`](#colorlevel)

##### `impl StructuralPartialEq for ColorLevel`

##### `impl ToOwned for ColorLevel`

- <span id="colorlevel-toowned-type-owned"></span>`type Owned = T`

- <span id="colorlevel-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="colorlevel-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ColorLevel`

- <span id="colorlevel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colorlevel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ColorLevel`

- <span id="colorlevel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colorlevel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Stream`

```rust
enum Stream {
    Stdout,
    Stderr,
}
```

*Defined in [`supports-color-3.0.2/src/lib.rs:31-34`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L31-L34)*

possible stream sources

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

##### `impl<T> From for Stream`

- <span id="stream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Stream`

- <span id="stream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

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

## Functions

### `env_force_color`

```rust
fn env_force_color() -> usize
```

*Defined in [`supports-color-3.0.2/src/lib.rs:36-52`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L36-L52)*

### `env_no_color`

```rust
fn env_no_color() -> bool
```

*Defined in [`supports-color-3.0.2/src/lib.rs:54-59`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L54-L59)*

### `as_str`

```rust
fn as_str<E>(option: &Result<String, E>) -> Result<&str, &E>
```

*Defined in [`supports-color-3.0.2/src/lib.rs:62-67`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L62-L67)*

### `translate_level`

```rust
fn translate_level(level: usize) -> Option<ColorLevel>
```

*Defined in [`supports-color-3.0.2/src/lib.rs:69-80`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L69-L80)*

### `is_a_tty`

```rust
fn is_a_tty(stream: Stream) -> bool
```

*Defined in [`supports-color-3.0.2/src/lib.rs:82-88`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L82-L88)*

### `supports_color`

```rust
fn supports_color(stream: Stream) -> usize
```

*Defined in [`supports-color-3.0.2/src/lib.rs:90-117`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L90-L117)*

### `check_ansi_color`

```rust
fn check_ansi_color(term: Option<&str>) -> bool
```

*Defined in [`supports-color-3.0.2/src/lib.rs:132-140`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L132-L140)*

### `check_colorterm_16m`

```rust
fn check_colorterm_16m(colorterm: &str) -> bool
```

*Defined in [`supports-color-3.0.2/src/lib.rs:142-144`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L142-L144)*

### `check_term_16m`

```rust
fn check_term_16m(term: &str) -> bool
```

*Defined in [`supports-color-3.0.2/src/lib.rs:146-148`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L146-L148)*

### `check_256_color`

```rust
fn check_256_color(term: &str) -> bool
```

*Defined in [`supports-color-3.0.2/src/lib.rs:150-152`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L150-L152)*

### `on`

```rust
fn on(stream: Stream) -> Option<ColorLevel>
```

*Defined in [`supports-color-3.0.2/src/lib.rs:157-159`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L157-L159)*

Returns a [ColorLevel] if a [Stream] supports terminal colors.

### `on_cached`

```rust
fn on_cached(stream: Stream) -> Option<ColorLevel>
```

*Defined in [`supports-color-3.0.2/src/lib.rs:178-183`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L178-L183)*

Returns a [ColorLevel] if a [Stream] supports terminal colors, caching the result to
be returned from then on.

If you expect your environment to change between calls, use [`on`](#on)

## Macros

### `assert_stream_in_bounds!`

*Defined in [`supports-color-3.0.2/src/lib.rs:161-167`](../../.source_1765521767/supports-color-3.0.2/src/lib.rs#L161-L167)*

