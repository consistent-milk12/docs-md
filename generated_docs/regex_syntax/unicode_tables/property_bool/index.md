*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [property_bool](index.md)*

---

# Module `property_bool`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by_name)
  - [`ASCII_HEX_DIGIT`](#ascii_hex_digit)
  - [`ALPHABETIC`](#alphabetic)
  - [`BIDI_CONTROL`](#bidi_control)
  - [`BIDI_MIRRORED`](#bidi_mirrored)
  - [`CASE_IGNORABLE`](#case_ignorable)
  - [`CASED`](#cased)
  - [`CHANGES_WHEN_CASEFOLDED`](#changes_when_casefolded)
  - [`CHANGES_WHEN_CASEMAPPED`](#changes_when_casemapped)
  - [`CHANGES_WHEN_LOWERCASED`](#changes_when_lowercased)
  - [`CHANGES_WHEN_TITLECASED`](#changes_when_titlecased)
  - [`CHANGES_WHEN_UPPERCASED`](#changes_when_uppercased)
  - [`DASH`](#dash)
  - [`DEFAULT_IGNORABLE_CODE_POINT`](#default_ignorable_code_point)
  - [`DEPRECATED`](#deprecated)
  - [`DIACRITIC`](#diacritic)
  - [`EMOJI`](#emoji)
  - [`EMOJI_COMPONENT`](#emoji_component)
  - [`EMOJI_MODIFIER`](#emoji_modifier)
  - [`EMOJI_MODIFIER_BASE`](#emoji_modifier_base)
  - [`EMOJI_PRESENTATION`](#emoji_presentation)
  - [`EXTENDED_PICTOGRAPHIC`](#extended_pictographic)
  - [`EXTENDER`](#extender)
  - [`GRAPHEME_BASE`](#grapheme_base)
  - [`GRAPHEME_EXTEND`](#grapheme_extend)
  - [`GRAPHEME_LINK`](#grapheme_link)
  - [`HEX_DIGIT`](#hex_digit)
  - [`HYPHEN`](#hyphen)
  - [`IDS_BINARY_OPERATOR`](#ids_binary_operator)
  - [`IDS_TRINARY_OPERATOR`](#ids_trinary_operator)
  - [`IDS_UNARY_OPERATOR`](#ids_unary_operator)
  - [`ID_COMPAT_MATH_CONTINUE`](#id_compat_math_continue)
  - [`ID_COMPAT_MATH_START`](#id_compat_math_start)
  - [`ID_CONTINUE`](#id_continue)
  - [`ID_START`](#id_start)
  - [`IDEOGRAPHIC`](#ideographic)
  - [`INCB`](#incb)
  - [`JOIN_CONTROL`](#join_control)
  - [`LOGICAL_ORDER_EXCEPTION`](#logical_order_exception)
  - [`LOWERCASE`](#lowercase)
  - [`MATH`](#math)
  - [`MODIFIER_COMBINING_MARK`](#modifier_combining_mark)
  - [`NONCHARACTER_CODE_POINT`](#noncharacter_code_point)
  - [`OTHER_ALPHABETIC`](#other_alphabetic)
  - [`OTHER_DEFAULT_IGNORABLE_CODE_POINT`](#other_default_ignorable_code_point)
  - [`OTHER_GRAPHEME_EXTEND`](#other_grapheme_extend)
  - [`OTHER_ID_CONTINUE`](#other_id_continue)
  - [`OTHER_ID_START`](#other_id_start)
  - [`OTHER_LOWERCASE`](#other_lowercase)
  - [`OTHER_MATH`](#other_math)
  - [`OTHER_UPPERCASE`](#other_uppercase)
  - [`PATTERN_SYNTAX`](#pattern_syntax)
  - [`PATTERN_WHITE_SPACE`](#pattern_white_space)
  - [`PREPENDED_CONCATENATION_MARK`](#prepended_concatenation_mark)
  - [`QUOTATION_MARK`](#quotation_mark)
  - [`RADICAL`](#radical)
  - [`REGIONAL_INDICATOR`](#regional_indicator)
  - [`SENTENCE_TERMINAL`](#sentence_terminal)
  - [`SOFT_DOTTED`](#soft_dotted)
  - [`TERMINAL_PUNCTUATION`](#terminal_punctuation)
  - [`UNIFIED_IDEOGRAPH`](#unified_ideograph)
  - [`UPPERCASE`](#uppercase)
  - [`VARIATION_SELECTOR`](#variation_selector)
  - [`WHITE_SPACE`](#white_space)
  - [`XID_CONTINUE`](#xid_continue)
  - [`XID_START`](#xid_start)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by_name) | const |  |
| [`ASCII_HEX_DIGIT`](#ascii_hex_digit) | const |  |
| [`ALPHABETIC`](#alphabetic) | const |  |
| [`BIDI_CONTROL`](#bidi_control) | const |  |
| [`BIDI_MIRRORED`](#bidi_mirrored) | const |  |
| [`CASE_IGNORABLE`](#case_ignorable) | const |  |
| [`CASED`](#cased) | const |  |
| [`CHANGES_WHEN_CASEFOLDED`](#changes_when_casefolded) | const |  |
| [`CHANGES_WHEN_CASEMAPPED`](#changes_when_casemapped) | const |  |
| [`CHANGES_WHEN_LOWERCASED`](#changes_when_lowercased) | const |  |
| [`CHANGES_WHEN_TITLECASED`](#changes_when_titlecased) | const |  |
| [`CHANGES_WHEN_UPPERCASED`](#changes_when_uppercased) | const |  |
| [`DASH`](#dash) | const |  |
| [`DEFAULT_IGNORABLE_CODE_POINT`](#default_ignorable_code_point) | const |  |
| [`DEPRECATED`](#deprecated) | const |  |
| [`DIACRITIC`](#diacritic) | const |  |
| [`EMOJI`](#emoji) | const |  |
| [`EMOJI_COMPONENT`](#emoji_component) | const |  |
| [`EMOJI_MODIFIER`](#emoji_modifier) | const |  |
| [`EMOJI_MODIFIER_BASE`](#emoji_modifier_base) | const |  |
| [`EMOJI_PRESENTATION`](#emoji_presentation) | const |  |
| [`EXTENDED_PICTOGRAPHIC`](#extended_pictographic) | const |  |
| [`EXTENDER`](#extender) | const |  |
| [`GRAPHEME_BASE`](#grapheme_base) | const |  |
| [`GRAPHEME_EXTEND`](#grapheme_extend) | const |  |
| [`GRAPHEME_LINK`](#grapheme_link) | const |  |
| [`HEX_DIGIT`](#hex_digit) | const |  |
| [`HYPHEN`](#hyphen) | const |  |
| [`IDS_BINARY_OPERATOR`](#ids_binary_operator) | const |  |
| [`IDS_TRINARY_OPERATOR`](#ids_trinary_operator) | const |  |
| [`IDS_UNARY_OPERATOR`](#ids_unary_operator) | const |  |
| [`ID_COMPAT_MATH_CONTINUE`](#id_compat_math_continue) | const |  |
| [`ID_COMPAT_MATH_START`](#id_compat_math_start) | const |  |
| [`ID_CONTINUE`](#id_continue) | const |  |
| [`ID_START`](#id_start) | const |  |
| [`IDEOGRAPHIC`](#ideographic) | const |  |
| [`INCB`](#incb) | const |  |
| [`JOIN_CONTROL`](#join_control) | const |  |
| [`LOGICAL_ORDER_EXCEPTION`](#logical_order_exception) | const |  |
| [`LOWERCASE`](#lowercase) | const |  |
| [`MATH`](#math) | const |  |
| [`MODIFIER_COMBINING_MARK`](#modifier_combining_mark) | const |  |
| [`NONCHARACTER_CODE_POINT`](#noncharacter_code_point) | const |  |
| [`OTHER_ALPHABETIC`](#other_alphabetic) | const |  |
| [`OTHER_DEFAULT_IGNORABLE_CODE_POINT`](#other_default_ignorable_code_point) | const |  |
| [`OTHER_GRAPHEME_EXTEND`](#other_grapheme_extend) | const |  |
| [`OTHER_ID_CONTINUE`](#other_id_continue) | const |  |
| [`OTHER_ID_START`](#other_id_start) | const |  |
| [`OTHER_LOWERCASE`](#other_lowercase) | const |  |
| [`OTHER_MATH`](#other_math) | const |  |
| [`OTHER_UPPERCASE`](#other_uppercase) | const |  |
| [`PATTERN_SYNTAX`](#pattern_syntax) | const |  |
| [`PATTERN_WHITE_SPACE`](#pattern_white_space) | const |  |
| [`PREPENDED_CONCATENATION_MARK`](#prepended_concatenation_mark) | const |  |
| [`QUOTATION_MARK`](#quotation_mark) | const |  |
| [`RADICAL`](#radical) | const |  |
| [`REGIONAL_INDICATOR`](#regional_indicator) | const |  |
| [`SENTENCE_TERMINAL`](#sentence_terminal) | const |  |
| [`SOFT_DOTTED`](#soft_dotted) | const |  |
| [`TERMINAL_PUNCTUATION`](#terminal_punctuation) | const |  |
| [`UNIFIED_IDEOGRAPH`](#unified_ideograph) | const |  |
| [`UPPERCASE`](#uppercase) | const |  |
| [`VARIATION_SELECTOR`](#variation_selector) | const |  |
| [`WHITE_SPACE`](#white_space) | const |  |
| [`XID_CONTINUE`](#xid_continue) | const |  |
| [`XID_START`](#xid_start) | const |  |

## Constants

### `BY_NAME`

```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

### `ASCII_HEX_DIGIT`

```rust
const ASCII_HEX_DIGIT: &'static [(char, char)];
```

### `ALPHABETIC`

```rust
const ALPHABETIC: &'static [(char, char)];
```

### `BIDI_CONTROL`

```rust
const BIDI_CONTROL: &'static [(char, char)];
```

### `BIDI_MIRRORED`

```rust
const BIDI_MIRRORED: &'static [(char, char)];
```

### `CASE_IGNORABLE`

```rust
const CASE_IGNORABLE: &'static [(char, char)];
```

### `CASED`

```rust
const CASED: &'static [(char, char)];
```

### `CHANGES_WHEN_CASEFOLDED`

```rust
const CHANGES_WHEN_CASEFOLDED: &'static [(char, char)];
```

### `CHANGES_WHEN_CASEMAPPED`

```rust
const CHANGES_WHEN_CASEMAPPED: &'static [(char, char)];
```

### `CHANGES_WHEN_LOWERCASED`

```rust
const CHANGES_WHEN_LOWERCASED: &'static [(char, char)];
```

### `CHANGES_WHEN_TITLECASED`

```rust
const CHANGES_WHEN_TITLECASED: &'static [(char, char)];
```

### `CHANGES_WHEN_UPPERCASED`

```rust
const CHANGES_WHEN_UPPERCASED: &'static [(char, char)];
```

### `DASH`

```rust
const DASH: &'static [(char, char)];
```

### `DEFAULT_IGNORABLE_CODE_POINT`

```rust
const DEFAULT_IGNORABLE_CODE_POINT: &'static [(char, char)];
```

### `DEPRECATED`

```rust
const DEPRECATED: &'static [(char, char)];
```

### `DIACRITIC`

```rust
const DIACRITIC: &'static [(char, char)];
```

### `EMOJI`

```rust
const EMOJI: &'static [(char, char)];
```

### `EMOJI_COMPONENT`

```rust
const EMOJI_COMPONENT: &'static [(char, char)];
```

### `EMOJI_MODIFIER`

```rust
const EMOJI_MODIFIER: &'static [(char, char)];
```

### `EMOJI_MODIFIER_BASE`

```rust
const EMOJI_MODIFIER_BASE: &'static [(char, char)];
```

### `EMOJI_PRESENTATION`

```rust
const EMOJI_PRESENTATION: &'static [(char, char)];
```

### `EXTENDED_PICTOGRAPHIC`

```rust
const EXTENDED_PICTOGRAPHIC: &'static [(char, char)];
```

### `EXTENDER`

```rust
const EXTENDER: &'static [(char, char)];
```

### `GRAPHEME_BASE`

```rust
const GRAPHEME_BASE: &'static [(char, char)];
```

### `GRAPHEME_EXTEND`

```rust
const GRAPHEME_EXTEND: &'static [(char, char)];
```

### `GRAPHEME_LINK`

```rust
const GRAPHEME_LINK: &'static [(char, char)];
```

### `HEX_DIGIT`

```rust
const HEX_DIGIT: &'static [(char, char)];
```

### `HYPHEN`

```rust
const HYPHEN: &'static [(char, char)];
```

### `IDS_BINARY_OPERATOR`

```rust
const IDS_BINARY_OPERATOR: &'static [(char, char)];
```

### `IDS_TRINARY_OPERATOR`

```rust
const IDS_TRINARY_OPERATOR: &'static [(char, char)];
```

### `IDS_UNARY_OPERATOR`

```rust
const IDS_UNARY_OPERATOR: &'static [(char, char)];
```

### `ID_COMPAT_MATH_CONTINUE`

```rust
const ID_COMPAT_MATH_CONTINUE: &'static [(char, char)];
```

### `ID_COMPAT_MATH_START`

```rust
const ID_COMPAT_MATH_START: &'static [(char, char)];
```

### `ID_CONTINUE`

```rust
const ID_CONTINUE: &'static [(char, char)];
```

### `ID_START`

```rust
const ID_START: &'static [(char, char)];
```

### `IDEOGRAPHIC`

```rust
const IDEOGRAPHIC: &'static [(char, char)];
```

### `INCB`

```rust
const INCB: &'static [(char, char)];
```

### `JOIN_CONTROL`

```rust
const JOIN_CONTROL: &'static [(char, char)];
```

### `LOGICAL_ORDER_EXCEPTION`

```rust
const LOGICAL_ORDER_EXCEPTION: &'static [(char, char)];
```

### `LOWERCASE`

```rust
const LOWERCASE: &'static [(char, char)];
```

### `MATH`

```rust
const MATH: &'static [(char, char)];
```

### `MODIFIER_COMBINING_MARK`

```rust
const MODIFIER_COMBINING_MARK: &'static [(char, char)];
```

### `NONCHARACTER_CODE_POINT`

```rust
const NONCHARACTER_CODE_POINT: &'static [(char, char)];
```

### `OTHER_ALPHABETIC`

```rust
const OTHER_ALPHABETIC: &'static [(char, char)];
```

### `OTHER_DEFAULT_IGNORABLE_CODE_POINT`

```rust
const OTHER_DEFAULT_IGNORABLE_CODE_POINT: &'static [(char, char)];
```

### `OTHER_GRAPHEME_EXTEND`

```rust
const OTHER_GRAPHEME_EXTEND: &'static [(char, char)];
```

### `OTHER_ID_CONTINUE`

```rust
const OTHER_ID_CONTINUE: &'static [(char, char)];
```

### `OTHER_ID_START`

```rust
const OTHER_ID_START: &'static [(char, char)];
```

### `OTHER_LOWERCASE`

```rust
const OTHER_LOWERCASE: &'static [(char, char)];
```

### `OTHER_MATH`

```rust
const OTHER_MATH: &'static [(char, char)];
```

### `OTHER_UPPERCASE`

```rust
const OTHER_UPPERCASE: &'static [(char, char)];
```

### `PATTERN_SYNTAX`

```rust
const PATTERN_SYNTAX: &'static [(char, char)];
```

### `PATTERN_WHITE_SPACE`

```rust
const PATTERN_WHITE_SPACE: &'static [(char, char)];
```

### `PREPENDED_CONCATENATION_MARK`

```rust
const PREPENDED_CONCATENATION_MARK: &'static [(char, char)];
```

### `QUOTATION_MARK`

```rust
const QUOTATION_MARK: &'static [(char, char)];
```

### `RADICAL`

```rust
const RADICAL: &'static [(char, char)];
```

### `REGIONAL_INDICATOR`

```rust
const REGIONAL_INDICATOR: &'static [(char, char)];
```

### `SENTENCE_TERMINAL`

```rust
const SENTENCE_TERMINAL: &'static [(char, char)];
```

### `SOFT_DOTTED`

```rust
const SOFT_DOTTED: &'static [(char, char)];
```

### `TERMINAL_PUNCTUATION`

```rust
const TERMINAL_PUNCTUATION: &'static [(char, char)];
```

### `UNIFIED_IDEOGRAPH`

```rust
const UNIFIED_IDEOGRAPH: &'static [(char, char)];
```

### `UPPERCASE`

```rust
const UPPERCASE: &'static [(char, char)];
```

### `VARIATION_SELECTOR`

```rust
const VARIATION_SELECTOR: &'static [(char, char)];
```

### `WHITE_SPACE`

```rust
const WHITE_SPACE: &'static [(char, char)];
```

### `XID_CONTINUE`

```rust
const XID_CONTINUE: &'static [(char, char)];
```

### `XID_START`

```rust
const XID_START: &'static [(char, char)];
```

