*[gimli](../../index.md) / [read](../index.md) / [line](index.md)*

---

# Module `line`

## Structs

### `DebugLine<R>`

```rust
struct DebugLine<R> {
    debug_line_section: R,
}
```

The `DebugLine` struct contains the source location to instruction mapping
found in the `.debug_line` section.

#### Implementations

- `fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> DebugLine<R>` — [`DebugLine`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugLine<R>`

- `fn clone(self: &Self) -> DebugLine<R>` — [`DebugLine`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugLine<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugLine<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugLine<R>`

- `fn default() -> DebugLine<R>` — [`DebugLine`](../index.md)

##### `impl<R> Section for DebugLine<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

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

- `fn new(program: IncompleteLineProgram<R, Offset>) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`IncompleteLineProgram`](../index.md), [`LineRows`](../index.md)

- `fn resume<'program>(program: &'program CompleteLineProgram<R, Offset>, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`CompleteLineProgram`](../index.md), [`LineSequence`](../index.md), [`LineRows`](../index.md)

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md)

- `fn next_row(self: &mut Self) -> Result<Option<(&LineProgramHeader<R, Offset>, &LineRow)>>` — [`Result`](../../index.md), [`LineProgramHeader`](../index.md), [`LineRow`](../index.md)

#### Trait Implementations

##### `impl<R, Program, Offset> Clone for LineRows<R, Program, Offset>`

- `fn clone(self: &Self) -> LineRows<R, Program, Offset>` — [`LineRows`](../index.md)

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

- `fn next_instruction(self: &mut Self, header: &LineProgramHeader<R>) -> Result<Option<LineInstruction<R>>>` — [`LineProgramHeader`](../index.md), [`Result`](../../index.md), [`LineInstruction`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for LineInstructions<R>`

- `fn clone(self: &Self) -> LineInstructions<R>` — [`LineInstructions`](../index.md)

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

- `fn new<R: Reader>(header: &LineProgramHeader<R>) -> Self` — [`LineProgramHeader`](../index.md)

- `fn address(self: &Self) -> u64`

- `fn op_index(self: &Self) -> u64`

- `fn file_index(self: &Self) -> u64`

- `fn file<'header, R: Reader>(self: &Self, header: &'header LineProgramHeader<R>) -> Option<&'header FileEntry<R>>` — [`LineProgramHeader`](../index.md), [`FileEntry`](../index.md)

- `fn line(self: &Self) -> Option<NonZeroU64>`

- `fn column(self: &Self) -> ColumnType` — [`ColumnType`](../index.md)

- `fn is_stmt(self: &Self) -> bool`

- `fn basic_block(self: &Self) -> bool`

- `fn end_sequence(self: &Self) -> bool`

- `fn prologue_end(self: &Self) -> bool`

- `fn epilogue_begin(self: &Self) -> bool`

- `fn isa(self: &Self) -> u64`

- `fn discriminator(self: &Self) -> u64`

- `fn execute<R, Program>(self: &mut Self, instruction: LineInstruction<R>, program: &mut Program) -> Result<bool>` — [`LineInstruction`](../index.md), [`Result`](../../index.md)

- `fn reset<R: Reader>(self: &mut Self, header: &LineProgramHeader<R>)` — [`LineProgramHeader`](../index.md)

- `fn apply_line_advance(self: &mut Self, line_increment: i64)`

