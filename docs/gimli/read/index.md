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

## Modules

- [`util`](util/index.md) - 
- [`addr`](addr/index.md) - 
- [`cfi`](cfi/index.md) - 
- [`dwarf`](dwarf/index.md) - 
- [`endian_slice`](endian_slice/index.md) - Working with byte slices that have an associated endianity.
- [`reader`](reader/index.md) - 
- [`relocate`](relocate/index.md) - 
- [`abbrev`](abbrev/index.md) - Functions for parsing DWARF debugging abbreviations.
- [`aranges`](aranges/index.md) - 
- [`index`](index/index.md) - 
- [`line`](line/index.md) - 
- [`lists`](lists/index.md) - 
- [`loclists`](loclists/index.md) - 
- [`lookup`](lookup/index.md) - 
- [`macros`](macros/index.md) - 
- [`op`](op/index.md) - Functions for parsing and evaluating DWARF expressions.
- [`pubnames`](pubnames/index.md) - 
- [`pubtypes`](pubtypes/index.md) - 
- [`rnglists`](rnglists/index.md) - 
- [`str`](str/index.md) - 
- [`unit`](unit/index.md) - Functions for parsing DWARF `.debug_info` and `.debug_types` sections.
- [`value`](value/index.md) - Definitions for values used in DWARF expressions.
- [`sealed`](sealed/index.md) - 

## Structs

### `UnitOffset<T>`

```rust
struct UnitOffset<T>(T);
```

An offset into the current compilation or type unit.

#### Implementations

