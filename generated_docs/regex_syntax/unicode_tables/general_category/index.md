*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [general_category](index.md)*

---

# Module `general_category`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by-name)
  - [`CASED_LETTER`](#cased-letter)
  - [`CLOSE_PUNCTUATION`](#close-punctuation)
  - [`CONNECTOR_PUNCTUATION`](#connector-punctuation)
  - [`CONTROL`](#control)
  - [`CURRENCY_SYMBOL`](#currency-symbol)
  - [`DASH_PUNCTUATION`](#dash-punctuation)
  - [`DECIMAL_NUMBER`](#decimal-number)
  - [`ENCLOSING_MARK`](#enclosing-mark)
  - [`FINAL_PUNCTUATION`](#final-punctuation)
  - [`FORMAT`](#format)
  - [`INITIAL_PUNCTUATION`](#initial-punctuation)
  - [`LETTER`](#letter)
  - [`LETTER_NUMBER`](#letter-number)
  - [`LINE_SEPARATOR`](#line-separator)
  - [`LOWERCASE_LETTER`](#lowercase-letter)
  - [`MARK`](#mark)
  - [`MATH_SYMBOL`](#math-symbol)
  - [`MODIFIER_LETTER`](#modifier-letter)
  - [`MODIFIER_SYMBOL`](#modifier-symbol)
  - [`NONSPACING_MARK`](#nonspacing-mark)
  - [`NUMBER`](#number)
  - [`OPEN_PUNCTUATION`](#open-punctuation)
  - [`OTHER`](#other)
  - [`OTHER_LETTER`](#other-letter)
  - [`OTHER_NUMBER`](#other-number)
  - [`OTHER_PUNCTUATION`](#other-punctuation)
  - [`OTHER_SYMBOL`](#other-symbol)
  - [`PARAGRAPH_SEPARATOR`](#paragraph-separator)
  - [`PRIVATE_USE`](#private-use)
  - [`PUNCTUATION`](#punctuation)
  - [`SEPARATOR`](#separator)
  - [`SPACE_SEPARATOR`](#space-separator)
  - [`SPACING_MARK`](#spacing-mark)
  - [`SYMBOL`](#symbol)
  - [`TITLECASE_LETTER`](#titlecase-letter)
  - [`UNASSIGNED`](#unassigned)
  - [`UPPERCASE_LETTER`](#uppercase-letter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by-name) | const |  |
| [`CASED_LETTER`](#cased-letter) | const |  |
| [`CLOSE_PUNCTUATION`](#close-punctuation) | const |  |
| [`CONNECTOR_PUNCTUATION`](#connector-punctuation) | const |  |
| [`CONTROL`](#control) | const |  |
| [`CURRENCY_SYMBOL`](#currency-symbol) | const |  |
| [`DASH_PUNCTUATION`](#dash-punctuation) | const |  |
| [`DECIMAL_NUMBER`](#decimal-number) | const |  |
| [`ENCLOSING_MARK`](#enclosing-mark) | const |  |
| [`FINAL_PUNCTUATION`](#final-punctuation) | const |  |
| [`FORMAT`](#format) | const |  |
| [`INITIAL_PUNCTUATION`](#initial-punctuation) | const |  |
| [`LETTER`](#letter) | const |  |
| [`LETTER_NUMBER`](#letter-number) | const |  |
| [`LINE_SEPARATOR`](#line-separator) | const |  |
| [`LOWERCASE_LETTER`](#lowercase-letter) | const |  |
| [`MARK`](#mark) | const |  |
| [`MATH_SYMBOL`](#math-symbol) | const |  |
| [`MODIFIER_LETTER`](#modifier-letter) | const |  |
| [`MODIFIER_SYMBOL`](#modifier-symbol) | const |  |
| [`NONSPACING_MARK`](#nonspacing-mark) | const |  |
| [`NUMBER`](#number) | const |  |
| [`OPEN_PUNCTUATION`](#open-punctuation) | const |  |
| [`OTHER`](#other) | const |  |
| [`OTHER_LETTER`](#other-letter) | const |  |
| [`OTHER_NUMBER`](#other-number) | const |  |
| [`OTHER_PUNCTUATION`](#other-punctuation) | const |  |
| [`OTHER_SYMBOL`](#other-symbol) | const |  |
| [`PARAGRAPH_SEPARATOR`](#paragraph-separator) | const |  |
| [`PRIVATE_USE`](#private-use) | const |  |
| [`PUNCTUATION`](#punctuation) | const |  |
| [`SEPARATOR`](#separator) | const |  |
| [`SPACE_SEPARATOR`](#space-separator) | const |  |
| [`SPACING_MARK`](#spacing-mark) | const |  |
| [`SYMBOL`](#symbol) | const |  |
| [`TITLECASE_LETTER`](#titlecase-letter) | const |  |
| [`UNASSIGNED`](#unassigned) | const |  |
| [`UPPERCASE_LETTER`](#uppercase-letter) | const |  |

## Constants

### `BY_NAME`
```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:9-47`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L9-L47)*

### `CASED_LETTER`
```rust
const CASED_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:49-195`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L49-L195)*

### `CLOSE_PUNCTUATION`
```rust
const CLOSE_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:197-274`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L197-L274)*

### `CONNECTOR_PUNCTUATION`
```rust
const CONNECTOR_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:276-283`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L276-L283)*

### `CONTROL`
```rust
const CONTROL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:285-286`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L285-L286)*

### `CURRENCY_SYMBOL`
```rust
const CURRENCY_SYMBOL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:288-310`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L288-L310)*

### `DASH_PUNCTUATION`
```rust
const DASH_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:312-333`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L312-L333)*

### `DECIMAL_NUMBER`
```rust
const DECIMAL_NUMBER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:335-407`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L335-L407)*

### `ENCLOSING_MARK`
```rust
const ENCLOSING_MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:409-415`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L409-L415)*

### `FINAL_PUNCTUATION`
```rust
const FINAL_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:417-428`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L417-L428)*

### `FORMAT`
```rust
const FORMAT: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:430-452`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L430-L452)*

### `INITIAL_PUNCTUATION`
```rust
const INITIAL_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:454-466`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L454-L466)*

### `LETTER`
```rust
const LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:468-1146`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L468-L1146)*

### `LETTER_NUMBER`
```rust
const LETTER_NUMBER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:1148-1161`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L1148-L1161)*

### `LINE_SEPARATOR`
```rust
const LINE_SEPARATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:1163-1164`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L1163-L1164)*

### `LOWERCASE_LETTER`
```rust
const LOWERCASE_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:1166-1829`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L1166-L1829)*

### `MARK`
```rust
const MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:1831-2153`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L1831-L2153)*

### `MATH_SYMBOL`
```rust
const MATH_SYMBOL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2155-2221`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2155-L2221)*

### `MODIFIER_LETTER`
```rust
const MODIFIER_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2223-2299`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2223-L2299)*

### `MODIFIER_SYMBOL`
```rust
const MODIFIER_SYMBOL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2301-2333`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2301-L2333)*

### `NONSPACING_MARK`
```rust
const NONSPACING_MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2335-2693`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2335-L2693)*

### `NUMBER`
```rust
const NUMBER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2695-2840`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2695-L2840)*

### `OPEN_PUNCTUATION`
```rust
const OPEN_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2842-2922`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2842-L2922)*

### `OTHER`
```rust
const OTHER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:2924-3661`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L2924-L3661)*

### `OTHER_LETTER`
```rust
const OTHER_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:3663-4192`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L3663-L4192)*

### `OTHER_NUMBER`
```rust
const OTHER_NUMBER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4194-4267`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4194-L4267)*

### `OTHER_PUNCTUATION`
```rust
const OTHER_PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4269-4463`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4269-L4463)*

### `OTHER_SYMBOL`
```rust
const OTHER_SYMBOL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4465-4653`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4465-L4653)*

### `PARAGRAPH_SEPARATOR`
```rust
const PARAGRAPH_SEPARATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4655-4656`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4655-L4656)*

### `PRIVATE_USE`
```rust
const PRIVATE_USE: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4658-4662`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4658-L4662)*

### `PUNCTUATION`
```rust
const PUNCTUATION: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4664-4863`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4664-L4863)*

### `SEPARATOR`
```rust
const SEPARATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4865-4874`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4865-L4874)*

### `SPACE_SEPARATOR`
```rust
const SPACE_SEPARATOR: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4876-4884`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4876-L4884)*

### `SPACING_MARK`
```rust
const SPACING_MARK: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:4886-5077`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L4886-L5077)*

### `SYMBOL`
```rust
const SYMBOL: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:5079-5316`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L5079-L5316)*

### `TITLECASE_LETTER`
```rust
const TITLECASE_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:5318-5329`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L5318-L5329)*

### `UNASSIGNED`
```rust
const UNASSIGNED: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:5331-6063`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L5331-L6063)*

### `UPPERCASE_LETTER`
```rust
const UPPERCASE_LETTER: &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode_tables/general_category.rs:6065-6717`](../../../../.source_1765210505/regex-syntax-0.8.8/src/unicode_tables/general_category.rs#L6065-L6717)*

