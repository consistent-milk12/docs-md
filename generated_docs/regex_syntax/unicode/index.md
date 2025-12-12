*[regex_syntax](../index.md) / [unicode](index.md)*

---

# Module `unicode`

## Contents

- [Structs](#structs)
  - [`CaseFoldError`](#casefolderror)
  - [`UnicodeWordError`](#unicodeworderror)
  - [`SimpleCaseFolder`](#simplecasefolder)
- [Enums](#enums)
  - [`Error`](#error)
  - [`ClassQuery`](#classquery)
  - [`CanonicalClassQuery`](#canonicalclassquery)
- [Functions](#functions)
  - [`class`](#class)
  - [`perl_word`](#perl-word)
  - [`perl_space`](#perl-space)
  - [`perl_digit`](#perl-digit)
  - [`hir_class`](#hir-class)
  - [`is_word_character`](#is-word-character)
  - [`canonical_gencat`](#canonical-gencat)
  - [`canonical_script`](#canonical-script)
  - [`canonical_prop`](#canonical-prop)
  - [`canonical_value`](#canonical-value)
  - [`property_values`](#property-values)
  - [`property_set`](#property-set)
  - [`ages`](#ages)
  - [`gencat`](#gencat)
  - [`script`](#script)
  - [`script_extension`](#script-extension)
  - [`bool_property`](#bool-property)
  - [`gcb`](#gcb)
  - [`wb`](#wb)
  - [`sb`](#sb)
  - [`symbolic_name_normalize`](#symbolic-name-normalize)
  - [`symbolic_name_normalize_bytes`](#symbolic-name-normalize-bytes)
- [Type Aliases](#type-aliases)
  - [`Range`](#range)
  - [`PropertyValues`](#propertyvalues)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CaseFoldError`](#casefolderror) | struct | An error that occurs when Unicode-aware simple case folding fails. |
| [`UnicodeWordError`](#unicodeworderror) | struct | An error that occurs when the Unicode-aware `\w` class is unavailable. |
| [`SimpleCaseFolder`](#simplecasefolder) | struct | A state oriented traverser of the simple case folding table. |
| [`Error`](#error) | enum | An error that occurs when dealing with Unicode. |
| [`ClassQuery`](#classquery) | enum | A query for finding a character class defined by Unicode. |
| [`CanonicalClassQuery`](#canonicalclassquery) | enum | Like ClassQuery, but its parameters have been canonicalized. |
| [`class`](#class) | fn | Looks up a Unicode class given a query. |
| [`perl_word`](#perl-word) | fn | Returns a Unicode aware class for \w. |
| [`perl_space`](#perl-space) | fn | Returns a Unicode aware class for \s. |
| [`perl_digit`](#perl-digit) | fn | Returns a Unicode aware class for \d. |
| [`hir_class`](#hir-class) | fn | Build a Unicode HIR class from a sequence of Unicode scalar value ranges. |
| [`is_word_character`](#is-word-character) | fn | Returns true only if the given codepoint is in the `\w` character class. |
| [`canonical_gencat`](#canonical-gencat) | fn |  |
| [`canonical_script`](#canonical-script) | fn |  |
| [`canonical_prop`](#canonical-prop) | fn | Find the canonical property name for the given normalized property name. |
| [`canonical_value`](#canonical-value) | fn | Find the canonical property value for the given normalized property value. |
| [`property_values`](#property-values) | fn | Return the table of property values for the given property name. |
| [`property_set`](#property-set) | fn |  |
| [`ages`](#ages) | fn | Returns an iterator over Unicode Age sets. |
| [`gencat`](#gencat) | fn | Returns the Unicode HIR class corresponding to the given general category. |
| [`script`](#script) | fn | Returns the Unicode HIR class corresponding to the given script. |
| [`script_extension`](#script-extension) | fn | Returns the Unicode HIR class corresponding to the given script extension. |
| [`bool_property`](#bool-property) | fn | Returns the Unicode HIR class corresponding to the given Unicode boolean property. |
| [`gcb`](#gcb) | fn | Returns the Unicode HIR class corresponding to the given grapheme cluster break property. |
| [`wb`](#wb) | fn | Returns the Unicode HIR class corresponding to the given word break property. |
| [`sb`](#sb) | fn | Returns the Unicode HIR class corresponding to the given sentence break property. |
| [`symbolic_name_normalize`](#symbolic-name-normalize) | fn | Like symbolic_name_normalize_bytes, but operates on a string. |
| [`symbolic_name_normalize_bytes`](#symbolic-name-normalize-bytes) | fn | Normalize the given symbolic name in place according to UAX44-LM3. |
| [`Range`](#range) | type | An inclusive range of codepoints from a generated file (hence the static lifetime). |
| [`PropertyValues`](#propertyvalues) | type | A mapping of property values for a specific property. |

## Structs

### `CaseFoldError`

```rust
struct CaseFoldError(());
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:31`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L31)*

An error that occurs when Unicode-aware simple case folding fails.

This error can occur when the case mapping tables necessary for Unicode
aware case folding are unavailable. This only occurs when the
`unicode-case` feature is disabled. (The feature is enabled by default.)

#### Trait Implementations

##### `impl Debug for CaseFoldError`

- <span id="casefolderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CaseFoldError`

- <span id="casefolderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CaseFoldError`

##### `impl ToString for CaseFoldError`

- <span id="casefolderror-to-string"></span>`fn to_string(&self) -> String`

### `UnicodeWordError`

```rust
struct UnicodeWordError(());
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:52`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L52)*

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. This only occurs when the
`unicode-perl` feature is disabled. (The feature is enabled by default.)

#### Trait Implementations

##### `impl Debug for UnicodeWordError`

- <span id="unicodeworderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for UnicodeWordError`

- <span id="unicodeworderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for UnicodeWordError`

##### `impl ToString for UnicodeWordError`

- <span id="unicodeworderror-to-string"></span>`fn to_string(&self) -> String`

### `SimpleCaseFolder`

```rust
struct SimpleCaseFolder {
    table: &'static [(char, &'static [char])],
    last: Option<char>,
    next: usize,
}
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:81-93`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L81-L93)*

A state oriented traverser of the simple case folding table.

A case folder can be constructed via `SimpleCaseFolder::new()`, which will
return an error if the underlying case folding table is unavailable.

After construction, it is expected that callers will use
`SimpleCaseFolder::mapping` by calling it with codepoints in strictly
increasing order. For example, calling it on `b` and then on `a` is illegal
and will result in a panic.

The main idea of this type is that it tries hard to make mapping lookups
fast by exploiting the structure of the underlying table, and the ordering
assumption enables this.

#### Fields

- **`table`**: `&'static [(char, &'static [char])]`

  The simple case fold table. It's a sorted association list, where the
  keys are Unicode scalar values and the values are the corresponding
  equivalence class (not including the key) of the "simple" case folded
  Unicode scalar values.

- **`last`**: `Option<char>`

  The last codepoint that was used for a lookup.

- **`next`**: `usize`

  The index to the entry in `table` corresponding to the smallest key `k`
  such that `k > k0`, where `k0` is the most recent key lookup. Note that
  in particular, `k0` may not be in the table!

#### Implementations

- <span id="simplecasefolder-new"></span>`fn new() -> Result<SimpleCaseFolder, CaseFoldError>` — [`SimpleCaseFolder`](#simplecasefolder), [`CaseFoldError`](#casefolderror)

- <span id="simplecasefolder-mapping"></span>`fn mapping(&mut self, c: char) -> &'static [char]`

- <span id="simplecasefolder-overlaps"></span>`fn overlaps(&self, start: char, end: char) -> bool`

- <span id="simplecasefolder-get"></span>`fn get(&self, c: char) -> Result<usize, usize>`

#### Trait Implementations

##### `impl Debug for SimpleCaseFolder`

- <span id="simplecasefolder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Error`

```rust
enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
    PerlClassNotFound,
}
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:17-23`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L17-L23)*

An error that occurs when dealing with Unicode.

We don't impl the Error trait here because these always get converted
into other public errors. (This error type isn't exported.)

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ClassQuery<'a>`

```rust
enum ClassQuery<'a> {
    OneLetter(char),
    Binary(&'a str),
    ByValue {
        property_name: &'a str,
        property_value: &'a str,
    },
}
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:216-235`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L216-L235)*

A query for finding a character class defined by Unicode. This supports
either use of a property name directly, or lookup by property value. The
former generally refers to Binary properties (see UTS#44, Table 8), but
as a special exception (see UTS#18, Section 1.2) both general categories
(an enumeration) and scripts (a catalog) are supported as if each of their
possible values were a binary property.

In all circumstances, property names and values are normalized and
canonicalized. That is, `GC == gc == GeneralCategory == general_category`.

The lifetime `'a` refers to the shorter of the lifetimes of property name
and property value.

#### Variants

- **`OneLetter`**

  Return a class corresponding to a Unicode binary property, named by
  a single letter.

- **`Binary`**

  Return a class corresponding to a Unicode binary property.
  
  Note that, by special exception (see UTS#18, Section 1.2), both
  general category values and script values are permitted here as if
  they were a binary property.

- **`ByValue`**

  Return a class corresponding to all codepoints whose property
  (identified by `property_name`) corresponds to the given value
  (identified by `property_value`).

#### Implementations

- <span id="classquery-canonicalize"></span>`fn canonicalize(&self) -> Result<CanonicalClassQuery, Error>` — [`CanonicalClassQuery`](#canonicalclassquery), [`Error`](#error)

- <span id="classquery-canonical-binary"></span>`fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery, Error>` — [`CanonicalClassQuery`](#canonicalclassquery), [`Error`](#error)

#### Trait Implementations

##### `impl Debug for ClassQuery<'a>`

- <span id="classquery-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CanonicalClassQuery`

```rust
enum CanonicalClassQuery {
    Binary(&'static str),
    GeneralCategory(&'static str),
    Script(&'static str),
    ByValue {
        property_name: &'static str,
        property_value: &'static str,
    },
}
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:328-347`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L328-L347)*

Like ClassQuery, but its parameters have been canonicalized. This also
differentiates binary properties from flattened general categories and
scripts.

#### Variants

- **`Binary`**

  The canonical binary property name.

- **`GeneralCategory`**

  The canonical general category name.

- **`Script`**

  The canonical script name.

- **`ByValue`**

  An arbitrary association between property and value, both of which
  have been canonicalized.
  
  Note that by construction, the property name of ByValue will never
  be General_Category or Script. Those two cases are subsumed by the
  eponymous variants.

#### Trait Implementations

##### `impl Debug for CanonicalClassQuery`

- <span id="canonicalclassquery-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CanonicalClassQuery`

##### `impl PartialEq for CanonicalClassQuery`

- <span id="canonicalclassquery-eq"></span>`fn eq(&self, other: &CanonicalClassQuery) -> bool` — [`CanonicalClassQuery`](#canonicalclassquery)

##### `impl StructuralPartialEq for CanonicalClassQuery`

## Functions

### `class`

```rust
fn class(query: ClassQuery<'_>) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:351-383`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L351-L383)*

Looks up a Unicode class given a query. If one doesn't exist, then
`None` is returned.

### `perl_word`

```rust
fn perl_word() -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:388-401`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L388-L401)*

Returns a Unicode aware class for \w.

This returns an error if the data is not available for \w.

### `perl_space`

```rust
fn perl_space() -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:406-425`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L406-L425)*

Returns a Unicode aware class for \s.

This returns an error if the data is not available for \s.

### `perl_digit`

```rust
fn perl_digit() -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:430-449`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L430-L449)*

Returns a Unicode aware class for \d.

This returns an error if the data is not available for \d.

### `hir_class`

```rust
fn hir_class(ranges: &[(char, char)]) -> hir::ClassUnicode
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:452-458`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L452-L458)*

Build a Unicode HIR class from a sequence of Unicode scalar value ranges.

### `is_word_character`

```rust
fn is_word_character(c: char) -> Result<bool, UnicodeWordError>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:463-492`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L463-L492)*

Returns true only if the given codepoint is in the `\w` character class.

If the `unicode-perl` feature is not enabled, then this returns an error.

### `canonical_gencat`

```rust
fn canonical_gencat(normalized_value: &str) -> Result<Option<&'static str>, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:501-513`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L501-L513)*

### `canonical_script`

```rust
fn canonical_script(normalized_value: &str) -> Result<Option<&'static str>, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:515-520`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L515-L520)*

### `canonical_prop`

```rust
fn canonical_prop(normalized_name: &str) -> Result<Option<&'static str>, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:530-563`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L530-L563)*

Find the canonical property name for the given normalized property name.

If no such property exists, then `None` is returned.

The normalized property name must have been normalized according to
UAX44 LM3, which can be done using `symbolic_name_normalize`.

If the property names data is not available, then an error is returned.

### `canonical_value`

```rust
fn canonical_value(vals: &'static [(&'static str, &'static str)], normalized_value: &str) -> Option<&'static str>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:575-582`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L575-L582)*

Find the canonical property value for the given normalized property
value.

The given property values should correspond to the values for the property
under question, which can be found using `property_values`.

If no such property value exists, then `None` is returned.

The normalized property value must have been normalized according to
UAX44 LM3, which can be done using `symbolic_name_normalize`.

### `property_values`

```rust
fn property_values(canonical_property_name: &'static str) -> Result<Option<&'static [(&'static str, &'static str)]>, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:587-620`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L587-L620)*

Return the table of property values for the given property name.

If the property values data is not available, then an error is returned.

### `property_set`

```rust
fn property_set(name_map: &'static [(&'static str, &'static [(char, char)])], canonical: &'static str) -> Option<&'static [(char, char)]>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:625-633`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L625-L633)*

### `ages`

```rust
fn ages(canonical_age: &str) -> Result<impl Iterator<Item = &'static [(char, char)]>, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:641-691`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L641-L691)*

Returns an iterator over Unicode Age sets. Each item corresponds to a set
of codepoints that were added in a particular revision of Unicode. The
iterator yields items in chronological order.

If the given age value isn't valid or if the data isn't available, then an
error is returned instead.

### `gencat`

```rust
fn gencat(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:699-726`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L699-L726)*

Returns the Unicode HIR class corresponding to the given general category.

Name canonicalization is assumed to be performed by the caller.

If the given general category could not be found, or if the general
category data is not available, then an error is returned.

### `script`

```rust
fn script(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:734-749`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L734-L749)*

Returns the Unicode HIR class corresponding to the given script.

Name canonicalization is assumed to be performed by the caller.

If the given script could not be found, or if the script data is not
available, then an error is returned.

### `script_extension`

```rust
fn script_extension(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:757-774`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L757-L774)*

Returns the Unicode HIR class corresponding to the given script extension.

Name canonicalization is assumed to be performed by the caller.

If the given script extension could not be found, or if the script data is
not available, then an error is returned.

### `bool_property`

```rust
fn bool_property(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:783-804`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L783-L804)*

Returns the Unicode HIR class corresponding to the given Unicode boolean
property.

Name canonicalization is assumed to be performed by the caller.

If the given boolean property could not be found, or if the boolean
property data is not available, then an error is returned.

### `gcb`

```rust
fn gcb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:813-828`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L813-L828)*

Returns the Unicode HIR class corresponding to the given grapheme cluster
break property.

Name canonicalization is assumed to be performed by the caller.

If the given property could not be found, or if the corresponding data is
not available, then an error is returned.

### `wb`

```rust
fn wb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:837-852`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L837-L852)*

Returns the Unicode HIR class corresponding to the given word break
property.

Name canonicalization is assumed to be performed by the caller.

If the given property could not be found, or if the corresponding data is
not available, then an error is returned.

### `sb`

```rust
fn sb(canonical_name: &'static str) -> Result<hir::ClassUnicode, Error>
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:861-876`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L861-L876)*

Returns the Unicode HIR class corresponding to the given sentence
break property.

Name canonicalization is assumed to be performed by the caller.

If the given property could not be found, or if the corresponding data is
not available, then an error is returned.

### `symbolic_name_normalize`

```rust
fn symbolic_name_normalize(x: &str) -> alloc::string::String
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:879-890`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L879-L890)*

Like symbolic_name_normalize_bytes, but operates on a string.

### `symbolic_name_normalize_bytes`

```rust
fn symbolic_name_normalize_bytes(slice: &mut [u8]) -> &mut [u8]
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:902-945`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L902-L945)*

Normalize the given symbolic name in place according to UAX44-LM3.

A "symbolic name" typically corresponds to property names and property
value aliases. Note, though, that it should not be applied to property
string values.

The slice returned is guaranteed to be valid UTF-8 for all possible values
of `slice`.

See: https://unicode.org/reports/tr44/#UAX44-LM3

## Type Aliases

### `Range`

```rust
type Range = &'static [(char, char)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:10`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L10)*

An inclusive range of codepoints from a generated file (hence the static
lifetime).

### `PropertyValues`

```rust
type PropertyValues = &'static [(&'static str, &'static str)];
```

*Defined in [`regex-syntax-0.8.8/src/unicode.rs:499`](../../../.source_1765521767/regex-syntax-0.8.8/src/unicode.rs#L499)*

A mapping of property values for a specific property.

The first element of each tuple is a normalized property value while the
second element of each tuple is the corresponding canonical property
value.

