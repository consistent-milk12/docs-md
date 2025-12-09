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

#### Trait Implementations

##### `impl Color for UserBlack`

- <span id="userblack-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblack-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblack-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblack-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBlack`

### `UserRed`

```rust
struct UserRed;
```

#### Trait Implementations

##### `impl Color for UserRed`

- <span id="userred-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userred-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userred-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userred-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserRed`

### `UserGreen`

```rust
struct UserGreen;
```

#### Trait Implementations

##### `impl Color for UserGreen`

- <span id="usergreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usergreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usergreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usergreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserGreen`

### `UserYellow`

```rust
struct UserYellow;
```

#### Trait Implementations

##### `impl Color for UserYellow`

- <span id="useryellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="useryellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="useryellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="useryellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserYellow`

### `UserBlue`

```rust
struct UserBlue;
```

#### Trait Implementations

##### `impl Color for UserBlue`

- <span id="userblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBlue`

### `UserMagenta`

```rust
struct UserMagenta;
```

#### Trait Implementations

##### `impl Color for UserMagenta`

- <span id="usermagenta-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usermagenta-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usermagenta-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usermagenta-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserMagenta`

### `UserCyan`

```rust
struct UserCyan;
```

#### Trait Implementations

##### `impl Color for UserCyan`

- <span id="usercyan-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usercyan-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usercyan-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usercyan-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserCyan`

### `UserWhite`

```rust
struct UserWhite;
```

#### Trait Implementations

##### `impl Color for UserWhite`

- <span id="userwhite-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userwhite-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userwhite-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userwhite-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserWhite`

### `UserBrightBlack`

```rust
struct UserBrightBlack;
```

#### Trait Implementations

##### `impl Color for UserBrightBlack`

- <span id="userbrightblack-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblack-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblack-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblack-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightBlack`

### `UserBrightRed`

```rust
struct UserBrightRed;
```

#### Trait Implementations

##### `impl Color for UserBrightRed`

- <span id="userbrightred-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightred-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightred-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightred-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightRed`

### `UserBrightGreen`

```rust
struct UserBrightGreen;
```

#### Trait Implementations

##### `impl Color for UserBrightGreen`

- <span id="userbrightgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightGreen`

### `UserBrightYellow`

```rust
struct UserBrightYellow;
```

#### Trait Implementations

##### `impl Color for UserBrightYellow`

- <span id="userbrightyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightYellow`

### `UserBrightBlue`

```rust
struct UserBrightBlue;
```

#### Trait Implementations

##### `impl Color for UserBrightBlue`

- <span id="userbrightblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightBlue`

### `UserBrightMagenta`

```rust
struct UserBrightMagenta;
```

#### Trait Implementations

##### `impl Color for UserBrightMagenta`

- <span id="userbrightmagenta-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightmagenta-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightmagenta-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightmagenta-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightMagenta`

### `UserBrightCyan`

```rust
struct UserBrightCyan;
```

#### Trait Implementations

##### `impl Color for UserBrightCyan`

- <span id="userbrightcyan-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightcyan-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightcyan-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightcyan-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightCyan`

### `UserBrightWhite`

```rust
struct UserBrightWhite;
```

#### Trait Implementations

##### `impl Color for UserBrightWhite`

- <span id="userbrightwhite-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightwhite-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightwhite-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightwhite-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for UserBrightWhite`

### `Black`

```rust
struct Black;
```

#### Trait Implementations

##### `impl Color for Black`

- <span id="black-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="black-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Black`

### `StratosBlue`

```rust
struct StratosBlue;
```

#### Trait Implementations

##### `impl Color for StratosBlue`

- <span id="stratosblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="stratosblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="stratosblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="stratosblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for StratosBlue`

### `NavyBlue`

```rust
struct NavyBlue;
```

#### Trait Implementations

##### `impl Color for NavyBlue`

- <span id="navyblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="navyblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="navyblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="navyblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for NavyBlue`

### `MidnightBlue`

```rust
struct MidnightBlue;
```

#### Trait Implementations

##### `impl Color for MidnightBlue`

- <span id="midnightblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="midnightblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="midnightblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="midnightblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MidnightBlue`

### `DarkBlue`

```rust
struct DarkBlue;
```

#### Trait Implementations

##### `impl Color for DarkBlue`

- <span id="darkblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkBlue`

### `Blue`

```rust
struct Blue;
```

#### Trait Implementations

##### `impl Color for Blue`

- <span id="blue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Blue`

### `CamaroneGreen`

```rust
struct CamaroneGreen;
```

#### Trait Implementations

##### `impl Color for CamaroneGreen`

- <span id="camaronegreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="camaronegreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="camaronegreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="camaronegreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CamaroneGreen`

### `BlueStone`

```rust
struct BlueStone;
```

#### Trait Implementations

##### `impl Color for BlueStone`

- <span id="bluestone-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bluestone-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bluestone-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bluestone-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BlueStone`

### `OrientBlue`

```rust
struct OrientBlue;
```

#### Trait Implementations

##### `impl Color for OrientBlue`

- <span id="orientblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orientblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orientblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orientblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for OrientBlue`

### `EndeavourBlue`

```rust
struct EndeavourBlue;
```

#### Trait Implementations

##### `impl Color for EndeavourBlue`

- <span id="endeavourblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="endeavourblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="endeavourblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="endeavourblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for EndeavourBlue`

### `ScienceBlue`

```rust
struct ScienceBlue;
```

#### Trait Implementations

##### `impl Color for ScienceBlue`

- <span id="scienceblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scienceblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scienceblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scienceblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ScienceBlue`

### `BlueRibbon`

```rust
struct BlueRibbon;
```

#### Trait Implementations

##### `impl Color for BlueRibbon`

- <span id="blueribbon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blueribbon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blueribbon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blueribbon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BlueRibbon`

### `JapaneseLaurel`

```rust
struct JapaneseLaurel;
```

#### Trait Implementations

##### `impl Color for JapaneseLaurel`

- <span id="japaneselaurel-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="japaneselaurel-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="japaneselaurel-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="japaneselaurel-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for JapaneseLaurel`

### `DeepSeaGreen`

```rust
struct DeepSeaGreen;
```

#### Trait Implementations

##### `impl Color for DeepSeaGreen`

- <span id="deepseagreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepseagreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepseagreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepseagreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DeepSeaGreen`

### `Teal`

```rust
struct Teal;
```

#### Trait Implementations

##### `impl Color for Teal`

- <span id="teal-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="teal-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="teal-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="teal-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Teal`

### `DeepCerulean`

```rust
struct DeepCerulean;
```

#### Trait Implementations

##### `impl Color for DeepCerulean`

- <span id="deepcerulean-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepcerulean-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepcerulean-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepcerulean-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DeepCerulean`

### `LochmaraBlue`

```rust
struct LochmaraBlue;
```

#### Trait Implementations

##### `impl Color for LochmaraBlue`

- <span id="lochmarablue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lochmarablue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lochmarablue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lochmarablue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LochmaraBlue`

### `AzureRadiance`

```rust
struct AzureRadiance;
```

#### Trait Implementations

##### `impl Color for AzureRadiance`

