*[gimli](../../index.md) / [read](../index.md) / [abbrev](index.md)*

---

# Module `abbrev`

Functions for parsing DWARF debugging abbreviations.

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

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugAbbrev<R>`

- `fn clone(self: &Self) -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugAbbrev<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugAbbrev<R>`

- `fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md)

##### `impl<R> Section for DebugAbbrev<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `AbbreviationsCache`

```rust
struct AbbreviationsCache {
    abbreviations: btree_map::BTreeMap<u64, crate::read::Result<alloc::sync::Arc<Abbreviations>>>,
}
```

A cache of previously parsed `Abbreviations`.

#### Implementations

- `fn new() -> Self`

- `fn populate<R: Reader>(self: &mut Self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](../index.md), [`DebugAbbrev`](../index.md), [`DebugInfoUnitHeadersIter`](../index.md)

- `fn set<R: Reader>(self: &mut Self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../../index.md), [`Reader`](../index.md), [`Abbreviations`](../index.md)

- `fn get<R: Reader>(self: &Self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](../index.md), [`DebugAbbrevOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`Abbreviations`](../index.md)

#### Trait Implementations

##### `impl Debug for AbbreviationsCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AbbreviationsCache`

- `fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](../index.md)

### `Abbreviations`

```rust
struct Abbreviations {
    vec: alloc::vec::Vec<Abbreviation>,
    map: btree_map::BTreeMap<u64, Abbreviation>,
}
```

A set of type abbreviations.

Construct an `Abbreviations` instance with the
`abbreviations()`
method.

#### Implementations

- `fn empty() -> Abbreviations` — [`Abbreviations`](../index.md)

- `fn insert(self: &mut Self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](../index.md)

- `fn get(self: &Self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](../index.md)

- `fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../../index.md), [`Abbreviations`](../index.md)

#### Trait Implementations

##### `impl Clone for Abbreviations`

- `fn clone(self: &Self) -> Abbreviations` — [`Abbreviations`](../index.md)

##### `impl Debug for Abbreviations`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Abbreviations`

- `fn default() -> Abbreviations` — [`Abbreviations`](../index.md)

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

- `fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../../index.md), [`DwChildren`](../../index.md), [`Attributes`](#attributes), [`Abbreviation`](../index.md)

- `fn code(self: &Self) -> u64`

- `fn tag(self: &Self) -> constants::DwTag` — [`DwTag`](../../index.md)

- `fn has_children(self: &Self) -> bool`

- `fn attributes(self: &Self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md)

- `fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../../index.md), [`DwTag`](../../index.md)

- `fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../../index.md), [`DwChildren`](../../index.md)

- `fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../../index.md), [`Attributes`](#attributes)

- `fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../../index.md), [`Abbreviation`](../index.md)

#### Trait Implementations

##### `impl Clone for Abbreviation`

- `fn clone(self: &Self) -> Abbreviation` — [`Abbreviation`](../index.md)

##### `impl Debug for Abbreviation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Abbreviation`

##### `impl PartialEq for Abbreviation`

- `fn eq(self: &Self, other: &Abbreviation) -> bool` — [`Abbreviation`](../index.md)

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

- `fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../../index.md), [`DwForm`](../../index.md), [`AttributeSpecification`](../index.md)

- `fn name(self: &Self) -> constants::DwAt` — [`DwAt`](../../index.md)

- `fn form(self: &Self) -> constants::DwForm` — [`DwForm`](../../index.md)

- `fn implicit_const_value(self: &Self) -> Option<i64>`

- `fn size<R: Reader>(self: &Self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](../index.md)

- `fn parse_form<R: Reader>(input: &mut R) -> Result<constants::DwForm>` — [`Result`](../../index.md), [`DwForm`](../../index.md)

- `fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../../index.md), [`AttributeSpecification`](../index.md)

#### Trait Implementations

##### `impl Clone for AttributeSpecification`

- `fn clone(self: &Self) -> AttributeSpecification` — [`AttributeSpecification`](../index.md)

##### `impl Copy for AttributeSpecification`

##### `impl Debug for AttributeSpecification`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AttributeSpecification`

##### `impl PartialEq for AttributeSpecification`

- `fn eq(self: &Self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](../index.md)

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

- `fn clone(self: &Self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](../index.md)

##### `impl Copy for AbbreviationsCacheStrategy`

##### `impl Debug for AbbreviationsCacheStrategy`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AbbreviationsCacheStrategy`

##### `impl PartialEq for AbbreviationsCacheStrategy`

- `fn eq(self: &Self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](../index.md)

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

- `fn new() -> Attributes` — [`Attributes`](#attributes)

- `fn push(self: &mut Self, attr: AttributeSpecification)` — [`AttributeSpecification`](../index.md)

#### Trait Implementations

##### `impl Clone for Attributes`

- `fn clone(self: &Self) -> Attributes` — [`Attributes`](#attributes)

##### `impl Debug for Attributes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- `type Target = [AttributeSpecification]`

- `fn deref(self: &Self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md)

##### `impl Eq for Attributes`

##### `impl FromIterator for Attributes`

- `fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](#attributes)

##### `impl PartialEq for Attributes`

- `fn eq(self: &Self, other: &Attributes) -> bool` — [`Attributes`](#attributes)

##### `impl<P, T> Receiver for Attributes`

- `type Target = T`

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

