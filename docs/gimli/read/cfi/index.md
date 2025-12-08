*[gimli](../../index.md) / [read](../index.md) / [cfi](index.md)*

---

# Module `cfi`

## Structs

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

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for DebugFrame<R>`

- `fn clone(self: &Self) -> DebugFrame<R>` — [`DebugFrame`](../index.md)

##### `impl<R: $crate::marker::Copy + Reader> Copy for DebugFrame<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for DebugFrame<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for DebugFrame<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for DebugFrame<R>`

- `fn eq(self: &Self, other: &DebugFrame<R>) -> bool` — [`DebugFrame`](../index.md)

##### `impl<R: Reader> Section for DebugFrame<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

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

- `fn new(section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for EhFrameHdr<R>`

- `fn clone(self: &Self) -> EhFrameHdr<R>` — [`EhFrameHdr`](../index.md)

##### `impl<R: $crate::marker::Copy + Reader> Copy for EhFrameHdr<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EhFrameHdr<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for EhFrameHdr<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for EhFrameHdr<R>`

- `fn eq(self: &Self, other: &EhFrameHdr<R>) -> bool` — [`EhFrameHdr`](../index.md)

##### `impl<R: Reader> Section for EhFrameHdr<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

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

- `fn eh_frame_ptr(self: &Self) -> Pointer` — [`Pointer`](../index.md)

- `fn table(self: &Self) -> Option<EhHdrTable<'_, R>>` — [`EhHdrTable`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for ParsedEhFrameHdr<R>`

- `fn clone(self: &Self) -> ParsedEhFrameHdr<R>` — [`ParsedEhFrameHdr`](../index.md)

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

- `fn next(self: &mut Self) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../../index.md), [`Pointer`](../index.md)

- `fn nth(self: &mut Self, n: usize) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../../index.md), [`Pointer`](../index.md)

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

- `fn iter<'bases>(self: &Self, bases: &'bases BaseAddresses) -> EhHdrTableIter<'_, 'bases, R>` — [`BaseAddresses`](../index.md), [`EhHdrTableIter`](../index.md)

- `fn lookup(self: &Self, address: u64, bases: &BaseAddresses) -> Result<Pointer>` — [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`Pointer`](../index.md)

- `fn pointer_to_offset(self: &Self, ptr: Pointer) -> Result<EhFrameOffset<<R as >::Offset>>` — [`Pointer`](../index.md), [`Result`](../../index.md), [`EhFrameOffset`](../../index.md), [`Reader`](../index.md)

- `fn fde_for_address<F>(self: &Self, frame: &EhFrame<R>, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`EhFrame`](../index.md), [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`FrameDescriptionEntry`](../index.md)

- `fn unwind_info_for_address<'ctx, F, S>(self: &Self, frame: &EhFrame<R>, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`EhFrame`](../index.md), [`BaseAddresses`](../index.md), [`UnwindContext`](../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`UnwindTableRow`](../index.md)

#### Trait Implementations

##### `impl<'a, R: $crate::clone::Clone + Reader> Clone for EhHdrTable<'a, R>`

- `fn clone(self: &Self) -> EhHdrTable<'a, R>` — [`EhHdrTable`](../index.md)

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

- `fn clone(self: &Self) -> EhFrame<R>` — [`EhFrame`](../index.md)

##### `impl<R: $crate::marker::Copy + Reader> Copy for EhFrame<R>`

##### `impl<R: $crate::fmt::Debug + Reader> Debug for EhFrame<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::cmp::Eq + Reader> Eq for EhFrame<R>`

##### `impl<R: $crate::cmp::PartialEq + Reader> PartialEq for EhFrame<R>`

- `fn eq(self: &Self, other: &EhFrame<R>) -> bool` — [`EhFrame`](../index.md)

##### `impl<R: Reader> Section for EhFrame<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

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

- `fn clone(self: &Self) -> BaseAddresses` — [`BaseAddresses`](../index.md)

##### `impl Debug for BaseAddresses`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BaseAddresses`

- `fn default() -> BaseAddresses` — [`BaseAddresses`](../index.md)

