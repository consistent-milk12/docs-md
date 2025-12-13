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
  - [`compute_pc`](#compute-pc)
  - [`generic_type`](#generic-type)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Piece`](#piece) | struct | The description of a single piece of the result of a DWARF expression. |
| [`Expression`](#expression) | struct | The bytecode for a DWARF expression or location description. |
| [`OperationIter`](#operationiter) | struct | An iterator for the operations in an expression. |
| [`Evaluation`](#evaluation) | struct | A DWARF expression evaluator. |
| [`DieReference`](#diereference) | enum | A reference to a DIE, either relative to the current CU or relative to the section. |
| [`Operation`](#operation) | enum | A single decoded DWARF expression operation. |
| [`OperationEvaluationResult`](#operationevaluationresult) | enum |  |
| [`Location`](#location) | enum | A single location of a piece of the result of a DWARF expression. |
| [`EvaluationState`](#evaluationstate) | enum |  |
| [`EvaluationWaiting`](#evaluationwaiting) | enum |  |
| [`EvaluationResult`](#evaluationresult) | enum | The state of an `Evaluation` after evaluating a DWARF expression. |
| [`EvaluationStorage`](#evaluationstorage) | trait | Specification of what storage should be used for [`Evaluation`]. |
| [`compute_pc`](#compute-pc) | fn |  |
| [`generic_type`](#generic-type) | fn |  |

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

*Defined in [`gimli-0.32.3/src/read/op.rs:356-378`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L356-L378)*

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

- <span id="piece-clone"></span>`fn clone(&self) -> Piece<R, Offset>` — [`Piece`](../index.md#piece)

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

- <span id="piece-partialeq-eq"></span>`fn eq(&self, other: &Piece<R, Offset>) -> bool` — [`Piece`](../index.md#piece)

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

*Defined in [`gimli-0.32.3/src/read/op.rs:924`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L924)*

The bytecode for a DWARF expression or location description.

#### Implementations

- <span id="expression-evaluation"></span>`fn evaluation(self, encoding: Encoding) -> Evaluation<R>` — [`Encoding`](../../index.md#encoding), [`Evaluation`](../index.md#evaluation)

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

- <span id="expression-operations"></span>`fn operations(self, encoding: Encoding) -> OperationIter<R>` — [`Encoding`](../../index.md#encoding), [`OperationIter`](../index.md#operationiter)

  Return an iterator for the operations in the expression.

#### Trait Implementations

##### `impl Any for Expression<R>`

- <span id="expression-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Expression<R>`

- <span id="expression-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Expression<R>`

- <span id="expression-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for Expression<R>`

- <span id="expression-clone"></span>`fn clone(&self) -> Expression<R>` — [`Expression`](../index.md#expression)

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

- <span id="expression-partialeq-eq"></span>`fn eq(&self, other: &Expression<R>) -> bool` — [`Expression`](../index.md#expression)

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

*Defined in [`gimli-0.32.3/src/read/op.rs:962-965`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L962-L965)*

An iterator for the operations in an expression.

#### Implementations

- <span id="operationiter-next"></span>`fn next(&mut self) -> Result<Option<Operation<R>>>` — [`Result`](../../index.md#result), [`Operation`](../index.md#operation)

  Read the next operation in an expression.

- <span id="operationiter-offset-from"></span>`fn offset_from(&self, expression: &Expression<R>) -> <R as >::Offset` — [`Expression`](../index.md#expression), [`Reader`](../index.md#reader)

  Return the current byte offset of the iterator.

#### Trait Implementations

##### `impl Any for OperationIter<R>`

- <span id="operationiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OperationIter<R>`

- <span id="operationiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OperationIter<R>`

- <span id="operationiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: clone::Clone + Reader> Clone for OperationIter<R>`

- <span id="operationiter-clone"></span>`fn clone(&self) -> OperationIter<R>` — [`OperationIter`](../index.md#operationiter)

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

*Defined in [`gimli-0.32.3/src/read/op.rs:1106-1131`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L1106-L1131)*

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

- <span id="evaluation-new"></span>`fn new(bytecode: R, encoding: Encoding) -> Self` — [`Encoding`](../../index.md#encoding)

  Create a new DWARF expression evaluator.

  

  The new evaluator is created without an initial value, without

  an object address, and without a maximum number of iterations.

- <span id="evaluation-result"></span>`fn result(self) -> Vec<Piece<R>>` — [`Piece`](../index.md#piece)

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

## Enums

### `DieReference<T>`

```rust
enum DieReference<T> {
    UnitRef(crate::read::UnitOffset<T>),
    DebugInfoRef(crate::common::DebugInfoOffset<T>),
}
```

*Defined in [`gimli-0.32.3/src/read/op.rs:15-20`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L15-L20)*

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

- <span id="diereference-clone"></span>`fn clone(&self) -> DieReference<T>` — [`DieReference`](../index.md#diereference)

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

- <span id="diereference-partialeq-eq"></span>`fn eq(&self, other: &DieReference<T>) -> bool` — [`DieReference`](../index.md#diereference)

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

*Defined in [`gimli-0.32.3/src/read/op.rs:34-293`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L34-L293)*

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

- <span id="operation-parse"></span>`fn parse(bytes: &mut R, encoding: Encoding) -> Result<Operation<R, Offset>>` — [`Encoding`](../../index.md#encoding), [`Result`](../../index.md#result), [`Operation`](../index.md#operation)

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

- <span id="operation-clone"></span>`fn clone(&self) -> Operation<R, Offset>` — [`Operation`](../index.md#operation)

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

- <span id="operation-partialeq-eq"></span>`fn eq(&self, other: &Operation<R, Offset>) -> bool` — [`Operation`](../index.md#operation)

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

*Defined in [`gimli-0.32.3/src/read/op.rs:296-301`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L296-L301)*

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

*Defined in [`gimli-0.32.3/src/read/op.rs:305-340`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L305-L340)*

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

- <span id="location-clone"></span>`fn clone(&self) -> Location<R, Offset>` — [`Location`](../index.md#location)

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

- <span id="location-partialeq-eq"></span>`fn eq(&self, other: &Location<R, Offset>) -> bool` — [`Location`](../index.md#location)

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

*Defined in [`gimli-0.32.3/src/read/op.rs:816-822`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L816-L822)*

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

*Defined in [`gimli-0.32.3/src/read/op.rs:825-839`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L825-L839)*

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

*Defined in [`gimli-0.32.3/src/read/op.rs:845-920`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L845-L920)*

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

- <span id="evaluationresult-partialeq-eq"></span>`fn eq(&self, other: &EvaluationResult<R>) -> bool` — [`EvaluationResult`](../index.md#evaluationresult)

##### `impl<R: Reader> StructuralPartialEq for EvaluationResult<R>`

##### `impl<U> TryFrom for EvaluationResult<R>`

- <span id="evaluationresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="evaluationresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EvaluationResult<R>`

- <span id="evaluationresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="evaluationresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `EvaluationStorage<R: Reader>`

```rust
trait EvaluationStorage<R: Reader> { ... }
```

*Defined in [`gimli-0.32.3/src/read/op.rs:1044-1051`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L1044-L1051)*

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

#### Associated Types

- `type Stack: 1`

- `type ExpressionStack: 1`

- `type Result: 1`

#### Implementors

- [`StoreOnHeap`](../../index.md#storeonheap)

## Functions

### `compute_pc`

```rust
fn compute_pc<R: Reader>(pc: &R, bytecode: &R, offset: i16) -> crate::read::Result<R>
```

*Defined in [`gimli-0.32.3/src/read/op.rs:381-391`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L381-L391)*

### `generic_type`

```rust
fn generic_type<O: ReaderOffset>() -> crate::read::UnitOffset<O>
```

*Defined in [`gimli-0.32.3/src/read/op.rs:393-395`](../../../../.source_1765521767/gimli-0.32.3/src/read/op.rs#L393-L395)*

