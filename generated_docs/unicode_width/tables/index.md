*[unicode_width](../index.md) / [tables](index.md)*

---

# Module `tables`

## Contents

- [Structs](#structs)
  - [`WidthInfo`](#widthinfo)
  - [`Align32`](#align32)
  - [`Align64`](#align64)
  - [`Align128`](#align128)
- [Functions](#functions)
  - [`lookup_width`](#lookup-width)
  - [`single_char_width`](#single-char-width)
  - [`width_in_str`](#width-in-str)
  - [`str_width`](#str-width)
  - [`lookup_width_cjk`](#lookup-width-cjk)
  - [`single_char_width_cjk`](#single-char-width-cjk)
  - [`width_in_str_cjk`](#width-in-str-cjk)
  - [`str_width_cjk`](#str-width-cjk)
  - [`is_transparent_zero_width`](#is-transparent-zero-width)
  - [`is_ligature_transparent`](#is-ligature-transparent)
  - [`is_solidus_transparent`](#is-solidus-transparent)
  - [`starts_emoji_presentation_seq`](#starts-emoji-presentation-seq)
  - [`starts_non_ideographic_text_presentation_seq`](#starts-non-ideographic-text-presentation-seq)
  - [`is_emoji_modifier_base`](#is-emoji-modifier-base)
- [Constants](#constants)
  - [`LIGATURE_TRANSPARENT_MASK`](#ligature-transparent-mask)
  - [`UNICODE_VERSION`](#unicode-version)
  - [`WIDTH_MIDDLE_LEN`](#width-middle-len)
  - [`WIDTH_LEAVES_LEN`](#width-leaves-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WidthInfo`](#widthinfo) | struct |  |
| [`Align32`](#align32) | struct |  |
| [`Align64`](#align64) | struct |  |
| [`Align128`](#align128) | struct |  |
| [`lookup_width`](#lookup-width) | fn | Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c` by consulting a multi-level lookup table. |
| [`single_char_width`](#single-char-width) | fn | Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`, or `None` if `c` is a control character. |
| [`width_in_str`](#width-in-str) | fn | Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`. |
| [`str_width`](#str-width) | fn |  |
| [`lookup_width_cjk`](#lookup-width-cjk) | fn | Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c` by consulting a multi-level lookup table. |
| [`single_char_width_cjk`](#single-char-width-cjk) | fn | Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`, or `None` if `c` is a control character. |
| [`width_in_str_cjk`](#width-in-str-cjk) | fn | Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`. |
| [`str_width_cjk`](#str-width-cjk) | fn |  |
| [`is_transparent_zero_width`](#is-transparent-zero-width) | fn | Whether this character is a zero-width character with `Joining_Type=Transparent`. |
| [`is_ligature_transparent`](#is-ligature-transparent) | fn | Whether this character is a default-ignorable combining mark or ZWJ. |
| [`is_solidus_transparent`](#is-solidus-transparent) | fn | Whether this character is transparent wrt the effect of U+0338 COMBINING LONG SOLIDUS OVERLAY on its base character. |
| [`starts_emoji_presentation_seq`](#starts-emoji-presentation-seq) | fn | Whether this character forms an [emoji presentation sequence] (https://www.unicode.org/reports/tr51/#def_emoji_presentation_sequence) when followed by `'\u{FEOF}'`. |
| [`starts_non_ideographic_text_presentation_seq`](#starts-non-ideographic-text-presentation-seq) | fn | Returns `true` if `c` has default emoji presentation, but forms a [text presentation sequence] (https://www.unicode.org/reports/tr51/#def_text_presentation_sequence) when followed by `'\u{FEOE}'`, and is not ideographic. |
| [`is_emoji_modifier_base`](#is-emoji-modifier-base) | fn | Returns `true` if `c` is an `Emoji_Modifier_Base`. |
| [`LIGATURE_TRANSPARENT_MASK`](#ligature-transparent-mask) | const |  |
| [`UNICODE_VERSION`](#unicode-version) | const | The version of [Unicode](http://www.unicode.org/) that this version of unicode-width is based on. |
| [`WIDTH_MIDDLE_LEN`](#width-middle-len) | const |  |
| [`WIDTH_LEAVES_LEN`](#width-leaves-len) | const |  |

## Structs

### `WidthInfo`

```rust
struct WidthInfo(u16);
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:16`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L16)*

#### Implementations

- <span id="widthinfo-const-default"></span>`const DEFAULT: Self`

- <span id="widthinfo-const-line-feed"></span>`const LINE_FEED: Self`

- <span id="widthinfo-const-emoji-modifier"></span>`const EMOJI_MODIFIER: Self`

- <span id="widthinfo-const-regional-indicator"></span>`const REGIONAL_INDICATOR: Self`

- <span id="widthinfo-const-several-regional-indicator"></span>`const SEVERAL_REGIONAL_INDICATOR: Self`

- <span id="widthinfo-const-emoji-presentation"></span>`const EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-zwj-emoji-presentation"></span>`const ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-vs16-zwj-emoji-presentation"></span>`const VS16_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-keycap-zwj-emoji-presentation"></span>`const KEYCAP_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-vs16-keycap-zwj-emoji-presentation"></span>`const VS16_KEYCAP_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-regional-indicator-zwj-presentation"></span>`const REGIONAL_INDICATOR_ZWJ_PRESENTATION: Self`

- <span id="widthinfo-const-even-regional-indicator-zwj-presentation"></span>`const EVEN_REGIONAL_INDICATOR_ZWJ_PRESENTATION: Self`

- <span id="widthinfo-const-odd-regional-indicator-zwj-presentation"></span>`const ODD_REGIONAL_INDICATOR_ZWJ_PRESENTATION: Self`

- <span id="widthinfo-const-tag-end-zwj-emoji-presentation"></span>`const TAG_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-d1-end-zwj-emoji-presentation"></span>`const TAG_D1_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-d2-end-zwj-emoji-presentation"></span>`const TAG_D2_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-d3-end-zwj-emoji-presentation"></span>`const TAG_D3_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-a1-end-zwj-emoji-presentation"></span>`const TAG_A1_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-a2-end-zwj-emoji-presentation"></span>`const TAG_A2_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-a3-end-zwj-emoji-presentation"></span>`const TAG_A3_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-a4-end-zwj-emoji-presentation"></span>`const TAG_A4_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-a5-end-zwj-emoji-presentation"></span>`const TAG_A5_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-tag-a6-end-zwj-emoji-presentation"></span>`const TAG_A6_END_ZWJ_EMOJI_PRESENTATION: Self`

- <span id="widthinfo-const-kirat-rai-vowel-sign-e"></span>`const KIRAT_RAI_VOWEL_SIGN_E: Self`

- <span id="widthinfo-const-kirat-rai-vowel-sign-ai"></span>`const KIRAT_RAI_VOWEL_SIGN_AI: Self`

- <span id="widthinfo-const-variation-selector-1-2-or-3"></span>`const VARIATION_SELECTOR_1_2_OR_3: Self`

- <span id="widthinfo-const-variation-selector-15"></span>`const VARIATION_SELECTOR_15: Self`

- <span id="widthinfo-const-variation-selector-16"></span>`const VARIATION_SELECTOR_16: Self`

- <span id="widthinfo-const-joining-group-alef"></span>`const JOINING_GROUP_ALEF: Self`

- <span id="widthinfo-const-combining-long-solidus-overlay"></span>`const COMBINING_LONG_SOLIDUS_OVERLAY: Self`

- <span id="widthinfo-const-solidus-overlay-alef"></span>`const SOLIDUS_OVERLAY_ALEF: Self`

- <span id="widthinfo-const-hebrew-letter-lamed"></span>`const HEBREW_LETTER_LAMED: Self`

- <span id="widthinfo-const-zwj-hebrew-letter-lamed"></span>`const ZWJ_HEBREW_LETTER_LAMED: Self`

- <span id="widthinfo-const-buginese-letter-ya"></span>`const BUGINESE_LETTER_YA: Self`

- <span id="widthinfo-const-zwj-buginese-letter-ya"></span>`const ZWJ_BUGINESE_LETTER_YA: Self`

- <span id="widthinfo-const-buginese-vowel-sign-i-zwj-letter-ya"></span>`const BUGINESE_VOWEL_SIGN_I_ZWJ_LETTER_YA: Self`

- <span id="widthinfo-const-tifinagh-consonant"></span>`const TIFINAGH_CONSONANT: Self`

- <span id="widthinfo-const-zwj-tifinagh-consonant"></span>`const ZWJ_TIFINAGH_CONSONANT: Self`

- <span id="widthinfo-const-tifinagh-joiner-consonant"></span>`const TIFINAGH_JOINER_CONSONANT: Self`

- <span id="widthinfo-const-lisu-tone-letter-mya-na-jeu"></span>`const LISU_TONE_LETTER_MYA_NA_JEU: Self`

- <span id="widthinfo-const-old-turkic-letter-orkhon-i"></span>`const OLD_TURKIC_LETTER_ORKHON_I: Self`

- <span id="widthinfo-const-zwj-old-turkic-letter-orkhon-i"></span>`const ZWJ_OLD_TURKIC_LETTER_ORKHON_I: Self`

- <span id="widthinfo-const-khmer-coeng-eligible-letter"></span>`const KHMER_COENG_ELIGIBLE_LETTER: Self`

- <span id="widthinfo-is-ligature-transparent"></span>`fn is_ligature_transparent(self) -> bool`

- <span id="widthinfo-set-zwj-bit"></span>`fn set_zwj_bit(self) -> Self`

- <span id="widthinfo-is-emoji-presentation"></span>`fn is_emoji_presentation(self) -> bool`

- <span id="widthinfo-is-zwj-emoji-presentation"></span>`fn is_zwj_emoji_presentation(self) -> bool`

- <span id="widthinfo-set-emoji-presentation"></span>`fn set_emoji_presentation(self) -> Self`

- <span id="widthinfo-unset-emoji-presentation"></span>`fn unset_emoji_presentation(self) -> Self`

- <span id="widthinfo-is-text-presentation"></span>`fn is_text_presentation(self) -> bool`

- <span id="widthinfo-set-text-presentation"></span>`fn set_text_presentation(self) -> Self`

- <span id="widthinfo-unset-text-presentation"></span>`fn unset_text_presentation(self) -> Self`

- <span id="widthinfo-is-vs1-2-3"></span>`fn is_vs1_2_3(self) -> bool`

- <span id="widthinfo-set-vs1-2-3"></span>`fn set_vs1_2_3(self) -> Self`

- <span id="widthinfo-unset-vs1-2-3"></span>`fn unset_vs1_2_3(self) -> Self`

#### Trait Implementations

##### `impl Clone for WidthInfo`

- <span id="widthinfo-clone"></span>`fn clone(&self) -> WidthInfo` — [`WidthInfo`](#widthinfo)

##### `impl Copy for WidthInfo`

##### `impl Debug for WidthInfo`

- <span id="widthinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WidthInfo`

##### `impl PartialEq for WidthInfo`

- <span id="widthinfo-eq"></span>`fn eq(&self, other: &WidthInfo) -> bool` — [`WidthInfo`](#widthinfo)

##### `impl Sealed for WidthInfo`

##### `impl StructuralPartialEq for WidthInfo`

### `Align32<T>`

```rust
struct Align32<T>(T);
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:947`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L947)*

#### Trait Implementations

##### `impl<T> Sealed for Align32<T>`

### `Align64<T>`

```rust
struct Align64<T>(T);
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:950`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L950)*

#### Trait Implementations

##### `impl<T> Sealed for Align64<T>`

### `Align128<T>`

```rust
struct Align128<T>(T);
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:953`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L953)*

#### Trait Implementations

##### `impl<T> Sealed for Align128<T>`

## Functions

### `lookup_width`

```rust
fn lookup_width(c: char) -> (u8, WidthInfo)
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:176-218`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L176-L218)*

Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c` by
consulting a multi-level lookup table.

# Maintenance
The tables themselves are autogenerated but this function is hardcoded. You should have
nothing to worry about if you re-run `unicode.py` (for example, when updating Unicode.)
However, if you change the *actual structure* of the lookup tables (perhaps by editing the
`make_tables` function in `unicode.py`) you must ensure that this code reflects those changes.

### `single_char_width`

```rust
fn single_char_width(c: char) -> Option<usize>
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:224-240`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L224-L240)*

Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`, or
`None` if `c` is a control character.
Ambiguous width characters are treated as narrow.

### `width_in_str`

```rust
fn width_in_str(c: char, next_info: WidthInfo) -> (i8, WidthInfo)
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:245-462`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L245-L462)*

Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`.
Ambiguous width characters are treated as narrow.

### `str_width`

```rust
fn str_width(s: &str) -> usize
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:465-475`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L465-L475)*

### `lookup_width_cjk`

```rust
fn lookup_width_cjk(c: char) -> (u8, WidthInfo)
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:487-529`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L487-L529)*

Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c` by
consulting a multi-level lookup table.

# Maintenance
The tables themselves are autogenerated but this function is hardcoded. You should have
nothing to worry about if you re-run `unicode.py` (for example, when updating Unicode.)
However, if you change the *actual structure* of the lookup tables (perhaps by editing the
`make_tables` function in `unicode.py`) you must ensure that this code reflects those changes.

### `single_char_width_cjk`

```rust
fn single_char_width_cjk(c: char) -> Option<usize>
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:536-552`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L536-L552)*

Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`, or
`None` if `c` is a control character.
Ambiguous width characters are treated as wide.

### `width_in_str_cjk`

```rust
fn width_in_str_cjk(c: char, next_info: WidthInfo) -> (i8, WidthInfo)
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:558-782`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L558-L782)*

Returns the [UAX #11](https://www.unicode.org/reports/tr11/) based width of `c`.
Ambiguous width characters are treated as wide.

### `str_width_cjk`

```rust
fn str_width_cjk(s: &str) -> usize
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:786-796`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L786-L796)*

### `is_transparent_zero_width`

```rust
fn is_transparent_zero_width(c: char) -> bool
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:802-822`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L802-L822)*

Whether this character is a zero-width character with
`Joining_Type=Transparent`. Used by the Alef-Lamed ligatures.
See also [`is_ligature_transparent`](#is-ligature-transparent), a near-subset of this (only ZWJ is excepted)
which is transparent for non-Arabic ligatures.

### `is_ligature_transparent`

```rust
fn is_ligature_transparent(c: char) -> bool
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:826-828`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L826-L828)*

Whether this character is a default-ignorable combining mark
or ZWJ. These characters won't interrupt non-Arabic ligatures.

### `is_solidus_transparent`

```rust
fn is_solidus_transparent(c: char) -> bool
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:834-850`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L834-L850)*

Whether this character is transparent wrt the effect of
U+0338 COMBINING LONG SOLIDUS OVERLAY
on its base character.

### `starts_emoji_presentation_seq`

```rust
fn starts_emoji_presentation_seq(c: char) -> bool
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:857-877`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L857-L877)*

Whether this character forms an [emoji presentation sequence]
(https://www.unicode.org/reports/tr51/#def_emoji_presentation_sequence)
when followed by `'\u{FEOF}'`.
Emoji presentation sequences are considered to have width 2.

### `starts_non_ideographic_text_presentation_seq`

```rust
fn starts_non_ideographic_text_presentation_seq(c: char) -> bool
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:884-913`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L884-L913)*

Returns `true` if `c` has default emoji presentation, but forms a [text presentation sequence]
(https://www.unicode.org/reports/tr51/#def_text_presentation_sequence)
when followed by `'\u{FEOE}'`, and is not ideographic.
Such sequences are considered to have width 1.

### `is_emoji_modifier_base`

```rust
fn is_emoji_modifier_base(c: char) -> bool
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:917-944`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L917-L944)*

Returns `true` if `c` is an `Emoji_Modifier_Base`.

## Constants

### `LIGATURE_TRANSPARENT_MASK`
```rust
const LIGATURE_TRANSPARENT_MASK: u16 = 8_192u16;
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:18`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L18)*

### `UNICODE_VERSION`
```rust
const UNICODE_VERSION: (u8, u8, u8);
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:165`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L165)*

The version of [Unicode](http://www.unicode.org/)
that this version of unicode-width is based on.

### `WIDTH_MIDDLE_LEN`
```rust
const WIDTH_MIDDLE_LEN: usize = 20usize;
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:995`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L995)*

### `WIDTH_LEAVES_LEN`
```rust
const WIDTH_LEAVES_LEN: usize = 186usize;
```

*Defined in [`unicode-width-0.2.2/src/tables.rs:1147`](../../../.source_1765521767/unicode-width-0.2.2/src/tables.rs#L1147)*