##### `impl Eq for BaseAddresses`

##### `impl PartialEq for BaseAddresses`

- `fn eq(self: &Self, other: &BaseAddresses) -> bool` — [`BaseAddresses`](../index.md)

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

- `fn clone(self: &Self) -> SectionBaseAddresses` — [`SectionBaseAddresses`](../index.md)

##### `impl Debug for SectionBaseAddresses`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SectionBaseAddresses`

- `fn default() -> SectionBaseAddresses` — [`SectionBaseAddresses`](../index.md)

##### `impl Eq for SectionBaseAddresses`

##### `impl PartialEq for SectionBaseAddresses`

- `fn eq(self: &Self, other: &SectionBaseAddresses) -> bool` — [`SectionBaseAddresses`](../index.md)

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

- `fn next(self: &mut Self) -> Result<Option<CieOrFde<'bases, Section, R>>>` — [`Result`](../../index.md), [`CieOrFde`](../index.md)

#### Trait Implementations

##### `impl<'bases, Section, R> Clone for CfiEntriesIter<'bases, Section, R>`

- `fn clone(self: &Self) -> CfiEntriesIter<'bases, Section, R>` — [`CfiEntriesIter`](../index.md)

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

- `fn parse<Section, R>(augmentation_str: &mut R, bases: &BaseAddresses, address_size: u8, section: &Section, input: &mut R) -> Result<Augmentation>` — [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`Augmentation`](../index.md)

#### Trait Implementations

##### `impl Clone for Augmentation`

- `fn clone(self: &Self) -> Augmentation` — [`Augmentation`](../index.md)

##### `impl Copy for Augmentation`

##### `impl Debug for Augmentation`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Augmentation`

- `fn default() -> Augmentation` — [`Augmentation`](../index.md)

##### `impl Eq for Augmentation`

##### `impl PartialEq for Augmentation`

- `fn eq(self: &Self, other: &Augmentation) -> bool` — [`Augmentation`](../index.md)

##### `impl StructuralPartialEq for Augmentation`

### `AugmentationData`

```rust
struct AugmentationData {
    lsda: Option<Pointer>,
}
```

Parsed augmentation data for a `FrameDescriptEntry`.

#### Implementations

