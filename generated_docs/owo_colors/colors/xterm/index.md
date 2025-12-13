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

##### `impl Any for UserBlack`

- <span id="userblack-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBlack`

- <span id="userblack-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBlack`

- <span id="userblack-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBlack`

- <span id="userblack-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblack-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblack-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblack-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBlack`

- <span id="userblack-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBlack`

- <span id="userblack-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBlack`

##### `impl<U> TryFrom for UserBlack`

- <span id="userblack-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userblack-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBlack`

- <span id="userblack-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userblack-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserRed`

```rust
struct UserRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserRed`

- <span id="userred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserRed`

- <span id="userred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserRed`

- <span id="userred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserRed`

- <span id="userred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserRed`

- <span id="userred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserRed`

- <span id="userred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserRed`

##### `impl<U> TryFrom for UserRed`

- <span id="userred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserRed`

- <span id="userred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserGreen`

```rust
struct UserGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserGreen`

- <span id="usergreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserGreen`

- <span id="usergreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserGreen`

- <span id="usergreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserGreen`

- <span id="usergreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usergreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usergreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usergreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserGreen`

- <span id="usergreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserGreen`

- <span id="usergreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserGreen`

##### `impl<U> TryFrom for UserGreen`

- <span id="usergreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usergreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserGreen`

- <span id="usergreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usergreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserYellow`

```rust
struct UserYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserYellow`

- <span id="useryellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserYellow`

- <span id="useryellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserYellow`

- <span id="useryellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserYellow`

- <span id="useryellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="useryellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="useryellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="useryellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserYellow`

- <span id="useryellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserYellow`

- <span id="useryellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserYellow`

##### `impl<U> TryFrom for UserYellow`

- <span id="useryellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="useryellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserYellow`

- <span id="useryellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="useryellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBlue`

```rust
struct UserBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBlue`

- <span id="userblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBlue`

- <span id="userblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBlue`

- <span id="userblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBlue`

- <span id="userblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBlue`

- <span id="userblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBlue`

- <span id="userblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBlue`

##### `impl<U> TryFrom for UserBlue`

- <span id="userblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBlue`

- <span id="userblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserMagenta`

```rust
struct UserMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserMagenta`

- <span id="usermagenta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserMagenta`

- <span id="usermagenta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserMagenta`

- <span id="usermagenta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserMagenta`

- <span id="usermagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usermagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usermagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usermagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserMagenta`

- <span id="usermagenta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserMagenta`

- <span id="usermagenta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserMagenta`

##### `impl<U> TryFrom for UserMagenta`

- <span id="usermagenta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usermagenta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserMagenta`

- <span id="usermagenta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usermagenta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserCyan`

```rust
struct UserCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserCyan`

- <span id="usercyan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserCyan`

- <span id="usercyan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserCyan`

- <span id="usercyan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserCyan`

- <span id="usercyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="usercyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="usercyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="usercyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserCyan`

- <span id="usercyan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserCyan`

- <span id="usercyan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserCyan`

##### `impl<U> TryFrom for UserCyan`

- <span id="usercyan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usercyan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserCyan`

- <span id="usercyan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usercyan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserWhite`

```rust
struct UserWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserWhite`

- <span id="userwhite-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserWhite`

- <span id="userwhite-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserWhite`

- <span id="userwhite-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserWhite`

- <span id="userwhite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userwhite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userwhite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userwhite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserWhite`

- <span id="userwhite-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserWhite`

- <span id="userwhite-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserWhite`

##### `impl<U> TryFrom for UserWhite`

- <span id="userwhite-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userwhite-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserWhite`

- <span id="userwhite-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userwhite-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightBlack`

```rust
struct UserBrightBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightBlack`

- <span id="userbrightblack-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightBlack`

- <span id="userbrightblack-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightBlack`

- <span id="userbrightblack-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightBlack`

- <span id="userbrightblack-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblack-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblack-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblack-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightBlack`

- <span id="userbrightblack-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightBlack`

- <span id="userbrightblack-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightBlack`

##### `impl<U> TryFrom for UserBrightBlack`

- <span id="userbrightblack-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightblack-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightBlack`

- <span id="userbrightblack-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightblack-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightRed`

```rust
struct UserBrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightRed`

- <span id="userbrightred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightRed`

- <span id="userbrightred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightRed`

- <span id="userbrightred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightRed`

- <span id="userbrightred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightRed`

- <span id="userbrightred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightRed`

- <span id="userbrightred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightRed`

##### `impl<U> TryFrom for UserBrightRed`

- <span id="userbrightred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightRed`

- <span id="userbrightred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightGreen`

```rust
struct UserBrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightGreen`

- <span id="userbrightgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightGreen`

- <span id="userbrightgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightGreen`

- <span id="userbrightgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightGreen`

- <span id="userbrightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightGreen`

- <span id="userbrightgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightGreen`

- <span id="userbrightgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightGreen`

##### `impl<U> TryFrom for UserBrightGreen`

- <span id="userbrightgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightGreen`

- <span id="userbrightgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightYellow`

```rust
struct UserBrightYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightYellow`

- <span id="userbrightyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightYellow`

- <span id="userbrightyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightYellow`

- <span id="userbrightyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightYellow`

- <span id="userbrightyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightYellow`

- <span id="userbrightyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightYellow`

- <span id="userbrightyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightYellow`

##### `impl<U> TryFrom for UserBrightYellow`

- <span id="userbrightyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightYellow`

- <span id="userbrightyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightBlue`

```rust
struct UserBrightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightBlue`

- <span id="userbrightblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightBlue`

- <span id="userbrightblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightBlue`

- <span id="userbrightblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightBlue`

- <span id="userbrightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightBlue`

- <span id="userbrightblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightBlue`

- <span id="userbrightblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightBlue`

##### `impl<U> TryFrom for UserBrightBlue`

- <span id="userbrightblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightBlue`

- <span id="userbrightblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightMagenta`

```rust
struct UserBrightMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightMagenta`

- <span id="userbrightmagenta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightMagenta`

- <span id="userbrightmagenta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightMagenta`

- <span id="userbrightmagenta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightMagenta`

- <span id="userbrightmagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightmagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightmagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightmagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightMagenta`

- <span id="userbrightmagenta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightMagenta`

- <span id="userbrightmagenta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightMagenta`

##### `impl<U> TryFrom for UserBrightMagenta`

- <span id="userbrightmagenta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightmagenta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightMagenta`

- <span id="userbrightmagenta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightmagenta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightCyan`

```rust
struct UserBrightCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightCyan`

- <span id="userbrightcyan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightCyan`

- <span id="userbrightcyan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightCyan`

- <span id="userbrightcyan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightCyan`

- <span id="userbrightcyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightcyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightcyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightcyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightCyan`

- <span id="userbrightcyan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightCyan`

- <span id="userbrightcyan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightCyan`

##### `impl<U> TryFrom for UserBrightCyan`

- <span id="userbrightcyan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightcyan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightCyan`

- <span id="userbrightcyan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightcyan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UserBrightWhite`

```rust
struct UserBrightWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for UserBrightWhite`

- <span id="userbrightwhite-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UserBrightWhite`

- <span id="userbrightwhite-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UserBrightWhite`

- <span id="userbrightwhite-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for UserBrightWhite`

- <span id="userbrightwhite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="userbrightwhite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="userbrightwhite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="userbrightwhite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for UserBrightWhite`

- <span id="userbrightwhite-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UserBrightWhite`

- <span id="userbrightwhite-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for UserBrightWhite`

##### `impl<U> TryFrom for UserBrightWhite`

- <span id="userbrightwhite-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="userbrightwhite-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UserBrightWhite`

- <span id="userbrightwhite-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="userbrightwhite-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Black`

```rust
struct Black;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Black`

- <span id="black-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Black`

- <span id="black-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Black`

- <span id="black-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Black`

- <span id="black-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="black-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Black`

- <span id="black-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Black`

- <span id="black-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Black`

##### `impl<U> TryFrom for Black`

- <span id="black-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="black-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Black`

- <span id="black-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="black-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StratosBlue`

```rust
struct StratosBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for StratosBlue`

- <span id="stratosblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StratosBlue`

- <span id="stratosblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StratosBlue`

- <span id="stratosblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for StratosBlue`

- <span id="stratosblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="stratosblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="stratosblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="stratosblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for StratosBlue`

- <span id="stratosblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StratosBlue`

- <span id="stratosblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StratosBlue`

##### `impl<U> TryFrom for StratosBlue`

- <span id="stratosblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stratosblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StratosBlue`

- <span id="stratosblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stratosblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NavyBlue`

```rust
struct NavyBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for NavyBlue`

- <span id="navyblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NavyBlue`

- <span id="navyblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NavyBlue`

- <span id="navyblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for NavyBlue`

- <span id="navyblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="navyblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="navyblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="navyblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for NavyBlue`

- <span id="navyblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NavyBlue`

- <span id="navyblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for NavyBlue`

##### `impl<U> TryFrom for NavyBlue`

- <span id="navyblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="navyblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NavyBlue`

- <span id="navyblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="navyblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MidnightBlue`

```rust
struct MidnightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MidnightBlue`

- <span id="midnightblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MidnightBlue`

- <span id="midnightblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MidnightBlue`

- <span id="midnightblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MidnightBlue`

- <span id="midnightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="midnightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="midnightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="midnightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MidnightBlue`

- <span id="midnightblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MidnightBlue`

- <span id="midnightblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MidnightBlue`

##### `impl<U> TryFrom for MidnightBlue`

- <span id="midnightblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="midnightblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MidnightBlue`

- <span id="midnightblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="midnightblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkBlue`

```rust
struct DarkBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkBlue`

- <span id="darkblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkBlue`

- <span id="darkblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkBlue`

- <span id="darkblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkBlue`

- <span id="darkblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkBlue`

- <span id="darkblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkBlue`

- <span id="darkblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkBlue`

##### `impl<U> TryFrom for DarkBlue`

- <span id="darkblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkBlue`

- <span id="darkblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Blue`

```rust
struct Blue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Blue`

- <span id="blue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Blue`

- <span id="blue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Blue`

- <span id="blue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Blue`

- <span id="blue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Blue`

- <span id="blue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Blue`

- <span id="blue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Blue`

##### `impl<U> TryFrom for Blue`

- <span id="blue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Blue`

- <span id="blue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CamaroneGreen`

```rust
struct CamaroneGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CamaroneGreen`

- <span id="camaronegreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CamaroneGreen`

- <span id="camaronegreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CamaroneGreen`

- <span id="camaronegreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CamaroneGreen`

- <span id="camaronegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="camaronegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="camaronegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="camaronegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CamaroneGreen`

- <span id="camaronegreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CamaroneGreen`

- <span id="camaronegreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CamaroneGreen`

##### `impl<U> TryFrom for CamaroneGreen`

- <span id="camaronegreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="camaronegreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CamaroneGreen`

- <span id="camaronegreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="camaronegreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BlueStone`

```rust
struct BlueStone;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BlueStone`

- <span id="bluestone-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlueStone`

- <span id="bluestone-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlueStone`

- <span id="bluestone-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BlueStone`

- <span id="bluestone-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bluestone-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bluestone-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bluestone-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BlueStone`

- <span id="bluestone-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BlueStone`

- <span id="bluestone-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BlueStone`

##### `impl<U> TryFrom for BlueStone`

- <span id="bluestone-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bluestone-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlueStone`

- <span id="bluestone-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bluestone-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OrientBlue`

```rust
struct OrientBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for OrientBlue`

- <span id="orientblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OrientBlue`

- <span id="orientblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OrientBlue`

- <span id="orientblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for OrientBlue`

- <span id="orientblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orientblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orientblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orientblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for OrientBlue`

- <span id="orientblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OrientBlue`

- <span id="orientblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for OrientBlue`

##### `impl<U> TryFrom for OrientBlue`

- <span id="orientblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="orientblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OrientBlue`

- <span id="orientblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="orientblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EndeavourBlue`

```rust
struct EndeavourBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for EndeavourBlue`

- <span id="endeavourblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EndeavourBlue`

- <span id="endeavourblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EndeavourBlue`

- <span id="endeavourblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for EndeavourBlue`

- <span id="endeavourblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="endeavourblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="endeavourblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="endeavourblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for EndeavourBlue`

- <span id="endeavourblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EndeavourBlue`

- <span id="endeavourblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for EndeavourBlue`

##### `impl<U> TryFrom for EndeavourBlue`

- <span id="endeavourblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="endeavourblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EndeavourBlue`

- <span id="endeavourblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="endeavourblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScienceBlue`

```rust
struct ScienceBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ScienceBlue`

- <span id="scienceblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScienceBlue`

- <span id="scienceblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScienceBlue`

- <span id="scienceblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ScienceBlue`

- <span id="scienceblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scienceblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scienceblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scienceblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ScienceBlue`

- <span id="scienceblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScienceBlue`

- <span id="scienceblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ScienceBlue`

##### `impl<U> TryFrom for ScienceBlue`

- <span id="scienceblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scienceblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScienceBlue`

- <span id="scienceblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scienceblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BlueRibbon`

```rust
struct BlueRibbon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BlueRibbon`

- <span id="blueribbon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlueRibbon`

- <span id="blueribbon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlueRibbon`

- <span id="blueribbon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BlueRibbon`

- <span id="blueribbon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blueribbon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blueribbon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blueribbon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BlueRibbon`

- <span id="blueribbon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BlueRibbon`

- <span id="blueribbon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BlueRibbon`

##### `impl<U> TryFrom for BlueRibbon`

- <span id="blueribbon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blueribbon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlueRibbon`

- <span id="blueribbon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blueribbon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JapaneseLaurel`

```rust
struct JapaneseLaurel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for JapaneseLaurel`

- <span id="japaneselaurel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JapaneseLaurel`

- <span id="japaneselaurel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JapaneseLaurel`

- <span id="japaneselaurel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for JapaneseLaurel`

- <span id="japaneselaurel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="japaneselaurel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="japaneselaurel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="japaneselaurel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for JapaneseLaurel`

- <span id="japaneselaurel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JapaneseLaurel`

- <span id="japaneselaurel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for JapaneseLaurel`

##### `impl<U> TryFrom for JapaneseLaurel`

- <span id="japaneselaurel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="japaneselaurel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JapaneseLaurel`

- <span id="japaneselaurel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="japaneselaurel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DeepSeaGreen`

```rust
struct DeepSeaGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DeepSeaGreen`

- <span id="deepseagreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DeepSeaGreen`

- <span id="deepseagreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DeepSeaGreen`

- <span id="deepseagreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DeepSeaGreen`

- <span id="deepseagreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepseagreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepseagreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepseagreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DeepSeaGreen`

- <span id="deepseagreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DeepSeaGreen`

- <span id="deepseagreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DeepSeaGreen`

##### `impl<U> TryFrom for DeepSeaGreen`

- <span id="deepseagreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deepseagreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DeepSeaGreen`

- <span id="deepseagreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deepseagreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Teal`

```rust
struct Teal;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Teal`

- <span id="teal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Teal`

- <span id="teal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Teal`

- <span id="teal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Teal`

- <span id="teal-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="teal-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="teal-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="teal-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Teal`

- <span id="teal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Teal`

- <span id="teal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Teal`

##### `impl<U> TryFrom for Teal`

- <span id="teal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="teal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Teal`

- <span id="teal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="teal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DeepCerulean`

```rust
struct DeepCerulean;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DeepCerulean`

- <span id="deepcerulean-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DeepCerulean`

- <span id="deepcerulean-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DeepCerulean`

- <span id="deepcerulean-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DeepCerulean`

- <span id="deepcerulean-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="deepcerulean-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="deepcerulean-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="deepcerulean-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DeepCerulean`

- <span id="deepcerulean-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DeepCerulean`

- <span id="deepcerulean-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DeepCerulean`

##### `impl<U> TryFrom for DeepCerulean`

- <span id="deepcerulean-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deepcerulean-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DeepCerulean`

- <span id="deepcerulean-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deepcerulean-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LochmaraBlue`

```rust
struct LochmaraBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LochmaraBlue`

- <span id="lochmarablue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LochmaraBlue`

- <span id="lochmarablue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LochmaraBlue`

- <span id="lochmarablue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LochmaraBlue`

- <span id="lochmarablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lochmarablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lochmarablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lochmarablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LochmaraBlue`

- <span id="lochmarablue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LochmaraBlue`

- <span id="lochmarablue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LochmaraBlue`

##### `impl<U> TryFrom for LochmaraBlue`

- <span id="lochmarablue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lochmarablue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LochmaraBlue`

- <span id="lochmarablue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lochmarablue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AzureRadiance`

```rust
struct AzureRadiance;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for AzureRadiance`

- <span id="azureradiance-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AzureRadiance`

- <span id="azureradiance-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AzureRadiance`

- <span id="azureradiance-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for AzureRadiance`

- <span id="azureradiance-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="azureradiance-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="azureradiance-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="azureradiance-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for AzureRadiance`

- <span id="azureradiance-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AzureRadiance`

- <span id="azureradiance-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for AzureRadiance`

##### `impl<U> TryFrom for AzureRadiance`

- <span id="azureradiance-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="azureradiance-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AzureRadiance`

- <span id="azureradiance-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="azureradiance-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightJapaneseLaurel`

