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
  - [`Visitor`](#visitor)
- [Functions](#functions)
  - [`visit`](#visit)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse`](#parse) | mod | This module provides a regular expression parser. |
| [`print`](#print) | mod | This module provides a regular expression printer for `Ast`. |
| [`visitor`](#visitor) | mod |  |
| [`Error`](#error) | struct | An error that occurred while parsing a regular expression into an abstract syntax tree. |
| [`Span`](#span) | struct | Span represents the position information of a single AST item. |
| [`Position`](#position) | struct | A single position in a regular expression. |
| [`WithComments`](#withcomments) | struct | An abstract syntax tree for a singular expression along with comments found. |
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
| [`Visitor`](#visitor) | trait |  |
| [`visit`](#visit) | fn |  |

## Modules

- [`parse`](parse/index.md) — This module provides a regular expression parser.
- [`print`](print/index.md) — This module provides a regular expression printer for `Ast`.
- [`visitor`](visitor/index.md)

## Structs

### `Error`

```rust
struct Error {
    kind: ErrorKind,
    pattern: alloc::string::String,
    span: Span,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:24-32`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L24-L32)*

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

  Return the type of this error.

- <span id="error-pattern"></span>`fn pattern(&self) -> &str`

  The original pattern string in which this error occurred.

  

  Every span reported by this error is reported in terms of this string.

- <span id="error-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

  Return the span at which this error occurred.

- <span id="error-auxiliary-span"></span>`fn auxiliary_span(&self) -> Option<&Span>` — [`Span`](#span)

  Return an auxiliary span. This span exists only for some errors that

  benefit from being able to point to two locations in the original

  regular expression. For example, "duplicate" errors will have the

  main error position set to the duplicate occurrence while its

  auxiliary span will be set to the initial occurrence.

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Span`

```rust
struct Span {
    pub start: Position,
    pub end: Position,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:319-324`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L319-L324)*

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

  Create a new span with the given positions.

- <span id="span-splat"></span>`fn splat(pos: Position) -> Span` — [`Position`](#position), [`Span`](#span)

  Create a new span using the given position as the start and end.

- <span id="span-with-start"></span>`fn with_start(self, pos: Position) -> Span` — [`Position`](#position), [`Span`](#span)

  Create a new span by replacing the starting the position with the one

  given.

- <span id="span-with-end"></span>`fn with_end(self, pos: Position) -> Span` — [`Position`](#position), [`Span`](#span)

  Create a new span by replacing the ending the position with the one

  given.

- <span id="span-is-one-line"></span>`fn is_one_line(&self) -> bool`

  Returns true if and only if this span occurs on a single line.

- <span id="span-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this span is empty. That is, it points to

  a single position in the concrete syntax of a regular expression.

#### Trait Implementations

##### `impl Any for Span`

- <span id="span-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Span`

- <span id="span-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Span`

- <span id="span-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](#span)

##### `impl CloneToUninit for Span`

- <span id="span-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl<T> From for Span`

- <span id="span-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Span`

- <span id="span-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Span`

- <span id="span-ord-cmp"></span>`fn cmp(&self, other: &Span) -> Ordering` — [`Span`](#span)

##### `impl PartialEq for Span`

- <span id="span-partialeq-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](#span)

##### `impl PartialOrd for Span`

- <span id="span-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Span) -> Option<Ordering>` — [`Span`](#span)

##### `impl StructuralPartialEq for Span`

##### `impl ToOwned for Span`

- <span id="span-toowned-type-owned"></span>`type Owned = T`

- <span id="span-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="span-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Span`

- <span id="span-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="span-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Span`

- <span id="span-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="span-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Position`

```rust
struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:350-358`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L350-L358)*

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

  Create a new position with the given information.

  

  `offset` is the absolute offset of the position, starting at `0` from

  the beginning of the regular expression pattern string.

  

  `line` is the line number, starting at `1`.

  

  `column` is the approximate column number, starting at `1`.

#### Trait Implementations

##### `impl Any for Position`

- <span id="position-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Position`

- <span id="position-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Position`

- <span id="position-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Position`

- <span id="position-clone"></span>`fn clone(&self) -> Position` — [`Position`](#position)

##### `impl CloneToUninit for Position`

- <span id="position-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Position`

##### `impl Debug for Position`

- <span id="position-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Position`

##### `impl<T> From for Position`

- <span id="position-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Position`

- <span id="position-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Position`

- <span id="position-ord-cmp"></span>`fn cmp(&self, other: &Position) -> Ordering` — [`Position`](#position)

##### `impl PartialEq for Position`

- <span id="position-partialeq-eq"></span>`fn eq(&self, other: &Position) -> bool` — [`Position`](#position)

##### `impl PartialOrd for Position`

- <span id="position-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Position) -> Option<Ordering>` — [`Position`](#position)

##### `impl StructuralPartialEq for Position`

##### `impl ToOwned for Position`

- <span id="position-toowned-type-owned"></span>`type Owned = T`

- <span id="position-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="position-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Position`

- <span id="position-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="position-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Position`

- <span id="position-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="position-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithComments`

```rust
struct WithComments {
    pub ast: Ast,
    pub comments: alloc::vec::Vec<Comment>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:439-444`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L439-L444)*

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

##### `impl Any for WithComments`

- <span id="withcomments-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithComments`

- <span id="withcomments-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithComments`

- <span id="withcomments-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WithComments`

- <span id="withcomments-clone"></span>`fn clone(&self) -> WithComments` — [`WithComments`](#withcomments)

##### `impl CloneToUninit for WithComments`

- <span id="withcomments-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WithComments`

- <span id="withcomments-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WithComments`

##### `impl<T> From for WithComments`

- <span id="withcomments-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithComments`

- <span id="withcomments-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WithComments`

- <span id="withcomments-partialeq-eq"></span>`fn eq(&self, other: &WithComments) -> bool` — [`WithComments`](#withcomments)

##### `impl StructuralPartialEq for WithComments`

##### `impl ToOwned for WithComments`

- <span id="withcomments-toowned-type-owned"></span>`type Owned = T`

- <span id="withcomments-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="withcomments-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WithComments`

- <span id="withcomments-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withcomments-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithComments`

- <span id="withcomments-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withcomments-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Comment`

```rust
struct Comment {
    pub span: Span,
    pub comment: alloc::string::String,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:452-458`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L452-L458)*

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

##### `impl Any for Comment`

- <span id="comment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Comment`

- <span id="comment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Comment`

- <span id="comment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Comment`

- <span id="comment-clone"></span>`fn clone(&self) -> Comment` — [`Comment`](#comment)

##### `impl CloneToUninit for Comment`

- <span id="comment-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Comment`

- <span id="comment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Comment`

##### `impl<T> From for Comment`

- <span id="comment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Comment`

- <span id="comment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Comment`

- <span id="comment-partialeq-eq"></span>`fn eq(&self, other: &Comment) -> bool` — [`Comment`](#comment)

##### `impl StructuralPartialEq for Comment`

##### `impl ToOwned for Comment`

- <span id="comment-toowned-type-owned"></span>`type Owned = T`

- <span id="comment-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="comment-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Comment`

- <span id="comment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Comment`

- <span id="comment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Alternation`

```rust
struct Alternation {
    pub span: Span,
    pub asts: alloc::vec::Vec<Ast>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:623-628`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L623-L628)*

An alternation of regular expressions.

#### Fields

- **`span`**: `Span`

  The span of this alternation.

- **`asts`**: `alloc::vec::Vec<Ast>`

  The alternate regular expressions.

#### Implementations

- <span id="alternation-into-ast"></span>`fn into_ast(self) -> Ast` — [`Ast`](#ast)

  Return this alternation as an AST.

  

  If this alternation contains zero ASTs, then `Ast::empty` is returned.

  If this alternation contains exactly 1 AST, then the corresponding AST

  is returned. Otherwise, `Ast::alternation` is returned.

#### Trait Implementations

##### `impl Any for Alternation`

- <span id="alternation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Alternation`

- <span id="alternation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Alternation`

- <span id="alternation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Alternation`

- <span id="alternation-clone"></span>`fn clone(&self) -> Alternation` — [`Alternation`](#alternation)

##### `impl CloneToUninit for Alternation`

- <span id="alternation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Alternation`

- <span id="alternation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Alternation`

##### `impl<T> From for Alternation`

- <span id="alternation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Alternation`

- <span id="alternation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Alternation`

- <span id="alternation-partialeq-eq"></span>`fn eq(&self, other: &Alternation) -> bool` — [`Alternation`](#alternation)

##### `impl StructuralPartialEq for Alternation`

##### `impl ToOwned for Alternation`

- <span id="alternation-toowned-type-owned"></span>`type Owned = T`

- <span id="alternation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="alternation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Alternation`

- <span id="alternation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="alternation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Alternation`

- <span id="alternation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="alternation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Concat`

```rust
struct Concat {
    pub span: Span,
    pub asts: alloc::vec::Vec<Ast>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:648-653`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L648-L653)*

A concatenation of regular expressions.

#### Fields

- **`span`**: `Span`

  The span of this concatenation.

- **`asts`**: `alloc::vec::Vec<Ast>`

  The concatenation regular expressions.

#### Implementations

- <span id="concat-into-ast"></span>`fn into_ast(self) -> Ast` — [`Ast`](#ast)

  Return this concatenation as an AST.

  

  If this alternation contains zero ASTs, then `Ast::empty` is returned.

  If this alternation contains exactly 1 AST, then the corresponding AST

  is returned. Otherwise, `Ast::concat` is returned.

#### Trait Implementations

##### `impl Any for Concat`

- <span id="concat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Concat`

- <span id="concat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Concat`

- <span id="concat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Concat`

- <span id="concat-clone"></span>`fn clone(&self) -> Concat` — [`Concat`](#concat)

##### `impl CloneToUninit for Concat`

- <span id="concat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Concat`

- <span id="concat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Concat`

##### `impl<T> From for Concat`

- <span id="concat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Concat`

- <span id="concat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Concat`

- <span id="concat-partialeq-eq"></span>`fn eq(&self, other: &Concat) -> bool` — [`Concat`](#concat)

##### `impl StructuralPartialEq for Concat`

##### `impl ToOwned for Concat`

- <span id="concat-toowned-type-owned"></span>`type Owned = T`

- <span id="concat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="concat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Concat`

- <span id="concat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="concat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Concat`

- <span id="concat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="concat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Literal`

```rust
struct Literal {
    pub span: Span,
    pub kind: LiteralKind,
    pub c: char,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:677-684`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L677-L684)*

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

  If this literal was written as a `\x` hex escape, then this returns

  the corresponding byte value. Otherwise, this returns `None`.

#### Trait Implementations

##### `impl Any for Literal`

- <span id="literal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Literal`

- <span id="literal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Literal`

- <span id="literal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl CloneToUninit for Literal`

- <span id="literal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Literal`

- <span id="literal-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Literal`

##### `impl<T> From for Literal`

- <span id="literal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Literal`

- <span id="literal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Literal`

- <span id="literal-partialeq-eq"></span>`fn eq(&self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

##### `impl ToOwned for Literal`

- <span id="literal-toowned-type-owned"></span>`type Owned = T`

- <span id="literal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="literal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Literal`

- <span id="literal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="literal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Literal`

- <span id="literal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="literal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassPerl`

```rust
struct ClassPerl {
    pub span: Span,
    pub kind: ClassPerlKind,
    pub negated: bool,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:785-793`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L785-L793)*

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

##### `impl Any for ClassPerl`

- <span id="classperl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassPerl`

- <span id="classperl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassPerl`

- <span id="classperl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassPerl`

- <span id="classperl-clone"></span>`fn clone(&self) -> ClassPerl` — [`ClassPerl`](#classperl)

##### `impl CloneToUninit for ClassPerl`

- <span id="classperl-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassPerl`

- <span id="classperl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassPerl`

##### `impl<T> From for ClassPerl`

- <span id="classperl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassPerl`

- <span id="classperl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassPerl`

- <span id="classperl-partialeq-eq"></span>`fn eq(&self, other: &ClassPerl) -> bool` — [`ClassPerl`](#classperl)

##### `impl StructuralPartialEq for ClassPerl`

##### `impl ToOwned for ClassPerl`

- <span id="classperl-toowned-type-owned"></span>`type Owned = T`

- <span id="classperl-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classperl-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassPerl`

- <span id="classperl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classperl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassPerl`

- <span id="classperl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classperl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassAscii`

```rust
struct ClassAscii {
    pub span: Span,
    pub kind: ClassAsciiKind,
    pub negated: bool,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:810-818`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L810-L818)*

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

##### `impl Any for ClassAscii`

- <span id="classascii-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassAscii`

- <span id="classascii-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassAscii`

- <span id="classascii-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassAscii`

- <span id="classascii-clone"></span>`fn clone(&self) -> ClassAscii` — [`ClassAscii`](#classascii)

##### `impl CloneToUninit for ClassAscii`

- <span id="classascii-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassAscii`

- <span id="classascii-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassAscii`

##### `impl<T> From for ClassAscii`

- <span id="classascii-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassAscii`

- <span id="classascii-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassAscii`

- <span id="classascii-partialeq-eq"></span>`fn eq(&self, other: &ClassAscii) -> bool` — [`ClassAscii`](#classascii)

##### `impl StructuralPartialEq for ClassAscii`

##### `impl ToOwned for ClassAscii`

- <span id="classascii-toowned-type-owned"></span>`type Owned = T`

- <span id="classascii-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classascii-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassAscii`

- <span id="classascii-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classascii-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassAscii`

- <span id="classascii-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classascii-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassUnicode`

```rust
struct ClassUnicode {
    pub span: Span,
    pub negated: bool,
    pub kind: ClassUnicodeKind,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:887-902`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L887-L902)*

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

  Returns true if this class has been negated.

  

  Note that this takes the Unicode op into account, if it's present.

  e.g., `is_negated` for `\P{scx!=Katakana}` will return `false`.

#### Trait Implementations

##### `impl Any for ClassUnicode`

- <span id="classunicode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassUnicode`

- <span id="classunicode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassUnicode`

- <span id="classunicode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassUnicode`

- <span id="classunicode-clone"></span>`fn clone(&self) -> ClassUnicode` — [`ClassUnicode`](#classunicode)

##### `impl CloneToUninit for ClassUnicode`

- <span id="classunicode-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassUnicode`

- <span id="classunicode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicode`

##### `impl<T> From for ClassUnicode`

- <span id="classunicode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassUnicode`

- <span id="classunicode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassUnicode`

- <span id="classunicode-partialeq-eq"></span>`fn eq(&self, other: &ClassUnicode) -> bool` — [`ClassUnicode`](#classunicode)

##### `impl StructuralPartialEq for ClassUnicode`

##### `impl ToOwned for ClassUnicode`

- <span id="classunicode-toowned-type-owned"></span>`type Owned = T`

- <span id="classunicode-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classunicode-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassUnicode`

- <span id="classunicode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classunicode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassUnicode`

- <span id="classunicode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classunicode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassBracketed`

```rust
struct ClassBracketed {
    pub span: Span,
    pub negated: bool,
    pub kind: ClassSet,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1112-1121`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1112-L1121)*

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

##### `impl Any for ClassBracketed`

- <span id="classbracketed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassBracketed`

- <span id="classbracketed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassBracketed`

- <span id="classbracketed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassBracketed`

- <span id="classbracketed-clone"></span>`fn clone(&self) -> ClassBracketed` — [`ClassBracketed`](#classbracketed)

##### `impl CloneToUninit for ClassBracketed`

- <span id="classbracketed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassBracketed`

- <span id="classbracketed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassBracketed`

##### `impl<T> From for ClassBracketed`

- <span id="classbracketed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassBracketed`

- <span id="classbracketed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassBracketed`

- <span id="classbracketed-partialeq-eq"></span>`fn eq(&self, other: &ClassBracketed) -> bool` — [`ClassBracketed`](#classbracketed)

##### `impl StructuralPartialEq for ClassBracketed`

##### `impl ToOwned for ClassBracketed`

- <span id="classbracketed-toowned-type-owned"></span>`type Owned = T`

- <span id="classbracketed-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classbracketed-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassBracketed`

- <span id="classbracketed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classbracketed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassBracketed`

- <span id="classbracketed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classbracketed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassSetRange`

```rust
struct ClassSetRange {
    pub span: Span,
    pub start: Literal,
    pub end: Literal,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1209-1216`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1209-L1216)*

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

  Returns true if and only if this character class range is valid.

  

  The only case where a range is invalid is if its start is greater than

  its end.

#### Trait Implementations

##### `impl Any for ClassSetRange`

- <span id="classsetrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassSetRange`

- <span id="classsetrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassSetRange`

- <span id="classsetrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassSetRange`

- <span id="classsetrange-clone"></span>`fn clone(&self) -> ClassSetRange` — [`ClassSetRange`](#classsetrange)

##### `impl CloneToUninit for ClassSetRange`

- <span id="classsetrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassSetRange`

- <span id="classsetrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetRange`

##### `impl<T> From for ClassSetRange`

- <span id="classsetrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassSetRange`

- <span id="classsetrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassSetRange`

- <span id="classsetrange-partialeq-eq"></span>`fn eq(&self, other: &ClassSetRange) -> bool` — [`ClassSetRange`](#classsetrange)

##### `impl StructuralPartialEq for ClassSetRange`

##### `impl ToOwned for ClassSetRange`

- <span id="classsetrange-toowned-type-owned"></span>`type Owned = T`

- <span id="classsetrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classsetrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassSetRange`

- <span id="classsetrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classsetrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassSetRange`

- <span id="classsetrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classsetrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassSetUnion`

```rust
struct ClassSetUnion {
    pub span: Span,
    pub items: alloc::vec::Vec<ClassSetItem>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1231-1237`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1231-L1237)*

A union of items inside a character class set.

#### Fields

- **`span`**: `Span`

  The span of the items in this operation. e.g., the `a-z0-9` in
  `[^a-z0-9]`

- **`items`**: `alloc::vec::Vec<ClassSetItem>`

  The sequence of items that make up this union.

#### Implementations

- <span id="classsetunion-push"></span>`fn push(&mut self, item: ClassSetItem)` — [`ClassSetItem`](#classsetitem)

  Push a new item in this union.

  

  The ending position of this union's span is updated to the ending

  position of the span of the item given. If the union is empty, then

  the starting position of this union is set to the starting position

  of this item.

  

  In other words, if you only use this method to add items to a union

  and you set the spans on each item correctly, then you should never

  need to adjust the span of the union directly.

- <span id="classsetunion-into-item"></span>`fn into_item(self) -> ClassSetItem` — [`ClassSetItem`](#classsetitem)

  Return this union as a character class set item.

  

  If this union contains zero items, then an empty union is

  returned. If this concatenation contains exactly 1 item, then the

  corresponding item is returned. Otherwise, ClassSetItem::Union is

  returned.

#### Trait Implementations

##### `impl Any for ClassSetUnion`

- <span id="classsetunion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassSetUnion`

- <span id="classsetunion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassSetUnion`

- <span id="classsetunion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassSetUnion`

- <span id="classsetunion-clone"></span>`fn clone(&self) -> ClassSetUnion` — [`ClassSetUnion`](#classsetunion)

##### `impl CloneToUninit for ClassSetUnion`

- <span id="classsetunion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassSetUnion`

- <span id="classsetunion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetUnion`

##### `impl<T> From for ClassSetUnion`

- <span id="classsetunion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassSetUnion`

- <span id="classsetunion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassSetUnion`

- <span id="classsetunion-partialeq-eq"></span>`fn eq(&self, other: &ClassSetUnion) -> bool` — [`ClassSetUnion`](#classsetunion)

##### `impl StructuralPartialEq for ClassSetUnion`

##### `impl ToOwned for ClassSetUnion`

- <span id="classsetunion-toowned-type-owned"></span>`type Owned = T`

- <span id="classsetunion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classsetunion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassSetUnion`

- <span id="classsetunion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classsetunion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassSetUnion`

- <span id="classsetunion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classsetunion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassSetBinaryOp`

```rust
struct ClassSetBinaryOp {
    pub span: Span,
    pub kind: ClassSetBinaryOpKind,
    pub lhs: alloc::boxed::Box<ClassSet>,
    pub rhs: alloc::boxed::Box<ClassSet>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1276-1285`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1276-L1285)*

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

##### `impl Any for ClassSetBinaryOp`

- <span id="classsetbinaryop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassSetBinaryOp`

- <span id="classsetbinaryop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassSetBinaryOp`

- <span id="classsetbinaryop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassSetBinaryOp`

- <span id="classsetbinaryop-clone"></span>`fn clone(&self) -> ClassSetBinaryOp` — [`ClassSetBinaryOp`](#classsetbinaryop)

##### `impl CloneToUninit for ClassSetBinaryOp`

- <span id="classsetbinaryop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassSetBinaryOp`

- <span id="classsetbinaryop-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetBinaryOp`

##### `impl<T> From for ClassSetBinaryOp`

- <span id="classsetbinaryop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassSetBinaryOp`

- <span id="classsetbinaryop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassSetBinaryOp`

- <span id="classsetbinaryop-partialeq-eq"></span>`fn eq(&self, other: &ClassSetBinaryOp) -> bool` — [`ClassSetBinaryOp`](#classsetbinaryop)

##### `impl StructuralPartialEq for ClassSetBinaryOp`

##### `impl ToOwned for ClassSetBinaryOp`

- <span id="classsetbinaryop-toowned-type-owned"></span>`type Owned = T`

- <span id="classsetbinaryop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classsetbinaryop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassSetBinaryOp`

- <span id="classsetbinaryop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classsetbinaryop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassSetBinaryOp`

- <span id="classsetbinaryop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classsetbinaryop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Assertion`

```rust
struct Assertion {
    pub span: Span,
    pub kind: AssertionKind,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1308-1313`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1308-L1313)*

A single zero-width assertion.

#### Fields

- **`span`**: `Span`

  The span of this assertion.

- **`kind`**: `AssertionKind`

  The assertion kind, e.g., `\b` or `^`.

#### Trait Implementations

##### `impl Any for Assertion`

- <span id="assertion-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Assertion`

- <span id="assertion-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Assertion`

- <span id="assertion-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Assertion`

- <span id="assertion-clone"></span>`fn clone(&self) -> Assertion` — [`Assertion`](#assertion)

##### `impl CloneToUninit for Assertion`

- <span id="assertion-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Assertion`

- <span id="assertion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Assertion`

##### `impl<T> From for Assertion`

- <span id="assertion-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Assertion`

- <span id="assertion-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Assertion`

- <span id="assertion-partialeq-eq"></span>`fn eq(&self, other: &Assertion) -> bool` — [`Assertion`](#assertion)

##### `impl StructuralPartialEq for Assertion`

##### `impl ToOwned for Assertion`

- <span id="assertion-toowned-type-owned"></span>`type Owned = T`

- <span id="assertion-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="assertion-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Assertion`

- <span id="assertion-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assertion-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Assertion`

- <span id="assertion-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assertion-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Repetition`

```rust
struct Repetition {
    pub span: Span,
    pub op: RepetitionOp,
    pub greedy: bool,
    pub ast: alloc::boxed::Box<Ast>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1348-1357`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1348-L1357)*

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

##### `impl Any for Repetition`

- <span id="repetition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Repetition`

- <span id="repetition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Repetition`

- <span id="repetition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Repetition`

- <span id="repetition-clone"></span>`fn clone(&self) -> Repetition` — [`Repetition`](#repetition)

##### `impl CloneToUninit for Repetition`

- <span id="repetition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Repetition`

- <span id="repetition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Repetition`

##### `impl<T> From for Repetition`

- <span id="repetition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Repetition`

- <span id="repetition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Repetition`

- <span id="repetition-partialeq-eq"></span>`fn eq(&self, other: &Repetition) -> bool` — [`Repetition`](#repetition)

##### `impl StructuralPartialEq for Repetition`

##### `impl ToOwned for Repetition`

- <span id="repetition-toowned-type-owned"></span>`type Owned = T`

- <span id="repetition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repetition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Repetition`

- <span id="repetition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repetition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Repetition`

- <span id="repetition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repetition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepetitionOp`

```rust
struct RepetitionOp {
    pub span: Span,
    pub kind: RepetitionKind,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1362-1368`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1362-L1368)*

The repetition operator itself.

#### Fields

- **`span`**: `Span`

  The span of this operator. This includes things like `+`, `*?` and
  `{m,n}`.

- **`kind`**: `RepetitionKind`

  The type of operation.

#### Trait Implementations

##### `impl Any for RepetitionOp`

- <span id="repetitionop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepetitionOp`

- <span id="repetitionop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepetitionOp`

- <span id="repetitionop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RepetitionOp`

- <span id="repetitionop-clone"></span>`fn clone(&self) -> RepetitionOp` — [`RepetitionOp`](#repetitionop)

##### `impl CloneToUninit for RepetitionOp`

- <span id="repetitionop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RepetitionOp`

- <span id="repetitionop-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RepetitionOp`

##### `impl<T> From for RepetitionOp`

- <span id="repetitionop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RepetitionOp`

- <span id="repetitionop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RepetitionOp`

- <span id="repetitionop-partialeq-eq"></span>`fn eq(&self, other: &RepetitionOp) -> bool` — [`RepetitionOp`](#repetitionop)

##### `impl StructuralPartialEq for RepetitionOp`

##### `impl ToOwned for RepetitionOp`

- <span id="repetitionop-toowned-type-owned"></span>`type Owned = T`

- <span id="repetitionop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repetitionop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RepetitionOp`

- <span id="repetitionop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repetitionop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RepetitionOp`

- <span id="repetitionop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repetitionop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Group`

```rust
struct Group {
    pub span: Span,
    pub kind: GroupKind,
    pub ast: alloc::boxed::Box<Ast>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1417-1424`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1417-L1424)*

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

  If this group is non-capturing, then this returns the (possibly empty)

  set of flags. Otherwise, `None` is returned.

- <span id="group-is-capturing"></span>`fn is_capturing(&self) -> bool`

  Returns true if and only if this group is capturing.

- <span id="group-capture-index"></span>`fn capture_index(&self) -> Option<u32>`

  Returns the capture index of this group, if this is a capturing group.

  

  This returns a capture index precisely when `is_capturing` is `true`.

#### Trait Implementations

##### `impl Any for Group`

- <span id="group-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Group`

- <span id="group-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Group`

- <span id="group-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl CloneToUninit for Group`

- <span id="group-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Group`

- <span id="group-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Group`

##### `impl<T> From for Group`

- <span id="group-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Group`

- <span id="group-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Group`

- <span id="group-partialeq-eq"></span>`fn eq(&self, other: &Group) -> bool` — [`Group`](#group)

##### `impl StructuralPartialEq for Group`

##### `impl ToOwned for Group`

- <span id="group-toowned-type-owned"></span>`type Owned = T`

- <span id="group-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="group-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Group`

- <span id="group-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="group-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Group`

- <span id="group-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="group-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CaptureName`

```rust
struct CaptureName {
    pub span: Span,
    pub name: alloc::string::String,
    pub index: u32,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1478-1485`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1478-L1485)*

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

##### `impl Any for CaptureName`

- <span id="capturename-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaptureName`

- <span id="capturename-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaptureName`

- <span id="capturename-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CaptureName`

- <span id="capturename-clone"></span>`fn clone(&self) -> CaptureName` — [`CaptureName`](#capturename)

##### `impl CloneToUninit for CaptureName`

- <span id="capturename-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CaptureName`

- <span id="capturename-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CaptureName`

##### `impl<T> From for CaptureName`

- <span id="capturename-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CaptureName`

- <span id="capturename-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CaptureName`

- <span id="capturename-partialeq-eq"></span>`fn eq(&self, other: &CaptureName) -> bool` — [`CaptureName`](#capturename)

##### `impl StructuralPartialEq for CaptureName`

##### `impl ToOwned for CaptureName`

- <span id="capturename-toowned-type-owned"></span>`type Owned = T`

- <span id="capturename-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capturename-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CaptureName`

- <span id="capturename-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturename-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaptureName`

- <span id="capturename-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturename-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SetFlags`

```rust
struct SetFlags {
    pub span: Span,
    pub flags: Flags,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1519-1524`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1519-L1524)*

A group of flags that is not applied to a particular regular expression.

#### Fields

- **`span`**: `Span`

  The span of these flags, including the grouping parentheses.

- **`flags`**: `Flags`

  The actual sequence of flags.

#### Trait Implementations

##### `impl Any for SetFlags`

- <span id="setflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SetFlags`

- <span id="setflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SetFlags`

- <span id="setflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SetFlags`

- <span id="setflags-clone"></span>`fn clone(&self) -> SetFlags` — [`SetFlags`](#setflags)

##### `impl CloneToUninit for SetFlags`

- <span id="setflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SetFlags`

- <span id="setflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SetFlags`

##### `impl<T> From for SetFlags`

- <span id="setflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SetFlags`

- <span id="setflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SetFlags`

- <span id="setflags-partialeq-eq"></span>`fn eq(&self, other: &SetFlags) -> bool` — [`SetFlags`](#setflags)

##### `impl StructuralPartialEq for SetFlags`

##### `impl ToOwned for SetFlags`

- <span id="setflags-toowned-type-owned"></span>`type Owned = T`

- <span id="setflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="setflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SetFlags`

- <span id="setflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SetFlags`

- <span id="setflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Flags`

```rust
struct Flags {
    pub span: Span,
    pub items: alloc::vec::Vec<FlagsItem>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1531-1537`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1531-L1537)*

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

  Add the given item to this sequence of flags.

  

  If the item was added successfully, then `None` is returned. If the

  given item is a duplicate, then `Some(i)` is returned, where

  `items[i].kind == item.kind`.

- <span id="flags-flag-state"></span>`fn flag_state(&self, flag: Flag) -> Option<bool>` — [`Flag`](#flag)

  Returns the state of the given flag in this set.

  

  If the given flag is in the set but is negated, then `Some(false)` is

  returned.

  

  If the given flag is in the set and is not negated, then `Some(true)`

  is returned.

  

  Otherwise, `None` is returned.

#### Trait Implementations

##### `impl Any for Flags`

- <span id="flags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flags`

- <span id="flags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flags`

- <span id="flags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Flags`

- <span id="flags-clone"></span>`fn clone(&self) -> Flags` — [`Flags`](#flags)

##### `impl CloneToUninit for Flags`

- <span id="flags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Flags`

- <span id="flags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Flags`

##### `impl<T> From for Flags`

- <span id="flags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flags`

- <span id="flags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Flags`

- <span id="flags-partialeq-eq"></span>`fn eq(&self, other: &Flags) -> bool` — [`Flags`](#flags)

##### `impl StructuralPartialEq for Flags`

##### `impl ToOwned for Flags`

- <span id="flags-toowned-type-owned"></span>`type Owned = T`

- <span id="flags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Flags`

- <span id="flags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flags`

- <span id="flags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlagsItem`

```rust
struct FlagsItem {
    pub span: Span,
    pub kind: FlagsItemKind,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1584-1589`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1584-L1589)*

A single item in a group of flags.

#### Fields

- **`span`**: `Span`

  The span of this item.

- **`kind`**: `FlagsItemKind`

  The kind of this item.

#### Trait Implementations

##### `impl Any for FlagsItem`

- <span id="flagsitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlagsItem`

- <span id="flagsitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlagsItem`

- <span id="flagsitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FlagsItem`

- <span id="flagsitem-clone"></span>`fn clone(&self) -> FlagsItem` — [`FlagsItem`](#flagsitem)

##### `impl CloneToUninit for FlagsItem`

- <span id="flagsitem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FlagsItem`

- <span id="flagsitem-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlagsItem`

##### `impl<T> From for FlagsItem`

- <span id="flagsitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlagsItem`

- <span id="flagsitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FlagsItem`

- <span id="flagsitem-partialeq-eq"></span>`fn eq(&self, other: &FlagsItem) -> bool` — [`FlagsItem`](#flagsitem)

##### `impl StructuralPartialEq for FlagsItem`

##### `impl ToOwned for FlagsItem`

- <span id="flagsitem-toowned-type-owned"></span>`type Owned = T`

- <span id="flagsitem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flagsitem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlagsItem`

- <span id="flagsitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flagsitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlagsItem`

- <span id="flagsitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flagsitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:75-190`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L75-L190)*

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

##### `impl Any for ErrorKind`

- <span id="errorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorKind`

- <span id="errorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorKind`

- <span id="errorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl CloneToUninit for ErrorKind`

- <span id="errorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ErrorKind`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl<T> From for ErrorKind`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-partialeq-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl ToOwned for ErrorKind`

- <span id="errorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="errorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ErrorKind`

- <span id="errorkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ErrorKind`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:469-496`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L469-L496)*

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

  Create an "empty" AST item.

- <span id="ast-flags"></span>`fn flags(e: SetFlags) -> Ast` — [`SetFlags`](#setflags), [`Ast`](#ast)

  Create a "flags" AST item.

- <span id="ast-literal"></span>`fn literal(e: Literal) -> Ast` — [`Literal`](#literal), [`Ast`](#ast)

  Create a "literal" AST item.

- <span id="ast-dot"></span>`fn dot(span: Span) -> Ast` — [`Span`](#span), [`Ast`](#ast)

  Create a "dot" AST item.

- <span id="ast-assertion"></span>`fn assertion(e: Assertion) -> Ast` — [`Assertion`](#assertion), [`Ast`](#ast)

  Create a "assertion" AST item.

- <span id="ast-class-unicode"></span>`fn class_unicode(e: ClassUnicode) -> Ast` — [`ClassUnicode`](#classunicode), [`Ast`](#ast)

  Create a "Unicode class" AST item.

- <span id="ast-class-perl"></span>`fn class_perl(e: ClassPerl) -> Ast` — [`ClassPerl`](#classperl), [`Ast`](#ast)

  Create a "Perl class" AST item.

- <span id="ast-class-bracketed"></span>`fn class_bracketed(e: ClassBracketed) -> Ast` — [`ClassBracketed`](#classbracketed), [`Ast`](#ast)

  Create a "bracketed class" AST item.

- <span id="ast-repetition"></span>`fn repetition(e: Repetition) -> Ast` — [`Repetition`](#repetition), [`Ast`](#ast)

  Create a "repetition" AST item.

- <span id="ast-group"></span>`fn group(e: Group) -> Ast` — [`Group`](#group), [`Ast`](#ast)

  Create a "group" AST item.

- <span id="ast-alternation"></span>`fn alternation(e: Alternation) -> Ast` — [`Alternation`](#alternation), [`Ast`](#ast)

  Create a "alternation" AST item.

- <span id="ast-concat"></span>`fn concat(e: Concat) -> Ast` — [`Concat`](#concat), [`Ast`](#ast)

  Create a "concat" AST item.

- <span id="ast-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

  Return the span of this abstract syntax tree.

- <span id="ast-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if and only if this Ast is empty.

- <span id="ast-has-subexprs"></span>`fn has_subexprs(&self) -> bool`

  Returns true if and only if this AST has any (including possibly empty)

  subexpressions.

#### Trait Implementations

##### `impl Any for Ast`

- <span id="ast-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ast`

- <span id="ast-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ast`

- <span id="ast-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ast`

- <span id="ast-clone"></span>`fn clone(&self) -> Ast` — [`Ast`](#ast)

##### `impl CloneToUninit for Ast`

- <span id="ast-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Ast`

- <span id="ast-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Ast`

- <span id="ast-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Drop for Ast`

- <span id="ast-drop"></span>`fn drop(&mut self)`

##### `impl Eq for Ast`

##### `impl<T> From for Ast`

- <span id="ast-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ast`

- <span id="ast-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Ast`

- <span id="ast-partialeq-eq"></span>`fn eq(&self, other: &Ast) -> bool` — [`Ast`](#ast)

##### `impl StructuralPartialEq for Ast`

##### `impl ToOwned for Ast`

- <span id="ast-toowned-type-owned"></span>`type Owned = T`

- <span id="ast-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ast-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Ast`

- <span id="ast-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Ast`

- <span id="ast-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ast-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ast`

- <span id="ast-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ast-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:702-724`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L702-L724)*

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

##### `impl Any for LiteralKind`

- <span id="literalkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LiteralKind`

- <span id="literalkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LiteralKind`

- <span id="literalkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LiteralKind`

- <span id="literalkind-clone"></span>`fn clone(&self) -> LiteralKind` — [`LiteralKind`](#literalkind)

##### `impl CloneToUninit for LiteralKind`

- <span id="literalkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LiteralKind`

- <span id="literalkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LiteralKind`

##### `impl<T> From for LiteralKind`

- <span id="literalkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LiteralKind`

- <span id="literalkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LiteralKind`

- <span id="literalkind-partialeq-eq"></span>`fn eq(&self, other: &LiteralKind) -> bool` — [`LiteralKind`](#literalkind)

##### `impl StructuralPartialEq for LiteralKind`

##### `impl ToOwned for LiteralKind`

- <span id="literalkind-toowned-type-owned"></span>`type Owned = T`

- <span id="literalkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="literalkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LiteralKind`

- <span id="literalkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="literalkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LiteralKind`

- <span id="literalkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="literalkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:732-748`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L732-L748)*

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

##### `impl Any for SpecialLiteralKind`

- <span id="specialliteralkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpecialLiteralKind`

- <span id="specialliteralkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpecialLiteralKind`

- <span id="specialliteralkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SpecialLiteralKind`

- <span id="specialliteralkind-clone"></span>`fn clone(&self) -> SpecialLiteralKind` — [`SpecialLiteralKind`](#specialliteralkind)

##### `impl CloneToUninit for SpecialLiteralKind`

- <span id="specialliteralkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SpecialLiteralKind`

- <span id="specialliteralkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SpecialLiteralKind`

##### `impl<T> From for SpecialLiteralKind`

- <span id="specialliteralkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpecialLiteralKind`

- <span id="specialliteralkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SpecialLiteralKind`

- <span id="specialliteralkind-partialeq-eq"></span>`fn eq(&self, other: &SpecialLiteralKind) -> bool` — [`SpecialLiteralKind`](#specialliteralkind)

##### `impl StructuralPartialEq for SpecialLiteralKind`

##### `impl ToOwned for SpecialLiteralKind`

- <span id="specialliteralkind-toowned-type-owned"></span>`type Owned = T`

- <span id="specialliteralkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="specialliteralkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SpecialLiteralKind`

- <span id="specialliteralkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="specialliteralkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpecialLiteralKind`

- <span id="specialliteralkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="specialliteralkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HexLiteralKind`

```rust
enum HexLiteralKind {
    X,
    UnicodeShort,
    UnicodeLong,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:757-767`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L757-L767)*

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

  The number of digits that must be used with this literal form when

  used without brackets. When used with brackets, there is no

  restriction on the number of digits.

#### Trait Implementations

##### `impl Any for HexLiteralKind`

- <span id="hexliteralkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HexLiteralKind`

- <span id="hexliteralkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HexLiteralKind`

- <span id="hexliteralkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for HexLiteralKind`

- <span id="hexliteralkind-clone"></span>`fn clone(&self) -> HexLiteralKind` — [`HexLiteralKind`](#hexliteralkind)

##### `impl CloneToUninit for HexLiteralKind`

- <span id="hexliteralkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for HexLiteralKind`

- <span id="hexliteralkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HexLiteralKind`

##### `impl<T> From for HexLiteralKind`

- <span id="hexliteralkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HexLiteralKind`

- <span id="hexliteralkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for HexLiteralKind`

- <span id="hexliteralkind-partialeq-eq"></span>`fn eq(&self, other: &HexLiteralKind) -> bool` — [`HexLiteralKind`](#hexliteralkind)

##### `impl StructuralPartialEq for HexLiteralKind`

##### `impl ToOwned for HexLiteralKind`

- <span id="hexliteralkind-toowned-type-owned"></span>`type Owned = T`

- <span id="hexliteralkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hexliteralkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for HexLiteralKind`

- <span id="hexliteralkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hexliteralkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HexLiteralKind`

- <span id="hexliteralkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hexliteralkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassPerlKind`

```rust
enum ClassPerlKind {
    Digit,
    Space,
    Word,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:798-805`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L798-L805)*

The available Perl character classes.

#### Variants

- **`Digit`**

  Decimal numbers.

- **`Space`**

  Whitespace.

- **`Word`**

  Word characters.

#### Trait Implementations

##### `impl Any for ClassPerlKind`

- <span id="classperlkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassPerlKind`

- <span id="classperlkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassPerlKind`

- <span id="classperlkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassPerlKind`

- <span id="classperlkind-clone"></span>`fn clone(&self) -> ClassPerlKind` — [`ClassPerlKind`](#classperlkind)

##### `impl CloneToUninit for ClassPerlKind`

- <span id="classperlkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassPerlKind`

- <span id="classperlkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassPerlKind`

##### `impl<T> From for ClassPerlKind`

- <span id="classperlkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassPerlKind`

- <span id="classperlkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassPerlKind`

- <span id="classperlkind-partialeq-eq"></span>`fn eq(&self, other: &ClassPerlKind) -> bool` — [`ClassPerlKind`](#classperlkind)

##### `impl StructuralPartialEq for ClassPerlKind`

##### `impl ToOwned for ClassPerlKind`

- <span id="classperlkind-toowned-type-owned"></span>`type Owned = T`

- <span id="classperlkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classperlkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassPerlKind`

- <span id="classperlkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classperlkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassPerlKind`

- <span id="classperlkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classperlkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:823-852`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L823-L852)*

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

  Return the corresponding ClassAsciiKind variant for the given name.

  

  The name given should correspond to the lowercase version of the

  variant name. e.g., `cntrl` is the name for `ClassAsciiKind::Cntrl`.

  

  If no variant with the corresponding name exists, then `None` is

  returned.

#### Trait Implementations

##### `impl Any for ClassAsciiKind`

- <span id="classasciikind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassAsciiKind`

- <span id="classasciikind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassAsciiKind`

- <span id="classasciikind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassAsciiKind`

- <span id="classasciikind-clone"></span>`fn clone(&self) -> ClassAsciiKind` — [`ClassAsciiKind`](#classasciikind)

##### `impl CloneToUninit for ClassAsciiKind`

- <span id="classasciikind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassAsciiKind`

- <span id="classasciikind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassAsciiKind`

##### `impl<T> From for ClassAsciiKind`

- <span id="classasciikind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassAsciiKind`

- <span id="classasciikind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassAsciiKind`

- <span id="classasciikind-partialeq-eq"></span>`fn eq(&self, other: &ClassAsciiKind) -> bool` — [`ClassAsciiKind`](#classasciikind)

##### `impl StructuralPartialEq for ClassAsciiKind`

##### `impl ToOwned for ClassAsciiKind`

- <span id="classasciikind-toowned-type-owned"></span>`type Owned = T`

- <span id="classasciikind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classasciikind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassAsciiKind`

- <span id="classasciikind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classasciikind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassAsciiKind`

- <span id="classasciikind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classasciikind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:922-937`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L922-L937)*

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

##### `impl Any for ClassUnicodeKind`

- <span id="classunicodekind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassUnicodeKind`

- <span id="classunicodekind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassUnicodeKind`

- <span id="classunicodekind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassUnicodeKind`

- <span id="classunicodekind-clone"></span>`fn clone(&self) -> ClassUnicodeKind` — [`ClassUnicodeKind`](#classunicodekind)

##### `impl CloneToUninit for ClassUnicodeKind`

- <span id="classunicodekind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassUnicodeKind`

- <span id="classunicodekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicodeKind`

##### `impl<T> From for ClassUnicodeKind`

- <span id="classunicodekind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassUnicodeKind`

- <span id="classunicodekind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassUnicodeKind`

- <span id="classunicodekind-partialeq-eq"></span>`fn eq(&self, other: &ClassUnicodeKind) -> bool` — [`ClassUnicodeKind`](#classunicodekind)

##### `impl StructuralPartialEq for ClassUnicodeKind`

##### `impl ToOwned for ClassUnicodeKind`

- <span id="classunicodekind-toowned-type-owned"></span>`type Owned = T`

- <span id="classunicodekind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classunicodekind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassUnicodeKind`

- <span id="classunicodekind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classunicodekind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassUnicodeKind`

- <span id="classunicodekind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classunicodekind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassUnicodeOpKind`

```rust
enum ClassUnicodeOpKind {
    Equal,
    Colon,
    NotEqual,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1089-1097`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1089-L1097)*

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

  Whether the op is an equality op or not.

#### Trait Implementations

##### `impl Any for ClassUnicodeOpKind`

- <span id="classunicodeopkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassUnicodeOpKind`

- <span id="classunicodeopkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassUnicodeOpKind`

- <span id="classunicodeopkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassUnicodeOpKind`

- <span id="classunicodeopkind-clone"></span>`fn clone(&self) -> ClassUnicodeOpKind` — [`ClassUnicodeOpKind`](#classunicodeopkind)

##### `impl CloneToUninit for ClassUnicodeOpKind`

- <span id="classunicodeopkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassUnicodeOpKind`

- <span id="classunicodeopkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassUnicodeOpKind`

##### `impl<T> From for ClassUnicodeOpKind`

- <span id="classunicodeopkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassUnicodeOpKind`

- <span id="classunicodeopkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassUnicodeOpKind`

- <span id="classunicodeopkind-partialeq-eq"></span>`fn eq(&self, other: &ClassUnicodeOpKind) -> bool` — [`ClassUnicodeOpKind`](#classunicodeopkind)

##### `impl StructuralPartialEq for ClassUnicodeOpKind`

##### `impl ToOwned for ClassUnicodeOpKind`

- <span id="classunicodeopkind-toowned-type-owned"></span>`type Owned = T`

- <span id="classunicodeopkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classunicodeopkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassUnicodeOpKind`

- <span id="classunicodeopkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classunicodeopkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassUnicodeOpKind`

- <span id="classunicodeopkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classunicodeopkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassSet`

```rust
enum ClassSet {
    Item(ClassSetItem),
    BinaryOp(ClassSetBinaryOp),
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1131-1137`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1131-L1137)*

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

  Build a set from a union.

- <span id="classset-span"></span>`fn span(&self) -> &Span` — [`Span`](#span)

  Return the span of this character class set.

- <span id="classset-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if and only if this class set is empty.

#### Trait Implementations

##### `impl Any for ClassSet`

- <span id="classset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassSet`

- <span id="classset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassSet`

- <span id="classset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassSet`

- <span id="classset-clone"></span>`fn clone(&self) -> ClassSet` — [`ClassSet`](#classset)

##### `impl CloneToUninit for ClassSet`

- <span id="classset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassSet`

- <span id="classset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for ClassSet`

- <span id="classset-drop"></span>`fn drop(&mut self)`

##### `impl Eq for ClassSet`

##### `impl<T> From for ClassSet`

- <span id="classset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassSet`

- <span id="classset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassSet`

- <span id="classset-partialeq-eq"></span>`fn eq(&self, other: &ClassSet) -> bool` — [`ClassSet`](#classset)

##### `impl StructuralPartialEq for ClassSet`

##### `impl ToOwned for ClassSet`

- <span id="classset-toowned-type-owned"></span>`type Owned = T`

- <span id="classset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassSet`

- <span id="classset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassSet`

- <span id="classset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1165-1188`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1165-L1188)*

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

  Return the span of this character class set item.

#### Trait Implementations

##### `impl Any for ClassSetItem`

- <span id="classsetitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassSetItem`

- <span id="classsetitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassSetItem`

- <span id="classsetitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassSetItem`

- <span id="classsetitem-clone"></span>`fn clone(&self) -> ClassSetItem` — [`ClassSetItem`](#classsetitem)

##### `impl CloneToUninit for ClassSetItem`

- <span id="classsetitem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassSetItem`

- <span id="classsetitem-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetItem`

##### `impl<T> From for ClassSetItem`

- <span id="classsetitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassSetItem`

- <span id="classsetitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassSetItem`

- <span id="classsetitem-partialeq-eq"></span>`fn eq(&self, other: &ClassSetItem) -> bool` — [`ClassSetItem`](#classsetitem)

##### `impl StructuralPartialEq for ClassSetItem`

##### `impl ToOwned for ClassSetItem`

- <span id="classsetitem-toowned-type-owned"></span>`type Owned = T`

- <span id="classsetitem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classsetitem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassSetItem`

- <span id="classsetitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classsetitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassSetItem`

- <span id="classsetitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classsetitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassSetBinaryOpKind`

```rust
enum ClassSetBinaryOpKind {
    Intersection,
    Difference,
    SymmetricDifference,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1294-1303`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1294-L1303)*

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

##### `impl Any for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-clone"></span>`fn clone(&self) -> ClassSetBinaryOpKind` — [`ClassSetBinaryOpKind`](#classsetbinaryopkind)

##### `impl CloneToUninit for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ClassSetBinaryOpKind`

##### `impl Debug for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ClassSetBinaryOpKind`

##### `impl<T> From for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-partialeq-eq"></span>`fn eq(&self, other: &ClassSetBinaryOpKind) -> bool` — [`ClassSetBinaryOpKind`](#classsetbinaryopkind)

##### `impl StructuralPartialEq for ClassSetBinaryOpKind`

##### `impl ToOwned for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-toowned-type-owned"></span>`type Owned = T`

- <span id="classsetbinaryopkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classsetbinaryopkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classsetbinaryopkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassSetBinaryOpKind`

- <span id="classsetbinaryopkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classsetbinaryopkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1318-1343`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1318-L1343)*

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

##### `impl Any for AssertionKind`

- <span id="assertionkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AssertionKind`

- <span id="assertionkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AssertionKind`

- <span id="assertionkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AssertionKind`

- <span id="assertionkind-clone"></span>`fn clone(&self) -> AssertionKind` — [`AssertionKind`](#assertionkind)

##### `impl CloneToUninit for AssertionKind`

- <span id="assertionkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AssertionKind`

- <span id="assertionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AssertionKind`

##### `impl<T> From for AssertionKind`

- <span id="assertionkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AssertionKind`

- <span id="assertionkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AssertionKind`

- <span id="assertionkind-partialeq-eq"></span>`fn eq(&self, other: &AssertionKind) -> bool` — [`AssertionKind`](#assertionkind)

##### `impl StructuralPartialEq for AssertionKind`

##### `impl ToOwned for AssertionKind`

- <span id="assertionkind-toowned-type-owned"></span>`type Owned = T`

- <span id="assertionkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="assertionkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AssertionKind`

- <span id="assertionkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="assertionkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AssertionKind`

- <span id="assertionkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="assertionkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepetitionKind`

```rust
enum RepetitionKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Range(RepetitionRange),
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1373-1382`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1373-L1382)*

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

##### `impl Any for RepetitionKind`

- <span id="repetitionkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepetitionKind`

- <span id="repetitionkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepetitionKind`

- <span id="repetitionkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RepetitionKind`

- <span id="repetitionkind-clone"></span>`fn clone(&self) -> RepetitionKind` — [`RepetitionKind`](#repetitionkind)

##### `impl CloneToUninit for RepetitionKind`

- <span id="repetitionkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RepetitionKind`

- <span id="repetitionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RepetitionKind`

##### `impl<T> From for RepetitionKind`

- <span id="repetitionkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RepetitionKind`

- <span id="repetitionkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RepetitionKind`

- <span id="repetitionkind-partialeq-eq"></span>`fn eq(&self, other: &RepetitionKind) -> bool` — [`RepetitionKind`](#repetitionkind)

##### `impl StructuralPartialEq for RepetitionKind`

##### `impl ToOwned for RepetitionKind`

- <span id="repetitionkind-toowned-type-owned"></span>`type Owned = T`

- <span id="repetitionkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repetitionkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RepetitionKind`

- <span id="repetitionkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repetitionkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RepetitionKind`

- <span id="repetitionkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repetitionkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepetitionRange`

```rust
enum RepetitionRange {
    Exactly(u32),
    AtLeast(u32),
    Bounded(u32, u32),
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1387-1394`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1387-L1394)*

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

  Returns true if and only if this repetition range is valid.

  

  The only case where a repetition range is invalid is if it is bounded

  and its start is greater than its end.

#### Trait Implementations

##### `impl Any for RepetitionRange`

- <span id="repetitionrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepetitionRange`

- <span id="repetitionrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepetitionRange`

- <span id="repetitionrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RepetitionRange`

- <span id="repetitionrange-clone"></span>`fn clone(&self) -> RepetitionRange` — [`RepetitionRange`](#repetitionrange)

##### `impl CloneToUninit for RepetitionRange`

- <span id="repetitionrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RepetitionRange`

- <span id="repetitionrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RepetitionRange`

##### `impl<T> From for RepetitionRange`

- <span id="repetitionrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RepetitionRange`

- <span id="repetitionrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RepetitionRange`

- <span id="repetitionrange-partialeq-eq"></span>`fn eq(&self, other: &RepetitionRange) -> bool` — [`RepetitionRange`](#repetitionrange)

##### `impl StructuralPartialEq for RepetitionRange`

##### `impl ToOwned for RepetitionRange`

- <span id="repetitionrange-toowned-type-owned"></span>`type Owned = T`

- <span id="repetitionrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repetitionrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RepetitionRange`

- <span id="repetitionrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repetitionrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RepetitionRange`

- <span id="repetitionrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repetitionrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1459-1471`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1459-L1471)*

The kind of a group.

#### Variants

- **`CaptureIndex`**

  `(a)`

- **`CaptureName`**

  `(?<name>a)` or `(?P<name>a)`

- **`NonCapturing`**

  `(?:a)` and `(?i:a)`

#### Trait Implementations

##### `impl Any for GroupKind`

- <span id="groupkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GroupKind`

- <span id="groupkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GroupKind`

- <span id="groupkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GroupKind`

- <span id="groupkind-clone"></span>`fn clone(&self) -> GroupKind` — [`GroupKind`](#groupkind)

##### `impl CloneToUninit for GroupKind`

- <span id="groupkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GroupKind`

- <span id="groupkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for GroupKind`

##### `impl<T> From for GroupKind`

- <span id="groupkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GroupKind`

- <span id="groupkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for GroupKind`

- <span id="groupkind-partialeq-eq"></span>`fn eq(&self, other: &GroupKind) -> bool` — [`GroupKind`](#groupkind)

##### `impl StructuralPartialEq for GroupKind`

##### `impl ToOwned for GroupKind`

- <span id="groupkind-toowned-type-owned"></span>`type Owned = T`

- <span id="groupkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="groupkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GroupKind`

- <span id="groupkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groupkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GroupKind`

- <span id="groupkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groupkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlagsItemKind`

```rust
enum FlagsItemKind {
    Negation,
    Flag(Flag),
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1594-1600`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1594-L1600)*

The kind of an item in a group of flags.

#### Variants

- **`Negation`**

  A negation operator applied to all subsequent flags in the enclosing
  group.

- **`Flag`**

  A single flag in a group.

#### Implementations

- <span id="flagsitemkind-is-negation"></span>`fn is_negation(&self) -> bool`

  Returns true if and only if this item is a negation operator.

#### Trait Implementations

##### `impl Any for FlagsItemKind`

- <span id="flagsitemkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlagsItemKind`

- <span id="flagsitemkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlagsItemKind`

- <span id="flagsitemkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FlagsItemKind`

- <span id="flagsitemkind-clone"></span>`fn clone(&self) -> FlagsItemKind` — [`FlagsItemKind`](#flagsitemkind)

##### `impl CloneToUninit for FlagsItemKind`

- <span id="flagsitemkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FlagsItemKind`

- <span id="flagsitemkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FlagsItemKind`

##### `impl<T> From for FlagsItemKind`

- <span id="flagsitemkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlagsItemKind`

- <span id="flagsitemkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FlagsItemKind`

- <span id="flagsitemkind-partialeq-eq"></span>`fn eq(&self, other: &FlagsItemKind) -> bool` — [`FlagsItemKind`](#flagsitemkind)

##### `impl StructuralPartialEq for FlagsItemKind`

##### `impl ToOwned for FlagsItemKind`

- <span id="flagsitemkind-toowned-type-owned"></span>`type Owned = T`

- <span id="flagsitemkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flagsitemkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlagsItemKind`

- <span id="flagsitemkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flagsitemkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlagsItemKind`

- <span id="flagsitemkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flagsitemkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/mod.rs:1615-1630`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/mod.rs#L1615-L1630)*

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

##### `impl Any for Flag`

- <span id="flag-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flag`

- <span id="flag-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flag`

- <span id="flag-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Flag`

- <span id="flag-clone"></span>`fn clone(&self) -> Flag` — [`Flag`](#flag)

##### `impl CloneToUninit for Flag`

- <span id="flag-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Flag`

##### `impl Debug for Flag`

- <span id="flag-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Flag`

##### `impl<T> From for Flag`

- <span id="flag-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flag`

- <span id="flag-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Flag`

- <span id="flag-partialeq-eq"></span>`fn eq(&self, other: &Flag) -> bool` — [`Flag`](#flag)

##### `impl StructuralPartialEq for Flag`

##### `impl ToOwned for Flag`

- <span id="flag-toowned-type-owned"></span>`type Owned = T`

- <span id="flag-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flag-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Flag`

- <span id="flag-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flag-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flag`

- <span id="flag-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flag-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:20-102`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/visitor.rs#L20-L102)*

A trait for visiting an abstract syntax tree (AST) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on an abstract syntax tree without necessarily using recursion.
In particular, this permits callers to do case analysis with constant stack
usage, which can be important since the size of an abstract syntax tree
may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`](visitor/index.md) function.

Note that the abstract syntax tree for a regular expression is quite
complex. Unless you specifically need it, you might be able to use the much
simpler [high-level intermediate representation](crate::hir::Hir) and its
[corresponding `Visitor` trait](crate::hir::Visitor) instead.

#### Associated Types

- `type Output`

- `type Err`

#### Required Methods

- `fn finish(self) -> Result<<Self as >::Output, <Self as >::Err>`

  All implementors of `Visitor` must provide a `finish` method, which

#### Provided Methods

- `fn start(&mut self)`

  This method is called before beginning traversal of the AST.

- `fn visit_pre(&mut self, _ast: &Ast) -> Result<(), <Self as >::Err>`

  This method is called on an `Ast` before descending into child `Ast`

- `fn visit_post(&mut self, _ast: &Ast) -> Result<(), <Self as >::Err>`

  This method is called on an `Ast` after descending all of its child

- `fn visit_alternation_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of an

- `fn visit_concat_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of a concatenation.

- `fn visit_class_set_item_pre(&mut self, _ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>`

  This method is called on every [`ClassSetItem`](ast::ClassSetItem)

- `fn visit_class_set_item_post(&mut self, _ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>`

  This method is called on every [`ClassSetItem`](ast::ClassSetItem)

- `fn visit_class_set_binary_op_pre(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>`

  This method is called on every

- `fn visit_class_set_binary_op_post(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>`

  This method is called on every

- `fn visit_class_set_binary_op_in(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>`

  This method is called between the left hand and right hand child nodes

#### Implementors

- [`NestLimiter`](parse/index.md#nestlimiter)
- [`TranslatorI`](../hir/translate/index.md#translatori)
- [`Writer`](print/index.md#writer)

## Functions

### `visit`

```rust
fn visit<V: Visitor>(ast: &crate::ast::Ast, visitor: V) -> Result<<V as >::Output, <V as >::Err>
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:118-120`](../../../.source_1765521767/regex-syntax-0.8.8/src/ast/visitor.rs#L118-L120)*

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Ast` while calling the
appropriate methods provided by the [`Visitor`](visitor/index.md) trait.

The primary use case for this method is when one wants to perform case
analysis over an `Ast` without using a stack size proportional to the depth
of the `Ast`. Namely, this method will instead use constant stack size, but
will use heap space proportional to the size of the `Ast`. This may be
desirable in cases where the size of `Ast` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

