*[gimli](../../index.md) / [read](../index.md) / [abbrev](index.md)*

---

# Module `abbrev`

Functions for parsing DWARF debugging abbreviations.

## Contents

- [Structs](#structs)
  - [`DebugAbbrev`](#debugabbrev)
  - [`AbbreviationsCache`](#abbreviationscache)
  - [`Abbreviations`](#abbreviations)
  - [`Abbreviation`](#abbreviation)
  - [`AttributeSpecification`](#attributespecification)
- [Enums](#enums)
  - [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)
  - [`Attributes`](#attributes)
- [Functions](#functions)
  - [`get_attribute_size`](#get_attribute_size)
- [Constants](#constants)
  - [`MAX_ATTRIBUTES_INLINE`](#max_attributes_inline)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAbbrev`](#debugabbrev) | struct | The `DebugAbbrev` struct represents the abbreviations describing |
| [`AbbreviationsCache`](#abbreviationscache) | struct | A cache of previously parsed `Abbreviations`. |
| [`Abbreviations`](#abbreviations) | struct | A set of type abbreviations. |
| [`Abbreviation`](#abbreviation) | struct | An abbreviation describes the shape of a `DebuggingInformationEntry`'s type |
| [`AttributeSpecification`](#attributespecification) | struct | The description of an attribute in an abbreviated type. |
| [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy) | enum | The strategy to use for caching abbreviations. |
| [`Attributes`](#attributes) | enum | A list of attributes found in an `Abbreviation` |
| [`get_attribute_size`](#get_attribute_size) | fn |  |
| [`MAX_ATTRIBUTES_INLINE`](#max_attributes_inline) | const |  |

## Structs

### `DebugAbbrev<R>`

```rust
struct DebugAbbrev<R> {
    debug_abbrev_section: R,
}
```

The `DebugAbbrev` struct represents the abbreviations describing
`DebuggingInformationEntry`s' attribute names and forms found in the
`.debug_abbrev` section.

#### Implementations

- <span id="debugabbrev-new"></span>`fn new(debug_abbrev_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAbbrev<R>`

- <span id="debugabbrev-clone"></span>`fn clone(&self) -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md)

##### `impl<R: marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: fmt::Debug> Debug for DebugAbbrev<R>`

- <span id="debugabbrev-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAbbrev<R>`

- <span id="debugabbrev-default"></span>`fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md)

##### `impl<R> Section for DebugAbbrev<R>`

- <span id="debugabbrev-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md)

- <span id="debugabbrev-reader"></span>`fn reader(&self) -> &R`

### `AbbreviationsCache`

```rust
struct AbbreviationsCache {
    abbreviations: btree_map::BTreeMap<u64, crate::read::Result<alloc::sync::Arc<Abbreviations>>>,
}
```

A cache of previously parsed `Abbreviations`.

#### Implementations

- <span id="abbreviationscache-new"></span>`fn new() -> Self`

- <span id="abbreviationscache-populate"></span>`fn populate<R: Reader>(&mut self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](../index.md), [`DebugAbbrev`](../index.md), [`DebugInfoUnitHeadersIter`](../index.md)

- <span id="abbreviationscache-set"></span>`fn set<R: Reader>(&mut self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../../index.md), [`Reader`](../index.md), [`Abbreviations`](../index.md)

- <span id="abbreviationscache-get"></span>`fn get<R: Reader>(&self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](../index.md), [`DebugAbbrevOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`Abbreviations`](../index.md)

#### Trait Implementations

##### `impl Debug for AbbreviationsCache`

- <span id="abbreviationscache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AbbreviationsCache`

- <span id="abbreviationscache-default"></span>`fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](../index.md)

### `Abbreviations`

```rust
struct Abbreviations {
    vec: alloc::vec::Vec<Abbreviation>,
    map: btree_map::BTreeMap<u64, Abbreviation>,
}
```

A set of type abbreviations.

Construct an `Abbreviations` instance with the
[`abbreviations()`](#unitheader-abbreviations)
method.

#### Implementations

- <span id="abbreviations-empty"></span>`fn empty() -> Abbreviations` — [`Abbreviations`](../index.md)

- <span id="abbreviations-insert"></span>`fn insert(&mut self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](../index.md)

- <span id="abbreviations-get"></span>`fn get(&self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](../index.md)

- <span id="abbreviations-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../../index.md), [`Abbreviations`](../index.md)

#### Trait Implementations

##### `impl Clone for Abbreviations`

- <span id="abbreviations-clone"></span>`fn clone(&self) -> Abbreviations` — [`Abbreviations`](../index.md)

##### `impl Debug for Abbreviations`

- <span id="abbreviations-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abbreviations`

- <span id="abbreviations-default"></span>`fn default() -> Abbreviations` — [`Abbreviations`](../index.md)

### `Abbreviation`

```rust
struct Abbreviation {
    code: u64,
    tag: constants::DwTag,
    has_children: constants::DwChildren,
    attributes: Attributes,
}
```

An abbreviation describes the shape of a `DebuggingInformationEntry`'s type:
its code, tag type, whether it has children, and its set of attributes.

#### Implementations

- <span id="abbreviation-new"></span>`fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../../index.md), [`DwChildren`](../../index.md), [`Attributes`](#attributes), [`Abbreviation`](../index.md)

- <span id="abbreviation-code"></span>`fn code(&self) -> u64`

- <span id="abbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../../index.md)

- <span id="abbreviation-has-children"></span>`fn has_children(&self) -> bool`

- <span id="abbreviation-attributes"></span>`fn attributes(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md)

- <span id="abbreviation-parse-tag"></span>`fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../../index.md), [`DwTag`](../../index.md)

- <span id="abbreviation-parse-has-children"></span>`fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../../index.md), [`DwChildren`](../../index.md)

- <span id="abbreviation-parse-attributes"></span>`fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../../index.md), [`Attributes`](#attributes)

- <span id="abbreviation-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../../index.md), [`Abbreviation`](../index.md)

#### Trait Implementations

##### `impl Clone for Abbreviation`

- <span id="abbreviation-clone"></span>`fn clone(&self) -> Abbreviation` — [`Abbreviation`](../index.md)

##### `impl Debug for Abbreviation`

- <span id="abbreviation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Abbreviation`

##### `impl PartialEq for Abbreviation`

- <span id="abbreviation-eq"></span>`fn eq(&self, other: &Abbreviation) -> bool` — [`Abbreviation`](../index.md)

##### `impl StructuralPartialEq for Abbreviation`

### `AttributeSpecification`

```rust
struct AttributeSpecification {
    name: constants::DwAt,
    form: constants::DwForm,
    implicit_const_value: i64,
}
```

The description of an attribute in an abbreviated type. It is a pair of name
and form.

#### Implementations

- <span id="attributespecification-new"></span>`fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../../index.md), [`DwForm`](../../index.md), [`AttributeSpecification`](../index.md)

- <span id="attributespecification-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../../index.md)

- <span id="attributespecification-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../../index.md)

- <span id="attributespecification-implicit-const-value"></span>`fn implicit_const_value(&self) -> Option<i64>`

- <span id="attributespecification-size"></span>`fn size<R: Reader>(&self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](../index.md)

- <span id="attributespecification-parse-form"></span>`fn parse_form<R: Reader>(input: &mut R) -> Result<constants::DwForm>` — [`Result`](../../index.md), [`DwForm`](../../index.md)

- <span id="attributespecification-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../../index.md), [`AttributeSpecification`](../index.md)

#### Trait Implementations

##### `impl Clone for AttributeSpecification`

- <span id="attributespecification-clone"></span>`fn clone(&self) -> AttributeSpecification` — [`AttributeSpecification`](../index.md)

##### `impl Copy for AttributeSpecification`

##### `impl Debug for AttributeSpecification`

- <span id="attributespecification-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AttributeSpecification`

##### `impl PartialEq for AttributeSpecification`

- <span id="attributespecification-eq"></span>`fn eq(&self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](../index.md)

##### `impl StructuralPartialEq for AttributeSpecification`

## Enums

### `AbbreviationsCacheStrategy`

```rust
enum AbbreviationsCacheStrategy {
    Duplicates,
    All,
}
```

The strategy to use for caching abbreviations.

#### Variants

- **`Duplicates`**

  Cache abbreviations that are used more than once.
  
  This is useful if the units in the `.debug_info` section will be parsed only once.

- **`All`**

  Cache all abbreviations.
  
  This is useful if the units in the `.debug_info` section will be parsed more than once.

#### Trait Implementations

##### `impl Clone for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-clone"></span>`fn clone(&self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](../index.md)

##### `impl Copy for AbbreviationsCacheStrategy`

##### `impl Debug for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbbreviationsCacheStrategy`

##### `impl PartialEq for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-eq"></span>`fn eq(&self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](../index.md)

##### `impl StructuralPartialEq for AbbreviationsCacheStrategy`

### `Attributes`

```rust
enum Attributes {
    Inline {
        buf: [AttributeSpecification; 5],
        len: usize,
    },
    Heap(alloc::vec::Vec<AttributeSpecification>),
}
```

A list of attributes found in an `Abbreviation`

#### Implementations

- <span id="attributes-new"></span>`fn new() -> Attributes` — [`Attributes`](#attributes)

- <span id="attributes-push"></span>`fn push(&mut self, attr: AttributeSpecification)` — [`AttributeSpecification`](../index.md)

#### Trait Implementations

##### `impl Clone for Attributes`

- <span id="attributes-clone"></span>`fn clone(&self) -> Attributes` — [`Attributes`](#attributes)

##### `impl Debug for Attributes`

- <span id="attributes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- <span id="attributes-target"></span>`type Target = [AttributeSpecification]`

- <span id="attributes-deref"></span>`fn deref(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md)

##### `impl Eq for Attributes`

##### `impl FromIterator for Attributes`

- <span id="attributes-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](#attributes)

##### `impl PartialEq for Attributes`

- <span id="attributes-eq"></span>`fn eq(&self, other: &Attributes) -> bool` — [`Attributes`](#attributes)

##### `impl<P, T> Receiver for Attributes`

- <span id="attributes-target"></span>`type Target = T`

## Functions

### `get_attribute_size`

```rust
fn get_attribute_size(form: constants::DwForm, encoding: crate::common::Encoding) -> Option<u8>
```

## Constants

### `MAX_ATTRIBUTES_INLINE`

```rust
const MAX_ATTRIBUTES_INLINE: usize = 5usize;
```