```rust
struct LightJapaneseLaurel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightjapaneselaurel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightjapaneselaurel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightJapaneseLaurel`

##### `impl<U> TryFrom for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightjapaneselaurel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightJapaneseLaurel`

- <span id="lightjapaneselaurel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightjapaneselaurel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Jade`

```rust
struct Jade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Jade`

- <span id="jade-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Jade`

- <span id="jade-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Jade`

- <span id="jade-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Jade`

- <span id="jade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="jade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="jade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="jade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Jade`

- <span id="jade-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Jade`

- <span id="jade-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Jade`

##### `impl<U> TryFrom for Jade`

- <span id="jade-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="jade-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Jade`

- <span id="jade-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="jade-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PersianGreen`

```rust
struct PersianGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PersianGreen`

- <span id="persiangreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PersianGreen`

- <span id="persiangreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PersianGreen`

- <span id="persiangreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PersianGreen`

- <span id="persiangreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="persiangreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="persiangreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="persiangreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PersianGreen`

- <span id="persiangreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PersianGreen`

- <span id="persiangreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PersianGreen`

##### `impl<U> TryFrom for PersianGreen`

- <span id="persiangreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="persiangreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PersianGreen`

- <span id="persiangreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="persiangreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BondiBlue`

```rust
struct BondiBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BondiBlue`

- <span id="bondiblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BondiBlue`

- <span id="bondiblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BondiBlue`

- <span id="bondiblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BondiBlue`

- <span id="bondiblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bondiblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bondiblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bondiblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BondiBlue`

- <span id="bondiblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BondiBlue`

- <span id="bondiblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BondiBlue`

##### `impl<U> TryFrom for BondiBlue`

- <span id="bondiblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bondiblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BondiBlue`

- <span id="bondiblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bondiblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cerulean`

```rust
struct Cerulean;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Cerulean`

- <span id="cerulean-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cerulean`

- <span id="cerulean-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cerulean`

- <span id="cerulean-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Cerulean`

- <span id="cerulean-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cerulean-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cerulean-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cerulean-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Cerulean`

- <span id="cerulean-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cerulean`

- <span id="cerulean-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Cerulean`

##### `impl<U> TryFrom for Cerulean`

- <span id="cerulean-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cerulean-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cerulean`

- <span id="cerulean-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cerulean-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightAzureRadiance`

```rust
struct LightAzureRadiance;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightAzureRadiance`

- <span id="lightazureradiance-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightAzureRadiance`

- <span id="lightazureradiance-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightAzureRadiance`

- <span id="lightazureradiance-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightAzureRadiance`

- <span id="lightazureradiance-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightazureradiance-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightazureradiance-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightazureradiance-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightAzureRadiance`

- <span id="lightazureradiance-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightAzureRadiance`

- <span id="lightazureradiance-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightAzureRadiance`

##### `impl<U> TryFrom for LightAzureRadiance`

- <span id="lightazureradiance-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightazureradiance-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightAzureRadiance`

- <span id="lightazureradiance-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightazureradiance-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkGreen`

```rust
struct DarkGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkGreen`

- <span id="darkgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkGreen`

- <span id="darkgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkGreen`

- <span id="darkgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkGreen`

- <span id="darkgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkGreen`

- <span id="darkgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkGreen`

- <span id="darkgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkGreen`

##### `impl<U> TryFrom for DarkGreen`

- <span id="darkgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkGreen`

- <span id="darkgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Malachite`

```rust
struct Malachite;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Malachite`

- <span id="malachite-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Malachite`

- <span id="malachite-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Malachite`

- <span id="malachite-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Malachite`

- <span id="malachite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malachite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malachite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malachite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Malachite`

- <span id="malachite-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Malachite`

- <span id="malachite-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Malachite`

##### `impl<U> TryFrom for Malachite`

- <span id="malachite-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="malachite-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Malachite`

- <span id="malachite-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="malachite-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CaribbeanGreen`

```rust
struct CaribbeanGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CaribbeanGreen`

- <span id="caribbeangreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaribbeanGreen`

- <span id="caribbeangreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaribbeanGreen`

- <span id="caribbeangreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CaribbeanGreen`

- <span id="caribbeangreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caribbeangreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caribbeangreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caribbeangreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CaribbeanGreen`

- <span id="caribbeangreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CaribbeanGreen`

- <span id="caribbeangreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CaribbeanGreen`

##### `impl<U> TryFrom for CaribbeanGreen`

- <span id="caribbeangreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="caribbeangreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaribbeanGreen`

- <span id="caribbeangreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="caribbeangreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightCaribbeanGreen`

```rust
struct LightCaribbeanGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcaribbeangreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcaribbeangreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightCaribbeanGreen`

##### `impl<U> TryFrom for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightcaribbeangreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightCaribbeanGreen`

- <span id="lightcaribbeangreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightcaribbeangreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RobinEggBlue`

```rust
struct RobinEggBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for RobinEggBlue`

- <span id="robineggblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RobinEggBlue`

- <span id="robineggblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RobinEggBlue`

- <span id="robineggblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for RobinEggBlue`

- <span id="robineggblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="robineggblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="robineggblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="robineggblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for RobinEggBlue`

- <span id="robineggblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RobinEggBlue`

- <span id="robineggblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for RobinEggBlue`

##### `impl<U> TryFrom for RobinEggBlue`

- <span id="robineggblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="robineggblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RobinEggBlue`

- <span id="robineggblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="robineggblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Aqua`

```rust
struct Aqua;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Aqua`

- <span id="aqua-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Aqua`

- <span id="aqua-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Aqua`

- <span id="aqua-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Aqua`

- <span id="aqua-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aqua-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aqua-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aqua-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Aqua`

- <span id="aqua-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Aqua`

- <span id="aqua-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Aqua`

##### `impl<U> TryFrom for Aqua`

- <span id="aqua-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aqua-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Aqua`

- <span id="aqua-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aqua-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Green`

```rust
struct Green;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Green`

- <span id="green-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Green`

- <span id="green-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Green`

- <span id="green-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Green`

- <span id="green-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="green-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Green`

- <span id="green-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Green`

- <span id="green-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Green`

##### `impl<U> TryFrom for Green`

- <span id="green-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="green-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Green`

- <span id="green-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="green-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkSpringGreen`

```rust
struct DarkSpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkSpringGreen`

- <span id="darkspringgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkSpringGreen`

- <span id="darkspringgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkSpringGreen`

- <span id="darkspringgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkSpringGreen`

- <span id="darkspringgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkspringgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkspringgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkspringgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkSpringGreen`

- <span id="darkspringgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkSpringGreen`

- <span id="darkspringgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkSpringGreen`

##### `impl<U> TryFrom for DarkSpringGreen`

- <span id="darkspringgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkspringgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkSpringGreen`

- <span id="darkspringgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkspringgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SpringGreen`

```rust
struct SpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for SpringGreen`

- <span id="springgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpringGreen`

- <span id="springgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpringGreen`

- <span id="springgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for SpringGreen`

- <span id="springgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="springgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="springgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="springgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for SpringGreen`

- <span id="springgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpringGreen`

- <span id="springgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SpringGreen`

##### `impl<U> TryFrom for SpringGreen`

- <span id="springgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="springgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpringGreen`

- <span id="springgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="springgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightSpringGreen`

```rust
struct LightSpringGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightSpringGreen`

- <span id="lightspringgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightSpringGreen`

- <span id="lightspringgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightSpringGreen`

- <span id="lightspringgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightSpringGreen`

- <span id="lightspringgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightspringgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightspringgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightspringgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightSpringGreen`

- <span id="lightspringgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightSpringGreen`

- <span id="lightspringgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightSpringGreen`

##### `impl<U> TryFrom for LightSpringGreen`

- <span id="lightspringgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightspringgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightSpringGreen`

- <span id="lightspringgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightspringgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightTurquoise`

```rust
struct BrightTurquoise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BrightTurquoise`

- <span id="brightturquoise-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightTurquoise`

- <span id="brightturquoise-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightTurquoise`

- <span id="brightturquoise-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightTurquoise`

- <span id="brightturquoise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightturquoise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightturquoise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightturquoise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BrightTurquoise`

- <span id="brightturquoise-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightTurquoise`

- <span id="brightturquoise-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightTurquoise`

##### `impl<U> TryFrom for BrightTurquoise`

- <span id="brightturquoise-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightturquoise-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightTurquoise`

- <span id="brightturquoise-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightturquoise-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cyan`

```rust
struct Cyan;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Cyan`

- <span id="cyan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cyan`

- <span id="cyan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cyan`

- <span id="cyan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Cyan`

- <span id="cyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Cyan`

- <span id="cyan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cyan`

- <span id="cyan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Cyan`

##### `impl<U> TryFrom for Cyan`

- <span id="cyan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cyan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cyan`

- <span id="cyan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cyan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Rosewood`

```rust
struct Rosewood;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Rosewood`

- <span id="rosewood-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rosewood`

- <span id="rosewood-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rosewood`

- <span id="rosewood-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Rosewood`

- <span id="rosewood-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rosewood-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rosewood-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rosewood-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Rosewood`

- <span id="rosewood-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Rosewood`

- <span id="rosewood-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Rosewood`

##### `impl<U> TryFrom for Rosewood`

- <span id="rosewood-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rosewood-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rosewood`

- <span id="rosewood-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rosewood-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PompadourMagenta`

```rust
struct PompadourMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PompadourMagenta`

- <span id="pompadourmagenta-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PompadourMagenta`

- <span id="pompadourmagenta-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PompadourMagenta`

- <span id="pompadourmagenta-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PompadourMagenta`

- <span id="pompadourmagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pompadourmagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pompadourmagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pompadourmagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PompadourMagenta`

- <span id="pompadourmagenta-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PompadourMagenta`

- <span id="pompadourmagenta-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PompadourMagenta`

##### `impl<U> TryFrom for PompadourMagenta`

- <span id="pompadourmagenta-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pompadourmagenta-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PompadourMagenta`

- <span id="pompadourmagenta-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pompadourmagenta-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PigmentIndigo`

```rust
struct PigmentIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PigmentIndigo`

- <span id="pigmentindigo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PigmentIndigo`

- <span id="pigmentindigo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PigmentIndigo`

- <span id="pigmentindigo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PigmentIndigo`

- <span id="pigmentindigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pigmentindigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pigmentindigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pigmentindigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PigmentIndigo`

- <span id="pigmentindigo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PigmentIndigo`

- <span id="pigmentindigo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PigmentIndigo`

##### `impl<U> TryFrom for PigmentIndigo`

- <span id="pigmentindigo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pigmentindigo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PigmentIndigo`

- <span id="pigmentindigo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pigmentindigo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkPurple`

```rust
struct DarkPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkPurple`

- <span id="darkpurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkPurple`

- <span id="darkpurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkPurple`

- <span id="darkpurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkPurple`

- <span id="darkpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkPurple`

- <span id="darkpurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkPurple`

- <span id="darkpurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkPurple`

##### `impl<U> TryFrom for DarkPurple`

- <span id="darkpurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkpurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkPurple`

- <span id="darkpurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkpurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ElectricIndigo`

```rust
struct ElectricIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ElectricIndigo`

- <span id="electricindigo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElectricIndigo`

- <span id="electricindigo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElectricIndigo`

- <span id="electricindigo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ElectricIndigo`

- <span id="electricindigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricindigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricindigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricindigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ElectricIndigo`

- <span id="electricindigo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElectricIndigo`

- <span id="electricindigo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ElectricIndigo`

##### `impl<U> TryFrom for ElectricIndigo`

- <span id="electricindigo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="electricindigo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElectricIndigo`

- <span id="electricindigo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="electricindigo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ElectricPurple`

```rust
struct ElectricPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ElectricPurple`

- <span id="electricpurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElectricPurple`

- <span id="electricpurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElectricPurple`

- <span id="electricpurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ElectricPurple`

- <span id="electricpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ElectricPurple`

- <span id="electricpurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElectricPurple`

- <span id="electricpurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ElectricPurple`

##### `impl<U> TryFrom for ElectricPurple`

- <span id="electricpurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="electricpurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElectricPurple`

- <span id="electricpurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="electricpurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerdunGreen`

```rust
struct VerdunGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for VerdunGreen`

- <span id="verdungreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerdunGreen`

- <span id="verdungreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerdunGreen`

- <span id="verdungreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for VerdunGreen`

- <span id="verdungreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="verdungreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="verdungreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="verdungreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for VerdunGreen`

- <span id="verdungreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerdunGreen`

- <span id="verdungreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for VerdunGreen`

##### `impl<U> TryFrom for VerdunGreen`

- <span id="verdungreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verdungreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerdunGreen`

- <span id="verdungreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verdungreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScorpionOlive`

```rust
struct ScorpionOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ScorpionOlive`

- <span id="scorpionolive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScorpionOlive`

- <span id="scorpionolive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScorpionOlive`

- <span id="scorpionolive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ScorpionOlive`

- <span id="scorpionolive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpionolive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpionolive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpionolive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ScorpionOlive`

- <span id="scorpionolive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScorpionOlive`

- <span id="scorpionolive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ScorpionOlive`

##### `impl<U> TryFrom for ScorpionOlive`

- <span id="scorpionolive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scorpionolive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScorpionOlive`

- <span id="scorpionolive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scorpionolive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lilac`

```rust
struct Lilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Lilac`

- <span id="lilac-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lilac`

- <span id="lilac-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lilac`

- <span id="lilac-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Lilac`

- <span id="lilac-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lilac-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lilac-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lilac-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Lilac`

- <span id="lilac-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lilac`

- <span id="lilac-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Lilac`

##### `impl<U> TryFrom for Lilac`

- <span id="lilac-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lilac-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lilac`

- <span id="lilac-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lilac-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScampiIndigo`

```rust
struct ScampiIndigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ScampiIndigo`

- <span id="scampiindigo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScampiIndigo`

- <span id="scampiindigo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScampiIndigo`

- <span id="scampiindigo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ScampiIndigo`

- <span id="scampiindigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scampiindigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scampiindigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scampiindigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ScampiIndigo`

- <span id="scampiindigo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScampiIndigo`

- <span id="scampiindigo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ScampiIndigo`

##### `impl<U> TryFrom for ScampiIndigo`

- <span id="scampiindigo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scampiindigo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScampiIndigo`

- <span id="scampiindigo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scampiindigo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Indigo`

```rust
struct Indigo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Indigo`

- <span id="indigo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Indigo`

- <span id="indigo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Indigo`

- <span id="indigo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Indigo`

- <span id="indigo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="indigo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="indigo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="indigo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Indigo`

- <span id="indigo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Indigo`

- <span id="indigo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Indigo`

##### `impl<U> TryFrom for Indigo`

- <span id="indigo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="indigo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Indigo`

- <span id="indigo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="indigo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkCornflowerBlue`

```rust
struct DarkCornflowerBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkCornflowerBlue`

- <span id="darkcornflowerblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkCornflowerBlue`

- <span id="darkcornflowerblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkCornflowerBlue`

- <span id="darkcornflowerblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkCornflowerBlue`

- <span id="darkcornflowerblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcornflowerblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcornflowerblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcornflowerblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkCornflowerBlue`

- <span id="darkcornflowerblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkCornflowerBlue`

- <span id="darkcornflowerblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkCornflowerBlue`

##### `impl<U> TryFrom for DarkCornflowerBlue`

- <span id="darkcornflowerblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkcornflowerblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkCornflowerBlue`

- <span id="darkcornflowerblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkcornflowerblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkLimeade`

```rust
struct DarkLimeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkLimeade`

- <span id="darklimeade-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkLimeade`

- <span id="darklimeade-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkLimeade`

- <span id="darklimeade-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkLimeade`

- <span id="darklimeade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklimeade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklimeade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklimeade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkLimeade`

- <span id="darklimeade-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkLimeade`

- <span id="darklimeade-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkLimeade`

##### `impl<U> TryFrom for DarkLimeade`

- <span id="darklimeade-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darklimeade-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkLimeade`

- <span id="darklimeade-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darklimeade-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GladeGreen`

```rust
struct GladeGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for GladeGreen`

- <span id="gladegreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GladeGreen`

- <span id="gladegreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GladeGreen`

- <span id="gladegreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for GladeGreen`

- <span id="gladegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gladegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gladegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gladegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for GladeGreen`

- <span id="gladegreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GladeGreen`

- <span id="gladegreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GladeGreen`

##### `impl<U> TryFrom for GladeGreen`

- <span id="gladegreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gladegreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GladeGreen`

- <span id="gladegreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gladegreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JuniperGreen`

```rust
struct JuniperGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for JuniperGreen`

- <span id="junipergreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JuniperGreen`

- <span id="junipergreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JuniperGreen`

- <span id="junipergreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for JuniperGreen`

- <span id="junipergreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junipergreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junipergreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junipergreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for JuniperGreen`

- <span id="junipergreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JuniperGreen`

- <span id="junipergreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for JuniperGreen`

##### `impl<U> TryFrom for JuniperGreen`

- <span id="junipergreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="junipergreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JuniperGreen`

- <span id="junipergreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="junipergreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HippieBlue`

```rust
struct HippieBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for HippieBlue`

- <span id="hippieblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HippieBlue`

- <span id="hippieblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HippieBlue`

- <span id="hippieblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for HippieBlue`