- `fn parse<R: Reader>(augmentation: &Augmentation, encoding_parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> Result<AugmentationData>` — [`Augmentation`](../index.md), [`PointerEncodingParameters`](#pointerencodingparameters), [`Result`](../../index.md), [`AugmentationData`](#augmentationdata)

#### Trait Implementations

##### `impl Clone for AugmentationData`

- `fn clone(self: &Self) -> AugmentationData` — [`AugmentationData`](#augmentationdata)

##### `impl Debug for AugmentationData`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AugmentationData`

- `fn default() -> AugmentationData` — [`AugmentationData`](#augmentationdata)

##### `impl Eq for AugmentationData`

##### `impl PartialEq for AugmentationData`

- `fn eq(self: &Self, other: &AugmentationData) -> bool` — [`AugmentationData`](#augmentationdata)

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

- `fn parse<Section: UnwindSection<R>>(bases: &BaseAddresses, section: &Section, input: &mut R) -> Result<CommonInformationEntry<R>>` — [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`CommonInformationEntry`](../index.md)

- `fn parse_rest<Section: UnwindSection<R>>(offset: <R as >::Offset, length: <R as >::Offset, format: Format, bases: &BaseAddresses, section: &Section, rest: R) -> Result<CommonInformationEntry<R>>` — [`Reader`](../index.md), [`Format`](../../index.md), [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`CommonInformationEntry`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for CommonInformationEntry<R, Offset>`

- `fn clone(self: &Self) -> CommonInformationEntry<R, Offset>` — [`CommonInformationEntry`](../index.md)

##### `impl<R, Offset> Debug for CommonInformationEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for CommonInformationEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for CommonInformationEntry<R, Offset>`

- `fn eq(self: &Self, other: &CommonInformationEntry<R, Offset>) -> bool` — [`CommonInformationEntry`](../index.md)

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

- `fn parse_partial(section: &Section, bases: &'bases BaseAddresses, input: &mut R) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`PartialFrameDescriptionEntry`](../index.md)

- `fn parse<F>(self: &Self, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Result`](../../index.md), [`FrameDescriptionEntry`](../index.md)

- `fn offset(self: &Self) -> <R as >::Offset` — [`Reader`](../index.md)

- `fn cie_offset(self: &Self) -> <Section as >::Offset` — [`UnwindSection`](../index.md)

- `fn entry_len(self: &Self) -> <R as >::Offset` — [`Reader`](../index.md)

#### Trait Implementations

##### `impl<'bases, Section, R> Clone for PartialFrameDescriptionEntry<'bases, Section, R>`

- `fn clone(self: &Self) -> PartialFrameDescriptionEntry<'bases, Section, R>` — [`PartialFrameDescriptionEntry`](../index.md)

##### `impl<'bases, Section, R> Debug for PartialFrameDescriptionEntry<'bases, Section, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'bases, Section, R> Eq for PartialFrameDescriptionEntry<'bases, Section, R>`

##### `impl<'bases, Section, R> PartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

- `fn eq(self: &Self, other: &PartialFrameDescriptionEntry<'bases, Section, R>) -> bool` — [`PartialFrameDescriptionEntry`](../index.md)

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

- `fn parse_rest<Section, F>(offset: <R as >::Offset, length: <R as >::Offset, format: Format, cie_pointer: <Section as >::Offset, rest: R, section: &Section, bases: &BaseAddresses, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Reader`](../index.md), [`Format`](../../index.md), [`BaseAddresses`](../index.md), [`Result`](../../index.md), [`FrameDescriptionEntry`](../index.md)

- `fn parse_addresses(input: &mut R, cie: &CommonInformationEntry<R>, parameters: &PointerEncodingParameters<'_, R>) -> Result<(u64, u64)>` — [`CommonInformationEntry`](../index.md), [`PointerEncodingParameters`](#pointerencodingparameters), [`Result`](../../index.md)

- `fn rows<'a, 'ctx, Section, S>(self: &Self, section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>) -> Result<UnwindTable<'a, 'ctx, R, S>>` — [`BaseAddresses`](../index.md), [`UnwindContext`](../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`UnwindTable`](../index.md)

- `fn unwind_info_for_address<'ctx, Section, S>(self: &Self, section: &Section, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`BaseAddresses`](../index.md), [`UnwindContext`](../index.md), [`Reader`](../index.md), [`Result`](../../index.md), [`UnwindTableRow`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for FrameDescriptionEntry<R, Offset>`

- `fn clone(self: &Self) -> FrameDescriptionEntry<R, Offset>` — [`FrameDescriptionEntry`](../index.md)

##### `impl<R, Offset> Debug for FrameDescriptionEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for FrameDescriptionEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FrameDescriptionEntry<R, Offset>`

- `fn eq(self: &Self, other: &FrameDescriptionEntry<R, Offset>) -> bool` — [`FrameDescriptionEntry`](../index.md)

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
on the heap using [`Box`](../../../allocator_api2/stable/boxed/index.md) during `UnwindContext::new`.

This can be overridden by providing a custom [`UnwindContextStorage`](../index.md) type parameter.
When using a custom storage with in-line arrays, the [`UnwindContext`](../index.md) type itself
will be big, so in that case it's recommended to place [`UnwindContext`](../index.md) on the
heap, e.g. using `Box::new(UnwindContext::<R, MyCustomStorage>::new_in())`.

To avoid re-allocating the context multiple times when evaluating multiple
CFI programs, the same [`UnwindContext`](../index.md) can be reused for multiple unwinds.

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

- `fn new() -> Self`

#### Trait Implementations

##### `impl<T, S> Clone for UnwindContext<T, S>`

- `fn clone(self: &Self) -> UnwindContext<T, S>` — [`UnwindContext`](../index.md)

##### `impl<T, S> Debug for UnwindContext<T, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindContext<T, S>`

- `fn default() -> Self`

##### `impl<T, S> Eq for UnwindContext<T, S>`

##### `impl<T, S> PartialEq for UnwindContext<T, S>`

- `fn eq(self: &Self, other: &UnwindContext<T, S>) -> bool` — [`UnwindContext`](../index.md)

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

- `fn new<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Result<Self>` — [`BaseAddresses`](../index.md), [`UnwindContext`](../index.md), [`Reader`](../index.md), [`FrameDescriptionEntry`](../index.md), [`Result`](../../index.md)

- `fn new_for_fde<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Self` — [`BaseAddresses`](../index.md), [`UnwindContext`](../index.md), [`Reader`](../index.md), [`FrameDescriptionEntry`](../index.md)

- `fn new_for_cie<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, cie: &CommonInformationEntry<R>) -> Self` — [`BaseAddresses`](../index.md), [`UnwindContext`](../index.md), [`Reader`](../index.md), [`CommonInformationEntry`](../index.md)

- `fn next_row(self: &mut Self) -> Result<Option<&UnwindTableRow<<R as >::Offset, S>>>` — [`Result`](../../index.md), [`UnwindTableRow`](../index.md), [`Reader`](../index.md)

- `fn into_current_row(self: Self) -> Option<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`UnwindTableRow`](../index.md), [`Reader`](../index.md)

- `fn evaluate(self: &mut Self, instruction: CallFrameInstruction<<R as >::Offset>) -> Result<bool>` — [`CallFrameInstruction`](../index.md), [`Reader`](../index.md), [`Result`](../../index.md)

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

- `fn get(self: &Self, register: Register) -> RegisterRule<T>` — [`Register`](../../index.md), [`RegisterRule`](../index.md)

- `fn set(self: &mut Self, register: Register, rule: RegisterRule<T>) -> Result<()>` — [`Register`](../../index.md), [`RegisterRule`](../index.md), [`Result`](../../index.md)

- `fn iter(self: &Self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](../index.md)

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

- `fn clone(self: &Self) -> RegisterRuleIter<'iter, T>` — [`RegisterRuleIter`](../index.md)

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

- `fn cfa(self: &Self) -> &CfaRule<T>` — [`CfaRule`](../index.md)

- `fn register(self: &Self, register: Register) -> RegisterRule<T>` — [`Register`](../../index.md), [`RegisterRule`](../index.md)

- `fn registers(self: &Self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](../index.md)

#### Trait Implementations

##### `impl<T, S> Clone for UnwindTableRow<T, S>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S> Debug for UnwindTableRow<T, S>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindTableRow<T, S>`

- `fn default() -> Self`

##### `impl<T, S> Eq for UnwindTableRow<T, S>`

##### `impl<T, S> PartialEq for UnwindTableRow<T, S>`

- `fn eq(self: &Self, other: &UnwindTableRow<T, S>) -> bool` — [`UnwindTableRow`](../index.md)

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

- `fn next(self: &mut Self) -> Result<Option<CallFrameInstruction<<R as >::Offset>>>` — [`Result`](../../index.md), [`CallFrameInstruction`](../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<'a, R: $crate::clone::Clone + Reader> Clone for CallFrameInstructionIter<'a, R>`

- `fn clone(self: &Self) -> CallFrameInstructionIter<'a, R>` — [`CallFrameInstructionIter`](../index.md)

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
`Reader` to avoid lifetime issues when reusing [`UnwindContext`](../index.md).

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

- `fn get<R, S>(self: &Self, section: &S) -> Result<Expression<R>>` — [`Result`](../../index.md), [`Expression`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + ReaderOffset> Clone for UnwindExpression<T>`

- `fn clone(self: &Self) -> UnwindExpression<T>` — [`UnwindExpression`](../index.md)

##### `impl<T: $crate::marker::Copy + ReaderOffset> Copy for UnwindExpression<T>`

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for UnwindExpression<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for UnwindExpression<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for UnwindExpression<T>`

- `fn eq(self: &Self, other: &UnwindExpression<T>) -> bool` — [`UnwindExpression`](../index.md)

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

- `fn clone(self: &Self) -> PointerEncodingParameters<'a, R>` — [`PointerEncodingParameters`](#pointerencodingparameters)

##### `impl<'a, R: $crate::fmt::Debug + Reader> Debug for PointerEncodingParameters<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

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

- `fn clone(self: &Self) -> CieOrFde<'bases, Section, R>` — [`CieOrFde`](../index.md)

##### `impl<'bases, Section, R> Debug for CieOrFde<'bases, Section, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'bases, Section, R> Eq for CieOrFde<'bases, Section, R>`

##### `impl<'bases, Section, R> PartialEq for CieOrFde<'bases, Section, R>`

- `fn eq(self: &Self, other: &CieOrFde<'bases, Section, R>) -> bool` — [`CieOrFde`](../index.md)

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

- `fn clone(self: &Self) -> CfaRule<T>` — [`CfaRule`](../index.md)

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for CfaRule<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: ReaderOffset> Default for CfaRule<T>`

- `fn default() -> Self`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for CfaRule<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for CfaRule<T>`

- `fn eq(self: &Self, other: &CfaRule<T>) -> bool` — [`CfaRule`](../index.md)

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

- `fn clone(self: &Self) -> RegisterRule<T>` — [`RegisterRule`](../index.md)

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for RegisterRule<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for RegisterRule<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for RegisterRule<T>`

- `fn eq(self: &Self, other: &RegisterRule<T>) -> bool` — [`RegisterRule`](../index.md)

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

- `fn parse<R: Reader<Offset = T>>(input: &mut R, address_encoding: Option<DwEhPe>, parameters: &PointerEncodingParameters<'_, R>, vendor: Vendor) -> Result<CallFrameInstruction<T>>` — [`DwEhPe`](../../index.md), [`PointerEncodingParameters`](#pointerencodingparameters), [`Vendor`](../../index.md), [`Result`](../../index.md), [`CallFrameInstruction`](../index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + ReaderOffset> Clone for CallFrameInstruction<T>`

- `fn clone(self: &Self) -> CallFrameInstruction<T>` — [`CallFrameInstruction`](../index.md)

##### `impl<T: $crate::fmt::Debug + ReaderOffset> Debug for CallFrameInstruction<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq + ReaderOffset> Eq for CallFrameInstruction<T>`

##### `impl<T: $crate::cmp::PartialEq + ReaderOffset> PartialEq for CallFrameInstruction<T>`

- `fn eq(self: &Self, other: &CallFrameInstruction<T>) -> bool` — [`CallFrameInstruction`](../index.md)

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

- `fn new(encoding: constants::DwEhPe, address: u64) -> Pointer` — [`DwEhPe`](../../index.md), [`Pointer`](../index.md)

- `fn direct(self: Self) -> Result<u64>` — [`Result`](../../index.md)

- `fn pointer(self: Self) -> u64`

#### Trait Implementations

##### `impl Clone for Pointer`

- `fn clone(self: &Self) -> Pointer` — [`Pointer`](../index.md)

##### `impl Copy for Pointer`

##### `impl Debug for Pointer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Pointer`

- `fn default() -> Self`

##### `impl Eq for Pointer`

##### `impl PartialEq for Pointer`

- `fn eq(self: &Self, other: &Pointer) -> bool` — [`Pointer`](../index.md)

##### `impl StructuralPartialEq for Pointer`

## Traits

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

Specification of what storage should be used for [`UnwindContext`](../index.md).

Normally you would only need to use [`StoreOnHeap`](../../index.md), which places the stack
on the heap using [`Box`](../../../allocator_api2/stable/boxed/index.md). This is the default storage type parameter for [`UnwindContext`](../index.md).

You may want to supply your own storage type for one of the following reasons:

  1. In rare cases you may run into failed unwinds due to the fixed stack size
     used by [`StoreOnHeap`](../../index.md), so you may want to try a larger `Box`. If denial
     of service is not a concern, then you could also try a `Vec`-based stack which
     can grow as needed.
  2. You may want to avoid heap allocations entirely. You can use a fixed-size
     stack with in-line arrays, which will place the entire storage in-line into
     [`UnwindContext`](../index.md).

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