- `fn to_debug_info_offset<R>(self: &Self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>` — [`UnitHeader`](#unitheader), [`DebugInfoOffset`](../index.md)

- `fn to_debug_types_offset<R>(self: &Self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>` — [`UnitHeader`](#unitheader), [`DebugTypesOffset`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for UnitOffset<T>`

- `fn clone(self: &Self) -> UnitOffset<T>` — [`UnitOffset`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for UnitOffset<T>`

##### `impl<T: $crate::fmt::Debug> Debug for UnitOffset<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for UnitOffset<T>`

##### `impl<T: $crate::hash::Hash> Hash for UnitOffset<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T: $crate::cmp::Ord> Ord for UnitOffset<T>`

- `fn cmp(self: &Self, other: &UnitOffset<T>) -> $crate::cmp::Ordering` — [`UnitOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for UnitOffset<T>`

- `fn eq(self: &Self, other: &UnitOffset<T>) -> bool` — [`UnitOffset`](../index.md)

##### `impl<T: $crate::cmp::PartialOrd> PartialOrd for UnitOffset<T>`

- `fn partial_cmp(self: &Self, other: &UnitOffset<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`UnitOffset`](../index.md)

##### `impl<T> StructuralPartialEq for UnitOffset<T>`

### `StoreOnHeap`

```rust
struct StoreOnHeap;
```

Indicates that storage should be allocated on heap.

#### Trait Implementations

##### `impl Clone for StoreOnHeap`

- `fn clone(self: &Self) -> StoreOnHeap` — [`StoreOnHeap`](../index.md)

##### `impl Copy for StoreOnHeap`

##### `impl Debug for StoreOnHeap`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for StoreOnHeap`

##### `impl<R: Reader> EvaluationStorage for crate::read::StoreOnHeap`

- `type Stack = Vec<Value>`

- `type ExpressionStack = Vec<(R, R)>`

- `type Result = Vec<Piece<R>>`

##### `impl PartialEq for StoreOnHeap`

- `fn eq(self: &Self, other: &StoreOnHeap) -> bool` — [`StoreOnHeap`](../index.md)

##### `impl StructuralPartialEq for StoreOnHeap`

##### `impl<T: ReaderOffset> UnwindContextStorage for crate::read::StoreOnHeap`

- `type Rules = [(Register, RegisterRule<T>); 192]`

- `type Stack = Box<[UnwindTableRow<T>; 4]>`

### `ArrayVec<A: ArrayLike>`

```rust
struct ArrayVec<A: ArrayLike> {
    storage: <A as >::Storage,
    len: usize,
}
```

#### Implementations

- `fn into_vec(self: Self) -> Vec<T>`

#### Trait Implementations

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- `fn clone(self: &Self) -> Self`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- `fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- `type Target = [<A as ArrayLike>::Item]`

- `fn deref(self: &Self) -> &[<A as >::Item]` — [`ArrayLike`](#arraylike)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- `fn deref_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`ArrayLike`](#arraylike)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- `fn drop(self: &mut Self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<P, T> Receiver for ArrayVec<A>`

- `type Target = T`

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

The raw contents of the `.debug_addr` section.

#### Implementations

- `fn get_address(self: &Self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../index.md), [`Reader`](#reader), [`DebugAddrIndex`](../index.md), [`Result`](../index.md)

- `fn headers(self: &Self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](#addrheaderiter)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugAddr<R>`

- `fn clone(self: &Self) -> DebugAddr<R>` — [`DebugAddr`](#debugaddr)

##### `impl<R: $crate::marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugAddr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugAddr<R>`

- `fn default() -> DebugAddr<R>` — [`DebugAddr`](#debugaddr)

##### `impl<R> Section for DebugAddr<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../index.md), [`AddrHeader`](#addrheader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- `fn clone(self: &Self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](#addrheaderiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- `fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../index.md), [`Result`](../index.md)

- `fn offset(self: &Self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../index.md)

- `fn length(self: &Self) -> Offset`

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../index.md)

- `fn entries(self: &Self) -> AddrEntryIter<R>` — [`AddrEntryIter`](#addrentryiter)

#### Trait Implementations

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- `fn clone(self: &Self) -> AddrHeader<R, Offset>` — [`AddrHeader`](#addrheader)

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- `fn eq(self: &Self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](#addrheader)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the addresses from a `.debug_addr` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<u64>>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for AddrEntryIter<R>`

- `fn clone(self: &Self) -> AddrEntryIter<R>` — [`AddrEntryIter`](#addrentryiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DebugFrame<R: Reader>`

```rust
struct DebugFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

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

- `fn set_address_size(self: &mut Self, address_size: u8)`

- `fn set_vendor(self: &mut Self, vendor: Vendor)` — [`Vendor`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugFrame<R>`

- `fn clone(self: &Self) -> DebugFrame<R>` — [`DebugFrame`](#debugframe)

##### `impl<R: $crate::marker::Copy + Reader> Copy for DebugFrame<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugFrame<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for DebugFrame<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for DebugFrame<R>`

- `fn eq(self: &Self, other: &DebugFrame<R>) -> bool` — [`DebugFrame`](#debugframe)

##### `impl<R: Reader> Section for DebugFrame<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for DebugFrame<R>`

##### `impl<R: Reader> UnwindSection for DebugFrame<R>`

- `type Offset = DebugFrameOffset<<R as Reader>::Offset>`

### `EhFrameHdr<R: Reader>`

```rust
struct EhFrameHdr<R: Reader>(R);
```

`EhFrameHdr` contains the information about the `.eh_frame_hdr` section.

A pointer to the start of the `.eh_frame` data, and optionally, a binary
search table of pointers to the `.eh_frame` records that are found in this section.

#### Implementations

- `fn parse(self: &Self, bases: &BaseAddresses, address_size: u8) -> Result<ParsedEhFrameHdr<R>>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md), [`ParsedEhFrameHdr`](#parsedehframehdr)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for EhFrameHdr<R>`

- `fn clone(self: &Self) -> EhFrameHdr<R>` — [`EhFrameHdr`](#ehframehdr)

##### `impl<R: $crate::marker::Copy + Reader> Copy for EhFrameHdr<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EhFrameHdr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for EhFrameHdr<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for EhFrameHdr<R>`

- `fn eq(self: &Self, other: &EhFrameHdr<R>) -> bool` — [`EhFrameHdr`](#ehframehdr)

##### `impl<R: Reader> Section for EhFrameHdr<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrameHdr<R>`

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

`ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section.

#### Implementations

- `fn eh_frame_ptr(self: &Self) -> Pointer` — [`Pointer`](#pointer)

- `fn table(self: &Self) -> Option<EhHdrTable<'_, R>>` — [`EhHdrTable`](#ehhdrtable)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for ParsedEhFrameHdr<R>`

- `fn clone(self: &Self) -> ParsedEhFrameHdr<R>` — [`ParsedEhFrameHdr`](#parsedehframehdr)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for ParsedEhFrameHdr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EhHdrTableIter<'a, 'bases, R: Reader>`

```rust
struct EhHdrTableIter<'a, 'bases, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
    table: R,
    bases: &'bases BaseAddresses,
    remain: u64,
}
```

An iterator for `.eh_frame_hdr` section's binary search table.

Each table entry consists of a tuple containing an  `initial_location` and `address`.
The `initial location` represents the first address that the targeted FDE
is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.
The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../index.md), [`Pointer`](#pointer)

- `fn nth(self: &mut Self, n: usize) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../index.md), [`Pointer`](#pointer)

#### Trait Implementations

##### `impl<'a, 'bases, R: $crate::fmt::Debug + Reader> Debug for EhHdrTableIter<'a, 'bases, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EhHdrTable<'a, R: Reader>`

```rust
struct EhHdrTable<'a, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
}
```

The CFI binary search table that is an optional part of the `.eh_frame_hdr` section.

#### Implementations

- `fn iter<'bases>(self: &Self, bases: &'bases BaseAddresses) -> EhHdrTableIter<'_, 'bases, R>` — [`BaseAddresses`](#baseaddresses), [`EhHdrTableIter`](#ehhdrtableiter)

- `fn lookup(self: &Self, address: u64, bases: &BaseAddresses) -> Result<Pointer>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md), [`Pointer`](#pointer)

- `fn pointer_to_offset(self: &Self, ptr: Pointer) -> Result<EhFrameOffset<<R as >::Offset>>` — [`Pointer`](#pointer), [`Result`](../index.md), [`EhFrameOffset`](../index.md), [`Reader`](#reader)

- `fn fde_for_address<F>(self: &Self, frame: &EhFrame<R>, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`EhFrame`](#ehframe), [`BaseAddresses`](#baseaddresses), [`Result`](../index.md), [`FrameDescriptionEntry`](#framedescriptionentry)

- `fn unwind_info_for_address<'ctx, F, S>(self: &Self, frame: &EhFrame<R>, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`EhFrame`](#ehframe), [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md), [`UnwindTableRow`](#unwindtablerow)

#### Trait Implementations

##### `impl<'a, R: $crate::clone::Clone + Reader> Clone for EhHdrTable<'a, R>`

- `fn clone(self: &Self) -> EhHdrTable<'a, R>` — [`EhHdrTable`](#ehhdrtable)

##### `impl<'a, R: $crate::fmt::Debug + Reader> Debug for EhHdrTable<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `EhFrame<R: Reader>`

```rust
struct EhFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

`EhFrame` contains the frame unwinding information needed during exception
handling found in the `.eh_frame` section.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

See
[`DebugFrame`](./struct.DebugFrame.html#differences-between-debug_frame-and-eh_frame)
for some discussion on the differences between `.debug_frame` and
`.eh_frame`.

#### Implementations

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for EhFrame<R>`

- `fn clone(self: &Self) -> EhFrame<R>` — [`EhFrame`](#ehframe)

##### `impl<R: $crate::marker::Copy + Reader> Copy for EhFrame<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EhFrame<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for EhFrame<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for EhFrame<R>`

- `fn eq(self: &Self, other: &EhFrame<R>) -> bool` — [`EhFrame`](#ehframe)

##### `impl<R: Reader> Section for EhFrame<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrame<R>`

##### `impl<R: Reader> UnwindSection for EhFrame<R>`

- `type Offset = EhFrameOffset<<R as Reader>::Offset>`

### `BaseAddresses`

```rust
struct BaseAddresses {
    pub eh_frame_hdr: SectionBaseAddresses,
    pub eh_frame: SectionBaseAddresses,
}
```

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

- `fn set_eh_frame_hdr(self: Self, addr: u64) -> Self`

- `fn set_eh_frame(self: Self, addr: u64) -> Self`

- `fn set_text(self: Self, addr: u64) -> Self`

- `fn set_got(self: Self, addr: u64) -> Self`

#### Trait Implementations

##### `impl Clone for BaseAddresses`

- `fn clone(self: &Self) -> BaseAddresses` — [`BaseAddresses`](#baseaddresses)

##### `impl Debug for BaseAddresses`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BaseAddresses`

- `fn default() -> BaseAddresses` — [`BaseAddresses`](#baseaddresses)

##### `impl Eq for BaseAddresses`

##### `impl PartialEq for BaseAddresses`

- `fn eq(self: &Self, other: &BaseAddresses) -> bool` — [`BaseAddresses`](#baseaddresses)

##### `impl StructuralPartialEq for BaseAddresses`

### `SectionBaseAddresses`

```rust
struct SectionBaseAddresses {
    pub section: Option<u64>,
    pub text: Option<u64>,
    pub data: Option<u64>,
}
```

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

##### `impl Clone for SectionBaseAddresses`

- `fn clone(self: &Self) -> SectionBaseAddresses` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl Debug for SectionBaseAddresses`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SectionBaseAddresses`

- `fn default() -> SectionBaseAddresses` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl Eq for SectionBaseAddresses`

##### `impl PartialEq for SectionBaseAddresses`

- `fn eq(self: &Self, other: &SectionBaseAddresses) -> bool` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl StructuralPartialEq for SectionBaseAddresses`

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

- `fn next(self: &mut Self) -> Result<Option<CieOrFde<'bases, Section, R>>>` — [`Result`](../index.md), [`CieOrFde`](#cieorfde)

#### Trait Implementations

##### `impl<'bases, Section, R> Clone for CfiEntriesIter<'bases, Section, R>`

- `fn clone(self: &Self) -> CfiEntriesIter<'bases, Section, R>` — [`CfiEntriesIter`](#cfientriesiter)

##### `impl<'bases, Section, R> Debug for CfiEntriesIter<'bases, Section, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Augmentation`

```rust
struct Augmentation {
    lsda: Option<constants::DwEhPe>,
    personality: Option<(constants::DwEhPe, Pointer)>,
    fde_address_encoding: Option<constants::DwEhPe>,
    is_signal_trampoline: bool,
}
```

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

- `fn parse<Section, R>(augmentation_str: &mut R, bases: &BaseAddresses, address_size: u8, section: &Section, input: &mut R) -> Result<Augmentation>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md), [`Augmentation`](#augmentation)

#### Trait Implementations

##### `impl Clone for Augmentation`

- `fn clone(self: &Self) -> Augmentation` — [`Augmentation`](#augmentation)

##### `impl Copy for Augmentation`

##### `impl Debug for Augmentation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Augmentation`

- `fn default() -> Augmentation` — [`Augmentation`](#augmentation)

##### `impl Eq for Augmentation`

##### `impl PartialEq for Augmentation`

- `fn eq(self: &Self, other: &Augmentation) -> bool` — [`Augmentation`](#augmentation)

##### `impl StructuralPartialEq for Augmentation`

### `AugmentationData`

```rust
struct AugmentationData {
    lsda: Option<Pointer>,
}
```

Parsed augmentation data for a `FrameDescriptEntry`.

#### Implementations

- `fn parse<R: Reader>(augmentation: &Augmentation, encoding_parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> Result<AugmentationData>` — [`Augmentation`](#augmentation), [`PointerEncodingParameters`](cfi/index.md), [`Result`](../index.md), [`AugmentationData`](cfi/index.md)

#### Trait Implementations

##### `impl Clone for AugmentationData`

- `fn clone(self: &Self) -> AugmentationData` — [`AugmentationData`](cfi/index.md)

##### `impl Debug for AugmentationData`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AugmentationData`

- `fn default() -> AugmentationData` — [`AugmentationData`](cfi/index.md)

##### `impl Eq for AugmentationData`

##### `impl PartialEq for AugmentationData`

- `fn eq(self: &Self, other: &AugmentationData) -> bool` — [`AugmentationData`](cfi/index.md)

##### `impl StructuralPartialEq for AugmentationData`

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

- `fn offset(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../index.md)

- `fn address_size(self: &Self) -> u8`

- `fn instructions<'a, Section>(self: &Self, section: &'a Section, bases: &'a BaseAddresses) -> CallFrameInstructionIter<'a, R>` — [`BaseAddresses`](#baseaddresses), [`CallFrameInstructionIter`](#callframeinstructioniter)

- `fn entry_len(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn version(self: &Self) -> u8`

- `fn augmentation(self: &Self) -> Option<&Augmentation>` — [`Augmentation`](#augmentation)

- `fn has_lsda(self: &Self) -> bool`

- `fn lsda_encoding(self: &Self) -> Option<constants::DwEhPe>` — [`DwEhPe`](../index.md)

- `fn personality_with_encoding(self: &Self) -> Option<(constants::DwEhPe, Pointer)>` — [`DwEhPe`](../index.md), [`Pointer`](#pointer)

- `fn personality(self: &Self) -> Option<Pointer>` — [`Pointer`](#pointer)

- `fn fde_address_encoding(self: &Self) -> Option<constants::DwEhPe>` — [`DwEhPe`](../index.md)

- `fn is_signal_trampoline(self: &Self) -> bool`

- `fn code_alignment_factor(self: &Self) -> u64`

- `fn data_alignment_factor(self: &Self) -> i64`

- `fn return_address_register(self: &Self) -> Register` — [`Register`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for CommonInformationEntry<R, Offset>`

- `fn clone(self: &Self) -> CommonInformationEntry<R, Offset>` — [`CommonInformationEntry`](#commoninformationentry)

##### `impl<R, Offset> Debug for CommonInformationEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for CommonInformationEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for CommonInformationEntry<R, Offset>`

- `fn eq(self: &Self, other: &CommonInformationEntry<R, Offset>) -> bool` — [`CommonInformationEntry`](#commoninformationentry)

##### `impl<R, Offset> StructuralPartialEq for CommonInformationEntry<R, Offset>`

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

A partially parsed `FrameDescriptionEntry`.

Fully parsing this FDE requires first parsing its CIE.

#### Implementations

- `fn parse_partial(section: &Section, bases: &'bases BaseAddresses, input: &mut R) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md), [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

- `fn parse<F>(self: &Self, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Result`](../index.md), [`FrameDescriptionEntry`](#framedescriptionentry)

- `fn offset(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn cie_offset(self: &Self) -> <Section as >::Offset` — [`UnwindSection`](#unwindsection)

- `fn entry_len(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

#### Trait Implementations

##### `impl<'bases, Section, R> Clone for PartialFrameDescriptionEntry<'bases, Section, R>`

- `fn clone(self: &Self) -> PartialFrameDescriptionEntry<'bases, Section, R>` — [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

##### `impl<'bases, Section, R> Debug for PartialFrameDescriptionEntry<'bases, Section, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'bases, Section, R> Eq for PartialFrameDescriptionEntry<'bases, Section, R>`

##### `impl<'bases, Section, R> PartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

- `fn eq(self: &Self, other: &PartialFrameDescriptionEntry<'bases, Section, R>) -> bool` — [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

##### `impl<'bases, Section, R> StructuralPartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

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

- `fn offset(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn cie(self: &Self) -> &CommonInformationEntry<R>` — [`CommonInformationEntry`](#commoninformationentry)

- `fn entry_len(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn instructions<'a, Section>(self: &Self, section: &'a Section, bases: &'a BaseAddresses) -> CallFrameInstructionIter<'a, R>` — [`BaseAddresses`](#baseaddresses), [`CallFrameInstructionIter`](#callframeinstructioniter)

- `fn initial_address(self: &Self) -> u64`

- `fn end_address(self: &Self) -> u64`

- `fn len(self: &Self) -> u64`

- `fn contains(self: &Self, address: u64) -> bool`

- `fn lsda(self: &Self) -> Option<Pointer>` — [`Pointer`](#pointer)

- `fn is_signal_trampoline(self: &Self) -> bool`

- `fn personality(self: &Self) -> Option<Pointer>` — [`Pointer`](#pointer)

#### Trait Implementations

##### `impl<R, Offset> Clone for FrameDescriptionEntry<R, Offset>`

- `fn clone(self: &Self) -> FrameDescriptionEntry<R, Offset>` — [`FrameDescriptionEntry`](#framedescriptionentry)

##### `impl<R, Offset> Debug for FrameDescriptionEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for FrameDescriptionEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FrameDescriptionEntry<R, Offset>`

- `fn eq(self: &Self, other: &FrameDescriptionEntry<R, Offset>) -> bool` — [`FrameDescriptionEntry`](#framedescriptionentry)

##### `impl<R, Offset> StructuralPartialEq for FrameDescriptionEntry<R, Offset>`

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

- `fn new_in() -> Self`

- `fn initialize<Section, R>(self: &mut Self, section: &Section, bases: &BaseAddresses, cie: &CommonInformationEntry<R>) -> Result<()>` — [`BaseAddresses`](#baseaddresses), [`CommonInformationEntry`](#commoninformationentry), [`Result`](../index.md)

- `fn reset(self: &mut Self)`

- `fn row(self: &Self) -> &UnwindTableRow<T, S>` — [`UnwindTableRow`](#unwindtablerow)

- `fn row_mut(self: &mut Self) -> &mut UnwindTableRow<T, S>` — [`UnwindTableRow`](#unwindtablerow)

- `fn save_initial_rules(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn start_address(self: &Self) -> u64`

- `fn set_start_address(self: &mut Self, start_address: u64)`

- `fn set_register_rule(self: &mut Self, register: Register, rule: RegisterRule<T>) -> Result<()>` — [`Register`](../index.md), [`RegisterRule`](#registerrule), [`Result`](../index.md)

- `fn get_initial_rule(self: &Self, register: Register) -> Option<RegisterRule<T>>` — [`Register`](../index.md), [`RegisterRule`](#registerrule)

- `fn set_cfa(self: &mut Self, cfa: CfaRule<T>)` — [`CfaRule`](#cfarule)

- `fn cfa_mut(self: &mut Self) -> &mut CfaRule<T>` — [`CfaRule`](#cfarule)

- `fn push_row(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

- `fn pop_row(self: &mut Self) -> Result<()>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<T, S> Clone for UnwindContext<T, S>`

- `fn clone(self: &Self) -> UnwindContext<T, S>` — [`UnwindContext`](#unwindcontext)

##### `impl<T, S> Debug for UnwindContext<T, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindContext<T, S>`

- `fn default() -> Self`

##### `impl<T, S> Eq for UnwindContext<T, S>`

##### `impl<T, S> PartialEq for UnwindContext<T, S>`

- `fn eq(self: &Self, other: &UnwindContext<T, S>) -> bool` — [`UnwindContext`](#unwindcontext)

##### `impl<T, S> StructuralPartialEq for UnwindContext<T, S>`

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

- `fn new<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Result<Self>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`FrameDescriptionEntry`](#framedescriptionentry), [`Result`](../index.md)

- `fn new_for_fde<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Self` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`FrameDescriptionEntry`](#framedescriptionentry)

- `fn new_for_cie<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, cie: &CommonInformationEntry<R>) -> Self` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`CommonInformationEntry`](#commoninformationentry)

- `fn next_row(self: &mut Self) -> Result<Option<&UnwindTableRow<<R as >::Offset, S>>>` — [`Result`](../index.md), [`UnwindTableRow`](#unwindtablerow), [`Reader`](#reader)

- `fn into_current_row(self: Self) -> Option<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`UnwindTableRow`](#unwindtablerow), [`Reader`](#reader)

- `fn evaluate(self: &mut Self, instruction: CallFrameInstruction<<R as >::Offset>) -> Result<bool>` — [`CallFrameInstruction`](#callframeinstruction), [`Reader`](#reader), [`Result`](../index.md)

#### Trait Implementations

##### `impl<'a, 'ctx, R, S> Debug for UnwindTable<'a, 'ctx, R, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RegisterRuleMap<T, S>`

```rust
struct RegisterRuleMap<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    rules: super::util::ArrayVec<<S as >::Rules>,
}
```

#### Implementations

- `fn is_default(self: &Self) -> bool`

- `fn get(self: &Self, register: Register) -> RegisterRule<T>` — [`Register`](../index.md), [`RegisterRule`](#registerrule)

- `fn set(self: &mut Self, register: Register, rule: RegisterRule<T>) -> Result<()>` — [`Register`](../index.md), [`RegisterRule`](#registerrule), [`Result`](../index.md)

- `fn iter(self: &Self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](#registerruleiter)

#### Trait Implementations

##### `impl<T, S> Clone for RegisterRuleMap<T, S>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S> Debug for RegisterRuleMap<T, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for RegisterRuleMap<T, S>`

- `fn default() -> Self`

##### `impl<T, S> Eq for RegisterRuleMap<T, S>`

##### `impl<'a, R, S> FromIterator for RegisterRuleMap<R, S>`

- `fn from_iter<T>(iter: T) -> Self`

##### `impl<T, S> PartialEq for RegisterRuleMap<T, S>`

- `fn eq(self: &Self, rhs: &Self) -> bool`

### `RegisterRuleIter<'iter, T>`

```rust
struct RegisterRuleIter<'iter, T>(::core::slice::Iter<'iter, (crate::common::Register, RegisterRule<T>)>)
where
    T: ReaderOffset;
```

An unordered iterator for register rules.

#### Trait Implementations

##### `impl<'iter, T> Clone for RegisterRuleIter<'iter, T>`

- `fn clone(self: &Self) -> RegisterRuleIter<'iter, T>` — [`RegisterRuleIter`](#registerruleiter)

##### `impl<'iter, T> Debug for RegisterRuleIter<'iter, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for RegisterRuleIter<'iter, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'iter, T: ReaderOffset> Iterator for RegisterRuleIter<'iter, T>`

- `type Item = &'iter (Register, RegisterRule<T>)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

A row in the virtual unwind table that describes how to find the values of
the registers in the *previous* frame for a range of PC addresses.

#### Implementations

- `fn is_default(self: &Self) -> bool`

- `fn start_address(self: &Self) -> u64`

- `fn end_address(self: &Self) -> u64`

- `fn contains(self: &Self, address: u64) -> bool`

- `fn saved_args_size(self: &Self) -> u64`

- `fn cfa(self: &Self) -> &CfaRule<T>` — [`CfaRule`](#cfarule)

- `fn register(self: &Self, register: Register) -> RegisterRule<T>` — [`Register`](../index.md), [`RegisterRule`](#registerrule)

- `fn registers(self: &Self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](#registerruleiter)

#### Trait Implementations

##### `impl<T, S> Clone for UnwindTableRow<T, S>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S> Debug for UnwindTableRow<T, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindTableRow<T, S>`

- `fn default() -> Self`

##### `impl<T, S> Eq for UnwindTableRow<T, S>`

##### `impl<T, S> PartialEq for UnwindTableRow<T, S>`

- `fn eq(self: &Self, other: &UnwindTableRow<T, S>) -> bool` — [`UnwindTableRow`](#unwindtablerow)

##### `impl<T, S> StructuralPartialEq for UnwindTableRow<T, S>`

### `CallFrameInstructionIter<'a, R: Reader>`

```rust
struct CallFrameInstructionIter<'a, R: Reader> {
    input: R,
    address_encoding: Option<constants::DwEhPe>,
    parameters: PointerEncodingParameters<'a, R>,
    vendor: crate::common::Vendor,
}
```

A lazy iterator parsing call frame instructions.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<CallFrameInstruction<<R as >::Offset>>>` — [`Result`](../index.md), [`CallFrameInstruction`](#callframeinstruction), [`Reader`](#reader)

#### Trait Implementations

##### `impl<'a, R: $crate::clone::Clone + Reader> Clone for CallFrameInstructionIter<'a, R>`

- `fn clone(self: &Self) -> CallFrameInstructionIter<'a, R>` — [`CallFrameInstructionIter`](#callframeinstructioniter)

##### `impl<'a, R: $crate::fmt::Debug + Reader> Debug for CallFrameInstructionIter<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UnwindExpression<T: ReaderOffset>`

```rust
struct UnwindExpression<T: ReaderOffset> {
    pub offset: T,
    pub length: T,
}
```

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

- `fn get<R, S>(self: &Self, section: &S) -> Result<Expression<R>>` — [`Result`](../index.md), [`Expression`](#expression)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + ReaderOffset> Clone for UnwindExpression<T>`

- `fn clone(self: &Self) -> UnwindExpression<T>` — [`UnwindExpression`](#unwindexpression)

##### `impl<T: $crate::marker::Copy + ReaderOffset> Copy for UnwindExpression<T>`

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for UnwindExpression<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for UnwindExpression<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for UnwindExpression<T>`

- `fn eq(self: &Self, other: &UnwindExpression<T>) -> bool` — [`UnwindExpression`](#unwindexpression)

##### `impl<T: ReaderOffset> StructuralPartialEq for UnwindExpression<T>`

### `PointerEncodingParameters<'a, R: Reader>`

```rust
struct PointerEncodingParameters<'a, R: Reader> {
    bases: &'a SectionBaseAddresses,
    func_base: Option<u64>,
    address_size: u8,
    section: &'a R,
}
```

#### Trait Implementations

##### `impl<'a, R: $crate::clone::Clone + Reader> Clone for PointerEncodingParameters<'a, R>`

- `fn clone(self: &Self) -> PointerEncodingParameters<'a, R>` — [`PointerEncodingParameters`](cfi/index.md)

##### `impl<'a, R: $crate::fmt::Debug + Reader> Debug for PointerEncodingParameters<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn load<F, E>(section: F) -> core::result::Result<Self, E>`

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

- `fn borrow_with_sup<'a, F, R>(self: &'a Self, sup: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for DwarfSections<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::default::Default> Default for DwarfSections<T>`

- `fn default() -> DwarfSections<T>` — [`DwarfSections`](#dwarfsections)

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

- `fn make_dwo(self: &mut Self, parent: &Dwarf<R>)` — [`Dwarf`](#dwarf)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug> Debug for Dwarf<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for Dwarf<R>`

- `fn default() -> Dwarf<R>` — [`Dwarf`](#dwarf)

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

- `fn load<F, E>(section: F) -> core::result::Result<Self, E>`

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F, empty: R) -> Result<DwarfPackage<R>>` — [`Result`](../index.md), [`DwarfPackage`](#dwarfpackage)

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for DwarfPackageSections<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::default::Default> Default for DwarfPackageSections<T>`

- `fn default() -> DwarfPackageSections<T>` — [`DwarfPackageSections`](#dwarfpackagesections)

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

- `fn load<F, E>(section: F, empty: R) -> core::result::Result<Self, E>`

- `fn from_sections(sections: DwarfPackageSections<R>, empty: R) -> Result<Self>` — [`DwarfPackageSections`](#dwarfpackagesections), [`Result`](../index.md)

- `fn find_cu(self: &Self, id: DwoId, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DwoId`](../index.md), [`Dwarf`](#dwarf), [`Result`](../index.md)

- `fn find_tu(self: &Self, signature: DebugTypeSignature, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DebugTypeSignature`](../index.md), [`Dwarf`](#dwarf), [`Result`](../index.md)

- `fn cu_sections(self: &Self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](#dwarf), [`Result`](../index.md)

- `fn tu_sections(self: &Self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](#dwarf), [`Result`](../index.md)

- `fn sections(self: &Self, sections: UnitIndexSectionIterator<'_, R>, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`UnitIndexSectionIterator`](#unitindexsectioniterator), [`Dwarf`](#dwarf), [`Result`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DwarfPackage<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn new(dwarf: &Dwarf<R>, header: UnitHeader<R>) -> Result<Self>` — [`Dwarf`](#dwarf), [`UnitHeader`](#unitheader), [`Result`](../index.md)

- `fn new_with_abbreviations(dwarf: &Dwarf<R>, header: UnitHeader<R>, abbreviations: Arc<Abbreviations>) -> Result<Self>` — [`Dwarf`](#dwarf), [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations), [`Result`](../index.md)

- `fn unit_ref<'a>(self: &'a Self, dwarf: &'a Dwarf<R>) -> UnitRef<'a, R>` — [`Dwarf`](#dwarf), [`UnitRef`](#unitref)

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../index.md)

- `fn entry(self: &Self, offset: UnitOffset<<R as >::Offset>) -> Result<DebuggingInformationEntry<'_, '_, R>>` — [`UnitOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`DebuggingInformationEntry`](#debugginginformationentry)

- `fn entries(self: &Self) -> EntriesCursor<'_, '_, R>` — [`EntriesCursor`](#entriescursor)

- `fn entries_at_offset(self: &Self, offset: UnitOffset<<R as >::Offset>) -> Result<EntriesCursor<'_, '_, R>>` — [`UnitOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`EntriesCursor`](#entriescursor)

- `fn entries_tree(self: &Self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesTree<'_, '_, R>>` — [`UnitOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`EntriesTree`](#entriestree)

- `fn entries_raw(self: &Self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesRaw<'_, '_, R>>` — [`UnitOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`EntriesRaw`](#entriesraw)

- `fn copy_relocated_attributes(self: &mut Self, other: &Unit<R>)` — [`Unit`](#unit)

- `fn dwo_name(self: &Self) -> Result<Option<AttributeValue<R>>>` — [`Result`](../index.md), [`AttributeValue`](#attributevalue)

#### Trait Implementations

##### `impl<R, Offset> Debug for Unit<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UnitRef<'a, R: Reader>`

```rust
struct UnitRef<'a, R: Reader> {
    pub dwarf: &'a Dwarf<R>,
    pub unit: &'a Unit<R>,
}
```

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

- `fn new(dwarf: &'a Dwarf<R>, unit: &'a Unit<R>) -> Self` — [`Dwarf`](#dwarf), [`Unit`](#unit)

- `fn string_offset(self: &Self, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`DebugStrOffsetsIndex`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`DebugStrOffset`](../index.md)

- `fn string(self: &Self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md)

- `fn line_string(self: &Self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugLineStrOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md)

- `fn sup_string(self: &Self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md)

- `fn attr_string(self: &Self, attr: AttributeValue<R>) -> Result<R>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md)

- `fn address(self: &Self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md), [`Reader`](#reader), [`Result`](../index.md)

- `fn attr_address(self: &Self, attr: AttributeValue<R>) -> Result<Option<u64>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md)

- `fn ranges_offset_from_raw(self: &Self, offset: RawRangeListsOffset<<R as >::Offset>) -> RangeListsOffset<<R as >::Offset>` — [`RawRangeListsOffset`](../index.md), [`Reader`](#reader), [`RangeListsOffset`](../index.md)

- `fn ranges_offset(self: &Self, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`DebugRngListsIndex`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`RangeListsOffset`](../index.md)

- `fn ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`RngListIter`](#rnglistiter)

- `fn raw_ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`RawRngListIter`](#rawrnglistiter)

- `fn attr_ranges_offset(self: &Self, attr: AttributeValue<R>) -> Result<Option<RangeListsOffset<<R as >::Offset>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md), [`RangeListsOffset`](../index.md), [`Reader`](#reader)

- `fn attr_ranges(self: &Self, attr: AttributeValue<R>) -> Result<Option<RngListIter<R>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md), [`RngListIter`](#rnglistiter)

- `fn die_ranges(self: &Self, entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<RangeIter<R>>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md), [`RangeIter`](#rangeiter)

- `fn unit_ranges(self: &Self) -> Result<RangeIter<R>>` — [`Result`](../index.md), [`RangeIter`](#rangeiter)

- `fn locations_offset(self: &Self, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` — [`DebugLocListsIndex`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`LocationListsOffset`](../index.md)

- `fn locations(self: &Self, offset: LocationListsOffset<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`LocListIter`](#loclistiter)

- `fn raw_locations(self: &Self, offset: LocationListsOffset<<R as >::Offset>) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`RawLocListIter`](#rawloclistiter)

- `fn attr_locations_offset(self: &Self, attr: AttributeValue<R>) -> Result<Option<LocationListsOffset<<R as >::Offset>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md), [`LocationListsOffset`](../index.md), [`Reader`](#reader)

- `fn attr_locations(self: &Self, attr: AttributeValue<R>) -> Result<Option<LocListIter<R>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md), [`LocListIter`](#loclistiter)

- `fn macinfo(self: &Self, offset: DebugMacinfoOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacinfoOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`MacroIter`](#macroiter)

- `fn macros(self: &Self, offset: DebugMacroOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacroOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`MacroIter`](#macroiter)

#### Trait Implementations

##### `impl<'a, R: Reader> Clone for UnitRef<'a, R>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, R: Reader> Copy for UnitRef<'a, R>`

##### `impl<'a, R: $crate::fmt::Debug + Reader> Debug for UnitRef<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, R: Reader> Deref for UnitRef<'a, R>`

- `type Target = Unit<R>`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for UnitRef<'a, R>`

- `type Target = T`

### `RangeIter<R: Reader>`

```rust
struct RangeIter<R: Reader>(RangeIterInner<R>);
```

An iterator for the address ranges of a `DebuggingInformationEntry`.

Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<Range>>` — [`Result`](../index.md), [`Range`](#range)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RangeIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> Default for RangeIter<R>`

- `fn default() -> Self`

### `EndianSlice<'input, Endian>`

```rust
struct EndianSlice<'input, Endian>
where
    Endian: Endianity {
    slice: &'input [u8],
    endian: Endian,
}
```

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- `fn range(self: &Self, idx: Range<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

- `fn range_from(self: &Self, idx: RangeFrom<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

- `fn range_to(self: &Self, idx: RangeTo<usize>) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

#### Trait Implementations

##### `impl<'input, Endian> Clone for EndianSlice<'input, Endian>`

- `fn clone(self: &Self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

##### `impl<'input, Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<'input, Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<'input, Endian> Default for EndianSlice<'input, Endian>`

- `fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

##### `impl<'input, Endian> Deref for EndianSlice<'input, Endian>`

- `type Target = [u8]`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<'input, Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<'input, Endian> Hash for EndianSlice<'input, Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<'input, Endian> PartialEq for EndianSlice<'input, Endian>`

- `fn eq(self: &Self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](#endianslice)

##### `impl<'input, Endian> Reader for EndianSlice<'input, Endian>`

- `type Endian = Endian`

- `type Offset = usize`

- `fn endian(self: &Self) -> Endian`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn empty(self: &mut Self)`

- `fn truncate(self: &mut Self, len: usize) -> Result<()>` — [`Result`](../index.md)

- `fn offset_from(self: &Self, base: &Self) -> usize`

- `fn offset_id(self: &Self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- `fn find(self: &Self, byte: u8) -> Result<usize>` — [`Result`](../index.md)

- `fn skip(self: &mut Self, len: usize) -> Result<()>` — [`Result`](../index.md)

- `fn split(self: &mut Self, len: usize) -> Result<Self>` — [`Result`](../index.md)

- `fn to_slice(self: &Self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md)

- `fn to_string(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../index.md)

- `fn to_string_lossy(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../index.md)

- `fn read_slice(self: &mut Self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md)

##### `impl<P, T> Receiver for EndianSlice<'input, Endian>`

- `type Target = T`

##### `impl<'input, Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

#### Trait Implementations

##### `impl<'input> Debug for DebugBytes<'input>`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReaderOffsetId`

```rust
struct ReaderOffsetId(u64);
```

An identifier for an offset within a section reader.

This is used for error reporting. The meaning of this value is specific to
each reader implementation. The values should be chosen to be unique amongst
all readers. If values are not unique then errors may point to the wrong reader.

#### Trait Implementations

##### `impl Clone for ReaderOffsetId`

- `fn clone(self: &Self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

##### `impl Copy for ReaderOffsetId`

##### `impl Debug for ReaderOffsetId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ReaderOffsetId`

##### `impl PartialEq for ReaderOffsetId`

- `fn eq(self: &Self, other: &ReaderOffsetId) -> bool` — [`ReaderOffsetId`](#readeroffsetid)

##### `impl StructuralPartialEq for ReaderOffsetId`

### `RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>>`

```rust
struct RelocateReader<R: Reader<Offset = usize>, T: Relocate<<R as >::Offset>> {
    section: R,
    reader: R,
    relocate: T,
}
```

A `Reader` which applies relocations to addresses and offsets.

This is useful for reading sections which contain relocations,
such as those in a relocatable object file.
It is generally not used for reading sections in an executable file.

#### Implementations

- `fn new(section: R, relocate: T) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader<Offset = usize>, T: $crate::clone::Clone + Relocate<<R as >::Offset>> Clone for RelocateReader<R, T>`

- `fn clone(self: &Self) -> RelocateReader<R, T>` — [`RelocateReader`](#relocatereader)

##### `impl<R: $crate::fmt::Debug + Reader<Offset = usize>, T: $crate::fmt::Debug + Relocate<<R as >::Offset>> Debug for RelocateReader<R, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, T> Reader for RelocateReader<R, T>`

- `type Endian = <R as Reader>::Endian`

- `type Offset = <R as Reader>::Offset`

- `fn read_address(self: &mut Self, address_size: u8) -> Result<u64>` — [`Result`](../index.md)

- `fn read_offset(self: &mut Self, format: Format) -> Result<<R as >::Offset>` — [`Format`](../index.md), [`Result`](../index.md), [`Reader`](#reader)

- `fn read_sized_offset(self: &mut Self, size: u8) -> Result<<R as >::Offset>` — [`Result`](../index.md), [`Reader`](#reader)

- `fn split(self: &mut Self, len: <Self as >::Offset) -> Result<Self>` — [`Reader`](#reader), [`Result`](../index.md)

- `fn endian(self: &Self) -> <Self as >::Endian` — [`Reader`](#reader)

- `fn len(self: &Self) -> <Self as >::Offset` — [`Reader`](#reader)

- `fn empty(self: &mut Self)`

- `fn truncate(self: &mut Self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](#reader), [`Result`](../index.md)

- `fn offset_from(self: &Self, base: &Self) -> <Self as >::Offset` — [`Reader`](#reader)

- `fn offset_id(self: &Self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- `fn find(self: &Self, byte: u8) -> Result<<Self as >::Offset>` — [`Result`](../index.md), [`Reader`](#reader)

- `fn skip(self: &mut Self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](#reader), [`Result`](../index.md)

- `fn to_slice(self: &Self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md)

- `fn to_string(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../index.md)

- `fn to_string_lossy(self: &Self) -> Result<Cow<'_, str>>` — [`Result`](../index.md)

- `fn read_slice(self: &mut Self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md)

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

- `fn abbreviations(self: &Self, debug_abbrev_offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Abbreviations>` — [`DebugAbbrevOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`Abbreviations`](#abbreviations)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugAbbrev<R>`

- `fn clone(self: &Self) -> DebugAbbrev<R>` — [`DebugAbbrev`](#debugabbrev)

##### `impl<R: $crate::marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugAbbrev<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugAbbrev<R>`

- `fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](#debugabbrev)

##### `impl<R> Section for DebugAbbrev<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

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

- `fn populate<R: Reader>(self: &mut Self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy), [`DebugAbbrev`](#debugabbrev), [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

- `fn set<R: Reader>(self: &mut Self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../index.md), [`Reader`](#reader), [`Abbreviations`](#abbreviations)

- `fn get<R: Reader>(self: &Self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](#debugabbrev), [`DebugAbbrevOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`Abbreviations`](#abbreviations)

#### Trait Implementations

##### `impl Debug for AbbreviationsCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AbbreviationsCache`

- `fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](#abbreviationscache)

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

- `fn empty() -> Abbreviations` — [`Abbreviations`](#abbreviations)

- `fn insert(self: &mut Self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](#abbreviation)

- `fn get(self: &Self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](#abbreviation)

- `fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../index.md), [`Abbreviations`](#abbreviations)

#### Trait Implementations

##### `impl Clone for Abbreviations`

- `fn clone(self: &Self) -> Abbreviations` — [`Abbreviations`](#abbreviations)

##### `impl Debug for Abbreviations`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Abbreviations`

- `fn default() -> Abbreviations` — [`Abbreviations`](#abbreviations)

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

- `fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../index.md), [`DwChildren`](../index.md), [`Attributes`](abbrev/index.md), [`Abbreviation`](#abbreviation)

- `fn code(self: &Self) -> u64`

- `fn tag(self: &Self) -> constants::DwTag` — [`DwTag`](../index.md)

- `fn has_children(self: &Self) -> bool`

- `fn attributes(self: &Self) -> &[AttributeSpecification]` — [`AttributeSpecification`](#attributespecification)

- `fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../index.md), [`DwTag`](../index.md)

- `fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../index.md), [`DwChildren`](../index.md)

- `fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../index.md), [`Attributes`](abbrev/index.md)

- `fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../index.md), [`Abbreviation`](#abbreviation)

#### Trait Implementations

##### `impl Clone for Abbreviation`

- `fn clone(self: &Self) -> Abbreviation` — [`Abbreviation`](#abbreviation)

##### `impl Debug for Abbreviation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Abbreviation`

##### `impl PartialEq for Abbreviation`

- `fn eq(self: &Self, other: &Abbreviation) -> bool` — [`Abbreviation`](#abbreviation)

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

- `fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../index.md), [`DwForm`](../index.md), [`AttributeSpecification`](#attributespecification)

- `fn name(self: &Self) -> constants::DwAt` — [`DwAt`](../index.md)

- `fn form(self: &Self) -> constants::DwForm` — [`DwForm`](../index.md)

- `fn implicit_const_value(self: &Self) -> Option<i64>`

- `fn size<R: Reader>(self: &Self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](#unitheader)

- `fn parse_form<R: Reader>(input: &mut R) -> Result<constants::DwForm>` — [`Result`](../index.md), [`DwForm`](../index.md)

- `fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../index.md), [`AttributeSpecification`](#attributespecification)

#### Trait Implementations

##### `impl Clone for AttributeSpecification`

- `fn clone(self: &Self) -> AttributeSpecification` — [`AttributeSpecification`](#attributespecification)

##### `impl Copy for AttributeSpecification`

##### `impl Debug for AttributeSpecification`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AttributeSpecification`

##### `impl PartialEq for AttributeSpecification`

- `fn eq(self: &Self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](#attributespecification)

##### `impl StructuralPartialEq for AttributeSpecification`

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugAranges<R>`

- `fn clone(self: &Self) -> DebugAranges<R>` — [`DebugAranges`](#debugaranges)

##### `impl<R: $crate::marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugAranges<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugAranges<R>`

- `fn default() -> DebugAranges<R>` — [`DebugAranges`](#debugaranges)

##### `impl<R> Section for DebugAranges<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../index.md), [`ArangeHeader`](#arangeheader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- `fn clone(self: &Self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](#arangeheaderiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- `fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../index.md), [`Result`](../index.md)

- `fn offset(self: &Self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../index.md)

- `fn length(self: &Self) -> Offset`

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../index.md)

- `fn debug_info_offset(self: &Self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../index.md)

- `fn entries(self: &Self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](#arangeentryiter)

#### Trait Implementations

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- `fn clone(self: &Self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](#arangeheader)

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- `fn eq(self: &Self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](#arangeheader)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the aranges from a `.debug_aranges` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<ArangeEntry>>` — [`Result`](../index.md), [`ArangeEntry`](#arangeentry)

- `fn next_raw(self: &mut Self) -> Result<Option<ArangeEntry>>` — [`Result`](../index.md), [`ArangeEntry`](#arangeentry)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- `fn clone(self: &Self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](#arangeentryiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

A single parsed arange.

#### Implementations

- `fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../index.md), [`Result`](../index.md)

- `fn address(self: &Self) -> u64`

- `fn length(self: &Self) -> u64`

- `fn range(self: &Self) -> Range` — [`Range`](#range)

#### Trait Implementations

##### `impl Clone for ArangeEntry`

- `fn clone(self: &Self) -> ArangeEntry` — [`ArangeEntry`](#arangeentry)

##### `impl Debug for ArangeEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl Ord for ArangeEntry`

- `fn cmp(self: &Self, other: &ArangeEntry) -> $crate::cmp::Ordering` — [`ArangeEntry`](#arangeentry)

##### `impl PartialEq for ArangeEntry`

- `fn eq(self: &Self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](#arangeentry)

##### `impl PartialOrd for ArangeEntry`

- `fn partial_cmp(self: &Self, other: &ArangeEntry) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ArangeEntry`](#arangeentry)

##### `impl StructuralPartialEq for ArangeEntry`

### `DebugCuIndex<R>`

```rust
struct DebugCuIndex<R> {
    section: R,
}
```

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

#### Implementations

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugCuIndex<R>`

- `fn clone(self: &Self) -> DebugCuIndex<R>` — [`DebugCuIndex`](#debugcuindex)

##### `impl<R: $crate::marker::Copy> Copy for DebugCuIndex<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugCuIndex<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugCuIndex<R>`

- `fn default() -> DebugCuIndex<R>` — [`DebugCuIndex`](#debugcuindex)

##### `impl<R> Section for DebugCuIndex<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugTuIndex<R>`

```rust
struct DebugTuIndex<R> {
    section: R,
}
```

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugTuIndex<R>`

- `fn clone(self: &Self) -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

##### `impl<R: $crate::marker::Copy> Copy for DebugTuIndex<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugTuIndex<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugTuIndex<R>`

- `fn default() -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

##### `impl<R> Section for DebugTuIndex<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

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

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

#### Implementations

- `fn parse(input: R) -> Result<UnitIndex<R>>` — [`Result`](../index.md), [`UnitIndex`](#unitindex)

- `fn find(self: &Self, id: u64) -> Option<u32>`

- `fn sections(self: &Self, row: u32) -> Result<UnitIndexSectionIterator<'_, R>>` — [`Result`](../index.md), [`UnitIndexSectionIterator`](#unitindexsectioniterator)

- `fn version(self: &Self) -> u16`

- `fn section_count(self: &Self) -> u32`

- `fn unit_count(self: &Self) -> u32`

- `fn slot_count(self: &Self) -> u32`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for UnitIndex<R>`

- `fn clone(self: &Self) -> UnitIndex<R>` — [`UnitIndex`](#unitindex)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for UnitIndex<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UnitIndexSectionIterator<'index, R: Reader>`

```rust
struct UnitIndexSectionIterator<'index, R: Reader> {
    sections: slice::Iter<'index, IndexSectionId>,
    offsets: R,
    sizes: R,
}
```

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

#### Trait Implementations

##### `impl<'index, R: $crate::clone::Clone + Reader> Clone for UnitIndexSectionIterator<'index, R>`

- `fn clone(self: &Self) -> UnitIndexSectionIterator<'index, R>` — [`UnitIndexSectionIterator`](#unitindexsectioniterator)

##### `impl<'index, R: $crate::fmt::Debug + Reader> Debug for UnitIndexSectionIterator<'index, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for UnitIndexSectionIterator<'index, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'index, R: Reader> Iterator for UnitIndexSectionIterator<'index, R>`

- `type Item = UnitIndexSection`

- `fn next(self: &mut Self) -> Option<UnitIndexSection>` — [`UnitIndexSection`](#unitindexsection)

### `UnitIndexSection`

```rust
struct UnitIndexSection {
    pub section: IndexSectionId,
    pub offset: u32,
    pub size: u32,
}
```

Information about a unit's contribution to a section in a `.dwp` file.

#### Fields

- **`section`**: `IndexSectionId`

  The section kind.

- **`offset`**: `u32`

  The base offset of the unit's contribution to the section.

- **`size`**: `u32`

  The size of the unit's contribution to the section.

#### Trait Implementations

##### `impl Clone for UnitIndexSection`

- `fn clone(self: &Self) -> UnitIndexSection` — [`UnitIndexSection`](#unitindexsection)

##### `impl Copy for UnitIndexSection`

##### `impl Debug for UnitIndexSection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for UnitIndexSection`

##### `impl PartialEq for UnitIndexSection`

- `fn eq(self: &Self, other: &UnitIndexSection) -> bool` — [`UnitIndexSection`](#unitindexsection)

##### `impl StructuralPartialEq for UnitIndexSection`

### `DebugLine<R>`

```rust
struct DebugLine<R> {
    debug_line_section: R,
}
```

The `DebugLine` struct contains the source location to instruction mapping
found in the `.debug_line` section.

#### Implementations

- `fn new(debug_line_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLine<R>`

- `fn clone(self: &Self) -> DebugLine<R>` — [`DebugLine`](#debugline)

##### `impl<R: $crate::marker::Copy> Copy for DebugLine<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLine<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLine<R>`

- `fn default() -> DebugLine<R>` — [`DebugLine`](#debugline)

##### `impl<R> Section for DebugLine<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

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

Executes a `LineProgram` to iterate over the rows in the matrix of line number information.

"The hypothetical machine used by a consumer of the line number information
to expand the byte-coded instruction stream into a matrix of line number
information." -- Section 6.2.1

#### Implementations

- `fn new(program: IncompleteLineProgram<R, Offset>) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`IncompleteLineProgram`](#incompletelineprogram), [`LineRows`](#linerows)

- `fn resume<'program>(program: &'program CompleteLineProgram<R, Offset>, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`CompleteLineProgram`](#completelineprogram), [`LineSequence`](#linesequence), [`LineRows`](#linerows)

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- `fn next_row(self: &mut Self) -> Result<Option<(&LineProgramHeader<R, Offset>, &LineRow)>>` — [`Result`](../index.md), [`LineProgramHeader`](#lineprogramheader), [`LineRow`](#linerow)

#### Trait Implementations

##### `impl<R, Program, Offset> Clone for LineRows<R, Program, Offset>`

- `fn clone(self: &Self) -> LineRows<R, Program, Offset>` — [`LineRows`](#linerows)

##### `impl<R, Program, Offset> Debug for LineRows<R, Program, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LineInstructions<R: Reader>`

```rust
struct LineInstructions<R: Reader> {
    input: R,
}
```

An iterator yielding parsed instructions.

See
[`LineProgramHeader::instructions`](./struct.LineProgramHeader.html#method.instructions)
for more details.

#### Implementations

- `fn remove_trailing(self: &Self, other: &LineInstructions<R>) -> Result<LineInstructions<R>>` — [`LineInstructions`](#lineinstructions), [`Result`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for LineInstructions<R>`

- `fn clone(self: &Self) -> LineInstructions<R>` — [`LineInstructions`](#lineinstructions)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for LineInstructions<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

A row in the line number program's resulting matrix.

Each row is a copy of the registers of the state machine, as defined in section 6.2.2.

#### Implementations

- `fn new<R: Reader>(header: &LineProgramHeader<R>) -> Self` — [`LineProgramHeader`](#lineprogramheader)

- `fn address(self: &Self) -> u64`

- `fn op_index(self: &Self) -> u64`

- `fn file_index(self: &Self) -> u64`

- `fn file<'header, R: Reader>(self: &Self, header: &'header LineProgramHeader<R>) -> Option<&'header FileEntry<R>>` — [`LineProgramHeader`](#lineprogramheader), [`FileEntry`](#fileentry)

- `fn line(self: &Self) -> Option<NonZeroU64>`

- `fn column(self: &Self) -> ColumnType` — [`ColumnType`](#columntype)

- `fn is_stmt(self: &Self) -> bool`

- `fn basic_block(self: &Self) -> bool`

- `fn end_sequence(self: &Self) -> bool`

- `fn prologue_end(self: &Self) -> bool`

- `fn epilogue_begin(self: &Self) -> bool`

- `fn isa(self: &Self) -> u64`

- `fn discriminator(self: &Self) -> u64`

- `fn execute<R, Program>(self: &mut Self, instruction: LineInstruction<R>, program: &mut Program) -> Result<bool>` — [`LineInstruction`](#lineinstruction), [`Result`](../index.md)

- `fn reset<R: Reader>(self: &mut Self, header: &LineProgramHeader<R>)` — [`LineProgramHeader`](#lineprogramheader)

- `fn apply_line_advance(self: &mut Self, line_increment: i64)`

- `fn apply_operation_advance<R: Reader>(self: &mut Self, operation_advance: u64, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md)

- `fn adjust_opcode<R: Reader>(self: &Self, opcode: u8, header: &LineProgramHeader<R>) -> u8` — [`LineProgramHeader`](#lineprogramheader)

- `fn exec_special_opcode<R: Reader>(self: &mut Self, opcode: u8, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for LineRow`

- `fn clone(self: &Self) -> LineRow` — [`LineRow`](#linerow)

##### `impl Copy for LineRow`

##### `impl Debug for LineRow`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LineRow`

##### `impl PartialEq for LineRow`

- `fn eq(self: &Self, other: &LineRow) -> bool` — [`LineRow`](#linerow)

##### `impl StructuralPartialEq for LineRow`

### `LineSequence<R: Reader>`

```rust
struct LineSequence<R: Reader> {
    pub start: u64,
    pub end: u64,
    instructions: LineInstructions<R>,
}
```

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

##### `impl<R: $crate::clone::Clone + Reader> Clone for LineSequence<R>`

- `fn clone(self: &Self) -> LineSequence<R>` — [`LineSequence`](#linesequence)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for LineSequence<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn offset(self: &Self) -> DebugLineOffset<<R as >::Offset>` — [`DebugLineOffset`](../index.md), [`Reader`](#reader)

- `fn unit_length(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../index.md)

- `fn version(self: &Self) -> u16`

- `fn header_length(self: &Self) -> <R as >::Offset` — [`Reader`](#reader)

- `fn address_size(self: &Self) -> u8`

- `fn format(self: &Self) -> Format` — [`Format`](../index.md)

- `fn line_encoding(self: &Self) -> LineEncoding` — [`LineEncoding`](../index.md)

- `fn minimum_instruction_length(self: &Self) -> u8`

- `fn maximum_operations_per_instruction(self: &Self) -> u8`

- `fn default_is_stmt(self: &Self) -> bool`

- `fn line_base(self: &Self) -> i8`

- `fn line_range(self: &Self) -> u8`

- `fn opcode_base(self: &Self) -> u8`

- `fn standard_opcode_lengths(self: &Self) -> &R`

- `fn directory_entry_format(self: &Self) -> &[FileEntryFormat]` — [`FileEntryFormat`](#fileentryformat)

- `fn include_directories(self: &Self) -> &[AttributeValue<R, Offset>]` — [`AttributeValue`](#attributevalue)

- `fn directory(self: &Self, directory: u64) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](#attributevalue)

- `fn file_name_entry_format(self: &Self) -> &[FileEntryFormat]` — [`FileEntryFormat`](#fileentryformat)

- `fn file_has_timestamp(self: &Self) -> bool`

- `fn file_has_size(self: &Self) -> bool`

- `fn file_has_md5(self: &Self) -> bool`

- `fn file_has_source(self: &Self) -> bool`

- `fn file_names(self: &Self) -> &[FileEntry<R, Offset>]` — [`FileEntry`](#fileentry)

- `fn file(self: &Self, file: u64) -> Option<&FileEntry<R, Offset>>` — [`FileEntry`](#fileentry)

- `fn raw_program_buf(self: &Self) -> R`

- `fn instructions(self: &Self) -> LineInstructions<R>` — [`LineInstructions`](#lineinstructions)

- `fn parse(input: &mut R, offset: DebugLineOffset<Offset>, address_size: u8, comp_dir: Option<R>, comp_name: Option<R>) -> Result<LineProgramHeader<R, Offset>>` — [`DebugLineOffset`](../index.md), [`Result`](../index.md), [`LineProgramHeader`](#lineprogramheader)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineProgramHeader<R, Offset>`

- `fn clone(self: &Self) -> LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

##### `impl<R, Offset> Debug for LineProgramHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for LineProgramHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for LineProgramHeader<R, Offset>`

- `fn eq(self: &Self, other: &LineProgramHeader<R, Offset>) -> bool` — [`LineProgramHeader`](#lineprogramheader)

##### `impl<R, Offset> StructuralPartialEq for LineProgramHeader<R, Offset>`

### `IncompleteLineProgram<R, Offset>`

```rust
struct IncompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

A line number program that has not been run to completion.

#### Implementations

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- `fn rows(self: Self) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`LineRows`](#linerows), [`IncompleteLineProgram`](#incompletelineprogram)

- `fn sequences(self: Self) -> Result<(CompleteLineProgram<R, Offset>, Vec<LineSequence<R>>)>` — [`Result`](../index.md), [`CompleteLineProgram`](#completelineprogram), [`LineSequence`](#linesequence)

#### Trait Implementations

##### `impl<R, Offset> Clone for IncompleteLineProgram<R, Offset>`

- `fn clone(self: &Self) -> IncompleteLineProgram<R, Offset>` — [`IncompleteLineProgram`](#incompletelineprogram)

##### `impl<R, Offset> Debug for IncompleteLineProgram<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for IncompleteLineProgram<R, Offset>`

##### `impl<R, Offset> LineProgram for IncompleteLineProgram<R, Offset>`

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- `fn add_file(self: &mut Self, file: FileEntry<R, Offset>)` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> PartialEq for IncompleteLineProgram<R, Offset>`

- `fn eq(self: &Self, other: &IncompleteLineProgram<R, Offset>) -> bool` — [`IncompleteLineProgram`](#incompletelineprogram)

##### `impl<R, Offset> StructuralPartialEq for IncompleteLineProgram<R, Offset>`

### `CompleteLineProgram<R, Offset>`

```rust
struct CompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

A line number program that has previously been run to completion.

#### Implementations

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- `fn resume_from<'program>(self: &'program Self, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`LineSequence`](#linesequence), [`LineRows`](#linerows), [`CompleteLineProgram`](#completelineprogram)

#### Trait Implementations

##### `impl<R, Offset> Clone for CompleteLineProgram<R, Offset>`

- `fn clone(self: &Self) -> CompleteLineProgram<R, Offset>` — [`CompleteLineProgram`](#completelineprogram)

##### `impl<R, Offset> Debug for CompleteLineProgram<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for CompleteLineProgram<R, Offset>`

##### `impl<R, Offset> PartialEq for CompleteLineProgram<R, Offset>`

- `fn eq(self: &Self, other: &CompleteLineProgram<R, Offset>) -> bool` — [`CompleteLineProgram`](#completelineprogram)

##### `impl<R, Offset> StructuralPartialEq for CompleteLineProgram<R, Offset>`

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

An entry in the `LineProgramHeader`'s `file_names` set.

#### Implementations

- `fn parse(input: &mut R, path_name: R) -> Result<FileEntry<R, Offset>>` — [`Result`](../index.md), [`FileEntry`](#fileentry)

- `fn path_name(self: &Self) -> AttributeValue<R, Offset>` — [`AttributeValue`](#attributevalue)

- `fn directory_index(self: &Self) -> u64`

- `fn directory(self: &Self, header: &LineProgramHeader<R>) -> Option<AttributeValue<R, Offset>>` — [`LineProgramHeader`](#lineprogramheader), [`AttributeValue`](#attributevalue)

- `fn timestamp(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn md5(self: &Self) -> &[u8; 16]`

- `fn source(self: &Self) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](#attributevalue)

#### Trait Implementations

##### `impl<R, Offset> Clone for FileEntry<R, Offset>`

- `fn clone(self: &Self) -> FileEntry<R, Offset>` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> Copy for FileEntry<R, Offset>`

##### `impl<R, Offset> Debug for FileEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for FileEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FileEntry<R, Offset>`

- `fn eq(self: &Self, other: &FileEntry<R, Offset>) -> bool` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> StructuralPartialEq for FileEntry<R, Offset>`

### `FileEntryFormat`

```rust
struct FileEntryFormat {
    pub content_type: constants::DwLnct,
    pub form: constants::DwForm,
}
```

The format of a component of an include directory or file name entry.

#### Fields

- **`content_type`**: `constants::DwLnct`

  The type of information that is represented by the component.

- **`form`**: `constants::DwForm`

  The encoding form of the component value.

#### Implementations

- `fn parse<R: Reader>(input: &mut R) -> Result<Vec<FileEntryFormat>>` — [`Result`](../index.md), [`FileEntryFormat`](#fileentryformat)

#### Trait Implementations

##### `impl Clone for FileEntryFormat`

- `fn clone(self: &Self) -> FileEntryFormat` — [`FileEntryFormat`](#fileentryformat)

##### `impl Copy for FileEntryFormat`

##### `impl Debug for FileEntryFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FileEntryFormat`

##### `impl PartialEq for FileEntryFormat`

- `fn eq(self: &Self, other: &FileEntryFormat) -> bool` — [`FileEntryFormat`](#fileentryformat)

##### `impl StructuralPartialEq for FileEntryFormat`

### `DebugLoc<R>`

```rust
struct DebugLoc<R> {
    section: R,
}
```

The raw contents of the `.debug_loc` section.

#### Implementations

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLoc<R>`

- `fn clone(self: &Self) -> DebugLoc<R>` — [`DebugLoc`](#debugloc)

##### `impl<R: $crate::marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLoc<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLoc<R>`

- `fn default() -> DebugLoc<R>` — [`DebugLoc`](#debugloc)

##### `impl<R> Section for DebugLoc<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugLocLists<R>`

```rust
struct DebugLocLists<R> {
    section: R,
}
```

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLocLists<R>`

- `fn clone(self: &Self) -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

##### `impl<R: $crate::marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLocLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLocLists<R>`

- `fn default() -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

##### `impl<R> Section for DebugLocLists<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `LocationLists<R>`

```rust
struct LocationLists<R> {
    debug_loc: DebugLoc<R>,
    debug_loclists: DebugLocLists<R>,
}
```

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

#### Implementations

- `fn new(debug_loc: DebugLoc<R>, debug_loclists: DebugLocLists<R>) -> LocationLists<R>` — [`DebugLoc`](#debugloc), [`DebugLocLists`](#debugloclists), [`LocationLists`](#locationlists)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for LocationLists<R>`

- `fn clone(self: &Self) -> LocationLists<R>` — [`LocationLists`](#locationlists)

##### `impl<R: $crate::marker::Copy> Copy for LocationLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for LocationLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for LocationLists<R>`

- `fn default() -> LocationLists<R>` — [`LocationLists`](#locationlists)

### `RawLocListIter<R: Reader>`

```rust
struct RawLocListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: LocListsFormat,
}
```

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

#### Implementations

- `fn new(input: R, encoding: Encoding, format: LocListsFormat) -> RawLocListIter<R>` — [`Encoding`](../index.md), [`LocListsFormat`](loclists/index.md), [`RawLocListIter`](#rawloclistiter)

- `fn next(self: &mut Self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../index.md), [`RawLocListEntry`](#rawloclistentry)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RawLocListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LocListIter<R: Reader>`

```rust
struct LocListIter<R: Reader> {
    raw: RawLocListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

#### Implementations

- `fn new(raw: RawLocListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> LocListIter<R>` — [`RawLocListIter`](#rawloclistiter), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md), [`Reader`](#reader), [`LocListIter`](#loclistiter)

- `fn get_address(self: &Self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md), [`Reader`](#reader), [`Result`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../index.md), [`LocationListEntry`](#locationlistentry)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for LocListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LocationListEntry<R: Reader>`

```rust
struct LocationListEntry<R: Reader> {
    pub range: crate::read::Range,
    pub data: crate::read::Expression<R>,
}
```

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

#### Fields

- **`range`**: `crate::read::Range`

  The address range that this location is valid for.

- **`data`**: `crate::read::Expression<R>`

  The data containing a single location description.

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for LocationListEntry<R>`

- `fn clone(self: &Self) -> LocationListEntry<R>` — [`LocationListEntry`](#locationlistentry)

##### `impl<R: $crate::marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for LocationListEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<R: $crate::hash::Hash + Reader> Hash for LocationListEntry<R>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- `fn eq(self: &Self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](#locationlistentry)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

### `DebugMacinfo<R>`

```rust
struct DebugMacinfo<R> {
    section: R,
}
```

The raw contents of the `.debug_macinfo` section.

#### Implementations

- `fn new(macinfo_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugMacinfo<R>`

- `fn clone(self: &Self) -> DebugMacinfo<R>` — [`DebugMacinfo`](#debugmacinfo)

##### `impl<R: $crate::marker::Copy> Copy for DebugMacinfo<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugMacinfo<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugMacinfo<R>`

- `fn default() -> DebugMacinfo<R>` — [`DebugMacinfo`](#debugmacinfo)

##### `impl<R> Section for DebugMacinfo<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugMacro<R>`

```rust
struct DebugMacro<R> {
    section: R,
}
```

The raw contents of the `.debug_macro` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugMacro<R>`

- `fn clone(self: &Self) -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

##### `impl<R: $crate::marker::Copy> Copy for DebugMacro<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugMacro<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugMacro<R>`

- `fn default() -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

##### `impl<R> Section for DebugMacro<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `MacroUnitHeader<R: Reader>`

```rust
struct MacroUnitHeader<R: Reader> {
    _version: u16,
    flags: u8,
    _debug_line_offset: crate::DebugLineOffset<<R as >::Offset>,
}
```

#### Fields

- **`_version`**: `u16`

  The version of the macro unit header. At the moment only version 5 is defined.

#### Implementations

- `const OFFSET_SIZE_FLAG: u8`

- `const DEBUG_LINE_OFFSET_FLAG: u8`

- `const OPCODE_OPERANDS_TABLE_FLAG: u8`

- `fn parse(input: &mut R) -> Result<Self>` — [`Result`](../index.md)

- `fn format(self: &Self) -> Format` — [`Format`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for MacroUnitHeader<R>`

- `fn clone(self: &Self) -> MacroUnitHeader<R>` — [`MacroUnitHeader`](macros/index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for MacroUnitHeader<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MacroIter<R: Reader>`

```rust
struct MacroIter<R: Reader> {
    input: R,
    format: crate::Format,
    is_macro: bool,
}
```

Iterator over the entries in the `.debug_macro` section.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<MacroEntry<R>>>` — [`Result`](../index.md), [`MacroEntry`](#macroentry)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for MacroIter<R>`

- `fn clone(self: &Self) -> MacroIter<R>` — [`MacroIter`](#macroiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for MacroIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl<R, Offset> Clone for Piece<R, Offset>`

- `fn clone(self: &Self) -> Piece<R, Offset>` — [`Piece`](#piece)

##### `impl<R, Offset> Copy for Piece<R, Offset>`

##### `impl<R, Offset> Debug for Piece<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> PartialEq for Piece<R, Offset>`

- `fn eq(self: &Self, other: &Piece<R, Offset>) -> bool` — [`Piece`](#piece)

##### `impl<R, Offset> StructuralPartialEq for Piece<R, Offset>`

### `Expression<R: Reader>`

```rust
struct Expression<R: Reader>(R);
```

The bytecode for a DWARF expression or location description.

#### Implementations

- `fn evaluation(self: Self, encoding: Encoding) -> Evaluation<R>` — [`Encoding`](../index.md), [`Evaluation`](#evaluation)

- `fn operations(self: Self, encoding: Encoding) -> OperationIter<R>` — [`Encoding`](../index.md), [`OperationIter`](#operationiter)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for Expression<R>`

- `fn clone(self: &Self) -> Expression<R>` — [`Expression`](#expression)

##### `impl<R: $crate::marker::Copy + Reader> Copy for Expression<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for Expression<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for Expression<R>`

##### `impl<R: $crate::hash::Hash + Reader> Hash for Expression<R>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for Expression<R>`

- `fn eq(self: &Self, other: &Expression<R>) -> bool` — [`Expression`](#expression)

##### `impl<R: Reader> StructuralPartialEq for Expression<R>`

### `OperationIter<R: Reader>`

```rust
struct OperationIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator for the operations in an expression.

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<Operation<R>>>` — [`Result`](../index.md), [`Operation`](#operation)

- `fn offset_from(self: &Self, expression: &Expression<R>) -> <R as >::Offset` — [`Expression`](#expression), [`Reader`](#reader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for OperationIter<R>`

- `fn clone(self: &Self) -> OperationIter<R>` — [`OperationIter`](#operationiter)

##### `impl<R: $crate::marker::Copy + Reader> Copy for OperationIter<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for OperationIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn new_in(bytecode: R, encoding: Encoding) -> Self` — [`Encoding`](../index.md)

- `fn set_initial_value(self: &mut Self, value: u64)`

- `fn set_object_address(self: &mut Self, value: u64)`

- `fn set_max_iterations(self: &mut Self, value: u32)`

- `fn pop(self: &mut Self) -> Result<Value>` — [`Result`](../index.md), [`Value`](#value)

- `fn push(self: &mut Self, value: Value) -> Result<()>` — [`Value`](#value), [`Result`](../index.md)

- `fn evaluate_one_operation(self: &mut Self) -> Result<OperationEvaluationResult<R>>` — [`Result`](../index.md), [`OperationEvaluationResult`](op/index.md)

- `fn value_result(self: &Self) -> Option<Value>` — [`Value`](#value)

- `fn as_result(self: &Self) -> &[Piece<R>]` — [`Piece`](#piece)

- `fn evaluate(self: &mut Self) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_memory(self: &mut Self, value: Value) -> Result<EvaluationResult<R>>` — [`Value`](#value), [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_register(self: &mut Self, value: Value) -> Result<EvaluationResult<R>>` — [`Value`](#value), [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_frame_base(self: &mut Self, frame_base: u64) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_tls(self: &mut Self, value: u64) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_call_frame_cfa(self: &mut Self, cfa: u64) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_at_location(self: &mut Self, bytes: R) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_entry_value(self: &mut Self, entry_value: Value) -> Result<EvaluationResult<R>>` — [`Value`](#value), [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_parameter_ref(self: &mut Self, parameter_value: u64) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_relocated_address(self: &mut Self, address: u64) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_indexed_address(self: &mut Self, address: u64) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn resume_with_base_type(self: &mut Self, base_type: ValueType) -> Result<EvaluationResult<R>>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

- `fn end_of_expression(self: &mut Self) -> bool`

- `fn evaluate_internal(self: &mut Self) -> Result<EvaluationResult<R>>` — [`Result`](../index.md), [`EvaluationResult`](#evaluationresult)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader, S: $crate::fmt::Debug + EvaluationStorage<R>> Debug for Evaluation<R, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PubNamesEntry<R: Reader>`

```rust
struct PubNamesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubname.

#### Implementations

- `fn name(self: &Self) -> &R`

- `fn unit_header_offset(self: &Self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../index.md), [`Reader`](#reader)

- `fn die_offset(self: &Self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md), [`Reader`](#reader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubNamesEntry<R>`

- `fn clone(self: &Self) -> PubNamesEntry<R>` — [`PubNamesEntry`](#pubnamesentry)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../index.md), [`Reader`](#reader), [`DebugInfoOffset`](../index.md)

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- `fn new(debug_pubnames_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugPubNames<R>`

- `fn clone(self: &Self) -> DebugPubNames<R>` — [`DebugPubNames`](#debugpubnames)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugPubNames<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> Section for DebugPubNames<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

An iterator over the pubnames from a `.debug_pubnames` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../index.md), [`PubNamesEntry`](#pubnamesentry)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- `fn clone(self: &Self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](#pubnamesentryiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubtype.

#### Implementations

- `fn name(self: &Self) -> &R`

- `fn unit_header_offset(self: &Self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../index.md), [`Reader`](#reader)

- `fn die_offset(self: &Self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md), [`Reader`](#reader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubTypesEntry<R>`

- `fn clone(self: &Self) -> PubTypesEntry<R>` — [`PubTypesEntry`](#pubtypesentry)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../index.md), [`Reader`](#reader), [`DebugInfoOffset`](../index.md)

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- `fn items(self: &Self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](#pubtypesentryiter)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugPubTypes<R>`

- `fn clone(self: &Self) -> DebugPubTypes<R>` — [`DebugPubTypes`](#debugpubtypes)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

An iterator over the pubtypes from a `.debug_pubtypes` section.

Can be [used with
`FallibleIterator`](./index.html#using-with-fallibleiterator).

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../index.md), [`PubTypesEntry`](#pubtypesentry)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- `fn clone(self: &Self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](#pubtypesentryiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DebugRanges<R>`

```rust
struct DebugRanges<R> {
    section: R,
}
```

The raw contents of the `.debug_ranges` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugRanges<R>`

- `fn clone(self: &Self) -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

##### `impl<R: $crate::marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugRanges<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugRanges<R>`

- `fn default() -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

##### `impl<R> Section for DebugRanges<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugRngLists<R>`

- `fn clone(self: &Self) -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

##### `impl<R: $crate::marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugRngLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugRngLists<R>`

- `fn default() -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

##### `impl<R> Section for DebugRngLists<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- `fn ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>, unit_encoding: Encoding, base_address: u64, debug_addr: &DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../index.md), [`Reader`](#reader), [`Encoding`](../index.md), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md), [`Result`](../index.md), [`RngListIter`](#rnglistiter)

- `fn raw_ranges(self: &Self, offset: RangeListsOffset<<R as >::Offset>, unit_encoding: Encoding) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../index.md), [`Reader`](#reader), [`Encoding`](../index.md), [`Result`](../index.md), [`RawRngListIter`](#rawrnglistiter)

- `fn get_offset(self: &Self, unit_encoding: Encoding, base: DebugRngListsBase<<R as >::Offset>, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`Encoding`](../index.md), [`DebugRngListsBase`](../index.md), [`Reader`](#reader), [`DebugRngListsIndex`](../index.md), [`Result`](../index.md), [`RangeListsOffset`](../index.md)

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>` — [`ReaderOffsetId`](#readeroffsetid), [`SectionId`](../index.md), [`Reader`](#reader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for RangeLists<R>`

- `fn clone(self: &Self) -> RangeLists<R>` — [`RangeLists`](#rangelists)

##### `impl<R: $crate::marker::Copy> Copy for RangeLists<R>`

##### `impl<R: $crate::fmt::Debug> Debug for RangeLists<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for RangeLists<R>`

- `fn default() -> RangeLists<R>` — [`RangeLists`](#rangelists)

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- `fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../index.md), [`RangeListsFormat`](rnglists/index.md), [`RawRngListIter`](#rawrnglistiter)

- `fn next(self: &mut Self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../index.md), [`RawRngListEntry`](#rawrnglistentry), [`Reader`](#reader)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RawRngListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- `fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](#rawrnglistiter), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md), [`Reader`](#reader), [`RngListIter`](#rnglistiter)

- `fn get_address(self: &Self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md), [`Reader`](#reader), [`Result`](../index.md)

- `fn next(self: &mut Self) -> Result<Option<Range>>` — [`Result`](../index.md), [`Range`](#range)

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RngListIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- `fn is_end(self: &Self) -> bool`

- `fn is_base_address(self: &Self, address_size: u8) -> bool`

- `fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../index.md), [`RawRange`](rnglists/index.md)

#### Trait Implementations

##### `impl Clone for RawRange`

- `fn clone(self: &Self) -> RawRange` — [`RawRange`](rnglists/index.md)

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RawRange`

##### `impl Hash for RawRange`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RawRange`

- `fn eq(self: &Self, other: &RawRange) -> bool` — [`RawRange`](rnglists/index.md)

##### `impl StructuralPartialEq for RawRange`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- `fn add_base_address(self: &mut Self, base_address: u64, address_size: u8)`

#### Trait Implementations

##### `impl Clone for Range`

- `fn clone(self: &Self) -> Range` — [`Range`](#range)

##### `impl Copy for Range`

##### `impl Debug for Range`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Range`

##### `impl Hash for Range`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Range`

- `fn cmp(self: &Self, other: &Range) -> $crate::cmp::Ordering` — [`Range`](#range)

##### `impl PartialEq for Range`

- `fn eq(self: &Self, other: &Range) -> bool` — [`Range`](#range)

##### `impl PartialOrd for Range`

- `fn partial_cmp(self: &Self, other: &Range) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Range`](#range)

##### `impl StructuralPartialEq for Range`

### `DebugStr<R>`

```rust
struct DebugStr<R> {
    debug_str_section: R,
}
```

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugStr<R>` — [`DebugStr`](#debugstr)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugStr<R>`

- `fn clone(self: &Self) -> DebugStr<R>` — [`DebugStr`](#debugstr)

##### `impl<R: $crate::marker::Copy> Copy for DebugStr<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugStr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugStr<R>`

- `fn default() -> DebugStr<R>` — [`DebugStr`](#debugstr)

##### `impl<R> Section for DebugStr<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- `fn get_str_offset(self: &Self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../index.md), [`DebugStrOffsetsBase`](../index.md), [`Reader`](#reader), [`DebugStrOffsetsIndex`](../index.md), [`Result`](../index.md), [`DebugStrOffset`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugStrOffsets<R>`

- `fn clone(self: &Self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](#debugstroffsets)

##### `impl<R: $crate::marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugStrOffsets<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugStrOffsets<R>`

- `fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](#debugstroffsets)

##### `impl<R> Section for DebugStrOffsets<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLineStr<R>`

- `fn clone(self: &Self) -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

##### `impl<R: $crate::marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLineStr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLineStr<R>`

- `fn default() -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

##### `impl<R> Section for DebugLineStr<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugInfo<R>`

```rust
struct DebugInfo<R> {
    debug_info_section: R,
}
```

The `DebugInfo` struct represents the DWARF debugging information found in
the `.debug_info` section.

#### Implementations

- `fn units(self: &Self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

- `fn header_from_offset(self: &Self, offset: DebugInfoOffset<<R as >::Offset>) -> Result<UnitHeader<R>>` — [`DebugInfoOffset`](../index.md), [`Reader`](#reader), [`Result`](../index.md), [`UnitHeader`](#unitheader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugInfo<R>`

- `fn clone(self: &Self) -> DebugInfo<R>` — [`DebugInfo`](#debuginfo)

##### `impl<R: $crate::marker::Copy> Copy for DebugInfo<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugInfo<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugInfo<R>`

- `fn default() -> DebugInfo<R>` — [`DebugInfo`](#debuginfo)

##### `impl<R> Section for DebugInfo<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

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

- `fn next(self: &mut Self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../index.md), [`UnitHeader`](#unitheader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugInfoUnitHeadersIter<R>`

- `fn clone(self: &Self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

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

- `fn new(encoding: Encoding, unit_length: Offset, unit_type: UnitType<Offset>, debug_abbrev_offset: DebugAbbrevOffset<Offset>, unit_offset: UnitSectionOffset<Offset>, entries_buf: R) -> Self` — [`Encoding`](../index.md), [`UnitType`](#unittype), [`DebugAbbrevOffset`](../index.md), [`UnitSectionOffset`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for UnitHeader<R, Offset>`

- `fn clone(self: &Self) -> UnitHeader<R, Offset>` — [`UnitHeader`](#unitheader)

##### `impl<R, Offset> Copy for UnitHeader<R, Offset>`

##### `impl<R, Offset> Debug for UnitHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for UnitHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for UnitHeader<R, Offset>`

- `fn eq(self: &Self, other: &UnitHeader<R, Offset>) -> bool` — [`UnitHeader`](#unitheader)

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

- `fn new(offset: UnitOffset<Offset>, attrs_slice: R, abbrev: &'abbrev Abbreviation, unit: &'unit UnitHeader<R, Offset>) -> Self` — [`UnitOffset`](../index.md), [`Abbreviation`](#abbreviation), [`UnitHeader`](#unitheader)

- `fn code(self: &Self) -> u64`

- `fn offset(self: &Self) -> UnitOffset<Offset>` — [`UnitOffset`](../index.md)

- `fn tag(self: &Self) -> constants::DwTag` — [`DwTag`](../index.md)

- `fn has_children(self: &Self) -> bool`

- `fn attrs<'me>(self: &'me Self) -> AttrsIter<'abbrev, 'me, 'unit, R>` — [`AttrsIter`](#attrsiter)

- `fn attr(self: &Self, name: constants::DwAt) -> Result<Option<Attribute<R>>>` — [`DwAt`](../index.md), [`Result`](../index.md), [`Attribute`](#attribute)

- `fn attr_value_raw(self: &Self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../index.md), [`Result`](../index.md), [`AttributeValue`](#attributevalue)

- `fn attr_value(self: &Self, name: constants::DwAt) -> Result<Option<AttributeValue<R>>>` — [`DwAt`](../index.md), [`Result`](../index.md), [`AttributeValue`](#attributevalue)

- `fn after_attrs(self: &Self) -> Result<R>` — [`Result`](../index.md)

- `fn sibling(self: &Self) -> Option<R>`

- `fn parse(input: &mut R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Result<Option<Self>>` — [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations), [`Result`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R, Offset> Clone for DebuggingInformationEntry<'abbrev, 'unit, R, Offset>`

- `fn clone(self: &Self) -> DebuggingInformationEntry<'abbrev, 'unit, R, Offset>` — [`DebuggingInformationEntry`](#debugginginformationentry)

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

- `fn name(self: &Self) -> constants::DwAt` — [`DwAt`](../index.md)

- `fn raw_value(self: &Self) -> AttributeValue<R>` — [`AttributeValue`](#attributevalue)

- `fn value(self: &Self) -> AttributeValue<R>` — [`AttributeValue`](#attributevalue)

- `fn u8_value(self: &Self) -> Option<u8>`

- `fn u16_value(self: &Self) -> Option<u16>`

- `fn udata_value(self: &Self) -> Option<u64>`

- `fn sdata_value(self: &Self) -> Option<i64>`

- `fn offset_value(self: &Self) -> Option<<R as >::Offset>` — [`Reader`](#reader)

- `fn exprloc_value(self: &Self) -> Option<Expression<R>>` — [`Expression`](#expression)

- `fn string_value(self: &Self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](#debugstr)

- `fn string_value_sup(self: &Self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](#debugstr)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for Attribute<R>`

- `fn clone(self: &Self) -> Attribute<R>` — [`Attribute`](#attribute)

##### `impl<R: $crate::marker::Copy + Reader> Copy for Attribute<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for Attribute<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for Attribute<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for Attribute<R>`

- `fn eq(self: &Self, other: &Attribute<R>) -> bool` — [`Attribute`](#attribute)

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

- `fn next(self: &mut Self) -> Result<Option<Attribute<R>>>` — [`Result`](../index.md), [`Attribute`](#attribute)

#### Trait Implementations

##### `impl<'abbrev, 'entry, 'unit, R: $crate::clone::Clone + Reader> Clone for AttrsIter<'abbrev, 'entry, 'unit, R>`

- `fn clone(self: &Self) -> AttrsIter<'abbrev, 'entry, 'unit, R>` — [`AttrsIter`](#attrsiter)

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

- `fn next_offset(self: &Self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md), [`Reader`](#reader)

- `fn next_depth(self: &Self) -> isize`

- `fn read_abbreviation(self: &mut Self) -> Result<Option<&'abbrev Abbreviation>>` — [`Result`](../index.md), [`Abbreviation`](#abbreviation)

- `fn read_attribute(self: &mut Self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md), [`Attribute`](#attribute)

- `fn skip_attributes(self: &mut Self, specs: &[AttributeSpecification]) -> Result<()>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R> Clone for EntriesRaw<'abbrev, 'unit, R>`

- `fn clone(self: &Self) -> EntriesRaw<'abbrev, 'unit, R>` — [`EntriesRaw`](#entriesraw)

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

- `fn current(self: &Self) -> Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>` — [`DebuggingInformationEntry`](#debugginginformationentry)

- `fn next_entry(self: &mut Self) -> Result<Option<()>>` — [`Result`](../index.md)

- `fn next_dfs(self: &mut Self) -> Result<Option<(isize, &DebuggingInformationEntry<'abbrev, 'unit, R>)>>` — [`Result`](../index.md), [`DebuggingInformationEntry`](#debugginginformationentry)

- `fn next_sibling(self: &mut Self) -> Result<Option<&DebuggingInformationEntry<'abbrev, 'unit, R>>>` — [`Result`](../index.md), [`DebuggingInformationEntry`](#debugginginformationentry)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R> Clone for EntriesCursor<'abbrev, 'unit, R>`

- `fn clone(self: &Self) -> EntriesCursor<'abbrev, 'unit, R>` — [`EntriesCursor`](#entriescursor)

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

- `fn new(root: R, unit: &'unit UnitHeader<R>, abbreviations: &'abbrev Abbreviations) -> Self` — [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations)

- `fn root<'me>(self: &'me mut Self) -> Result<EntriesTreeNode<'abbrev, 'unit, 'me, R>>` — [`Result`](../index.md), [`EntriesTreeNode`](#entriestreenode)

- `fn next(self: &mut Self, depth: isize) -> Result<bool>` — [`Result`](../index.md)

#### Trait Implementations

##### `impl<'abbrev, 'unit, R> Clone for EntriesTree<'abbrev, 'unit, R>`

- `fn clone(self: &Self) -> EntriesTree<'abbrev, 'unit, R>` — [`EntriesTree`](#entriestree)

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

- `fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeNode<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](#entriestree), [`EntriesTreeNode`](#entriestreenode)

- `fn entry(self: &Self) -> &DebuggingInformationEntry<'abbrev, 'unit, R>` — [`DebuggingInformationEntry`](#debugginginformationentry)

- `fn children(self: Self) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTreeIter`](#entriestreeiter)

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

- `fn new(tree: &'tree mut EntriesTree<'abbrev, 'unit, R>, depth: isize) -> EntriesTreeIter<'abbrev, 'unit, 'tree, R>` — [`EntriesTree`](#entriestree), [`EntriesTreeIter`](#entriestreeiter)

- `fn next<'me>(self: &'me mut Self) -> Result<Option<EntriesTreeNode<'abbrev, 'unit, 'me, R>>>` — [`Result`](../index.md), [`EntriesTreeNode`](#entriestreenode)

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

- `fn new(debug_types_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugTypes<R>`

- `fn clone(self: &Self) -> DebugTypes<R>` — [`DebugTypes`](#debugtypes)

##### `impl<R: $crate::marker::Copy> Copy for DebugTypes<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugTypes<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugTypes<R>`

- `fn default() -> DebugTypes<R>` — [`DebugTypes`](#debugtypes)

##### `impl<R> Section for DebugTypes<R>`

- `fn id() -> SectionId` — [`SectionId`](../index.md)

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

- `fn next(self: &mut Self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../index.md), [`UnitHeader`](#unitheader)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugTypesUnitHeadersIter<R>`

- `fn clone(self: &Self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugTypesUnitHeadersIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl Clone for Error`

- `fn clone(self: &Self) -> Error` — [`Error`](../index.md)

##### `impl Copy for Error`

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> ::core::result::Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- `fn eq(self: &Self, other: &Error) -> bool` — [`Error`](../index.md)

##### `impl StructuralPartialEq for Error`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

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

Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE).

#### Variants

- **`Cie`**

  This CFI entry is a `CommonInformationEntry`.

- **`Fde`**

  This CFI entry is a `FrameDescriptionEntry`, however fully parsing it
  requires parsing its CIE first, so it is left in a partially parsed
  state.

#### Trait Implementations

##### `impl<'bases, Section, R> Clone for CieOrFde<'bases, Section, R>`

- `fn clone(self: &Self) -> CieOrFde<'bases, Section, R>` — [`CieOrFde`](#cieorfde)

##### `impl<'bases, Section, R> Debug for CieOrFde<'bases, Section, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'bases, Section, R> Eq for CieOrFde<'bases, Section, R>`

##### `impl<'bases, Section, R> PartialEq for CieOrFde<'bases, Section, R>`

- `fn eq(self: &Self, other: &CieOrFde<'bases, Section, R>) -> bool` — [`CieOrFde`](#cieorfde)

##### `impl<'bases, Section, R> StructuralPartialEq for CieOrFde<'bases, Section, R>`

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

The canonical frame address (CFA) recovery rules.

#### Variants

- **`RegisterAndOffset`**

  The CFA is given offset from the given register's value.

- **`Expression`**

  The CFA is obtained by evaluating a DWARF expression program.

#### Implementations

- `fn is_default(self: &Self) -> bool`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + ReaderOffset> Clone for CfaRule<T>`

- `fn clone(self: &Self) -> CfaRule<T>` — [`CfaRule`](#cfarule)

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for CfaRule<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: ReaderOffset> Default for CfaRule<T>`

- `fn default() -> Self`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for CfaRule<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for CfaRule<T>`

- `fn eq(self: &Self, other: &CfaRule<T>) -> bool` — [`CfaRule`](#cfarule)

##### `impl<T: ReaderOffset> StructuralPartialEq for CfaRule<T>`

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

- `fn is_defined(self: &Self) -> bool`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + ReaderOffset> Clone for RegisterRule<T>`

- `fn clone(self: &Self) -> RegisterRule<T>` — [`RegisterRule`](#registerrule)

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for RegisterRule<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for RegisterRule<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for RegisterRule<T>`

- `fn eq(self: &Self, other: &RegisterRule<T>) -> bool` — [`RegisterRule`](#registerrule)

##### `impl<T: ReaderOffset> StructuralPartialEq for RegisterRule<T>`

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

- `fn parse<R: Reader<Offset = T>>(input: &mut R, address_encoding: Option<DwEhPe>, parameters: &PointerEncodingParameters<'_, R>, vendor: Vendor) -> Result<CallFrameInstruction<T>>` — [`DwEhPe`](../index.md), [`PointerEncodingParameters`](cfi/index.md), [`Vendor`](../index.md), [`Result`](../index.md), [`CallFrameInstruction`](#callframeinstruction)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + ReaderOffset> Clone for CallFrameInstruction<T>`

- `fn clone(self: &Self) -> CallFrameInstruction<T>` — [`CallFrameInstruction`](#callframeinstruction)

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for CallFrameInstruction<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for CallFrameInstruction<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for CallFrameInstruction<T>`

- `fn eq(self: &Self, other: &CallFrameInstruction<T>) -> bool` — [`CallFrameInstruction`](#callframeinstruction)

##### `impl<T: ReaderOffset> StructuralPartialEq for CallFrameInstruction<T>`

### `Pointer`

```rust
enum Pointer {
    Direct(u64),
    Indirect(u64),
}
```

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

- `fn new(encoding: constants::DwEhPe, address: u64) -> Pointer` — [`DwEhPe`](../index.md), [`Pointer`](#pointer)

- `fn direct(self: Self) -> Result<u64>` — [`Result`](../index.md)

- `fn pointer(self: Self) -> u64`

#### Trait Implementations

##### `impl Clone for Pointer`

- `fn clone(self: &Self) -> Pointer` — [`Pointer`](#pointer)

##### `impl Copy for Pointer`

##### `impl Debug for Pointer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Pointer`

- `fn default() -> Self`

##### `impl Eq for Pointer`

##### `impl PartialEq for Pointer`

- `fn eq(self: &Self, other: &Pointer) -> bool` — [`Pointer`](#pointer)

##### `impl StructuralPartialEq for Pointer`

### `RangeIterInner<R: Reader>`

```rust
enum RangeIterInner<R: Reader> {
    Single(Option<crate::read::Range>),
    List(crate::read::RngListIter<R>),
}
```

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RangeIterInner<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)

##### `impl Copy for AbbreviationsCacheStrategy`

##### `impl Debug for AbbreviationsCacheStrategy`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AbbreviationsCacheStrategy`

##### `impl PartialEq for AbbreviationsCacheStrategy`

- `fn eq(self: &Self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)

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

- `fn new() -> Attributes` — [`Attributes`](abbrev/index.md)

- `fn push(self: &mut Self, attr: AttributeSpecification)` — [`AttributeSpecification`](#attributespecification)

#### Trait Implementations

##### `impl Clone for Attributes`

- `fn clone(self: &Self) -> Attributes` — [`Attributes`](abbrev/index.md)

##### `impl Debug for Attributes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- `type Target = [AttributeSpecification]`

- `fn deref(self: &Self) -> &[AttributeSpecification]` — [`AttributeSpecification`](#attributespecification)

##### `impl Eq for Attributes`

##### `impl FromIterator for Attributes`

- `fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](abbrev/index.md)

##### `impl PartialEq for Attributes`

- `fn eq(self: &Self, other: &Attributes) -> bool` — [`Attributes`](abbrev/index.md)

##### `impl<P, T> Receiver for Attributes`

- `type Target = T`

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

- `fn section_id(self: Self) -> SectionId` — [`SectionId`](../index.md)

- `fn dwo_name(self: Self) -> &'static str`

#### Trait Implementations

##### `impl Clone for IndexSectionId`

- `fn clone(self: &Self) -> IndexSectionId` — [`IndexSectionId`](#indexsectionid)

##### `impl Copy for IndexSectionId`

##### `impl Debug for IndexSectionId`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for IndexSectionId`

##### `impl PartialEq for IndexSectionId`

- `fn eq(self: &Self, other: &IndexSectionId) -> bool` — [`IndexSectionId`](#indexsectionid)

##### `impl StructuralPartialEq for IndexSectionId`

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

- `fn parse<'header>(header: &'header LineProgramHeader<R>, input: &mut R) -> Result<LineInstruction<R>>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md), [`LineInstruction`](#lineinstruction)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineInstruction<R, Offset>`

- `fn clone(self: &Self) -> LineInstruction<R, Offset>` — [`LineInstruction`](#lineinstruction)

##### `impl<R, Offset> Copy for LineInstruction<R, Offset>`

##### `impl<R, Offset> Debug for LineInstruction<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for LineInstruction<R, Offset>`

##### `impl<R, Offset> PartialEq for LineInstruction<R, Offset>`

- `fn eq(self: &Self, other: &LineInstruction<R, Offset>) -> bool` — [`LineInstruction`](#lineinstruction)

##### `impl<R, Offset> StructuralPartialEq for LineInstruction<R, Offset>`

### `ColumnType`

```rust
enum ColumnType {
    LeftEdge,
    Column(core::num::NonZeroU64),
}
```

The type of column that a row is referring to.

#### Variants

- **`LeftEdge`**

  The `LeftEdge` means that the statement begins at the start of the new
  line.

- **`Column`**

  A column number, whose range begins at 1.

#### Trait Implementations

##### `impl Clone for ColumnType`

- `fn clone(self: &Self) -> ColumnType` — [`ColumnType`](#columntype)

##### `impl Copy for ColumnType`

##### `impl Debug for ColumnType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ColumnType`

##### `impl Ord for ColumnType`

- `fn cmp(self: &Self, other: &ColumnType) -> $crate::cmp::Ordering` — [`ColumnType`](#columntype)

##### `impl PartialEq for ColumnType`

- `fn eq(self: &Self, other: &ColumnType) -> bool` — [`ColumnType`](#columntype)

##### `impl PartialOrd for ColumnType`

- `fn partial_cmp(self: &Self, other: &ColumnType) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ColumnType`](#columntype)

##### `impl StructuralPartialEq for ColumnType`

### `LocListsFormat`

```rust
enum LocListsFormat {
    Bare,
    Lle,
}
```

#### Variants

- **`Bare`**

  The bare location list format used before DWARF 5.

- **`Lle`**

  The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU
  split dwarf extension.

#### Trait Implementations

##### `impl Clone for LocListsFormat`

- `fn clone(self: &Self) -> LocListsFormat` — [`LocListsFormat`](loclists/index.md)

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl PartialEq for LocListsFormat`

- `fn eq(self: &Self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](loclists/index.md)

##### `impl StructuralPartialEq for LocListsFormat`

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

- `fn parse(input: &mut R, encoding: Encoding, format: LocListsFormat) -> Result<Option<Self>>` — [`Encoding`](../index.md), [`LocListsFormat`](loclists/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for RawLocListEntry<R>`

- `fn clone(self: &Self) -> RawLocListEntry<R>` — [`RawLocListEntry`](#rawloclistentry)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn string(self: &Self, unit: UnitRef<'_, R>) -> Result<R>` — [`UnitRef`](#unitref), [`Result`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for MacroString<R, Offset>`

- `fn clone(self: &Self) -> MacroString<R, Offset>` — [`MacroString`](#macrostring)

##### `impl<R, Offset> Debug for MacroString<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for MacroString<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroString<R, Offset>`

- `fn eq(self: &Self, other: &MacroString<R, Offset>) -> bool` — [`MacroString`](#macrostring)

##### `impl<R, Offset> StructuralPartialEq for MacroString<R, Offset>`

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

##### `impl<R, Offset> Clone for MacroEntry<R, Offset>`

- `fn clone(self: &Self) -> MacroEntry<R, Offset>` — [`MacroEntry`](#macroentry)

##### `impl<R, Offset> Debug for MacroEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for MacroEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroEntry<R, Offset>`

- `fn eq(self: &Self, other: &MacroEntry<R, Offset>) -> bool` — [`MacroEntry`](#macroentry)

##### `impl<R, Offset> StructuralPartialEq for MacroEntry<R, Offset>`

### `DieReference<T>`

```rust
enum DieReference<T> {
    UnitRef(crate::read::UnitOffset<T>),
    DebugInfoRef(crate::common::DebugInfoOffset<T>),
}
```

A reference to a DIE, either relative to the current CU or
relative to the section.

#### Variants

- **`UnitRef`**

  A CU-relative reference.

- **`DebugInfoRef`**

  A section-relative reference.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for DieReference<T>`

- `fn clone(self: &Self) -> DieReference<T>` — [`DieReference`](#diereference)

##### `impl<T: $crate::marker::Copy> Copy for DieReference<T>`

##### `impl<T: $crate::fmt::Debug> Debug for DieReference<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for DieReference<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for DieReference<T>`

- `fn eq(self: &Self, other: &DieReference<T>) -> bool` — [`DieReference`](#diereference)

##### `impl<T> StructuralPartialEq for DieReference<T>`

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

- `fn parse(bytes: &mut R, encoding: Encoding) -> Result<Operation<R, Offset>>` — [`Encoding`](../index.md), [`Result`](../index.md), [`Operation`](#operation)

#### Trait Implementations

##### `impl<R, Offset> Clone for Operation<R, Offset>`

- `fn clone(self: &Self) -> Operation<R, Offset>` — [`Operation`](#operation)

##### `impl<R, Offset> Copy for Operation<R, Offset>`

##### `impl<R, Offset> Debug for Operation<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for Operation<R, Offset>`

##### `impl<R, Offset> PartialEq for Operation<R, Offset>`

- `fn eq(self: &Self, other: &Operation<R, Offset>) -> bool` — [`Operation`](#operation)

##### `impl<R, Offset> StructuralPartialEq for Operation<R, Offset>`

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

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for OperationEvaluationResult<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl<R, Offset> Clone for Location<R, Offset>`

- `fn clone(self: &Self) -> Location<R, Offset>` — [`Location`](#location)

##### `impl<R, Offset> Copy for Location<R, Offset>`

##### `impl<R, Offset> Debug for Location<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> PartialEq for Location<R, Offset>`

- `fn eq(self: &Self, other: &Location<R, Offset>) -> bool` — [`Location`](#location)

##### `impl<R, Offset> StructuralPartialEq for Location<R, Offset>`

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

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EvaluationState<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

#### Trait Implementations

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EvaluationWaiting<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EvaluationResult<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for EvaluationResult<R>`

- `fn eq(self: &Self, other: &EvaluationResult<R>) -> bool` — [`EvaluationResult`](#evaluationresult)

##### `impl<R: Reader> StructuralPartialEq for EvaluationResult<R>`

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Clone for RangeListsFormat`

- `fn clone(self: &Self) -> RangeListsFormat` — [`RangeListsFormat`](rnglists/index.md)

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl PartialEq for RangeListsFormat`

- `fn eq(self: &Self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](rnglists/index.md)

##### `impl StructuralPartialEq for RangeListsFormat`

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

- `fn parse<R: Reader<Offset = T>>(input: &mut R, encoding: Encoding, format: RangeListsFormat) -> Result<Option<Self>>` — [`Encoding`](../index.md), [`RangeListsFormat`](rnglists/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RawRngListEntry<T>`

- `fn clone(self: &Self) -> RawRngListEntry<T>` — [`RawRngListEntry`](#rawrnglistentry)

##### `impl<T: $crate::fmt::Debug> Debug for RawRngListEntry<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn dw_ut(self: &Self) -> constants::DwUt` — [`DwUt`](../index.md)

#### Trait Implementations

##### `impl<Offset> Clone for UnitType<Offset>`

- `fn clone(self: &Self) -> UnitType<Offset>` — [`UnitType`](#unittype)

##### `impl<Offset> Copy for UnitType<Offset>`

##### `impl<Offset> Debug for UnitType<Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<Offset> Eq for UnitType<Offset>`

##### `impl<Offset> PartialEq for UnitType<Offset>`

- `fn eq(self: &Self, other: &UnitType<Offset>) -> bool` — [`UnitType`](#unittype)

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

- `fn offset_value(self: &Self) -> Option<<R as >::Offset>` — [`Reader`](#reader)

- `fn exprloc_value(self: &Self) -> Option<Expression<R>>` — [`Expression`](#expression)

- `fn string_value(self: &Self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](#debugstr)

- `fn string_value_sup(self: &Self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](#debugstr)

#### Trait Implementations

##### `impl<R, Offset> Clone for AttributeValue<R, Offset>`

- `fn clone(self: &Self) -> AttributeValue<R, Offset>` — [`AttributeValue`](#attributevalue)

##### `impl<R, Offset> Copy for AttributeValue<R, Offset>`

##### `impl<R, Offset> Debug for AttributeValue<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for AttributeValue<R, Offset>`

##### `impl<R, Offset> PartialEq for AttributeValue<R, Offset>`

- `fn eq(self: &Self, other: &AttributeValue<R, Offset>) -> bool` — [`AttributeValue`](#attributevalue)

##### `impl<R, Offset> StructuralPartialEq for AttributeValue<R, Offset>`

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

- `fn bit_size(self: Self, addr_mask: u64) -> u32`

- `fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` — [`DwAte`](../index.md), [`ValueType`](#valuetype)

- `fn from_entry<R: Reader>(entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<Option<ValueType>>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md), [`ValueType`](#valuetype)

#### Trait Implementations

##### `impl Clone for ValueType`

- `fn clone(self: &Self) -> ValueType` — [`ValueType`](#valuetype)

##### `impl Copy for ValueType`

##### `impl Debug for ValueType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ValueType`

##### `impl PartialEq for ValueType`

- `fn eq(self: &Self, other: &ValueType) -> bool` — [`ValueType`](#valuetype)

##### `impl StructuralPartialEq for ValueType`

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

- `fn value_type(self: &Self) -> ValueType` — [`ValueType`](#valuetype)

- `fn parse<R: Reader>(value_type: ValueType, bytes: R) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`Value`](#value)

- `fn to_u64(self: Self, addr_mask: u64) -> Result<u64>` — [`Result`](../index.md)

- `fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`Value`](#value)

- `fn from_f32(value_type: ValueType, value: f32) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`Value`](#value)

- `fn from_f64(value_type: ValueType, value: f64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`Value`](#value)

- `fn convert(self: Self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`Value`](#value)

- `fn reinterpret(self: Self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md), [`Value`](#value)

- `fn abs(self: Self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md), [`Value`](#value)

- `fn neg(self: Self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md), [`Value`](#value)

- `fn add(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn sub(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn mul(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn div(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn rem(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn not(self: Self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md), [`Value`](#value)

- `fn and(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn or(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn xor(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn shift_length(self: Self) -> Result<u64>` — [`Result`](../index.md)

- `fn shl(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn shr(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn shra(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn eq(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn ge(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn gt(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn le(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn lt(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

- `fn ne(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for Value`

- `fn clone(self: &Self) -> Value` — [`Value`](#value)

##### `impl Copy for Value`

##### `impl Debug for Value`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for Value`

- `fn eq(self: &Self, other: &Value) -> bool` — [`Value`](#value)

##### `impl StructuralPartialEq for Value`

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

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Required Methods

- `type Item`

### `UnwindOffset<T>`

```rust
trait UnwindOffset<T>: Copy + Debug + Eq + From<T>
where
    T: ReaderOffset { ... }
```

An offset into an `UnwindSection`.

#### Required Methods

- `fn into(self: Self) -> T`

  Convert an `UnwindOffset<T>` into a `T`.

### `UnwindSection<R: Reader>`

```rust
trait UnwindSection<R: Reader>: Clone + Debug + _UnwindSectionPrivate<R> { ... }
```

A section holding unwind information: either `.debug_frame` or
`.eh_frame`. See [`DebugFrame`](./struct.DebugFrame.html) and
[`EhFrame`](./struct.EhFrame.html) respectively.

#### Required Methods

- `type Offset: 1`

- `fn entries<'bases>(self: &Self, bases: &'bases BaseAddresses) -> CfiEntriesIter<'bases, Self, R>`

  Iterate over the `CommonInformationEntry`s and `FrameDescriptionEntry`s

- `fn cie_from_offset(self: &Self, bases: &BaseAddresses, offset: <Self as >::Offset) -> Result<CommonInformationEntry<R>>`

  Parse the `CommonInformationEntry` at the given offset.

- `fn partial_fde_from_offset<'bases>(self: &Self, bases: &'bases BaseAddresses, offset: <Self as >::Offset) -> Result<PartialFrameDescriptionEntry<'bases, Self, R>>`

  Parse the `PartialFrameDescriptionEntry` at the given offset.

- `fn fde_from_offset<F>(self: &Self, bases: &BaseAddresses, offset: <Self as >::Offset, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Parse the `FrameDescriptionEntry` at the given offset.

- `fn fde_for_address<F>(self: &Self, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Find the `FrameDescriptionEntry` for the given address.

- `fn unwind_info_for_address<'ctx, F, S>(self: &Self, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>`

  Find the frame unwind information for the given address.

### `UnwindContextStorage<T: ReaderOffset>`

```rust
trait UnwindContextStorage<T: ReaderOffset>: Sized { ... }
```

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

#### Required Methods

- `type Rules: 1`

- `type Stack: 1`

### `ReaderOffset`

```rust
trait ReaderOffset: Debug + Copy + Eq + Ord + Hash + Add<Output = Self> + AddAssign + Sub<Output = Self> { ... }
```

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

- `fn into_u64(self: Self) -> u64`

  Convert an offset to a u64.

- `fn wrapping_add(self: Self, other: Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`.

- `fn checked_sub(self: Self, other: Self) -> Option<Self>`

  Checked subtraction. Computes `self - other`.

### `ReaderAddress`

```rust
trait ReaderAddress: Sized { ... }
```

A trait for addresses within a DWARF section.

Currently this is a simple extension trait for `u64`, but it may be expanded
in the future to support user-defined address types.

#### Required Methods

- `fn add_sized(self: Self, length: u64, size: u8) -> Result<Self>`

  Add a length to an address of the given size.

- `fn wrapping_add_sized(self: Self, length: u64, size: u8) -> Self`

  Add a length to an address of the given size.

- `fn zeros() -> Self`

  The all-zeros value of an address.

- `fn ones_sized(size: u8) -> Self`

  The all-ones value of an address of the given size.

- `fn min_tombstone(size: u8) -> Self`

  Return the minimum value for a tombstone address.

### `Reader`

```rust
trait Reader: Debug + Clone { ... }
```

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

#### Required Methods

- `type Endian: 1`

- `type Offset: 1`

- `fn endian(self: &Self) -> <Self as >::Endian`

  Return the endianity of bytes that are read.

- `fn len(self: &Self) -> <Self as >::Offset`

  Return the number of bytes remaining.

- `fn empty(self: &mut Self)`

  Set the number of bytes remaining to zero.

- `fn truncate(self: &mut Self, len: <Self as >::Offset) -> Result<()>`

  Set the number of bytes remaining to the specified length.

- `fn offset_from(self: &Self, base: &Self) -> <Self as >::Offset`

  Return the offset of this reader's data relative to the start of

- `fn offset_id(self: &Self) -> ReaderOffsetId`

  Return an identifier for the current reader offset.

- `fn lookup_offset_id(self: &Self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`

  Return the offset corresponding to the given `id` if

- `fn find(self: &Self, byte: u8) -> Result<<Self as >::Offset>`

  Find the index of the first occurrence of the given byte.

- `fn skip(self: &mut Self, len: <Self as >::Offset) -> Result<()>`

  Discard the specified number of bytes.

- `fn split(self: &mut Self, len: <Self as >::Offset) -> Result<Self>`

  Split a reader in two.

- `fn to_slice(self: &Self) -> Result<Cow<'_, [u8]>>`

  Return all remaining data as a clone-on-write slice.

- `fn to_string(self: &Self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string.

- `fn to_string_lossy(self: &Self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string, including invalid characters.

- `fn read_slice(self: &mut Self, buf: &mut [u8]) -> Result<()>`

  Read exactly `buf.len()` bytes into `buf`.

- `fn read_u8_array<A>(self: &mut Self) -> Result<A>`

  Read a u8 array.

- `fn is_empty(self: &Self) -> bool`

  Return true if the number of bytes remaining is zero.

- `fn read_u8(self: &mut Self) -> Result<u8>`

  Read a u8.

- `fn read_i8(self: &mut Self) -> Result<i8>`

  Read an i8.

- `fn read_u16(self: &mut Self) -> Result<u16>`

  Read a u16.

- `fn read_i16(self: &mut Self) -> Result<i16>`

  Read an i16.

- `fn read_u32(self: &mut Self) -> Result<u32>`

  Read a u32.

- `fn read_i32(self: &mut Self) -> Result<i32>`

  Read an i32.

- `fn read_u64(self: &mut Self) -> Result<u64>`

  Read a u64.

- `fn read_i64(self: &mut Self) -> Result<i64>`

  Read an i64.

- `fn read_f32(self: &mut Self) -> Result<f32>`

  Read a f32.

- `fn read_f64(self: &mut Self) -> Result<f64>`

  Read a f64.

- `fn read_uint(self: &mut Self, n: usize) -> Result<u64>`

  Read an unsigned n-bytes integer u64.

- `fn read_null_terminated_slice(self: &mut Self) -> Result<Self>`

  Read a null-terminated slice, and return it (excluding the null).

- `fn skip_leb128(self: &mut Self) -> Result<()>`

  Skip a LEB128 encoded integer.

- `fn read_uleb128(self: &mut Self) -> Result<u64>`

  Read an unsigned LEB128 encoded integer.

- `fn read_uleb128_u32(self: &mut Self) -> Result<u32>`

  Read an unsigned LEB128 encoded u32.

- `fn read_uleb128_u16(self: &mut Self) -> Result<u16>`

  Read an unsigned LEB128 encoded u16.

- `fn read_sleb128(self: &mut Self) -> Result<i64>`

  Read a signed LEB128 encoded integer.

- `fn read_initial_length(self: &mut Self) -> Result<(<Self as >::Offset, Format)>`

  Read an initial length field.

- `fn read_address_size(self: &mut Self) -> Result<u8>`

  Read a byte and validate it as an address size.

- `fn read_address(self: &mut Self, address_size: u8) -> Result<u64>`

  Read an address-sized integer, and return it as a `u64`.

- `fn read_word(self: &mut Self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized integer according to the DWARF format.

- `fn read_length(self: &mut Self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section length according to the DWARF format.

- `fn read_offset(self: &mut Self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section offset according to the DWARF format.

- `fn read_sized_offset(self: &mut Self, size: u8) -> Result<<Self as >::Offset>`

  Parse a section offset of the given size.

### `Relocate<T: ReaderOffset>`

```rust
trait Relocate<T: ReaderOffset> { ... }
```

Trait for relocating addresses and offsets while reading a section.

#### Required Methods

- `fn relocate_address(self: &Self, offset: T, value: u64) -> Result<u64>`

  Relocate an address which was read from the given section offset.

- `fn relocate_offset(self: &Self, offset: T, value: T) -> Result<T>`

  Relocate a value which was read from the given section offset.

### `LineProgram<R, Offset>`

```rust
trait LineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset { ... }
```

A `LineProgram` provides access to a `LineProgramHeader` and
a way to add files to the files table if necessary. Gimli consumers should
never need to use or see this trait.

#### Required Methods

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>`

  Get a reference to the held `LineProgramHeader`.

- `fn add_file(self: &mut Self, file: FileEntry<R, Offset>)`

  Add a file to the file table if necessary.

### `EvaluationStorage<R: Reader>`

```rust
trait EvaluationStorage<R: Reader> { ... }
```

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

#### Required Methods

- `type Stack: 1`

- `type ExpressionStack: 1`

- `type Result: 1`

## Functions

### `parse_cfi_entry`

```rust
fn parse_cfi_entry<'bases, Section, R>(bases: &'bases BaseAddresses, section: &Section, input: &mut R) -> crate::read::Result<Option<CieOrFde<'bases, Section, R>>>
where
    R: Reader,
    Section: UnwindSection<R>
```

### `parse_encoded_pointer`

```rust
fn parse_encoded_pointer<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<Pointer>
```

### `parse_encoded_value`

```rust
fn parse_encoded_value<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<u64>
```

### `get_attribute_size`

```rust
fn get_attribute_size(form: constants::DwForm, encoding: crate::common::Encoding) -> Option<u8>
```

### `parse_directory_v5`

```rust
fn parse_directory_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<crate::read::AttributeValue<R>>
```

### `parse_file_v5`

```rust
fn parse_file_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<FileEntry<R>>
```

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, form: constants::DwForm) -> crate::read::Result<crate::read::AttributeValue<R>>
```

### `parse_data`

```rust
fn parse_data<R: Reader>(input: &mut R, encoding: crate::common::Encoding) -> crate::read::Result<crate::read::Expression<R>>
```

### `compute_pc`

```rust
fn compute_pc<R: Reader>(pc: &R, bytecode: &R, offset: i16) -> crate::read::Result<R>
```

### `generic_type`

```rust
fn generic_type<O: ReaderOffset>() -> crate::read::UnitOffset<O>
```

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

### `sign_extend`

```rust
fn sign_extend(value: u64, mask: u64) -> i64
```

Convert a u64 to an i64, with sign extension if required.

This is primarily used when needing to treat `Value::Generic`
as a signed value.

### `mask_bit_size`

```rust
fn mask_bit_size(addr_mask: u64) -> u32
```

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

### `LineNumberProgram<R, Offset>`

```rust
type LineNumberProgram<R, Offset> = dyn LineProgram<R, Offset>;
```

Deprecated. `LineNumberProgram` has been renamed to `LineProgram`.

### `StateMachine<R, Program, Offset>`

```rust
type StateMachine<R, Program, Offset> = LineRows<R, Program, Offset>;
```

Deprecated. `StateMachine` has been renamed to `LineRows`.

### `OneShotLineRows<R, Offset>`

```rust
type OneShotLineRows<R, Offset> = LineRows<R, IncompleteLineProgram<R, Offset>, Offset>;
```

### `ResumedLineRows<'program, R, Offset>`

```rust
type ResumedLineRows<'program, R, Offset> = LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>;
```

### `Opcode<R>`

```rust
type Opcode<R> = LineInstruction<R, <R as Reader>::Offset>;
```

Deprecated. `Opcode` has been renamed to `LineInstruction`.

### `OpcodesIter<R>`

```rust
type OpcodesIter<R> = LineInstructions<R>;
```

Deprecated. `OpcodesIter` has been renamed to `LineInstructions`.

### `LineNumberRow`

```rust
type LineNumberRow = LineRow;
```

Deprecated. `LineNumberRow` has been renamed to `LineRow`.

### `LineNumberSequence<R>`

```rust
type LineNumberSequence<R> = LineSequence<R>;
```

Deprecated. `LineNumberSequence` has been renamed to `LineSequence`.

### `LineNumberProgramHeader<R, Offset>`

```rust
type LineNumberProgramHeader<R, Offset> = LineProgramHeader<R, Offset>;
```

Deprecated. `LineNumberProgramHeader` has been renamed to `LineProgramHeader`.

### `IncompleteLineNumberProgram<R, Offset>`

```rust
type IncompleteLineNumberProgram<R, Offset> = IncompleteLineProgram<R, Offset>;
```

Deprecated. `IncompleteLineNumberProgram` has been renamed to `IncompleteLineProgram`.

### `CompleteLineNumberProgram<R, Offset>`

```rust
type CompleteLineNumberProgram<R, Offset> = CompleteLineProgram<R, Offset>;
```

Deprecated. `CompleteLineNumberProgram` has been renamed to `CompleteLineProgram`.

### `LocListsHeader`

```rust
type LocListsHeader = crate::read::lists::ListsHeader;
```

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

## Constants

### `MAX_RULES`

```rust
const MAX_RULES: usize = 192usize;
```

### `MAX_UNWIND_STACK_DEPTH`

```rust
const MAX_UNWIND_STACK_DEPTH: usize = 4usize;
```

### `CFI_INSTRUCTION_HIGH_BITS_MASK`

```rust
const CFI_INSTRUCTION_HIGH_BITS_MASK: u8 = 192u8;
```

### `CFI_INSTRUCTION_LOW_BITS_MASK`

```rust
const CFI_INSTRUCTION_LOW_BITS_MASK: u8 = 63u8;
```

### `MAX_ATTRIBUTES_INLINE`

```rust
const MAX_ATTRIBUTES_INLINE: usize = 5usize;
```

### `SECTION_COUNT_MAX`

```rust
const SECTION_COUNT_MAX: u8 = 8u8;
```

