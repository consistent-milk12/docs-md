*[rayon](../../index.md) / [iter](../index.md) / [walk_tree](index.md)*

---

# Module `walk_tree`

## Contents

- [Structs](#structs)
  - [`WalkTreePrefixProducer`](#walktreeprefixproducer)
  - [`WalkTreePrefix`](#walktreeprefix)
  - [`WalkTreePostfixProducer`](#walktreepostfixproducer)
  - [`WalkTreePostfix`](#walktreepostfix)
  - [`WalkTree`](#walktree)
- [Functions](#functions)
  - [`walk_tree_prefix`](#walk-tree-prefix)
  - [`consume_rec_postfix`](#consume-rec-postfix)
  - [`split_vec`](#split-vec)
  - [`walk_tree_postfix`](#walk-tree-postfix)
  - [`walk_tree`](#walk-tree)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WalkTreePrefixProducer`](#walktreeprefixproducer) | struct |  |
| [`WalkTreePrefix`](#walktreeprefix) | struct | ParallelIterator for arbitrary tree-shaped patterns. |
| [`WalkTreePostfixProducer`](#walktreepostfixproducer) | struct |  |
| [`WalkTreePostfix`](#walktreepostfix) | struct | ParallelIterator for arbitrary tree-shaped patterns. |
| [`WalkTree`](#walktree) | struct | ParallelIterator for arbitrary tree-shaped patterns. |
| [`walk_tree_prefix`](#walk-tree-prefix) | fn | Create a tree-like prefix parallel iterator from an initial root node. |
| [`consume_rec_postfix`](#consume-rec-postfix) | fn |  |
| [`split_vec`](#split-vec) | fn | Divide given vector in two equally sized vectors. |
| [`walk_tree_postfix`](#walk-tree-postfix) | fn | Create a tree like postfix parallel iterator from an initial root node. |
| [`walk_tree`](#walk-tree) | fn | Create a tree like parallel iterator from an initial root node. |

## Structs

### `WalkTreePrefixProducer<'b, S, B>`

```rust
struct WalkTreePrefixProducer<'b, S, B> {
    to_explore: Vec<S>,
    seen: Vec<S>,
    children_of: &'b B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:6-10`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L6-L10)*

#### Trait Implementations

##### `impl Any for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTreePrefixProducer<'b, S, B>`

##### `impl Pointable for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktreeprefixproducer-pointable-type-init"></span>`type Init = T`

- <span id="walktreeprefixproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreeprefixproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreeprefixproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreeprefixproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktreeprefixproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktreeprefixproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S, B> UnindexedProducer for WalkTreePrefixProducer<'_, S, B>`

- <span id="walktreeprefixproducer-unindexedproducer-type-item"></span>`type Item = S`

- <span id="walktreeprefixproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="walktreeprefixproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `WalkTreePrefix<S, B>`

```rust
struct WalkTreePrefix<S, B> {
    initial_state: S,
    children_of: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:76-79`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L76-L79)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_prefix()`](#walk-tree-prefix) function.

#### Trait Implementations

##### `impl Any for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTreePrefix<S, B>`

##### `impl IntoParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="walktreeprefix-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreeprefix-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B> ParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-paralleliterator-type-item"></span>`type Item = S`

- <span id="walktreeprefix-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktreeprefix-pointable-type-init"></span>`type Init = T`

- <span id="walktreeprefix-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreeprefix-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreeprefix-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreeprefix-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktreeprefix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktreeprefix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkTreePostfixProducer<'b, S, B>`

```rust
struct WalkTreePostfixProducer<'b, S, B> {
    to_explore: Vec<S>,
    seen: Vec<S>,
    children_of: &'b B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:219-223`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L219-L223)*

#### Trait Implementations

##### `impl Any for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTreePostfixProducer<'b, S, B>`

##### `impl Pointable for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktreepostfixproducer-pointable-type-init"></span>`type Init = T`

- <span id="walktreepostfixproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreepostfixproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreepostfixproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreepostfixproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktreepostfixproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktreepostfixproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<S, B> UnindexedProducer for WalkTreePostfixProducer<'_, S, B>`

- <span id="walktreepostfixproducer-unindexedproducer-type-item"></span>`type Item = S`

- <span id="walktreepostfixproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="walktreepostfixproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `WalkTreePostfix<S, B>`

```rust
struct WalkTreePostfix<S, B> {
    initial_state: S,
    children_of: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:302-305`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L302-L305)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_postfix()`](#walk-tree-postfix) function.

#### Trait Implementations

##### `impl Any for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTreePostfix<S, B>`

##### `impl IntoParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="walktreepostfix-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreepostfix-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B> ParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-paralleliterator-type-item"></span>`type Item = S`

- <span id="walktreepostfix-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktreepostfix-pointable-type-init"></span>`type Init = T`

- <span id="walktreepostfix-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreepostfix-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreepostfix-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreepostfix-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktreepostfix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktreepostfix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkTree<S, B>`

```rust
struct WalkTree<S, B>(WalkTreePostfix<S, B>);
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:459`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L459)*

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree()`](#walk-tree) function.

#### Trait Implementations

##### `impl Any for WalkTree<S, B>`

- <span id="walktree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkTree<S, B>`

- <span id="walktree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkTree<S, B>`

- <span id="walktree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTree<S, B>`

- <span id="walktree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkTree<S, B>`

- <span id="walktree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkTree<S, B>`

- <span id="walktree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WalkTree<S, B>`

##### `impl IntoParallelIterator for WalkTree<S, B>`

- <span id="walktree-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="walktree-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktree-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B> ParallelIterator for WalkTree<S, B>`

- <span id="walktree-paralleliterator-type-item"></span>`type Item = S`

- <span id="walktree-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for WalkTree<S, B>`

- <span id="walktree-pointable-const-align"></span>`const ALIGN: usize`

- <span id="walktree-pointable-type-init"></span>`type Init = T`

- <span id="walktree-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktree-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktree-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktree-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for WalkTree<S, B>`

- <span id="walktree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walktree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkTree<S, B>`

- <span id="walktree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walktree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `walk_tree_prefix`

```rust
fn walk_tree_prefix<S, B, I>(root: S, children_of: B) -> WalkTreePrefix<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S, IntoIter: DoubleEndedIterator>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:204-214`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L204-L214)*

Create a tree-like prefix parallel iterator from an initial root node.
The `children_of` function should take a node and return an iterator over its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function guarantees a prefix ordering. See also [`walk_tree_postfix`](#walk-tree-postfix),
which guarantees a postfix order.
If you don't care about ordering, you should use [`walk_tree`](#walk-tree),
which will use whatever is believed to be fastest.
For example a perfect binary tree of 7 nodes will reduced in the following order:

```text
     a
    / \
   /   \
  b     c
 / \   / \
d   e f   g

reduced as a,b,d,e,c,f,g

```

# Example

```text
     4
    / \
   /   \
  2     3
       / \
      1   2
```

```rust
use rayon::iter::walk_tree_prefix;
use rayon::prelude::*;

let par_iter = walk_tree_prefix(4, |&e| {
    if e <= 2 {
        Vec::new()
    } else {
        vec![e / 2, e / 2 + 1]
    }
});
assert_eq!(par_iter.sum::<u32>(), 12);
```

# Example

```rust
use rayon::prelude::*;
use rayon::iter::walk_tree_prefix;

struct Node {
    content: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Here we loop on the following tree:
//
//       10
//      /  \
//     /    \
//    3     14
//            \
//             \
//              18

let root = Node {
    content: 10,
    left: Some(Box::new(Node {
        content: 3,
        left: None,
        right: None,
    })),
    right: Some(Box::new(Node {
        content: 14,
        left: None,
        right: Some(Box::new(Node {
            content: 18,
            left: None,
            right: None,
        })),
    })),
};

let mut v: Vec<u32> = walk_tree_prefix(&root, |r| {
    r.left
        .as_ref()
        .into_iter()
        .chain(r.right.as_ref())
        .map(|n| &**n)
})
.map(|node| node.content)
.collect();
assert_eq!(v, vec![10, 3, 14, 18]);
```


### `consume_rec_postfix`

```rust
fn consume_rec_postfix<F, S, B, I>(children_of: &B, s: S, folder: F) -> F
where
    F: Folder<S>,
    B: Fn(&S) -> I,
    I: IntoIterator<Item = S>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:283-297`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L283-L297)*

### `split_vec`

```rust
fn split_vec<T>(v: &mut Vec<T>) -> Option<Vec<T>>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:331-338`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L331-L338)*

Divide given vector in two equally sized vectors.
Return `None` if initial size is <=1.
We return the first half and keep the last half in `v`.

### `walk_tree_postfix`

```rust
fn walk_tree_postfix<S, B, I>(root: S, children_of: B) -> WalkTreePostfix<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:444-454`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L444-L454)*

Create a tree like postfix parallel iterator from an initial root node.
The `children_of` function should take a node and iterate on all of its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function guarantees a postfix ordering. See also [`walk_tree_prefix`](#walk-tree-prefix) which guarantees a
prefix order. If you don't care about ordering, you should use [`walk_tree`](#walk-tree), which will use
whatever is believed to be fastest.

Between siblings, children are reduced in order -- that is first children are reduced first.

For example a perfect binary tree of 7 nodes will reduced in the following order:

```text
     a
    / \
   /   \
  b     c
 / \   / \
d   e f   g

reduced as d,e,b,f,g,c,a

```

# Example

```text
     4
    / \
   /   \
  2     3
       / \
      1   2
```

```rust
use rayon::iter::walk_tree_postfix;
use rayon::prelude::*;

let par_iter = walk_tree_postfix(4, |&e| {
    if e <= 2 {
        Vec::new()
    } else {
        vec![e / 2, e / 2 + 1]
    }
});
assert_eq!(par_iter.sum::<u32>(), 12);
```

# Example

```rust
use rayon::prelude::*;
use rayon::iter::walk_tree_postfix;

struct Node {
    content: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Here we loop on the following tree:
//
//       10
//      /  \
//     /    \
//    3     14
//            \
//             \
//              18

let root = Node {
    content: 10,
    left: Some(Box::new(Node {
        content: 3,
        left: None,
        right: None,
    })),
    right: Some(Box::new(Node {
        content: 14,
        left: None,
        right: Some(Box::new(Node {
            content: 18,
            left: None,
            right: None,
        })),
    })),
};

let mut v: Vec<u32> = walk_tree_postfix(&root, |r| {
    r.left
        .as_ref()
        .into_iter()
        .chain(r.right.as_ref())
        .map(|n| &**n)
})
.map(|node| node.content)
.collect();
assert_eq!(v, vec![3, 18, 14, 10]);
```


### `walk_tree`

```rust
fn walk_tree<S, B, I>(root: S, children_of: B) -> WalkTree<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S, IntoIter: DoubleEndedIterator>
```

*Defined in [`rayon-1.11.0/src/iter/walk_tree.rs:497-508`](../../../../.source_1765633015/rayon-1.11.0/src/iter/walk_tree.rs#L497-L508)*

Create a tree like parallel iterator from an initial root node.
The `children_of` function should take a node and iterate on all of its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function does not guarantee any ordering but will
use whatever algorithm is thought to achieve the fastest traversal.
See also [`walk_tree_prefix`](#walk-tree-prefix) which guarantees a
prefix order and [`walk_tree_postfix`](#walk-tree-postfix) which guarantees a postfix order.

# Example

```text
     4
    / \
   /   \
  2     3
       / \
      1   2
```

```rust
use rayon::iter::walk_tree;
use rayon::prelude::*;

let par_iter = walk_tree(4, |&e| {
    if e <= 2 {
        Vec::new()
    } else {
        vec![e / 2, e / 2 + 1]
    }
});
assert_eq!(par_iter.sum::<u32>(), 12);
```

