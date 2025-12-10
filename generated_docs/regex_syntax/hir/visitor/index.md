*[regex_syntax](../../index.md) / [hir](../index.md) / [visitor](index.md)*

---

# Module `visitor`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeapVisitor`](#heapvisitor) | struct | HeapVisitor visits every item in an `Hir` recursively using constant stack size and a heap size proportional to the size of the `Hir`. |
| [`Frame`](#frame) | enum | Represents a single stack frame while performing structural induction over an `Hir`. |
| [`Visitor`](#visitor) | trait | A trait for visiting the high-level IR (HIR) in depth first order. |
| [`visit`](#visit) | fn | Executes an implementation of `Visitor` in constant stack space. |

## Structs

### `HeapVisitor<'a>`

```rust
struct HeapVisitor<'a> {
    stack: alloc::vec::Vec<(&'a crate::hir::Hir, Frame<'a>)>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:71-75`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/visitor.rs#L71-L75)*

HeapVisitor visits every item in an `Hir` recursively using constant stack
size and a heap size proportional to the size of the `Hir`.

#### Fields

- **`stack`**: `alloc::vec::Vec<(&'a crate::hir::Hir, Frame<'a>)>`

  A stack of `Hir` nodes. This is roughly analogous to the call stack
  used in a typical recursive visitor.

#### Implementations

- <span id="heapvisitor-new"></span>`fn new() -> HeapVisitor<'a>` — [`HeapVisitor`](#heapvisitor)

- <span id="heapvisitor-visit"></span>`fn visit<V: Visitor>(&mut self, hir: &'a Hir, visitor: V) -> Result<<V as >::Output, <V as >::Err>` — [`Hir`](../index.md#hir), [`Visitor`](#visitor)

- <span id="heapvisitor-induct"></span>`fn induct(&mut self, hir: &'a Hir) -> Option<Frame<'a>>` — [`Hir`](../index.md#hir), [`Frame`](#frame)

- <span id="heapvisitor-pop"></span>`fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>>` — [`Frame`](#frame)

## Enums

### `Frame<'a>`

```rust
enum Frame<'a> {
    Repetition(&'a hir::Repetition),
    Capture(&'a hir::Capture),
    Concat {
        head: &'a crate::hir::Hir,
        tail: &'a [crate::hir::Hir],
    },
    Alternation {
        head: &'a crate::hir::Hir,
        tail: &'a [crate::hir::Hir],
    },
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:79-102`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/visitor.rs#L79-L102)*

Represents a single stack frame while performing structural induction over
an `Hir`.

#### Variants

- **`Repetition`**

  A stack frame allocated just before descending into a repetition
  operator's child node.

- **`Capture`**

  A stack frame allocated just before descending into a capture's child
  node.

- **`Concat`**

  The stack frame used while visiting every child node of a concatenation
  of expressions.

- **`Alternation`**

  The stack frame used while visiting every child node of an alternation
  of expressions.

#### Implementations

- <span id="frame-child"></span>`fn child(&self) -> &'a Hir` — [`Hir`](../index.md#hir)

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:15-49`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/visitor.rs#L15-L49)*

A trait for visiting the high-level IR (HIR) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on a high-level intermediate representation of a regular
expression without necessarily using recursion. In particular, this permits
callers to do case analysis with constant stack usage, which can be
important since the size of an HIR may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`](#visit) function.

#### Associated Types

- `type Output`

- `type Err`

#### Required Methods

- `fn finish(self) -> Result<<Self as >::Output, <Self as >::Err>`

  All implementors of `Visitor` must provide a `finish` method, which

#### Provided Methods

- `fn start(&mut self)`

  This method is called before beginning traversal of the HIR.

- `fn visit_pre(&mut self, _hir: &Hir) -> Result<(), <Self as >::Err>`

  This method is called on an `Hir` before descending into child `Hir`

- `fn visit_post(&mut self, _hir: &Hir) -> Result<(), <Self as >::Err>`

  This method is called on an `Hir` after descending all of its child

- `fn visit_alternation_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of an alternation.

- `fn visit_concat_in(&mut self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of a concatenation.

#### Implementors

- [`Writer`](../print/index.md#writer)

## Functions

### `visit`

```rust
fn visit<V: Visitor>(hir: &crate::hir::Hir, visitor: V) -> Result<<V as >::Output, <V as >::Err>
```

*Defined in [`regex-syntax-0.8.8/src/hir/visitor.rs:65-67`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/visitor.rs#L65-L67)*

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Hir` while calling
appropriate methods provided by the [`Visitor`](#visitor) trait.

The primary use case for this method is when one wants to perform case
analysis over an `Hir` without using a stack size proportional to the depth
of the `Hir`. Namely, this method will instead use constant stack space,
but will use heap space proportional to the size of the `Hir`. This may be
desirable in cases where the size of `Hir` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

