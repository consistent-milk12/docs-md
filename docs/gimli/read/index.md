*[gimli](../index.md) / [read](index.md)*

---

# Module `read`

Read DWARF debugging information.

* [Example Usage](#example-usage)
* [API Structure](#api-structure)
* [Using with `FallibleIterator`](#using-with-fallibleiterator)

## Example Usage

Print out all of the functions in the debuggee program:

```rust,no_run
fn example() -> Result<(), gimli::Error> {
type R = gimli::EndianSlice<'static, gimli::LittleEndian>;
let get_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
let get_sup_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
// Read the DWARF sections with whatever object loader you're using.
// These closures should return a `Reader` instance (e.g. `EndianSlice`).
let loader = |section: gimli::SectionId| { get_file_section_reader(section.name()) };
let sup_loader = |section: gimli::SectionId| { get_sup_file_section_reader(section.name()) };
let mut dwarf = gimli::Dwarf::load(loader)?;
dwarf.load_sup(sup_loader)?;

// Iterate over all compilation units.
let mut iter = dwarf.units();
while let Some(header) = iter.next()? {
    // Parse the abbreviations and other information for this compilation unit.
    let unit = dwarf.unit(header)?;

    // Iterate over all of this compilation unit's entries.
    let mut entries = unit.entries();
    while let Some((_, entry)) = entries.next_dfs()? {
        // If we find an entry for a function, print it.
        if entry.tag() == gimli::DW_TAG_subprogram {
            println!("Found a function: {:?}", entry);
        }
    }
}
unreachable!()
}
```

Full example programs:

  * [A simple `.debug_info` parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple.rs)

  * [A simple `.debug_line` parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple_line.rs)

  * [A `dwarfdump`
    clone](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarfdump.rs)

  * [An `addr2line` clone](https://github.com/gimli-rs/addr2line)

  * [`ddbug`](https://github.com/gimli-rs/ddbug), a utility giving insight into
    code generation by making debugging information readable

  * [`dwprod`](https://github.com/fitzgen/dwprod), a tiny utility to list the
    compilers used to create each compilation unit within a shared library or
    executable (via `DW_AT_producer`)

  * [`dwarf-validate`](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarf-validate.rs),
    a program to validate the integrity of some DWARF and its references
    between sections and compilation units.

## API Structure

* Basic familiarity with DWARF is assumed.

* The [`Dwarf`](./struct.Dwarf.html) type contains the commonly used DWARF
  sections. It has methods that simplify access to debugging data that spans
  multiple sections. Use of this type is optional, but recommended.

* The [`DwarfPackage`](./struct.Dwarf.html) type contains the DWARF
  package (DWP) sections. It has methods to find a DWARF object (DWO)
  within the package.

* Each section gets its own type. Consider these types the entry points to
  the library:

  * [`DebugAbbrev`](./struct.DebugAbbrev.html): The `.debug_abbrev` section.

  * [`DebugAddr`](./struct.DebugAddr.html): The `.debug_addr` section.

  * [`DebugAranges`](./struct.DebugAranges.html): The `.debug_aranges`
    section.

  * [`DebugFrame`](./struct.DebugFrame.html): The `.debug_frame` section.

  * [`DebugInfo`](./struct.DebugInfo.html): The `.debug_info` section.

  * [`DebugLine`](./struct.DebugLine.html): The `.debug_line` section.

  * [`DebugLineStr`](./struct.DebugLineStr.html): The `.debug_line_str` section.

  * [`DebugLoc`](./struct.DebugLoc.html): The `.debug_loc` section.

  * [`DebugLocLists`](./struct.DebugLocLists.html): The `.debug_loclists` section.

  * [`DebugPubNames`](./struct.DebugPubNames.html): The `.debug_pubnames`
    section.

  * [`DebugPubTypes`](./struct.DebugPubTypes.html): The `.debug_pubtypes`
    section.

  * [`DebugRanges`](./struct.DebugRanges.html): The `.debug_ranges` section.

  * [`DebugRngLists`](./struct.DebugRngLists.html): The `.debug_rnglists` section.

  * [`DebugStr`](./struct.DebugStr.html): The `.debug_str` section.

  * [`DebugStrOffsets`](./struct.DebugStrOffsets.html): The `.debug_str_offsets` section.

  * [`DebugTypes`](./struct.DebugTypes.html): The `.debug_types` section.

  * [`DebugCuIndex`](./struct.DebugCuIndex.html): The `.debug_cu_index` section.

  * [`DebugTuIndex`](./struct.DebugTuIndex.html): The `.debug_tu_index` section.

  * [`EhFrame`](./struct.EhFrame.html): The `.eh_frame` section.

  * [`EhFrameHdr`](./struct.EhFrameHdr.html): The `.eh_frame_hdr` section.

* Each section type exposes methods for accessing the debugging data encoded
  in that section. For example, the [`DebugInfo`](./struct.DebugInfo.html)
  struct has the [`units`](./struct.DebugInfo.html#method.units) method for
  iterating over the compilation units defined within it.

* Offsets into a section are strongly typed: an offset into `.debug_info` is
  the [`DebugInfoOffset`](./struct.DebugInfoOffset.html) type. It cannot be
  used to index into the [`DebugLine`](./struct.DebugLine.html) type because
  `DebugLine` represents the `.debug_line` section. There are similar types
  for offsets relative to a compilation unit rather than a section.

## Using with `FallibleIterator`

The standard library's `Iterator` trait and related APIs do not play well
with iterators where the `next` operation is fallible. One can make the
`Iterator`'s associated `Item` type be a `Result<T, E>`, however the
provided methods cannot gracefully handle the case when an `Err` is
returned.

This situation led to the
[`fallible-iterator`](https://crates.io/crates/fallible-iterator) crate's
existence. You can read more of the rationale for its existence in its
docs. The crate provides the helpers you have come to expect (eg `map`,
`filter`, etc) for iterators that can fail.

`gimli`'s many lazy parsing iterators are a perfect match for the
`fallible-iterator` crate's `FallibleIterator` trait because parsing is not
done eagerly. Parse errors later in the input might only be discovered after
having iterated through many items.

To use `gimli` iterators with `FallibleIterator`, import the crate and trait
into your code:

```rust
#[cfg(feature = "fallible-iterator")]
fn foo() {
// Use the `FallibleIterator` trait so its methods are in scope!
use fallible_iterator::FallibleIterator;
use gimli::{DebugAranges, EndianSlice, LittleEndian};

fn find_sum_of_address_range_lengths(aranges: DebugAranges<EndianSlice<LittleEndian>>)
    -> gimli::Result<u64>
{
    // `DebugAranges::headers` returns a `FallibleIterator`!
    aranges.headers()
        // `flat_map` is provided by `FallibleIterator`!
        .flat_map(|header| Ok(header.entries()))
        // `map` is provided by `FallibleIterator`!
        .map(|arange| Ok(arange.length()))
        // `fold` is provided by `FallibleIterator`!
        .fold(0, |sum, len| Ok(sum + len))
}
}
fn main() {}
```

## Structs

### `UnitOffset<T>`

```rust
struct UnitOffset<T>(T);
```

An offset into the current compilation or type unit.

#### Implementations

- `fn to_unit_section_offset<R>(self: &Self, unit: &Unit<R>) -> UnitSectionOffset<T>` — [`Unit`](../../read/dwarf/index.md), [`UnitSectionOffset`](../../common/index.md)

#### Trait Implementations

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> UnitOffset<T>` — [`UnitOffset`](../../read/index.md)

##### `impl Copy<T: $crate::marker::Copy>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<T: $crate::cmp::Eq>`

##### `impl Hash<T: $crate::hash::Hash>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<T: $crate::cmp::Ord>`

- `fn cmp(self: &Self, other: &UnitOffset<T>) -> $crate::cmp::Ordering` — [`UnitOffset`](../../read/index.md)

##### `impl PartialEq<T: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &UnitOffset<T>) -> bool` — [`UnitOffset`](../../read/index.md)

##### `impl PartialOrd<T: $crate::cmp::PartialOrd>`

- `fn partial_cmp(self: &Self, other: &UnitOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`UnitOffset`](../../read/index.md)

##### `impl StructuralPartialEq<T>`

### `StoreOnHeap`

```rust
struct StoreOnHeap;
```

Indicates that storage should be allocated on heap.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> StoreOnHeap` — [`StoreOnHeap`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl EvaluationStorage<R: Reader>`

- `type Stack = Vec<Value>`

- `type ExpressionStack = Vec<(R, R)>`

- `type Result = Vec<Piece<R>>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StoreOnHeap) -> bool` — [`StoreOnHeap`](../../read/index.md)

##### `impl StructuralPartialEq`

##### `impl UnwindContextStorage<T: ReaderOffset>`

- `type Rules = [(Register, RegisterRule<T>); 192]`

- `type Stack = Box<[UnwindTableRow<T>; 4]>`

## Enums

### `Error`

```rust
enum Error {
    Io,
    PcRelativePointerButSectionBaseIsUndefined,
    TextRelativePointerButTextBaseIsUndefined,
    DataRelativePointerButDataBaseIsUndefined,
    FuncRelativePointerInBadContext,
    CannotParseOmitPointerEncoding,
    BadUnsignedLeb128,
    BadSignedLeb128,
    AbbreviationTagZero,
    AttributeFormZero,
    BadHasChildren,
    BadLength,
    UnknownForm(constants::DwForm),
    ExpectedZero,
    DuplicateAbbreviationCode,
    DuplicateArange,
    UnknownReservedLength,
    UnknownVersion(u64),
    UnknownAbbreviation(u64),
    UnexpectedEof(ReaderOffsetId),
    UnexpectedNull,
    UnknownStandardOpcode(constants::DwLns),
    UnknownExtendedOpcode(constants::DwLne),
    UnknownLocListsEntry(constants::DwLle),
    UnknownRangeListsEntry(constants::DwRle),
    UnsupportedAddressSize(u8),
    UnsupportedOffsetSize(u8),
    UnsupportedFieldSize(u8),
    MinimumInstructionLengthZero,
    MaximumOperationsPerInstructionZero,
    LineRangeZero,
    OpcodeBaseZero,
    BadUtf8,
    NotCieId,
    NotCiePointer,
    NotFdePointer,
    BadBranchTarget(u64),
    InvalidPushObjectAddress,
    NotEnoughStackItems,
    TooManyIterations,
    InvalidExpression(constants::DwOp),
    UnsupportedEvaluation,
    InvalidPiece,
    InvalidExpressionTerminator(u64),
    DivisionByZero,
    TypeMismatch,
    IntegralTypeRequired,
    UnsupportedTypeOperation,
    InvalidShiftExpression,
    InvalidDerefSize(u8),
    UnknownCallFrameInstruction(constants::DwCfa),
    InvalidAddressRange,
    AddressOverflow,
    CfiInstructionInInvalidContext,
    PopWithEmptyStack,
    NoUnwindInfoForAddress,
    UnsupportedOffset,
    UnknownPointerEncoding(constants::DwEhPe),
    NoEntryAtGivenOffset,
    OffsetOutOfBounds,
    UnknownAugmentation,
    UnsupportedPointerEncoding,
    UnsupportedRegister(u64),
    TooManyRegisterRules,
    StackFull,
    VariableLengthSearchTable,
    UnsupportedUnitType,
    UnsupportedAddressIndex,
    UnsupportedSegmentSize,
    MissingUnitDie,
    UnsupportedAttributeForm,
    MissingFileEntryFormatPath,
    ExpectedStringAttributeValue,
    InvalidImplicitConst,
    InvalidIndexSectionCount,
    InvalidIndexSlotCount,
    InvalidIndexRow,
    UnknownIndexSection(constants::DwSect),
    UnknownIndexSectionV2(constants::DwSectV2),
    InvalidMacinfoType(constants::DwMacinfo),
    InvalidMacroType(constants::DwMacro),
    UnsupportedOpcodeOperandsTable,
}
```

An error that occurred when parsing.

#### Variants

- **`Io`**

  An I/O error occurred while reading.

- **`PcRelativePointerButSectionBaseIsUndefined`**

  Found a PC relative pointer, but the section base is undefined.

- **`TextRelativePointerButTextBaseIsUndefined`**

  Found a `.text` relative pointer, but the `.text` base is undefined.

- **`DataRelativePointerButDataBaseIsUndefined`**

  Found a data relative pointer, but the data base is undefined.

- **`FuncRelativePointerInBadContext`**

  Found a function relative pointer in a context that does not have a
  function base.

- **`CannotParseOmitPointerEncoding`**

  Cannot parse a pointer with a `DW_EH_PE_omit` encoding.

- **`BadUnsignedLeb128`**

  An error parsing an unsigned LEB128 value.

- **`BadSignedLeb128`**

  An error parsing a signed LEB128 value.

- **`AbbreviationTagZero`**

  An abbreviation declared that its tag is zero, but zero is reserved for
  null records.

- **`AttributeFormZero`**

  An attribute specification declared that its form is zero, but zero is
  reserved for null records.

- **`BadHasChildren`**

  The abbreviation's has-children byte was not one of
  `DW_CHILDREN_{yes,no}`.

- **`BadLength`**

  The specified length is impossible.

- **`UnknownForm`**

  Found an unknown `DW_FORM_*` type.

- **`ExpectedZero`**

  Expected a zero, found something else.

- **`DuplicateAbbreviationCode`**

  Found an abbreviation code that has already been used.

- **`DuplicateArange`**

  Found a duplicate arange.

- **`UnknownReservedLength`**

  Found an unknown reserved length value.

- **`UnknownVersion`**

  Found an unknown DWARF version.

- **`UnknownAbbreviation`**

  Found a record with an unknown abbreviation code.

- **`UnexpectedEof`**

  Hit the end of input before it was expected.

- **`UnexpectedNull`**

  Read a null entry before it was expected.

- **`UnknownStandardOpcode`**

  Found an unknown standard opcode.

- **`UnknownExtendedOpcode`**

  Found an unknown extended opcode.

- **`UnknownLocListsEntry`**

  Found an unknown location-lists format.

- **`UnknownRangeListsEntry`**

  Found an unknown range-lists format.

- **`UnsupportedAddressSize`**

  The specified address size is not supported.

- **`UnsupportedOffsetSize`**

  The specified offset size is not supported.

- **`UnsupportedFieldSize`**

  The specified field size is not supported.

- **`MinimumInstructionLengthZero`**

  The minimum instruction length must not be zero.

- **`MaximumOperationsPerInstructionZero`**

  The maximum operations per instruction must not be zero.

- **`LineRangeZero`**

  The line range must not be zero.

- **`OpcodeBaseZero`**

  The opcode base must not be zero.

- **`BadUtf8`**

  Found an invalid UTF-8 string.

- **`NotCieId`**

  Expected to find the CIE ID, but found something else.

- **`NotCiePointer`**

  Expected to find a pointer to a CIE, but found the CIE ID instead.

- **`NotFdePointer`**

  Expected to find a pointer to an FDE, but found a CIE instead.

- **`BadBranchTarget`**

  Invalid branch target for a DW_OP_bra or DW_OP_skip.

- **`InvalidPushObjectAddress`**

  DW_OP_push_object_address used but no address passed in.

- **`NotEnoughStackItems`**

  Not enough items on the stack when evaluating an expression.

- **`TooManyIterations`**

  Too many iterations to compute the expression.

- **`InvalidExpression`**

  An unrecognized operation was found while parsing a DWARF
  expression.

- **`UnsupportedEvaluation`**

  An unsupported operation was found while evaluating a DWARF expression.

- **`InvalidPiece`**

  The expression had a piece followed by an expression
  terminator without a piece.

- **`InvalidExpressionTerminator`**

  An expression-terminating operation was followed by something
  other than the end of the expression or a piece operation.

- **`DivisionByZero`**

  Division or modulus by zero when evaluating an expression.

- **`TypeMismatch`**

  An expression operation used mismatching types.

- **`IntegralTypeRequired`**

  An expression operation required an integral type but saw a
  floating point type.

- **`UnsupportedTypeOperation`**

  An expression operation used types that are not supported.

- **`InvalidShiftExpression`**

  The shift value in an expression must be a non-negative integer.

- **`InvalidDerefSize`**

  The size of a deref expression must not be larger than the size of an address.

- **`UnknownCallFrameInstruction`**

  An unknown DW_CFA_* instruction.

- **`InvalidAddressRange`**

  The end of an address range was before the beginning.

- **`AddressOverflow`**

  An address calculation overflowed.
  
  This is returned in cases where the address is expected to be
  larger than a previous address, but the calculation overflowed.

- **`CfiInstructionInInvalidContext`**

  Encountered a call frame instruction in a context in which it is not
  valid.

- **`PopWithEmptyStack`**

  When evaluating call frame instructions, found a `DW_CFA_restore_state`
  stack pop instruction, but the stack was empty, and had nothing to pop.

- **`NoUnwindInfoForAddress`**

  Do not have unwind info for the given address.

- **`UnsupportedOffset`**

  An offset value was larger than the maximum supported value.

- **`UnknownPointerEncoding`**

  The given pointer encoding is either unknown or invalid.

- **`NoEntryAtGivenOffset`**

  Did not find an entry at the given offset.

- **`OffsetOutOfBounds`**

  The given offset is out of bounds.

- **`UnknownAugmentation`**

  Found an unknown CFI augmentation.

- **`UnsupportedPointerEncoding`**

  We do not support the given pointer encoding yet.

- **`UnsupportedRegister`**

  Registers larger than `u16` are not supported.

- **`TooManyRegisterRules`**

  The CFI program defined more register rules than we have storage for.

- **`StackFull`**

  Attempted to push onto the CFI or evaluation stack, but it was already
  at full capacity.

- **`VariableLengthSearchTable`**

  The `.eh_frame_hdr` binary search table claims to be variable-length encoded,
  which makes binary search impossible.

- **`UnsupportedUnitType`**

  The `DW_UT_*` value for this unit is not supported yet.

- **`UnsupportedAddressIndex`**

  Ranges using AddressIndex are not supported yet.

- **`UnsupportedSegmentSize`**

  Nonzero segment selector sizes aren't supported yet.

- **`MissingUnitDie`**

  A compilation unit or type unit is missing its top level DIE.

- **`UnsupportedAttributeForm`**

  A DIE attribute used an unsupported form.

- **`MissingFileEntryFormatPath`**

  Missing DW_LNCT_path in file entry format.

- **`ExpectedStringAttributeValue`**

  Expected an attribute value to be a string form.

- **`InvalidImplicitConst`**

  `DW_FORM_implicit_const` used in an invalid context.

- **`InvalidIndexSectionCount`**

  Invalid section count in `.dwp` index.

- **`InvalidIndexSlotCount`**

  Invalid slot count in `.dwp` index.

- **`InvalidIndexRow`**

  Invalid hash row in `.dwp` index.

- **`UnknownIndexSection`**

  Unknown section type in `.dwp` index.

- **`UnknownIndexSectionV2`**

  Unknown section type in version 2 `.dwp` index.

- **`InvalidMacinfoType`**

  Invalid macinfo type in `.debug_macinfo`.

- **`InvalidMacroType`**

  Invalid macro type in `.debug_macro`.

- **`UnsupportedOpcodeOperandsTable`**

  The optional `opcode_operands_table` in `.debug_macro` is currently not supported.

#### Implementations

- `fn description(self: &Self) -> &str`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Error` — [`Error`](../../read/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> ::core::result::Result<(), fmt::Error>`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](../../read/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

## Traits

### `Section<R>`

```rust
trait Section<R>: From<R> { ... }
```

A convenience trait for loading DWARF sections from object files.  To be
used like:

```rust
use gimli::{DebugInfo, EndianSlice, LittleEndian, Reader, Section};

let buf = [0x00, 0x01, 0x02, 0x03];
let reader = EndianSlice::new(&buf, LittleEndian);
let loader = |name| -> Result<_, ()> { Ok(reader) };

let debug_info: DebugInfo<_> = Section::load(loader).unwrap();
```

#### Required Methods

- `fn id() -> SectionId`

  Returns the section id for this type.

- `fn section_name() -> &'static str`

  Returns the ELF section name for this type.

- `fn dwo_section_name() -> Option<&'static str>`

  Returns the ELF section name (if any) for this type when used in a dwo

- `fn xcoff_section_name() -> Option<&'static str>`

  Returns the XCOFF section name (if any) for this type when used in a XCOFF

- `fn load<F, E>(f: F) -> core::result::Result<Self, E>`

  Try to load the section using the given loader function.

- `fn reader(self: &Self) -> &R`

  Returns the `Reader` for this section.

- `fn dwp_range(self: &Self, offset: u32, size: u32) -> Result<Self>`

  Returns the subrange of the section that is the contribution of

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>`

  Returns the `Reader` for this section.

## Type Aliases

### `EndianBuf<'input, Endian>`

```rust
type EndianBuf<'input, Endian> = EndianSlice<'input, Endian>;
```

`EndianBuf` has been renamed to `EndianSlice`. For ease of upgrading across
`gimli` versions, we export this type alias.

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result of a parse.

