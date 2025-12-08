*[regex_syntax](../../index.md) / [unicode_tables](../index.md) / [general_category](index.md)*

---

# Module `general_category`

## Contents

- [Constants](#constants)
  - [`BY_NAME`](#by_name)
  - [`CASED_LETTER`](#cased_letter)
  - [`CLOSE_PUNCTUATION`](#close_punctuation)
  - [`CONNECTOR_PUNCTUATION`](#connector_punctuation)
  - [`CONTROL`](#control)
  - [`CURRENCY_SYMBOL`](#currency_symbol)
  - [`DASH_PUNCTUATION`](#dash_punctuation)
  - [`DECIMAL_NUMBER`](#decimal_number)
  - [`ENCLOSING_MARK`](#enclosing_mark)
  - [`FINAL_PUNCTUATION`](#final_punctuation)
  - [`FORMAT`](#format)
  - [`INITIAL_PUNCTUATION`](#initial_punctuation)
  - [`LETTER`](#letter)
  - [`LETTER_NUMBER`](#letter_number)
  - [`LINE_SEPARATOR`](#line_separator)
  - [`LOWERCASE_LETTER`](#lowercase_letter)
  - [`MARK`](#mark)
  - [`MATH_SYMBOL`](#math_symbol)
  - [`MODIFIER_LETTER`](#modifier_letter)
  - [`MODIFIER_SYMBOL`](#modifier_symbol)
  - [`NONSPACING_MARK`](#nonspacing_mark)
  - [`NUMBER`](#number)
  - [`OPEN_PUNCTUATION`](#open_punctuation)
  - [`OTHER`](#other)
  - [`OTHER_LETTER`](#other_letter)
  - [`OTHER_NUMBER`](#other_number)
  - [`OTHER_PUNCTUATION`](#other_punctuation)
  - [`OTHER_SYMBOL`](#other_symbol)
  - [`PARAGRAPH_SEPARATOR`](#paragraph_separator)
  - [`PRIVATE_USE`](#private_use)
  - [`PUNCTUATION`](#punctuation)
  - [`SEPARATOR`](#separator)
  - [`SPACE_SEPARATOR`](#space_separator)
  - [`SPACING_MARK`](#spacing_mark)
  - [`SYMBOL`](#symbol)
  - [`TITLECASE_LETTER`](#titlecase_letter)
  - [`UNASSIGNED`](#unassigned)
  - [`UPPERCASE_LETTER`](#uppercase_letter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BY_NAME`](#by_name) | const |  |
| [`CASED_LETTER`](#cased_letter) | const |  |
| [`CLOSE_PUNCTUATION`](#close_punctuation) | const |  |
| [`CONNECTOR_PUNCTUATION`](#connector_punctuation) | const |  |
| [`CONTROL`](#control) | const |  |
| [`CURRENCY_SYMBOL`](#currency_symbol) | const |  |
| [`DASH_PUNCTUATION`](#dash_punctuation) | const |  |
| [`DECIMAL_NUMBER`](#decimal_number) | const |  |
| [`ENCLOSING_MARK`](#enclosing_mark) | const |  |
| [`FINAL_PUNCTUATION`](#final_punctuation) | const |  |
| [`FORMAT`](#format) | const |  |
| [`INITIAL_PUNCTUATION`](#initial_punctuation) | const |  |
| [`LETTER`](#letter) | const |  |
| [`LETTER_NUMBER`](#letter_number) | const |  |
| [`LINE_SEPARATOR`](#line_separator) | const |  |
| [`LOWERCASE_LETTER`](#lowercase_letter) | const |  |
| [`MARK`](#mark) | const |  |
| [`MATH_SYMBOL`](#math_symbol) | const |  |
| [`MODIFIER_LETTER`](#modifier_letter) | const |  |
| [`MODIFIER_SYMBOL`](#modifier_symbol) | const |  |
| [`NONSPACING_MARK`](#nonspacing_mark) | const |  |
| [`NUMBER`](#number) | const |  |
| [`OPEN_PUNCTUATION`](#open_punctuation) | const |  |
| [`OTHER`](#other) | const |  |
| [`OTHER_LETTER`](#other_letter) | const |  |
| [`OTHER_NUMBER`](#other_number) | const |  |
| [`OTHER_PUNCTUATION`](#other_punctuation) | const |  |
| [`OTHER_SYMBOL`](#other_symbol) | const |  |
| [`PARAGRAPH_SEPARATOR`](#paragraph_separator) | const |  |
| [`PRIVATE_USE`](#private_use) | const |  |
| [`PUNCTUATION`](#punctuation) | const |  |
| [`SEPARATOR`](#separator) | const |  |
| [`SPACE_SEPARATOR`](#space_separator) | const |  |
| [`SPACING_MARK`](#spacing_mark) | const |  |
| [`SYMBOL`](#symbol) | const |  |
| [`TITLECASE_LETTER`](#titlecase_letter) | const |  |
| [`UNASSIGNED`](#unassigned) | const |  |
| [`UPPERCASE_LETTER`](#uppercase_letter) | const |  |

## Constants

### `BY_NAME`

```rust
const BY_NAME: &'static [(&'static str, &'static [(char, char)])];
```

### `CASED_LETTER`

```rust
const CASED_LETTER: &'static [(char, char)];
```

### `CLOSE_PUNCTUATION`

```rust
const CLOSE_PUNCTUATION: &'static [(char, char)];
```

### `CONNECTOR_PUNCTUATION`

```rust
const CONNECTOR_PUNCTUATION: &'static [(char, char)];
```

### `CONTROL`

```rust
const CONTROL: &'static [(char, char)];
```

### `CURRENCY_SYMBOL`

```rust
const CURRENCY_SYMBOL: &'static [(char, char)];
```

### `DASH_PUNCTUATION`

```rust
const DASH_PUNCTUATION: &'static [(char, char)];
```

### `DECIMAL_NUMBER`

```rust
const DECIMAL_NUMBER: &'static [(char, char)];
```

### `ENCLOSING_MARK`

```rust
const ENCLOSING_MARK: &'static [(char, char)];
```

### `FINAL_PUNCTUATION`

```rust
const FINAL_PUNCTUATION: &'static [(char, char)];
```

### `FORMAT`

```rust
const FORMAT: &'static [(char, char)];
```

### `INITIAL_PUNCTUATION`

```rust
const INITIAL_PUNCTUATION: &'static [(char, char)];
```

### `LETTER`

```rust
const LETTER: &'static [(char, char)];
```

### `LETTER_NUMBER`

```rust
const LETTER_NUMBER: &'static [(char, char)];
```

### `LINE_SEPARATOR`

```rust
const LINE_SEPARATOR: &'static [(char, char)];
```

### `LOWERCASE_LETTER`

```rust
const LOWERCASE_LETTER: &'static [(char, char)];
```

### `MARK`

```rust
const MARK: &'static [(char, char)];
```

### `MATH_SYMBOL`

```rust
const MATH_SYMBOL: &'static [(char, char)];
```

### `MODIFIER_LETTER`

```rust
const MODIFIER_LETTER: &'static [(char, char)];
```

### `MODIFIER_SYMBOL`

```rust
const MODIFIER_SYMBOL: &'static [(char, char)];
```

### `NONSPACING_MARK`

```rust
const NONSPACING_MARK: &'static [(char, char)];
```

### `NUMBER`

```rust
const NUMBER: &'static [(char, char)];
```

### `OPEN_PUNCTUATION`

```rust
const OPEN_PUNCTUATION: &'static [(char, char)];
```

### `OTHER`

```rust
const OTHER: &'static [(char, char)];
```

### `OTHER_LETTER`

```rust
const OTHER_LETTER: &'static [(char, char)];
```

### `OTHER_NUMBER`

```rust
const OTHER_NUMBER: &'static [(char, char)];
```

### `OTHER_PUNCTUATION`

```rust
const OTHER_PUNCTUATION: &'static [(char, char)];
```

### `OTHER_SYMBOL`

```rust
const OTHER_SYMBOL: &'static [(char, char)];
```

### `PARAGRAPH_SEPARATOR`

```rust
const PARAGRAPH_SEPARATOR: &'static [(char, char)];
```

### `PRIVATE_USE`

```rust
const PRIVATE_USE: &'static [(char, char)];
```

### `PUNCTUATION`

```rust
const PUNCTUATION: &'static [(char, char)];
```

### `SEPARATOR`

```rust
const SEPARATOR: &'static [(char, char)];
```

### `SPACE_SEPARATOR`

```rust
const SPACE_SEPARATOR: &'static [(char, char)];
```

### `SPACING_MARK`

```rust
const SPACING_MARK: &'static [(char, char)];
```

### `SYMBOL`

```rust
const SYMBOL: &'static [(char, char)];
```

### `TITLECASE_LETTER`

```rust
const TITLECASE_LETTER: &'static [(char, char)];
```

### `UNASSIGNED`

```rust
const UNASSIGNED: &'static [(char, char)];
```

### `UPPERCASE_LETTER`

```rust
const UPPERCASE_LETTER: &'static [(char, char)];
```

