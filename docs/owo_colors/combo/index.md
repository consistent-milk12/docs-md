*[owo_colors](../index.md) / [combo](index.md)*

---

# Module `combo`

## Structs

### `ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>`

```rust
struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>(&'a T, core::marker::PhantomData<(Fg, Bg)>);
```

A wrapper type which applies both a foreground and background color

#### Implementations

- `const fn new(thing: &'a T) -> Self`

- `const fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](../index.md), [`Color`](../index.md)

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](../index.md), [`Color`](../index.md)

- `const fn fg<C: Color>(self: Self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](../index.md)

- `const fn bg<C: Color>(self: Self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](../index.md)

- `const fn on_black(self: Self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](../index.md), [`Black`](../colors/index.md)

- `const fn black(self: Self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Black`](../colors/index.md)

- `const fn on_red(self: Self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](../index.md), [`Red`](../colors/index.md)

- `const fn red(self: Self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Red`](../colors/index.md)

- `const fn on_green(self: Self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](../index.md), [`Green`](../colors/index.md)

- `const fn green(self: Self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Green`](../colors/index.md)

- `const fn on_yellow(self: Self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](../index.md), [`Yellow`](../colors/index.md)

- `const fn yellow(self: Self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Yellow`](../colors/index.md)

- `const fn on_blue(self: Self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](../index.md), [`Blue`](../colors/index.md)

- `const fn blue(self: Self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Blue`](../colors/index.md)

- `const fn on_magenta(self: Self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](../index.md), [`Magenta`](../colors/index.md)

- `const fn magenta(self: Self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Magenta`](../colors/index.md)

- `const fn on_purple(self: Self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](../index.md), [`Magenta`](../colors/index.md)

- `const fn purple(self: Self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Magenta`](../colors/index.md)

- `const fn on_cyan(self: Self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](../index.md), [`Cyan`](../colors/index.md)

- `const fn cyan(self: Self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](../index.md), [`Cyan`](../colors/index.md)

- `const fn on_white(self: Self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](../index.md), [`White`](../colors/index.md)

- `const fn white(self: Self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](../index.md), [`White`](../colors/index.md)

- `const fn on_bright_black(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](../index.md), [`BrightBlack`](../colors/index.md)

- `const fn bright_black(self: Self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightBlack`](../colors/index.md)

- `const fn on_bright_red(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](../index.md), [`BrightRed`](../colors/index.md)

- `const fn bright_red(self: Self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightRed`](../colors/index.md)

- `const fn on_bright_green(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](../index.md), [`BrightGreen`](../colors/index.md)

- `const fn bright_green(self: Self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightGreen`](../colors/index.md)

- `const fn on_bright_yellow(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](../index.md), [`BrightYellow`](../colors/index.md)

- `const fn bright_yellow(self: Self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightYellow`](../colors/index.md)

- `const fn on_bright_blue(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](../index.md), [`BrightBlue`](../colors/index.md)

- `const fn bright_blue(self: Self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightBlue`](../colors/index.md)

- `const fn on_bright_magenta(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](../index.md), [`BrightMagenta`](../colors/index.md)

- `const fn bright_magenta(self: Self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightMagenta`](../colors/index.md)

- `const fn on_bright_purple(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](../index.md), [`BrightMagenta`](../colors/index.md)

- `const fn bright_purple(self: Self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightMagenta`](../colors/index.md)

- `const fn on_bright_cyan(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](../index.md), [`BrightCyan`](../colors/index.md)

- `const fn bright_cyan(self: Self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightCyan`](../colors/index.md)

- `const fn on_bright_white(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](../index.md), [`BrightWhite`](../colors/index.md)

- `const fn bright_white(self: Self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](../index.md), [`BrightWhite`](../colors/index.md)

#### Trait Implementations

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Binary> Binary for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Debug> Debug for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Display> Display for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerExp> LowerExp for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerHex> LowerHex for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Octal> Octal for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ComboColorDisplay<'a, Fg, Bg, T>`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Pointer> Pointer for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperExp> UpperExp for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperHex> UpperHex for ComboColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>`

```rust
struct ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>(&'a T, Fg, Bg);
```

Wrapper around a type which implements all the formatters the wrapped type does, with the
addition of changing the foreground and background color.

If compile-time coloring is an option, consider using [`ComboColorDisplay`](../index.md) instead.

#### Implementations

- `const fn new(thing: &'a T, fg: Fg, bg: Bg) -> Self`

- `fn into_styled(self: Self) -> Styled<&'a T>` — [`Styled`](../index.md)

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> ComboDynColorDisplay<'a, Fg, NewBg, T>` — [`ComboDynColorDisplay`](../index.md)

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, Bg, T>` — [`ComboDynColorDisplay`](../index.md)

#### Trait Implementations

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Binary> Binary for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Debug> Debug for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Display> Display for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerExp> LowerExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerHex> LowerHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Octal> Octal for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ComboDynColorDisplay<'a, Fg, Bg, T>`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Pointer> Pointer for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperExp> UpperExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperHex> UpperHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `impl_fmt_for_combo!`

### `color_methods!`

implement specialized color methods for FgColorDisplay BgColorDisplay, ComboColorDisplay