- <span id="azureradiance-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="azureradiance-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="azureradiance-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="azureradiance-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for AzureRadiance`

### `LightJapaneseLaurel`

```rust
struct LightJapaneseLaurel;
```

#### Trait Implementations

##### `impl Color for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightjapaneselaurel-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightJapaneseLaurel`

### `Jade`

```rust
struct Jade;
```

#### Trait Implementations

##### `impl Color for Jade`

- <span id="jade-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="jade-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="jade-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="jade-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Jade`

### `PersianGreen`

```rust
struct PersianGreen;
```

#### Trait Implementations

##### `impl Color for PersianGreen`

- <span id="persiangreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="persiangreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="persiangreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="persiangreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PersianGreen`

### `BondiBlue`

```rust
struct BondiBlue;
```

#### Trait Implementations

##### `impl Color for BondiBlue`

- <span id="bondiblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bondiblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bondiblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bondiblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BondiBlue`

### `Cerulean`

```rust
struct Cerulean;
```

#### Trait Implementations

##### `impl Color for Cerulean`

- <span id="cerulean-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cerulean-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cerulean-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cerulean-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Cerulean`

### `LightAzureRadiance`

```rust
struct LightAzureRadiance;
```

#### Trait Implementations

##### `impl Color for LightAzureRadiance`

- <span id="lightazureradiance-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightazureradiance-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightazureradiance-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightazureradiance-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightAzureRadiance`

### `DarkGreen`

```rust
struct DarkGreen;
```

#### Trait Implementations

##### `impl Color for DarkGreen`

- <span id="darkgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkGreen`

### `Malachite`

```rust
struct Malachite;
```

#### Trait Implementations

##### `impl Color for Malachite`

- <span id="malachite-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malachite-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malachite-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malachite-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Malachite`

### `CaribbeanGreen`

```rust
struct CaribbeanGreen;
```

#### Trait Implementations

##### `impl Color for CaribbeanGreen`

- <span id="caribbeangreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caribbeangreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caribbeangreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caribbeangreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CaribbeanGreen`

### `LightCaribbeanGreen`

```rust
struct LightCaribbeanGreen;
```

#### Trait Implementations

##### `impl Color for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcaribbeangreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightCaribbeanGreen`

### `RobinEggBlue`

```rust
struct RobinEggBlue;
```

#### Trait Implementations

##### `impl Color for RobinEggBlue`

- <span id="robineggblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="robineggblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="robineggblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="robineggblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for RobinEggBlue`

### `Aqua`

```rust
struct Aqua;
```

#### Trait Implementations

##### `impl Color for Aqua`

- <span id="aqua-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aqua-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aqua-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aqua-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Aqua`

### `Green`

```rust
struct Green;
```

#### Trait Implementations

##### `impl Color for Green`

- <span id="green-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="green-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Green`

### `DarkSpringGreen`

```rust
struct DarkSpringGreen;
```

#### Trait Implementations

##### `impl Color for DarkSpringGreen`

- <span id="darkspringgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkspringgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkspringgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkspringgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkSpringGreen`

### `SpringGreen`

```rust
struct SpringGreen;
```

#### Trait Implementations

##### `impl Color for SpringGreen`

- <span id="springgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="springgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="springgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="springgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for SpringGreen`

### `LightSpringGreen`

```rust
struct LightSpringGreen;
```

#### Trait Implementations

##### `impl Color for LightSpringGreen`

- <span id="lightspringgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightspringgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightspringgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightspringgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightSpringGreen`

### `BrightTurquoise`

```rust
struct BrightTurquoise;
```

#### Trait Implementations

##### `impl Color for BrightTurquoise`

- <span id="brightturquoise-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightturquoise-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightturquoise-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightturquoise-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BrightTurquoise`

### `Cyan`

```rust
struct Cyan;
```

#### Trait Implementations

##### `impl Color for Cyan`

- <span id="cyan-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cyan-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Cyan`

### `Rosewood`

```rust
struct Rosewood;
```

#### Trait Implementations

##### `impl Color for Rosewood`

- <span id="rosewood-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rosewood-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rosewood-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rosewood-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Rosewood`

### `PompadourMagenta`

```rust
struct PompadourMagenta;
```

#### Trait Implementations

##### `impl Color for PompadourMagenta`

- <span id="pompadourmagenta-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pompadourmagenta-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pompadourmagenta-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pompadourmagenta-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PompadourMagenta`

### `PigmentIndigo`

```rust
struct PigmentIndigo;
```

#### Trait Implementations

##### `impl Color for PigmentIndigo`

- <span id="pigmentindigo-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pigmentindigo-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pigmentindigo-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pigmentindigo-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PigmentIndigo`

### `DarkPurple`

```rust
struct DarkPurple;
```

#### Trait Implementations

##### `impl Color for DarkPurple`

- <span id="darkpurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkPurple`

### `ElectricIndigo`

```rust
struct ElectricIndigo;
```

#### Trait Implementations

##### `impl Color for ElectricIndigo`

- <span id="electricindigo-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricindigo-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricindigo-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricindigo-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ElectricIndigo`

### `ElectricPurple`

```rust
struct ElectricPurple;
```

#### Trait Implementations

##### `impl Color for ElectricPurple`

- <span id="electricpurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricpurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricpurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricpurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ElectricPurple`

### `VerdunGreen`

```rust
struct VerdunGreen;
```

#### Trait Implementations

##### `impl Color for VerdunGreen`

- <span id="verdungreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="verdungreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="verdungreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="verdungreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for VerdunGreen`

### `ScorpionOlive`

```rust
struct ScorpionOlive;
```

#### Trait Implementations

##### `impl Color for ScorpionOlive`

- <span id="scorpionolive-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpionolive-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpionolive-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpionolive-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ScorpionOlive`

### `Lilac`

```rust
struct Lilac;
```

#### Trait Implementations

##### `impl Color for Lilac`

- <span id="lilac-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lilac-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lilac-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lilac-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Lilac`

### `ScampiIndigo`

```rust
struct ScampiIndigo;
```

#### Trait Implementations

##### `impl Color for ScampiIndigo`

- <span id="scampiindigo-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scampiindigo-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scampiindigo-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scampiindigo-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ScampiIndigo`

### `Indigo`

```rust
struct Indigo;
```

#### Trait Implementations

##### `impl Color for Indigo`

- <span id="indigo-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="indigo-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="indigo-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="indigo-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Indigo`

### `DarkCornflowerBlue`

```rust
struct DarkCornflowerBlue;
```

#### Trait Implementations

##### `impl Color for DarkCornflowerBlue`

- <span id="darkcornflowerblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcornflowerblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcornflowerblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcornflowerblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkCornflowerBlue`

### `DarkLimeade`

```rust
struct DarkLimeade;
```

#### Trait Implementations

##### `impl Color for DarkLimeade`

- <span id="darklimeade-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklimeade-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklimeade-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklimeade-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkLimeade`

### `GladeGreen`

```rust
struct GladeGreen;
```

#### Trait Implementations

##### `impl Color for GladeGreen`

- <span id="gladegreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gladegreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gladegreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gladegreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for GladeGreen`

### `JuniperGreen`

