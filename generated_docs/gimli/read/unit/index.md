*[gimli](../../index.md) / [read](../index.md) / [unit](index.md)*

---

# Module `unit`

Functions for parsing DWARF `.debug_info` and `.debug_types` sections.

## Structs

### `DebugInfo<R>`

```rust
struct DebugInfo<R> {
    debug_info_section: R,
}
```

The `DebugInfo` struct represents the DWARF debugging information found in
the `.debug_info` section.

#### Implementations

- `fn units(self: &Self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](../index.md)

- `fn header_from_offset(self: &Self, offset: DebugInfoOffset<<R as >::Offset>) -> Result<UnitHeader<R>>` — [`DebugInfoOffset`](../../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`UnitHeader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugInfo<R>`

- `fn clone(self: &Self) -> DebugInfo<R>` — [`DebugInfo`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugInfo<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugInfo<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugInfo<R>`

- `fn default() -> DebugInfo<R>` — [`DebugInfo`](../index.md)

##### `impl<R> Section for DebugInfo<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugInfoUnitHeadersIter<R: Reader>`

```rust
struct DebugInfoUnitHeadersIter<R: Reader> {
    input: R,
    offset: crate::common::DebugInfoOffset<<R as >::Offset>,
}
```

An iterator over the units of a .debug_info section.

