*[gimli](../../index.md) / [read](../index.md) / [unit](index.md)*

---

# Module `unit`

Functions for parsing DWARF `.debug_info` and `.debug_types` sections.

## Contents

- [Structs](#structs)
  - [`DebugInfo`](#debuginfo)
  - [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)
  - [`UnitHeader`](#unitheader)
  - [`DebuggingInformationEntry`](#debugginginformationentry)
  - [`Attribute`](#attribute)
  - [`AttrsIter`](#attrsiter)
  - [`EntriesRaw`](#entriesraw)
  - [`EntriesCursor`](#entriescursor)
  - [`EntriesTree`](#entriestree)
  - [`EntriesTreeNode`](#entriestreenode)
  - [`EntriesTreeIter`](#entriestreeiter)
  - [`DebugTypes`](#debugtypes)
  - [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter)
- [Enums](#enums)
  - [`UnitType`](#unittype)
  - [`AttributeValue`](#attributevalue)
- [Functions](#functions)
  - [`parse_unit_type`](#parse-unit-type)
  - [`parse_debug_abbrev_offset`](#parse-debug-abbrev-offset)
  - [`parse_debug_info_offset`](#parse-debug-info-offset)
  - [`parse_unit_header`](#parse-unit-header)
  - [`parse_dwo_id`](#parse-dwo-id)
  - [`length_u8_value`](#length-u8-value)
  - [`length_u16_value`](#length-u16-value)
  - [`length_u32_value`](#length-u32-value)
  - [`length_uleb128_value`](#length-uleb128-value)
  - [`allow_section_offset`](#allow-section-offset)
  - [`parse_attribute`](#parse-attribute)
  - [`skip_attributes`](#skip-attributes)
  - [`parse_type_signature`](#parse-type-signature)
  - [`parse_type_offset`](#parse-type-offset)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugInfo`](#debuginfo) | struct | The `DebugInfo` struct represents the DWARF debugging information found in the `.debug_info` section. |
| [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter) | struct | An iterator over the units of a .debug_info section. |
| [`UnitHeader`](#unitheader) | struct | The common fields for the headers of compilation units and type units. |
| [`DebuggingInformationEntry`](#debugginginformationentry) | struct | A Debugging Information Entry (DIE). |
| [`Attribute`](#attribute) | struct | An attribute in a `DebuggingInformationEntry`, consisting of a name and associated value. |
| [`AttrsIter`](#attrsiter) | struct | An iterator over a particular entry's attributes. |
| [`EntriesRaw`](#entriesraw) | struct | A raw reader of the data that defines the Debugging Information Entries. |
| [`EntriesCursor`](#entriescursor) | struct | A cursor into the Debugging Information Entries tree for a compilation unit. |
| [`EntriesTree`](#entriestree) | struct | The state information for a tree view of the Debugging Information Entries. |
| [`EntriesTreeNode`](#entriestreenode) | struct | A node in the Debugging Information Entry tree. |
| [`EntriesTreeIter`](#entriestreeiter) | struct | An iterator that allows traversal of the children of an `EntriesTreeNode`. |
| [`DebugTypes`](#debugtypes) | struct | The `DebugTypes` struct represents the DWARF type information found in the `.debug_types` section. |
| [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter) | struct | An iterator over the type-units of this `.debug_types` section. |
| [`UnitType`](#unittype) | enum | This enum specifies the type of the unit and any type specific data carried in the header (e.g. the type signature/type offset of a type unit). |
| [`AttributeValue`](#attributevalue) | enum | The value of an attribute in a `DebuggingInformationEntry`. |
| [`parse_unit_type`](#parse-unit-type) | fn | Parse the unit type from the unit header. |
| [`parse_debug_abbrev_offset`](#parse-debug-abbrev-offset) | fn | Parse the `debug_abbrev_offset` in the compilation unit header. |
| [`parse_debug_info_offset`](#parse-debug-info-offset) | fn | Parse the `debug_info_offset` in the arange header. |
| [`parse_unit_header`](#parse-unit-header) | fn | Parse a unit header. |
| [`parse_dwo_id`](#parse-dwo-id) | fn | Parse a dwo_id from a header |
| [`length_u8_value`](#length-u8-value) | fn |  |
| [`length_u16_value`](#length-u16-value) | fn |  |
| [`length_u32_value`](#length-u32-value) | fn |  |
| [`length_uleb128_value`](#length-uleb128-value) | fn |  |
| [`allow_section_offset`](#allow-section-offset) | fn |  |
| [`parse_attribute`](#parse-attribute) | fn |  |
| [`skip_attributes`](#skip-attributes) | fn |  |
| [`parse_type_signature`](#parse-type-signature) | fn | Parse a type unit header's unique type signature. |
| [`parse_type_offset`](#parse-type-offset) | fn | Parse a type unit header's type offset. |

## Structs

### `DebugInfo<R>`

```rust
struct DebugInfo<R> {
    debug_info_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:82-84`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L82-L84)*

The `DebugInfo` struct represents the DWARF debugging information found in
the `.debug_info` section.

#### Implementations

- <span id="debuginfo-new"></span>`fn new(debug_info_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugInfo` instance from the data in the `.debug_info`

  section.

  

  It is the caller's responsibility to read the `.debug_info` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugInfo, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_info_section_somehow = || &buf;

  let debug_info = DebugInfo::new(read_debug_info_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugInfo<R>`

- <span id="debuginfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugInfo<R>`

- <span id="debuginfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugInfo<R>`

- <span id="debuginfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugInfo<R>`

- <span id="debuginfo-clone"></span>`fn clone(&self) -> DebugInfo<R>` — [`DebugInfo`](../index.md#debuginfo)

##### `impl CloneToUninit for DebugInfo<R>`

- <span id="debuginfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugInfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugInfo<R>`

- <span id="debuginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugInfo<R>`

- <span id="debuginfo-default"></span>`fn default() -> DebugInfo<R>` — [`DebugInfo`](../index.md#debuginfo)

##### `impl<T> From for DebugInfo<R>`

- <span id="debuginfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugInfo<R>`

- <span id="debuginfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugInfo<R>`

- <span id="debuginfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debuginfo-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugInfo<R>`

- <span id="debuginfo-toowned-type-owned"></span>`type Owned = T`

- <span id="debuginfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuginfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugInfo<R>`

- <span id="debuginfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuginfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugInfo<R>`

- <span id="debuginfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuginfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugInfoUnitHeadersIter<R: Reader>`

```rust
struct DebugInfoUnitHeadersIter<R: Reader> {
    input: R,
    offset: crate::common::DebugInfoOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:179-182`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L179-L182)*

An iterator over the units of a .debug_info section.

See the [documentation on
`DebugInfo::units`](./struct.DebugInfo.html#method.units) for more detail.

#### Implementations

- <span id="debuginfounitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../../index.md#result), [`UnitHeader`](../index.md#unitheader)

  Advance the iterator to the next unit header.

#### Trait Implementations

##### `impl Any for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-clone"></span>`fn clone(&self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](../index.md#debuginfounitheadersiter)

##### `impl CloneToUninit for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-toowned-type-owned"></span>`type Owned = T`

- <span id="debuginfounitheadersiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuginfounitheadersiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuginfounitheadersiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuginfounitheadersiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitHeader<R, Offset>`

```rust
struct UnitHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    encoding: crate::common::Encoding,
    unit_length: Offset,
    unit_type: UnitType<Offset>,
    debug_abbrev_offset: crate::common::DebugAbbrevOffset<Offset>,
    unit_offset: crate::common::UnitSectionOffset<Offset>,
    entries_buf: R,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:303-314`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L303-L314)*

The common fields for the headers of compilation units and
type units.

#### Implementations

- <span id="unitheader-new"></span>`fn new(encoding: Encoding, unit_length: Offset, unit_type: UnitType<Offset>, debug_abbrev_offset: DebugAbbrevOffset<Offset>, unit_offset: UnitSectionOffset<Offset>, entries_buf: R) -> Self` — [`Encoding`](../../index.md#encoding), [`UnitType`](../index.md#unittype), [`DebugAbbrevOffset`](../../index.md#debugabbrevoffset), [`UnitSectionOffset`](../../index.md#unitsectionoffset)

  Construct a new `UnitHeader`.

#### Trait Implementations

##### `impl Any for UnitHeader<R, Offset>`

- <span id="unitheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitHeader<R, Offset>`

- <span id="unitheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitHeader<R, Offset>`

- <span id="unitheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for UnitHeader<R, Offset>`

- <span id="unitheader-clone"></span>`fn clone(&self) -> UnitHeader<R, Offset>` — [`UnitHeader`](../index.md#unitheader)

##### `impl CloneToUninit for UnitHeader<R, Offset>`

- <span id="unitheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for UnitHeader<R, Offset>`

##### `impl<R, Offset> Debug for UnitHeader<R, Offset>`

- <span id="unitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for UnitHeader<R, Offset>`

##### `impl<T> From for UnitHeader<R, Offset>`

- <span id="unitheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitHeader<R, Offset>`

- <span id="unitheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for UnitHeader<R, Offset>`

- <span id="unitheader-partialeq-eq"></span>`fn eq(&self, other: &UnitHeader<R, Offset>) -> bool` — [`UnitHeader`](../index.md#unitheader)

##### `impl<R, Offset> StructuralPartialEq for UnitHeader<R, Offset>`

##### `impl ToOwned for UnitHeader<R, Offset>`

- <span id="unitheader-toowned-type-owned"></span>`type Owned = T`

- <span id="unitheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitHeader<R, Offset>`

- <span id="unitheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitHeader<R, Offset>`

- <span id="unitheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

```rust
struct DebuggingInformationEntry<'abbrev, 'unit, R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::read::UnitOffset<Offset>,
    attrs_slice: R,
    attrs_len: core::cell::Cell<Option<Offset>>,
    abbrev: &'abbrev crate::read::Abbreviation,
    unit: &'unit UnitHeader<R, Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:647-657`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L647-L657)*

A Debugging Information Entry (DIE).

DIEs have a set of attributes and optionally have children DIEs as well.

#### Implementations

- <span id="debugginginformationentry-new"></span>`fn new(offset: UnitOffset<Offset>, attrs_slice: R, abbrev: &'abbrev Abbreviation, unit: &'unit UnitHeader<R, Offset>) -> Self` — [`UnitOffset`](../../index.md#unitoffset), [`Abbreviation`](../index.md#abbreviation), [`UnitHeader`](../index.md#unitheader)

  Construct a new `DebuggingInformationEntry`.

- <span id="debugginginformationentry-code"></span>`fn code(&self) -> u64`

  Get this entry's code.

- <span id="debugginginformationentry-offset"></span>`fn offset(&self) -> UnitOffset<Offset>` — [`UnitOffset`](../../index.md#unitoffset)

  Get this entry's offset.

- <span id="debugginginformationentry-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../../index.md#dwtag)

  Get this entry's `DW_TAG_whatever` tag.

  

  ```rust

  use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

  let info_buf = [

      // Comilation unit header

  

      // 32-bit unit length = 12

      0x0c, 0x00, 0x00, 0x00,

      // Version 4

      0x04, 0x00,

      // debug_abbrev_offset

      0x00, 0x00, 0x00, 0x00,

      // Address size

      0x04,

  

      // DIEs

  

      // Abbreviation code

      0x01,

      // Attribute of form DW_FORM_string = "foo\0"

      0x66, 0x6f, 0x6f, 0x00,

  ];

  let debug_info = DebugInfo::new(&info_buf, LittleEndian);

  let abbrev_buf = [

      // Code

      0x01,

      // DW_TAG_subprogram

      0x2e,

      // DW_CHILDREN_no

      0x00,

      // Begin attributes

        // Attribute name = DW_AT_name

        0x03,

        // Attribute form = DW_FORM_string

        0x08,

      // End attributes

      0x00,

      0x00,

      // Null terminator

      0x00

  ];

  let debug_abbrev = DebugAbbrev::new(&abbrev_buf, LittleEndian);

  let unit = debug_info.units().next().unwrap().unwrap();

  let abbrevs = unit.abbreviations(&debug_abbrev).unwrap();

  let mut cursor = unit.entries(&abbrevs);

  let (_, entry) = cursor.next_dfs().unwrap().unwrap();

  let mut get_some_entry = || entry;

  let entry = get_some_entry();

  

  match entry.tag() {

      gimli::DW_TAG_subprogram =>

          println!("this entry contains debug info about a function"),

      gimli::DW_TAG_inlined_subroutine =>

          println!("this entry contains debug info about a particular instance of inlining"),

      gimli::DW_TAG_variable =>

          println!("this entry contains debug info about a local variable"),

      gimli::DW_TAG_formal_parameter =>

          println!("this entry contains debug info about a function parameter"),

      otherwise =>

          println!("this entry is some other kind of data: {:?}", otherwise),

  };

  ```

- <span id="debugginginformationentry-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this entry's type can have children, false otherwise.

- <span id="debugginginformationentry-attrs"></span>`fn attrs<'me>(self: &'me Self) -> AttrsIter<'abbrev, 'me, 'unit, R>` — [`AttrsIter`](../index.md#attrsiter)

  Iterate over this entry's set of attributes.

  

  ```rust

  use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

  

  // Read the `.debug_info` section.

  

  let info_buf = [

      // Comilation unit header

  

      // 32-bit unit length = 12

      0x0c, 0x00, 0x00, 0x00,

      // Version 4

      0x04, 0x00,

      // debug_abbrev_offset

      0x00, 0x00, 0x00, 0x00,

      // Address size

      0x04,

  

      // DIEs

  

      // Abbreviation code

      0x01,

      // Attribute of form DW_FORM_string = "foo\0"

      0x66, 0x6f, 0x6f, 0x00,

  ];

  let read_debug_info_section_somehow = || &info_buf;

  let debug_info = DebugInfo::new(read_debug_info_section_somehow(), LittleEndian);

  

  // Get the data about the first compilation unit out of the `.debug_info`.

  

  let unit = debug_info.units().next()

      .expect("Should have at least one compilation unit")

      .expect("and it should parse ok");

  

  // Read the `.debug_abbrev` section and parse the

  // abbreviations for our compilation unit.

  

  let abbrev_buf = [

      // Code

      0x01,

      // DW_TAG_subprogram

      0x2e,

      // DW_CHILDREN_no

      0x00,

      // Begin attributes

        // Attribute name = DW_AT_name

        0x03,

        // Attribute form = DW_FORM_string

        0x08,

      // End attributes

      0x00,

      0x00,

      // Null terminator

      0x00

  ];

  let read_debug_abbrev_section_somehow = || &abbrev_buf;

  let debug_abbrev = DebugAbbrev::new(read_debug_abbrev_section_somehow(), LittleEndian);

  let abbrevs = unit.abbreviations(&debug_abbrev).unwrap();

  

  // Get the first entry from that compilation unit.

  

  let mut cursor = unit.entries(&abbrevs);

  let (_, entry) = cursor.next_dfs()

      .expect("Should parse next entry")

      .expect("Should have at least one entry");

  

  // Finally, print the first entry's attributes.

  

  let mut attrs = entry.attrs();

  while let Some(attr) = attrs.next().unwrap() {

      println!("Attribute name = {:?}", attr.name());

      println!("Attribute value = {:?}", attr.value());

  }

  ```

  

  Can be [used with

  `FallibleIterator`](./index.html#using-with-fallibleiterator).

- <span id="debugginginformationentry-attr"></span>`fn attr(&self, name: constants::DwAt) -> Result<Option<Attribute<R>>>` — [`DwAt`](../../index.md#dwat), [`Result`](../../index.md#result), [`Attribute`](../index.md#attribute)

  Find the first attribute in this entry which has the given name,

  and return it. Returns `Ok(None)` if no attribute is found.

- <span id="debugginginformationentry-attr-value-raw"></span>`fn attr_value_raw(&self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../../index.md#dwat), [`Result`](../../index.md#result), [`AttributeValue`](../index.md#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its raw value. Returns `Ok(None)` if no attribute is found.

- <span id="debugginginformationentry-attr-value"></span>`fn attr_value(&self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../../index.md#dwat), [`Result`](../../index.md#result), [`AttributeValue`](../index.md#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its normalized value.  Returns `Ok(None)` if no

  attribute is found.

- <span id="debugginginformationentry-after-attrs"></span>`fn after_attrs(&self) -> Result<R>` — [`Result`](../../index.md#result)

  Return the input buffer after the last attribute.

- <span id="debugginginformationentry-sibling"></span>`fn sibling(&self) -> Option<R>`

  Use the `DW_AT_sibling` attribute to find the input buffer for the

  next sibling. Returns `None` if the attribute is missing or invalid.

- <span id="debugginginformationentry-parse"></span>`fn parse(input: &mut R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Result<Option<Self>>` — [`UnitHeader`](../index.md#unitheader), [`Abbreviations`](../index.md#abbreviations), [`Result`](../../index.md#result)

  Parse an entry. Returns `Ok(None)` for null entries.

#### Trait Implementations

##### `impl Any for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-clone"></span>`fn clone(&self) -> DebuggingInformationEntry<'abbrev, 'unit, R, Offset>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

##### `impl CloneToUninit for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-toowned-type-owned"></span>`type Owned = T`

- <span id="debugginginformationentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugginginformationentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugginginformationentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugginginformationentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Attribute<R: Reader>`

```rust
struct Attribute<R: Reader> {
    name: constants::DwAt,
    value: AttributeValue<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1111-1114`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1111-L1114)*

An attribute in a `DebuggingInformationEntry`, consisting of a name and
associated value.

#### Implementations

- <span id="attribute-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../../index.md#dwat)

  Get this attribute's name.

- <span id="attribute-raw-value"></span>`fn raw_value(&self) -> AttributeValue<R>` — [`AttributeValue`](../index.md#attributevalue)

  Get this attribute's raw value.

- <span id="attribute-value"></span>`fn value(&self) -> AttributeValue<R>` — [`AttributeValue`](../index.md#attributevalue)

  Get this attribute's normalized value.

  

  Attribute values can potentially be encoded in multiple equivalent forms,

  and may have special meaning depending on the attribute name.  This method

  converts the attribute value to a normalized form based on the attribute

  name.

  

  See "Table 7.5: Attribute encodings" and "Table 7.6: Attribute form encodings".

- <span id="attribute-u8-value"></span>`fn u8_value(&self) -> Option<u8>`

  Try to convert this attribute's value to a u8.

- <span id="attribute-u16-value"></span>`fn u16_value(&self) -> Option<u16>`

  Try to convert this attribute's value to a u16.

- <span id="attribute-udata-value"></span>`fn udata_value(&self) -> Option<u64>`

  Try to convert this attribute's value to an unsigned integer.

- <span id="attribute-sdata-value"></span>`fn sdata_value(&self) -> Option<i64>`

  Try to convert this attribute's value to a signed integer.

- <span id="attribute-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](../index.md#reader)

  Try to convert this attribute's value to an offset.

- <span id="attribute-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](../index.md#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attribute-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attribute-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, or a `DW_FORM_strp_sup` reference to an offset into a supplementary

  object file, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

#### Trait Implementations

##### `impl Any for Attribute<R>`

- <span id="attribute-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attribute<R>`

- <span id="attribute-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attribute<R>`

- <span id="attribute-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for Attribute<R>`

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute<R>` — [`Attribute`](../index.md#attribute)

##### `impl CloneToUninit for Attribute<R>`

- <span id="attribute-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for Attribute<R>`

##### `impl<R: fmt::Debug + Reader> Debug for Attribute<R>`

- <span id="attribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for Attribute<R>`

##### `impl<T> From for Attribute<R>`

- <span id="attribute-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Attribute<R>`

- <span id="attribute-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for Attribute<R>`

- <span id="attribute-partialeq-eq"></span>`fn eq(&self, other: &Attribute<R>) -> bool` — [`Attribute`](../index.md#attribute)

##### `impl<R: Reader> StructuralPartialEq for Attribute<R>`

##### `impl ToOwned for Attribute<R>`

- <span id="attribute-toowned-type-owned"></span>`type Owned = T`

- <span id="attribute-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attribute-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Attribute<R>`

- <span id="attribute-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attribute-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Attribute<R>`

- <span id="attribute-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attribute-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttrsIter<'abbrev, 'entry, 'unit, R: Reader>`

```rust
struct AttrsIter<'abbrev, 'entry, 'unit, R: Reader> {
    input: R,
    attributes: &'abbrev [crate::read::AttributeSpecification],
    entry: &'entry DebuggingInformationEntry<'abbrev, 'unit, R>,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2272-2276`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L2272-L2276)*

An iterator over a particular entry's attributes.

See [the documentation for
`DebuggingInformationEntry::attrs()`](./struct.DebuggingInformationEntry.html#method.attrs)
for details.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="attrsiter-next"></span>`fn next(&mut self) -> Result<Option<Attribute<R>>>` — [`Result`](../../index.md#result), [`Attribute`](../index.md#attribute)

  Advance the iterator and return the next attribute.

  

  Returns `None` when iteration is finished. If an error

  occurs while parsing the next attribute, then this error

  is returned, and all subsequent calls return `None`.

#### Trait Implementations

##### `impl Any for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-clone"></span>`fn clone(&self) -> AttrsIter<'abbrev, 'entry, 'unit, R>` — [`AttrsIter`](../index.md#attrsiter)

##### `impl CloneToUninit for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for AttrsIter<'abbrev, 'entry, 'unit, R>`

##### `impl<R: fmt::Debug + Reader> Debug for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-toowned-type-owned"></span>`type Owned = T`

- <span id="attrsiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attrsiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attrsiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttrsIter<'abbrev, 'entry, 'unit, R>`

- <span id="attrsiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attrsiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntriesRaw<'abbrev, 'unit, R>`

```rust
struct EntriesRaw<'abbrev, 'unit, R>
where
    R: Reader {
    input: R,
    unit: &'unit UnitHeader<R>,
    abbreviations: &'abbrev crate::read::Abbreviations,
    depth: isize,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2382-2390`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L2382-L2390)*

A raw reader of the data that defines the Debugging Information Entries.

`EntriesRaw` provides primitives to read the components of Debugging Information
Entries (DIEs). A DIE consists of an abbreviation code (read with `read_abbreviation`)
followed by a number of attributes (read with `read_attribute`).
The user must provide the control flow to read these correctly.
In particular, all attributes must always be read before reading another
abbreviation code.

`EntriesRaw` lacks some features of `EntriesCursor`, such as the ability to skip
to the next sibling DIE. However, this also allows it to optimize better, since it
does not need to perform the extra bookkeeping required to support these features,
and thus it is suitable for cases where performance is important.

## Example Usage
```rust,no_run
fn example() -> Result<(), gimli::Error> {
let debug_info = gimli::DebugInfo::new(&[], gimli::LittleEndian);
let get_some_unit = || debug_info.units().next().unwrap().unwrap();
let unit = get_some_unit();
let debug_abbrev = gimli::DebugAbbrev::new(&[], gimli::LittleEndian);
let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();
let abbrevs = get_abbrevs_for_unit(&unit);

let mut entries = unit.entries_raw(&abbrevs, None)?;
while !entries.is_empty() {
    let abbrev = if let Some(abbrev) = entries.read_abbreviation()? {
        abbrev
    } else {
        // Null entry with no attributes.
        continue
    };
    match abbrev.tag() {
        gimli::DW_TAG_subprogram => {
            // Loop over attributes for DIEs we care about.
            for spec in abbrev.attributes() {
                let attr = entries.read_attribute(*spec)?;
                match attr.name() {
                    // Handle attributes.
                    _ => {}
                }
            }
        }
        _ => {
            // Skip attributes for DIEs we don't care about.
            entries.skip_attributes(abbrev.attributes());
        }
    }
}
unreachable!()
}
```

#### Implementations

- <span id="entriesraw-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if there is no more input.

- <span id="entriesraw-next-offset"></span>`fn next_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Return the unit offset at which the reader will read next.

  

  If you want the offset of the next entry, then this must be called prior to reading

  the next entry.

- <span id="entriesraw-next-depth"></span>`fn next_depth(&self) -> isize`

  Return the depth of the next entry.

  

  This depth is updated when `read_abbreviation` is called, and is updated

  based on null entries and the `has_children` field in the abbreviation.

- <span id="entriesraw-read-abbreviation"></span>`fn read_abbreviation(&mut self) -> Result<Option<&'abbrev Abbreviation>>` — [`Result`](../../index.md#result), [`Abbreviation`](../index.md#abbreviation)

  Read an abbreviation code and lookup the corresponding `Abbreviation`.

  

  Returns `Ok(None)` for null entries.

- <span id="entriesraw-read-attribute"></span>`fn read_attribute(&mut self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](../index.md#attributespecification), [`Result`](../../index.md#result), [`Attribute`](../index.md#attribute)

  Read an attribute.

- <span id="entriesraw-skip-attributes"></span>`fn skip_attributes(&mut self, specs: &[AttributeSpecification]) -> Result<()>` — [`AttributeSpecification`](../index.md#attributespecification), [`Result`](../../index.md#result)

  Skip all the attributes of an abbreviation.

#### Trait Implementations

##### `impl Any for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-clone"></span>`fn clone(&self) -> EntriesRaw<'abbrev, 'unit, R>` — [`EntriesRaw`](../index.md#entriesraw)

##### `impl CloneToUninit for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R> Debug for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-toowned-type-owned"></span>`type Owned = T`

- <span id="entriesraw-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="entriesraw-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entriesraw-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entriesraw-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntriesCursor<'abbrev, 'unit, R>`

```rust
struct EntriesCursor<'abbrev, 'unit, R>
where
    R: Reader {
    input: R,
    unit: &'unit UnitHeader<R>,
    abbreviations: &'abbrev crate::read::Abbreviations,
    cached_current: Option<DebuggingInformationEntry<'abbrev, 'unit, R>>,
    delta_depth: isize,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2463-2472`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L2463-L2472)*

A cursor into the Debugging Information Entries tree for a compilation unit.

The `EntriesCursor` can traverse the DIE tree in DFS order using `next_dfs()`,
or skip to the next sibling of the entry the cursor is currently pointing to
using `next_sibling()`.

It is also possible to traverse the DIE tree at a lower abstraction level
using `next_entry()`. This method does not skip over null entries, or provide
any indication of the current tree depth. In this case, you must use `current()`
to obtain the current entry, and `current().has_children()` to determine if
the entry following the current entry will be a sibling or child. `current()`
will return `None` if the current entry is a null entry, which signifies the
end of the current tree depth.

#### Implementations

- <span id="entriescursor-current"></span>`fn current(&self) -> Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Get a reference to the entry that the cursor is currently pointing to.

  

  If the cursor is not pointing at an entry, or if the current entry is a

  null entry, then `None` is returned.

- <span id="entriescursor-next-entry"></span>`fn next_entry(&mut self) -> Result<Option<()>>` — [`Result`](../../index.md#result)

  Move the cursor to the next DIE in the tree.

  

  Returns `Some` if there is a next entry, even if this entry is null.

  If there is no next entry, then `None` is returned.

- <span id="entriescursor-next-dfs"></span>`fn next_dfs(&mut self) -> Result<Option<(isize, &DebuggingInformationEntry<'abbrev, 'unit, R>)>>` — [`Result`](../../index.md#result), [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Move the cursor to the next DIE in the tree in DFS order.

  

  Upon successful movement of the cursor, return the delta traversal

  depth and the entry:

  

    * If we moved down into the previous current entry's children, we get

      `Some((1, entry))`.

  

    * If we moved to the previous current entry's sibling, we get

      `Some((0, entry))`.

  

    * If the previous entry does not have any siblings and we move up to

      its parent's next sibling, then we get `Some((-1, entry))`. Note that

      if the parent doesn't have a next sibling, then it could go up to the

      parent's parent's next sibling and return `Some((-2, entry))`, etc.

  

  If there is no next entry, then `None` is returned.

  

  Here is an example that finds the first entry in a compilation unit that

  does not have any children.

  

  ```rust

  use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

  let info_buf = [

      // Comilation unit header

  

      // 32-bit unit length = 25

      0x19, 0x00, 0x00, 0x00,

      // Version 4

      0x04, 0x00,

      // debug_abbrev_offset

      0x00, 0x00, 0x00, 0x00,

      // Address size

      0x04,

  

      // DIEs

  

      // Abbreviation code

      0x01,

      // Attribute of form DW_FORM_string = "foo\0"

      0x66, 0x6f, 0x6f, 0x00,

  

        // Children

  

        // Abbreviation code

        0x01,

        // Attribute of form DW_FORM_string = "foo\0"

        0x66, 0x6f, 0x6f, 0x00,

  

          // Children

  

          // Abbreviation code

          0x01,

          // Attribute of form DW_FORM_string = "foo\0"

          0x66, 0x6f, 0x6f, 0x00,

  

            // Children

  

            // End of children

            0x00,

  

          // End of children

          0x00,

  

        // End of children

        0x00,

  ];

  let debug_info = DebugInfo::new(&info_buf, LittleEndian);

  

  let abbrev_buf = [

      // Code

      0x01,

      // DW_TAG_subprogram

      0x2e,

      // DW_CHILDREN_yes

      0x01,

      // Begin attributes

        // Attribute name = DW_AT_name

        0x03,

        // Attribute form = DW_FORM_string

        0x08,

      // End attributes

      0x00,

      0x00,

      // Null terminator

      0x00

  ];

  let debug_abbrev = DebugAbbrev::new(&abbrev_buf, LittleEndian);

  

  let get_some_unit = || debug_info.units().next().unwrap().unwrap();

  

  let unit = get_some_unit();

  let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();

  let abbrevs = get_abbrevs_for_unit(&unit);

  

  let mut first_entry_with_no_children = None;

  let mut cursor = unit.entries(&abbrevs);

  

  // Move the cursor to the root.

  assert!(cursor.next_dfs().unwrap().is_some());

  

  // Traverse the DIE tree in depth-first search order.

  let mut depth = 0;

  while let Some((delta_depth, current)) = cursor.next_dfs().expect("Should parse next dfs") {

      // Update depth value, and break out of the loop when we

      // return to the original starting position.

      depth += delta_depth;

      if depth <= 0 {

          break;

      }

  

      first_entry_with_no_children = Some(current.clone());

  }

  

  println!("The first entry with no children is {:?}",

           first_entry_with_no_children.unwrap());

  ```

- <span id="entriescursor-next-sibling"></span>`fn next_sibling(&mut self) -> Result<Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>>` — [`Result`](../../index.md#result), [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Move the cursor to the next sibling DIE of the current one.

  

  Returns `Ok(Some(entry))` when the cursor has been moved to

  the next sibling, `Ok(None)` when there is no next sibling.

  

  The depth of the cursor is never changed if this method returns `Ok`.

  Once `Ok(None)` is returned, this method will continue to return

  `Ok(None)` until either `next_entry` or `next_dfs` is called.

  

  Here is an example that iterates over all of the direct children of the

  root entry:

  

  ```rust

  use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

  let info_buf = [

      // Comilation unit header

  

      // 32-bit unit length = 25

      0x19, 0x00, 0x00, 0x00,

      // Version 4

      0x04, 0x00,

      // debug_abbrev_offset

      0x00, 0x00, 0x00, 0x00,

      // Address size

      0x04,

  

      // DIEs

  

      // Abbreviation code

      0x01,

      // Attribute of form DW_FORM_string = "foo\0"

      0x66, 0x6f, 0x6f, 0x00,

  

        // Children

  

        // Abbreviation code

        0x01,

        // Attribute of form DW_FORM_string = "foo\0"

        0x66, 0x6f, 0x6f, 0x00,

  

          // Children

  

          // Abbreviation code

          0x01,

          // Attribute of form DW_FORM_string = "foo\0"

          0x66, 0x6f, 0x6f, 0x00,

  

            // Children

  

            // End of children

            0x00,

  

          // End of children

          0x00,

  

        // End of children

        0x00,

  ];

  let debug_info = DebugInfo::new(&info_buf, LittleEndian);

  

  let get_some_unit = || debug_info.units().next().unwrap().unwrap();

  

  let abbrev_buf = [

      // Code

      0x01,

      // DW_TAG_subprogram

      0x2e,

      // DW_CHILDREN_yes

      0x01,

      // Begin attributes

        // Attribute name = DW_AT_name

        0x03,

        // Attribute form = DW_FORM_string

        0x08,

      // End attributes

      0x00,

      0x00,

      // Null terminator

      0x00

  ];

  let debug_abbrev = DebugAbbrev::new(&abbrev_buf, LittleEndian);

  

  let unit = get_some_unit();

  let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();

  let abbrevs = get_abbrevs_for_unit(&unit);

  

  let mut cursor = unit.entries(&abbrevs);

  

  // Move the cursor to the root.

  assert!(cursor.next_dfs().unwrap().is_some());

  

  // Move the cursor to the root's first child.

  assert!(cursor.next_dfs().unwrap().is_some());

  

  // Iterate the root's children.

  loop {

      {

          let current = cursor.current().expect("Should be at an entry");

          println!("{:?} is a child of the root", current);

      }

  

      if cursor.next_sibling().expect("Should parse next sibling").is_none() {

          break;

      }

  }

  ```

#### Trait Implementations

##### `impl Any for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-clone"></span>`fn clone(&self) -> EntriesCursor<'abbrev, 'unit, R>` — [`EntriesCursor`](../index.md#entriescursor)

##### `impl CloneToUninit for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R> Debug for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-toowned-type-owned"></span>`type Owned = T`

- <span id="entriescursor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="entriescursor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entriescursor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntriesCursor<'abbrev, 'unit, R>`

- <span id="entriescursor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entriescursor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntriesTree<'abbrev, 'unit, R>`

```rust
struct EntriesTree<'abbrev, 'unit, R>
where
    R: Reader {
    root: R,
    unit: &'unit UnitHeader<R>,
    abbreviations: &'abbrev crate::read::Abbreviations,
    input: R,
    entry: Option<DebuggingInformationEntry<'abbrev, 'unit, R>>,
    depth: isize,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2847-2857`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L2847-L2857)*

The state information for a tree view of the Debugging Information Entries.

The `EntriesTree` can be used to recursively iterate through the DIE
tree, following the parent/child relationships. The `EntriesTree` contains
shared state for all nodes in the tree, avoiding any duplicate parsing of
entries during the traversal.

## Example Usage
```rust,no_run
fn example() -> Result<(), gimli::Error> {
let debug_info = gimli::DebugInfo::new(&[], gimli::LittleEndian);
let get_some_unit = || debug_info.units().next().unwrap().unwrap();
let unit = get_some_unit();
let debug_abbrev = gimli::DebugAbbrev::new(&[], gimli::LittleEndian);
let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();
let abbrevs = get_abbrevs_for_unit(&unit);

let mut tree = unit.entries_tree(&abbrevs, None)?;
let root = tree.root()?;
process_tree(root)?;
unreachable!()
}

fn process_tree<R>(mut node: gimli::EntriesTreeNode<R>) -> gimli::Result<()>
    where R: gimli::Reader
{
    {
        // Examine the entry attributes.
        let mut attrs = node.entry().attrs();
        while let Some(attr) = attrs.next()? {
        }
    }
    let mut children = node.children();
    while let Some(child) = children.next()? {
        // Recursively process a child.
        process_tree(child);
    }
    Ok(())
}
```

#### Implementations

- <span id="entriestree-new"></span>`fn new(root: R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Self` — [`UnitHeader`](../index.md#unitheader), [`Abbreviations`](../index.md#abbreviations)

- <span id="entriestree-root"></span>`fn root<'me>(self: &'me mut Self) -> Result<EntriesTreeNode<'abbrev, 'unit, 'me, R>>` — [`Result`](../../index.md#result), [`EntriesTreeNode`](../index.md#entriestreenode)

  Returns the root node of the tree.

- <span id="entriestree-next"></span>`fn next(&mut self, depth: isize) -> Result<bool>` — [`Result`](../../index.md#result)

  Move the cursor to the next entry at the specified depth.

  

  Requires `depth <= self.depth + 1`.

  

  Returns `true` if successful.

#### Trait Implementations

##### `impl Any for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-clone"></span>`fn clone(&self) -> EntriesTree<'abbrev, 'unit, R>` — [`EntriesTree`](../index.md#entriestree)

##### `impl CloneToUninit for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R> Debug for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-toowned-type-owned"></span>`type Owned = T`

- <span id="entriestree-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="entriestree-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entriestree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntriesTree<'abbrev, 'unit, R>`

- <span id="entriestree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entriestree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntriesTreeNode<'abbrev, 'unit, 'tree, R: Reader>`

```rust
struct EntriesTreeNode<'abbrev, 'unit, 'tree, R: Reader> {
    tree: &'tree mut EntriesTree<'abbrev, 'unit, R>,
    depth: isize,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2979-2982`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L2979-L2982)*

A node in the Debugging Information Entry tree.

The root node of a tree can be obtained
via [`EntriesTree::root`](./struct.EntriesTree.html#method.root).

#### Implementations

- <span id="entriestreenode-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeNode<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](../index.md#entriestree), [`EntriesTreeNode`](../index.md#entriestreenode)

- <span id="entriestreenode-entry"></span>`fn entry(&self) -> &DebuggingInformationEntry<'abbrev, 'unit, R>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Returns the current entry in the tree.

- <span id="entriestreenode-children"></span>`fn children(self) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTreeIter`](../index.md#entriestreeiter)

  Create an iterator for the children of the current entry.

  

  The current entry can no longer be accessed after creating the

  iterator.

#### Trait Implementations

##### `impl Any for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entriestreenode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreenode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entriestreenode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntriesTreeIter<'abbrev, 'unit, 'tree, R: Reader>`

```rust
struct EntriesTreeIter<'abbrev, 'unit, 'tree, R: Reader> {
    tree: &'tree mut EntriesTree<'abbrev, 'unit, R>,
    depth: isize,
    empty: bool,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3014-3018`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L3014-L3018)*

An iterator that allows traversal of the children of an
`EntriesTreeNode`.

The items returned by this iterator are also `EntriesTreeNode`s,
which allow recursive traversal of grandchildren, etc.

#### Implementations

- <span id="entriestreeiter-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](../index.md#entriestree), [`EntriesTreeIter`](../index.md#entriestreeiter)

- <span id="entriestreeiter-next"></span>`fn next<'me>(self: &'me mut Self) -> Result<Option<EntriesTreeNode<'abbrev, 'unit, 'me, R>>>` — [`Result`](../../index.md#result), [`EntriesTreeNode`](../index.md#entriestreenode)

  Returns an `EntriesTreeNode` for the next child entry.

  

  Returns `None` if there are no more children.

#### Trait Implementations

##### `impl Any for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entriestreeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- <span id="entriestreeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entriestreeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugTypes<R>`

```rust
struct DebugTypes<R> {
    debug_types_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3061-3063`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L3061-L3063)*

The `DebugTypes` struct represents the DWARF type information
found in the `.debug_types` section.

#### Implementations

- <span id="debugtypes-new"></span>`fn new(debug_types_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugTypes` instance from the data in the `.debug_types`

  section.

  

  It is the caller's responsibility to read the `.debug_types` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugTypes, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_types_section_somehow = || &buf;

  let debug_types = DebugTypes::new(read_debug_types_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugTypes<R>`

- <span id="debugtypes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTypes<R>`

- <span id="debugtypes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTypes<R>`

- <span id="debugtypes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugTypes<R>`

- <span id="debugtypes-clone"></span>`fn clone(&self) -> DebugTypes<R>` — [`DebugTypes`](../index.md#debugtypes)

##### `impl CloneToUninit for DebugTypes<R>`

- <span id="debugtypes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugTypes<R>`

##### `impl<R: fmt::Debug> Debug for DebugTypes<R>`

- <span id="debugtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTypes<R>`

- <span id="debugtypes-default"></span>`fn default() -> DebugTypes<R>` — [`DebugTypes`](../index.md#debugtypes)

##### `impl<T> From for DebugTypes<R>`

- <span id="debugtypes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugTypes<R>`

- <span id="debugtypes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugTypes<R>`

- <span id="debugtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugtypes-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugTypes<R>`

- <span id="debugtypes-toowned-type-owned"></span>`type Owned = T`

- <span id="debugtypes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugtypes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugTypes<R>`

- <span id="debugtypes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugtypes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugTypes<R>`

- <span id="debugtypes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugtypes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugTypesUnitHeadersIter<R: Reader>`

```rust
struct DebugTypesUnitHeadersIter<R: Reader> {
    input: R,
    offset: crate::common::DebugTypesOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3152-3155`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L3152-L3155)*

An iterator over the type-units of this `.debug_types` section.

See the [documentation on
`DebugTypes::units`](./struct.DebugTypes.html#method.units) for
more detail.

#### Implementations

- <span id="debugtypesunitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../../index.md#result), [`UnitHeader`](../index.md#unitheader)

  Advance the iterator to the next type unit header.

#### Trait Implementations

##### `impl Any for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-clone"></span>`fn clone(&self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](../index.md#debugtypesunitheadersiter)

##### `impl CloneToUninit for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-toowned-type-owned"></span>`type Owned = T`

- <span id="debugtypesunitheadersiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugtypesunitheadersiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugtypesunitheadersiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugtypesunitheadersiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `UnitType<Offset>`

```rust
enum UnitType<Offset>
where
    Offset: ReaderOffset {
    Compilation,
    Type {
        type_signature: crate::common::DebugTypeSignature,
        type_offset: crate::read::UnitOffset<Offset>,
    },
    Partial,
    Skeleton(crate::common::DwoId),
    SplitCompilation(crate::common::DwoId),
    SplitType {
        type_signature: crate::common::DebugTypeSignature,
        type_offset: crate::read::UnitOffset<Offset>,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:241-279`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L241-L279)*

This enum specifies the type of the unit and any type
specific data carried in the header (e.g. the type
signature/type offset of a type unit).

#### Variants

- **`Compilation`**

  In DWARF5, a unit with type `DW_UT_compile`. In previous DWARF versions,
  any unit appearing in the .debug_info section.

- **`Type`**

  In DWARF5, a unit with type `DW_UT_type`. In DWARF4, any unit appearing
  in the .debug_types section.

- **`Partial`**

  A unit with type `DW_UT_partial`. The root DIE of this unit should be a
  `DW_TAG_partial_unit`.

- **`Skeleton`**

  A unit with type `DW_UT_skeleton`. The enclosed dwo_id can be used to
  link this with the corresponding `SplitCompilation` unit in a dwo file.
  NB: The non-standard GNU split DWARF extension to DWARF 4 will instead
  be a `Compilation` unit with the dwo_id present as an attribute on the
  root DIE.

- **`SplitCompilation`**

  A unit with type `DW_UT_split_compile`. The enclosed dwo_id can be used to
  link this with the corresponding `Skeleton` unit in the original binary.
  NB: The non-standard GNU split DWARF extension to DWARF 4 will instead
  be a `Compilation` unit with the dwo_id present as an attribute on the
  root DIE.

- **`SplitType`**

  A unit with type `DW_UT_split_type`. A split type unit is identical to a
  conventional type unit except for the section in which it appears.

#### Implementations

- <span id="unittype-dw-ut"></span>`fn dw_ut(&self) -> constants::DwUt` — [`DwUt`](../../index.md#dwut)

#### Trait Implementations

##### `impl Any for UnitType<Offset>`

- <span id="unittype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitType<Offset>`

- <span id="unittype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitType<Offset>`

- <span id="unittype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Offset> Clone for UnitType<Offset>`

- <span id="unittype-clone"></span>`fn clone(&self) -> UnitType<Offset>` — [`UnitType`](../index.md#unittype)

##### `impl CloneToUninit for UnitType<Offset>`

- <span id="unittype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Offset> Copy for UnitType<Offset>`

##### `impl<Offset> Debug for UnitType<Offset>`

- <span id="unittype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Offset> Eq for UnitType<Offset>`

##### `impl<T> From for UnitType<Offset>`

- <span id="unittype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitType<Offset>`

- <span id="unittype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Offset> PartialEq for UnitType<Offset>`

- <span id="unittype-partialeq-eq"></span>`fn eq(&self, other: &UnitType<Offset>) -> bool` — [`UnitType`](../index.md#unittype)

##### `impl<Offset> StructuralPartialEq for UnitType<Offset>`

##### `impl ToOwned for UnitType<Offset>`

- <span id="unittype-toowned-type-owned"></span>`type Owned = T`

- <span id="unittype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unittype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitType<Offset>`

- <span id="unittype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unittype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitType<Offset>`

- <span id="unittype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unittype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AttributeValue<R, Offset>`

```rust
enum AttributeValue<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Addr(u64),
    Block(R),
    Data1(u8),
    Data2(u16),
    Data4(u32),
    Data8(u64),
    Sdata(i64),
    Udata(u64),
    Exprloc(crate::read::Expression<R>),
    Flag(bool),
    SecOffset(Offset),
    DebugAddrBase(crate::common::DebugAddrBase<Offset>),
    DebugAddrIndex(crate::common::DebugAddrIndex<Offset>),
    UnitRef(crate::read::UnitOffset<Offset>),
    DebugInfoRef(crate::common::DebugInfoOffset<Offset>),
    DebugInfoRefSup(crate::common::DebugInfoOffset<Offset>),
    DebugLineRef(crate::common::DebugLineOffset<Offset>),
    LocationListsRef(crate::common::LocationListsOffset<Offset>),
    DebugLocListsBase(crate::common::DebugLocListsBase<Offset>),
    DebugLocListsIndex(crate::common::DebugLocListsIndex<Offset>),
    DebugMacinfoRef(crate::common::DebugMacinfoOffset<Offset>),
    DebugMacroRef(crate::common::DebugMacroOffset<Offset>),
    RangeListsRef(crate::common::RawRangeListsOffset<Offset>),
    DebugRngListsBase(crate::common::DebugRngListsBase<Offset>),
    DebugRngListsIndex(crate::common::DebugRngListsIndex<Offset>),
    DebugTypesRef(crate::common::DebugTypeSignature),
    DebugStrRef(crate::common::DebugStrOffset<Offset>),
    DebugStrRefSup(crate::common::DebugStrOffset<Offset>),
    DebugStrOffsetsBase(crate::common::DebugStrOffsetsBase<Offset>),
    DebugStrOffsetsIndex(crate::common::DebugStrOffsetsIndex<Offset>),
    DebugLineStrRef(crate::common::DebugLineStrOffset<Offset>),
    String(R),
    Encoding(constants::DwAte),
    DecimalSign(constants::DwDs),
    Endianity(constants::DwEnd),
    Accessibility(constants::DwAccess),
    Visibility(constants::DwVis),
    Virtuality(constants::DwVirtuality),
    Language(constants::DwLang),
    AddressClass(constants::DwAddr),
    IdentifierCase(constants::DwId),
    CallingConvention(constants::DwCc),
    Inline(constants::DwInl),
    Ordering(constants::DwOrd),
    FileIndex(u64),
    DwoId(crate::common::DwoId),
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:933-1106`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L933-L1106)*

The value of an attribute in a `DebuggingInformationEntry`.

#### Variants

- **`Addr`**

  "Refers to some location in the address space of the described program."

- **`Block`**

  A slice of an arbitrary number of bytes.

- **`Data1`**

  A one byte constant data value. How to interpret the byte depends on context.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data2`**

  A two byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data4`**

  A four byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data8`**

  An eight byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Sdata`**

  A signed integer constant.

- **`Udata`**

  An unsigned integer constant.

- **`Exprloc`**

  "The information bytes contain a DWARF expression (see Section 2.5) or
  location description (see Section 2.6)."

- **`Flag`**

  A boolean that indicates presence or absence of the attribute.

- **`SecOffset`**

  An offset into another section. Which section this is an offset into
  depends on context.

- **`DebugAddrBase`**

  An offset to a set of addresses in the `.debug_addr` section.

- **`DebugAddrIndex`**

  An index into a set of addresses in the `.debug_addr` section.

- **`UnitRef`**

  An offset into the current compilation unit.

- **`DebugInfoRef`**

  An offset into the current `.debug_info` section, but possibly a
  different compilation unit from the current one.

- **`DebugInfoRefSup`**

  An offset into the `.debug_info` section of the supplementary object file.

- **`DebugLineRef`**

  An offset into the `.debug_line` section.

- **`LocationListsRef`**

  An offset into either the `.debug_loc` section or the `.debug_loclists` section.

- **`DebugLocListsBase`**

  An offset to a set of offsets in the `.debug_loclists` section.

- **`DebugLocListsIndex`**

  An index into a set of offsets in the `.debug_loclists` section.

- **`DebugMacinfoRef`**

  An offset into the `.debug_macinfo` section.

- **`DebugMacroRef`**

  An offset into the `.debug_macro` section.

- **`RangeListsRef`**

  An offset into the `.debug_ranges` section.

- **`DebugRngListsBase`**

  An offset to a set of offsets in the `.debug_rnglists` section.

- **`DebugRngListsIndex`**

  An index into a set of offsets in the `.debug_rnglists` section.

- **`DebugTypesRef`**

  A type signature.

- **`DebugStrRef`**

  An offset into the `.debug_str` section.

- **`DebugStrRefSup`**

  An offset into the `.debug_str` section of the supplementary object file.

- **`DebugStrOffsetsBase`**

  An offset to a set of entries in the `.debug_str_offsets` section.

- **`DebugStrOffsetsIndex`**

  An index into a set of entries in the `.debug_str_offsets` section.

- **`DebugLineStrRef`**

  An offset into the `.debug_line_str` section.

- **`String`**

  A slice of bytes representing a string. Does not include a final null byte.
  Not guaranteed to be UTF-8 or anything like that.

- **`Encoding`**

  The value of a `DW_AT_encoding` attribute.

- **`DecimalSign`**

  The value of a `DW_AT_decimal_sign` attribute.

- **`Endianity`**

  The value of a `DW_AT_endianity` attribute.

- **`Accessibility`**

  The value of a `DW_AT_accessibility` attribute.

- **`Visibility`**

  The value of a `DW_AT_visibility` attribute.

- **`Virtuality`**

  The value of a `DW_AT_virtuality` attribute.

- **`Language`**

  The value of a `DW_AT_language` attribute.

- **`AddressClass`**

  The value of a `DW_AT_address_class` attribute.

- **`IdentifierCase`**

  The value of a `DW_AT_identifier_case` attribute.

- **`CallingConvention`**

  The value of a `DW_AT_calling_convention` attribute.

- **`Inline`**

  The value of a `DW_AT_inline` attribute.

- **`Ordering`**

  The value of a `DW_AT_ordering` attribute.

- **`FileIndex`**

  An index into the filename entries from the line number information
  table for the compilation unit containing this value.

- **`DwoId`**

  An implementation-defined identifier uniquely identifying a compilation
  unit.

#### Implementations

- <span id="attributevalue-u8-value"></span>`fn u8_value(&self) -> Option<u8>`

  Try to convert this attribute's value to a u8.

- <span id="attributevalue-u16-value"></span>`fn u16_value(&self) -> Option<u16>`

  Try to convert this attribute's value to a u16.

- <span id="attributevalue-udata-value"></span>`fn udata_value(&self) -> Option<u64>`

  Try to convert this attribute's value to an unsigned integer.

- <span id="attributevalue-sdata-value"></span>`fn sdata_value(&self) -> Option<i64>`

  Try to convert this attribute's value to a signed integer.

- <span id="attributevalue-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](../index.md#reader)

  Try to convert this attribute's value to an offset.

- <span id="attributevalue-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](../index.md#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attributevalue-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attributevalue-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, or a `DW_FORM_strp_sup` reference to an offset into a supplementary

  object file, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

#### Trait Implementations

##### `impl Any for AttributeValue<R, Offset>`

- <span id="attributevalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AttributeValue<R, Offset>`

- <span id="attributevalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AttributeValue<R, Offset>`

- <span id="attributevalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for AttributeValue<R, Offset>`

- <span id="attributevalue-clone"></span>`fn clone(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](../index.md#attributevalue)

##### `impl CloneToUninit for AttributeValue<R, Offset>`

- <span id="attributevalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for AttributeValue<R, Offset>`

##### `impl<R, Offset> Debug for AttributeValue<R, Offset>`

- <span id="attributevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AttributeValue<R, Offset>`

##### `impl<T> From for AttributeValue<R, Offset>`

- <span id="attributevalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AttributeValue<R, Offset>`

- <span id="attributevalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for AttributeValue<R, Offset>`

- <span id="attributevalue-partialeq-eq"></span>`fn eq(&self, other: &AttributeValue<R, Offset>) -> bool` — [`AttributeValue`](../index.md#attributevalue)

##### `impl<R, Offset> StructuralPartialEq for AttributeValue<R, Offset>`

##### `impl ToOwned for AttributeValue<R, Offset>`

- <span id="attributevalue-toowned-type-owned"></span>`type Owned = T`

- <span id="attributevalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="attributevalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AttributeValue<R, Offset>`

- <span id="attributevalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="attributevalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AttributeValue<R, Offset>`

- <span id="attributevalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="attributevalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_unit_type`

```rust
fn parse_unit_type<R: Reader>(input: &mut R) -> crate::read::Result<constants::DwUt>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:216-219`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L216-L219)*

Parse the unit type from the unit header.

### `parse_debug_abbrev_offset`

```rust
fn parse_debug_abbrev_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::common::DebugAbbrevOffset<<R as >::Offset>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:222-227`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L222-L227)*

Parse the `debug_abbrev_offset` in the compilation unit header.

### `parse_debug_info_offset`

```rust
fn parse_debug_info_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::common::DebugInfoOffset<<R as >::Offset>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:230-235`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L230-L235)*

Parse the `debug_info_offset` in the arange header.

### `parse_unit_header`

```rust
fn parse_unit_header<R, Offset>(input: &mut R, unit_offset: crate::common::UnitSectionOffset<Offset>) -> crate::read::Result<UnitHeader<R>>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:558-636`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L558-L636)*

Parse a unit header.

### `parse_dwo_id`

```rust
fn parse_dwo_id<R: Reader>(input: &mut R) -> crate::read::Result<crate::common::DwoId>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:639-641`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L639-L641)*

Parse a dwo_id from a header

### `length_u8_value`

```rust
fn length_u8_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1928-1931`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1928-L1931)*

### `length_u16_value`

```rust
fn length_u16_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1933-1936`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1933-L1936)*

### `length_u32_value`

```rust
fn length_u32_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1938-1941`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1938-L1941)*

### `length_uleb128_value`

```rust
fn length_uleb128_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1943-1946`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1943-L1946)*

### `allow_section_offset`

```rust
fn allow_section_offset(name: constants::DwAt, version: u16) -> bool
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1950-1968`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1950-L1968)*

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, spec: crate::read::AttributeSpecification) -> crate::read::Result<Attribute<R>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1970-2193`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L1970-L2193)*

### `skip_attributes`

```rust
fn skip_attributes<R: Reader>(input: &mut R, encoding: crate::common::Encoding, specs: &[crate::read::AttributeSpecification]) -> crate::read::Result<()>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2195-2261`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L2195-L2261)*

### `parse_type_signature`

```rust
fn parse_type_signature<R: Reader>(input: &mut R) -> crate::read::Result<crate::common::DebugTypeSignature>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3049-3051`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L3049-L3051)*

Parse a type unit header's unique type signature. Callers should handle
unique-ness checking.

### `parse_type_offset`

```rust
fn parse_type_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::read::UnitOffset<<R as >::Offset>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3054-3056`](../../../../.source_1765521767/gimli-0.32.3/src/read/unit.rs#L3054-L3056)*

Parse a type unit header's type offset.

