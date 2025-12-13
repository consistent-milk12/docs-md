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


## Contents

- [Enums](#enums)
  - [`BreakClass`](#breakclass)
  - [`BreakOpportunity`](#breakopportunity)
- [Functions](#functions)
  - [`is_safe_pair`](#is-safe-pair)
  - [`break_property`](#break-property)
  - [`linebreaks`](#linebreaks)
  - [`split_at_safe`](#split-at-safe)
- [Constants](#constants)
  - [`UNICODE_VERSION`](#unicode-version)
  - [`BMP_LIMIT`](#bmp-limit)
  - [`SHIFT_3`](#shift-3)
  - [`SHIFT_2`](#shift-2)
  - [`SHIFT_1`](#shift-1)
  - [`BMP_SHIFT`](#bmp-shift)
  - [`INDEX_2_BLOCK_LENGTH`](#index-2-block-length)
  - [`INDEX_3_BLOCK_LENGTH`](#index-3-block-length)
  - [`SMALL_DATA_BLOCK_LENGTH`](#small-data-block-length)
  - [`BMP_DATA_BLOCK_LENGTH`](#bmp-data-block-length)
  - [`ALLOWED_BREAK_BIT`](#allowed-break-bit)
  - [`MANDATORY_BREAK_BIT`](#mandatory-break-bit)
  - [`eot`](#eot)
  - [`sot`](#sot)
  - [`BREAK_PROP_TRIE_HIGH_START`](#break-prop-trie-high-start)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BreakClass`](#breakclass) | enum | Unicode line breaking class. |
| [`BreakOpportunity`](#breakopportunity) | enum | Break opportunity type. |
| [`is_safe_pair`](#is-safe-pair) | fn |  |
| [`break_property`](#break-property) | fn | Returns the line break property of the specified code point. |
| [`linebreaks`](#linebreaks) | fn | Returns an iterator over line break opportunities in the specified string. |
| [`split_at_safe`](#split-at-safe) | fn | Divides the string at the last index where further breaks do not depend on prior context. |
| [`UNICODE_VERSION`](#unicode-version) | const | The [Unicode version](https://www.unicode.org/versions/) conformed to. |
| [`BMP_LIMIT`](#bmp-limit) | const | Ceiling for code points in the Basic Multilingual Place (BMP). |
| [`SHIFT_3`](#shift-3) | const | Shift size for getting index-3 table offset. |
| [`SHIFT_2`](#shift-2) | const | Shift size for getting index-2 table offset. |
| [`SHIFT_1`](#shift-1) | const | Shift size for getting index-1 table offset. |
| [`BMP_SHIFT`](#bmp-shift) | const | Shift size for getting BMP block start. |
| [`INDEX_2_BLOCK_LENGTH`](#index-2-block-length) | const |  |
| [`INDEX_3_BLOCK_LENGTH`](#index-3-block-length) | const |  |
| [`SMALL_DATA_BLOCK_LENGTH`](#small-data-block-length) | const |  |
| [`BMP_DATA_BLOCK_LENGTH`](#bmp-data-block-length) | const |  |
| [`ALLOWED_BREAK_BIT`](#allowed-break-bit) | const |  |
| [`MANDATORY_BREAK_BIT`](#mandatory-break-bit) | const |  |
| [`eot`](#eot) | const |  |
| [`sot`](#sot) | const |  |
| [`BREAK_PROP_TRIE_HIGH_START`](#break-prop-trie-high-start) | const |  |

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

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:4-96`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L4-L96)*

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

##### `impl Any for BreakClass`

- <span id="breakclass-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BreakClass`

- <span id="breakclass-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BreakClass`

- <span id="breakclass-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BreakClass`

- <span id="breakclass-clone"></span>`fn clone(&self) -> BreakClass` — [`BreakClass`](#breakclass)

##### `impl CloneToUninit for BreakClass`

- <span id="breakclass-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BreakClass`

##### `impl Debug for BreakClass`

- <span id="breakclass-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BreakClass`

##### `impl<T> From for BreakClass`

- <span id="breakclass-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BreakClass`

- <span id="breakclass-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for BreakClass`

- <span id="breakclass-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BreakClass`

- <span id="breakclass-partialeq-eq"></span>`fn eq(&self, other: &BreakClass) -> bool` — [`BreakClass`](#breakclass)

##### `impl StructuralPartialEq for BreakClass`

##### `impl<U> TryFrom for BreakClass`

- <span id="breakclass-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="breakclass-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BreakClass`

- <span id="breakclass-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="breakclass-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BreakOpportunity`

```rust
enum BreakOpportunity {
    Mandatory,
    Allowed,
}
```

*Defined in [`unicode-linebreak-0.1.5/src/lib.rs:67-72`](../../.source_1765521767/unicode-linebreak-0.1.5/src/lib.rs#L67-L72)*

Break opportunity type.

#### Variants

- **`Mandatory`**

  A line must break at this spot.

- **`Allowed`**

  A line is allowed to end at this spot.

#### Trait Implementations

##### `impl Any for BreakOpportunity`

- <span id="breakopportunity-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BreakOpportunity`

- <span id="breakopportunity-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BreakOpportunity`

- <span id="breakopportunity-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BreakOpportunity`

- <span id="breakopportunity-clone"></span>`fn clone(&self) -> BreakOpportunity` — [`BreakOpportunity`](#breakopportunity)

##### `impl CloneToUninit for BreakOpportunity`

- <span id="breakopportunity-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BreakOpportunity`

##### `impl Debug for BreakOpportunity`

- <span id="breakopportunity-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BreakOpportunity`

##### `impl<T> From for BreakOpportunity`

- <span id="breakopportunity-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BreakOpportunity`

- <span id="breakopportunity-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BreakOpportunity`

- <span id="breakopportunity-partialeq-eq"></span>`fn eq(&self, other: &BreakOpportunity) -> bool` — [`BreakOpportunity`](#breakopportunity)

##### `impl StructuralPartialEq for BreakOpportunity`

##### `impl<U> TryFrom for BreakOpportunity`

- <span id="breakopportunity-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="breakopportunity-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BreakOpportunity`

- <span id="breakopportunity-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="breakopportunity-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `is_safe_pair`

```rust
fn is_safe_pair(a: BreakClass, b: BreakClass) -> bool
```

*Defined in [`unicode-linebreak-0.1.5/src/tables.rs:8-10`](../../.source_1765521767/unicode-linebreak-0.1.5/src/tables.rs#L8-L10)*

### `break_property`

```rust
fn break_property(codepoint: u32) -> BreakClass
```

*Defined in [`unicode-linebreak-0.1.5/src/lib.rs:41-63`](../../.source_1765521767/unicode-linebreak-0.1.5/src/lib.rs#L41-L63)*

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

*Defined in [`unicode-linebreak-0.1.5/src/lib.rs:89-114`](../../.source_1765521767/unicode-linebreak-0.1.5/src/lib.rs#L89-L114)*

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

*Defined in [`unicode-linebreak-0.1.5/src/lib.rs:136-147`](../../.source_1765521767/unicode-linebreak-0.1.5/src/lib.rs#L136-L147)*

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

*Defined in [`unicode-linebreak-0.1.5/src/lib.rs:27`](../../.source_1765521767/unicode-linebreak-0.1.5/src/lib.rs#L27)*

The [Unicode version](https://www.unicode.org/versions/) conformed to.

### `BMP_LIMIT`
```rust
const BMP_LIMIT: u32 = 65_536u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:112`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L112)*

Ceiling for code points in the Basic Multilingual Place (BMP).

### `SHIFT_3`
```rust
const SHIFT_3: u32 = 4u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:115`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L115)*

Shift size for getting index-3 table offset.

### `SHIFT_2`
```rust
const SHIFT_2: u32 = 9u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:117`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L117)*

Shift size for getting index-2 table offset.

### `SHIFT_1`
```rust
const SHIFT_1: u32 = 14u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:119`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L119)*

Shift size for getting index-1 table offset.

### `BMP_SHIFT`
```rust
const BMP_SHIFT: u32 = 6u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:121`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L121)*

Shift size for getting BMP block start.

### `INDEX_2_BLOCK_LENGTH`
```rust
const INDEX_2_BLOCK_LENGTH: u32 = 32u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:123`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L123)*

### `INDEX_3_BLOCK_LENGTH`
```rust
const INDEX_3_BLOCK_LENGTH: u32 = 32u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:124`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L124)*

### `SMALL_DATA_BLOCK_LENGTH`
```rust
const SMALL_DATA_BLOCK_LENGTH: u32 = 16u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:125`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L125)*

### `BMP_DATA_BLOCK_LENGTH`
```rust
const BMP_DATA_BLOCK_LENGTH: u32 = 64u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:126`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L126)*

### `ALLOWED_BREAK_BIT`
```rust
const ALLOWED_BREAK_BIT: u8 = 128u8;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:128`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L128)*

### `MANDATORY_BREAK_BIT`
```rust
const MANDATORY_BREAK_BIT: u8 = 64u8;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:129`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L129)*

### `eot`
```rust
const eot: u8 = 43u8;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:132`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L132)*

### `sot`
```rust
const sot: u8 = 44u8;
```

*Defined in [`unicode-linebreak-0.1.5/src/shared.rs:134`](../../.source_1765521767/unicode-linebreak-0.1.5/src/shared.rs#L134)*

### `BREAK_PROP_TRIE_HIGH_START`
```rust
const BREAK_PROP_TRIE_HIGH_START: u32 = 918_016u32;
```

*Defined in [`unicode-linebreak-0.1.5/src/tables.rs:1`](../../.source_1765521767/unicode-linebreak-0.1.5/src/tables.rs#L1)*