```rust
struct JuniperGreen;
```

#### Trait Implementations

##### `impl Color for JuniperGreen`

- <span id="junipergreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junipergreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junipergreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junipergreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for JuniperGreen`

### `HippieBlue`

```rust
struct HippieBlue;
```

#### Trait Implementations

##### `impl Color for HippieBlue`

- <span id="hippieblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hippieblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hippieblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hippieblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for HippieBlue`

### `HavelockBlue`

```rust
struct HavelockBlue;
```

#### Trait Implementations

##### `impl Color for HavelockBlue`

- <span id="havelockblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="havelockblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="havelockblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="havelockblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for HavelockBlue`

### `CornflowerBlue`

```rust
struct CornflowerBlue;
```

#### Trait Implementations

##### `impl Color for CornflowerBlue`

- <span id="cornflowerblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cornflowerblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cornflowerblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cornflowerblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CornflowerBlue`

### `Limeade`

```rust
struct Limeade;
```

#### Trait Implementations

##### `impl Color for Limeade`

- <span id="limeade-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="limeade-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="limeade-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="limeade-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Limeade`

### `FernGreen`

```rust
struct FernGreen;
```

#### Trait Implementations

##### `impl Color for FernGreen`

- <span id="ferngreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="ferngreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="ferngreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="ferngreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for FernGreen`

### `SilverTree`

```rust
struct SilverTree;
```

#### Trait Implementations

##### `impl Color for SilverTree`

- <span id="silvertree-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silvertree-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silvertree-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silvertree-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for SilverTree`

### `Tradewind`

```rust
struct Tradewind;
```

#### Trait Implementations

##### `impl Color for Tradewind`

- <span id="tradewind-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tradewind-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tradewind-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tradewind-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Tradewind`

### `ShakespeareBlue`

```rust
struct ShakespeareBlue;
```

#### Trait Implementations

##### `impl Color for ShakespeareBlue`

- <span id="shakespeareblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="shakespeareblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="shakespeareblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="shakespeareblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ShakespeareBlue`

### `DarkMalibuBlue`

```rust
struct DarkMalibuBlue;
```

#### Trait Implementations

##### `impl Color for DarkMalibuBlue`

- <span id="darkmalibublue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmalibublue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmalibublue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmalibublue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkMalibuBlue`

### `DarkBrightGreen`

```rust
struct DarkBrightGreen;
```

#### Trait Implementations

##### `impl Color for DarkBrightGreen`

- <span id="darkbrightgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkbrightgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkbrightgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkbrightgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkBrightGreen`

### `DarkPastelGreen`

```rust
struct DarkPastelGreen;
```

#### Trait Implementations

##### `impl Color for DarkPastelGreen`

- <span id="darkpastelgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpastelgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpastelgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpastelgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkPastelGreen`

### `PastelGreen`

```rust
struct PastelGreen;
```

#### Trait Implementations

##### `impl Color for PastelGreen`

- <span id="pastelgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pastelgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pastelgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pastelgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PastelGreen`

### `DownyTeal`

```rust
struct DownyTeal;
```

#### Trait Implementations

##### `impl Color for DownyTeal`

- <span id="downyteal-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="downyteal-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="downyteal-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="downyteal-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DownyTeal`

### `Viking`

```rust
struct Viking;
```

#### Trait Implementations

##### `impl Color for Viking`

- <span id="viking-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="viking-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="viking-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="viking-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Viking`

### `MalibuBlue`

```rust
struct MalibuBlue;
```

#### Trait Implementations

##### `impl Color for MalibuBlue`

- <span id="malibublue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malibublue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malibublue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malibublue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MalibuBlue`

### `BrightGreen`

```rust
struct BrightGreen;
```

#### Trait Implementations

##### `impl Color for BrightGreen`

- <span id="brightgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BrightGreen`

### `DarkScreaminGreen`

```rust
struct DarkScreaminGreen;
```

#### Trait Implementations

##### `impl Color for DarkScreaminGreen`

- <span id="darkscreamingreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkscreamingreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkscreamingreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkscreamingreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkScreaminGreen`

### `ScreaminGreen`

```rust
struct ScreaminGreen;
```

#### Trait Implementations

##### `impl Color for ScreaminGreen`

- <span id="screamingreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="screamingreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="screamingreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="screamingreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ScreaminGreen`

### `DarkAquamarine`

```rust
struct DarkAquamarine;
```

#### Trait Implementations

##### `impl Color for DarkAquamarine`

- <span id="darkaquamarine-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkaquamarine-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkaquamarine-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkaquamarine-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkAquamarine`

### `Aquamarine`

```rust
struct Aquamarine;
```

#### Trait Implementations

##### `impl Color for Aquamarine`

- <span id="aquamarine-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aquamarine-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aquamarine-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aquamarine-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Aquamarine`

### `LightAquamarine`

```rust
struct LightAquamarine;
```

#### Trait Implementations

##### `impl Color for LightAquamarine`

- <span id="lightaquamarine-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightaquamarine-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightaquamarine-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightaquamarine-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightAquamarine`

### `Maroon`

```rust
struct Maroon;
```

#### Trait Implementations

##### `impl Color for Maroon`

- <span id="maroon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="maroon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="maroon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="maroon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Maroon`

### `DarkFreshEggplant`

```rust
struct DarkFreshEggplant;
```

#### Trait Implementations

##### `impl Color for DarkFreshEggplant`

- <span id="darkfresheggplant-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfresheggplant-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfresheggplant-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfresheggplant-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkFreshEggplant`

### `LightFreshEggplant`

```rust
struct LightFreshEggplant;
```

#### Trait Implementations

##### `impl Color for LightFreshEggplant`

- <span id="lightfresheggplant-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightfresheggplant-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightfresheggplant-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightfresheggplant-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightFreshEggplant`

### `Purple`

```rust
struct Purple;
```

#### Trait Implementations

##### `impl Color for Purple`

- <span id="purple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Purple`

### `ElectricViolet`

```rust
struct ElectricViolet;
```

#### Trait Implementations

##### `impl Color for ElectricViolet`

- <span id="electricviolet-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricviolet-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricviolet-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricviolet-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ElectricViolet`

### `LightElectricViolet`

```rust
struct LightElectricViolet;
```

#### Trait Implementations

##### `impl Color for LightElectricViolet`

- <span id="lightelectricviolet-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightelectricviolet-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightelectricviolet-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightelectricviolet-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightElectricViolet`

### `Brown`

```rust
struct Brown;
```

#### Trait Implementations

##### `impl Color for Brown`

- <span id="brown-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brown-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brown-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brown-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Brown`

### `CopperRose`

```rust
struct CopperRose;
```

#### Trait Implementations

##### `impl Color for CopperRose`

- <span id="copperrose-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperrose-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperrose-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperrose-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CopperRose`

### `StrikemasterPurple`

```rust
struct StrikemasterPurple;
```

#### Trait Implementations

##### `impl Color for StrikemasterPurple`

- <span id="strikemasterpurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="strikemasterpurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="strikemasterpurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="strikemasterpurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for StrikemasterPurple`

### `DelugePurple`

```rust
struct DelugePurple;
```

#### Trait Implementations

##### `impl Color for DelugePurple`

- <span id="delugepurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="delugepurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="delugepurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="delugepurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DelugePurple`