- <span id="hippieblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hippieblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hippieblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hippieblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for HippieBlue`

- <span id="hippieblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HippieBlue`

- <span id="hippieblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HippieBlue`

##### `impl<U> TryFrom for HippieBlue`

- <span id="hippieblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hippieblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HippieBlue`

- <span id="hippieblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hippieblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HavelockBlue`

```rust
struct HavelockBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for HavelockBlue`

- <span id="havelockblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HavelockBlue`

- <span id="havelockblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HavelockBlue`

- <span id="havelockblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for HavelockBlue`

- <span id="havelockblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="havelockblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="havelockblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="havelockblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for HavelockBlue`

- <span id="havelockblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HavelockBlue`

- <span id="havelockblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HavelockBlue`

##### `impl<U> TryFrom for HavelockBlue`

- <span id="havelockblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="havelockblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HavelockBlue`

- <span id="havelockblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="havelockblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CornflowerBlue`

```rust
struct CornflowerBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CornflowerBlue`

- <span id="cornflowerblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CornflowerBlue`

- <span id="cornflowerblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CornflowerBlue`

- <span id="cornflowerblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CornflowerBlue`

- <span id="cornflowerblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cornflowerblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cornflowerblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cornflowerblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CornflowerBlue`

- <span id="cornflowerblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CornflowerBlue`

- <span id="cornflowerblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CornflowerBlue`

##### `impl<U> TryFrom for CornflowerBlue`

- <span id="cornflowerblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cornflowerblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CornflowerBlue`

- <span id="cornflowerblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cornflowerblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Limeade`

```rust
struct Limeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Limeade`

- <span id="limeade-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Limeade`

- <span id="limeade-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Limeade`

- <span id="limeade-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Limeade`

- <span id="limeade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="limeade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="limeade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="limeade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Limeade`

- <span id="limeade-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Limeade`

- <span id="limeade-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Limeade`

##### `impl<U> TryFrom for Limeade`

- <span id="limeade-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="limeade-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Limeade`

- <span id="limeade-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="limeade-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FernGreen`

```rust
struct FernGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for FernGreen`

- <span id="ferngreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FernGreen`

- <span id="ferngreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FernGreen`

- <span id="ferngreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for FernGreen`

- <span id="ferngreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="ferngreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="ferngreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="ferngreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for FernGreen`

- <span id="ferngreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FernGreen`

- <span id="ferngreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for FernGreen`

##### `impl<U> TryFrom for FernGreen`

- <span id="ferngreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ferngreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FernGreen`

- <span id="ferngreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ferngreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SilverTree`

```rust
struct SilverTree;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for SilverTree`

- <span id="silvertree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SilverTree`

- <span id="silvertree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SilverTree`

- <span id="silvertree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for SilverTree`

- <span id="silvertree-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silvertree-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silvertree-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silvertree-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for SilverTree`

- <span id="silvertree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SilverTree`

- <span id="silvertree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SilverTree`

##### `impl<U> TryFrom for SilverTree`

- <span id="silvertree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="silvertree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SilverTree`

- <span id="silvertree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="silvertree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tradewind`

```rust
struct Tradewind;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Tradewind`

- <span id="tradewind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tradewind`

- <span id="tradewind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tradewind`

- <span id="tradewind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Tradewind`

- <span id="tradewind-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tradewind-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tradewind-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tradewind-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Tradewind`

- <span id="tradewind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tradewind`

- <span id="tradewind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Tradewind`

##### `impl<U> TryFrom for Tradewind`

- <span id="tradewind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tradewind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tradewind`

- <span id="tradewind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tradewind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ShakespeareBlue`

```rust
struct ShakespeareBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ShakespeareBlue`

- <span id="shakespeareblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ShakespeareBlue`

- <span id="shakespeareblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ShakespeareBlue`

- <span id="shakespeareblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ShakespeareBlue`

- <span id="shakespeareblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="shakespeareblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="shakespeareblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="shakespeareblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ShakespeareBlue`

- <span id="shakespeareblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ShakespeareBlue`

- <span id="shakespeareblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ShakespeareBlue`

##### `impl<U> TryFrom for ShakespeareBlue`

- <span id="shakespeareblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shakespeareblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ShakespeareBlue`

- <span id="shakespeareblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shakespeareblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkMalibuBlue`

```rust
struct DarkMalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkMalibuBlue`

- <span id="darkmalibublue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkMalibuBlue`

- <span id="darkmalibublue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkMalibuBlue`

- <span id="darkmalibublue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkMalibuBlue`

- <span id="darkmalibublue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmalibublue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmalibublue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmalibublue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkMalibuBlue`

- <span id="darkmalibublue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkMalibuBlue`

- <span id="darkmalibublue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkMalibuBlue`

##### `impl<U> TryFrom for DarkMalibuBlue`

- <span id="darkmalibublue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkmalibublue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkMalibuBlue`

- <span id="darkmalibublue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkmalibublue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkBrightGreen`

```rust
struct DarkBrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkBrightGreen`

- <span id="darkbrightgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkBrightGreen`

- <span id="darkbrightgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkBrightGreen`

- <span id="darkbrightgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkBrightGreen`

- <span id="darkbrightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkbrightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkbrightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkbrightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkBrightGreen`

- <span id="darkbrightgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkBrightGreen`

- <span id="darkbrightgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkBrightGreen`

##### `impl<U> TryFrom for DarkBrightGreen`

- <span id="darkbrightgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkbrightgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkBrightGreen`

- <span id="darkbrightgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkbrightgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkPastelGreen`

```rust
struct DarkPastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkPastelGreen`

- <span id="darkpastelgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkPastelGreen`

- <span id="darkpastelgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkPastelGreen`

- <span id="darkpastelgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkPastelGreen`

- <span id="darkpastelgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpastelgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpastelgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpastelgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkPastelGreen`

- <span id="darkpastelgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkPastelGreen`

- <span id="darkpastelgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkPastelGreen`

##### `impl<U> TryFrom for DarkPastelGreen`

- <span id="darkpastelgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkpastelgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkPastelGreen`

- <span id="darkpastelgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkpastelgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PastelGreen`

```rust
struct PastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PastelGreen`

- <span id="pastelgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PastelGreen`

- <span id="pastelgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PastelGreen`

- <span id="pastelgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PastelGreen`

- <span id="pastelgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pastelgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pastelgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pastelgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PastelGreen`

- <span id="pastelgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PastelGreen`

- <span id="pastelgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PastelGreen`

##### `impl<U> TryFrom for PastelGreen`

- <span id="pastelgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pastelgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PastelGreen`

- <span id="pastelgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pastelgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DownyTeal`

```rust
struct DownyTeal;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DownyTeal`

- <span id="downyteal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DownyTeal`

- <span id="downyteal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DownyTeal`

- <span id="downyteal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DownyTeal`

- <span id="downyteal-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="downyteal-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="downyteal-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="downyteal-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DownyTeal`

- <span id="downyteal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DownyTeal`

- <span id="downyteal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DownyTeal`

##### `impl<U> TryFrom for DownyTeal`

- <span id="downyteal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="downyteal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DownyTeal`

- <span id="downyteal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="downyteal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Viking`

```rust
struct Viking;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Viking`

- <span id="viking-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Viking`

- <span id="viking-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Viking`

- <span id="viking-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Viking`

- <span id="viking-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="viking-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="viking-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="viking-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Viking`

- <span id="viking-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Viking`

- <span id="viking-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Viking`

##### `impl<U> TryFrom for Viking`

- <span id="viking-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="viking-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Viking`

- <span id="viking-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="viking-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MalibuBlue`

```rust
struct MalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MalibuBlue`

- <span id="malibublue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MalibuBlue`

- <span id="malibublue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MalibuBlue`

- <span id="malibublue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MalibuBlue`

- <span id="malibublue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="malibublue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="malibublue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="malibublue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MalibuBlue`

- <span id="malibublue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MalibuBlue`

- <span id="malibublue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MalibuBlue`

##### `impl<U> TryFrom for MalibuBlue`

- <span id="malibublue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="malibublue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MalibuBlue`

- <span id="malibublue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="malibublue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightGreen`

```rust
struct BrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BrightGreen`

- <span id="brightgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightGreen`

- <span id="brightgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightGreen`

- <span id="brightgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightGreen`

- <span id="brightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BrightGreen`

- <span id="brightgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightGreen`

- <span id="brightgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightGreen`

##### `impl<U> TryFrom for BrightGreen`

- <span id="brightgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightGreen`

- <span id="brightgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkScreaminGreen`

```rust
struct DarkScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkScreaminGreen`

- <span id="darkscreamingreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkScreaminGreen`

- <span id="darkscreamingreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkScreaminGreen`

- <span id="darkscreamingreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkScreaminGreen`

- <span id="darkscreamingreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkscreamingreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkscreamingreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkscreamingreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkScreaminGreen`

- <span id="darkscreamingreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkScreaminGreen`

- <span id="darkscreamingreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkScreaminGreen`

##### `impl<U> TryFrom for DarkScreaminGreen`

- <span id="darkscreamingreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkscreamingreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkScreaminGreen`

- <span id="darkscreamingreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkscreamingreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScreaminGreen`

```rust
struct ScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ScreaminGreen`

- <span id="screamingreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScreaminGreen`

- <span id="screamingreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScreaminGreen`

- <span id="screamingreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ScreaminGreen`

- <span id="screamingreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="screamingreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="screamingreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="screamingreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ScreaminGreen`

- <span id="screamingreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScreaminGreen`

- <span id="screamingreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ScreaminGreen`

##### `impl<U> TryFrom for ScreaminGreen`

- <span id="screamingreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="screamingreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScreaminGreen`

- <span id="screamingreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="screamingreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkAquamarine`

```rust
struct DarkAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkAquamarine`

- <span id="darkaquamarine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkAquamarine`

- <span id="darkaquamarine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkAquamarine`

- <span id="darkaquamarine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkAquamarine`

- <span id="darkaquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkaquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkaquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkaquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkAquamarine`

- <span id="darkaquamarine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkAquamarine`

- <span id="darkaquamarine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkAquamarine`

##### `impl<U> TryFrom for DarkAquamarine`

- <span id="darkaquamarine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkaquamarine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkAquamarine`

- <span id="darkaquamarine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkaquamarine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Aquamarine`

```rust
struct Aquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Aquamarine`

- <span id="aquamarine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Aquamarine`

- <span id="aquamarine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Aquamarine`

- <span id="aquamarine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Aquamarine`

- <span id="aquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Aquamarine`

- <span id="aquamarine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Aquamarine`

- <span id="aquamarine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Aquamarine`

##### `impl<U> TryFrom for Aquamarine`

- <span id="aquamarine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aquamarine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Aquamarine`

- <span id="aquamarine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aquamarine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightAquamarine`

```rust
struct LightAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightAquamarine`

- <span id="lightaquamarine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightAquamarine`

- <span id="lightaquamarine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightAquamarine`

- <span id="lightaquamarine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightAquamarine`

- <span id="lightaquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightaquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightaquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightaquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightAquamarine`

- <span id="lightaquamarine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightAquamarine`

- <span id="lightaquamarine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightAquamarine`

##### `impl<U> TryFrom for LightAquamarine`

- <span id="lightaquamarine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightaquamarine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightAquamarine`

- <span id="lightaquamarine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightaquamarine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Maroon`

```rust
struct Maroon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Maroon`

- <span id="maroon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Maroon`

- <span id="maroon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Maroon`

- <span id="maroon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Maroon`

- <span id="maroon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="maroon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="maroon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="maroon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Maroon`

- <span id="maroon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Maroon`

- <span id="maroon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Maroon`

##### `impl<U> TryFrom for Maroon`

- <span id="maroon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maroon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Maroon`

- <span id="maroon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maroon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkFreshEggplant`

```rust
struct DarkFreshEggplant;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkFreshEggplant`

- <span id="darkfresheggplant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkFreshEggplant`

- <span id="darkfresheggplant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkFreshEggplant`

- <span id="darkfresheggplant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkFreshEggplant`

- <span id="darkfresheggplant-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfresheggplant-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfresheggplant-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfresheggplant-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkFreshEggplant`

- <span id="darkfresheggplant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkFreshEggplant`

- <span id="darkfresheggplant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkFreshEggplant`

##### `impl<U> TryFrom for DarkFreshEggplant`

- <span id="darkfresheggplant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkfresheggplant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkFreshEggplant`

- <span id="darkfresheggplant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkfresheggplant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightFreshEggplant`

```rust
struct LightFreshEggplant;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightFreshEggplant`

- <span id="lightfresheggplant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightFreshEggplant`

- <span id="lightfresheggplant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightFreshEggplant`

- <span id="lightfresheggplant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightFreshEggplant`

- <span id="lightfresheggplant-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightfresheggplant-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightfresheggplant-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightfresheggplant-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightFreshEggplant`

- <span id="lightfresheggplant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightFreshEggplant`

- <span id="lightfresheggplant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightFreshEggplant`

##### `impl<U> TryFrom for LightFreshEggplant`

- <span id="lightfresheggplant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightfresheggplant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightFreshEggplant`

- <span id="lightfresheggplant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightfresheggplant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Purple`

```rust
struct Purple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Purple`

- <span id="purple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Purple`

- <span id="purple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Purple`

- <span id="purple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Purple`

- <span id="purple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Purple`

- <span id="purple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Purple`

- <span id="purple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Purple`

##### `impl<U> TryFrom for Purple`

- <span id="purple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="purple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Purple`

- <span id="purple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="purple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ElectricViolet`

```rust
struct ElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ElectricViolet`

- <span id="electricviolet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ElectricViolet`

- <span id="electricviolet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ElectricViolet`

- <span id="electricviolet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ElectricViolet`

- <span id="electricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="electricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="electricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="electricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ElectricViolet`

- <span id="electricviolet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ElectricViolet`

- <span id="electricviolet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ElectricViolet`

##### `impl<U> TryFrom for ElectricViolet`

- <span id="electricviolet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="electricviolet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ElectricViolet`

- <span id="electricviolet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="electricviolet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightElectricViolet`

```rust
struct LightElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightElectricViolet`

- <span id="lightelectricviolet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightElectricViolet`

- <span id="lightelectricviolet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightElectricViolet`

- <span id="lightelectricviolet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightElectricViolet`

- <span id="lightelectricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightelectricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightelectricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightelectricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightElectricViolet`

- <span id="lightelectricviolet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightElectricViolet`

- <span id="lightelectricviolet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightElectricViolet`

##### `impl<U> TryFrom for LightElectricViolet`

- <span id="lightelectricviolet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightelectricviolet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightElectricViolet`

- <span id="lightelectricviolet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightelectricviolet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Brown`

```rust
struct Brown;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Brown`

- <span id="brown-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Brown`

- <span id="brown-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Brown`

- <span id="brown-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Brown`

- <span id="brown-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brown-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brown-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brown-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Brown`

- <span id="brown-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Brown`

- <span id="brown-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Brown`

##### `impl<U> TryFrom for Brown`

- <span id="brown-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brown-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Brown`

- <span id="brown-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brown-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CopperRose`

```rust
struct CopperRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CopperRose`

- <span id="copperrose-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CopperRose`

- <span id="copperrose-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CopperRose`

- <span id="copperrose-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CopperRose`

- <span id="copperrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CopperRose`

- <span id="copperrose-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CopperRose`

- <span id="copperrose-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CopperRose`

##### `impl<U> TryFrom for CopperRose`

- <span id="copperrose-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copperrose-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CopperRose`

- <span id="copperrose-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copperrose-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StrikemasterPurple`

```rust
struct StrikemasterPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for StrikemasterPurple`

- <span id="strikemasterpurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrikemasterPurple`

- <span id="strikemasterpurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrikemasterPurple`

- <span id="strikemasterpurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for StrikemasterPurple`

- <span id="strikemasterpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="strikemasterpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="strikemasterpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="strikemasterpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for StrikemasterPurple`

- <span id="strikemasterpurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StrikemasterPurple`

- <span id="strikemasterpurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StrikemasterPurple`

##### `impl<U> TryFrom for StrikemasterPurple`

- <span id="strikemasterpurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strikemasterpurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StrikemasterPurple`

- <span id="strikemasterpurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strikemasterpurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DelugePurple`

```rust
struct DelugePurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DelugePurple`

- <span id="delugepurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DelugePurple`

- <span id="delugepurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DelugePurple`

- <span id="delugepurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DelugePurple`

- <span id="delugepurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="delugepurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="delugepurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="delugepurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DelugePurple`

- <span id="delugepurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DelugePurple`

- <span id="delugepurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DelugePurple`

##### `impl<U> TryFrom for DelugePurple`

- <span id="delugepurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="delugepurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DelugePurple`

- <span id="delugepurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="delugepurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkMediumPurple`

```rust
struct DarkMediumPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkMediumPurple`

- <span id="darkmediumpurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkMediumPurple`

- <span id="darkmediumpurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkMediumPurple`

- <span id="darkmediumpurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkMediumPurple`

- <span id="darkmediumpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmediumpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmediumpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmediumpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkMediumPurple`

- <span id="darkmediumpurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkMediumPurple`

- <span id="darkmediumpurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkMediumPurple`

##### `impl<U> TryFrom for DarkMediumPurple`

- <span id="darkmediumpurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkmediumpurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkMediumPurple`

- <span id="darkmediumpurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkmediumpurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkHeliotropePurple`

