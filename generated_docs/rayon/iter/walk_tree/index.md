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
  - [`walk_tree_prefix`](#walk_tree_prefix)
  - [`consume_rec_postfix`](#consume_rec_postfix)
  - [`split_vec`](#split_vec)
  - [`walk_tree_postfix`](#walk_tree_postfix)
  - [`walk_tree`](#walk_tree)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WalkTreePrefixProducer`](#walktreeprefixproducer) | struct |  |
| [`WalkTreePrefix`](#walktreeprefix) | struct | ParallelIterator for arbitrary tree-shaped patterns. |
| [`WalkTreePostfixProducer`](#walktreepostfixproducer) | struct |  |
| [`WalkTreePostfix`](#walktreepostfix) | struct | ParallelIterator for arbitrary tree-shaped patterns. |
| [`WalkTree`](#walktree) | struct | ParallelIterator for arbitrary tree-shaped patterns. |
| [`walk_tree_prefix`](#walk_tree_prefix) | fn | Create a tree-like prefix parallel iterator from an initial root node. |
| [`consume_rec_postfix`](#consume_rec_postfix) | fn |  |
| [`split_vec`](#split_vec) | fn | Divide given vector in two equally sized vectors. |
| [`walk_tree_postfix`](#walk_tree_postfix) | fn | Create a tree like postfix parallel iterator from an initial root node. |
| [`walk_tree`](#walk_tree) | fn | Create a tree like parallel iterator from an initial root node. |

## Structs

### `WalkTreePrefixProducer<'b, S, B>`

```rust
struct WalkTreePrefixProducer<'b, S, B> {
    to_explore: Vec<S>,
    seen: Vec<S>,
    children_of: &'b B,
}
```

#### Trait Implementations

##### `impl<'b, S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTreePrefixProducer<'b, S, B>`

##### `impl<T> Pointable for WalkTreePrefixProducer<'b, S, B>`

- <span id="walktreeprefixproducer-align"></span>`const ALIGN: usize`

- <span id="walktreeprefixproducer-init"></span>`type Init = T`

- <span id="walktreeprefixproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreeprefixproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreeprefixproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreeprefixproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<S, B, I> UnindexedProducer for WalkTreePrefixProducer<'_, S, B>`

- <span id="walktreeprefixproducer-item"></span>`type Item = S`

- <span id="walktreeprefixproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="walktreeprefixproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `WalkTreePrefix<S, B>`

```rust
struct WalkTreePrefix<S, B> {
    initial_state: S,
    children_of: B,
}
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_prefix()`](../index.md) function.

#### Trait Implementations

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTreePrefix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-iter"></span>`type Iter = T`

- <span id="walktreeprefix-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreeprefix-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-item"></span>`type Item = S`

- <span id="walktreeprefix-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WalkTreePrefix<S, B>`

- <span id="walktreeprefix-align"></span>`const ALIGN: usize`

- <span id="walktreeprefix-init"></span>`type Init = T`

- <span id="walktreeprefix-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreeprefix-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreeprefix-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreeprefix-drop"></span>`unsafe fn drop(ptr: usize)`

### `WalkTreePostfixProducer<'b, S, B>`

```rust
struct WalkTreePostfixProducer<'b, S, B> {
    to_explore: Vec<S>,
    seen: Vec<S>,
    children_of: &'b B,
}
```

#### Trait Implementations

##### `impl<'b, S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTreePostfixProducer<'b, S, B>`

##### `impl<T> Pointable for WalkTreePostfixProducer<'b, S, B>`

- <span id="walktreepostfixproducer-align"></span>`const ALIGN: usize`

- <span id="walktreepostfixproducer-init"></span>`type Init = T`

- <span id="walktreepostfixproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreepostfixproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreepostfixproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreepostfixproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<S, B, I> UnindexedProducer for WalkTreePostfixProducer<'_, S, B>`

- <span id="walktreepostfixproducer-item"></span>`type Item = S`

- <span id="walktreepostfixproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="walktreepostfixproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `WalkTreePostfix<S, B>`

```rust
struct WalkTreePostfix<S, B> {
    initial_state: S,
    children_of: B,
}
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_postfix()`](../index.md) function.

#### Trait Implementations

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTreePostfix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-iter"></span>`type Iter = T`

- <span id="walktreepostfix-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktreepostfix-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-item"></span>`type Item = S`

- <span id="walktreepostfix-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WalkTreePostfix<S, B>`

- <span id="walktreepostfix-align"></span>`const ALIGN: usize`

- <span id="walktreepostfix-init"></span>`type Init = T`

- <span id="walktreepostfix-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktreepostfix-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktreepostfix-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktreepostfix-drop"></span>`unsafe fn drop(ptr: usize)`

### `WalkTree<S, B>`

```rust
struct WalkTree<S, B>(WalkTreePostfix<S, B>);
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree()`](../index.md) function.

#### Trait Implementations

##### `impl<S: fmt::Debug, B: fmt::Debug> Debug for WalkTree<S, B>`

- <span id="walktree-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for WalkTree<S, B>`

##### `impl<T> IntoParallelIterator for WalkTree<S, B>`

- <span id="walktree-iter"></span>`type Iter = T`

- <span id="walktree-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="walktree-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTree<S, B>`

- <span id="walktree-item"></span>`type Item = S`

- <span id="walktree-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WalkTree<S, B>`

- <span id="walktree-align"></span>`const ALIGN: usize`

- <span id="walktree-init"></span>`type Init = T`

- <span id="walktree-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="walktree-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="walktree-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="walktree-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `walk_tree_prefix`

```rust
fn walk_tree_prefix<S, B, I>(root: S, children_of: B) -> WalkTreePrefix<S, B>
where
    S: Send,
    B: Fn(&S) -> I + Send + Sync,
    I: IntoIterator<Item = S, IntoIter: DoubleEndedIterator>
```

Create a tree-like prefix parallel iterator from an initial root node.
The `children_of` function should take a node and return an iterator over its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function guarantees a prefix ordering. See also [`walk_tree_postfix`](../index.md),
which guarantees a postfix order.
If you don't care about ordering, you should use [`walk_tree`](../index.md),
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

### `split_vec`

```rust
fn split_vec<T>(v: &mut Vec<T>) -> Option<Vec<T>>
```

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

Create a tree like postfix parallel iterator from an initial root node.
The `children_of` function should take a node and iterate on all of its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function guarantees a postfix ordering. See also [`walk_tree_prefix`](../index.md) which guarantees a
prefix order. If you don't care about ordering, you should use [`walk_tree`](../index.md), which will use
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

Create a tree like parallel iterator from an initial root node.
The `children_of` function should take a node and iterate on all of its child nodes.
The best parallelization is obtained when the tree is balanced
but we should also be able to handle harder cases.

# Ordering

This function does not guarantee any ordering but will
use whatever algorithm is thought to achieve the fastest traversal.
See also [`walk_tree_prefix`](../index.md) which guarantees a
prefix order and [`walk_tree_postfix`](../index.md) which guarantees a postfix order.

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

