*[regex_syntax](../../index.md) / [ast](../index.md) / [visitor](index.md)*

---

# Module `visitor`

## Contents

- [Structs](#structs)
  - [`HeapVisitor`](#heapvisitor)
- [Enums](#enums)
  - [`Frame`](#frame)
  - [`ClassFrame`](#classframe)
  - [`ClassInduct`](#classinduct)
- [Traits](#traits)
  - [`Visitor`](#visitor)
- [Functions](#functions)
  - [`visit`](#visit)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeapVisitor`](#heapvisitor) | struct | HeapVisitor visits every item in an `Ast` recursively using constant stack size and a heap size proportional to the size of the `Ast`. |
| [`Frame`](#frame) | enum | Represents a single stack frame while performing structural induction over an `Ast`. |
| [`ClassFrame`](#classframe) | enum | Represents a single stack frame while performing structural induction over a character class. |
| [`ClassInduct`](#classinduct) | enum | A representation of the inductive step when performing structural induction over a character class. |
| [`Visitor`](#visitor) | trait | A trait for visiting an abstract syntax tree (AST) in depth first order. |
| [`visit`](#visit) | fn | Executes an implementation of `Visitor` in constant stack space. |

## Structs

### `HeapVisitor<'a>`

```rust
struct HeapVisitor<'a> {
    stack: alloc::vec::Vec<(&'a crate::ast::Ast, Frame<'a>)>,
    stack_class: alloc::vec::Vec<(ClassInduct<'a>, ClassFrame<'a>)>,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:124-132`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/visitor.rs#L124-L132)*

HeapVisitor visits every item in an `Ast` recursively using constant stack
size and a heap size proportional to the size of the `Ast`.

#### Fields

- **`stack`**: `alloc::vec::Vec<(&'a crate::ast::Ast, Frame<'a>)>`

  A stack of `Ast` nodes. This is roughly analogous to the call stack
  used in a typical recursive visitor.

- **`stack_class`**: `alloc::vec::Vec<(ClassInduct<'a>, ClassFrame<'a>)>`

  Similar to the `Ast` stack above, but is used only for character
  classes. In particular, character classes embed their own mini
  recursive syntax.

#### Implementations

- <span id="heapvisitor-new"></span>`fn new() -> HeapVisitor<'a>` — [`HeapVisitor`](#heapvisitor)

- <span id="heapvisitor-visit"></span>`fn visit<V: Visitor>(&mut self, ast: &'a Ast, visitor: V) -> Result<<V as >::Output, <V as >::Err>` — [`Ast`](../index.md#ast), [`Visitor`](#visitor)

- <span id="heapvisitor-induct"></span>`fn induct<V: Visitor>(&mut self, ast: &'a Ast, visitor: &mut V) -> Result<Option<Frame<'a>>, <V as >::Err>` — [`Ast`](../index.md#ast), [`Frame`](#frame), [`Visitor`](#visitor)

  Build a stack frame for the given AST if one is needed (which occurs if

  and only if there are child nodes in the AST). Otherwise, return None.

  

  If this visits a class, then the underlying visitor implementation may

  return an error which will be passed on here.

- <span id="heapvisitor-pop"></span>`fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>>` — [`Frame`](#frame)

  Pops the given frame. If the frame has an additional inductive step,

  then return it, otherwise return `None`.

- <span id="heapvisitor-visit-class"></span>`fn visit_class<V: Visitor>(&mut self, ast: &'a ast::ClassBracketed, visitor: &mut V) -> Result<(), <V as >::Err>` — [`ClassBracketed`](../index.md#classbracketed), [`Visitor`](#visitor)

- <span id="heapvisitor-visit-class-pre"></span>`fn visit_class_pre<V: Visitor>(&self, ast: &ClassInduct<'a>, visitor: &mut V) -> Result<(), <V as >::Err>` — [`ClassInduct`](#classinduct), [`Visitor`](#visitor)

  Call the appropriate `Visitor` methods given an inductive step.

- <span id="heapvisitor-visit-class-post"></span>`fn visit_class_post<V: Visitor>(&self, ast: &ClassInduct<'a>, visitor: &mut V) -> Result<(), <V as >::Err>` — [`ClassInduct`](#classinduct), [`Visitor`](#visitor)

  Call the appropriate `Visitor` methods given an inductive step.

- <span id="heapvisitor-induct-class"></span>`fn induct_class(&self, ast: &ClassInduct<'a>) -> Option<ClassFrame<'a>>` — [`ClassInduct`](#classinduct), [`ClassFrame`](#classframe)

  Build a stack frame for the given class node if one is needed (which

  occurs if and only if there are child nodes). Otherwise, return None.

- <span id="heapvisitor-pop-class"></span>`fn pop_class(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>>` — [`ClassFrame`](#classframe)

  Pops the given frame. If the frame has an additional inductive step,

  then return it, otherwise return `None`.

#### Trait Implementations

##### `impl Any for HeapVisitor<'a>`

- <span id="heapvisitor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HeapVisitor<'a>`

- <span id="heapvisitor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HeapVisitor<'a>`

- <span id="heapvisitor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for HeapVisitor<'a>`

- <span id="heapvisitor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HeapVisitor<'a>`

- <span id="heapvisitor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HeapVisitor<'a>`

- <span id="heapvisitor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="heapvisitor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HeapVisitor<'a>`

- <span id="heapvisitor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="heapvisitor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Frame<'a>`

```rust
enum Frame<'a> {
    Repetition(&'a ast::Repetition),
    Group(&'a ast::Group),
    Concat {
        head: &'a crate::ast::Ast,
        tail: &'a [crate::ast::Ast],
    },
    Alternation {
        head: &'a crate::ast::Ast,
        tail: &'a [crate::ast::Ast],
    },
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:136-159`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/visitor.rs#L136-L159)*

Represents a single stack frame while performing structural induction over
an `Ast`.

#### Variants

- **`Repetition`**

  A stack frame allocated just before descending into a repetition
  operator's child node.

- **`Group`**

  A stack frame allocated just before descending into a group's child
  node.

- **`Concat`**

  The stack frame used while visiting every child node of a concatenation
  of expressions.

- **`Alternation`**

  The stack frame used while visiting every child node of an alternation
  of expressions.

#### Implementations

- <span id="frame-child"></span>`fn child(&self) -> &'a Ast` — [`Ast`](../index.md#ast)

  Perform the next inductive step on this frame and return the next

  child AST node to visit.

#### Trait Implementations

##### `impl Any for Frame<'a>`

- <span id="frame-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Frame<'a>`

- <span id="frame-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Frame<'a>`

- <span id="frame-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Frame<'a>`

- <span id="frame-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Frame<'a>`

- <span id="frame-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Frame<'a>`

- <span id="frame-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frame-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Frame<'a>`

- <span id="frame-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frame-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassFrame<'a>`

```rust
enum ClassFrame<'a> {
    Union {
        head: &'a ast::ClassSetItem,
        tail: &'a [ast::ClassSetItem],
    },
    Binary {
        op: &'a ast::ClassSetBinaryOp,
    },
    BinaryLHS {
        op: &'a ast::ClassSetBinaryOp,
        lhs: &'a ast::ClassSet,
        rhs: &'a ast::ClassSet,
    },
    BinaryRHS {
        op: &'a ast::ClassSetBinaryOp,
        rhs: &'a ast::ClassSet,
    },
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:163-184`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/visitor.rs#L163-L184)*

Represents a single stack frame while performing structural induction over
a character class.

#### Variants

- **`Union`**

  The stack frame used while visiting every child node of a union of
  character class items.

- **`Binary`**

  The stack frame used while a binary class operation.

- **`BinaryLHS`**

  A stack frame allocated just before descending into a binary operator's
  left hand child node.

- **`BinaryRHS`**

  A stack frame allocated just before descending into a binary operator's
  right hand child node.

#### Implementations

- <span id="classframe-child"></span>`fn child(&self) -> ClassInduct<'a>` — [`ClassInduct`](#classinduct)

  Perform the next inductive step on this frame and return the next

  child class node to visit.

#### Trait Implementations

##### `impl Any for ClassFrame<'a>`

- <span id="classframe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassFrame<'a>`

- <span id="classframe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassFrame<'a>`

- <span id="classframe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ClassFrame<'a>`

- <span id="classframe-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for ClassFrame<'a>`

- <span id="classframe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassFrame<'a>`

- <span id="classframe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ClassFrame<'a>`

- <span id="classframe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classframe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassFrame<'a>`

- <span id="classframe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classframe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClassInduct<'a>`

```rust
enum ClassInduct<'a> {
    Item(&'a ast::ClassSetItem),
    BinaryOp(&'a ast::ClassSetBinaryOp),
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:195-198`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/visitor.rs#L195-L198)*

A representation of the inductive step when performing structural induction
over a character class.

Note that there is no analogous explicit type for the inductive step for
`Ast` nodes because the inductive step is just an `Ast`. For character
classes, the inductive step can produce one of two possible child nodes:
an item or a binary operation. (An item cannot be a binary operation
because that would imply binary operations can be unioned in the concrete
syntax, which is not possible.)

#### Implementations

- <span id="classinduct-from-bracketed"></span>`fn from_bracketed(ast: &'a ast::ClassBracketed) -> ClassInduct<'a>` — [`ClassBracketed`](../index.md#classbracketed), [`ClassInduct`](#classinduct)

- <span id="classinduct-from-set"></span>`fn from_set(ast: &'a ast::ClassSet) -> ClassInduct<'a>` — [`ClassSet`](../index.md#classset), [`ClassInduct`](#classinduct)

#### Trait Implementations

##### `impl Any for ClassInduct<'a>`

- <span id="classinduct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassInduct<'a>`

- <span id="classinduct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassInduct<'a>`

- <span id="classinduct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ClassInduct<'a>`

- <span id="classinduct-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for ClassInduct<'a>`

- <span id="classinduct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassInduct<'a>`

- <span id="classinduct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ClassInduct<'a>`

- <span id="classinduct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classinduct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassInduct<'a>`

- <span id="classinduct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classinduct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:20-102`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/visitor.rs#L20-L102)*

A trait for visiting an abstract syntax tree (AST) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on an abstract syntax tree without necessarily using recursion.
In particular, this permits callers to do case analysis with constant stack
usage, which can be important since the size of an abstract syntax tree
may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`](#visit) function.

Note that the abstract syntax tree for a regular expression is quite
complex. Unless you specifically need it, you might be able to use the much
simpler [high-level intermediate representation](crate::hir::Hir) and its
[corresponding `Visitor` trait](crate::hir::Visitor) instead.

#### Associated Types

- `type Output`

- `type Err`

#### Required Methods

- `fn finish(self) -> Result<<Self as >::Output, <Self as >::Err>`

  All implementors of `Visitor` must provide a `finish` method, which

#### Provided Methods

- `fn start(&mut self)`

  This method is called before beginning traversal of the AST.

- `fn visit_pre(&mut self, _ast: &Ast) -> Result<(), <Self as >::Err>`

  This method is called on an `Ast` before descending into child `Ast`

- `fn visit_post(&mut self, _ast: &Ast) -> Result<(), <Self as >::Err>`

  This method is called on an `Ast` after descending all of its child

- `fn visit_alternation_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of an

- `fn visit_concat_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of a concatenation.

- `fn visit_class_set_item_pre(&mut self, _ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>`

  This method is called on every [`ClassSetItem`](ast::ClassSetItem)

- `fn visit_class_set_item_post(&mut self, _ast: &ast::ClassSetItem) -> Result<(), <Self as >::Err>`

  This method is called on every [`ClassSetItem`](ast::ClassSetItem)

- `fn visit_class_set_binary_op_pre(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>`

  This method is called on every

- `fn visit_class_set_binary_op_post(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>`

  This method is called on every

- `fn visit_class_set_binary_op_in(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), <Self as >::Err>`

  This method is called between the left hand and right hand child nodes

#### Implementors

- [`NestLimiter`](../parse/index.md#nestlimiter)
- [`TranslatorI`](../../hir/translate/index.md#translatori)
- [`Writer`](../print/index.md#writer)

## Functions

### `visit`

```rust
fn visit<V: Visitor>(ast: &crate::ast::Ast, visitor: V) -> Result<<V as >::Output, <V as >::Err>
```

*Defined in [`regex-syntax-0.8.8/src/ast/visitor.rs:118-120`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/visitor.rs#L118-L120)*

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Ast` while calling the
appropriate methods provided by the [`Visitor`](#visitor) trait.

The primary use case for this method is when one wants to perform case
analysis over an `Ast` without using a stack size proportional to the depth
of the `Ast`. Namely, this method will instead use constant stack size, but
will use heap space proportional to the size of the `Ast`. This may be
desirable in cases where the size of `Ast` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