### `DarkMediumPurple`

```rust
struct DarkMediumPurple;
```

#### Trait Implementations

##### `impl Color for DarkMediumPurple`

- <span id="darkmediumpurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmediumpurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmediumpurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmediumpurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkMediumPurple`

### `DarkHeliotropePurple`

```rust
struct DarkHeliotropePurple;
```

#### Trait Implementations

##### `impl Color for DarkHeliotropePurple`

- <span id="darkheliotropepurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkheliotropepurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkheliotropepurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkheliotropepurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkHeliotropePurple`

### `Olive`

```rust
struct Olive;
```

#### Trait Implementations

##### `impl Color for Olive`

- <span id="olive-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olive-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olive-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olive-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Olive`

### `ClayCreekOlive`

```rust
struct ClayCreekOlive;
```

#### Trait Implementations

##### `impl Color for ClayCreekOlive`

- <span id="claycreekolive-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="claycreekolive-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="claycreekolive-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="claycreekolive-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ClayCreekOlive`

### `DarkGray`

```rust
struct DarkGray;
```

#### Trait Implementations

##### `impl Color for DarkGray`

- <span id="darkgray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkGray`

### `WildBlueYonder`

```rust
struct WildBlueYonder;
```

#### Trait Implementations

##### `impl Color for WildBlueYonder`

- <span id="wildblueyonder-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildblueyonder-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildblueyonder-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildblueyonder-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for WildBlueYonder`

### `ChetwodeBlue`

```rust
struct ChetwodeBlue;
```

#### Trait Implementations

##### `impl Color for ChetwodeBlue`

- <span id="chetwodeblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chetwodeblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chetwodeblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chetwodeblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ChetwodeBlue`

### `SlateBlue`

```rust
struct SlateBlue;
```

#### Trait Implementations

##### `impl Color for SlateBlue`

- <span id="slateblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="slateblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="slateblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="slateblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for SlateBlue`

### `LightLimeade`

```rust
struct LightLimeade;
```

#### Trait Implementations

##### `impl Color for LightLimeade`

- <span id="lightlimeade-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightlimeade-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightlimeade-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightlimeade-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightLimeade`

### `ChelseaCucumber`

```rust
struct ChelseaCucumber;
```

#### Trait Implementations

##### `impl Color for ChelseaCucumber`

- <span id="chelseacucumber-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chelseacucumber-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chelseacucumber-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chelseacucumber-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ChelseaCucumber`

### `BayLeaf`

```rust
struct BayLeaf;
```

#### Trait Implementations

##### `impl Color for BayLeaf`

- <span id="bayleaf-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bayleaf-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bayleaf-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bayleaf-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BayLeaf`

### `GulfStream`

```rust
struct GulfStream;
```

#### Trait Implementations

##### `impl Color for GulfStream`

- <span id="gulfstream-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gulfstream-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gulfstream-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gulfstream-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for GulfStream`

### `PoloBlue`

```rust
struct PoloBlue;
```

#### Trait Implementations

##### `impl Color for PoloBlue`

- <span id="poloblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="poloblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="poloblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="poloblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PoloBlue`

### `LightMalibuBlue`

```rust
struct LightMalibuBlue;
```

#### Trait Implementations

##### `impl Color for LightMalibuBlue`

- <span id="lightmalibublue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmalibublue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmalibublue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmalibublue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightMalibuBlue`

### `Pistachio`

```rust
struct Pistachio;
```

#### Trait Implementations

##### `impl Color for Pistachio`

- <span id="pistachio-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pistachio-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pistachio-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pistachio-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Pistachio`

### `LightPastelGreen`

```rust
struct LightPastelGreen;
```

#### Trait Implementations

##### `impl Color for LightPastelGreen`

- <span id="lightpastelgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightpastelgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightpastelgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightpastelgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightPastelGreen`

### `DarkFeijoaGreen`

```rust
struct DarkFeijoaGreen;
```

#### Trait Implementations

##### `impl Color for DarkFeijoaGreen`

- <span id="darkfeijoagreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfeijoagreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfeijoagreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfeijoagreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkFeijoaGreen`

### `VistaBlue`

```rust
struct VistaBlue;
```

#### Trait Implementations

##### `impl Color for VistaBlue`

- <span id="vistablue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vistablue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vistablue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vistablue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for VistaBlue`

### `Bermuda`

```rust
struct Bermuda;
```

#### Trait Implementations

##### `impl Color for Bermuda`

- <span id="bermuda-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bermuda-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bermuda-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bermuda-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Bermuda`

### `DarkAnakiwaBlue`

```rust
struct DarkAnakiwaBlue;
```

#### Trait Implementations

##### `impl Color for DarkAnakiwaBlue`

- <span id="darkanakiwablue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkanakiwablue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkanakiwablue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkanakiwablue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkAnakiwaBlue`

### `ChartreuseGreen`

```rust
struct ChartreuseGreen;
```

#### Trait Implementations

##### `impl Color for ChartreuseGreen`

- <span id="chartreusegreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreusegreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreusegreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreusegreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ChartreuseGreen`

### `LightScreaminGreen`

```rust
struct LightScreaminGreen;
```

#### Trait Implementations

##### `impl Color for LightScreaminGreen`

- <span id="lightscreamingreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightscreamingreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightscreamingreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightscreamingreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightScreaminGreen`

### `DarkMintGreen`

```rust
struct DarkMintGreen;
```

#### Trait Implementations

##### `impl Color for DarkMintGreen`

- <span id="darkmintgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmintgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmintgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmintgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkMintGreen`

### `MintGreen`

```rust
struct MintGreen;
```

#### Trait Implementations

##### `impl Color for MintGreen`

- <span id="mintgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mintgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mintgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mintgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MintGreen`

### `LighterAquamarine`

```rust
struct LighterAquamarine;
```

#### Trait Implementations

##### `impl Color for LighterAquamarine`

- <span id="lighteraquamarine-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighteraquamarine-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighteraquamarine-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighteraquamarine-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LighterAquamarine`

### `AnakiwaBlue`

```rust
struct AnakiwaBlue;
```

#### Trait Implementations

##### `impl Color for AnakiwaBlue`

- <span id="anakiwablue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="anakiwablue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="anakiwablue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="anakiwablue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for AnakiwaBlue`

### `BrightRed`

```rust
struct BrightRed;
```

#### Trait Implementations

##### `impl Color for BrightRed`

- <span id="brightred-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightred-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BrightRed`

### `DarkFlirt`

```rust
struct DarkFlirt;
```

#### Trait Implementations

##### `impl Color for DarkFlirt`

- <span id="darkflirt-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkflirt-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkflirt-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkflirt-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkFlirt`

### `Flirt`

```rust
struct Flirt;
```

#### Trait Implementations

##### `impl Color for Flirt`

- <span id="flirt-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flirt-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flirt-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flirt-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Flirt`

### `LightFlirt`

```rust
struct LightFlirt;
```

#### Trait Implementations

##### `impl Color for LightFlirt`

- <span id="lightflirt-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightflirt-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightflirt-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightflirt-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightFlirt`

