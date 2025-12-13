*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [script_extension](index.md)*

---

# Module `script_extension`

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

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:9-180`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L9-L180)*

### `ADLAM`
```rust
const ADLAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:182-190`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L182-L190)*

### `AHOM`
```rust
const AHOM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:192-193`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L192-L193)*

### `ANATOLIAN_HIEROGLYPHS`
```rust
const ANATOLIAN_HIEROGLYPHS: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:195`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L195)*

### `ARABIC`
```rust
const ARABIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:197-253`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L197-L253)*

### `ARMENIAN`
```rust
const ARMENIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:255-256`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L255-L256)*

### `AVESTAN`
```rust
const AVESTAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:258-259`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L258-L259)*

### `BALINESE`
```rust
const BALINESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:261`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L261)*

### `BAMUM`
```rust
const BAMUM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:263`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L263)*

### `BASSA_VAH`
```rust
const BASSA_VAH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:265-266`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L265-L266)*

### `BATAK`
```rust
const BATAK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:268`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L268)*

### `BENGALI`
```rust
const BENGALI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:270-298`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L270-L298)*

### `BHAIKSUKI`
```rust
const BHAIKSUKI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:300-301`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L300-L301)*

### `BOPOMOFO`
```rust
const BOPOMOFO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:303-319`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L303-L319)*

### `BRAHMI`
```rust
const BRAHMI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:321-322`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L321-L322)*

### `BRAILLE`
```rust
const BRAILLE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:324`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L324)*

### `BUGINESE`
```rust
const BUGINESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:326-327`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L326-L327)*

### `BUHID`
```rust
const BUHID: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:329`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L329)*

### `CANADIAN_ABORIGINAL`
```rust
const CANADIAN_ABORIGINAL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:331-332`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L331-L332)*

### `CARIAN`
```rust
const CARIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:334-335`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L334-L335)*

### `CAUCASIAN_ALBANIAN`
```rust
const CAUCASIAN_ALBANIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:337-343`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L337-L343)*

### `CHAKMA`
```rust
const CHAKMA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:345-346`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L345-L346)*

### `CHAM`
```rust
const CHAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:348-349`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L348-L349)*

### `CHEROKEE`
```rust
const CHEROKEE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:351-360`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L351-L360)*

### `CHORASMIAN`
```rust
const CHORASMIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:362`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L362)*

### `COMMON`
```rust
const COMMON: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:364-524`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L364-L524)*

### `COPTIC`
```rust
const COPTIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:526-537`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L526-L537)*

### `CUNEIFORM`
```rust
const CUNEIFORM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:539-540`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L539-L540)*

### `CYPRIOT`
```rust
const CYPRIOT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:542-552`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L542-L552)*

### `CYPRO_MINOAN`
```rust
const CYPRO_MINOAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:554`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L554)*

### `CYRILLIC`
```rust
const CYRILLIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:556-575`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L556-L575)*

### `DESERET`
```rust
const DESERET: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:577`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L577)*

### `DEVANAGARI`
```rust
const DEVANAGARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:579-589`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L579-L589)*

### `DIVES_AKURU`
```rust
const DIVES_AKURU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:591-600`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L591-L600)*

### `DOGRA`
```rust
const DOGRA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:602-603`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L602-L603)*

### `DUPLOYAN`
```rust
const DUPLOYAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:605-616`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L605-L616)*

### `EGYPTIAN_HIEROGLYPHS`
```rust
const EGYPTIAN_HIEROGLYPHS: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:618-619`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L618-L619)*

### `ELBASAN`
```rust
const ELBASAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:621-622`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L621-L622)*

### `ELYMAIC`
```rust
const ELYMAIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:624`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L624)*

### `ETHIOPIC`
```rust
const ETHIOPIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:626-664`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L626-L664)*

### `GARAY`
```rust
const GARAY: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:666-673`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L666-L673)*

### `GEORGIAN`
```rust
const GEORGIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:675-689`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L675-L689)*

### `GLAGOLITIC`
```rust
const GLAGOLITIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:691-708`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L691-L708)*

### `GOTHIC`
```rust
const GOTHIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:710-716`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L710-L716)*

### `GRANTHA`
```rust
const GRANTHA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:718-744`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L718-L744)*

### `GREEK`
```rust
const GREEK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:746-791`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L746-L791)*

### `GUJARATI`
```rust
const GUJARATI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:793-811`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L793-L811)*

### `GUNJALA_GONDI`
```rust
const GUNJALA_GONDI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:813-822`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L813-L822)*

### `GURMUKHI`
```rust
const GURMUKHI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:824-844`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L824-L844)*

### `GURUNG_KHEMA`
```rust
const GURUNG_KHEMA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:846`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L846)*

### `HAN`
```rust
const HAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:848-891`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L848-L891)*

### `HANGUL`
```rust
const HANGUL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:893-915`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L893-L915)*

### `HANIFI_ROHINGYA`
```rust
const HANIFI_ROHINGYA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:917-925`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L917-L925)*

### `HANUNOO`
```rust
const HANUNOO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:927`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L927)*

### `HATRAN`
```rust
const HATRAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:929-930`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L929-L930)*

### `HEBREW`
```rust
const HEBREW: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:932-943`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L932-L943)*

### `HIRAGANA`
```rust
const HIRAGANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:945-963`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L945-L963)*

### `IMPERIAL_ARAMAIC`
```rust
const IMPERIAL_ARAMAIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:965-966`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L965-L966)*

### `INHERITED`
```rust
const INHERITED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:968-997`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L968-L997)*

### `INSCRIPTIONAL_PAHLAVI`
```rust
const INSCRIPTIONAL_PAHLAVI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:999-1000`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L999-L1000)*

### `INSCRIPTIONAL_PARTHIAN`
```rust
const INSCRIPTIONAL_PARTHIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1002-1003`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1002-L1003)*

### `JAVANESE`
```rust
const JAVANESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1005-1006`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1005-L1006)*

### `KAITHI`
```rust
const KAITHI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1008-1014`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1008-L1014)*

### `KANNADA`
```rust
const KANNADA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1016-1038`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1016-L1038)*

### `KATAKANA`
```rust
const KATAKANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1040-1063`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1040-L1063)*

### `KAWI`
```rust
const KAWI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1065-1066`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1065-L1066)*

### `KAYAH_LI`
```rust
const KAYAH_LI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1068`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1068)*

### `KHAROSHTHI`
```rust
const KHAROSHTHI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1070-1079`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1070-L1079)*

### `KHITAN_SMALL_SCRIPT`
```rust
const KHITAN_SMALL_SCRIPT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1081-1082`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1081-L1082)*

### `KHMER`
```rust
const KHMER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1084-1085`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1084-L1085)*

### `KHOJKI`
```rust
const KHOJKI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1087-1088`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1087-L1088)*

### `KHUDAWADI`
```rust
const KHUDAWADI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1090-1091`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1090-L1091)*

### `KIRAT_RAI`
```rust
const KIRAT_RAI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1093`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1093)*

### `LAO`
```rust
const LAO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1095-1107`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1095-L1107)*

### `LATIN`
```rust
const LATIN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1109-1175`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1109-L1175)*

### `LEPCHA`
```rust
const LEPCHA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1177-1178`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1177-L1178)*

### `LIMBU`
```rust
const LIMBU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1180-1187`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1180-L1187)*

### `LINEAR_A`
```rust
const LINEAR_A: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1189-1190`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1189-L1190)*

### `LINEAR_B`
```rust
const LINEAR_B: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1192-1203`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1192-L1203)*

### `LISU`
```rust
const LISU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1205-1206`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1205-L1206)*

### `LYCIAN`
```rust
const LYCIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1208`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1208)*

### `LYDIAN`
```rust
const LYDIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1210-1211`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1210-L1211)*

### `MAHAJANI`
```rust
const MAHAJANI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1213-1214`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1213-L1214)*

### `MAKASAR`
```rust
const MAKASAR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1216`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1216)*

### `MALAYALAM`
```rust
const MALAYALAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1218-1231`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1218-L1231)*

### `MANDAIC`
```rust
const MANDAIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1233-1234`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1233-L1234)*

### `MANICHAEAN`
```rust
const MANICHAEAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1236-1237`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1236-L1237)*

### `MARCHEN`
```rust
const MARCHEN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1239-1240`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1239-L1240)*

### `MASARAM_GONDI`
```rust
const MASARAM_GONDI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1242-1251`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1242-L1251)*

### `MEDEFAIDRIN`
```rust
const MEDEFAIDRIN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1253`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1253)*

### `MEETEI_MAYEK`
```rust
const MEETEI_MAYEK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1255-1256`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1255-L1256)*

### `MENDE_KIKAKUI`
```rust
const MENDE_KIKAKUI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1258-1259`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1258-L1259)*

### `MEROITIC_CURSIVE`
```rust
const MEROITIC_CURSIVE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1261-1262`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1261-L1262)*

### `MEROITIC_HIEROGLYPHS`
```rust
const MEROITIC_HIEROGLYPHS: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1264-1265`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1264-L1265)*

### `MIAO`
```rust
const MIAO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1267-1268`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1267-L1268)*

### `MODI`
```rust
const MODI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1270-1271`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1270-L1271)*

### `MONGOLIAN`
```rust
const MONGOLIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1273-1281`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1273-L1281)*

### `MRO`
```rust
const MRO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1283`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1283)*

### `MULTANI`
```rust
const MULTANI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1285-1286`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1285-L1286)*

### `MYANMAR`
```rust
const MYANMAR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1288-1289`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1288-L1289)*

### `NABATAEAN`
```rust
const NABATAEAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1291`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1291)*

### `NAG_MUNDARI`
```rust
const NAG_MUNDARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1293`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1293)*

### `NANDINAGARI`
```rust
const NANDINAGARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1295-1305`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1295-L1305)*

### `NEW_TAI_LUE`
```rust
const NEW_TAI_LUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1307-1308`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1307-L1308)*

### `NEWA`
```rust
const NEWA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1310`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1310)*

### `NKO`
```rust
const NKO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1312-1319`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1312-L1319)*

### `NUSHU`
```rust
const NUSHU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1321`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1321)*

### `NYIAKENG_PUACHUE_HMONG`
```rust
const NYIAKENG_PUACHUE_HMONG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1323-1324`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1323-L1324)*

### `OGHAM`
```rust
const OGHAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1326`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1326)*

### `OL_CHIKI`
```rust
const OL_CHIKI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1328`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1328)*

### `OL_ONAL`
```rust
const OL_ONAL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1330-1331`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1330-L1331)*

### `OLD_HUNGARIAN`
```rust
const OLD_HUNGARIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1333-1341`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1333-L1341)*

### `OLD_ITALIC`
```rust
const OLD_ITALIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1343`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1343)*

### `OLD_NORTH_ARABIAN`
```rust
const OLD_NORTH_ARABIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1345`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1345)*

### `OLD_PERMIC`
```rust
const OLD_PERMIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1347-1354`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1347-L1354)*

### `OLD_PERSIAN`
```rust
const OLD_PERSIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1356`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1356)*

### `OLD_SOGDIAN`
```rust
const OLD_SOGDIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1358`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1358)*

### `OLD_SOUTH_ARABIAN`
```rust
const OLD_SOUTH_ARABIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1360`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1360)*

### `OLD_TURKIC`
```rust
const OLD_TURKIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1362-1363`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1362-L1363)*

### `OLD_UYGHUR`
```rust
const OLD_UYGHUR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1365-1366`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1365-L1366)*

### `ORIYA`
```rust
const ORIYA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1368-1387`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1368-L1387)*

### `OSAGE`
```rust
const OSAGE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1389-1396`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1389-L1396)*

### `OSMANYA`
```rust
const OSMANYA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1398`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1398)*

### `PAHAWH_HMONG`
```rust
const PAHAWH_HMONG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1400-1401`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1400-L1401)*

### `PALMYRENE`
```rust
const PALMYRENE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1403`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1403)*

### `PAU_CIN_HAU`
```rust
const PAU_CIN_HAU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1405`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1405)*

### `PHAGS_PA`
```rust
const PHAGS_PA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1407-1413`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1407-L1413)*

### `PHOENICIAN`
```rust
const PHOENICIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1415`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1415)*

### `PSALTER_PAHLAVI`
```rust
const PSALTER_PAHLAVI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1417-1418`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1417-L1418)*

### `REJANG`
```rust
const REJANG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1420`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1420)*

### `RUNIC`
```rust
const RUNIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1422`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1422)*

### `SAMARITAN`
```rust
const SAMARITAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1424-1425`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1424-L1425)*

### `SAURASHTRA`
```rust
const SAURASHTRA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1427-1428`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1427-L1428)*

### `SHARADA`
```rust
const SHARADA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1430-1439`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1430-L1439)*

### `SHAVIAN`
```rust
const SHAVIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1441`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1441)*

### `SIDDHAM`
```rust
const SIDDHAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1443-1444`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1443-L1444)*

### `SIGNWRITING`
```rust
const SIGNWRITING: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1446-1447`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1446-L1447)*

### `SINHALA`
```rust
const SINHALA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1449-1465`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1449-L1465)*

### `SOGDIAN`
```rust
const SOGDIAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1467`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1467)*

### `SORA_SOMPENG`
```rust
const SORA_SOMPENG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1469`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1469)*

### `SOYOMBO`
```rust
const SOYOMBO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1471`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1471)*

### `SUNDANESE`
```rust
const SUNDANESE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1473-1474`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1473-L1474)*

### `SUNUWAR`
```rust
const SUNUWAR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1476-1485`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1476-L1485)*

### `SYLOTI_NAGRI`
```rust
const SYLOTI_NAGRI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1487-1488`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1487-L1488)*

### `SYRIAC`
```rust
const SYRIAC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1490-1510`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1490-L1510)*

### `TAGALOG`
```rust
const TAGALOG: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1512-1513`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1512-L1513)*

### `TAGBANWA`
```rust
const TAGBANWA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1515-1516`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1515-L1516)*

### `TAI_LE`
```rust
const TAI_LE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1518-1525`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1518-L1525)*

### `TAI_THAM`
```rust
const TAI_THAM: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1527-1533`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1527-L1533)*

### `TAI_VIET`
```rust
const TAI_VIET: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1535`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1535)*

### `TAKRI`
```rust
const TAKRI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1537-1538`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1537-L1538)*

### `TAMIL`
```rust
const TAMIL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1540-1566`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1540-L1566)*

### `TANGSA`
```rust
const TANGSA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1568`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1568)*

### `TANGUT`
```rust
const TANGUT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1570-1577`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1570-L1577)*

### `TELUGU`
```rust
const TELUGU: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1579-1597`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1579-L1597)*

### `THAANA`
```rust
const THAANA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1599-1607`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1599-L1607)*

### `THAI`
```rust
const THAI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1609-1616`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1609-L1616)*

### `TIBETAN`
```rust
const TIBETAN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1618-1627`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1618-L1627)*

### `TIFINAGH`
```rust
const TIFINAGH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1629-1637`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1629-L1637)*

### `TIRHUTA`
```rust
const TIRHUTA: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1639-1646`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1639-L1646)*

### `TODHRI`
```rust
const TODHRI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1648-1656`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1648-L1656)*

### `TOTO`
```rust
const TOTO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1658`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1658)*

### `TULU_TIGALARI`
```rust
const TULU_TIGALARI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1660-1677`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1660-L1677)*

### `UGARITIC`
```rust
const UGARITIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1679`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1679)*

### `VAI`
```rust
const VAI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1681`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1681)*

### `VITHKUQI`
```rust
const VITHKUQI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1683-1692`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1683-L1692)*

### `WANCHO`
```rust
const WANCHO: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1694`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1694)*

### `WARANG_CITI`
```rust
const WARANG_CITI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1696`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1696)*

### `YEZIDI`
```rust
const YEZIDI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1698-1706`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1698-L1706)*

### `YI`
```rust
const YI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1708-1716`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1708-L1716)*

### `ZANABAZAR_SQUARE`
```rust
const ZANABAZAR_SQUARE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/script_extension.rs:1718`](../../../../.source_1765633015/regex-syntax-0.8.8/src/unicode_tables/script_extension.rs#L1718)*