```rust
struct DarkHeliotropePurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkHeliotropePurple`

- <span id="darkheliotropepurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkHeliotropePurple`

- <span id="darkheliotropepurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkHeliotropePurple`

- <span id="darkheliotropepurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkHeliotropePurple`

- <span id="darkheliotropepurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkheliotropepurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkheliotropepurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkheliotropepurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkHeliotropePurple`

- <span id="darkheliotropepurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkHeliotropePurple`

- <span id="darkheliotropepurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkHeliotropePurple`

##### `impl<U> TryFrom for DarkHeliotropePurple`

- <span id="darkheliotropepurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkheliotropepurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkHeliotropePurple`

- <span id="darkheliotropepurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkheliotropepurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Olive`

```rust
struct Olive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Olive`

- <span id="olive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Olive`

- <span id="olive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Olive`

- <span id="olive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Olive`

- <span id="olive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Olive`

- <span id="olive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Olive`

- <span id="olive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Olive`

##### `impl<U> TryFrom for Olive`

- <span id="olive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="olive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Olive`

- <span id="olive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="olive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClayCreekOlive`

```rust
struct ClayCreekOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ClayCreekOlive`

- <span id="claycreekolive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClayCreekOlive`

- <span id="claycreekolive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClayCreekOlive`

- <span id="claycreekolive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ClayCreekOlive`

- <span id="claycreekolive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="claycreekolive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="claycreekolive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="claycreekolive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ClayCreekOlive`

- <span id="claycreekolive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClayCreekOlive`

- <span id="claycreekolive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ClayCreekOlive`

##### `impl<U> TryFrom for ClayCreekOlive`

- <span id="claycreekolive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="claycreekolive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClayCreekOlive`

- <span id="claycreekolive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="claycreekolive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkGray`

```rust
struct DarkGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkGray`

- <span id="darkgray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkGray`

- <span id="darkgray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkGray`

- <span id="darkgray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkGray`

- <span id="darkgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkGray`

- <span id="darkgray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkGray`

- <span id="darkgray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkGray`

##### `impl<U> TryFrom for DarkGray`

- <span id="darkgray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkgray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkGray`

- <span id="darkgray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkgray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WildBlueYonder`

```rust
struct WildBlueYonder;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for WildBlueYonder`

- <span id="wildblueyonder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WildBlueYonder`

- <span id="wildblueyonder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WildBlueYonder`

- <span id="wildblueyonder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for WildBlueYonder`

- <span id="wildblueyonder-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildblueyonder-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildblueyonder-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildblueyonder-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for WildBlueYonder`

- <span id="wildblueyonder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WildBlueYonder`

- <span id="wildblueyonder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for WildBlueYonder`

##### `impl<U> TryFrom for WildBlueYonder`

- <span id="wildblueyonder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wildblueyonder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WildBlueYonder`

- <span id="wildblueyonder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wildblueyonder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChetwodeBlue`

```rust
struct ChetwodeBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ChetwodeBlue`

- <span id="chetwodeblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChetwodeBlue`

- <span id="chetwodeblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChetwodeBlue`

- <span id="chetwodeblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ChetwodeBlue`

- <span id="chetwodeblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chetwodeblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chetwodeblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chetwodeblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ChetwodeBlue`

- <span id="chetwodeblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChetwodeBlue`

- <span id="chetwodeblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ChetwodeBlue`

##### `impl<U> TryFrom for ChetwodeBlue`

- <span id="chetwodeblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chetwodeblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChetwodeBlue`

- <span id="chetwodeblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chetwodeblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SlateBlue`

```rust
struct SlateBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for SlateBlue`

- <span id="slateblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlateBlue`

- <span id="slateblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlateBlue`

- <span id="slateblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for SlateBlue`

- <span id="slateblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="slateblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="slateblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="slateblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for SlateBlue`

- <span id="slateblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SlateBlue`

- <span id="slateblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SlateBlue`

##### `impl<U> TryFrom for SlateBlue`

- <span id="slateblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slateblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlateBlue`

- <span id="slateblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slateblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightLimeade`

```rust
struct LightLimeade;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightLimeade`

- <span id="lightlimeade-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightLimeade`

- <span id="lightlimeade-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightLimeade`

- <span id="lightlimeade-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightLimeade`

- <span id="lightlimeade-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightlimeade-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightlimeade-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightlimeade-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightLimeade`

- <span id="lightlimeade-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightLimeade`

- <span id="lightlimeade-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightLimeade`

##### `impl<U> TryFrom for LightLimeade`

- <span id="lightlimeade-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightlimeade-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightLimeade`

- <span id="lightlimeade-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightlimeade-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChelseaCucumber`

```rust
struct ChelseaCucumber;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ChelseaCucumber`

- <span id="chelseacucumber-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChelseaCucumber`

- <span id="chelseacucumber-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChelseaCucumber`

- <span id="chelseacucumber-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ChelseaCucumber`

- <span id="chelseacucumber-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chelseacucumber-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chelseacucumber-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chelseacucumber-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ChelseaCucumber`

- <span id="chelseacucumber-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChelseaCucumber`

- <span id="chelseacucumber-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ChelseaCucumber`

##### `impl<U> TryFrom for ChelseaCucumber`

- <span id="chelseacucumber-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chelseacucumber-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChelseaCucumber`

- <span id="chelseacucumber-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chelseacucumber-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BayLeaf`

```rust
struct BayLeaf;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BayLeaf`

- <span id="bayleaf-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BayLeaf`

- <span id="bayleaf-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BayLeaf`

- <span id="bayleaf-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BayLeaf`

- <span id="bayleaf-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bayleaf-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bayleaf-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bayleaf-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BayLeaf`

- <span id="bayleaf-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BayLeaf`

- <span id="bayleaf-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BayLeaf`

##### `impl<U> TryFrom for BayLeaf`

- <span id="bayleaf-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bayleaf-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BayLeaf`

- <span id="bayleaf-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bayleaf-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GulfStream`

```rust
struct GulfStream;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for GulfStream`

- <span id="gulfstream-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GulfStream`

- <span id="gulfstream-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GulfStream`

- <span id="gulfstream-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for GulfStream`

- <span id="gulfstream-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gulfstream-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gulfstream-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gulfstream-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for GulfStream`

- <span id="gulfstream-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GulfStream`

- <span id="gulfstream-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GulfStream`

##### `impl<U> TryFrom for GulfStream`

- <span id="gulfstream-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gulfstream-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GulfStream`

- <span id="gulfstream-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gulfstream-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PoloBlue`

```rust
struct PoloBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PoloBlue`

- <span id="poloblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PoloBlue`

- <span id="poloblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PoloBlue`

- <span id="poloblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PoloBlue`

- <span id="poloblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="poloblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="poloblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="poloblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PoloBlue`

- <span id="poloblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PoloBlue`

- <span id="poloblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PoloBlue`

##### `impl<U> TryFrom for PoloBlue`

- <span id="poloblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="poloblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PoloBlue`

- <span id="poloblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="poloblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightMalibuBlue`

```rust
struct LightMalibuBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightMalibuBlue`

- <span id="lightmalibublue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightMalibuBlue`

- <span id="lightmalibublue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightMalibuBlue`

- <span id="lightmalibublue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightMalibuBlue`

- <span id="lightmalibublue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmalibublue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmalibublue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmalibublue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightMalibuBlue`

- <span id="lightmalibublue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightMalibuBlue`

- <span id="lightmalibublue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightMalibuBlue`

##### `impl<U> TryFrom for LightMalibuBlue`

- <span id="lightmalibublue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightmalibublue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightMalibuBlue`

- <span id="lightmalibublue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightmalibublue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Pistachio`

```rust
struct Pistachio;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Pistachio`

- <span id="pistachio-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pistachio`

- <span id="pistachio-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pistachio`

- <span id="pistachio-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Pistachio`

- <span id="pistachio-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pistachio-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pistachio-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pistachio-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Pistachio`

- <span id="pistachio-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Pistachio`

- <span id="pistachio-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Pistachio`

##### `impl<U> TryFrom for Pistachio`

- <span id="pistachio-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pistachio-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pistachio`

- <span id="pistachio-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pistachio-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightPastelGreen`

```rust
struct LightPastelGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightPastelGreen`

- <span id="lightpastelgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightPastelGreen`

- <span id="lightpastelgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightPastelGreen`

- <span id="lightpastelgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightPastelGreen`

- <span id="lightpastelgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightpastelgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightpastelgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightpastelgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightPastelGreen`

- <span id="lightpastelgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightPastelGreen`

- <span id="lightpastelgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightPastelGreen`

##### `impl<U> TryFrom for LightPastelGreen`

- <span id="lightpastelgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightpastelgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightPastelGreen`

- <span id="lightpastelgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightpastelgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkFeijoaGreen`

```rust
struct DarkFeijoaGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkFeijoaGreen`

- <span id="darkfeijoagreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkFeijoaGreen`

- <span id="darkfeijoagreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkFeijoaGreen`

- <span id="darkfeijoagreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkFeijoaGreen`

- <span id="darkfeijoagreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkfeijoagreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkfeijoagreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkfeijoagreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkFeijoaGreen`

- <span id="darkfeijoagreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkFeijoaGreen`

- <span id="darkfeijoagreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkFeijoaGreen`

##### `impl<U> TryFrom for DarkFeijoaGreen`

- <span id="darkfeijoagreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkfeijoagreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkFeijoaGreen`

- <span id="darkfeijoagreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkfeijoagreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VistaBlue`

```rust
struct VistaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for VistaBlue`

- <span id="vistablue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VistaBlue`

- <span id="vistablue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VistaBlue`

- <span id="vistablue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for VistaBlue`

- <span id="vistablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vistablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vistablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vistablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for VistaBlue`

- <span id="vistablue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VistaBlue`

- <span id="vistablue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for VistaBlue`

##### `impl<U> TryFrom for VistaBlue`

- <span id="vistablue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vistablue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VistaBlue`

- <span id="vistablue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vistablue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Bermuda`

```rust
struct Bermuda;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Bermuda`

- <span id="bermuda-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bermuda`

- <span id="bermuda-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bermuda`

- <span id="bermuda-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Bermuda`

- <span id="bermuda-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bermuda-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bermuda-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bermuda-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Bermuda`

- <span id="bermuda-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bermuda`

- <span id="bermuda-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Bermuda`

##### `impl<U> TryFrom for Bermuda`

- <span id="bermuda-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bermuda-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bermuda`

- <span id="bermuda-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bermuda-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkAnakiwaBlue`

```rust
struct DarkAnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkAnakiwaBlue`

- <span id="darkanakiwablue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkAnakiwaBlue`

- <span id="darkanakiwablue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkAnakiwaBlue`

- <span id="darkanakiwablue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkAnakiwaBlue`

- <span id="darkanakiwablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkanakiwablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkanakiwablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkanakiwablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkAnakiwaBlue`

- <span id="darkanakiwablue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkAnakiwaBlue`

- <span id="darkanakiwablue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkAnakiwaBlue`

##### `impl<U> TryFrom for DarkAnakiwaBlue`

- <span id="darkanakiwablue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkanakiwablue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkAnakiwaBlue`

- <span id="darkanakiwablue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkanakiwablue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChartreuseGreen`

```rust
struct ChartreuseGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ChartreuseGreen`

- <span id="chartreusegreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChartreuseGreen`

- <span id="chartreusegreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChartreuseGreen`

- <span id="chartreusegreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ChartreuseGreen`

- <span id="chartreusegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreusegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreusegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreusegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ChartreuseGreen`

- <span id="chartreusegreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChartreuseGreen`

- <span id="chartreusegreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ChartreuseGreen`

##### `impl<U> TryFrom for ChartreuseGreen`

- <span id="chartreusegreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chartreusegreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChartreuseGreen`

- <span id="chartreusegreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chartreusegreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightScreaminGreen`

```rust
struct LightScreaminGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightScreaminGreen`

- <span id="lightscreamingreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightScreaminGreen`

- <span id="lightscreamingreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightScreaminGreen`

- <span id="lightscreamingreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightScreaminGreen`

- <span id="lightscreamingreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightscreamingreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightscreamingreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightscreamingreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightScreaminGreen`

- <span id="lightscreamingreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightScreaminGreen`

- <span id="lightscreamingreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightScreaminGreen`

##### `impl<U> TryFrom for LightScreaminGreen`

- <span id="lightscreamingreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightscreamingreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightScreaminGreen`

- <span id="lightscreamingreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightscreamingreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkMintGreen`

```rust
struct DarkMintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkMintGreen`

- <span id="darkmintgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkMintGreen`

- <span id="darkmintgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkMintGreen`

- <span id="darkmintgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkMintGreen`

- <span id="darkmintgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmintgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmintgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmintgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkMintGreen`

- <span id="darkmintgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkMintGreen`

- <span id="darkmintgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkMintGreen`

##### `impl<U> TryFrom for DarkMintGreen`

- <span id="darkmintgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkmintgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkMintGreen`

- <span id="darkmintgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkmintgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MintGreen`

```rust
struct MintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MintGreen`

- <span id="mintgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MintGreen`

- <span id="mintgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MintGreen`

- <span id="mintgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MintGreen`

- <span id="mintgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mintgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mintgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mintgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MintGreen`

- <span id="mintgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MintGreen`

- <span id="mintgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MintGreen`

##### `impl<U> TryFrom for MintGreen`

- <span id="mintgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mintgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MintGreen`

- <span id="mintgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mintgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LighterAquamarine`

```rust
struct LighterAquamarine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LighterAquamarine`

- <span id="lighteraquamarine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LighterAquamarine`

- <span id="lighteraquamarine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LighterAquamarine`

- <span id="lighteraquamarine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LighterAquamarine`

- <span id="lighteraquamarine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighteraquamarine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighteraquamarine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighteraquamarine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LighterAquamarine`

- <span id="lighteraquamarine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LighterAquamarine`

- <span id="lighteraquamarine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LighterAquamarine`

##### `impl<U> TryFrom for LighterAquamarine`

- <span id="lighteraquamarine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lighteraquamarine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LighterAquamarine`

- <span id="lighteraquamarine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lighteraquamarine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AnakiwaBlue`

```rust
struct AnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for AnakiwaBlue`

- <span id="anakiwablue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnakiwaBlue`

- <span id="anakiwablue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnakiwaBlue`

- <span id="anakiwablue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for AnakiwaBlue`

- <span id="anakiwablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="anakiwablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="anakiwablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="anakiwablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for AnakiwaBlue`

- <span id="anakiwablue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AnakiwaBlue`

- <span id="anakiwablue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for AnakiwaBlue`

##### `impl<U> TryFrom for AnakiwaBlue`

- <span id="anakiwablue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anakiwablue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnakiwaBlue`

- <span id="anakiwablue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anakiwablue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightRed`

```rust
struct BrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BrightRed`

- <span id="brightred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightRed`

- <span id="brightred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightRed`

- <span id="brightred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightRed`

- <span id="brightred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BrightRed`

- <span id="brightred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightRed`

- <span id="brightred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightRed`

##### `impl<U> TryFrom for BrightRed`

- <span id="brightred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightRed`

- <span id="brightred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkFlirt`

```rust
struct DarkFlirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkFlirt`

- <span id="darkflirt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkFlirt`

- <span id="darkflirt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkFlirt`

- <span id="darkflirt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkFlirt`

- <span id="darkflirt-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkflirt-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkflirt-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkflirt-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkFlirt`

- <span id="darkflirt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkFlirt`

- <span id="darkflirt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkFlirt`

##### `impl<U> TryFrom for DarkFlirt`

- <span id="darkflirt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkflirt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkFlirt`

- <span id="darkflirt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkflirt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Flirt`

```rust
struct Flirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Flirt`

- <span id="flirt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flirt`

- <span id="flirt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flirt`

- <span id="flirt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Flirt`

- <span id="flirt-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flirt-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flirt-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flirt-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Flirt`

- <span id="flirt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flirt`

- <span id="flirt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Flirt`

##### `impl<U> TryFrom for Flirt`

- <span id="flirt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flirt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flirt`

- <span id="flirt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flirt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightFlirt`

```rust
struct LightFlirt;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightFlirt`

- <span id="lightflirt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightFlirt`

- <span id="lightflirt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightFlirt`

- <span id="lightflirt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightFlirt`

- <span id="lightflirt-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightflirt-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightflirt-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightflirt-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightFlirt`

- <span id="lightflirt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightFlirt`

- <span id="lightflirt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightFlirt`

##### `impl<U> TryFrom for LightFlirt`

- <span id="lightflirt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightflirt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightFlirt`

- <span id="lightflirt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightflirt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkViolet`

```rust
struct DarkViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkViolet`

- <span id="darkviolet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkViolet`

- <span id="darkviolet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkViolet`

- <span id="darkviolet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkViolet`

- <span id="darkviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkViolet`

- <span id="darkviolet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkViolet`

- <span id="darkviolet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkViolet`

##### `impl<U> TryFrom for DarkViolet`

- <span id="darkviolet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkviolet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkViolet`

- <span id="darkviolet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkviolet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightElectricViolet`

```rust
struct BrightElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BrightElectricViolet`

- <span id="brightelectricviolet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightElectricViolet`

- <span id="brightelectricviolet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightElectricViolet`

- <span id="brightelectricviolet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightElectricViolet`

