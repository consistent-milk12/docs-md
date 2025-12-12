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

*Defined in [`owo-colors-4.2.3/src/styles.rs:26`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L26)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of boldening it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::bold).

#### Implementations

- <span id="bolddisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BoldDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for BoldDisplay<'a, T>`

- <span id="bolddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DimDisplay<'a, T: ?Sized>`

```rust
struct DimDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:66`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L66)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of dimming it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::dimmed).

#### Implementations

- <span id="dimdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for DimDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for DimDisplay<'a, T>`

- <span id="dimdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ItalicDisplay<'a, T: ?Sized>`

```rust
struct ItalicDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:106`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L106)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of italics. Recommended to be constructed using
[`OwoColorize`](OwoColorize::italic).

#### Implementations

- <span id="italicdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ItalicDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for ItalicDisplay<'a, T>`

- <span id="italicdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnderlineDisplay<'a, T: ?Sized>`

```rust
struct UnderlineDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:146`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L146)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while underlining it. Recommended to be constructed using
[`OwoColorize`](OwoColorize::underline).

#### Implementations

- <span id="underlinedisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for UnderlineDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for UnderlineDisplay<'a, T>`

- <span id="underlinedisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkDisplay<'a, T: ?Sized>`

```rust
struct BlinkDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:186`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L186)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
while blinking. Recommended to be constructed using
[`OwoColorize`](OwoColorize::blink).

#### Implementations

- <span id="blinkdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BlinkDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for BlinkDisplay<'a, T>`

- <span id="blinkdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BlinkFastDisplay<'a, T: ?Sized>`

```rust
struct BlinkFastDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:225`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L225)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of making it blink fast. Use [`OwoColorize`](OwoColorize::blink_fast)

#### Implementations

- <span id="blinkfastdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BlinkFastDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for BlinkFastDisplay<'a, T>`

- <span id="blinkfastdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReversedDisplay<'a, T: ?Sized>`

```rust
struct ReversedDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:264`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L264)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of swapping fg and bg colors. Use [`OwoColorize`](OwoColorize::reversed)

#### Implementations

- <span id="reverseddisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ReversedDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for ReversedDisplay<'a, T>`

- <span id="reverseddisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HiddenDisplay<'a, T: ?Sized>`

```rust
struct HiddenDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:303`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L303)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of hiding the text. Use [`OwoColorize`](OwoColorize::hidden).

#### Implementations

- <span id="hiddendisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for HiddenDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for HiddenDisplay<'a, T>`

- <span id="hiddendisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StrikeThroughDisplay<'a, T: ?Sized>`

```rust
struct StrikeThroughDisplay<'a, T: ?Sized>(&'a T);
```

*Defined in [`owo-colors-4.2.3/src/styles.rs:343`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L343)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
crossed out. Recommended to be constructed using
[`OwoColorize`](OwoColorize::strikethrough).

#### Implementations

- <span id="strikethroughdisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Binary> Binary for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Debug> Debug for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Display> Display for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerExp> LowerExp for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::LowerHex> LowerHex for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::Octal> Octal for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for StrikeThroughDisplay<'a, T>`

##### `impl<T: ?Sized + fmt::Pointer> Pointer for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperExp> UpperExp for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + fmt::UpperHex> UpperHex for StrikeThroughDisplay<'a, T>`

- <span id="strikethroughdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `impl_fmt_for_style!`

*Defined in [`owo-colors-4.2.3/src/styles.rs:8-20`](../../../.source_1765210505/owo-colors-4.2.3/src/styles.rs#L8-L20)*