See the [documentation on
`DebugInfo::units`](./struct.DebugInfo.html#method.units) for more detail.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../../index.md), [`UnitHeader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugInfoUnitHeadersIter<R>`

- `fn clone(self: &Self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugInfoUnitHeadersIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

The common fields for the headers of compilation units and
type units.

#### Implementations

- `fn new(encoding: Encoding, unit_length: Offset, unit_type: UnitType<Offset>, debug_abbrev_offset: DebugAbbrevOffset<Offset>, unit_offset: UnitSectionOffset<Offset>, entries_buf: R) -> Self` — [`Encoding`](../../index.md), [`UnitType`](../index.md), [`DebugAbbrevOffset`](../../index.md), [`UnitSectionOffset`](../../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for UnitHeader<R, Offset>`

- `fn clone(self: &Self) -> UnitHeader<R, Offset>` — [`UnitHeader`](../index.md)

##### `impl<R, Offset> Copy for UnitHeader<R, Offset>`

##### `impl<R, Offset> Debug for UnitHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for UnitHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for UnitHeader<R, Offset>`

- `fn eq(self: &Self, other: &UnitHeader<R, Offset>) -> bool` — [`UnitHeader`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for UnitHeader<R, Offset>`

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

A Debugging Information Entry (DIE).

DIEs have a set of attributes and optionally have children DIEs as well.

#### Implementations

- `fn new(offset: UnitOffset<Offset>, attrs_slice: R, abbrev: &'abbrev Abbreviation, unit: &'unit UnitHeader<R, Offset>) -> Self` — [`UnitOffset`](../../index.md), [`Abbreviation`](../index.md), [`UnitHeader`](../index.md)

- `fn code(self: &Self) -> u64`

- `fn offset(self: &Self) -> UnitOffset<Offset>` — [`UnitOffset`](../../index.md)

- `fn tag(self: &Self) -> constants::DwTag` — [`DwTag`](../../index.md)

- `fn has_children(self: &Self) -> bool`

- `fn attrs<'me>(self: &'me Self) -> AttrsIter<'abbrev, 'me, 'unit, R>` — [`AttrsIter`](../index.md)

- `fn attr(self: &Self, name: constants::DwAt) -> Result<Option<Attribute<R>>>` — [`DwAt`](../../index.md), [`Result`](../../index.md), [`Attribute`](../index.md)

- `fn attr_value_raw(self: &Self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../../index.md), [`Result`](../../index.md), [`AttributeValue`](../index.md)

- `fn attr_value(self: &Self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../../index.md), [`Result`](../../index.md), [`AttributeValue`](../index.md)

- `fn after_attrs(self: &Self) -> Result<R>` — [`Result`](../../index.md)

- `fn sibling(self: &Self) -> Option<R>`

- `fn parse(input: &mut R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Result<Option<Self>>` — [`UnitHeader`](../index.md), [`Abbreviations`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R, Offset> Clone for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- `fn clone(self: &Self) -> DebuggingInformationEntry<'abbrev, 'unit, R, Offset>` — [`DebuggingInformationEntry`](../index.md)

##### `impl<'abbrev, 'unit, R, Offset> Debug for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Attribute<R: Reader>`

```rust
struct Attribute<R: Reader> {
    name: constants::DwAt,
    value: AttributeValue<R>,
}
```

An attribute in a `DebuggingInformationEntry`, consisting of a name and
associated value.

#### Implementations

- `fn name(self: &Self) -> constants::DwAt` — [`DwAt`](../../index.md)

- `fn raw_value(self: &Self) -> AttributeValue<R>` — [`AttributeValue`](../index.md)

- `fn value(self: &Self) -> AttributeValue<R>` — [`AttributeValue`](../index.md)

- `fn u8_value(self: &Self) -> Option<u8>`

- `fn u16_value(self: &Self) -> Option<u16>`

- `fn udata_value(self: &Self) -> Option<u64>`

- `fn sdata_value(self: &Self) -> Option<i64>`

- `fn offset_value(self: &Self) -> Option<<R as >::Offset>` — [`Reader`](../index.md)

- `fn exprloc_value(self: &Self) -> Option<Expression<R>>` — [`Expression`](../index.md)

- `fn string_value(self: &Self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](../index.md)

- `fn string_value_sup(self: &Self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for Attribute<R>`

- `fn clone(self: &Self) -> Attribute<R>` — [`Attribute`](../index.md)

##### `impl<R: $crate::marker::Copy + Reader> Copy for Attribute<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for Attribute<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for Attribute<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for Attribute<R>`

- `fn eq(self: &Self, other: &Attribute<R>) -> bool` — [`Attribute`](../index.md)

##### `impl<R: Reader> StructuralPartialEq for Attribute<R>`

### `AttrsIter<'abbrev, 'entry, 'unit, R: Reader>`

```rust
struct AttrsIter<'abbrev, 'entry, 'unit, R: Reader> {
    input: R,
    attributes: &'abbrev [crate::read::AttributeSpecification],
    entry: &'entry DebuggingInformationEntry<'abbrev, 'unit, R>,
}
```

An iterator over a particular entry's attributes.

See [the documentation for
`DebuggingInformationEntry::attrs()`](./struct.DebuggingInformationEntry.html#method.attrs)
for details.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<Attribute<R>>>` — [`Result`](../../index.md), [`Attribute`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'entry, 'unit, R: $crate::clone::Clone + Reader> Clone for AttrsIter<'abbrev, 'entry, 'unit, R>`

- `fn clone(self: &Self) -> AttrsIter<'abbrev, 'entry, 'unit, R>` — [`AttrsIter`](../index.md)

##### `impl<'abbrev, 'entry, 'unit, R: $crate::marker::Copy + Reader> Copy for AttrsIter<'abbrev, 'entry, 'unit, R>`

##### `impl<'abbrev, 'entry, 'unit, R: $crate::fmt::Debug + Reader> Debug for AttrsIter<'abbrev, 'entry, 'unit, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn is_empty(self: &Self) -> bool`

- `fn next_offset(self: &Self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md), [`Reader`](../index.md)

- `fn next_depth(self: &Self) -> isize`

- `fn read_abbreviation(self: &mut Self) -> Result<Option<&'abbrev Abbreviation>>` — [`Result`](../../index.md), [`Abbreviation`](../index.md)

- `fn read_attribute(self: &mut Self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](../index.md), [`Result`](../../index.md), [`Attribute`](../index.md)

- `fn skip_attributes(self: &mut Self, specs: &[AttributeSpecification]) -> Result<()>` — [`AttributeSpecification`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R> Clone for EntriesRaw<'abbrev, 'unit, R>`

- `fn clone(self: &Self) -> EntriesRaw<'abbrev, 'unit, R>` — [`EntriesRaw`](../index.md)

##### `impl<'abbrev, 'unit, R> Debug for EntriesRaw<'abbrev, 'unit, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn current(self: &Self) -> Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>` — [`DebuggingInformationEntry`](../index.md)

- `fn next_entry(self: &mut Self) -> Result<Option<()>>` — [`Result`](../../index.md)

- `fn next_dfs(self: &mut Self) -> Result<Option<(isize, &DebuggingInformationEntry<'abbrev, 'unit, R>)>>` — [`Result`](../../index.md), [`DebuggingInformationEntry`](../index.md)

- `fn next_sibling(self: &mut Self) -> Result<Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>>` — [`Result`](../../index.md), [`DebuggingInformationEntry`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R> Clone for EntriesCursor<'abbrev, 'unit, R>`

- `fn clone(self: &Self) -> EntriesCursor<'abbrev, 'unit, R>` — [`EntriesCursor`](../index.md)

##### `impl<'abbrev, 'unit, R> Debug for EntriesCursor<'abbrev, 'unit, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn new(root: R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Self` — [`UnitHeader`](../index.md), [`Abbreviations`](../index.md)

- `fn root<'me>(self: &'me mut Self) -> Result<EntriesTreeNode<'abbrev, 'unit, 'me, R>>` — [`Result`](../../index.md), [`EntriesTreeNode`](../index.md)

- `fn next(self: &mut Self, depth: isize) -> Result<bool>` — [`Result`](../../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R> Clone for EntriesTree<'abbrev, 'unit, R>`

- `fn clone(self: &Self) -> EntriesTree<'abbrev, 'unit, R>` — [`EntriesTree`](../index.md)

##### `impl<'abbrev, 'unit, R> Debug for EntriesTree<'abbrev, 'unit, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EntriesTreeNode<'abbrev, 'unit, 'tree, R: Reader>`

```rust
struct EntriesTreeNode<'abbrev, 'unit, 'tree, R: Reader> {
    tree: &'tree mut EntriesTree<'abbrev, 'unit, R>,
    depth: isize,
}
```

A node in the Debugging Information Entry tree.

The root node of a tree can be obtained
via [`EntriesTree::root`](./struct.EntriesTree.html#method.root).

#### Implementations

- `fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeNode<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](../index.md), [`EntriesTreeNode`](../index.md)

- `fn entry(self: &Self) -> &DebuggingInformationEntry<'abbrev, 'unit, R>` — [`DebuggingInformationEntry`](../index.md)

- `fn children(self: Self) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTreeIter`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, 'tree, R: $crate::fmt::Debug + Reader> Debug for EntriesTreeNode<'abbrev, 'unit, 'tree, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EntriesTreeIter<'abbrev, 'unit, 'tree, R: Reader>`

```rust
struct EntriesTreeIter<'abbrev, 'unit, 'tree, R: Reader> {
    tree: &'tree mut EntriesTree<'abbrev, 'unit, R>,
    depth: isize,
    empty: bool,
}
```

An iterator that allows traversal of the children of an
`EntriesTreeNode`.

The items returned by this iterator are also `EntriesTreeNode`s,
which allow recursive traversal of grandchildren, etc.

#### Implementations

- `fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](../index.md), [`EntriesTreeIter`](../index.md)

- `fn next<'me>(self: &'me mut Self) -> Result<Option<EntriesTreeNode<'abbrev, 'unit, 'me, R>>>` — [`Result`](../../index.md), [`EntriesTreeNode`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, 'tree, R: $crate::fmt::Debug + Reader> Debug for EntriesTreeIter<'abbrev, 'unit, 'tree, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DebugTypes<R>`

```rust
struct DebugTypes<R> {
    debug_types_section: R,
}
```

The `DebugTypes` struct represents the DWARF type information
found in the `.debug_types` section.

#### Implementations

- `fn units(self: &Self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugTypes<R>`

- `fn clone(self: &Self) -> DebugTypes<R>` — [`DebugTypes`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugTypes<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugTypes<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugTypes<R>`

- `fn default() -> DebugTypes<R>` — [`DebugTypes`](../index.md)

##### `impl<R> Section for DebugTypes<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugTypesUnitHeadersIter<R: Reader>`

```rust
struct DebugTypesUnitHeadersIter<R: Reader> {
    input: R,
    offset: crate::common::DebugTypesOffset<<R as >::Offset>,
}
```

An iterator over the type-units of this `.debug_types` section.

See the [documentation on
`DebugTypes::units`](./struct.DebugTypes.html#method.units) for
more detail.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../../index.md), [`UnitHeader`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugTypesUnitHeadersIter<R>`

- `fn clone(self: &Self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugTypesUnitHeadersIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn dw_ut(self: &Self) -> constants::DwUt` — [`DwUt`](../../index.md)

#### Trait Implementations

##### `impl<Offset> Clone for UnitType<Offset>`

- `fn clone(self: &Self) -> UnitType<Offset>` — [`UnitType`](../index.md)

##### `impl<Offset> Copy for UnitType<Offset>`

##### `impl<Offset> Debug for UnitType<Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Offset> Eq for UnitType<Offset>`

##### `impl<Offset> PartialEq for UnitType<Offset>`

- `fn eq(self: &Self, other: &UnitType<Offset>) -> bool` — [`UnitType`](../index.md)

##### `impl<Offset> StructuralPartialEq for UnitType<Offset>`

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

- `fn u8_value(self: &Self) -> Option<u8>`

- `fn u16_value(self: &Self) -> Option<u16>`

- `fn udata_value(self: &Self) -> Option<u64>`

- `fn sdata_value(self: &Self) -> Option<i64>`

- `fn offset_value(self: &Self) -> Option<<R as >::Offset>` — [`Reader`](../index.md)

- `fn exprloc_value(self: &Self) -> Option<Expression<R>>` — [`Expression`](../index.md)

- `fn string_value(self: &Self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](../index.md)

- `fn string_value_sup(self: &Self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for AttributeValue<R, Offset>`

- `fn clone(self: &Self) -> AttributeValue<R, Offset>` — [`AttributeValue`](../index.md)

##### `impl<R, Offset> Copy for AttributeValue<R, Offset>`

##### `impl<R, Offset> Debug for AttributeValue<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for AttributeValue<R, Offset>`

##### `impl<R, Offset> PartialEq for AttributeValue<R, Offset>`

- `fn eq(self: &Self, other: &AttributeValue<R, Offset>) -> bool` — [`AttributeValue`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for AttributeValue<R, Offset>`

## Functions

### `parse_unit_type`

```rust
fn parse_unit_type<R: Reader>(input: &mut R) -> crate::read::Result<constants::DwUt>
```

Parse the unit type from the unit header.

### `parse_debug_abbrev_offset`

```rust
fn parse_debug_abbrev_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::common::DebugAbbrevOffset<<R as >::Offset>>
```

Parse the `debug_abbrev_offset` in the compilation unit header.

### `parse_debug_info_offset`

```rust
fn parse_debug_info_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::common::DebugInfoOffset<<R as >::Offset>>
```

Parse the `debug_info_offset` in the arange header.

### `parse_unit_header`

```rust
fn parse_unit_header<R, Offset>(input: &mut R, unit_offset: crate::common::UnitSectionOffset<Offset>) -> crate::read::Result<UnitHeader<R>>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset
```

Parse a unit header.

### `parse_dwo_id`

```rust
fn parse_dwo_id<R: Reader>(input: &mut R) -> crate::read::Result<crate::common::DwoId>
```

Parse a dwo_id from a header

### `length_u8_value`

```rust
fn length_u8_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

### `length_u16_value`

```rust
fn length_u16_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

### `length_u32_value`

```rust
fn length_u32_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

### `length_uleb128_value`

```rust
fn length_uleb128_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

### `allow_section_offset`

```rust
fn allow_section_offset(name: constants::DwAt, version: u16) -> bool
```

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, spec: crate::read::AttributeSpecification) -> crate::read::Result<Attribute<R>>
```

### `skip_attributes`

```rust
fn skip_attributes<R: Reader>(input: &mut R, encoding: crate::common::Encoding, specs: &[crate::read::AttributeSpecification]) -> crate::read::Result<()>
```

### `parse_type_signature`

```rust
fn parse_type_signature<R: Reader>(input: &mut R) -> crate::read::Result<crate::common::DebugTypeSignature>
```

Parse a type unit header's unique type signature. Callers should handle
unique-ness checking.

### `parse_type_offset`

```rust
fn parse_type_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::read::UnitOffset<<R as >::Offset>>
```

Parse a type unit header's type offset.