- <span id="brightelectricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightelectricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightelectricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightelectricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BrightElectricViolet`

- <span id="brightelectricviolet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightElectricViolet`

- <span id="brightelectricviolet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightElectricViolet`

##### `impl<U> TryFrom for BrightElectricViolet`

- <span id="brightelectricviolet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightelectricviolet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightElectricViolet`

- <span id="brightelectricviolet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightelectricviolet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RoseofSharonOrange`

```rust
struct RoseofSharonOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for RoseofSharonOrange`

- <span id="roseofsharonorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RoseofSharonOrange`

- <span id="roseofsharonorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RoseofSharonOrange`

- <span id="roseofsharonorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for RoseofSharonOrange`

- <span id="roseofsharonorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="roseofsharonorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="roseofsharonorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="roseofsharonorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for RoseofSharonOrange`

- <span id="roseofsharonorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RoseofSharonOrange`

- <span id="roseofsharonorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for RoseofSharonOrange`

##### `impl<U> TryFrom for RoseofSharonOrange`

- <span id="roseofsharonorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="roseofsharonorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RoseofSharonOrange`

- <span id="roseofsharonorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="roseofsharonorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatrixPink`

```rust
struct MatrixPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MatrixPink`

- <span id="matrixpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatrixPink`

- <span id="matrixpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatrixPink`

- <span id="matrixpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MatrixPink`

- <span id="matrixpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="matrixpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="matrixpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="matrixpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MatrixPink`

- <span id="matrixpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatrixPink`

- <span id="matrixpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MatrixPink`

##### `impl<U> TryFrom for MatrixPink`

- <span id="matrixpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matrixpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatrixPink`

- <span id="matrixpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matrixpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TapestryPink`

```rust
struct TapestryPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for TapestryPink`

- <span id="tapestrypink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TapestryPink`

- <span id="tapestrypink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TapestryPink`

- <span id="tapestrypink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for TapestryPink`

- <span id="tapestrypink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tapestrypink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tapestrypink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tapestrypink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for TapestryPink`

- <span id="tapestrypink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TapestryPink`

- <span id="tapestrypink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for TapestryPink`

##### `impl<U> TryFrom for TapestryPink`

- <span id="tapestrypink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tapestrypink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TapestryPink`

- <span id="tapestrypink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tapestrypink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FuchsiaPink`

```rust
struct FuchsiaPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for FuchsiaPink`

- <span id="fuchsiapink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FuchsiaPink`

- <span id="fuchsiapink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FuchsiaPink`

- <span id="fuchsiapink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for FuchsiaPink`

- <span id="fuchsiapink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsiapink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsiapink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsiapink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for FuchsiaPink`

- <span id="fuchsiapink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FuchsiaPink`

- <span id="fuchsiapink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for FuchsiaPink`

##### `impl<U> TryFrom for FuchsiaPink`

- <span id="fuchsiapink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fuchsiapink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FuchsiaPink`

- <span id="fuchsiapink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fuchsiapink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MediumPurple`

```rust
struct MediumPurple;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MediumPurple`

- <span id="mediumpurple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MediumPurple`

- <span id="mediumpurple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MediumPurple`

- <span id="mediumpurple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MediumPurple`

- <span id="mediumpurple-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumpurple-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumpurple-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumpurple-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MediumPurple`

- <span id="mediumpurple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MediumPurple`

- <span id="mediumpurple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MediumPurple`

##### `impl<U> TryFrom for MediumPurple`

- <span id="mediumpurple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mediumpurple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MediumPurple`

- <span id="mediumpurple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mediumpurple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Heliotrope`

```rust
struct Heliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Heliotrope`

- <span id="heliotrope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Heliotrope`

- <span id="heliotrope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Heliotrope`

- <span id="heliotrope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Heliotrope`

- <span id="heliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="heliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="heliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="heliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Heliotrope`

- <span id="heliotrope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Heliotrope`

- <span id="heliotrope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Heliotrope`

##### `impl<U> TryFrom for Heliotrope`

- <span id="heliotrope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="heliotrope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Heliotrope`

- <span id="heliotrope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="heliotrope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PirateGold`

```rust
struct PirateGold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PirateGold`

- <span id="pirategold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PirateGold`

- <span id="pirategold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PirateGold`

- <span id="pirategold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PirateGold`

- <span id="pirategold-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pirategold-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pirategold-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pirategold-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PirateGold`

- <span id="pirategold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PirateGold`

- <span id="pirategold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PirateGold`

##### `impl<U> TryFrom for PirateGold`

- <span id="pirategold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pirategold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PirateGold`

- <span id="pirategold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pirategold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MuesliOrange`

```rust
struct MuesliOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MuesliOrange`

- <span id="muesliorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MuesliOrange`

- <span id="muesliorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MuesliOrange`

- <span id="muesliorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MuesliOrange`

- <span id="muesliorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="muesliorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="muesliorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="muesliorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MuesliOrange`

- <span id="muesliorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MuesliOrange`

- <span id="muesliorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MuesliOrange`

##### `impl<U> TryFrom for MuesliOrange`

- <span id="muesliorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="muesliorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MuesliOrange`

- <span id="muesliorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="muesliorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PharlapPink`

```rust
struct PharlapPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PharlapPink`

- <span id="pharlappink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PharlapPink`

- <span id="pharlappink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PharlapPink`

- <span id="pharlappink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PharlapPink`

- <span id="pharlappink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pharlappink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pharlappink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pharlappink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PharlapPink`

- <span id="pharlappink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PharlapPink`

- <span id="pharlappink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PharlapPink`

##### `impl<U> TryFrom for PharlapPink`

- <span id="pharlappink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pharlappink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PharlapPink`

- <span id="pharlappink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pharlappink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Bouquet`

```rust
struct Bouquet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Bouquet`

- <span id="bouquet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bouquet`

- <span id="bouquet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bouquet`

- <span id="bouquet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Bouquet`

- <span id="bouquet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bouquet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bouquet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bouquet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Bouquet`

- <span id="bouquet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bouquet`

- <span id="bouquet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Bouquet`

##### `impl<U> TryFrom for Bouquet`

- <span id="bouquet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bouquet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bouquet`

- <span id="bouquet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bouquet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lavender`

```rust
struct Lavender;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Lavender`

- <span id="lavender-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lavender`

- <span id="lavender-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lavender`

- <span id="lavender-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Lavender`

- <span id="lavender-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavender-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavender-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavender-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Lavender`

- <span id="lavender-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lavender`

- <span id="lavender-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Lavender`

##### `impl<U> TryFrom for Lavender`

- <span id="lavender-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lavender-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lavender`

- <span id="lavender-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lavender-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightHeliotrope`

```rust
struct LightHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightHeliotrope`

- <span id="lightheliotrope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightHeliotrope`

- <span id="lightheliotrope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightHeliotrope`

- <span id="lightheliotrope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightHeliotrope`

- <span id="lightheliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightheliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightheliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightheliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightHeliotrope`

- <span id="lightheliotrope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightHeliotrope`

- <span id="lightheliotrope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightHeliotrope`

##### `impl<U> TryFrom for LightHeliotrope`

- <span id="lightheliotrope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightheliotrope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightHeliotrope`

- <span id="lightheliotrope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightheliotrope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuddhaGold`

```rust
struct BuddhaGold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BuddhaGold`

- <span id="buddhagold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuddhaGold`

- <span id="buddhagold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuddhaGold`

- <span id="buddhagold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BuddhaGold`

- <span id="buddhagold-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="buddhagold-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="buddhagold-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="buddhagold-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BuddhaGold`

- <span id="buddhagold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuddhaGold`

- <span id="buddhagold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BuddhaGold`

##### `impl<U> TryFrom for BuddhaGold`

- <span id="buddhagold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="buddhagold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuddhaGold`

- <span id="buddhagold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="buddhagold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OliveGreen`

```rust
struct OliveGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for OliveGreen`

- <span id="olivegreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OliveGreen`

- <span id="olivegreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OliveGreen`

- <span id="olivegreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for OliveGreen`

- <span id="olivegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="olivegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="olivegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="olivegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for OliveGreen`

- <span id="olivegreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OliveGreen`

- <span id="olivegreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for OliveGreen`

##### `impl<U> TryFrom for OliveGreen`

- <span id="olivegreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="olivegreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OliveGreen`

- <span id="olivegreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="olivegreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HillaryOlive`

```rust
struct HillaryOlive;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for HillaryOlive`

- <span id="hillaryolive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HillaryOlive`

- <span id="hillaryolive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HillaryOlive`

- <span id="hillaryolive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for HillaryOlive`

- <span id="hillaryolive-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hillaryolive-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hillaryolive-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hillaryolive-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for HillaryOlive`

- <span id="hillaryolive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HillaryOlive`

- <span id="hillaryolive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HillaryOlive`

##### `impl<U> TryFrom for HillaryOlive`

- <span id="hillaryolive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hillaryolive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HillaryOlive`

- <span id="hillaryolive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hillaryolive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SilverChalice`

```rust
struct SilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for SilverChalice`

- <span id="silverchalice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SilverChalice`

- <span id="silverchalice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SilverChalice`

- <span id="silverchalice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for SilverChalice`

- <span id="silverchalice-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silverchalice-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silverchalice-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silverchalice-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for SilverChalice`

- <span id="silverchalice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SilverChalice`

- <span id="silverchalice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SilverChalice`

##### `impl<U> TryFrom for SilverChalice`

- <span id="silverchalice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="silverchalice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SilverChalice`

- <span id="silverchalice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="silverchalice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WistfulLilac`

```rust
struct WistfulLilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for WistfulLilac`

- <span id="wistfullilac-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WistfulLilac`

- <span id="wistfullilac-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WistfulLilac`

- <span id="wistfullilac-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for WistfulLilac`

- <span id="wistfullilac-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wistfullilac-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wistfullilac-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wistfullilac-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for WistfulLilac`

- <span id="wistfullilac-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WistfulLilac`

- <span id="wistfullilac-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for WistfulLilac`

##### `impl<U> TryFrom for WistfulLilac`

- <span id="wistfullilac-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wistfullilac-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WistfulLilac`

- <span id="wistfullilac-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wistfullilac-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MelroseLilac`

```rust
struct MelroseLilac;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MelroseLilac`

- <span id="melroselilac-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MelroseLilac`

- <span id="melroselilac-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MelroseLilac`

- <span id="melroselilac-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MelroseLilac`

- <span id="melroselilac-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="melroselilac-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="melroselilac-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="melroselilac-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MelroseLilac`

- <span id="melroselilac-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MelroseLilac`

- <span id="melroselilac-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MelroseLilac`

##### `impl<U> TryFrom for MelroseLilac`

- <span id="melroselilac-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="melroselilac-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MelroseLilac`

- <span id="melroselilac-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="melroselilac-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RioGrandeGreen`

```rust
struct RioGrandeGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for RioGrandeGreen`

- <span id="riograndegreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RioGrandeGreen`

- <span id="riograndegreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RioGrandeGreen`

- <span id="riograndegreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for RioGrandeGreen`

- <span id="riograndegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="riograndegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="riograndegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="riograndegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for RioGrandeGreen`

- <span id="riograndegreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RioGrandeGreen`

- <span id="riograndegreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for RioGrandeGreen`

##### `impl<U> TryFrom for RioGrandeGreen`

- <span id="riograndegreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="riograndegreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RioGrandeGreen`

- <span id="riograndegreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="riograndegreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ConiferGreen`

```rust
struct ConiferGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ConiferGreen`

- <span id="conifergreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConiferGreen`

- <span id="conifergreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConiferGreen`

- <span id="conifergreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ConiferGreen`

- <span id="conifergreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="conifergreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="conifergreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="conifergreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ConiferGreen`

- <span id="conifergreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ConiferGreen`

- <span id="conifergreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ConiferGreen`

##### `impl<U> TryFrom for ConiferGreen`

- <span id="conifergreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="conifergreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConiferGreen`

- <span id="conifergreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="conifergreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Feijoa`

```rust
struct Feijoa;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Feijoa`

- <span id="feijoa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Feijoa`

- <span id="feijoa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Feijoa`

- <span id="feijoa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Feijoa`

- <span id="feijoa-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="feijoa-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="feijoa-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="feijoa-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Feijoa`

- <span id="feijoa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Feijoa`

- <span id="feijoa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Feijoa`

##### `impl<U> TryFrom for Feijoa`

- <span id="feijoa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="feijoa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Feijoa`

- <span id="feijoa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="feijoa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PixieGreen`

```rust
struct PixieGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PixieGreen`

- <span id="pixiegreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PixieGreen`

- <span id="pixiegreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PixieGreen`

- <span id="pixiegreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PixieGreen`

- <span id="pixiegreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pixiegreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pixiegreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pixiegreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PixieGreen`

- <span id="pixiegreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PixieGreen`

- <span id="pixiegreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PixieGreen`

##### `impl<U> TryFrom for PixieGreen`

- <span id="pixiegreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pixiegreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PixieGreen`

- <span id="pixiegreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pixiegreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `JungleMist`

```rust
struct JungleMist;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for JungleMist`

- <span id="junglemist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for JungleMist`

- <span id="junglemist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for JungleMist`

- <span id="junglemist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for JungleMist`

- <span id="junglemist-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="junglemist-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="junglemist-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="junglemist-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for JungleMist`

- <span id="junglemist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for JungleMist`

- <span id="junglemist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for JungleMist`

##### `impl<U> TryFrom for JungleMist`

- <span id="junglemist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="junglemist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for JungleMist`

- <span id="junglemist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="junglemist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightAnakiwaBlue`

```rust
struct LightAnakiwaBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightAnakiwaBlue`

- <span id="lightanakiwablue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightAnakiwaBlue`

- <span id="lightanakiwablue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightAnakiwaBlue`

- <span id="lightanakiwablue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightAnakiwaBlue`

- <span id="lightanakiwablue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightanakiwablue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightanakiwablue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightanakiwablue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightAnakiwaBlue`

- <span id="lightanakiwablue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightAnakiwaBlue`

- <span id="lightanakiwablue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightAnakiwaBlue`

##### `impl<U> TryFrom for LightAnakiwaBlue`

- <span id="lightanakiwablue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightanakiwablue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightAnakiwaBlue`

- <span id="lightanakiwablue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightanakiwablue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lime`

```rust
struct Lime;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Lime`

- <span id="lime-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lime`

- <span id="lime-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lime`

- <span id="lime-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Lime`

- <span id="lime-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lime-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lime-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lime-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Lime`

- <span id="lime-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lime`

- <span id="lime-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Lime`

##### `impl<U> TryFrom for Lime`

- <span id="lime-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lime-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lime`

- <span id="lime-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lime-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GreenYellow`

```rust
struct GreenYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for GreenYellow`

- <span id="greenyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GreenYellow`

- <span id="greenyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GreenYellow`

- <span id="greenyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for GreenYellow`

- <span id="greenyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="greenyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="greenyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="greenyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for GreenYellow`

- <span id="greenyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GreenYellow`

- <span id="greenyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GreenYellow`

##### `impl<U> TryFrom for GreenYellow`

- <span id="greenyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="greenyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GreenYellow`

- <span id="greenyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="greenyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightMintGreen`

```rust
struct LightMintGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightMintGreen`

- <span id="lightmintgreen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightMintGreen`

- <span id="lightmintgreen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightMintGreen`

- <span id="lightmintgreen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightMintGreen`

- <span id="lightmintgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmintgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmintgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmintgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightMintGreen`

- <span id="lightmintgreen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightMintGreen`

- <span id="lightmintgreen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightMintGreen`

##### `impl<U> TryFrom for LightMintGreen`

- <span id="lightmintgreen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightmintgreen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightMintGreen`

- <span id="lightmintgreen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightmintgreen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Celadon`

```rust
struct Celadon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Celadon`

- <span id="celadon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Celadon`

- <span id="celadon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Celadon`

- <span id="celadon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Celadon`

- <span id="celadon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="celadon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="celadon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="celadon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Celadon`

- <span id="celadon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Celadon`

- <span id="celadon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Celadon`

##### `impl<U> TryFrom for Celadon`

- <span id="celadon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="celadon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Celadon`

- <span id="celadon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="celadon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AeroBlue`

```rust
struct AeroBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for AeroBlue`

- <span id="aeroblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AeroBlue`

- <span id="aeroblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AeroBlue`

- <span id="aeroblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for AeroBlue`

- <span id="aeroblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="aeroblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="aeroblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="aeroblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for AeroBlue`

- <span id="aeroblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AeroBlue`

- <span id="aeroblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for AeroBlue`

##### `impl<U> TryFrom for AeroBlue`

- <span id="aeroblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="aeroblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AeroBlue`

- <span id="aeroblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="aeroblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FrenchPassLightBlue`

```rust
struct FrenchPassLightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for FrenchPassLightBlue`

- <span id="frenchpasslightblue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FrenchPassLightBlue`

- <span id="frenchpasslightblue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrenchPassLightBlue`

- <span id="frenchpasslightblue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for FrenchPassLightBlue`

- <span id="frenchpasslightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="frenchpasslightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="frenchpasslightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="frenchpasslightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for FrenchPassLightBlue`

- <span id="frenchpasslightblue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrenchPassLightBlue`

- <span id="frenchpasslightblue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for FrenchPassLightBlue`

##### `impl<U> TryFrom for FrenchPassLightBlue`

- <span id="frenchpasslightblue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frenchpasslightblue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FrenchPassLightBlue`

- <span id="frenchpasslightblue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frenchpasslightblue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GuardsmanRed`

