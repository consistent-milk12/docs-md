*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [property_bool](index.md)*

---

# Module `property_bool`

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

