*[owo_colors](../../index.md) / [colors](../index.md) / [css](index.md)*

---

# Module `css`

CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.

Reference: <https://www.w3schools.com/cssref/css_colors.asp>
Reference: <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value>

## Contents

- [Modules](#modules)
  - [`dynamic`](#dynamic)
- [Type Aliases](#type-aliases)
  - [`AliceBlue`](#aliceblue)
  - [`AntiqueWhite`](#antiquewhite)
  - [`Aqua`](#aqua)
  - [`Aquamarine`](#aquamarine)
  - [`Azure`](#azure)
  - [`Beige`](#beige)
  - [`Bisque`](#bisque)
  - [`Black`](#black)
  - [`BlanchedAlmond`](#blanchedalmond)
  - [`Blue`](#blue)
  - [`BlueViolet`](#blueviolet)
  - [`Brown`](#brown)
  - [`BurlyWood`](#burlywood)
  - [`CadetBlue`](#cadetblue)
  - [`Chartreuse`](#chartreuse)
  - [`Chocolate`](#chocolate)
  - [`Coral`](#coral)
  - [`CornflowerBlue`](#cornflowerblue)
  - [`Cornsilk`](#cornsilk)
  - [`Crimson`](#crimson)
  - [`DarkBlue`](#darkblue)
  - [`DarkCyan`](#darkcyan)
  - [`DarkGoldenRod`](#darkgoldenrod)
  - [`DarkGray`](#darkgray)
  - [`DarkGrey`](#darkgrey)
  - [`DarkGreen`](#darkgreen)
  - [`DarkKhaki`](#darkkhaki)
  - [`DarkMagenta`](#darkmagenta)
  - [`DarkOliveGreen`](#darkolivegreen)
  - [`DarkOrange`](#darkorange)
  - [`DarkOrchid`](#darkorchid)
  - [`DarkRed`](#darkred)
  - [`DarkSalmon`](#darksalmon)
  - [`DarkSeaGreen`](#darkseagreen)
  - [`DarkSlateBlue`](#darkslateblue)
  - [`DarkSlateGray`](#darkslategray)
  - [`DarkSlateGrey`](#darkslategrey)
  - [`DarkTurquoise`](#darkturquoise)
  - [`DarkViolet`](#darkviolet)
  - [`DeepPink`](#deeppink)
  - [`DeepSkyBlue`](#deepskyblue)
  - [`DimGray`](#dimgray)
  - [`DimGrey`](#dimgrey)
  - [`DodgerBlue`](#dodgerblue)
  - [`FireBrick`](#firebrick)
  - [`FloralWhite`](#floralwhite)
  - [`ForestGreen`](#forestgreen)
  - [`Fuchsia`](#fuchsia)
  - [`Gainsboro`](#gainsboro)
  - [`GhostWhite`](#ghostwhite)
  - [`Gold`](#gold)
  - [`GoldenRod`](#goldenrod)
  - [`Gray`](#gray)
  - [`Grey`](#grey)
  - [`Green`](#green)
  - [`GreenYellow`](#greenyellow)
  - [`HoneyDew`](#honeydew)
  - [`HotPink`](#hotpink)
  - [`IndianRed`](#indianred)
  - [`Indigo`](#indigo)
  - [`Ivory`](#ivory)
  - [`Khaki`](#khaki)
  - [`Lavender`](#lavender)
  - [`LavenderBlush`](#lavenderblush)
  - [`LawnGreen`](#lawngreen)
  - [`LemonChiffon`](#lemonchiffon)
  - [`LightBlue`](#lightblue)
  - [`LightCoral`](#lightcoral)
  - [`LightCyan`](#lightcyan)
  - [`LightGoldenRodYellow`](#lightgoldenrodyellow)
  - [`LightGray`](#lightgray)
  - [`LightGrey`](#lightgrey)
  - [`LightGreen`](#lightgreen)
  - [`LightPink`](#lightpink)
  - [`LightSalmon`](#lightsalmon)
  - [`LightSeaGreen`](#lightseagreen)
  - [`LightSkyBlue`](#lightskyblue)
  - [`LightSlateGray`](#lightslategray)
  - [`LightSlateGrey`](#lightslategrey)
  - [`LightSteelBlue`](#lightsteelblue)
  - [`LightYellow`](#lightyellow)
  - [`Lime`](#lime)
  - [`LimeGreen`](#limegreen)
  - [`Linen`](#linen)
  - [`Magenta`](#magenta)
  - [`Maroon`](#maroon)
  - [`MediumAquaMarine`](#mediumaquamarine)
  - [`MediumBlue`](#mediumblue)
  - [`MediumOrchid`](#mediumorchid)
  - [`MediumPurple`](#mediumpurple)
  - [`MediumSeaGreen`](#mediumseagreen)
  - [`MediumSlateBlue`](#mediumslateblue)
  - [`MediumSpringGreen`](#mediumspringgreen)
  - [`MediumTurquoise`](#mediumturquoise)
  - [`MediumVioletRed`](#mediumvioletred)
  - [`MidnightBlue`](#midnightblue)
  - [`MintCream`](#mintcream)
  - [`MistyRose`](#mistyrose)
  - [`Moccasin`](#moccasin)
  - [`NavajoWhite`](#navajowhite)
  - [`Navy`](#navy)
  - [`OldLace`](#oldlace)
  - [`Olive`](#olive)
  - [`OliveDrab`](#olivedrab)
  - [`Orange`](#orange)
  - [`OrangeRed`](#orangered)
  - [`Orchid`](#orchid)
  - [`PaleGoldenRod`](#palegoldenrod)
  - [`PaleGreen`](#palegreen)
  - [`PaleTurquoise`](#paleturquoise)
  - [`PaleVioletRed`](#palevioletred)
  - [`PapayaWhip`](#papayawhip)
  - [`PeachPuff`](#peachpuff)
  - [`Peru`](#peru)
  - [`Pink`](#pink)
  - [`Plum`](#plum)
  - [`PowderBlue`](#powderblue)
  - [`Purple`](#purple)
  - [`RebeccaPurple`](#rebeccapurple)
  - [`Red`](#red)
  - [`RosyBrown`](#rosybrown)
  - [`RoyalBlue`](#royalblue)
  - [`SaddleBrown`](#saddlebrown)
  - [`Salmon`](#salmon)
  - [`SandyBrown`](#sandybrown)
  - [`SeaGreen`](#seagreen)
  - [`SeaShell`](#seashell)
  - [`Sienna`](#sienna)
  - [`Silver`](#silver)
  - [`SkyBlue`](#skyblue)
  - [`SlateBlue`](#slateblue)
  - [`SlateGray`](#slategray)
  - [`SlateGrey`](#slategrey)
  - [`Snow`](#snow)
  - [`SpringGreen`](#springgreen)
  - [`SteelBlue`](#steelblue)
  - [`Tan`](#tan)
  - [`Teal`](#teal)
  - [`Thistle`](#thistle)
  - [`Tomato`](#tomato)
  - [`Turquoise`](#turquoise)
  - [`Violet`](#violet)
  - [`Wheat`](#wheat)
  - [`White`](#white)
  - [`WhiteSmoke`](#whitesmoke)
  - [`Yellow`](#yellow)
  - [`YellowGreen`](#yellowgreen)
- [Macros](#macros)
  - [`css_color_types!`](#css_color_types)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dynamic`](#dynamic) | mod |  |
| [`AliceBlue`](#aliceblue) | type |  |
| [`AntiqueWhite`](#antiquewhite) | type |  |
| [`Aqua`](#aqua) | type |  |
| [`Aquamarine`](#aquamarine) | type |  |
| [`Azure`](#azure) | type |  |
| [`Beige`](#beige) | type |  |
| [`Bisque`](#bisque) | type |  |
| [`Black`](#black) | type |  |
| [`BlanchedAlmond`](#blanchedalmond) | type |  |
| [`Blue`](#blue) | type |  |
| [`BlueViolet`](#blueviolet) | type |  |
| [`Brown`](#brown) | type |  |
| [`BurlyWood`](#burlywood) | type |  |
| [`CadetBlue`](#cadetblue) | type |  |
| [`Chartreuse`](#chartreuse) | type |  |
| [`Chocolate`](#chocolate) | type |  |
| [`Coral`](#coral) | type |  |
| [`CornflowerBlue`](#cornflowerblue) | type |  |
| [`Cornsilk`](#cornsilk) | type |  |
| [`Crimson`](#crimson) | type |  |
| [`DarkBlue`](#darkblue) | type |  |
| [`DarkCyan`](#darkcyan) | type |  |
| [`DarkGoldenRod`](#darkgoldenrod) | type |  |
| [`DarkGray`](#darkgray) | type |  |
| [`DarkGrey`](#darkgrey) | type |  |
| [`DarkGreen`](#darkgreen) | type |  |
| [`DarkKhaki`](#darkkhaki) | type |  |
| [`DarkMagenta`](#darkmagenta) | type |  |
| [`DarkOliveGreen`](#darkolivegreen) | type |  |
| [`DarkOrange`](#darkorange) | type |  |
| [`DarkOrchid`](#darkorchid) | type |  |
| [`DarkRed`](#darkred) | type |  |
| [`DarkSalmon`](#darksalmon) | type |  |
| [`DarkSeaGreen`](#darkseagreen) | type |  |
| [`DarkSlateBlue`](#darkslateblue) | type |  |
| [`DarkSlateGray`](#darkslategray) | type |  |
| [`DarkSlateGrey`](#darkslategrey) | type |  |
| [`DarkTurquoise`](#darkturquoise) | type |  |
| [`DarkViolet`](#darkviolet) | type |  |
| [`DeepPink`](#deeppink) | type |  |
| [`DeepSkyBlue`](#deepskyblue) | type |  |
| [`DimGray`](#dimgray) | type |  |
| [`DimGrey`](#dimgrey) | type |  |
| [`DodgerBlue`](#dodgerblue) | type |  |
| [`FireBrick`](#firebrick) | type |  |
| [`FloralWhite`](#floralwhite) | type |  |
| [`ForestGreen`](#forestgreen) | type |  |
| [`Fuchsia`](#fuchsia) | type |  |
| [`Gainsboro`](#gainsboro) | type |  |
| [`GhostWhite`](#ghostwhite) | type |  |
| [`Gold`](#gold) | type |  |
| [`GoldenRod`](#goldenrod) | type |  |
| [`Gray`](#gray) | type |  |
| [`Grey`](#grey) | type |  |
| [`Green`](#green) | type |  |
| [`GreenYellow`](#greenyellow) | type |  |
| [`HoneyDew`](#honeydew) | type |  |
| [`HotPink`](#hotpink) | type |  |
| [`IndianRed`](#indianred) | type |  |
| [`Indigo`](#indigo) | type |  |
| [`Ivory`](#ivory) | type |  |
| [`Khaki`](#khaki) | type |  |
| [`Lavender`](#lavender) | type |  |
| [`LavenderBlush`](#lavenderblush) | type |  |
| [`LawnGreen`](#lawngreen) | type |  |
| [`LemonChiffon`](#lemonchiffon) | type |  |
| [`LightBlue`](#lightblue) | type |  |
| [`LightCoral`](#lightcoral) | type |  |
| [`LightCyan`](#lightcyan) | type |  |
| [`LightGoldenRodYellow`](#lightgoldenrodyellow) | type |  |
| [`LightGray`](#lightgray) | type |  |
| [`LightGrey`](#lightgrey) | type |  |
| [`LightGreen`](#lightgreen) | type |  |
| [`LightPink`](#lightpink) | type |  |
| [`LightSalmon`](#lightsalmon) | type |  |
| [`LightSeaGreen`](#lightseagreen) | type |  |
| [`LightSkyBlue`](#lightskyblue) | type |  |
| [`LightSlateGray`](#lightslategray) | type |  |
| [`LightSlateGrey`](#lightslategrey) | type |  |
| [`LightSteelBlue`](#lightsteelblue) | type |  |
| [`LightYellow`](#lightyellow) | type |  |
| [`Lime`](#lime) | type |  |
| [`LimeGreen`](#limegreen) | type |  |
| [`Linen`](#linen) | type |  |
| [`Magenta`](#magenta) | type |  |
| [`Maroon`](#maroon) | type |  |
| [`MediumAquaMarine`](#mediumaquamarine) | type |  |
| [`MediumBlue`](#mediumblue) | type |  |
| [`MediumOrchid`](#mediumorchid) | type |  |
| [`MediumPurple`](#mediumpurple) | type |  |
| [`MediumSeaGreen`](#mediumseagreen) | type |  |
| [`MediumSlateBlue`](#mediumslateblue) | type |  |
| [`MediumSpringGreen`](#mediumspringgreen) | type |  |
| [`MediumTurquoise`](#mediumturquoise) | type |  |
| [`MediumVioletRed`](#mediumvioletred) | type |  |
| [`MidnightBlue`](#midnightblue) | type |  |
| [`MintCream`](#mintcream) | type |  |
| [`MistyRose`](#mistyrose) | type |  |
| [`Moccasin`](#moccasin) | type |  |
| [`NavajoWhite`](#navajowhite) | type |  |
| [`Navy`](#navy) | type |  |
| [`OldLace`](#oldlace) | type |  |
| [`Olive`](#olive) | type |  |
| [`OliveDrab`](#olivedrab) | type |  |
| [`Orange`](#orange) | type |  |
| [`OrangeRed`](#orangered) | type |  |
| [`Orchid`](#orchid) | type |  |
| [`PaleGoldenRod`](#palegoldenrod) | type |  |
| [`PaleGreen`](#palegreen) | type |  |
| [`PaleTurquoise`](#paleturquoise) | type |  |
| [`PaleVioletRed`](#palevioletred) | type |  |
| [`PapayaWhip`](#papayawhip) | type |  |
| [`PeachPuff`](#peachpuff) | type |  |
| [`Peru`](#peru) | type |  |
| [`Pink`](#pink) | type |  |
| [`Plum`](#plum) | type |  |
| [`PowderBlue`](#powderblue) | type |  |
| [`Purple`](#purple) | type |  |
| [`RebeccaPurple`](#rebeccapurple) | type |  |
| [`Red`](#red) | type |  |
| [`RosyBrown`](#rosybrown) | type |  |
| [`RoyalBlue`](#royalblue) | type |  |
| [`SaddleBrown`](#saddlebrown) | type |  |
| [`Salmon`](#salmon) | type |  |
| [`SandyBrown`](#sandybrown) | type |  |
| [`SeaGreen`](#seagreen) | type |  |
| [`SeaShell`](#seashell) | type |  |
| [`Sienna`](#sienna) | type |  |
| [`Silver`](#silver) | type |  |
| [`SkyBlue`](#skyblue) | type |  |
| [`SlateBlue`](#slateblue) | type |  |
| [`SlateGray`](#slategray) | type |  |
| [`SlateGrey`](#slategrey) | type |  |
| [`Snow`](#snow) | type |  |
| [`SpringGreen`](#springgreen) | type |  |
| [`SteelBlue`](#steelblue) | type |  |
| [`Tan`](#tan) | type |  |
| [`Teal`](#teal) | type |  |
| [`Thistle`](#thistle) | type |  |
| [`Tomato`](#tomato) | type |  |
| [`Turquoise`](#turquoise) | type |  |
| [`Violet`](#violet) | type |  |
| [`Wheat`](#wheat) | type |  |
| [`White`](#white) | type |  |
| [`WhiteSmoke`](#whitesmoke) | type |  |
| [`Yellow`](#yellow) | type |  |
| [`YellowGreen`](#yellowgreen) | type |  |
| [`css_color_types!`](#css_color_types) | macro |  |

## Modules

- [`dynamic`](dynamic/index.md) - 

## Type Aliases

### `AliceBlue`

```rust
type AliceBlue = CustomColor<240, 248, 255>;
```

### `AntiqueWhite`

```rust
type AntiqueWhite = CustomColor<250, 235, 215>;
```

### `Aqua`

```rust
type Aqua = CustomColor<0, 255, 255>;
```

### `Aquamarine`

```rust
type Aquamarine = CustomColor<127, 255, 212>;
```

### `Azure`

```rust
type Azure = CustomColor<240, 255, 255>;
```

### `Beige`

```rust
type Beige = CustomColor<245, 245, 220>;
```

### `Bisque`

```rust
type Bisque = CustomColor<255, 228, 196>;
```

### `Black`

```rust
type Black = CustomColor<0, 0, 0>;
```

### `BlanchedAlmond`

```rust
type BlanchedAlmond = CustomColor<255, 235, 205>;
```

### `Blue`

```rust
type Blue = CustomColor<0, 0, 255>;
```

### `BlueViolet`

```rust
type BlueViolet = CustomColor<138, 43, 226>;
```

### `Brown`

```rust
type Brown = CustomColor<165, 42, 42>;
```

### `BurlyWood`

```rust
type BurlyWood = CustomColor<222, 184, 135>;
```

### `CadetBlue`

```rust
type CadetBlue = CustomColor<95, 158, 160>;
```

### `Chartreuse`

```rust
type Chartreuse = CustomColor<127, 255, 0>;
```

### `Chocolate`

```rust
type Chocolate = CustomColor<210, 105, 30>;
```

### `Coral`

```rust
type Coral = CustomColor<255, 127, 80>;
```

### `CornflowerBlue`

```rust
type CornflowerBlue = CustomColor<100, 149, 237>;
```

### `Cornsilk`

```rust
type Cornsilk = CustomColor<255, 248, 220>;
```

### `Crimson`

```rust
type Crimson = CustomColor<220, 20, 60>;
```

### `DarkBlue`

```rust
type DarkBlue = CustomColor<0, 0, 139>;
```

### `DarkCyan`

```rust
type DarkCyan = CustomColor<0, 139, 139>;
```

### `DarkGoldenRod`

```rust
type DarkGoldenRod = CustomColor<184, 134, 11>;
```

### `DarkGray`

```rust
type DarkGray = CustomColor<169, 169, 169>;
```

### `DarkGrey`

```rust
type DarkGrey = CustomColor<169, 169, 169>;
```

### `DarkGreen`

```rust
type DarkGreen = CustomColor<0, 100, 0>;
```

### `DarkKhaki`

```rust
type DarkKhaki = CustomColor<189, 183, 107>;
```

### `DarkMagenta`

```rust
type DarkMagenta = CustomColor<139, 0, 139>;
```

### `DarkOliveGreen`

```rust
type DarkOliveGreen = CustomColor<85, 107, 47>;
```

### `DarkOrange`

```rust
type DarkOrange = CustomColor<255, 140, 0>;
```

### `DarkOrchid`

```rust
type DarkOrchid = CustomColor<153, 50, 204>;
```

### `DarkRed`

```rust
type DarkRed = CustomColor<139, 0, 0>;
```

### `DarkSalmon`

```rust
type DarkSalmon = CustomColor<233, 150, 122>;
```

### `DarkSeaGreen`

```rust
type DarkSeaGreen = CustomColor<143, 188, 143>;
```

### `DarkSlateBlue`

```rust
type DarkSlateBlue = CustomColor<72, 61, 139>;
```

### `DarkSlateGray`

```rust
type DarkSlateGray = CustomColor<47, 79, 79>;
```

### `DarkSlateGrey`

```rust
type DarkSlateGrey = CustomColor<47, 79, 79>;
```

### `DarkTurquoise`

```rust
type DarkTurquoise = CustomColor<0, 206, 209>;
```

### `DarkViolet`

```rust
type DarkViolet = CustomColor<148, 0, 211>;
```

### `DeepPink`

```rust
type DeepPink = CustomColor<255, 20, 147>;
```

### `DeepSkyBlue`

```rust
type DeepSkyBlue = CustomColor<0, 191, 255>;
```

### `DimGray`

```rust
type DimGray = CustomColor<105, 105, 105>;
```

### `DimGrey`

```rust
type DimGrey = CustomColor<105, 105, 105>;
```

### `DodgerBlue`

```rust
type DodgerBlue = CustomColor<30, 144, 255>;
```

### `FireBrick`

```rust
type FireBrick = CustomColor<178, 34, 34>;
```

### `FloralWhite`

```rust
type FloralWhite = CustomColor<255, 250, 240>;
```

### `ForestGreen`

```rust
type ForestGreen = CustomColor<34, 139, 34>;
```

### `Fuchsia`

```rust
type Fuchsia = CustomColor<255, 0, 255>;
```

### `Gainsboro`

```rust
type Gainsboro = CustomColor<220, 220, 220>;
```

### `GhostWhite`

```rust
type GhostWhite = CustomColor<248, 248, 255>;
```

### `Gold`

```rust
type Gold = CustomColor<255, 215, 0>;
```

### `GoldenRod`

```rust
type GoldenRod = CustomColor<218, 165, 32>;
```

### `Gray`

```rust
type Gray = CustomColor<128, 128, 128>;
```

### `Grey`

```rust
type Grey = CustomColor<128, 128, 128>;
```

### `Green`

```rust
type Green = CustomColor<0, 128, 0>;
```

### `GreenYellow`

```rust
type GreenYellow = CustomColor<173, 255, 47>;
```

### `HoneyDew`

```rust
type HoneyDew = CustomColor<240, 255, 240>;
```

### `HotPink`

```rust
type HotPink = CustomColor<255, 105, 180>;
```

### `IndianRed`

```rust
type IndianRed = CustomColor<205, 92, 92>;
```

### `Indigo`

```rust
type Indigo = CustomColor<75, 0, 130>;
```

### `Ivory`

```rust
type Ivory = CustomColor<255, 255, 240>;
```

### `Khaki`

```rust
type Khaki = CustomColor<240, 230, 140>;
```

### `Lavender`

```rust
type Lavender = CustomColor<230, 230, 250>;
```

### `LavenderBlush`

```rust
type LavenderBlush = CustomColor<255, 240, 245>;
```

### `LawnGreen`

```rust
type LawnGreen = CustomColor<124, 252, 0>;
```

### `LemonChiffon`

```rust
type LemonChiffon = CustomColor<255, 250, 205>;
```

### `LightBlue`

```rust
type LightBlue = CustomColor<173, 216, 230>;
```

### `LightCoral`

```rust
type LightCoral = CustomColor<240, 128, 128>;
```

### `LightCyan`

```rust
type LightCyan = CustomColor<224, 255, 255>;
```

### `LightGoldenRodYellow`

```rust
type LightGoldenRodYellow = CustomColor<250, 250, 210>;
```

### `LightGray`

```rust
type LightGray = CustomColor<211, 211, 211>;
```

### `LightGrey`

```rust
type LightGrey = CustomColor<211, 211, 211>;
```

### `LightGreen`

```rust
type LightGreen = CustomColor<144, 238, 144>;
```

### `LightPink`

```rust
type LightPink = CustomColor<255, 182, 193>;
```

### `LightSalmon`

```rust
type LightSalmon = CustomColor<255, 160, 122>;
```

### `LightSeaGreen`

```rust
type LightSeaGreen = CustomColor<32, 178, 170>;
```

### `LightSkyBlue`

```rust
type LightSkyBlue = CustomColor<135, 206, 250>;
```

### `LightSlateGray`

```rust
type LightSlateGray = CustomColor<119, 136, 153>;
```

### `LightSlateGrey`

```rust
type LightSlateGrey = CustomColor<119, 136, 153>;
```

### `LightSteelBlue`

```rust
type LightSteelBlue = CustomColor<176, 196, 222>;
```

### `LightYellow`

```rust
type LightYellow = CustomColor<255, 255, 224>;
```

### `Lime`

```rust
type Lime = CustomColor<0, 255, 0>;
```

### `LimeGreen`

```rust
type LimeGreen = CustomColor<50, 205, 50>;
```

### `Linen`

```rust
type Linen = CustomColor<250, 240, 230>;
```

### `Magenta`

```rust
type Magenta = CustomColor<255, 0, 255>;
```

### `Maroon`

```rust
type Maroon = CustomColor<128, 0, 0>;
```

### `MediumAquaMarine`

```rust
type MediumAquaMarine = CustomColor<102, 205, 170>;
```

### `MediumBlue`

```rust
type MediumBlue = CustomColor<0, 0, 205>;
```

### `MediumOrchid`

```rust
type MediumOrchid = CustomColor<186, 85, 211>;
```

### `MediumPurple`

```rust
type MediumPurple = CustomColor<147, 112, 219>;
```

### `MediumSeaGreen`

```rust
type MediumSeaGreen = CustomColor<60, 179, 113>;
```

### `MediumSlateBlue`

```rust
type MediumSlateBlue = CustomColor<123, 104, 238>;
```

### `MediumSpringGreen`

```rust
type MediumSpringGreen = CustomColor<0, 250, 154>;
```

### `MediumTurquoise`

```rust
type MediumTurquoise = CustomColor<72, 209, 204>;
```

### `MediumVioletRed`

```rust
type MediumVioletRed = CustomColor<199, 21, 133>;
```

### `MidnightBlue`

```rust
type MidnightBlue = CustomColor<25, 25, 112>;
```

### `MintCream`

```rust
type MintCream = CustomColor<245, 255, 250>;
```

### `MistyRose`

```rust
type MistyRose = CustomColor<255, 228, 225>;
```

### `Moccasin`

```rust
type Moccasin = CustomColor<255, 228, 181>;
```

### `NavajoWhite`

```rust
type NavajoWhite = CustomColor<255, 222, 173>;
```

### `Navy`

```rust
type Navy = CustomColor<0, 0, 128>;
```

### `OldLace`

```rust
type OldLace = CustomColor<253, 245, 230>;
```

### `Olive`

```rust
type Olive = CustomColor<128, 128, 0>;
```

### `OliveDrab`

```rust
type OliveDrab = CustomColor<107, 142, 35>;
```

### `Orange`

```rust
type Orange = CustomColor<255, 165, 0>;
```

### `OrangeRed`

```rust
type OrangeRed = CustomColor<255, 69, 0>;
```

### `Orchid`

```rust
type Orchid = CustomColor<218, 112, 214>;
```

### `PaleGoldenRod`

```rust
type PaleGoldenRod = CustomColor<238, 232, 170>;
```

### `PaleGreen`

```rust
type PaleGreen = CustomColor<152, 251, 152>;
```

### `PaleTurquoise`

```rust
type PaleTurquoise = CustomColor<175, 238, 238>;
```

### `PaleVioletRed`

```rust
type PaleVioletRed = CustomColor<219, 112, 147>;
```

### `PapayaWhip`

```rust
type PapayaWhip = CustomColor<255, 239, 213>;
```

### `PeachPuff`

```rust
type PeachPuff = CustomColor<255, 218, 185>;
```

### `Peru`

```rust
type Peru = CustomColor<205, 133, 63>;
```

### `Pink`

```rust
type Pink = CustomColor<255, 192, 203>;
```

### `Plum`

```rust
type Plum = CustomColor<221, 160, 221>;
```

### `PowderBlue`

```rust
type PowderBlue = CustomColor<176, 224, 230>;
```

### `Purple`

```rust
type Purple = CustomColor<128, 0, 128>;
```

### `RebeccaPurple`

```rust
type RebeccaPurple = CustomColor<102, 51, 153>;
```

### `Red`

```rust
type Red = CustomColor<255, 0, 0>;
```

### `RosyBrown`

```rust
type RosyBrown = CustomColor<188, 143, 143>;
```

### `RoyalBlue`

```rust
type RoyalBlue = CustomColor<65, 105, 225>;
```

### `SaddleBrown`

```rust
type SaddleBrown = CustomColor<139, 69, 19>;
```

### `Salmon`

```rust
type Salmon = CustomColor<250, 128, 114>;
```

### `SandyBrown`

```rust
type SandyBrown = CustomColor<244, 164, 96>;
```

### `SeaGreen`

```rust
type SeaGreen = CustomColor<46, 139, 87>;
```

### `SeaShell`

```rust
type SeaShell = CustomColor<255, 245, 238>;
```

### `Sienna`

```rust
type Sienna = CustomColor<160, 82, 45>;
```

### `Silver`

```rust
type Silver = CustomColor<192, 192, 192>;
```

### `SkyBlue`

```rust
type SkyBlue = CustomColor<135, 206, 235>;
```

### `SlateBlue`

```rust
type SlateBlue = CustomColor<106, 90, 205>;
```

### `SlateGray`

```rust
type SlateGray = CustomColor<112, 128, 144>;
```

### `SlateGrey`

```rust
type SlateGrey = CustomColor<112, 128, 144>;
```

### `Snow`

```rust
type Snow = CustomColor<255, 250, 250>;
```

### `SpringGreen`

```rust
type SpringGreen = CustomColor<0, 255, 127>;
```

### `SteelBlue`

```rust
type SteelBlue = CustomColor<70, 130, 180>;
```

### `Tan`

```rust
type Tan = CustomColor<210, 180, 140>;
```

### `Teal`

```rust
type Teal = CustomColor<0, 128, 128>;
```

### `Thistle`

```rust
type Thistle = CustomColor<216, 191, 216>;
```

### `Tomato`

```rust
type Tomato = CustomColor<255, 99, 71>;
```

### `Turquoise`

```rust
type Turquoise = CustomColor<64, 224, 208>;
```

### `Violet`

```rust
type Violet = CustomColor<238, 130, 238>;
```

### `Wheat`

```rust
type Wheat = CustomColor<245, 222, 179>;
```

### `White`

```rust
type White = CustomColor<255, 255, 255>;
```

### `WhiteSmoke`

```rust
type WhiteSmoke = CustomColor<245, 245, 245>;
```

### `Yellow`

```rust
type Yellow = CustomColor<255, 255, 0>;
```

### `YellowGreen`

```rust
type YellowGreen = CustomColor<154, 205, 50>;
```

## Macros

### `css_color_types!`

