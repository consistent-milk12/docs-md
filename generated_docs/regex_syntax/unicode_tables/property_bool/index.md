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

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9-75`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9-L75)*

### `ASCII_HEX_DIGIT`
```rust
const ASCII_HEX_DIGIT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:77-78`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L77-L78)*

### `ALPHABETIC`
```rust
const ALPHABETIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:80-838`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L80-L838)*

### `BIDI_CONTROL`
```rust
const BIDI_CONTROL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:840-845`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L840-L845)*

### `BIDI_MIRRORED`
```rust
const BIDI_MIRRORED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:847-962`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L847-L962)*

### `CASE_IGNORABLE`
```rust
const CASE_IGNORABLE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:964-1417`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L964-L1417)*

### `CASED`
```rust
const CASED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:1419-1579`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L1419-L1579)*

### `CHANGES_WHEN_CASEFOLDED`
```rust
const CHANGES_WHEN_CASEFOLDED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:1581-2208`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L1581-L2208)*

### `CHANGES_WHEN_CASEMAPPED`
```rust
const CHANGES_WHEN_CASEMAPPED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:2210-2342`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L2210-L2342)*

### `CHANGES_WHEN_LOWERCASED`
```rust
const CHANGES_WHEN_LOWERCASED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:2344-2959`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L2344-L2959)*

### `CHANGES_WHEN_TITLECASED`
```rust
const CHANGES_WHEN_TITLECASED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:2961-3591`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L2961-L3591)*

### `CHANGES_WHEN_UPPERCASED`
```rust
const CHANGES_WHEN_UPPERCASED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:3593-4224`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L3593-L4224)*

### `DASH`
```rust
const DASH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4226-4251`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4226-L4251)*

### `DEFAULT_IGNORABLE_CODE_POINT`
```rust
const DEFAULT_IGNORABLE_CODE_POINT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4253-4271`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4253-L4271)*

### `DEPRECATED`
```rust
const DEPRECATED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4273-4282`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4273-L4282)*

### `DIACRITIC`
```rust
const DIACRITIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4284-4499`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4284-L4499)*

### `EMOJI`
```rust
const EMOJI: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4501-4652`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4501-L4652)*

### `EMOJI_COMPONENT`
```rust
const EMOJI_COMPONENT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4654-4665`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4654-L4665)*

### `EMOJI_MODIFIER`
```rust
const EMOJI_MODIFIER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4667`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4667)*

### `EMOJI_MODIFIER_BASE`
```rust
const EMOJI_MODIFIER_BASE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4669-4710`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4669-L4710)*

### `EMOJI_PRESENTATION`
```rust
const EMOJI_PRESENTATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4712-4793`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4712-L4793)*

### `EXTENDED_PICTOGRAPHIC`
```rust
const EXTENDED_PICTOGRAPHIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4795-4874`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4795-L4874)*

### `EXTENDER`
```rust
const EXTENDER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4876-4918`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4876-L4918)*

### `GRAPHEME_BASE`
```rust
const GRAPHEME_BASE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:4920-5815`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L4920-L5815)*

### `GRAPHEME_EXTEND`
```rust
const GRAPHEME_EXTEND: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:5817-6193`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L5817-L6193)*

### `GRAPHEME_LINK`
```rust
const GRAPHEME_LINK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6195-6254`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6195-L6254)*

### `HEX_DIGIT`
```rust
const HEX_DIGIT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6256-6263`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6256-L6263)*

### `HYPHEN`
```rust
const HYPHEN: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6265-6276`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6265-L6276)*

### `IDS_BINARY_OPERATOR`
```rust
const IDS_BINARY_OPERATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6278-6279`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6278-L6279)*

### `IDS_TRINARY_OPERATOR`
```rust
const IDS_TRINARY_OPERATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6281`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6281)*

### `IDS_UNARY_OPERATOR`
```rust
const IDS_UNARY_OPERATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6283`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6283)*

### `ID_COMPAT_MATH_CONTINUE`
```rust
const ID_COMPAT_MATH_CONTINUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6285-6304`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6285-L6304)*

### `ID_COMPAT_MATH_START`
```rust
const ID_COMPAT_MATH_START: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6306-6320`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6306-L6320)*

### `ID_CONTINUE`
```rust
const ID_CONTINUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:6322-7116`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L6322-L7116)*

### `ID_START`
```rust
const ID_START: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:7118-7796`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L7118-L7796)*

### `IDEOGRAPHIC`
```rust
const IDEOGRAPHIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:7798-7820`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L7798-L7820)*

### `INCB`
```rust
const INCB: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:7822-8221`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L7822-L8221)*

### `JOIN_CONTROL`
```rust
const JOIN_CONTROL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:8223`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L8223)*

### `LOGICAL_ORDER_EXCEPTION`
```rust
const LOGICAL_ORDER_EXCEPTION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:8225-8233`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L8225-L8233)*

### `LOWERCASE`
```rust
const LOWERCASE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:8235-8911`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L8235-L8911)*

### `MATH`
```rust
const MATH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:8913-9053`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L8913-L9053)*

### `MODIFIER_COMBINING_MARK`
```rust
const MODIFIER_COMBINING_MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9055-9065`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9055-L9065)*

### `NONCHARACTER_CODE_POINT`
```rust
const NONCHARACTER_CODE_POINT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9067-9086`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9067-L9086)*

### `OTHER_ALPHABETIC`
```rust
const OTHER_ALPHABETIC: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9088-9339`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9088-L9339)*

### `OTHER_DEFAULT_IGNORABLE_CODE_POINT`
```rust
const OTHER_DEFAULT_IGNORABLE_CODE_POINT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9341-9353`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9341-L9353)*

### `OTHER_GRAPHEME_EXTEND`
```rust
const OTHER_GRAPHEME_EXTEND: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9355-9405`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9355-L9405)*

### `OTHER_ID_CONTINUE`
```rust
const OTHER_ID_CONTINUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9407-9415`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9407-L9415)*

### `OTHER_ID_START`
```rust
const OTHER_ID_START: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9417-9418`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9417-L9418)*

### `OTHER_LOWERCASE`
```rust
const OTHER_LOWERCASE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9420-9449`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9420-L9449)*

### `OTHER_MATH`
```rust
const OTHER_MATH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9451-9586`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9451-L9586)*

### `OTHER_UPPERCASE`
```rust
const OTHER_UPPERCASE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9588-9589`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9588-L9589)*

### `PATTERN_SYNTAX`
```rust
const PATTERN_SYNTAX: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9591-9620`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9591-L9620)*

### `PATTERN_WHITE_SPACE`
```rust
const PATTERN_WHITE_SPACE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9622-9628`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9622-L9628)*

### `PREPENDED_CONCATENATION_MARK`
```rust
const PREPENDED_CONCATENATION_MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9630-9638`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9630-L9638)*

### `QUOTATION_MARK`
```rust
const QUOTATION_MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9640-9654`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9640-L9654)*

### `RADICAL`
```rust
const RADICAL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9656-9657`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9656-L9657)*

### `REGIONAL_INDICATOR`
```rust
const REGIONAL_INDICATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9659`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9659)*

### `SENTENCE_TERMINAL`
```rust
const SENTENCE_TERMINAL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9661-9750`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9661-L9750)*

### `SOFT_DOTTED`
```rust
const SOFT_DOTTED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9752-9787`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9752-L9787)*

### `TERMINAL_PUNCTUATION`
```rust
const TERMINAL_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9789-9906`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9789-L9906)*

### `UNIFIED_IDEOGRAPH`
```rust
const UNIFIED_IDEOGRAPH: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9908-9926`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9908-L9926)*

### `UPPERCASE`
```rust
const UPPERCASE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:9928-10585`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L9928-L10585)*

### `VARIATION_SELECTOR`
```rust
const VARIATION_SELECTOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:10587-10592`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L10587-L10592)*

### `WHITE_SPACE`
```rust
const WHITE_SPACE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:10594-10605`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L10594-L10605)*

### `XID_CONTINUE`
```rust
const XID_CONTINUE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:10607-11408`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L10607-L11408)*

### `XID_START`
```rust
const XID_START: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/property_bool.rs:11410-12095`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/property_bool.rs#L11410-L12095)*

