# Crate `unicode_linebreak`

Implementation of the Line Breaking Algorithm described in [Unicode Standard Annex #14][UAX14].

Given an input text, locates "line break opportunities", or positions appropriate for wrapping
lines when displaying text.

# Example

```rust
use unicode_linebreak::{linebreaks, BreakOpportunity::{Mandatory, Allowed}};

let text = "a b \nc";
assert!(linebreaks(text).eq([
    (2, Allowed),   // May break after first space
    (5, Mandatory), // Must break after line feed
    (6, Mandatory)  // Must break at end of text, so that there always is at least one LB
]));
```

[UAX14]: https://www.unicode.org/reports/tr14/

## Enums

### `BreakClass`

```rust
enum BreakClass {
    Mandatory,
    CarriageReturn,
    LineFeed,
    CombiningMark,
    NextLine,
    Surrogate,
    WordJoiner,
    ZeroWidthSpace,
    NonBreakingGlue,
    Space,
    ZeroWidthJoiner,
    BeforeAndAfter,
    After,
    Before,
    Hyphen,
    Contingent,
    ClosePunctuation,
    CloseParenthesis,
    Exclamation,
    Inseparable,
    NonStarter,
    OpenPunctuation,
    Quotation,
    InfixSeparator,
    Numeric,
    Postfix,
    Prefix,
    Symbol,
    Ambiguous,
    Alphabetic,
    ConditionalJapaneseStarter,
    EmojiBase,
    EmojiModifier,
    HangulLvSyllable,
    HangulLvtSyllable,
    HebrewLetter,
    Ideographic,
    HangulLJamo,
    HangulVJamo,
    HangulTJamo,
    RegionalIndicator,
    ComplexContext,
    Unknown,
}
```

Unicode line breaking class.

#### Variants

- **`Mandatory`**

  Cause a line break (after)

- **`CarriageReturn`**

  Cause a line break (after), except between CR and LF

- **`LineFeed`**

  Cause a line break (after)

- **`CombiningMark`**

  Prohibit a line break between the character and the preceding character

- **`NextLine`**

  Cause a line break (after)

- **`Surrogate`**

  Do not occur in well-formed text

- **`WordJoiner`**

  Prohibit line breaks before and after

- **`ZeroWidthSpace`**

  Provide a break opportunity

- **`NonBreakingGlue`**

  Prohibit line breaks before and after

- **`Space`**

  Enable indirect line breaks

- **`ZeroWidthJoiner`**

  Prohibit line breaks within joiner sequences

- **`BeforeAndAfter`**

  Provide a line break opportunity before and after the character

- **`After`**

  Generally provide a line break opportunity after the character

- **`Before`**

  Generally provide a line break opportunity before the character

- **`Hyphen`**

  Provide a line break opportunity after the character, except in numeric context

- **`Contingent`**

  Provide a line break opportunity contingent on additional information

- **`ClosePunctuation`**

  Prohibit line breaks before

- **`CloseParenthesis`**

  Prohibit line breaks before

- **`Exclamation`**

  Prohibit line breaks before

- **`Inseparable`**

  Allow only indirect line breaks between pairs

- **`NonStarter`**

  Allow only indirect line breaks before

- **`OpenPunctuation`**

  Prohibit line breaks after

- **`Quotation`**

  Act like they are both opening and closing

- **`InfixSeparator`**

  Prevent breaks after any and before numeric

- **`Numeric`**

  Form numeric expressions for line breaking purposes

- **`Postfix`**

  Do not break following a numeric expression

- **`Prefix`**

  Do not break in front of a numeric expression

- **`Symbol`**

  Prevent a break before, and allow a break after

- **`Ambiguous`**

  Act like AL when the resolved EAW is N; otherwise, act as ID

- **`Alphabetic`**

  Are alphabetic characters or symbols that are used with alphabetic characters

- **`ConditionalJapaneseStarter`**

  Treat as NS or ID for strict or normal breaking.

- **`EmojiBase`**

  Do not break from following Emoji Modifier

- **`EmojiModifier`**

  Do not break from preceding Emoji Base

- **`HangulLvSyllable`**

  Form Korean syllable blocks

- **`HangulLvtSyllable`**

  Form Korean syllable blocks

- **`HebrewLetter`**

  Do not break around a following hyphen; otherwise act as Alphabetic

- **`Ideographic`**

  Break before or after, except in some numeric context

- **`HangulLJamo`**

  Form Korean syllable blocks

- **`HangulVJamo`**

  Form Korean syllable blocks

- **`HangulTJamo`**

  Form Korean syllable blocks

- **`RegionalIndicator`**

  Keep pairs together. For pairs, break before and after other classes

- **`ComplexContext`**

  Provide a line break opportunity contingent on additional, language-specific context analysis

- **`Unknown`**

  Have as yet unknown line breaking behavior or unassigned code positions

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> BreakClass` — [`BreakClass`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &BreakClass) -> bool` — [`BreakClass`](../index.md)

##### `impl StructuralPartialEq`

### `BreakOpportunity`

```rust
enum BreakOpportunity {
    Mandatory,
    Allowed,
}
```

Break opportunity type.

#### Variants

- **`Mandatory`**

  A line must break at this spot.

- **`Allowed`**

  A line is allowed to end at this spot.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> BreakOpportunity` — [`BreakOpportunity`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &BreakOpportunity) -> bool` — [`BreakOpportunity`](../index.md)

##### `impl StructuralPartialEq`

## Functions

### `break_property`

```rust
fn break_property(codepoint: u32) -> BreakClass
```

Returns the line break property of the specified code point.

# Examples

```rust
use unicode_linebreak::{BreakClass, break_property};
assert_eq!(break_property(0x2CF3), BreakClass::Alphabetic);
```

### `linebreaks`

```rust
fn linebreaks(s: &str) -> impl Iterator<Item = (usize, BreakOpportunity)> + Clone + '_
```

Returns an iterator over line break opportunities in the specified string.

Break opportunities are given as tuples of the byte index of the character succeeding the break
and the type.

Uses the default Line Breaking Algorithm with the tailoring that Complex-Context Dependent
(SA) characters get resolved to Ordinary Alphabetic and Symbol Characters (AL) regardless of
General_Category.

# Examples

```rust
use unicode_linebreak::{linebreaks, BreakOpportunity::{Mandatory, Allowed}};
assert!(linebreaks("Hello world!").eq(vec![(6, Allowed), (12, Mandatory)]));
```

### `split_at_safe`

```rust
fn split_at_safe(s: &str) -> (&str, &str)
```

Divides the string at the last index where further breaks do not depend on prior context.

The trivial index at `eot` is excluded.

A common optimization is to determine only the nearest line break opportunity before the first
character that would cause the line to become overfull, requiring backward traversal, of which
there are two approaches:

* Cache breaks from forward traversals
* Step backward and with `split_at_safe` find a pos to safely search forward from, repeatedly

# Examples

```rust
use unicode_linebreak::{linebreaks, split_at_safe};
let s = "Not allowed to break within em dashes: — —";
let (prev, safe) = split_at_safe(s);
let n = prev.len();
assert!(linebreaks(safe).eq(linebreaks(s).filter_map(|(i, x)| i.checked_sub(n).map(|i| (i, x)))));
```

## Constants

### `UNICODE_VERSION`

```rust
const UNICODE_VERSION: (u8, u8, u8);
```

The [Unicode version](https://www.unicode.org/versions/) conformed to.

