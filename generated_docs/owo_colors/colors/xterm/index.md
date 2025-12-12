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
  - [`xterm_colors!`](#xterm-colors)

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
| [`xterm_colors!`](#xterm-colors) | macro |  |

## Modules

- [`dynamic`](dynamic/index.md)

## Structs

### `UserBlack`

```rust
struct UserBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBlack`

- <span id="userblack-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblack-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblack-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblack-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBlack`

### `UserRed`

```rust
struct UserRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserRed`

- <span id="userred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserRed`

### `UserGreen`

```rust
struct UserGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserGreen`

- <span id="usergreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usergreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usergreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usergreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserGreen`

### `UserYellow`

```rust
struct UserYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserYellow`

- <span id="useryellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="useryellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="useryellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="useryellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserYellow`

### `UserBlue`

```rust
struct UserBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBlue`

- <span id="userblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBlue`

### `UserMagenta`

```rust
struct UserMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserMagenta`

- <span id="usermagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usermagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usermagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usermagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserMagenta`

### `UserCyan`

```rust
struct UserCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserCyan`

- <span id="usercyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usercyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usercyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usercyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserCyan`

### `UserWhite`

```rust
struct UserWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserWhite`

- <span id="userwhite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userwhite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userwhite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userwhite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserWhite`

### `UserBrightBlack`

```rust
struct UserBrightBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightBlack`

- <span id="userbrightblack-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblack-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblack-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblack-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightBlack`

### `UserBrightRed`

```rust
struct UserBrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightRed`

- <span id="userbrightred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightRed`

### `UserBrightGreen`

```rust
struct UserBrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightGreen`

- <span id="userbrightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightGreen`

### `UserBrightYellow`

```rust
struct UserBrightYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightYellow`

- <span id="userbrightyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightYellow`

### `UserBrightBlue`

```rust
struct UserBrightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightBlue`

- <span id="userbrightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightBlue`

### `UserBrightMagenta`

```rust
struct UserBrightMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightMagenta`

- <span id="userbrightmagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightmagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightmagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightmagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightMagenta`

### `UserBrightCyan`

```rust
struct UserBrightCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightCyan`

- <span id="userbrightcyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightcyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightcyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightcyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightCyan`

### `UserBrightWhite`

```rust
struct UserBrightWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for UserBrightWhite`

- <span id="userbrightwhite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightwhite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightwhite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightwhite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for UserBrightWhite`

### `Black`

```rust
struct Black;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Black`

- <span id="black-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="black-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Black`

### `StratosBlue`

```rust
struct StratosBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for StratosBlue`

- <span id="stratosblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="stratosblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="stratosblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="stratosblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for StratosBlue`

### `NavyBlue`

```rust
struct NavyBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for NavyBlue`

- <span id="navyblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="navyblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="navyblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="navyblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for NavyBlue`

### `MidnightBlue`

```rust
struct MidnightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MidnightBlue`

- <span id="midnightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="midnightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="midnightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="midnightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MidnightBlue`

### `DarkBlue`

```rust
struct DarkBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkBlue`

- <span id="darkblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkBlue`

### `Blue`

```rust
struct Blue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Blue`

- <span id="blue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Blue`

### `CamaroneGreen`

```rust
struct CamaroneGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CamaroneGreen`

- <span id="camaronegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="camaronegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="camaronegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="camaronegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CamaroneGreen`

### `BlueStone`

```rust
struct BlueStone;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlueStone`

- <span id="bluestone-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bluestone-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bluestone-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bluestone-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlueStone`

### `OrientBlue`

```rust
struct OrientBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for OrientBlue`

- <span id="orientblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orientblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orientblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orientblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for OrientBlue`

### `EndeavourBlue`

```rust
struct EndeavourBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for EndeavourBlue`

- <span id="endeavourblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="endeavourblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="endeavourblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="endeavourblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for EndeavourBlue`

### `ScienceBlue`

```rust
struct ScienceBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScienceBlue`

- <span id="scienceblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scienceblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scienceblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scienceblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScienceBlue`

### `BlueRibbon`

```rust
struct BlueRibbon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlueRibbon`

- <span id="blueribbon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blueribbon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blueribbon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blueribbon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlueRibbon`

### `JapaneseLaurel`

```rust
struct JapaneseLaurel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for JapaneseLaurel`

- <span id="japaneselaurel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="japaneselaurel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="japaneselaurel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="japaneselaurel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for JapaneseLaurel`

### `DeepSeaGreen`

```rust
struct DeepSeaGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DeepSeaGreen`

- <span id="deepseagreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepseagreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepseagreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepseagreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DeepSeaGreen`

### `Teal`

```rust
struct Teal;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Teal`

- <span id="teal-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="teal-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="teal-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="teal-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Teal`

### `DeepCerulean`

```rust
struct DeepCerulean;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DeepCerulean`

- <span id="deepcerulean-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepcerulean-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepcerulean-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepcerulean-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DeepCerulean`

### `LochmaraBlue`

```rust
struct LochmaraBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LochmaraBlue`

- <span id="lochmarablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lochmarablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lochmarablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lochmarablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LochmaraBlue`

### `AzureRadiance`

```rust
struct AzureRadiance;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AzureRadiance`

- <span id="azureradiance-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="azureradiance-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="azureradiance-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="azureradiance-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AzureRadiance`

### `LightJapaneseLaurel`

```rust
struct LightJapaneseLaurel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightjapaneselaurel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightJapaneseLaurel`

### `Jade`

```rust
struct Jade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Jade`

- <span id="jade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="jade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="jade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="jade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Jade`

### `PersianGreen`

```rust
struct PersianGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PersianGreen`

- <span id="persiangreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="persiangreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="persiangreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="persiangreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PersianGreen`

### `BondiBlue`

```rust
struct BondiBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BondiBlue`

- <span id="bondiblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bondiblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bondiblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bondiblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BondiBlue`

### `Cerulean`

```rust
struct Cerulean;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Cerulean`

- <span id="cerulean-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cerulean-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cerulean-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cerulean-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Cerulean`

### `LightAzureRadiance`

```rust
struct LightAzureRadiance;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightAzureRadiance`

- <span id="lightazureradiance-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightazureradiance-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightazureradiance-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightazureradiance-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightAzureRadiance`

### `DarkGreen`

```rust
struct DarkGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkGreen`

- <span id="darkgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkGreen`

### `Malachite`

```rust
struct Malachite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Malachite`

- <span id="malachite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malachite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malachite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malachite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Malachite`

### `CaribbeanGreen`

```rust
struct CaribbeanGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CaribbeanGreen`

- <span id="caribbeangreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caribbeangreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caribbeangreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caribbeangreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CaribbeanGreen`

### `LightCaribbeanGreen`

```rust
struct LightCaribbeanGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcaribbeangreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightCaribbeanGreen`

### `RobinEggBlue`

```rust
struct RobinEggBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RobinEggBlue`

- <span id="robineggblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="robineggblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="robineggblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="robineggblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RobinEggBlue`

### `Aqua`

```rust
struct Aqua;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Aqua`

- <span id="aqua-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aqua-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aqua-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aqua-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Aqua`

### `Green`

```rust
struct Green;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Green`

- <span id="green-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="green-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Green`

### `DarkSpringGreen`

```rust
struct DarkSpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkSpringGreen`

- <span id="darkspringgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkspringgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkspringgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkspringgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkSpringGreen`

### `SpringGreen`

```rust
struct SpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SpringGreen`

- <span id="springgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="springgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="springgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="springgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SpringGreen`

### `LightSpringGreen`

```rust
struct LightSpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightSpringGreen`

- <span id="lightspringgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightspringgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightspringgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightspringgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightSpringGreen`

### `BrightTurquoise`

```rust
struct BrightTurquoise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightTurquoise`

- <span id="brightturquoise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightturquoise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightturquoise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightturquoise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightTurquoise`

### `Cyan`

```rust
struct Cyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Cyan`

- <span id="cyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Cyan`

### `Rosewood`

```rust
struct Rosewood;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Rosewood`

- <span id="rosewood-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rosewood-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rosewood-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rosewood-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Rosewood`

### `PompadourMagenta`

```rust
struct PompadourMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PompadourMagenta`

- <span id="pompadourmagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pompadourmagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pompadourmagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pompadourmagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PompadourMagenta`

### `PigmentIndigo`

```rust
struct PigmentIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PigmentIndigo`

- <span id="pigmentindigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pigmentindigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pigmentindigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pigmentindigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PigmentIndigo`

### `DarkPurple`

```rust
struct DarkPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkPurple`

- <span id="darkpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkPurple`

### `ElectricIndigo`

```rust
struct ElectricIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ElectricIndigo`

- <span id="electricindigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricindigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricindigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricindigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ElectricIndigo`

### `ElectricPurple`

```rust
struct ElectricPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ElectricPurple`

- <span id="electricpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ElectricPurple`

### `VerdunGreen`

```rust
struct VerdunGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for VerdunGreen`

- <span id="verdungreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="verdungreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="verdungreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="verdungreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for VerdunGreen`

### `ScorpionOlive`

```rust
struct ScorpionOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScorpionOlive`

- <span id="scorpionolive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpionolive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpionolive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpionolive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScorpionOlive`

### `Lilac`

```rust
struct Lilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Lilac`

- <span id="lilac-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lilac-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lilac-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lilac-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Lilac`

### `ScampiIndigo`

```rust
struct ScampiIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScampiIndigo`

- <span id="scampiindigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scampiindigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scampiindigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scampiindigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScampiIndigo`

### `Indigo`

```rust
struct Indigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Indigo`

- <span id="indigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="indigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="indigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="indigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Indigo`

### `DarkCornflowerBlue`

```rust
struct DarkCornflowerBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkCornflowerBlue`

- <span id="darkcornflowerblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcornflowerblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcornflowerblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcornflowerblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkCornflowerBlue`

### `DarkLimeade`

```rust
struct DarkLimeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkLimeade`

- <span id="darklimeade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklimeade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklimeade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklimeade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkLimeade`

### `GladeGreen`

```rust
struct GladeGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GladeGreen`

- <span id="gladegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gladegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gladegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gladegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GladeGreen`

### `JuniperGreen`

```rust
struct JuniperGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for JuniperGreen`

- <span id="junipergreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junipergreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junipergreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junipergreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for JuniperGreen`

### `HippieBlue`

```rust
struct HippieBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HippieBlue`

- <span id="hippieblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hippieblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hippieblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hippieblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HippieBlue`

### `HavelockBlue`

```rust
struct HavelockBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HavelockBlue`

- <span id="havelockblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="havelockblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="havelockblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="havelockblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HavelockBlue`

### `CornflowerBlue`

```rust
struct CornflowerBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CornflowerBlue`

- <span id="cornflowerblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cornflowerblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cornflowerblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cornflowerblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CornflowerBlue`

### `Limeade`

```rust
struct Limeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Limeade`

- <span id="limeade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="limeade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="limeade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="limeade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Limeade`

### `FernGreen`

```rust
struct FernGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FernGreen`

- <span id="ferngreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="ferngreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="ferngreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="ferngreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FernGreen`

### `SilverTree`

```rust
struct SilverTree;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SilverTree`

- <span id="silvertree-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silvertree-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silvertree-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silvertree-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SilverTree`

### `Tradewind`

```rust
struct Tradewind;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Tradewind`

- <span id="tradewind-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tradewind-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tradewind-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tradewind-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Tradewind`

### `ShakespeareBlue`

```rust
struct ShakespeareBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ShakespeareBlue`

- <span id="shakespeareblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="shakespeareblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="shakespeareblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="shakespeareblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ShakespeareBlue`

### `DarkMalibuBlue`

```rust
struct DarkMalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMalibuBlue`

- <span id="darkmalibublue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmalibublue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmalibublue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmalibublue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMalibuBlue`

### `DarkBrightGreen`

```rust
struct DarkBrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkBrightGreen`

- <span id="darkbrightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkbrightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkbrightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkbrightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkBrightGreen`

### `DarkPastelGreen`

```rust
struct DarkPastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkPastelGreen`

- <span id="darkpastelgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpastelgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpastelgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpastelgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkPastelGreen`

### `PastelGreen`

```rust
struct PastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PastelGreen`

- <span id="pastelgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pastelgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pastelgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pastelgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PastelGreen`

### `DownyTeal`

```rust
struct DownyTeal;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DownyTeal`

- <span id="downyteal-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="downyteal-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="downyteal-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="downyteal-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DownyTeal`

### `Viking`

```rust
struct Viking;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Viking`

- <span id="viking-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="viking-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="viking-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="viking-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Viking`

### `MalibuBlue`

```rust
struct MalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MalibuBlue`

- <span id="malibublue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malibublue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malibublue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malibublue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MalibuBlue`

### `BrightGreen`

```rust
struct BrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightGreen`

- <span id="brightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightGreen`

### `DarkScreaminGreen`

```rust
struct DarkScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkScreaminGreen`

- <span id="darkscreamingreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkscreamingreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkscreamingreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkscreamingreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkScreaminGreen`

### `ScreaminGreen`

```rust
struct ScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScreaminGreen`

- <span id="screamingreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="screamingreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="screamingreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="screamingreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScreaminGreen`

### `DarkAquamarine`

```rust
struct DarkAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkAquamarine`

- <span id="darkaquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkaquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkaquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkaquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkAquamarine`

### `Aquamarine`

```rust
struct Aquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Aquamarine`

- <span id="aquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Aquamarine`

### `LightAquamarine`

```rust
struct LightAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightAquamarine`

- <span id="lightaquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightaquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightaquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightaquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightAquamarine`

### `Maroon`

```rust
struct Maroon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Maroon`

- <span id="maroon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="maroon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="maroon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="maroon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Maroon`

### `DarkFreshEggplant`

```rust
struct DarkFreshEggplant;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkFreshEggplant`

- <span id="darkfresheggplant-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfresheggplant-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfresheggplant-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfresheggplant-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkFreshEggplant`

### `LightFreshEggplant`

```rust
struct LightFreshEggplant;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightFreshEggplant`

- <span id="lightfresheggplant-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightfresheggplant-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightfresheggplant-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightfresheggplant-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightFreshEggplant`

### `Purple`

```rust
struct Purple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Purple`

- <span id="purple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Purple`

### `ElectricViolet`

```rust
struct ElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ElectricViolet`

- <span id="electricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ElectricViolet`

### `LightElectricViolet`

```rust
struct LightElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightElectricViolet`

- <span id="lightelectricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightelectricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightelectricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightelectricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightElectricViolet`

### `Brown`

```rust
struct Brown;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Brown`

- <span id="brown-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brown-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brown-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brown-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Brown`

### `CopperRose`

```rust
struct CopperRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CopperRose`

- <span id="copperrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CopperRose`

### `StrikemasterPurple`

```rust
struct StrikemasterPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for StrikemasterPurple`

- <span id="strikemasterpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="strikemasterpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="strikemasterpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="strikemasterpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for StrikemasterPurple`

### `DelugePurple`

```rust
struct DelugePurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DelugePurple`

- <span id="delugepurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="delugepurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="delugepurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="delugepurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DelugePurple`

### `DarkMediumPurple`

```rust
struct DarkMediumPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMediumPurple`

- <span id="darkmediumpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmediumpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmediumpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmediumpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMediumPurple`

### `DarkHeliotropePurple`

```rust
struct DarkHeliotropePurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkHeliotropePurple`

- <span id="darkheliotropepurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkheliotropepurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkheliotropepurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkheliotropepurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkHeliotropePurple`

### `Olive`

```rust
struct Olive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Olive`

- <span id="olive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Olive`

### `ClayCreekOlive`

```rust
struct ClayCreekOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ClayCreekOlive`

- <span id="claycreekolive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="claycreekolive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="claycreekolive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="claycreekolive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ClayCreekOlive`

### `DarkGray`

```rust
struct DarkGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkGray`

- <span id="darkgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkGray`

### `WildBlueYonder`

```rust
struct WildBlueYonder;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for WildBlueYonder`

- <span id="wildblueyonder-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildblueyonder-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildblueyonder-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildblueyonder-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for WildBlueYonder`

### `ChetwodeBlue`

```rust
struct ChetwodeBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChetwodeBlue`

- <span id="chetwodeblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chetwodeblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chetwodeblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chetwodeblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChetwodeBlue`

### `SlateBlue`

```rust
struct SlateBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SlateBlue`

- <span id="slateblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="slateblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="slateblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="slateblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SlateBlue`

### `LightLimeade`

```rust
struct LightLimeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightLimeade`

- <span id="lightlimeade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightlimeade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightlimeade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightlimeade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightLimeade`

### `ChelseaCucumber`

```rust
struct ChelseaCucumber;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChelseaCucumber`

- <span id="chelseacucumber-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chelseacucumber-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chelseacucumber-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chelseacucumber-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChelseaCucumber`

### `BayLeaf`

```rust
struct BayLeaf;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BayLeaf`

- <span id="bayleaf-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bayleaf-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bayleaf-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bayleaf-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BayLeaf`

### `GulfStream`

```rust
struct GulfStream;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GulfStream`

- <span id="gulfstream-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gulfstream-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gulfstream-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gulfstream-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GulfStream`

### `PoloBlue`

```rust
struct PoloBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PoloBlue`

- <span id="poloblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="poloblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="poloblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="poloblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PoloBlue`

### `LightMalibuBlue`

```rust
struct LightMalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightMalibuBlue`

- <span id="lightmalibublue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmalibublue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmalibublue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmalibublue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightMalibuBlue`

### `Pistachio`

```rust
struct Pistachio;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Pistachio`

- <span id="pistachio-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pistachio-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pistachio-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pistachio-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Pistachio`

### `LightPastelGreen`

```rust
struct LightPastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightPastelGreen`

- <span id="lightpastelgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightpastelgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightpastelgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightpastelgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightPastelGreen`

### `DarkFeijoaGreen`

```rust
struct DarkFeijoaGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkFeijoaGreen`

- <span id="darkfeijoagreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfeijoagreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfeijoagreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfeijoagreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkFeijoaGreen`

### `VistaBlue`

```rust
struct VistaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for VistaBlue`

- <span id="vistablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vistablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vistablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vistablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for VistaBlue`

### `Bermuda`

```rust
struct Bermuda;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Bermuda`

- <span id="bermuda-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bermuda-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bermuda-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bermuda-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Bermuda`

### `DarkAnakiwaBlue`

```rust
struct DarkAnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkAnakiwaBlue`

- <span id="darkanakiwablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkanakiwablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkanakiwablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkanakiwablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkAnakiwaBlue`

### `ChartreuseGreen`

```rust
struct ChartreuseGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChartreuseGreen`

- <span id="chartreusegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreusegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreusegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreusegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChartreuseGreen`

### `LightScreaminGreen`

```rust
struct LightScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightScreaminGreen`

- <span id="lightscreamingreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightscreamingreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightscreamingreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightscreamingreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightScreaminGreen`

### `DarkMintGreen`

```rust
struct DarkMintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMintGreen`

- <span id="darkmintgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmintgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmintgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmintgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMintGreen`

### `MintGreen`

```rust
struct MintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MintGreen`

- <span id="mintgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mintgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mintgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mintgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MintGreen`

### `LighterAquamarine`

```rust
struct LighterAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LighterAquamarine`

- <span id="lighteraquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighteraquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighteraquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighteraquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LighterAquamarine`

### `AnakiwaBlue`

```rust
struct AnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AnakiwaBlue`

- <span id="anakiwablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="anakiwablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="anakiwablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="anakiwablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AnakiwaBlue`

### `BrightRed`

```rust
struct BrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightRed`

- <span id="brightred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightRed`

### `DarkFlirt`

```rust
struct DarkFlirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkFlirt`

- <span id="darkflirt-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkflirt-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkflirt-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkflirt-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkFlirt`

### `Flirt`

```rust
struct Flirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Flirt`

- <span id="flirt-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flirt-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flirt-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flirt-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Flirt`

### `LightFlirt`

```rust
struct LightFlirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightFlirt`

- <span id="lightflirt-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightflirt-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightflirt-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightflirt-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightFlirt`

### `DarkViolet`

```rust
struct DarkViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkViolet`

- <span id="darkviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkViolet`

### `BrightElectricViolet`

```rust
struct BrightElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightElectricViolet`

- <span id="brightelectricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightelectricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightelectricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightelectricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightElectricViolet`

### `RoseofSharonOrange`

```rust
struct RoseofSharonOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RoseofSharonOrange`

- <span id="roseofsharonorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="roseofsharonorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="roseofsharonorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="roseofsharonorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RoseofSharonOrange`

### `MatrixPink`

```rust
struct MatrixPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MatrixPink`

- <span id="matrixpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="matrixpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="matrixpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="matrixpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MatrixPink`

### `TapestryPink`

```rust
struct TapestryPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TapestryPink`

- <span id="tapestrypink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tapestrypink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tapestrypink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tapestrypink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TapestryPink`

### `FuchsiaPink`

```rust
struct FuchsiaPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FuchsiaPink`

- <span id="fuchsiapink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsiapink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsiapink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsiapink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FuchsiaPink`

### `MediumPurple`

```rust
struct MediumPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MediumPurple`

- <span id="mediumpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MediumPurple`

### `Heliotrope`

```rust
struct Heliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Heliotrope`

- <span id="heliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="heliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="heliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="heliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Heliotrope`

### `PirateGold`

```rust
struct PirateGold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PirateGold`

- <span id="pirategold-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pirategold-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pirategold-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pirategold-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PirateGold`

### `MuesliOrange`

```rust
struct MuesliOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MuesliOrange`

- <span id="muesliorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="muesliorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="muesliorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="muesliorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MuesliOrange`

### `PharlapPink`

```rust
struct PharlapPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PharlapPink`

- <span id="pharlappink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pharlappink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pharlappink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pharlappink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PharlapPink`

### `Bouquet`

```rust
struct Bouquet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Bouquet`

- <span id="bouquet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bouquet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bouquet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bouquet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Bouquet`

### `Lavender`

```rust
struct Lavender;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Lavender`

- <span id="lavender-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavender-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavender-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavender-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Lavender`

### `LightHeliotrope`

```rust
struct LightHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightHeliotrope`

- <span id="lightheliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightheliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightheliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightheliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightHeliotrope`

### `BuddhaGold`

```rust
struct BuddhaGold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BuddhaGold`

- <span id="buddhagold-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="buddhagold-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="buddhagold-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="buddhagold-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BuddhaGold`

### `OliveGreen`

```rust
struct OliveGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for OliveGreen`

- <span id="olivegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olivegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olivegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olivegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for OliveGreen`

### `HillaryOlive`

```rust
struct HillaryOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HillaryOlive`

- <span id="hillaryolive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hillaryolive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hillaryolive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hillaryolive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HillaryOlive`

### `SilverChalice`

```rust
struct SilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SilverChalice`

- <span id="silverchalice-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silverchalice-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silverchalice-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silverchalice-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SilverChalice`

### `WistfulLilac`

```rust
struct WistfulLilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for WistfulLilac`

- <span id="wistfullilac-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wistfullilac-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wistfullilac-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wistfullilac-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for WistfulLilac`

### `MelroseLilac`

```rust
struct MelroseLilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MelroseLilac`

- <span id="melroselilac-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="melroselilac-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="melroselilac-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="melroselilac-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MelroseLilac`

### `RioGrandeGreen`

```rust
struct RioGrandeGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RioGrandeGreen`

- <span id="riograndegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="riograndegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="riograndegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="riograndegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RioGrandeGreen`

### `ConiferGreen`

```rust
struct ConiferGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ConiferGreen`

- <span id="conifergreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="conifergreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="conifergreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="conifergreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ConiferGreen`

### `Feijoa`

```rust
struct Feijoa;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Feijoa`

- <span id="feijoa-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="feijoa-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="feijoa-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="feijoa-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Feijoa`

### `PixieGreen`

```rust
struct PixieGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PixieGreen`

- <span id="pixiegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pixiegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pixiegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pixiegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PixieGreen`

### `JungleMist`

```rust
struct JungleMist;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for JungleMist`

- <span id="junglemist-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junglemist-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junglemist-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junglemist-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for JungleMist`

### `LightAnakiwaBlue`

```rust
struct LightAnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightAnakiwaBlue`

- <span id="lightanakiwablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightanakiwablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightanakiwablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightanakiwablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightAnakiwaBlue`

### `Lime`

```rust
struct Lime;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Lime`

- <span id="lime-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lime-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lime-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lime-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Lime`

### `GreenYellow`

```rust
struct GreenYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GreenYellow`

- <span id="greenyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="greenyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="greenyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="greenyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GreenYellow`

### `LightMintGreen`

```rust
struct LightMintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightMintGreen`

- <span id="lightmintgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmintgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmintgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmintgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightMintGreen`

### `Celadon`

```rust
struct Celadon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Celadon`

- <span id="celadon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="celadon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="celadon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="celadon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Celadon`

### `AeroBlue`

```rust
struct AeroBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AeroBlue`

- <span id="aeroblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aeroblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aeroblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aeroblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AeroBlue`

### `FrenchPassLightBlue`

```rust
struct FrenchPassLightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FrenchPassLightBlue`

- <span id="frenchpasslightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="frenchpasslightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="frenchpasslightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="frenchpasslightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FrenchPassLightBlue`

### `GuardsmanRed`

```rust
struct GuardsmanRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GuardsmanRed`

- <span id="guardsmanred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="guardsmanred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="guardsmanred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="guardsmanred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GuardsmanRed`

### `RazzmatazzCerise`

```rust
struct RazzmatazzCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RazzmatazzCerise`

- <span id="razzmatazzcerise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="razzmatazzcerise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="razzmatazzcerise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="razzmatazzcerise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RazzmatazzCerise`

### `MediumVioletRed`

```rust
struct MediumVioletRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MediumVioletRed`

- <span id="mediumvioletred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumvioletred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumvioletred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumvioletred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MediumVioletRed`

### `HollywoodCerise`

```rust
struct HollywoodCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HollywoodCerise`

- <span id="hollywoodcerise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hollywoodcerise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hollywoodcerise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hollywoodcerise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HollywoodCerise`

### `DarkPurplePizzazz`

```rust
struct DarkPurplePizzazz;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurplepizzazz-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkPurplePizzazz`

### `BrighterElectricViolet`

```rust
struct BrighterElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrighterElectricViolet`

- <span id="brighterelectricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brighterelectricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brighterelectricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brighterelectricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrighterElectricViolet`

### `TennOrange`

```rust
struct TennOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TennOrange`

- <span id="tennorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tennorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tennorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tennorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TennOrange`

### `RomanOrange`

```rust
struct RomanOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for RomanOrange`

- <span id="romanorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="romanorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="romanorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="romanorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for RomanOrange`

### `CranberryPink`

```rust
struct CranberryPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CranberryPink`

- <span id="cranberrypink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cranberrypink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cranberrypink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cranberrypink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CranberryPink`

### `HopbushPink`

```rust
struct HopbushPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HopbushPink`

- <span id="hopbushpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hopbushpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hopbushpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hopbushpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HopbushPink`

### `Orchid`

```rust
struct Orchid;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Orchid`

- <span id="orchid-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orchid-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orchid-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orchid-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Orchid`

### `LighterHeliotrope`

```rust
struct LighterHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LighterHeliotrope`

- <span id="lighterheliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighterheliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighterheliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighterheliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LighterHeliotrope`

### `MangoTango`

```rust
struct MangoTango;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MangoTango`

- <span id="mangotango-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mangotango-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mangotango-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mangotango-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MangoTango`

### `Copperfield`

```rust
struct Copperfield;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Copperfield`

- <span id="copperfield-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperfield-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperfield-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperfield-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Copperfield`

### `SeaPink`

```rust
struct SeaPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SeaPink`

- <span id="seapink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="seapink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="seapink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="seapink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SeaPink`

### `CanCanPink`

```rust
struct CanCanPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CanCanPink`

- <span id="cancanpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cancanpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cancanpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cancanpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CanCanPink`

### `LightOrchid`

```rust
struct LightOrchid;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightOrchid`

- <span id="lightorchid-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightorchid-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightorchid-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightorchid-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightOrchid`

### `BrightHeliotrope`

```rust
struct BrightHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BrightHeliotrope`

- <span id="brightheliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightheliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightheliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightheliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BrightHeliotrope`

### `DarkCorn`

```rust
struct DarkCorn;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkCorn`

- <span id="darkcorn-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcorn-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcorn-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcorn-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkCorn`

### `DarkTachaOrange`

```rust
struct DarkTachaOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkTachaOrange`

- <span id="darktachaorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktachaorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktachaorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktachaorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkTachaOrange`

### `TanBeige`

```rust
struct TanBeige;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TanBeige`

- <span id="tanbeige-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tanbeige-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tanbeige-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tanbeige-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TanBeige`

### `ClamShell`

```rust
struct ClamShell;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ClamShell`

- <span id="clamshell-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="clamshell-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="clamshell-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="clamshell-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ClamShell`

### `ThistlePink`

```rust
struct ThistlePink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ThistlePink`

- <span id="thistlepink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="thistlepink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="thistlepink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="thistlepink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ThistlePink`

### `Mauve`

```rust
struct Mauve;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Mauve`

- <span id="mauve-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mauve-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mauve-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mauve-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Mauve`

### `Corn`

```rust
struct Corn;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Corn`

- <span id="corn-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="corn-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="corn-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="corn-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Corn`

### `TachaOrange`

```rust
struct TachaOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TachaOrange`

- <span id="tachaorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tachaorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tachaorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tachaorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TachaOrange`

### `DecoOrange`

```rust
struct DecoOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DecoOrange`

- <span id="decoorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="decoorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="decoorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="decoorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DecoOrange`

### `PaleGoldenrod`

```rust
struct PaleGoldenrod;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PaleGoldenrod`

- <span id="palegoldenrod-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="palegoldenrod-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="palegoldenrod-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="palegoldenrod-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PaleGoldenrod`

### `AltoBeige`

```rust
struct AltoBeige;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for AltoBeige`

- <span id="altobeige-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="altobeige-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="altobeige-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="altobeige-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for AltoBeige`

### `FogPink`

```rust
struct FogPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FogPink`

- <span id="fogpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fogpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fogpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fogpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FogPink`

### `ChartreuseYellow`

```rust
struct ChartreuseYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ChartreuseYellow`

- <span id="chartreuseyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreuseyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreuseyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreuseyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ChartreuseYellow`

### `Canary`

```rust
struct Canary;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Canary`

- <span id="canary-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="canary-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="canary-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="canary-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Canary`

### `Honeysuckle`

```rust
struct Honeysuckle;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Honeysuckle`

- <span id="honeysuckle-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="honeysuckle-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="honeysuckle-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="honeysuckle-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Honeysuckle`

### `ReefPaleYellow`

```rust
struct ReefPaleYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ReefPaleYellow`

- <span id="reefpaleyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="reefpaleyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="reefpaleyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="reefpaleyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ReefPaleYellow`

### `SnowyMint`

```rust
struct SnowyMint;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for SnowyMint`

- <span id="snowymint-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="snowymint-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="snowymint-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="snowymint-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for SnowyMint`

### `OysterBay`

```rust
struct OysterBay;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for OysterBay`

- <span id="oysterbay-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="oysterbay-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="oysterbay-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="oysterbay-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for OysterBay`

### `Red`

```rust
struct Red;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Red`

- <span id="red-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="red-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Red`

### `DarkRose`

```rust
struct DarkRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkRose`

- <span id="darkrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkRose`

### `Rose`

```rust
struct Rose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Rose`

- <span id="rose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Rose`

### `LightHollywoodCerise`

```rust
struct LightHollywoodCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightHollywoodCerise`

- <span id="lighthollywoodcerise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighthollywoodcerise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightHollywoodCerise`

### `PurplePizzazz`

```rust
struct PurplePizzazz;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PurplePizzazz`

- <span id="purplepizzazz-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purplepizzazz-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purplepizzazz-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purplepizzazz-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PurplePizzazz`

### `Fuchsia`

```rust
struct Fuchsia;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Fuchsia`

- <span id="fuchsia-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsia-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsia-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsia-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Fuchsia`

### `BlazeOrange`

```rust
struct BlazeOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlazeOrange`

- <span id="blazeorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blazeorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blazeorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blazeorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlazeOrange`

### `BittersweetOrange`

```rust
struct BittersweetOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BittersweetOrange`

- <span id="bittersweetorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bittersweetorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bittersweetorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bittersweetorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BittersweetOrange`

### `WildWatermelon`

```rust
struct WildWatermelon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for WildWatermelon`

- <span id="wildwatermelon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildwatermelon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildwatermelon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildwatermelon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for WildWatermelon`

### `DarkHotPink`

```rust
struct DarkHotPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkHotPink`

- <span id="darkhotpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkhotpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkhotpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkhotpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkHotPink`

### `HotPink`

```rust
struct HotPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for HotPink`

- <span id="hotpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hotpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hotpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hotpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for HotPink`

### `PinkFlamingo`

```rust
struct PinkFlamingo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PinkFlamingo`

- <span id="pinkflamingo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinkflamingo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinkflamingo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinkflamingo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PinkFlamingo`

### `FlushOrange`

```rust
struct FlushOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for FlushOrange`

- <span id="flushorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flushorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flushorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flushorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for FlushOrange`

### `Salmon`

```rust
struct Salmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Salmon`

- <span id="salmon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="salmon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="salmon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="salmon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Salmon`

### `VividTangerine`

```rust
struct VividTangerine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for VividTangerine`

- <span id="vividtangerine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vividtangerine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vividtangerine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vividtangerine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for VividTangerine`

### `PinkSalmon`

```rust
struct PinkSalmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PinkSalmon`

- <span id="pinksalmon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinksalmon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinksalmon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinksalmon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PinkSalmon`

### `DarkLavenderRose`

```rust
struct DarkLavenderRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkLavenderRose`

- <span id="darklavenderrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklavenderrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklavenderrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklavenderrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkLavenderRose`

### `BlushPink`

```rust
struct BlushPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for BlushPink`

- <span id="blushpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blushpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blushpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blushpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for BlushPink`

### `YellowSea`

```rust
struct YellowSea;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for YellowSea`

- <span id="yellowsea-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellowsea-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellowsea-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellowsea-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for YellowSea`

### `TexasRose`

```rust
struct TexasRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for TexasRose`

- <span id="texasrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="texasrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="texasrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="texasrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for TexasRose`

### `Tacao`

```rust
struct Tacao;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Tacao`

- <span id="tacao-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tacao-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tacao-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tacao-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Tacao`

### `Sundown`

```rust
struct Sundown;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Sundown`

- <span id="sundown-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="sundown-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="sundown-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="sundown-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Sundown`

### `CottonCandy`

```rust
struct CottonCandy;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CottonCandy`

- <span id="cottoncandy-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cottoncandy-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cottoncandy-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cottoncandy-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CottonCandy`

### `LavenderRose`

```rust
struct LavenderRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LavenderRose`

- <span id="lavenderrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavenderrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavenderrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavenderrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LavenderRose`

### `Gold`

```rust
struct Gold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Gold`

- <span id="gold-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gold-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gold-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gold-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Gold`

### `Dandelion`

```rust
struct Dandelion;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Dandelion`

- <span id="dandelion-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dandelion-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dandelion-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dandelion-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Dandelion`

### `GrandisCaramel`

```rust
struct GrandisCaramel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GrandisCaramel`

- <span id="grandiscaramel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="grandiscaramel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="grandiscaramel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="grandiscaramel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GrandisCaramel`

### `Caramel`

```rust
struct Caramel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Caramel`

- <span id="caramel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caramel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caramel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caramel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Caramel`

### `CosmosSalmon`

```rust
struct CosmosSalmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CosmosSalmon`

- <span id="cosmossalmon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cosmossalmon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cosmossalmon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cosmossalmon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CosmosSalmon`

### `PinkLace`

```rust
struct PinkLace;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PinkLace`

- <span id="pinklace-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinklace-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinklace-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinklace-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PinkLace`

### `Yellow`

```rust
struct Yellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Yellow`

- <span id="yellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Yellow`

### `LaserLemon`

```rust
struct LaserLemon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LaserLemon`

- <span id="laserlemon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="laserlemon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="laserlemon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="laserlemon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LaserLemon`

### `DollyYellow`

```rust
struct DollyYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DollyYellow`

- <span id="dollyyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dollyyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dollyyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dollyyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DollyYellow`

### `PortafinoYellow`

```rust
struct PortafinoYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for PortafinoYellow`

- <span id="portafinoyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="portafinoyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="portafinoyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="portafinoyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for PortafinoYellow`

### `Cumulus`

```rust
struct Cumulus;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Cumulus`

- <span id="cumulus-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cumulus-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cumulus-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cumulus-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Cumulus`

### `White`

```rust
struct White;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for White`

- <span id="white-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="white-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for White`

### `DarkCodGray`

```rust
struct DarkCodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkCodGray`

- <span id="darkcodgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcodgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcodgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcodgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkCodGray`

### `CodGray`

```rust
struct CodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for CodGray`

- <span id="codgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="codgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="codgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="codgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for CodGray`

### `LightCodGray`

```rust
struct LightCodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightCodGray`

- <span id="lightcodgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcodgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcodgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcodgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightCodGray`

### `DarkMineShaft`

```rust
struct DarkMineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkMineShaft`

- <span id="darkmineshaft-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmineshaft-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmineshaft-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmineshaft-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkMineShaft`

### `MineShaft`

```rust
struct MineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for MineShaft`

- <span id="mineshaft-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mineshaft-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mineshaft-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mineshaft-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for MineShaft`

### `LightMineShaft`

```rust
struct LightMineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightMineShaft`

- <span id="lightmineshaft-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmineshaft-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmineshaft-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmineshaft-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightMineShaft`

### `DarkTundora`

```rust
struct DarkTundora;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkTundora`

- <span id="darktundora-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktundora-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktundora-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktundora-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkTundora`

### `Tundora`

```rust
struct Tundora;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Tundora`

- <span id="tundora-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tundora-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tundora-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tundora-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Tundora`

### `ScorpionGray`

```rust
struct ScorpionGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for ScorpionGray`

- <span id="scorpiongray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpiongray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpiongray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpiongray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for ScorpionGray`

### `DarkDoveGray`

```rust
struct DarkDoveGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkDoveGray`

- <span id="darkdovegray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkdovegray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkdovegray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkdovegray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkDoveGray`

### `DoveGray`

```rust
struct DoveGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DoveGray`

- <span id="dovegray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dovegray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dovegray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dovegray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DoveGray`

### `Boulder`

```rust
struct Boulder;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Boulder`

- <span id="boulder-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="boulder-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="boulder-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="boulder-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Boulder`

### `Gray`

```rust
struct Gray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Gray`

- <span id="gray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Gray`

### `LightGray`

```rust
struct LightGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightGray`

- <span id="lightgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightGray`

### `DustyGray`

```rust
struct DustyGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DustyGray`

- <span id="dustygray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dustygray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dustygray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dustygray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DustyGray`

### `NobelGray`

```rust
struct NobelGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for NobelGray`

- <span id="nobelgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="nobelgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="nobelgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="nobelgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for NobelGray`

### `DarkSilverChalice`

```rust
struct DarkSilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkSilverChalice`

- <span id="darksilverchalice-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilverchalice-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilverchalice-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilverchalice-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkSilverChalice`

### `LightSilverChalice`

```rust
struct LightSilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for LightSilverChalice`

- <span id="lightsilverchalice-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightsilverchalice-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightsilverchalice-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightsilverchalice-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for LightSilverChalice`

### `DarkSilver`

```rust
struct DarkSilver;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkSilver`

- <span id="darksilver-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilver-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilver-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilver-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkSilver`

### `Silver`

```rust
struct Silver;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Silver`

- <span id="silver-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silver-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silver-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silver-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Silver`

### `DarkAlto`

```rust
struct DarkAlto;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for DarkAlto`

- <span id="darkalto-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkalto-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkalto-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkalto-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for DarkAlto`

### `Alto`

```rust
struct Alto;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Alto`

- <span id="alto-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="alto-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="alto-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="alto-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Alto`

### `Mercury`

```rust
struct Mercury;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for Mercury`

- <span id="mercury-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mercury-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mercury-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mercury-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for Mercury`

### `GalleryGray`

```rust
struct GalleryGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Color for GalleryGray`

- <span id="gallerygray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gallerygray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gallerygray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gallerygray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl OwoColorize for GalleryGray`

## Macros

### `xterm_colors!`

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:1-121`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L1-L121)*

