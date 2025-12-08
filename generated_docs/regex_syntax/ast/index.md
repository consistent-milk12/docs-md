*[regex_syntax](../index.md) / [ast](index.md)*

---

# Module `ast`

Defines an abstract syntax for regular expressions.

## Contents

- [Modules](#modules)
  - [`parse`](#parse)
  - [`print`](#print)
  - [`visitor`](#visitor)
- [Structs](#structs)
  - [`Error`](#error)
  - [`Span`](#span)
  - [`Position`](#position)
  - [`WithComments`](#withcomments)
  - [`Comment`](#comment)
  - [`Alternation`](#alternation)
  - [`Concat`](#concat)
  - [`Literal`](#literal)
  - [`ClassPerl`](#classperl)
  - [`ClassAscii`](#classascii)
  - [`ClassUnicode`](#classunicode)
  - [`ClassBracketed`](#classbracketed)
  - [`ClassSetRange`](#classsetrange)
  - [`ClassSetUnion`](#classsetunion)
  - [`ClassSetBinaryOp`](#classsetbinaryop)
  - [`Assertion`](#assertion)
  - [`Repetition`](#repetition)
  - [`RepetitionOp`](#repetitionop)
  - [`Group`](#group)
  - [`CaptureName`](#capturename)
  - [`SetFlags`](#setflags)
  - [`Flags`](#flags)
  - [`FlagsItem`](#flagsitem)
- [Enums](#enums)
  - [`ErrorKind`](#errorkind)
  - [`Ast`](#ast)
  - [`LiteralKind`](#literalkind)
  - [`SpecialLiteralKind`](#specialliteralkind)
  - [`HexLiteralKind`](#hexliteralkind)
  - [`ClassPerlKind`](#classperlkind)
  - [`ClassAsciiKind`](#classasciikind)
  - [`ClassUnicodeKind`](#classunicodekind)
  - [`ClassUnicodeOpKind`](#classunicodeopkind)
  - [`ClassSet`](#classset)
  - [`ClassSetItem`](#classsetitem)
  - [`ClassSetBinaryOpKind`](#classsetbinaryopkind)
  - [`AssertionKind`](#assertionkind)
  - [`RepetitionKind`](#repetitionkind)
  - [`RepetitionRange`](#repetitionrange)
  - [`GroupKind`](#groupkind)
  - [`FlagsItemKind`](#flagsitemkind)
  - [`Flag`](#flag)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`unnamed`](#unnamed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse`](#parse) | mod | This module provides a regular expression parser. |
| [`print`](#print) | mod | This module provides a regular expression printer for `Ast`. |
| [`visitor`](#visitor) | mod |  |
| [`Error`](#error) | struct | An error that occurred while parsing a regular expression into an abstract |
| [`Span`](#span) | struct | Span represents the position information of a single AST item. |
| [`Position`](#position) | struct | A single position in a regular expression. |
| [`WithComments`](#withcomments) | struct | An abstract syntax tree for a singular expression along with comments |
| [`Comment`](#comment) | struct | A comment from a regular expression with an associated span. |
| [`Alternation`](#alternation) | struct | An alternation of regular expressions. |
| [`Concat`](#concat) | struct | A concatenation of regular expressions. |
| [`Literal`](#literal) | struct | A single literal expression. |
| [`ClassPerl`](#classperl) | struct | A Perl character class. |
| [`ClassAscii`](#classascii) | struct | An ASCII character class. |
| [`ClassUnicode`](#classunicode) | struct | A Unicode character class. |
| [`ClassBracketed`](#classbracketed) | struct | A bracketed character class, e.g., `[a-z0-9]`. |
| [`ClassSetRange`](#classsetrange) | struct | A single character class range in a set. |
| [`ClassSetUnion`](#classsetunion) | struct | A union of items inside a character class set. |
| [`ClassSetBinaryOp`](#classsetbinaryop) | struct | A Unicode character class set operation. |
| [`Assertion`](#assertion) | struct | A single zero-width assertion. |
| [`Repetition`](#repetition) | struct | A repetition operation applied to a regular expression. |
| [`RepetitionOp`](#repetitionop) | struct | The repetition operator itself. |
| [`Group`](#group) | struct | A grouped regular expression. |
| [`CaptureName`](#capturename) | struct | A capture name. |
| [`SetFlags`](#setflags) | struct | A group of flags that is not applied to a particular regular expression. |
| [`Flags`](#flags) | struct | A group of flags. |
| [`FlagsItem`](#flagsitem) | struct | A single item in a group of flags. |
| [`ErrorKind`](#errorkind) | enum | The type of an error that occurred while building an AST. |
| [`Ast`](#ast) | enum | An abstract syntax tree for a single regular expression. |
| [`LiteralKind`](#literalkind) | enum | The kind of a single literal expression. |
| [`SpecialLiteralKind`](#specialliteralkind) | enum | The type of a special literal. |
| [`HexLiteralKind`](#hexliteralkind) | enum | The type of a Unicode hex literal. |
| [`ClassPerlKind`](#classperlkind) | enum | The available Perl character classes. |
| [`ClassAsciiKind`](#classasciikind) | enum | The available ASCII character classes. |
| [`ClassUnicodeKind`](#classunicodekind) | enum | The available forms of Unicode character classes. |
| [`ClassUnicodeOpKind`](#classunicodeopkind) | enum | The type of op used in a Unicode character class. |
| [`ClassSet`](#classset) | enum | A character class set. |
| [`ClassSetItem`](#classsetitem) | enum | A single component of a character class set. |
| [`ClassSetBinaryOpKind`](#classsetbinaryopkind) | enum | The type of a Unicode character class set operation. |
| [`AssertionKind`](#assertionkind) | enum | An assertion kind. |
| [`RepetitionKind`](#repetitionkind) | enum | The kind of a repetition operator. |
| [`RepetitionRange`](#repetitionrange) | enum | A range repetition operator. |
| [`GroupKind`](#groupkind) | enum | The kind of a group. |
| [`FlagsItemKind`](#flagsitemkind) | enum | The kind of an item in a group of flags. |
| [`Flag`](#flag) | enum | A single flag. |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | fn |  |

## Modules

- [`parse`](parse/index.md) - This module provides a regular expression parser.
- [`print`](print/index.md) - This module provides a regular expression printer for `Ast`.
- [`visitor`](visitor/index.md) - 

## Structs

### `Error`

```rust
struct Error {
    kind: ErrorKind,
    pattern: alloc::string::String,
    span: Span,
}
```

An error that occurred while parsing a regular expression into an abstract
syntax tree.

Note that not all ASTs represents a valid regular expression. For example,
an AST is constructed without error for `\p{Quux}`, but `Quux` is not a
valid Unicode property name. That particular error is reported when
translating an AST to the high-level intermediate representation (`HIR`).

#### Fields

- **`kind`**: `ErrorKind`

  The kind of error.

- **`pattern`**: `alloc::string::String`

  The original pattern that the parser generated the error from. Every
  span in an error is a valid range into this string.

- **`span`**: `Span`

  The span of this error.

#### Implementations

- <span id="error-kind"></span>`fn kind(&self) -> &ErrorKind` — [`ErrorKind`](#errorkind)

- <span id="error-pattern"></span>`fn pattern(&self) -> &str`

- <span id="error-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

- <span id="error-auxiliary-span"></span>`fn auxiliary_span(&self) -> Option<&Span>` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `Span`

```rust
struct Span {
    pub start: Position,
    pub end: Position,
}
```

Span represents the position information of a single AST item.

All span positions are absolute byte offsets that can be used on the
original regular expression that was parsed.

#### Fields

- **`start`**: `Position`

  The start byte offset.

- **`end`**: `Position`

  The end byte offset.

#### Implementations

- <span id="span-new"></span>`fn new(start: Position, end: Position) -> Span` — [`Position`](#position), [`Span`](#span)

- <span id="span-splat"></span>`fn splat(pos: Position) -> Span` — [`Position`](#position), [`Span`](#span)

- <span id="span-with-start"></span>`fn with_start(self, pos: Position) -> Span` — [`Position`](#position), [`Span`](#span)

- <span id="span-with-end"></span>`fn with_end(self, pos: Position) -> Span` — [`Position`](#position), [`Span`](#span)

- <span id="span-is-one-line"></span>`fn is_one_line(&self) -> bool`

- <span id="span-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl Ord for Span`

- <span id="span-cmp"></span>`fn cmp(&self, other: &Span) -> Ordering` — [`Span`](#span)

##### `impl PartialEq for Span`

- <span id="span-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](#span)

##### `impl PartialOrd for Span`

- <span id="span-partial-cmp"></span>`fn partial_cmp(&self, other: &Span) -> Option<Ordering>` — [`Span`](#span)

##### `impl StructuralPartialEq for Span`

### `Position`

```rust
struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
```

A single position in a regular expression.

A position encodes one half of a span, and include the byte offset, line
number and column number.

#### Fields

- **`offset`**: `usize`

  The absolute offset of this position, starting at `0` from the
  beginning of the regular expression pattern string.

- **`line`**: `usize`

  The line number, starting at `1`.

- **`column`**: `usize`

  The approximate column number, starting at `1`.

#### Implementations

- <span id="position-new"></span>`fn new(offset: usize, line: usize, column: usize) -> Position` — [`Position`](#position)

#### Trait Implementations

##### `impl Clone for Position`

- <span id="position-clone"></span>`fn clone(&self) -> Position` — [`Position`](#position)

##### `impl Copy for Position`

##### `impl Debug for Position`

- <span id="position-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Position`

##### `impl Ord for Position`

- <span id="position-cmp"></span>`fn cmp(&self, other: &Position) -> Ordering` — [`Position`](#position)

##### `impl PartialEq for Position`

- <span id="position-eq"></span>`fn eq(&self, other: &Position) -> bool` — [`Position`](#position)

##### `impl PartialOrd for Position`

- <span id="position-partial-cmp"></span>`fn partial_cmp(&self, other: &Position) -> Option<Ordering>` — [`Position`](#position)

##### `impl StructuralPartialEq for Position`

### `WithComments`

```rust
struct WithComments {
    pub ast: Ast,
    pub comments: alloc::vec::Vec<Comment>,
}
```

An abstract syntax tree for a singular expression along with comments
found.

Comments are not stored in the tree itself to avoid complexity. Each
comment contains a span of precisely where it occurred in the original
regular expression.

#### Fields

- **`ast`**: `Ast`

  The actual ast.

- **`comments`**: `alloc::vec::Vec<Comment>`

  All comments found in the original regular expression.

#### Trait Implementations

##### `impl Clone for WithComments`

- <span id="withcomments-clone"></span>`fn clone(&self) -> WithComments` — [`WithComments`](#withcomments)

##### `impl Debug for WithComments`

- <span id="withcomments-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WithComments`

##### `impl PartialEq for WithComments`

- <span id="withcomments-eq"></span>`fn eq(&self, other: &WithComments) -> bool` — [`WithComments`](#withcomments)

##### `impl StructuralPartialEq for WithComments`

### `Comment`

```rust
struct Comment {
    pub span: Span,
    pub comment: alloc::string::String,
}
```

A comment from a regular expression with an associated span.

A regular expression can only contain comments when the `x` flag is
enabled.

#### Fields

- **`span`**: `Span`

  The span of this comment, including the beginning `#` and ending `\n`.

- **`comment`**: `alloc::string::String`

  The comment text, starting with the first character following the `#`
  and ending with the last character preceding the `\n`.

#### Trait Implementations

##### `impl Clone for Comment`

- <span id="comment-clone"></span>`fn clone(&self) -> Comment` — [`Comment`](#comment)

##### `impl Debug for Comment`

- <span id="comment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Comment`

##### `impl PartialEq for Comment`

- <span id="comment-eq"></span>`fn eq(&self, other: &Comment) -> bool` — [`Comment`](#comment)

##### `impl StructuralPartialEq for Comment`

### `Alternation`

```rust
struct Alternation {
    pub span: Span,
    pub asts: alloc::vec::Vec<Ast>,
}
```

An alternation of regular expressions.

#### Fields

- **`span`**: `Span`

  The span of this alternation.

- **`asts`**: `alloc::vec::Vec<Ast>`

  The alternate regular expressions.

#### Implementations

- <span id="alternation-into-ast"></span>`fn into_ast(self) -> Ast` — [`Ast`](#ast)

#### Trait Implementations

##### `impl Clone for Alternation`

- <span id="alternation-clone"></span>`fn clone(&self) -> Alternation` — [`Alternation`](#alternation)

##### `impl Debug for Alternation`

- <span id="alternation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Alternation`

##### `impl PartialEq for Alternation`

- <span id="alternation-eq"></span>`fn eq(&self, other: &Alternation) -> bool` — [`Alternation`](#alternation)

##### `impl StructuralPartialEq for Alternation`

### `Concat`

```rust
struct Concat {
    pub span: Span,
    pub asts: alloc::vec::Vec<Ast>,
}
```

A concatenation of regular expressions.

#### Fields

- **`span`**: `Span`

  The span of this concatenation.

- **`asts`**: `alloc::vec::Vec<Ast>`

  The concatenation regular expressions.

#### Implementations

- <span id="concat-into-ast"></span>`fn into_ast(self) -> Ast` — [`Ast`](#ast)

#### Trait Implementations

##### `impl Clone for Concat`

- <span id="concat-clone"></span>`fn clone(&self) -> Concat` — [`Concat`](#concat)

##### `impl Debug for Concat`

- <span id="concat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Concat`

##### `impl PartialEq for Concat`

- <span id="concat-eq"></span>`fn eq(&self, other: &Concat) -> bool` — [`Concat`](#concat)

##### `impl StructuralPartialEq for Concat`

### `Literal`

```rust
struct Literal {
    pub span: Span,
    pub kind: LiteralKind,
    pub c: char,
}
```

A single literal expression.

A literal corresponds to a single Unicode scalar value. Literals may be
represented in their literal form, e.g., `a` or in their escaped form,
e.g., `\x61`.

#### Fields

- **`span`**: `Span`

  The span of this literal.

- **`kind`**: `LiteralKind`

  The kind of this literal.

- **`c`**: `char`

  The Unicode scalar value corresponding to this literal.

#### Implementations

- <span id="literal-byte"></span>`fn byte(&self) -> Option<u8>`

#### Trait Implementations

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl Debug for Literal`

- <span id="literal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Literal`

##### `impl PartialEq for Literal`

- <span id="literal-eq"></span>`fn eq(&self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

### `ClassPerl`

```rust
struct ClassPerl {
    pub span: Span,
    pub kind: ClassPerlKind,
    pub negated: bool,
}
```

A Perl character class.

#### Fields

- **`span`**: `Span`

  The span of this class.

- **`kind`**: `ClassPerlKind`

  The kind of Perl class.

- **`negated`**: `bool`

  Whether the class is negated or not. e.g., `\d` is not negated but
  `\D` is.

#### Trait Implementations

##### `impl Clone for ClassPerl`

- <span id="classperl-clone"></span>`fn clone(&self) -> ClassPerl` — [`ClassPerl`](#classperl)

##### `impl Debug for ClassPerl`

- <span id="classperl-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassPerl`

##### `impl PartialEq for ClassPerl`

- <span id="classperl-eq"></span>`fn eq(&self, other: &ClassPerl) -> bool` — [`ClassPerl`](#classperl)

##### `impl StructuralPartialEq for ClassPerl`

### `ClassAscii`

```rust
struct ClassAscii {
    pub span: Span,
    pub kind: ClassAsciiKind,
    pub negated: bool,
}
```

An ASCII character class.

#### Fields

- **`span`**: `Span`

  The span of this class.

- **`kind`**: `ClassAsciiKind`

  The kind of ASCII class.

- **`negated`**: `bool`

  Whether the class is negated or not. e.g., `[[:alpha:]]` is not negated
  but `[[:^alpha:]]` is.

#### Trait Implementations

##### `impl Clone for ClassAscii`

- <span id="classascii-clone"></span>`fn clone(&self) -> ClassAscii` — [`ClassAscii`](#classascii)

##### `impl Debug for ClassAscii`

- <span id="classascii-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassAscii`

##### `impl PartialEq for ClassAscii`

- <span id="classascii-eq"></span>`fn eq(&self, other: &ClassAscii) -> bool` — [`ClassAscii`](#classascii)

##### `impl StructuralPartialEq for ClassAscii`

### `ClassUnicode`

```rust
struct ClassUnicode {
    pub span: Span,
    pub negated: bool,
    pub kind: ClassUnicodeKind,
}
```

A Unicode character class.

#### Fields

- **`span`**: `Span`

  The span of this class.

- **`negated`**: `bool`

  Whether this class is negated or not.
  
  Note: be careful when using this attribute. This specifically refers
  to whether the class is written as `\p` or `\P`, where the latter
  is `negated = true`. However, it also possible to write something like
  `\P{scx!=Katakana}` which is actually equivalent to
  `\p{scx=Katakana}` and is therefore not actually negated even though
  `negated = true` here. To test whether this class is truly negated
  or not, use the `is_negated` method.

- **`kind`**: `ClassUnicodeKind`

  The kind of Unicode class.

#### Implementations

- <span id="classunicode-is-negated"></span>`fn is_negated(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ClassUnicode`

- <span id="classunicode-clone"></span>`fn clone(&self) -> ClassUnicode` — [`ClassUnicode`](#classunicode)

##### `impl Debug for ClassUnicode`

- <span id="classunicode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicode`

##### `impl PartialEq for ClassUnicode`

- <span id="classunicode-eq"></span>`fn eq(&self, other: &ClassUnicode) -> bool` — [`ClassUnicode`](#classunicode)

##### `impl StructuralPartialEq for ClassUnicode`

### `ClassBracketed`

```rust
struct ClassBracketed {
    pub span: Span,
    pub negated: bool,
    pub kind: ClassSet,
}
```

A bracketed character class, e.g., `[a-z0-9]`.

#### Fields

- **`span`**: `Span`

  The span of this class.

- **`negated`**: `bool`

  Whether this class is negated or not. e.g., `[a]` is not negated but
  `[^a]` is.

- **`kind`**: `ClassSet`

  The type of this set. A set is either a normal union of things, e.g.,
  `[abc]` or a result of applying set operations, e.g., `[\pL--c]`.

#### Trait Implementations

##### `impl Clone for ClassBracketed`

- <span id="classbracketed-clone"></span>`fn clone(&self) -> ClassBracketed` — [`ClassBracketed`](#classbracketed)

##### `impl Debug for ClassBracketed`

- <span id="classbracketed-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassBracketed`

##### `impl PartialEq for ClassBracketed`

- <span id="classbracketed-eq"></span>`fn eq(&self, other: &ClassBracketed) -> bool` — [`ClassBracketed`](#classbracketed)

##### `impl StructuralPartialEq for ClassBracketed`

### `ClassSetRange`

```rust
struct ClassSetRange {
    pub span: Span,
    pub start: Literal,
    pub end: Literal,
}
```

A single character class range in a set.

#### Fields

- **`span`**: `Span`

  The span of this range.

- **`start`**: `Literal`

  The start of this range.

- **`end`**: `Literal`

  The end of this range.

#### Implementations

- <span id="classsetrange-is-valid"></span>`fn is_valid(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ClassSetRange`

- <span id="classsetrange-clone"></span>`fn clone(&self) -> ClassSetRange` — [`ClassSetRange`](#classsetrange)

##### `impl Debug for ClassSetRange`

- <span id="classsetrange-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetRange`

##### `impl PartialEq for ClassSetRange`

- <span id="classsetrange-eq"></span>`fn eq(&self, other: &ClassSetRange) -> bool` — [`ClassSetRange`](#classsetrange)

##### `impl StructuralPartialEq for ClassSetRange`

### `ClassSetUnion`

```rust
struct ClassSetUnion {
    pub span: Span,
    pub items: alloc::vec::Vec<ClassSetItem>,
}
```

A union of items inside a character class set.

#### Fields

- **`span`**: `Span`

  The span of the items in this operation. e.g., the `a-z0-9` in
  `[^a-z0-9]`

- **`items`**: `alloc::vec::Vec<ClassSetItem>`

  The sequence of items that make up this union.

#### Implementations

- <span id="classsetunion-push"></span>`fn push(&mut self, item: ClassSetItem)` — [`ClassSetItem`](#classsetitem)

- <span id="classsetunion-into-item"></span>`fn into_item(self) -> ClassSetItem` — [`ClassSetItem`](#classsetitem)

#### Trait Implementations

##### `impl Clone for ClassSetUnion`

- <span id="classsetunion-clone"></span>`fn clone(&self) -> ClassSetUnion` — [`ClassSetUnion`](#classsetunion)

##### `impl Debug for ClassSetUnion`

- <span id="classsetunion-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetUnion`

##### `impl PartialEq for ClassSetUnion`

- <span id="classsetunion-eq"></span>`fn eq(&self, other: &ClassSetUnion) -> bool` — [`ClassSetUnion`](#classsetunion)

##### `impl StructuralPartialEq for ClassSetUnion`

### `ClassSetBinaryOp`

```rust
struct ClassSetBinaryOp {
    pub span: Span,
    pub kind: ClassSetBinaryOpKind,
    pub lhs: alloc::boxed::Box<ClassSet>,
    pub rhs: alloc::boxed::Box<ClassSet>,
}
```

A Unicode character class set operation.

#### Fields

- **`span`**: `Span`

  The span of this operation. e.g., the `a-z--[h-p]` in `[a-z--h-p]`.

- **`kind`**: `ClassSetBinaryOpKind`

  The type of this set operation.

- **`lhs`**: `alloc::boxed::Box<ClassSet>`

  The left hand side of the operation.

- **`rhs`**: `alloc::boxed::Box<ClassSet>`

  The right hand side of the operation.

#### Trait Implementations

##### `impl Clone for ClassSetBinaryOp`

- <span id="classsetbinaryop-clone"></span>`fn clone(&self) -> ClassSetBinaryOp` — [`ClassSetBinaryOp`](#classsetbinaryop)

##### `impl Debug for ClassSetBinaryOp`

- <span id="classsetbinaryop-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetBinaryOp`

##### `impl PartialEq for ClassSetBinaryOp`

- <span id="classsetbinaryop-eq"></span>`fn eq(&self, other: &ClassSetBinaryOp) -> bool` — [`ClassSetBinaryOp`](#classsetbinaryop)

##### `impl StructuralPartialEq for ClassSetBinaryOp`

### `Assertion`

```rust
struct Assertion {
    pub span: Span,
    pub kind: AssertionKind,
}
```

A single zero-width assertion.

#### Fields

- **`span`**: `Span`

  The span of this assertion.

- **`kind`**: `AssertionKind`

  The assertion kind, e.g., `\b` or `^`.

#### Trait Implementations

##### `impl Clone for Assertion`

- <span id="assertion-clone"></span>`fn clone(&self) -> Assertion` — [`Assertion`](#assertion)

##### `impl Debug for Assertion`

- <span id="assertion-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Assertion`

##### `impl PartialEq for Assertion`

- <span id="assertion-eq"></span>`fn eq(&self, other: &Assertion) -> bool` — [`Assertion`](#assertion)

##### `impl StructuralPartialEq for Assertion`

### `Repetition`

```rust
struct Repetition {
    pub span: Span,
    pub op: RepetitionOp,
    pub greedy: bool,
    pub ast: alloc::boxed::Box<Ast>,
}
```

A repetition operation applied to a regular expression.

#### Fields

- **`span`**: `Span`

  The span of this operation.

- **`op`**: `RepetitionOp`

  The actual operation.

- **`greedy`**: `bool`

  Whether this operation was applied greedily or not.

- **`ast`**: `alloc::boxed::Box<Ast>`

  The regular expression under repetition.

#### Trait Implementations

##### `impl Clone for Repetition`

- <span id="repetition-clone"></span>`fn clone(&self) -> Repetition` — [`Repetition`](#repetition)

##### `impl Debug for Repetition`

- <span id="repetition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Repetition`

##### `impl PartialEq for Repetition`

- <span id="repetition-eq"></span>`fn eq(&self, other: &Repetition) -> bool` — [`Repetition`](#repetition)

##### `impl StructuralPartialEq for Repetition`

### `RepetitionOp`

```rust
struct RepetitionOp {
    pub span: Span,
    pub kind: RepetitionKind,
}
```

The repetition operator itself.

#### Fields

- **`span`**: `Span`

  The span of this operator. This includes things like `+`, `*?` and
  `{m,n}`.

- **`kind`**: `RepetitionKind`

  The type of operation.

#### Trait Implementations

##### `impl Clone for RepetitionOp`

- <span id="repetitionop-clone"></span>`fn clone(&self) -> RepetitionOp` — [`RepetitionOp`](#repetitionop)

##### `impl Debug for RepetitionOp`

- <span id="repetitionop-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RepetitionOp`

##### `impl PartialEq for RepetitionOp`

- <span id="repetitionop-eq"></span>`fn eq(&self, other: &RepetitionOp) -> bool` — [`RepetitionOp`](#repetitionop)

##### `impl StructuralPartialEq for RepetitionOp`

### `Group`

```rust
struct Group {
    pub span: Span,
    pub kind: GroupKind,
    pub ast: alloc::boxed::Box<Ast>,
}
```

A grouped regular expression.

This includes both capturing and non-capturing groups. This does **not**
include flag-only groups like `(?is)`, but does contain any group that
contains a sub-expression, e.g., `(a)`, `(?P<name>a)`, `(?:a)` and
`(?is:a)`.

#### Fields

- **`span`**: `Span`

  The span of this group.

- **`kind`**: `GroupKind`

  The kind of this group.

- **`ast`**: `alloc::boxed::Box<Ast>`

  The regular expression in this group.

#### Implementations

- <span id="group-flags"></span>`fn flags(&self) -> Option<&Flags>` — [`Flags`](#flags)

- <span id="group-is-capturing"></span>`fn is_capturing(&self) -> bool`

- <span id="group-capture-index"></span>`fn capture_index(&self) -> Option<u32>`

#### Trait Implementations

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl Debug for Group`

- <span id="group-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Group`

##### `impl PartialEq for Group`

- <span id="group-eq"></span>`fn eq(&self, other: &Group) -> bool` — [`Group`](#group)

##### `impl StructuralPartialEq for Group`

### `CaptureName`

```rust
struct CaptureName {
    pub span: Span,
    pub name: alloc::string::String,
    pub index: u32,
}
```

A capture name.

This corresponds to the name itself between the angle brackets in, e.g.,
`(?P<foo>expr)`.

#### Fields

- **`span`**: `Span`

  The span of this capture name.

- **`name`**: `alloc::string::String`

  The capture name.

- **`index`**: `u32`

  The capture index.

#### Trait Implementations

##### `impl Clone for CaptureName`

- <span id="capturename-clone"></span>`fn clone(&self) -> CaptureName` — [`CaptureName`](#capturename)

##### `impl Debug for CaptureName`

- <span id="capturename-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CaptureName`

##### `impl PartialEq for CaptureName`

- <span id="capturename-eq"></span>`fn eq(&self, other: &CaptureName) -> bool` — [`CaptureName`](#capturename)

##### `impl StructuralPartialEq for CaptureName`

### `SetFlags`

```rust
struct SetFlags {
    pub span: Span,
    pub flags: Flags,
}
```

A group of flags that is not applied to a particular regular expression.

#### Fields

- **`span`**: `Span`

  The span of these flags, including the grouping parentheses.

- **`flags`**: `Flags`

  The actual sequence of flags.

#### Trait Implementations

##### `impl Clone for SetFlags`

- <span id="setflags-clone"></span>`fn clone(&self) -> SetFlags` — [`SetFlags`](#setflags)

##### `impl Debug for SetFlags`

- <span id="setflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SetFlags`

##### `impl PartialEq for SetFlags`

- <span id="setflags-eq"></span>`fn eq(&self, other: &SetFlags) -> bool` — [`SetFlags`](#setflags)

##### `impl StructuralPartialEq for SetFlags`

### `Flags`

```rust
struct Flags {
    pub span: Span,
    pub items: alloc::vec::Vec<FlagsItem>,
}
```

A group of flags.

This corresponds only to the sequence of flags themselves, e.g., `is-u`.

#### Fields

- **`span`**: `Span`

  The span of this group of flags.

- **`items`**: `alloc::vec::Vec<FlagsItem>`

  A sequence of flag items. Each item is either a flag or a negation
  operator.

#### Implementations

- <span id="flags-add-item"></span>`fn add_item(&mut self, item: FlagsItem) -> Option<usize>` — [`FlagsItem`](#flagsitem)

- <span id="flags-flag-state"></span>`fn flag_state(&self, flag: Flag) -> Option<bool>` — [`Flag`](#flag)

#### Trait Implementations

##### `impl Clone for Flags`

- <span id="flags-clone"></span>`fn clone(&self) -> Flags` — [`Flags`](#flags)

##### `impl Debug for Flags`

- <span id="flags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Flags`

##### `impl PartialEq for Flags`

- <span id="flags-eq"></span>`fn eq(&self, other: &Flags) -> bool` — [`Flags`](#flags)

##### `impl StructuralPartialEq for Flags`

### `FlagsItem`

```rust
struct FlagsItem {
    pub span: Span,
    pub kind: FlagsItemKind,
}
```

A single item in a group of flags.

#### Fields

- **`span`**: `Span`

  The span of this item.

- **`kind`**: `FlagsItemKind`

  The kind of this item.

#### Trait Implementations

##### `impl Clone for FlagsItem`

- <span id="flagsitem-clone"></span>`fn clone(&self) -> FlagsItem` — [`FlagsItem`](#flagsitem)

##### `impl Debug for FlagsItem`

- <span id="flagsitem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlagsItem`

##### `impl PartialEq for FlagsItem`

- <span id="flagsitem-eq"></span>`fn eq(&self, other: &FlagsItem) -> bool` — [`FlagsItem`](#flagsitem)

##### `impl StructuralPartialEq for FlagsItem`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    CaptureLimitExceeded,
    ClassEscapeInvalid,
    ClassRangeInvalid,
    ClassRangeLiteral,
    ClassUnclosed,
    DecimalEmpty,
    DecimalInvalid,
    EscapeHexEmpty,
    EscapeHexInvalid,
    EscapeHexInvalidDigit,
    EscapeUnexpectedEof,
    EscapeUnrecognized,
    FlagDanglingNegation,
    FlagDuplicate {
        original: Span,
    },
    FlagRepeatedNegation {
        original: Span,
    },
    FlagUnexpectedEof,
    FlagUnrecognized,
    GroupNameDuplicate {
        original: Span,
    },
    GroupNameEmpty,
    GroupNameInvalid,
    GroupNameUnexpectedEof,
    GroupUnclosed,
    GroupUnopened,
    NestLimitExceeded(u32),
    RepetitionCountInvalid,
    RepetitionCountDecimalEmpty,
    RepetitionCountUnclosed,
    RepetitionMissing,
    SpecialWordBoundaryUnclosed,
    SpecialWordBoundaryUnrecognized,
    SpecialWordOrRepetitionUnexpectedEof,
    UnicodeClassInvalid,
    UnsupportedBackreference,
    UnsupportedLookAround,
}
```

The type of an error that occurred while building an AST.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

#### Variants

- **`CaptureLimitExceeded`**

  The capturing group limit was exceeded.
  
  Note that this represents a limit on the total number of capturing
  groups in a regex and not necessarily the number of nested capturing
  groups. That is, the nest limit can be low and it is still possible for
  this error to occur.

- **`ClassEscapeInvalid`**

  An invalid escape sequence was found in a character class set.

- **`ClassRangeInvalid`**

  An invalid character class range was found. An invalid range is any
  range where the start is greater than the end.

- **`ClassRangeLiteral`**

  An invalid range boundary was found in a character class. Range
  boundaries must be a single literal codepoint, but this error indicates
  that something else was found, such as a nested class.

- **`ClassUnclosed`**

  An opening `` was found with no corresponding closing ``.

- **`DecimalEmpty`**

  Note that this error variant is no longer used. Namely, a decimal
  number can only appear as a repetition quantifier. When the number
  in a repetition quantifier is empty, then it gets its own specialized
  error, `RepetitionCountDecimalEmpty`.

- **`DecimalInvalid`**

  An invalid decimal number was given where one was expected.

- **`EscapeHexEmpty`**

  A bracketed hex literal was empty.

- **`EscapeHexInvalid`**

  A bracketed hex literal did not correspond to a Unicode scalar value.

- **`EscapeHexInvalidDigit`**

  An invalid hexadecimal digit was found.

- **`EscapeUnexpectedEof`**

  EOF was found before an escape sequence was completed.

- **`EscapeUnrecognized`**

  An unrecognized escape sequence.

- **`FlagDanglingNegation`**

  A dangling negation was used when setting flags, e.g., `i-`.

- **`FlagDuplicate`**

  A flag was used twice, e.g., `i-i`.

- **`FlagRepeatedNegation`**

  The negation operator was used twice, e.g., `-i-s`.

- **`FlagUnexpectedEof`**

  Expected a flag but got EOF, e.g., `(?`.

- **`FlagUnrecognized`**

  Unrecognized flag, e.g., `a`.

- **`GroupNameDuplicate`**

  A duplicate capture name was found.

- **`GroupNameEmpty`**

  A capture group name is empty, e.g., `(?P<>abc)`.

- **`GroupNameInvalid`**

  An invalid character was seen for a capture group name. This includes
  errors where the first character is a digit (even though subsequent
  characters are allowed to be digits).

- **`GroupNameUnexpectedEof`**

  A closing `>` could not be found for a capture group name.

- **`GroupUnclosed`**

  An unclosed group, e.g., `(ab`.
  
  The span of this error corresponds to the unclosed parenthesis.

- **`GroupUnopened`**

  An unopened group, e.g., `ab)`.

- **`NestLimitExceeded`**

  The nest limit was exceeded. The limit stored here is the limit
  configured in the parser.

- **`RepetitionCountInvalid`**

  The range provided in a counted repetition operator is invalid. The
  range is invalid if the start is greater than the end.

- **`RepetitionCountDecimalEmpty`**

  An opening `{` was not followed by a valid decimal value.
  For example, `x{}` or `x{]}` would fail.

- **`RepetitionCountUnclosed`**

  An opening `{` was found with no corresponding closing `}`.

- **`RepetitionMissing`**

  A repetition operator was applied to a missing sub-expression. This
  occurs, for example, in the regex consisting of just a `*` or even
  `(?i)*`. It is, however, possible to create a repetition operating on
  an empty sub-expression. For example, `()*` is still considered valid.

- **`SpecialWordBoundaryUnclosed`**

  The special word boundary syntax, `\b{something}`, was used, but
  either EOF without `}` was seen, or an invalid character in the
  braces was seen.

- **`SpecialWordBoundaryUnrecognized`**

  The special word boundary syntax, `\b{something}`, was used, but
  `something` was not recognized as a valid word boundary kind.

- **`SpecialWordOrRepetitionUnexpectedEof`**

  The syntax `\b{` was observed, but afterwards the end of the pattern
  was observed without being able to tell whether it was meant to be a
  bounded repetition on the `\b` or the beginning of a special word
  boundary assertion.

- **`UnicodeClassInvalid`**

  The Unicode class is not valid. This typically occurs when a `\p` is
  followed by something other than a `{`.

- **`UnsupportedBackreference`**

  When octal support is disabled, this error is produced when an octal
  escape is used. The octal escape is assumed to be an invocation of
  a backreference, which is the common case.

- **`UnsupportedLookAround`**

  When syntax similar to PCRE's look-around is used, this error is
  returned. Some example syntaxes that are rejected include, but are
  not necessarily limited to, `(?=re)`, `(?!re)`, `(?<=re)` and
  `(?<!re)`. Note that all of these syntaxes are otherwise invalid; this
  error is used to improve the user experience.

#### Trait Implementations

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl<T> ToString for ErrorKind`

- <span id="errorkind-to-string"></span>`fn to_string(&self) -> String`

### `Ast`

```rust
enum Ast {
    Empty(alloc::boxed::Box<Span>),
    Flags(alloc::boxed::Box<SetFlags>),
    Literal(alloc::boxed::Box<Literal>),
    Dot(alloc::boxed::Box<Span>),
    Assertion(alloc::boxed::Box<Assertion>),
    ClassUnicode(alloc::boxed::Box<ClassUnicode>),
    ClassPerl(alloc::boxed::Box<ClassPerl>),
    ClassBracketed(alloc::boxed::Box<ClassBracketed>),
    Repetition(alloc::boxed::Box<Repetition>),
    Group(alloc::boxed::Box<Group>),
    Alternation(alloc::boxed::Box<Alternation>),
    Concat(alloc::boxed::Box<Concat>),
}
```

An abstract syntax tree for a single regular expression.

An `Ast`'s `fmt::Display` implementation uses constant stack space and heap
space proportional to the size of the `Ast`.

This type defines its own destructor that uses constant stack space and
heap space proportional to the size of the `Ast`.

#### Variants

- **`Empty`**

  An empty regex that matches everything.

- **`Flags`**

  A set of flags, e.g., `(?is)`.

- **`Literal`**

  A single character literal, which includes escape sequences.

- **`Dot`**

  The "any character" class.

- **`Assertion`**

  A single zero-width assertion.

- **`ClassUnicode`**

  A single Unicode character class, e.g., `\pL` or `\p{Greek}`.

- **`ClassPerl`**

  A single perl character class, e.g., `\d` or `\W`.

- **`ClassBracketed`**

  A single bracketed character class set, which may contain zero or more
  character ranges and/or zero or more nested classes. e.g.,
  `[a-zA-Z\pL]`.

- **`Repetition`**

  A repetition operator applied to an arbitrary regular expression.

- **`Group`**

  A grouped regular expression.

- **`Alternation`**

  An alternation of regular expressions.

- **`Concat`**

  A concatenation of regular expressions.

#### Implementations

- <span id="ast-empty"></span>`fn empty(span: Span) -> Ast` — [`Span`](#span), [`Ast`](#ast)

- <span id="ast-flags"></span>`fn flags(e: SetFlags) -> Ast` — [`SetFlags`](#setflags), [`Ast`](#ast)

- <span id="ast-literal"></span>`fn literal(e: Literal) -> Ast` — [`Literal`](#literal), [`Ast`](#ast)

- <span id="ast-dot"></span>`fn dot(span: Span) -> Ast` — [`Span`](#span), [`Ast`](#ast)

- <span id="ast-assertion"></span>`fn assertion(e: Assertion) -> Ast` — [`Assertion`](#assertion), [`Ast`](#ast)

- <span id="ast-class-unicode"></span>`fn class_unicode(e: ClassUnicode) -> Ast` — [`ClassUnicode`](#classunicode), [`Ast`](#ast)

- <span id="ast-class-perl"></span>`fn class_perl(e: ClassPerl) -> Ast` — [`ClassPerl`](#classperl), [`Ast`](#ast)

- <span id="ast-class-bracketed"></span>`fn class_bracketed(e: ClassBracketed) -> Ast` — [`ClassBracketed`](#classbracketed), [`Ast`](#ast)

- <span id="ast-repetition"></span>`fn repetition(e: Repetition) -> Ast` — [`Repetition`](#repetition), [`Ast`](#ast)

- <span id="ast-group"></span>`fn group(e: Group) -> Ast` — [`Group`](#group), [`Ast`](#ast)

- <span id="ast-alternation"></span>`fn alternation(e: Alternation) -> Ast` — [`Alternation`](#alternation), [`Ast`](#ast)

- <span id="ast-concat"></span>`fn concat(e: Concat) -> Ast` — [`Concat`](#concat), [`Ast`](#ast)

- <span id="ast-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

- <span id="ast-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="ast-has-subexprs"></span>`fn has_subexprs(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Ast`

- <span id="ast-clone"></span>`fn clone(&self) -> Ast` — [`Ast`](#ast)

##### `impl Debug for Ast`

- <span id="ast-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ast`

- <span id="ast-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Drop for Ast`

- <span id="ast-drop"></span>`fn drop(&mut self)`

##### `impl Eq for Ast`

##### `impl PartialEq for Ast`

- <span id="ast-eq"></span>`fn eq(&self, other: &Ast) -> bool` — [`Ast`](#ast)

##### `impl StructuralPartialEq for Ast`

##### `impl<T> ToString for Ast`

- <span id="ast-to-string"></span>`fn to_string(&self) -> String`

### `LiteralKind`

```rust
enum LiteralKind {
    Verbatim,
    Meta,
    Superfluous,
    Octal,
    HexFixed(HexLiteralKind),
    HexBrace(HexLiteralKind),
    Special(SpecialLiteralKind),
}
```

The kind of a single literal expression.

#### Variants

- **`Verbatim`**

  The literal is written verbatim, e.g., `a` or `☃`.

- **`Meta`**

  The literal is written as an escape because it is otherwise a special
  regex meta character, e.g., `\*` or `\[`.

- **`Superfluous`**

  The literal is written as an escape despite the fact that the escape is
  unnecessary, e.g., `\%` or `\/`.

- **`Octal`**

  The literal is written as an octal escape, e.g., `\141`.

- **`HexFixed`**

  The literal is written as a hex code with a fixed number of digits
  depending on the type of the escape, e.g., `\x61` or `\u0061` or
  `\U00000061`.

- **`HexBrace`**

  The literal is written as a hex code with a bracketed number of
  digits. The only restriction is that the bracketed hex code must refer
  to a valid Unicode scalar value.

- **`Special`**

  The literal is written as a specially recognized escape, e.g., `\f`
  or `\n`.

#### Trait Implementations

##### `impl Clone for LiteralKind`

- <span id="literalkind-clone"></span>`fn clone(&self) -> LiteralKind` — [`LiteralKind`](#literalkind)

##### `impl Debug for LiteralKind`

- <span id="literalkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LiteralKind`

##### `impl PartialEq for LiteralKind`

- <span id="literalkind-eq"></span>`fn eq(&self, other: &LiteralKind) -> bool` — [`LiteralKind`](#literalkind)

##### `impl StructuralPartialEq for LiteralKind`

### `SpecialLiteralKind`

```rust
enum SpecialLiteralKind {
    Bell,
    FormFeed,
    Tab,
    LineFeed,
    CarriageReturn,
    VerticalTab,
    Space,
}
```

The type of a special literal.

A special literal is a special escape sequence recognized by the regex
parser, e.g., `\f` or `\n`.

#### Variants

- **`Bell`**

  Bell, spelled `\a` (`\x07`).

- **`FormFeed`**

  Form feed, spelled `\f` (`\x0C`).

- **`Tab`**

  Tab, spelled `\t` (`\x09`).

- **`LineFeed`**

  Line feed, spelled `\n` (`\x0A`).

- **`CarriageReturn`**

  Carriage return, spelled `\r` (`\x0D`).

- **`VerticalTab`**

  Vertical tab, spelled `\v` (`\x0B`).

- **`Space`**

  Space, spelled `\ ` (`\x20`). Note that this can only appear when
  parsing in verbose mode.

#### Trait Implementations

##### `impl Clone for SpecialLiteralKind`

- <span id="specialliteralkind-clone"></span>`fn clone(&self) -> SpecialLiteralKind` — [`SpecialLiteralKind`](#specialliteralkind)

##### `impl Debug for SpecialLiteralKind`

- <span id="specialliteralkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SpecialLiteralKind`

##### `impl PartialEq for SpecialLiteralKind`

- <span id="specialliteralkind-eq"></span>`fn eq(&self, other: &SpecialLiteralKind) -> bool` — [`SpecialLiteralKind`](#specialliteralkind)

##### `impl StructuralPartialEq for SpecialLiteralKind`

### `HexLiteralKind`

```rust
enum HexLiteralKind {
    X,
    UnicodeShort,
    UnicodeLong,
}
```

The type of a Unicode hex literal.

Note that all variants behave the same when used with brackets. They only
differ when used without brackets in the number of hex digits that must
follow.

#### Variants

- **`X`**

  A `\x` prefix. When used without brackets, this form is limited to
  two digits.

- **`UnicodeShort`**

  A `\u` prefix. When used without brackets, this form is limited to
  four digits.

- **`UnicodeLong`**

  A `\U` prefix. When used without brackets, this form is limited to
  eight digits.

#### Implementations

- <span id="hexliteralkind-digits"></span>`fn digits(&self) -> u32`

#### Trait Implementations

##### `impl Clone for HexLiteralKind`

- <span id="hexliteralkind-clone"></span>`fn clone(&self) -> HexLiteralKind` — [`HexLiteralKind`](#hexliteralkind)

##### `impl Debug for HexLiteralKind`

- <span id="hexliteralkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HexLiteralKind`

##### `impl PartialEq for HexLiteralKind`

- <span id="hexliteralkind-eq"></span>`fn eq(&self, other: &HexLiteralKind) -> bool` — [`HexLiteralKind`](#hexliteralkind)

##### `impl StructuralPartialEq for HexLiteralKind`

### `ClassPerlKind`

```rust
enum ClassPerlKind {
    Digit,
    Space,
    Word,
}
```

The available Perl character classes.

#### Variants

- **`Digit`**

  Decimal numbers.

- **`Space`**

  Whitespace.

- **`Word`**

  Word characters.

#### Trait Implementations

##### `impl Clone for ClassPerlKind`

- <span id="classperlkind-clone"></span>`fn clone(&self) -> ClassPerlKind` — [`ClassPerlKind`](#classperlkind)

##### `impl Debug for ClassPerlKind`

- <span id="classperlkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassPerlKind`

##### `impl PartialEq for ClassPerlKind`

- <span id="classperlkind-eq"></span>`fn eq(&self, other: &ClassPerlKind) -> bool` — [`ClassPerlKind`](#classperlkind)

##### `impl StructuralPartialEq for ClassPerlKind`

### `ClassAsciiKind`

```rust
enum ClassAsciiKind {
    Alnum,
    Alpha,
    Ascii,
    Blank,
    Cntrl,
    Digit,
    Graph,
    Lower,
    Print,
    Punct,
    Space,
    Upper,
    Word,
    Xdigit,
}
```

The available ASCII character classes.

#### Variants

- **`Alnum`**

  `[0-9A-Za-z]`

- **`Alpha`**

  `[A-Za-z]`

- **`Ascii`**

  `[\x00-\x7F]`

- **`Blank`**

  `[ \t]`

- **`Cntrl`**

  `[\x00-\x1F\x7F]`

- **`Digit`**

  `[0-9]`

- **`Graph`**

  `[!-~]`

- **`Lower`**

  `[a-z]`

- **`Print`**

  `[ -~]`

- **`Punct`**

  ``[!-/:-@\[-`{-~]``

- **`Space`**

  `[\t\n\v\f\r ]`

- **`Upper`**

  `[A-Z]`

- **`Word`**

  `[0-9A-Za-z_]`

- **`Xdigit`**

  `[0-9A-Fa-f]`

#### Implementations

- <span id="classasciikind-from-name"></span>`fn from_name(name: &str) -> Option<ClassAsciiKind>` — [`ClassAsciiKind`](#classasciikind)

#### Trait Implementations

##### `impl Clone for ClassAsciiKind`

- <span id="classasciikind-clone"></span>`fn clone(&self) -> ClassAsciiKind` — [`ClassAsciiKind`](#classasciikind)

##### `impl Debug for ClassAsciiKind`

- <span id="classasciikind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassAsciiKind`

##### `impl PartialEq for ClassAsciiKind`

- <span id="classasciikind-eq"></span>`fn eq(&self, other: &ClassAsciiKind) -> bool` — [`ClassAsciiKind`](#classasciikind)

##### `impl StructuralPartialEq for ClassAsciiKind`

### `ClassUnicodeKind`

```rust
enum ClassUnicodeKind {
    OneLetter(char),
    Named(alloc::string::String),
    NamedValue {
        op: ClassUnicodeOpKind,
        name: alloc::string::String,
        value: alloc::string::String,
    },
}
```

The available forms of Unicode character classes.

#### Variants

- **`OneLetter`**

  A one letter abbreviated class, e.g., `\pN`.

- **`Named`**

  A binary property, general category or script. The string may be
  empty.

- **`NamedValue`**

  A property name and an associated value.

#### Trait Implementations

##### `impl Clone for ClassUnicodeKind`

- <span id="classunicodekind-clone"></span>`fn clone(&self) -> ClassUnicodeKind` — [`ClassUnicodeKind`](#classunicodekind)

##### `impl Debug for ClassUnicodeKind`

- <span id="classunicodekind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicodeKind`

##### `impl PartialEq for ClassUnicodeKind`

- <span id="classunicodekind-eq"></span>`fn eq(&self, other: &ClassUnicodeKind) -> bool` — [`ClassUnicodeKind`](#classunicodekind)

##### `impl StructuralPartialEq for ClassUnicodeKind`

### `ClassUnicodeOpKind`

```rust
enum ClassUnicodeOpKind {
    Equal,
    Colon,
    NotEqual,
}
```

The type of op used in a Unicode character class.

#### Variants

- **`Equal`**

  A property set to a specific value, e.g., `\p{scx=Katakana}`.

- **`Colon`**

  A property set to a specific value using a colon, e.g.,
  `\p{scx:Katakana}`.

- **`NotEqual`**

  A property that isn't a particular value, e.g., `\p{scx!=Katakana}`.

#### Implementations

- <span id="classunicodeopkind-is-equal"></span>`fn is_equal(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ClassUnicodeOpKind`

- <span id="classunicodeopkind-clone"></span>`fn clone(&self) -> ClassUnicodeOpKind` — [`ClassUnicodeOpKind`](#classunicodeopkind)

##### `impl Debug for ClassUnicodeOpKind`

- <span id="classunicodeopkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicodeOpKind`

##### `impl PartialEq for ClassUnicodeOpKind`

- <span id="classunicodeopkind-eq"></span>`fn eq(&self, other: &ClassUnicodeOpKind) -> bool` — [`ClassUnicodeOpKind`](#classunicodeopkind)

##### `impl StructuralPartialEq for ClassUnicodeOpKind`

### `ClassSet`

```rust
enum ClassSet {
    Item(ClassSetItem),
    BinaryOp(ClassSetBinaryOp),
}
```

A character class set.

This type corresponds to the internal structure of a bracketed character
class. That is, every bracketed character is one of two types: a union of
items (literals, ranges, other bracketed classes) or a tree of binary set
operations.

#### Variants

- **`Item`**

  An item, which can be a single literal, range, nested character class
  or a union of items.

- **`BinaryOp`**

  A single binary operation (i.e., &&, -- or ~~).

#### Implementations

- <span id="classset-union"></span>`fn union(ast: ClassSetUnion) -> ClassSet` — [`ClassSetUnion`](#classsetunion), [`ClassSet`](#classset)

- <span id="classset-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

- <span id="classset-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ClassSet`

- <span id="classset-clone"></span>`fn clone(&self) -> ClassSet` — [`ClassSet`](#classset)

##### `impl Debug for ClassSet`

- <span id="classset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ClassSet`

- <span id="classset-drop"></span>`fn drop(&mut self)`

##### `impl Eq for ClassSet`

##### `impl PartialEq for ClassSet`

- <span id="classset-eq"></span>`fn eq(&self, other: &ClassSet) -> bool` — [`ClassSet`](#classset)

##### `impl StructuralPartialEq for ClassSet`

### `ClassSetItem`

```rust
enum ClassSetItem {
    Empty(Span),
    Literal(Literal),
    Range(ClassSetRange),
    Ascii(ClassAscii),
    Unicode(ClassUnicode),
    Perl(ClassPerl),
    Bracketed(alloc::boxed::Box<ClassBracketed>),
    Union(ClassSetUnion),
}
```

A single component of a character class set.

#### Variants

- **`Empty`**

  An empty item.
  
  Note that a bracketed character class cannot contain a single empty
  item. Empty items can appear when using one of the binary operators.
  For example, `[&&]` is the intersection of two empty classes.

- **`Literal`**

  A single literal.

- **`Range`**

  A range between two literals.

- **`Ascii`**

  An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`.

- **`Unicode`**

  A Unicode character class, e.g., `\pL` or `\p{Greek}`.

- **`Perl`**

  A perl character class, e.g., `\d` or `\W`.

- **`Bracketed`**

  A bracketed character class set, which may contain zero or more
  character ranges and/or zero or more nested classes. e.g.,
  `[a-zA-Z\pL]`.

- **`Union`**

  A union of items.

#### Implementations

- <span id="classsetitem-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

#### Trait Implementations

##### `impl Clone for ClassSetItem`

- <span id="classsetitem-clone"></span>`fn clone(&self) -> ClassSetItem` — [`ClassSetItem`](#classsetitem)

##### `impl Debug for ClassSetItem`

- <span id="classsetitem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetItem`

##### `impl PartialEq for ClassSetItem`

- <span id="classsetitem-eq"></span>`fn eq(&self, other: &ClassSetItem) -> bool` — [`ClassSetItem`](#classsetitem)

##### `impl StructuralPartialEq for ClassSetItem`

### `ClassSetBinaryOpKind`

```rust
enum ClassSetBinaryOpKind {
    Intersection,
    Difference,
    SymmetricDifference,
}
```

The type of a Unicode character class set operation.

Note that this doesn't explicitly represent union since there is no
explicit union operator. Concatenation inside a character class corresponds
to the union operation.

#### Variants

- **`Intersection`**

  The intersection of two sets, e.g., `\pN&&[a-z]`.

- **`Difference`**

  The difference of two sets, e.g., `\pN--[0-9]`.

- **`SymmetricDifference`**

  The symmetric difference of two sets. The symmetric difference is the
  set of elements belonging to one but not both sets.
  e.g., `[\pL~~[:ascii:]]`.

#### Trait Implementations

##### `impl Clone for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-clone"></span>`fn clone(&self) -> ClassSetBinaryOpKind` — [`ClassSetBinaryOpKind`](#classsetbinaryopkind)

##### `impl Copy for ClassSetBinaryOpKind`

##### `impl Debug for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetBinaryOpKind`

##### `impl PartialEq for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-eq"></span>`fn eq(&self, other: &ClassSetBinaryOpKind) -> bool` — [`ClassSetBinaryOpKind`](#classsetbinaryopkind)

##### `impl StructuralPartialEq for ClassSetBinaryOpKind`

### `AssertionKind`

```rust
enum AssertionKind {
    StartLine,
    EndLine,
    StartText,
    EndText,
    WordBoundary,
    NotWordBoundary,
    WordBoundaryStart,
    WordBoundaryEnd,
    WordBoundaryStartAngle,
    WordBoundaryEndAngle,
    WordBoundaryStartHalf,
    WordBoundaryEndHalf,
}
```

An assertion kind.

#### Variants

- **`StartLine`**

  `^`

- **`EndLine`**

  `$`

- **`StartText`**

  `\A`

- **`EndText`**

  `\z`

- **`WordBoundary`**

  `\b`

- **`NotWordBoundary`**

  `\B`

- **`WordBoundaryStart`**

  `\b{start}`

- **`WordBoundaryEnd`**

  `\b{end}`

- **`WordBoundaryStartAngle`**

  `\<` (alias for `\b{start}`)

- **`WordBoundaryEndAngle`**

  `\>` (alias for `\b{end}`)

- **`WordBoundaryStartHalf`**

  `\b{start-half}`

- **`WordBoundaryEndHalf`**

  `\b{end-half}`

#### Trait Implementations

##### `impl Clone for AssertionKind`

- <span id="assertionkind-clone"></span>`fn clone(&self) -> AssertionKind` — [`AssertionKind`](#assertionkind)

##### `impl Debug for AssertionKind`

- <span id="assertionkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AssertionKind`

##### `impl PartialEq for AssertionKind`

- <span id="assertionkind-eq"></span>`fn eq(&self, other: &AssertionKind) -> bool` — [`AssertionKind`](#assertionkind)

##### `impl StructuralPartialEq for AssertionKind`

### `RepetitionKind`

```rust
enum RepetitionKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Range(RepetitionRange),
}
```

The kind of a repetition operator.

#### Variants

- **`ZeroOrOne`**

  `?`

- **`ZeroOrMore`**

  `*`

- **`OneOrMore`**

  `+`

- **`Range`**

  `{m,n}`

#### Trait Implementations

##### `impl Clone for RepetitionKind`

- <span id="repetitionkind-clone"></span>`fn clone(&self) -> RepetitionKind` — [`RepetitionKind`](#repetitionkind)

##### `impl Debug for RepetitionKind`

- <span id="repetitionkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RepetitionKind`

##### `impl PartialEq for RepetitionKind`

- <span id="repetitionkind-eq"></span>`fn eq(&self, other: &RepetitionKind) -> bool` — [`RepetitionKind`](#repetitionkind)

##### `impl StructuralPartialEq for RepetitionKind`

### `RepetitionRange`

```rust
enum RepetitionRange {
    Exactly(u32),
    AtLeast(u32),
    Bounded(u32, u32),
}
```

A range repetition operator.

#### Variants

- **`Exactly`**

  `{m}`

- **`AtLeast`**

  `{m,}`

- **`Bounded`**

  `{m,n}`

#### Implementations

- <span id="repetitionrange-is-valid"></span>`fn is_valid(&self) -> bool`

#### Trait Implementations

##### `impl Clone for RepetitionRange`

- <span id="repetitionrange-clone"></span>`fn clone(&self) -> RepetitionRange` — [`RepetitionRange`](#repetitionrange)

##### `impl Debug for RepetitionRange`

- <span id="repetitionrange-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RepetitionRange`

##### `impl PartialEq for RepetitionRange`

- <span id="repetitionrange-eq"></span>`fn eq(&self, other: &RepetitionRange) -> bool` — [`RepetitionRange`](#repetitionrange)

##### `impl StructuralPartialEq for RepetitionRange`

### `GroupKind`

```rust
enum GroupKind {
    CaptureIndex(u32),
    CaptureName {
        starts_with_p: bool,
        name: CaptureName,
    },
    NonCapturing(Flags),
}
```

The kind of a group.

#### Variants

- **`CaptureIndex`**

  `(a)`

- **`CaptureName`**

  `(?<name>a)` or `(?P<name>a)`

- **`NonCapturing`**

  `(?:a)` and `(?i:a)`

#### Trait Implementations

##### `impl Clone for GroupKind`

- <span id="groupkind-clone"></span>`fn clone(&self) -> GroupKind` — [`GroupKind`](#groupkind)

##### `impl Debug for GroupKind`

- <span id="groupkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for GroupKind`

##### `impl PartialEq for GroupKind`

- <span id="groupkind-eq"></span>`fn eq(&self, other: &GroupKind) -> bool` — [`GroupKind`](#groupkind)

##### `impl StructuralPartialEq for GroupKind`

### `FlagsItemKind`

```rust
enum FlagsItemKind {
    Negation,
    Flag(Flag),
}
```

The kind of an item in a group of flags.

#### Variants

- **`Negation`**

  A negation operator applied to all subsequent flags in the enclosing
  group.

- **`Flag`**

  A single flag in a group.

#### Implementations

- <span id="flagsitemkind-is-negation"></span>`fn is_negation(&self) -> bool`

#### Trait Implementations

##### `impl Clone for FlagsItemKind`

- <span id="flagsitemkind-clone"></span>`fn clone(&self) -> FlagsItemKind` — [`FlagsItemKind`](#flagsitemkind)

##### `impl Debug for FlagsItemKind`

- <span id="flagsitemkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlagsItemKind`

##### `impl PartialEq for FlagsItemKind`

- <span id="flagsitemkind-eq"></span>`fn eq(&self, other: &FlagsItemKind) -> bool` — [`FlagsItemKind`](#flagsitemkind)

##### `impl StructuralPartialEq for FlagsItemKind`

### `Flag`

```rust
enum Flag {
    CaseInsensitive,
    MultiLine,
    DotMatchesNewLine,
    SwapGreed,
    Unicode,
    CRLF,
    IgnoreWhitespace,
}
```

A single flag.

#### Variants

- **`CaseInsensitive`**

  `i`

- **`MultiLine`**

  `m`

- **`DotMatchesNewLine`**

  `s`

- **`SwapGreed`**

  `U`

- **`Unicode`**

  `u`

- **`CRLF`**

  `R`

- **`IgnoreWhitespace`**

  `x`

#### Trait Implementations

##### `impl Clone for Flag`

- <span id="flag-clone"></span>`fn clone(&self) -> Flag` — [`Flag`](#flag)

##### `impl Copy for Flag`

##### `impl Debug for Flag`

- <span id="flag-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Flag`

##### `impl PartialEq for Flag`

- <span id="flag-eq"></span>`fn eq(&self, other: &Flag) -> bool` — [`Flag`](#flag)

##### `impl StructuralPartialEq for Flag`

## Traits

## Functions