### `DarkViolet`

```rust
struct DarkViolet;
```

#### Trait Implementations

##### `impl Color for DarkViolet`

- <span id="darkviolet-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkviolet-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkviolet-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkviolet-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkViolet`

### `BrightElectricViolet`

```rust
struct BrightElectricViolet;
```

#### Trait Implementations

##### `impl Color for BrightElectricViolet`

- <span id="brightelectricviolet-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightelectricviolet-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightelectricviolet-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightelectricviolet-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BrightElectricViolet`

### `RoseofSharonOrange`

```rust
struct RoseofSharonOrange;
```

#### Trait Implementations

##### `impl Color for RoseofSharonOrange`

- <span id="roseofsharonorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="roseofsharonorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="roseofsharonorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="roseofsharonorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for RoseofSharonOrange`

### `MatrixPink`

```rust
struct MatrixPink;
```

#### Trait Implementations

##### `impl Color for MatrixPink`

- <span id="matrixpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="matrixpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="matrixpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="matrixpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MatrixPink`

### `TapestryPink`

```rust
struct TapestryPink;
```

#### Trait Implementations

##### `impl Color for TapestryPink`

- <span id="tapestrypink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tapestrypink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tapestrypink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tapestrypink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for TapestryPink`

### `FuchsiaPink`

```rust
struct FuchsiaPink;
```

#### Trait Implementations

##### `impl Color for FuchsiaPink`

- <span id="fuchsiapink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsiapink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsiapink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsiapink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for FuchsiaPink`

### `MediumPurple`

```rust
struct MediumPurple;
```

#### Trait Implementations

##### `impl Color for MediumPurple`

- <span id="mediumpurple-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumpurple-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumpurple-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumpurple-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MediumPurple`

### `Heliotrope`

```rust
struct Heliotrope;
```

#### Trait Implementations

##### `impl Color for Heliotrope`

- <span id="heliotrope-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="heliotrope-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="heliotrope-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="heliotrope-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Heliotrope`

### `PirateGold`

```rust
struct PirateGold;
```

#### Trait Implementations

##### `impl Color for PirateGold`

- <span id="pirategold-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pirategold-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pirategold-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pirategold-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PirateGold`

### `MuesliOrange`

```rust
struct MuesliOrange;
```

#### Trait Implementations

##### `impl Color for MuesliOrange`

- <span id="muesliorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="muesliorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="muesliorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="muesliorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MuesliOrange`

### `PharlapPink`

```rust
struct PharlapPink;
```

#### Trait Implementations

##### `impl Color for PharlapPink`

- <span id="pharlappink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pharlappink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pharlappink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pharlappink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PharlapPink`

### `Bouquet`

```rust
struct Bouquet;
```

#### Trait Implementations

##### `impl Color for Bouquet`

- <span id="bouquet-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bouquet-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bouquet-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bouquet-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Bouquet`

### `Lavender`

```rust
struct Lavender;
```

#### Trait Implementations

##### `impl Color for Lavender`

- <span id="lavender-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavender-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavender-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavender-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Lavender`

### `LightHeliotrope`

```rust
struct LightHeliotrope;
```

#### Trait Implementations

##### `impl Color for LightHeliotrope`

- <span id="lightheliotrope-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightheliotrope-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightheliotrope-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightheliotrope-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightHeliotrope`

### `BuddhaGold`

```rust
struct BuddhaGold;
```

#### Trait Implementations

##### `impl Color for BuddhaGold`

- <span id="buddhagold-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="buddhagold-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="buddhagold-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="buddhagold-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BuddhaGold`

### `OliveGreen`

```rust
struct OliveGreen;
```

#### Trait Implementations

##### `impl Color for OliveGreen`

- <span id="olivegreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olivegreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olivegreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olivegreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for OliveGreen`

### `HillaryOlive`

```rust
struct HillaryOlive;
```

#### Trait Implementations

##### `impl Color for HillaryOlive`

- <span id="hillaryolive-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hillaryolive-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hillaryolive-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hillaryolive-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for HillaryOlive`

### `SilverChalice`

```rust
struct SilverChalice;
```

#### Trait Implementations

##### `impl Color for SilverChalice`

- <span id="silverchalice-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silverchalice-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silverchalice-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silverchalice-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for SilverChalice`

### `WistfulLilac`

```rust
struct WistfulLilac;
```

#### Trait Implementations

##### `impl Color for WistfulLilac`

- <span id="wistfullilac-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wistfullilac-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wistfullilac-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wistfullilac-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for WistfulLilac`

### `MelroseLilac`

```rust
struct MelroseLilac;
```

#### Trait Implementations

##### `impl Color for MelroseLilac`

- <span id="melroselilac-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="melroselilac-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="melroselilac-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="melroselilac-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MelroseLilac`

### `RioGrandeGreen`

```rust
struct RioGrandeGreen;
```

#### Trait Implementations

##### `impl Color for RioGrandeGreen`

- <span id="riograndegreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="riograndegreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="riograndegreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="riograndegreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for RioGrandeGreen`

### `ConiferGreen`

```rust
struct ConiferGreen;
```

#### Trait Implementations

##### `impl Color for ConiferGreen`

- <span id="conifergreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="conifergreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="conifergreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="conifergreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ConiferGreen`

### `Feijoa`

```rust
struct Feijoa;
```

#### Trait Implementations

##### `impl Color for Feijoa`

- <span id="feijoa-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="feijoa-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="feijoa-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="feijoa-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Feijoa`

### `PixieGreen`

```rust
struct PixieGreen;
```

#### Trait Implementations

##### `impl Color for PixieGreen`

- <span id="pixiegreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pixiegreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pixiegreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pixiegreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PixieGreen`

### `JungleMist`

```rust
struct JungleMist;
```

#### Trait Implementations

##### `impl Color for JungleMist`

- <span id="junglemist-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junglemist-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junglemist-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junglemist-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for JungleMist`

### `LightAnakiwaBlue`

```rust
struct LightAnakiwaBlue;
```

#### Trait Implementations

##### `impl Color for LightAnakiwaBlue`

- <span id="lightanakiwablue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightanakiwablue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightanakiwablue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightanakiwablue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightAnakiwaBlue`

### `Lime`

```rust
struct Lime;
```

#### Trait Implementations

##### `impl Color for Lime`

- <span id="lime-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lime-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lime-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lime-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Lime`

### `GreenYellow`

```rust
struct GreenYellow;
```

#### Trait Implementations

##### `impl Color for GreenYellow`

- <span id="greenyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="greenyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="greenyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="greenyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for GreenYellow`

### `LightMintGreen`

```rust
struct LightMintGreen;
```

#### Trait Implementations

##### `impl Color for LightMintGreen`

- <span id="lightmintgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmintgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmintgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmintgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightMintGreen`

### `Celadon`

```rust
struct Celadon;
```

#### Trait Implementations

##### `impl Color for Celadon`

- <span id="celadon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="celadon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="celadon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="celadon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Celadon`

### `AeroBlue`

```rust
struct AeroBlue;
```

#### Trait Implementations

##### `impl Color for AeroBlue`

- <span id="aeroblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aeroblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aeroblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aeroblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for AeroBlue`

