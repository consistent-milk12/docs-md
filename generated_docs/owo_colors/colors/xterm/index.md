*[owo_colors](../../index.md) / [colors](../index.md) / [xterm](index.md)*

---

# Module `xterm`

XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.

## Contents

- [Modules](#modules)
  - [`dynamic`](#dynamic)
- [Structs](#structs)
  - [`UserBlack`](#userblack)
  - [`UserRed`](#userred)
  - [`UserGreen`](#usergreen)
  - [`UserYellow`](#useryellow)
  - [`UserBlue`](#userblue)
  - [`UserMagenta`](#usermagenta)
  - [`UserCyan`](#usercyan)
  - [`UserWhite`](#userwhite)
  - [`UserBrightBlack`](#userbrightblack)
  - [`UserBrightRed`](#userbrightred)
  - [`UserBrightGreen`](#userbrightgreen)
  - [`UserBrightYellow`](#userbrightyellow)
  - [`UserBrightBlue`](#userbrightblue)
  - [`UserBrightMagenta`](#userbrightmagenta)
  - [`UserBrightCyan`](#userbrightcyan)
  - [`UserBrightWhite`](#userbrightwhite)
  - [`Black`](#black)
  - [`StratosBlue`](#stratosblue)
  - [`NavyBlue`](#navyblue)
  - [`MidnightBlue`](#midnightblue)
  - [`DarkBlue`](#darkblue)
  - [`Blue`](#blue)
  - [`CamaroneGreen`](#camaronegreen)
  - [`BlueStone`](#bluestone)
  - [`OrientBlue`](#orientblue)
  - [`EndeavourBlue`](#endeavourblue)
  - [`ScienceBlue`](#scienceblue)
  - [`BlueRibbon`](#blueribbon)
  - [`JapaneseLaurel`](#japaneselaurel)
  - [`DeepSeaGreen`](#deepseagreen)
  - [`Teal`](#teal)
  - [`DeepCerulean`](#deepcerulean)
  - [`LochmaraBlue`](#lochmarablue)
  - [`AzureRadiance`](#azureradiance)
  - [`LightJapaneseLaurel`](#lightjapaneselaurel)
  - [`Jade`](#jade)
  - [`PersianGreen`](#persiangreen)
  - [`BondiBlue`](#bondiblue)
  - [`Cerulean`](#cerulean)
  - [`LightAzureRadiance`](#lightazureradiance)
  - [`DarkGreen`](#darkgreen)
  - [`Malachite`](#malachite)
  - [`CaribbeanGreen`](#caribbeangreen)
  - [`LightCaribbeanGreen`](#lightcaribbeangreen)
  - [`RobinEggBlue`](#robineggblue)
  - [`Aqua`](#aqua)
  - [`Green`](#green)
  - [`DarkSpringGreen`](#darkspringgreen)
  - [`SpringGreen`](#springgreen)
  - [`LightSpringGreen`](#lightspringgreen)
  - [`BrightTurquoise`](#brightturquoise)
  - [`Cyan`](#cyan)
  - [`Rosewood`](#rosewood)
  - [`PompadourMagenta`](#pompadourmagenta)
  - [`PigmentIndigo`](#pigmentindigo)
  - [`DarkPurple`](#darkpurple)
  - [`ElectricIndigo`](#electricindigo)
  - [`ElectricPurple`](#electricpurple)
  - [`VerdunGreen`](#verdungreen)
  - [`ScorpionOlive`](#scorpionolive)
  - [`Lilac`](#lilac)
  - [`ScampiIndigo`](#scampiindigo)
  - [`Indigo`](#indigo)
  - [`DarkCornflowerBlue`](#darkcornflowerblue)
  - [`DarkLimeade`](#darklimeade)
  - [`GladeGreen`](#gladegreen)
  - [`JuniperGreen`](#junipergreen)
  - [`HippieBlue`](#hippieblue)
  - [`HavelockBlue`](#havelockblue)
  - [`CornflowerBlue`](#cornflowerblue)
  - [`Limeade`](#limeade)
  - [`FernGreen`](#ferngreen)
  - [`SilverTree`](#silvertree)
  - [`Tradewind`](#tradewind)
  - [`ShakespeareBlue`](#shakespeareblue)
  - [`DarkMalibuBlue`](#darkmalibublue)
  - [`DarkBrightGreen`](#darkbrightgreen)
  - [`DarkPastelGreen`](#darkpastelgreen)
  - [`PastelGreen`](#pastelgreen)
  - [`DownyTeal`](#downyteal)
  - [`Viking`](#viking)
  - [`MalibuBlue`](#malibublue)
  - [`BrightGreen`](#brightgreen)
  - [`DarkScreaminGreen`](#darkscreamingreen)
  - [`ScreaminGreen`](#screamingreen)
  - [`DarkAquamarine`](#darkaquamarine)
  - [`Aquamarine`](#aquamarine)
  - [`LightAquamarine`](#lightaquamarine)
  - [`Maroon`](#maroon)
  - [`DarkFreshEggplant`](#darkfresheggplant)
  - [`LightFreshEggplant`](#lightfresheggplant)
  - [`Purple`](#purple)
  - [`ElectricViolet`](#electricviolet)
  - [`LightElectricViolet`](#lightelectricviolet)
  - [`Brown`](#brown)
  - [`CopperRose`](#copperrose)
  - [`StrikemasterPurple`](#strikemasterpurple)
  - [`DelugePurple`](#delugepurple)
  - [`DarkMediumPurple`](#darkmediumpurple)
  - [`DarkHeliotropePurple`](#darkheliotropepurple)
  - [`Olive`](#olive)
  - [`ClayCreekOlive`](#claycreekolive)
  - [`DarkGray`](#darkgray)
  - [`WildBlueYonder`](#wildblueyonder)
  - [`ChetwodeBlue`](#chetwodeblue)
  - [`SlateBlue`](#slateblue)
  - [`LightLimeade`](#lightlimeade)
  - [`ChelseaCucumber`](#chelseacucumber)
  - [`BayLeaf`](#bayleaf)
  - [`GulfStream`](#gulfstream)
  - [`PoloBlue`](#poloblue)
  - [`LightMalibuBlue`](#lightmalibublue)
  - [`Pistachio`](#pistachio)
  - [`LightPastelGreen`](#lightpastelgreen)
  - [`DarkFeijoaGreen`](#darkfeijoagreen)
  - [`VistaBlue`](#vistablue)
  - [`Bermuda`](#bermuda)
  - [`DarkAnakiwaBlue`](#darkanakiwablue)
  - [`ChartreuseGreen`](#chartreusegreen)
  - [`LightScreaminGreen`](#lightscreamingreen)
  - [`DarkMintGreen`](#darkmintgreen)
  - [`MintGreen`](#mintgreen)
  - [`LighterAquamarine`](#lighteraquamarine)
  - [`AnakiwaBlue`](#anakiwablue)
  - [`BrightRed`](#brightred)
  - [`DarkFlirt`](#darkflirt)
  - [`Flirt`](#flirt)
  - [`LightFlirt`](#lightflirt)
  - [`DarkViolet`](#darkviolet)
  - [`BrightElectricViolet`](#brightelectricviolet)
  - [`RoseofSharonOrange`](#roseofsharonorange)
  - [`MatrixPink`](#matrixpink)
  - [`TapestryPink`](#tapestrypink)
  - [`FuchsiaPink`](#fuchsiapink)
  - [`MediumPurple`](#mediumpurple)
  - [`Heliotrope`](#heliotrope)
  - [`PirateGold`](#pirategold)
  - [`MuesliOrange`](#muesliorange)
  - [`PharlapPink`](#pharlappink)
  - [`Bouquet`](#bouquet)
  - [`Lavender`](#lavender)
  - [`LightHeliotrope`](#lightheliotrope)
  - [`BuddhaGold`](#buddhagold)
  - [`OliveGreen`](#olivegreen)
  - [`HillaryOlive`](#hillaryolive)
  - [`SilverChalice`](#silverchalice)
  - [`WistfulLilac`](#wistfullilac)
  - [`MelroseLilac`](#melroselilac)
  - [`RioGrandeGreen`](#riograndegreen)
  - [`ConiferGreen`](#conifergreen)
  - [`Feijoa`](#feijoa)
  - [`PixieGreen`](#pixiegreen)
  - [`JungleMist`](#junglemist)
  - [`LightAnakiwaBlue`](#lightanakiwablue)
  - [`Lime`](#lime)
  - [`GreenYellow`](#greenyellow)
  - [`LightMintGreen`](#lightmintgreen)
  - [`Celadon`](#celadon)
  - [`AeroBlue`](#aeroblue)
  - [`FrenchPassLightBlue`](#frenchpasslightblue)
  - [`GuardsmanRed`](#guardsmanred)
  - [`RazzmatazzCerise`](#razzmatazzcerise)
  - [`MediumVioletRed`](#mediumvioletred)
  - [`HollywoodCerise`](#hollywoodcerise)
  - [`DarkPurplePizzazz`](#darkpurplepizzazz)
  - [`BrighterElectricViolet`](#brighterelectricviolet)
  - [`TennOrange`](#tennorange)
  - [`RomanOrange`](#romanorange)
  - [`CranberryPink`](#cranberrypink)
  - [`HopbushPink`](#hopbushpink)
  - [`Orchid`](#orchid)
  - [`LighterHeliotrope`](#lighterheliotrope)
  - [`MangoTango`](#mangotango)
  - [`Copperfield`](#copperfield)
  - [`SeaPink`](#seapink)
  - [`CanCanPink`](#cancanpink)
  - [`LightOrchid`](#lightorchid)
  - [`BrightHeliotrope`](#brightheliotrope)
  - [`DarkCorn`](#darkcorn)
  - [`DarkTachaOrange`](#darktachaorange)
  - [`TanBeige`](#tanbeige)
  - [`ClamShell`](#clamshell)
  - [`ThistlePink`](#thistlepink)
  - [`Mauve`](#mauve)
  - [`Corn`](#corn)
  - [`TachaOrange`](#tachaorange)
  - [`DecoOrange`](#decoorange)
  - [`PaleGoldenrod`](#palegoldenrod)
  - [`AltoBeige`](#altobeige)
  - [`FogPink`](#fogpink)
  - [`ChartreuseYellow`](#chartreuseyellow)
  - [`Canary`](#canary)
  - [`Honeysuckle`](#honeysuckle)
  - [`ReefPaleYellow`](#reefpaleyellow)
  - [`SnowyMint`](#snowymint)
  - [`OysterBay`](#oysterbay)
  - [`Red`](#red)
  - [`DarkRose`](#darkrose)
  - [`Rose`](#rose)
  - [`LightHollywoodCerise`](#lighthollywoodcerise)
  - [`PurplePizzazz`](#purplepizzazz)
  - [`Fuchsia`](#fuchsia)
  - [`BlazeOrange`](#blazeorange)
  - [`BittersweetOrange`](#bittersweetorange)
  - [`WildWatermelon`](#wildwatermelon)
  - [`DarkHotPink`](#darkhotpink)
  - [`HotPink`](#hotpink)
  - [`PinkFlamingo`](#pinkflamingo)
  - [`FlushOrange`](#flushorange)
  - [`Salmon`](#salmon)
  - [`VividTangerine`](#vividtangerine)
  - [`PinkSalmon`](#pinksalmon)
  - [`DarkLavenderRose`](#darklavenderrose)
  - [`BlushPink`](#blushpink)
  - [`YellowSea`](#yellowsea)
  - [`TexasRose`](#texasrose)
  - [`Tacao`](#tacao)
  - [`Sundown`](#sundown)
  - [`CottonCandy`](#cottoncandy)
  - [`LavenderRose`](#lavenderrose)
  - [`Gold`](#gold)
  - [`Dandelion`](#dandelion)
  - [`GrandisCaramel`](#grandiscaramel)
  - [`Caramel`](#caramel)
  - [`CosmosSalmon`](#cosmossalmon)
  - [`PinkLace`](#pinklace)
  - [`Yellow`](#yellow)
  - [`LaserLemon`](#laserlemon)
  - [`DollyYellow`](#dollyyellow)
  - [`PortafinoYellow`](#portafinoyellow)
  - [`Cumulus`](#cumulus)
  - [`White`](#white)
  - [`DarkCodGray`](#darkcodgray)
  - [`CodGray`](#codgray)
  - [`LightCodGray`](#lightcodgray)
  - [`DarkMineShaft`](#darkmineshaft)
  - [`MineShaft`](#mineshaft)
  - [`LightMineShaft`](#lightmineshaft)
  - [`DarkTundora`](#darktundora)
  - [`Tundora`](#tundora)
  - [`ScorpionGray`](#scorpiongray)
  - [`DarkDoveGray`](#darkdovegray)
  - [`DoveGray`](#dovegray)
  - [`Boulder`](#boulder)
  - [`Gray`](#gray)
  - [`LightGray`](#lightgray)
  - [`DustyGray`](#dustygray)
  - [`NobelGray`](#nobelgray)
  - [`DarkSilverChalice`](#darksilverchalice)
  - [`LightSilverChalice`](#lightsilverchalice)
  - [`DarkSilver`](#darksilver)
  - [`Silver`](#silver)
  - [`DarkAlto`](#darkalto)
  - [`Alto`](#alto)
  - [`Mercury`](#mercury)
  - [`GalleryGray`](#gallerygray)
- [Macros](#macros)
  - [`xterm_colors!`](#xterm_colors)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dynamic`](#dynamic) | mod |  |
| [`UserBlack`](#userblack) | struct |  |
| [`UserRed`](#userred) | struct |  |
| [`UserGreen`](#usergreen) | struct |  |
| [`UserYellow`](#useryellow) | struct |  |
| [`UserBlue`](#userblue) | struct |  |
| [`UserMagenta`](#usermagenta) | struct |  |
| [`UserCyan`](#usercyan) | struct |  |
| [`UserWhite`](#userwhite) | struct |  |
| [`UserBrightBlack`](#userbrightblack) | struct |  |
| [`UserBrightRed`](#userbrightred) | struct |  |
| [`UserBrightGreen`](#userbrightgreen) | struct |  |
| [`UserBrightYellow`](#userbrightyellow) | struct |  |
| [`UserBrightBlue`](#userbrightblue) | struct |  |
| [`UserBrightMagenta`](#userbrightmagenta) | struct |  |
| [`UserBrightCyan`](#userbrightcyan) | struct |  |
| [`UserBrightWhite`](#userbrightwhite) | struct |  |
| [`Black`](#black) | struct |  |
| [`StratosBlue`](#stratosblue) | struct |  |
| [`NavyBlue`](#navyblue) | struct |  |
| [`MidnightBlue`](#midnightblue) | struct |  |
| [`DarkBlue`](#darkblue) | struct |  |
| [`Blue`](#blue) | struct |  |
| [`CamaroneGreen`](#camaronegreen) | struct |  |
| [`BlueStone`](#bluestone) | struct |  |
| [`OrientBlue`](#orientblue) | struct |  |
| [`EndeavourBlue`](#endeavourblue) | struct |  |
| [`ScienceBlue`](#scienceblue) | struct |  |
| [`BlueRibbon`](#blueribbon) | struct |  |
| [`JapaneseLaurel`](#japaneselaurel) | struct |  |
| [`DeepSeaGreen`](#deepseagreen) | struct |  |
| [`Teal`](#teal) | struct |  |
| [`DeepCerulean`](#deepcerulean) | struct |  |
| [`LochmaraBlue`](#lochmarablue) | struct |  |
| [`AzureRadiance`](#azureradiance) | struct |  |
| [`LightJapaneseLaurel`](#lightjapaneselaurel) | struct |  |
| [`Jade`](#jade) | struct |  |
| [`PersianGreen`](#persiangreen) | struct |  |
| [`BondiBlue`](#bondiblue) | struct |  |
| [`Cerulean`](#cerulean) | struct |  |
| [`LightAzureRadiance`](#lightazureradiance) | struct |  |
| [`DarkGreen`](#darkgreen) | struct |  |
| [`Malachite`](#malachite) | struct |  |
| [`CaribbeanGreen`](#caribbeangreen) | struct |  |
| [`LightCaribbeanGreen`](#lightcaribbeangreen) | struct |  |
| [`RobinEggBlue`](#robineggblue) | struct |  |
| [`Aqua`](#aqua) | struct |  |
| [`Green`](#green) | struct |  |
| [`DarkSpringGreen`](#darkspringgreen) | struct |  |
| [`SpringGreen`](#springgreen) | struct |  |
| [`LightSpringGreen`](#lightspringgreen) | struct |  |
| [`BrightTurquoise`](#brightturquoise) | struct |  |
| [`Cyan`](#cyan) | struct |  |
| [`Rosewood`](#rosewood) | struct |  |
| [`PompadourMagenta`](#pompadourmagenta) | struct |  |
| [`PigmentIndigo`](#pigmentindigo) | struct |  |
| [`DarkPurple`](#darkpurple) | struct |  |
| [`ElectricIndigo`](#electricindigo) | struct |  |
| [`ElectricPurple`](#electricpurple) | struct |  |
| [`VerdunGreen`](#verdungreen) | struct |  |
| [`ScorpionOlive`](#scorpionolive) | struct |  |
| [`Lilac`](#lilac) | struct |  |
| [`ScampiIndigo`](#scampiindigo) | struct |  |
| [`Indigo`](#indigo) | struct |  |
| [`DarkCornflowerBlue`](#darkcornflowerblue) | struct |  |
| [`DarkLimeade`](#darklimeade) | struct |  |
| [`GladeGreen`](#gladegreen) | struct |  |
| [`JuniperGreen`](#junipergreen) | struct |  |
| [`HippieBlue`](#hippieblue) | struct |  |
| [`HavelockBlue`](#havelockblue) | struct |  |
| [`CornflowerBlue`](#cornflowerblue) | struct |  |
| [`Limeade`](#limeade) | struct |  |
| [`FernGreen`](#ferngreen) | struct |  |
| [`SilverTree`](#silvertree) | struct |  |
| [`Tradewind`](#tradewind) | struct |  |
| [`ShakespeareBlue`](#shakespeareblue) | struct |  |
| [`DarkMalibuBlue`](#darkmalibublue) | struct |  |
| [`DarkBrightGreen`](#darkbrightgreen) | struct |  |
| [`DarkPastelGreen`](#darkpastelgreen) | struct |  |
| [`PastelGreen`](#pastelgreen) | struct |  |
| [`DownyTeal`](#downyteal) | struct |  |
| [`Viking`](#viking) | struct |  |
| [`MalibuBlue`](#malibublue) | struct |  |
| [`BrightGreen`](#brightgreen) | struct |  |
| [`DarkScreaminGreen`](#darkscreamingreen) | struct |  |
| [`ScreaminGreen`](#screamingreen) | struct |  |
| [`DarkAquamarine`](#darkaquamarine) | struct |  |
| [`Aquamarine`](#aquamarine) | struct |  |
| [`LightAquamarine`](#lightaquamarine) | struct |  |
| [`Maroon`](#maroon) | struct |  |
| [`DarkFreshEggplant`](#darkfresheggplant) | struct |  |
| [`LightFreshEggplant`](#lightfresheggplant) | struct |  |
| [`Purple`](#purple) | struct |  |
| [`ElectricViolet`](#electricviolet) | struct |  |
| [`LightElectricViolet`](#lightelectricviolet) | struct |  |
| [`Brown`](#brown) | struct |  |
| [`CopperRose`](#copperrose) | struct |  |
| [`StrikemasterPurple`](#strikemasterpurple) | struct |  |
| [`DelugePurple`](#delugepurple) | struct |  |
| [`DarkMediumPurple`](#darkmediumpurple) | struct |  |
| [`DarkHeliotropePurple`](#darkheliotropepurple) | struct |  |
| [`Olive`](#olive) | struct |  |
| [`ClayCreekOlive`](#claycreekolive) | struct |  |
| [`DarkGray`](#darkgray) | struct |  |
| [`WildBlueYonder`](#wildblueyonder) | struct |  |
| [`ChetwodeBlue`](#chetwodeblue) | struct |  |
| [`SlateBlue`](#slateblue) | struct |  |
| [`LightLimeade`](#lightlimeade) | struct |  |
| [`ChelseaCucumber`](#chelseacucumber) | struct |  |
| [`BayLeaf`](#bayleaf) | struct |  |
| [`GulfStream`](#gulfstream) | struct |  |
| [`PoloBlue`](#poloblue) | struct |  |
| [`LightMalibuBlue`](#lightmalibublue) | struct |  |
| [`Pistachio`](#pistachio) | struct |  |
| [`LightPastelGreen`](#lightpastelgreen) | struct |  |
| [`DarkFeijoaGreen`](#darkfeijoagreen) | struct |  |
| [`VistaBlue`](#vistablue) | struct |  |
| [`Bermuda`](#bermuda) | struct |  |
| [`DarkAnakiwaBlue`](#darkanakiwablue) | struct |  |
| [`ChartreuseGreen`](#chartreusegreen) | struct |  |
| [`LightScreaminGreen`](#lightscreamingreen) | struct |  |
| [`DarkMintGreen`](#darkmintgreen) | struct |  |
| [`MintGreen`](#mintgreen) | struct |  |
| [`LighterAquamarine`](#lighteraquamarine) | struct |  |
| [`AnakiwaBlue`](#anakiwablue) | struct |  |
| [`BrightRed`](#brightred) | struct |  |
| [`DarkFlirt`](#darkflirt) | struct |  |
| [`Flirt`](#flirt) | struct |  |
| [`LightFlirt`](#lightflirt) | struct |  |
| [`DarkViolet`](#darkviolet) | struct |  |
| [`BrightElectricViolet`](#brightelectricviolet) | struct |  |
| [`RoseofSharonOrange`](#roseofsharonorange) | struct |  |
| [`MatrixPink`](#matrixpink) | struct |  |
| [`TapestryPink`](#tapestrypink) | struct |  |
| [`FuchsiaPink`](#fuchsiapink) | struct |  |
| [`MediumPurple`](#mediumpurple) | struct |  |
| [`Heliotrope`](#heliotrope) | struct |  |
| [`PirateGold`](#pirategold) | struct |  |
| [`MuesliOrange`](#muesliorange) | struct |  |
| [`PharlapPink`](#pharlappink) | struct |  |
| [`Bouquet`](#bouquet) | struct |  |
| [`Lavender`](#lavender) | struct |  |
| [`LightHeliotrope`](#lightheliotrope) | struct |  |
| [`BuddhaGold`](#buddhagold) | struct |  |
| [`OliveGreen`](#olivegreen) | struct |  |
| [`HillaryOlive`](#hillaryolive) | struct |  |
| [`SilverChalice`](#silverchalice) | struct |  |
| [`WistfulLilac`](#wistfullilac) | struct |  |
| [`MelroseLilac`](#melroselilac) | struct |  |
| [`RioGrandeGreen`](#riograndegreen) | struct |  |
| [`ConiferGreen`](#conifergreen) | struct |  |
| [`Feijoa`](#feijoa) | struct |  |
| [`PixieGreen`](#pixiegreen) | struct |  |
| [`JungleMist`](#junglemist) | struct |  |
| [`LightAnakiwaBlue`](#lightanakiwablue) | struct |  |
| [`Lime`](#lime) | struct |  |
| [`GreenYellow`](#greenyellow) | struct |  |
| [`LightMintGreen`](#lightmintgreen) | struct |  |
| [`Celadon`](#celadon) | struct |  |
| [`AeroBlue`](#aeroblue) | struct |  |
| [`FrenchPassLightBlue`](#frenchpasslightblue) | struct |  |
| [`GuardsmanRed`](#guardsmanred) | struct |  |
| [`RazzmatazzCerise`](#razzmatazzcerise) | struct |  |
| [`MediumVioletRed`](#mediumvioletred) | struct |  |
| [`HollywoodCerise`](#hollywoodcerise) | struct |  |
| [`DarkPurplePizzazz`](#darkpurplepizzazz) | struct |  |
| [`BrighterElectricViolet`](#brighterelectricviolet) | struct |  |
| [`TennOrange`](#tennorange) | struct |  |
| [`RomanOrange`](#romanorange) | struct |  |
| [`CranberryPink`](#cranberrypink) | struct |  |
| [`HopbushPink`](#hopbushpink) | struct |  |
| [`Orchid`](#orchid) | struct |  |
| [`LighterHeliotrope`](#lighterheliotrope) | struct |  |
| [`MangoTango`](#mangotango) | struct |  |
| [`Copperfield`](#copperfield) | struct |  |
| [`SeaPink`](#seapink) | struct |  |
| [`CanCanPink`](#cancanpink) | struct |  |
| [`LightOrchid`](#lightorchid) | struct |  |
| [`BrightHeliotrope`](#brightheliotrope) | struct |  |
| [`DarkCorn`](#darkcorn) | struct |  |
| [`DarkTachaOrange`](#darktachaorange) | struct |  |
| [`TanBeige`](#tanbeige) | struct |  |
| [`ClamShell`](#clamshell) | struct |  |
| [`ThistlePink`](#thistlepink) | struct |  |
| [`Mauve`](#mauve) | struct |  |
| [`Corn`](#corn) | struct |  |
| [`TachaOrange`](#tachaorange) | struct |  |
| [`DecoOrange`](#decoorange) | struct |  |
| [`PaleGoldenrod`](#palegoldenrod) | struct |  |
| [`AltoBeige`](#altobeige) | struct |  |
| [`FogPink`](#fogpink) | struct |  |
| [`ChartreuseYellow`](#chartreuseyellow) | struct |  |
| [`Canary`](#canary) | struct |  |
| [`Honeysuckle`](#honeysuckle) | struct |  |
| [`ReefPaleYellow`](#reefpaleyellow) | struct |  |
| [`SnowyMint`](#snowymint) | struct |  |
| [`OysterBay`](#oysterbay) | struct |  |
| [`Red`](#red) | struct |  |
| [`DarkRose`](#darkrose) | struct |  |
| [`Rose`](#rose) | struct |  |
| [`LightHollywoodCerise`](#lighthollywoodcerise) | struct |  |
| [`PurplePizzazz`](#purplepizzazz) | struct |  |
| [`Fuchsia`](#fuchsia) | struct |  |
| [`BlazeOrange`](#blazeorange) | struct |  |
| [`BittersweetOrange`](#bittersweetorange) | struct |  |
| [`WildWatermelon`](#wildwatermelon) | struct |  |
| [`DarkHotPink`](#darkhotpink) | struct |  |
| [`HotPink`](#hotpink) | struct |  |
| [`PinkFlamingo`](#pinkflamingo) | struct |  |
| [`FlushOrange`](#flushorange) | struct |  |
| [`Salmon`](#salmon) | struct |  |
| [`VividTangerine`](#vividtangerine) | struct |  |
| [`PinkSalmon`](#pinksalmon) | struct |  |
| [`DarkLavenderRose`](#darklavenderrose) | struct |  |
| [`BlushPink`](#blushpink) | struct |  |
| [`YellowSea`](#yellowsea) | struct |  |
| [`TexasRose`](#texasrose) | struct |  |
| [`Tacao`](#tacao) | struct |  |
| [`Sundown`](#sundown) | struct |  |
| [`CottonCandy`](#cottoncandy) | struct |  |
| [`LavenderRose`](#lavenderrose) | struct |  |
| [`Gold`](#gold) | struct |  |
| [`Dandelion`](#dandelion) | struct |  |
| [`GrandisCaramel`](#grandiscaramel) | struct |  |
| [`Caramel`](#caramel) | struct |  |
| [`CosmosSalmon`](#cosmossalmon) | struct |  |
| [`PinkLace`](#pinklace) | struct |  |
| [`Yellow`](#yellow) | struct |  |
| [`LaserLemon`](#laserlemon) | struct |  |
| [`DollyYellow`](#dollyyellow) | struct |  |
| [`PortafinoYellow`](#portafinoyellow) | struct |  |
| [`Cumulus`](#cumulus) | struct |  |
| [`White`](#white) | struct |  |
| [`DarkCodGray`](#darkcodgray) | struct |  |
| [`CodGray`](#codgray) | struct |  |
| [`LightCodGray`](#lightcodgray) | struct |  |
| [`DarkMineShaft`](#darkmineshaft) | struct |  |
| [`MineShaft`](#mineshaft) | struct |  |
| [`LightMineShaft`](#lightmineshaft) | struct |  |
| [`DarkTundora`](#darktundora) | struct |  |
| [`Tundora`](#tundora) | struct |  |
| [`ScorpionGray`](#scorpiongray) | struct |  |
| [`DarkDoveGray`](#darkdovegray) | struct |  |
| [`DoveGray`](#dovegray) | struct |  |
| [`Boulder`](#boulder) | struct |  |
| [`Gray`](#gray) | struct |  |
| [`LightGray`](#lightgray) | struct |  |
| [`DustyGray`](#dustygray) | struct |  |
| [`NobelGray`](#nobelgray) | struct |  |
| [`DarkSilverChalice`](#darksilverchalice) | struct |  |
| [`LightSilverChalice`](#lightsilverchalice) | struct |  |
| [`DarkSilver`](#darksilver) | struct |  |
| [`Silver`](#silver) | struct |  |
| [`DarkAlto`](#darkalto) | struct |  |
| [`Alto`](#alto) | struct |  |
| [`Mercury`](#mercury) | struct |  |
| [`GalleryGray`](#gallerygray) | struct |  |
| [`xterm_colors!`](#xterm_colors) | macro |  |

## Modules

- [`dynamic`](dynamic/index.md)

## Structs

### `UserBlack`

```rust
struct UserBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBlack`

- <span id="userblack-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblack-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblack-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblack-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBlack`

### `UserRed`

```rust
struct UserRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserRed`

- <span id="userred-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userred-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userred-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userred-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserRed`

### `UserGreen`

```rust
struct UserGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserGreen`

- <span id="usergreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usergreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usergreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usergreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserGreen`

### `UserYellow`

```rust
struct UserYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserYellow`

- <span id="useryellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="useryellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="useryellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="useryellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserYellow`

### `UserBlue`

```rust
struct UserBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBlue`

- <span id="userblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBlue`

### `UserMagenta`

```rust
struct UserMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserMagenta`

- <span id="usermagenta-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usermagenta-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usermagenta-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usermagenta-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserMagenta`

### `UserCyan`

```rust
struct UserCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserCyan`

- <span id="usercyan-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usercyan-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usercyan-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usercyan-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserCyan`

### `UserWhite`

```rust
struct UserWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserWhite`

- <span id="userwhite-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userwhite-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userwhite-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userwhite-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserWhite`

### `UserBrightBlack`

```rust
struct UserBrightBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightBlack`

- <span id="userbrightblack-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblack-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblack-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblack-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightBlack`

### `UserBrightRed`

```rust
struct UserBrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightRed`

- <span id="userbrightred-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightred-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightred-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightred-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightRed`

### `UserBrightGreen`

```rust
struct UserBrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightGreen`

- <span id="userbrightgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightGreen`

### `UserBrightYellow`

```rust
struct UserBrightYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightYellow`

- <span id="userbrightyellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightyellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightyellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightyellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightYellow`

### `UserBrightBlue`

```rust
struct UserBrightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightBlue`

- <span id="userbrightblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightBlue`

### `UserBrightMagenta`

```rust
struct UserBrightMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightMagenta`

- <span id="userbrightmagenta-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightmagenta-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightmagenta-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightmagenta-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightMagenta`

### `UserBrightCyan`

```rust
struct UserBrightCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightCyan`

- <span id="userbrightcyan-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightcyan-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightcyan-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightcyan-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightCyan`

### `UserBrightWhite`

```rust
struct UserBrightWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightWhite`

- <span id="userbrightwhite-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightwhite-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightwhite-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightwhite-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightWhite`

### `Black`

```rust
struct Black;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Black`

- <span id="black-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="black-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Black`

### `StratosBlue`

```rust
struct StratosBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for StratosBlue`

- <span id="stratosblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="stratosblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="stratosblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="stratosblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for StratosBlue`

### `NavyBlue`

```rust
struct NavyBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for NavyBlue`

- <span id="navyblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="navyblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="navyblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="navyblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for NavyBlue`

### `MidnightBlue`

```rust
struct MidnightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MidnightBlue`

- <span id="midnightblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="midnightblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="midnightblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="midnightblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MidnightBlue`

### `DarkBlue`

```rust
struct DarkBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkBlue`

- <span id="darkblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkBlue`

### `Blue`

```rust
struct Blue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Blue`

- <span id="blue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Blue`

### `CamaroneGreen`

```rust
struct CamaroneGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CamaroneGreen`

- <span id="camaronegreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="camaronegreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="camaronegreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="camaronegreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CamaroneGreen`

### `BlueStone`

```rust
struct BlueStone;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlueStone`

- <span id="bluestone-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bluestone-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bluestone-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bluestone-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlueStone`

### `OrientBlue`

```rust
struct OrientBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for OrientBlue`

- <span id="orientblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orientblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orientblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orientblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for OrientBlue`

### `EndeavourBlue`

```rust
struct EndeavourBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for EndeavourBlue`

- <span id="endeavourblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="endeavourblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="endeavourblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="endeavourblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for EndeavourBlue`

### `ScienceBlue`

```rust
struct ScienceBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScienceBlue`

- <span id="scienceblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scienceblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scienceblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scienceblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScienceBlue`

### `BlueRibbon`

```rust
struct BlueRibbon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlueRibbon`

- <span id="blueribbon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blueribbon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blueribbon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blueribbon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlueRibbon`

### `JapaneseLaurel`

```rust
struct JapaneseLaurel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for JapaneseLaurel`

- <span id="japaneselaurel-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="japaneselaurel-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="japaneselaurel-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="japaneselaurel-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for JapaneseLaurel`

### `DeepSeaGreen`

```rust
struct DeepSeaGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DeepSeaGreen`

- <span id="deepseagreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepseagreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepseagreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepseagreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DeepSeaGreen`

### `Teal`

```rust
struct Teal;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Teal`

- <span id="teal-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="teal-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="teal-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="teal-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Teal`

### `DeepCerulean`

```rust
struct DeepCerulean;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DeepCerulean`

- <span id="deepcerulean-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepcerulean-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepcerulean-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepcerulean-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DeepCerulean`

### `LochmaraBlue`

```rust
struct LochmaraBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LochmaraBlue`

- <span id="lochmarablue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lochmarablue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lochmarablue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lochmarablue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LochmaraBlue`

### `AzureRadiance`

```rust
struct AzureRadiance;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AzureRadiance`

- <span id="azureradiance-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="azureradiance-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="azureradiance-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="azureradiance-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AzureRadiance`

### `LightJapaneseLaurel`

```rust
struct LightJapaneseLaurel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightjapaneselaurel-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightJapaneseLaurel`

### `Jade`

```rust
struct Jade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Jade`

- <span id="jade-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="jade-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="jade-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="jade-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Jade`

### `PersianGreen`

```rust
struct PersianGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PersianGreen`

- <span id="persiangreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="persiangreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="persiangreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="persiangreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PersianGreen`

### `BondiBlue`

```rust
struct BondiBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BondiBlue`

- <span id="bondiblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bondiblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bondiblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bondiblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BondiBlue`

### `Cerulean`

```rust
struct Cerulean;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Cerulean`

- <span id="cerulean-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cerulean-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cerulean-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cerulean-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Cerulean`

### `LightAzureRadiance`

```rust
struct LightAzureRadiance;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightAzureRadiance`

- <span id="lightazureradiance-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightazureradiance-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightazureradiance-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightazureradiance-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightAzureRadiance`

### `DarkGreen`

```rust
struct DarkGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkGreen`

- <span id="darkgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkGreen`

### `Malachite`

```rust
struct Malachite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Malachite`

- <span id="malachite-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malachite-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malachite-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malachite-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Malachite`

### `CaribbeanGreen`

```rust
struct CaribbeanGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CaribbeanGreen`

- <span id="caribbeangreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caribbeangreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caribbeangreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caribbeangreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CaribbeanGreen`

### `LightCaribbeanGreen`

```rust
struct LightCaribbeanGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcaribbeangreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightCaribbeanGreen`

### `RobinEggBlue`

```rust
struct RobinEggBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RobinEggBlue`

- <span id="robineggblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="robineggblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="robineggblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="robineggblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RobinEggBlue`

### `Aqua`

```rust
struct Aqua;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Aqua`

- <span id="aqua-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aqua-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aqua-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aqua-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Aqua`

### `Green`

```rust
struct Green;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Green`

- <span id="green-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="green-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Green`

### `DarkSpringGreen`

```rust
struct DarkSpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkSpringGreen`

- <span id="darkspringgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkspringgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkspringgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkspringgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkSpringGreen`

### `SpringGreen`

```rust
struct SpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SpringGreen`

- <span id="springgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="springgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="springgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="springgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SpringGreen`

### `LightSpringGreen`

```rust
struct LightSpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightSpringGreen`

- <span id="lightspringgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightspringgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightspringgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightspringgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightSpringGreen`

### `BrightTurquoise`

```rust
struct BrightTurquoise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightTurquoise`

- <span id="brightturquoise-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightturquoise-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightturquoise-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightturquoise-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightTurquoise`

### `Cyan`

```rust
struct Cyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Cyan`

- <span id="cyan-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cyan-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Cyan`

### `Rosewood`

```rust
struct Rosewood;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Rosewood`

- <span id="rosewood-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rosewood-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rosewood-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rosewood-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Rosewood`

### `PompadourMagenta`

```rust
struct PompadourMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PompadourMagenta`

- <span id="pompadourmagenta-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pompadourmagenta-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pompadourmagenta-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pompadourmagenta-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PompadourMagenta`

### `PigmentIndigo`

```rust
struct PigmentIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PigmentIndigo`

- <span id="pigmentindigo-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pigmentindigo-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pigmentindigo-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pigmentindigo-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PigmentIndigo`

### `DarkPurple`

```rust
struct DarkPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkPurple`

- <span id="darkpurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkPurple`

### `ElectricIndigo`

```rust
struct ElectricIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ElectricIndigo`

- <span id="electricindigo-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricindigo-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricindigo-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricindigo-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ElectricIndigo`

### `ElectricPurple`

```rust
struct ElectricPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ElectricPurple`

- <span id="electricpurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricpurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricpurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricpurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ElectricPurple`

### `VerdunGreen`

```rust
struct VerdunGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for VerdunGreen`

- <span id="verdungreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="verdungreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="verdungreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="verdungreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for VerdunGreen`

### `ScorpionOlive`

```rust
struct ScorpionOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScorpionOlive`

- <span id="scorpionolive-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpionolive-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpionolive-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpionolive-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScorpionOlive`

### `Lilac`

```rust
struct Lilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Lilac`

- <span id="lilac-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lilac-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lilac-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lilac-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Lilac`

### `ScampiIndigo`

```rust
struct ScampiIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScampiIndigo`

- <span id="scampiindigo-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scampiindigo-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scampiindigo-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scampiindigo-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScampiIndigo`

### `Indigo`

```rust
struct Indigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Indigo`

- <span id="indigo-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="indigo-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="indigo-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="indigo-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Indigo`

### `DarkCornflowerBlue`

```rust
struct DarkCornflowerBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkCornflowerBlue`

- <span id="darkcornflowerblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcornflowerblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcornflowerblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcornflowerblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkCornflowerBlue`

### `DarkLimeade`

```rust
struct DarkLimeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkLimeade`

- <span id="darklimeade-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklimeade-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklimeade-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklimeade-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkLimeade`

### `GladeGreen`

```rust
struct GladeGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GladeGreen`

- <span id="gladegreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gladegreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gladegreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gladegreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GladeGreen`

### `JuniperGreen`

```rust
struct JuniperGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for JuniperGreen`

- <span id="junipergreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junipergreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junipergreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junipergreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for JuniperGreen`

### `HippieBlue`

```rust
struct HippieBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HippieBlue`

- <span id="hippieblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hippieblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hippieblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hippieblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HippieBlue`

### `HavelockBlue`

```rust
struct HavelockBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HavelockBlue`

- <span id="havelockblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="havelockblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="havelockblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="havelockblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HavelockBlue`

### `CornflowerBlue`

```rust
struct CornflowerBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CornflowerBlue`

- <span id="cornflowerblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cornflowerblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cornflowerblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cornflowerblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CornflowerBlue`

### `Limeade`

```rust
struct Limeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Limeade`

- <span id="limeade-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="limeade-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="limeade-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="limeade-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Limeade`

### `FernGreen`

```rust
struct FernGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FernGreen`

- <span id="ferngreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="ferngreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="ferngreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="ferngreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FernGreen`

### `SilverTree`

```rust
struct SilverTree;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SilverTree`

- <span id="silvertree-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silvertree-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silvertree-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silvertree-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SilverTree`

### `Tradewind`

```rust
struct Tradewind;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Tradewind`

- <span id="tradewind-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tradewind-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tradewind-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tradewind-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Tradewind`

### `ShakespeareBlue`

```rust
struct ShakespeareBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ShakespeareBlue`

- <span id="shakespeareblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="shakespeareblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="shakespeareblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="shakespeareblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ShakespeareBlue`

### `DarkMalibuBlue`

```rust
struct DarkMalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMalibuBlue`

- <span id="darkmalibublue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmalibublue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmalibublue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmalibublue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMalibuBlue`

### `DarkBrightGreen`

```rust
struct DarkBrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkBrightGreen`

- <span id="darkbrightgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkbrightgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkbrightgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkbrightgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkBrightGreen`

### `DarkPastelGreen`

```rust
struct DarkPastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkPastelGreen`

- <span id="darkpastelgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpastelgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpastelgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpastelgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkPastelGreen`

### `PastelGreen`

```rust
struct PastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PastelGreen`

- <span id="pastelgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pastelgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pastelgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pastelgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PastelGreen`

### `DownyTeal`

```rust
struct DownyTeal;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DownyTeal`

- <span id="downyteal-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="downyteal-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="downyteal-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="downyteal-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DownyTeal`

### `Viking`

```rust
struct Viking;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Viking`

- <span id="viking-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="viking-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="viking-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="viking-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Viking`

### `MalibuBlue`

```rust
struct MalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MalibuBlue`

- <span id="malibublue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malibublue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malibublue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malibublue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MalibuBlue`

### `BrightGreen`

```rust
struct BrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightGreen`

- <span id="brightgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightGreen`

### `DarkScreaminGreen`

```rust
struct DarkScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkScreaminGreen`

- <span id="darkscreamingreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkscreamingreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkscreamingreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkscreamingreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkScreaminGreen`

### `ScreaminGreen`

```rust
struct ScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScreaminGreen`

- <span id="screamingreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="screamingreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="screamingreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="screamingreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScreaminGreen`

### `DarkAquamarine`

```rust
struct DarkAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkAquamarine`

- <span id="darkaquamarine-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkaquamarine-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkaquamarine-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkaquamarine-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkAquamarine`

### `Aquamarine`

```rust
struct Aquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Aquamarine`

- <span id="aquamarine-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aquamarine-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aquamarine-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aquamarine-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Aquamarine`

### `LightAquamarine`

```rust
struct LightAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightAquamarine`

- <span id="lightaquamarine-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightaquamarine-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightaquamarine-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightaquamarine-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightAquamarine`

### `Maroon`

```rust
struct Maroon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Maroon`

- <span id="maroon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="maroon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="maroon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="maroon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Maroon`

### `DarkFreshEggplant`

```rust
struct DarkFreshEggplant;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkFreshEggplant`

- <span id="darkfresheggplant-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfresheggplant-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfresheggplant-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfresheggplant-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkFreshEggplant`

### `LightFreshEggplant`

```rust
struct LightFreshEggplant;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightFreshEggplant`

- <span id="lightfresheggplant-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightfresheggplant-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightfresheggplant-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightfresheggplant-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightFreshEggplant`

### `Purple`

```rust
struct Purple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Purple`

- <span id="purple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Purple`

### `ElectricViolet`

```rust
struct ElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ElectricViolet`

- <span id="electricviolet-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricviolet-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricviolet-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricviolet-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ElectricViolet`

### `LightElectricViolet`

```rust
struct LightElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightElectricViolet`

- <span id="lightelectricviolet-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightelectricviolet-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightelectricviolet-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightelectricviolet-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightElectricViolet`

### `Brown`

```rust
struct Brown;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Brown`

- <span id="brown-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brown-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brown-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brown-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Brown`

### `CopperRose`

```rust
struct CopperRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CopperRose`

- <span id="copperrose-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperrose-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperrose-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperrose-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CopperRose`

### `StrikemasterPurple`

```rust
struct StrikemasterPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for StrikemasterPurple`

- <span id="strikemasterpurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="strikemasterpurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="strikemasterpurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="strikemasterpurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for StrikemasterPurple`

### `DelugePurple`

```rust
struct DelugePurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DelugePurple`

- <span id="delugepurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="delugepurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="delugepurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="delugepurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DelugePurple`

### `DarkMediumPurple`

```rust
struct DarkMediumPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMediumPurple`

- <span id="darkmediumpurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmediumpurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmediumpurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmediumpurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMediumPurple`

### `DarkHeliotropePurple`

```rust
struct DarkHeliotropePurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkHeliotropePurple`

- <span id="darkheliotropepurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkheliotropepurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkheliotropepurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkheliotropepurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkHeliotropePurple`

### `Olive`

```rust
struct Olive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Olive`

- <span id="olive-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olive-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olive-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olive-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Olive`

### `ClayCreekOlive`

```rust
struct ClayCreekOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ClayCreekOlive`

- <span id="claycreekolive-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="claycreekolive-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="claycreekolive-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="claycreekolive-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ClayCreekOlive`

### `DarkGray`

```rust
struct DarkGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkGray`

- <span id="darkgray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkGray`

### `WildBlueYonder`

```rust
struct WildBlueYonder;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for WildBlueYonder`

- <span id="wildblueyonder-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildblueyonder-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildblueyonder-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildblueyonder-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for WildBlueYonder`

### `ChetwodeBlue`

```rust
struct ChetwodeBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChetwodeBlue`

- <span id="chetwodeblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chetwodeblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chetwodeblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chetwodeblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChetwodeBlue`

### `SlateBlue`

```rust
struct SlateBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SlateBlue`

- <span id="slateblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="slateblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="slateblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="slateblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SlateBlue`

### `LightLimeade`

```rust
struct LightLimeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightLimeade`

- <span id="lightlimeade-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightlimeade-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightlimeade-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightlimeade-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightLimeade`

### `ChelseaCucumber`

```rust
struct ChelseaCucumber;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChelseaCucumber`

- <span id="chelseacucumber-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chelseacucumber-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chelseacucumber-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chelseacucumber-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChelseaCucumber`

### `BayLeaf`

```rust
struct BayLeaf;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BayLeaf`

- <span id="bayleaf-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bayleaf-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bayleaf-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bayleaf-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BayLeaf`

### `GulfStream`

```rust
struct GulfStream;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GulfStream`

- <span id="gulfstream-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gulfstream-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gulfstream-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gulfstream-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GulfStream`

### `PoloBlue`

```rust
struct PoloBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PoloBlue`

- <span id="poloblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="poloblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="poloblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="poloblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PoloBlue`

### `LightMalibuBlue`

```rust
struct LightMalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightMalibuBlue`

- <span id="lightmalibublue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmalibublue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmalibublue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmalibublue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightMalibuBlue`

### `Pistachio`

```rust
struct Pistachio;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Pistachio`

- <span id="pistachio-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pistachio-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pistachio-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pistachio-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Pistachio`

### `LightPastelGreen`

```rust
struct LightPastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightPastelGreen`

- <span id="lightpastelgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightpastelgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightpastelgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightpastelgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightPastelGreen`

### `DarkFeijoaGreen`

```rust
struct DarkFeijoaGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkFeijoaGreen`

- <span id="darkfeijoagreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfeijoagreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfeijoagreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfeijoagreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkFeijoaGreen`

### `VistaBlue`

```rust
struct VistaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for VistaBlue`

- <span id="vistablue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vistablue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vistablue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vistablue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for VistaBlue`

### `Bermuda`

```rust
struct Bermuda;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Bermuda`

- <span id="bermuda-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bermuda-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bermuda-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bermuda-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Bermuda`

### `DarkAnakiwaBlue`

```rust
struct DarkAnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkAnakiwaBlue`

- <span id="darkanakiwablue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkanakiwablue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkanakiwablue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkanakiwablue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkAnakiwaBlue`

### `ChartreuseGreen`

```rust
struct ChartreuseGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChartreuseGreen`

- <span id="chartreusegreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreusegreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreusegreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreusegreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChartreuseGreen`

### `LightScreaminGreen`

```rust
struct LightScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightScreaminGreen`

- <span id="lightscreamingreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightscreamingreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightscreamingreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightscreamingreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightScreaminGreen`

### `DarkMintGreen`

```rust
struct DarkMintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMintGreen`

- <span id="darkmintgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmintgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmintgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmintgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMintGreen`

### `MintGreen`

```rust
struct MintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MintGreen`

- <span id="mintgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mintgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mintgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mintgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MintGreen`

### `LighterAquamarine`

```rust
struct LighterAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LighterAquamarine`

- <span id="lighteraquamarine-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighteraquamarine-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighteraquamarine-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighteraquamarine-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LighterAquamarine`

### `AnakiwaBlue`

```rust
struct AnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AnakiwaBlue`

- <span id="anakiwablue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="anakiwablue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="anakiwablue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="anakiwablue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AnakiwaBlue`

### `BrightRed`

```rust
struct BrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightRed`

- <span id="brightred-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightred-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightRed`

### `DarkFlirt`

```rust
struct DarkFlirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkFlirt`

- <span id="darkflirt-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkflirt-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkflirt-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkflirt-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkFlirt`

### `Flirt`

```rust
struct Flirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Flirt`

- <span id="flirt-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flirt-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flirt-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flirt-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Flirt`

### `LightFlirt`

```rust
struct LightFlirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightFlirt`

- <span id="lightflirt-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightflirt-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightflirt-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightflirt-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightFlirt`

### `DarkViolet`

```rust
struct DarkViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkViolet`

- <span id="darkviolet-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkviolet-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkviolet-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkviolet-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkViolet`

### `BrightElectricViolet`

```rust
struct BrightElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightElectricViolet`

- <span id="brightelectricviolet-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightelectricviolet-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightelectricviolet-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightelectricviolet-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightElectricViolet`

### `RoseofSharonOrange`

```rust
struct RoseofSharonOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RoseofSharonOrange`

- <span id="roseofsharonorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="roseofsharonorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="roseofsharonorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="roseofsharonorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RoseofSharonOrange`

### `MatrixPink`

```rust
struct MatrixPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MatrixPink`

- <span id="matrixpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="matrixpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="matrixpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="matrixpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MatrixPink`

### `TapestryPink`

```rust
struct TapestryPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TapestryPink`

- <span id="tapestrypink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tapestrypink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tapestrypink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tapestrypink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TapestryPink`

### `FuchsiaPink`

```rust
struct FuchsiaPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FuchsiaPink`

- <span id="fuchsiapink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsiapink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsiapink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsiapink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FuchsiaPink`

### `MediumPurple`

```rust
struct MediumPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MediumPurple`

- <span id="mediumpurple-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumpurple-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumpurple-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumpurple-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MediumPurple`

### `Heliotrope`

```rust
struct Heliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Heliotrope`

- <span id="heliotrope-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="heliotrope-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="heliotrope-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="heliotrope-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Heliotrope`

### `PirateGold`

```rust
struct PirateGold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PirateGold`

- <span id="pirategold-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pirategold-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pirategold-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pirategold-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PirateGold`

### `MuesliOrange`

```rust
struct MuesliOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MuesliOrange`

- <span id="muesliorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="muesliorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="muesliorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="muesliorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MuesliOrange`

### `PharlapPink`

```rust
struct PharlapPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PharlapPink`

- <span id="pharlappink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pharlappink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pharlappink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pharlappink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PharlapPink`

### `Bouquet`

```rust
struct Bouquet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Bouquet`

- <span id="bouquet-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bouquet-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bouquet-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bouquet-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Bouquet`

### `Lavender`

```rust
struct Lavender;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Lavender`

- <span id="lavender-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavender-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavender-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavender-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Lavender`

### `LightHeliotrope`

```rust
struct LightHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightHeliotrope`

- <span id="lightheliotrope-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightheliotrope-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightheliotrope-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightheliotrope-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightHeliotrope`

### `BuddhaGold`

```rust
struct BuddhaGold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BuddhaGold`

- <span id="buddhagold-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="buddhagold-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="buddhagold-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="buddhagold-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BuddhaGold`

### `OliveGreen`

```rust
struct OliveGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for OliveGreen`

- <span id="olivegreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olivegreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olivegreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olivegreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for OliveGreen`

### `HillaryOlive`

```rust
struct HillaryOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HillaryOlive`

- <span id="hillaryolive-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hillaryolive-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hillaryolive-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hillaryolive-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HillaryOlive`

### `SilverChalice`

```rust
struct SilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SilverChalice`

- <span id="silverchalice-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silverchalice-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silverchalice-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silverchalice-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SilverChalice`

### `WistfulLilac`

```rust
struct WistfulLilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for WistfulLilac`

- <span id="wistfullilac-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wistfullilac-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wistfullilac-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wistfullilac-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for WistfulLilac`

### `MelroseLilac`

```rust
struct MelroseLilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MelroseLilac`

- <span id="melroselilac-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="melroselilac-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="melroselilac-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="melroselilac-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MelroseLilac`

### `RioGrandeGreen`

```rust
struct RioGrandeGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RioGrandeGreen`

- <span id="riograndegreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="riograndegreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="riograndegreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="riograndegreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RioGrandeGreen`

### `ConiferGreen`

```rust
struct ConiferGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ConiferGreen`

- <span id="conifergreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="conifergreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="conifergreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="conifergreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ConiferGreen`

### `Feijoa`

```rust
struct Feijoa;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Feijoa`

- <span id="feijoa-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="feijoa-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="feijoa-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="feijoa-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Feijoa`

### `PixieGreen`

```rust
struct PixieGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PixieGreen`

- <span id="pixiegreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pixiegreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pixiegreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pixiegreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PixieGreen`

### `JungleMist`

```rust
struct JungleMist;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for JungleMist`

- <span id="junglemist-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junglemist-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junglemist-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junglemist-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for JungleMist`

### `LightAnakiwaBlue`

```rust
struct LightAnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightAnakiwaBlue`

- <span id="lightanakiwablue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightanakiwablue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightanakiwablue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightanakiwablue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightAnakiwaBlue`

### `Lime`

```rust
struct Lime;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Lime`

- <span id="lime-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lime-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lime-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lime-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Lime`

### `GreenYellow`

```rust
struct GreenYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GreenYellow`

- <span id="greenyellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="greenyellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="greenyellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="greenyellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GreenYellow`

### `LightMintGreen`

```rust
struct LightMintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightMintGreen`

- <span id="lightmintgreen-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmintgreen-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmintgreen-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmintgreen-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightMintGreen`

### `Celadon`

```rust
struct Celadon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Celadon`

- <span id="celadon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="celadon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="celadon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="celadon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Celadon`

### `AeroBlue`

```rust
struct AeroBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AeroBlue`

- <span id="aeroblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aeroblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aeroblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aeroblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AeroBlue`

### `FrenchPassLightBlue`

```rust
struct FrenchPassLightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FrenchPassLightBlue`

- <span id="frenchpasslightblue-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="frenchpasslightblue-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="frenchpasslightblue-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="frenchpasslightblue-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FrenchPassLightBlue`

### `GuardsmanRed`

```rust
struct GuardsmanRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GuardsmanRed`

- <span id="guardsmanred-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="guardsmanred-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="guardsmanred-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="guardsmanred-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GuardsmanRed`

### `RazzmatazzCerise`

```rust
struct RazzmatazzCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RazzmatazzCerise`

- <span id="razzmatazzcerise-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="razzmatazzcerise-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="razzmatazzcerise-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="razzmatazzcerise-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RazzmatazzCerise`

### `MediumVioletRed`

```rust
struct MediumVioletRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MediumVioletRed`

- <span id="mediumvioletred-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumvioletred-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumvioletred-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumvioletred-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MediumVioletRed`

### `HollywoodCerise`

```rust
struct HollywoodCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HollywoodCerise`

- <span id="hollywoodcerise-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hollywoodcerise-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hollywoodcerise-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hollywoodcerise-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HollywoodCerise`

### `DarkPurplePizzazz`

```rust
struct DarkPurplePizzazz;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurplepizzazz-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkPurplePizzazz`

### `BrighterElectricViolet`

```rust
struct BrighterElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrighterElectricViolet`

- <span id="brighterelectricviolet-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brighterelectricviolet-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brighterelectricviolet-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brighterelectricviolet-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrighterElectricViolet`

### `TennOrange`

```rust
struct TennOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TennOrange`

- <span id="tennorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tennorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tennorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tennorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TennOrange`

### `RomanOrange`

```rust
struct RomanOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RomanOrange`

- <span id="romanorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="romanorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="romanorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="romanorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RomanOrange`

### `CranberryPink`

```rust
struct CranberryPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CranberryPink`

- <span id="cranberrypink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cranberrypink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cranberrypink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cranberrypink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CranberryPink`

### `HopbushPink`

```rust
struct HopbushPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HopbushPink`

- <span id="hopbushpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hopbushpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hopbushpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hopbushpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HopbushPink`

### `Orchid`

```rust
struct Orchid;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Orchid`

- <span id="orchid-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orchid-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orchid-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orchid-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Orchid`

### `LighterHeliotrope`

```rust
struct LighterHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LighterHeliotrope`

- <span id="lighterheliotrope-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighterheliotrope-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighterheliotrope-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighterheliotrope-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LighterHeliotrope`

### `MangoTango`

```rust
struct MangoTango;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MangoTango`

- <span id="mangotango-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mangotango-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mangotango-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mangotango-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MangoTango`

### `Copperfield`

```rust
struct Copperfield;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Copperfield`

- <span id="copperfield-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperfield-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperfield-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperfield-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Copperfield`

### `SeaPink`

```rust
struct SeaPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SeaPink`

- <span id="seapink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="seapink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="seapink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="seapink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SeaPink`

### `CanCanPink`

```rust
struct CanCanPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CanCanPink`

- <span id="cancanpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cancanpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cancanpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cancanpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CanCanPink`

### `LightOrchid`

```rust
struct LightOrchid;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightOrchid`

- <span id="lightorchid-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightorchid-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightorchid-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightorchid-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightOrchid`

### `BrightHeliotrope`

```rust
struct BrightHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightHeliotrope`

- <span id="brightheliotrope-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightheliotrope-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightheliotrope-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightheliotrope-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightHeliotrope`

### `DarkCorn`

```rust
struct DarkCorn;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkCorn`

- <span id="darkcorn-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcorn-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcorn-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcorn-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkCorn`

### `DarkTachaOrange`

```rust
struct DarkTachaOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkTachaOrange`

- <span id="darktachaorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktachaorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktachaorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktachaorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkTachaOrange`

### `TanBeige`

```rust
struct TanBeige;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TanBeige`

- <span id="tanbeige-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tanbeige-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tanbeige-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tanbeige-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TanBeige`

### `ClamShell`

```rust
struct ClamShell;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ClamShell`

- <span id="clamshell-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="clamshell-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="clamshell-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="clamshell-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ClamShell`

### `ThistlePink`

```rust
struct ThistlePink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ThistlePink`

- <span id="thistlepink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="thistlepink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="thistlepink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="thistlepink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ThistlePink`

### `Mauve`

```rust
struct Mauve;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Mauve`

- <span id="mauve-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mauve-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mauve-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mauve-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Mauve`

### `Corn`

```rust
struct Corn;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Corn`

- <span id="corn-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="corn-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="corn-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="corn-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Corn`

### `TachaOrange`

```rust
struct TachaOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TachaOrange`

- <span id="tachaorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tachaorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tachaorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tachaorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TachaOrange`

### `DecoOrange`

```rust
struct DecoOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DecoOrange`

- <span id="decoorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="decoorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="decoorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="decoorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DecoOrange`

### `PaleGoldenrod`

```rust
struct PaleGoldenrod;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PaleGoldenrod`

- <span id="palegoldenrod-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="palegoldenrod-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="palegoldenrod-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="palegoldenrod-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PaleGoldenrod`

### `AltoBeige`

```rust
struct AltoBeige;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AltoBeige`

- <span id="altobeige-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="altobeige-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="altobeige-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="altobeige-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AltoBeige`

### `FogPink`

```rust
struct FogPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FogPink`

- <span id="fogpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fogpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fogpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fogpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FogPink`

### `ChartreuseYellow`

```rust
struct ChartreuseYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChartreuseYellow`

- <span id="chartreuseyellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreuseyellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreuseyellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreuseyellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChartreuseYellow`

### `Canary`

```rust
struct Canary;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Canary`

- <span id="canary-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="canary-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="canary-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="canary-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Canary`

### `Honeysuckle`

```rust
struct Honeysuckle;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Honeysuckle`

- <span id="honeysuckle-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="honeysuckle-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="honeysuckle-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="honeysuckle-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Honeysuckle`

### `ReefPaleYellow`

```rust
struct ReefPaleYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ReefPaleYellow`

- <span id="reefpaleyellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="reefpaleyellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="reefpaleyellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="reefpaleyellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ReefPaleYellow`

### `SnowyMint`

```rust
struct SnowyMint;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SnowyMint`

- <span id="snowymint-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="snowymint-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="snowymint-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="snowymint-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SnowyMint`

### `OysterBay`

```rust
struct OysterBay;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for OysterBay`

- <span id="oysterbay-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="oysterbay-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="oysterbay-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="oysterbay-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for OysterBay`

### `Red`

```rust
struct Red;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Red`

- <span id="red-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="red-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Red`

### `DarkRose`

```rust
struct DarkRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkRose`

- <span id="darkrose-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkrose-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkrose-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkrose-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkRose`

### `Rose`

```rust
struct Rose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Rose`

- <span id="rose-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rose-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rose-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rose-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Rose`

### `LightHollywoodCerise`

```rust
struct LightHollywoodCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightHollywoodCerise`

- <span id="lighthollywoodcerise-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighthollywoodcerise-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightHollywoodCerise`

### `PurplePizzazz`

```rust
struct PurplePizzazz;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PurplePizzazz`

- <span id="purplepizzazz-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purplepizzazz-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purplepizzazz-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purplepizzazz-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PurplePizzazz`

### `Fuchsia`

```rust
struct Fuchsia;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Fuchsia`

- <span id="fuchsia-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsia-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsia-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsia-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Fuchsia`

### `BlazeOrange`

```rust
struct BlazeOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlazeOrange`

- <span id="blazeorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blazeorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blazeorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blazeorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlazeOrange`

### `BittersweetOrange`

```rust
struct BittersweetOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BittersweetOrange`

- <span id="bittersweetorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bittersweetorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bittersweetorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bittersweetorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BittersweetOrange`

### `WildWatermelon`

```rust
struct WildWatermelon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for WildWatermelon`

- <span id="wildwatermelon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildwatermelon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildwatermelon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildwatermelon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for WildWatermelon`

### `DarkHotPink`

```rust
struct DarkHotPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkHotPink`

- <span id="darkhotpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkhotpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkhotpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkhotpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkHotPink`

### `HotPink`

```rust
struct HotPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HotPink`

- <span id="hotpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hotpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hotpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hotpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HotPink`

### `PinkFlamingo`

```rust
struct PinkFlamingo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PinkFlamingo`

- <span id="pinkflamingo-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinkflamingo-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinkflamingo-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinkflamingo-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PinkFlamingo`

### `FlushOrange`

```rust
struct FlushOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FlushOrange`

- <span id="flushorange-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flushorange-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flushorange-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flushorange-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FlushOrange`

### `Salmon`

```rust
struct Salmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Salmon`

- <span id="salmon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="salmon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="salmon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="salmon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Salmon`

### `VividTangerine`

```rust
struct VividTangerine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for VividTangerine`

- <span id="vividtangerine-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vividtangerine-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vividtangerine-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vividtangerine-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for VividTangerine`

### `PinkSalmon`

```rust
struct PinkSalmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PinkSalmon`

- <span id="pinksalmon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinksalmon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinksalmon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinksalmon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PinkSalmon`

### `DarkLavenderRose`

```rust
struct DarkLavenderRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkLavenderRose`

- <span id="darklavenderrose-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklavenderrose-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklavenderrose-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklavenderrose-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkLavenderRose`

### `BlushPink`

```rust
struct BlushPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlushPink`

- <span id="blushpink-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blushpink-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blushpink-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blushpink-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlushPink`

### `YellowSea`

```rust
struct YellowSea;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for YellowSea`

- <span id="yellowsea-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellowsea-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellowsea-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellowsea-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for YellowSea`

### `TexasRose`

```rust
struct TexasRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TexasRose`

- <span id="texasrose-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="texasrose-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="texasrose-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="texasrose-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TexasRose`

### `Tacao`

```rust
struct Tacao;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Tacao`

- <span id="tacao-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tacao-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tacao-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tacao-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Tacao`

### `Sundown`

```rust
struct Sundown;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Sundown`

- <span id="sundown-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="sundown-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="sundown-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="sundown-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Sundown`

### `CottonCandy`

```rust
struct CottonCandy;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CottonCandy`

- <span id="cottoncandy-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cottoncandy-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cottoncandy-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cottoncandy-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CottonCandy`

### `LavenderRose`

```rust
struct LavenderRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LavenderRose`

- <span id="lavenderrose-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavenderrose-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavenderrose-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavenderrose-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LavenderRose`

### `Gold`

```rust
struct Gold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Gold`

- <span id="gold-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gold-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gold-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gold-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Gold`

### `Dandelion`

```rust
struct Dandelion;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Dandelion`

- <span id="dandelion-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dandelion-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dandelion-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dandelion-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Dandelion`

### `GrandisCaramel`

```rust
struct GrandisCaramel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GrandisCaramel`

- <span id="grandiscaramel-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="grandiscaramel-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="grandiscaramel-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="grandiscaramel-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GrandisCaramel`

### `Caramel`

```rust
struct Caramel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Caramel`

- <span id="caramel-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caramel-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caramel-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caramel-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Caramel`

### `CosmosSalmon`

```rust
struct CosmosSalmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CosmosSalmon`

- <span id="cosmossalmon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cosmossalmon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cosmossalmon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cosmossalmon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CosmosSalmon`

### `PinkLace`

```rust
struct PinkLace;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PinkLace`

- <span id="pinklace-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinklace-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinklace-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinklace-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PinkLace`

### `Yellow`

```rust
struct Yellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Yellow`

- <span id="yellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Yellow`

### `LaserLemon`

```rust
struct LaserLemon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LaserLemon`

- <span id="laserlemon-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="laserlemon-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="laserlemon-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="laserlemon-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LaserLemon`

### `DollyYellow`

```rust
struct DollyYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DollyYellow`

- <span id="dollyyellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dollyyellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dollyyellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dollyyellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DollyYellow`

### `PortafinoYellow`

```rust
struct PortafinoYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PortafinoYellow`

- <span id="portafinoyellow-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="portafinoyellow-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="portafinoyellow-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="portafinoyellow-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PortafinoYellow`

### `Cumulus`

```rust
struct Cumulus;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Cumulus`

- <span id="cumulus-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cumulus-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cumulus-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cumulus-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Cumulus`

### `White`

```rust
struct White;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for White`

- <span id="white-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="white-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for White`

### `DarkCodGray`

```rust
struct DarkCodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkCodGray`

- <span id="darkcodgray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcodgray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcodgray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcodgray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkCodGray`

### `CodGray`

```rust
struct CodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CodGray`

- <span id="codgray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="codgray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="codgray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="codgray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CodGray`

### `LightCodGray`

```rust
struct LightCodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightCodGray`

- <span id="lightcodgray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcodgray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcodgray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcodgray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightCodGray`

### `DarkMineShaft`

```rust
struct DarkMineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMineShaft`

- <span id="darkmineshaft-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmineshaft-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmineshaft-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmineshaft-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMineShaft`

### `MineShaft`

```rust
struct MineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MineShaft`

- <span id="mineshaft-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mineshaft-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mineshaft-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mineshaft-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MineShaft`

### `LightMineShaft`

```rust
struct LightMineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightMineShaft`

- <span id="lightmineshaft-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmineshaft-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmineshaft-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmineshaft-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightMineShaft`

### `DarkTundora`

```rust
struct DarkTundora;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkTundora`

- <span id="darktundora-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktundora-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktundora-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktundora-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkTundora`

### `Tundora`

```rust
struct Tundora;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Tundora`

- <span id="tundora-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tundora-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tundora-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tundora-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Tundora`

### `ScorpionGray`

```rust
struct ScorpionGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScorpionGray`

- <span id="scorpiongray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpiongray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpiongray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpiongray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScorpionGray`

### `DarkDoveGray`

```rust
struct DarkDoveGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkDoveGray`

- <span id="darkdovegray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkdovegray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkdovegray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkdovegray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkDoveGray`

### `DoveGray`

```rust
struct DoveGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DoveGray`

- <span id="dovegray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dovegray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dovegray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dovegray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DoveGray`

### `Boulder`

```rust
struct Boulder;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Boulder`

- <span id="boulder-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="boulder-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="boulder-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="boulder-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Boulder`

### `Gray`

```rust
struct Gray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Gray`

- <span id="gray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Gray`

### `LightGray`

```rust
struct LightGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightGray`

- <span id="lightgray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightgray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightgray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightgray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightGray`

### `DustyGray`

```rust
struct DustyGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DustyGray`

- <span id="dustygray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dustygray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dustygray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dustygray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DustyGray`

### `NobelGray`

```rust
struct NobelGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for NobelGray`

- <span id="nobelgray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="nobelgray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="nobelgray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="nobelgray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for NobelGray`

### `DarkSilverChalice`

```rust
struct DarkSilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkSilverChalice`

- <span id="darksilverchalice-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilverchalice-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilverchalice-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilverchalice-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkSilverChalice`

### `LightSilverChalice`

```rust
struct LightSilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightSilverChalice`

- <span id="lightsilverchalice-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightsilverchalice-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightsilverchalice-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightsilverchalice-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightSilverChalice`

### `DarkSilver`

```rust
struct DarkSilver;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkSilver`

- <span id="darksilver-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilver-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilver-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilver-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkSilver`

### `Silver`

```rust
struct Silver;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Silver`

- <span id="silver-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silver-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silver-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silver-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Silver`

### `DarkAlto`

```rust
struct DarkAlto;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkAlto`

- <span id="darkalto-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkalto-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkalto-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkalto-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkAlto`

### `Alto`

```rust
struct Alto;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Alto`

- <span id="alto-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="alto-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="alto-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="alto-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Alto`

### `Mercury`

```rust
struct Mercury;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Mercury`

- <span id="mercury-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mercury-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mercury-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mercury-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Mercury`

### `GalleryGray`

```rust
struct GalleryGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GalleryGray`

- <span id="gallerygray-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gallerygray-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gallerygray-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gallerygray-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GalleryGray`

## Macros

### `xterm_colors!`

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:1-121`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/xterm.rs#L1-L121)*

