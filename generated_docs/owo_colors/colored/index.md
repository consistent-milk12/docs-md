*[owo_colors](../index.md) / [colored](index.md)*

---

# Module `colored`

Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from
[`colored`](https://docs.rs/colored) to owo-colors.

Just replace:

```rust
mod colored {}
use colored::*;
```

with

```rust
use owo_colors::colored::*;
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Color`](#color) | enum |  |
| [`OwoColorize`](#owocolorize) | trait |  |

## Enums

### `Color`

```rust
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}
```

Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for AnsiColors`

- <span id="ansicolors-clone"></span>`fn clone(&self) -> AnsiColors` — [`AnsiColors`](../index.md)

##### `impl Copy for AnsiColors`

##### `impl Debug for AnsiColors`

- <span id="ansicolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for AnsiColors`

- <span id="ansicolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColors`

##### `impl<D> OwoColorize for AnsiColors`

##### `impl PartialEq for AnsiColors`

- <span id="ansicolors-eq"></span>`fn eq(&self, other: &AnsiColors) -> bool` — [`AnsiColors`](../index.md)

##### `impl StructuralPartialEq for AnsiColors`

## Traits

### `OwoColorize`

```rust
trait OwoColorize: Sized { ... }
```

Extension trait for colorizing a type which implements any std formatter
([`Display`](core::fmt::Display), [`Debug`](core::fmt::Debug), [`UpperHex`](core::fmt::UpperHex),
etc.)

## Example

```rust
use owo_colors::OwoColorize;

println!("My number is {:#x}!", 10.green());
println!("My number is not {}!", 4.on_red());
```

## How to decide which method to use

**Do you have a specific color you want to use?**

Use the specific color's method, such as [`blue`](OwoColorize::blue) or
[`on_green`](OwoColorize::on_green).


**Do you want your colors configurable via generics?**

Use [`fg`](OwoColorize::fg) and [`bg`](OwoColorize::bg) to make it compile-time configurable.


**Do you need to pick a color at runtime?**

Use the [`color`](OwoColorize::color), [`on_color`](OwoColorize::on_color),
[`truecolor`](OwoColorize::truecolor) or [`on_truecolor`](OwoColorize::on_truecolor).

**Do you need some other text modifier?**

* [`bold`](OwoColorize::bold)
* [`dimmed`](OwoColorize::dimmed)
* [`italic`](OwoColorize::italic)
* [`underline`](OwoColorize::underline)
* [`blink`](OwoColorize::blink)
* [`blink_fast`](OwoColorize::blink_fast)
* [`reversed`](OwoColorize::reversed)
* [`hidden`](OwoColorize::hidden)
* [`strikethrough`](OwoColorize::strikethrough)

**Do you want it to only display colors if it's a terminal?**

1. Enable the `supports-colors` feature
2. Colorize inside [`if_supports_color`](OwoColorize::if_supports_color)

**Do you need to store a set of colors/effects to apply to multiple things?**

Use [`style`](OwoColorize::style) to apply a [`Style`](../index.md)


#### Provided Methods

- `fn fg<C: Color>(&self) -> FgColorDisplay<'_, C, Self>`

  Set the foreground color generically

- `fn bg<C: Color>(&self) -> BgColorDisplay<'_, C, Self>`

  Set the background color generically.

- `fn black(&self) -> FgColorDisplay<'_, colors::Black, Self>`

  Change the foreground color to black

- `fn on_black(&self) -> BgColorDisplay<'_, colors::Black, Self>`

  Change the background color to black

- `fn red(&self) -> FgColorDisplay<'_, colors::Red, Self>`

  Change the foreground color to red

- `fn on_red(&self) -> BgColorDisplay<'_, colors::Red, Self>`

  Change the background color to red

- `fn green(&self) -> FgColorDisplay<'_, colors::Green, Self>`

  Change the foreground color to green

- `fn on_green(&self) -> BgColorDisplay<'_, colors::Green, Self>`

  Change the background color to green

- `fn yellow(&self) -> FgColorDisplay<'_, colors::Yellow, Self>`

  Change the foreground color to yellow

- `fn on_yellow(&self) -> BgColorDisplay<'_, colors::Yellow, Self>`

  Change the background color to yellow

- `fn blue(&self) -> FgColorDisplay<'_, colors::Blue, Self>`

  Change the foreground color to blue

- `fn on_blue(&self) -> BgColorDisplay<'_, colors::Blue, Self>`

  Change the background color to blue

- `fn magenta(&self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to magenta

- `fn on_magenta(&self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to magenta

- `fn purple(&self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to purple

- `fn on_purple(&self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to purple

- `fn cyan(&self) -> FgColorDisplay<'_, colors::Cyan, Self>`

  Change the foreground color to cyan

- `fn on_cyan(&self) -> BgColorDisplay<'_, colors::Cyan, Self>`

  Change the background color to cyan

- `fn white(&self) -> FgColorDisplay<'_, colors::White, Self>`

  Change the foreground color to white

- `fn on_white(&self) -> BgColorDisplay<'_, colors::White, Self>`

  Change the background color to white

- `fn default_color(&self) -> FgColorDisplay<'_, colors::Default, Self>`

  Change the foreground color to the terminal default

- `fn on_default_color(&self) -> BgColorDisplay<'_, colors::Default, Self>`

  Change the background color to the terminal default

- `fn bright_black(&self) -> FgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the foreground color to bright black

- `fn on_bright_black(&self) -> BgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the background color to bright black

- `fn bright_red(&self) -> FgColorDisplay<'_, colors::BrightRed, Self>`

  Change the foreground color to bright red

- `fn on_bright_red(&self) -> BgColorDisplay<'_, colors::BrightRed, Self>`

  Change the background color to bright red

- `fn bright_green(&self) -> FgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the foreground color to bright green

- `fn on_bright_green(&self) -> BgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the background color to bright green

- `fn bright_yellow(&self) -> FgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the foreground color to bright yellow

- `fn on_bright_yellow(&self) -> BgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the background color to bright yellow

- `fn bright_blue(&self) -> FgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the foreground color to bright blue

- `fn on_bright_blue(&self) -> BgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the background color to bright blue

- `fn bright_magenta(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright magenta

- `fn on_bright_magenta(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright magenta

- `fn bright_purple(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright purple

- `fn on_bright_purple(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright purple

- `fn bright_cyan(&self) -> FgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the foreground color to bright cyan

- `fn on_bright_cyan(&self) -> BgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the background color to bright cyan

- `fn bright_white(&self) -> FgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the foreground color to bright white

- `fn on_bright_white(&self) -> BgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the background color to bright white

- `fn bold(&self) -> styles::BoldDisplay<'_, Self>`

  Make the text bold

- `fn dimmed(&self) -> styles::DimDisplay<'_, Self>`

  Make the text dim

- `fn italic(&self) -> styles::ItalicDisplay<'_, Self>`

  Make the text italicized

- `fn underline(&self) -> styles::UnderlineDisplay<'_, Self>`

  Make the text underlined

- `fn blink(&self) -> styles::BlinkDisplay<'_, Self>`

  Make the text blink

- `fn blink_fast(&self) -> styles::BlinkFastDisplay<'_, Self>`

  Make the text blink (but fast!)

- `fn reversed(&self) -> styles::ReversedDisplay<'_, Self>`

  Swap the foreground and background colors

- `fn hidden(&self) -> styles::HiddenDisplay<'_, Self>`

  Hide the text

- `fn strikethrough(&self) -> styles::StrikeThroughDisplay<'_, Self>`

  Cross out the text

- `fn color<Color: DynColor>(&self, color: Color) -> FgDynColorDisplay<'_, Color, Self>`

  Set the foreground color at runtime. Only use if you do not know which color will be used at

- `fn on_color<Color: DynColor>(&self, color: Color) -> BgDynColorDisplay<'_, Color, Self>`

  Set the background color at runtime. Only use if you do not know what color to use at

- `fn fg_rgb<const R: u8, const G: u8, const B: u8>(&self) -> FgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the foreground color to a specific RGB value.

- `fn bg_rgb<const R: u8, const G: u8, const B: u8>(&self) -> BgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the background color to a specific RGB value.

- `fn truecolor(&self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'_, Rgb, Self>`

  Sets the foreground color to an RGB value.

- `fn on_truecolor(&self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'_, Rgb, Self>`

  Sets the background color to an RGB value.

- `fn style(&self, style: Style) -> Styled<&Self>`

  Apply a runtime-determined style

#### Implementors

- [`AeroBlue`](../colors/xterm/index.md)
- [`AltoBeige`](../colors/xterm/index.md)
- [`Alto`](../colors/xterm/index.md)
- [`AnakiwaBlue`](../colors/xterm/index.md)
- [`AnsiColors`](../index.md)
- [`Aqua`](../colors/xterm/index.md)
- [`Aquamarine`](../colors/xterm/index.md)
- [`AzureRadiance`](../colors/xterm/index.md)
- [`BayLeaf`](../colors/xterm/index.md)
- [`Bermuda`](../colors/xterm/index.md)
- [`BgColorDisplay`](../index.md)
- [`BgDynColorDisplay`](../index.md)
- [`BittersweetOrange`](../colors/xterm/index.md)
- [`Black`](../colors/index.md)
- [`Black`](../colors/xterm/index.md)
- [`BlazeOrange`](../colors/xterm/index.md)
- [`BlinkDisplay`](../styles/index.md)
- [`BlinkFastDisplay`](../styles/index.md)
- [`BlueRibbon`](../colors/xterm/index.md)
- [`BlueStone`](../colors/xterm/index.md)
- [`Blue`](../colors/index.md)
- [`Blue`](../colors/xterm/index.md)
- [`BlushPink`](../colors/xterm/index.md)
- [`BoldDisplay`](../styles/index.md)
- [`BondiBlue`](../colors/xterm/index.md)
- [`Boulder`](../colors/xterm/index.md)
- [`Bouquet`](../colors/xterm/index.md)
- [`BrightBlack`](../colors/index.md)
- [`BrightBlue`](../colors/index.md)
- [`BrightCyan`](../colors/index.md)
- [`BrightElectricViolet`](../colors/xterm/index.md)
- [`BrightGreen`](../colors/index.md)
- [`BrightGreen`](../colors/xterm/index.md)
- [`BrightHeliotrope`](../colors/xterm/index.md)
- [`BrightMagenta`](../colors/index.md)
- [`BrightRed`](../colors/index.md)
- [`BrightRed`](../colors/xterm/index.md)
- [`BrightTurquoise`](../colors/xterm/index.md)
- [`BrightWhite`](../colors/index.md)
- [`BrightYellow`](../colors/index.md)
- [`BrighterElectricViolet`](../colors/xterm/index.md)
- [`Brown`](../colors/xterm/index.md)
- [`BuddhaGold`](../colors/xterm/index.md)
- [`CamaroneGreen`](../colors/xterm/index.md)
- [`CanCanPink`](../colors/xterm/index.md)
- [`Canary`](../colors/xterm/index.md)
- [`Caramel`](../colors/xterm/index.md)
- [`CaribbeanGreen`](../colors/xterm/index.md)
- [`Celadon`](../colors/xterm/index.md)
- [`Cerulean`](../colors/xterm/index.md)
- [`ChartreuseGreen`](../colors/xterm/index.md)
- [`ChartreuseYellow`](../colors/xterm/index.md)
- [`ChelseaCucumber`](../colors/xterm/index.md)
- [`ChetwodeBlue`](../colors/xterm/index.md)
- [`ClamShell`](../colors/xterm/index.md)
- [`ClayCreekOlive`](../colors/xterm/index.md)
- [`CodGray`](../colors/xterm/index.md)
- [`ComboColorDisplay`](../index.md)
- [`ComboDynColorDisplay`](../index.md)
- [`ConiferGreen`](../colors/xterm/index.md)
- [`CopperRose`](../colors/xterm/index.md)
- [`Copperfield`](../colors/xterm/index.md)
- [`Corn`](../colors/xterm/index.md)
- [`CornflowerBlue`](../colors/xterm/index.md)
- [`CosmosSalmon`](../colors/xterm/index.md)
- [`CottonCandy`](../colors/xterm/index.md)
- [`CranberryPink`](../colors/xterm/index.md)
- [`CssColors`](../index.md)
- [`Cumulus`](../colors/xterm/index.md)
- [`CustomColor`](../colors/index.md)
- [`Cyan`](../colors/index.md)
- [`Cyan`](../colors/xterm/index.md)
- [`Dandelion`](../colors/xterm/index.md)
- [`DarkAlto`](../colors/xterm/index.md)
- [`DarkAnakiwaBlue`](../colors/xterm/index.md)
- [`DarkAquamarine`](../colors/xterm/index.md)
- [`DarkBlue`](../colors/xterm/index.md)
- [`DarkBrightGreen`](../colors/xterm/index.md)
- [`DarkCodGray`](../colors/xterm/index.md)
- [`DarkCorn`](../colors/xterm/index.md)
- [`DarkCornflowerBlue`](../colors/xterm/index.md)
- [`DarkDoveGray`](../colors/xterm/index.md)
- [`DarkFeijoaGreen`](../colors/xterm/index.md)
- [`DarkFlirt`](../colors/xterm/index.md)
- [`DarkFreshEggplant`](../colors/xterm/index.md)
- [`DarkGray`](../colors/xterm/index.md)
- [`DarkGreen`](../colors/xterm/index.md)
- [`DarkHeliotropePurple`](../colors/xterm/index.md)
- [`DarkHotPink`](../colors/xterm/index.md)
- [`DarkLavenderRose`](../colors/xterm/index.md)
- [`DarkLimeade`](../colors/xterm/index.md)
- [`DarkMalibuBlue`](../colors/xterm/index.md)
- [`DarkMediumPurple`](../colors/xterm/index.md)
- [`DarkMineShaft`](../colors/xterm/index.md)
- [`DarkMintGreen`](../colors/xterm/index.md)
- [`DarkPastelGreen`](../colors/xterm/index.md)
- [`DarkPurplePizzazz`](../colors/xterm/index.md)
- [`DarkPurple`](../colors/xterm/index.md)
- [`DarkRose`](../colors/xterm/index.md)
- [`DarkScreaminGreen`](../colors/xterm/index.md)
- [`DarkSilverChalice`](../colors/xterm/index.md)
- [`DarkSilver`](../colors/xterm/index.md)
- [`DarkSpringGreen`](../colors/xterm/index.md)
- [`DarkTachaOrange`](../colors/xterm/index.md)
- [`DarkTundora`](../colors/xterm/index.md)
- [`DarkViolet`](../colors/xterm/index.md)
- [`DecoOrange`](../colors/xterm/index.md)
- [`DeepCerulean`](../colors/xterm/index.md)
- [`DeepSeaGreen`](../colors/xterm/index.md)
- [`Default`](../colors/index.md)
- [`DelugePurple`](../colors/xterm/index.md)
- [`DimDisplay`](../styles/index.md)
- [`DollyYellow`](../colors/xterm/index.md)
- [`DoveGray`](../colors/xterm/index.md)
- [`DownyTeal`](../colors/xterm/index.md)
- [`DustyGray`](../colors/xterm/index.md)
- [`DynColors`](../index.md)
- [`Effect`](../index.md)
- [`ElectricIndigo`](../colors/xterm/index.md)
- [`ElectricPurple`](../colors/xterm/index.md)
- [`ElectricViolet`](../colors/xterm/index.md)
- [`EndeavourBlue`](../colors/xterm/index.md)
- [`Feijoa`](../colors/xterm/index.md)
- [`FernGreen`](../colors/xterm/index.md)
- [`FgColorDisplay`](../index.md)
- [`FgDynColorDisplay`](../index.md)
- [`Flirt`](../colors/xterm/index.md)
- [`FlushOrange`](../colors/xterm/index.md)
- [`FogPink`](../colors/xterm/index.md)
- [`FrenchPassLightBlue`](../colors/xterm/index.md)
- [`FuchsiaPink`](../colors/xterm/index.md)
- [`Fuchsia`](../colors/xterm/index.md)
- [`GalleryGray`](../colors/xterm/index.md)
- [`GladeGreen`](../colors/xterm/index.md)
- [`Gold`](../colors/xterm/index.md)
- [`GrandisCaramel`](../colors/xterm/index.md)
- [`Gray`](../colors/xterm/index.md)
- [`GreenYellow`](../colors/xterm/index.md)
- [`Green`](../colors/index.md)
- [`Green`](../colors/xterm/index.md)
- [`GuardsmanRed`](../colors/xterm/index.md)
- [`GulfStream`](../colors/xterm/index.md)
- [`HavelockBlue`](../colors/xterm/index.md)
- [`Heliotrope`](../colors/xterm/index.md)
- [`HiddenDisplay`](../styles/index.md)
- [`HillaryOlive`](../colors/xterm/index.md)
- [`HippieBlue`](../colors/xterm/index.md)
- [`HollywoodCerise`](../colors/xterm/index.md)
- [`Honeysuckle`](../colors/xterm/index.md)
- [`HopbushPink`](../colors/xterm/index.md)
- [`HotPink`](../colors/xterm/index.md)
- [`Indigo`](../colors/xterm/index.md)
- [`ItalicDisplay`](../styles/index.md)
- [`Jade`](../colors/xterm/index.md)
- [`JapaneseLaurel`](../colors/xterm/index.md)
- [`JungleMist`](../colors/xterm/index.md)
- [`JuniperGreen`](../colors/xterm/index.md)
- [`LaserLemon`](../colors/xterm/index.md)
- [`LavenderRose`](../colors/xterm/index.md)
- [`Lavender`](../colors/xterm/index.md)
- [`LightAnakiwaBlue`](../colors/xterm/index.md)
- [`LightAquamarine`](../colors/xterm/index.md)
- [`LightAzureRadiance`](../colors/xterm/index.md)
- [`LightCaribbeanGreen`](../colors/xterm/index.md)
- [`LightCodGray`](../colors/xterm/index.md)
- [`LightElectricViolet`](../colors/xterm/index.md)
- [`LightFlirt`](../colors/xterm/index.md)
- [`LightFreshEggplant`](../colors/xterm/index.md)
- [`LightGray`](../colors/xterm/index.md)
- [`LightHeliotrope`](../colors/xterm/index.md)
- [`LightHollywoodCerise`](../colors/xterm/index.md)
- [`LightJapaneseLaurel`](../colors/xterm/index.md)
- [`LightLimeade`](../colors/xterm/index.md)
- [`LightMalibuBlue`](../colors/xterm/index.md)
- [`LightMineShaft`](../colors/xterm/index.md)
- [`LightMintGreen`](../colors/xterm/index.md)
- [`LightOrchid`](../colors/xterm/index.md)
- [`LightPastelGreen`](../colors/xterm/index.md)
- [`LightScreaminGreen`](../colors/xterm/index.md)
- [`LightSilverChalice`](../colors/xterm/index.md)
- [`LightSpringGreen`](../colors/xterm/index.md)
- [`LighterAquamarine`](../colors/xterm/index.md)
- [`LighterHeliotrope`](../colors/xterm/index.md)
- [`Lilac`](../colors/xterm/index.md)
- [`Lime`](../colors/xterm/index.md)
- [`Limeade`](../colors/xterm/index.md)
- [`LochmaraBlue`](../colors/xterm/index.md)
- [`Magenta`](../colors/index.md)
- [`Malachite`](../colors/xterm/index.md)
- [`MalibuBlue`](../colors/xterm/index.md)
- [`MangoTango`](../colors/xterm/index.md)
- [`Maroon`](../colors/xterm/index.md)
- [`MatrixPink`](../colors/xterm/index.md)
- [`Mauve`](../colors/xterm/index.md)
- [`MediumPurple`](../colors/xterm/index.md)
- [`MediumVioletRed`](../colors/xterm/index.md)
- [`MelroseLilac`](../colors/xterm/index.md)
- [`Mercury`](../colors/xterm/index.md)
- [`MidnightBlue`](../colors/xterm/index.md)
- [`MineShaft`](../colors/xterm/index.md)
- [`MintGreen`](../colors/xterm/index.md)
- [`MuesliOrange`](../colors/xterm/index.md)
- [`NavyBlue`](../colors/xterm/index.md)
- [`NobelGray`](../colors/xterm/index.md)
- [`OliveGreen`](../colors/xterm/index.md)
- [`Olive`](../colors/xterm/index.md)
- [`Orchid`](../colors/xterm/index.md)
- [`OrientBlue`](../colors/xterm/index.md)
- [`OysterBay`](../colors/xterm/index.md)
- [`PaleGoldenrod`](../colors/xterm/index.md)
- [`ParseColorError`](../index.md)
- [`PastelGreen`](../colors/xterm/index.md)
- [`PersianGreen`](../colors/xterm/index.md)
- [`PharlapPink`](../colors/xterm/index.md)
- [`PigmentIndigo`](../colors/xterm/index.md)
- [`PinkFlamingo`](../colors/xterm/index.md)
- [`PinkLace`](../colors/xterm/index.md)
- [`PinkSalmon`](../colors/xterm/index.md)
- [`PirateGold`](../colors/xterm/index.md)
- [`Pistachio`](../colors/xterm/index.md)
- [`PixieGreen`](../colors/xterm/index.md)
- [`Plane`](../colors/custom/index.md)
- [`PoloBlue`](../colors/xterm/index.md)
- [`PompadourMagenta`](../colors/xterm/index.md)
- [`PortafinoYellow`](../colors/xterm/index.md)
- [`PurplePizzazz`](../colors/xterm/index.md)
- [`Purple`](../colors/xterm/index.md)
- [`RazzmatazzCerise`](../colors/xterm/index.md)
- [`Red`](../colors/index.md)
- [`Red`](../colors/xterm/index.md)
- [`ReefPaleYellow`](../colors/xterm/index.md)
- [`ReversedDisplay`](../styles/index.md)
- [`Rgb`](../index.md)
- [`RioGrandeGreen`](../colors/xterm/index.md)
- [`RobinEggBlue`](../colors/xterm/index.md)
- [`RomanOrange`](../colors/xterm/index.md)
- [`Rose`](../colors/xterm/index.md)
- [`RoseofSharonOrange`](../colors/xterm/index.md)
- [`Rosewood`](../colors/xterm/index.md)
- [`Salmon`](../colors/xterm/index.md)
- [`ScampiIndigo`](../colors/xterm/index.md)
- [`ScienceBlue`](../colors/xterm/index.md)
- [`ScorpionGray`](../colors/xterm/index.md)
- [`ScorpionOlive`](../colors/xterm/index.md)
- [`ScreaminGreen`](../colors/xterm/index.md)
- [`SeaPink`](../colors/xterm/index.md)
- [`ShakespeareBlue`](../colors/xterm/index.md)
- [`SilverChalice`](../colors/xterm/index.md)
- [`SilverTree`](../colors/xterm/index.md)
- [`Silver`](../colors/xterm/index.md)
- [`SlateBlue`](../colors/xterm/index.md)
- [`SnowyMint`](../colors/xterm/index.md)
- [`SpringGreen`](../colors/xterm/index.md)
- [`StratosBlue`](../colors/xterm/index.md)
- [`StrikeThroughDisplay`](../styles/index.md)
- [`StrikemasterPurple`](../colors/xterm/index.md)
- [`StyleFlags`](../dyn_styles/index.md)
- [`StylePrefixFormatter`](../index.md)
- [`StyleSuffixFormatter`](../index.md)
- [`Style`](../index.md)
- [`StyledList`](../index.md)
- [`Styled`](../index.md)
- [`Sundown`](../colors/xterm/index.md)
- [`Tacao`](../colors/xterm/index.md)
- [`TachaOrange`](../colors/xterm/index.md)
- [`TanBeige`](../colors/xterm/index.md)
- [`TapestryPink`](../colors/xterm/index.md)
- [`Teal`](../colors/xterm/index.md)
- [`TennOrange`](../colors/xterm/index.md)
- [`TexasRose`](../colors/xterm/index.md)
- [`ThistlePink`](../colors/xterm/index.md)
- [`Tradewind`](../colors/xterm/index.md)
- [`Transition`](../styled_list/index.md)
- [`Tundora`](../colors/xterm/index.md)
- [`UnderlineDisplay`](../styles/index.md)
- [`UserBlack`](../colors/xterm/index.md)
- [`UserBlue`](../colors/xterm/index.md)
- [`UserBrightBlack`](../colors/xterm/index.md)
- [`UserBrightBlue`](../colors/xterm/index.md)
- [`UserBrightCyan`](../colors/xterm/index.md)
- [`UserBrightGreen`](../colors/xterm/index.md)
- [`UserBrightMagenta`](../colors/xterm/index.md)
- [`UserBrightRed`](../colors/xterm/index.md)
- [`UserBrightWhite`](../colors/xterm/index.md)
- [`UserBrightYellow`](../colors/xterm/index.md)
- [`UserCyan`](../colors/xterm/index.md)
- [`UserGreen`](../colors/xterm/index.md)
- [`UserMagenta`](../colors/xterm/index.md)
- [`UserRed`](../colors/xterm/index.md)
- [`UserWhite`](../colors/xterm/index.md)
- [`UserYellow`](../colors/xterm/index.md)
- [`VerdunGreen`](../colors/xterm/index.md)
- [`Viking`](../colors/xterm/index.md)
- [`VistaBlue`](../colors/xterm/index.md)
- [`VividTangerine`](../colors/xterm/index.md)
- [`White`](../colors/index.md)
- [`White`](../colors/xterm/index.md)
- [`WildBlueYonder`](../colors/xterm/index.md)
- [`WildWatermelon`](../colors/xterm/index.md)
- [`WistfulLilac`](../colors/xterm/index.md)
- [`XtermColors`](../index.md)
- [`YellowSea`](../colors/xterm/index.md)
- [`Yellow`](../colors/index.md)
- [`Yellow`](../colors/xterm/index.md)
- `D`