### `FrenchPassLightBlue`

```rust
struct FrenchPassLightBlue;
```

#### Trait Implementations

##### `impl Color for FrenchPassLightBlue`

- <span id="frenchpasslightblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="frenchpasslightblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="frenchpasslightblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="frenchpasslightblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for FrenchPassLightBlue`

### `GuardsmanRed`

```rust
struct GuardsmanRed;
```

#### Trait Implementations

##### `impl Color for GuardsmanRed`

- <span id="guardsmanred-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="guardsmanred-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="guardsmanred-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="guardsmanred-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for GuardsmanRed`

### `RazzmatazzCerise`

```rust
struct RazzmatazzCerise;
```

#### Trait Implementations

##### `impl Color for RazzmatazzCerise`

- <span id="razzmatazzcerise-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="razzmatazzcerise-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="razzmatazzcerise-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="razzmatazzcerise-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for RazzmatazzCerise`

### `MediumVioletRed`

```rust
struct MediumVioletRed;
```

#### Trait Implementations

##### `impl Color for MediumVioletRed`

- <span id="mediumvioletred-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumvioletred-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumvioletred-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumvioletred-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MediumVioletRed`

### `HollywoodCerise`

```rust
struct HollywoodCerise;
```

#### Trait Implementations

##### `impl Color for HollywoodCerise`

- <span id="hollywoodcerise-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hollywoodcerise-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hollywoodcerise-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hollywoodcerise-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for HollywoodCerise`

### `DarkPurplePizzazz`

```rust
struct DarkPurplePizzazz;
```

#### Trait Implementations

##### `impl Color for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurplepizzazz-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkPurplePizzazz`

### `BrighterElectricViolet`

```rust
struct BrighterElectricViolet;
```

#### Trait Implementations

##### `impl Color for BrighterElectricViolet`

- <span id="brighterelectricviolet-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brighterelectricviolet-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brighterelectricviolet-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brighterelectricviolet-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BrighterElectricViolet`

### `TennOrange`

```rust
struct TennOrange;
```

#### Trait Implementations

##### `impl Color for TennOrange`

- <span id="tennorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tennorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tennorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tennorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for TennOrange`

### `RomanOrange`

```rust
struct RomanOrange;
```

#### Trait Implementations

##### `impl Color for RomanOrange`

- <span id="romanorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="romanorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="romanorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="romanorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for RomanOrange`

### `CranberryPink`

```rust
struct CranberryPink;
```

#### Trait Implementations

##### `impl Color for CranberryPink`

- <span id="cranberrypink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cranberrypink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cranberrypink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cranberrypink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CranberryPink`

### `HopbushPink`

```rust
struct HopbushPink;
```

#### Trait Implementations

##### `impl Color for HopbushPink`

- <span id="hopbushpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hopbushpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hopbushpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hopbushpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for HopbushPink`

### `Orchid`

```rust
struct Orchid;
```

#### Trait Implementations

##### `impl Color for Orchid`

- <span id="orchid-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orchid-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orchid-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orchid-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Orchid`

### `LighterHeliotrope`

```rust
struct LighterHeliotrope;
```

#### Trait Implementations

##### `impl Color for LighterHeliotrope`

- <span id="lighterheliotrope-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighterheliotrope-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighterheliotrope-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighterheliotrope-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LighterHeliotrope`

### `MangoTango`

```rust
struct MangoTango;
```

#### Trait Implementations

##### `impl Color for MangoTango`

- <span id="mangotango-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mangotango-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mangotango-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mangotango-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MangoTango`

### `Copperfield`

```rust
struct Copperfield;
```

#### Trait Implementations

##### `impl Color for Copperfield`

- <span id="copperfield-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperfield-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperfield-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperfield-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Copperfield`

### `SeaPink`

```rust
struct SeaPink;
```

#### Trait Implementations

##### `impl Color for SeaPink`

- <span id="seapink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="seapink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="seapink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="seapink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for SeaPink`

### `CanCanPink`

```rust
struct CanCanPink;
```

#### Trait Implementations

##### `impl Color for CanCanPink`

- <span id="cancanpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cancanpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cancanpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cancanpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CanCanPink`

### `LightOrchid`

```rust
struct LightOrchid;
```

#### Trait Implementations

##### `impl Color for LightOrchid`

- <span id="lightorchid-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightorchid-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightorchid-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightorchid-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightOrchid`

### `BrightHeliotrope`

```rust
struct BrightHeliotrope;
```

#### Trait Implementations

##### `impl Color for BrightHeliotrope`

- <span id="brightheliotrope-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightheliotrope-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightheliotrope-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightheliotrope-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BrightHeliotrope`

### `DarkCorn`

```rust
struct DarkCorn;
```

#### Trait Implementations

##### `impl Color for DarkCorn`

- <span id="darkcorn-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcorn-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcorn-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcorn-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkCorn`

### `DarkTachaOrange`

```rust
struct DarkTachaOrange;
```

#### Trait Implementations

##### `impl Color for DarkTachaOrange`

- <span id="darktachaorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktachaorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktachaorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktachaorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkTachaOrange`

### `TanBeige`

```rust
struct TanBeige;
```

#### Trait Implementations

##### `impl Color for TanBeige`

- <span id="tanbeige-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tanbeige-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tanbeige-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tanbeige-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for TanBeige`

### `ClamShell`

```rust
struct ClamShell;
```

#### Trait Implementations

##### `impl Color for ClamShell`

- <span id="clamshell-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="clamshell-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="clamshell-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="clamshell-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ClamShell`

### `ThistlePink`

```rust
struct ThistlePink;
```

#### Trait Implementations

##### `impl Color for ThistlePink`

- <span id="thistlepink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="thistlepink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="thistlepink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="thistlepink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ThistlePink`

### `Mauve`

```rust
struct Mauve;
```

#### Trait Implementations

##### `impl Color for Mauve`

- <span id="mauve-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mauve-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mauve-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mauve-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Mauve`

### `Corn`

```rust
struct Corn;
```

#### Trait Implementations

##### `impl Color for Corn`

- <span id="corn-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="corn-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="corn-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="corn-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Corn`

### `TachaOrange`

```rust
struct TachaOrange;
```

#### Trait Implementations

##### `impl Color for TachaOrange`

- <span id="tachaorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tachaorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tachaorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tachaorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for TachaOrange`

### `DecoOrange`

```rust
struct DecoOrange;
```

#### Trait Implementations

##### `impl Color for DecoOrange`

- <span id="decoorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="decoorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="decoorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="decoorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DecoOrange`

### `PaleGoldenrod`

```rust
struct PaleGoldenrod;
```

#### Trait Implementations

##### `impl Color for PaleGoldenrod`

- <span id="palegoldenrod-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="palegoldenrod-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="palegoldenrod-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="palegoldenrod-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PaleGoldenrod`

### `AltoBeige`

```rust
struct AltoBeige;
```

#### Trait Implementations

##### `impl Color for AltoBeige`

- <span id="altobeige-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="altobeige-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="altobeige-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="altobeige-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for AltoBeige`

### `FogPink`

```rust
struct FogPink;
```

#### Trait Implementations

##### `impl Color for FogPink`

- <span id="fogpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fogpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fogpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fogpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for FogPink`