- `fn apply_operation_advance<R: Reader>(self: &mut Self, operation_advance: u64, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](../index.md), [`Result`](../../index.md)

- `fn adjust_opcode<R: Reader>(self: &Self, opcode: u8, header: &LineProgramHeader<R>) -> u8` — [`LineProgramHeader`](../index.md)

- `fn exec_special_opcode<R: Reader>(self: &mut Self, opcode: u8, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for LineRow`

- `fn clone(self: &Self) -> LineRow` — [`LineRow`](../index.md)

##### `impl Copy for LineRow`

##### `impl Debug for LineRow`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LineRow`

##### `impl PartialEq for LineRow`

- `fn eq(self: &Self, other: &LineRow) -> bool` — [`LineRow`](../index.md)

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

- `fn clone(self: &Self) -> LineSequence<R>` — [`LineSequence`](../index.md)

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

- `fn offset(self: &Self) -> DebugLineOffset<<R as >::Offset>` — [`DebugLineOffset`](../../index.md), [`Reader`](../index.md)

- `fn unit_length(self: &Self) -> <R as >::Offset` — [`Reader`](../index.md)

- `fn encoding(self: &Self) -> Encoding` — [`Encoding`](../../index.md)

- `fn version(self: &Self) -> u16`

- `fn header_length(self: &Self) -> <R as >::Offset` — [`Reader`](../index.md)

- `fn address_size(self: &Self) -> u8`

- `fn format(self: &Self) -> Format` — [`Format`](../../index.md)

- `fn line_encoding(self: &Self) -> LineEncoding` — [`LineEncoding`](../../index.md)

- `fn minimum_instruction_length(self: &Self) -> u8`

- `fn maximum_operations_per_instruction(self: &Self) -> u8`

- `fn default_is_stmt(self: &Self) -> bool`

- `fn line_base(self: &Self) -> i8`

- `fn line_range(self: &Self) -> u8`

- `fn opcode_base(self: &Self) -> u8`

- `fn standard_opcode_lengths(self: &Self) -> &R`

- `fn directory_entry_format(self: &Self) -> &[FileEntryFormat]` — [`FileEntryFormat`](../index.md)

- `fn include_directories(self: &Self) -> &[AttributeValue<R, Offset>]` — [`AttributeValue`](../index.md)

- `fn directory(self: &Self, directory: u64) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](../index.md)

- `fn file_name_entry_format(self: &Self) -> &[FileEntryFormat]` — [`FileEntryFormat`](../index.md)

- `fn file_has_timestamp(self: &Self) -> bool`

- `fn file_has_size(self: &Self) -> bool`

- `fn file_has_md5(self: &Self) -> bool`

- `fn file_has_source(self: &Self) -> bool`

- `fn file_names(self: &Self) -> &[FileEntry<R, Offset>]` — [`FileEntry`](../index.md)

- `fn file(self: &Self, file: u64) -> Option<&FileEntry<R, Offset>>` — [`FileEntry`](../index.md)

- `fn raw_program_buf(self: &Self) -> R`

- `fn instructions(self: &Self) -> LineInstructions<R>` — [`LineInstructions`](../index.md)

- `fn parse(input: &mut R, offset: DebugLineOffset<Offset>, address_size: u8, comp_dir: Option<R>, comp_name: Option<R>) -> Result<LineProgramHeader<R, Offset>>` — [`DebugLineOffset`](../../index.md), [`Result`](../../index.md), [`LineProgramHeader`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineProgramHeader<R, Offset>`

- `fn clone(self: &Self) -> LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md)

##### `impl<R, Offset> Debug for LineProgramHeader<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for LineProgramHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for LineProgramHeader<R, Offset>`

- `fn eq(self: &Self, other: &LineProgramHeader<R, Offset>) -> bool` — [`LineProgramHeader`](../index.md)

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

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md)

- `fn rows(self: Self) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`LineRows`](../index.md), [`IncompleteLineProgram`](../index.md)