```rust
struct GuardsmanRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for GuardsmanRed`

- <span id="guardsmanred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GuardsmanRed`

- <span id="guardsmanred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GuardsmanRed`

- <span id="guardsmanred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for GuardsmanRed`

- <span id="guardsmanred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="guardsmanred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="guardsmanred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="guardsmanred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for GuardsmanRed`

- <span id="guardsmanred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GuardsmanRed`

- <span id="guardsmanred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GuardsmanRed`

##### `impl<U> TryFrom for GuardsmanRed`

- <span id="guardsmanred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="guardsmanred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GuardsmanRed`

- <span id="guardsmanred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="guardsmanred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RazzmatazzCerise`

```rust
struct RazzmatazzCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for RazzmatazzCerise`

- <span id="razzmatazzcerise-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RazzmatazzCerise`

- <span id="razzmatazzcerise-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RazzmatazzCerise`

- <span id="razzmatazzcerise-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for RazzmatazzCerise`

- <span id="razzmatazzcerise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="razzmatazzcerise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="razzmatazzcerise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="razzmatazzcerise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for RazzmatazzCerise`

- <span id="razzmatazzcerise-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RazzmatazzCerise`

- <span id="razzmatazzcerise-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for RazzmatazzCerise`

##### `impl<U> TryFrom for RazzmatazzCerise`

- <span id="razzmatazzcerise-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="razzmatazzcerise-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RazzmatazzCerise`

- <span id="razzmatazzcerise-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="razzmatazzcerise-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MediumVioletRed`

```rust
struct MediumVioletRed;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MediumVioletRed`

- <span id="mediumvioletred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MediumVioletRed`

- <span id="mediumvioletred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MediumVioletRed`

- <span id="mediumvioletred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MediumVioletRed`

- <span id="mediumvioletred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mediumvioletred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mediumvioletred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mediumvioletred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MediumVioletRed`

- <span id="mediumvioletred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MediumVioletRed`

- <span id="mediumvioletred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MediumVioletRed`

##### `impl<U> TryFrom for MediumVioletRed`

- <span id="mediumvioletred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mediumvioletred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MediumVioletRed`

- <span id="mediumvioletred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mediumvioletred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HollywoodCerise`

```rust
struct HollywoodCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for HollywoodCerise`

- <span id="hollywoodcerise-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HollywoodCerise`

- <span id="hollywoodcerise-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HollywoodCerise`

- <span id="hollywoodcerise-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for HollywoodCerise`

- <span id="hollywoodcerise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hollywoodcerise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hollywoodcerise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hollywoodcerise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for HollywoodCerise`

- <span id="hollywoodcerise-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HollywoodCerise`

- <span id="hollywoodcerise-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HollywoodCerise`

##### `impl<U> TryFrom for HollywoodCerise`

- <span id="hollywoodcerise-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hollywoodcerise-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HollywoodCerise`

- <span id="hollywoodcerise-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hollywoodcerise-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkPurplePizzazz`

```rust
struct DarkPurplePizzazz;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkpurplepizzazz-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkpurplepizzazz-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkPurplePizzazz`

##### `impl<U> TryFrom for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkpurplepizzazz-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkPurplePizzazz`

- <span id="darkpurplepizzazz-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkpurplepizzazz-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrighterElectricViolet`

```rust
struct BrighterElectricViolet;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BrighterElectricViolet`

- <span id="brighterelectricviolet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrighterElectricViolet`

- <span id="brighterelectricviolet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrighterElectricViolet`

- <span id="brighterelectricviolet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrighterElectricViolet`

- <span id="brighterelectricviolet-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brighterelectricviolet-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brighterelectricviolet-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brighterelectricviolet-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BrighterElectricViolet`

- <span id="brighterelectricviolet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrighterElectricViolet`

- <span id="brighterelectricviolet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrighterElectricViolet`

##### `impl<U> TryFrom for BrighterElectricViolet`

- <span id="brighterelectricviolet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brighterelectricviolet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrighterElectricViolet`

- <span id="brighterelectricviolet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brighterelectricviolet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TennOrange`

```rust
struct TennOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for TennOrange`

- <span id="tennorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TennOrange`

- <span id="tennorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TennOrange`

- <span id="tennorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for TennOrange`

- <span id="tennorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tennorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tennorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tennorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for TennOrange`

- <span id="tennorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TennOrange`

- <span id="tennorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for TennOrange`

##### `impl<U> TryFrom for TennOrange`

- <span id="tennorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tennorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TennOrange`

- <span id="tennorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tennorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RomanOrange`

```rust
struct RomanOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for RomanOrange`

- <span id="romanorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RomanOrange`

- <span id="romanorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RomanOrange`

- <span id="romanorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for RomanOrange`

- <span id="romanorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="romanorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="romanorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="romanorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for RomanOrange`

- <span id="romanorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RomanOrange`

- <span id="romanorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for RomanOrange`

##### `impl<U> TryFrom for RomanOrange`

- <span id="romanorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="romanorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RomanOrange`

- <span id="romanorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="romanorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CranberryPink`

```rust
struct CranberryPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CranberryPink`

- <span id="cranberrypink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CranberryPink`

- <span id="cranberrypink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CranberryPink`

- <span id="cranberrypink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CranberryPink`

- <span id="cranberrypink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cranberrypink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cranberrypink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cranberrypink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CranberryPink`

- <span id="cranberrypink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CranberryPink`

- <span id="cranberrypink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CranberryPink`

##### `impl<U> TryFrom for CranberryPink`

- <span id="cranberrypink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cranberrypink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CranberryPink`

- <span id="cranberrypink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cranberrypink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HopbushPink`

```rust
struct HopbushPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for HopbushPink`

- <span id="hopbushpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HopbushPink`

- <span id="hopbushpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HopbushPink`

- <span id="hopbushpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for HopbushPink`

- <span id="hopbushpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hopbushpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hopbushpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hopbushpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for HopbushPink`

- <span id="hopbushpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HopbushPink`

- <span id="hopbushpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HopbushPink`

##### `impl<U> TryFrom for HopbushPink`

- <span id="hopbushpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hopbushpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HopbushPink`

- <span id="hopbushpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hopbushpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Orchid`

```rust
struct Orchid;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Orchid`

- <span id="orchid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Orchid`

- <span id="orchid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Orchid`

- <span id="orchid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Orchid`

- <span id="orchid-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="orchid-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="orchid-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="orchid-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Orchid`

- <span id="orchid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Orchid`

- <span id="orchid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Orchid`

##### `impl<U> TryFrom for Orchid`

- <span id="orchid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="orchid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Orchid`

- <span id="orchid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="orchid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LighterHeliotrope`

```rust
struct LighterHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LighterHeliotrope`

- <span id="lighterheliotrope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LighterHeliotrope`

- <span id="lighterheliotrope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LighterHeliotrope`

- <span id="lighterheliotrope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LighterHeliotrope`

- <span id="lighterheliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighterheliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighterheliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighterheliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LighterHeliotrope`

- <span id="lighterheliotrope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LighterHeliotrope`

- <span id="lighterheliotrope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LighterHeliotrope`

##### `impl<U> TryFrom for LighterHeliotrope`

- <span id="lighterheliotrope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lighterheliotrope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LighterHeliotrope`

- <span id="lighterheliotrope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lighterheliotrope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MangoTango`

```rust
struct MangoTango;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MangoTango`

- <span id="mangotango-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MangoTango`

- <span id="mangotango-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MangoTango`

- <span id="mangotango-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MangoTango`

- <span id="mangotango-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mangotango-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mangotango-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mangotango-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MangoTango`

- <span id="mangotango-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MangoTango`

- <span id="mangotango-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MangoTango`

##### `impl<U> TryFrom for MangoTango`

- <span id="mangotango-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mangotango-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MangoTango`

- <span id="mangotango-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mangotango-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Copperfield`

```rust
struct Copperfield;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Copperfield`

- <span id="copperfield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Copperfield`

- <span id="copperfield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Copperfield`

- <span id="copperfield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Copperfield`

- <span id="copperfield-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="copperfield-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="copperfield-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="copperfield-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Copperfield`

- <span id="copperfield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Copperfield`

- <span id="copperfield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Copperfield`

##### `impl<U> TryFrom for Copperfield`

- <span id="copperfield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copperfield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Copperfield`

- <span id="copperfield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copperfield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SeaPink`

```rust
struct SeaPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for SeaPink`

- <span id="seapink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeaPink`

- <span id="seapink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeaPink`

- <span id="seapink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for SeaPink`

- <span id="seapink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="seapink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="seapink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="seapink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for SeaPink`

- <span id="seapink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeaPink`

- <span id="seapink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SeaPink`

##### `impl<U> TryFrom for SeaPink`

- <span id="seapink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seapink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeaPink`

- <span id="seapink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seapink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CanCanPink`

```rust
struct CanCanPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CanCanPink`

- <span id="cancanpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CanCanPink`

- <span id="cancanpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CanCanPink`

- <span id="cancanpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CanCanPink`

- <span id="cancanpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cancanpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cancanpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cancanpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CanCanPink`

- <span id="cancanpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CanCanPink`

- <span id="cancanpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CanCanPink`

##### `impl<U> TryFrom for CanCanPink`

- <span id="cancanpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cancanpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CanCanPink`

- <span id="cancanpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cancanpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightOrchid`

```rust
struct LightOrchid;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightOrchid`

- <span id="lightorchid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightOrchid`

- <span id="lightorchid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightOrchid`

- <span id="lightorchid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightOrchid`

- <span id="lightorchid-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightorchid-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightorchid-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightorchid-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightOrchid`

- <span id="lightorchid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightOrchid`

- <span id="lightorchid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightOrchid`

##### `impl<U> TryFrom for LightOrchid`

- <span id="lightorchid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightorchid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightOrchid`

- <span id="lightorchid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightorchid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BrightHeliotrope`

```rust
struct BrightHeliotrope;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BrightHeliotrope`

- <span id="brightheliotrope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BrightHeliotrope`

- <span id="brightheliotrope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BrightHeliotrope`

- <span id="brightheliotrope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BrightHeliotrope`

- <span id="brightheliotrope-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightheliotrope-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightheliotrope-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="brightheliotrope-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BrightHeliotrope`

- <span id="brightheliotrope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BrightHeliotrope`

- <span id="brightheliotrope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BrightHeliotrope`

##### `impl<U> TryFrom for BrightHeliotrope`

- <span id="brightheliotrope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brightheliotrope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BrightHeliotrope`

- <span id="brightheliotrope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brightheliotrope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkCorn`

```rust
struct DarkCorn;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkCorn`

- <span id="darkcorn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkCorn`

- <span id="darkcorn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkCorn`

- <span id="darkcorn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkCorn`

- <span id="darkcorn-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcorn-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcorn-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcorn-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkCorn`

- <span id="darkcorn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkCorn`

- <span id="darkcorn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkCorn`

##### `impl<U> TryFrom for DarkCorn`

- <span id="darkcorn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkcorn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkCorn`

- <span id="darkcorn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkcorn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkTachaOrange`

```rust
struct DarkTachaOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkTachaOrange`

- <span id="darktachaorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkTachaOrange`

- <span id="darktachaorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkTachaOrange`

- <span id="darktachaorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkTachaOrange`

- <span id="darktachaorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktachaorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktachaorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktachaorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkTachaOrange`

- <span id="darktachaorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkTachaOrange`

- <span id="darktachaorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkTachaOrange`

##### `impl<U> TryFrom for DarkTachaOrange`

- <span id="darktachaorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darktachaorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkTachaOrange`

- <span id="darktachaorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darktachaorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TanBeige`

```rust
struct TanBeige;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for TanBeige`

- <span id="tanbeige-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TanBeige`

- <span id="tanbeige-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TanBeige`

- <span id="tanbeige-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for TanBeige`

- <span id="tanbeige-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tanbeige-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tanbeige-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tanbeige-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for TanBeige`

- <span id="tanbeige-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TanBeige`

- <span id="tanbeige-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for TanBeige`

##### `impl<U> TryFrom for TanBeige`

- <span id="tanbeige-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tanbeige-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TanBeige`

- <span id="tanbeige-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tanbeige-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClamShell`

```rust
struct ClamShell;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ClamShell`

- <span id="clamshell-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClamShell`

- <span id="clamshell-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClamShell`

- <span id="clamshell-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ClamShell`

- <span id="clamshell-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="clamshell-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="clamshell-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="clamshell-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ClamShell`

- <span id="clamshell-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClamShell`

- <span id="clamshell-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ClamShell`

##### `impl<U> TryFrom for ClamShell`

- <span id="clamshell-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clamshell-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClamShell`

- <span id="clamshell-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clamshell-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThistlePink`

```rust
struct ThistlePink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ThistlePink`

- <span id="thistlepink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThistlePink`

- <span id="thistlepink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThistlePink`

- <span id="thistlepink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ThistlePink`

- <span id="thistlepink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="thistlepink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="thistlepink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="thistlepink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ThistlePink`

- <span id="thistlepink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThistlePink`

- <span id="thistlepink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ThistlePink`

##### `impl<U> TryFrom for ThistlePink`

- <span id="thistlepink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="thistlepink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThistlePink`

- <span id="thistlepink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="thistlepink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Mauve`

```rust
struct Mauve;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Mauve`

- <span id="mauve-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mauve`

- <span id="mauve-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mauve`

- <span id="mauve-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Mauve`

- <span id="mauve-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mauve-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mauve-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mauve-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Mauve`

- <span id="mauve-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Mauve`

- <span id="mauve-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Mauve`

##### `impl<U> TryFrom for Mauve`

- <span id="mauve-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mauve-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mauve`

- <span id="mauve-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mauve-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Corn`

```rust
struct Corn;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Corn`

- <span id="corn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Corn`

- <span id="corn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Corn`

- <span id="corn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Corn`

- <span id="corn-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="corn-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="corn-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="corn-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Corn`

- <span id="corn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Corn`

- <span id="corn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Corn`

##### `impl<U> TryFrom for Corn`

- <span id="corn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="corn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Corn`

- <span id="corn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="corn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TachaOrange`

```rust
struct TachaOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for TachaOrange`

- <span id="tachaorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TachaOrange`

- <span id="tachaorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TachaOrange`

- <span id="tachaorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for TachaOrange`

- <span id="tachaorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tachaorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tachaorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tachaorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for TachaOrange`

- <span id="tachaorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TachaOrange`

- <span id="tachaorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for TachaOrange`

##### `impl<U> TryFrom for TachaOrange`

- <span id="tachaorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tachaorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TachaOrange`

- <span id="tachaorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tachaorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DecoOrange`

```rust
struct DecoOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DecoOrange`

- <span id="decoorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DecoOrange`

- <span id="decoorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DecoOrange`

- <span id="decoorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DecoOrange`

- <span id="decoorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="decoorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="decoorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="decoorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DecoOrange`

- <span id="decoorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DecoOrange`

- <span id="decoorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DecoOrange`

##### `impl<U> TryFrom for DecoOrange`

- <span id="decoorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decoorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DecoOrange`

- <span id="decoorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decoorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PaleGoldenrod`

```rust
struct PaleGoldenrod;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PaleGoldenrod`

- <span id="palegoldenrod-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PaleGoldenrod`

- <span id="palegoldenrod-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PaleGoldenrod`

- <span id="palegoldenrod-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PaleGoldenrod`

- <span id="palegoldenrod-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="palegoldenrod-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="palegoldenrod-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="palegoldenrod-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PaleGoldenrod`

- <span id="palegoldenrod-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PaleGoldenrod`

- <span id="palegoldenrod-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PaleGoldenrod`

##### `impl<U> TryFrom for PaleGoldenrod`

- <span id="palegoldenrod-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="palegoldenrod-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PaleGoldenrod`

- <span id="palegoldenrod-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="palegoldenrod-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AltoBeige`

```rust
struct AltoBeige;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for AltoBeige`

- <span id="altobeige-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AltoBeige`

- <span id="altobeige-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AltoBeige`

- <span id="altobeige-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for AltoBeige`

- <span id="altobeige-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="altobeige-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="altobeige-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="altobeige-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for AltoBeige`

- <span id="altobeige-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AltoBeige`

- <span id="altobeige-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for AltoBeige`

##### `impl<U> TryFrom for AltoBeige`

- <span id="altobeige-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="altobeige-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AltoBeige`

- <span id="altobeige-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="altobeige-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FogPink`

```rust
struct FogPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for FogPink`

- <span id="fogpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FogPink`

- <span id="fogpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FogPink`

- <span id="fogpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for FogPink`

- <span id="fogpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fogpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fogpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fogpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for FogPink`

- <span id="fogpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FogPink`

- <span id="fogpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for FogPink`

##### `impl<U> TryFrom for FogPink`

- <span id="fogpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fogpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FogPink`

- <span id="fogpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fogpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChartreuseYellow`

```rust
struct ChartreuseYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ChartreuseYellow`

- <span id="chartreuseyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChartreuseYellow`

- <span id="chartreuseyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChartreuseYellow`

- <span id="chartreuseyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ChartreuseYellow`

- <span id="chartreuseyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="chartreuseyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="chartreuseyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="chartreuseyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ChartreuseYellow`

- <span id="chartreuseyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChartreuseYellow`

- <span id="chartreuseyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ChartreuseYellow`

##### `impl<U> TryFrom for ChartreuseYellow`

- <span id="chartreuseyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chartreuseyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChartreuseYellow`

- <span id="chartreuseyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chartreuseyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Canary`

```rust
struct Canary;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Canary`

- <span id="canary-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Canary`

