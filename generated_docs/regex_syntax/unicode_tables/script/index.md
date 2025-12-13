*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [script](index.md)*

---

# Module `script`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by-name)
  - [`ADLAM`](#adlam)
  - [`AHOM`](#ahom)
  - [`ANATOLIAN_HIEROGLYPHS`](#anatolian-hieroglyphs)
  - [`ARABIC`](#arabic)
  - [`ARMENIAN`](#armenian)
  - [`AVESTAN`](#avestan)
  - [`BALINESE`](#balinese)
  - [`BAMUM`](#bamum)
  - [`BASSA_VAH`](#bassa-vah)
  - [`BATAK`](#batak)
  - [`BENGALI`](#bengali)
  - [`BHAIKSUKI`](#bhaiksuki)
  - [`BOPOMOFO`](#bopomofo)
  - [`BRAHMI`](#brahmi)
  - [`BRAILLE`](#braille)
  - [`BUGINESE`](#buginese)
  - [`BUHID`](#buhid)
  - [`CANADIAN_ABORIGINAL`](#canadian-aboriginal)
  - [`CARIAN`](#carian)
  - [`CAUCASIAN_ALBANIAN`](#caucasian-albanian)
  - [`CHAKMA`](#chakma)
  - [`CHAM`](#cham)
  - [`CHEROKEE`](#cherokee)
  - [`CHORASMIAN`](#chorasmian)
  - [`COMMON`](#common)
  - [`COPTIC`](#coptic)
  - [`CUNEIFORM`](#cuneiform)
  - [`CYPRIOT`](#cypriot)
  - [`CYPRO_MINOAN`](#cypro-minoan)
  - [`CYRILLIC`](#cyrillic)
  - [`DESERET`](#deseret)
  - [`DEVANAGARI`](#devanagari)
  - [`DIVES_AKURU`](#dives-akuru)
  - [`DOGRA`](#dogra)
  - [`DUPLOYAN`](#duployan)
  - [`EGYPTIAN_HIEROGLYPHS`](#egyptian-hieroglyphs)
  - [`ELBASAN`](#elbasan)
  - [`ELYMAIC`](#elymaic)
  - [`ETHIOPIC`](#ethiopic)
  - [`GARAY`](#garay)
  - [`GEORGIAN`](#georgian)
  - [`GLAGOLITIC`](#glagolitic)
  - [`GOTHIC`](#gothic)
  - [`GRANTHA`](#grantha)
  - [`GREEK`](#greek)
  - [`GUJARATI`](#gujarati)
  - [`GUNJALA_GONDI`](#gunjala-gondi)
  - [`GURMUKHI`](#gurmukhi)
  - [`GURUNG_KHEMA`](#gurung-khema)
  - [`HAN`](#han)
  - [`HANGUL`](#hangul)
  - [`HANIFI_ROHINGYA`](#hanifi-rohingya)
  - [`HANUNOO`](#hanunoo)
  - [`HATRAN`](#hatran)
  - [`HEBREW`](#hebrew)
  - [`HIRAGANA`](#hiragana)
  - [`IMPERIAL_ARAMAIC`](#imperial-aramaic)
  - [`INHERITED`](#inherited)
  - [`INSCRIPTIONAL_PAHLAVI`](#inscriptional-pahlavi)
  - [`INSCRIPTIONAL_PARTHIAN`](#inscriptional-parthian)
  - [`JAVANESE`](#javanese)
  - [`KAITHI`](#kaithi)
  - [`KANNADA`](#kannada)
  - [`KATAKANA`](#katakana)
  - [`KAWI`](#kawi)
  - [`KAYAH_LI`](#kayah-li)
  - [`KHAROSHTHI`](#kharoshthi)
  - [`KHITAN_SMALL_SCRIPT`](#khitan-small-script)
  - [`KHMER`](#khmer)
  - [`KHOJKI`](#khojki)
  - [`KHUDAWADI`](#khudawadi)
  - [`KIRAT_RAI`](#kirat-rai)
  - [`LAO`](#lao)
  - [`LATIN`](#latin)
  - [`LEPCHA`](#lepcha)
  - [`LIMBU`](#limbu)
  - [`LINEAR_A`](#linear-a)
  - [`LINEAR_B`](#linear-b)
  - [`LISU`](#lisu)
  - [`LYCIAN`](#lycian)
  - [`LYDIAN`](#lydian)
  - [`MAHAJANI`](#mahajani)
  - [`MAKASAR`](#makasar)
  - [`MALAYALAM`](#malayalam)
  - [`MANDAIC`](#mandaic)
  - [`MANICHAEAN`](#manichaean)
  - [`MARCHEN`](#marchen)
  - [`MASARAM_GONDI`](#masaram-gondi)
  - [`MEDEFAIDRIN`](#medefaidrin)
  - [`MEETEI_MAYEK`](#meetei-mayek)
  - [`MENDE_KIKAKUI`](#mende-kikakui)
  - [`MEROITIC_CURSIVE`](#meroitic-cursive)
  - [`MEROITIC_HIEROGLYPHS`](#meroitic-hieroglyphs)
  - [`MIAO`](#miao)
  - [`MODI`](#modi)
  - [`MONGOLIAN`](#mongolian)
  - [`MRO`](#mro)
  - [`MULTANI`](#multani)
  - [`MYANMAR`](#myanmar)
  - [`NABATAEAN`](#nabataean)
  - [`NAG_MUNDARI`](#nag-mundari)
  - [`NANDINAGARI`](#nandinagari)
  - [`NEW_TAI_LUE`](#new-tai-lue)
  - [`NEWA`](#newa)
  - [`NKO`](#nko)
  - [`NUSHU`](#nushu)
  - [`NYIAKENG_PUACHUE_HMONG`](#nyiakeng-puachue-hmong)
  - [`OGHAM`](#ogham)
  - [`OL_CHIKI`](#ol-chiki)
  - [`OL_ONAL`](#ol-onal)
  - [`OLD_HUNGARIAN`](#old-hungarian)
  - [`OLD_ITALIC`](#old-italic)
  - [`OLD_NORTH_ARABIAN`](#old-north-arabian)
  - [`OLD_PERMIC`](#old-permic)
  - [`OLD_PERSIAN`](#old-persian)
  - [`OLD_SOGDIAN`](#old-sogdian)
  - [`OLD_SOUTH_ARABIAN`](#old-south-arabian)
  - [`OLD_TURKIC`](#old-turkic)
  - [`OLD_UYGHUR`](#old-uyghur)
  - [`ORIYA`](#oriya)
  - [`OSAGE`](#osage)
  - [`OSMANYA`](#osmanya)
  - [`PAHAWH_HMONG`](#pahawh-hmong)
  - [`PALMYRENE`](#palmyrene)
  - [`PAU_CIN_HAU`](#pau-cin-hau)
  - [`PHAGS_PA`](#phags-pa)
  - [`PHOENICIAN`](#phoenician)
  - [`PSALTER_PAHLAVI`](#psalter-pahlavi)
  - [`REJANG`](#rejang)
  - [`RUNIC`](#runic)
  - [`SAMARITAN`](#samaritan)
  - [`SAURASHTRA`](#saurashtra)
  - [`SHARADA`](#sharada)
  - [`SHAVIAN`](#shavian)
  - [`SIDDHAM`](#siddham)
  - [`SIGNWRITING`](#signwriting)
  - [`SINHALA`](#sinhala)
  - [`SOGDIAN`](#sogdian)
  - [`SORA_SOMPENG`](#sora-sompeng)
  - [`SOYOMBO`](#soyombo)
  - [`SUNDANESE`](#sundanese)
  - [`SUNUWAR`](#sunuwar)
  - [`SYLOTI_NAGRI`](#syloti-nagri)
  - [`SYRIAC`](#syriac)
  - [`TAGALOG`](#tagalog)
  - [`TAGBANWA`](#tagbanwa)
  - [`TAI_LE`](#tai-le)
  - [`TAI_THAM`](#tai-tham)
  - [`TAI_VIET`](#tai-viet)
  - [`TAKRI`](#takri)
  - [`TAMIL`](#tamil)
  - [`TANGSA`](#tangsa)
  - [`TANGUT`](#tangut)
  - [`TELUGU`](#telugu)
  - [`THAANA`](#thaana)
  - [`THAI`](#thai)
  - [`TIBETAN`](#tibetan)
  - [`TIFINAGH`](#tifinagh)
  - [`TIRHUTA`](#tirhuta)
  - [`TODHRI`](#todhri)
  - [`TOTO`](#toto)
  - [`TULU_TIGALARI`](#tulu-tigalari)
  - [`UGARITIC`](#ugaritic)
  - [`VAI`](#vai)
  - [`VITHKUQI`](#vithkuqi)
  - [`WANCHO`](#wancho)
  - [`WARANG_CITI`](#warang-citi)
  - [`YEZIDI`](#yezidi)
  - [`YI`](#yi)
  - [`ZANABAZAR_SQUARE`](#zanabazar-square)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by-name) | const |  |
| [`ADLAM`](#adlam) | const |  |
| [`AHOM`](#ahom) | const |  |
| [`ANATOLIAN_HIEROGLYPHS`](#anatolian-hieroglyphs) | const |  |
| [`ARABIC`](#arabic) | const |  |
| [`ARMENIAN`](#armenian) | const |  |
| [`AVESTAN`](#avestan) | const |  |
| [`BALINESE`](#balinese) | const |  |
| [`BAMUM`](#bamum) | const |  |
| [`BASSA_VAH`](#bassa-vah) | const |  |
| [`BATAK`](#batak) | const |  |
| [`BENGALI`](#bengali) | const |  |
| [`BHAIKSUKI`](#bhaiksuki) | const |  |
| [`BOPOMOFO`](#bopomofo) | const |  |
| [`BRAHMI`](#brahmi) | const |  |
| [`BRAILLE`](#braille) | const |  |
| [`BUGINESE`](#buginese) | const |  |
| [`BUHID`](#buhid) | const |  |
| [`CANADIAN_ABORIGINAL`](#canadian-aboriginal) | const |  |
| [`CARIAN`](#carian) | const |  |
| [`CAUCASIAN_ALBANIAN`](#caucasian-albanian) | const |  |
| [`CHAKMA`](#chakma) | const |  |
| [`CHAM`](#cham) | const |  |
| [`CHEROKEE`](#cherokee) | const |  |
| [`CHORASMIAN`](#chorasmian) | const |  |
| [`COMMON`](#common) | const |  |
| [`COPTIC`](#coptic) | const |  |
| [`CUNEIFORM`](#cuneiform) | const |  |
| [`CYPRIOT`](#cypriot) | const |  |
| [`CYPRO_MINOAN`](#cypro-minoan) | const |  |
| [`CYRILLIC`](#cyrillic) | const |  |
| [`DESERET`](#deseret) | const |  |
| [`DEVANAGARI`](#devanagari) | const |  |
| [`DIVES_AKURU`](#dives-akuru) | const |  |
| [`DOGRA`](#dogra) | const |  |
| [`DUPLOYAN`](#duployan) | const |  |
| [`EGYPTIAN_HIEROGLYPHS`](#egyptian-hieroglyphs) | const |  |
| [`ELBASAN`](#elbasan) | const |  |
| [`ELYMAIC`](#elymaic) | const |  |
| [`ETHIOPIC`](#ethiopic) | const |  |
| [`GARAY`](#garay) | const |  |
| [`GEORGIAN`](#georgian) | const |  |
| [`GLAGOLITIC`](#glagolitic) | const |  |
| [`GOTHIC`](#gothic) | const |  |
| [`GRANTHA`](#grantha) | const |  |
| [`GREEK`](#greek) | const |  |
| [`GUJARATI`](#gujarati) | const |  |
| [`GUNJALA_GONDI`](#gunjala-gondi) | const |  |
| [`GURMUKHI`](#gurmukhi) | const |  |
| [`GURUNG_KHEMA`](#gurung-khema) | const |  |
| [`HAN`](#han) | const |  |
| [`HANGUL`](#hangul) | const |  |
| [`HANIFI_ROHINGYA`](#hanifi-rohingya) | const |  |
| [`HANUNOO`](#hanunoo) | const |  |
| [`HATRAN`](#hatran) | const |  |
| [`HEBREW`](#hebrew) | const |  |
| [`HIRAGANA`](#hiragana) | const |  |
| [`IMPERIAL_ARAMAIC`](#imperial-aramaic) | const |  |
| [`INHERITED`](#inherited) | const |  |
| [`INSCRIPTIONAL_PAHLAVI`](#inscriptional-pahlavi) | const |  |
| [`INSCRIPTIONAL_PARTHIAN`](#inscriptional-parthian) | const |  |
| [`JAVANESE`](#javanese) | const |  |
| [`KAITHI`](#kaithi) | const |  |
| [`KANNADA`](#kannada) | const |  |
| [`KATAKANA`](#katakana) | const |  |
| [`KAWI`](#kawi) | const |  |
| [`KAYAH_LI`](#kayah-li) | const |  |
| [`KHAROSHTHI`](#kharoshthi) | const |  |
| [`KHITAN_SMALL_SCRIPT`](#khitan-small-script) | const |  |
| [`KHMER`](#khmer) | const |  |
| [`KHOJKI`](#khojki) | const |  |
| [`KHUDAWADI`](#khudawadi) | const |  |
| [`KIRAT_RAI`](#kirat-rai) | const |  |
| [`LAO`](#lao) | const |  |
| [`LATIN`](#latin) | const |  |
| [`LEPCHA`](#lepcha) | const |  |
| [`LIMBU`](#limbu) | const |  |
| [`LINEAR_A`](#linear-a) | const |  |
| [`LINEAR_B`](#linear-b) | const |  |
| [`LISU`](#lisu) | const |  |
| [`LYCIAN`](#lycian) | const |  |
| [`LYDIAN`](#lydian) | const |  |
| [`MAHAJANI`](#mahajani) | const |  |
| [`MAKASAR`](#makasar) | const |  |
| [`MALAYALAM`](#malayalam) | const |  |
| [`MANDAIC`](#mandaic) | const |  |
| [`MANICHAEAN`](#manichaean) | const |  |
| [`MARCHEN`](#marchen) | const |  |
| [`MASARAM_GONDI`](#masaram-gondi) | const |  |
| [`MEDEFAIDRIN`](#medefaidrin) | const |  |
| [`MEETEI_MAYEK`](#meetei-mayek) | const |  |
| [`MENDE_KIKAKUI`](#mende-kikakui) | const |  |
| [`MEROITIC_CURSIVE`](#meroitic-cursive) | const |  |
| [`MEROITIC_HIEROGLYPHS`](#meroitic-hieroglyphs) | const |  |
| [`MIAO`](#miao) | const |  |
| [`MODI`](#modi) | const |  |
| [`MONGOLIAN`](#mongolian) | const |  |
| [`MRO`](#mro) | const |  |
| [`MULTANI`](#multani) | const |  |
| [`MYANMAR`](#myanmar) | const |  |
| [`NABATAEAN`](#nabataean) | const |  |
| [`NAG_MUNDARI`](#nag-mundari) | const |  |
| [`NANDINAGARI`](#nandinagari) | const |  |
| [`NEW_TAI_LUE`](#new-tai-lue) | const |  |
| [`NEWA`](#newa) | const |  |
| [`NKO`](#nko) | const |  |
| [`NUSHU`](#nushu) | const |  |
| [`NYIAKENG_PUACHUE_HMONG`](#nyiakeng-puachue-hmong) | const |  |
| [`OGHAM`](#ogham) | const |  |
| [`OL_CHIKI`](#ol-chiki) | const |  |
| [`OL_ONAL`](#ol-onal) | const |  |
| [`OLD_HUNGARIAN`](#old-hungarian) | const |  |
| [`OLD_ITALIC`](#old-italic) | const |  |
| [`OLD_NORTH_ARABIAN`](#old-north-arabian) | const |  |
| [`OLD_PERMIC`](#old-permic) | const |  |
| [`OLD_PERSIAN`](#old-persian) | const |  |
| [`OLD_SOGDIAN`](#old-sogdian) | const |  |
| [`OLD_SOUTH_ARABIAN`](#old-south-arabian) | const |  |
| [`OLD_TURKIC`](#old-turkic) | const |  |
| [`OLD_UYGHUR`](#old-uyghur) | const |  |
| [`ORIYA`](#oriya) | const |  |
| [`OSAGE`](#osage) | const |  |
| [`OSMANYA`](#osmanya) | const |  |
| [`PAHAWH_HMONG`](#pahawh-hmong) | const |  |
| [`PALMYRENE`](#palmyrene) | const |  |
| [`PAU_CIN_HAU`](#pau-cin-hau) | const |  |
| [`PHAGS_PA`](#phags-pa) | const |  |
| [`PHOENICIAN`](#phoenician) | const |  |
| [`PSALTER_PAHLAVI`](#psalter-pahlavi) | const |  |
| [`REJANG`](#rejang) | const |  |
| [`RUNIC`](#runic) | const |  |
| [`SAMARITAN`](#samaritan) | const |  |
| [`SAURASHTRA`](#saurashtra) | const |  |
| [`SHARADA`](#sharada) | const |  |
| [`SHAVIAN`](#shavian) | const |  |
| [`SIDDHAM`](#siddham) | const |  |
| [`SIGNWRITING`](#signwriting) | const |  |
| [`SINHALA`](#sinhala) | const |  |
| [`SOGDIAN`](#sogdian) | const |  |
| [`SORA_SOMPENG`](#sora-sompeng) | const |  |
| [`SOYOMBO`](#soyombo) | const |  |
| [`SUNDANESE`](#sundanese) | const |  |
| [`SUNUWAR`](#sunuwar) | const |  |
| [`SYLOTI_NAGRI`](#syloti-nagri) | const |  |
| [`SYRIAC`](#syriac) | const |  |
| [`TAGALOG`](#tagalog) | const |  |
| [`TAGBANWA`](#tagbanwa) | const |  |
| [`TAI_LE`](#tai-le) | const |  |
| [`TAI_THAM`](#tai-tham) | const |  |
| [`TAI_VIET`](#tai-viet) | const |  |
| [`TAKRI`](#takri) | const |  |
| [`TAMIL`](#tamil) | const |  |
| [`TANGSA`](#tangsa) | const |  |
| [`TANGUT`](#tangut) | const |  |
| [`TELUGU`](#telugu) | const |  |
| [`THAANA`](#thaana) | const |  |
| [`THAI`](#thai) | const |  |
| [`TIBETAN`](#tibetan) | const |  |
| [`TIFINAGH`](#tifinagh) | const |  |
| [`TIRHUTA`](#tirhuta) | const |  |
| [`TODHRI`](#todhri) | const |  |
| [`TOTO`](#toto) | const |  |
| [`TULU_TIGALARI`](#tulu-tigalari) | const |  |
| [`UGARITIC`](#ugaritic) | const |  |
| [`VAI`](#vai) | const |  |
| [`VITHKUQI`](#vithkuqi) | const |  |
| [`WANCHO`](#wancho) | const |  |
| [`WARANG_CITI`](#warang-citi) | const |  |
| [`YEZIDI`](#yezidi) | const |  |
| [`YI`](#yi) | const |  |
| [`ZANABAZAR_SQUARE`](#zanabazar-square) | const |  |

## Constants

### `BY_NAME`
```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:9-180`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L9-L180)*

### `ADLAM`
```rust
const ADLAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:182-183`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L182-L183)*

### `AHOM`
```rust
const AHOM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:185-186`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L185-L186)*

### `ANATOLIAN_HIEROGLYPHS`
```rust
const ANATOLIAN_HIEROGLYPHS: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:188`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L188)*

### `ARABIC`
```rust
const ARABIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:190-250`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L190-L250)*

### `ARMENIAN`
```rust
const ARMENIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:252-253`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L252-L253)*

### `AVESTAN`
```rust
const AVESTAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:255`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L255)*

### `BALINESE`
```rust
const BALINESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:257`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L257)*

### `BAMUM`
```rust
const BAMUM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:259`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L259)*

### `BASSA_VAH`
```rust
const BASSA_VAH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:261-262`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L261-L262)*

### `BATAK`
```rust
const BATAK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:264`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L264)*

### `BENGALI`
```rust
const BENGALI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:266-281`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L266-L281)*

### `BHAIKSUKI`
```rust
const BHAIKSUKI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:283-284`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L283-L284)*

### `BOPOMOFO`
```rust
const BOPOMOFO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:286-287`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L286-L287)*

### `BRAHMI`
```rust
const BRAHMI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:289-290`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L289-L290)*

### `BRAILLE`
```rust
const BRAILLE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:292`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L292)*

### `BUGINESE`
```rust
const BUGINESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:294`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L294)*

### `BUHID`
```rust
const BUHID: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:296`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L296)*

### `CANADIAN_ABORIGINAL`
```rust
const CANADIAN_ABORIGINAL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:298-299`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L298-L299)*

### `CARIAN`
```rust
const CARIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:301`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L301)*

### `CAUCASIAN_ALBANIAN`
```rust
const CAUCASIAN_ALBANIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:303-304`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L303-L304)*

### `CHAKMA`
```rust
const CHAKMA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:306-307`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L306-L307)*

### `CHAM`
```rust
const CHAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:309-310`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L309-L310)*

### `CHEROKEE`
```rust
const CHEROKEE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:312-313`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L312-L313)*

### `CHORASMIAN`
```rust
const CHORASMIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:315`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L315)*

### `COMMON`
```rust
const COMMON: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:317-492`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L317-L492)*

### `COPTIC`
```rust
const COPTIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:494-495`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L494-L495)*

### `CUNEIFORM`
```rust
const CUNEIFORM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:497-498`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L497-L498)*

### `CYPRIOT`
```rust
const CYPRIOT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:500-501`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L500-L501)*

### `CYPRO_MINOAN`
```rust
const CYPRO_MINOAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:503`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L503)*

### `CYRILLIC`
```rust
const CYRILLIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:505-516`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L505-L516)*

### `DESERET`
```rust
const DESERET: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:518`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L518)*

### `DEVANAGARI`
```rust
const DEVANAGARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:520-526`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L520-L526)*

### `DIVES_AKURU`
```rust
const DIVES_AKURU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:528-537`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L528-L537)*

### `DOGRA`
```rust
const DOGRA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:539`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L539)*

### `DUPLOYAN`
```rust
const DUPLOYAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:541-542`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L541-L542)*

### `EGYPTIAN_HIEROGLYPHS`
```rust
const EGYPTIAN_HIEROGLYPHS: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:544-545`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L544-L545)*

### `ELBASAN`
```rust
const ELBASAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:547`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L547)*

### `ELYMAIC`
```rust
const ELYMAIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:549`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L549)*

### `ETHIOPIC`
```rust
const ETHIOPIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:551-588`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L551-L588)*

### `GARAY`
```rust
const GARAY: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:590-591`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L590-L591)*

### `GEORGIAN`
```rust
const GEORGIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:593-604`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L593-L604)*

### `GLAGOLITIC`
```rust
const GLAGOLITIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:606-613`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L606-L613)*

### `GOTHIC`
```rust
const GOTHIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:615`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L615)*

### `GRANTHA`
```rust
const GRANTHA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:617-633`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L617-L633)*

### `GREEK`
```rust
const GREEK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:635-672`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L635-L672)*

### `GUJARATI`
```rust
const GUJARATI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:674-689`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L674-L689)*

### `GUNJALA_GONDI`
```rust
const GUNJALA_GONDI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:691-698`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L691-L698)*

### `GURMUKHI`
```rust
const GURMUKHI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:700-717`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L700-L717)*

### `GURUNG_KHEMA`
```rust
const GURUNG_KHEMA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:719`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L719)*

### `HAN`
```rust
const HAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:721-744`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L721-L744)*

### `HANGUL`
```rust
const HANGUL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:746-761`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L746-L761)*

### `HANIFI_ROHINGYA`
```rust
const HANIFI_ROHINGYA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:763-764`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L763-L764)*

### `HANUNOO`
```rust
const HANUNOO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:766`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L766)*

### `HATRAN`
```rust
const HATRAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:768-769`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L768-L769)*

### `HEBREW`
```rust
const HEBREW: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:771-781`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L771-L781)*

### `HIRAGANA`
```rust
const HIRAGANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:783-790`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L783-L790)*

### `IMPERIAL_ARAMAIC`
```rust
const IMPERIAL_ARAMAIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:792-793`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L792-L793)*

### `INHERITED`
```rust
const INHERITED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:795-825`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L795-L825)*

### `INSCRIPTIONAL_PAHLAVI`
```rust
const INSCRIPTIONAL_PAHLAVI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:827-828`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L827-L828)*

### `INSCRIPTIONAL_PARTHIAN`
```rust
const INSCRIPTIONAL_PARTHIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:830-831`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L830-L831)*

### `JAVANESE`
```rust
const JAVANESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:833-834`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L833-L834)*

### `KAITHI`
```rust
const KAITHI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:836-837`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L836-L837)*

### `KANNADA`
```rust
const KANNADA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:839-853`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L839-L853)*

### `KATAKANA`
```rust
const KATAKANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:855-870`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L855-L870)*

### `KAWI`
```rust
const KAWI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:872-873`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L872-L873)*

### `KAYAH_LI`
```rust
const KAYAH_LI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:875`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L875)*

### `KHAROSHTHI`
```rust
const KHAROSHTHI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:877-886`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L877-L886)*

### `KHITAN_SMALL_SCRIPT`
```rust
const KHITAN_SMALL_SCRIPT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:888-889`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L888-L889)*

### `KHMER`
```rust
const KHMER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:891-892`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L891-L892)*

### `KHOJKI`
```rust
const KHOJKI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:894`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L894)*

### `KHUDAWADI`
```rust
const KHUDAWADI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:896-897`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L896-L897)*

### `KIRAT_RAI`
```rust
const KIRAT_RAI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:899`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L899)*

### `LAO`
```rust
const LAO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:901-913`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L901-L913)*

### `LATIN`
```rust
const LATIN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:915-955`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L915-L955)*

### `LEPCHA`
```rust
const LEPCHA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:957-958`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L957-L958)*

### `LIMBU`
```rust
const LIMBU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:960-966`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L960-L966)*

### `LINEAR_A`
```rust
const LINEAR_A: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:968-969`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L968-L969)*

### `LINEAR_B`
```rust
const LINEAR_B: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:971-979`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L971-L979)*

### `LISU`
```rust
const LISU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:981`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L981)*

### `LYCIAN`
```rust
const LYCIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:983`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L983)*

### `LYDIAN`
```rust
const LYDIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:985`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L985)*

### `MAHAJANI`
```rust
const MAHAJANI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:987`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L987)*

### `MAKASAR`
```rust
const MAKASAR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:989`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L989)*

### `MALAYALAM`
```rust
const MALAYALAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:991-999`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L991-L999)*

### `MANDAIC`
```rust
const MANDAIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1001`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1001)*

### `MANICHAEAN`
```rust
const MANICHAEAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1003-1004`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1003-L1004)*

### `MARCHEN`
```rust
const MARCHEN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1006-1007`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1006-L1007)*

### `MASARAM_GONDI`
```rust
const MASARAM_GONDI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1009-1017`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1009-L1017)*

### `MEDEFAIDRIN`
```rust
const MEDEFAIDRIN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1019`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1019)*

### `MEETEI_MAYEK`
```rust
const MEETEI_MAYEK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1021-1022`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1021-L1022)*

### `MENDE_KIKAKUI`
```rust
const MENDE_KIKAKUI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1024-1025`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1024-L1025)*

### `MEROITIC_CURSIVE`
```rust
const MEROITIC_CURSIVE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1027-1028`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1027-L1028)*

### `MEROITIC_HIEROGLYPHS`
```rust
const MEROITIC_HIEROGLYPHS: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1030`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1030)*

### `MIAO`
```rust
const MIAO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1032-1033`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1032-L1033)*

### `MODI`
```rust
const MODI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1035`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1035)*

### `MONGOLIAN`
```rust
const MONGOLIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1037-1038`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1037-L1038)*

### `MRO`
```rust
const MRO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1040`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1040)*

### `MULTANI`
```rust
const MULTANI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1042-1043`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1042-L1043)*

### `MYANMAR`
```rust
const MYANMAR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1045-1046`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1045-L1046)*

### `NABATAEAN`
```rust
const NABATAEAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1048`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1048)*

### `NAG_MUNDARI`
```rust
const NAG_MUNDARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1050`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1050)*

### `NANDINAGARI`
```rust
const NANDINAGARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1052-1053`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1052-L1053)*

### `NEW_TAI_LUE`
```rust
const NEW_TAI_LUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1055-1056`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1055-L1056)*

### `NEWA`
```rust
const NEWA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1058`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1058)*

### `NKO`
```rust
const NKO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1060`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1060)*

### `NUSHU`
```rust
const NUSHU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1062`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1062)*

### `NYIAKENG_PUACHUE_HMONG`
```rust
const NYIAKENG_PUACHUE_HMONG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1064-1065`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1064-L1065)*

### `OGHAM`
```rust
const OGHAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1067`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1067)*

### `OL_CHIKI`
```rust
const OL_CHIKI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1069`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1069)*

### `OL_ONAL`
```rust
const OL_ONAL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1071`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1071)*

### `OLD_HUNGARIAN`
```rust
const OLD_HUNGARIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1073-1074`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1073-L1074)*

### `OLD_ITALIC`
```rust
const OLD_ITALIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1076`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1076)*

### `OLD_NORTH_ARABIAN`
```rust
const OLD_NORTH_ARABIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1078`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1078)*

### `OLD_PERMIC`
```rust
const OLD_PERMIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1080`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1080)*

### `OLD_PERSIAN`
```rust
const OLD_PERSIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1082`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1082)*

### `OLD_SOGDIAN`
```rust
const OLD_SOGDIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1084`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1084)*

### `OLD_SOUTH_ARABIAN`
```rust
const OLD_SOUTH_ARABIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1086`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1086)*

### `OLD_TURKIC`
```rust
const OLD_TURKIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1088`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1088)*

### `OLD_UYGHUR`
```rust
const OLD_UYGHUR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1090`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1090)*

### `ORIYA`
```rust
const ORIYA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1092-1107`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1092-L1107)*

### `OSAGE`
```rust
const OSAGE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1109`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1109)*

### `OSMANYA`
```rust
const OSMANYA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1111`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1111)*

### `PAHAWH_HMONG`
```rust
const PAHAWH_HMONG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1113-1114`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1113-L1114)*

### `PALMYRENE`
```rust
const PALMYRENE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1116`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1116)*

### `PAU_CIN_HAU`
```rust
const PAU_CIN_HAU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1118`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1118)*

### `PHAGS_PA`
```rust
const PHAGS_PA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1120`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1120)*

### `PHOENICIAN`
```rust
const PHOENICIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1122`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1122)*

### `PSALTER_PAHLAVI`
```rust
const PSALTER_PAHLAVI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1124-1125`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1124-L1125)*

### `REJANG`
```rust
const REJANG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1127`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1127)*

### `RUNIC`
```rust
const RUNIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1129`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1129)*

### `SAMARITAN`
```rust
const SAMARITAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1131`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1131)*

### `SAURASHTRA`
```rust
const SAURASHTRA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1133-1134`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1133-L1134)*

### `SHARADA`
```rust
const SHARADA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1136`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1136)*

### `SHAVIAN`
```rust
const SHAVIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1138`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1138)*

### `SIDDHAM`
```rust
const SIDDHAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1140-1141`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1140-L1141)*

### `SIGNWRITING`
```rust
const SIGNWRITING: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1143-1144`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1143-L1144)*

### `SINHALA`
```rust
const SINHALA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1146-1160`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1146-L1160)*

### `SOGDIAN`
```rust
const SOGDIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1162`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1162)*

### `SORA_SOMPENG`
```rust
const SORA_SOMPENG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1164`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1164)*

### `SOYOMBO`
```rust
const SOYOMBO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1166`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1166)*

### `SUNDANESE`
```rust
const SUNDANESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1168-1169`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1168-L1169)*

### `SUNUWAR`
```rust
const SUNUWAR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1171`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1171)*

### `SYLOTI_NAGRI`
```rust
const SYLOTI_NAGRI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1173`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1173)*

### `SYRIAC`
```rust
const SYRIAC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1175-1176`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1175-L1176)*

### `TAGALOG`
```rust
const TAGALOG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1178`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1178)*

### `TAGBANWA`
```rust
const TAGBANWA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1180-1181`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1180-L1181)*

### `TAI_LE`
```rust
const TAI_LE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1183`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1183)*

### `TAI_THAM`
```rust
const TAI_THAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1185-1191`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1185-L1191)*

### `TAI_VIET`
```rust
const TAI_VIET: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1193`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1193)*

### `TAKRI`
```rust
const TAKRI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1195`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1195)*

### `TAMIL`
```rust
const TAMIL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1197-1216`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1197-L1216)*

### `TANGSA`
```rust
const TANGSA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1218`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1218)*

### `TANGUT`
```rust
const TANGUT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1220-1221`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1220-L1221)*

### `TELUGU`
```rust
const TELUGU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1223-1237`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1223-L1237)*

### `THAANA`
```rust
const THAANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1239`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1239)*

### `THAI`
```rust
const THAI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1241`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1241)*

### `TIBETAN`
```rust
const TIBETAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1243-1251`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1243-L1251)*

### `TIFINAGH`
```rust
const TIFINAGH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1253-1254`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1253-L1254)*

### `TIRHUTA`
```rust
const TIRHUTA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1256`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1256)*

### `TODHRI`
```rust
const TODHRI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1258`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1258)*

### `TOTO`
```rust
const TOTO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1260`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1260)*

### `TULU_TIGALARI`
```rust
const TULU_TIGALARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1262-1274`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1262-L1274)*

### `UGARITIC`
```rust
const UGARITIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1276`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1276)*

### `VAI`
```rust
const VAI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1278`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1278)*

### `VITHKUQI`
```rust
const VITHKUQI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1280-1289`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1280-L1289)*

### `WANCHO`
```rust
const WANCHO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1291`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1291)*

### `WARANG_CITI`
```rust
const WARANG_CITI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1293`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1293)*

### `YEZIDI`
```rust
const YEZIDI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1295-1296`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1295-L1296)*

### `YI`
```rust
const YI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1298`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1298)*

### `ZANABAZAR_SQUARE`
```rust
const ZANABAZAR_SQUARE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script.rs:1300`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script.rs#L1300)*

