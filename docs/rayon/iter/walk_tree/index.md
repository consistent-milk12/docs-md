*[rayon](../../index.md) / [iter](../index.md) / [walk_tree](index.md)*

---

# Module `walk_tree`

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

##### `impl<'b, S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTreePrefixProducer<'b, S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTreePrefixProducer<'b, S, B>`

##### `impl<T> Pointable for WalkTreePrefixProducer<'b, S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<S, B, I> UnindexedProducer for WalkTreePrefixProducer<'_, S, B>`

- `type Item = S`

- `fn split(self: Self) -> (Self, Option<Self>)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `WalkTreePrefix<S, B>`

```rust
struct WalkTreePrefix<S, B> {
    initial_state: S,
    children_of: B,
}
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_prefix()`](#walk-tree-prefix) function.

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTreePrefix<S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTreePrefix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePrefix<S, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePrefix<S, B>`

- `type Item = S`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WalkTreePrefix<S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WalkTreePostfixProducer<'b, S, B>`

```rust
struct WalkTreePostfixProducer<'b, S, B> {
    to_explore: Vec<S>,
    seen: Vec<S>,
    children_of: &'b B,
}
```

#### Trait Implementations

##### `impl<'b, S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTreePostfixProducer<'b, S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTreePostfixProducer<'b, S, B>`

##### `impl<T> Pointable for WalkTreePostfixProducer<'b, S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<S, B, I> UnindexedProducer for WalkTreePostfixProducer<'_, S, B>`

- `type Item = S`

- `fn split(self: Self) -> (Self, Option<Self>)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `WalkTreePostfix<S, B>`

```rust
struct WalkTreePostfix<S, B> {
    initial_state: S,
    children_of: B,
}
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree_postfix()`](#walk-tree-postfix) function.

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTreePostfix<S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTreePostfix<S, B>`

##### `impl<T> IntoParallelIterator for WalkTreePostfix<S, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTreePostfix<S, B>`

- `type Item = S`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WalkTreePostfix<S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WalkTree<S, B>`

```rust
struct WalkTree<S, B>(WalkTreePostfix<S, B>);
```

ParallelIterator for arbitrary tree-shaped patterns.
Returned by the [`walk_tree()`](#walk-tree) function.

#### Trait Implementations

##### `impl<S: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for WalkTree<S, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WalkTree<S, B>`

##### `impl<T> IntoParallelIterator for WalkTree<S, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<S, B, I> ParallelIterator for WalkTree<S, B>`

- `type Item = S`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WalkTree<S, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

