*[owo_colors](../../../index.md) / [colors](../../index.md) / [xterm](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`XtermColors`](#xtermcolors) | enum | Available Xterm colors for use with [`OwoColorize::color`](OwoColorize::color) or [`OwoColorize::on_color`](OwoColorize::on_color) |

## Enums

### `XtermColors`

```rust
enum XtermColors {
    UserBlack,
    UserRed,
    UserGreen,
    UserYellow,
    UserBlue,
    UserMagenta,
    UserCyan,
    UserWhite,
    UserBrightBlack,
    UserBrightRed,
    UserBrightGreen,
    UserBrightYellow,
    UserBrightBlue,
    UserBrightMagenta,
    UserBrightCyan,
    UserBrightWhite,
    Black,
    StratosBlue,
    NavyBlue,
    MidnightBlue,
    DarkBlue,
    Blue,
    CamaroneGreen,
    BlueStone,
    OrientBlue,
    EndeavourBlue,
    ScienceBlue,
    BlueRibbon,
    JapaneseLaurel,
    DeepSeaGreen,
    Teal,
    DeepCerulean,
    LochmaraBlue,
    AzureRadiance,
    LightJapaneseLaurel,
    Jade,
    PersianGreen,
    BondiBlue,
    Cerulean,
    LightAzureRadiance,
    DarkGreen,
    Malachite,
    CaribbeanGreen,
    LightCaribbeanGreen,
    RobinEggBlue,
    Aqua,
    Green,
    DarkSpringGreen,
    SpringGreen,
    LightSpringGreen,
    BrightTurquoise,
    Cyan,
    Rosewood,
    PompadourMagenta,
    PigmentIndigo,
    DarkPurple,
    ElectricIndigo,
    ElectricPurple,
    VerdunGreen,
    ScorpionOlive,
    Lilac,
    ScampiIndigo,
    Indigo,
    DarkCornflowerBlue,
    DarkLimeade,
    GladeGreen,
    JuniperGreen,
    HippieBlue,
    HavelockBlue,
    CornflowerBlue,
    Limeade,
    FernGreen,
    SilverTree,
    Tradewind,
    ShakespeareBlue,
    DarkMalibuBlue,
    DarkBrightGreen,
    DarkPastelGreen,
    PastelGreen,
    DownyTeal,
    Viking,
    MalibuBlue,
    BrightGreen,
    DarkScreaminGreen,
    ScreaminGreen,
    DarkAquamarine,
    Aquamarine,
    LightAquamarine,
    Maroon,
    DarkFreshEggplant,
    LightFreshEggplant,
    Purple,
    ElectricViolet,
    LightElectricViolet,
    Brown,
    CopperRose,
    StrikemasterPurple,
    DelugePurple,
    DarkMediumPurple,
    DarkHeliotropePurple,
    Olive,
    ClayCreekOlive,
    DarkGray,
    WildBlueYonder,
    ChetwodeBlue,
    SlateBlue,
    LightLimeade,
    ChelseaCucumber,
    BayLeaf,
    GulfStream,
    PoloBlue,
    LightMalibuBlue,
    Pistachio,
    LightPastelGreen,
    DarkFeijoaGreen,
    VistaBlue,
    Bermuda,
    DarkAnakiwaBlue,
    ChartreuseGreen,
    LightScreaminGreen,
    DarkMintGreen,
    MintGreen,
    LighterAquamarine,
    AnakiwaBlue,
    BrightRed,
    DarkFlirt,
    Flirt,
    LightFlirt,
    DarkViolet,
    BrightElectricViolet,
    RoseofSharonOrange,
    MatrixPink,
    TapestryPink,
    FuchsiaPink,
    MediumPurple,
    Heliotrope,
    PirateGold,
    MuesliOrange,
    PharlapPink,
    Bouquet,
    Lavender,
    LightHeliotrope,
    BuddhaGold,
    OliveGreen,
    HillaryOlive,
    SilverChalice,
    WistfulLilac,
    MelroseLilac,
    RioGrandeGreen,
    ConiferGreen,
    Feijoa,
    PixieGreen,
    JungleMist,
    LightAnakiwaBlue,
    Lime,
    GreenYellow,
    LightMintGreen,
    Celadon,
    AeroBlue,
    FrenchPassLightBlue,
    GuardsmanRed,
    RazzmatazzCerise,
    MediumVioletRed,
    HollywoodCerise,
    DarkPurplePizzazz,
    BrighterElectricViolet,
    TennOrange,
    RomanOrange,
    CranberryPink,
    HopbushPink,
    Orchid,
    LighterHeliotrope,
    MangoTango,
    Copperfield,
    SeaPink,
    CanCanPink,
    LightOrchid,
    BrightHeliotrope,
    DarkCorn,
    DarkTachaOrange,
    TanBeige,
    ClamShell,
    ThistlePink,
    Mauve,
    Corn,
    TachaOrange,
    DecoOrange,
    PaleGoldenrod,
    AltoBeige,
    FogPink,
    ChartreuseYellow,
    Canary,
    Honeysuckle,
    ReefPaleYellow,
    SnowyMint,
    OysterBay,
    Red,
    DarkRose,
    Rose,
    LightHollywoodCerise,
    PurplePizzazz,
    Fuchsia,
    BlazeOrange,
    BittersweetOrange,
    WildWatermelon,
    DarkHotPink,
    HotPink,
    PinkFlamingo,
    FlushOrange,
    Salmon,
    VividTangerine,
    PinkSalmon,
    DarkLavenderRose,
    BlushPink,
    YellowSea,
    TexasRose,
    Tacao,
    Sundown,
    CottonCandy,
    LavenderRose,
    Gold,
    Dandelion,
    GrandisCaramel,
    Caramel,
    CosmosSalmon,
    PinkLace,
    Yellow,
    LaserLemon,
    DollyYellow,
    PortafinoYellow,
    Cumulus,
    White,
    DarkCodGray,
    CodGray,
    LightCodGray,
    DarkMineShaft,
    MineShaft,
    LightMineShaft,
    DarkTundora,
    Tundora,
    ScorpionGray,
    DarkDoveGray,
    DoveGray,
    Boulder,
    Gray,
    LightGray,
    DustyGray,
    NobelGray,
    DarkSilverChalice,
    LightSilverChalice,
    DarkSilver,
    Silver,
    DarkAlto,
    Alto,
    Mercury,
    GalleryGray,
}
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../../.source_1765633015/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

Available Xterm colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for XtermColors`

- <span id="xtermcolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XtermColors`

- <span id="xtermcolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XtermColors`

- <span id="xtermcolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for XtermColors`

- <span id="xtermcolors-clone"></span>`fn clone(&self) -> XtermColors` — [`XtermColors`](#xtermcolors)

##### `impl CloneToUninit for XtermColors`

- <span id="xtermcolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for XtermColors`

##### `impl Debug for XtermColors`

- <span id="xtermcolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for XtermColors`

- <span id="xtermcolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for XtermColors`

##### `impl<T> From for XtermColors`

- <span id="xtermcolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XtermColors`

- <span id="xtermcolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for XtermColors`

##### `impl PartialEq for XtermColors`

- <span id="xtermcolors-partialeq-eq"></span>`fn eq(&self, other: &XtermColors) -> bool` — [`XtermColors`](#xtermcolors)

##### `impl StructuralPartialEq for XtermColors`

##### `impl<U> TryFrom for XtermColors`

- <span id="xtermcolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xtermcolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XtermColors`

- <span id="xtermcolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xtermcolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