- `fn sequences(self: Self) -> Result<(CompleteLineProgram<R, Offset>, Vec<LineSequence<R>>)>` — [`Result`](../../index.md), [`CompleteLineProgram`](../index.md), [`LineSequence`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for IncompleteLineProgram<R, Offset>`

- `fn clone(self: &Self) -> IncompleteLineProgram<R, Offset>` — [`IncompleteLineProgram`](../index.md)

##### `impl<R, Offset> Debug for IncompleteLineProgram<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for IncompleteLineProgram<R, Offset>`

##### `impl<R, Offset> LineProgram for IncompleteLineProgram<R, Offset>`

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md)

- `fn add_file(self: &mut Self, file: FileEntry<R, Offset>)` — [`FileEntry`](../index.md)

##### `impl<R, Offset> PartialEq for IncompleteLineProgram<R, Offset>`

- `fn eq(self: &Self, other: &IncompleteLineProgram<R, Offset>) -> bool` — [`IncompleteLineProgram`](../index.md)

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

- `fn header(self: &Self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](../index.md)

- `fn resume_from<'program>(self: &'program Self, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`LineSequence`](../index.md), [`LineRows`](../index.md), [`CompleteLineProgram`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for CompleteLineProgram<R, Offset>`

- `fn clone(self: &Self) -> CompleteLineProgram<R, Offset>` — [`CompleteLineProgram`](../index.md)

##### `impl<R, Offset> Debug for CompleteLineProgram<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for CompleteLineProgram<R, Offset>`

##### `impl<R, Offset> PartialEq for CompleteLineProgram<R, Offset>`

- `fn eq(self: &Self, other: &CompleteLineProgram<R, Offset>) -> bool` — [`CompleteLineProgram`](../index.md)

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

- `fn parse(input: &mut R, path_name: R) -> Result<FileEntry<R, Offset>>` — [`Result`](../../index.md), [`FileEntry`](../index.md)

- `fn path_name(self: &Self) -> AttributeValue<R, Offset>` — [`AttributeValue`](../index.md)

- `fn directory_index(self: &Self) -> u64`

- `fn directory(self: &Self, header: &LineProgramHeader<R>) -> Option<AttributeValue<R, Offset>>` — [`LineProgramHeader`](../index.md), [`AttributeValue`](../index.md)

- `fn timestamp(self: &Self) -> u64`

- `fn size(self: &Self) -> u64`

- `fn md5(self: &Self) -> &[u8; 16]`

- `fn source(self: &Self) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for FileEntry<R, Offset>`

- `fn clone(self: &Self) -> FileEntry<R, Offset>` — [`FileEntry`](../index.md)

##### `impl<R, Offset> Copy for FileEntry<R, Offset>`

##### `impl<R, Offset> Debug for FileEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for FileEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FileEntry<R, Offset>`

- `fn eq(self: &Self, other: &FileEntry<R, Offset>) -> bool` — [`FileEntry`](../index.md)

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

- `fn parse<R: Reader>(input: &mut R) -> Result<Vec<FileEntryFormat>>` — [`Result`](../../index.md), [`FileEntryFormat`](../index.md)

#### Trait Implementations

##### `impl Clone for FileEntryFormat`

- `fn clone(self: &Self) -> FileEntryFormat` — [`FileEntryFormat`](../index.md)

##### `impl Copy for FileEntryFormat`

##### `impl Debug for FileEntryFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for FileEntryFormat`

##### `impl PartialEq for FileEntryFormat`

- `fn eq(self: &Self, other: &FileEntryFormat) -> bool` — [`FileEntryFormat`](../index.md)

##### `impl StructuralPartialEq for FileEntryFormat`

## Enums

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

  "[`LineInstruction::Copy`](../../index.md) appends a row to the matrix using the current
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

  "[`LineInstruction::SetPrologueEnd`](../../index.md) sets the prologue_end register to “true”."

- **`SetEpilogueBegin`**

  "[`LineInstruction::SetEpilogueBegin`](../../index.md) sets the epilogue_begin register to
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

  > [`LineInstruction::EndSequence`](../../index.md) sets the end_sequence register of the state
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

- `fn parse<'header>(header: &'header LineProgramHeader<R>, input: &mut R) -> Result<LineInstruction<R>>` — [`LineProgramHeader`](../index.md), [`Result`](../../index.md), [`LineInstruction`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineInstruction<R, Offset>`

- `fn clone(self: &Self) -> LineInstruction<R, Offset>` — [`LineInstruction`](../index.md)

##### `impl<R, Offset> Copy for LineInstruction<R, Offset>`

##### `impl<R, Offset> Debug for LineInstruction<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for LineInstruction<R, Offset>`

##### `impl<R, Offset> PartialEq for LineInstruction<R, Offset>`

- `fn eq(self: &Self, other: &LineInstruction<R, Offset>) -> bool` — [`LineInstruction`](../index.md)

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

- `fn clone(self: &Self) -> ColumnType` — [`ColumnType`](../index.md)

##### `impl Copy for ColumnType`

##### `impl Debug for ColumnType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ColumnType`

##### `impl Ord for ColumnType`

- `fn cmp(self: &Self, other: &ColumnType) -> $crate::cmp::Ordering` — [`ColumnType`](../index.md)

##### `impl PartialEq for ColumnType`

- `fn eq(self: &Self, other: &ColumnType) -> bool` — [`ColumnType`](../index.md)

##### `impl PartialOrd for ColumnType`

- `fn partial_cmp(self: &Self, other: &ColumnType) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ColumnType`](../index.md)

##### `impl StructuralPartialEq for ColumnType`

## Traits

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

## Functions

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

## Type Aliases

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

