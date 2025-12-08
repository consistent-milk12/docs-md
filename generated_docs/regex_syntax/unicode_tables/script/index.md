*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [script](index.md)*

---

# Module `script`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by_name)
  - [`ADLAM`](#adlam)
  - [`AHOM`](#ahom)
  - [`ANATOLIAN_HIEROGLYPHS`](#anatolian_hieroglyphs)
  - [`ARABIC`](#arabic)
  - [`ARMENIAN`](#armenian)
  - [`AVESTAN`](#avestan)
  - [`BALINESE`](#balinese)
  - [`BAMUM`](#bamum)
  - [`BASSA_VAH`](#bassa_vah)
  - [`BATAK`](#batak)
  - [`BENGALI`](#bengali)
  - [`BHAIKSUKI`](#bhaiksuki)
  - [`BOPOMOFO`](#bopomofo)
  - [`BRAHMI`](#brahmi)
  - [`BRAILLE`](#braille)
  - [`BUGINESE`](#buginese)
  - [`BUHID`](#buhid)
  - [`CANADIAN_ABORIGINAL`](#canadian_aboriginal)
  - [`CARIAN`](#carian)
  - [`CAUCASIAN_ALBANIAN`](#caucasian_albanian)
  - [`CHAKMA`](#chakma)
  - [`CHAM`](#cham)
  - [`CHEROKEE`](#cherokee)
  - [`CHORASMIAN`](#chorasmian)
  - [`COMMON`](#common)
  - [`COPTIC`](#coptic)
  - [`CUNEIFORM`](#cuneiform)
  - [`CYPRIOT`](#cypriot)
  - [`CYPRO_MINOAN`](#cypro_minoan)
  - [`CYRILLIC`](#cyrillic)
  - [`DESERET`](#deseret)
  - [`DEVANAGARI`](#devanagari)
  - [`DIVES_AKURU`](#dives_akuru)
  - [`DOGRA`](#dogra)
  - [`DUPLOYAN`](#duployan)
  - [`EGYPTIAN_HIEROGLYPHS`](#egyptian_hieroglyphs)
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
  - [`GUNJALA_GONDI`](#gunjala_gondi)
  - [`GURMUKHI`](#gurmukhi)
  - [`GURUNG_KHEMA`](#gurung_khema)
  - [`HAN`](#han)
  - [`HANGUL`](#hangul)
  - [`HANIFI_ROHINGYA`](#hanifi_rohingya)
  - [`HANUNOO`](#hanunoo)
  - [`HATRAN`](#hatran)
  - [`HEBREW`](#hebrew)
  - [`HIRAGANA`](#hiragana)
  - [`IMPERIAL_ARAMAIC`](#imperial_aramaic)
  - [`INHERITED`](#inherited)
  - [`INSCRIPTIONAL_PAHLAVI`](#inscriptional_pahlavi)
  - [`INSCRIPTIONAL_PARTHIAN`](#inscriptional_parthian)
  - [`JAVANESE`](#javanese)
  - [`KAITHI`](#kaithi)
  - [`KANNADA`](#kannada)
  - [`KATAKANA`](#katakana)
  - [`KAWI`](#kawi)
  - [`KAYAH_LI`](#kayah_li)
  - [`KHAROSHTHI`](#kharoshthi)
  - [`KHITAN_SMALL_SCRIPT`](#khitan_small_script)
  - [`KHMER`](#khmer)
  - [`KHOJKI`](#khojki)
  - [`KHUDAWADI`](#khudawadi)
  - [`KIRAT_RAI`](#kirat_rai)
  - [`LAO`](#lao)
  - [`LATIN`](#latin)
  - [`LEPCHA`](#lepcha)
  - [`LIMBU`](#limbu)
  - [`LINEAR_A`](#linear_a)
  - [`LINEAR_B`](#linear_b)
  - [`LISU`](#lisu)
  - [`LYCIAN`](#lycian)
  - [`LYDIAN`](#lydian)
  - [`MAHAJANI`](#mahajani)
  - [`MAKASAR`](#makasar)
  - [`MALAYALAM`](#malayalam)
  - [`MANDAIC`](#mandaic)
  - [`MANICHAEAN`](#manichaean)
  - [`MARCHEN`](#marchen)
  - [`MASARAM_GONDI`](#masaram_gondi)
  - [`MEDEFAIDRIN`](#medefaidrin)
  - [`MEETEI_MAYEK`](#meetei_mayek)
  - [`MENDE_KIKAKUI`](#mende_kikakui)
  - [`MEROITIC_CURSIVE`](#meroitic_cursive)
  - [`MEROITIC_HIEROGLYPHS`](#meroitic_hieroglyphs)
  - [`MIAO`](#miao)
  - [`MODI`](#modi)
  - [`MONGOLIAN`](#mongolian)
  - [`MRO`](#mro)
  - [`MULTANI`](#multani)
  - [`MYANMAR`](#myanmar)
  - [`NABATAEAN`](#nabataean)
  - [`NAG_MUNDARI`](#nag_mundari)
  - [`NANDINAGARI`](#nandinagari)
  - [`NEW_TAI_LUE`](#new_tai_lue)
  - [`NEWA`](#newa)
  - [`NKO`](#nko)
  - [`NUSHU`](#nushu)
  - [`NYIAKENG_PUACHUE_HMONG`](#nyiakeng_puachue_hmong)
  - [`OGHAM`](#ogham)
  - [`OL_CHIKI`](#ol_chiki)
  - [`OL_ONAL`](#ol_onal)
  - [`OLD_HUNGARIAN`](#old_hungarian)
  - [`OLD_ITALIC`](#old_italic)
  - [`OLD_NORTH_ARABIAN`](#old_north_arabian)
  - [`OLD_PERMIC`](#old_permic)
  - [`OLD_PERSIAN`](#old_persian)
  - [`OLD_SOGDIAN`](#old_sogdian)
  - [`OLD_SOUTH_ARABIAN`](#old_south_arabian)
  - [`OLD_TURKIC`](#old_turkic)
  - [`OLD_UYGHUR`](#old_uyghur)
  - [`ORIYA`](#oriya)
  - [`OSAGE`](#osage)
  - [`OSMANYA`](#osmanya)
  - [`PAHAWH_HMONG`](#pahawh_hmong)
  - [`PALMYRENE`](#palmyrene)
  - [`PAU_CIN_HAU`](#pau_cin_hau)
  - [`PHAGS_PA`](#phags_pa)
  - [`PHOENICIAN`](#phoenician)
  - [`PSALTER_PAHLAVI`](#psalter_pahlavi)
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
  - [`SORA_SOMPENG`](#sora_sompeng)
  - [`SOYOMBO`](#soyombo)
  - [`SUNDANESE`](#sundanese)
  - [`SUNUWAR`](#sunuwar)
  - [`SYLOTI_NAGRI`](#syloti_nagri)
  - [`SYRIAC`](#syriac)
  - [`TAGALOG`](#tagalog)
  - [`TAGBANWA`](#tagbanwa)
  - [`TAI_LE`](#tai_le)
  - [`TAI_THAM`](#tai_tham)
  - [`TAI_VIET`](#tai_viet)
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
  - [`TULU_TIGALARI`](#tulu_tigalari)
  - [`UGARITIC`](#ugaritic)
  - [`VAI`](#vai)
  - [`VITHKUQI`](#vithkuqi)
  - [`WANCHO`](#wancho)
  - [`WARANG_CITI`](#warang_citi)
  - [`YEZIDI`](#yezidi)
  - [`YI`](#yi)
  - [`ZANABAZAR_SQUARE`](#zanabazar_square)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by_name) | const |  |
| [`ADLAM`](#adlam) | const |  |
| [`AHOM`](#ahom) | const |  |
| [`ANATOLIAN_HIEROGLYPHS`](#anatolian_hieroglyphs) | const |  |
| [`ARABIC`](#arabic) | const |  |
| [`ARMENIAN`](#armenian) | const |  |
| [`AVESTAN`](#avestan) | const |  |
| [`BALINESE`](#balinese) | const |  |
| [`BAMUM`](#bamum) | const |  |
| [`BASSA_VAH`](#bassa_vah) | const |  |
| [`BATAK`](#batak) | const |  |
| [`BENGALI`](#bengali) | const |  |
| [`BHAIKSUKI`](#bhaiksuki) | const |  |
| [`BOPOMOFO`](#bopomofo) | const |  |
| [`BRAHMI`](#brahmi) | const |  |
| [`BRAILLE`](#braille) | const |  |
| [`BUGINESE`](#buginese) | const |  |
| [`BUHID`](#buhid) | const |  |
| [`CANADIAN_ABORIGINAL`](#canadian_aboriginal) | const |  |
| [`CARIAN`](#carian) | const |  |
| [`CAUCASIAN_ALBANIAN`](#caucasian_albanian) | const |  |
| [`CHAKMA`](#chakma) | const |  |
| [`CHAM`](#cham) | const |  |
| [`CHEROKEE`](#cherokee) | const |  |
| [`CHORASMIAN`](#chorasmian) | const |  |
| [`COMMON`](#common) | const |  |
| [`COPTIC`](#coptic) | const |  |
| [`CUNEIFORM`](#cuneiform) | const |  |
| [`CYPRIOT`](#cypriot) | const |  |
| [`CYPRO_MINOAN`](#cypro_minoan) | const |  |
| [`CYRILLIC`](#cyrillic) | const |  |
| [`DESERET`](#deseret) | const |  |
| [`DEVANAGARI`](#devanagari) | const |  |
| [`DIVES_AKURU`](#dives_akuru) | const |  |
| [`DOGRA`](#dogra) | const |  |
| [`DUPLOYAN`](#duployan) | const |  |
| [`EGYPTIAN_HIEROGLYPHS`](#egyptian_hieroglyphs) | const |  |
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
| [`GUNJALA_GONDI`](#gunjala_gondi) | const |  |
| [`GURMUKHI`](#gurmukhi) | const |  |
| [`GURUNG_KHEMA`](#gurung_khema) | const |  |
| [`HAN`](#han) | const |  |
| [`HANGUL`](#hangul) | const |  |
| [`HANIFI_ROHINGYA`](#hanifi_rohingya) | const |  |
| [`HANUNOO`](#hanunoo) | const |  |
| [`HATRAN`](#hatran) | const |  |
| [`HEBREW`](#hebrew) | const |  |
| [`HIRAGANA`](#hiragana) | const |  |
| [`IMPERIAL_ARAMAIC`](#imperial_aramaic) | const |  |
| [`INHERITED`](#inherited) | const |  |
| [`INSCRIPTIONAL_PAHLAVI`](#inscriptional_pahlavi) | const |  |
| [`INSCRIPTIONAL_PARTHIAN`](#inscriptional_parthian) | const |  |
| [`JAVANESE`](#javanese) | const |  |
| [`KAITHI`](#kaithi) | const |  |
| [`KANNADA`](#kannada) | const |  |
| [`KATAKANA`](#katakana) | const |  |
| [`KAWI`](#kawi) | const |  |
| [`KAYAH_LI`](#kayah_li) | const |  |
| [`KHAROSHTHI`](#kharoshthi) | const |  |
| [`KHITAN_SMALL_SCRIPT`](#khitan_small_script) | const |  |
| [`KHMER`](#khmer) | const |  |
| [`KHOJKI`](#khojki) | const |  |
| [`KHUDAWADI`](#khudawadi) | const |  |
| [`KIRAT_RAI`](#kirat_rai) | const |  |
| [`LAO`](#lao) | const |  |
| [`LATIN`](#latin) | const |  |
| [`LEPCHA`](#lepcha) | const |  |
| [`LIMBU`](#limbu) | const |  |
| [`LINEAR_A`](#linear_a) | const |  |
| [`LINEAR_B`](#linear_b) | const |  |
| [`LISU`](#lisu) | const |  |
| [`LYCIAN`](#lycian) | const |  |
| [`LYDIAN`](#lydian) | const |  |
| [`MAHAJANI`](#mahajani) | const |  |
| [`MAKASAR`](#makasar) | const |  |
| [`MALAYALAM`](#malayalam) | const |  |
| [`MANDAIC`](#mandaic) | const |  |
| [`MANICHAEAN`](#manichaean) | const |  |
| [`MARCHEN`](#marchen) | const |  |
| [`MASARAM_GONDI`](#masaram_gondi) | const |  |
| [`MEDEFAIDRIN`](#medefaidrin) | const |  |
| [`MEETEI_MAYEK`](#meetei_mayek) | const |  |
| [`MENDE_KIKAKUI`](#mende_kikakui) | const |  |
| [`MEROITIC_CURSIVE`](#meroitic_cursive) | const |  |
| [`MEROITIC_HIEROGLYPHS`](#meroitic_hieroglyphs) | const |  |
| [`MIAO`](#miao) | const |  |
| [`MODI`](#modi) | const |  |
| [`MONGOLIAN`](#mongolian) | const |  |
| [`MRO`](#mro) | const |  |
| [`MULTANI`](#multani) | const |  |
| [`MYANMAR`](#myanmar) | const |  |
| [`NABATAEAN`](#nabataean) | const |  |
| [`NAG_MUNDARI`](#nag_mundari) | const |  |
| [`NANDINAGARI`](#nandinagari) | const |  |
| [`NEW_TAI_LUE`](#new_tai_lue) | const |  |
| [`NEWA`](#newa) | const |  |
| [`NKO`](#nko) | const |  |
| [`NUSHU`](#nushu) | const |  |
| [`NYIAKENG_PUACHUE_HMONG`](#nyiakeng_puachue_hmong) | const |  |
| [`OGHAM`](#ogham) | const |  |
| [`OL_CHIKI`](#ol_chiki) | const |  |
| [`OL_ONAL`](#ol_onal) | const |  |
| [`OLD_HUNGARIAN`](#old_hungarian) | const |  |
| [`OLD_ITALIC`](#old_italic) | const |  |
| [`OLD_NORTH_ARABIAN`](#old_north_arabian) | const |  |
| [`OLD_PERMIC`](#old_permic) | const |  |
| [`OLD_PERSIAN`](#old_persian) | const |  |
| [`OLD_SOGDIAN`](#old_sogdian) | const |  |
| [`OLD_SOUTH_ARABIAN`](#old_south_arabian) | const |  |
| [`OLD_TURKIC`](#old_turkic) | const |  |
| [`OLD_UYGHUR`](#old_uyghur) | const |  |
| [`ORIYA`](#oriya) | const |  |
| [`OSAGE`](#osage) | const |  |
| [`OSMANYA`](#osmanya) | const |  |
| [`PAHAWH_HMONG`](#pahawh_hmong) | const |  |
| [`PALMYRENE`](#palmyrene) | const |  |
| [`PAU_CIN_HAU`](#pau_cin_hau) | const |  |
| [`PHAGS_PA`](#phags_pa) | const |  |
| [`PHOENICIAN`](#phoenician) | const |  |
| [`PSALTER_PAHLAVI`](#psalter_pahlavi) | const |  |
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
| [`SORA_SOMPENG`](#sora_sompeng) | const |  |
| [`SOYOMBO`](#soyombo) | const |  |
| [`SUNDANESE`](#sundanese) | const |  |
| [`SUNUWAR`](#sunuwar) | const |  |
| [`SYLOTI_NAGRI`](#syloti_nagri) | const |  |
| [`SYRIAC`](#syriac) | const |  |
| [`TAGALOG`](#tagalog) | const |  |
| [`TAGBANWA`](#tagbanwa) | const |  |
| [`TAI_LE`](#tai_le) | const |  |
| [`TAI_THAM`](#tai_tham) | const |  |
| [`TAI_VIET`](#tai_viet) | const |  |
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
| [`TULU_TIGALARI`](#tulu_tigalari) | const |  |
| [`UGARITIC`](#ugaritic) | const |  |
| [`VAI`](#vai) | const |  |
| [`VITHKUQI`](#vithkuqi) | const |  |
| [`WANCHO`](#wancho) | const |  |
| [`WARANG_CITI`](#warang_citi) | const |  |
| [`YEZIDI`](#yezidi) | const |  |
| [`YI`](#yi) | const |  |
| [`ZANABAZAR_SQUARE`](#zanabazar_square) | const |  |

## Constants

### `BY_NAME`

```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

### `ADLAM`

```rust
const ADLAM: &'static [(char, char)];
```

### `AHOM`

```rust
const AHOM: &'static [(char, char)];
```

### `ANATOLIAN_HIEROGLYPHS`

```rust
const ANATOLIAN_HIEROGLYPHS: &'static [(char, char)];
```

### `ARABIC`

```rust
const ARABIC: &'static [(char, char)];
```

### `ARMENIAN`

```rust
const ARMENIAN: &'static [(char, char)];
```

### `AVESTAN`

```rust
const AVESTAN: &'static [(char, char)];
```

### `BALINESE`

```rust
const BALINESE: &'static [(char, char)];
```

### `BAMUM`

```rust
const BAMUM: &'static [(char, char)];
```

### `BASSA_VAH`

```rust
const BASSA_VAH: &'static [(char, char)];
```

### `BATAK`

```rust
const BATAK: &'static [(char, char)];
```

### `BENGALI`

```rust
const BENGALI: &'static [(char, char)];
```

### `BHAIKSUKI`

```rust
const BHAIKSUKI: &'static [(char, char)];
```

### `BOPOMOFO`

```rust
const BOPOMOFO: &'static [(char, char)];
```

### `BRAHMI`

```rust
const BRAHMI: &'static [(char, char)];
```

### `BRAILLE`

```rust
const BRAILLE: &'static [(char, char)];
```

### `BUGINESE`

```rust
const BUGINESE: &'static [(char, char)];
```

### `BUHID`

```rust
const BUHID: &'static [(char, char)];
```

### `CANADIAN_ABORIGINAL`

```rust
const CANADIAN_ABORIGINAL: &'static [(char, char)];
```

### `CARIAN`

```rust
const CARIAN: &'static [(char, char)];
```

### `CAUCASIAN_ALBANIAN`

```rust
const CAUCASIAN_ALBANIAN: &'static [(char, char)];
```

### `CHAKMA`

```rust
const CHAKMA: &'static [(char, char)];
```

### `CHAM`

```rust
const CHAM: &'static [(char, char)];
```

### `CHEROKEE`

```rust
const CHEROKEE: &'static [(char, char)];
```

### `CHORASMIAN`

```rust
const CHORASMIAN: &'static [(char, char)];
```

### `COMMON`

```rust
const COMMON: &'static [(char, char)];
```

### `COPTIC`

```rust
const COPTIC: &'static [(char, char)];
```

### `CUNEIFORM`

```rust
const CUNEIFORM: &'static [(char, char)];
```

### `CYPRIOT`

```rust
const CYPRIOT: &'static [(char, char)];
```

### `CYPRO_MINOAN`

```rust
const CYPRO_MINOAN: &'static [(char, char)];
```

### `CYRILLIC`

```rust
const CYRILLIC: &'static [(char, char)];
```

### `DESERET`

```rust
const DESERET: &'static [(char, char)];
```

### `DEVANAGARI`

```rust
const DEVANAGARI: &'static [(char, char)];
```

### `DIVES_AKURU`

```rust
const DIVES_AKURU: &'static [(char, char)];
```

### `DOGRA`

```rust
const DOGRA: &'static [(char, char)];
```

### `DUPLOYAN`

```rust
const DUPLOYAN: &'static [(char, char)];
```

### `EGYPTIAN_HIEROGLYPHS`

```rust
const EGYPTIAN_HIEROGLYPHS: &'static [(char, char)];
```

### `ELBASAN`

```rust
const ELBASAN: &'static [(char, char)];
```

### `ELYMAIC`

```rust
const ELYMAIC: &'static [(char, char)];
```

### `ETHIOPIC`

```rust
const ETHIOPIC: &'static [(char, char)];
```

### `GARAY`

```rust
const GARAY: &'static [(char, char)];
```

### `GEORGIAN`

```rust
const GEORGIAN: &'static [(char, char)];
```

### `GLAGOLITIC`

```rust
const GLAGOLITIC: &'static [(char, char)];
```

### `GOTHIC`

```rust
const GOTHIC: &'static [(char, char)];
```

### `GRANTHA`

```rust
const GRANTHA: &'static [(char, char)];
```

### `GREEK`

```rust
const GREEK: &'static [(char, char)];
```

### `GUJARATI`

```rust
const GUJARATI: &'static [(char, char)];
```

### `GUNJALA_GONDI`

```rust
const GUNJALA_GONDI: &'static [(char, char)];
```

### `GURMUKHI`

```rust
const GURMUKHI: &'static [(char, char)];
```

### `GURUNG_KHEMA`

```rust
const GURUNG_KHEMA: &'static [(char, char)];
```

### `HAN`

```rust
const HAN: &'static [(char, char)];
```

### `HANGUL`

```rust
const HANGUL: &'static [(char, char)];
```

### `HANIFI_ROHINGYA`

```rust
const HANIFI_ROHINGYA: &'static [(char, char)];
```

### `HANUNOO`

```rust
const HANUNOO: &'static [(char, char)];
```

### `HATRAN`

```rust
const HATRAN: &'static [(char, char)];
```

### `HEBREW`

```rust
const HEBREW: &'static [(char, char)];
```

### `HIRAGANA`

```rust
const HIRAGANA: &'static [(char, char)];
```

### `IMPERIAL_ARAMAIC`

```rust
const IMPERIAL_ARAMAIC: &'static [(char, char)];
```

### `INHERITED`

```rust
const INHERITED: &'static [(char, char)];
```

### `INSCRIPTIONAL_PAHLAVI`

```rust
const INSCRIPTIONAL_PAHLAVI: &'static [(char, char)];
```

### `INSCRIPTIONAL_PARTHIAN`

```rust
const INSCRIPTIONAL_PARTHIAN: &'static [(char, char)];
```

### `JAVANESE`

```rust
const JAVANESE: &'static [(char, char)];
```

### `KAITHI`

```rust
const KAITHI: &'static [(char, char)];
```

### `KANNADA`

```rust
const KANNADA: &'static [(char, char)];
```

### `KATAKANA`

```rust
const KATAKANA: &'static [(char, char)];
```

### `KAWI`

```rust
const KAWI: &'static [(char, char)];
```

### `KAYAH_LI`

```rust
const KAYAH_LI: &'static [(char, char)];
```

### `KHAROSHTHI`

```rust
const KHAROSHTHI: &'static [(char, char)];
```

### `KHITAN_SMALL_SCRIPT`

```rust
const KHITAN_SMALL_SCRIPT: &'static [(char, char)];
```

### `KHMER`

```rust
const KHMER: &'static [(char, char)];
```

### `KHOJKI`

```rust
const KHOJKI: &'static [(char, char)];
```

### `KHUDAWADI`

```rust
const KHUDAWADI: &'static [(char, char)];
```

### `KIRAT_RAI`

```rust
const KIRAT_RAI: &'static [(char, char)];
```

### `LAO`

```rust
const LAO: &'static [(char, char)];
```

### `LATIN`

```rust
const LATIN: &'static [(char, char)];
```

### `LEPCHA`

```rust
const LEPCHA: &'static [(char, char)];
```

### `LIMBU`

```rust
const LIMBU: &'static [(char, char)];
```

### `LINEAR_A`

```rust
const LINEAR_A: &'static [(char, char)];
```

### `LINEAR_B`

```rust
const LINEAR_B: &'static [(char, char)];
```

### `LISU`

```rust
const LISU: &'static [(char, char)];
```

### `LYCIAN`

```rust
const LYCIAN: &'static [(char, char)];
```

### `LYDIAN`

```rust
const LYDIAN: &'static [(char, char)];
```

### `MAHAJANI`

```rust
const MAHAJANI: &'static [(char, char)];
```

### `MAKASAR`

```rust
const MAKASAR: &'static [(char, char)];
```

### `MALAYALAM`

```rust
const MALAYALAM: &'static [(char, char)];
```

### `MANDAIC`

```rust
const MANDAIC: &'static [(char, char)];
```

### `MANICHAEAN`

```rust
const MANICHAEAN: &'static [(char, char)];
```

### `MARCHEN`

```rust
const MARCHEN: &'static [(char, char)];
```

### `MASARAM_GONDI`

```rust
const MASARAM_GONDI: &'static [(char, char)];
```

### `MEDEFAIDRIN`

```rust
const MEDEFAIDRIN: &'static [(char, char)];
```

### `MEETEI_MAYEK`

```rust
const MEETEI_MAYEK: &'static [(char, char)];
```

### `MENDE_KIKAKUI`

```rust
const MENDE_KIKAKUI: &'static [(char, char)];
```

### `MEROITIC_CURSIVE`

```rust
const MEROITIC_CURSIVE: &'static [(char, char)];
```

### `MEROITIC_HIEROGLYPHS`

```rust
const MEROITIC_HIEROGLYPHS: &'static [(char, char)];
```

### `MIAO`

```rust
const MIAO: &'static [(char, char)];
```

### `MODI`

```rust
const MODI: &'static [(char, char)];
```

### `MONGOLIAN`

```rust
const MONGOLIAN: &'static [(char, char)];
```

### `MRO`

```rust
const MRO: &'static [(char, char)];
```

### `MULTANI`

```rust
const MULTANI: &'static [(char, char)];
```

### `MYANMAR`

```rust
const MYANMAR: &'static [(char, char)];
```

### `NABATAEAN`

```rust
const NABATAEAN: &'static [(char, char)];
```

### `NAG_MUNDARI`

```rust
const NAG_MUNDARI: &'static [(char, char)];
```

### `NANDINAGARI`

```rust
const NANDINAGARI: &'static [(char, char)];
```

### `NEW_TAI_LUE`

```rust
const NEW_TAI_LUE: &'static [(char, char)];
```

### `NEWA`

```rust
const NEWA: &'static [(char, char)];
```

### `NKO`

```rust
const NKO: &'static [(char, char)];
```

### `NUSHU`

```rust
const NUSHU: &'static [(char, char)];
```

### `NYIAKENG_PUACHUE_HMONG`

```rust
const NYIAKENG_PUACHUE_HMONG: &'static [(char, char)];
```

### `OGHAM`

```rust
const OGHAM: &'static [(char, char)];
```

### `OL_CHIKI`

```rust
const OL_CHIKI: &'static [(char, char)];
```

### `OL_ONAL`

```rust
const OL_ONAL: &'static [(char, char)];
```

### `OLD_HUNGARIAN`

```rust
const OLD_HUNGARIAN: &'static [(char, char)];
```

### `OLD_ITALIC`

```rust
const OLD_ITALIC: &'static [(char, char)];
```

### `OLD_NORTH_ARABIAN`

```rust
const OLD_NORTH_ARABIAN: &'static [(char, char)];
```

### `OLD_PERMIC`

```rust
const OLD_PERMIC: &'static [(char, char)];
```

### `OLD_PERSIAN`

```rust
const OLD_PERSIAN: &'static [(char, char)];
```

### `OLD_SOGDIAN`

```rust
const OLD_SOGDIAN: &'static [(char, char)];
```

### `OLD_SOUTH_ARABIAN`

```rust
const OLD_SOUTH_ARABIAN: &'static [(char, char)];
```

### `OLD_TURKIC`

```rust
const OLD_TURKIC: &'static [(char, char)];
```

### `OLD_UYGHUR`

```rust
const OLD_UYGHUR: &'static [(char, char)];
```

### `ORIYA`

```rust
const ORIYA: &'static [(char, char)];
```

### `OSAGE`

```rust
const OSAGE: &'static [(char, char)];
```

### `OSMANYA`

```rust
const OSMANYA: &'static [(char, char)];
```

### `PAHAWH_HMONG`

```rust
const PAHAWH_HMONG: &'static [(char, char)];
```

### `PALMYRENE`

```rust
const PALMYRENE: &'static [(char, char)];
```

### `PAU_CIN_HAU`

```rust
const PAU_CIN_HAU: &'static [(char, char)];
```

### `PHAGS_PA`

```rust
const PHAGS_PA: &'static [(char, char)];
```

### `PHOENICIAN`

```rust
const PHOENICIAN: &'static [(char, char)];
```

### `PSALTER_PAHLAVI`

```rust
const PSALTER_PAHLAVI: &'static [(char, char)];
```

### `REJANG`

```rust
const REJANG: &'static [(char, char)];
```

### `RUNIC`

```rust
const RUNIC: &'static [(char, char)];
```

### `SAMARITAN`

```rust
const SAMARITAN: &'static [(char, char)];
```

### `SAURASHTRA`

```rust
const SAURASHTRA: &'static [(char, char)];
```

### `SHARADA`

```rust
const SHARADA: &'static [(char, char)];
```

### `SHAVIAN`

```rust
const SHAVIAN: &'static [(char, char)];
```

### `SIDDHAM`

```rust
const SIDDHAM: &'static [(char, char)];
```

### `SIGNWRITING`

```rust
const SIGNWRITING: &'static [(char, char)];
```

### `SINHALA`

```rust
const SINHALA: &'static [(char, char)];
```

### `SOGDIAN`

```rust
const SOGDIAN: &'static [(char, char)];
```

### `SORA_SOMPENG`

```rust
const SORA_SOMPENG: &'static [(char, char)];
```

### `SOYOMBO`

```rust
const SOYOMBO: &'static [(char, char)];
```

### `SUNDANESE`

```rust
const SUNDANESE: &'static [(char, char)];
```

### `SUNUWAR`

```rust
const SUNUWAR: &'static [(char, char)];
```

### `SYLOTI_NAGRI`

```rust
const SYLOTI_NAGRI: &'static [(char, char)];
```

### `SYRIAC`

```rust
const SYRIAC: &'static [(char, char)];
```

### `TAGALOG`

```rust
const TAGALOG: &'static [(char, char)];
```

### `TAGBANWA`

```rust
const TAGBANWA: &'static [(char, char)];
```

### `TAI_LE`

```rust
const TAI_LE: &'static [(char, char)];
```

### `TAI_THAM`

```rust
const TAI_THAM: &'static [(char, char)];
```

### `TAI_VIET`

```rust
const TAI_VIET: &'static [(char, char)];
```

### `TAKRI`

```rust
const TAKRI: &'static [(char, char)];
```

### `TAMIL`

```rust
const TAMIL: &'static [(char, char)];
```

### `TANGSA`

```rust
const TANGSA: &'static [(char, char)];
```

### `TANGUT`

```rust
const TANGUT: &'static [(char, char)];
```

### `TELUGU`

```rust
const TELUGU: &'static [(char, char)];
```

### `THAANA`

```rust
const THAANA: &'static [(char, char)];
```

### `THAI`

```rust
const THAI: &'static [(char, char)];
```

### `TIBETAN`

```rust
const TIBETAN: &'static [(char, char)];
```

### `TIFINAGH`

```rust
const TIFINAGH: &'static [(char, char)];
```

### `TIRHUTA`

```rust
const TIRHUTA: &'static [(char, char)];
```

### `TODHRI`

```rust
const TODHRI: &'static [(char, char)];
```

### `TOTO`

```rust
const TOTO: &'static [(char, char)];
```

### `TULU_TIGALARI`

```rust
const TULU_TIGALARI: &'static [(char, char)];
```

### `UGARITIC`

```rust
const UGARITIC: &'static [(char, char)];
```

### `VAI`

```rust
const VAI: &'static [(char, char)];
```

### `VITHKUQI`

```rust
const VITHKUQI: &'static [(char, char)];
```

### `WANCHO`

```rust
const WANCHO: &'static [(char, char)];
```

### `WARANG_CITI`

```rust
const WARANG_CITI: &'static [(char, char)];
```

### `YEZIDI`

```rust
const YEZIDI: &'static [(char, char)];
```

### `YI`

```rust
const YI: &'static [(char, char)];
```

### `ZANABAZAR_SQUARE`

```rust
const ZANABAZAR_SQUARE: &'static [(char, char)];
```