- <span id="canary-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Canary`

- <span id="canary-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Canary`

- <span id="canary-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="canary-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="canary-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="canary-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Canary`

- <span id="canary-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Canary`

- <span id="canary-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Canary`

##### `impl<U> TryFrom for Canary`

- <span id="canary-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="canary-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Canary`

- <span id="canary-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="canary-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Honeysuckle`

```rust
struct Honeysuckle;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Honeysuckle`

- <span id="honeysuckle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Honeysuckle`

- <span id="honeysuckle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Honeysuckle`

- <span id="honeysuckle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Honeysuckle`

- <span id="honeysuckle-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="honeysuckle-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="honeysuckle-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="honeysuckle-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Honeysuckle`

- <span id="honeysuckle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Honeysuckle`

- <span id="honeysuckle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Honeysuckle`

##### `impl<U> TryFrom for Honeysuckle`

- <span id="honeysuckle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="honeysuckle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Honeysuckle`

- <span id="honeysuckle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="honeysuckle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReefPaleYellow`

```rust
struct ReefPaleYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ReefPaleYellow`

- <span id="reefpaleyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReefPaleYellow`

- <span id="reefpaleyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReefPaleYellow`

- <span id="reefpaleyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ReefPaleYellow`

- <span id="reefpaleyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="reefpaleyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="reefpaleyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="reefpaleyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ReefPaleYellow`

- <span id="reefpaleyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReefPaleYellow`

- <span id="reefpaleyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ReefPaleYellow`

##### `impl<U> TryFrom for ReefPaleYellow`

- <span id="reefpaleyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reefpaleyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReefPaleYellow`

- <span id="reefpaleyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reefpaleyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SnowyMint`

```rust
struct SnowyMint;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for SnowyMint`

- <span id="snowymint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SnowyMint`

- <span id="snowymint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SnowyMint`

- <span id="snowymint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for SnowyMint`

- <span id="snowymint-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="snowymint-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="snowymint-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="snowymint-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for SnowyMint`

- <span id="snowymint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SnowyMint`

- <span id="snowymint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for SnowyMint`

##### `impl<U> TryFrom for SnowyMint`

- <span id="snowymint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="snowymint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SnowyMint`

- <span id="snowymint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="snowymint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OysterBay`

```rust
struct OysterBay;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for OysterBay`

- <span id="oysterbay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OysterBay`

- <span id="oysterbay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OysterBay`

- <span id="oysterbay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for OysterBay`

- <span id="oysterbay-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="oysterbay-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="oysterbay-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="oysterbay-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for OysterBay`

- <span id="oysterbay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OysterBay`

- <span id="oysterbay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for OysterBay`

##### `impl<U> TryFrom for OysterBay`

- <span id="oysterbay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oysterbay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OysterBay`

- <span id="oysterbay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oysterbay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Red`

```rust
struct Red;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Red`

- <span id="red-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Red`

- <span id="red-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Red`

- <span id="red-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Red`

- <span id="red-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="red-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Red`

- <span id="red-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Red`

- <span id="red-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Red`

##### `impl<U> TryFrom for Red`

- <span id="red-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="red-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Red`

- <span id="red-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="red-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkRose`

```rust
struct DarkRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkRose`

- <span id="darkrose-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkRose`

- <span id="darkrose-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkRose`

- <span id="darkrose-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkRose`

- <span id="darkrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkRose`

- <span id="darkrose-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkRose`

- <span id="darkrose-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkRose`

##### `impl<U> TryFrom for DarkRose`

- <span id="darkrose-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkrose-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkRose`

- <span id="darkrose-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkrose-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Rose`

```rust
struct Rose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Rose`

- <span id="rose-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rose`

- <span id="rose-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rose`

- <span id="rose-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Rose`

- <span id="rose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="rose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="rose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="rose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Rose`

- <span id="rose-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Rose`

- <span id="rose-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Rose`

##### `impl<U> TryFrom for Rose`

- <span id="rose-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rose-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rose`

- <span id="rose-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rose-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightHollywoodCerise`

```rust
struct LightHollywoodCerise;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightHollywoodCerise`

- <span id="lighthollywoodcerise-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightHollywoodCerise`

- <span id="lighthollywoodcerise-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightHollywoodCerise`

- <span id="lighthollywoodcerise-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightHollywoodCerise`

- <span id="lighthollywoodcerise-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lighthollywoodcerise-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lighthollywoodcerise-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightHollywoodCerise`

- <span id="lighthollywoodcerise-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightHollywoodCerise`

- <span id="lighthollywoodcerise-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightHollywoodCerise`

##### `impl<U> TryFrom for LightHollywoodCerise`

- <span id="lighthollywoodcerise-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lighthollywoodcerise-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightHollywoodCerise`

- <span id="lighthollywoodcerise-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lighthollywoodcerise-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PurplePizzazz`

```rust
struct PurplePizzazz;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PurplePizzazz`

- <span id="purplepizzazz-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PurplePizzazz`

- <span id="purplepizzazz-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PurplePizzazz`

- <span id="purplepizzazz-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PurplePizzazz`

- <span id="purplepizzazz-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="purplepizzazz-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="purplepizzazz-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="purplepizzazz-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PurplePizzazz`

- <span id="purplepizzazz-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PurplePizzazz`

- <span id="purplepizzazz-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PurplePizzazz`

##### `impl<U> TryFrom for PurplePizzazz`

- <span id="purplepizzazz-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="purplepizzazz-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PurplePizzazz`

- <span id="purplepizzazz-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="purplepizzazz-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fuchsia`

```rust
struct Fuchsia;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Fuchsia`

- <span id="fuchsia-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fuchsia`

- <span id="fuchsia-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fuchsia`

- <span id="fuchsia-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Fuchsia`

- <span id="fuchsia-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="fuchsia-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="fuchsia-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="fuchsia-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Fuchsia`

- <span id="fuchsia-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fuchsia`

- <span id="fuchsia-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Fuchsia`

##### `impl<U> TryFrom for Fuchsia`

- <span id="fuchsia-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fuchsia-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fuchsia`

- <span id="fuchsia-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fuchsia-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BlazeOrange`

```rust
struct BlazeOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BlazeOrange`

- <span id="blazeorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlazeOrange`

- <span id="blazeorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlazeOrange`

- <span id="blazeorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BlazeOrange`

- <span id="blazeorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blazeorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blazeorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blazeorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BlazeOrange`

- <span id="blazeorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BlazeOrange`

- <span id="blazeorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BlazeOrange`

##### `impl<U> TryFrom for BlazeOrange`

- <span id="blazeorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blazeorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlazeOrange`

- <span id="blazeorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blazeorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BittersweetOrange`

```rust
struct BittersweetOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BittersweetOrange`

- <span id="bittersweetorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BittersweetOrange`

- <span id="bittersweetorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BittersweetOrange`

- <span id="bittersweetorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BittersweetOrange`

- <span id="bittersweetorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="bittersweetorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="bittersweetorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="bittersweetorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BittersweetOrange`

- <span id="bittersweetorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BittersweetOrange`

- <span id="bittersweetorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BittersweetOrange`

##### `impl<U> TryFrom for BittersweetOrange`

- <span id="bittersweetorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bittersweetorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BittersweetOrange`

- <span id="bittersweetorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bittersweetorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WildWatermelon`

```rust
struct WildWatermelon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for WildWatermelon`

- <span id="wildwatermelon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WildWatermelon`

- <span id="wildwatermelon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WildWatermelon`

- <span id="wildwatermelon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for WildWatermelon`

- <span id="wildwatermelon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="wildwatermelon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="wildwatermelon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="wildwatermelon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for WildWatermelon`

- <span id="wildwatermelon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WildWatermelon`

- <span id="wildwatermelon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for WildWatermelon`

##### `impl<U> TryFrom for WildWatermelon`

- <span id="wildwatermelon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wildwatermelon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WildWatermelon`

- <span id="wildwatermelon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wildwatermelon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkHotPink`

```rust
struct DarkHotPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkHotPink`

- <span id="darkhotpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkHotPink`

- <span id="darkhotpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkHotPink`

- <span id="darkhotpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkHotPink`

- <span id="darkhotpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkhotpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkhotpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkhotpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkHotPink`

- <span id="darkhotpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkHotPink`

- <span id="darkhotpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkHotPink`

##### `impl<U> TryFrom for DarkHotPink`

- <span id="darkhotpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkhotpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkHotPink`

- <span id="darkhotpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkhotpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HotPink`

```rust
struct HotPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for HotPink`

- <span id="hotpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HotPink`

- <span id="hotpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HotPink`

- <span id="hotpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for HotPink`

- <span id="hotpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="hotpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="hotpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="hotpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for HotPink`

- <span id="hotpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HotPink`

- <span id="hotpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for HotPink`

##### `impl<U> TryFrom for HotPink`

- <span id="hotpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hotpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HotPink`

- <span id="hotpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hotpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PinkFlamingo`

```rust
struct PinkFlamingo;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PinkFlamingo`

- <span id="pinkflamingo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PinkFlamingo`

- <span id="pinkflamingo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PinkFlamingo`

- <span id="pinkflamingo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PinkFlamingo`

- <span id="pinkflamingo-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinkflamingo-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinkflamingo-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinkflamingo-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PinkFlamingo`

- <span id="pinkflamingo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PinkFlamingo`

- <span id="pinkflamingo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PinkFlamingo`

##### `impl<U> TryFrom for PinkFlamingo`

- <span id="pinkflamingo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pinkflamingo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PinkFlamingo`

- <span id="pinkflamingo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pinkflamingo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlushOrange`

```rust
struct FlushOrange;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for FlushOrange`

- <span id="flushorange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlushOrange`

- <span id="flushorange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlushOrange`

- <span id="flushorange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for FlushOrange`

- <span id="flushorange-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="flushorange-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="flushorange-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="flushorange-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for FlushOrange`

- <span id="flushorange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlushOrange`

- <span id="flushorange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for FlushOrange`

##### `impl<U> TryFrom for FlushOrange`

- <span id="flushorange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flushorange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlushOrange`

- <span id="flushorange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flushorange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Salmon`

```rust
struct Salmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Salmon`

- <span id="salmon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Salmon`

- <span id="salmon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Salmon`

- <span id="salmon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Salmon`

- <span id="salmon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="salmon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="salmon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="salmon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Salmon`

- <span id="salmon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Salmon`

- <span id="salmon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Salmon`

##### `impl<U> TryFrom for Salmon`

- <span id="salmon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="salmon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Salmon`

- <span id="salmon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="salmon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VividTangerine`

```rust
struct VividTangerine;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for VividTangerine`

- <span id="vividtangerine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VividTangerine`

- <span id="vividtangerine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VividTangerine`

- <span id="vividtangerine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for VividTangerine`

- <span id="vividtangerine-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="vividtangerine-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="vividtangerine-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="vividtangerine-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for VividTangerine`

- <span id="vividtangerine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VividTangerine`

- <span id="vividtangerine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for VividTangerine`

##### `impl<U> TryFrom for VividTangerine`

- <span id="vividtangerine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vividtangerine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VividTangerine`

- <span id="vividtangerine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vividtangerine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PinkSalmon`

```rust
struct PinkSalmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PinkSalmon`

- <span id="pinksalmon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PinkSalmon`

- <span id="pinksalmon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PinkSalmon`

- <span id="pinksalmon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PinkSalmon`

- <span id="pinksalmon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinksalmon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinksalmon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinksalmon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PinkSalmon`

- <span id="pinksalmon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PinkSalmon`

- <span id="pinksalmon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PinkSalmon`

##### `impl<U> TryFrom for PinkSalmon`

- <span id="pinksalmon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pinksalmon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PinkSalmon`

- <span id="pinksalmon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pinksalmon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkLavenderRose`

```rust
struct DarkLavenderRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkLavenderRose`

- <span id="darklavenderrose-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkLavenderRose`

- <span id="darklavenderrose-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkLavenderRose`

- <span id="darklavenderrose-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkLavenderRose`

- <span id="darklavenderrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darklavenderrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darklavenderrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darklavenderrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkLavenderRose`

- <span id="darklavenderrose-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkLavenderRose`

- <span id="darklavenderrose-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkLavenderRose`

##### `impl<U> TryFrom for DarkLavenderRose`

- <span id="darklavenderrose-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darklavenderrose-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkLavenderRose`

- <span id="darklavenderrose-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darklavenderrose-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BlushPink`

```rust
struct BlushPink;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for BlushPink`

- <span id="blushpink-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BlushPink`

- <span id="blushpink-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BlushPink`

- <span id="blushpink-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for BlushPink`

- <span id="blushpink-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blushpink-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blushpink-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="blushpink-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for BlushPink`

- <span id="blushpink-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BlushPink`

- <span id="blushpink-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BlushPink`

##### `impl<U> TryFrom for BlushPink`

- <span id="blushpink-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="blushpink-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BlushPink`

- <span id="blushpink-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="blushpink-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `YellowSea`

```rust
struct YellowSea;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for YellowSea`

- <span id="yellowsea-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for YellowSea`

- <span id="yellowsea-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for YellowSea`

- <span id="yellowsea-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for YellowSea`

- <span id="yellowsea-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellowsea-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellowsea-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellowsea-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for YellowSea`

- <span id="yellowsea-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for YellowSea`

- <span id="yellowsea-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for YellowSea`

##### `impl<U> TryFrom for YellowSea`

- <span id="yellowsea-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="yellowsea-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for YellowSea`

- <span id="yellowsea-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="yellowsea-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TexasRose`

```rust
struct TexasRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for TexasRose`

- <span id="texasrose-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TexasRose`

- <span id="texasrose-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TexasRose`

- <span id="texasrose-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for TexasRose`

- <span id="texasrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="texasrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="texasrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="texasrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for TexasRose`

- <span id="texasrose-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TexasRose`

- <span id="texasrose-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for TexasRose`

##### `impl<U> TryFrom for TexasRose`

- <span id="texasrose-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="texasrose-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TexasRose`

- <span id="texasrose-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="texasrose-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tacao`

```rust
struct Tacao;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Tacao`

- <span id="tacao-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tacao`

- <span id="tacao-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tacao`

- <span id="tacao-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Tacao`

- <span id="tacao-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tacao-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tacao-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tacao-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Tacao`

- <span id="tacao-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tacao`

- <span id="tacao-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Tacao`

##### `impl<U> TryFrom for Tacao`

- <span id="tacao-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tacao-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tacao`

- <span id="tacao-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tacao-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Sundown`

```rust
struct Sundown;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Sundown`

- <span id="sundown-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Sundown`

- <span id="sundown-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Sundown`

- <span id="sundown-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Sundown`

- <span id="sundown-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="sundown-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="sundown-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="sundown-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Sundown`

- <span id="sundown-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Sundown`

- <span id="sundown-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Sundown`

##### `impl<U> TryFrom for Sundown`

- <span id="sundown-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sundown-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Sundown`

- <span id="sundown-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sundown-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CottonCandy`

```rust
struct CottonCandy;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CottonCandy`

- <span id="cottoncandy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CottonCandy`

- <span id="cottoncandy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CottonCandy`

- <span id="cottoncandy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CottonCandy`

- <span id="cottoncandy-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cottoncandy-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cottoncandy-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cottoncandy-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CottonCandy`

- <span id="cottoncandy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CottonCandy`

- <span id="cottoncandy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CottonCandy`

##### `impl<U> TryFrom for CottonCandy`

- <span id="cottoncandy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cottoncandy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CottonCandy`

- <span id="cottoncandy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cottoncandy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LavenderRose`

```rust
struct LavenderRose;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LavenderRose`

- <span id="lavenderrose-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LavenderRose`

- <span id="lavenderrose-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LavenderRose`

- <span id="lavenderrose-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LavenderRose`

- <span id="lavenderrose-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lavenderrose-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lavenderrose-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lavenderrose-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LavenderRose`

- <span id="lavenderrose-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LavenderRose`

- <span id="lavenderrose-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LavenderRose`

##### `impl<U> TryFrom for LavenderRose`

- <span id="lavenderrose-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lavenderrose-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LavenderRose`

- <span id="lavenderrose-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lavenderrose-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Gold`

```rust
struct Gold;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Gold`

- <span id="gold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Gold`

- <span id="gold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Gold`

- <span id="gold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Gold`

- <span id="gold-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gold-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gold-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gold-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Gold`

- <span id="gold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Gold`

- <span id="gold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Gold`

##### `impl<U> TryFrom for Gold`

- <span id="gold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Gold`

- <span id="gold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dandelion`

```rust
struct Dandelion;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Dandelion`

- <span id="dandelion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dandelion`

- <span id="dandelion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dandelion`

- <span id="dandelion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Dandelion`

- <span id="dandelion-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dandelion-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dandelion-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dandelion-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Dandelion`

- <span id="dandelion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Dandelion`

- <span id="dandelion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Dandelion`

##### `impl<U> TryFrom for Dandelion`

- <span id="dandelion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dandelion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dandelion`

- <span id="dandelion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dandelion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GrandisCaramel`

```rust
struct GrandisCaramel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for GrandisCaramel`

- <span id="grandiscaramel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GrandisCaramel`

- <span id="grandiscaramel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GrandisCaramel`

- <span id="grandiscaramel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for GrandisCaramel`

- <span id="grandiscaramel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="grandiscaramel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="grandiscaramel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="grandiscaramel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for GrandisCaramel`

- <span id="grandiscaramel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GrandisCaramel`

- <span id="grandiscaramel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GrandisCaramel`

##### `impl<U> TryFrom for GrandisCaramel`

