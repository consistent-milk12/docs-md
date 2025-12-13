*[object](../index.md) / [common](index.md)*

---

# Module `common`

## Contents

- [Enums](#enums)
  - [`Architecture`](#architecture)
  - [`SubArchitecture`](#subarchitecture)
  - [`AddressSize`](#addresssize)
  - [`BinaryFormat`](#binaryformat)
  - [`SectionKind`](#sectionkind)
  - [`ComdatKind`](#comdatkind)
  - [`SymbolKind`](#symbolkind)
  - [`SymbolScope`](#symbolscope)
  - [`RelocationKind`](#relocationkind)
  - [`RelocationEncoding`](#relocationencoding)
  - [`FileFlags`](#fileflags)
  - [`SegmentFlags`](#segmentflags)
  - [`SectionFlags`](#sectionflags)
  - [`SymbolFlags`](#symbolflags)
  - [`RelocationFlags`](#relocationflags)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Architecture`](#architecture) | enum | A CPU architecture. |
| [`SubArchitecture`](#subarchitecture) | enum | A CPU sub-architecture. |
| [`AddressSize`](#addresssize) | enum | The size of an address value for an architecture. |
| [`BinaryFormat`](#binaryformat) | enum | A binary file format. |
| [`SectionKind`](#sectionkind) | enum | The kind of a section. |
| [`ComdatKind`](#comdatkind) | enum | The selection kind for a COMDAT section group. |
| [`SymbolKind`](#symbolkind) | enum | The kind of a symbol. |
| [`SymbolScope`](#symbolscope) | enum | A symbol scope. |
| [`RelocationKind`](#relocationkind) | enum | The operation used to calculate the result of the relocation. |
| [`RelocationEncoding`](#relocationencoding) | enum | Information about how the result of the relocation operation is encoded in the place. |
| [`FileFlags`](#fileflags) | enum | File flags that are specific to each file format. |
| [`SegmentFlags`](#segmentflags) | enum | Segment flags that are specific to each file format. |
| [`SectionFlags`](#sectionflags) | enum | Section flags that are specific to each file format. |
| [`SymbolFlags`](#symbolflags) | enum | Symbol flags that are specific to each file format. |
| [`RelocationFlags`](#relocationflags) | enum | Relocation fields that are specific to each file format and architecture. |

## Enums

### `Architecture`

```rust
enum Architecture {
    Unknown,
    Aarch64,
    Aarch64_Ilp32,
    Alpha,
    Arm,
    Avr,
    Bpf,
    Csky,
    E2K32,
    E2K64,
    I386,
    X86_64,
    X86_64_X32,
    Hexagon,
    Hppa,
    LoongArch32,
    LoongArch64,
    M68k,
    Mips,
    Mips64,
    Mips64_N32,
    Msp430,
    PowerPc,
    PowerPc64,
    Riscv32,
    Riscv64,
    S390x,
    Sbf,
    Sharc,
    Sparc,
    Sparc32Plus,
    Sparc64,
    SuperH,
    Wasm32,
    Wasm64,
    Xtensa,
}
```

*Defined in [`object-0.37.3/src/common.rs:5-45`](../../../.source_1765633015/object-0.37.3/src/common.rs#L5-L45)*

A CPU architecture.

#### Implementations

- <span id="architecture-address-size"></span>`fn address_size(self) -> Option<AddressSize>` — [`AddressSize`](../index.md#addresssize)

  The size of an address value for this architecture.

  

  Returns `None` for unknown architectures.

#### Trait Implementations

##### `impl Any for Architecture`

- <span id="architecture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Architecture`

- <span id="architecture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Architecture`

- <span id="architecture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Architecture`

- <span id="architecture-clone"></span>`fn clone(&self) -> Architecture` — [`Architecture`](../index.md#architecture)

##### `impl CloneToUninit for Architecture`

- <span id="architecture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- <span id="architecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Architecture`

##### `impl<T> From for Architecture`

- <span id="architecture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Architecture`

- <span id="architecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Architecture`

- <span id="architecture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Architecture`

- <span id="architecture-partialeq-eq"></span>`fn eq(&self, other: &Architecture) -> bool` — [`Architecture`](../index.md#architecture)

##### `impl StructuralPartialEq for Architecture`

##### `impl ToOwned for Architecture`

- <span id="architecture-toowned-type-owned"></span>`type Owned = T`

- <span id="architecture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="architecture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Architecture`

- <span id="architecture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="architecture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Architecture`

- <span id="architecture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="architecture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubArchitecture`

```rust
enum SubArchitecture {
    Arm64E,
    Arm64EC,
}
```

*Defined in [`object-0.37.3/src/common.rs:51-54`](../../../.source_1765633015/object-0.37.3/src/common.rs#L51-L54)*

A CPU sub-architecture.

#### Trait Implementations

##### `impl Any for SubArchitecture`

- <span id="subarchitecture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubArchitecture`

- <span id="subarchitecture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubArchitecture`

- <span id="subarchitecture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SubArchitecture`

- <span id="subarchitecture-clone"></span>`fn clone(&self) -> SubArchitecture` — [`SubArchitecture`](../index.md#subarchitecture)

##### `impl CloneToUninit for SubArchitecture`

- <span id="subarchitecture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- <span id="subarchitecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl<T> From for SubArchitecture`

- <span id="subarchitecture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SubArchitecture`

- <span id="subarchitecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SubArchitecture`

- <span id="subarchitecture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SubArchitecture`

- <span id="subarchitecture-partialeq-eq"></span>`fn eq(&self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](../index.md#subarchitecture)

##### `impl StructuralPartialEq for SubArchitecture`

##### `impl ToOwned for SubArchitecture`

- <span id="subarchitecture-toowned-type-owned"></span>`type Owned = T`

- <span id="subarchitecture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="subarchitecture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubArchitecture`

- <span id="subarchitecture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="subarchitecture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubArchitecture`

- <span id="subarchitecture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="subarchitecture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AddressSize`

```rust
enum AddressSize {
    U8,
    U16,
    U32,
    U64,
}
```

*Defined in [`object-0.37.3/src/common.rs:109-114`](../../../.source_1765633015/object-0.37.3/src/common.rs#L109-L114)*

The size of an address value for an architecture.

This may differ from the address size supported by the file format (such as for COFF).

#### Implementations

- <span id="addresssize-bytes"></span>`fn bytes(self) -> u8`

  The size in bytes of an address value.

#### Trait Implementations

##### `impl Any for AddressSize`

- <span id="addresssize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AddressSize`

- <span id="addresssize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AddressSize`

- <span id="addresssize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AddressSize`

- <span id="addresssize-clone"></span>`fn clone(&self) -> AddressSize` — [`AddressSize`](../index.md#addresssize)

##### `impl CloneToUninit for AddressSize`

- <span id="addresssize-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- <span id="addresssize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddressSize`

##### `impl<T> From for AddressSize`

- <span id="addresssize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AddressSize`

- <span id="addresssize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for AddressSize`

- <span id="addresssize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AddressSize`

- <span id="addresssize-partialeq-eq"></span>`fn eq(&self, other: &AddressSize) -> bool` — [`AddressSize`](../index.md#addresssize)

##### `impl StructuralPartialEq for AddressSize`

##### `impl ToOwned for AddressSize`

- <span id="addresssize-toowned-type-owned"></span>`type Owned = T`

- <span id="addresssize-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="addresssize-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AddressSize`

- <span id="addresssize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="addresssize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AddressSize`

- <span id="addresssize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="addresssize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BinaryFormat`

```rust
enum BinaryFormat {
    Coff,
    Elf,
    MachO,
    Pe,
    Wasm,
    Xcoff,
}
```

*Defined in [`object-0.37.3/src/common.rs:128-135`](../../../.source_1765633015/object-0.37.3/src/common.rs#L128-L135)*

A binary file format.

#### Implementations

- <span id="binaryformat-native-object"></span>`fn native_object() -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

  The target's native binary format for relocatable object files.

  

  Defaults to `Elf` for unknown platforms.

#### Trait Implementations

##### `impl Any for BinaryFormat`

- <span id="binaryformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BinaryFormat`

- <span id="binaryformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BinaryFormat`

- <span id="binaryformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BinaryFormat`

- <span id="binaryformat-clone"></span>`fn clone(&self) -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

##### `impl CloneToUninit for BinaryFormat`

- <span id="binaryformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- <span id="binaryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl<T> From for BinaryFormat`

- <span id="binaryformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BinaryFormat`

- <span id="binaryformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for BinaryFormat`

- <span id="binaryformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BinaryFormat`

- <span id="binaryformat-partialeq-eq"></span>`fn eq(&self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](../index.md#binaryformat)

##### `impl StructuralPartialEq for BinaryFormat`

##### `impl ToOwned for BinaryFormat`

- <span id="binaryformat-toowned-type-owned"></span>`type Owned = T`

- <span id="binaryformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="binaryformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BinaryFormat`

- <span id="binaryformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="binaryformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BinaryFormat`

- <span id="binaryformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="binaryformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionKind`

```rust
enum SectionKind {
    Unknown,
    Text,
    Data,
    ReadOnlyData,
    ReadOnlyDataWithRel,
    ReadOnlyString,
    UninitializedData,
    Common,
    Tls,
    UninitializedTls,
    TlsVariables,
    OtherString,
    Other,
    Debug,
    DebugString,
    Linker,
    Note,
    Metadata,
    Elf(u32),
}
```

*Defined in [`object-0.37.3/src/common.rs:155-247`](../../../.source_1765633015/object-0.37.3/src/common.rs#L155-L247)*

The kind of a section.

#### Variants

- **`Unknown`**

  The section kind is unknown.

- **`Text`**

  An executable code section.
  
  Example ELF sections: `.text`
  
  Example Mach-O sections: `__TEXT/__text`

- **`Data`**

  A data section.
  
  Example ELF sections: `.data`
  
  Example Mach-O sections: `__DATA/__data`

- **`ReadOnlyData`**

  A read only data section.
  
  Example ELF sections: `.rodata`
  
  Example Mach-O sections: `__TEXT/__const`, `__DATA/__const`, `__TEXT/__literal4`

- **`ReadOnlyDataWithRel`**

  A read only data section with relocations.
  
  This is the same as either `Data` or `ReadOnlyData`, depending on the file format.
  This value is only used in the API for writing files. It is never returned when reading files.

- **`ReadOnlyString`**

  A loadable string section.
  
  Example ELF sections: `.rodata.str`
  
  Example Mach-O sections: `__TEXT/__cstring`

- **`UninitializedData`**

  An uninitialized data section.
  
  Example ELF sections: `.bss`
  
  Example Mach-O sections: `__DATA/__bss`

- **`Common`**

  An uninitialized common data section.
  
  Example Mach-O sections: `__DATA/__common`

- **`Tls`**

  A TLS data section.
  
  Example ELF sections: `.tdata`
  
  Example Mach-O sections: `__DATA/__thread_data`

- **`UninitializedTls`**

  An uninitialized TLS data section.
  
  Example ELF sections: `.tbss`
  
  Example Mach-O sections: `__DATA/__thread_bss`

- **`TlsVariables`**

  A TLS variables section.
  
  This contains TLS variable structures, rather than the variable initializers.
  
  Example Mach-O sections: `__DATA/__thread_vars`

- **`OtherString`**

  A non-loadable string section.
  
  Example ELF sections: `.comment`, `.debug_str`

- **`Other`**

  Some other non-loadable section.
  
  Example ELF sections: `.debug_info`

- **`Debug`**

  Debug information.
  
  Example Mach-O sections: `__DWARF/__debug_info`

- **`DebugString`**

  Debug strings.
  
  This is the same as either `Debug` or `OtherString`, depending on the file format.
  This value is only used in the API for writing files. It is never returned when reading files.

- **`Linker`**

  Information for the linker.
  
  Example COFF sections: `.drectve`

- **`Note`**

  ELF note section.

- **`Metadata`**

  Metadata such as symbols or relocations.
  
  Example ELF sections: `.symtab`, `.strtab`, `.group`

- **`Elf`**

  Some other ELF section type.
  
  This is the `sh_type` field in the section header.
  The meaning may be dependent on the architecture.

#### Implementations

- <span id="sectionkind-is-bss"></span>`fn is_bss(self) -> bool`

  Return true if this section contains zerofill data.

#### Trait Implementations

##### `impl Any for SectionKind`

- <span id="sectionkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionKind`

- <span id="sectionkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionKind`

- <span id="sectionkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionKind`

- <span id="sectionkind-clone"></span>`fn clone(&self) -> SectionKind` — [`SectionKind`](../index.md#sectionkind)

##### `impl CloneToUninit for SectionKind`

- <span id="sectionkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- <span id="sectionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionKind`

##### `impl<T> From for SectionKind`

- <span id="sectionkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SectionKind`

- <span id="sectionkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SectionKind`

- <span id="sectionkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SectionKind`

- <span id="sectionkind-partialeq-eq"></span>`fn eq(&self, other: &SectionKind) -> bool` — [`SectionKind`](../index.md#sectionkind)

##### `impl StructuralPartialEq for SectionKind`

##### `impl ToOwned for SectionKind`

- <span id="sectionkind-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionKind`

- <span id="sectionkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionKind`

- <span id="sectionkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComdatKind`

```rust
enum ComdatKind {
    Unknown,
    Any,
    NoDuplicates,
    SameSize,
    ExactMatch,
    Largest,
    Newest,
}
```

*Defined in [`object-0.37.3/src/common.rs:264-291`](../../../.source_1765633015/object-0.37.3/src/common.rs#L264-L291)*

The selection kind for a COMDAT section group.

This determines the way in which the linker resolves multiple definitions of the COMDAT
sections.

#### Variants

- **`Unknown`**

  The selection kind is unknown.

- **`Any`**

  Multiple definitions are allowed.
  
  An arbitrary definition is selected, and the rest are removed.
  
  This is the only supported selection kind for ELF.

- **`NoDuplicates`**

  Multiple definitions are not allowed.
  
  This is used to group sections without allowing duplicates.

- **`SameSize`**

  Multiple definitions must have the same size.
  
  An arbitrary definition is selected, and the rest are removed.

- **`ExactMatch`**

  Multiple definitions must match exactly.
  
  An arbitrary definition is selected, and the rest are removed.

- **`Largest`**

  Multiple definitions are allowed, and the largest is selected.
  
  An arbitrary definition with the largest size is selected, and the rest are removed.

- **`Newest`**

  Multiple definitions are allowed, and the newest is selected.

#### Trait Implementations

##### `impl Any for ComdatKind`

- <span id="comdatkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ComdatKind`

- <span id="comdatkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComdatKind`

- <span id="comdatkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ComdatKind`

- <span id="comdatkind-clone"></span>`fn clone(&self) -> ComdatKind` — [`ComdatKind`](../index.md#comdatkind)

##### `impl CloneToUninit for ComdatKind`

- <span id="comdatkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- <span id="comdatkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl<T> From for ComdatKind`

- <span id="comdatkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ComdatKind`

- <span id="comdatkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ComdatKind`

- <span id="comdatkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ComdatKind`

- <span id="comdatkind-partialeq-eq"></span>`fn eq(&self, other: &ComdatKind) -> bool` — [`ComdatKind`](../index.md#comdatkind)

##### `impl StructuralPartialEq for ComdatKind`

##### `impl ToOwned for ComdatKind`

- <span id="comdatkind-toowned-type-owned"></span>`type Owned = T`

- <span id="comdatkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="comdatkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ComdatKind`

- <span id="comdatkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comdatkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ComdatKind`

- <span id="comdatkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comdatkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolKind`

```rust
enum SymbolKind {
    Unknown,
    Text,
    Data,
    Section,
    File,
    Label,
    Tls,
}
```

*Defined in [`object-0.37.3/src/common.rs:296-311`](../../../.source_1765633015/object-0.37.3/src/common.rs#L296-L311)*

The kind of a symbol.

#### Variants

- **`Unknown`**

  The symbol kind is unknown.

- **`Text`**

  The symbol is for executable code.

- **`Data`**

  The symbol is for a data object.

- **`Section`**

  The symbol is for a section.

- **`File`**

  The symbol is the name of a file. It precedes symbols within that file.

- **`Label`**

  The symbol is for a code label.

- **`Tls`**

  The symbol is for a thread local storage entity.

#### Trait Implementations

##### `impl Any for SymbolKind`

- <span id="symbolkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolKind`

- <span id="symbolkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolKind`

- <span id="symbolkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolKind`

- <span id="symbolkind-clone"></span>`fn clone(&self) -> SymbolKind` — [`SymbolKind`](../index.md#symbolkind)

##### `impl CloneToUninit for SymbolKind`

- <span id="symbolkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- <span id="symbolkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl<T> From for SymbolKind`

- <span id="symbolkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolKind`

- <span id="symbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolKind`

- <span id="symbolkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolKind`

- <span id="symbolkind-partialeq-eq"></span>`fn eq(&self, other: &SymbolKind) -> bool` — [`SymbolKind`](../index.md#symbolkind)

##### `impl StructuralPartialEq for SymbolKind`

##### `impl ToOwned for SymbolKind`

- <span id="symbolkind-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolKind`

- <span id="symbolkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolKind`

- <span id="symbolkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolScope`

```rust
enum SymbolScope {
    Unknown,
    Compilation,
    Linkage,
    Dynamic,
}
```

*Defined in [`object-0.37.3/src/common.rs:315-324`](../../../.source_1765633015/object-0.37.3/src/common.rs#L315-L324)*

A symbol scope.

#### Variants

- **`Unknown`**

  Unknown scope.

- **`Compilation`**

  Symbol is visible to the compilation unit.

- **`Linkage`**

  Symbol is visible to the static linkage unit.

- **`Dynamic`**

  Symbol is visible to dynamically linked objects.

#### Trait Implementations

##### `impl Any for SymbolScope`

- <span id="symbolscope-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolScope`

- <span id="symbolscope-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolScope`

- <span id="symbolscope-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SymbolScope`

- <span id="symbolscope-clone"></span>`fn clone(&self) -> SymbolScope` — [`SymbolScope`](../index.md#symbolscope)

##### `impl CloneToUninit for SymbolScope`

- <span id="symbolscope-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- <span id="symbolscope-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl<T> From for SymbolScope`

- <span id="symbolscope-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SymbolScope`

- <span id="symbolscope-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolScope`

- <span id="symbolscope-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SymbolScope`

- <span id="symbolscope-partialeq-eq"></span>`fn eq(&self, other: &SymbolScope) -> bool` — [`SymbolScope`](../index.md#symbolscope)

##### `impl StructuralPartialEq for SymbolScope`

##### `impl ToOwned for SymbolScope`

- <span id="symbolscope-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolscope-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolscope-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolScope`

- <span id="symbolscope-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolscope-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolScope`

- <span id="symbolscope-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolscope-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationKind`

```rust
enum RelocationKind {
    Unknown,
    Absolute,
    Relative,
    Got,
    GotRelative,
    GotBaseRelative,
    GotBaseOffset,
    PltRelative,
    ImageOffset,
    SectionOffset,
    SectionIndex,
}
```

*Defined in [`object-0.37.3/src/common.rs:343-366`](../../../.source_1765633015/object-0.37.3/src/common.rs#L343-L366)*

The operation used to calculate the result of the relocation.

The relocation descriptions use the following definitions. Note that
these definitions probably don't match any ELF ABI.

* A - The value of the addend.
* G - The address of the symbol's entry within the global offset table.
* L - The address of the symbol's entry within the procedure linkage table.
* P - The address of the place of the relocation.
* S - The address of the symbol.
* GotBase - The address of the global offset table.
* Image - The base address of the image.
* Section - The address of the section containing the symbol.

'XxxRelative' means 'Xxx + A - P'.  'XxxOffset' means 'S + A - Xxx'.

#### Variants

- **`Unknown`**

  The operation is unknown.

- **`Absolute`**

  S + A

- **`Relative`**

  S + A - P

- **`Got`**

  G + A - GotBase

- **`GotRelative`**

  G + A - P

- **`GotBaseRelative`**

  GotBase + A - P

- **`GotBaseOffset`**

  S + A - GotBase

- **`PltRelative`**

  L + A - P

- **`ImageOffset`**

  S + A - Image

- **`SectionOffset`**

  S + A - Section

- **`SectionIndex`**

  The index of the section containing the symbol.

#### Trait Implementations

##### `impl Any for RelocationKind`

- <span id="relocationkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationKind`

- <span id="relocationkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationKind`

- <span id="relocationkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationKind`

- <span id="relocationkind-clone"></span>`fn clone(&self) -> RelocationKind` — [`RelocationKind`](../index.md#relocationkind)

##### `impl CloneToUninit for RelocationKind`

- <span id="relocationkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- <span id="relocationkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl<T> From for RelocationKind`

- <span id="relocationkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationKind`

- <span id="relocationkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationKind`

- <span id="relocationkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationKind`

- <span id="relocationkind-partialeq-eq"></span>`fn eq(&self, other: &RelocationKind) -> bool` — [`RelocationKind`](../index.md#relocationkind)

##### `impl StructuralPartialEq for RelocationKind`

##### `impl ToOwned for RelocationKind`

- <span id="relocationkind-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationKind`

- <span id="relocationkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationKind`

- <span id="relocationkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationEncoding`

```rust
enum RelocationEncoding {
    Unknown,
    Generic,
    X86Signed,
    X86RipRelative,
    X86RipRelativeMovq,
    X86Branch,
    S390xDbl,
    AArch64Call,
    LoongArchBranch,
    SharcTypeA,
    SharcTypeB,
    E2KLit,
    E2KDisp,
}
```

*Defined in [`object-0.37.3/src/common.rs:374-447`](../../../.source_1765633015/object-0.37.3/src/common.rs#L374-L447)*

Information about how the result of the relocation operation is encoded in the place.

This is usually architecture specific, such as specifying an addressing mode or
a specific instruction.

#### Variants

- **`Unknown`**

  The relocation encoding is unknown.

- **`Generic`**

  Generic encoding.

- **`X86Signed`**

  x86 sign extension at runtime.
  
  Used with `RelocationKind::Absolute`.

- **`X86RipRelative`**

  x86 rip-relative addressing.
  
  The `RelocationKind` must be PC relative.

- **`X86RipRelativeMovq`**

  x86 rip-relative addressing in movq instruction.
  
  The `RelocationKind` must be PC relative.

- **`X86Branch`**

  x86 branch instruction.
  
  The `RelocationKind` must be PC relative.

- **`S390xDbl`**

  s390x PC-relative offset shifted right by one bit.
  
  The `RelocationKind` must be PC relative.

- **`AArch64Call`**

  AArch64 call target.
  
  The `RelocationKind` must be PC relative.

- **`LoongArchBranch`**

  LoongArch branch offset with two trailing zeros.
  
  The `RelocationKind` must be PC relative.

- **`SharcTypeA`**

  SHARC+ 48-bit Type A instruction
  
  Represents these possible variants, each with a corresponding
  `R_SHARC_*` constant:
  
  * 24-bit absolute address
  * 32-bit absolute address
  * 6-bit relative address
  * 24-bit relative address
  * 6-bit absolute address in the immediate value field
  * 16-bit absolute address in the immediate value field

- **`SharcTypeB`**

  SHARC+ 32-bit Type B instruction
  
  Represents these possible variants, each with a corresponding
  `R_SHARC_*` constant:
  
  * 6-bit absolute address in the immediate value field
  * 7-bit absolute address in the immediate value field
  * 16-bit absolute address
  * 6-bit relative address

- **`E2KLit`**

  E2K 64-bit value stored in two LTS
  
  Memory representation:
  ```text
  0: LTS1 = value[63:32]
  4: LTS0 = value[31:0]
  ```

- **`E2KDisp`**

  E2K 28-bit value stored in CS0

#### Trait Implementations

##### `impl Any for RelocationEncoding`

- <span id="relocationencoding-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationEncoding`

- <span id="relocationencoding-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationEncoding`

- <span id="relocationencoding-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationEncoding`

- <span id="relocationencoding-clone"></span>`fn clone(&self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md#relocationencoding)

##### `impl CloneToUninit for RelocationEncoding`

- <span id="relocationencoding-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- <span id="relocationencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl<T> From for RelocationEncoding`

- <span id="relocationencoding-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationEncoding`

- <span id="relocationencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationEncoding`

- <span id="relocationencoding-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationEncoding`

- <span id="relocationencoding-partialeq-eq"></span>`fn eq(&self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](../index.md#relocationencoding)

##### `impl StructuralPartialEq for RelocationEncoding`

##### `impl ToOwned for RelocationEncoding`

- <span id="relocationencoding-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationencoding-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationencoding-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationEncoding`

- <span id="relocationencoding-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationencoding-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationEncoding`

- <span id="relocationencoding-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationencoding-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FileFlags`

```rust
enum FileFlags {
    None,
    Elf {
        os_abi: u8,
        abi_version: u8,
        e_flags: u32,
    },
    MachO {
        flags: u32,
    },
    Coff {
        characteristics: u16,
    },
    Xcoff {
        f_flags: u16,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:452-479`](../../../.source_1765633015/object-0.37.3/src/common.rs#L452-L479)*

File flags that are specific to each file format.

#### Variants

- **`None`**

  No file flags.

- **`Elf`**

  ELF file flags.

- **`MachO`**

  Mach-O file flags.

- **`Coff`**

  COFF file flags.

- **`Xcoff`**

  XCOFF file flags.

#### Trait Implementations

##### `impl Any for FileFlags`

- <span id="fileflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FileFlags`

- <span id="fileflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FileFlags`

- <span id="fileflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FileFlags`

- <span id="fileflags-clone"></span>`fn clone(&self) -> FileFlags` — [`FileFlags`](../index.md#fileflags)

##### `impl CloneToUninit for FileFlags`

- <span id="fileflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- <span id="fileflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileFlags`

##### `impl<T> From for FileFlags`

- <span id="fileflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FileFlags`

- <span id="fileflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for FileFlags`

- <span id="fileflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FileFlags`

- <span id="fileflags-partialeq-eq"></span>`fn eq(&self, other: &FileFlags) -> bool` — [`FileFlags`](../index.md#fileflags)

##### `impl StructuralPartialEq for FileFlags`

##### `impl ToOwned for FileFlags`

- <span id="fileflags-toowned-type-owned"></span>`type Owned = T`

- <span id="fileflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fileflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FileFlags`

- <span id="fileflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fileflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FileFlags`

- <span id="fileflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fileflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SegmentFlags`

```rust
enum SegmentFlags {
    None,
    Elf {
        p_flags: u32,
    },
    MachO {
        flags: u32,
        maxprot: u32,
        initprot: u32,
    },
    Coff {
        characteristics: u32,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:484-506`](../../../.source_1765633015/object-0.37.3/src/common.rs#L484-L506)*

Segment flags that are specific to each file format.

#### Variants

- **`None`**

  No segment flags.

- **`Elf`**

  ELF segment flags.

- **`MachO`**

  Mach-O segment flags.

- **`Coff`**

  COFF segment flags.

#### Trait Implementations

##### `impl Any for SegmentFlags`

- <span id="segmentflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SegmentFlags`

- <span id="segmentflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SegmentFlags`

- <span id="segmentflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SegmentFlags`

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl CloneToUninit for SegmentFlags`

- <span id="segmentflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl<T> From for SegmentFlags`

- <span id="segmentflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SegmentFlags`

- <span id="segmentflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-partialeq-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl StructuralPartialEq for SegmentFlags`

##### `impl ToOwned for SegmentFlags`

- <span id="segmentflags-toowned-type-owned"></span>`type Owned = T`

- <span id="segmentflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="segmentflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SegmentFlags`

- <span id="segmentflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="segmentflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SegmentFlags`

- <span id="segmentflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="segmentflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SectionFlags`

```rust
enum SectionFlags {
    None,
    Elf {
        sh_flags: u64,
    },
    MachO {
        flags: u32,
    },
    Coff {
        characteristics: u32,
    },
    Xcoff {
        s_flags: u32,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:511-534`](../../../.source_1765633015/object-0.37.3/src/common.rs#L511-L534)*

Section flags that are specific to each file format.

#### Variants

- **`None`**

  No section flags.

- **`Elf`**

  ELF section flags.

- **`MachO`**

  Mach-O section flags.

- **`Coff`**

  COFF section flags.

- **`Xcoff`**

  XCOFF section flags.

#### Trait Implementations

##### `impl Any for SectionFlags`

- <span id="sectionflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SectionFlags`

- <span id="sectionflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SectionFlags`

- <span id="sectionflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SectionFlags`

- <span id="sectionflags-clone"></span>`fn clone(&self) -> SectionFlags` — [`SectionFlags`](../index.md#sectionflags)

##### `impl CloneToUninit for SectionFlags`

- <span id="sectionflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- <span id="sectionflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl<T> From for SectionFlags`

- <span id="sectionflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SectionFlags`

- <span id="sectionflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SectionFlags`

- <span id="sectionflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SectionFlags`

- <span id="sectionflags-partialeq-eq"></span>`fn eq(&self, other: &SectionFlags) -> bool` — [`SectionFlags`](../index.md#sectionflags)

##### `impl StructuralPartialEq for SectionFlags`

##### `impl ToOwned for SectionFlags`

- <span id="sectionflags-toowned-type-owned"></span>`type Owned = T`

- <span id="sectionflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sectionflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SectionFlags`

- <span id="sectionflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sectionflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SectionFlags`

- <span id="sectionflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sectionflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymbolFlags<Section, Symbol>`

```rust
enum SymbolFlags<Section, Symbol> {
    None,
    Elf {
        st_info: u8,
        st_other: u8,
    },
    MachO {
        n_desc: u16,
    },
    CoffSection {
        selection: u8,
        associative_section: Option<Section>,
    },
    Xcoff {
        n_sclass: u8,
        x_smtyp: u8,
        x_smclas: u8,
        containing_csect: Option<Symbol>,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:539-578`](../../../.source_1765633015/object-0.37.3/src/common.rs#L539-L578)*

Symbol flags that are specific to each file format.

#### Variants

- **`None`**

  No symbol flags.

- **`Elf`**

  ELF symbol flags.

- **`MachO`**

  Mach-O symbol flags.

- **`CoffSection`**

  COFF flags for a section symbol.

- **`Xcoff`**

  XCOFF symbol flags.

#### Trait Implementations

##### `impl Any for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Section: clone::Clone, Symbol: clone::Clone> Clone for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl CloneToUninit for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Section: marker::Copy, Symbol: marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: fmt::Debug, Symbol: fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section: cmp::Eq, Symbol: cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<T> From for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<Section: hash::Hash, Symbol: hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Section: cmp::PartialEq, Symbol: cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-partialeq-eq"></span>`fn eq(&self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl<Section, Symbol> StructuralPartialEq for SymbolFlags<Section, Symbol>`

##### `impl ToOwned for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-toowned-type-owned"></span>`type Owned = T`

- <span id="symbolflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symbolflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symbolflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symbolflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RelocationFlags`

```rust
enum RelocationFlags {
    Generic {
        kind: RelocationKind,
        encoding: RelocationEncoding,
        size: u8,
    },
    Elf {
        r_type: u32,
    },
    MachO {
        r_type: u8,
        r_pcrel: bool,
        r_length: u8,
    },
    Coff {
        typ: u16,
    },
    Xcoff {
        r_rtype: u8,
        r_rsize: u8,
    },
}
```

*Defined in [`object-0.37.3/src/common.rs:583-619`](../../../.source_1765633015/object-0.37.3/src/common.rs#L583-L619)*

Relocation fields that are specific to each file format and architecture.

#### Variants

- **`Generic`**

  Format independent representation.

- **`Elf`**

  ELF relocation fields.

- **`MachO`**

  Mach-O relocation fields.

- **`Coff`**

  COFF relocation fields.

- **`Xcoff`**

  XCOFF relocation fields.

#### Trait Implementations

##### `impl Any for RelocationFlags`

- <span id="relocationflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RelocationFlags`

- <span id="relocationflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RelocationFlags`

- <span id="relocationflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RelocationFlags`

- <span id="relocationflags-clone"></span>`fn clone(&self) -> RelocationFlags` — [`RelocationFlags`](../index.md#relocationflags)

##### `impl CloneToUninit for RelocationFlags`

- <span id="relocationflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- <span id="relocationflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl<T> From for RelocationFlags`

- <span id="relocationflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RelocationFlags`

- <span id="relocationflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for RelocationFlags`

- <span id="relocationflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RelocationFlags`

- <span id="relocationflags-partialeq-eq"></span>`fn eq(&self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](../index.md#relocationflags)

##### `impl StructuralPartialEq for RelocationFlags`

##### `impl ToOwned for RelocationFlags`

- <span id="relocationflags-toowned-type-owned"></span>`type Owned = T`

- <span id="relocationflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="relocationflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RelocationFlags`

- <span id="relocationflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="relocationflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RelocationFlags`

- <span id="relocationflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="relocationflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

