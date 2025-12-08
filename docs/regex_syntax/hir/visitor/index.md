*[regex_syntax](../../index.md) / [hir](../index.md) / [visitor](index.md)*

---

# Module `visitor`

## Structs

### `HeapVisitor<'a>`

```rust
struct HeapVisitor<'a> {
    stack: alloc::vec::Vec<(&'a crate::hir::Hir, Frame<'a>)>,
}
```

HeapVisitor visits every item in an `Hir` recursively using constant stack
size and a heap size proportional to the size of the `Hir`.

#### Fields

- **`stack`**: `alloc::vec::Vec<(&'a crate::hir::Hir, Frame<'a>)>`

  A stack of `Hir` nodes. This is roughly analogous to the call stack
  used in a typical recursive visitor.

#### Implementations

- `fn new() -> HeapVisitor<'a>` — [`HeapVisitor`](#heapvisitor)

- `fn visit<V: Visitor>(self: &mut Self, hir: &'a Hir, visitor: V) -> Result<<V as >::Output, <V as >::Err>` — [`Hir`](../index.md), [`Visitor`](#visitor)

- `fn induct(self: &mut Self, hir: &'a Hir) -> Option<Frame<'a>>` — [`Hir`](../index.md), [`Frame`](#frame)

- `fn pop(self: &Self, induct: Frame<'a>) -> Option<Frame<'a>>` — [`Frame`](#frame)

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

- `fn child(self: &Self) -> &'a Hir` — [`Hir`](../index.md)

## Traits

### `Visitor`

```rust
trait Visitor { ... }
```

A trait for visiting the high-level IR (HIR) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on a high-level intermediate representation of a regular
expression without necessarily using recursion. In particular, this permits
callers to do case analysis with constant stack usage, which can be
important since the size of an HIR may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`](#visit) function.

#### Required Methods

- `type Output`

- `type Err`

- `fn finish(self: Self) -> Result<<Self as >::Output, <Self as >::Err>`

  All implementors of `Visitor` must provide a `finish` method, which

- `fn start(self: &mut Self)`

  This method is called before beginning traversal of the HIR.

- `fn visit_pre(self: &mut Self, _hir: &Hir) -> Result<(), <Self as >::Err>`

  This method is called on an `Hir` before descending into child `Hir`

- `fn visit_post(self: &mut Self, _hir: &Hir) -> Result<(), <Self as >::Err>`

  This method is called on an `Hir` after descending all of its child

- `fn visit_alternation_in(self: &mut Self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of an alternation.

- `fn visit_concat_in(self: &mut Self) -> Result<(), <Self as >::Err>`

  This method is called between child nodes of a concatenation.

## Functions

### `visit`

```rust
fn visit<V: Visitor>(hir: &crate::hir::Hir, visitor: V) -> Result<<V as >::Output, <V as >::Err>
```

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