### `ChartreuseYellow`

```rust
struct ChartreuseYellow;
```

#### Trait Implementations

##### `impl Color for ChartreuseYellow`

- <span id="chartreuseyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreuseyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreuseyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreuseyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ChartreuseYellow`

### `Canary`

```rust
struct Canary;
```

#### Trait Implementations

##### `impl Color for Canary`

- <span id="canary-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="canary-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="canary-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="canary-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Canary`

### `Honeysuckle`

```rust
struct Honeysuckle;
```

#### Trait Implementations

##### `impl Color for Honeysuckle`

- <span id="honeysuckle-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="honeysuckle-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="honeysuckle-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="honeysuckle-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Honeysuckle`

### `ReefPaleYellow`

```rust
struct ReefPaleYellow;
```

#### Trait Implementations

##### `impl Color for ReefPaleYellow`

- <span id="reefpaleyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="reefpaleyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="reefpaleyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="reefpaleyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ReefPaleYellow`

### `SnowyMint`

```rust
struct SnowyMint;
```

#### Trait Implementations

##### `impl Color for SnowyMint`

- <span id="snowymint-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="snowymint-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="snowymint-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="snowymint-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for SnowyMint`

### `OysterBay`

```rust
struct OysterBay;
```

#### Trait Implementations

##### `impl Color for OysterBay`

- <span id="oysterbay-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="oysterbay-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="oysterbay-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="oysterbay-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for OysterBay`

### `Red`

```rust
struct Red;
```

#### Trait Implementations

##### `impl Color for Red`

- <span id="red-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="red-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Red`

### `DarkRose`

```rust
struct DarkRose;
```

#### Trait Implementations

##### `impl Color for DarkRose`

- <span id="darkrose-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkrose-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkrose-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkrose-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkRose`

### `Rose`

```rust
struct Rose;
```

#### Trait Implementations

##### `impl Color for Rose`

- <span id="rose-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rose-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rose-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rose-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Rose`

### `LightHollywoodCerise`

```rust
struct LightHollywoodCerise;
```

#### Trait Implementations

##### `impl Color for LightHollywoodCerise`

- <span id="lighthollywoodcerise-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighthollywoodcerise-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightHollywoodCerise`

### `PurplePizzazz`

```rust
struct PurplePizzazz;
```

#### Trait Implementations

##### `impl Color for PurplePizzazz`

- <span id="purplepizzazz-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purplepizzazz-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purplepizzazz-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purplepizzazz-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PurplePizzazz`

### `Fuchsia`

```rust
struct Fuchsia;
```

#### Trait Implementations

##### `impl Color for Fuchsia`

- <span id="fuchsia-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsia-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsia-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsia-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Fuchsia`

### `BlazeOrange`

```rust
struct BlazeOrange;
```

#### Trait Implementations

##### `impl Color for BlazeOrange`

- <span id="blazeorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blazeorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blazeorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blazeorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BlazeOrange`

### `BittersweetOrange`

```rust
struct BittersweetOrange;
```

#### Trait Implementations

##### `impl Color for BittersweetOrange`

- <span id="bittersweetorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bittersweetorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bittersweetorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bittersweetorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BittersweetOrange`

### `WildWatermelon`

```rust
struct WildWatermelon;
```

#### Trait Implementations

##### `impl Color for WildWatermelon`

- <span id="wildwatermelon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildwatermelon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildwatermelon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildwatermelon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for WildWatermelon`

### `DarkHotPink`

```rust
struct DarkHotPink;
```

#### Trait Implementations

##### `impl Color for DarkHotPink`

- <span id="darkhotpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkhotpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkhotpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkhotpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkHotPink`

### `HotPink`

```rust
struct HotPink;
```

#### Trait Implementations

##### `impl Color for HotPink`

- <span id="hotpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hotpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hotpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hotpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for HotPink`

### `PinkFlamingo`

```rust
struct PinkFlamingo;
```

#### Trait Implementations

##### `impl Color for PinkFlamingo`

- <span id="pinkflamingo-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinkflamingo-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinkflamingo-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinkflamingo-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PinkFlamingo`

### `FlushOrange`

```rust
struct FlushOrange;
```

#### Trait Implementations

##### `impl Color for FlushOrange`

- <span id="flushorange-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flushorange-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flushorange-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flushorange-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for FlushOrange`

### `Salmon`

```rust
struct Salmon;
```

#### Trait Implementations

##### `impl Color for Salmon`

- <span id="salmon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="salmon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="salmon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="salmon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Salmon`

### `VividTangerine`

```rust
struct VividTangerine;
```

#### Trait Implementations

##### `impl Color for VividTangerine`

- <span id="vividtangerine-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vividtangerine-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vividtangerine-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vividtangerine-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for VividTangerine`

### `PinkSalmon`

```rust
struct PinkSalmon;
```

#### Trait Implementations

##### `impl Color for PinkSalmon`

- <span id="pinksalmon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinksalmon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinksalmon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinksalmon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PinkSalmon`

### `DarkLavenderRose`

```rust
struct DarkLavenderRose;
```

#### Trait Implementations

##### `impl Color for DarkLavenderRose`

- <span id="darklavenderrose-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklavenderrose-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklavenderrose-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklavenderrose-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkLavenderRose`

### `BlushPink`

```rust
struct BlushPink;
```

#### Trait Implementations

##### `impl Color for BlushPink`

- <span id="blushpink-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blushpink-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blushpink-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blushpink-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for BlushPink`

### `YellowSea`

```rust
struct YellowSea;
```

#### Trait Implementations

##### `impl Color for YellowSea`

- <span id="yellowsea-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellowsea-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellowsea-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellowsea-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for YellowSea`

### `TexasRose`

```rust
struct TexasRose;
```

#### Trait Implementations

##### `impl Color for TexasRose`

- <span id="texasrose-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="texasrose-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="texasrose-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="texasrose-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for TexasRose`

### `Tacao`

```rust
struct Tacao;
```

#### Trait Implementations

##### `impl Color for Tacao`

- <span id="tacao-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tacao-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tacao-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tacao-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Tacao`

### `Sundown`

```rust
struct Sundown;
```

#### Trait Implementations

##### `impl Color for Sundown`

- <span id="sundown-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="sundown-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="sundown-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="sundown-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Sundown`

### `CottonCandy`

```rust
struct CottonCandy;
```

#### Trait Implementations

##### `impl Color for CottonCandy`

- <span id="cottoncandy-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cottoncandy-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cottoncandy-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cottoncandy-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CottonCandy`

### `LavenderRose`

```rust
struct LavenderRose;
```

#### Trait Implementations

##### `impl Color for LavenderRose`

- <span id="lavenderrose-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavenderrose-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavenderrose-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavenderrose-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LavenderRose`

### `Gold`

```rust
struct Gold;
```

#### Trait Implementations

##### `impl Color for Gold`

- <span id="gold-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gold-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gold-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gold-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Gold`

### `Dandelion`

```rust
struct Dandelion;
```

#### Trait Implementations

##### `impl Color for Dandelion`

- <span id="dandelion-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dandelion-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dandelion-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dandelion-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Dandelion`

### `GrandisCaramel`

```rust
struct GrandisCaramel;
```

#### Trait Implementations

