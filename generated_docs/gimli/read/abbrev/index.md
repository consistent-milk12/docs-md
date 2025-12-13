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
  - [`get_attribute_size`](#get-attribute-size)
- [Constants](#constants)
  - [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAbbrev`](#debugabbrev) | struct | The `DebugAbbrev` struct represents the abbreviations describing `DebuggingInformationEntry`s' attribute names and forms found in the `.debug_abbrev` section. |
| [`AbbreviationsCache`](#abbreviationscache) | struct | A cache of previously parsed `Abbreviations`. |
| [`Abbreviations`](#abbreviations) | struct | A set of type abbreviations. |
| [`Abbreviation`](#abbreviation) | struct | An abbreviation describes the shape of a `DebuggingInformationEntry`'s type: its code, tag type, whether it has children, and its set of attributes. |
| [`AttributeSpecification`](#attributespecification) | struct | The description of an attribute in an abbreviated type. |
| [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy) | enum | The strategy to use for caching abbreviations. |
| [`Attributes`](#attributes) | enum | A list of attributes found in an `Abbreviation` |
| [`get_attribute_size`](#get-attribute-size) | fn |  |
| [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline) | const |  |

## Structs

### `DebugAbbrev<R>`

```rust
struct DebugAbbrev<R> {
    debug_abbrev_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:22-24`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L22-L24)*

The `DebugAbbrev` struct represents the abbreviations describing
`DebuggingInformationEntry`s' attribute names and forms found in the
`.debug_abbrev` section.

#### Implementations

- <span id="debugabbrev-new"></span>`fn new(debug_abbrev_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugAbbrev` instance from the data in the `.debug_abbrev`

  section.

  

  It is the caller's responsibility to read the `.debug_abbrev` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugAbbrev, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_abbrev_section_somehow = || &buf;

  let debug_abbrev = DebugAbbrev::new(read_debug_abbrev_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugAbbrev<R>`

- <span id="debugabbrev-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAbbrev<R>`

- <span id="debugabbrev-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAbbrev<R>`

- <span id="debugabbrev-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugAbbrev<R>`

- <span id="debugabbrev-clone"></span>`fn clone(&self) -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md#debugabbrev)

##### `impl CloneToUninit for DebugAbbrev<R>`

- <span id="debugabbrev-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: fmt::Debug> Debug for DebugAbbrev<R>`

- <span id="debugabbrev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAbbrev<R>`

- <span id="debugabbrev-default"></span>`fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md#debugabbrev)

##### `impl<T> From for DebugAbbrev<R>`

- <span id="debugabbrev-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugAbbrev<R>`

- <span id="debugabbrev-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugAbbrev<R>`

- <span id="debugabbrev-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugabbrev-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugAbbrev<R>`

- <span id="debugabbrev-toowned-type-owned"></span>`type Owned = T`

- <span id="debugabbrev-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugabbrev-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugAbbrev<R>`

- <span id="debugabbrev-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugabbrev-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugAbbrev<R>`

- <span id="debugabbrev-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugabbrev-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AbbreviationsCache`

```rust
struct AbbreviationsCache {
    abbreviations: btree_map::BTreeMap<u64, crate::read::Result<alloc::sync::Arc<Abbreviations>>>,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:112-114`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L112-L114)*

A cache of previously parsed `Abbreviations`.

#### Implementations

- <span id="abbreviationscache-new"></span>`fn new() -> Self`

  Create an empty abbreviations cache.

- <span id="abbreviationscache-populate"></span>`fn populate<R: Reader>(&mut self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](../index.md#abbreviationscachestrategy), [`DebugAbbrev`](../index.md#debugabbrev), [`DebugInfoUnitHeadersIter`](../index.md#debuginfounitheadersiter)

  Parse abbreviations and store them in the cache.

  

  This will iterate over the given units to determine the abbreviations

  offsets. Any existing cache entries are discarded.

  

  Errors during parsing abbreviations are also stored in the cache.

  Errors during iterating over the units are ignored.

- <span id="abbreviationscache-set"></span>`fn set<R: Reader>(&mut self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../../index.md#debugabbrevoffset), [`Reader`](../index.md#reader), [`Abbreviations`](../index.md#abbreviations)

  Set an entry in the abbreviations cache.

  

  This is only required if you want to manually populate the cache.

- <span id="abbreviationscache-get"></span>`fn get<R: Reader>(&self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](../index.md#debugabbrev), [`DebugAbbrevOffset`](../../index.md#debugabbrevoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`Abbreviations`](../index.md#abbreviations)

  Parse the abbreviations at the given offset.

  

  This uses the cache if possible, but does not update it.

#### Trait Implementations

##### `impl Any for AbbreviationsCache`

- <span id="abbreviationscache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AbbreviationsCache`

- <span id="abbreviationscache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AbbreviationsCache`

- <span id="abbreviationscache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AbbreviationsCache`

- <span id="abbreviationscache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AbbreviationsCache`

- <span id="abbreviationscache-default"></span>`fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](../index.md#abbreviationscache)

##### `impl<T> From for AbbreviationsCache`

- <span id="abbreviationscache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AbbreviationsCache`

- <span id="abbreviationscache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AbbreviationsCache`

- <span id="abbreviationscache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abbreviationscache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AbbreviationsCache`

- <span id="abbreviationscache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abbreviationscache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Abbreviations`

```rust
struct Abbreviations {
    vec: alloc::vec::Vec<Abbreviation>,
    map: btree_map::BTreeMap<u64, Abbreviation>,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:206-209`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L206-L209)*

A set of type abbreviations.

Construct an `Abbreviations` instance with the
[`abbreviations()`](#unitheader-abbreviations)
method.

#### Implementations

- <span id="abbreviations-empty"></span>`fn empty() -> Abbreviations` — [`Abbreviations`](../index.md#abbreviations)

  Construct a new, empty set of abbreviations.

- <span id="abbreviations-insert"></span>`fn insert(&mut self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](../index.md#abbreviation)

  Insert an abbreviation into the set.

  

  Returns `Ok` if it is the first abbreviation in the set with its code,

  `Err` if the code is a duplicate and there already exists an

  abbreviation in the set with the given abbreviation's code.

- <span id="abbreviations-get"></span>`fn get(&self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](../index.md#abbreviation)

  Get the abbreviation associated with the given code.

- <span id="abbreviations-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../../index.md#result), [`Abbreviations`](../index.md#abbreviations)

  Parse a series of abbreviations, terminated by a null abbreviation.

#### Trait Implementations

##### `impl Any for Abbreviations`

- <span id="abbreviations-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abbreviations`

- <span id="abbreviations-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abbreviations`

- <span id="abbreviations-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Abbreviations`

- <span id="abbreviations-clone"></span>`fn clone(&self) -> Abbreviations` — [`Abbreviations`](../index.md#abbreviations)

##### `impl CloneToUninit for Abbreviations`

- <span id="abbreviations-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Abbreviations`

- <span id="abbreviations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abbreviations`

- <span id="abbreviations-default"></span>`fn default() -> Abbreviations` — [`Abbreviations`](../index.md#abbreviations)

##### `impl<T> From for Abbreviations`

- <span id="abbreviations-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Abbreviations`

- <span id="abbreviations-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Abbreviations`

- <span id="abbreviations-toowned-type-owned"></span>`type Owned = T`

- <span id="abbreviations-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abbreviations-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Abbreviations`

- <span id="abbreviations-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abbreviations-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Abbreviations`

- <span id="abbreviations-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abbreviations-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Abbreviation`

```rust
struct Abbreviation {
    code: u64,
    tag: constants::DwTag,
    has_children: constants::DwChildren,
    attributes: Attributes,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:282-287`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L282-L287)*

An abbreviation describes the shape of a `DebuggingInformationEntry`'s type:
its code, tag type, whether it has children, and its set of attributes.

#### Implementations

- <span id="abbreviation-new"></span>`fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../../index.md#dwtag), [`DwChildren`](../../index.md#dwchildren), [`Attributes`](#attributes), [`Abbreviation`](../index.md#abbreviation)

  Construct a new `Abbreviation`.

  

  ### Panics

  

  Panics if `code` is `0`.

- <span id="abbreviation-code"></span>`fn code(&self) -> u64`

  Get this abbreviation's code.

- <span id="abbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../../index.md#dwtag)

  Get this abbreviation's tag.

- <span id="abbreviation-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this abbreviation's type has children, false otherwise.

- <span id="abbreviation-attributes"></span>`fn attributes(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md#attributespecification)

  Get this abbreviation's attributes.

- <span id="abbreviation-parse-tag"></span>`fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../../index.md#result), [`DwTag`](../../index.md#dwtag)

  Parse an abbreviation's tag.

- <span id="abbreviation-parse-has-children"></span>`fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../../index.md#result), [`DwChildren`](../../index.md#dwchildren)

  Parse an abbreviation's "does the type have children?" byte.

- <span id="abbreviation-parse-attributes"></span>`fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../../index.md#result), [`Attributes`](#attributes)

  Parse a series of attribute specifications, terminated by a null attribute

  specification.

- <span id="abbreviation-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../../index.md#result), [`Abbreviation`](../index.md#abbreviation)

  Parse an abbreviation. Return `None` for the null abbreviation, `Some`

  for an actual abbreviation.

#### Trait Implementations

##### `impl Any for Abbreviation`

- <span id="abbreviation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abbreviation`

- <span id="abbreviation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abbreviation`

- <span id="abbreviation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Abbreviation`

- <span id="abbreviation-clone"></span>`fn clone(&self) -> Abbreviation` — [`Abbreviation`](../index.md#abbreviation)

##### `impl CloneToUninit for Abbreviation`

- <span id="abbreviation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Abbreviation`

- <span id="abbreviation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Abbreviation`

##### `impl<T> From for Abbreviation`

- <span id="abbreviation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Abbreviation`

- <span id="abbreviation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Abbreviation`

- <span id="abbreviation-partialeq-eq"></span>`fn eq(&self, other: &Abbreviation) -> bool` — [`Abbreviation`](../index.md#abbreviation)

##### `impl StructuralPartialEq for Abbreviation`

##### `impl ToOwned for Abbreviation`

- <span id="abbreviation-toowned-type-owned"></span>`type Owned = T`

- <span id="abbreviation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abbreviation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Abbreviation`

- <span id="abbreviation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abbreviation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Abbreviation`

- <span id="abbreviation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abbreviation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributeSpecification`

```rust
struct AttributeSpecification {
    name: constants::DwAt,
    form: constants::DwForm,
    implicit_const_value: i64,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:479-483`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L479-L483)*

The description of an attribute in an abbreviated type. It is a pair of name
and form.

#### Implementations

- <span id="attributespecification-new"></span>`fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../../index.md#dwat), [`DwForm`](../../index.md#dwform), [`AttributeSpecification`](../index.md#attributespecification)

  Construct a new `AttributeSpecification` from the given name and form

  and implicit const value.

- <span id="attributespecification-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../../index.md#dwat)

  Get the attribute's name.

- <span id="attributespecification-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../../index.md#dwform)

  Get the attribute's form.

- <span id="attributespecification-implicit-const-value"></span>`fn implicit_const_value(&self) -> Option<i64>`

  Get the attribute's implicit const value.

- <span id="attributespecification-size"></span>`fn size<R: Reader>(&self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](../index.md#unitheader)

  Return the size of the attribute, in bytes.

  

  Note that because some attributes are variably sized, the size cannot

  always be known without parsing, in which case we return `None`.

- <span id="attributespecification-parse-form"></span>`fn parse_form<R: Reader>(input: &mut R) -> Result<constants::DwForm>` — [`Result`](../../index.md#result), [`DwForm`](../../index.md#dwform)

  Parse an attribute's form.

- <span id="attributespecification-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../../index.md#result), [`AttributeSpecification`](../index.md#attributespecification)

  Parse an attribute specification. Returns `None` for the null attribute

  specification, `Some` for an actual attribute specification.

#### Trait Implementations

##### `impl Any for AttributeSpecification`

- <span id="attributespecification-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeSpecification`

- <span id="attributespecification-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeSpecification`

- <span id="attributespecification-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AttributeSpecification`

- <span id="attributespecification-clone"></span>`fn clone(&self) -> AttributeSpecification` — [`AttributeSpecification`](../index.md#attributespecification)

##### `impl CloneToUninit for AttributeSpecification`

- <span id="attributespecification-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AttributeSpecification`

##### `impl Debug for AttributeSpecification`

- <span id="attributespecification-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AttributeSpecification`

##### `impl<T> From for AttributeSpecification`

- <span id="attributespecification-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](#attributes)

##### `impl<U> Into for AttributeSpecification`

- <span id="attributespecification-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AttributeSpecification`

- <span id="attributespecification-partialeq-eq"></span>`fn eq(&self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](../index.md#attributespecification)

##### `impl StructuralPartialEq for AttributeSpecification`

##### `impl ToOwned for AttributeSpecification`

- <span id="attributespecification-toowned-type-owned"></span>`type Owned = T`

- <span id="attributespecification-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributespecification-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeSpecification`

- <span id="attributespecification-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributespecification-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeSpecification`

- <span id="attributespecification-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributespecification-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `AbbreviationsCacheStrategy`

```rust
enum AbbreviationsCacheStrategy {
    Duplicates,
    All,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:99-108`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L99-L108)*

The strategy to use for caching abbreviations.

#### Variants

- **`Duplicates`**

  Cache abbreviations that are used more than once.
  
  This is useful if the units in the `.debug_info` section will be parsed only once.

- **`All`**

  Cache all abbreviations.
  
  This is useful if the units in the `.debug_info` section will be parsed more than once.

#### Trait Implementations

##### `impl Any for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-clone"></span>`fn clone(&self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](../index.md#abbreviationscachestrategy)

##### `impl CloneToUninit for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AbbreviationsCacheStrategy`

##### `impl Debug for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbbreviationsCacheStrategy`

##### `impl<T> From for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-partialeq-eq"></span>`fn eq(&self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](../index.md#abbreviationscachestrategy)

##### `impl StructuralPartialEq for AbbreviationsCacheStrategy`

##### `impl ToOwned for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-toowned-type-owned"></span>`type Owned = T`

- <span id="abbreviationscachestrategy-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abbreviationscachestrategy-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abbreviationscachestrategy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abbreviationscachestrategy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:391-397`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L391-L397)*

A list of attributes found in an `Abbreviation`

#### Implementations

- <span id="attributes-new"></span>`fn new() -> Attributes` — [`Attributes`](#attributes)

  Returns a new empty list of attributes

- <span id="attributes-push"></span>`fn push(&mut self, attr: AttributeSpecification)` — [`AttributeSpecification`](../index.md#attributespecification)

  Pushes a new value onto this list of attributes.

#### Trait Implementations

##### `impl Any for Attributes`

- <span id="attributes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attributes`

- <span id="attributes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attributes`

- <span id="attributes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Attributes`

- <span id="attributes-clone"></span>`fn clone(&self) -> Attributes` — [`Attributes`](#attributes)

##### `impl CloneToUninit for Attributes`

- <span id="attributes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Attributes`

- <span id="attributes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- <span id="attributes-deref-type-target"></span>`type Target = [AttributeSpecification]`

- <span id="attributes-deref"></span>`fn deref(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md#attributespecification)

##### `impl Eq for Attributes`

##### `impl<T> From for Attributes`

- <span id="attributes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](#attributes)

##### `impl<U> Into for Attributes`

- <span id="attributes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Attributes`

- <span id="attributes-partialeq-eq"></span>`fn eq(&self, other: &Attributes) -> bool` — [`Attributes`](#attributes)

##### `impl Receiver for Attributes`

- <span id="attributes-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for Attributes`

- <span id="attributes-toowned-type-owned"></span>`type Owned = T`

- <span id="attributes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Attributes`

- <span id="attributes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Attributes`

- <span id="attributes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `get_attribute_size`

```rust
fn get_attribute_size(form: constants::DwForm, encoding: crate::common::Encoding) -> Option<u8>
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:572-637`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L572-L637)*

## Constants

### `MAX_ATTRIBUTES_INLINE`
```rust
const MAX_ATTRIBUTES_INLINE: usize = 5usize;
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:400`](../../../../.source_1765521767/gimli-0.32.3/src/read/abbrev.rs#L400)*

