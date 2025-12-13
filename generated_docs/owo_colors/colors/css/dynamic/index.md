*[owo_colors](../../../index.md) / [colors](../../index.md) / [css](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CssColors`](#csscolors) | enum | Available CSS colors for use with [`OwoColorize::color`](OwoColorize::color) or [`OwoColorize::on_color`](OwoColorize::on_color) |

## Enums

### `CssColors`

```rust
enum CssColors {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGrey,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    Gray,
    Grey,
    Green,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGrey,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}
```

*Defined in [`owo-colors-4.2.3/src/colors/css.rs:84-232`](../../../../../.source_1765633015/owo-colors-4.2.3/src/colors/css.rs#L84-L232)*

Available CSS colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for CssColors`

- <span id="csscolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CssColors`

- <span id="csscolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CssColors`

- <span id="csscolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CssColors`

- <span id="csscolors-clone"></span>`fn clone(&self) -> CssColors` — [`CssColors`](#csscolors)

##### `impl CloneToUninit for CssColors`

- <span id="csscolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CssColors`

##### `impl Debug for CssColors`

- <span id="csscolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for CssColors`

- <span id="csscolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CssColors`

##### `impl<T> From for CssColors`

- <span id="csscolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CssColors`

- <span id="csscolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CssColors`

##### `impl PartialEq for CssColors`

- <span id="csscolors-partialeq-eq"></span>`fn eq(&self, other: &CssColors) -> bool` — [`CssColors`](#csscolors)

##### `impl StructuralPartialEq for CssColors`

##### `impl<U> TryFrom for CssColors`

- <span id="csscolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="csscolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CssColors`

- <span id="csscolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="csscolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

