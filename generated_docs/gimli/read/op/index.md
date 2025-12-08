*[gimli](../../index.md) / [read](../index.md) / [op](index.md)*

---

# Module `op`

Functions for parsing and evaluating DWARF expressions.

## Contents

- [Structs](#structs)
  - [`Piece`](#piece)
  - [`Expression`](#expression)
  - [`OperationIter`](#operationiter)
  - [`Evaluation`](#evaluation)
- [Enums](#enums)
  - [`DieReference`](#diereference)
  - [`Operation`](#operation)
  - [`OperationEvaluationResult`](#operationevaluationresult)
  - [`Location`](#location)
  - [`EvaluationState`](#evaluationstate)
  - [`EvaluationWaiting`](#evaluationwaiting)
  - [`EvaluationResult`](#evaluationresult)
- [Traits](#traits)
  - [`EvaluationStorage`](#evaluationstorage)
- [Functions](#functions)
  - [`compute_pc`](#compute_pc)
  - [`generic_type`](#generic_type)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Piece`](#piece) | struct | The description of a single piece of the result of a DWARF |
| [`Expression`](#expression) | struct | The bytecode for a DWARF expression or location description. |
| [`OperationIter`](#operationiter) | struct | An iterator for the operations in an expression. |
| [`Evaluation`](#evaluation) | struct | A DWARF expression evaluator. |
| [`DieReference`](#diereference) | enum | A reference to a DIE, either relative to the current CU or |
| [`Operation`](#operation) | enum | A single decoded DWARF expression operation. |
| [`OperationEvaluationResult`](#operationevaluationresult) | enum |  |
| [`Location`](#location) | enum | A single location of a piece of the result of a DWARF expression. |
| [`EvaluationState`](#evaluationstate) | enum |  |
| [`EvaluationWaiting`](#evaluationwaiting) | enum |  |
| [`EvaluationResult`](#evaluationresult) | enum | The state of an `Evaluation` after evaluating a DWARF expression. |
| [`EvaluationStorage`](#evaluationstorage) | trait | Specification of what storage should be used for [`Evaluation`]. |
| [`compute_pc`](#compute_pc) | fn |  |
| [`generic_type`](#generic_type) | fn |  |

## Structs

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

- <span id="piece-clone"></span>`fn clone(&self) -> Piece<R, Offset>` — [`Piece`](../index.md)

##### `impl<R, Offset> Copy for Piece<R, Offset>`

##### `impl<R, Offset> Debug for Piece<R, Offset>`

- <span id="piece-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> PartialEq for Piece<R, Offset>`

- <span id="piece-eq"></span>`fn eq(&self, other: &Piece<R, Offset>) -> bool` — [`Piece`](../index.md)

##### `impl<R, Offset> StructuralPartialEq for Piece<R, Offset>`

### `Expression<R: Reader>`

```rust
struct Expression<R: Reader>(R);
```

The bytecode for a DWARF expression or location description.

#### Implementations

- <span id="expression-evaluation"></span>`fn evaluation(self, encoding: Encoding) -> Evaluation<R>` — [`Encoding`](../../index.md), [`Evaluation`](../index.md)

- <span id="expression-operations"></span>`fn operations(self, encoding: Encoding) -> OperationIter<R>` — [`Encoding`](../../index.md), [`OperationIter`](../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for Expression<R>`

- <span id="expression-clone"></span>`fn clone(&self) -> Expression<R>` — [`Expression`](../index.md)

##### `impl<R: marker::Copy + Reader> Copy for Expression<R>`

##### `impl<R: fmt::Debug + Reader> Debug for Expression<R>`

- <span id="expression-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for Expression<R>`

##### `impl<R: hash::Hash + Reader> Hash for Expression<R>`

- <span id="expression-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for Expression<R>`

- <span id="expression-eq"></span>`fn eq(&self, other: &Expression<R>) -> bool` — [`Expression`](../index.md)

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

- <span id="operationiter-next"></span>`fn next(&mut self) -> Result<Option<Operation<R>>>` — [`Result`](../../index.md), [`Operation`](../index.md)

- <span id="operationiter-offset-from"></span>`fn offset_from(&self, expression: &Expression<R>) -> <R as >::Offset` — [`Expression`](../index.md), [`Reader`](../index.md)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for OperationIter<R>`

- <span id="operationiter-clone"></span>`fn clone(&self) -> OperationIter<R>` — [`OperationIter`](../index.md)

##### `impl<R: marker::Copy + Reader> Copy for OperationIter<R>`

##### `impl<R: fmt::Debug + Reader> Debug for OperationIter<R>`

- <span id="operationiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="evaluation-new"></span>`fn new(bytecode: R, encoding: Encoding) -> Self` — [`Encoding`](../../index.md)

- <span id="evaluation-result"></span>`fn result(self) -> Vec<Piece<R>>` — [`Piece`](../index.md)

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader, S: fmt::Debug + EvaluationStorage<R>> Debug for Evaluation<R, S>`

- <span id="evaluation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

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

##### `impl<T: clone::Clone> Clone for DieReference<T>`

- <span id="diereference-clone"></span>`fn clone(&self) -> DieReference<T>` — [`DieReference`](../index.md)

##### `impl<T: marker::Copy> Copy for DieReference<T>`

##### `impl<T: fmt::Debug> Debug for DieReference<T>`

- <span id="diereference-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DieReference<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DieReference<T>`

- <span id="diereference-eq"></span>`fn eq(&self, other: &DieReference<T>) -> bool` — [`DieReference`](../index.md)

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

- <span id="operation-parse"></span>`fn parse(bytes: &mut R, encoding: Encoding) -> Result<Operation<R, Offset>>` — [`Encoding`](../../index.md), [`Result`](../../index.md), [`Operation`](../index.md)

#### Trait Implementations

##### `impl<R, Offset> Clone for Operation<R, Offset>`

- <span id="operation-clone"></span>`fn clone(&self) -> Operation<R, Offset>` — [`Operation`](../index.md)

##### `impl<R, Offset> Copy for Operation<R, Offset>`

##### `impl<R, Offset> Debug for Operation<R, Offset>`

- <span id="operation-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for Operation<R, Offset>`

##### `impl<R, Offset> PartialEq for Operation<R, Offset>`

- <span id="operation-eq"></span>`fn eq(&self, other: &Operation<R, Offset>) -> bool` — [`Operation`](../index.md)

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

##### `impl<R: fmt::Debug + Reader> Debug for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="location-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl<R, Offset> Clone for Location<R, Offset>`

- <span id="location-clone"></span>`fn clone(&self) -> Location<R, Offset>` — [`Location`](../index.md)

##### `impl<R, Offset> Copy for Location<R, Offset>`

##### `impl<R, Offset> Debug for Location<R, Offset>`

- <span id="location-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> PartialEq for Location<R, Offset>`

- <span id="location-eq"></span>`fn eq(&self, other: &Location<R, Offset>) -> bool` — [`Location`](../index.md)

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

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationState<R>`

- <span id="evaluationstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationWaiting<R>`

- <span id="evaluationwaiting-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationResult<R>`

- <span id="evaluationresult-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EvaluationResult<R>`

- <span id="evaluationresult-eq"></span>`fn eq(&self, other: &EvaluationResult<R>) -> bool` — [`EvaluationResult`](../index.md)

##### `impl<R: Reader> StructuralPartialEq for EvaluationResult<R>`

## Traits

### `EvaluationStorage<R: Reader>`

```rust
trait EvaluationStorage<R: Reader> { ... }
```

Specification of what storage should be used for [`Evaluation`](../index.md).

Normally you would only need to use [`StoreOnHeap`](../../index.md), which places the stacks and the results
on the heap using [`Vec`](../../../addr2line/maybe_small/index.md). This is the default storage type parameter for [`Evaluation`](../index.md).

If you need to avoid [`Evaluation`](../index.md) from allocating memory, e.g. for signal safety,
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

### `compute_pc`

```rust
fn compute_pc<R: Reader>(pc: &R, bytecode: &R, offset: i16) -> crate::read::Result<R>
```

### `generic_type`

```rust
fn generic_type<O: ReaderOffset>() -> crate::read::UnitOffset<O>
```