##### `impl Color for GrandisCaramel`

- <span id="grandiscaramel-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="grandiscaramel-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="grandiscaramel-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="grandiscaramel-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for GrandisCaramel`

### `Caramel`

```rust
struct Caramel;
```

#### Trait Implementations

##### `impl Color for Caramel`

- <span id="caramel-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caramel-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caramel-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caramel-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Caramel`

### `CosmosSalmon`

```rust
struct CosmosSalmon;
```

#### Trait Implementations

##### `impl Color for CosmosSalmon`

- <span id="cosmossalmon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cosmossalmon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cosmossalmon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cosmossalmon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CosmosSalmon`

### `PinkLace`

```rust
struct PinkLace;
```

#### Trait Implementations

##### `impl Color for PinkLace`

- <span id="pinklace-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinklace-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinklace-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinklace-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PinkLace`

### `Yellow`

```rust
struct Yellow;
```

#### Trait Implementations

##### `impl Color for Yellow`

- <span id="yellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Yellow`

### `LaserLemon`

```rust
struct LaserLemon;
```

#### Trait Implementations

##### `impl Color for LaserLemon`

- <span id="laserlemon-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="laserlemon-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="laserlemon-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="laserlemon-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LaserLemon`

### `DollyYellow`

```rust
struct DollyYellow;
```

#### Trait Implementations

##### `impl Color for DollyYellow`

- <span id="dollyyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dollyyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dollyyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dollyyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DollyYellow`

### `PortafinoYellow`

```rust
struct PortafinoYellow;
```

#### Trait Implementations

##### `impl Color for PortafinoYellow`

- <span id="portafinoyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="portafinoyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="portafinoyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="portafinoyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for PortafinoYellow`

### `Cumulus`

```rust
struct Cumulus;
```

#### Trait Implementations

##### `impl Color for Cumulus`

- <span id="cumulus-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cumulus-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cumulus-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cumulus-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Cumulus`

### `White`

```rust
struct White;
```

#### Trait Implementations

##### `impl Color for White`

- <span id="white-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="white-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for White`

### `DarkCodGray`

```rust
struct DarkCodGray;
```

#### Trait Implementations

##### `impl Color for DarkCodGray`

- <span id="darkcodgray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcodgray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcodgray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcodgray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkCodGray`

### `CodGray`

```rust
struct CodGray;
```

#### Trait Implementations

##### `impl Color for CodGray`

- <span id="codgray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="codgray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="codgray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="codgray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for CodGray`

### `LightCodGray`

```rust
struct LightCodGray;
```

#### Trait Implementations

##### `impl Color for LightCodGray`

- <span id="lightcodgray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcodgray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcodgray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcodgray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightCodGray`

### `DarkMineShaft`

```rust
struct DarkMineShaft;
```

#### Trait Implementations

##### `impl Color for DarkMineShaft`

- <span id="darkmineshaft-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmineshaft-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmineshaft-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmineshaft-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkMineShaft`

### `MineShaft`

```rust
struct MineShaft;
```

#### Trait Implementations

##### `impl Color for MineShaft`

- <span id="mineshaft-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mineshaft-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mineshaft-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mineshaft-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for MineShaft`

### `LightMineShaft`

```rust
struct LightMineShaft;
```

#### Trait Implementations

##### `impl Color for LightMineShaft`

- <span id="lightmineshaft-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmineshaft-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmineshaft-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmineshaft-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightMineShaft`

### `DarkTundora`

```rust
struct DarkTundora;
```

#### Trait Implementations

##### `impl Color for DarkTundora`

- <span id="darktundora-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktundora-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktundora-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktundora-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkTundora`

### `Tundora`

```rust
struct Tundora;
```

#### Trait Implementations

##### `impl Color for Tundora`

- <span id="tundora-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tundora-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tundora-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tundora-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Tundora`

### `ScorpionGray`

```rust
struct ScorpionGray;
```

#### Trait Implementations

##### `impl Color for ScorpionGray`

- <span id="scorpiongray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpiongray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpiongray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpiongray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for ScorpionGray`

### `DarkDoveGray`

```rust
struct DarkDoveGray;
```

#### Trait Implementations

##### `impl Color for DarkDoveGray`

- <span id="darkdovegray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkdovegray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkdovegray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkdovegray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkDoveGray`

### `DoveGray`

```rust
struct DoveGray;
```

#### Trait Implementations

##### `impl Color for DoveGray`

- <span id="dovegray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dovegray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dovegray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dovegray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DoveGray`

### `Boulder`

```rust
struct Boulder;
```

#### Trait Implementations

##### `impl Color for Boulder`

- <span id="boulder-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="boulder-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="boulder-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="boulder-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Boulder`

### `Gray`

```rust
struct Gray;
```

#### Trait Implementations

##### `impl Color for Gray`

- <span id="gray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Gray`

### `LightGray`

```rust
struct LightGray;
```

#### Trait Implementations

##### `impl Color for LightGray`

- <span id="lightgray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightgray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightgray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightgray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightGray`

### `DustyGray`

```rust
struct DustyGray;
```

#### Trait Implementations

##### `impl Color for DustyGray`

- <span id="dustygray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dustygray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dustygray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dustygray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DustyGray`

### `NobelGray`

```rust
struct NobelGray;
```

#### Trait Implementations

##### `impl Color for NobelGray`

- <span id="nobelgray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="nobelgray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="nobelgray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="nobelgray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for NobelGray`

### `DarkSilverChalice`

```rust
struct DarkSilverChalice;
```

#### Trait Implementations

##### `impl Color for DarkSilverChalice`

- <span id="darksilverchalice-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilverchalice-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilverchalice-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilverchalice-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkSilverChalice`

### `LightSilverChalice`

```rust
struct LightSilverChalice;
```

#### Trait Implementations

##### `impl Color for LightSilverChalice`

- <span id="lightsilverchalice-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightsilverchalice-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightsilverchalice-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightsilverchalice-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for LightSilverChalice`

### `DarkSilver`

```rust
struct DarkSilver;
```

#### Trait Implementations

##### `impl Color for DarkSilver`

- <span id="darksilver-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilver-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilver-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilver-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkSilver`

### `Silver`

```rust
struct Silver;
```

#### Trait Implementations

##### `impl Color for Silver`

- <span id="silver-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silver-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silver-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silver-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Silver`

### `DarkAlto`

```rust
struct DarkAlto;
```

#### Trait Implementations

##### `impl Color for DarkAlto`

- <span id="darkalto-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkalto-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkalto-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkalto-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for DarkAlto`

### `Alto`

```rust
struct Alto;
```

#### Trait Implementations

##### `impl Color for Alto`

- <span id="alto-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="alto-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="alto-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="alto-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Alto`

### `Mercury`

```rust
struct Mercury;
```

#### Trait Implementations

##### `impl Color for Mercury`

- <span id="mercury-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mercury-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mercury-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mercury-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for Mercury`

### `GalleryGray`

```rust
struct GalleryGray;
```

#### Trait Implementations

##### `impl Color for GalleryGray`

- <span id="gallerygray-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gallerygray-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gallerygray-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gallerygray-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<D> OwoColorize for GalleryGray`

## Macros

### `xterm_colors!`