- <span id="grandiscaramel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="grandiscaramel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GrandisCaramel`

- <span id="grandiscaramel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="grandiscaramel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Caramel`

```rust
struct Caramel;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Caramel`

- <span id="caramel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Caramel`

- <span id="caramel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Caramel`

- <span id="caramel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Caramel`

- <span id="caramel-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="caramel-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="caramel-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="caramel-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Caramel`

- <span id="caramel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Caramel`

- <span id="caramel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Caramel`

##### `impl<U> TryFrom for Caramel`

- <span id="caramel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="caramel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Caramel`

- <span id="caramel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="caramel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CosmosSalmon`

```rust
struct CosmosSalmon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CosmosSalmon`

- <span id="cosmossalmon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CosmosSalmon`

- <span id="cosmossalmon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CosmosSalmon`

- <span id="cosmossalmon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CosmosSalmon`

- <span id="cosmossalmon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cosmossalmon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cosmossalmon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cosmossalmon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CosmosSalmon`

- <span id="cosmossalmon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CosmosSalmon`

- <span id="cosmossalmon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CosmosSalmon`

##### `impl<U> TryFrom for CosmosSalmon`

- <span id="cosmossalmon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cosmossalmon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CosmosSalmon`

- <span id="cosmossalmon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cosmossalmon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PinkLace`

```rust
struct PinkLace;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PinkLace`

- <span id="pinklace-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PinkLace`

- <span id="pinklace-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PinkLace`

- <span id="pinklace-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PinkLace`

- <span id="pinklace-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="pinklace-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="pinklace-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="pinklace-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PinkLace`

- <span id="pinklace-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PinkLace`

- <span id="pinklace-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PinkLace`

##### `impl<U> TryFrom for PinkLace`

- <span id="pinklace-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pinklace-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PinkLace`

- <span id="pinklace-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pinklace-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Yellow`

```rust
struct Yellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Yellow`

- <span id="yellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Yellow`

- <span id="yellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Yellow`

- <span id="yellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Yellow`

- <span id="yellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="yellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Yellow`

- <span id="yellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Yellow`

- <span id="yellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Yellow`

##### `impl<U> TryFrom for Yellow`

- <span id="yellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="yellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Yellow`

- <span id="yellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="yellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LaserLemon`

```rust
struct LaserLemon;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LaserLemon`

- <span id="laserlemon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LaserLemon`

- <span id="laserlemon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LaserLemon`

- <span id="laserlemon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LaserLemon`

- <span id="laserlemon-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="laserlemon-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="laserlemon-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="laserlemon-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LaserLemon`

- <span id="laserlemon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LaserLemon`

- <span id="laserlemon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LaserLemon`

##### `impl<U> TryFrom for LaserLemon`

- <span id="laserlemon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="laserlemon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LaserLemon`

- <span id="laserlemon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="laserlemon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DollyYellow`

```rust
struct DollyYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DollyYellow`

- <span id="dollyyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DollyYellow`

- <span id="dollyyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DollyYellow`

- <span id="dollyyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DollyYellow`

- <span id="dollyyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dollyyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dollyyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dollyyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DollyYellow`

- <span id="dollyyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DollyYellow`

- <span id="dollyyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DollyYellow`

##### `impl<U> TryFrom for DollyYellow`

- <span id="dollyyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dollyyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DollyYellow`

- <span id="dollyyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dollyyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PortafinoYellow`

```rust
struct PortafinoYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for PortafinoYellow`

- <span id="portafinoyellow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PortafinoYellow`

- <span id="portafinoyellow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PortafinoYellow`

- <span id="portafinoyellow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for PortafinoYellow`

- <span id="portafinoyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="portafinoyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="portafinoyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="portafinoyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for PortafinoYellow`

- <span id="portafinoyellow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PortafinoYellow`

- <span id="portafinoyellow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for PortafinoYellow`

##### `impl<U> TryFrom for PortafinoYellow`

- <span id="portafinoyellow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="portafinoyellow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PortafinoYellow`

- <span id="portafinoyellow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="portafinoyellow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cumulus`

```rust
struct Cumulus;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Cumulus`

- <span id="cumulus-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cumulus`

- <span id="cumulus-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cumulus`

- <span id="cumulus-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Cumulus`

- <span id="cumulus-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cumulus-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cumulus-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="cumulus-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Cumulus`

- <span id="cumulus-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cumulus`

- <span id="cumulus-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Cumulus`

##### `impl<U> TryFrom for Cumulus`

- <span id="cumulus-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cumulus-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cumulus`

- <span id="cumulus-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cumulus-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `White`

```rust
struct White;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for White`

- <span id="white-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for White`

- <span id="white-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for White`

- <span id="white-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for White`

- <span id="white-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="white-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for White`

- <span id="white-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for White`

- <span id="white-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for White`

##### `impl<U> TryFrom for White`

- <span id="white-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="white-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for White`

- <span id="white-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="white-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkCodGray`

```rust
struct DarkCodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkCodGray`

- <span id="darkcodgray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkCodGray`

- <span id="darkcodgray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkCodGray`

- <span id="darkcodgray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkCodGray`

- <span id="darkcodgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkcodgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkcodgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkcodgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkCodGray`

- <span id="darkcodgray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkCodGray`

- <span id="darkcodgray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkCodGray`

##### `impl<U> TryFrom for DarkCodGray`

- <span id="darkcodgray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkcodgray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkCodGray`

- <span id="darkcodgray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkcodgray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CodGray`

```rust
struct CodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for CodGray`

- <span id="codgray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CodGray`

- <span id="codgray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CodGray`

- <span id="codgray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CodGray`

- <span id="codgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="codgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="codgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="codgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for CodGray`

- <span id="codgray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CodGray`

- <span id="codgray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CodGray`

##### `impl<U> TryFrom for CodGray`

- <span id="codgray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="codgray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CodGray`

- <span id="codgray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="codgray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightCodGray`

```rust
struct LightCodGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightCodGray`

- <span id="lightcodgray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightCodGray`

- <span id="lightcodgray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightCodGray`

- <span id="lightcodgray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightCodGray`

- <span id="lightcodgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightcodgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightcodgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightcodgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightCodGray`

- <span id="lightcodgray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightCodGray`

- <span id="lightcodgray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightCodGray`

##### `impl<U> TryFrom for LightCodGray`

- <span id="lightcodgray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightcodgray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightCodGray`

- <span id="lightcodgray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightcodgray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkMineShaft`

```rust
struct DarkMineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkMineShaft`

- <span id="darkmineshaft-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkMineShaft`

- <span id="darkmineshaft-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkMineShaft`

- <span id="darkmineshaft-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkMineShaft`

- <span id="darkmineshaft-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkmineshaft-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkmineshaft-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkmineshaft-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkMineShaft`

- <span id="darkmineshaft-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkMineShaft`

- <span id="darkmineshaft-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkMineShaft`

##### `impl<U> TryFrom for DarkMineShaft`

- <span id="darkmineshaft-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkmineshaft-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkMineShaft`

- <span id="darkmineshaft-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkmineshaft-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MineShaft`

```rust
struct MineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for MineShaft`

- <span id="mineshaft-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MineShaft`

- <span id="mineshaft-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MineShaft`

- <span id="mineshaft-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for MineShaft`

- <span id="mineshaft-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mineshaft-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mineshaft-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mineshaft-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for MineShaft`

- <span id="mineshaft-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MineShaft`

- <span id="mineshaft-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MineShaft`

##### `impl<U> TryFrom for MineShaft`

- <span id="mineshaft-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mineshaft-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MineShaft`

- <span id="mineshaft-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mineshaft-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightMineShaft`

```rust
struct LightMineShaft;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightMineShaft`

- <span id="lightmineshaft-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightMineShaft`

- <span id="lightmineshaft-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightMineShaft`

- <span id="lightmineshaft-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightMineShaft`

- <span id="lightmineshaft-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightmineshaft-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightmineshaft-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightmineshaft-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightMineShaft`

- <span id="lightmineshaft-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightMineShaft`

- <span id="lightmineshaft-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightMineShaft`

##### `impl<U> TryFrom for LightMineShaft`

- <span id="lightmineshaft-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightmineshaft-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightMineShaft`

- <span id="lightmineshaft-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightmineshaft-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkTundora`

```rust
struct DarkTundora;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkTundora`

- <span id="darktundora-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkTundora`

- <span id="darktundora-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkTundora`

- <span id="darktundora-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkTundora`

- <span id="darktundora-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darktundora-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darktundora-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darktundora-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkTundora`

- <span id="darktundora-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkTundora`

- <span id="darktundora-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkTundora`

##### `impl<U> TryFrom for DarkTundora`

- <span id="darktundora-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darktundora-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkTundora`

- <span id="darktundora-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darktundora-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tundora`

```rust
struct Tundora;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Tundora`

- <span id="tundora-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tundora`

- <span id="tundora-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tundora`

- <span id="tundora-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Tundora`

- <span id="tundora-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="tundora-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="tundora-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="tundora-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Tundora`

- <span id="tundora-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tundora`

- <span id="tundora-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Tundora`

##### `impl<U> TryFrom for Tundora`

- <span id="tundora-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tundora-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tundora`

- <span id="tundora-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tundora-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScorpionGray`

```rust
struct ScorpionGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for ScorpionGray`

- <span id="scorpiongray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScorpionGray`

- <span id="scorpiongray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScorpionGray`

- <span id="scorpiongray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for ScorpionGray`

- <span id="scorpiongray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="scorpiongray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="scorpiongray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="scorpiongray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for ScorpionGray`

- <span id="scorpiongray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScorpionGray`

- <span id="scorpiongray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ScorpionGray`

##### `impl<U> TryFrom for ScorpionGray`

- <span id="scorpiongray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scorpiongray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScorpionGray`

- <span id="scorpiongray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scorpiongray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkDoveGray`

```rust
struct DarkDoveGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkDoveGray`

- <span id="darkdovegray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkDoveGray`

- <span id="darkdovegray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkDoveGray`

- <span id="darkdovegray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkDoveGray`

- <span id="darkdovegray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkdovegray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkdovegray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkdovegray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkDoveGray`

- <span id="darkdovegray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkDoveGray`

- <span id="darkdovegray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkDoveGray`

##### `impl<U> TryFrom for DarkDoveGray`

- <span id="darkdovegray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkdovegray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkDoveGray`

- <span id="darkdovegray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkdovegray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DoveGray`

```rust
struct DoveGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DoveGray`

- <span id="dovegray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DoveGray`

- <span id="dovegray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DoveGray`

- <span id="dovegray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DoveGray`

- <span id="dovegray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dovegray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dovegray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dovegray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DoveGray`

- <span id="dovegray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DoveGray`

- <span id="dovegray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DoveGray`

##### `impl<U> TryFrom for DoveGray`

- <span id="dovegray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dovegray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DoveGray`

- <span id="dovegray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dovegray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Boulder`

```rust
struct Boulder;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Boulder`

- <span id="boulder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Boulder`

- <span id="boulder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Boulder`

- <span id="boulder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Boulder`

- <span id="boulder-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="boulder-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="boulder-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="boulder-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Boulder`

- <span id="boulder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Boulder`

- <span id="boulder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Boulder`

##### `impl<U> TryFrom for Boulder`

- <span id="boulder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boulder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Boulder`

- <span id="boulder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boulder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Gray`

```rust
struct Gray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Gray`

- <span id="gray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Gray`

- <span id="gray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Gray`

- <span id="gray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Gray`

- <span id="gray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Gray`

- <span id="gray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Gray`

- <span id="gray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Gray`

##### `impl<U> TryFrom for Gray`

- <span id="gray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Gray`

- <span id="gray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightGray`

```rust
struct LightGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightGray`

- <span id="lightgray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightGray`

- <span id="lightgray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightGray`

- <span id="lightgray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightGray`

- <span id="lightgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightGray`

- <span id="lightgray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightGray`

- <span id="lightgray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightGray`

##### `impl<U> TryFrom for LightGray`

- <span id="lightgray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightgray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightGray`

- <span id="lightgray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightgray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DustyGray`

```rust
struct DustyGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DustyGray`

- <span id="dustygray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DustyGray`

- <span id="dustygray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DustyGray`

- <span id="dustygray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DustyGray`

- <span id="dustygray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="dustygray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="dustygray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="dustygray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DustyGray`

- <span id="dustygray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DustyGray`

- <span id="dustygray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DustyGray`

##### `impl<U> TryFrom for DustyGray`

- <span id="dustygray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dustygray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DustyGray`

- <span id="dustygray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dustygray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NobelGray`

```rust
struct NobelGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for NobelGray`

- <span id="nobelgray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NobelGray`

- <span id="nobelgray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NobelGray`

- <span id="nobelgray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for NobelGray`

- <span id="nobelgray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="nobelgray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="nobelgray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="nobelgray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for NobelGray`

- <span id="nobelgray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NobelGray`

- <span id="nobelgray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for NobelGray`

##### `impl<U> TryFrom for NobelGray`

- <span id="nobelgray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nobelgray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NobelGray`

- <span id="nobelgray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nobelgray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkSilverChalice`

```rust
struct DarkSilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkSilverChalice`

- <span id="darksilverchalice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkSilverChalice`

- <span id="darksilverchalice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkSilverChalice`

- <span id="darksilverchalice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkSilverChalice`

- <span id="darksilverchalice-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilverchalice-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilverchalice-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilverchalice-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkSilverChalice`

- <span id="darksilverchalice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkSilverChalice`

- <span id="darksilverchalice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkSilverChalice`

##### `impl<U> TryFrom for DarkSilverChalice`

- <span id="darksilverchalice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darksilverchalice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkSilverChalice`

- <span id="darksilverchalice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darksilverchalice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LightSilverChalice`

```rust
struct LightSilverChalice;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for LightSilverChalice`

- <span id="lightsilverchalice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LightSilverChalice`

- <span id="lightsilverchalice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LightSilverChalice`

- <span id="lightsilverchalice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for LightSilverChalice`

- <span id="lightsilverchalice-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="lightsilverchalice-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="lightsilverchalice-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="lightsilverchalice-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for LightSilverChalice`

- <span id="lightsilverchalice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LightSilverChalice`

- <span id="lightsilverchalice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for LightSilverChalice`

##### `impl<U> TryFrom for LightSilverChalice`

- <span id="lightsilverchalice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lightsilverchalice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LightSilverChalice`

- <span id="lightsilverchalice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lightsilverchalice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkSilver`

```rust
struct DarkSilver;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkSilver`

- <span id="darksilver-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkSilver`

- <span id="darksilver-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkSilver`

- <span id="darksilver-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkSilver`

- <span id="darksilver-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darksilver-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darksilver-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darksilver-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkSilver`

- <span id="darksilver-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkSilver`

- <span id="darksilver-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkSilver`

##### `impl<U> TryFrom for DarkSilver`

- <span id="darksilver-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darksilver-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkSilver`

- <span id="darksilver-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darksilver-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Silver`

```rust
struct Silver;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Silver`

- <span id="silver-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Silver`

- <span id="silver-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Silver`

- <span id="silver-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Silver`

- <span id="silver-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="silver-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="silver-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="silver-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Silver`

- <span id="silver-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Silver`

- <span id="silver-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Silver`

##### `impl<U> TryFrom for Silver`

- <span id="silver-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="silver-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Silver`

- <span id="silver-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="silver-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DarkAlto`

```rust
struct DarkAlto;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for DarkAlto`

- <span id="darkalto-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DarkAlto`

- <span id="darkalto-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DarkAlto`

- <span id="darkalto-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for DarkAlto`

- <span id="darkalto-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="darkalto-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="darkalto-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="darkalto-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for DarkAlto`

- <span id="darkalto-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DarkAlto`

- <span id="darkalto-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DarkAlto`

##### `impl<U> TryFrom for DarkAlto`

- <span id="darkalto-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="darkalto-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DarkAlto`

- <span id="darkalto-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="darkalto-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Alto`

```rust
struct Alto;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Alto`

- <span id="alto-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Alto`

- <span id="alto-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Alto`

- <span id="alto-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Alto`

- <span id="alto-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="alto-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="alto-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="alto-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Alto`

- <span id="alto-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Alto`

- <span id="alto-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Alto`

##### `impl<U> TryFrom for Alto`

- <span id="alto-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="alto-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Alto`

- <span id="alto-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="alto-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Mercury`

```rust
struct Mercury;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for Mercury`

- <span id="mercury-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mercury`

- <span id="mercury-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mercury`

- <span id="mercury-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for Mercury`

- <span id="mercury-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="mercury-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="mercury-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="mercury-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for Mercury`

- <span id="mercury-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Mercury`

- <span id="mercury-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Mercury`

##### `impl<U> TryFrom for Mercury`

- <span id="mercury-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mercury-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mercury`

- <span id="mercury-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mercury-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GalleryGray`

```rust
struct GalleryGray;
```

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

#### Trait Implementations

##### `impl Any for GalleryGray`

- <span id="gallerygray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GalleryGray`

- <span id="gallerygray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GalleryGray`

- <span id="gallerygray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for GalleryGray`

- <span id="gallerygray-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="gallerygray-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="gallerygray-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

- <span id="gallerygray-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

##### `impl<T> From for GalleryGray`

- <span id="gallerygray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GalleryGray`

- <span id="gallerygray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for GalleryGray`

##### `impl<U> TryFrom for GalleryGray`

- <span id="gallerygray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gallerygray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GalleryGray`

- <span id="gallerygray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gallerygray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `xterm_colors!`

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:1-121`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors/xterm.rs#L1-L121)*

