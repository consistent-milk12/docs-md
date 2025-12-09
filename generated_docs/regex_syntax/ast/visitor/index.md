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
| [`HeapVisitor`](#heapvisitor) | struct | HeapVisitor visits every item in an `Ast` recursively using constant stack |
| [`Frame`](#frame) | enum | Represents a single stack frame while performing structural induction over |
| [`ClassFrame`](#classframe) | enum | Represents a single stack frame while performing structural induction over |
| [`ClassInduct`](#classinduct) | enum | A representation of the inductive step when performing structural induction |
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

- <span id="heapvisitor-visit"></span>`fn visit<V: Visitor>(&mut self, ast: &'a Ast, visitor: V) -> Result<<V as >::Output, <V as >::Err>` — [`Ast`](../index.md), [`Visitor`](../index.md)

- <span id="heapvisitor-induct"></span>`fn induct<V: Visitor>(&mut self, ast: &'a Ast, visitor: &mut V) -> Result<Option<Frame<'a>>, <V as >::Err>` — [`Ast`](../index.md), [`Frame`](#frame), [`Visitor`](../index.md)

- <span id="heapvisitor-pop"></span>`fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>>` — [`Frame`](#frame)

- <span id="heapvisitor-visit-class"></span>`fn visit_class<V: Visitor>(&mut self, ast: &'a ast::ClassBracketed, visitor: &mut V) -> Result<(), <V as >::Err>` — [`ClassBracketed`](../index.md), [`Visitor`](../index.md)

- <span id="heapvisitor-visit-class-pre"></span>`fn visit_class_pre<V: Visitor>(&self, ast: &ClassInduct<'a>, visitor: &mut V) -> Result<(), <V as >::Err>` — [`ClassInduct`](#classinduct), [`Visitor`](../index.md)

- <span id="heapvisitor-visit-class-post"></span>`fn visit_class_post<V: Visitor>(&self, ast: &ClassInduct<'a>, visitor: &mut V) -> Result<(), <V as >::Err>` — [`ClassInduct`](#classinduct), [`Visitor`](../index.md)

- <span id="heapvisitor-induct-class"></span>`fn induct_class(&self, ast: &ClassInduct<'a>) -> Option<ClassFrame<'a>>` — [`ClassInduct`](#classinduct), [`ClassFrame`](#classframe)

- <span id="heapvisitor-pop-class"></span>`fn pop_class(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>>` — [`ClassFrame`](#classframe)

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

- <span id="frame-child"></span>`fn child(&self) -> &'a Ast` — [`Ast`](../index.md)

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

#### Trait Implementations

##### `impl<'a> Debug for ClassFrame<'a>`

- <span id="classframe-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ClassInduct<'a>`

```rust
enum ClassInduct<'a> {
    Item(&'a ast::ClassSetItem),
    BinaryOp(&'a ast::ClassSetBinaryOp),
}
```

A representation of the inductive step when performing structural induction
over a character class.

Note that there is no analogous explicit type for the inductive step for
`Ast` nodes because the inductive step is just an `Ast`. For character
classes, the inductive step can produce one of two possible child nodes:
an item or a binary operation. (An item cannot be a binary operation
because that would imply binary operations can be unioned in the concrete
syntax, which is not possible.)

#### Implementations

- <span id="classinduct-from-bracketed"></span>`fn from_bracketed(ast: &'a ast::ClassBracketed) -> ClassInduct<'a>` — [`ClassBracketed`](../index.md), [`ClassInduct`](#classinduct)

- <span id="classinduct-from-set"></span>`fn from_set(ast: &'a ast::ClassSet) -> ClassInduct<'a>` — [`ClassSet`](../index.md), [`ClassInduct`](#classinduct)

#### Trait Implementations

##### `impl<'a> Debug for ClassInduct<'a>`

- <span id="classinduct-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

A trait for visiting an abstract syntax tree (AST) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on an abstract syntax tree without necessarily using recursion.
In particular, this permits callers to do case analysis with constant stack
usage, which can be important since the size of an abstract syntax tree
may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`](../index.md) function.

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

- [`NestLimiter`](../parse/index.md)
- [`TranslatorI`](../../hir/translate/index.md)
- [`Writer`](../print/index.md)

## Functions

### `visit`

```rust
fn visit<V: Visitor>(ast: &crate::ast::Ast, visitor: V) -> Result<<V as >::Output, <V as >::Err>
```

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Ast` while calling the
appropriate methods provided by the [`Visitor`](../index.md) trait.

The primary use case for this method is when one wants to perform case
analysis over an `Ast` without using a stack size proportional to the depth
of the `Ast`. Namely, this method will instead use constant stack size, but
will use heap space proportional to the size of the `Ast`. This may be
desirable in cases where the size of `Ast` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

