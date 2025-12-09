*[owo_colors](../index.md) / [combo](index.md)*

---

# Module `combo`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ComboColorDisplay`](#combocolordisplay) | struct | A wrapper type which applies both a foreground and background color |
| [`ComboDynColorDisplay`](#combodyncolordisplay) | struct | Wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the foreground and background color. |
| [`impl_fmt_for_combo!`](#impl_fmt_for_combo) | macro |  |
| [`color_methods!`](#color_methods) | macro | implement specialized color methods for FgColorDisplay BgColorDisplay, ComboColorDisplay |

## Structs

### `ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>`

```rust
struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>(&'a T, core::marker::PhantomData<(Fg, Bg)>);
```

*Defined in [`owo-colors-4.2.3/src/combo.rs:11`](../../../.source_1765210505/owo-colors-4.2.3/src/combo.rs#L11)*

A wrapper type which applies both a foreground and background color

#### Implementations

- <span id="combocolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

- <span id="combocolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md)

- <span id="combocolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](../index.md)

- <span id="combocolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](../index.md)

- <span id="combocolordisplay-fg"></span>`const fn fg<C: Color>(self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay)

- <span id="combocolordisplay-bg"></span>`const fn bg<C: Color>(self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](#combocolordisplay)

- <span id="combocolordisplay-on-black"></span>`const fn on_black(self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](../colors/index.md)

- <span id="combocolordisplay-black"></span>`const fn black(self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](../colors/index.md)

- <span id="combocolordisplay-on-red"></span>`const fn on_red(self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](../colors/index.md)

- <span id="combocolordisplay-red"></span>`const fn red(self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](../colors/index.md)

- <span id="combocolordisplay-on-green"></span>`const fn on_green(self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](../colors/index.md)

- <span id="combocolordisplay-green"></span>`const fn green(self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](../colors/index.md)

- <span id="combocolordisplay-on-yellow"></span>`const fn on_yellow(self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](../colors/index.md)

- <span id="combocolordisplay-yellow"></span>`const fn yellow(self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](../colors/index.md)

- <span id="combocolordisplay-on-blue"></span>`const fn on_blue(self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](../colors/index.md)

- <span id="combocolordisplay-blue"></span>`const fn blue(self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](../colors/index.md)

- <span id="combocolordisplay-on-magenta"></span>`const fn on_magenta(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md)

- <span id="combocolordisplay-magenta"></span>`const fn magenta(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md)

- <span id="combocolordisplay-on-purple"></span>`const fn on_purple(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md)

- <span id="combocolordisplay-purple"></span>`const fn purple(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md)

- <span id="combocolordisplay-on-cyan"></span>`const fn on_cyan(self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](../colors/index.md)

- <span id="combocolordisplay-cyan"></span>`const fn cyan(self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](../colors/index.md)

- <span id="combocolordisplay-on-white"></span>`const fn on_white(self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](../colors/index.md)

- <span id="combocolordisplay-white"></span>`const fn white(self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](../colors/index.md)

- <span id="combocolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](../colors/index.md)

- <span id="combocolordisplay-bright-black"></span>`const fn bright_black(self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](../colors/index.md)

- <span id="combocolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](../colors/index.md)

- <span id="combocolordisplay-bright-red"></span>`const fn bright_red(self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](../colors/index.md)

- <span id="combocolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](../colors/index.md)

- <span id="combocolordisplay-bright-green"></span>`const fn bright_green(self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](../colors/index.md)

- <span id="combocolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](../colors/index.md)

- <span id="combocolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](../colors/index.md)

- <span id="combocolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](../colors/index.md)

- <span id="combocolordisplay-bright-blue"></span>`const fn bright_blue(self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](../colors/index.md)

- <span id="combocolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md)

- <span id="combocolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md)

- <span id="combocolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md)

- <span id="combocolordisplay-bright-purple"></span>`const fn bright_purple(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md)

- <span id="combocolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](../colors/index.md)

- <span id="combocolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](../colors/index.md)

- <span id="combocolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](../colors/index.md)

- <span id="combocolordisplay-bright-white"></span>`const fn bright_white(self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](../colors/index.md)

#### Trait Implementations

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Binary> Binary for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Debug> Debug for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Display> Display for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerExp> LowerExp for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerHex> LowerHex for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Octal> Octal for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ComboColorDisplay<'a, Fg, Bg, T>`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Pointer> Pointer for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperExp> UpperExp for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperHex> UpperHex for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>`

```rust
struct ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>(&'a T, Fg, Bg);
```

*Defined in [`owo-colors-4.2.3/src/combo.rs:17`](../../../.source_1765210505/owo-colors-4.2.3/src/combo.rs#L17)*

Wrapper around a type which implements all the formatters the wrapped type does, with the
addition of changing the foreground and background color.

If compile-time coloring is an option, consider using [`ComboColorDisplay`](#combocolordisplay) instead.

#### Implementations

- <span id="combodyncolordisplay-new"></span>`const fn new(thing: &'a T, fg: Fg, bg: Bg) -> Self`

- <span id="combodyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md)

- <span id="combodyncolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, Fg, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

- <span id="combodyncolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, Bg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

#### Trait Implementations

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Binary> Binary for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Debug> Debug for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Display> Display for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerExp> LowerExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerHex> LowerHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Octal> Octal for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ComboDynColorDisplay<'a, Fg, Bg, T>`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Pointer> Pointer for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperExp> UpperExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperHex> UpperHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `impl_fmt_for_combo!`

*Defined in [`owo-colors-4.2.3/src/combo.rs:19-51`](../../../.source_1765210505/owo-colors-4.2.3/src/combo.rs#L19-L51)*

### `color_methods!`

*Defined in [`owo-colors-4.2.3/src/combo.rs:66-516`](../../../.source_1765210505/owo-colors-4.2.3/src/combo.rs#L66-L516)*

implement specialized color methods for FgColorDisplay BgColorDisplay, ComboColorDisplay

