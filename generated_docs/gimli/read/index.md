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

## Contents

- [Modules](#modules)
  - [`util`](#util)
  - [`addr`](#addr)
  - [`cfi`](#cfi)
  - [`dwarf`](#dwarf)
  - [`endian_slice`](#endian-slice)
  - [`reader`](#reader)
  - [`relocate`](#relocate)
  - [`abbrev`](#abbrev)
  - [`aranges`](#aranges)
  - [`index`](#index)
  - [`line`](#line)
  - [`lists`](#lists)
  - [`loclists`](#loclists)
  - [`lookup`](#lookup)
  - [`macros`](#macros)
  - [`op`](#op)
  - [`pubnames`](#pubnames)
  - [`pubtypes`](#pubtypes)
  - [`rnglists`](#rnglists)
  - [`str`](#str)
  - [`unit`](#unit)
  - [`value`](#value)
  - [`sealed`](#sealed)
- [Structs](#structs)
  - [`UnitOffset`](#unitoffset)
  - [`StoreOnHeap`](#storeonheap)
  - [`ArrayVec`](#arrayvec)
  - [`DebugAddr`](#debugaddr)
  - [`AddrHeaderIter`](#addrheaderiter)
  - [`AddrHeader`](#addrheader)
  - [`AddrEntryIter`](#addrentryiter)
  - [`DebugFrame`](#debugframe)
  - [`EhFrameHdr`](#ehframehdr)
  - [`ParsedEhFrameHdr`](#parsedehframehdr)
  - [`EhHdrTableIter`](#ehhdrtableiter)
  - [`EhHdrTable`](#ehhdrtable)
  - [`EhFrame`](#ehframe)
  - [`BaseAddresses`](#baseaddresses)
  - [`SectionBaseAddresses`](#sectionbaseaddresses)
  - [`CfiEntriesIter`](#cfientriesiter)
  - [`Augmentation`](#augmentation)
  - [`AugmentationData`](#augmentationdata)
  - [`CommonInformationEntry`](#commoninformationentry)
  - [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)
  - [`FrameDescriptionEntry`](#framedescriptionentry)
  - [`UnwindContext`](#unwindcontext)
  - [`UnwindTable`](#unwindtable)
  - [`RegisterRuleMap`](#registerrulemap)
  - [`RegisterRuleIter`](#registerruleiter)
  - [`UnwindTableRow`](#unwindtablerow)
  - [`CallFrameInstructionIter`](#callframeinstructioniter)
  - [`UnwindExpression`](#unwindexpression)
  - [`PointerEncodingParameters`](#pointerencodingparameters)
  - [`DwarfSections`](#dwarfsections)
  - [`Dwarf`](#dwarf)
  - [`DwarfPackageSections`](#dwarfpackagesections)
  - [`DwarfPackage`](#dwarfpackage)
  - [`Unit`](#unit)
  - [`UnitRef`](#unitref)
  - [`RangeIter`](#rangeiter)
  - [`EndianSlice`](#endianslice)
  - [`DebugBytes`](#debugbytes)
  - [`DebugByte`](#debugbyte)
  - [`DebugLen`](#debuglen)
  - [`ReaderOffsetId`](#readeroffsetid)
  - [`RelocateReader`](#relocatereader)
  - [`DebugAbbrev`](#debugabbrev)
  - [`AbbreviationsCache`](#abbreviationscache)
  - [`Abbreviations`](#abbreviations)
  - [`Abbreviation`](#abbreviation)
  - [`AttributeSpecification`](#attributespecification)
  - [`DebugAranges`](#debugaranges)
  - [`ArangeHeaderIter`](#arangeheaderiter)
  - [`ArangeHeader`](#arangeheader)
  - [`ArangeEntryIter`](#arangeentryiter)
  - [`ArangeEntry`](#arangeentry)
  - [`DebugCuIndex`](#debugcuindex)
  - [`DebugTuIndex`](#debugtuindex)
  - [`UnitIndex`](#unitindex)
  - [`UnitIndexSectionIterator`](#unitindexsectioniterator)
  - [`UnitIndexSection`](#unitindexsection)
  - [`DebugLine`](#debugline)
  - [`LineRows`](#linerows)
  - [`LineInstructions`](#lineinstructions)
  - [`LineRow`](#linerow)
  - [`LineSequence`](#linesequence)
  - [`LineProgramHeader`](#lineprogramheader)
  - [`IncompleteLineProgram`](#incompletelineprogram)
  - [`CompleteLineProgram`](#completelineprogram)
  - [`FileEntry`](#fileentry)
  - [`FileEntryFormat`](#fileentryformat)
  - [`DebugLoc`](#debugloc)
  - [`DebugLocLists`](#debugloclists)
  - [`LocationLists`](#locationlists)
  - [`RawLocListIter`](#rawloclistiter)
  - [`LocListIter`](#loclistiter)
  - [`LocationListEntry`](#locationlistentry)
  - [`DebugMacinfo`](#debugmacinfo)
  - [`DebugMacro`](#debugmacro)
  - [`MacroUnitHeader`](#macrounitheader)
  - [`MacroIter`](#macroiter)
  - [`Piece`](#piece)
  - [`Expression`](#expression)
  - [`OperationIter`](#operationiter)
  - [`Evaluation`](#evaluation)
  - [`PubNamesEntry`](#pubnamesentry)
  - [`DebugPubNames`](#debugpubnames)
  - [`PubNamesEntryIter`](#pubnamesentryiter)
  - [`PubTypesEntry`](#pubtypesentry)
  - [`DebugPubTypes`](#debugpubtypes)
  - [`PubTypesEntryIter`](#pubtypesentryiter)
  - [`DebugRanges`](#debugranges)
  - [`DebugRngLists`](#debugrnglists)
  - [`RangeLists`](#rangelists)
  - [`RawRngListIter`](#rawrnglistiter)
  - [`RngListIter`](#rnglistiter)
  - [`RawRange`](#rawrange)
  - [`Range`](#range)
  - [`DebugStr`](#debugstr)
  - [`DebugStrOffsets`](#debugstroffsets)
  - [`DebugLineStr`](#debuglinestr)
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
  - [`Error`](#error)
  - [`CieOrFde`](#cieorfde)
  - [`CfaRule`](#cfarule)
  - [`RegisterRule`](#registerrule)
  - [`CallFrameInstruction`](#callframeinstruction)
  - [`Pointer`](#pointer)
  - [`RangeIterInner`](#rangeiterinner)
  - [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)
  - [`Attributes`](#attributes)
  - [`IndexSectionId`](#indexsectionid)
  - [`LineInstruction`](#lineinstruction)
  - [`ColumnType`](#columntype)
  - [`LocListsFormat`](#loclistsformat)
  - [`RawLocListEntry`](#rawloclistentry)
  - [`MacroString`](#macrostring)
  - [`MacroEntry`](#macroentry)
  - [`DieReference`](#diereference)
  - [`Operation`](#operation)
  - [`OperationEvaluationResult`](#operationevaluationresult)
  - [`Location`](#location)
  - [`EvaluationState`](#evaluationstate)
  - [`EvaluationWaiting`](#evaluationwaiting)
  - [`EvaluationResult`](#evaluationresult)
  - [`RangeListsFormat`](#rangelistsformat)
  - [`RawRngListEntry`](#rawrnglistentry)
  - [`UnitType`](#unittype)
  - [`AttributeValue`](#attributevalue)
  - [`ValueType`](#valuetype)
  - [`Value`](#value)
- [Traits](#traits)
  - [`Section`](#section)
  - [`ArrayLike`](#arraylike)
  - [`UnwindOffset`](#unwindoffset)
  - [`UnwindSection`](#unwindsection)
  - [`UnwindContextStorage`](#unwindcontextstorage)
  - [`ReaderOffset`](#readeroffset)
  - [`ReaderAddress`](#readeraddress)
  - [`Reader`](#reader)
  - [`Relocate`](#relocate)
  - [`LineProgram`](#lineprogram)
  - [`EvaluationStorage`](#evaluationstorage)
- [Functions](#functions)
  - [`parse_cfi_entry`](#parse-cfi-entry)
  - [`parse_encoded_pointer`](#parse-encoded-pointer)
  - [`parse_encoded_value`](#parse-encoded-value)
  - [`get_attribute_size`](#get-attribute-size)
  - [`parse_directory_v5`](#parse-directory-v5)
  - [`parse_file_v5`](#parse-file-v5)
  - [`parse_attribute`](#parse-attribute)
  - [`parse_data`](#parse-data)
  - [`compute_pc`](#compute-pc)
  - [`generic_type`](#generic-type)
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
  - [`sign_extend`](#sign-extend)
  - [`mask_bit_size`](#mask-bit-size)
- [Type Aliases](#type-aliases)
  - [`EndianBuf`](#endianbuf)
  - [`Result`](#result)
  - [`LineNumberProgram`](#linenumberprogram)
  - [`StateMachine`](#statemachine)
  - [`OneShotLineRows`](#oneshotlinerows)
  - [`ResumedLineRows`](#resumedlinerows)
  - [`Opcode`](#opcode)
  - [`OpcodesIter`](#opcodesiter)
  - [`LineNumberRow`](#linenumberrow)
  - [`LineNumberSequence`](#linenumbersequence)
  - [`LineNumberProgramHeader`](#linenumberprogramheader)
  - [`IncompleteLineNumberProgram`](#incompletelinenumberprogram)
  - [`CompleteLineNumberProgram`](#completelinenumberprogram)
  - [`LocListsHeader`](#loclistsheader)
  - [`RngListsHeader`](#rnglistsheader)
- [Constants](#constants)
  - [`MAX_RULES`](#max-rules)
  - [`MAX_UNWIND_STACK_DEPTH`](#max-unwind-stack-depth)
  - [`CFI_INSTRUCTION_HIGH_BITS_MASK`](#cfi-instruction-high-bits-mask)
  - [`CFI_INSTRUCTION_LOW_BITS_MASK`](#cfi-instruction-low-bits-mask)
  - [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline)
  - [`SECTION_COUNT_MAX`](#section-count-max)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`util`](#util) | mod |  |
| [`addr`](#addr) | mod |  |
| [`cfi`](#cfi) | mod |  |
| [`dwarf`](#dwarf) | mod |  |
| [`endian_slice`](#endian-slice) | mod | Working with byte slices that have an associated endianity. |
| [`reader`](#reader) | mod |  |
| [`relocate`](#relocate) | mod |  |
| [`abbrev`](#abbrev) | mod | Functions for parsing DWARF debugging abbreviations. |
| [`aranges`](#aranges) | mod |  |
| [`index`](#index) | mod |  |
| [`line`](#line) | mod |  |
| [`lists`](#lists) | mod |  |
| [`loclists`](#loclists) | mod |  |
| [`lookup`](#lookup) | mod |  |
| [`macros`](#macros) | mod |  |
| [`op`](#op) | mod | Functions for parsing and evaluating DWARF expressions. |
| [`pubnames`](#pubnames) | mod |  |
| [`pubtypes`](#pubtypes) | mod |  |
| [`rnglists`](#rnglists) | mod |  |
| [`str`](#str) | mod |  |
| [`unit`](#unit) | mod | Functions for parsing DWARF `.debug_info` and `.debug_types` sections. |
| [`value`](#value) | mod | Definitions for values used in DWARF expressions. |
| [`sealed`](#sealed) | mod |  |
| [`UnitOffset`](#unitoffset) | struct | An offset into the current compilation or type unit. |
| [`StoreOnHeap`](#storeonheap) | struct | Indicates that storage should be allocated on heap. |
| [`ArrayVec`](#arrayvec) | struct |  |
| [`DebugAddr`](#debugaddr) | struct | The raw contents of the `.debug_addr` section. |
| [`AddrHeaderIter`](#addrheaderiter) | struct | An iterator over the headers of a `.debug_addr` section. |
| [`AddrHeader`](#addrheader) | struct | A header for a set of entries in the `.debug_addr` section. |
| [`AddrEntryIter`](#addrentryiter) | struct | An iterator over the addresses from a `.debug_addr` section. |
| [`DebugFrame`](#debugframe) | struct | `DebugFrame` contains the `.debug_frame` section's frame unwinding information required to unwind to and recover registers from older frames on the stack. |
| [`EhFrameHdr`](#ehframehdr) | struct | `EhFrameHdr` contains the information about the `.eh_frame_hdr` section. |
| [`ParsedEhFrameHdr`](#parsedehframehdr) | struct | `ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section. |
| [`EhHdrTableIter`](#ehhdrtableiter) | struct | An iterator for `.eh_frame_hdr` section's binary search table. |
| [`EhHdrTable`](#ehhdrtable) | struct | The CFI binary search table that is an optional part of the `.eh_frame_hdr` section. |
| [`EhFrame`](#ehframe) | struct | `EhFrame` contains the frame unwinding information needed during exception handling found in the `.eh_frame` section. |
| [`BaseAddresses`](#baseaddresses) | struct | Optional base addresses for the relative `DW_EH_PE_*` encoded pointers. |
| [`SectionBaseAddresses`](#sectionbaseaddresses) | struct | Optional base addresses for the relative `DW_EH_PE_*` encoded pointers in a particular section. |
| [`CfiEntriesIter`](#cfientriesiter) | struct | An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame` section. |
| [`Augmentation`](#augmentation) | struct | We support the z-style augmentation [defined by `.eh_frame`][ehframe]. |
| [`AugmentationData`](#augmentationdata) | struct | Parsed augmentation data for a `FrameDescriptEntry`. |
| [`CommonInformationEntry`](#commoninformationentry) | struct | > A Common Information Entry holds information that is shared among many > Frame Description Entries. |
| [`PartialFrameDescriptionEntry`](#partialframedescriptionentry) | struct | A partially parsed `FrameDescriptionEntry`. |
| [`FrameDescriptionEntry`](#framedescriptionentry) | struct | A `FrameDescriptionEntry` is a set of CFA instructions for an address range. |
| [`UnwindContext`](#unwindcontext) | struct | Common context needed when evaluating the call frame unwinding information. |
| [`UnwindTable`](#unwindtable) | struct | The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s `CallFrameInstruction` program, yielding the each row one at a time. |
| [`RegisterRuleMap`](#registerrulemap) | struct |  |
| [`RegisterRuleIter`](#registerruleiter) | struct | An unordered iterator for register rules. |
| [`UnwindTableRow`](#unwindtablerow) | struct | A row in the virtual unwind table that describes how to find the values of the registers in the *previous* frame for a range of PC addresses. |
| [`CallFrameInstructionIter`](#callframeinstructioniter) | struct | A lazy iterator parsing call frame instructions. |
| [`UnwindExpression`](#unwindexpression) | struct | The location of a DWARF expression within an unwind section. |
| [`PointerEncodingParameters`](#pointerencodingparameters) | struct |  |
| [`DwarfSections`](#dwarfsections) | struct | All of the commonly used DWARF sections. |
| [`Dwarf`](#dwarf) | struct | All of the commonly used DWARF sections, and other common information. |
| [`DwarfPackageSections`](#dwarfpackagesections) | struct | The sections from a `.dwp` file. |
| [`DwarfPackage`](#dwarfpackage) | struct | The sections from a `.dwp` file, with parsed indices. |
| [`Unit`](#unit) | struct | All of the commonly used information for a unit in the `.debug_info` or `.debug_types` sections. |
| [`UnitRef`](#unitref) | struct | A reference to a `Unit` and its associated `Dwarf`. |
| [`RangeIter`](#rangeiter) | struct | An iterator for the address ranges of a `DebuggingInformationEntry`. |
| [`EndianSlice`](#endianslice) | struct | A `&[u8]` slice with endianity metadata. |
| [`DebugBytes`](#debugbytes) | struct |  |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |
| [`ReaderOffsetId`](#readeroffsetid) | struct | An identifier for an offset within a section reader. |
| [`RelocateReader`](#relocatereader) | struct | A `Reader` which applies relocations to addresses and offsets. |
| [`DebugAbbrev`](#debugabbrev) | struct | The `DebugAbbrev` struct represents the abbreviations describing `DebuggingInformationEntry`s' attribute names and forms found in the `.debug_abbrev` section. |
| [`AbbreviationsCache`](#abbreviationscache) | struct | A cache of previously parsed `Abbreviations`. |
| [`Abbreviations`](#abbreviations) | struct | A set of type abbreviations. |
| [`Abbreviation`](#abbreviation) | struct | An abbreviation describes the shape of a `DebuggingInformationEntry`'s type: its code, tag type, whether it has children, and its set of attributes. |
| [`AttributeSpecification`](#attributespecification) | struct | The description of an attribute in an abbreviated type. |
| [`DebugAranges`](#debugaranges) | struct | The `DebugAranges` struct represents the DWARF address range information found in the `.debug_aranges` section. |
| [`ArangeHeaderIter`](#arangeheaderiter) | struct | An iterator over the headers of a `.debug_aranges` section. |
| [`ArangeHeader`](#arangeheader) | struct | A header for a set of entries in the `.debug_arange` section. |
| [`ArangeEntryIter`](#arangeentryiter) | struct | An iterator over the aranges from a `.debug_aranges` section. |
| [`ArangeEntry`](#arangeentry) | struct | A single parsed arange. |
| [`DebugCuIndex`](#debugcuindex) | struct | The data in the `.debug_cu_index` section of a `.dwp` file. |
| [`DebugTuIndex`](#debugtuindex) | struct | The data in the `.debug_tu_index` section of a `.dwp` file. |
| [`UnitIndex`](#unitindex) | struct | The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`. |
| [`UnitIndexSectionIterator`](#unitindexsectioniterator) | struct | An iterator over the section offsets and sizes for a row in a `UnitIndex`. |
| [`UnitIndexSection`](#unitindexsection) | struct | Information about a unit's contribution to a section in a `.dwp` file. |
| [`DebugLine`](#debugline) | struct | The `DebugLine` struct contains the source location to instruction mapping found in the `.debug_line` section. |
| [`LineRows`](#linerows) | struct | Executes a `LineProgram` to iterate over the rows in the matrix of line number information. |
| [`LineInstructions`](#lineinstructions) | struct | An iterator yielding parsed instructions. |
| [`LineRow`](#linerow) | struct | A row in the line number program's resulting matrix. |
| [`LineSequence`](#linesequence) | struct | A sequence within a line number program. |
| [`LineProgramHeader`](#lineprogramheader) | struct | A header for a line number program in the `.debug_line` section, as defined in section 6.2.4 of the standard. |
| [`IncompleteLineProgram`](#incompletelineprogram) | struct | A line number program that has not been run to completion. |
| [`CompleteLineProgram`](#completelineprogram) | struct | A line number program that has previously been run to completion. |
| [`FileEntry`](#fileentry) | struct | An entry in the `LineProgramHeader`'s `file_names` set. |
| [`FileEntryFormat`](#fileentryformat) | struct | The format of a component of an include directory or file name entry. |
| [`DebugLoc`](#debugloc) | struct | The raw contents of the `.debug_loc` section. |
| [`DebugLocLists`](#debugloclists) | struct | The `DebugLocLists` struct represents the DWARF data found in the `.debug_loclists` section. |
| [`LocationLists`](#locationlists) | struct | The DWARF data found in `.debug_loc` and `.debug_loclists` sections. |
| [`RawLocListIter`](#rawloclistiter) | struct | A raw iterator over a location list. |
| [`LocListIter`](#loclistiter) | struct | An iterator over a location list. |
| [`LocationListEntry`](#locationlistentry) | struct | A location list entry from the `.debug_loc` or `.debug_loclists` sections. |
| [`DebugMacinfo`](#debugmacinfo) | struct | The raw contents of the `.debug_macinfo` section. |
| [`DebugMacro`](#debugmacro) | struct | The raw contents of the `.debug_macro` section. |
| [`MacroUnitHeader`](#macrounitheader) | struct |  |
| [`MacroIter`](#macroiter) | struct | Iterator over the entries in the `.debug_macro` section. |
| [`Piece`](#piece) | struct | The description of a single piece of the result of a DWARF expression. |
| [`Expression`](#expression) | struct | The bytecode for a DWARF expression or location description. |
| [`OperationIter`](#operationiter) | struct | An iterator for the operations in an expression. |
| [`Evaluation`](#evaluation) | struct | A DWARF expression evaluator. |
| [`PubNamesEntry`](#pubnamesentry) | struct | A single parsed pubname. |
| [`DebugPubNames`](#debugpubnames) | struct | The `DebugPubNames` struct represents the DWARF public names information found in the `.debug_pubnames` section. |
| [`PubNamesEntryIter`](#pubnamesentryiter) | struct | An iterator over the pubnames from a `.debug_pubnames` section. |
| [`PubTypesEntry`](#pubtypesentry) | struct | A single parsed pubtype. |
| [`DebugPubTypes`](#debugpubtypes) | struct | The `DebugPubTypes` struct represents the DWARF public types information found in the `.debug_info` section. |
| [`PubTypesEntryIter`](#pubtypesentryiter) | struct | An iterator over the pubtypes from a `.debug_pubtypes` section. |
| [`DebugRanges`](#debugranges) | struct | The raw contents of the `.debug_ranges` section. |
| [`DebugRngLists`](#debugrnglists) | struct | The `DebugRngLists` struct represents the contents of the `.debug_rnglists` section. |
| [`RangeLists`](#rangelists) | struct | The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections. |
| [`RawRngListIter`](#rawrnglistiter) | struct | A raw iterator over an address range list. |
| [`RngListIter`](#rnglistiter) | struct | An iterator over an address range list. |
| [`RawRange`](#rawrange) | struct | A raw address range from the `.debug_ranges` section. |
| [`Range`](#range) | struct | An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections. |
| [`DebugStr`](#debugstr) | struct | The `DebugStr` struct represents the DWARF strings found in the `.debug_str` section. |
| [`DebugStrOffsets`](#debugstroffsets) | struct | The raw contents of the `.debug_str_offsets` section. |
| [`DebugLineStr`](#debuglinestr) | struct | The `DebugLineStr` struct represents the DWARF strings found in the `.debug_line_str` section. |
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
| [`Error`](#error) | enum | An error that occurred when parsing. |
| [`CieOrFde`](#cieorfde) | enum | Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE). |
| [`CfaRule`](#cfarule) | enum | The canonical frame address (CFA) recovery rules. |
| [`RegisterRule`](#registerrule) | enum | An entry in the abstract CFI table that describes how to find the value of a register. |
| [`CallFrameInstruction`](#callframeinstruction) | enum | A parsed call frame instruction. |
| [`Pointer`](#pointer) | enum | A decoded pointer. |
| [`RangeIterInner`](#rangeiterinner) | enum |  |
| [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy) | enum | The strategy to use for caching abbreviations. |
| [`Attributes`](#attributes) | enum | A list of attributes found in an `Abbreviation` |
| [`IndexSectionId`](#indexsectionid) | enum | Section kinds which are permitted in a `.dwp` index. |
| [`LineInstruction`](#lineinstruction) | enum | A parsed line number program instruction. |
| [`ColumnType`](#columntype) | enum | The type of column that a row is referring to. |
| [`LocListsFormat`](#loclistsformat) | enum |  |
| [`RawLocListEntry`](#rawloclistentry) | enum | A raw entry in .debug_loclists. |
| [`MacroString`](#macrostring) | enum | A string in a macro entry. |
| [`MacroEntry`](#macroentry) | enum | an Entry in the `.debug_macro` section. |
| [`DieReference`](#diereference) | enum | A reference to a DIE, either relative to the current CU or relative to the section. |
| [`Operation`](#operation) | enum | A single decoded DWARF expression operation. |
| [`OperationEvaluationResult`](#operationevaluationresult) | enum |  |
| [`Location`](#location) | enum | A single location of a piece of the result of a DWARF expression. |
| [`EvaluationState`](#evaluationstate) | enum |  |
| [`EvaluationWaiting`](#evaluationwaiting) | enum |  |
| [`EvaluationResult`](#evaluationresult) | enum | The state of an `Evaluation` after evaluating a DWARF expression. |
| [`RangeListsFormat`](#rangelistsformat) | enum |  |
| [`RawRngListEntry`](#rawrnglistentry) | enum | A raw entry in .debug_rnglists |
| [`UnitType`](#unittype) | enum | This enum specifies the type of the unit and any type specific data carried in the header (e.g. the type signature/type offset of a type unit). |
| [`AttributeValue`](#attributevalue) | enum | The value of an attribute in a `DebuggingInformationEntry`. |
| [`ValueType`](#valuetype) | enum | The type of an entry on the DWARF stack. |
| [`Value`](#value) | enum | The value of an entry on the DWARF stack. |
| [`Section`](#section) | trait | A convenience trait for loading DWARF sections from object files. |
| [`ArrayLike`](#arraylike) | trait | Marker trait for types that can be used as backing storage when a growable array type is needed. |
| [`UnwindOffset`](#unwindoffset) | trait | An offset into an `UnwindSection`. |
| [`UnwindSection`](#unwindsection) | trait | A section holding unwind information: either `.debug_frame` or `.eh_frame`. |
| [`UnwindContextStorage`](#unwindcontextstorage) | trait | Specification of what storage should be used for [`UnwindContext`]. |
| [`ReaderOffset`](#readeroffset) | trait | A trait for offsets with a DWARF section. |
| [`ReaderAddress`](#readeraddress) | trait | A trait for addresses within a DWARF section. |
| [`Reader`](#reader) | trait | A trait for reading the data from a DWARF section. |
| [`Relocate`](#relocate) | trait | Trait for relocating addresses and offsets while reading a section. |
| [`LineProgram`](#lineprogram) | trait | A `LineProgram` provides access to a `LineProgramHeader` and a way to add files to the files table if necessary. |
| [`EvaluationStorage`](#evaluationstorage) | trait | Specification of what storage should be used for [`Evaluation`]. |
| [`parse_cfi_entry`](#parse-cfi-entry) | fn |  |
| [`parse_encoded_pointer`](#parse-encoded-pointer) | fn |  |
| [`parse_encoded_value`](#parse-encoded-value) | fn |  |
| [`get_attribute_size`](#get-attribute-size) | fn |  |
| [`parse_directory_v5`](#parse-directory-v5) | fn |  |
| [`parse_file_v5`](#parse-file-v5) | fn |  |
| [`parse_attribute`](#parse-attribute) | fn |  |
| [`parse_data`](#parse-data) | fn |  |
| [`compute_pc`](#compute-pc) | fn |  |
| [`generic_type`](#generic-type) | fn |  |
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
| [`sign_extend`](#sign-extend) | fn | Convert a u64 to an i64, with sign extension if required. |
| [`mask_bit_size`](#mask-bit-size) | fn |  |
| [`EndianBuf`](#endianbuf) | type | `EndianBuf` has been renamed to `EndianSlice`. |
| [`Result`](#result) | type | The result of a parse. |
| [`LineNumberProgram`](#linenumberprogram) | type | Deprecated. |
| [`StateMachine`](#statemachine) | type | Deprecated. |
| [`OneShotLineRows`](#oneshotlinerows) | type |  |
| [`ResumedLineRows`](#resumedlinerows) | type |  |
| [`Opcode`](#opcode) | type | Deprecated. |
| [`OpcodesIter`](#opcodesiter) | type | Deprecated. |
| [`LineNumberRow`](#linenumberrow) | type | Deprecated. |
| [`LineNumberSequence`](#linenumbersequence) | type | Deprecated. |
| [`LineNumberProgramHeader`](#linenumberprogramheader) | type | Deprecated. |
| [`IncompleteLineNumberProgram`](#incompletelinenumberprogram) | type | Deprecated. |
| [`CompleteLineNumberProgram`](#completelinenumberprogram) | type | Deprecated. |
| [`LocListsHeader`](#loclistsheader) | type |  |
| [`RngListsHeader`](#rnglistsheader) | type |  |
| [`MAX_RULES`](#max-rules) | const |  |
| [`MAX_UNWIND_STACK_DEPTH`](#max-unwind-stack-depth) | const |  |
| [`CFI_INSTRUCTION_HIGH_BITS_MASK`](#cfi-instruction-high-bits-mask) | const |  |
| [`CFI_INSTRUCTION_LOW_BITS_MASK`](#cfi-instruction-low-bits-mask) | const |  |
| [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline) | const |  |
| [`SECTION_COUNT_MAX`](#section-count-max) | const |  |

## Modules

- [`util`](util/index.md)
- [`addr`](addr/index.md)
- [`cfi`](cfi/index.md)
- [`dwarf`](dwarf/index.md)
- [`endian_slice`](endian_slice/index.md) — Working with byte slices that have an associated endianity.
- [`reader`](reader/index.md)
- [`relocate`](relocate/index.md)
- [`abbrev`](abbrev/index.md) — Functions for parsing DWARF debugging abbreviations.
- [`aranges`](aranges/index.md)
- [`index`](index/index.md)
- [`line`](line/index.md)
- [`lists`](lists/index.md)
- [`loclists`](loclists/index.md)
- [`lookup`](lookup/index.md)
- [`macros`](macros/index.md)
- [`op`](op/index.md) — Functions for parsing and evaluating DWARF expressions.
- [`pubnames`](pubnames/index.md)
- [`pubtypes`](pubtypes/index.md)
- [`rnglists`](rnglists/index.md)
- [`str`](str/index.md)
- [`unit`](unit/index.md) — Functions for parsing DWARF `.debug_info` and `.debug_types` sections.
- [`value`](value/index.md) — Definitions for values used in DWARF expressions.
- [`sealed`](sealed/index.md)

## Structs

### `UnitOffset<T>`

```rust
struct UnitOffset<T>(T);
```

*Defined in [`gimli-0.32.3/src/read/mod.rs:264`](../../../.source_1765633015/gimli-0.32.3/src/read/mod.rs#L264)*

An offset into the current compilation or type unit.

#### Implementations

- <span id="cratereadunitoffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &Unit<R>) -> UnitSectionOffset<T>` — [`Unit`](#unit), [`UnitSectionOffset`](../index.md#unitsectionoffset)

  Convert an offset to be relative to the start of the .debug_info section,

  instead of relative to the start of the given compilation unit.

  

  Does not check that the offset is valid.

#### Trait Implementations

##### `impl<T> Any for UnitOffset<T>`

- <span id="unitoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitOffset<T>`

- <span id="unitoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitOffset<T>`

- <span id="unitoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for UnitOffset<T>`

- <span id="unitoffset-clone"></span>`fn clone(&self) -> UnitOffset<T>` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T> CloneToUninit for UnitOffset<T>`

- <span id="unitoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for UnitOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitOffset<T>`

- <span id="unitoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitOffset<T>`

##### `impl<T> From for UnitOffset<T>`

- <span id="unitoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for UnitOffset<T>`

- <span id="unitoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for UnitOffset<T>`

- <span id="unitoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::Ord> Ord for UnitOffset<T>`

- <span id="unitoffset-ord-cmp"></span>`fn cmp(&self, other: &UnitOffset<T>) -> cmp::Ordering` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitOffset<T>`

- <span id="unitoffset-partialeq-eq"></span>`fn eq(&self, other: &UnitOffset<T>) -> bool` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitOffset<T>`

- <span id="unitoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T> StructuralPartialEq for UnitOffset<T>`

##### `impl<T> ToOwned for UnitOffset<T>`

- <span id="unitoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="unitoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for UnitOffset<T>`

- <span id="unitoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnitOffset<T>`

- <span id="unitoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StoreOnHeap`

```rust
struct StoreOnHeap;
```

*Defined in [`gimli-0.32.3/src/read/mod.rs:276`](../../../.source_1765633015/gimli-0.32.3/src/read/mod.rs#L276)*

Indicates that storage should be allocated on heap.

#### Trait Implementations

##### `impl Any for StoreOnHeap`

- <span id="storeonheap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StoreOnHeap`

- <span id="storeonheap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StoreOnHeap`

- <span id="storeonheap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StoreOnHeap`

- <span id="storeonheap-clone"></span>`fn clone(&self) -> StoreOnHeap` — [`StoreOnHeap`](../index.md#storeonheap)

##### `impl CloneToUninit for StoreOnHeap`

- <span id="storeonheap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StoreOnHeap`

##### `impl Debug for StoreOnHeap`

- <span id="storeonheap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StoreOnHeap`

##### `impl<R: Reader> EvaluationStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-evaluationstorage-type-stack"></span>`type Stack = Vec<Value>`

- <span id="cratereadstoreonheap-evaluationstorage-type-expressionstack"></span>`type ExpressionStack = Vec<(R, R)>`

- <span id="cratereadstoreonheap-evaluationstorage-type-result"></span>`type Result = Vec<Piece<R>>`

##### `impl<T> From for StoreOnHeap`

- <span id="storeonheap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StoreOnHeap`

- <span id="storeonheap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StoreOnHeap`

- <span id="storeonheap-partialeq-eq"></span>`fn eq(&self, other: &StoreOnHeap) -> bool` — [`StoreOnHeap`](../index.md#storeonheap)

##### `impl StructuralPartialEq for StoreOnHeap`

##### `impl ToOwned for StoreOnHeap`

- <span id="storeonheap-toowned-type-owned"></span>`type Owned = T`

- <span id="storeonheap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="storeonheap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StoreOnHeap`

- <span id="storeonheap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="storeonheap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StoreOnHeap`

- <span id="storeonheap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="storeonheap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: ReaderOffset> UnwindContextStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-unwindcontextstorage-type-rules"></span>`type Rules = [(Register, RegisterRule<T>); 192]`

- <span id="cratereadstoreonheap-unwindcontextstorage-type-stack"></span>`type Stack = Box<[UnwindTableRow<T>; 4]>`

### `ArrayVec<A: ArrayLike>`

```rust
struct ArrayVec<A: ArrayLike> {
    storage: <A as >::Storage,
    len: usize,
}
```

*Defined in [`gimli-0.32.3/src/read/util.rs:121-124`](../../../.source_1765633015/gimli-0.32.3/src/read/util.rs#L121-L124)*

#### Implementations

- <span id="arrayvec-new"></span>`fn new() -> Self`

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, value: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](#arraylike), [`CapacityFull`](util/sealed/index.md#capacityfull)

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, element: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](#arraylike), [`CapacityFull`](util/sealed/index.md#capacityfull)

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`ArrayLike`](#arraylike)

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`ArrayLike`](#arraylike)

#### Trait Implementations

##### `impl Any for ArrayVec<A>`

- <span id="arrayvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVec<A>`

- <span id="arrayvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVec<A>`

- <span id="arrayvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ArrayVec<A>`

- <span id="arrayvec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- <span id="arrayvec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- <span id="arrayvec-deref-type-target"></span>`type Target = [<A as ArrayLike>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &[<A as >::Item]` — [`ArrayLike`](#arraylike)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- <span id="arrayvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [<A as >::Item]` — [`ArrayLike`](#arraylike)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- <span id="arrayvec-drop"></span>`fn drop(&mut self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<T> From for ArrayVec<A>`

- <span id="arrayvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArrayVec<A>`

- <span id="arrayvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- <span id="arrayvec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Receiver for ArrayVec<A>`

- <span id="arrayvec-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for ArrayVec<A>`

- <span id="arrayvec-toowned-type-owned"></span>`type Owned = T`

- <span id="arrayvec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arrayvec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArrayVec<A>`

- <span id="arrayvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVec<A>`

- <span id="arrayvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:6-8`](../../../.source_1765633015/gimli-0.32.3/src/read/addr.rs#L6-L8)*

The raw contents of the `.debug_addr` section.

#### Implementations

- <span id="debugaddr-get-address"></span>`fn get_address(&self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../index.md#debugaddrbase), [`Reader`](#reader), [`DebugAddrIndex`](../index.md#debugaddrindex), [`Result`](../index.md#result)

  Returns the address at the given `base` and `index`.

  

  A set of addresses in the `.debug_addr` section consists of a header

  followed by a series of addresses.

  

  The `base` must be the `DW_AT_addr_base` value from the compilation unit DIE.

  This is an offset that points to the first address following the header.

  

  The `index` is the value of a `DW_FORM_addrx` attribute.

  

  The `address_size` must be the size of the address for the compilation unit.

  This value must also match the header. However, note that we do not parse the

  header to validate this, since locating the header is unreliable, and the GNU

  extensions do not emit it.

- <span id="debugaddr-headers"></span>`fn headers(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](#addrheaderiter)

  Iterate the sets of entries in the `.debug_addr` section.

  

  Each set of entries belongs to a single unit.

#### Trait Implementations

##### `impl Any for DebugAddr<R>`

- <span id="debugaddr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAddr<R>`

- <span id="debugaddr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAddr<R>`

- <span id="debugaddr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugAddr<R>`

- <span id="debugaddr-clone"></span>`fn clone(&self) -> DebugAddr<R>` — [`DebugAddr`](#debugaddr)

##### `impl CloneToUninit for DebugAddr<R>`

- <span id="debugaddr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: fmt::Debug> Debug for DebugAddr<R>`

- <span id="debugaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAddr<R>`

- <span id="debugaddr-default"></span>`fn default() -> DebugAddr<R>` — [`DebugAddr`](#debugaddr)

##### `impl<T> From for DebugAddr<R>`

- <span id="debugaddr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugAddr<R>`

- <span id="debugaddr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugAddr<R>`

- <span id="debugaddr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugaddr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugAddr<R>`

- <span id="debugaddr-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaddr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaddr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugAddr<R>`

- <span id="debugaddr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaddr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugAddr<R>`

- <span id="debugaddr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaddr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:82-85`](../../../.source_1765633015/gimli-0.32.3/src/read/addr.rs#L82-L85)*

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- <span id="addrheaderiter-next"></span>`fn next(&mut self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../index.md#result), [`AddrHeader`](#addrheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl Any for AddrHeaderIter<R>`

- <span id="addrheaderiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddrHeaderIter<R>`

- <span id="addrheaderiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddrHeaderIter<R>`

- <span id="addrheaderiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- <span id="addrheaderiter-clone"></span>`fn clone(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](#addrheaderiter)

##### `impl CloneToUninit for AddrHeaderIter<R>`

- <span id="addrheaderiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- <span id="addrheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AddrHeaderIter<R>`

- <span id="addrheaderiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AddrHeaderIter<R>`

- <span id="addrheaderiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AddrHeaderIter<R>`

- <span id="addrheaderiter-toowned-type-owned"></span>`type Owned = T`

- <span id="addrheaderiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addrheaderiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddrHeaderIter<R>`

- <span id="addrheaderiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addrheaderiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddrHeaderIter<R>`

- <span id="addrheaderiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addrheaderiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddrHeader<R, Offset>`

```rust
struct AddrHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugAddrOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    entries: R,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:122-131`](../../../.source_1765633015/gimli-0.32.3/src/read/addr.rs#L122-L131)*

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- <span id="addrheader-parse"></span>`fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../index.md#debugaddroffset), [`Result`](../index.md#result)

- <span id="addrheader-offset"></span>`fn offset(&self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../index.md#debugaddroffset)

  Return the offset of this header within the `.debug_addr` section.

- <span id="addrheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="addrheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="addrheader-entries"></span>`fn entries(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](#addrentryiter)

  Return the address entries in this set.

#### Trait Implementations

##### `impl Any for AddrHeader<R, Offset>`

- <span id="addrheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddrHeader<R, Offset>`

- <span id="addrheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddrHeader<R, Offset>`

- <span id="addrheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- <span id="addrheader-clone"></span>`fn clone(&self) -> AddrHeader<R, Offset>` — [`AddrHeader`](#addrheader)

##### `impl CloneToUninit for AddrHeader<R, Offset>`

- <span id="addrheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- <span id="addrheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<T> From for AddrHeader<R, Offset>`

- <span id="addrheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AddrHeader<R, Offset>`

- <span id="addrheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- <span id="addrheader-partialeq-eq"></span>`fn eq(&self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](#addrheader)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

##### `impl ToOwned for AddrHeader<R, Offset>`

- <span id="addrheader-toowned-type-owned"></span>`type Owned = T`

- <span id="addrheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addrheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddrHeader<R, Offset>`

- <span id="addrheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addrheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddrHeader<R, Offset>`

- <span id="addrheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addrheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/addr.rs:217-220`](../../../.source_1765633015/gimli-0.32.3/src/read/addr.rs#L217-L220)*

An iterator over the addresses from a `.debug_addr` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="addrentryiter-next"></span>`fn next(&mut self) -> Result<Option<u64>>` — [`Result`](../index.md#result)

  Advance the iterator and return the next address.

  

  Returns the newly parsed address as `Ok(Some(addr))`. Returns `Ok(None)`

  when iteration is complete and all addresses have already been parsed and

  yielded. If an error occurs while parsing the next address, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

#### Trait Implementations

##### `impl Any for AddrEntryIter<R>`

- <span id="addrentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddrEntryIter<R>`

- <span id="addrentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddrEntryIter<R>`

- <span id="addrentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for AddrEntryIter<R>`

- <span id="addrentryiter-clone"></span>`fn clone(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](#addrentryiter)

##### `impl CloneToUninit for AddrEntryIter<R>`

- <span id="addrentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- <span id="addrentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AddrEntryIter<R>`

- <span id="addrentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AddrEntryIter<R>`

- <span id="addrentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AddrEntryIter<R>`

- <span id="addrentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="addrentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addrentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddrEntryIter<R>`

- <span id="addrentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addrentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddrEntryIter<R>`

- <span id="addrentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addrentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugFrame<R: Reader>`

```rust
struct DebugFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:36-40`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L36-L40)*

`DebugFrame` contains the `.debug_frame` section's frame unwinding
information required to unwind to and recover registers from older frames on
the stack. For example, this is useful for a debugger that wants to print
locals in a backtrace.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

### Differences between `.debug_frame` and `.eh_frame`

While the `.debug_frame` section's information has a lot of overlap with the
`.eh_frame` section's information, the `.eh_frame` information tends to only
encode the subset of information needed for exception handling. Often, only
one of `.eh_frame` or `.debug_frame` will be present in an object file.

#### Implementations

- <span id="debugframe-set-address-size"></span>`fn set_address_size(&mut self, address_size: u8)`

  Set the size of a target address in bytes.

  

  This defaults to the native word size.

  This is only used if the CIE version is less than 4.

- <span id="debugframe-set-vendor"></span>`fn set_vendor(&mut self, vendor: Vendor)` — [`Vendor`](../index.md#vendor)

  Set the vendor extensions to use.

  

  This defaults to `Vendor::Default`.

#### Trait Implementations

##### `impl Any for DebugFrame<R>`

- <span id="debugframe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugFrame<R>`

- <span id="debugframe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugFrame<R>`

- <span id="debugframe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugFrame<R>`

- <span id="debugframe-clone"></span>`fn clone(&self) -> DebugFrame<R>` — [`DebugFrame`](#debugframe)

##### `impl CloneToUninit for DebugFrame<R>`

- <span id="debugframe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for DebugFrame<R>`

##### `impl<R: fmt::Debug + Reader> Debug for DebugFrame<R>`

- <span id="debugframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for DebugFrame<R>`

##### `impl<T> From for DebugFrame<R>`

- <span id="debugframe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugFrame<R>`

- <span id="debugframe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for DebugFrame<R>`

- <span id="debugframe-partialeq-eq"></span>`fn eq(&self, other: &DebugFrame<R>) -> bool` — [`DebugFrame`](#debugframe)

##### `impl<R: Reader> Section for DebugFrame<R>`

- <span id="debugframe-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugframe-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for DebugFrame<R>`

##### `impl ToOwned for DebugFrame<R>`

- <span id="debugframe-toowned-type-owned"></span>`type Owned = T`

- <span id="debugframe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugframe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugFrame<R>`

- <span id="debugframe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugframe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugFrame<R>`

- <span id="debugframe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugframe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R: Reader> UnwindSection for DebugFrame<R>`

- <span id="debugframe-unwindsection-type-offset"></span>`type Offset = DebugFrameOffset<<R as Reader>::Offset>`

### `EhFrameHdr<R: Reader>`

```rust
struct EhFrameHdr<R: Reader>(R);
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:109`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L109)*

`EhFrameHdr` contains the information about the `.eh_frame_hdr` section.

A pointer to the start of the `.eh_frame` data, and optionally, a binary
search table of pointers to the `.eh_frame` records that are found in this section.

#### Implementations

- <span id="ehframehdr-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Constructs a new `EhFrameHdr` instance from the data in the `.eh_frame_hdr` section.

#### Trait Implementations

##### `impl Any for EhFrameHdr<R>`

- <span id="ehframehdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EhFrameHdr<R>`

- <span id="ehframehdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EhFrameHdr<R>`

- <span id="ehframehdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for EhFrameHdr<R>`

- <span id="ehframehdr-clone"></span>`fn clone(&self) -> EhFrameHdr<R>` — [`EhFrameHdr`](#ehframehdr)

##### `impl CloneToUninit for EhFrameHdr<R>`

- <span id="ehframehdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for EhFrameHdr<R>`

##### `impl<R: fmt::Debug + Reader> Debug for EhFrameHdr<R>`

- <span id="ehframehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for EhFrameHdr<R>`

##### `impl<T> From for EhFrameHdr<R>`

- <span id="ehframehdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EhFrameHdr<R>`

- <span id="ehframehdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EhFrameHdr<R>`

- <span id="ehframehdr-partialeq-eq"></span>`fn eq(&self, other: &EhFrameHdr<R>) -> bool` — [`EhFrameHdr`](#ehframehdr)

##### `impl<R: Reader> Section for EhFrameHdr<R>`

- <span id="ehframehdr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="ehframehdr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrameHdr<R>`

##### `impl ToOwned for EhFrameHdr<R>`

- <span id="ehframehdr-toowned-type-owned"></span>`type Owned = T`

- <span id="ehframehdr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ehframehdr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EhFrameHdr<R>`

- <span id="ehframehdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ehframehdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EhFrameHdr<R>`

- <span id="ehframehdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ehframehdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParsedEhFrameHdr<R: Reader>`

```rust
struct ParsedEhFrameHdr<R: Reader> {
    address_size: u8,
    section: R,
    eh_frame_ptr: Pointer,
    fde_count: u64,
    table_enc: crate::constants::DwEhPe,
    table: R,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:113-121`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L113-L121)*

`ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section.

#### Implementations

- <span id="parsedehframehdr-eh-frame-ptr"></span>`fn eh_frame_ptr(&self) -> Pointer` — [`Pointer`](#pointer)

  Returns the address of the binary's `.eh_frame` section.

- <span id="parsedehframehdr-table"></span>`fn table(&self) -> Option<EhHdrTable<'_, R>>` — [`EhHdrTable`](#ehhdrtable)

  Retrieves the CFI binary search table, if there is one.

#### Trait Implementations

##### `impl Any for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-clone"></span>`fn clone(&self) -> ParsedEhFrameHdr<R>` — [`ParsedEhFrameHdr`](#parsedehframehdr)

##### `impl CloneToUninit for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-toowned-type-owned"></span>`type Owned = T`

- <span id="parsedehframehdr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parsedehframehdr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsedehframehdr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsedehframehdr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EhHdrTableIter<'a, 'bases, R: Reader>`

```rust
struct EhHdrTableIter<'a, 'bases, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
    table: R,
    bases: &'bases BaseAddresses,
    remain: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:229-234`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L229-L234)*

An iterator for `.eh_frame_hdr` section's binary search table.

Each table entry consists of a tuple containing an  `initial_location` and `address`.
The `initial location` represents the first address that the targeted FDE
is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.
The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

#### Implementations

- <span id="ehhdrtableiter-next"></span>`fn next(&mut self) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../index.md#result), [`Pointer`](#pointer)

  Yield the next entry in the `EhHdrTableIter`.

- <span id="ehhdrtableiter-nth"></span>`fn nth(&mut self, n: usize) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../index.md#result), [`Pointer`](#pointer)

  Yield the nth entry in the `EhHdrTableIter`

#### Trait Implementations

##### `impl Any for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ehhdrtableiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ehhdrtableiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EhHdrTable<'a, R: Reader>`

```rust
struct EhHdrTable<'a, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:299-301`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L299-L301)*

The CFI binary search table that is an optional part of the `.eh_frame_hdr` section.

#### Implementations

- <span id="ehhdrtable-iter"></span>`fn iter<'bases>(&self, bases: &'bases BaseAddresses) -> EhHdrTableIter<'_, 'bases, R>` — [`BaseAddresses`](#baseaddresses), [`EhHdrTableIter`](#ehhdrtableiter)

  Return an iterator that can walk the `.eh_frame_hdr` table.

  

  Each table entry consists of a tuple containing an `initial_location` and `address`.

  The `initial location` represents the first address that the targeted FDE

  is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.

  The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

- <span id="ehhdrtable-lookup"></span>`fn lookup(&self, address: u64, bases: &BaseAddresses) -> Result<Pointer>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`Pointer`](#pointer)

  *Probably* returns a pointer to the FDE for the given address.

  

  This performs a binary search, so if there is no FDE for the given address,

  this function **will** return a pointer to any other FDE that's close by.

  

  To be sure, you **must** call `contains` on the FDE.

- <span id="ehhdrtable-pointer-to-offset"></span>`fn pointer_to_offset(&self, ptr: Pointer) -> Result<EhFrameOffset<<R as >::Offset>>` — [`Pointer`](#pointer), [`Result`](../index.md#result), [`EhFrameOffset`](../index.md#ehframeoffset), [`Reader`](#reader)

  Convert a `Pointer` to a section offset.

  

  This does not support indirect pointers.

- <span id="ehhdrtable-fde-for-address"></span>`fn fde_for_address<F>(&self, frame: &EhFrame<R>, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`EhFrame`](#ehframe), [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`FrameDescriptionEntry`](#framedescriptionentry)

  Returns a parsed FDE for the given address, or `NoUnwindInfoForAddress`

  if there are none.

  

  You must provide a function to get its associated CIE. See

  `PartialFrameDescriptionEntry::parse` for more information.

  

  # Example

  

  ```rust

  use gimli::{BaseAddresses, EhFrame, ParsedEhFrameHdr, EndianSlice, NativeEndian, Error, UnwindSection};

  fn foo() -> Result<(), Error> {

  let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();

  let eh_frame_hdr: ParsedEhFrameHdr<EndianSlice<NativeEndian>> = unimplemented!();

  let addr = 0;

  let bases = unimplemented!();

  let table = eh_frame_hdr.table().unwrap();

  let fde = table.fde_for_address(&eh_frame, &bases, addr, EhFrame::cie_from_offset)?;

  Ok(())

  }

  ```

- <span id="ehhdrtable-unwind-info-for-address"></span>`fn unwind_info_for_address<'ctx, F, S>(&self, frame: &EhFrame<R>, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`EhFrame`](#ehframe), [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md#result), [`UnwindTableRow`](#unwindtablerow)

  Returns the frame unwind information for the given address,

  or `NoUnwindInfoForAddress` if there are none.

  

  You must provide a function to get the associated CIE. See

  `PartialFrameDescriptionEntry::parse` for more information.

#### Trait Implementations

##### `impl Any for EhHdrTable<'a, R>`

- <span id="ehhdrtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EhHdrTable<'a, R>`

- <span id="ehhdrtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EhHdrTable<'a, R>`

- <span id="ehhdrtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for EhHdrTable<'a, R>`

- <span id="ehhdrtable-clone"></span>`fn clone(&self) -> EhHdrTable<'a, R>` — [`EhHdrTable`](#ehhdrtable)

##### `impl CloneToUninit for EhHdrTable<'a, R>`

- <span id="ehhdrtable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for EhHdrTable<'a, R>`

- <span id="ehhdrtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EhHdrTable<'a, R>`

- <span id="ehhdrtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EhHdrTable<'a, R>`

- <span id="ehhdrtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for EhHdrTable<'a, R>`

- <span id="ehhdrtable-toowned-type-owned"></span>`type Owned = T`

- <span id="ehhdrtable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ehhdrtable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EhHdrTable<'a, R>`

- <span id="ehhdrtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ehhdrtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EhHdrTable<'a, R>`

- <span id="ehhdrtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ehhdrtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EhFrame<R: Reader>`

```rust
struct EhFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:488-492`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L488-L492)*

`EhFrame` contains the frame unwinding information needed during exception
handling found in the `.eh_frame` section.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

See
[`DebugFrame`](./struct.DebugFrame.html#differences-between-debug_frame-and-eh_frame)
for some discussion on the differences between `.debug_frame` and
`.eh_frame`.

#### Implementations

- <span id="ehframe-set-address-size"></span>`fn set_address_size(&mut self, address_size: u8)`

  Set the size of a target address in bytes.

  

  This defaults to the native word size.

- <span id="ehframe-set-vendor"></span>`fn set_vendor(&mut self, vendor: Vendor)` — [`Vendor`](../index.md#vendor)

  Set the vendor extensions to use.

  

  This defaults to `Vendor::Default`.

#### Trait Implementations

##### `impl Any for EhFrame<R>`

- <span id="ehframe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EhFrame<R>`

- <span id="ehframe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EhFrame<R>`

- <span id="ehframe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for EhFrame<R>`

- <span id="ehframe-clone"></span>`fn clone(&self) -> EhFrame<R>` — [`EhFrame`](#ehframe)

##### `impl CloneToUninit for EhFrame<R>`

- <span id="ehframe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for EhFrame<R>`

##### `impl<R: fmt::Debug + Reader> Debug for EhFrame<R>`

- <span id="ehframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for EhFrame<R>`

##### `impl<T> From for EhFrame<R>`

- <span id="ehframe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EhFrame<R>`

- <span id="ehframe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EhFrame<R>`

- <span id="ehframe-partialeq-eq"></span>`fn eq(&self, other: &EhFrame<R>) -> bool` — [`EhFrame`](#ehframe)

##### `impl<R: Reader> Section for EhFrame<R>`

- <span id="ehframe-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="ehframe-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrame<R>`

##### `impl ToOwned for EhFrame<R>`

- <span id="ehframe-toowned-type-owned"></span>`type Owned = T`

- <span id="ehframe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ehframe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EhFrame<R>`

- <span id="ehframe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ehframe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EhFrame<R>`

- <span id="ehframe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ehframe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R: Reader> UnwindSection for EhFrame<R>`

- <span id="ehframe-unwindsection-type-offset"></span>`type Offset = EhFrameOffset<<R as Reader>::Offset>`

### `BaseAddresses`

```rust
struct BaseAddresses {
    pub eh_frame_hdr: SectionBaseAddresses,
    pub eh_frame: SectionBaseAddresses,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:895-901`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L895-L901)*

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers.

During CIE/FDE parsing, if a relative pointer is encountered for a base
address that is unknown, an Err will be returned.

```rust
use gimli::BaseAddresses;

fn foo() {
let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
let address_of_eh_frame_section_in_memory = unimplemented!();
let address_of_text_section_in_memory = unimplemented!();
let address_of_got_section_in_memory = unimplemented!();
let address_of_the_start_of_current_func = unimplemented!();
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);
let _ = bases;
}
```

#### Fields

- **`eh_frame_hdr`**: `SectionBaseAddresses`

  The base addresses to use for pointers in the `.eh_frame_hdr` section.

- **`eh_frame`**: `SectionBaseAddresses`

  The base addresses to use for pointers in the `.eh_frame` section.

#### Implementations

- <span id="baseaddresses-set-eh-frame-hdr"></span>`fn set_eh_frame_hdr(self, addr: u64) -> Self`

  Set the `.eh_frame_hdr` section base address.

- <span id="baseaddresses-set-eh-frame"></span>`fn set_eh_frame(self, addr: u64) -> Self`

  Set the `.eh_frame` section base address.

- <span id="baseaddresses-set-text"></span>`fn set_text(self, addr: u64) -> Self`

  Set the `.text` section base address.

- <span id="baseaddresses-set-got"></span>`fn set_got(self, addr: u64) -> Self`

  Set the `.got` section base address.

#### Trait Implementations

##### `impl Any for BaseAddresses`

- <span id="baseaddresses-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BaseAddresses`

- <span id="baseaddresses-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BaseAddresses`

- <span id="baseaddresses-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BaseAddresses`

- <span id="baseaddresses-clone"></span>`fn clone(&self) -> BaseAddresses` — [`BaseAddresses`](#baseaddresses)

##### `impl CloneToUninit for BaseAddresses`

- <span id="baseaddresses-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BaseAddresses`

- <span id="baseaddresses-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BaseAddresses`

- <span id="baseaddresses-default"></span>`fn default() -> BaseAddresses` — [`BaseAddresses`](#baseaddresses)

##### `impl Eq for BaseAddresses`

##### `impl<T> From for BaseAddresses`

- <span id="baseaddresses-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BaseAddresses`

- <span id="baseaddresses-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BaseAddresses`

- <span id="baseaddresses-partialeq-eq"></span>`fn eq(&self, other: &BaseAddresses) -> bool` — [`BaseAddresses`](#baseaddresses)

##### `impl StructuralPartialEq for BaseAddresses`

##### `impl ToOwned for BaseAddresses`

- <span id="baseaddresses-toowned-type-owned"></span>`type Owned = T`

- <span id="baseaddresses-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="baseaddresses-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BaseAddresses`

- <span id="baseaddresses-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="baseaddresses-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BaseAddresses`

- <span id="baseaddresses-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="baseaddresses-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionBaseAddresses`

```rust
struct SectionBaseAddresses {
    pub section: Option<u64>,
    pub text: Option<u64>,
    pub data: Option<u64>,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:908-924`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L908-L924)*

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers
in a particular section.

See `BaseAddresses` for methods that are helpful in setting these addresses.

#### Fields

- **`section`**: `Option<u64>`

  The address of the section containing the pointer.

- **`text`**: `Option<u64>`

  The base address for text relative pointers.
  This is generally the address of the `.text` section.

- **`data`**: `Option<u64>`

  The base address for data relative pointers.
  
  For pointers in the `.eh_frame_hdr` section, this is the address
  of the `.eh_frame_hdr` section
  
  For pointers in the `.eh_frame` section, this is generally the
  global pointer, such as the address of the `.got` section.

#### Trait Implementations

##### `impl Any for SectionBaseAddresses`

- <span id="sectionbaseaddresses-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionBaseAddresses`

- <span id="sectionbaseaddresses-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionBaseAddresses`

- <span id="sectionbaseaddresses-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionBaseAddresses`

- <span id="sectionbaseaddresses-clone"></span>`fn clone(&self) -> SectionBaseAddresses` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl CloneToUninit for SectionBaseAddresses`

- <span id="sectionbaseaddresses-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SectionBaseAddresses`

- <span id="sectionbaseaddresses-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionBaseAddresses`

- <span id="sectionbaseaddresses-default"></span>`fn default() -> SectionBaseAddresses` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl Eq for SectionBaseAddresses`

##### `impl<T> From for SectionBaseAddresses`

- <span id="sectionbaseaddresses-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SectionBaseAddresses`

- <span id="sectionbaseaddresses-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SectionBaseAddresses`

- <span id="sectionbaseaddresses-partialeq-eq"></span>`fn eq(&self, other: &SectionBaseAddresses) -> bool` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl StructuralPartialEq for SectionBaseAddresses`

##### `impl ToOwned for SectionBaseAddresses`

- <span id="sectionbaseaddresses-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionbaseaddresses-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionbaseaddresses-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionBaseAddresses`

- <span id="sectionbaseaddresses-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionbaseaddresses-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionBaseAddresses`

- <span id="sectionbaseaddresses-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionbaseaddresses-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CfiEntriesIter<'bases, Section, R>`

```rust
struct CfiEntriesIter<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    section: Section,
    bases: &'bases BaseAddresses,
    input: R,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:998-1006`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L998-L1006)*

An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame`
section.

Some pointers may be encoded relative to various base addresses. Use the
[`BaseAddresses`](./struct.BaseAddresses.html) parameter to provide them. By
default, none are provided. If a relative pointer is encountered for a base
address that is unknown, an `Err` will be returned and iteration will abort.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

```rust
use gimli::{BaseAddresses, EhFrame, EndianSlice, NativeEndian, UnwindSection};

fn foo() -> gimli::Result<()> {
let read_eh_frame_somehow = || unimplemented!();
let eh_frame = EhFrame::new(read_eh_frame_somehow(), NativeEndian);

let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
let address_of_eh_frame_section_in_memory = unimplemented!();
let address_of_text_section_in_memory = unimplemented!();
let address_of_got_section_in_memory = unimplemented!();
let address_of_the_start_of_current_func = unimplemented!();
// Provide base addresses for relative pointers.
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);

let mut entries = eh_frame.entries(&bases);

let do_stuff_with = |_| unimplemented!();
while let Some(entry) = entries.next()? {
    do_stuff_with(entry)
}
unreachable!()
}
```

#### Implementations

- <span id="cfientriesiter-next"></span>`fn next(&mut self) -> Result<Option<CieOrFde<'bases, Section, R>>>` — [`Result`](../index.md#result), [`CieOrFde`](#cieorfde)

  Advance the iterator to the next entry.

#### Trait Implementations

##### `impl Any for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Section, R> Clone for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-clone"></span>`fn clone(&self) -> CfiEntriesIter<'bases, Section, R>` — [`CfiEntriesIter`](#cfientriesiter)

##### `impl CloneToUninit for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Section, R> Debug for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-toowned-type-owned"></span>`type Owned = T`

- <span id="cfientriesiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cfientriesiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cfientriesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cfientriesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Augmentation`

```rust
struct Augmentation {
    lsda: Option<constants::DwEhPe>,
    personality: Option<(constants::DwEhPe, Pointer)>,
    fde_address_encoding: Option<constants::DwEhPe>,
    is_signal_trampoline: bool,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1122-1152`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1122-L1152)*

We support the z-style augmentation [defined by `.eh_frame`][ehframe].


#### Fields

- **`lsda`**: `Option<constants::DwEhPe>`

  > A 'L' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, it indicates the presence of one argument in
  > the Augmentation Data of the CIE, and a corresponding argument in the
  > Augmentation Data of the FDE. The argument in the Augmentation Data of
  > the CIE is 1-byte and represents the pointer encoding used for the
  > argument in the Augmentation Data of the FDE, which is the address of a
  > language-specific data area (LSDA). The size of the LSDA pointer is
  > specified by the pointer encoding used.

- **`personality`**: `Option<(constants::DwEhPe, Pointer)>`

  > A 'P' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, it indicates the presence of two arguments in
  > the Augmentation Data of the CIE. The first argument is 1-byte and
  > represents the pointer encoding used for the second argument, which is
  > the address of a personality routine handler. The size of the
  > personality routine pointer is specified by the pointer encoding used.

- **`fde_address_encoding`**: `Option<constants::DwEhPe>`

  > A 'R' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, The Augmentation Data shall include a 1 byte
  > argument that represents the pointer encoding for the address pointers
  > used in the FDE.

- **`is_signal_trampoline`**: `bool`

  True if this CIE's FDEs are trampolines for signal handlers.

#### Implementations

- <span id="augmentation-parse"></span>`fn parse<Section, R>(augmentation_str: &mut R, bases: &BaseAddresses, address_size: u8, section: &Section, input: &mut R) -> Result<Augmentation>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`Augmentation`](#augmentation)

#### Trait Implementations

##### `impl Any for Augmentation`

- <span id="augmentation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Augmentation`

- <span id="augmentation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Augmentation`

- <span id="augmentation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Augmentation`

- <span id="augmentation-clone"></span>`fn clone(&self) -> Augmentation` — [`Augmentation`](#augmentation)

##### `impl CloneToUninit for Augmentation`

- <span id="augmentation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Augmentation`

##### `impl Debug for Augmentation`

- <span id="augmentation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Augmentation`

- <span id="augmentation-default"></span>`fn default() -> Augmentation` — [`Augmentation`](#augmentation)

##### `impl Eq for Augmentation`

##### `impl<T> From for Augmentation`

- <span id="augmentation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Augmentation`

- <span id="augmentation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Augmentation`

- <span id="augmentation-partialeq-eq"></span>`fn eq(&self, other: &Augmentation) -> bool` — [`Augmentation`](#augmentation)

##### `impl StructuralPartialEq for Augmentation`

##### `impl ToOwned for Augmentation`

- <span id="augmentation-toowned-type-owned"></span>`type Owned = T`

- <span id="augmentation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="augmentation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Augmentation`

- <span id="augmentation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="augmentation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Augmentation`

- <span id="augmentation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="augmentation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AugmentationData`

```rust
struct AugmentationData {
    lsda: Option<Pointer>,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1223-1225`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1223-L1225)*

Parsed augmentation data for a `FrameDescriptEntry`.

#### Implementations

- <span id="augmentationdata-parse"></span>`fn parse<R: Reader>(augmentation: &Augmentation, encoding_parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> Result<AugmentationData>` — [`Augmentation`](#augmentation), [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters), [`Result`](../index.md#result), [`AugmentationData`](cfi/index.md#augmentationdata)

#### Trait Implementations

##### `impl Any for AugmentationData`

- <span id="augmentationdata-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AugmentationData`

- <span id="augmentationdata-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AugmentationData`

- <span id="augmentationdata-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AugmentationData`

- <span id="augmentationdata-clone"></span>`fn clone(&self) -> AugmentationData` — [`AugmentationData`](cfi/index.md#augmentationdata)

##### `impl CloneToUninit for AugmentationData`

- <span id="augmentationdata-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AugmentationData`

- <span id="augmentationdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AugmentationData`

- <span id="augmentationdata-default"></span>`fn default() -> AugmentationData` — [`AugmentationData`](cfi/index.md#augmentationdata)

##### `impl Eq for AugmentationData`

##### `impl<T> From for AugmentationData`

- <span id="augmentationdata-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AugmentationData`

- <span id="augmentationdata-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AugmentationData`

- <span id="augmentationdata-partialeq-eq"></span>`fn eq(&self, other: &AugmentationData) -> bool` — [`AugmentationData`](cfi/index.md#augmentationdata)

##### `impl StructuralPartialEq for AugmentationData`

##### `impl ToOwned for AugmentationData`

- <span id="augmentationdata-toowned-type-owned"></span>`type Owned = T`

- <span id="augmentationdata-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="augmentationdata-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AugmentationData`

- <span id="augmentationdata-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="augmentationdata-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AugmentationData`

- <span id="augmentationdata-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="augmentationdata-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CommonInformationEntry<R, Offset>`

```rust
struct CommonInformationEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: Offset,
    length: Offset,
    format: crate::common::Format,
    version: u8,
    augmentation: Option<Augmentation>,
    address_size: u8,
    code_alignment_factor: u64,
    data_alignment_factor: i64,
    return_address_register: crate::common::Register,
    initial_instructions: R,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1254-1306`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1254-L1306)*

> A Common Information Entry holds information that is shared among many
> Frame Description Entries. There is at least one CIE in every non-empty
> `.debug_frame` section.

#### Fields

- **`offset`**: `Offset`

  The offset of this entry from the start of its containing section.

- **`length`**: `Offset`

  > A constant that gives the number of bytes of the CIE structure, not
  > including the length field itself (see Section 7.2.2). The size of the
  > length field plus the value of length must be an integral multiple of
  > the address size.

- **`version`**: `u8`

  > A version number (see Section 7.23). This number is specific to the
  > call frame information and is independent of the DWARF version number.

- **`augmentation`**: `Option<Augmentation>`

  The parsed augmentation, if any.

- **`address_size`**: `u8`

  > The size of a target address in this CIE and any FDEs that use it, in
  > bytes. If a compilation unit exists for this frame, its address size
  > must match the address size here.

- **`code_alignment_factor`**: `u64`

  "A constant that is factored out of all advance location instructions
  (see Section 6.4.2.1)."

- **`data_alignment_factor`**: `i64`

  > A constant that is factored out of certain offset instructions (see
  > below). The resulting value is (operand * data_alignment_factor).

- **`return_address_register`**: `crate::common::Register`

  > An unsigned LEB128 constant that indicates which column in the rule
  > table represents the return address of the function. Note that this
  > column might not correspond to an actual machine register.

- **`initial_instructions`**: `R`

  > A sequence of rules that are interpreted to create the initial setting
  > of each column in the table.
  
  > The default rule for all columns before interpretation of the initial
  > instructions is the undefined rule. However, an ABI authoring body or a
  > compilation system authoring body may specify an alternate default
  > value for any or all columns.
  
  This is followed by `DW_CFA_nop` padding until the end of `length` bytes
  in the input.

#### Implementations

- <span id="commoninformationentry-parse"></span>`fn parse<Section: UnwindSection<R>>(bases: &BaseAddresses, section: &Section, input: &mut R) -> Result<CommonInformationEntry<R>>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`CommonInformationEntry`](#commoninformationentry)

- <span id="commoninformationentry-parse-rest"></span>`fn parse_rest<Section: UnwindSection<R>>(offset: <R as >::Offset, length: <R as >::Offset, format: Format, bases: &BaseAddresses, section: &Section, rest: R) -> Result<CommonInformationEntry<R>>` — [`Reader`](#reader), [`Format`](../index.md#format), [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`CommonInformationEntry`](#commoninformationentry)

#### Trait Implementations

##### `impl Any for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-clone"></span>`fn clone(&self) -> CommonInformationEntry<R, Offset>` — [`CommonInformationEntry`](#commoninformationentry)

##### `impl CloneToUninit for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for CommonInformationEntry<R, Offset>`

##### `impl<T> From for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-partialeq-eq"></span>`fn eq(&self, other: &CommonInformationEntry<R, Offset>) -> bool` — [`CommonInformationEntry`](#commoninformationentry)

##### `impl<R, Offset> StructuralPartialEq for CommonInformationEntry<R, Offset>`

##### `impl ToOwned for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-toowned-type-owned"></span>`type Owned = T`

- <span id="commoninformationentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="commoninformationentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="commoninformationentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="commoninformationentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PartialFrameDescriptionEntry<'bases, Section, R>`

```rust
struct PartialFrameDescriptionEntry<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    offset: <R as >::Offset,
    length: <R as >::Offset,
    format: crate::common::Format,
    cie_offset: <Section as >::Offset,
    rest: R,
    section: Section,
    bases: &'bases BaseAddresses,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1520-1532`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1520-L1532)*

A partially parsed `FrameDescriptionEntry`.

Fully parsing this FDE requires first parsing its CIE.

#### Implementations

- <span id="partialframedescriptionentry-parse-partial"></span>`fn parse_partial(section: &Section, bases: &'bases BaseAddresses, input: &mut R) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

- <span id="partialframedescriptionentry-parse"></span>`fn parse<F>(&self, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Result`](../index.md#result), [`FrameDescriptionEntry`](#framedescriptionentry)

  Fully parse this FDE.

  

  You must provide a function get its associated CIE (either by parsing it

  on demand, or looking it up in some table mapping offsets to CIEs that

  you've already parsed, etc.)

- <span id="partialframedescriptionentry-offset"></span>`fn offset(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Get the offset of this entry from the start of its containing section.

- <span id="partialframedescriptionentry-cie-offset"></span>`fn cie_offset(&self) -> <Section as >::Offset` — [`UnwindSection`](#unwindsection)

  Get the offset of this FDE's CIE.

- <span id="partialframedescriptionentry-entry-len"></span>`fn entry_len(&self) -> <R as >::Offset` — [`Reader`](#reader)

  > A constant that gives the number of bytes of the header and

  > instruction stream for this function, not including the length field

  > itself (see Section 7.2.2). The size of the length field plus the value

  > of length must be an integral multiple of the address size.

#### Trait Implementations

##### `impl Any for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Section, R> Clone for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-clone"></span>`fn clone(&self) -> PartialFrameDescriptionEntry<'bases, Section, R>` — [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

##### `impl CloneToUninit for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Section, R> Debug for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section, R> Eq for PartialFrameDescriptionEntry<'bases, Section, R>`

##### `impl<T> From for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Section, R> PartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-partialeq-eq"></span>`fn eq(&self, other: &PartialFrameDescriptionEntry<'bases, Section, R>) -> bool` — [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

##### `impl<Section, R> StructuralPartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

##### `impl ToOwned for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-toowned-type-owned"></span>`type Owned = T`

- <span id="partialframedescriptionentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="partialframedescriptionentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="partialframedescriptionentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="partialframedescriptionentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FrameDescriptionEntry<R, Offset>`

```rust
struct FrameDescriptionEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: Offset,
    length: Offset,
    format: crate::common::Format,
    cie: CommonInformationEntry<R, Offset>,
    initial_address: u64,
    address_range: u64,
    augmentation: Option<AugmentationData>,
    instructions: R,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1593-1631`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1593-L1631)*

A `FrameDescriptionEntry` is a set of CFA instructions for an address range.

#### Fields

- **`offset`**: `Offset`

  The start of this entry within its containing section.

- **`length`**: `Offset`

  > A constant that gives the number of bytes of the header and
  > instruction stream for this function, not including the length field
  > itself (see Section 7.2.2). The size of the length field plus the value
  > of length must be an integral multiple of the address size.

- **`cie`**: `CommonInformationEntry<R, Offset>`

  "A constant offset into the .debug_frame section that denotes the CIE
  that is associated with this FDE."
  
  This is the CIE at that offset.

- **`initial_address`**: `u64`

  > The address of the first location associated with this table entry. If
  > the segment_size field of this FDE's CIE is non-zero, the initial
  > location is preceded by a segment selector of the given length.

- **`address_range`**: `u64`

  "The number of bytes of program instructions described by this entry."

- **`augmentation`**: `Option<AugmentationData>`

  The parsed augmentation data, if we have any.

- **`instructions`**: `R`

  "A sequence of table defining instructions that are described below."
  
  This is followed by `DW_CFA_nop` padding until `length` bytes of the
  input are consumed.

#### Implementations

- <span id="framedescriptionentry-parse-rest"></span>`fn parse_rest<Section, F>(offset: <R as >::Offset, length: <R as >::Offset, format: Format, cie_pointer: <Section as >::Offset, rest: R, section: &Section, bases: &BaseAddresses, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Reader`](#reader), [`Format`](../index.md#format), [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`FrameDescriptionEntry`](#framedescriptionentry)

- <span id="framedescriptionentry-parse-addresses"></span>`fn parse_addresses(input: &mut R, cie: &CommonInformationEntry<R>, parameters: &PointerEncodingParameters<'_, R>) -> Result<(u64, u64)>` — [`CommonInformationEntry`](#commoninformationentry), [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters), [`Result`](../index.md#result)

- <span id="framedescriptionentry-rows"></span>`fn rows<'a, 'ctx, Section, S>(&self, section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>) -> Result<UnwindTable<'a, 'ctx, R, S>>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md#result), [`UnwindTable`](#unwindtable)

  Return the table of unwind information for this FDE.

- <span id="framedescriptionentry-unwind-info-for-address"></span>`fn unwind_info_for_address<'ctx, Section, S>(&self, section: &Section, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md#result), [`UnwindTableRow`](#unwindtablerow)

  Find the frame unwind information for the given address.

  

  If found, the unwind information is returned along with the reset

  context in the form `Ok((unwind_info, context))`. If not found,

  `Err(gimli::Error::NoUnwindInfoForAddress)` is returned. If parsing or

  CFI evaluation fails, the error is returned.

#### Trait Implementations

##### `impl Any for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-clone"></span>`fn clone(&self) -> FrameDescriptionEntry<R, Offset>` — [`FrameDescriptionEntry`](#framedescriptionentry)

##### `impl CloneToUninit for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for FrameDescriptionEntry<R, Offset>`

##### `impl<T> From for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-partialeq-eq"></span>`fn eq(&self, other: &FrameDescriptionEntry<R, Offset>) -> bool` — [`FrameDescriptionEntry`](#framedescriptionentry)

##### `impl<R, Offset> StructuralPartialEq for FrameDescriptionEntry<R, Offset>`

##### `impl ToOwned for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-toowned-type-owned"></span>`type Owned = T`

- <span id="framedescriptionentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="framedescriptionentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="framedescriptionentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="framedescriptionentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnwindContext<T, S>`

```rust
struct UnwindContext<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    stack: super::util::ArrayVec<<S as >::Stack>,
    initial_rule: Option<(crate::common::Register, RegisterRule<T>)>,
    is_initialized: bool,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1951-1972`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1951-L1972)*

Common context needed when evaluating the call frame unwinding information.

By default, this structure is small and allocates its internal storage
on the heap using [`Box`](../../allocator_api2/stable/boxed/index.md) during `UnwindContext::new`.

This can be overridden by providing a custom [`UnwindContextStorage`](#unwindcontextstorage) type parameter.
When using a custom storage with in-line arrays, the [`UnwindContext`](#unwindcontext) type itself
will be big, so in that case it's recommended to place [`UnwindContext`](#unwindcontext) on the
heap, e.g. using `Box::new(UnwindContext::<R, MyCustomStorage>::new_in())`.

To avoid re-allocating the context multiple times when evaluating multiple
CFI programs, the same [`UnwindContext`](#unwindcontext) can be reused for multiple unwinds.

```rust
use gimli::{UnwindContext, UnwindTable};

fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
           -> gimli::Result<()> {
let eh_frame: gimli::EhFrame<_> = unreachable!();
let bases = unimplemented!();
// An uninitialized context.
let mut ctx = UnwindContext::new();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
  let _ = row;
}
unreachable!()
}
```

#### Implementations

- <span id="unwindcontext-new"></span>`fn new() -> Self`

  Construct a new call frame unwinding context.

#### Trait Implementations

##### `impl<T> Any for UnwindContext<T, S>`

- <span id="unwindcontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnwindContext<T, S>`

- <span id="unwindcontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnwindContext<T, S>`

- <span id="unwindcontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S> Clone for UnwindContext<T, S>`

- <span id="unwindcontext-clone"></span>`fn clone(&self) -> UnwindContext<T, S>` — [`UnwindContext`](#unwindcontext)

##### `impl<T> CloneToUninit for UnwindContext<T, S>`

- <span id="unwindcontext-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S> Debug for UnwindContext<T, S>`

- <span id="unwindcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindContext<T, S>`

- <span id="unwindcontext-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for UnwindContext<T, S>`

##### `impl<T> From for UnwindContext<T, S>`

- <span id="unwindcontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for UnwindContext<T, S>`

- <span id="unwindcontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, S> PartialEq for UnwindContext<T, S>`

- <span id="unwindcontext-partialeq-eq"></span>`fn eq(&self, other: &UnwindContext<T, S>) -> bool` — [`UnwindContext`](#unwindcontext)

##### `impl<T, S> StructuralPartialEq for UnwindContext<T, S>`

##### `impl<T> ToOwned for UnwindContext<T, S>`

- <span id="unwindcontext-toowned-type-owned"></span>`type Owned = T`

- <span id="unwindcontext-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unwindcontext-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for UnwindContext<T, S>`

- <span id="unwindcontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unwindcontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnwindContext<T, S>`

- <span id="unwindcontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unwindcontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnwindTable<'a, 'ctx, R, S>`

```rust
struct UnwindTable<'a, 'ctx, R, S>
where
    R: Reader,
    S: UnwindContextStorage<<R as >::Offset> {
    code_alignment_factor: core::num::Wrapping<u64>,
    data_alignment_factor: core::num::Wrapping<i64>,
    address_size: u8,
    next_start_address: u64,
    last_end_address: u64,
    returned_last_row: bool,
    current_row_valid: bool,
    instructions: CallFrameInstructionIter<'a, R>,
    ctx: &'ctx mut UnwindContext<<R as >::Offset, S>,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2193-2207`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2193-L2207)*

The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s
`CallFrameInstruction` program, yielding the each row one at a time.

> 6.4.1 Structure of Call Frame Information
>
> DWARF supports virtual unwinding by defining an architecture independent
> basis for recording how procedures save and restore registers during their
> lifetimes. This basis must be augmented on some machines with specific
> information that is defined by an architecture specific ABI authoring
> committee, a hardware vendor, or a compiler producer. The body defining a
> specific augmentation is referred to below as the “augmenter.”
>
> Abstractly, this mechanism describes a very large table that has the
> following structure:
>
> <table>
>   <tr>
>     <th>LOC</th><th>CFA</th><th>R0</th><th>R1</th><td>...</td><th>RN</th>
>   </tr>
>   <tr>
>     <th>L0</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <th>L1</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <td>...</td><td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <th>LN</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
> </table>
>
> The first column indicates an address for every location that contains code
> in a program. (In shared objects, this is an object-relative offset.) The
> remaining columns contain virtual unwinding rules that are associated with
> the indicated location.
>
> The CFA column defines the rule which computes the Canonical Frame Address
> value; it may be either a register and a signed offset that are added
> together, or a DWARF expression that is evaluated.
>
> The remaining columns are labeled by register number. This includes some
> registers that have special designation on some architectures such as the PC
> and the stack pointer register. (The actual mapping of registers for a
> particular architecture is defined by the augmenter.) The register columns
> contain rules that describe whether a given register has been saved and the
> rule to find the value for the register in the previous frame.
>
> ...
>
> This table would be extremely large if actually constructed as
> described. Most of the entries at any point in the table are identical to
> the ones above them. The whole table can be represented quite compactly by
> recording just the differences starting at the beginning address of each
> subroutine in the program.

#### Implementations

- <span id="unwindtable-new"></span>`fn new<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Result<Self>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`FrameDescriptionEntry`](#framedescriptionentry), [`Result`](../index.md#result)

  Construct a new `UnwindTable` for the given

  `FrameDescriptionEntry`'s CFI unwinding program.

- <span id="unwindtable-new-for-fde"></span>`fn new_for_fde<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Self` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`FrameDescriptionEntry`](#framedescriptionentry)

- <span id="unwindtable-new-for-cie"></span>`fn new_for_cie<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, cie: &CommonInformationEntry<R>) -> Self` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`CommonInformationEntry`](#commoninformationentry)

- <span id="unwindtable-next-row"></span>`fn next_row(&mut self) -> Result<Option<&UnwindTableRow<<R as >::Offset, S>>>` — [`Result`](../index.md#result), [`UnwindTableRow`](#unwindtablerow), [`Reader`](#reader)

  Evaluate call frame instructions until the next row of the table is

  completed, and return it.

  

  Unfortunately, this cannot be used with `FallibleIterator` because of

  the restricted lifetime of the yielded item.

- <span id="unwindtable-into-current-row"></span>`fn into_current_row(self) -> Option<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`UnwindTableRow`](#unwindtablerow), [`Reader`](#reader)

  Returns the current row with the lifetime of the context.

- <span id="unwindtable-evaluate"></span>`fn evaluate(&mut self, instruction: CallFrameInstruction<<R as >::Offset>) -> Result<bool>` — [`CallFrameInstruction`](#callframeinstruction), [`Reader`](#reader), [`Result`](../index.md#result)

  Evaluate one call frame instruction. Return `Ok(true)` if the row is

  complete, `Ok(false)` otherwise.

#### Trait Implementations

##### `impl Any for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, S> Debug for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unwindtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unwindtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegisterRuleMap<T, S>`

```rust
struct RegisterRuleMap<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    rules: super::util::ArrayVec<<S as >::Rules>,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2530-2536`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2530-L2536)*

#### Implementations

- <span id="registerrulemap-is-default"></span>`fn is_default(&self) -> bool`

- <span id="registerrulemap-get"></span>`fn get(&self, register: Register) -> RegisterRule<T>` — [`Register`](../index.md#register), [`RegisterRule`](#registerrule)

- <span id="registerrulemap-set"></span>`fn set(&mut self, register: Register, rule: RegisterRule<T>) -> Result<()>` — [`Register`](../index.md#register), [`RegisterRule`](#registerrule), [`Result`](../index.md#result)

- <span id="registerrulemap-iter"></span>`fn iter(&self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](#registerruleiter)

#### Trait Implementations

##### `impl<T> Any for RegisterRuleMap<T, S>`

- <span id="registerrulemap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegisterRuleMap<T, S>`

- <span id="registerrulemap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegisterRuleMap<T, S>`

- <span id="registerrulemap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S> Clone for RegisterRuleMap<T, S>`

- <span id="registerrulemap-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for RegisterRuleMap<T, S>`

- <span id="registerrulemap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S> Debug for RegisterRuleMap<T, S>`

- <span id="registerrulemap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for RegisterRuleMap<T, S>`

- <span id="registerrulemap-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for RegisterRuleMap<T, S>`

##### `impl<T> From for RegisterRuleMap<T, S>`

- <span id="registerrulemap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<R, S> FromIterator for RegisterRuleMap<R, S>`

- <span id="registerrulemap-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl<T, U> Into for RegisterRuleMap<T, S>`

- <span id="registerrulemap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, S> PartialEq for RegisterRuleMap<T, S>`

- <span id="registerrulemap-partialeq-eq"></span>`fn eq(&self, rhs: &Self) -> bool`

##### `impl<T> ToOwned for RegisterRuleMap<T, S>`

- <span id="registerrulemap-toowned-type-owned"></span>`type Owned = T`

- <span id="registerrulemap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="registerrulemap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RegisterRuleMap<T, S>`

- <span id="registerrulemap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="registerrulemap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RegisterRuleMap<T, S>`

- <span id="registerrulemap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="registerrulemap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegisterRuleIter<'iter, T>`

```rust
struct RegisterRuleIter<'iter, T>(::core::slice::Iter<'iter, (crate::common::Register, RegisterRule<T>)>)
where
    T: ReaderOffset;
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2684-2686`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2684-L2686)*

An unordered iterator for register rules.

#### Trait Implementations

##### `impl<T> Any for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-clone"></span>`fn clone(&self) -> RegisterRuleIter<'iter, T>` — [`RegisterRuleIter`](#registerruleiter)

##### `impl<T> CloneToUninit for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="registerruleiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="registerruleiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: ReaderOffset> Iterator for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-iterator-type-item"></span>`type Item = &'iter (Register, RegisterRule<T>)`

- <span id="registerruleiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ToOwned for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-toowned-type-owned"></span>`type Owned = T`

- <span id="registerruleiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="registerruleiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="registerruleiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="registerruleiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnwindTableRow<T, S>`

```rust
struct UnwindTableRow<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    start_address: u64,
    end_address: u64,
    saved_args_size: u64,
    cfa: CfaRule<T>,
    registers: RegisterRuleMap<T, S>,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2699-2709`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2699-L2709)*

A row in the virtual unwind table that describes how to find the values of
the registers in the *previous* frame for a range of PC addresses.

#### Implementations

- <span id="unwindtablerow-is-default"></span>`fn is_default(&self) -> bool`

- <span id="unwindtablerow-start-address"></span>`fn start_address(&self) -> u64`

  Get the starting PC address that this row applies to.

- <span id="unwindtablerow-end-address"></span>`fn end_address(&self) -> u64`

  Get the end PC address where this row's register rules become

  unapplicable.

  

  In other words, this row describes how to recover the last frame's

  registers for all PCs where `row.start_address() <= PC <

  row.end_address()`. This row does NOT describe how to recover registers

  when `PC == row.end_address()`.

- <span id="unwindtablerow-contains"></span>`fn contains(&self, address: u64) -> bool`

  Return `true` if the given `address` is within this row's address range,

  `false` otherwise.

- <span id="unwindtablerow-saved-args-size"></span>`fn saved_args_size(&self) -> u64`

  Returns the amount of args currently on the stack.

  

  When unwinding, if the personality function requested a change in IP,

  the SP needs to be adjusted by saved_args_size.

- <span id="unwindtablerow-cfa"></span>`fn cfa(&self) -> &CfaRule<T>` — [`CfaRule`](#cfarule)

  Get the canonical frame address (CFA) recovery rule for this row.

- <span id="unwindtablerow-register"></span>`fn register(&self, register: Register) -> RegisterRule<T>` — [`Register`](../index.md#register), [`RegisterRule`](#registerrule)

  Get the register recovery rule for the given register number.

  

  The register number mapping is architecture dependent. For example, in

  the x86-64 ABI the register number mapping is defined in Figure 3.36:

  

  > Figure 3.36: DWARF Register Number Mapping

  >

  > <table>

  >   <tr><th>Register Name</th>                    <th>Number</th>  <th>Abbreviation</th></tr>

  >   <tr><td>General Purpose Register RAX</td>     <td>0</td>       <td>%rax</td></tr>

  >   <tr><td>General Purpose Register RDX</td>     <td>1</td>       <td>%rdx</td></tr>

  >   <tr><td>General Purpose Register RCX</td>     <td>2</td>       <td>%rcx</td></tr>

  >   <tr><td>General Purpose Register RBX</td>     <td>3</td>       <td>%rbx</td></tr>

  >   <tr><td>General Purpose Register RSI</td>     <td>4</td>       <td>%rsi</td></tr>

  >   <tr><td>General Purpose Register RDI</td>     <td>5</td>       <td>%rdi</td></tr>

  >   <tr><td>General Purpose Register RBP</td>     <td>6</td>       <td>%rbp</td></tr>

  >   <tr><td>Stack Pointer Register RSP</td>       <td>7</td>       <td>%rsp</td></tr>

  >   <tr><td>Extended Integer Registers 8-15</td>  <td>8-15</td>    <td>%r8-%r15</td></tr>

  >   <tr><td>Return Address RA</td>                <td>16</td>      <td></td></tr>

  >   <tr><td>Vector Registers 0–7</td>             <td>17-24</td>   <td>%xmm0–%xmm7</td></tr>

  >   <tr><td>Extended Vector Registers 8–15</td>   <td>25-32</td>   <td>%xmm8–%xmm15</td></tr>

  >   <tr><td>Floating Point Registers 0–7</td>     <td>33-40</td>   <td>%st0–%st7</td></tr>

  >   <tr><td>MMX Registers 0–7</td>                <td>41-48</td>   <td>%mm0–%mm7</td></tr>

  >   <tr><td>Flag Register</td>                    <td>49</td>      <td>%rFLAGS</td></tr>

  >   <tr><td>Segment Register ES</td>              <td>50</td>      <td>%es</td></tr>

  >   <tr><td>Segment Register CS</td>              <td>51</td>      <td>%cs</td></tr>

  >   <tr><td>Segment Register SS</td>              <td>52</td>      <td>%ss</td></tr>

  >   <tr><td>Segment Register DS</td>              <td>53</td>      <td>%ds</td></tr>

  >   <tr><td>Segment Register FS</td>              <td>54</td>      <td>%fs</td></tr>

  >   <tr><td>Segment Register GS</td>              <td>55</td>      <td>%gs</td></tr>

  >   <tr><td>Reserved</td>                         <td>56-57</td>   <td></td></tr>

  >   <tr><td>FS Base address</td>                  <td>58</td>      <td>%fs.base</td></tr>

  >   <tr><td>GS Base address</td>                  <td>59</td>      <td>%gs.base</td></tr>

  >   <tr><td>Reserved</td>                         <td>60-61</td>   <td></td></tr>

  >   <tr><td>Task Register</td>                    <td>62</td>      <td>%tr</td></tr>

  >   <tr><td>LDT Register</td>                     <td>63</td>      <td>%ldtr</td></tr>

  >   <tr><td>128-bit Media Control and Status</td> <td>64</td>      <td>%mxcsr</td></tr>

  >   <tr><td>x87 Control Word</td>                 <td>65</td>      <td>%fcw</td></tr>

  >   <tr><td>x87 Status Word</td>                  <td>66</td>      <td>%fsw</td></tr>

  >   <tr><td>Upper Vector Registers 16–31</td>     <td>67-82</td>   <td>%xmm16–%xmm31</td></tr>

  >   <tr><td>Reserved</td>                         <td>83-117</td>  <td></td></tr>

  >   <tr><td>Vector Mask Registers 0–7</td>        <td>118-125</td> <td>%k0–%k7</td></tr>

  >   <tr><td>Reserved</td>                         <td>126-129</td> <td></td></tr>

  > </table>

- <span id="unwindtablerow-registers"></span>`fn registers(&self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](#registerruleiter)

  Iterate over all defined register `(number, rule)` pairs.

  

  The rules are not iterated in any guaranteed order. Any register that

  does not make an appearance in the iterator implicitly has the rule

  `RegisterRule::Undefined`.

  

  ```rust

  use gimli::{EndianSlice, LittleEndian, UnwindTableRow};

  fn foo<'input>(unwind_table_row: UnwindTableRow<usize>) {

  for &(register, ref rule) in unwind_table_row.registers() {

      // ...

      drop(register); drop(rule);

  }

  }

  ```

#### Trait Implementations

##### `impl<T> Any for UnwindTableRow<T, S>`

- <span id="unwindtablerow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnwindTableRow<T, S>`

- <span id="unwindtablerow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnwindTableRow<T, S>`

- <span id="unwindtablerow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S> Clone for UnwindTableRow<T, S>`

- <span id="unwindtablerow-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for UnwindTableRow<T, S>`

- <span id="unwindtablerow-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S> Debug for UnwindTableRow<T, S>`

- <span id="unwindtablerow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindTableRow<T, S>`

- <span id="unwindtablerow-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for UnwindTableRow<T, S>`

##### `impl<T> From for UnwindTableRow<T, S>`

- <span id="unwindtablerow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for UnwindTableRow<T, S>`

- <span id="unwindtablerow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, S> PartialEq for UnwindTableRow<T, S>`

- <span id="unwindtablerow-partialeq-eq"></span>`fn eq(&self, other: &UnwindTableRow<T, S>) -> bool` — [`UnwindTableRow`](#unwindtablerow)

##### `impl<T, S> StructuralPartialEq for UnwindTableRow<T, S>`

##### `impl<T> ToOwned for UnwindTableRow<T, S>`

- <span id="unwindtablerow-toowned-type-owned"></span>`type Owned = T`

- <span id="unwindtablerow-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unwindtablerow-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for UnwindTableRow<T, S>`

- <span id="unwindtablerow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unwindtablerow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnwindTableRow<T, S>`

- <span id="unwindtablerow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unwindtablerow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CallFrameInstructionIter<'a, R: Reader>`

```rust
struct CallFrameInstructionIter<'a, R: Reader> {
    input: R,
    address_encoding: Option<constants::DwEhPe>,
    parameters: PointerEncodingParameters<'a, R>,
    vendor: crate::common::Vendor,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3471-3476`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3471-L3476)*

A lazy iterator parsing call frame instructions.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="callframeinstructioniter-next"></span>`fn next(&mut self) -> Result<Option<CallFrameInstruction<<R as >::Offset>>>` — [`Result`](../index.md#result), [`CallFrameInstruction`](#callframeinstruction), [`Reader`](#reader)

  Parse the next call frame instruction.

#### Trait Implementations

##### `impl Any for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-clone"></span>`fn clone(&self) -> CallFrameInstructionIter<'a, R>` — [`CallFrameInstructionIter`](#callframeinstructioniter)

##### `impl CloneToUninit for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-toowned-type-owned"></span>`type Owned = T`

- <span id="callframeinstructioniter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="callframeinstructioniter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="callframeinstructioniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="callframeinstructioniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnwindExpression<T: ReaderOffset>`

```rust
struct UnwindExpression<T: ReaderOffset> {
    pub offset: T,
    pub length: T,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3537-3542`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3537-L3542)*

The location of a DWARF expression within an unwind section.

This is stored as an offset and length within the section instead of as a
`Reader` to avoid lifetime issues when reusing [`UnwindContext`](#unwindcontext).

# Example
```rust
use gimli::{EhFrame, EndianSlice, NativeEndian, Error, FrameDescriptionEntry, UnwindExpression, EvaluationResult};
fn foo() -> Result<(), Error> {
let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();
let fde: FrameDescriptionEntry<EndianSlice<NativeEndian>> = unimplemented!();
let unwind_expression: UnwindExpression<_> = unimplemented!();
let expression = unwind_expression.get(&eh_frame)?;
let mut evaluation = expression.evaluation(fde.cie().encoding());
let mut result = evaluation.evaluate()?;
loop {
  match result {
     EvaluationResult::Complete => break,
     // Provide information to the evaluation.
     _ => { unimplemented!()}
  }
}
let value = evaluation.value_result();
Ok(())
}
```

#### Fields

- **`offset`**: `T`

  The offset of the expression within the section.

- **`length`**: `T`

  The length of the expression.

#### Implementations

- <span id="unwindexpression-get"></span>`fn get<R, S>(&self, section: &S) -> Result<Expression<R>>` — [`Result`](../index.md#result), [`Expression`](#expression)

  Get the expression from the section.

  

  The offset and length were previously validated when the

  `UnwindExpression` was created, so this should not fail.

#### Trait Implementations

##### `impl<T> Any for UnwindExpression<T>`

- <span id="unwindexpression-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnwindExpression<T>`

- <span id="unwindexpression-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnwindExpression<T>`

- <span id="unwindexpression-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + ReaderOffset> Clone for UnwindExpression<T>`

- <span id="unwindexpression-clone"></span>`fn clone(&self) -> UnwindExpression<T>` — [`UnwindExpression`](#unwindexpression)

##### `impl<T> CloneToUninit for UnwindExpression<T>`

- <span id="unwindexpression-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy + ReaderOffset> Copy for UnwindExpression<T>`

##### `impl<T: fmt::Debug + ReaderOffset> Debug for UnwindExpression<T>`

- <span id="unwindexpression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for UnwindExpression<T>`

##### `impl<T> From for UnwindExpression<T>`

- <span id="unwindexpression-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for UnwindExpression<T>`

- <span id="unwindexpression-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for UnwindExpression<T>`

- <span id="unwindexpression-partialeq-eq"></span>`fn eq(&self, other: &UnwindExpression<T>) -> bool` — [`UnwindExpression`](#unwindexpression)

##### `impl<T: ReaderOffset> StructuralPartialEq for UnwindExpression<T>`

##### `impl<T> ToOwned for UnwindExpression<T>`

- <span id="unwindexpression-toowned-type-owned"></span>`type Owned = T`

- <span id="unwindexpression-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unwindexpression-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for UnwindExpression<T>`

- <span id="unwindexpression-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unwindexpression-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for UnwindExpression<T>`

- <span id="unwindexpression-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unwindexpression-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PointerEncodingParameters<'a, R: Reader>`

```rust
struct PointerEncodingParameters<'a, R: Reader> {
    bases: &'a SectionBaseAddresses,
    func_base: Option<u64>,
    address_size: u8,
    section: &'a R,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3626-3631`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3626-L3631)*

#### Trait Implementations

##### `impl Any for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-clone"></span>`fn clone(&self) -> PointerEncodingParameters<'a, R>` — [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters)

##### `impl CloneToUninit for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-toowned-type-owned"></span>`type Owned = T`

- <span id="pointerencodingparameters-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pointerencodingparameters-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pointerencodingparameters-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pointerencodingparameters-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DwarfSections<T>`

```rust
struct DwarfSections<T> {
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_addr: crate::read::DebugAddr<T>,
    pub debug_aranges: crate::read::DebugAranges<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_line_str: crate::read::DebugLineStr<T>,
    pub debug_macinfo: crate::read::DebugMacinfo<T>,
    pub debug_macro: crate::read::DebugMacro<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_types: crate::read::DebugTypes<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_ranges: crate::read::DebugRanges<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:51-82`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L51-L82)*

All of the commonly used DWARF sections.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `Dwarf` that references the data in `self`.
If `T` does implement `Reader`, then use `Dwarf` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;
// Create references to the DWARF sections.
let dwarf: gimli::Dwarf<_> = dwarf_sections.borrow(|section| {
    gimli::EndianSlice::new(&section, gimli::LittleEndian)
});
unreachable!()
}
```

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<T>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<T>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<T>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<T>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<T>`

  The `.debug_macro` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc` section.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists` section.

- **`debug_ranges`**: `crate::read::DebugRanges<T>`

  The `.debug_ranges` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists` section.

#### Implementations

- <span id="dwarfsections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the DWARF sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfsections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

  Create a `Dwarf` structure that references the data in `self`.

- <span id="dwarfsections-borrow-with-sup"></span>`fn borrow_with_sup<'a, F, R>(self: &'a Self, sup: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

  Create a `Dwarf` structure that references the data in `self` and `sup`.

  

  This is like `borrow`, but also includes the supplementary object file.

  This is useful when `R` implements `Reader` but `T` does not.

  

  ## Example Usage

  

  ```rust,no_run

  fn example() -> Result<(), gimli::Error> {

  let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  let sup_loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  // Read the DWARF sections into `Vec`s with whatever object loader you're using.

  let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;

  let dwarf_sup_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(sup_loader)?;

  // Create references to the DWARF sections.

  let dwarf = dwarf_sections.borrow_with_sup(&dwarf_sup_sections, |section| {

      gimli::EndianSlice::new(&section, gimli::LittleEndian)

  });

  unreachable!()

  }

  ```

#### Trait Implementations

##### `impl<T> Any for DwarfSections<T>`

- <span id="dwarfsections-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DwarfSections<T>`

- <span id="dwarfsections-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DwarfSections<T>`

- <span id="dwarfsections-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for DwarfSections<T>`

- <span id="dwarfsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfSections<T>`

- <span id="dwarfsections-default"></span>`fn default() -> DwarfSections<T>` — [`DwarfSections`](#dwarfsections)

##### `impl<T> From for DwarfSections<T>`

- <span id="dwarfsections-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DwarfSections<T>`

- <span id="dwarfsections-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for DwarfSections<T>`

- <span id="dwarfsections-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwarfsections-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DwarfSections<T>`

- <span id="dwarfsections-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwarfsections-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dwarf<R>`

```rust
struct Dwarf<R> {
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_addr: crate::read::DebugAddr<R>,
    pub debug_aranges: crate::read::DebugAranges<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_line_str: crate::read::DebugLineStr<R>,
    pub debug_macinfo: crate::read::DebugMacinfo<R>,
    pub debug_macro: crate::read::DebugMacro<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub locations: crate::read::LocationLists<R>,
    pub ranges: crate::read::RangeLists<R>,
    pub file_type: crate::common::DwarfFileType,
    pub sup: Option<alloc::sync::Arc<Dwarf<R>>>,
    pub abbreviations_cache: crate::read::AbbreviationsCache,
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:170-218`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L170-L218)*

All of the commonly used DWARF sections, and other common information.

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<R>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<R>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<R>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<R>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<R>`

  The `.debug_macro` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types` section.

- **`locations`**: `crate::read::LocationLists<R>`

  The location lists in the `.debug_loc` and `.debug_loclists` sections.

- **`ranges`**: `crate::read::RangeLists<R>`

  The range lists in the `.debug_ranges` and `.debug_rnglists` sections.

- **`file_type`**: `crate::common::DwarfFileType`

  The type of this file.

- **`sup`**: `Option<alloc::sync::Arc<Dwarf<R>>>`

  The DWARF sections for a supplementary object file.

- **`abbreviations_cache`**: `crate::read::AbbreviationsCache`

  A cache of previously parsed abbreviations for units in this file.

#### Implementations

- <span id="dwarf-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the DWARF sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

  

  After loading, the user should set the `file_type` field and

  call `load_sup` if required.

- <span id="dwarf-load-sup"></span>`fn load_sup<F, E>(&mut self, section: F) -> core::result::Result<(), E>`

  Load the DWARF sections from the supplementary object file.

  

  `section` operates the same as for `load`.

  

  Sets `self.sup`, replacing any previous value.

- <span id="dwarf-from-sections"></span>`fn from_sections(sections: DwarfSections<T>) -> Self` — [`DwarfSections`](#dwarfsections)

  Create a `Dwarf` structure from the given sections.

  

  The caller should set the `file_type` and `sup` fields if required.

- <span id="dwarf-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

  Create a `Dwarf` structure that references the data in `self`.

  

  This is useful when `R` implements `Reader` but `T` does not.

  

  ## Example Usage

  

  It can be useful to load DWARF sections into owned data structures,

  such as `Vec`. However, we do not implement the `Reader` trait

  for `Vec`, because it would be very inefficient, but this trait

  is required for all of the methods that parse the DWARF data.

  So we first load the DWARF sections into `Vec`s, and then use

  `borrow` to create `Reader`s that reference the data.

  

  ```rust,no_run

  fn example() -> Result<(), gimli::Error> {

  let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  let sup_loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  // Read the DWARF sections into `Vec`s with whatever object loader you're using.

  let mut owned_dwarf: gimli::Dwarf<Vec<u8>> = gimli::Dwarf::load(loader)?;

  owned_dwarf.load_sup(sup_loader)?;

  // Create references to the DWARF sections.

  let dwarf = owned_dwarf.borrow(|section| {

      gimli::EndianSlice::new(&section, gimli::LittleEndian)

  });

  unreachable!()

  }

  ```

- <span id="dwarf-set-sup"></span>`fn set_sup(&mut self, sup: Dwarf<T>)` — [`Dwarf`](#dwarf)

  Store the DWARF sections for the supplementary object file.

- <span id="dwarf-sup"></span>`fn sup(&self) -> Option<&Dwarf<T>>` — [`Dwarf`](#dwarf)

  Return a reference to the DWARF sections for the supplementary object file.

#### Trait Implementations

##### `impl Any for Dwarf<R>`

- <span id="dwarf-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dwarf<R>`

- <span id="dwarf-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dwarf<R>`

- <span id="dwarf-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug> Debug for Dwarf<R>`

- <span id="dwarf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for Dwarf<R>`

- <span id="dwarf-default"></span>`fn default() -> Dwarf<R>` — [`Dwarf`](#dwarf)

##### `impl<T> From for Dwarf<R>`

- <span id="dwarf-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Dwarf<R>`

- <span id="dwarf-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Dwarf<R>`

- <span id="dwarf-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwarf-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dwarf<R>`

- <span id="dwarf-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwarf-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DwarfPackageSections<T>`

```rust
struct DwarfPackageSections<T> {
    pub cu_index: crate::read::DebugCuIndex<T>,
    pub tu_index: crate::read::DebugTuIndex<T>,
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
    pub debug_types: crate::read::DebugTypes<T>,
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:804-831`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L804-L831)*

The sections from a `.dwp` file.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `DwarfPackage` that references the data in `self`.
If `T` does implement `Reader`, then use `DwarfPackage` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwp_sections: gimli::DwarfPackageSections<Vec<u8>> = gimli::DwarfPackageSections::load(loader)?;
// Create references to the DWARF sections.
let dwp: gimli::DwarfPackage<_> = dwp_sections.borrow(
    |section| gimli::EndianSlice::new(&section, gimli::LittleEndian),
    gimli::EndianSlice::new(&[], gimli::LittleEndian),
)?;
unreachable!()
}
```

#### Fields

- **`cu_index`**: `crate::read::DebugCuIndex<T>`

  The `.debug_cu_index` section.

- **`tu_index`**: `crate::read::DebugTuIndex<T>`

  The `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

#### Implementations

- <span id="dwarfpackagesections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the `.dwp` sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfpackagesections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F, empty: R) -> Result<DwarfPackage<R>>` — [`Result`](../index.md#result), [`DwarfPackage`](#dwarfpackage)

  Create a `DwarfPackage` structure that references the data in `self`.

#### Trait Implementations

##### `impl<T> Any for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-default"></span>`fn default() -> DwarfPackageSections<T>` — [`DwarfPackageSections`](#dwarfpackagesections)

##### `impl<T> From for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwarfpackagesections-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwarfpackagesections-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DwarfPackage<R: Reader>`

```rust
struct DwarfPackage<R: Reader> {
    pub cu_index: crate::read::UnitIndex<R>,
    pub tu_index: crate::read::UnitIndex<R>,
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_loc: crate::read::DebugLoc<R>,
    pub debug_loclists: crate::read::DebugLocLists<R>,
    pub debug_rnglists: crate::read::DebugRngLists<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub empty: R,
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:886-928`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L886-L928)*

The sections from a `.dwp` file, with parsed indices.

#### Fields

- **`cu_index`**: `crate::read::UnitIndex<R>`

  The compilation unit index in the `.debug_cu_index` section.

- **`tu_index`**: `crate::read::UnitIndex<R>`

  The type unit index in the `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<R>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<R>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<R>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`empty`**: `R`

  An empty section.
  
  Used when creating `Dwarf<R>`.

#### Implementations

- <span id="dwarfpackage-load"></span>`fn load<F, E>(section: F, empty: R) -> core::result::Result<Self, E>`

  Try to load the `.dwp` sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfpackage-from-sections"></span>`fn from_sections(sections: DwarfPackageSections<R>, empty: R) -> Result<Self>` — [`DwarfPackageSections`](#dwarfpackagesections), [`Result`](../index.md#result)

  Create a `DwarfPackage` structure from the given sections.

- <span id="dwarfpackage-find-cu"></span>`fn find_cu(&self, id: DwoId, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DwoId`](../index.md#dwoid), [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Find the compilation unit with the given DWO identifier and return its section

  contributions.

  

  ## Example Usage

  

  ```rust,no_run

  fn example<R: gimli::Reader>(

         dwarf: &gimli::Dwarf<R>,

         dwp: &gimli::DwarfPackage<R>,

         dwo_id: gimli::DwoId,

  ) -> Result<(), gimli::Error> {

  if let Some(dwo) = dwp.find_cu(dwo_id, dwarf)? {

     let dwo_header = dwo.units().next()?.expect("DWO should have one unit");

     let dwo_unit = dwo.unit(dwo_header)?;

     // Do something with `dwo_unit`.

  }

  unreachable!()

  }

- <span id="dwarfpackage-find-tu"></span>`fn find_tu(&self, signature: DebugTypeSignature, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DebugTypeSignature`](../index.md#debugtypesignature), [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Find the type unit with the given type signature and return its section

  contributions.

- <span id="dwarfpackage-cu-sections"></span>`fn cu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Return the section contributions of the compilation unit at the given index.

  

  The index must be in the range `1..cu_index.unit_count`.

  

  This function should only be needed by low level parsers.

- <span id="dwarfpackage-tu-sections"></span>`fn tu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Return the section contributions of the compilation unit at the given index.

  

  The index must be in the range `1..tu_index.unit_count`.

  

  This function should only be needed by low level parsers.

- <span id="dwarfpackage-sections"></span>`fn sections(&self, sections: UnitIndexSectionIterator<'_, R>, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`UnitIndexSectionIterator`](#unitindexsectioniterator), [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Return the section contributions of a unit.

  

  This function should only be needed by low level parsers.

#### Trait Implementations

##### `impl Any for DwarfPackage<R>`

- <span id="dwarfpackage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DwarfPackage<R>`

- <span id="dwarfpackage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DwarfPackage<R>`

- <span id="dwarfpackage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for DwarfPackage<R>`

- <span id="dwarfpackage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DwarfPackage<R>`

- <span id="dwarfpackage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DwarfPackage<R>`

- <span id="dwarfpackage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DwarfPackage<R>`

- <span id="dwarfpackage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dwarfpackage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DwarfPackage<R>`

- <span id="dwarfpackage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dwarfpackage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unit<R, Offset>`

```rust
struct Unit<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub header: crate::read::UnitHeader<R, Offset>,
    pub abbreviations: alloc::sync::Arc<crate::read::Abbreviations>,
    pub name: Option<R>,
    pub comp_dir: Option<R>,
    pub low_pc: u64,
    pub str_offsets_base: crate::common::DebugStrOffsetsBase<Offset>,
    pub addr_base: crate::common::DebugAddrBase<Offset>,
    pub loclists_base: crate::common::DebugLocListsBase<Offset>,
    pub rnglists_base: crate::common::DebugRngListsBase<Offset>,
    pub line_program: Option<crate::read::IncompleteLineProgram<R, Offset>>,
    pub dwo_id: Option<crate::common::DwoId>,
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:1133-1170`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L1133-L1170)*

All of the commonly used information for a unit in the `.debug_info` or `.debug_types`
sections.

#### Fields

- **`header`**: `crate::read::UnitHeader<R, Offset>`

  The header of the unit.

- **`abbreviations`**: `alloc::sync::Arc<crate::read::Abbreviations>`

  The parsed abbreviations for the unit.

- **`name`**: `Option<R>`

  The `DW_AT_name` attribute of the unit.

- **`comp_dir`**: `Option<R>`

  The `DW_AT_comp_dir` attribute of the unit.

- **`low_pc`**: `u64`

  The `DW_AT_low_pc` attribute of the unit. Defaults to 0.

- **`str_offsets_base`**: `crate::common::DebugStrOffsetsBase<Offset>`

  The `DW_AT_str_offsets_base` attribute of the unit. Defaults to 0.

- **`addr_base`**: `crate::common::DebugAddrBase<Offset>`

  The `DW_AT_addr_base` attribute of the unit. Defaults to 0.

- **`loclists_base`**: `crate::common::DebugLocListsBase<Offset>`

  The `DW_AT_loclists_base` attribute of the unit. Defaults to 0.

- **`rnglists_base`**: `crate::common::DebugRngListsBase<Offset>`

  The `DW_AT_rnglists_base` attribute of the unit. Defaults to 0.

- **`line_program`**: `Option<crate::read::IncompleteLineProgram<R, Offset>>`

  The line number program of the unit.

- **`dwo_id`**: `Option<crate::common::DwoId>`

  The DWO ID of a skeleton unit or split compilation unit.

#### Implementations

- <span id="unit-new"></span>`fn new(dwarf: &Dwarf<R>, header: UnitHeader<R>) -> Result<Self>` — [`Dwarf`](#dwarf), [`UnitHeader`](#unitheader), [`Result`](../index.md#result)

  Construct a new `Unit` from the given unit header.

- <span id="unit-new-with-abbreviations"></span>`fn new_with_abbreviations(dwarf: &Dwarf<R>, header: UnitHeader<R>, abbreviations: Arc<Abbreviations>) -> Result<Self>` — [`Dwarf`](#dwarf), [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations), [`Result`](../index.md#result)

  Construct a new `Unit` from the given unit header and abbreviations.

  

  The abbreviations for this call can be obtained using `dwarf.abbreviations(&header)`.

  The caller may implement caching to reuse the `Abbreviations` across units with the

  same `header.debug_abbrev_offset()` value.

- <span id="unit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, dwarf: &'a Dwarf<R>) -> UnitRef<'a, R>` — [`Dwarf`](#dwarf), [`UnitRef`](#unitref)

  Return a reference to this unit and its associated `Dwarf`.

- <span id="unit-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this unit.

- <span id="unit-entry"></span>`fn entry(&self, offset: UnitOffset<<R as >::Offset>) -> Result<DebuggingInformationEntry<'_, '_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`DebuggingInformationEntry`](#debugginginformationentry)

  Read the `DebuggingInformationEntry` at the given offset.

- <span id="unit-entries"></span>`fn entries(&self) -> EntriesCursor<'_, '_, R>` — [`EntriesCursor`](#entriescursor)

  Navigate this unit's `DebuggingInformationEntry`s.

- <span id="unit-entries-at-offset"></span>`fn entries_at_offset(&self, offset: UnitOffset<<R as >::Offset>) -> Result<EntriesCursor<'_, '_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`EntriesCursor`](#entriescursor)

  Navigate this unit's `DebuggingInformationEntry`s

  starting at the given offset.

- <span id="unit-entries-tree"></span>`fn entries_tree(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesTree<'_, '_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`EntriesTree`](#entriestree)

  Navigate this unit's `DebuggingInformationEntry`s as a tree

  starting at the given offset.

- <span id="unit-entries-raw"></span>`fn entries_raw(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesRaw<'_, '_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`EntriesRaw`](#entriesraw)

  Read the raw data that defines the Debugging Information Entries.

- <span id="unit-copy-relocated-attributes"></span>`fn copy_relocated_attributes(&mut self, other: &Unit<R>)` — [`Unit`](#unit)

  Copy attributes that are subject to relocation from another unit. This is intended

  to be used to copy attributes from a skeleton compilation unit to the corresponding

  split compilation unit.

- <span id="unit-dwo-name"></span>`fn dwo_name(&self) -> Result<Option<AttributeValue<R>>>` — [`Result`](../index.md#result), [`AttributeValue`](#attributevalue)

  Find the dwo name (if any) for this unit, automatically handling the differences

  between the standardized DWARF 5 split DWARF format and the pre-DWARF 5 GNU

  extension.

  

  The returned value is relative to this unit's `comp_dir`.

#### Trait Implementations

##### `impl Any for Unit<R, Offset>`

- <span id="unit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unit<R, Offset>`

- <span id="unit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unit<R, Offset>`

- <span id="unit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Debug for Unit<R, Offset>`

- <span id="unit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Unit<R, Offset>`

- <span id="unit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unit<R, Offset>`

- <span id="unit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Unit<R, Offset>`

- <span id="unit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unit<R, Offset>`

- <span id="unit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitRef<'a, R: Reader>`

```rust
struct UnitRef<'a, R: Reader> {
    pub dwarf: &'a Dwarf<R>,
    pub unit: &'a Unit<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:1389-1395`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L1389-L1395)*

A reference to a `Unit` and its associated `Dwarf`.

These often need to be passed around together, so this struct makes that easier.

It implements `Deref` to `Unit`, so you can use it as if it were a `Unit`.
It also implements methods that correspond to methods on `Dwarf` that take a `Unit`.

#### Fields

- **`dwarf`**: `&'a Dwarf<R>`

  The `Dwarf` that contains the unit.

- **`unit`**: `&'a Unit<R>`

  The `Unit` being referenced.

#### Implementations

- <span id="unitref-new"></span>`fn new(dwarf: &'a Dwarf<R>, unit: &'a Unit<R>) -> Self` — [`Dwarf`](#dwarf), [`Unit`](#unit)

  Construct a new `UnitRef` from a `Dwarf` and a `Unit`.

- <span id="unitref-string-offset"></span>`fn string_offset(&self, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex), [`Reader`](#reader), [`Result`](../index.md#result), [`DebugStrOffset`](../index.md#debugstroffset)

  Return the string offset at the given index.

- <span id="unitref-string"></span>`fn string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../index.md#debugstroffset), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the string at the given offset in `.debug_str`.

- <span id="unitref-line-string"></span>`fn line_string(&self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugLineStrOffset`](../index.md#debuglinestroffset), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the string at the given offset in `.debug_line_str`.

- <span id="unitref-sup-string"></span>`fn sup_string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../index.md#debugstroffset), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the string at the given offset in the `.debug_str`

  in the supplementary object file.

- <span id="unitref-attr-string"></span>`fn attr_string(&self, attr: AttributeValue<R>) -> Result<R>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result)

  Return an attribute value as a string slice.

  

  See `Dwarf::attr_string` for more information.

- <span id="unitref-address"></span>`fn address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md#debugaddrindex), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the address at the given index.

- <span id="unitref-attr-address"></span>`fn attr_address(&self, attr: AttributeValue<R>) -> Result<Option<u64>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result)

  Try to return an attribute value as an address.

  

  See `Dwarf::attr_address` for more information.

- <span id="unitref-ranges-offset-from-raw"></span>`fn ranges_offset_from_raw(&self, offset: RawRangeListsOffset<<R as >::Offset>) -> RangeListsOffset<<R as >::Offset>` — [`RawRangeListsOffset`](../index.md#rawrangelistsoffset), [`Reader`](#reader), [`RangeListsOffset`](../index.md#rangelistsoffset)

  Return the range list offset for the given raw offset.

  

  This handles adding `DW_AT_GNU_ranges_base` if required.

- <span id="unitref-ranges-offset"></span>`fn ranges_offset(&self, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`DebugRngListsIndex`](../index.md#debugrnglistsindex), [`Reader`](#reader), [`Result`](../index.md#result), [`RangeListsOffset`](../index.md#rangelistsoffset)

  Return the range list offset at the given index.

- <span id="unitref-ranges"></span>`fn ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../index.md#rangelistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`RngListIter`](#rnglistiter)

  Iterate over the `RangeListEntry`s starting at the given offset.

- <span id="unitref-raw-ranges"></span>`fn raw_ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../index.md#rangelistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`RawRngListIter`](#rawrnglistiter)

  Iterate over the `RawRngListEntry`ies starting at the given offset.

- <span id="unitref-attr-ranges-offset"></span>`fn attr_ranges_offset(&self, attr: AttributeValue<R>) -> Result<Option<RangeListsOffset<<R as >::Offset>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`RangeListsOffset`](../index.md#rangelistsoffset), [`Reader`](#reader)

  Try to return an attribute value as a range list offset.

  

  See `Dwarf::attr_ranges_offset` for more information.

- <span id="unitref-attr-ranges"></span>`fn attr_ranges(&self, attr: AttributeValue<R>) -> Result<Option<RngListIter<R>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`RngListIter`](#rnglistiter)

  Try to return an attribute value as a range list entry iterator.

  

  See `Dwarf::attr_ranges` for more information.

- <span id="unitref-die-ranges"></span>`fn die_ranges(&self, entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<RangeIter<R>>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md#result), [`RangeIter`](#rangeiter)

  Return an iterator for the address ranges of a `DebuggingInformationEntry`.

  

  This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges`.

- <span id="unitref-unit-ranges"></span>`fn unit_ranges(&self) -> Result<RangeIter<R>>` — [`Result`](../index.md#result), [`RangeIter`](#rangeiter)

  Return an iterator for the address ranges of the `Unit`.

  

  This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges` of the

  root `DebuggingInformationEntry`.

- <span id="unitref-locations-offset"></span>`fn locations_offset(&self, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` — [`DebugLocListsIndex`](../index.md#debugloclistsindex), [`Reader`](#reader), [`Result`](../index.md#result), [`LocationListsOffset`](../index.md#locationlistsoffset)

  Return the location list offset at the given index.

- <span id="unitref-locations"></span>`fn locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../index.md#locationlistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`LocListIter`](#loclistiter)

  Iterate over the `LocationListEntry`s starting at the given offset.

- <span id="unitref-raw-locations"></span>`fn raw_locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../index.md#locationlistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`RawLocListIter`](#rawloclistiter)

  Iterate over the raw `LocationListEntry`s starting at the given offset.

- <span id="unitref-attr-locations-offset"></span>`fn attr_locations_offset(&self, attr: AttributeValue<R>) -> Result<Option<LocationListsOffset<<R as >::Offset>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`LocationListsOffset`](../index.md#locationlistsoffset), [`Reader`](#reader)

  Try to return an attribute value as a location list offset.

  

  See `Dwarf::attr_locations_offset` for more information.

- <span id="unitref-attr-locations"></span>`fn attr_locations(&self, attr: AttributeValue<R>) -> Result<Option<LocListIter<R>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`LocListIter`](#loclistiter)

  Try to return an attribute value as a location list entry iterator.

  

  See `Dwarf::attr_locations` for more information.

- <span id="unitref-macinfo"></span>`fn macinfo(&self, offset: DebugMacinfoOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacinfoOffset`](../index.md#debugmacinfooffset), [`Reader`](#reader), [`Result`](../index.md#result), [`MacroIter`](#macroiter)

  Try to return an iterator for the list of macros at the given `.debug_macinfo` offset.

- <span id="unitref-macros"></span>`fn macros(&self, offset: DebugMacroOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacroOffset`](../index.md#debugmacrooffset), [`Reader`](#reader), [`Result`](../index.md#result), [`MacroIter`](#macroiter)

  Try to return an iterator for the list of macros at the given `.debug_macro` offset.

#### Trait Implementations

##### `impl Any for UnitRef<'a, R>`

- <span id="unitref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitRef<'a, R>`

- <span id="unitref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitRef<'a, R>`

- <span id="unitref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: Reader> Clone for UnitRef<'a, R>`

- <span id="unitref-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UnitRef<'a, R>`

- <span id="unitref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: Reader> Copy for UnitRef<'a, R>`

##### `impl<R: fmt::Debug + Reader> Debug for UnitRef<'a, R>`

- <span id="unitref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Deref for UnitRef<'a, R>`

- <span id="unitref-deref-type-target"></span>`type Target = Unit<R>`

- <span id="unitref-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for UnitRef<'a, R>`

- <span id="unitref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitRef<'a, R>`

- <span id="unitref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for UnitRef<'a, R>`

- <span id="unitref-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for UnitRef<'a, R>`

- <span id="unitref-toowned-type-owned"></span>`type Owned = T`

- <span id="unitref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitRef<'a, R>`

- <span id="unitref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitRef<'a, R>`

- <span id="unitref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeIter<R: Reader>`

```rust
struct RangeIter<R: Reader>(RangeIterInner<R>);
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:1630`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L1630)*

An iterator for the address ranges of a `DebuggingInformationEntry`.

Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`.

#### Implementations

- <span id="rangeiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../index.md#result), [`Range`](#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl Any for RangeIter<R>`

- <span id="rangeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeIter<R>`

- <span id="rangeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeIter<R>`

- <span id="rangeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RangeIter<R>`

- <span id="rangeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Default for RangeIter<R>`

- <span id="rangeiter-default"></span>`fn default() -> Self`

##### `impl<T> From for RangeIter<R>`

- <span id="rangeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeIter<R>`

- <span id="rangeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RangeIter<R>`

- <span id="rangeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeIter<R>`

- <span id="rangeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EndianSlice<'input, Endian>`

```rust
struct EndianSlice<'input, Endian>
where
    Endian: Endianity {
    slice: &'input [u8],
    endian: Endian,
}
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:18-24`](../../../.source_1765633015/gimli-0.32.3/src/read/endian_slice.rs#L18-L24)*

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- <span id="endianslice-new"></span>`fn new(slice: &'input [u8], endian: Endian) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

  Construct a new `EndianSlice` with the given slice and endianity.

- <span id="endianslice-slice"></span>`fn slice(&self) -> &'input [u8]`

  Return a reference to the raw slice.

- <span id="endianslice-split-at"></span>`fn split_at(&self, idx: usize) -> (EndianSlice<'input, Endian>, EndianSlice<'input, Endian>)` — [`EndianSlice`](#endianslice)

  Split the slice in two at the given index, resulting in the tuple where

  the first item has range [0, idx), and the second has range [idx,

  len). Panics if the index is out of bounds.

- <span id="endianslice-find"></span>`fn find(&self, byte: u8) -> Option<usize>`

  Find the first occurrence of a byte in the slice, and return its index.

- <span id="endianslice-offset-from"></span>`fn offset_from(&self, base: EndianSlice<'input, Endian>) -> usize` — [`EndianSlice`](#endianslice)

  Return the offset of the start of the slice relative to the start

  of the given slice.

- <span id="endianslice-to-string"></span>`fn to_string(&self) -> Result<&'input str>` — [`Result`](../index.md#result)

  Converts the slice to a string using `str::from_utf8`.

  

  Returns an error if the slice contains invalid characters.

- <span id="endianslice-to-string-lossy"></span>`fn to_string_lossy(&self) -> Cow<'input, str>`

  Converts the slice to a string, including invalid characters,

  using `String::from_utf8_lossy`.

- <span id="endianslice-read-slice"></span>`fn read_slice(&mut self, len: usize) -> Result<&'input [u8]>` — [`Result`](../index.md#result)

#### Trait Implementations

##### `impl Any for EndianSlice<'input, Endian>`

- <span id="endianslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EndianSlice<'input, Endian>`

- <span id="endianslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EndianSlice<'input, Endian>`

- <span id="endianslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Endian> Clone for EndianSlice<'input, Endian>`

- <span id="endianslice-clone"></span>`fn clone(&self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

##### `impl CloneToUninit for EndianSlice<'input, Endian>`

- <span id="endianslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- <span id="endianslice-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<Endian> Default for EndianSlice<'input, Endian>`

- <span id="endianslice-default"></span>`fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

##### `impl<Endian> Deref for EndianSlice<'input, Endian>`

- <span id="endianslice-deref-type-target"></span>`type Target = [u8]`

- <span id="endianslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<T> From for EndianSlice<'input, Endian>`

- <span id="endianslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<Endian> Hash for EndianSlice<'input, Endian>`

- <span id="endianslice-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for EndianSlice<'input, Endian>`

- <span id="endianslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Endian> PartialEq for EndianSlice<'input, Endian>`

- <span id="endianslice-partialeq-eq"></span>`fn eq(&self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](#endianslice)

##### `impl<Endian> Reader for EndianSlice<'input, Endian>`

- <span id="endianslice-reader-type-endian"></span>`type Endian = Endian`

- <span id="endianslice-reader-type-offset"></span>`type Offset = usize`

- <span id="endianslice-reader-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianslice-reader-len"></span>`fn len(&self) -> usize`

- <span id="endianslice-reader-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="endianslice-reader-empty"></span>`fn empty(&mut self)`

- <span id="endianslice-reader-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> usize`

- <span id="endianslice-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- <span id="endianslice-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- <span id="endianslice-reader-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md#result)

##### `impl Receiver for EndianSlice<'input, Endian>`

- <span id="endianslice-receiver-type-target"></span>`type Target = T`

##### `impl<Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

##### `impl ToOwned for EndianSlice<'input, Endian>`

- <span id="endianslice-toowned-type-owned"></span>`type Owned = T`

- <span id="endianslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="endianslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EndianSlice<'input, Endian>`

- <span id="endianslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="endianslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EndianSlice<'input, Endian>`

- <span id="endianslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="endianslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:190`](../../../.source_1765633015/gimli-0.32.3/src/read/endian_slice.rs#L190)*

#### Trait Implementations

##### `impl Any for DebugBytes<'input>`

- <span id="debugbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugBytes<'input>`

- <span id="debugbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugBytes<'input>`

- <span id="debugbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugBytes<'input>`

- <span id="debugbytes-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<T> From for DebugBytes<'input>`

- <span id="debugbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugBytes<'input>`

- <span id="debugbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugBytes<'input>`

- <span id="debugbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugBytes<'input>`

- <span id="debugbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:203`](../../../.source_1765633015/gimli-0.32.3/src/read/endian_slice.rs#L203)*

#### Trait Implementations

##### `impl Any for DebugByte`

- <span id="debugbyte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugByte`

- <span id="debugbyte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugByte`

- <span id="debugbyte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugByte`

- <span id="debugbyte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugByte`

- <span id="debugbyte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugByte`

- <span id="debugbyte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugbyte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugByte`

- <span id="debugbyte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugbyte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLen`

```rust
struct DebugLen(usize);
```

*Defined in [`gimli-0.32.3/src/read/endian_slice.rs:211`](../../../.source_1765633015/gimli-0.32.3/src/read/endian_slice.rs#L211)*

#### Trait Implementations

##### `impl Any for DebugLen`

- <span id="debuglen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLen`

- <span id="debuglen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLen`

- <span id="debuglen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DebugLen`

- <span id="debuglen-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugLen`

- <span id="debuglen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLen`

- <span id="debuglen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DebugLen`

- <span id="debuglen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLen`

- <span id="debuglen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReaderOffsetId`

```rust
struct ReaderOffsetId(u64);
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:19`](../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L19)*

An identifier for an offset within a section reader.

This is used for error reporting. The meaning of this value is specific to
each reader implementation. The values should be chosen to be unique amongst
all readers. If values are not unique then errors may point to the wrong reader.

#### Trait Implementations

##### `impl Any for ReaderOffsetId`

- <span id="readeroffsetid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReaderOffsetId`

- <span id="readeroffsetid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReaderOffsetId`

- <span id="readeroffsetid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReaderOffsetId`

- <span id="readeroffsetid-clone"></span>`fn clone(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

##### `impl CloneToUninit for ReaderOffsetId`

- <span id="readeroffsetid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ReaderOffsetId`

##### `impl Debug for ReaderOffsetId`

- <span id="readeroffsetid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ReaderOffsetId`

##### `impl<T> From for ReaderOffsetId`

- <span id="readeroffsetid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReaderOffsetId`

- <span id="readeroffsetid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ReaderOffsetId`

- <span id="readeroffsetid-partialeq-eq"></span>`fn eq(&self, other: &ReaderOffsetId) -> bool` — [`ReaderOffsetId`](#readeroffsetid)

##### `impl StructuralPartialEq for ReaderOffsetId`

##### `impl ToOwned for ReaderOffsetId`

- <span id="readeroffsetid-toowned-type-owned"></span>`type Owned = T`

- <span id="readeroffsetid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="readeroffsetid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReaderOffsetId`

- <span id="readeroffsetid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readeroffsetid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReaderOffsetId`

- <span id="readeroffsetid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readeroffsetid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>>`

```rust
struct RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>> {
    section: R,
    reader: R,
    relocate: T,
}
```

*Defined in [`gimli-0.32.3/src/read/relocate.rs:23-27`](../../../.source_1765633015/gimli-0.32.3/src/read/relocate.rs#L23-L27)*

A `Reader` which applies relocations to addresses and offsets.

This is useful for reading sections which contain relocations,
such as those in a relocatable object file.
It is generally not used for reading sections in an executable file.

#### Implementations

- <span id="relocatereader-new"></span>`fn new(section: R, relocate: T) -> Self`

  Create a new `RelocateReader` which applies relocations to the given section reader.

#### Trait Implementations

##### `impl<T> Any for RelocateReader<R, T>`

- <span id="relocatereader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocateReader<R, T>`

- <span id="relocatereader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocateReader<R, T>`

- <span id="relocatereader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader<Offset = usize>, T: clone::Clone + Relocate<<R as >::Offset>> Clone for RelocateReader<R, T>`

- <span id="relocatereader-clone"></span>`fn clone(&self) -> RelocateReader<R, T>` — [`RelocateReader`](#relocatereader)

##### `impl<T> CloneToUninit for RelocateReader<R, T>`

- <span id="relocatereader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader<Offset = usize>, T: fmt::Debug + Relocate<<R as >::Offset>> Debug for RelocateReader<R, T>`

- <span id="relocatereader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RelocateReader<R, T>`

- <span id="relocatereader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RelocateReader<R, T>`

- <span id="relocatereader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, T> Reader for RelocateReader<R, T>`

- <span id="relocatereader-reader-type-endian"></span>`type Endian = <R as Reader>::Endian`

- <span id="relocatereader-reader-type-offset"></span>`type Offset = <R as Reader>::Offset`

- <span id="relocatereader-reader-read-address"></span>`fn read_address(&mut self, address_size: u8) -> Result<u64>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-read-offset"></span>`fn read_offset(&mut self, format: Format) -> Result<<R as >::Offset>` — [`Format`](../index.md#format), [`Result`](../index.md#result), [`Reader`](#reader)

- <span id="relocatereader-reader-read-sized-offset"></span>`fn read_sized_offset(&mut self, size: u8) -> Result<<R as >::Offset>` — [`Result`](../index.md#result), [`Reader`](#reader)

- <span id="relocatereader-reader-split"></span>`fn split(&mut self, len: <Self as >::Offset) -> Result<Self>` — [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="relocatereader-reader-endian"></span>`fn endian(&self) -> <Self as >::Endian` — [`Reader`](#reader)

- <span id="relocatereader-reader-len"></span>`fn len(&self) -> <Self as >::Offset` — [`Reader`](#reader)

- <span id="relocatereader-reader-empty"></span>`fn empty(&mut self)`

- <span id="relocatereader-reader-truncate"></span>`fn truncate(&mut self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="relocatereader-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> <Self as >::Offset` — [`Reader`](#reader)

- <span id="relocatereader-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- <span id="relocatereader-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- <span id="relocatereader-reader-find"></span>`fn find(&self, byte: u8) -> Result<<Self as >::Offset>` — [`Result`](../index.md#result), [`Reader`](#reader)

- <span id="relocatereader-reader-skip"></span>`fn skip(&mut self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="relocatereader-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md#result)

##### `impl<T> ToOwned for RelocateReader<R, T>`

- <span id="relocatereader-toowned-type-owned"></span>`type Owned = T`

- <span id="relocatereader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocatereader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RelocateReader<R, T>`

- <span id="relocatereader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocatereader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RelocateReader<R, T>`

- <span id="relocatereader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocatereader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugAbbrev<R>`

```rust
struct DebugAbbrev<R> {
    debug_abbrev_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:22-24`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L22-L24)*

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

- <span id="debugabbrev-clone"></span>`fn clone(&self) -> DebugAbbrev<R>` — [`DebugAbbrev`](#debugabbrev)

##### `impl CloneToUninit for DebugAbbrev<R>`

- <span id="debugabbrev-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: fmt::Debug> Debug for DebugAbbrev<R>`

- <span id="debugabbrev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAbbrev<R>`

- <span id="debugabbrev-default"></span>`fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](#debugabbrev)

##### `impl<T> From for DebugAbbrev<R>`

- <span id="debugabbrev-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugAbbrev<R>`

- <span id="debugabbrev-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugAbbrev<R>`

- <span id="debugabbrev-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

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

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:112-114`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L112-L114)*

A cache of previously parsed `Abbreviations`.

#### Implementations

- <span id="abbreviationscache-new"></span>`fn new() -> Self`

  Create an empty abbreviations cache.

- <span id="abbreviationscache-populate"></span>`fn populate<R: Reader>(&mut self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy), [`DebugAbbrev`](#debugabbrev), [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

  Parse abbreviations and store them in the cache.

  

  This will iterate over the given units to determine the abbreviations

  offsets. Any existing cache entries are discarded.

  

  Errors during parsing abbreviations are also stored in the cache.

  Errors during iterating over the units are ignored.

- <span id="abbreviationscache-set"></span>`fn set<R: Reader>(&mut self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../index.md#debugabbrevoffset), [`Reader`](#reader), [`Abbreviations`](#abbreviations)

  Set an entry in the abbreviations cache.

  

  This is only required if you want to manually populate the cache.

- <span id="abbreviationscache-get"></span>`fn get<R: Reader>(&self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](#debugabbrev), [`DebugAbbrevOffset`](../index.md#debugabbrevoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`Abbreviations`](#abbreviations)

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

- <span id="abbreviationscache-default"></span>`fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](#abbreviationscache)

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

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:206-209`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L206-L209)*

A set of type abbreviations.

Construct an `Abbreviations` instance with the
[`abbreviations()`](#unitheader-abbreviations)
method.

#### Implementations

- <span id="abbreviations-empty"></span>`fn empty() -> Abbreviations` — [`Abbreviations`](#abbreviations)

  Construct a new, empty set of abbreviations.

- <span id="abbreviations-insert"></span>`fn insert(&mut self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](#abbreviation)

  Insert an abbreviation into the set.

  

  Returns `Ok` if it is the first abbreviation in the set with its code,

  `Err` if the code is a duplicate and there already exists an

  abbreviation in the set with the given abbreviation's code.

- <span id="abbreviations-get"></span>`fn get(&self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](#abbreviation)

  Get the abbreviation associated with the given code.

- <span id="abbreviations-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../index.md#result), [`Abbreviations`](#abbreviations)

  Parse a series of abbreviations, terminated by a null abbreviation.

#### Trait Implementations

##### `impl Any for Abbreviations`

- <span id="abbreviations-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abbreviations`

- <span id="abbreviations-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abbreviations`

- <span id="abbreviations-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Abbreviations`

- <span id="abbreviations-clone"></span>`fn clone(&self) -> Abbreviations` — [`Abbreviations`](#abbreviations)

##### `impl CloneToUninit for Abbreviations`

- <span id="abbreviations-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Abbreviations`

- <span id="abbreviations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abbreviations`

- <span id="abbreviations-default"></span>`fn default() -> Abbreviations` — [`Abbreviations`](#abbreviations)

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

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:282-287`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L282-L287)*

An abbreviation describes the shape of a `DebuggingInformationEntry`'s type:
its code, tag type, whether it has children, and its set of attributes.

#### Implementations

- <span id="abbreviation-new"></span>`fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../index.md#dwtag), [`DwChildren`](../index.md#dwchildren), [`Attributes`](abbrev/index.md#attributes), [`Abbreviation`](#abbreviation)

  Construct a new `Abbreviation`.

  

  ### Panics

  

  Panics if `code` is `0`.

- <span id="abbreviation-code"></span>`fn code(&self) -> u64`

  Get this abbreviation's code.

- <span id="abbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../index.md#dwtag)

  Get this abbreviation's tag.

- <span id="abbreviation-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this abbreviation's type has children, false otherwise.

- <span id="abbreviation-attributes"></span>`fn attributes(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](#attributespecification)

  Get this abbreviation's attributes.

- <span id="abbreviation-parse-tag"></span>`fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../index.md#result), [`DwTag`](../index.md#dwtag)

  Parse an abbreviation's tag.

- <span id="abbreviation-parse-has-children"></span>`fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../index.md#result), [`DwChildren`](../index.md#dwchildren)

  Parse an abbreviation's "does the type have children?" byte.

- <span id="abbreviation-parse-attributes"></span>`fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../index.md#result), [`Attributes`](abbrev/index.md#attributes)

  Parse a series of attribute specifications, terminated by a null attribute

  specification.

- <span id="abbreviation-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../index.md#result), [`Abbreviation`](#abbreviation)

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

- <span id="abbreviation-clone"></span>`fn clone(&self) -> Abbreviation` — [`Abbreviation`](#abbreviation)

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

- <span id="abbreviation-partialeq-eq"></span>`fn eq(&self, other: &Abbreviation) -> bool` — [`Abbreviation`](#abbreviation)

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

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:479-483`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L479-L483)*

The description of an attribute in an abbreviated type. It is a pair of name
and form.

#### Implementations

- <span id="attributespecification-new"></span>`fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../index.md#dwat), [`DwForm`](../index.md#dwform), [`AttributeSpecification`](#attributespecification)

  Construct a new `AttributeSpecification` from the given name and form

  and implicit const value.

- <span id="attributespecification-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../index.md#dwat)

  Get the attribute's name.

- <span id="attributespecification-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../index.md#dwform)

  Get the attribute's form.

- <span id="attributespecification-implicit-const-value"></span>`fn implicit_const_value(&self) -> Option<i64>`

  Get the attribute's implicit const value.

- <span id="attributespecification-size"></span>`fn size<R: Reader>(&self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](#unitheader)

  Return the size of the attribute, in bytes.

  

  Note that because some attributes are variably sized, the size cannot

  always be known without parsing, in which case we return `None`.

- <span id="attributespecification-parse-form"></span>`fn parse_form<R: Reader>(input: &mut R) -> Result<constants::DwForm>` — [`Result`](../index.md#result), [`DwForm`](../index.md#dwform)

  Parse an attribute's form.

- <span id="attributespecification-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../index.md#result), [`AttributeSpecification`](#attributespecification)

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

- <span id="attributespecification-clone"></span>`fn clone(&self) -> AttributeSpecification` — [`AttributeSpecification`](#attributespecification)

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

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

##### `impl<U> Into for AttributeSpecification`

- <span id="attributespecification-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AttributeSpecification`

- <span id="attributespecification-partialeq-eq"></span>`fn eq(&self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](#attributespecification)

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

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:10-12`](../../../.source_1765633015/gimli-0.32.3/src/read/aranges.rs#L10-L12)*

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- <span id="debugaranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugAranges` instance from the data in the `.debug_aranges`

  section.

  

  It is the caller's responsibility to read the `.debug_aranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugAranges, LittleEndian};

  

  let buf = [];

  let read_debug_aranges_section = || &buf;

  let debug_aranges =

      DebugAranges::new(read_debug_aranges_section(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugAranges<R>`

- <span id="debugaranges-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugAranges<R>`

- <span id="debugaranges-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugAranges<R>`

- <span id="debugaranges-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugAranges<R>`

- <span id="debugaranges-clone"></span>`fn clone(&self) -> DebugAranges<R>` — [`DebugAranges`](#debugaranges)

##### `impl CloneToUninit for DebugAranges<R>`

- <span id="debugaranges-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: fmt::Debug> Debug for DebugAranges<R>`

- <span id="debugaranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAranges<R>`

- <span id="debugaranges-default"></span>`fn default() -> DebugAranges<R>` — [`DebugAranges`](#debugaranges)

##### `impl<T> From for DebugAranges<R>`

- <span id="debugaranges-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugAranges<R>`

- <span id="debugaranges-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugAranges<R>`

- <span id="debugaranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugaranges-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugAranges<R>`

- <span id="debugaranges-toowned-type-owned"></span>`type Owned = T`

- <span id="debugaranges-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugaranges-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugAranges<R>`

- <span id="debugaranges-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugaranges-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugAranges<R>`

- <span id="debugaranges-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugaranges-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:91-94`](../../../.source_1765633015/gimli-0.32.3/src/read/aranges.rs#L91-L94)*

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- <span id="arangeheaderiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../index.md#result), [`ArangeHeader`](#arangeheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl Any for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clone"></span>`fn clone(&self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](#arangeheaderiter)

##### `impl CloneToUninit for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeheaderiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeheaderiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeheaderiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeheaderiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeHeader<R, Offset>`

```rust
struct ArangeHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugArangesOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    debug_info_offset: crate::common::DebugInfoOffset<Offset>,
    entries: R,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:131-141`](../../../.source_1765633015/gimli-0.32.3/src/read/aranges.rs#L131-L141)*

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- <span id="arangeheader-parse"></span>`fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../index.md#debugarangesoffset), [`Result`](../index.md#result)

- <span id="arangeheader-offset"></span>`fn offset(&self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../index.md#debugarangesoffset)

  Return the offset of this header within the `.debug_aranges` section.

- <span id="arangeheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="arangeheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="arangeheader-debug-info-offset"></span>`fn debug_info_offset(&self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

  Return the offset into the .debug_info section for this set of arange entries.

- <span id="arangeheader-entries"></span>`fn entries(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](#arangeentryiter)

  Return the arange entries in this set.

#### Trait Implementations

##### `impl Any for ArangeHeader<R, Offset>`

- <span id="arangeheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeHeader<R, Offset>`

- <span id="arangeheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeHeader<R, Offset>`

- <span id="arangeheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- <span id="arangeheader-clone"></span>`fn clone(&self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](#arangeheader)

##### `impl CloneToUninit for ArangeHeader<R, Offset>`

- <span id="arangeheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- <span id="arangeheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<T> From for ArangeHeader<R, Offset>`

- <span id="arangeheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeHeader<R, Offset>`

- <span id="arangeheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- <span id="arangeheader-partialeq-eq"></span>`fn eq(&self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](#arangeheader)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

##### `impl ToOwned for ArangeHeader<R, Offset>`

- <span id="arangeheader-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeHeader<R, Offset>`

- <span id="arangeheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeHeader<R, Offset>`

- <span id="arangeheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:239-242`](../../../.source_1765633015/gimli-0.32.3/src/read/aranges.rs#L239-L242)*

An iterator over the aranges from a `.debug_aranges` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="arangeentryiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../index.md#result), [`ArangeEntry`](#arangeentry)

  Advance the iterator and return the next arange.

  

  Returns the newly parsed arange as `Ok(Some(arange))`. Returns `Ok(None)`

  when iteration is complete and all aranges have already been parsed and

  yielded. If an error occurs while parsing the next arange, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

- <span id="arangeentryiter-next-raw"></span>`fn next_raw(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../index.md#result), [`ArangeEntry`](#arangeentry)

  Advance the iterator and return the next arange without validating it.

  

  The returned entry will have `range.end` set to 0.

  This will return tombstone entries as well.

#### Trait Implementations

##### `impl Any for ArangeEntryIter<R>`

- <span id="arangeentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeEntryIter<R>`

- <span id="arangeentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeEntryIter<R>`

- <span id="arangeentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- <span id="arangeentryiter-clone"></span>`fn clone(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](#arangeentryiter)

##### `impl CloneToUninit for ArangeEntryIter<R>`

- <span id="arangeentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- <span id="arangeentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArangeEntryIter<R>`

- <span id="arangeentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeEntryIter<R>`

- <span id="arangeentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ArangeEntryIter<R>`

- <span id="arangeentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeEntryIter<R>`

- <span id="arangeentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeEntryIter<R>`

- <span id="arangeentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/aranges.rs:318-321`](../../../.source_1765633015/gimli-0.32.3/src/read/aranges.rs#L318-L321)*

A single parsed arange.

#### Implementations

- <span id="arangeentry-parse"></span>`fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../index.md#encoding), [`Result`](../index.md#result)

  Parse a single arange. Return `None` for the null arange, `Some` for an actual arange.

- <span id="arangeentry-address"></span>`fn address(&self) -> u64`

  Return the beginning address of this arange.

- <span id="arangeentry-length"></span>`fn length(&self) -> u64`

  Return the length of this arange.

- <span id="arangeentry-range"></span>`fn range(&self) -> Range` — [`Range`](#range)

  Return the range.

#### Trait Implementations

##### `impl Any for ArangeEntry`

- <span id="arangeentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArangeEntry`

- <span id="arangeentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArangeEntry`

- <span id="arangeentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArangeEntry`

- <span id="arangeentry-clone"></span>`fn clone(&self) -> ArangeEntry` — [`ArangeEntry`](#arangeentry)

##### `impl CloneToUninit for ArangeEntry`

- <span id="arangeentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArangeEntry`

- <span id="arangeentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl<T> From for ArangeEntry`

- <span id="arangeentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArangeEntry`

- <span id="arangeentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ArangeEntry`

- <span id="arangeentry-ord-cmp"></span>`fn cmp(&self, other: &ArangeEntry) -> cmp::Ordering` — [`ArangeEntry`](#arangeentry)

##### `impl PartialEq for ArangeEntry`

- <span id="arangeentry-partialeq-eq"></span>`fn eq(&self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](#arangeentry)

##### `impl PartialOrd for ArangeEntry`

- <span id="arangeentry-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArangeEntry) -> option::Option<cmp::Ordering>` — [`ArangeEntry`](#arangeentry)

##### `impl StructuralPartialEq for ArangeEntry`

##### `impl ToOwned for ArangeEntry`

- <span id="arangeentry-toowned-type-owned"></span>`type Owned = T`

- <span id="arangeentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arangeentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArangeEntry`

- <span id="arangeentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arangeentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArangeEntry`

- <span id="arangeentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arangeentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugCuIndex<R>`

```rust
struct DebugCuIndex<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:12-14`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L12-L14)*

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

#### Implementations

- <span id="debugcuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugCuIndex` instance from the data in the `.debug_cu_index`

  section.

#### Trait Implementations

##### `impl Any for DebugCuIndex<R>`

- <span id="debugcuindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugCuIndex<R>`

- <span id="debugcuindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugCuIndex<R>`

- <span id="debugcuindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugCuIndex<R>`

- <span id="debugcuindex-clone"></span>`fn clone(&self) -> DebugCuIndex<R>` — [`DebugCuIndex`](#debugcuindex)

##### `impl CloneToUninit for DebugCuIndex<R>`

- <span id="debugcuindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugCuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugCuIndex<R>`

- <span id="debugcuindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugCuIndex<R>`

- <span id="debugcuindex-default"></span>`fn default() -> DebugCuIndex<R>` — [`DebugCuIndex`](#debugcuindex)

##### `impl<T> From for DebugCuIndex<R>`

- <span id="debugcuindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugCuIndex<R>`

- <span id="debugcuindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugCuIndex<R>`

- <span id="debugcuindex-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugcuindex-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugCuIndex<R>`

- <span id="debugcuindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugcuindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugcuindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugCuIndex<R>`

- <span id="debugcuindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugcuindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugCuIndex<R>`

- <span id="debugcuindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugcuindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugTuIndex<R>`

```rust
struct DebugTuIndex<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:68-70`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L68-L70)*

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

#### Implementations

- <span id="debugtuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugTuIndex` instance from the data in the `.debug_tu_index`

  section.

#### Trait Implementations

##### `impl Any for DebugTuIndex<R>`

- <span id="debugtuindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTuIndex<R>`

- <span id="debugtuindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTuIndex<R>`

- <span id="debugtuindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugTuIndex<R>`

- <span id="debugtuindex-clone"></span>`fn clone(&self) -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

##### `impl CloneToUninit for DebugTuIndex<R>`

- <span id="debugtuindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugTuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugTuIndex<R>`

- <span id="debugtuindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTuIndex<R>`

- <span id="debugtuindex-default"></span>`fn default() -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

##### `impl<T> From for DebugTuIndex<R>`

- <span id="debugtuindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugTuIndex<R>`

- <span id="debugtuindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugTuIndex<R>`

- <span id="debugtuindex-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugtuindex-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugTuIndex<R>`

- <span id="debugtuindex-toowned-type-owned"></span>`type Owned = T`

- <span id="debugtuindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugtuindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugTuIndex<R>`

- <span id="debugtuindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugtuindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugTuIndex<R>`

- <span id="debugtuindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugtuindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitIndex<R: Reader>`

```rust
struct UnitIndex<R: Reader> {
    version: u16,
    section_count: u32,
    unit_count: u32,
    slot_count: u32,
    hash_ids: R,
    hash_rows: R,
    sections: [IndexSectionId; 8],
    offsets: R,
    sizes: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:124-135`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L124-L135)*

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

#### Implementations

- <span id="unitindex-parse"></span>`fn parse(input: R) -> Result<UnitIndex<R>>` — [`Result`](../index.md#result), [`UnitIndex`](#unitindex)

- <span id="unitindex-find"></span>`fn find(&self, id: u64) -> Option<u32>`

  Find `id` in the index hash table, and return the row index.

  

  `id` may be a compilation unit ID if this index is from `.debug_cu_index`,

  or a type signature if this index is from `.debug_tu_index`.

- <span id="unitindex-sections"></span>`fn sections(&self, row: u32) -> Result<UnitIndexSectionIterator<'_, R>>` — [`Result`](../index.md#result), [`UnitIndexSectionIterator`](#unitindexsectioniterator)

  Return the section offsets and sizes for the given row index.

- <span id="unitindex-version"></span>`fn version(&self) -> u16`

  Return the version.

  

  Defaults to 0 for empty sections.

- <span id="unitindex-section-count"></span>`fn section_count(&self) -> u32`

  Return the number of sections.

- <span id="unitindex-unit-count"></span>`fn unit_count(&self) -> u32`

  Return the number of units.

- <span id="unitindex-slot-count"></span>`fn slot_count(&self) -> u32`

  Return the number of slots.

#### Trait Implementations

##### `impl Any for UnitIndex<R>`

- <span id="unitindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitIndex<R>`

- <span id="unitindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitIndex<R>`

- <span id="unitindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for UnitIndex<R>`

- <span id="unitindex-clone"></span>`fn clone(&self) -> UnitIndex<R>` — [`UnitIndex`](#unitindex)

##### `impl CloneToUninit for UnitIndex<R>`

- <span id="unitindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndex<R>`

- <span id="unitindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnitIndex<R>`

- <span id="unitindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitIndex<R>`

- <span id="unitindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for UnitIndex<R>`

- <span id="unitindex-toowned-type-owned"></span>`type Owned = T`

- <span id="unitindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitIndex<R>`

- <span id="unitindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitIndex<R>`

- <span id="unitindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitIndexSectionIterator<'index, R: Reader>`

```rust
struct UnitIndexSectionIterator<'index, R: Reader> {
    sections: slice::Iter<'index, IndexSectionId>,
    offsets: R,
    sizes: R,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:307-311`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L307-L311)*

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

#### Trait Implementations

##### `impl Any for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-clone"></span>`fn clone(&self) -> UnitIndexSectionIterator<'index, R>` — [`UnitIndexSectionIterator`](#unitindexsectioniterator)

##### `impl CloneToUninit for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unitindexsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unitindexsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-iterator-type-item"></span>`type Item = UnitIndexSection`

- <span id="unitindexsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<UnitIndexSection>` — [`UnitIndexSection`](#unitindexsection)

##### `impl ToOwned for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-toowned-type-owned"></span>`type Owned = T`

- <span id="unitindexsectioniterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitindexsectioniterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitindexsectioniterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitindexsectioniterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnitIndexSection`

```rust
struct UnitIndexSection {
    pub section: IndexSectionId,
    pub offset: u32,
    pub size: u32,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:331-338`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L331-L338)*

Information about a unit's contribution to a section in a `.dwp` file.

#### Fields

- **`section`**: `IndexSectionId`

  The section kind.

- **`offset`**: `u32`

  The base offset of the unit's contribution to the section.

- **`size`**: `u32`

  The size of the unit's contribution to the section.

#### Trait Implementations

##### `impl Any for UnitIndexSection`

- <span id="unitindexsection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitIndexSection`

- <span id="unitindexsection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitIndexSection`

- <span id="unitindexsection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for UnitIndexSection`

- <span id="unitindexsection-clone"></span>`fn clone(&self) -> UnitIndexSection` — [`UnitIndexSection`](#unitindexsection)

##### `impl CloneToUninit for UnitIndexSection`

- <span id="unitindexsection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for UnitIndexSection`

##### `impl Debug for UnitIndexSection`

- <span id="unitindexsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnitIndexSection`

##### `impl<T> From for UnitIndexSection`

- <span id="unitindexsection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitIndexSection`

- <span id="unitindexsection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for UnitIndexSection`

- <span id="unitindexsection-partialeq-eq"></span>`fn eq(&self, other: &UnitIndexSection) -> bool` — [`UnitIndexSection`](#unitindexsection)

##### `impl StructuralPartialEq for UnitIndexSection`

##### `impl ToOwned for UnitIndexSection`

- <span id="unitindexsection-toowned-type-owned"></span>`type Owned = T`

- <span id="unitindexsection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitindexsection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitIndexSection`

- <span id="unitindexsection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitindexsection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitIndexSection`

- <span id="unitindexsection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitindexsection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLine<R>`

```rust
struct DebugLine<R> {
    debug_line_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:17-19`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L17-L19)*

The `DebugLine` struct contains the source location to instruction mapping
found in the `.debug_line` section.

#### Implementations

- <span id="debugline-new"></span>`fn new(debug_line_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLine` instance from the data in the `.debug_line`

  section.

  

  It is the caller's responsibility to read the `.debug_line` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLine, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_section_somehow = || &buf;

  let debug_line = DebugLine::new(read_debug_line_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLine<R>`

- <span id="debugline-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLine<R>`

- <span id="debugline-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLine<R>`

- <span id="debugline-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLine<R>`

- <span id="debugline-clone"></span>`fn clone(&self) -> DebugLine<R>` — [`DebugLine`](#debugline)

##### `impl CloneToUninit for DebugLine<R>`

- <span id="debugline-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLine<R>`

##### `impl<R: fmt::Debug> Debug for DebugLine<R>`

- <span id="debugline-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLine<R>`

- <span id="debugline-default"></span>`fn default() -> DebugLine<R>` — [`DebugLine`](#debugline)

##### `impl<T> From for DebugLine<R>`

- <span id="debugline-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLine<R>`

- <span id="debugline-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLine<R>`

- <span id="debugline-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugline-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLine<R>`

- <span id="debugline-toowned-type-owned"></span>`type Owned = T`

- <span id="debugline-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugline-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLine<R>`

- <span id="debugline-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugline-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLine<R>`

- <span id="debugline-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugline-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineRows<R, Program, Offset>`

```rust
struct LineRows<R, Program, Offset>
where
    Program: LineProgram<R, Offset>,
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    program: Program,
    row: LineRow,
    instructions: LineInstructions<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:168-177`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L168-L177)*

Executes a `LineProgram` to iterate over the rows in the matrix of line number information.

"The hypothetical machine used by a consumer of the line number information
to expand the byte-coded instruction stream into a matrix of line number
information." -- Section 6.2.1

#### Implementations

- <span id="linerows-new"></span>`fn new(program: IncompleteLineProgram<R, Offset>) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`IncompleteLineProgram`](#incompletelineprogram), [`LineRows`](#linerows)

- <span id="linerows-resume"></span>`fn resume<'program>(program: &'program CompleteLineProgram<R, Offset>, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`CompleteLineProgram`](#completelineprogram), [`LineSequence`](#linesequence), [`LineRows`](#linerows)

- <span id="linerows-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

  Get a reference to the header for this state machine's line number

  program.

- <span id="linerows-next-row"></span>`fn next_row(&mut self) -> Result<Option<(&LineProgramHeader<R, Offset>, &LineRow)>>` — [`Result`](../index.md#result), [`LineProgramHeader`](#lineprogramheader), [`LineRow`](#linerow)

  Parse and execute the next instructions in the line number program until

  another row in the line number matrix is computed.

  

  The freshly computed row is returned as `Ok(Some((header, row)))`.

  If the matrix is complete, and there are no more new rows in the line

  number matrix, then `Ok(None)` is returned. If there was an error parsing

  an instruction, then `Err(e)` is returned.

  

  Unfortunately, the references mean that this cannot be a

  `FallibleIterator`.

#### Trait Implementations

##### `impl Any for LineRows<R, Program, Offset>`

- <span id="linerows-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineRows<R, Program, Offset>`

- <span id="linerows-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineRows<R, Program, Offset>`

- <span id="linerows-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Program, Offset> Clone for LineRows<R, Program, Offset>`

- <span id="linerows-clone"></span>`fn clone(&self) -> LineRows<R, Program, Offset>` — [`LineRows`](#linerows)

##### `impl CloneToUninit for LineRows<R, Program, Offset>`

- <span id="linerows-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Program, Offset> Debug for LineRows<R, Program, Offset>`

- <span id="linerows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LineRows<R, Program, Offset>`

- <span id="linerows-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineRows<R, Program, Offset>`

- <span id="linerows-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LineRows<R, Program, Offset>`

- <span id="linerows-toowned-type-owned"></span>`type Owned = T`

- <span id="linerows-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linerows-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineRows<R, Program, Offset>`

- <span id="linerows-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linerows-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineRows<R, Program, Offset>`

- <span id="linerows-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linerows-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineInstructions<R: Reader>`

```rust
struct LineInstructions<R: Reader> {
    input: R,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:529-531`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L529-L531)*

An iterator yielding parsed instructions.

See
[`LineProgramHeader::instructions`](./struct.LineProgramHeader.html#method.instructions)
for more details.

#### Implementations

- <span id="lineinstructions-remove-trailing"></span>`fn remove_trailing(&self, other: &LineInstructions<R>) -> Result<LineInstructions<R>>` — [`LineInstructions`](#lineinstructions), [`Result`](../index.md#result)

#### Trait Implementations

##### `impl Any for LineInstructions<R>`

- <span id="lineinstructions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineInstructions<R>`

- <span id="lineinstructions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineInstructions<R>`

- <span id="lineinstructions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for LineInstructions<R>`

- <span id="lineinstructions-clone"></span>`fn clone(&self) -> LineInstructions<R>` — [`LineInstructions`](#lineinstructions)

##### `impl CloneToUninit for LineInstructions<R>`

- <span id="lineinstructions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for LineInstructions<R>`

- <span id="lineinstructions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LineInstructions<R>`

- <span id="lineinstructions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineInstructions<R>`

- <span id="lineinstructions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LineInstructions<R>`

- <span id="lineinstructions-toowned-type-owned"></span>`type Owned = T`

- <span id="lineinstructions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lineinstructions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineInstructions<R>`

- <span id="lineinstructions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lineinstructions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineInstructions<R>`

- <span id="lineinstructions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lineinstructions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineRow`

```rust
struct LineRow {
    tombstone: bool,
    address: u64,
    op_index: core::num::Wrapping<u64>,
    file: u64,
    line: core::num::Wrapping<u64>,
    column: u64,
    is_stmt: bool,
    basic_block: bool,
    end_sequence: bool,
    prologue_end: bool,
    epilogue_begin: bool,
    isa: u64,
    discriminator: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:580-594`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L580-L594)*

A row in the line number program's resulting matrix.

Each row is a copy of the registers of the state machine, as defined in section 6.2.2.

#### Implementations

- <span id="linerow-new"></span>`fn new<R: Reader>(header: &LineProgramHeader<R>) -> Self` — [`LineProgramHeader`](#lineprogramheader)

  Create a line number row in the initial state for the given program.

- <span id="linerow-address"></span>`fn address(&self) -> u64`

  "The program-counter value corresponding to a machine instruction

  generated by the compiler."

- <span id="linerow-op-index"></span>`fn op_index(&self) -> u64`

  > An unsigned integer representing the index of an operation within a VLIW

  > instruction. The index of the first operation is 0. For non-VLIW

  > architectures, this register will always be 0.

  >

  > The address and op_index registers, taken together, form an operation

  > pointer that can reference any individual operation with the

  > instruction stream.

- <span id="linerow-file-index"></span>`fn file_index(&self) -> u64`

  "An unsigned integer indicating the identity of the source file

  corresponding to a machine instruction."

- <span id="linerow-file"></span>`fn file<'header, R: Reader>(&self, header: &'header LineProgramHeader<R>) -> Option<&'header FileEntry<R>>` — [`LineProgramHeader`](#lineprogramheader), [`FileEntry`](#fileentry)

  The source file corresponding to the current machine instruction.

- <span id="linerow-line"></span>`fn line(&self) -> Option<NonZeroU64>`

  "An unsigned integer indicating a source line number. Lines are numbered

  beginning at 1. The compiler may emit the value 0 in cases where an

  instruction cannot be attributed to any source line."

  Line number values of 0 are represented as `None`.

- <span id="linerow-column"></span>`fn column(&self) -> ColumnType` — [`ColumnType`](#columntype)

  "An unsigned integer indicating a column number within a source

  line. Columns are numbered beginning at 1. The value 0 is reserved to

  indicate that a statement begins at the “left edge” of the line."

- <span id="linerow-is-stmt"></span>`fn is_stmt(&self) -> bool`

  "A boolean indicating that the current instruction is a recommended

  breakpoint location. A recommended breakpoint location is intended to

  “represent” a line, a statement and/or a semantically distinct subpart

  of a statement."

- <span id="linerow-basic-block"></span>`fn basic_block(&self) -> bool`

  "A boolean indicating that the current instruction is the beginning of a

  basic block."

- <span id="linerow-end-sequence"></span>`fn end_sequence(&self) -> bool`

  "A boolean indicating that the current address is that of the first byte

  after the end of a sequence of target machine instructions. end_sequence

  terminates a sequence of lines; therefore other information in the same

  row is not meaningful."

- <span id="linerow-prologue-end"></span>`fn prologue_end(&self) -> bool`

  "A boolean indicating that the current address is one (of possibly many)

  where execution should be suspended for an entry breakpoint of a

  function."

- <span id="linerow-epilogue-begin"></span>`fn epilogue_begin(&self) -> bool`

  "A boolean indicating that the current address is one (of possibly many)

  where execution should be suspended for an exit breakpoint of a

  function."

- <span id="linerow-isa"></span>`fn isa(&self) -> u64`

  Tag for the current instruction set architecture.

  

  > An unsigned integer whose value encodes the applicable instruction set

  > architecture for the current instruction.

  >

  > The encoding of instruction sets should be shared by all users of a

  > given architecture. It is recommended that this encoding be defined by

  > the ABI authoring committee for each architecture.

- <span id="linerow-discriminator"></span>`fn discriminator(&self) -> u64`

  "An unsigned integer identifying the block to which the current

  instruction belongs. Discriminator values are assigned arbitrarily by

  the DWARF producer and serve to distinguish among multiple blocks that

  may all be associated with the same source file, line, and column. Where

  only one block exists for a given source position, the discriminator

  value should be zero."

- <span id="linerow-execute"></span>`fn execute<R, Program>(&mut self, instruction: LineInstruction<R>, program: &mut Program) -> Result<bool>` — [`LineInstruction`](#lineinstruction), [`Result`](../index.md#result)

  Execute the given instruction, and return true if a new row in the

  line number matrix needs to be generated.

  

  Unknown opcodes are treated as no-ops.

- <span id="linerow-reset"></span>`fn reset<R: Reader>(&mut self, header: &LineProgramHeader<R>)` — [`LineProgramHeader`](#lineprogramheader)

  Perform any reset that was required after copying the previous row.

- <span id="linerow-apply-line-advance"></span>`fn apply_line_advance(&mut self, line_increment: i64)`

  Step 1 of section 6.2.5.1

- <span id="linerow-apply-operation-advance"></span>`fn apply_operation_advance<R: Reader>(&mut self, operation_advance: u64, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md#result)

  Step 2 of section 6.2.5.1

- <span id="linerow-adjust-opcode"></span>`fn adjust_opcode<R: Reader>(&self, opcode: u8, header: &LineProgramHeader<R>) -> u8` — [`LineProgramHeader`](#lineprogramheader)

- <span id="linerow-exec-special-opcode"></span>`fn exec_special_opcode<R: Reader>(&mut self, opcode: u8, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md#result)

  Section 6.2.5.1

#### Trait Implementations

##### `impl Any for LineRow`

- <span id="linerow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineRow`

- <span id="linerow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineRow`

- <span id="linerow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LineRow`

- <span id="linerow-clone"></span>`fn clone(&self) -> LineRow` — [`LineRow`](#linerow)

##### `impl CloneToUninit for LineRow`

- <span id="linerow-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LineRow`

##### `impl Debug for LineRow`

- <span id="linerow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineRow`

##### `impl<T> From for LineRow`

- <span id="linerow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineRow`

- <span id="linerow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LineRow`

- <span id="linerow-partialeq-eq"></span>`fn eq(&self, other: &LineRow) -> bool` — [`LineRow`](#linerow)

##### `impl StructuralPartialEq for LineRow`

##### `impl ToOwned for LineRow`

- <span id="linerow-toowned-type-owned"></span>`type Owned = T`

- <span id="linerow-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linerow-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineRow`

- <span id="linerow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linerow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineRow`

- <span id="linerow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linerow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineSequence<R: Reader>`

```rust
struct LineSequence<R: Reader> {
    pub start: u64,
    pub end: u64,
    instructions: LineInstructions<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:977-985`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L977-L985)*

A sequence within a line number program.  A sequence, as defined in section
6.2.5 of the standard, is a linear subset of a line number program within
which addresses are monotonically increasing.

#### Fields

- **`start`**: `u64`

  The first address that is covered by this sequence within the line number
  program.

- **`end`**: `u64`

  The first address that is *not* covered by this sequence within the line
  number program.

#### Trait Implementations

##### `impl Any for LineSequence<R>`

- <span id="linesequence-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineSequence<R>`

- <span id="linesequence-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineSequence<R>`

- <span id="linesequence-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for LineSequence<R>`

- <span id="linesequence-clone"></span>`fn clone(&self) -> LineSequence<R>` — [`LineSequence`](#linesequence)

##### `impl CloneToUninit for LineSequence<R>`

- <span id="linesequence-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for LineSequence<R>`

- <span id="linesequence-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LineSequence<R>`

- <span id="linesequence-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineSequence<R>`

- <span id="linesequence-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LineSequence<R>`

- <span id="linesequence-toowned-type-owned"></span>`type Owned = T`

- <span id="linesequence-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="linesequence-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineSequence<R>`

- <span id="linesequence-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linesequence-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineSequence<R>`

- <span id="linesequence-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linesequence-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineProgramHeader<R, Offset>`

```rust
struct LineProgramHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    encoding: crate::common::Encoding,
    offset: crate::common::DebugLineOffset<Offset>,
    unit_length: Offset,
    header_length: Offset,
    line_encoding: crate::common::LineEncoding,
    opcode_base: u8,
    standard_opcode_lengths: R,
    directory_entry_format: alloc::vec::Vec<FileEntryFormat>,
    include_directories: alloc::vec::Vec<crate::read::AttributeValue<R, Offset>>,
    file_name_entry_format: alloc::vec::Vec<FileEntryFormat>,
    file_names: alloc::vec::Vec<FileEntry<R, Offset>>,
    program_buf: R,
    comp_dir: Option<R>,
    comp_file: Option<FileEntry<R, Offset>>,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:996-1047`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L996-L1047)*

A header for a line number program in the `.debug_line` section, as defined
in section 6.2.4 of the standard.

#### Fields

- **`opcode_base`**: `u8`

  "The number assigned to the first special opcode."

- **`standard_opcode_lengths`**: `R`

  "This array specifies the number of LEB128 operands for each of the
  standard opcodes. The first element of the array corresponds to the
  opcode whose value is 1, and the last element corresponds to the opcode
  whose value is `opcode_base - 1`."

- **`directory_entry_format`**: `alloc::vec::Vec<FileEntryFormat>`

  "A sequence of directory entry format descriptions."

- **`include_directories`**: `alloc::vec::Vec<crate::read::AttributeValue<R, Offset>>`

  > Entries in this sequence describe each path that was searched for
  > included source files in this compilation. (The paths include those
  > directories specified explicitly by the user for the compiler to search
  > and those the compiler searches without explicit direction.) Each path
  > entry is either a full path name or is relative to the current directory
  > of the compilation.
  >
  > The last entry is followed by a single null byte.

- **`file_name_entry_format`**: `alloc::vec::Vec<FileEntryFormat>`

  "A sequence of file entry format descriptions."

- **`file_names`**: `alloc::vec::Vec<FileEntry<R, Offset>>`

  "Entries in this sequence describe source files that contribute to the
  line number information for this compilation unit or is used in other
  contexts."

- **`program_buf`**: `R`

  The encoded line program instructions.

- **`comp_dir`**: `Option<R>`

  The current directory of the compilation.

- **`comp_file`**: `Option<FileEntry<R, Offset>>`

  The primary source file.

#### Implementations

- <span id="lineprogramheader-offset"></span>`fn offset(&self) -> DebugLineOffset<<R as >::Offset>` — [`DebugLineOffset`](../index.md#debuglineoffset), [`Reader`](#reader)

  Return the offset of the line number program header in the `.debug_line` section.

- <span id="lineprogramheader-unit-length"></span>`fn unit_length(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Return the length of the line number program and header, not including

  the length of the encoded length itself.

- <span id="lineprogramheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this header's line program.

- <span id="lineprogramheader-version"></span>`fn version(&self) -> u16`

  Get the version of this header's line program.

- <span id="lineprogramheader-header-length"></span>`fn header_length(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Get the length of the encoded line number program header, not including

  the length of the encoded length itself.

- <span id="lineprogramheader-address-size"></span>`fn address_size(&self) -> u8`

  Get the size in bytes of a target machine address.

- <span id="lineprogramheader-format"></span>`fn format(&self) -> Format` — [`Format`](../index.md#format)

  Whether this line program is encoded in 64- or 32-bit DWARF.

- <span id="lineprogramheader-line-encoding"></span>`fn line_encoding(&self) -> LineEncoding` — [`LineEncoding`](../index.md#lineencoding)

  Get the line encoding parameters for this header's line program.

- <span id="lineprogramheader-minimum-instruction-length"></span>`fn minimum_instruction_length(&self) -> u8`

  Get the minimum instruction length any instruction in this header's line

  program may have.

- <span id="lineprogramheader-maximum-operations-per-instruction"></span>`fn maximum_operations_per_instruction(&self) -> u8`

  Get the maximum number of operations each instruction in this header's

  line program may have.

- <span id="lineprogramheader-default-is-stmt"></span>`fn default_is_stmt(&self) -> bool`

  Get the default value of the `is_stmt` register for this header's line

  program.

- <span id="lineprogramheader-line-base"></span>`fn line_base(&self) -> i8`

  Get the line base for this header's line program.

- <span id="lineprogramheader-line-range"></span>`fn line_range(&self) -> u8`

  Get the line range for this header's line program.

- <span id="lineprogramheader-opcode-base"></span>`fn opcode_base(&self) -> u8`

  Get opcode base for this header's line program.

- <span id="lineprogramheader-standard-opcode-lengths"></span>`fn standard_opcode_lengths(&self) -> &R`

  An array of `u8` that specifies the number of LEB128 operands for

  each of the standard opcodes.

- <span id="lineprogramheader-directory-entry-format"></span>`fn directory_entry_format(&self) -> &[FileEntryFormat]` — [`FileEntryFormat`](#fileentryformat)

  Get the format of a directory entry.

- <span id="lineprogramheader-include-directories"></span>`fn include_directories(&self) -> &[AttributeValue<R, Offset>]` — [`AttributeValue`](#attributevalue)

  Get the set of include directories for this header's line program.

  

  For DWARF version <= 4, the compilation's current directory is not included

  in the return value, but is implicitly considered to be in the set per spec.

- <span id="lineprogramheader-directory"></span>`fn directory(&self, directory: u64) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](#attributevalue)

  The include directory with the given directory index.

  

  A directory index of 0 corresponds to the compilation unit directory.

- <span id="lineprogramheader-file-name-entry-format"></span>`fn file_name_entry_format(&self) -> &[FileEntryFormat]` — [`FileEntryFormat`](#fileentryformat)

  Get the format of a file name entry.

- <span id="lineprogramheader-file-has-timestamp"></span>`fn file_has_timestamp(&self) -> bool`

  Return true if the file entries may have valid timestamps.

  

  Only returns false if we definitely know that all timestamp fields

  are invalid.

- <span id="lineprogramheader-file-has-size"></span>`fn file_has_size(&self) -> bool`

  Return true if the file entries may have valid sizes.

  

  Only returns false if we definitely know that all size fields

  are invalid.

- <span id="lineprogramheader-file-has-md5"></span>`fn file_has_md5(&self) -> bool`

  Return true if the file name entry format contains an MD5 field.

- <span id="lineprogramheader-file-has-source"></span>`fn file_has_source(&self) -> bool`

  Return true if the file name entry format contains a source field.

- <span id="lineprogramheader-file-names"></span>`fn file_names(&self) -> &[FileEntry<R, Offset>]` — [`FileEntry`](#fileentry)

  Get the list of source files that appear in this header's line program.

- <span id="lineprogramheader-file"></span>`fn file(&self, file: u64) -> Option<&FileEntry<R, Offset>>` — [`FileEntry`](#fileentry)

  The source file with the given file index.

  

  A file index of 0 corresponds to the compilation unit file.

  Note that a file index of 0 is invalid for DWARF version <= 4,

  but we support it anyway.

- <span id="lineprogramheader-raw-program-buf"></span>`fn raw_program_buf(&self) -> R`

  Get the raw, un-parsed `EndianSlice` containing this header's line number

  program.

  

  ```rust

  fn foo() {

  use gimli::{LineProgramHeader, EndianSlice, NativeEndian};

  

  fn get_line_number_program_header<'a>() -> LineProgramHeader<EndianSlice<'a, NativeEndian>> {

      // Get a line number program header from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let header = get_line_number_program_header();

  let raw_program = header.raw_program_buf();

  println!("The length of the raw program in bytes is {}", raw_program.len());

  }

  ```

- <span id="lineprogramheader-instructions"></span>`fn instructions(&self) -> LineInstructions<R>` — [`LineInstructions`](#lineinstructions)

  Iterate over the instructions in this header's line number program, parsing

  them as we go.

- <span id="lineprogramheader-parse"></span>`fn parse(input: &mut R, offset: DebugLineOffset<Offset>, address_size: u8, comp_dir: Option<R>, comp_name: Option<R>) -> Result<LineProgramHeader<R, Offset>>` — [`DebugLineOffset`](../index.md#debuglineoffset), [`Result`](../index.md#result), [`LineProgramHeader`](#lineprogramheader)

#### Trait Implementations

##### `impl Any for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-clone"></span>`fn clone(&self) -> LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

##### `impl CloneToUninit for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for LineProgramHeader<R, Offset>`

##### `impl<T> From for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-partialeq-eq"></span>`fn eq(&self, other: &LineProgramHeader<R, Offset>) -> bool` — [`LineProgramHeader`](#lineprogramheader)

##### `impl<R, Offset> StructuralPartialEq for LineProgramHeader<R, Offset>`

##### `impl ToOwned for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-toowned-type-owned"></span>`type Owned = T`

- <span id="lineprogramheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lineprogramheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lineprogramheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lineprogramheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IncompleteLineProgram<R, Offset>`

```rust
struct IncompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1411-1417`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1411-L1417)*

A line number program that has not been run to completion.

#### Implementations

- <span id="incompletelineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

  Retrieve the `LineProgramHeader` for this program.

- <span id="incompletelineprogram-rows"></span>`fn rows(self) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`LineRows`](#linerows), [`IncompleteLineProgram`](#incompletelineprogram)

  Construct a new `LineRows` for executing this program to iterate

  over rows in the line information matrix.

- <span id="incompletelineprogram-sequences"></span>`fn sequences(self) -> Result<(CompleteLineProgram<R, Offset>, Vec<LineSequence<R>>)>` — [`Result`](../index.md#result), [`CompleteLineProgram`](#completelineprogram), [`LineSequence`](#linesequence)

  Execute the line number program, completing the `IncompleteLineProgram`

  into a `CompleteLineProgram` and producing an array of sequences within

  the line number program that can later be used with

  `CompleteLineProgram::resume_from`.

  

  ```rust

  fn foo() {

  use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};

  

  fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {

      // Get a line number program from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let program = get_line_number_program();

  let (program, sequences) = program.sequences().unwrap();

  println!("There are {} sequences in this line number program", sequences.len());

  }

  ```

#### Trait Implementations

##### `impl Any for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-clone"></span>`fn clone(&self) -> IncompleteLineProgram<R, Offset>` — [`IncompleteLineProgram`](#incompletelineprogram)

##### `impl CloneToUninit for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for IncompleteLineProgram<R, Offset>`

##### `impl<T> From for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> LineProgram for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-lineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- <span id="incompletelineprogram-lineprogram-add-file"></span>`fn add_file(&mut self, file: FileEntry<R, Offset>)` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> PartialEq for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-partialeq-eq"></span>`fn eq(&self, other: &IncompleteLineProgram<R, Offset>) -> bool` — [`IncompleteLineProgram`](#incompletelineprogram)

##### `impl<R, Offset> StructuralPartialEq for IncompleteLineProgram<R, Offset>`

##### `impl ToOwned for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-toowned-type-owned"></span>`type Owned = T`

- <span id="incompletelineprogram-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="incompletelineprogram-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="incompletelineprogram-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="incompletelineprogram-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CompleteLineProgram<R, Offset>`

```rust
struct CompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1504-1510`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1504-L1510)*

A line number program that has previously been run to completion.

#### Implementations

- <span id="completelineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

  Retrieve the `LineProgramHeader` for this program.

- <span id="completelineprogram-resume-from"></span>`fn resume_from<'program>(self: &'program Self, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`LineSequence`](#linesequence), [`LineRows`](#linerows), [`CompleteLineProgram`](#completelineprogram)

  Construct a new `LineRows` for executing the subset of the line

  number program identified by 'sequence' and  generating the line information

  matrix.

  

  ```rust

  fn foo() {

  use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};

  

  fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {

      // Get a line number program from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let program = get_line_number_program();

  let (program, sequences) = program.sequences().unwrap();

  for sequence in &sequences {

      let mut sm = program.resume_from(sequence);

  }

  }

  ```

#### Trait Implementations

##### `impl Any for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-clone"></span>`fn clone(&self) -> CompleteLineProgram<R, Offset>` — [`CompleteLineProgram`](#completelineprogram)

##### `impl CloneToUninit for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for CompleteLineProgram<R, Offset>`

##### `impl<T> From for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> LineProgram for &'program CompleteLineProgram<R, Offset>`

- <span id="program-completelineprogram-lineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- <span id="program-completelineprogram-lineprogram-add-file"></span>`fn add_file(&mut self, _: FileEntry<R, Offset>)` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> PartialEq for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-partialeq-eq"></span>`fn eq(&self, other: &CompleteLineProgram<R, Offset>) -> bool` — [`CompleteLineProgram`](#completelineprogram)

##### `impl<R, Offset> StructuralPartialEq for CompleteLineProgram<R, Offset>`

##### `impl ToOwned for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-toowned-type-owned"></span>`type Owned = T`

- <span id="completelineprogram-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="completelineprogram-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="completelineprogram-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="completelineprogram-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FileEntry<R, Offset>`

```rust
struct FileEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    path_name: crate::read::AttributeValue<R, Offset>,
    directory_index: u64,
    timestamp: u64,
    size: u64,
    md5: [u8; 16],
    source: Option<crate::read::AttributeValue<R, Offset>>,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1553-1564`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1553-L1564)*

An entry in the `LineProgramHeader`'s `file_names` set.

#### Implementations

- <span id="fileentry-parse"></span>`fn parse(input: &mut R, path_name: R) -> Result<FileEntry<R, Offset>>` — [`Result`](../index.md#result), [`FileEntry`](#fileentry)

- <span id="fileentry-path-name"></span>`fn path_name(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](#attributevalue)

  > A slice containing the full or relative path name of

  > a source file. If the entry contains a file name or a relative path

  > name, the file is located relative to either the compilation directory

  > (as specified by the DW_AT_comp_dir attribute given in the compilation

  > unit) or one of the directories in the include_directories section.

- <span id="fileentry-directory-index"></span>`fn directory_index(&self) -> u64`

  > An unsigned LEB128 number representing the directory index of the

  > directory in which the file was found.

  >

  > ...

  >

  > The directory index represents an entry in the include_directories

  > section of the line number program header. The index is 0 if the file

  > was found in the current directory of the compilation, 1 if it was found

  > in the first directory in the include_directories section, and so

  > on. The directory index is ignored for file names that represent full

  > path names.

- <span id="fileentry-directory"></span>`fn directory(&self, header: &LineProgramHeader<R>) -> Option<AttributeValue<R, Offset>>` — [`LineProgramHeader`](#lineprogramheader), [`AttributeValue`](#attributevalue)

  Get this file's directory.

  

  A directory index of 0 corresponds to the compilation unit directory.

- <span id="fileentry-timestamp"></span>`fn timestamp(&self) -> u64`

  The implementation-defined time of last modification of the file,

  or 0 if not available.

- <span id="fileentry-size"></span>`fn size(&self) -> u64`

  The size of the file in bytes, or 0 if not available.

- <span id="fileentry-md5"></span>`fn md5(&self) -> &[u8; 16]`

  A 16-byte MD5 digest of the file contents.

  

  Only valid if `LineProgramHeader::file_has_md5` returns `true`.

- <span id="fileentry-source"></span>`fn source(&self) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](#attributevalue)

  The source code of this file. (UTF-8 source text string with "\n" line

  endings).

  

  Note: For DWARF v5 files this may return an empty attribute that

  indicates that no source code is available, which this function

  represents as `Some(<zero-length attr>)`.

#### Trait Implementations

##### `impl Any for FileEntry<R, Offset>`

- <span id="fileentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FileEntry<R, Offset>`

- <span id="fileentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FileEntry<R, Offset>`

- <span id="fileentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for FileEntry<R, Offset>`

- <span id="fileentry-clone"></span>`fn clone(&self) -> FileEntry<R, Offset>` — [`FileEntry`](#fileentry)

##### `impl CloneToUninit for FileEntry<R, Offset>`

- <span id="fileentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for FileEntry<R, Offset>`

##### `impl<R, Offset> Debug for FileEntry<R, Offset>`

- <span id="fileentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for FileEntry<R, Offset>`

##### `impl<T> From for FileEntry<R, Offset>`

- <span id="fileentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FileEntry<R, Offset>`

- <span id="fileentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for FileEntry<R, Offset>`

- <span id="fileentry-partialeq-eq"></span>`fn eq(&self, other: &FileEntry<R, Offset>) -> bool` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> StructuralPartialEq for FileEntry<R, Offset>`

##### `impl ToOwned for FileEntry<R, Offset>`

- <span id="fileentry-toowned-type-owned"></span>`type Owned = T`

- <span id="fileentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fileentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FileEntry<R, Offset>`

- <span id="fileentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fileentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FileEntry<R, Offset>`

- <span id="fileentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fileentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FileEntryFormat`

```rust
struct FileEntryFormat {
    pub content_type: constants::DwLnct,
    pub form: constants::DwForm,
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1667-1673`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1667-L1673)*

The format of a component of an include directory or file name entry.

#### Fields

- **`content_type`**: `constants::DwLnct`

  The type of information that is represented by the component.

- **`form`**: `constants::DwForm`

  The encoding form of the component value.

#### Implementations

- <span id="fileentryformat-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Vec<FileEntryFormat>>` — [`Result`](../index.md#result), [`FileEntryFormat`](#fileentryformat)

#### Trait Implementations

##### `impl Any for FileEntryFormat`

- <span id="fileentryformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FileEntryFormat`

- <span id="fileentryformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FileEntryFormat`

- <span id="fileentryformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FileEntryFormat`

- <span id="fileentryformat-clone"></span>`fn clone(&self) -> FileEntryFormat` — [`FileEntryFormat`](#fileentryformat)

##### `impl CloneToUninit for FileEntryFormat`

- <span id="fileentryformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FileEntryFormat`

##### `impl Debug for FileEntryFormat`

- <span id="fileentryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileEntryFormat`

##### `impl<T> From for FileEntryFormat`

- <span id="fileentryformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FileEntryFormat`

- <span id="fileentryformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FileEntryFormat`

- <span id="fileentryformat-partialeq-eq"></span>`fn eq(&self, other: &FileEntryFormat) -> bool` — [`FileEntryFormat`](#fileentryformat)

##### `impl StructuralPartialEq for FileEntryFormat`

##### `impl ToOwned for FileEntryFormat`

- <span id="fileentryformat-toowned-type-owned"></span>`type Owned = T`

- <span id="fileentryformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fileentryformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FileEntryFormat`

- <span id="fileentryformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fileentryformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FileEntryFormat`

- <span id="fileentryformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fileentryformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLoc<R>`

```rust
struct DebugLoc<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:14-16`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L14-L16)*

The raw contents of the `.debug_loc` section.

#### Implementations

- <span id="debugloc-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLoc` instance from the data in the `.debug_loc`

  section.

  

  It is the caller's responsibility to read the `.debug_loc` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLoc, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loc_section_somehow = || &buf;

  let debug_loc = DebugLoc::new(read_debug_loc_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLoc<R>`

- <span id="debugloc-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLoc<R>`

- <span id="debugloc-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLoc<R>`

- <span id="debugloc-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLoc<R>`

- <span id="debugloc-clone"></span>`fn clone(&self) -> DebugLoc<R>` — [`DebugLoc`](#debugloc)

##### `impl CloneToUninit for DebugLoc<R>`

- <span id="debugloc-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: fmt::Debug> Debug for DebugLoc<R>`

- <span id="debugloc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLoc<R>`

- <span id="debugloc-default"></span>`fn default() -> DebugLoc<R>` — [`DebugLoc`](#debugloc)

##### `impl<T> From for DebugLoc<R>`

- <span id="debugloc-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLoc<R>`

- <span id="debugloc-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLoc<R>`

- <span id="debugloc-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugloc-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLoc<R>`

- <span id="debugloc-toowned-type-owned"></span>`type Owned = T`

- <span id="debugloc-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugloc-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLoc<R>`

- <span id="debugloc-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugloc-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLoc<R>`

- <span id="debugloc-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugloc-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLocLists<R>`

```rust
struct DebugLocLists<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:74-76`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L74-L76)*

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

#### Implementations

- <span id="debugloclists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLocLists` instance from the data in the `.debug_loclists`

  section.

  

  It is the caller's responsibility to read the `.debug_loclists` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLocLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loclists_section_somehow = || &buf;

  let debug_loclists = DebugLocLists::new(read_debug_loclists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLocLists<R>`

- <span id="debugloclists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLocLists<R>`

- <span id="debugloclists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLocLists<R>`

- <span id="debugloclists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLocLists<R>`

- <span id="debugloclists-clone"></span>`fn clone(&self) -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

##### `impl CloneToUninit for DebugLocLists<R>`

- <span id="debugloclists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugLocLists<R>`

- <span id="debugloclists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLocLists<R>`

- <span id="debugloclists-default"></span>`fn default() -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

##### `impl<T> From for DebugLocLists<R>`

- <span id="debugloclists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLocLists<R>`

- <span id="debugloclists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLocLists<R>`

- <span id="debugloclists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugloclists-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLocLists<R>`

- <span id="debugloclists-toowned-type-owned"></span>`type Owned = T`

- <span id="debugloclists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugloclists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLocLists<R>`

- <span id="debugloclists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugloclists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLocLists<R>`

- <span id="debugloclists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugloclists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocationLists<R>`

```rust
struct LocationLists<R> {
    debug_loc: DebugLoc<R>,
    debug_loclists: DebugLocLists<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:156-159`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L156-L159)*

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

#### Implementations

- <span id="locationlists-new"></span>`fn new(debug_loc: DebugLoc<R>, debug_loclists: DebugLocLists<R>) -> LocationLists<R>` — [`DebugLoc`](#debugloc), [`DebugLocLists`](#debugloclists), [`LocationLists`](#locationlists)

  Construct a new `LocationLists` instance from the data in the `.debug_loc` and

  `.debug_loclists` sections.

#### Trait Implementations

##### `impl Any for LocationLists<R>`

- <span id="locationlists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocationLists<R>`

- <span id="locationlists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocationLists<R>`

- <span id="locationlists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for LocationLists<R>`

- <span id="locationlists-clone"></span>`fn clone(&self) -> LocationLists<R>` — [`LocationLists`](#locationlists)

##### `impl CloneToUninit for LocationLists<R>`

- <span id="locationlists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for LocationLists<R>`

##### `impl<R: fmt::Debug> Debug for LocationLists<R>`

- <span id="locationlists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for LocationLists<R>`

- <span id="locationlists-default"></span>`fn default() -> LocationLists<R>` — [`LocationLists`](#locationlists)

##### `impl<T> From for LocationLists<R>`

- <span id="locationlists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocationLists<R>`

- <span id="locationlists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LocationLists<R>`

- <span id="locationlists-toowned-type-owned"></span>`type Owned = T`

- <span id="locationlists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="locationlists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocationLists<R>`

- <span id="locationlists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locationlists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocationLists<R>`

- <span id="locationlists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locationlists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawLocListIter<R: Reader>`

```rust
struct RawLocListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: LocListsFormat,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:329-333`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L329-L333)*

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

#### Implementations

- <span id="rawloclistiter-new"></span>`fn new(input: R, encoding: Encoding, format: LocListsFormat) -> RawLocListIter<R>` — [`Encoding`](../index.md#encoding), [`LocListsFormat`](loclists/index.md#loclistsformat), [`RawLocListIter`](#rawloclistiter)

  Construct a `RawLocListIter`.

- <span id="rawloclistiter-next"></span>`fn next(&mut self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../index.md#result), [`RawLocListEntry`](#rawloclistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl Any for RawLocListIter<R>`

- <span id="rawloclistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawLocListIter<R>`

- <span id="rawloclistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawLocListIter<R>`

- <span id="rawloclistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListIter<R>`

- <span id="rawloclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawLocListIter<R>`

- <span id="rawloclistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawLocListIter<R>`

- <span id="rawloclistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawLocListIter<R>`

- <span id="rawloclistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawloclistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawLocListIter<R>`

- <span id="rawloclistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawloclistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocListIter<R: Reader>`

```rust
struct LocListIter<R: Reader> {
    raw: RawLocListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:536-541`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L536-L541)*

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="loclistiter-new"></span>`fn new(raw: RawLocListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> LocListIter<R>` — [`RawLocListIter`](#rawloclistiter), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md#debugaddrbase), [`Reader`](#reader), [`LocListIter`](#loclistiter)

  Construct a `LocListIter`.

- <span id="loclistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md#debugaddrindex), [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="loclistiter-next"></span>`fn next(&mut self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../index.md#result), [`LocationListEntry`](#locationlistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl Any for LocListIter<R>`

- <span id="loclistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocListIter<R>`

- <span id="loclistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocListIter<R>`

- <span id="loclistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for LocListIter<R>`

- <span id="loclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LocListIter<R>`

- <span id="loclistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocListIter<R>`

- <span id="loclistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LocListIter<R>`

- <span id="loclistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loclistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocListIter<R>`

- <span id="loclistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loclistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocationListEntry<R: Reader>`

```rust
struct LocationListEntry<R: Reader> {
    pub range: crate::read::Range,
    pub data: crate::read::Expression<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:679-685`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L679-L685)*

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

#### Fields

- **`range`**: `crate::read::Range`

  The address range that this location is valid for.

- **`data`**: `crate::read::Expression<R>`

  The data containing a single location description.

#### Trait Implementations

##### `impl Any for LocationListEntry<R>`

- <span id="locationlistentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocationListEntry<R>`

- <span id="locationlistentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocationListEntry<R>`

- <span id="locationlistentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for LocationListEntry<R>`

- <span id="locationlistentry-clone"></span>`fn clone(&self) -> LocationListEntry<R>` — [`LocationListEntry`](#locationlistentry)

##### `impl CloneToUninit for LocationListEntry<R>`

- <span id="locationlistentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: fmt::Debug + Reader> Debug for LocationListEntry<R>`

- <span id="locationlistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<T> From for LocationListEntry<R>`

- <span id="locationlistentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<R: hash::Hash + Reader> Hash for LocationListEntry<R>`

- <span id="locationlistentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LocationListEntry<R>`

- <span id="locationlistentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- <span id="locationlistentry-partialeq-eq"></span>`fn eq(&self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](#locationlistentry)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

##### `impl ToOwned for LocationListEntry<R>`

- <span id="locationlistentry-toowned-type-owned"></span>`type Owned = T`

- <span id="locationlistentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="locationlistentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocationListEntry<R>`

- <span id="locationlistentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="locationlistentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocationListEntry<R>`

- <span id="locationlistentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="locationlistentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugMacinfo<R>`

```rust
struct DebugMacinfo<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:11-13`](../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L11-L13)*

The raw contents of the `.debug_macinfo` section.

#### Implementations

- <span id="debugmacinfo-new"></span>`fn new(macinfo_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacinfo` instance from the data in the `.debug_macinfo`

  section.

  

  It is the caller's responsibility to read the `.debug_macinfo` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacinfo, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacinfo::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugMacinfo<R>`

- <span id="debugmacinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugMacinfo<R>`

- <span id="debugmacinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugMacinfo<R>`

- <span id="debugmacinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugMacinfo<R>`

- <span id="debugmacinfo-clone"></span>`fn clone(&self) -> DebugMacinfo<R>` — [`DebugMacinfo`](#debugmacinfo)

##### `impl CloneToUninit for DebugMacinfo<R>`

- <span id="debugmacinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugMacinfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacinfo<R>`

- <span id="debugmacinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacinfo<R>`

- <span id="debugmacinfo-default"></span>`fn default() -> DebugMacinfo<R>` — [`DebugMacinfo`](#debugmacinfo)

##### `impl<T> From for DebugMacinfo<R>`

- <span id="debugmacinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugMacinfo<R>`

- <span id="debugmacinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugMacinfo<R>`

- <span id="debugmacinfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugmacinfo-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugMacinfo<R>`

- <span id="debugmacinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="debugmacinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugmacinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugMacinfo<R>`

- <span id="debugmacinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugmacinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugMacinfo<R>`

- <span id="debugmacinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugmacinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugMacro<R>`

```rust
struct DebugMacro<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:104-106`](../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L104-L106)*

The raw contents of the `.debug_macro` section.

#### Implementations

- <span id="debugmacro-new"></span>`fn new(macro_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacro` instance from the data in the `.debug_macro`

  section.

  

  It is the caller's responsibility to read the `.debug_macro` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacro, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacro::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugMacro<R>`

- <span id="debugmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugMacro<R>`

- <span id="debugmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugMacro<R>`

- <span id="debugmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugMacro<R>`

- <span id="debugmacro-clone"></span>`fn clone(&self) -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

##### `impl CloneToUninit for DebugMacro<R>`

- <span id="debugmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugMacro<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacro<R>`

- <span id="debugmacro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacro<R>`

- <span id="debugmacro-default"></span>`fn default() -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

##### `impl<T> From for DebugMacro<R>`

- <span id="debugmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugMacro<R>`

- <span id="debugmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugMacro<R>`

- <span id="debugmacro-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugmacro-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugMacro<R>`

- <span id="debugmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="debugmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugMacro<R>`

- <span id="debugmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugMacro<R>`

- <span id="debugmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroUnitHeader<R: Reader>`

```rust
struct MacroUnitHeader<R: Reader> {
    _version: u16,
    flags: u8,
    _debug_line_offset: crate::DebugLineOffset<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:197-202`](../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L197-L202)*

#### Fields

- **`_version`**: `u16`

  The version of the macro unit header. At the moment only version 5 is defined.

#### Implementations

- <span id="macrounitheader-const-offset-size-flag"></span>`const OFFSET_SIZE_FLAG: u8`

- <span id="macrounitheader-const-debug-line-offset-flag"></span>`const DEBUG_LINE_OFFSET_FLAG: u8`

- <span id="macrounitheader-const-opcode-operands-table-flag"></span>`const OPCODE_OPERANDS_TABLE_FLAG: u8`

- <span id="macrounitheader-parse"></span>`fn parse(input: &mut R) -> Result<Self>` — [`Result`](../index.md#result)

- <span id="macrounitheader-format"></span>`fn format(&self) -> Format` — [`Format`](../index.md#format)

#### Trait Implementations

##### `impl Any for MacroUnitHeader<R>`

- <span id="macrounitheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroUnitHeader<R>`

- <span id="macrounitheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroUnitHeader<R>`

- <span id="macrounitheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for MacroUnitHeader<R>`

- <span id="macrounitheader-clone"></span>`fn clone(&self) -> MacroUnitHeader<R>` — [`MacroUnitHeader`](macros/index.md#macrounitheader)

##### `impl CloneToUninit for MacroUnitHeader<R>`

- <span id="macrounitheader-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for MacroUnitHeader<R>`

- <span id="macrounitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MacroUnitHeader<R>`

- <span id="macrounitheader-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroUnitHeader<R>`

- <span id="macrounitheader-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MacroUnitHeader<R>`

- <span id="macrounitheader-toowned-type-owned"></span>`type Owned = T`

- <span id="macrounitheader-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macrounitheader-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroUnitHeader<R>`

- <span id="macrounitheader-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macrounitheader-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroUnitHeader<R>`

- <span id="macrounitheader-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macrounitheader-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroIter<R: Reader>`

```rust
struct MacroIter<R: Reader> {
    input: R,
    format: crate::Format,
    is_macro: bool,
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:327-331`](../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L327-L331)*

Iterator over the entries in the `.debug_macro` section.

#### Implementations

- <span id="macroiter-next"></span>`fn next(&mut self) -> Result<Option<MacroEntry<R>>>` — [`Result`](../index.md#result), [`MacroEntry`](#macroentry)

  Advance the iterator to the next entry in the `.debug_macro` section.

#### Trait Implementations

##### `impl Any for MacroIter<R>`

- <span id="macroiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroIter<R>`

- <span id="macroiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroIter<R>`

- <span id="macroiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for MacroIter<R>`

- <span id="macroiter-clone"></span>`fn clone(&self) -> MacroIter<R>` — [`MacroIter`](#macroiter)

##### `impl CloneToUninit for MacroIter<R>`

- <span id="macroiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for MacroIter<R>`

- <span id="macroiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MacroIter<R>`

- <span id="macroiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroIter<R>`

- <span id="macroiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MacroIter<R>`

- <span id="macroiter-toowned-type-owned"></span>`type Owned = T`

- <span id="macroiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macroiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroIter<R>`

- <span id="macroiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macroiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroIter<R>`

- <span id="macroiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macroiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Piece<R, Offset>`

```rust
struct Piece<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub size_in_bits: Option<u64>,
    pub bit_offset: Option<u64>,
    pub location: Location<R, Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:356-378`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L356-L378)*

The description of a single piece of the result of a DWARF
expression.

#### Fields

- **`size_in_bits`**: `Option<u64>`

  If given, the size of the piece in bits.  If `None`, there
  must be only one piece whose size is all of the object.

- **`bit_offset`**: `Option<u64>`

  If given, the bit offset of the piece within the location.
  If the location is a `Location::Register` or `Location::Value`,
  then this offset is from the least significant bit end of
  the register or value.
  If the location is a `Location::Address` then the offset uses
  the bit numbering and direction conventions of the language
  and target system.
  
  If `None`, the piece starts at the location. If the
  location is a register whose size is larger than the piece,
  then placement within the register is defined by the ABI.

- **`location`**: `Location<R, Offset>`

  Where this piece is to be found.

#### Trait Implementations

##### `impl Any for Piece<R, Offset>`

- <span id="piece-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Piece<R, Offset>`

- <span id="piece-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Piece<R, Offset>`

- <span id="piece-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for Piece<R, Offset>`

- <span id="piece-clone"></span>`fn clone(&self) -> Piece<R, Offset>` — [`Piece`](#piece)

##### `impl CloneToUninit for Piece<R, Offset>`

- <span id="piece-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for Piece<R, Offset>`

##### `impl<R, Offset> Debug for Piece<R, Offset>`

- <span id="piece-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Piece<R, Offset>`

- <span id="piece-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Piece<R, Offset>`

- <span id="piece-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for Piece<R, Offset>`

- <span id="piece-partialeq-eq"></span>`fn eq(&self, other: &Piece<R, Offset>) -> bool` — [`Piece`](#piece)

##### `impl<R, Offset> StructuralPartialEq for Piece<R, Offset>`

##### `impl ToOwned for Piece<R, Offset>`

- <span id="piece-toowned-type-owned"></span>`type Owned = T`

- <span id="piece-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="piece-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Piece<R, Offset>`

- <span id="piece-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="piece-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Piece<R, Offset>`

- <span id="piece-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="piece-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Expression<R: Reader>`

```rust
struct Expression<R: Reader>(R);
```

*Defined in [`gimli-0.32.3/src/read/op.rs:924`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L924)*

The bytecode for a DWARF expression or location description.

#### Implementations

- <span id="expression-evaluation"></span>`fn evaluation(self, encoding: Encoding) -> Evaluation<R>` — [`Encoding`](../index.md#encoding), [`Evaluation`](#evaluation)

  Create an evaluation for this expression.

  

  The `encoding` is determined by the

  [`CompilationUnitHeader`](#compilationunitheader) or

  [`TypeUnitHeader`](#typeunitheader) that this expression

  relates to.

  

  # Examples

  ```rust,no_run

  use gimli::Expression;

  let endian = gimli::LittleEndian;

  let debug_info = gimli::DebugInfo::from(gimli::EndianSlice::new(&[], endian));

  let unit = debug_info.units().next().unwrap().unwrap();

  let bytecode = gimli::EndianSlice::new(&[], endian);

  let expression = gimli::Expression(bytecode);

  let mut eval = expression.evaluation(unit.encoding());

  let mut result = eval.evaluate().unwrap();

  ```

- <span id="expression-operations"></span>`fn operations(self, encoding: Encoding) -> OperationIter<R>` — [`Encoding`](../index.md#encoding), [`OperationIter`](#operationiter)

  Return an iterator for the operations in the expression.

#### Trait Implementations

##### `impl Any for Expression<R>`

- <span id="expression-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Expression<R>`

- <span id="expression-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Expression<R>`

- <span id="expression-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for Expression<R>`

- <span id="expression-clone"></span>`fn clone(&self) -> Expression<R>` — [`Expression`](#expression)

##### `impl CloneToUninit for Expression<R>`

- <span id="expression-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for Expression<R>`

##### `impl<R: fmt::Debug + Reader> Debug for Expression<R>`

- <span id="expression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for Expression<R>`

##### `impl<T> From for Expression<R>`

- <span id="expression-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<R: hash::Hash + Reader> Hash for Expression<R>`

- <span id="expression-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Expression<R>`

- <span id="expression-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for Expression<R>`

- <span id="expression-partialeq-eq"></span>`fn eq(&self, other: &Expression<R>) -> bool` — [`Expression`](#expression)

##### `impl<R: Reader> StructuralPartialEq for Expression<R>`

##### `impl ToOwned for Expression<R>`

- <span id="expression-toowned-type-owned"></span>`type Owned = T`

- <span id="expression-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="expression-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Expression<R>`

- <span id="expression-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expression-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Expression<R>`

- <span id="expression-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expression-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OperationIter<R: Reader>`

```rust
struct OperationIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:962-965`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L962-L965)*

An iterator for the operations in an expression.

#### Implementations

- <span id="operationiter-next"></span>`fn next(&mut self) -> Result<Option<Operation<R>>>` — [`Result`](../index.md#result), [`Operation`](#operation)

  Read the next operation in an expression.

- <span id="operationiter-offset-from"></span>`fn offset_from(&self, expression: &Expression<R>) -> <R as >::Offset` — [`Expression`](#expression), [`Reader`](#reader)

  Return the current byte offset of the iterator.

#### Trait Implementations

##### `impl Any for OperationIter<R>`

- <span id="operationiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OperationIter<R>`

- <span id="operationiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OperationIter<R>`

- <span id="operationiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for OperationIter<R>`

- <span id="operationiter-clone"></span>`fn clone(&self) -> OperationIter<R>` — [`OperationIter`](#operationiter)

##### `impl CloneToUninit for OperationIter<R>`

- <span id="operationiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy + Reader> Copy for OperationIter<R>`

##### `impl<R: fmt::Debug + Reader> Debug for OperationIter<R>`

- <span id="operationiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OperationIter<R>`

- <span id="operationiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OperationIter<R>`

- <span id="operationiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for OperationIter<R>`

- <span id="operationiter-toowned-type-owned"></span>`type Owned = T`

- <span id="operationiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="operationiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OperationIter<R>`

- <span id="operationiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="operationiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OperationIter<R>`

- <span id="operationiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="operationiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Evaluation<R: Reader, S: EvaluationStorage<R>>`

```rust
struct Evaluation<R: Reader, S: EvaluationStorage<R>> {
    bytecode: R,
    encoding: crate::common::Encoding,
    object_address: Option<u64>,
    max_iterations: Option<u32>,
    iteration: u32,
    state: EvaluationState<R>,
    addr_mask: u64,
    stack: super::util::ArrayVec<<S as >::Stack>,
    pc: R,
    expression_stack: super::util::ArrayVec<<S as >::ExpressionStack>,
    value_result: Option<crate::read::Value>,
    result: super::util::ArrayVec<<S as >::Result>,
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:1106-1131`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L1106-L1131)*

A DWARF expression evaluator.

# Usage
A DWARF expression may require additional data to produce a final result,
such as the value of a register or a memory location.  Once initial setup
is complete (i.e. `set_initial_value()`, `set_object_address()`) the
consumer calls the `evaluate()` method.  That returns an `EvaluationResult`,
which is either `EvaluationResult::Complete` or a value indicating what
data is needed to resume the `Evaluation`.  The consumer is responsible for
producing that data and resuming the computation with the correct method,
as documented for `EvaluationResult`.  Only once an `EvaluationResult::Complete`
is returned can the consumer call `result()`.

This design allows the consumer of `Evaluation` to decide how and when to
produce the required data and resume the computation.  The `Evaluation` can
be driven synchronously (as shown below) or by some asynchronous mechanism
such as futures.

# Examples
```rust,no_run
use gimli::{Evaluation, EvaluationResult, Expression};
let bytecode = gimli::EndianSlice::new(&[], gimli::LittleEndian);
let encoding = unimplemented!();
let get_register_value = |_, _| gimli::Value::Generic(42);
let get_frame_base = || 0xdeadbeef;

let mut eval = Evaluation::new(bytecode, encoding);
let mut result = eval.evaluate().unwrap();
while result != EvaluationResult::Complete {
  match result {
    EvaluationResult::RequiresRegister { register, base_type } => {
      let value = get_register_value(register, base_type);
      result = eval.resume_with_register(value).unwrap();
    },
    EvaluationResult::RequiresFrameBase => {
      let frame_base = get_frame_base();
      result = eval.resume_with_frame_base(frame_base).unwrap();
    },
    _ => unimplemented!(),
  };
}

let result = eval.result();
println!("{:?}", result);
```

#### Implementations

- <span id="evaluation-new"></span>`fn new(bytecode: R, encoding: Encoding) -> Self` — [`Encoding`](../index.md#encoding)

  Create a new DWARF expression evaluator.

  

  The new evaluator is created without an initial value, without

  an object address, and without a maximum number of iterations.

- <span id="evaluation-result"></span>`fn result(self) -> Vec<Piece<R>>` — [`Piece`](#piece)

  Get the result of this `Evaluation`.

  

  # Panics

  Panics if this `Evaluation` has not been driven to completion.

#### Trait Implementations

##### `impl Any for Evaluation<R, S>`

- <span id="evaluation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Evaluation<R, S>`

- <span id="evaluation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Evaluation<R, S>`

- <span id="evaluation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader, S: fmt::Debug + EvaluationStorage<R>> Debug for Evaluation<R, S>`

- <span id="evaluation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Evaluation<R, S>`

- <span id="evaluation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Evaluation<R, S>`

- <span id="evaluation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Evaluation<R, S>`

- <span id="evaluation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="evaluation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Evaluation<R, S>`

- <span id="evaluation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="evaluation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubNamesEntry<R: Reader>`

```rust
struct PubNamesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

*Defined in [`gimli-0.32.3/src/read/pubnames.rs:8-12`](../../../.source_1765633015/gimli-0.32.3/src/read/pubnames.rs#L8-L12)*

A single parsed pubname.

#### Implementations

- <span id="pubnamesentry-name"></span>`fn name(&self) -> &R`

  Returns the name this entry refers to.

- <span id="pubnamesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains this name.

- <span id="pubnamesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  has this name.

#### Trait Implementations

##### `impl Any for PubNamesEntry<R>`

- <span id="pubnamesentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubNamesEntry<R>`

- <span id="pubnamesentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubNamesEntry<R>`

- <span id="pubnamesentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntry<R>`

- <span id="pubnamesentry-clone"></span>`fn clone(&self) -> PubNamesEntry<R>` — [`PubNamesEntry`](#pubnamesentry)

##### `impl CloneToUninit for PubNamesEntry<R>`

- <span id="pubnamesentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- <span id="pubnamesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubNamesEntry<R>`

- <span id="pubnamesentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubNamesEntry<R>`

- <span id="pubnamesentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- <span id="pubnamesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl ToOwned for PubNamesEntry<R>`

- <span id="pubnamesentry-toowned-type-owned"></span>`type Owned = T`

- <span id="pubnamesentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubnamesentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubNamesEntry<R>`

- <span id="pubnamesentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubnamesentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubNamesEntry<R>`

- <span id="pubnamesentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubnamesentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubnames.rs:50`](../../../.source_1765633015/gimli-0.32.3/src/read/pubnames.rs#L50)*

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- <span id="debugpubnames-new"></span>`fn new(debug_pubnames_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubNames` instance from the data in the `.debug_pubnames`

  section.

  

  It is the caller's responsibility to read the `.debug_pubnames` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubNames, LittleEndian};

  

  let buf = [];

  let read_debug_pubnames_section_somehow = || &buf;

  let debug_pubnames =

      DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugPubNames<R>`

- <span id="debugpubnames-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugPubNames<R>`

- <span id="debugpubnames-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugPubNames<R>`

- <span id="debugpubnames-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugPubNames<R>`

- <span id="debugpubnames-clone"></span>`fn clone(&self) -> DebugPubNames<R>` — [`DebugPubNames`](#debugpubnames)

##### `impl CloneToUninit for DebugPubNames<R>`

- <span id="debugpubnames-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubNames<R>`

- <span id="debugpubnames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugPubNames<R>`

- <span id="debugpubnames-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugPubNames<R>`

- <span id="debugpubnames-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> Section for DebugPubNames<R>`

- <span id="debugpubnames-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugpubnames-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugPubNames<R>`

- <span id="debugpubnames-toowned-type-owned"></span>`type Owned = T`

- <span id="debugpubnames-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugpubnames-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugPubNames<R>`

- <span id="debugpubnames-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugpubnames-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugPubNames<R>`

- <span id="debugpubnames-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugpubnames-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubnames.rs:118`](../../../.source_1765633015/gimli-0.32.3/src/read/pubnames.rs#L118)*

An iterator over the pubnames from a `.debug_pubnames` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="pubnamesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../index.md#result), [`PubNamesEntry`](#pubnamesentry)

  Advance the iterator and return the next pubname.

  

  Returns the newly parsed pubname as `Ok(Some(pubname))`. Returns

  `Ok(None)` when iteration is complete and all pubnames have already been

  parsed and yielded. If an error occurs while parsing the next pubname,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl Any for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clone"></span>`fn clone(&self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](#pubnamesentryiter)

##### `impl CloneToUninit for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="pubnamesentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubnamesentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubnamesentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubnamesentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:8-12`](../../../.source_1765633015/gimli-0.32.3/src/read/pubtypes.rs#L8-L12)*

A single parsed pubtype.

#### Implementations

- <span id="pubtypesentry-name"></span>`fn name(&self) -> &R`

  Returns the name of the type this entry refers to.

- <span id="pubtypesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains the type with this name.

- <span id="pubtypesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  the type with this name.

#### Trait Implementations

##### `impl Any for PubTypesEntry<R>`

- <span id="pubtypesentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubTypesEntry<R>`

- <span id="pubtypesentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubTypesEntry<R>`

- <span id="pubtypesentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntry<R>`

- <span id="pubtypesentry-clone"></span>`fn clone(&self) -> PubTypesEntry<R>` — [`PubTypesEntry`](#pubtypesentry)

##### `impl CloneToUninit for PubTypesEntry<R>`

- <span id="pubtypesentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- <span id="pubtypesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubTypesEntry<R>`

- <span id="pubtypesentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubTypesEntry<R>`

- <span id="pubtypesentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- <span id="pubtypesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`DebugInfoOffset`](../index.md#debuginfooffset)

##### `impl ToOwned for PubTypesEntry<R>`

- <span id="pubtypesentry-toowned-type-owned"></span>`type Owned = T`

- <span id="pubtypesentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubtypesentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubTypesEntry<R>`

- <span id="pubtypesentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubtypesentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubTypesEntry<R>`

- <span id="pubtypesentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubtypesentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:50`](../../../.source_1765633015/gimli-0.32.3/src/read/pubtypes.rs#L50)*

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- <span id="debugpubtypes-new"></span>`fn new(debug_pubtypes_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubTypes` instance from the data in the `.debug_pubtypes`

  section.

  

  It is the caller's responsibility to read the `.debug_pubtypes` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubTypes, LittleEndian};

  

  let buf = [];

  let read_debug_pubtypes_somehow = || &buf;

  let debug_pubtypes =

      DebugPubTypes::new(read_debug_pubtypes_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugPubTypes<R>`

- <span id="debugpubtypes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugPubTypes<R>`

- <span id="debugpubtypes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugPubTypes<R>`

- <span id="debugpubtypes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugPubTypes<R>`

- <span id="debugpubtypes-clone"></span>`fn clone(&self) -> DebugPubTypes<R>` — [`DebugPubTypes`](#debugpubtypes)

##### `impl CloneToUninit for DebugPubTypes<R>`

- <span id="debugpubtypes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- <span id="debugpubtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DebugPubTypes<R>`

- <span id="debugpubtypes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugPubTypes<R>`

- <span id="debugpubtypes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- <span id="debugpubtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugpubtypes-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugPubTypes<R>`

- <span id="debugpubtypes-toowned-type-owned"></span>`type Owned = T`

- <span id="debugpubtypes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugpubtypes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugPubTypes<R>`

- <span id="debugpubtypes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugpubtypes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugPubTypes<R>`

- <span id="debugpubtypes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugpubtypes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

*Defined in [`gimli-0.32.3/src/read/pubtypes.rs:118`](../../../.source_1765633015/gimli-0.32.3/src/read/pubtypes.rs#L118)*

An iterator over the pubtypes from a `.debug_pubtypes` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="pubtypesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../index.md#result), [`PubTypesEntry`](#pubtypesentry)

  Advance the iterator and return the next pubtype.

  

  Returns the newly parsed pubtype as `Ok(Some(pubtype))`. Returns

  `Ok(None)` when iteration is complete and all pubtypes have already been

  parsed and yielded. If an error occurs while parsing the next pubtype,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl Any for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clone"></span>`fn clone(&self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](#pubtypesentryiter)

##### `impl CloneToUninit for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-toowned-type-owned"></span>`type Owned = T`

- <span id="pubtypesentryiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pubtypesentryiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pubtypesentryiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pubtypesentryiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugRanges<R>`

```rust
struct DebugRanges<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:14-16`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L14-L16)*

The raw contents of the `.debug_ranges` section.

#### Implementations

- <span id="debugranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRanges` instance from the data in the `.debug_ranges`

  section.

  

  It is the caller's responsibility to read the `.debug_ranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRanges, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_ranges_section_somehow = || &buf;

  let debug_ranges = DebugRanges::new(read_debug_ranges_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugRanges<R>`

- <span id="debugranges-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugRanges<R>`

- <span id="debugranges-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugRanges<R>`

- <span id="debugranges-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugRanges<R>`

- <span id="debugranges-clone"></span>`fn clone(&self) -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

##### `impl CloneToUninit for DebugRanges<R>`

- <span id="debugranges-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: fmt::Debug> Debug for DebugRanges<R>`

- <span id="debugranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRanges<R>`

- <span id="debugranges-default"></span>`fn default() -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

##### `impl<T> From for DebugRanges<R>`

- <span id="debugranges-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugRanges<R>`

- <span id="debugranges-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugRanges<R>`

- <span id="debugranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugranges-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugRanges<R>`

- <span id="debugranges-toowned-type-owned"></span>`type Owned = T`

- <span id="debugranges-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugranges-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugRanges<R>`

- <span id="debugranges-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugranges-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugRanges<R>`

- <span id="debugranges-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugranges-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:74-76`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L74-L76)*

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- <span id="debugrnglists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRngLists` instance from the data in the

  `.debug_rnglists` section.

  

  It is the caller's responsibility to read the `.debug_rnglists`

  section and present it as a `&[u8]` slice. That means using some ELF

  loader on Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRngLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_rnglists_section_somehow = || &buf;

  let debug_rnglists =

      DebugRngLists::new(read_debug_rnglists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugRngLists<R>`

- <span id="debugrnglists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugRngLists<R>`

- <span id="debugrnglists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugRngLists<R>`

- <span id="debugrnglists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugRngLists<R>`

- <span id="debugrnglists-clone"></span>`fn clone(&self) -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

##### `impl CloneToUninit for DebugRngLists<R>`

- <span id="debugrnglists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugRngLists<R>`

- <span id="debugrnglists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRngLists<R>`

- <span id="debugrnglists-default"></span>`fn default() -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

##### `impl<T> From for DebugRngLists<R>`

- <span id="debugrnglists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugRngLists<R>`

- <span id="debugrnglists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugRngLists<R>`

- <span id="debugrnglists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugrnglists-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugRngLists<R>`

- <span id="debugrnglists-toowned-type-owned"></span>`type Owned = T`

- <span id="debugrnglists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugrnglists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugRngLists<R>`

- <span id="debugrnglists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugrnglists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugRngLists<R>`

- <span id="debugrnglists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugrnglists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:158-161`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L158-L161)*

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- <span id="rangelists-new"></span>`fn new(debug_ranges: DebugRanges<R>, debug_rnglists: DebugRngLists<R>) -> RangeLists<R>` — [`DebugRanges`](#debugranges), [`DebugRngLists`](#debugrnglists), [`RangeLists`](#rangelists)

  Construct a new `RangeLists` instance from the data in the `.debug_ranges` and

  `.debug_rnglists` sections.

- <span id="rangelists-debug-ranges"></span>`fn debug_ranges(&self) -> &DebugRanges<R>` — [`DebugRanges`](#debugranges)

  Return the `.debug_ranges` section.

- <span id="rangelists-set-debug-ranges"></span>`fn set_debug_ranges(&mut self, debug_ranges: DebugRanges<R>)` — [`DebugRanges`](#debugranges)

  Replace the `.debug_ranges` section.

  

  This is useful for `.dwo` files when using the GNU split-dwarf extension to DWARF 4.

- <span id="rangelists-debug-rnglists"></span>`fn debug_rnglists(&self) -> &DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

  Return the `.debug_rnglists` section.

#### Trait Implementations

##### `impl Any for RangeLists<R>`

- <span id="rangelists-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeLists<R>`

- <span id="rangelists-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeLists<R>`

- <span id="rangelists-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for RangeLists<R>`

- <span id="rangelists-clone"></span>`fn clone(&self) -> RangeLists<R>` — [`RangeLists`](#rangelists)

##### `impl CloneToUninit for RangeLists<R>`

- <span id="rangelists-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for RangeLists<R>`

##### `impl<R: fmt::Debug> Debug for RangeLists<R>`

- <span id="rangelists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for RangeLists<R>`

- <span id="rangelists-default"></span>`fn default() -> RangeLists<R>` — [`RangeLists`](#rangelists)

##### `impl<T> From for RangeLists<R>`

- <span id="rangelists-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeLists<R>`

- <span id="rangelists-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RangeLists<R>`

- <span id="rangelists-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelists-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelists-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RangeLists<R>`

- <span id="rangelists-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelists-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeLists<R>`

- <span id="rangelists-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelists-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:306-310`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L306-L310)*

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- <span id="rawrnglistiter-new"></span>`fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../index.md#encoding), [`RangeListsFormat`](rnglists/index.md#rangelistsformat), [`RawRngListIter`](#rawrnglistiter)

  Construct a `RawRngListIter`.

- <span id="rawrnglistiter-next"></span>`fn next(&mut self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../index.md#result), [`RawRngListEntry`](#rawrnglistentry), [`Reader`](#reader)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl Any for RawRngListIter<R>`

- <span id="rawrnglistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRngListIter<R>`

- <span id="rawrnglistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRngListIter<R>`

- <span id="rawrnglistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RawRngListIter<R>`

- <span id="rawrnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawRngListIter<R>`

- <span id="rawrnglistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawRngListIter<R>`

- <span id="rawrnglistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawRngListIter<R>`

- <span id="rawrnglistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrnglistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawRngListIter<R>`

- <span id="rawrnglistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrnglistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:473-478`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L473-L478)*

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="rnglistiter-new"></span>`fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](#rawrnglistiter), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md#debugaddrbase), [`Reader`](#reader), [`RngListIter`](#rnglistiter)

  Construct a `RngListIter`.

- <span id="rnglistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md#debugaddrindex), [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="rnglistiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../index.md#result), [`Range`](#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl Any for RngListIter<R>`

- <span id="rnglistiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RngListIter<R>`

- <span id="rnglistiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RngListIter<R>`

- <span id="rnglistiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RngListIter<R>`

- <span id="rnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RngListIter<R>`

- <span id="rnglistiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RngListIter<R>`

- <span id="rnglistiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RngListIter<R>`

- <span id="rnglistiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rnglistiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RngListIter<R>`

- <span id="rnglistiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rnglistiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:598-604`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L598-L604)*

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="rawrange-is-end"></span>`fn is_end(&self) -> bool`

  Check if this is a range end entry.

- <span id="rawrange-is-base-address"></span>`fn is_base_address(&self, address_size: u8) -> bool`

  Check if this is a base address selection entry.

  

  A base address selection entry changes the base address that subsequent

  range entries are relative to.

- <span id="rawrange-parse"></span>`fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../index.md#result), [`RawRange`](rnglists/index.md#rawrange)

  Parse an address range entry from `.debug_ranges` or `.debug_loc`.

#### Trait Implementations

##### `impl Any for RawRange`

- <span id="rawrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRange`

- <span id="rawrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRange`

- <span id="rawrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RawRange`

- <span id="rawrange-clone"></span>`fn clone(&self) -> RawRange` — [`RawRange`](rnglists/index.md#rawrange)

##### `impl CloneToUninit for RawRange`

- <span id="rawrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- <span id="rawrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RawRange`

##### `impl<T> From for RawRange`

- <span id="rawrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RawRange`

- <span id="rawrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RawRange`

- <span id="rawrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RawRange`

- <span id="rawrange-partialeq-eq"></span>`fn eq(&self, other: &RawRange) -> bool` — [`RawRange`](rnglists/index.md#rawrange)

##### `impl StructuralPartialEq for RawRange`

##### `impl ToOwned for RawRange`

- <span id="rawrange-toowned-type-owned"></span>`type Owned = T`

- <span id="rawrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RawRange`

- <span id="rawrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawRange`

- <span id="rawrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:634-640`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L634-L640)*

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="range-add-base-address"></span>`fn add_base_address(&mut self, base_address: u64, address_size: u8)`

  Add a base address to this range.

#### Trait Implementations

##### `impl Any for Range`

- <span id="range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Range`

- <span id="range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Range`

- <span id="range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Range`

- <span id="range-clone"></span>`fn clone(&self) -> Range` — [`Range`](#range)

##### `impl CloneToUninit for Range`

- <span id="range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Range`

##### `impl Debug for Range`

- <span id="range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Range`

##### `impl<T> From for Range`

- <span id="range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Range`

- <span id="range-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Range`

- <span id="range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Range`

- <span id="range-ord-cmp"></span>`fn cmp(&self, other: &Range) -> cmp::Ordering` — [`Range`](#range)

##### `impl PartialEq for Range`

- <span id="range-partialeq-eq"></span>`fn eq(&self, other: &Range) -> bool` — [`Range`](#range)

##### `impl PartialOrd for Range`

- <span id="range-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Range) -> option::Option<cmp::Ordering>` — [`Range`](#range)

##### `impl StructuralPartialEq for Range`

##### `impl ToOwned for Range`

- <span id="range-toowned-type-owned"></span>`type Owned = T`

- <span id="range-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="range-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Range`

- <span id="range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Range`

- <span id="range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugStr<R>`

```rust
struct DebugStr<R> {
    debug_str_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:12-14`](../../../.source_1765633015/gimli-0.32.3/src/read/str.rs#L12-L14)*

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- <span id="debugstr-new"></span>`fn new(debug_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugStr` instance from the data in the `.debug_str`

  section.

  

  It is the caller's responsibility to read the `.debug_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_str_section_somehow = || &buf;

  let debug_str = DebugStr::new(read_debug_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugStr<R>`

- <span id="debugstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStr<R>`

- <span id="debugstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStr<R>`

- <span id="debugstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugStr<R>`

- <span id="debugstr-clone"></span>`fn clone(&self) -> DebugStr<R>` — [`DebugStr`](#debugstr)

##### `impl CloneToUninit for DebugStr<R>`

- <span id="debugstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugStr<R>`

- <span id="debugstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStr<R>`

- <span id="debugstr-default"></span>`fn default() -> DebugStr<R>` — [`DebugStr`](#debugstr)

##### `impl<T> From for DebugStr<R>`

- <span id="debugstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugStr<R>`

- <span id="debugstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugStr<R>`

- <span id="debugstr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugstr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugStr<R>`

- <span id="debugstr-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugStr<R>`

- <span id="debugstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugStr<R>`

- <span id="debugstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:91-93`](../../../.source_1765633015/gimli-0.32.3/src/read/str.rs#L91-L93)*

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- <span id="debugstroffsets-get-str-offset"></span>`fn get_str_offset(&self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../index.md#format), [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase), [`Reader`](#reader), [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex), [`Result`](../index.md#result), [`DebugStrOffset`](../index.md#debugstroffset)

  Returns the `.debug_str` offset at the given `base` and `index`.

  

  A set of entries in the `.debug_str_offsets` section consists of a header

  followed by a series of string table offsets.

  

  The `base` must be the `DW_AT_str_offsets_base` value from the compilation unit DIE.

  This is an offset that points to the first entry following the header.

  

  The `index` is the value of a `DW_FORM_strx` attribute.

  

  The `format` must be the DWARF format of the compilation unit. This format must

  match the header. However, note that we do not parse the header to validate this,

  since locating the header is unreliable, and the GNU extensions do not emit it.

#### Trait Implementations

##### `impl Any for DebugStrOffsets<R>`

- <span id="debugstroffsets-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugStrOffsets<R>`

- <span id="debugstroffsets-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugStrOffsets<R>`

- <span id="debugstroffsets-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugStrOffsets<R>`

- <span id="debugstroffsets-clone"></span>`fn clone(&self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](#debugstroffsets)

##### `impl CloneToUninit for DebugStrOffsets<R>`

- <span id="debugstroffsets-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: fmt::Debug> Debug for DebugStrOffsets<R>`

- <span id="debugstroffsets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStrOffsets<R>`

- <span id="debugstroffsets-default"></span>`fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](#debugstroffsets)

##### `impl<T> From for DebugStrOffsets<R>`

- <span id="debugstroffsets-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugStrOffsets<R>`

- <span id="debugstroffsets-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugStrOffsets<R>`

- <span id="debugstroffsets-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugstroffsets-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugStrOffsets<R>`

- <span id="debugstroffsets-toowned-type-owned"></span>`type Owned = T`

- <span id="debugstroffsets-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debugstroffsets-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugStrOffsets<R>`

- <span id="debugstroffsets-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debugstroffsets-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugStrOffsets<R>`

- <span id="debugstroffsets-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debugstroffsets-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/str.rs:184-186`](../../../.source_1765633015/gimli-0.32.3/src/read/str.rs#L184-L186)*

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- <span id="debuglinestr-new"></span>`fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLineStr` instance from the data in the `.debug_line_str`

  section.

  

  It is the caller's responsibility to read the `.debug_line_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLineStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_str_section_somehow = || &buf;

  let debug_str = DebugLineStr::new(read_debug_line_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl Any for DebugLineStr<R>`

- <span id="debuglinestr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugLineStr<R>`

- <span id="debuglinestr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugLineStr<R>`

- <span id="debuglinestr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone> Clone for DebugLineStr<R>`

- <span id="debuglinestr-clone"></span>`fn clone(&self) -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

##### `impl CloneToUninit for DebugLineStr<R>`

- <span id="debuglinestr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugLineStr<R>`

- <span id="debuglinestr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLineStr<R>`

- <span id="debuglinestr-default"></span>`fn default() -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

##### `impl<T> From for DebugLineStr<R>`

- <span id="debuglinestr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugLineStr<R>`

- <span id="debuglinestr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugLineStr<R>`

- <span id="debuglinestr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debuglinestr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl ToOwned for DebugLineStr<R>`

- <span id="debuglinestr-toowned-type-owned"></span>`type Owned = T`

- <span id="debuglinestr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="debuglinestr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DebugLineStr<R>`

- <span id="debuglinestr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="debuglinestr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DebugLineStr<R>`

- <span id="debuglinestr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="debuglinestr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DebugInfo<R>`

```rust
struct DebugInfo<R> {
    debug_info_section: R,
}
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:82-84`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L82-L84)*

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

- <span id="debuginfo-clone"></span>`fn clone(&self) -> DebugInfo<R>` — [`DebugInfo`](#debuginfo)

##### `impl CloneToUninit for DebugInfo<R>`

- <span id="debuginfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugInfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugInfo<R>`

- <span id="debuginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugInfo<R>`

- <span id="debuginfo-default"></span>`fn default() -> DebugInfo<R>` — [`DebugInfo`](#debuginfo)

##### `impl<T> From for DebugInfo<R>`

- <span id="debuginfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugInfo<R>`

- <span id="debuginfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugInfo<R>`

- <span id="debuginfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:179-182`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L179-L182)*

An iterator over the units of a .debug_info section.

See the [documentation on
`DebugInfo::units`](./struct.DebugInfo.html#method.units) for more detail.

#### Implementations

- <span id="debuginfounitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../index.md#result), [`UnitHeader`](#unitheader)

  Advance the iterator to the next unit header.

#### Trait Implementations

##### `impl Any for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-clone"></span>`fn clone(&self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:303-314`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L303-L314)*

The common fields for the headers of compilation units and
type units.

#### Implementations

- <span id="unitheader-new"></span>`fn new(encoding: Encoding, unit_length: Offset, unit_type: UnitType<Offset>, debug_abbrev_offset: DebugAbbrevOffset<Offset>, unit_offset: UnitSectionOffset<Offset>, entries_buf: R) -> Self` — [`Encoding`](../index.md#encoding), [`UnitType`](#unittype), [`DebugAbbrevOffset`](../index.md#debugabbrevoffset), [`UnitSectionOffset`](../index.md#unitsectionoffset)

  Construct a new `UnitHeader`.

#### Trait Implementations

##### `impl Any for UnitHeader<R, Offset>`

- <span id="unitheader-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitHeader<R, Offset>`

- <span id="unitheader-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitHeader<R, Offset>`

- <span id="unitheader-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for UnitHeader<R, Offset>`

- <span id="unitheader-clone"></span>`fn clone(&self) -> UnitHeader<R, Offset>` — [`UnitHeader`](#unitheader)

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

- <span id="unitheader-partialeq-eq"></span>`fn eq(&self, other: &UnitHeader<R, Offset>) -> bool` — [`UnitHeader`](#unitheader)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:647-657`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L647-L657)*

A Debugging Information Entry (DIE).

DIEs have a set of attributes and optionally have children DIEs as well.

#### Implementations

- <span id="debugginginformationentry-new"></span>`fn new(offset: UnitOffset<Offset>, attrs_slice: R, abbrev: &'abbrev Abbreviation, unit: &'unit UnitHeader<R, Offset>) -> Self` — [`UnitOffset`](../index.md#unitoffset), [`Abbreviation`](#abbreviation), [`UnitHeader`](#unitheader)

  Construct a new `DebuggingInformationEntry`.

- <span id="debugginginformationentry-code"></span>`fn code(&self) -> u64`

  Get this entry's code.

- <span id="debugginginformationentry-offset"></span>`fn offset(&self) -> UnitOffset<Offset>` — [`UnitOffset`](../index.md#unitoffset)

  Get this entry's offset.

- <span id="debugginginformationentry-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../index.md#dwtag)

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

- <span id="debugginginformationentry-attrs"></span>`fn attrs<'me>(self: &'me Self) -> AttrsIter<'abbrev, 'me, 'unit, R>` — [`AttrsIter`](#attrsiter)

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

- <span id="debugginginformationentry-attr"></span>`fn attr(&self, name: constants::DwAt) -> Result<Option<Attribute<R>>>` — [`DwAt`](../index.md#dwat), [`Result`](../index.md#result), [`Attribute`](#attribute)

  Find the first attribute in this entry which has the given name,

  and return it. Returns `Ok(None)` if no attribute is found.

- <span id="debugginginformationentry-attr-value-raw"></span>`fn attr_value_raw(&self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../index.md#dwat), [`Result`](../index.md#result), [`AttributeValue`](#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its raw value. Returns `Ok(None)` if no attribute is found.

- <span id="debugginginformationentry-attr-value"></span>`fn attr_value(&self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../index.md#dwat), [`Result`](../index.md#result), [`AttributeValue`](#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its normalized value.  Returns `Ok(None)` if no

  attribute is found.

- <span id="debugginginformationentry-after-attrs"></span>`fn after_attrs(&self) -> Result<R>` — [`Result`](../index.md#result)

  Return the input buffer after the last attribute.

- <span id="debugginginformationentry-sibling"></span>`fn sibling(&self) -> Option<R>`

  Use the `DW_AT_sibling` attribute to find the input buffer for the

  next sibling. Returns `None` if the attribute is missing or invalid.

- <span id="debugginginformationentry-parse"></span>`fn parse(input: &mut R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Result<Option<Self>>` — [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations), [`Result`](../index.md#result)

  Parse an entry. Returns `Ok(None)` for null entries.

#### Trait Implementations

##### `impl Any for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- <span id="debugginginformationentry-clone"></span>`fn clone(&self) -> DebuggingInformationEntry<'abbrev, 'unit, R, Offset>` — [`DebuggingInformationEntry`](#debugginginformationentry)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:1111-1114`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1111-L1114)*

An attribute in a `DebuggingInformationEntry`, consisting of a name and
associated value.

#### Implementations

- <span id="attribute-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../index.md#dwat)

  Get this attribute's name.

- <span id="attribute-raw-value"></span>`fn raw_value(&self) -> AttributeValue<R>` — [`AttributeValue`](#attributevalue)

  Get this attribute's raw value.

- <span id="attribute-value"></span>`fn value(&self) -> AttributeValue<R>` — [`AttributeValue`](#attributevalue)

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

- <span id="attribute-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](#reader)

  Try to convert this attribute's value to an offset.

- <span id="attribute-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attribute-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attribute-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](#debugstr)

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

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute<R>` — [`Attribute`](#attribute)

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

- <span id="attribute-partialeq-eq"></span>`fn eq(&self, other: &Attribute<R>) -> bool` — [`Attribute`](#attribute)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:2272-2276`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L2272-L2276)*

An iterator over a particular entry's attributes.

See [the documentation for
`DebuggingInformationEntry::attrs()`](./struct.DebuggingInformationEntry.html#method.attrs)
for details.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- <span id="attrsiter-next"></span>`fn next(&mut self) -> Result<Option<Attribute<R>>>` — [`Result`](../index.md#result), [`Attribute`](#attribute)

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

- <span id="attrsiter-clone"></span>`fn clone(&self) -> AttrsIter<'abbrev, 'entry, 'unit, R>` — [`AttrsIter`](#attrsiter)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:2382-2390`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L2382-L2390)*

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

- <span id="entriesraw-next-offset"></span>`fn next_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Return the unit offset at which the reader will read next.

  

  If you want the offset of the next entry, then this must be called prior to reading

  the next entry.

- <span id="entriesraw-next-depth"></span>`fn next_depth(&self) -> isize`

  Return the depth of the next entry.

  

  This depth is updated when `read_abbreviation` is called, and is updated

  based on null entries and the `has_children` field in the abbreviation.

- <span id="entriesraw-read-abbreviation"></span>`fn read_abbreviation(&mut self) -> Result<Option<&'abbrev Abbreviation>>` — [`Result`](../index.md#result), [`Abbreviation`](#abbreviation)

  Read an abbreviation code and lookup the corresponding `Abbreviation`.

  

  Returns `Ok(None)` for null entries.

- <span id="entriesraw-read-attribute"></span>`fn read_attribute(&mut self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md#result), [`Attribute`](#attribute)

  Read an attribute.

- <span id="entriesraw-skip-attributes"></span>`fn skip_attributes(&mut self, specs: &[AttributeSpecification]) -> Result<()>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md#result)

  Skip all the attributes of an abbreviation.

#### Trait Implementations

##### `impl Any for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for EntriesRaw<'abbrev, 'unit, R>`

- <span id="entriesraw-clone"></span>`fn clone(&self) -> EntriesRaw<'abbrev, 'unit, R>` — [`EntriesRaw`](#entriesraw)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:2463-2472`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L2463-L2472)*

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

- <span id="entriescursor-current"></span>`fn current(&self) -> Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>` — [`DebuggingInformationEntry`](#debugginginformationentry)

  Get a reference to the entry that the cursor is currently pointing to.

  

  If the cursor is not pointing at an entry, or if the current entry is a

  null entry, then `None` is returned.

- <span id="entriescursor-next-entry"></span>`fn next_entry(&mut self) -> Result<Option<()>>` — [`Result`](../index.md#result)

  Move the cursor to the next DIE in the tree.

  

  Returns `Some` if there is a next entry, even if this entry is null.

  If there is no next entry, then `None` is returned.

- <span id="entriescursor-next-dfs"></span>`fn next_dfs(&mut self) -> Result<Option<(isize, &DebuggingInformationEntry<'abbrev, 'unit, R>)>>` — [`Result`](../index.md#result), [`DebuggingInformationEntry`](#debugginginformationentry)

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

- <span id="entriescursor-next-sibling"></span>`fn next_sibling(&mut self) -> Result<Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>>` — [`Result`](../index.md#result), [`DebuggingInformationEntry`](#debugginginformationentry)

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

- <span id="entriescursor-clone"></span>`fn clone(&self) -> EntriesCursor<'abbrev, 'unit, R>` — [`EntriesCursor`](#entriescursor)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:2847-2857`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L2847-L2857)*

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

- <span id="entriestree-new"></span>`fn new(root: R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Self` — [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations)

- <span id="entriestree-root"></span>`fn root<'me>(self: &'me mut Self) -> Result<EntriesTreeNode<'abbrev, 'unit, 'me, R>>` — [`Result`](../index.md#result), [`EntriesTreeNode`](#entriestreenode)

  Returns the root node of the tree.

- <span id="entriestree-next"></span>`fn next(&mut self, depth: isize) -> Result<bool>` — [`Result`](../index.md#result)

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

- <span id="entriestree-clone"></span>`fn clone(&self) -> EntriesTree<'abbrev, 'unit, R>` — [`EntriesTree`](#entriestree)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:2979-2982`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L2979-L2982)*

A node in the Debugging Information Entry tree.

The root node of a tree can be obtained
via [`EntriesTree::root`](./struct.EntriesTree.html#method.root).

#### Implementations

- <span id="entriestreenode-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeNode<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](#entriestree), [`EntriesTreeNode`](#entriestreenode)

- <span id="entriestreenode-entry"></span>`fn entry(&self) -> &DebuggingInformationEntry<'abbrev, 'unit, R>` — [`DebuggingInformationEntry`](#debugginginformationentry)

  Returns the current entry in the tree.

- <span id="entriestreenode-children"></span>`fn children(self) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTreeIter`](#entriestreeiter)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:3014-3018`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L3014-L3018)*

An iterator that allows traversal of the children of an
`EntriesTreeNode`.

The items returned by this iterator are also `EntriesTreeNode`s,
which allow recursive traversal of grandchildren, etc.

#### Implementations

- <span id="entriestreeiter-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](#entriestree), [`EntriesTreeIter`](#entriestreeiter)

- <span id="entriestreeiter-next"></span>`fn next<'me>(self: &'me mut Self) -> Result<Option<EntriesTreeNode<'abbrev, 'unit, 'me, R>>>` — [`Result`](../index.md#result), [`EntriesTreeNode`](#entriestreenode)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:3061-3063`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L3061-L3063)*

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

- <span id="debugtypes-clone"></span>`fn clone(&self) -> DebugTypes<R>` — [`DebugTypes`](#debugtypes)

##### `impl CloneToUninit for DebugTypes<R>`

- <span id="debugtypes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: marker::Copy> Copy for DebugTypes<R>`

##### `impl<R: fmt::Debug> Debug for DebugTypes<R>`

- <span id="debugtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTypes<R>`

- <span id="debugtypes-default"></span>`fn default() -> DebugTypes<R>` — [`DebugTypes`](#debugtypes)

##### `impl<T> From for DebugTypes<R>`

- <span id="debugtypes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DebugTypes<R>`

- <span id="debugtypes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R> Section for DebugTypes<R>`

- <span id="debugtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:3152-3155`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L3152-L3155)*

An iterator over the type-units of this `.debug_types` section.

See the [documentation on
`DebugTypes::units`](./struct.DebugTypes.html#method.units) for
more detail.

#### Implementations

- <span id="debugtypesunitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../index.md#result), [`UnitHeader`](#unitheader)

  Advance the iterator to the next type unit header.

#### Trait Implementations

##### `impl Any for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-clone"></span>`fn clone(&self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter)

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

*Defined in [`gimli-0.32.3/src/read/mod.rs:286-466`](../../../.source_1765633015/gimli-0.32.3/src/read/mod.rs#L286-L466)*

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

- <span id="error-description"></span>`fn description(&self) -> &str`

  A short description of the error.

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](../index.md#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::core::result::Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](../index.md#error)

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

### `CieOrFde<'bases, Section, R>`

```rust
enum CieOrFde<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    Cie(CommonInformationEntry<R>),
    Fde(PartialFrameDescriptionEntry<'bases, Section, R>),
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1059-1070`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1059-L1070)*

Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE).

#### Variants

- **`Cie`**

  This CFI entry is a `CommonInformationEntry`.

- **`Fde`**

  This CFI entry is a `FrameDescriptionEntry`, however fully parsing it
  requires parsing its CIE first, so it is left in a partially parsed
  state.

#### Trait Implementations

##### `impl Any for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Section, R> Clone for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-clone"></span>`fn clone(&self) -> CieOrFde<'bases, Section, R>` — [`CieOrFde`](#cieorfde)

##### `impl CloneToUninit for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Section, R> Debug for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section, R> Eq for CieOrFde<'bases, Section, R>`

##### `impl<T> From for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Section, R> PartialEq for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-partialeq-eq"></span>`fn eq(&self, other: &CieOrFde<'bases, Section, R>) -> bool` — [`CieOrFde`](#cieorfde)

##### `impl<Section, R> StructuralPartialEq for CieOrFde<'bases, Section, R>`

##### `impl ToOwned for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-toowned-type-owned"></span>`type Owned = T`

- <span id="cieorfde-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cieorfde-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cieorfde-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cieorfde-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CfaRule<T: ReaderOffset>`

```rust
enum CfaRule<T: ReaderOffset> {
    RegisterAndOffset {
        register: crate::common::Register,
        offset: i64,
    },
    Expression(UnwindExpression<T>),
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2876-2886`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2876-L2886)*

The canonical frame address (CFA) recovery rules.

#### Variants

- **`RegisterAndOffset`**

  The CFA is given offset from the given register's value.

- **`Expression`**

  The CFA is obtained by evaluating a DWARF expression program.

#### Implementations

- <span id="cfarule-is-default"></span>`fn is_default(&self) -> bool`

#### Trait Implementations

##### `impl<T> Any for CfaRule<T>`

- <span id="cfarule-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CfaRule<T>`

- <span id="cfarule-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CfaRule<T>`

- <span id="cfarule-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + ReaderOffset> Clone for CfaRule<T>`

- <span id="cfarule-clone"></span>`fn clone(&self) -> CfaRule<T>` — [`CfaRule`](#cfarule)

##### `impl<T> CloneToUninit for CfaRule<T>`

- <span id="cfarule-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + ReaderOffset> Debug for CfaRule<T>`

- <span id="cfarule-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ReaderOffset> Default for CfaRule<T>`

- <span id="cfarule-default"></span>`fn default() -> Self`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for CfaRule<T>`

##### `impl<T> From for CfaRule<T>`

- <span id="cfarule-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CfaRule<T>`

- <span id="cfarule-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for CfaRule<T>`

- <span id="cfarule-partialeq-eq"></span>`fn eq(&self, other: &CfaRule<T>) -> bool` — [`CfaRule`](#cfarule)

##### `impl<T: ReaderOffset> StructuralPartialEq for CfaRule<T>`

##### `impl<T> ToOwned for CfaRule<T>`

- <span id="cfarule-toowned-type-owned"></span>`type Owned = T`

- <span id="cfarule-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cfarule-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for CfaRule<T>`

- <span id="cfarule-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cfarule-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CfaRule<T>`

- <span id="cfarule-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cfarule-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegisterRule<T: ReaderOffset>`

```rust
enum RegisterRule<T: ReaderOffset> {
    Undefined,
    SameValue,
    Offset(i64),
    ValOffset(i64),
    Register(crate::common::Register),
    Expression(UnwindExpression<T>),
    ValExpression(UnwindExpression<T>),
    Architectural,
    Constant(u64),
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2916-2951`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2916-L2951)*

An entry in the abstract CFI table that describes how to find the value of a
register.

"The register columns contain rules that describe whether a given register
has been saved and the rule to find the value for the register in the
previous frame."

#### Variants

- **`Undefined`**

  > A register that has this rule has no recoverable value in the previous
  > frame. (By convention, it is not preserved by a callee.)

- **`SameValue`**

  > This register has not been modified from the previous frame. (By
  > convention, it is preserved by the callee, but the callee has not
  > modified it.)

- **`Offset`**

  "The previous value of this register is saved at the address CFA+N where
  CFA is the current CFA value and N is a signed offset."

- **`ValOffset`**

  "The previous value of this register is the value CFA+N where CFA is the
  current CFA value and N is a signed offset."

- **`Register`**

  "The previous value of this register is stored in another register
  numbered R."

- **`Expression`**

  "The previous value of this register is located at the address produced
  by executing the DWARF expression."

- **`ValExpression`**

  "The previous value of this register is the value produced by executing
  the DWARF expression."

- **`Architectural`**

  "The rule is defined externally to this specification by the augmenter."

- **`Constant`**

  This is a pseudo-register with a constant value.

#### Implementations

- <span id="registerrule-is-defined"></span>`fn is_defined(&self) -> bool`

#### Trait Implementations

##### `impl<T> Any for RegisterRule<T>`

- <span id="registerrule-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegisterRule<T>`

- <span id="registerrule-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegisterRule<T>`

- <span id="registerrule-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + ReaderOffset> Clone for RegisterRule<T>`

- <span id="registerrule-clone"></span>`fn clone(&self) -> RegisterRule<T>` — [`RegisterRule`](#registerrule)

##### `impl<T> CloneToUninit for RegisterRule<T>`

- <span id="registerrule-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + ReaderOffset> Debug for RegisterRule<T>`

- <span id="registerrule-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for RegisterRule<T>`

##### `impl<T> From for RegisterRule<T>`

- <span id="registerrule-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RegisterRule<T>`

- <span id="registerrule-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for RegisterRule<T>`

- <span id="registerrule-partialeq-eq"></span>`fn eq(&self, other: &RegisterRule<T>) -> bool` — [`RegisterRule`](#registerrule)

##### `impl<T: ReaderOffset> StructuralPartialEq for RegisterRule<T>`

##### `impl<T> ToOwned for RegisterRule<T>`

- <span id="registerrule-toowned-type-owned"></span>`type Owned = T`

- <span id="registerrule-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="registerrule-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RegisterRule<T>`

- <span id="registerrule-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="registerrule-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RegisterRule<T>`

- <span id="registerrule-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="registerrule-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CallFrameInstruction<T: ReaderOffset>`

```rust
enum CallFrameInstruction<T: ReaderOffset> {
    SetLoc {
        address: u64,
    },
    AdvanceLoc {
        delta: u32,
    },
    DefCfa {
        register: crate::common::Register,
        offset: u64,
    },
    DefCfaSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    DefCfaRegister {
        register: crate::common::Register,
    },
    DefCfaOffset {
        offset: u64,
    },
    DefCfaOffsetSf {
        factored_offset: i64,
    },
    DefCfaExpression {
        expression: UnwindExpression<T>,
    },
    Undefined {
        register: crate::common::Register,
    },
    SameValue {
        register: crate::common::Register,
    },
    Offset {
        register: crate::common::Register,
        factored_offset: u64,
    },
    OffsetExtendedSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    ValOffset {
        register: crate::common::Register,
        factored_offset: u64,
    },
    ValOffsetSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    Register {
        dest_register: crate::common::Register,
        src_register: crate::common::Register,
    },
    Expression {
        register: crate::common::Register,
        expression: UnwindExpression<T>,
    },
    ValExpression {
        register: crate::common::Register,
        expression: UnwindExpression<T>,
    },
    Restore {
        register: crate::common::Register,
    },
    RememberState,
    RestoreState,
    ArgsSize {
        size: u64,
    },
    NegateRaState,
    Nop,
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:2961-3255`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L2961-L3255)*

A parsed call frame instruction.

#### Variants

- **`SetLoc`**

  > 1. DW_CFA_set_loc
  >
  > The DW_CFA_set_loc instruction takes a single operand that represents
  > a target address. The required action is to create a new table row
  > using the specified address as the location. All other values in the
  > new row are initially identical to the current row. The new location
  > value is always greater than the current one. If the segment_size
  > field of this FDE's CIE is non- zero, the initial location is preceded
  > by a segment selector of the given length.

- **`AdvanceLoc`**

  The `AdvanceLoc` instruction is used for all of `DW_CFA_advance_loc` and
  `DW_CFA_advance_loc{1,2,4}`.
  
  > 2. DW_CFA_advance_loc
  >
  > The DW_CFA_advance instruction takes a single operand (encoded with
  > the opcode) that represents a constant delta. The required action is
  > to create a new table row with a location value that is computed by
  > taking the current entry’s location value and adding the value of
  > delta * code_alignment_factor. All other values in the new row are
  > initially identical to the current row.

- **`DefCfa`**

  > 1. DW_CFA_def_cfa
  >
  > The DW_CFA_def_cfa instruction takes two unsigned LEB128 operands
  > representing a register number and a (non-factored) offset. The
  > required action is to define the current CFA rule to use the provided
  > register and offset.

- **`DefCfaSf`**

  > 2. DW_CFA_def_cfa_sf
  >
  > The DW_CFA_def_cfa_sf instruction takes two operands: an unsigned
  > LEB128 value representing a register number and a signed LEB128
  > factored offset. This instruction is identical to DW_CFA_def_cfa
  > except that the second operand is signed and factored. The resulting
  > offset is factored_offset * data_alignment_factor.

- **`DefCfaRegister`**

  > 3. DW_CFA_def_cfa_register
  >
  > The DW_CFA_def_cfa_register instruction takes a single unsigned LEB128
  > operand representing a register number. The required action is to
  > define the current CFA rule to use the provided register (but to keep
  > the old offset). This operation is valid only if the current CFA rule
  > is defined to use a register and offset.

- **`DefCfaOffset`**

  > 4. DW_CFA_def_cfa_offset
  >
  > The DW_CFA_def_cfa_offset instruction takes a single unsigned LEB128
  > operand representing a (non-factored) offset. The required action is
  > to define the current CFA rule to use the provided offset (but to keep
  > the old register). This operation is valid only if the current CFA
  > rule is defined to use a register and offset.

- **`DefCfaOffsetSf`**

  > 5. DW_CFA_def_cfa_offset_sf
  >
  > The DW_CFA_def_cfa_offset_sf instruction takes a signed LEB128 operand
  > representing a factored offset. This instruction is identical to
  > DW_CFA_def_cfa_offset except that the operand is signed and
  > factored. The resulting offset is factored_offset *
  > data_alignment_factor. This operation is valid only if the current CFA
  > rule is defined to use a register and offset.

- **`DefCfaExpression`**

  > 6. DW_CFA_def_cfa_expression
  >
  > The DW_CFA_def_cfa_expression instruction takes a single operand
  > encoded as a DW_FORM_exprloc value representing a DWARF
  > expression. The required action is to establish that expression as the
  > means by which the current CFA is computed.

- **`Undefined`**

  > 1. DW_CFA_undefined
  >
  > The DW_CFA_undefined instruction takes a single unsigned LEB128
  > operand that represents a register number. The required action is to
  > set the rule for the specified register to “undefined.”

- **`SameValue`**

  > 2. DW_CFA_same_value
  >
  > The DW_CFA_same_value instruction takes a single unsigned LEB128
  > operand that represents a register number. The required action is to
  > set the rule for the specified register to “same value.”

- **`Offset`**

  The `Offset` instruction represents both `DW_CFA_offset` and
  `DW_CFA_offset_extended`.
  
  > 3. DW_CFA_offset
  >
  > The DW_CFA_offset instruction takes two operands: a register number
  > (encoded with the opcode) and an unsigned LEB128 constant representing
  > a factored offset. The required action is to change the rule for the
  > register indicated by the register number to be an offset(N) rule
  > where the value of N is factored offset * data_alignment_factor.

- **`OffsetExtendedSf`**

  > 5. DW_CFA_offset_extended_sf
  >
  > The DW_CFA_offset_extended_sf instruction takes two operands: an
  > unsigned LEB128 value representing a register number and a signed
  > LEB128 factored offset. This instruction is identical to
  > DW_CFA_offset_extended except that the second operand is signed and
  > factored. The resulting offset is factored_offset *
  > data_alignment_factor.

- **`ValOffset`**

  > 6. DW_CFA_val_offset
  >
  > The DW_CFA_val_offset instruction takes two unsigned LEB128 operands
  > representing a register number and a factored offset. The required
  > action is to change the rule for the register indicated by the
  > register number to be a val_offset(N) rule where the value of N is
  > factored_offset * data_alignment_factor.

- **`ValOffsetSf`**

  > 7. DW_CFA_val_offset_sf
  >
  > The DW_CFA_val_offset_sf instruction takes two operands: an unsigned
  > LEB128 value representing a register number and a signed LEB128
  > factored offset. This instruction is identical to DW_CFA_val_offset
  > except that the second operand is signed and factored. The resulting
  > offset is factored_offset * data_alignment_factor.

- **`Register`**

  > 8. DW_CFA_register
  >
  > The DW_CFA_register instruction takes two unsigned LEB128 operands
  > representing register numbers. The required action is to set the rule
  > for the first register to be register(R) where R is the second
  > register.

- **`Expression`**

  > 9. DW_CFA_expression
  >
  > The DW_CFA_expression instruction takes two operands: an unsigned
  > LEB128 value representing a register number, and a DW_FORM_block value
  > representing a DWARF expression. The required action is to change the
  > rule for the register indicated by the register number to be an
  > expression(E) rule where E is the DWARF expression. That is, the DWARF
  > expression computes the address. The value of the CFA is pushed on the
  > DWARF evaluation stack prior to execution of the DWARF expression.

- **`ValExpression`**

  > 10. DW_CFA_val_expression
  >
  > The DW_CFA_val_expression instruction takes two operands: an unsigned
  > LEB128 value representing a register number, and a DW_FORM_block value
  > representing a DWARF expression. The required action is to change the
  > rule for the register indicated by the register number to be a
  > val_expression(E) rule where E is the DWARF expression. That is, the
  > DWARF expression computes the value of the given register. The value
  > of the CFA is pushed on the DWARF evaluation stack prior to execution
  > of the DWARF expression.

- **`Restore`**

  The `Restore` instruction represents both `DW_CFA_restore` and
  `DW_CFA_restore_extended`.
  
  > 11. DW_CFA_restore
  >
  > The DW_CFA_restore instruction takes a single operand (encoded with
  > the opcode) that represents a register number. The required action is
  > to change the rule for the indicated register to the rule assigned it
  > by the initial_instructions in the CIE.

- **`RememberState`**

  > 1. DW_CFA_remember_state
  >
  > The DW_CFA_remember_state instruction takes no operands. The required
  > action is to push the set of rules for every register onto an implicit
  > stack.

- **`RestoreState`**

  > 2. DW_CFA_restore_state
  >
  > The DW_CFA_restore_state instruction takes no operands. The required
  > action is to pop the set of rules off the implicit stack and place
  > them in the current row.

- **`ArgsSize`**

  > DW_CFA_GNU_args_size
  >
  > GNU Extension
  >
  > The DW_CFA_GNU_args_size instruction takes an unsigned LEB128 operand
  > representing an argument size. This instruction specifies the total of
  > the size of the arguments which have been pushed onto the stack.

- **`NegateRaState`**

  > DW_CFA_AARCH64_negate_ra_state
  >
  > AArch64 Extension
  >
  > The DW_CFA_AARCH64_negate_ra_state operation negates bit 0 of the
  > RA_SIGN_STATE pseudo-register. It does not take any operands. The
  > DW_CFA_AARCH64_negate_ra_state must not be mixed with other DWARF Register
  > Rule Instructions on the RA_SIGN_STATE pseudo-register in one Common
  > Information Entry (CIE) and Frame Descriptor Entry (FDE) program sequence.

- **`Nop`**

  > 1. DW_CFA_nop
  >
  > The DW_CFA_nop instruction has no operands and no required actions. It
  > is used as padding to make a CIE or FDE an appropriate size.

#### Implementations

- <span id="callframeinstruction-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, address_encoding: Option<DwEhPe>, parameters: &PointerEncodingParameters<'_, R>, vendor: Vendor) -> Result<CallFrameInstruction<T>>` — [`DwEhPe`](../index.md#dwehpe), [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters), [`Vendor`](../index.md#vendor), [`Result`](../index.md#result), [`CallFrameInstruction`](#callframeinstruction)

#### Trait Implementations

##### `impl<T> Any for CallFrameInstruction<T>`

- <span id="callframeinstruction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CallFrameInstruction<T>`

- <span id="callframeinstruction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CallFrameInstruction<T>`

- <span id="callframeinstruction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + ReaderOffset> Clone for CallFrameInstruction<T>`

- <span id="callframeinstruction-clone"></span>`fn clone(&self) -> CallFrameInstruction<T>` — [`CallFrameInstruction`](#callframeinstruction)

##### `impl<T> CloneToUninit for CallFrameInstruction<T>`

- <span id="callframeinstruction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug + ReaderOffset> Debug for CallFrameInstruction<T>`

- <span id="callframeinstruction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for CallFrameInstruction<T>`

##### `impl<T> From for CallFrameInstruction<T>`

- <span id="callframeinstruction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CallFrameInstruction<T>`

- <span id="callframeinstruction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for CallFrameInstruction<T>`

- <span id="callframeinstruction-partialeq-eq"></span>`fn eq(&self, other: &CallFrameInstruction<T>) -> bool` — [`CallFrameInstruction`](#callframeinstruction)

##### `impl<T: ReaderOffset> StructuralPartialEq for CallFrameInstruction<T>`

##### `impl<T> ToOwned for CallFrameInstruction<T>`

- <span id="callframeinstruction-toowned-type-owned"></span>`type Owned = T`

- <span id="callframeinstruction-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="callframeinstruction-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for CallFrameInstruction<T>`

- <span id="callframeinstruction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="callframeinstruction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CallFrameInstruction<T>`

- <span id="callframeinstruction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="callframeinstruction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Pointer`

```rust
enum Pointer {
    Direct(u64),
    Indirect(u64),
}
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3577-3588`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3577-L3588)*

A decoded pointer.

#### Variants

- **`Direct`**

  This value is the decoded pointer value.

- **`Indirect`**

  This value is *not* the pointer value, but points to the address of
  where the real pointer value lives. In other words, deref this pointer
  to get the real pointer value.
  
  Chase this pointer at your own risk: do you trust the DWARF data it came
  from?

#### Implementations

- <span id="pointer-new"></span>`fn new(encoding: constants::DwEhPe, address: u64) -> Pointer` — [`DwEhPe`](../index.md#dwehpe), [`Pointer`](#pointer)

- <span id="pointer-direct"></span>`fn direct(self) -> Result<u64>` — [`Result`](../index.md#result)

  Return the direct pointer value.

- <span id="pointer-pointer"></span>`fn pointer(self) -> u64`

  Return the pointer value, discarding indirectness information.

#### Trait Implementations

##### `impl Any for Pointer`

- <span id="pointer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pointer`

- <span id="pointer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pointer`

- <span id="pointer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pointer`

- <span id="pointer-clone"></span>`fn clone(&self) -> Pointer` — [`Pointer`](#pointer)

##### `impl CloneToUninit for Pointer`

- <span id="pointer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Pointer`

##### `impl Debug for Pointer`

- <span id="pointer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pointer`

- <span id="pointer-default"></span>`fn default() -> Self`

##### `impl Eq for Pointer`

##### `impl<T> From for Pointer`

- <span id="pointer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Pointer`

- <span id="pointer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Pointer`

- <span id="pointer-partialeq-eq"></span>`fn eq(&self, other: &Pointer) -> bool` — [`Pointer`](#pointer)

##### `impl StructuralPartialEq for Pointer`

##### `impl ToOwned for Pointer`

- <span id="pointer-toowned-type-owned"></span>`type Owned = T`

- <span id="pointer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pointer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Pointer`

- <span id="pointer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pointer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pointer`

- <span id="pointer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pointer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeIterInner<R: Reader>`

```rust
enum RangeIterInner<R: Reader> {
    Single(Option<crate::read::Range>),
    List(crate::read::RngListIter<R>),
}
```

*Defined in [`gimli-0.32.3/src/read/dwarf.rs:1633-1636`](../../../.source_1765633015/gimli-0.32.3/src/read/dwarf.rs#L1633-L1636)*

#### Trait Implementations

##### `impl Any for RangeIterInner<R>`

- <span id="rangeiterinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeIterInner<R>`

- <span id="rangeiterinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeIterInner<R>`

- <span id="rangeiterinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for RangeIterInner<R>`

- <span id="rangeiterinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RangeIterInner<R>`

- <span id="rangeiterinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeIterInner<R>`

- <span id="rangeiterinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RangeIterInner<R>`

- <span id="rangeiterinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangeiterinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeIterInner<R>`

- <span id="rangeiterinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangeiterinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AbbreviationsCacheStrategy`

```rust
enum AbbreviationsCacheStrategy {
    Duplicates,
    All,
}
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:99-108`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L99-L108)*

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

- <span id="abbreviationscachestrategy-clone"></span>`fn clone(&self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)

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

- <span id="abbreviationscachestrategy-partialeq-eq"></span>`fn eq(&self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)

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

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:391-397`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L391-L397)*

A list of attributes found in an `Abbreviation`

#### Implementations

- <span id="attributes-new"></span>`fn new() -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

  Returns a new empty list of attributes

- <span id="attributes-push"></span>`fn push(&mut self, attr: AttributeSpecification)` — [`AttributeSpecification`](#attributespecification)

  Pushes a new value onto this list of attributes.

#### Trait Implementations

##### `impl Any for Attributes`

- <span id="attributes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Attributes`

- <span id="attributes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Attributes`

- <span id="attributes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Attributes`

- <span id="attributes-clone"></span>`fn clone(&self) -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

##### `impl CloneToUninit for Attributes`

- <span id="attributes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Attributes`

- <span id="attributes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- <span id="attributes-deref-type-target"></span>`type Target = [AttributeSpecification]`

- <span id="attributes-deref"></span>`fn deref(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](#attributespecification)

##### `impl Eq for Attributes`

##### `impl<T> From for Attributes`

- <span id="attributes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

##### `impl<U> Into for Attributes`

- <span id="attributes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Attributes`

- <span id="attributes-partialeq-eq"></span>`fn eq(&self, other: &Attributes) -> bool` — [`Attributes`](abbrev/index.md#attributes)

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

### `IndexSectionId`

```rust
enum IndexSectionId {
    DebugAbbrev,
    DebugInfo,
    DebugLine,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugRngLists,
    DebugStrOffsets,
    DebugTypes,
}
```

*Defined in [`gimli-0.32.3/src/read/index.rs:342-363`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L342-L363)*

Section kinds which are permitted in a `.dwp` index.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev.dwo` section.

- **`DebugInfo`**

  The `.debug_info.dwo` section.

- **`DebugLine`**

  The `.debug_line.dwo` section.

- **`DebugLoc`**

  The `.debug_loc.dwo` section.

- **`DebugLocLists`**

  The `.debug_loclists.dwo` section.

- **`DebugMacinfo`**

  The `.debug_macinfo.dwo` section.

- **`DebugMacro`**

  The `.debug_macro.dwo` section.

- **`DebugRngLists`**

  The `.debug_rnglists.dwo` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets.dwo` section.

- **`DebugTypes`**

  The `.debug_types.dwo` section.

#### Implementations

- <span id="indexsectionid-section-id"></span>`fn section_id(self) -> SectionId` — [`SectionId`](../index.md#sectionid)

  Returns the corresponding `SectionId`.

- <span id="indexsectionid-dwo-name"></span>`fn dwo_name(self) -> &'static str`

  Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

#### Trait Implementations

##### `impl Any for IndexSectionId`

- <span id="indexsectionid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IndexSectionId`

- <span id="indexsectionid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IndexSectionId`

- <span id="indexsectionid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for IndexSectionId`

- <span id="indexsectionid-clone"></span>`fn clone(&self) -> IndexSectionId` — [`IndexSectionId`](#indexsectionid)

##### `impl CloneToUninit for IndexSectionId`

- <span id="indexsectionid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for IndexSectionId`

##### `impl Debug for IndexSectionId`

- <span id="indexsectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IndexSectionId`

##### `impl<T> From for IndexSectionId`

- <span id="indexsectionid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IndexSectionId`

- <span id="indexsectionid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for IndexSectionId`

- <span id="indexsectionid-partialeq-eq"></span>`fn eq(&self, other: &IndexSectionId) -> bool` — [`IndexSectionId`](#indexsectionid)

##### `impl StructuralPartialEq for IndexSectionId`

##### `impl ToOwned for IndexSectionId`

- <span id="indexsectionid-toowned-type-owned"></span>`type Owned = T`

- <span id="indexsectionid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="indexsectionid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IndexSectionId`

- <span id="indexsectionid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="indexsectionid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IndexSectionId`

- <span id="indexsectionid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="indexsectionid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LineInstruction<R, Offset>`

```rust
enum LineInstruction<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Special(u8),
    Copy,
    AdvancePc(u64),
    AdvanceLine(i64),
    SetFile(u64),
    SetColumn(u64),
    NegateStatement,
    SetBasicBlock,
    ConstAddPc,
    FixedAddPc(u16),
    SetPrologueEnd,
    SetEpilogueBegin,
    SetIsa(u64),
    UnknownStandard0(constants::DwLns),
    UnknownStandard1(constants::DwLns, u64),
    UnknownStandardN(constants::DwLns, R),
    EndSequence,
    SetAddress(u64),
    DefineFile(FileEntry<R, Offset>),
    SetDiscriminator(u64),
    UnknownExtended(constants::DwLne, R),
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:267-399`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L267-L399)*

A parsed line number program instruction.

#### Variants

- **`Special`**

  > ### 6.2.5.1 Special Opcodes
  >
  > Each ubyte special opcode has the following effect on the state machine:
  >
  >   1. Add a signed integer to the line register.
  >
  >   2. Modify the operation pointer by incrementing the address and
  >      op_index registers as described below.
  >
  >   3. Append a row to the matrix using the current values of the state
  >      machine registers.
  >
  >   4. Set the basic_block register to “false.”
  >
  >   5. Set the prologue_end register to “false.”
  >
  >   6. Set the epilogue_begin register to “false.”
  >
  >   7. Set the discriminator register to 0.
  >
  > All of the special opcodes do those same seven things; they differ from
  > one another only in what values they add to the line, address and
  > op_index registers.

- **`Copy`**

  "[`LineInstruction::Copy`](../index.md) appends a row to the matrix using the current
  values of the state machine registers. Then it sets the discriminator
  register to 0, and sets the basic_block, prologue_end and epilogue_begin
  registers to “false.”"

- **`AdvancePc`**

  "The DW_LNS_advance_pc opcode takes a single unsigned LEB128 operand as
  the operation advance and modifies the address and op_index registers
  [the same as `LineInstruction::Special`]"

- **`AdvanceLine`**

  "The DW_LNS_advance_line opcode takes a single signed LEB128 operand and
  adds that value to the line register of the state machine."

- **`SetFile`**

  "The DW_LNS_set_file opcode takes a single unsigned LEB128 operand and
  stores it in the file register of the state machine."

- **`SetColumn`**

  "The DW_LNS_set_column opcode takes a single unsigned LEB128 operand and
  stores it in the column register of the state machine."

- **`NegateStatement`**

  "The DW_LNS_negate_stmt opcode takes no operands. It sets the is_stmt
  register of the state machine to the logical negation of its current
  value."

- **`SetBasicBlock`**

  "The DW_LNS_set_basic_block opcode takes no operands. It sets the
  basic_block register of the state machine to “true.”"

- **`ConstAddPc`**

  > The DW_LNS_const_add_pc opcode takes no operands. It advances the
  > address and op_index registers by the increments corresponding to
  > special opcode 255.
  >
  > When the line number program needs to advance the address by a small
  > amount, it can use a single special opcode, which occupies a single
  > byte. When it needs to advance the address by up to twice the range of
  > the last special opcode, it can use DW_LNS_const_add_pc followed by a
  > special opcode, for a total of two bytes. Only if it needs to advance
  > the address by more than twice that range will it need to use both
  > DW_LNS_advance_pc and a special opcode, requiring three or more bytes.

- **`FixedAddPc`**

  > The DW_LNS_fixed_advance_pc opcode takes a single uhalf (unencoded)
  > operand and adds it to the address register of the state machine and
  > sets the op_index register to 0. This is the only standard opcode whose
  > operand is not a variable length number. It also does not multiply the
  > operand by the minimum_instruction_length field of the header.

- **`SetPrologueEnd`**

  "[`LineInstruction::SetPrologueEnd`](../index.md) sets the prologue_end register to “true”."

- **`SetEpilogueBegin`**

  "[`LineInstruction::SetEpilogueBegin`](../index.md) sets the epilogue_begin register to
  “true”."

- **`SetIsa`**

  "The DW_LNS_set_isa opcode takes a single unsigned LEB128 operand and
  stores that value in the isa register of the state machine."

- **`UnknownStandard0`**

  An unknown standard opcode with zero operands.

- **`UnknownStandard1`**

  An unknown standard opcode with one operand.

- **`UnknownStandardN`**

  An unknown standard opcode with multiple operands.

- **`EndSequence`**

  > [`LineInstruction::EndSequence`](../index.md) sets the end_sequence register of the state
  > machine to “true” and appends a row to the matrix using the current
  > values of the state-machine registers. Then it resets the registers to
  > the initial values specified above (see Section 6.2.2). Every line
  > number program sequence must end with a DW_LNE_end_sequence instruction
  > which creates a row whose address is that of the byte after the last
  > target machine instruction of the sequence.

- **`SetAddress`**

  > The DW_LNE_set_address opcode takes a single relocatable address as an
  > operand. The size of the operand is the size of an address on the target
  > machine. It sets the address register to the value given by the
  > relocatable address and sets the op_index register to 0.
  >
  > All of the other line number program opcodes that affect the address
  > register add a delta to it. This instruction stores a relocatable value
  > into it instead.

- **`DefineFile`**

  Defines a new source file in the line number program and appends it to
  the line number program header's list of source files.

- **`SetDiscriminator`**

  "The DW_LNE_set_discriminator opcode takes a single parameter, an
  unsigned LEB128 integer. It sets the discriminator register to the new
  value."

- **`UnknownExtended`**

  An unknown extended opcode and the slice of its unparsed operands.

#### Implementations

- <span id="lineinstruction-parse"></span>`fn parse<'header>(header: &'header LineProgramHeader<R>, input: &mut R) -> Result<LineInstruction<R>>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md#result), [`LineInstruction`](#lineinstruction)

#### Trait Implementations

##### `impl Any for LineInstruction<R, Offset>`

- <span id="lineinstruction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LineInstruction<R, Offset>`

- <span id="lineinstruction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LineInstruction<R, Offset>`

- <span id="lineinstruction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for LineInstruction<R, Offset>`

- <span id="lineinstruction-clone"></span>`fn clone(&self) -> LineInstruction<R, Offset>` — [`LineInstruction`](#lineinstruction)

##### `impl CloneToUninit for LineInstruction<R, Offset>`

- <span id="lineinstruction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for LineInstruction<R, Offset>`

##### `impl<R, Offset> Debug for LineInstruction<R, Offset>`

- <span id="lineinstruction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for LineInstruction<R, Offset>`

##### `impl<T> From for LineInstruction<R, Offset>`

- <span id="lineinstruction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LineInstruction<R, Offset>`

- <span id="lineinstruction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for LineInstruction<R, Offset>`

- <span id="lineinstruction-partialeq-eq"></span>`fn eq(&self, other: &LineInstruction<R, Offset>) -> bool` — [`LineInstruction`](#lineinstruction)

##### `impl<R, Offset> StructuralPartialEq for LineInstruction<R, Offset>`

##### `impl ToOwned for LineInstruction<R, Offset>`

- <span id="lineinstruction-toowned-type-owned"></span>`type Owned = T`

- <span id="lineinstruction-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lineinstruction-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LineInstruction<R, Offset>`

- <span id="lineinstruction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lineinstruction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LineInstruction<R, Offset>`

- <span id="lineinstruction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lineinstruction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ColumnType`

```rust
enum ColumnType {
    LeftEdge,
    Column(core::num::NonZeroU64),
}
```

*Defined in [`gimli-0.32.3/src/read/line.rs:961-967`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L961-L967)*

The type of column that a row is referring to.

#### Variants

- **`LeftEdge`**

  The `LeftEdge` means that the statement begins at the start of the new
  line.

- **`Column`**

  A column number, whose range begins at 1.

#### Trait Implementations

##### `impl Any for ColumnType`

- <span id="columntype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ColumnType`

- <span id="columntype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ColumnType`

- <span id="columntype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ColumnType`

- <span id="columntype-clone"></span>`fn clone(&self) -> ColumnType` — [`ColumnType`](#columntype)

##### `impl CloneToUninit for ColumnType`

- <span id="columntype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ColumnType`

##### `impl Debug for ColumnType`

- <span id="columntype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColumnType`

##### `impl<T> From for ColumnType`

- <span id="columntype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ColumnType`

- <span id="columntype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ColumnType`

- <span id="columntype-ord-cmp"></span>`fn cmp(&self, other: &ColumnType) -> cmp::Ordering` — [`ColumnType`](#columntype)

##### `impl PartialEq for ColumnType`

- <span id="columntype-partialeq-eq"></span>`fn eq(&self, other: &ColumnType) -> bool` — [`ColumnType`](#columntype)

##### `impl PartialOrd for ColumnType`

- <span id="columntype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ColumnType) -> option::Option<cmp::Ordering>` — [`ColumnType`](#columntype)

##### `impl StructuralPartialEq for ColumnType`

##### `impl ToOwned for ColumnType`

- <span id="columntype-toowned-type-owned"></span>`type Owned = T`

- <span id="columntype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="columntype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ColumnType`

- <span id="columntype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="columntype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ColumnType`

- <span id="columntype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="columntype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocListsFormat`

```rust
enum LocListsFormat {
    Bare,
    Lle,
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:316-322`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L316-L322)*

#### Variants

- **`Bare`**

  The bare location list format used before DWARF 5.

- **`Lle`**

  The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU
  split dwarf extension.

#### Trait Implementations

##### `impl Any for LocListsFormat`

- <span id="loclistsformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocListsFormat`

- <span id="loclistsformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocListsFormat`

- <span id="loclistsformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LocListsFormat`

- <span id="loclistsformat-clone"></span>`fn clone(&self) -> LocListsFormat` — [`LocListsFormat`](loclists/index.md#loclistsformat)

##### `impl CloneToUninit for LocListsFormat`

- <span id="loclistsformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- <span id="loclistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl<T> From for LocListsFormat`

- <span id="loclistsformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocListsFormat`

- <span id="loclistsformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LocListsFormat`

- <span id="loclistsformat-partialeq-eq"></span>`fn eq(&self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](loclists/index.md#loclistsformat)

##### `impl StructuralPartialEq for LocListsFormat`

##### `impl ToOwned for LocListsFormat`

- <span id="loclistsformat-toowned-type-owned"></span>`type Owned = T`

- <span id="loclistsformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loclistsformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocListsFormat`

- <span id="loclistsformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loclistsformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocListsFormat`

- <span id="loclistsformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loclistsformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawLocListEntry<R: Reader>`

```rust
enum RawLocListEntry<R: Reader> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<<R as >::Offset>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        end: crate::common::DebugAddrIndex<<R as >::Offset>,
        data: crate::read::Expression<R>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        length: u64,
        data: crate::read::Expression<R>,
    },
    OffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    DefaultLocation {
        data: crate::read::Expression<R>,
    },
    StartEnd {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    StartLength {
        begin: u64,
        length: u64,
        data: crate::read::Expression<R>,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:337-407`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L337-L407)*

A raw entry in .debug_loclists.

#### Variants

- **`AddressOrOffsetPair`**

  A location from DWARF version <= 4.

- **`BaseAddress`**

  DW_LLE_base_address

- **`BaseAddressx`**

  DW_LLE_base_addressx

- **`StartxEndx`**

  DW_LLE_startx_endx

- **`StartxLength`**

  DW_LLE_startx_length

- **`OffsetPair`**

  DW_LLE_offset_pair

- **`DefaultLocation`**

  DW_LLE_default_location

- **`StartEnd`**

  DW_LLE_start_end

- **`StartLength`**

  DW_LLE_start_length

#### Implementations

- <span id="rawloclistentry-parse"></span>`fn parse(input: &mut R, encoding: Encoding, format: LocListsFormat) -> Result<Option<Self>>` — [`Encoding`](../index.md#encoding), [`LocListsFormat`](loclists/index.md#loclistsformat), [`Result`](../index.md#result)

  Parse a location list entry from `.debug_loclists`

#### Trait Implementations

##### `impl Any for RawLocListEntry<R>`

- <span id="rawloclistentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawLocListEntry<R>`

- <span id="rawloclistentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawLocListEntry<R>`

- <span id="rawloclistentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for RawLocListEntry<R>`

- <span id="rawloclistentry-clone"></span>`fn clone(&self) -> RawLocListEntry<R>` — [`RawLocListEntry`](#rawloclistentry)

##### `impl CloneToUninit for RawLocListEntry<R>`

- <span id="rawloclistentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- <span id="rawloclistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawLocListEntry<R>`

- <span id="rawloclistentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawLocListEntry<R>`

- <span id="rawloclistentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RawLocListEntry<R>`

- <span id="rawloclistentry-toowned-type-owned"></span>`type Owned = T`

- <span id="rawloclistentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawloclistentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RawLocListEntry<R>`

- <span id="rawloclistentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawloclistentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawLocListEntry<R>`

- <span id="rawloclistentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawloclistentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroString<R, Offset>`

```rust
enum MacroString<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Direct(R),
    StringPointer(crate::DebugStrOffset<Offset>),
    IndirectStringPointer(crate::DebugStrOffsetsIndex<Offset>),
    Supplementary(crate::DebugStrOffset<Offset>),
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:244-258`](../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L244-L258)*

A string in a macro entry.

#### Variants

- **`Direct`**

  The string is directly embedded in the macro entry

- **`StringPointer`**

  The macro refers to a string in the `.debug_str` section using a `DebugStrOffset`.

- **`IndirectStringPointer`**

  The macro contains an index into an array in the `.debug_str_offsets`
  section, which refers to a string in the `.debug_str` section.

- **`Supplementary`**

  The macro refers to a string in the `.debug_str` section in the supplementary object file

#### Implementations

- <span id="macrostring-string"></span>`fn string(&self, unit: UnitRef<'_, R>) -> Result<R>` — [`UnitRef`](#unitref), [`Result`](../index.md#result)

  Get the string slice from the macro entry.

#### Trait Implementations

##### `impl Any for MacroString<R, Offset>`

- <span id="macrostring-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroString<R, Offset>`

- <span id="macrostring-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroString<R, Offset>`

- <span id="macrostring-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for MacroString<R, Offset>`

- <span id="macrostring-clone"></span>`fn clone(&self) -> MacroString<R, Offset>` — [`MacroString`](#macrostring)

##### `impl CloneToUninit for MacroString<R, Offset>`

- <span id="macrostring-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for MacroString<R, Offset>`

- <span id="macrostring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroString<R, Offset>`

##### `impl<T> From for MacroString<R, Offset>`

- <span id="macrostring-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroString<R, Offset>`

- <span id="macrostring-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for MacroString<R, Offset>`

- <span id="macrostring-partialeq-eq"></span>`fn eq(&self, other: &MacroString<R, Offset>) -> bool` — [`MacroString`](#macrostring)

##### `impl<R, Offset> StructuralPartialEq for MacroString<R, Offset>`

##### `impl ToOwned for MacroString<R, Offset>`

- <span id="macrostring-toowned-type-owned"></span>`type Owned = T`

- <span id="macrostring-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macrostring-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroString<R, Offset>`

- <span id="macrostring-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macrostring-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroString<R, Offset>`

- <span id="macrostring-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macrostring-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MacroEntry<R, Offset>`

```rust
enum MacroEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Define {
        line: u64,
        text: MacroString<R>,
    },
    Undef {
        line: u64,
        name: MacroString<R>,
    },
    StartFile {
        line: u64,
        file: u64,
    },
    EndFile,
    Import {
        offset: crate::DebugMacroOffset<Offset>,
    },
    ImportSup {
        offset: crate::DebugMacroOffset<Offset>,
    },
    VendorExt {
        numeric: u64,
        string: R,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/macros.rs:277-323`](../../../.source_1765633015/gimli-0.32.3/src/read/macros.rs#L277-L323)*

an Entry in the `.debug_macro` section.

#### Variants

- **`Define`**

  A macro definition.

- **`Undef`**

  A macro undefinition.

- **`StartFile`**

  The start of a file.

- **`EndFile`**

  The end of the current included file.

- **`Import`**

  import a macro unit

- **`ImportSup`**

  import a macro unit from the supplementary object file

- **`VendorExt`**

  A vendor-specific extension.

#### Trait Implementations

##### `impl Any for MacroEntry<R, Offset>`

- <span id="macroentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroEntry<R, Offset>`

- <span id="macroentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroEntry<R, Offset>`

- <span id="macroentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for MacroEntry<R, Offset>`

- <span id="macroentry-clone"></span>`fn clone(&self) -> MacroEntry<R, Offset>` — [`MacroEntry`](#macroentry)

##### `impl CloneToUninit for MacroEntry<R, Offset>`

- <span id="macroentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Debug for MacroEntry<R, Offset>`

- <span id="macroentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroEntry<R, Offset>`

##### `impl<T> From for MacroEntry<R, Offset>`

- <span id="macroentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MacroEntry<R, Offset>`

- <span id="macroentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for MacroEntry<R, Offset>`

- <span id="macroentry-partialeq-eq"></span>`fn eq(&self, other: &MacroEntry<R, Offset>) -> bool` — [`MacroEntry`](#macroentry)

##### `impl<R, Offset> StructuralPartialEq for MacroEntry<R, Offset>`

##### `impl ToOwned for MacroEntry<R, Offset>`

- <span id="macroentry-toowned-type-owned"></span>`type Owned = T`

- <span id="macroentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macroentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroEntry<R, Offset>`

- <span id="macroentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macroentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroEntry<R, Offset>`

- <span id="macroentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macroentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DieReference<T>`

```rust
enum DieReference<T> {
    UnitRef(crate::read::UnitOffset<T>),
    DebugInfoRef(crate::common::DebugInfoOffset<T>),
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:15-20`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L15-L20)*

A reference to a DIE, either relative to the current CU or
relative to the section.

#### Variants

- **`UnitRef`**

  A CU-relative reference.

- **`DebugInfoRef`**

  A section-relative reference.

#### Trait Implementations

##### `impl<T> Any for DieReference<T>`

- <span id="diereference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DieReference<T>`

- <span id="diereference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DieReference<T>`

- <span id="diereference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for DieReference<T>`

- <span id="diereference-clone"></span>`fn clone(&self) -> DieReference<T>` — [`DieReference`](#diereference)

##### `impl<T> CloneToUninit for DieReference<T>`

- <span id="diereference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for DieReference<T>`

##### `impl<T: fmt::Debug> Debug for DieReference<T>`

- <span id="diereference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DieReference<T>`

##### `impl<T> From for DieReference<T>`

- <span id="diereference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DieReference<T>`

- <span id="diereference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for DieReference<T>`

- <span id="diereference-partialeq-eq"></span>`fn eq(&self, other: &DieReference<T>) -> bool` — [`DieReference`](#diereference)

##### `impl<T> StructuralPartialEq for DieReference<T>`

##### `impl<T> ToOwned for DieReference<T>`

- <span id="diereference-toowned-type-owned"></span>`type Owned = T`

- <span id="diereference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="diereference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for DieReference<T>`

- <span id="diereference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diereference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DieReference<T>`

- <span id="diereference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diereference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Operation<R, Offset>`

```rust
enum Operation<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Deref {
        base_type: crate::read::UnitOffset<Offset>,
        size: u8,
        space: bool,
    },
    Drop,
    Pick {
        index: u8,
    },
    Swap,
    Rot,
    Abs,
    And,
    Div,
    Minus,
    Mod,
    Mul,
    Neg,
    Not,
    Or,
    Plus,
    PlusConstant {
        value: u64,
    },
    Shl,
    Shr,
    Shra,
    Xor,
    Bra {
        target: i16,
    },
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
    Skip {
        target: i16,
    },
    UnsignedConstant {
        value: u64,
    },
    SignedConstant {
        value: i64,
    },
    Register {
        register: crate::common::Register,
    },
    RegisterOffset {
        register: crate::common::Register,
        offset: i64,
        base_type: crate::read::UnitOffset<Offset>,
    },
    FrameOffset {
        offset: i64,
    },
    Nop,
    PushObjectAddress,
    Call {
        offset: DieReference<Offset>,
    },
    TLS,
    CallFrameCFA,
    Piece {
        size_in_bits: u64,
        bit_offset: Option<u64>,
    },
    ImplicitValue {
        data: R,
    },
    StackValue,
    ImplicitPointer {
        value: crate::common::DebugInfoOffset<Offset>,
        byte_offset: i64,
    },
    EntryValue {
        expression: R,
    },
    ParameterRef {
        offset: crate::read::UnitOffset<Offset>,
    },
    Address {
        address: u64,
    },
    AddressIndex {
        index: crate::common::DebugAddrIndex<Offset>,
    },
    ConstantIndex {
        index: crate::common::DebugAddrIndex<Offset>,
    },
    TypedLiteral {
        base_type: crate::read::UnitOffset<Offset>,
        value: R,
    },
    Convert {
        base_type: crate::read::UnitOffset<Offset>,
    },
    Reinterpret {
        base_type: crate::read::UnitOffset<Offset>,
    },
    WasmLocal {
        index: u32,
    },
    WasmGlobal {
        index: u32,
    },
    WasmStack {
        index: u32,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:34-293`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L34-L293)*

A single decoded DWARF expression operation.

DWARF expression evaluation is done in two parts: first the raw
bytes of the next part of the expression are decoded; and then the
decoded operation is evaluated.  This approach lets other
consumers inspect the DWARF expression without reimplementing the
decoding operation.

Multiple DWARF opcodes may decode into a single `Operation`.  For
example, both `DW_OP_deref` and `DW_OP_xderef` are represented
using `Operation::Deref`.

#### Variants

- **`Deref`**

  Dereference the topmost value of the stack.

- **`Drop`**

  Drop an item from the stack.

- **`Pick`**

  Pick an item from the stack and push it on top of the stack.
  This operation handles `DW_OP_pick`, `DW_OP_dup`, and
  `DW_OP_over`.

- **`Swap`**

  Swap the top two stack items.

- **`Rot`**

  Rotate the top three stack items.

- **`Abs`**

  Take the absolute value of the top of the stack.

- **`And`**

  Bitwise `and` of the top two values on the stack.

- **`Div`**

  Divide the top two values on the stack.

- **`Minus`**

  Subtract the top two values on the stack.

- **`Mod`**

  Modulus of the top two values on the stack.

- **`Mul`**

  Multiply the top two values on the stack.

- **`Neg`**

  Negate the top of the stack.

- **`Not`**

  Bitwise `not` of the top of the stack.

- **`Or`**

  Bitwise `or` of the top two values on the stack.

- **`Plus`**

  Add the top two values on the stack.

- **`PlusConstant`**

  Add a constant to the topmost value on the stack.

- **`Shl`**

  Logical left shift of the 2nd value on the stack by the number
  of bits given by the topmost value on the stack.

- **`Shr`**

  Right shift of the 2nd value on the stack by the number of
  bits given by the topmost value on the stack.

- **`Shra`**

  Arithmetic left shift of the 2nd value on the stack by the
  number of bits given by the topmost value on the stack.

- **`Xor`**

  Bitwise `xor` of the top two values on the stack.

- **`Bra`**

  Branch to the target location if the top of stack is nonzero.

- **`Eq`**

  Compare the top two stack values for equality.

- **`Ge`**

  Compare the top two stack values using `>=`.

- **`Gt`**

  Compare the top two stack values using `>`.

- **`Le`**

  Compare the top two stack values using `<=`.

- **`Lt`**

  Compare the top two stack values using `<`.

- **`Ne`**

  Compare the top two stack values using `!=`.

- **`Skip`**

  Unconditional branch to the target location.

- **`UnsignedConstant`**

  Push an unsigned constant value on the stack.  This handles multiple
  DWARF opcodes.

- **`SignedConstant`**

  Push a signed constant value on the stack.  This handles multiple
  DWARF opcodes.

- **`Register`**

  Indicate that this piece's location is in the given register.
  
  Completes the piece or expression.

- **`RegisterOffset`**

  Find the value of the given register, add the offset, and then
  push the resulting sum on the stack.

- **`FrameOffset`**

  Compute the frame base (using `DW_AT_frame_base`), add the
  given offset, and then push the resulting sum on the stack.

- **`Nop`**

  No operation.

- **`PushObjectAddress`**

  Push the object address on the stack.

- **`Call`**

  Evaluate a DWARF expression as a subroutine.  The expression
  comes from the `DW_AT_location` attribute of the indicated
  DIE.

- **`TLS`**

  Compute the address of a thread-local variable and push it on
  the stack.

- **`CallFrameCFA`**

  Compute the call frame CFA and push it on the stack.

- **`Piece`**

  Terminate a piece.

- **`ImplicitValue`**

  The object has no location, but has a known constant value.
  
  Represents `DW_OP_implicit_value`.
  Completes the piece or expression.

- **`StackValue`**

  The object has no location, but its value is at the top of the stack.
  
  Represents `DW_OP_stack_value`.
  Completes the piece or expression.

- **`ImplicitPointer`**

  The object is a pointer to a value which has no actual location,
  such as an implicit value or a stack value.
  
  Represents `DW_OP_implicit_pointer`.
  Completes the piece or expression.

- **`EntryValue`**

  Evaluate an expression at the entry to the current subprogram, and push it on the stack.
  
  Represents `DW_OP_entry_value`.

- **`ParameterRef`**

  This represents a parameter that was optimized out.
  
  The offset points to the definition of the parameter, and is
  matched to the `DW_TAG_GNU_call_site_parameter` in the caller that also
  points to the same definition of the parameter.
  
  Represents `DW_OP_GNU_parameter_ref`.

- **`Address`**

  Relocate the address if needed, and push it on the stack.
  
  Represents `DW_OP_addr`.

- **`AddressIndex`**

  Read the address at the given index in `.debug_addr, relocate the address if needed,
  and push it on the stack.
  
  Represents `DW_OP_addrx`.

- **`ConstantIndex`**

  Read the address at the given index in `.debug_addr, and push it on the stack.
  Do not relocate the address.
  
  Represents `DW_OP_constx`.

- **`TypedLiteral`**

  Interpret the value bytes as a constant of a given type, and push it on the stack.
  
  Represents `DW_OP_const_type`.

- **`Convert`**

  Pop the top stack entry, convert it to a different type, and push it on the stack.
  
  Represents `DW_OP_convert`.

- **`Reinterpret`**

  Pop the top stack entry, reinterpret the bits in its value as a different type,
  and push it on the stack.
  
  Represents `DW_OP_reinterpret`.

- **`WasmLocal`**

  The index of a local in the currently executing function.
  
  Represents `DW_OP_WASM_location 0x00`.
  Completes the piece or expression.

- **`WasmGlobal`**

  The index of a global.
  
  Represents `DW_OP_WASM_location 0x01` or `DW_OP_WASM_location 0x03`.
  Completes the piece or expression.

- **`WasmStack`**

  The index of an item on the operand stack.
  
  Represents `DW_OP_WASM_location 0x02`.
  Completes the piece or expression.

#### Implementations

- <span id="operation-parse"></span>`fn parse(bytes: &mut R, encoding: Encoding) -> Result<Operation<R, Offset>>` — [`Encoding`](../index.md#encoding), [`Result`](../index.md#result), [`Operation`](#operation)

  Parse a single DWARF expression operation.

  

  This is useful when examining a DWARF expression for reasons other

  than direct evaluation.

  

  `bytes` points to a the operation to decode.  It should point into

  the same array as `bytecode`, which should be the entire

  expression.

#### Trait Implementations

##### `impl Any for Operation<R, Offset>`

- <span id="operation-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Operation<R, Offset>`

- <span id="operation-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Operation<R, Offset>`

- <span id="operation-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for Operation<R, Offset>`

- <span id="operation-clone"></span>`fn clone(&self) -> Operation<R, Offset>` — [`Operation`](#operation)

##### `impl CloneToUninit for Operation<R, Offset>`

- <span id="operation-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for Operation<R, Offset>`

##### `impl<R, Offset> Debug for Operation<R, Offset>`

- <span id="operation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for Operation<R, Offset>`

##### `impl<T> From for Operation<R, Offset>`

- <span id="operation-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Operation<R, Offset>`

- <span id="operation-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for Operation<R, Offset>`

- <span id="operation-partialeq-eq"></span>`fn eq(&self, other: &Operation<R, Offset>) -> bool` — [`Operation`](#operation)

##### `impl<R, Offset> StructuralPartialEq for Operation<R, Offset>`

##### `impl ToOwned for Operation<R, Offset>`

- <span id="operation-toowned-type-owned"></span>`type Owned = T`

- <span id="operation-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="operation-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Operation<R, Offset>`

- <span id="operation-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="operation-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Operation<R, Offset>`

- <span id="operation-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="operation-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OperationEvaluationResult<R: Reader>`

```rust
enum OperationEvaluationResult<R: Reader> {
    Piece,
    Incomplete,
    Complete {
        location: Location<R>,
    },
    Waiting(EvaluationWaiting<R>, EvaluationResult<R>),
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:296-301`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L296-L301)*

#### Trait Implementations

##### `impl Any for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="operationevaluationresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="operationevaluationresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Location<R, Offset>`

```rust
enum Location<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Empty,
    Register {
        register: crate::common::Register,
    },
    Address {
        address: u64,
    },
    Value {
        value: crate::read::Value,
    },
    Bytes {
        value: R,
    },
    ImplicitPointer {
        value: crate::common::DebugInfoOffset<Offset>,
        byte_offset: i64,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:305-340`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L305-L340)*

A single location of a piece of the result of a DWARF expression.

#### Variants

- **`Empty`**

  The piece is empty.  Ordinarily this means the piece has been
  optimized away.

- **`Register`**

  The piece is found in a register.

- **`Address`**

  The piece is found in memory.

- **`Value`**

  The piece has no location but its value is known.

- **`Bytes`**

  The piece is represented by some constant bytes.

- **`ImplicitPointer`**

  The piece is a pointer to a value which has no actual location.

#### Implementations

- <span id="location-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the piece is empty.

#### Trait Implementations

##### `impl Any for Location<R, Offset>`

- <span id="location-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Location<R, Offset>`

- <span id="location-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Location<R, Offset>`

- <span id="location-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, Offset> Clone for Location<R, Offset>`

- <span id="location-clone"></span>`fn clone(&self) -> Location<R, Offset>` — [`Location`](#location)

##### `impl CloneToUninit for Location<R, Offset>`

- <span id="location-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, Offset> Copy for Location<R, Offset>`

##### `impl<R, Offset> Debug for Location<R, Offset>`

- <span id="location-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Location<R, Offset>`

- <span id="location-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Location<R, Offset>`

- <span id="location-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R, Offset> PartialEq for Location<R, Offset>`

- <span id="location-partialeq-eq"></span>`fn eq(&self, other: &Location<R, Offset>) -> bool` — [`Location`](#location)

##### `impl<R, Offset> StructuralPartialEq for Location<R, Offset>`

##### `impl ToOwned for Location<R, Offset>`

- <span id="location-toowned-type-owned"></span>`type Owned = T`

- <span id="location-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="location-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Location<R, Offset>`

- <span id="location-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="location-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Location<R, Offset>`

- <span id="location-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="location-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EvaluationState<R: Reader>`

```rust
enum EvaluationState<R: Reader> {
    Start(Option<u64>),
    Ready,
    Error(crate::read::Error),
    Complete,
    Waiting(EvaluationWaiting<R>),
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:816-822`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L816-L822)*

#### Trait Implementations

##### `impl Any for EvaluationState<R>`

- <span id="evaluationstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EvaluationState<R>`

- <span id="evaluationstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EvaluationState<R>`

- <span id="evaluationstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationState<R>`

- <span id="evaluationstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EvaluationState<R>`

- <span id="evaluationstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EvaluationState<R>`

- <span id="evaluationstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EvaluationState<R>`

- <span id="evaluationstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="evaluationstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EvaluationState<R>`

- <span id="evaluationstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="evaluationstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EvaluationWaiting<R: Reader>`

```rust
enum EvaluationWaiting<R: Reader> {
    Memory,
    Register {
        offset: i64,
    },
    FrameBase {
        offset: i64,
    },
    Tls,
    Cfa,
    AtLocation,
    EntryValue,
    ParameterRef,
    RelocatedAddress,
    IndexedAddress,
    TypedLiteral {
        value: R,
    },
    Convert,
    Reinterpret,
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:825-839`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L825-L839)*

#### Trait Implementations

##### `impl Any for EvaluationWaiting<R>`

- <span id="evaluationwaiting-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EvaluationWaiting<R>`

- <span id="evaluationwaiting-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EvaluationWaiting<R>`

- <span id="evaluationwaiting-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationWaiting<R>`

- <span id="evaluationwaiting-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EvaluationWaiting<R>`

- <span id="evaluationwaiting-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EvaluationWaiting<R>`

- <span id="evaluationwaiting-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EvaluationWaiting<R>`

- <span id="evaluationwaiting-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="evaluationwaiting-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EvaluationWaiting<R>`

- <span id="evaluationwaiting-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="evaluationwaiting-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EvaluationResult<R: Reader>`

```rust
enum EvaluationResult<R: Reader> {
    Complete,
    RequiresMemory {
        address: u64,
        size: u8,
        space: Option<u64>,
        base_type: crate::read::UnitOffset<<R as >::Offset>,
    },
    RequiresRegister {
        register: crate::common::Register,
        base_type: crate::read::UnitOffset<<R as >::Offset>,
    },
    RequiresFrameBase,
    RequiresTls(u64),
    RequiresCallFrameCfa,
    RequiresAtLocation(DieReference<<R as >::Offset>),
    RequiresEntryValue(Expression<R>),
    RequiresParameterRef(crate::read::UnitOffset<<R as >::Offset>),
    RequiresRelocatedAddress(u64),
    RequiresIndexedAddress {
        index: crate::common::DebugAddrIndex<<R as >::Offset>,
        relocate: bool,
    },
    RequiresBaseType(crate::read::UnitOffset<<R as >::Offset>),
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:845-920`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L845-L920)*

The state of an `Evaluation` after evaluating a DWARF expression.
The evaluation is either `Complete`, or it requires more data
to continue, as described by the variant.

#### Variants

- **`Complete`**

  The `Evaluation` is complete, and `Evaluation::result()` can be called.

- **`RequiresMemory`**

  The `Evaluation` needs a value from memory to proceed further.  Once the
  caller determines what value to provide it should resume the `Evaluation`
  by calling `Evaluation::resume_with_memory`.

- **`RequiresRegister`**

  The `Evaluation` needs a value from a register to proceed further.  Once
  the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_register`.

- **`RequiresFrameBase`**

  The `Evaluation` needs the frame base address to proceed further.  Once
  the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_frame_base`.  The frame
  base address is the address produced by the location description in the
  `DW_AT_frame_base` attribute of the current function.

- **`RequiresTls`**

  The `Evaluation` needs a value from TLS to proceed further.  Once the
  caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_tls`.

- **`RequiresCallFrameCfa`**

  The `Evaluation` needs the CFA to proceed further.  Once the caller
  determines what value to provide it should resume the `Evaluation` by
  calling `Evaluation::resume_with_call_frame_cfa`.

- **`RequiresAtLocation`**

  The `Evaluation` needs the DWARF expression at the given location to
  proceed further.  Once the caller determines what value to provide it
  should resume the `Evaluation` by calling
  `Evaluation::resume_with_at_location`.

- **`RequiresEntryValue`**

  The `Evaluation` needs the value produced by evaluating a DWARF
  expression at the entry point of the current subprogram.  Once the
  caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_entry_value`.

- **`RequiresParameterRef`**

  The `Evaluation` needs the value of the parameter at the given location
  in the current function's caller.  Once the caller determines what value
  to provide it should resume the `Evaluation` by calling
  `Evaluation::resume_with_parameter_ref`.

- **`RequiresRelocatedAddress`**

  The `Evaluation` needs an address to be relocated to proceed further.
  Once the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_relocated_address`.

- **`RequiresIndexedAddress`**

  The `Evaluation` needs an address from the `.debug_addr` section.
  This address may also need to be relocated.
  Once the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_indexed_address`.

- **`RequiresBaseType`**

  The `Evaluation` needs the `ValueType` for the base type DIE at
  the give unit offset.  Once the caller determines what value to provide it
  should resume the `Evaluation` by calling
  `Evaluation::resume_with_base_type`.

#### Trait Implementations

##### `impl Any for EvaluationResult<R>`

- <span id="evaluationresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EvaluationResult<R>`

- <span id="evaluationresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EvaluationResult<R>`

- <span id="evaluationresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationResult<R>`

- <span id="evaluationresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EvaluationResult<R>`

- <span id="evaluationresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EvaluationResult<R>`

- <span id="evaluationresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EvaluationResult<R>`

- <span id="evaluationresult-partialeq-eq"></span>`fn eq(&self, other: &EvaluationResult<R>) -> bool` — [`EvaluationResult`](#evaluationresult)

##### `impl<R: Reader> StructuralPartialEq for EvaluationResult<R>`

##### `impl<U> TryFrom for EvaluationResult<R>`

- <span id="evaluationresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="evaluationresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EvaluationResult<R>`

- <span id="evaluationresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="evaluationresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:294-299`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L294-L299)*

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Any for RangeListsFormat`

- <span id="rangelistsformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeListsFormat`

- <span id="rangelistsformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeListsFormat`

- <span id="rangelistsformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RangeListsFormat`

- <span id="rangelistsformat-clone"></span>`fn clone(&self) -> RangeListsFormat` — [`RangeListsFormat`](rnglists/index.md#rangelistsformat)

##### `impl CloneToUninit for RangeListsFormat`

- <span id="rangelistsformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- <span id="rangelistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl<T> From for RangeListsFormat`

- <span id="rangelistsformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeListsFormat`

- <span id="rangelistsformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RangeListsFormat`

- <span id="rangelistsformat-partialeq-eq"></span>`fn eq(&self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](rnglists/index.md#rangelistsformat)

##### `impl StructuralPartialEq for RangeListsFormat`

##### `impl ToOwned for RangeListsFormat`

- <span id="rangelistsformat-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelistsformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelistsformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RangeListsFormat`

- <span id="rangelistsformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelistsformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeListsFormat`

- <span id="rangelistsformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelistsformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawRngListEntry<T>`

```rust
enum RawRngListEntry<T> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<T>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<T>,
        end: crate::common::DebugAddrIndex<T>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<T>,
        length: u64,
    },
    OffsetPair {
        begin: u64,
        end: u64,
    },
    StartEnd {
        begin: u64,
        end: u64,
    },
    StartLength {
        begin: u64,
        length: u64,
    },
}
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:314-367`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L314-L367)*

A raw entry in .debug_rnglists

#### Variants

- **`AddressOrOffsetPair`**

  A range from DWARF version <= 4.

- **`BaseAddress`**

  DW_RLE_base_address

- **`BaseAddressx`**

  DW_RLE_base_addressx

- **`StartxEndx`**

  DW_RLE_startx_endx

- **`StartxLength`**

  DW_RLE_startx_length

- **`OffsetPair`**

  DW_RLE_offset_pair

- **`StartEnd`**

  DW_RLE_start_end

- **`StartLength`**

  DW_RLE_start_length

#### Implementations

- <span id="rawrnglistentry-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, encoding: Encoding, format: RangeListsFormat) -> Result<Option<Self>>` — [`Encoding`](../index.md#encoding), [`RangeListsFormat`](rnglists/index.md#rangelistsformat), [`Result`](../index.md#result)

  Parse a range entry from `.debug_rnglists`

#### Trait Implementations

##### `impl<T> Any for RawRngListEntry<T>`

- <span id="rawrnglistentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawRngListEntry<T>`

- <span id="rawrnglistentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawRngListEntry<T>`

- <span id="rawrnglistentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RawRngListEntry<T>`

- <span id="rawrnglistentry-clone"></span>`fn clone(&self) -> RawRngListEntry<T>` — [`RawRngListEntry`](#rawrnglistentry)

##### `impl<T> CloneToUninit for RawRngListEntry<T>`

- <span id="rawrnglistentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for RawRngListEntry<T>`

- <span id="rawrnglistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawRngListEntry<T>`

- <span id="rawrnglistentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RawRngListEntry<T>`

- <span id="rawrnglistentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> ToOwned for RawRngListEntry<T>`

- <span id="rawrnglistentry-toowned-type-owned"></span>`type Owned = T`

- <span id="rawrnglistentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawrnglistentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RawRngListEntry<T>`

- <span id="rawrnglistentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawrnglistentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RawRngListEntry<T>`

- <span id="rawrnglistentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawrnglistentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:241-279`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L241-L279)*

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

- <span id="unittype-dw-ut"></span>`fn dw_ut(&self) -> constants::DwUt` — [`DwUt`](../index.md#dwut)

#### Trait Implementations

##### `impl Any for UnitType<Offset>`

- <span id="unittype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitType<Offset>`

- <span id="unittype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitType<Offset>`

- <span id="unittype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Offset> Clone for UnitType<Offset>`

- <span id="unittype-clone"></span>`fn clone(&self) -> UnitType<Offset>` — [`UnitType`](#unittype)

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

- <span id="unittype-partialeq-eq"></span>`fn eq(&self, other: &UnitType<Offset>) -> bool` — [`UnitType`](#unittype)

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

*Defined in [`gimli-0.32.3/src/read/unit.rs:933-1106`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L933-L1106)*

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

- <span id="attributevalue-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](#reader)

  Try to convert this attribute's value to an offset.

- <span id="attributevalue-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attributevalue-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attributevalue-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](#debugstr)

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

- <span id="attributevalue-clone"></span>`fn clone(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](#attributevalue)

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

- <span id="attributevalue-partialeq-eq"></span>`fn eq(&self, other: &AttributeValue<R, Offset>) -> bool` — [`AttributeValue`](#attributevalue)

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

### `ValueType`

```rust
enum ValueType {
    Generic,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}
```

*Defined in [`gimli-0.32.3/src/read/value.rs:26-51`](../../../.source_1765633015/gimli-0.32.3/src/read/value.rs#L26-L51)*

The type of an entry on the DWARF stack.

#### Variants

- **`Generic`**

  The generic type, which is address-sized and of unspecified sign,
  as specified in the DWARF 5 standard, section 2.5.1.
  This type is also used to represent address base types.

- **`I8`**

  Signed 8-bit integer type.

- **`U8`**

  Unsigned 8-bit integer type.

- **`I16`**

  Signed 16-bit integer type.

- **`U16`**

  Unsigned 16-bit integer type.

- **`I32`**

  Signed 32-bit integer type.

- **`U32`**

  Unsigned 32-bit integer type.

- **`I64`**

  Signed 64-bit integer type.

- **`U64`**

  Unsigned 64-bit integer type.

- **`F32`**

  32-bit floating point type.

- **`F64`**

  64-bit floating point type.

#### Implementations

- <span id="valuetype-bit-size"></span>`fn bit_size(self, addr_mask: u64) -> u32`

  The size in bits of a value for this type.

- <span id="valuetype-from-encoding"></span>`fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` — [`DwAte`](../index.md#dwate), [`ValueType`](#valuetype)

  Construct a `ValueType` from the attributes of a base type DIE.

- <span id="valuetype-from-entry"></span>`fn from_entry<R: Reader>(entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<Option<ValueType>>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md#result), [`ValueType`](#valuetype)

  Construct a `ValueType` from a base type DIE.

#### Trait Implementations

##### `impl Any for ValueType`

- <span id="valuetype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueType`

- <span id="valuetype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueType`

- <span id="valuetype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueType`

- <span id="valuetype-clone"></span>`fn clone(&self) -> ValueType` — [`ValueType`](#valuetype)

##### `impl CloneToUninit for ValueType`

- <span id="valuetype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueType`

##### `impl Debug for ValueType`

- <span id="valuetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueType`

##### `impl<T> From for ValueType`

- <span id="valuetype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ValueType`

- <span id="valuetype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ValueType`

- <span id="valuetype-partialeq-eq"></span>`fn eq(&self, other: &ValueType) -> bool` — [`ValueType`](#valuetype)

##### `impl StructuralPartialEq for ValueType`

##### `impl ToOwned for ValueType`

- <span id="valuetype-toowned-type-owned"></span>`type Owned = T`

- <span id="valuetype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuetype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueType`

- <span id="valuetype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuetype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueType`

- <span id="valuetype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuetype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Value`

```rust
enum Value {
    Generic(u64),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
}
```

*Defined in [`gimli-0.32.3/src/read/value.rs:55-78`](../../../.source_1765633015/gimli-0.32.3/src/read/value.rs#L55-L78)*

The value of an entry on the DWARF stack.

#### Variants

- **`Generic`**

  A generic value, which is address-sized and of unspecified sign.

- **`I8`**

  A signed 8-bit integer value.

- **`U8`**

  An unsigned 8-bit integer value.

- **`I16`**

  A signed 16-bit integer value.

- **`U16`**

  An unsigned 16-bit integer value.

- **`I32`**

  A signed 32-bit integer value.

- **`U32`**

  An unsigned 32-bit integer value.

- **`I64`**

  A signed 64-bit integer value.

- **`U64`**

  An unsigned 64-bit integer value.

- **`F32`**

  A 32-bit floating point value.

- **`F64`**

  A 64-bit floating point value.

#### Implementations

- <span id="value-value-type"></span>`fn value_type(&self) -> ValueType` — [`ValueType`](#valuetype)

  Return the `ValueType` corresponding to this `Value`.

- <span id="value-parse"></span>`fn parse<R: Reader>(value_type: ValueType, bytes: R) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Read a `Value` with the given `value_type` from a `Reader`.

- <span id="value-to-u64"></span>`fn to_u64(self, addr_mask: u64) -> Result<u64>` — [`Result`](../index.md#result)

  Convert a `Value` to a `u64`.

  

  The `ValueType` of `self` must be integral.

  Values are sign extended if the source value is signed.

- <span id="value-from-u64"></span>`fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Create a `Value` with the given `value_type` from a `u64` value.

  

  The `value_type` may be integral or floating point.

  The result is truncated if the `u64` value does

  not fit the bounds of the `value_type`.

- <span id="value-from-f32"></span>`fn from_f32(value_type: ValueType, value: f32) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Create a `Value` with the given `value_type` from a `f32` value.

  

  The `value_type` may be integral or floating point.

  The result is not defined if the `f32` value does

  not fit the bounds of the `value_type`.

- <span id="value-from-f64"></span>`fn from_f64(value_type: ValueType, value: f64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Create a `Value` with the given `value_type` from a `f64` value.

  

  The `value_type` may be integral or floating point.

  The result is not defined if the `f64` value does

  not fit the bounds of the `value_type`.

- <span id="value-convert"></span>`fn convert(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Convert a `Value` to the given `value_type`.

  

  When converting between integral types, the result is truncated

  if the source value does not fit the bounds of the `value_type`.

  When converting from floating point types, the result is not defined

  if the source value does not fit the bounds of the `value_type`.

  

  This corresponds to the DWARF `DW_OP_convert` operation.

- <span id="value-reinterpret"></span>`fn reinterpret(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Reinterpret the bits in a `Value` as the given `value_type`.

  

  The source and result value types must have equal sizes.

  

  This corresponds to the DWARF `DW_OP_reinterpret` operation.

- <span id="value-abs"></span>`fn abs(self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md#result), [`Value`](#value)

  Perform an absolute value operation.

  

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_abs` operation.

- <span id="value-neg"></span>`fn neg(self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md#result), [`Value`](#value)

  Perform a negation operation.

  

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_neg` operation.

- <span id="value-add"></span>`fn add(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform an addition operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_plus` operation.

- <span id="value-sub"></span>`fn sub(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a subtraction operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_minus` operation.

- <span id="value-mul"></span>`fn mul(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a multiplication operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_mul` operation.

- <span id="value-div"></span>`fn div(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a division operation.

  

  This operation requires matching types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_div` operation.

- <span id="value-rem"></span>`fn rem(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a remainder operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as an unsigned value.

  

  This corresponds to the DWARF `DW_OP_mod` operation.

- <span id="value-not"></span>`fn not(self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md#result), [`Value`](#value)

  Perform a bitwise not operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_not` operation.

- <span id="value-and"></span>`fn and(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a bitwise and operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_and` operation.

- <span id="value-or"></span>`fn or(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a bitwise or operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_or` operation.

- <span id="value-xor"></span>`fn xor(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a bitwise exclusive-or operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_xor` operation.

- <span id="value-shift-length"></span>`fn shift_length(self) -> Result<u64>` — [`Result`](../index.md#result)

  Convert value to bit length suitable for a shift operation.

  

  If the value is negative then an error is returned.

- <span id="value-shl"></span>`fn shl(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a shift left operation.

  

  This operation requires integral types.

  If the shift length exceeds the type size, then 0 is returned.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shl` operation.

- <span id="value-shr"></span>`fn shr(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a logical shift right operation.

  

  This operation requires an unsigned integral type for the value.

  If the value type is `Generic`, then it is interpreted as an unsigned value.

  

  This operation requires an integral type for the shift length.

  If the shift length exceeds the type size, then 0 is returned.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shr` operation.

- <span id="value-shra"></span>`fn shra(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform an arithmetic shift right operation.

  

  This operation requires a signed integral type for the value.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This operation requires an integral type for the shift length.

  If the shift length exceeds the type size, then 0 is returned for positive values,

  and -1 is returned for negative values.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shra` operation.

- <span id="value-eq"></span>`fn eq(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `==` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_eq` operation.

- <span id="value-ge"></span>`fn ge(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `>=` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_ge` operation.

- <span id="value-gt"></span>`fn gt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `>` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_gt` operation.

- <span id="value-le"></span>`fn le(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `<= relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_le` operation.

- <span id="value-lt"></span>`fn lt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `< relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_lt` operation.

- <span id="value-ne"></span>`fn ne(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `!= relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_ne` operation.

#### Trait Implementations

##### `impl Any for Value`

- <span id="value-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Value`

- <span id="value-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Value`

- <span id="value-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](#value)

##### `impl CloneToUninit for Value`

- <span id="value-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Value`

##### `impl Debug for Value`

- <span id="value-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Value`

- <span id="value-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Value`

- <span id="value-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Value`

- <span id="value-partialeq-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](#value)

##### `impl StructuralPartialEq for Value`

##### `impl ToOwned for Value`

- <span id="value-toowned-type-owned"></span>`type Owned = T`

- <span id="value-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="value-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Value`

- <span id="value-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="value-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Value`

- <span id="value-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="value-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Section<R>`

```rust
trait Section<R>: From<R> { ... }
```

*Defined in [`gimli-0.32.3/src/read/mod.rs:653-708`](../../../.source_1765633015/gimli-0.32.3/src/read/mod.rs#L653-L708)*

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

- `fn reader(&self) -> &R`

  Returns the `Reader` for this section.

#### Provided Methods

- `fn section_name() -> &'static str`

  Returns the ELF section name for this type.

- `fn dwo_section_name() -> Option<&'static str>`

  Returns the ELF section name (if any) for this type when used in a dwo

- `fn xcoff_section_name() -> Option<&'static str>`

  Returns the XCOFF section name (if any) for this type when used in a XCOFF

- `fn load<F, E>(f: F) -> core::result::Result<Self, E>`

  Try to load the section using the given loader function.

- `fn dwp_range(&self, offset: u32, size: u32) -> Result<Self>`

  Returns the subrange of the section that is the contribution of

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>`

  Returns the `Reader` for this section.

#### Implementors

- [`DebugAbbrev`](#debugabbrev)
- [`DebugAddr`](#debugaddr)
- [`DebugAranges`](#debugaranges)
- [`DebugCuIndex`](#debugcuindex)
- [`DebugFrame`](#debugframe)
- [`DebugInfo`](#debuginfo)
- [`DebugLineStr`](#debuglinestr)
- [`DebugLine`](#debugline)
- [`DebugLocLists`](#debugloclists)
- [`DebugLoc`](#debugloc)
- [`DebugMacinfo`](#debugmacinfo)
- [`DebugMacro`](#debugmacro)
- [`DebugPubNames`](#debugpubnames)
- [`DebugPubTypes`](#debugpubtypes)
- [`DebugRanges`](#debugranges)
- [`DebugRngLists`](#debugrnglists)
- [`DebugStrOffsets`](#debugstroffsets)
- [`DebugStr`](#debugstr)
- [`DebugTuIndex`](#debugtuindex)
- [`DebugTypes`](#debugtypes)
- [`EhFrameHdr`](#ehframehdr)
- [`EhFrame`](#ehframe)

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

*Defined in [`gimli-0.32.3/src/read/util.rs:33-42`](../../../.source_1765633015/gimli-0.32.3/src/read/util.rs#L33-L42)*

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Item`

#### Implementors

- `[T; N]`
- `alloc::boxed::Box<[T; N]>`
- `alloc::vec::Vec<T>`

### `UnwindOffset<T>`

```rust
trait UnwindOffset<T>: Copy + Debug + Eq + From<T>
where
    T: ReaderOffset { ... }
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:568-574`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L568-L574)*

An offset into an `UnwindSection`.

#### Required Methods

- `fn into(self) -> T`

  Convert an `UnwindOffset<T>` into a `T`.

#### Implementors

- [`DebugFrameOffset`](../index.md#debugframeoffset)
- [`EhFrameOffset`](../index.md#ehframeoffset)

### `UnwindSection<R: Reader>`

```rust
trait UnwindSection<R: Reader>: Clone + Debug + _UnwindSectionPrivate<R> { ... }
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:635-786`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L635-L786)*

A section holding unwind information: either `.debug_frame` or
`.eh_frame`. See [`DebugFrame`](./struct.DebugFrame.html) and
[`EhFrame`](./struct.EhFrame.html) respectively.

#### Associated Types

- `type Offset: 1`

#### Provided Methods

- `fn entries<'bases>(&self, bases: &'bases BaseAddresses) -> CfiEntriesIter<'bases, Self, R>`

  Iterate over the `CommonInformationEntry`s and `FrameDescriptionEntry`s

- `fn cie_from_offset(&self, bases: &BaseAddresses, offset: <Self as >::Offset) -> Result<CommonInformationEntry<R>>`

  Parse the `CommonInformationEntry` at the given offset.

- `fn partial_fde_from_offset<'bases>(&self, bases: &'bases BaseAddresses, offset: <Self as >::Offset) -> Result<PartialFrameDescriptionEntry<'bases, Self, R>>`

  Parse the `PartialFrameDescriptionEntry` at the given offset.

- `fn fde_from_offset<F>(&self, bases: &BaseAddresses, offset: <Self as >::Offset, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Parse the `FrameDescriptionEntry` at the given offset.

- `fn fde_for_address<F>(&self, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Find the `FrameDescriptionEntry` for the given address.

- `fn unwind_info_for_address<'ctx, F, S>(&self, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>`

  Find the frame unwind information for the given address.

#### Implementors

- [`DebugFrame`](#debugframe)
- [`EhFrame`](#ehframe)

### `UnwindContextStorage<T: ReaderOffset>`

```rust
trait UnwindContextStorage<T: ReaderOffset>: Sized { ... }
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1896-1904`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1896-L1904)*

Specification of what storage should be used for [`UnwindContext`](#unwindcontext).

Normally you would only need to use [`StoreOnHeap`](../index.md), which places the stack
on the heap using [`Box`](../../allocator_api2/stable/boxed/index.md). This is the default storage type parameter for [`UnwindContext`](#unwindcontext).

You may want to supply your own storage type for one of the following reasons:

  1. In rare cases you may run into failed unwinds due to the fixed stack size
     used by [`StoreOnHeap`](../index.md), so you may want to try a larger `Box`. If denial
     of service is not a concern, then you could also try a `Vec`-based stack which
     can grow as needed.
  2. You may want to avoid heap allocations entirely. You can use a fixed-size
     stack with in-line arrays, which will place the entire storage in-line into
     [`UnwindContext`](#unwindcontext).

Here's an implementation which uses a fixed-size stack and allocates everything in-line,
which will cause `UnwindContext` to be large:

```rust,no_run
use gimli::*;

fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
           -> gimli::Result<()> {
let eh_frame: gimli::EhFrame<_> = unreachable!();
let bases = unimplemented!();

struct StoreOnStack;

impl<T: ReaderOffset> UnwindContextStorage<T> for StoreOnStack {
    type Rules = [(Register, RegisterRule<T>); 192];
    type Stack = [UnwindTableRow<T, Self>; 4];
}

let mut ctx = UnwindContext::<_, StoreOnStack>::new_in();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
  let _ = row;
}
unreachable!()
}
```

#### Associated Types

- `type Rules: 1`

- `type Stack: 1`

#### Implementors

- [`StoreOnHeap`](../index.md#storeonheap)

### `ReaderOffset`

```rust
trait ReaderOffset: Debug + Copy + Eq + Ord + Hash + Add<Output = Self> + AddAssign + Sub<Output = Self> { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:24-52`](../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L24-L52)*

A trait for offsets with a DWARF section.

This allows consumers to choose a size that is appropriate for their address space.

#### Required Methods

- `fn from_u8(offset: u8) -> Self`

  Convert a u8 to an offset.

- `fn from_u16(offset: u16) -> Self`

  Convert a u16 to an offset.

- `fn from_i16(offset: i16) -> Self`

  Convert an i16 to an offset.

- `fn from_u32(offset: u32) -> Self`

  Convert a u32 to an offset.

- `fn from_u64(offset: u64) -> Result<Self>`

  Convert a u64 to an offset.

- `fn into_u64(self) -> u64`

  Convert an offset to a u64.

- `fn wrapping_add(self, other: Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`.

- `fn checked_sub(self, other: Self) -> Option<Self>`

  Checked subtraction. Computes `self - other`.

#### Implementors

- `u32`
- `u64`
- `usize`

### `ReaderAddress`

```rust
trait ReaderAddress: Sized { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:194-230`](../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L194-L230)*

A trait for addresses within a DWARF section.

Currently this is a simple extension trait for `u64`, but it may be expanded
in the future to support user-defined address types.

#### Required Methods

- `fn add_sized(self, length: u64, size: u8) -> Result<Self>`

  Add a length to an address of the given size.

- `fn wrapping_add_sized(self, length: u64, size: u8) -> Self`

  Add a length to an address of the given size.

- `fn zeros() -> Self`

  The all-zeros value of an address.

- `fn ones_sized(size: u8) -> Self`

  The all-ones value of an address of the given size.

#### Provided Methods

- `fn min_tombstone(size: u8) -> Self`

  Return the minimum value for a tombstone address.

#### Implementors

- `u64`

### `Reader`

```rust
trait Reader: Debug + Clone { ... }
```

*Defined in [`gimli-0.32.3/src/read/reader.rs:285-581`](../../../.source_1765633015/gimli-0.32.3/src/read/reader.rs#L285-L581)*

A trait for reading the data from a DWARF section.

All read operations advance the section offset of the reader
unless specified otherwise.

## Choosing a `Reader` Implementation

`gimli` comes with a few different `Reader` implementations and lets you
choose the one that is right for your use case. A `Reader` is essentially a
view into the raw bytes that make up some DWARF, but this view might borrow
the underlying data or use reference counting ownership, and it might be
thread safe or not.

| Implementation    | Ownership         | Thread Safe | Notes |
|:------------------|:------------------|:------------|:------|
| [`EndianSlice`](./struct.EndianSlice.html)        | Borrowed          | Yes         | Fastest, but requires that all of your code work with borrows. |
| [`EndianRcSlice`](./struct.EndianRcSlice.html)    | Reference counted | No          | Shared ownership via reference counting, which alleviates the borrow restrictions of `EndianSlice` but imposes reference counting increments and decrements. Cannot be sent across threads, because the reference count is not atomic. |
| [`EndianArcSlice`](./struct.EndianArcSlice.html)  | Reference counted | Yes         | The same as `EndianRcSlice`, but uses atomic reference counting, and therefore reference counting operations are slower but `EndianArcSlice`s may be sent across threads. |
| [`EndianReader<T>`](./struct.EndianReader.html)   | Same as `T`       | Same as `T` | Escape hatch for easily defining your own type of `Reader`. |

#### Associated Types

- `type Endian: 1`

- `type Offset: 1`

#### Required Methods

- `fn endian(&self) -> <Self as >::Endian`

  Return the endianity of bytes that are read.

- `fn len(&self) -> <Self as >::Offset`

  Return the number of bytes remaining.

- `fn empty(&mut self)`

  Set the number of bytes remaining to zero.

- `fn truncate(&mut self, len: <Self as >::Offset) -> Result<()>`

  Set the number of bytes remaining to the specified length.

- `fn offset_from(&self, base: &Self) -> <Self as >::Offset`

  Return the offset of this reader's data relative to the start of

- `fn offset_id(&self) -> ReaderOffsetId`

  Return an identifier for the current reader offset.

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`

  Return the offset corresponding to the given `id` if

- `fn find(&self, byte: u8) -> Result<<Self as >::Offset>`

  Find the index of the first occurrence of the given byte.

- `fn skip(&mut self, len: <Self as >::Offset) -> Result<()>`

  Discard the specified number of bytes.

- `fn split(&mut self, len: <Self as >::Offset) -> Result<Self>`

  Split a reader in two.

- `fn to_slice(&self) -> Result<Cow<'_, [u8]>>`

  Return all remaining data as a clone-on-write slice.

- `fn to_string(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string.

- `fn to_string_lossy(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string, including invalid characters.

- `fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>`

  Read exactly `buf.len()` bytes into `buf`.

#### Provided Methods

- `fn read_u8_array<A>(&mut self) -> Result<A>`

  Read a u8 array.

- `fn is_empty(&self) -> bool`

  Return true if the number of bytes remaining is zero.

- `fn read_u8(&mut self) -> Result<u8>`

  Read a u8.

- `fn read_i8(&mut self) -> Result<i8>`

  Read an i8.

- `fn read_u16(&mut self) -> Result<u16>`

  Read a u16.

- `fn read_i16(&mut self) -> Result<i16>`

  Read an i16.

- `fn read_u32(&mut self) -> Result<u32>`

  Read a u32.

- `fn read_i32(&mut self) -> Result<i32>`

  Read an i32.

- `fn read_u64(&mut self) -> Result<u64>`

  Read a u64.

- `fn read_i64(&mut self) -> Result<i64>`

  Read an i64.

- `fn read_f32(&mut self) -> Result<f32>`

  Read a f32.

- `fn read_f64(&mut self) -> Result<f64>`

  Read a f64.

- `fn read_uint(&mut self, n: usize) -> Result<u64>`

  Read an unsigned n-bytes integer u64.

- `fn read_null_terminated_slice(&mut self) -> Result<Self>`

  Read a null-terminated slice, and return it (excluding the null).

- `fn skip_leb128(&mut self) -> Result<()>`

  Skip a LEB128 encoded integer.

- `fn read_uleb128(&mut self) -> Result<u64>`

  Read an unsigned LEB128 encoded integer.

- `fn read_uleb128_u32(&mut self) -> Result<u32>`

  Read an unsigned LEB128 encoded u32.

- `fn read_uleb128_u16(&mut self) -> Result<u16>`

  Read an unsigned LEB128 encoded u16.

- `fn read_sleb128(&mut self) -> Result<i64>`

  Read a signed LEB128 encoded integer.

- `fn read_initial_length(&mut self) -> Result<(<Self as >::Offset, Format)>`

  Read an initial length field.

- `fn read_address_size(&mut self) -> Result<u8>`

  Read a byte and validate it as an address size.

- `fn read_address(&mut self, address_size: u8) -> Result<u64>`

  Read an address-sized integer, and return it as a `u64`.

- `fn read_word(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized integer according to the DWARF format.

- `fn read_length(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section length according to the DWARF format.

- `fn read_offset(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section offset according to the DWARF format.

- `fn read_sized_offset(&mut self, size: u8) -> Result<<Self as >::Offset>`

  Parse a section offset of the given size.

#### Implementors

- [`EndianSlice`](#endianslice)
- [`RelocateReader`](#relocatereader)

### `Relocate<T: ReaderOffset>`

```rust
trait Relocate<T: ReaderOffset> { ... }
```

*Defined in [`gimli-0.32.3/src/read/relocate.rs:9-15`](../../../.source_1765633015/gimli-0.32.3/src/read/relocate.rs#L9-L15)*

Trait for relocating addresses and offsets while reading a section.

#### Required Methods

- `fn relocate_address(&self, offset: T, value: u64) -> Result<u64>`

  Relocate an address which was read from the given section offset.

- `fn relocate_offset(&self, offset: T, value: T) -> Result<T>`

  Relocate a value which was read from the given section offset.

### `LineProgram<R, Offset>`

```rust
trait LineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset { ... }
```

*Defined in [`gimli-0.32.3/src/read/line.rs:121-130`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L121-L130)*

A `LineProgram` provides access to a `LineProgramHeader` and
a way to add files to the files table if necessary. Gimli consumers should
never need to use or see this trait.

#### Required Methods

- `fn header(&self) -> &LineProgramHeader<R, Offset>`

  Get a reference to the held `LineProgramHeader`.

- `fn add_file(&mut self, file: FileEntry<R, Offset>)`

  Add a file to the file table if necessary.

#### Implementors

- [`IncompleteLineProgram`](#incompletelineprogram)
- `&'program CompleteLineProgram<R, Offset>`

### `EvaluationStorage<R: Reader>`

```rust
trait EvaluationStorage<R: Reader> { ... }
```

*Defined in [`gimli-0.32.3/src/read/op.rs:1044-1051`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L1044-L1051)*

Specification of what storage should be used for [`Evaluation`](#evaluation).

Normally you would only need to use [`StoreOnHeap`](../index.md), which places the stacks and the results
on the heap using [`Vec`](../../addr2line/maybe_small/index.md). This is the default storage type parameter for [`Evaluation`](#evaluation).

If you need to avoid [`Evaluation`](#evaluation) from allocating memory, e.g. for signal safety,
you can provide you own storage specification:
```rust,no_run
use gimli::*;
let bytecode = EndianSlice::new(&[], LittleEndian);
let encoding = unimplemented!();
let get_register_value = |_, _| Value::Generic(42);
let get_frame_base = || 0xdeadbeef;

struct StoreOnStack;

impl<R: Reader> EvaluationStorage<R> for StoreOnStack {
    type Stack = [Value; 64];
    type ExpressionStack = [(R, R); 4];
    type Result = [Piece<R>; 1];
}

let mut eval = Evaluation::<_, StoreOnStack>::new_in(bytecode, encoding);
let mut result = eval.evaluate().unwrap();
while result != EvaluationResult::Complete {
  match result {
    EvaluationResult::RequiresRegister { register, base_type } => {
      let value = get_register_value(register, base_type);
      result = eval.resume_with_register(value).unwrap();
    },
    EvaluationResult::RequiresFrameBase => {
      let frame_base = get_frame_base();
      result = eval.resume_with_frame_base(frame_base).unwrap();
    },
    _ => unimplemented!(),
  };
}

let result = eval.as_result();
println!("{:?}", result);
```

#### Associated Types

- `type Stack: 1`

- `type ExpressionStack: 1`

- `type Result: 1`

#### Implementors

- [`StoreOnHeap`](../index.md#storeonheap)

## Functions

### `parse_cfi_entry`

```rust
fn parse_cfi_entry<'bases, Section, R>(bases: &'bases BaseAddresses, section: &Section, input: &mut R) -> crate::read::Result<Option<CieOrFde<'bases, Section, R>>>
where
    R: Reader,
    Section: UnwindSection<R>
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1072-1116`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1072-L1116)*

### `parse_encoded_pointer`

```rust
fn parse_encoded_pointer<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<Pointer>
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3633-3688`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3633-L3688)*

### `parse_encoded_value`

```rust
fn parse_encoded_value<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<u64>
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3690-3715`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3690-L3715)*

### `get_attribute_size`

```rust
fn get_attribute_size(form: constants::DwForm, encoding: crate::common::Encoding) -> Option<u8>
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:572-637`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L572-L637)*

### `parse_directory_v5`

```rust
fn parse_directory_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<crate::read::AttributeValue<R>>
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1702-1717`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1702-L1717)*

### `parse_file_v5`

```rust
fn parse_file_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<FileEntry<R>>
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1719-1773`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1719-L1773)*

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, form: constants::DwForm) -> crate::read::Result<crate::read::AttributeValue<R>>
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1776-1878`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1776-L1878)*

### `parse_data`

```rust
fn parse_data<R: Reader>(input: &mut R, encoding: crate::common::Encoding) -> crate::read::Result<crate::read::Expression<R>>
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:409-418`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L409-L418)*

### `compute_pc`

```rust
fn compute_pc<R: Reader>(pc: &R, bytecode: &R, offset: i16) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/op.rs:381-391`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L381-L391)*

### `generic_type`

```rust
fn generic_type<O: ReaderOffset>() -> crate::read::UnitOffset<O>
```

*Defined in [`gimli-0.32.3/src/read/op.rs:393-395`](../../../.source_1765633015/gimli-0.32.3/src/read/op.rs#L393-L395)*

### `parse_unit_type`

```rust
fn parse_unit_type<R: Reader>(input: &mut R) -> crate::read::Result<constants::DwUt>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:216-219`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L216-L219)*

Parse the unit type from the unit header.

### `parse_debug_abbrev_offset`

```rust
fn parse_debug_abbrev_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::common::DebugAbbrevOffset<<R as >::Offset>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:222-227`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L222-L227)*

Parse the `debug_abbrev_offset` in the compilation unit header.

### `parse_debug_info_offset`

```rust
fn parse_debug_info_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::common::DebugInfoOffset<<R as >::Offset>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:230-235`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L230-L235)*

Parse the `debug_info_offset` in the arange header.

### `parse_unit_header`

```rust
fn parse_unit_header<R, Offset>(input: &mut R, unit_offset: crate::common::UnitSectionOffset<Offset>) -> crate::read::Result<UnitHeader<R>>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:558-636`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L558-L636)*

Parse a unit header.

### `parse_dwo_id`

```rust
fn parse_dwo_id<R: Reader>(input: &mut R) -> crate::read::Result<crate::common::DwoId>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:639-641`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L639-L641)*

Parse a dwo_id from a header

### `length_u8_value`

```rust
fn length_u8_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1928-1931`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1928-L1931)*

### `length_u16_value`

```rust
fn length_u16_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1933-1936`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1933-L1936)*

### `length_u32_value`

```rust
fn length_u32_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1938-1941`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1938-L1941)*

### `length_uleb128_value`

```rust
fn length_uleb128_value<R: Reader>(input: &mut R) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1943-1946`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1943-L1946)*

### `allow_section_offset`

```rust
fn allow_section_offset(name: constants::DwAt, version: u16) -> bool
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1950-1968`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1950-L1968)*

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, spec: crate::read::AttributeSpecification) -> crate::read::Result<Attribute<R>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:1970-2193`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L1970-L2193)*

### `skip_attributes`

```rust
fn skip_attributes<R: Reader>(input: &mut R, encoding: crate::common::Encoding, specs: &[crate::read::AttributeSpecification]) -> crate::read::Result<()>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:2195-2261`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L2195-L2261)*

### `parse_type_signature`

```rust
fn parse_type_signature<R: Reader>(input: &mut R) -> crate::read::Result<crate::common::DebugTypeSignature>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3049-3051`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L3049-L3051)*

Parse a type unit header's unique type signature. Callers should handle
unique-ness checking.

### `parse_type_offset`

```rust
fn parse_type_offset<R: Reader>(input: &mut R, format: crate::common::Format) -> crate::read::Result<crate::read::UnitOffset<<R as >::Offset>>
```

*Defined in [`gimli-0.32.3/src/read/unit.rs:3054-3056`](../../../.source_1765633015/gimli-0.32.3/src/read/unit.rs#L3054-L3056)*

Parse a type unit header's type offset.

### `sign_extend`

```rust
fn sign_extend(value: u64, mask: u64) -> i64
```

*Defined in [`gimli-0.32.3/src/read/value.rs:13-17`](../../../.source_1765633015/gimli-0.32.3/src/read/value.rs#L13-L17)*

Convert a u64 to an i64, with sign extension if required.

This is primarily used when needing to treat `Value::Generic`
as a signed value.

### `mask_bit_size`

```rust
fn mask_bit_size(addr_mask: u64) -> u32
```

*Defined in [`gimli-0.32.3/src/read/value.rs:20-22`](../../../.source_1765633015/gimli-0.32.3/src/read/value.rs#L20-L22)*

## Type Aliases

### `EndianBuf<'input, Endian>`

```rust
type EndianBuf<'input, Endian> = EndianSlice<'input, Endian>;
```

*Defined in [`gimli-0.32.3/src/read/mod.rs:281`](../../../.source_1765633015/gimli-0.32.3/src/read/mod.rs#L281)*

`EndianBuf` has been renamed to `EndianSlice`. For ease of upgrading across
`gimli` versions, we export this type alias.

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

*Defined in [`gimli-0.32.3/src/read/mod.rs:639`](../../../.source_1765633015/gimli-0.32.3/src/read/mod.rs#L639)*

The result of a parse.

### `LineNumberProgram<R, Offset>`

```rust
type LineNumberProgram<R, Offset> = dyn LineProgram<R, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:116`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L116)*

Deprecated. `LineNumberProgram` has been renamed to `LineProgram`.

### `StateMachine<R, Program, Offset>`

```rust
type StateMachine<R, Program, Offset> = LineRows<R, Program, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:160`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L160)*

Deprecated. `StateMachine` has been renamed to `LineRows`.

### `OneShotLineRows<R, Offset>`

```rust
type OneShotLineRows<R, Offset> = LineRows<R, IncompleteLineProgram<R, Offset>, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:179-180`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L179-L180)*

### `ResumedLineRows<'program, R, Offset>`

```rust
type ResumedLineRows<'program, R, Offset> = LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:182-183`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L182-L183)*

### `Opcode<R>`

```rust
type Opcode<R> = LineInstruction<R, <R as Reader>::Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:263`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L263)*

Deprecated. `Opcode` has been renamed to `LineInstruction`.

### `OpcodesIter<R>`

```rust
type OpcodesIter<R> = LineInstructions<R>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:521`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L521)*

Deprecated. `OpcodesIter` has been renamed to `LineInstructions`.

### `LineNumberRow`

```rust
type LineNumberRow = LineRow;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:574`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L574)*

Deprecated. `LineNumberRow` has been renamed to `LineRow`.

### `LineNumberSequence<R>`

```rust
type LineNumberSequence<R> = LineSequence<R>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:971`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L971)*

Deprecated. `LineNumberSequence` has been renamed to `LineSequence`.

### `LineNumberProgramHeader<R, Offset>`

```rust
type LineNumberProgramHeader<R, Offset> = LineProgramHeader<R, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:991`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L991)*

Deprecated. `LineNumberProgramHeader` has been renamed to `LineProgramHeader`.

### `IncompleteLineNumberProgram<R, Offset>`

```rust
type IncompleteLineNumberProgram<R, Offset> = IncompleteLineProgram<R, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1407`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1407)*

Deprecated. `IncompleteLineNumberProgram` has been renamed to `IncompleteLineProgram`.

### `CompleteLineNumberProgram<R, Offset>`

```rust
type CompleteLineNumberProgram<R, Offset> = CompleteLineProgram<R, Offset>;
```

*Defined in [`gimli-0.32.3/src/read/line.rs:1500`](../../../.source_1765633015/gimli-0.32.3/src/read/line.rs#L1500)*

Deprecated. `CompleteLineNumberProgram` has been renamed to `CompleteLineProgram`.

### `LocListsHeader`

```rust
type LocListsHeader = crate::read::lists::ListsHeader;
```

*Defined in [`gimli-0.32.3/src/read/loclists.rs:131`](../../../.source_1765633015/gimli-0.32.3/src/read/loclists.rs#L131)*

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

*Defined in [`gimli-0.32.3/src/read/rnglists.rs:133`](../../../.source_1765633015/gimli-0.32.3/src/read/rnglists.rs#L133)*

## Constants

### `MAX_RULES`
```rust
const MAX_RULES: usize = 192usize;
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1907`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1907)*

### `MAX_UNWIND_STACK_DEPTH`
```rust
const MAX_UNWIND_STACK_DEPTH: usize = 4usize;
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:1909`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L1909)*

### `CFI_INSTRUCTION_HIGH_BITS_MASK`
```rust
const CFI_INSTRUCTION_HIGH_BITS_MASK: u8 = 192u8;
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3257`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3257)*

### `CFI_INSTRUCTION_LOW_BITS_MASK`
```rust
const CFI_INSTRUCTION_LOW_BITS_MASK: u8 = 63u8;
```

*Defined in [`gimli-0.32.3/src/read/cfi.rs:3258`](../../../.source_1765633015/gimli-0.32.3/src/read/cfi.rs#L3258)*

### `MAX_ATTRIBUTES_INLINE`
```rust
const MAX_ATTRIBUTES_INLINE: usize = 5usize;
```

*Defined in [`gimli-0.32.3/src/read/abbrev.rs:400`](../../../.source_1765633015/gimli-0.32.3/src/read/abbrev.rs#L400)*

### `SECTION_COUNT_MAX`
```rust
const SECTION_COUNT_MAX: u8 = 8u8;
```

*Defined in [`gimli-0.32.3/src/read/index.rs:120`](../../../.source_1765633015/gimli-0.32.3/src/read/index.rs#L120)*

