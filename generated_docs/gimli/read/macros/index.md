*[gimli](../../index.md) / [read](../index.md) / [macros](index.md)*

---

# Module `macros`

## Structs

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

- `fn clone(self: &Self) -> DebugMacinfo<R>` — [`DebugMacinfo`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugMacinfo<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugMacinfo<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugMacinfo<R>`

- `fn default() -> DebugMacinfo<R>` — [`DebugMacinfo`](../index.md)

##### `impl<R> Section for DebugMacinfo<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

- `fn reader(self: &Self) -> &R`

### `DebugMacro<R>`

```rust
struct DebugMacro<R> {
    section: R,
}
```

The raw contents of the `.debug_macro` section.

#### Implementations

- `fn new(macro_section: &'input [u8], endian: Endian) -> Self`

#### Trait Implementations

##### `impl<R: $crate::clone::Clone> Clone for DebugMacro<R>`

- `fn clone(self: &Self) -> DebugMacro<R>` — [`DebugMacro`](../index.md)

##### `impl<R: $crate::marker::Copy> Copy for DebugMacro<R>`

##### `impl<R: $crate::fmt::Debug> Debug for DebugMacro<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R: $crate::default::Default> Default for DebugMacro<R>`

- `fn default() -> DebugMacro<R>` — [`DebugMacro`](../index.md)

##### `impl<R> Section for DebugMacro<R>`

- `fn id() -> SectionId` — [`SectionId`](../../index.md)

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

- `fn parse(input: &mut R) -> Result<Self>` — [`Result`](../../index.md)

- `fn format(self: &Self) -> Format` — [`Format`](../../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for MacroUnitHeader<R>`

- `fn clone(self: &Self) -> MacroUnitHeader<R>` — [`MacroUnitHeader`](#macrounitheader)

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

- `fn next(self: &mut Self) -> Result<Option<MacroEntry<R>>>` — [`Result`](../../index.md), [`MacroEntry`](../index.md)

#### Trait Implementations

##### `impl<R: $crate::clone::Clone + Reader> Clone for MacroIter<R>`

- `fn clone(self: &Self) -> MacroIter<R>` — [`MacroIter`](../index.md)

##### `impl<R: $crate::fmt::Debug + Reader> Debug for MacroIter<R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

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

- `fn string(self: &Self, unit: UnitRef<'_, R>) -> Result<R>` — [`UnitRef`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for MacroString<R, Offset>`

- `fn clone(self: &Self) -> MacroString<R, Offset>` — [`MacroString`](../index.md)

##### `impl<R, Offset> Debug for MacroString<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for MacroString<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroString<R, Offset>`

- `fn eq(self: &Self, other: &MacroString<R, Offset>) -> bool` — [`MacroString`](../index.md)

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

- `fn clone(self: &Self) -> MacroEntry<R, Offset>` — [`MacroEntry`](../index.md)

##### `impl<R, Offset> Debug for MacroEntry<R, Offset>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Offset> Eq for MacroEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroEntry<R, Offset>`

- `fn eq(self: &Self, other: &MacroEntry<R, Offset>) -> bool` — [`MacroEntry`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for MacroEntry<R, Offset>`

