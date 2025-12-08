# Crate `unicode_width`

Determine displayed width of `char` and `str` types according to
[Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
and other portions of the Unicode standard.
See the [Rules for determining width](#rules-for-determining-width) section
for the exact rules.

This crate is `#![no_std]`.

```rust
use unicode_width::UnicodeWidthStr;

let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
let width = UnicodeWidthStr::width(teststr);
println!("{}", teststr);
println!("The above string is {} columns wide.", width);
```

# `"cjk"` feature flag

This crate has one Cargo feature flag, `"cjk"`
(enabled by default).
It enables the `UnicodeWidthChar::width_cjk`
and `UnicodeWidthStr::width_cjk`,
which perform an alternate width calculation
more suited to CJK contexts. The flag also unseals the
[`UnicodeWidthChar`](#unicodewidthchar) and [`UnicodeWidthStr`](#unicodewidthstr) traits.

Disabling the flag (with `no_default_features` in `Cargo.toml`)
will reduce the amount of static data needed by the crate.

```rust
use unicode_width::UnicodeWidthStr;

let teststr = "“𘀀”";
assert_eq!(teststr.width(), 4);

#[cfg(feature = "cjk")]
assert_eq!(teststr.width_cjk(), 6);
```

# Rules for determining width

This crate currently uses the following rules to determine the width of a
character or string, in order of decreasing precedence. These may be tweaked in the future.

1. In the following cases, the width of a string differs from the sum of the widths of its constituent characters:
   - The sequence `"\r\n"` has width 1.
   - Emoji-specific ligatures:
     - Well-formed, fully-qualified [emoji ZWJ sequences] have width 2.
     - [Emoji modifier sequences] have width 2.
     - [Emoji presentation sequences] have width 2.
     - Outside of an East Asian context, [text presentation sequences] have width 1 if their base character:
       - Has the `Emoji_Presentation` property, and
       - Is not in the [Enclosed Ideographic Supplement] block.
   - [`'\u{2018}'`, `'\u{2019}'`, `'\u{201C}'`, and `'\u{201D}'`][General Punctuation] always have width 1
     when followed by '\u{FE00}' or '\u{FE02}', and width 2 when followed by '\u{FE01}'.
   - Script-specific ligatures:
     - For all the following ligatures, the insertion of any number of [default-ignorable]`Default_Ignorable_Code_Point`
       [combining marks] anywhere in the sequence will not change the total width. In addition, for all non-Arabic
       ligatures, the insertion of any number of [`'\u{200D}'` ZERO WIDTH JOINER](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-23/#G23126)s
       will not affect the width.
     - **[Arabic]**: A character sequence consisting of one character with `Joining_Group``=Lam`,
       followed by any number of characters with `Joining_Type``=Transparent`, followed by one character
       with `Joining_Group``=Alef`, has total width 1. For example: `لا`‎, `لآ`‎, `ڸا`‎, `لٟٞأ`
     - **[Buginese]**: `"\u{1A15}\u{1A17}\u{200D}\u{1A10}"` (<a, -i> ya, `ᨕᨗ‍ᨐ`) has total width 1.
     - **[Hebrew]**: `"א\u{200D}ל"` (Alef-Lamed, `א‍ל`) has total width 1.
     - **[Khmer]**: Coeng signs consisting of `'\u{17D2}'` followed by a character in
       `'\u{1780}'..='\u{1782}' | '\u{1784}'..='\u{1787}' | '\u{1789}'..='\u{178C}' | '\u{178E}'..='\u{1793}' | '\u{1795}'..='\u{1798}' | '\u{179B}'..='\u{179D}' | '\u{17A0}' | '\u{17A2}'  | '\u{17A7}' | '\u{17AB}'..='\u{17AC}' | '\u{17AF}'`
       have width 0.
     - **[Kirat Rai]**: Any sequence canonically equivalent to `'\u{16D68}'`, `'\u{16D69}'`, or `'\u{16D6A}'` has total width 1.
     - **[Lisu]**: Tone letter combinations consisting of a character in the range `'\u{A4F8}'..='\u{A4FB}'`
       followed by a character in the range `'\u{A4FC}'..='\u{A4FD}'` have width 1. For example: `ꓹꓼ`
     - **[Old Turkic]**: `"\u{10C32}\u{200D}\u{10C03}"` (`𐰲‍𐰃`) has total width 1.
     - **[Tifinagh]**: A sequence of a Tifinagh consonant in the range `'\u{2D31}'..='\u{2D65}' | '\u{2D6F}'`, followed by either
       [`'\u{2D7F}'` TIFINAGH CONSONANT JOINER] or `'\u{200D}'`, followed by another Tifinangh consonant, has total width 1.
       For example: `ⵏ⵿ⴾ`
   - In an East Asian context only, `<`, `=`, or `>` have width 2 when followed by [`'\u{0338}'` COMBINING LONG SOLIDUS OVERLAY].
     The two characters may be separated by any number of characters whose canonical decompositions consist only of characters meeting
     one of the following requirements:
     - Has `Canonical_Combining_Class` greater than 1, or
     - Is a [default-ignorable]`Default_Ignorable_Code_Point` [combining mark][combining marks].
2. In all other cases, the width of the string equals the sum of its character widths:
   1. [`'\u{2D7F}'` TIFINAGH CONSONANT JOINER] has width 1 (outside of the ligatures described previously).
   2. [`'\u{115F}'` HANGUL CHOSEONG FILLER](https://util.unicode.org/UnicodeJsps/character.jsp?a=115F) and
      [`'\u{17A4}'` KHMER INDEPENDENT VOWEL QAA](https://util.unicode.org/UnicodeJsps/character.jsp?a=17A4) have width 2.
   3. [`'\u{17D8}'` KHMER SIGN BEYYAL](https://util.unicode.org/UnicodeJsps/character.jsp?a=17D8) has width 3.
   4. The following have width 0:
      - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BDefault_Ignorable_Code_Point%7D)
        with the `Default_Ignorable_Code_Point` property.
      - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BGrapheme_Extend%7D)
        with the `Grapheme_Extend` property.
      - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BHangul_Syllable_Type%3DV%7D%5Cp%7BHangul_Syllable_Type%3DT%7D)
        with a `Hangul_Syllable_Type` of `Vowel_Jamo` (`V`) or `Trailing_Jamo` (`T`).
      - The following `Prepended_Concatenation_Mark`s:
        - [`'\u{0605}'` NUMBER MARK ABOVE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0605),
        - [`'\u{070F}'` SYRIAC ABBREVIATION MARK](https://util.unicode.org/UnicodeJsps/character.jsp?a=070F),
        - [`'\u{0890}'` POUND MARK ABOVE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0890),
        - [`'\u{0891}'` PIASTRE MARK ABOVE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0891), and
        - [`'\u{08E2}'` DISPUTED END OF AYAH](https://util.unicode.org/UnicodeJsps/character.jsp?a=08E2).
      - [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BGrapheme_Cluster_Break%3DPrepend%7D-%5Cp%7BPrepended_Concatenation_Mark%7D)
        with the `Grapheme_Extend=Prepend` property, that are not also `Prepended_Concatenation_Mark`s.
      - [`'\u{A8FA}'` DEVANAGARI CARET](https://util.unicode.org/UnicodeJsps/character.jsp?a=A8FA).
   5. [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BEast_Asian_Width%3DF%7D%5Cp%7BEast_Asian_Width%3DW%7D)
      with an `East_Asian_Width` of `Fullwidth` or [`Wide`](#wide) have width 2.
   6. Characters fulfilling all of the following conditions have width 2 in an East Asian context, and width 1 otherwise:
      - Fulfills one of the following conditions:
        - Has an `East_Asian_Width` of `Ambiguous`, or
        - Has a `Line_Break` of `AI`, or
        - Has a canonical decomposition to an `Ambiguous` character followed by [`'\u{0338}'` COMBINING LONG SOLIDUS OVERLAY], or
        - Is [`'\u{0387}'` GREEK ANO TELEIA](https://util.unicode.org/UnicodeJsps/character.jsp?a=0387); and
      - Does not have a `General_Category` of `Letter` or `Modifier_Symbol`.
   7. All other characters have width 1.



























## Canonical equivalence

Canonically equivalent strings are assigned the same width (CJK and non-CJK).

## Traits

### `UnicodeWidthChar`

```rust
trait UnicodeWidthChar: private::Sealed { ... }
```

Methods for determining displayed width of Unicode characters.

#### Required Methods

- `fn width(self: Self) -> Option<usize>`

  Returns the character's displayed width in columns, or `None` if the

- `fn width_cjk(self: Self) -> Option<usize>`

  Returns the character's displayed width in columns, or `None` if the

### `UnicodeWidthStr`

```rust
trait UnicodeWidthStr: private::Sealed { ... }
```

Methods for determining displayed width of Unicode strings.

#### Required Methods

- `fn width(self: &Self) -> usize`

  Returns the string's displayed width in columns.

- `fn width_cjk(self: &Self) -> usize`

  Returns the string's displayed width in columns.

## Constants

