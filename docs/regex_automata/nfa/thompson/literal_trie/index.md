*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [literal_trie](index.md)*

---

# Module `literal_trie`

## Structs

### `LiteralTrie`

```rust
struct LiteralTrie {
    states: alloc::vec::Vec<State>,
    rev: bool,
}
```

A trie that preserves leftmost-first match semantics.

This is a purpose-built data structure for optimizing 'lit1|lit2|..|litN'
patterns. It can *only* handle alternations of literals, which makes it
somewhat restricted in its scope, but literal alternations are fairly
common.

At a 5,000 foot level, the main idea of this trie is make an alternation of
literals look more like a DFA than an NFA via epsilon removal.

More precisely, the main issue is in how alternations are compiled into
a Thompson NFA. Namely, each alternation gets a single NFA "union" state
with an epsilon transition for every branch of the alternation pointing to
an NFA state corresponding to the start of that branch. The main problem
with this representation is the cost of computing an epsilon closure. Once
you hit the alternation's start state, it acts as a sort of "clog" that
requires you to traverse all of the epsilon transitions to compute the full
closure.

While fixing such clogs in the general case is pretty tricky without going
to a DFA (or perhaps a Glushkov NFA, but that comes with other problems).
But at least in the case of an alternation of literals, we can convert
that to a prefix trie without too much cost. In theory, that's all you
really need to do: build the trie and then compile it to a Thompson NFA.
For example, if you have the pattern 'bar|baz|foo', then using a trie, it
is transformed to something like 'b(a(r|z))|f'. This reduces the clog by
reducing the number of epsilon transitions out of the alternation's start
state from 3 to 2 (it actually gets down to 1 when you use a sparse state,
which we do below). It's a small effect here, but when your alternation is
huge, the savings is also huge.

And that is... essentially what a LiteralTrie does. But there is one
hiccup. Consider a regex like 'sam|samwise'. How does a prefix trie compile
that when leftmost-first semantics are used? If 'sam|samwise' was the
entire regex, then you could just drop the 'samwise' branch entirely since
it is impossible to match ('sam' will always take priority, and since it
is a prefix of 'samwise', 'samwise' will never match). But what about the
regex '\b(sam|samwise)\b'? In that case, you can't remove 'samwise' because
it might match when 'sam' doesn't fall on a word boundary.

The main idea is that 'sam|samwise' can be translated to 'sam(?:|wise)',
which is a precisely equivalent regex that also gets rid of the clog.

Another example is 'zapper|z|zap'. That gets translated to
'z(?:apper||ap)'.

We accomplish this by giving each state in the trie multiple "chunks" of
transitions. Each chunk barrier represents a match. The idea is that once
you know a match occurs, none of the transitions after the match can be
re-ordered and mixed in with the transitions before the match. Otherwise,
the match semantics could be changed.

See the 'State' data type for a bit more detail.

Future work:

* In theory, it would be nice to generalize the idea of removing clogs and
apply it to the NFA graph itself. Then this could in theory work for
case insensitive alternations of literals, or even just alternations where
each branch starts with a non-epsilon transition.
* Could we instead use the Aho-Corasick algorithm here? The aho-corasick
crate deals with leftmost-first matches correctly, but I think this implies
encoding failure transitions into a Thompson NFA somehow. Which seems fine,
because failure transitions are just unconditional epsilon transitions?
* Or perhaps even better, could we use an aho_corasick::AhoCorasick
directly? At time of writing, 0.7 is the current version of the
aho-corasick crate, and that definitely cannot be used as-is. But if we
expose the underlying finite state machine API, then could we use it? That
would be super. If we could figure that out, it might also lend itself to
more general composition of finite state machines.

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  The set of trie states. Each state contains one or more chunks, where
  each chunk is a sparse set of transitions to other states. A leaf state
  is always a match state that contains only empty chunks (i.e., no
  transitions).

- **`rev`**: `bool`

  Whether to add literals in reverse to the trie. Useful when building
  a reverse NFA automaton.

#### Implementations

- `fn forward() -> LiteralTrie` — [`LiteralTrie`](#literaltrie)

- `fn reverse() -> LiteralTrie` — [`LiteralTrie`](#literaltrie)

- `fn add(self: &mut Self, bytes: &[u8]) -> Result<(), BuildError>` — [`BuildError`](../index.md)

- `fn get_or_add_state(self: &mut Self, from: StateID, byte: u8) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn compile(self: &Self, builder: &mut Builder) -> Result<ThompsonRef, BuildError>` — [`Builder`](../index.md), [`ThompsonRef`](../compiler/index.md), [`BuildError`](../index.md)

#### Trait Implementations

##### `impl Clone for LiteralTrie`

- `fn clone(self: &Self) -> LiteralTrie` — [`LiteralTrie`](#literaltrie)

##### `impl Debug for LiteralTrie`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Frame<'a>`

```rust
struct Frame<'a> {
    chunks: StateChunksIter<'a>,
    transitions: core::slice::Iter<'a, Transition>,
    union: alloc::vec::Vec<crate::util::primitives::StateID>,
    sparse: alloc::vec::Vec<thompson::Transition>,
}
```

An explicit stack frame used for traversing the trie without using
recursion.

Each frame is tied to the traversal of a single trie state. The frame is
dropped once the entire state (and all of its children) have been visited.
The "output" of compiling a state is the 'union' vector, which is turn
converted to a NFA union state. Each branch of the union corresponds to a
chunk in the trie state.

'sparse' corresponds to the set of transitions for a particular chunk in a
trie state. It is ultimately converted to an NFA sparse state. The 'sparse'
field, after being converted to a sparse NFA state, is reused for any
subsequent chunks in the trie state, if any exist.

#### Fields

- **`chunks`**: `StateChunksIter<'a>`

  The remaining chunks to visit for a trie state.

- **`transitions`**: `core::slice::Iter<'a, Transition>`

  The transitions of the current chunk that we're iterating over. Since
  every trie state has at least one chunk, every frame is initialized
  with the first chunk's transitions ready to be consumed.

- **`union`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The NFA state IDs pointing to the start of each chunk compiled by
  this trie state. This ultimately gets converted to an NFA union once
  the entire trie state (and all of its children) have been compiled.
  The order of these matters for leftmost-first match semantics, since
  earlier matches in the union are preferred over later ones.

- **`sparse`**: `alloc::vec::Vec<thompson::Transition>`

  The actual NFA transitions for a single chunk in a trie state. This
  gets converted to an NFA sparse state, and its corresponding NFA state
  ID should get added to 'union'.

#### Implementations

- `fn new(state: &'a State) -> Frame<'a>` — [`State`](#state), [`Frame`](#frame)

#### Trait Implementations

##### `impl<'a> Debug for Frame<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `State`

```rust
struct State {
    transitions: alloc::vec::Vec<Transition>,
    chunks: alloc::vec::Vec<(usize, usize)>,
}
```

A state in a trie.

This uses a sparse representation. Since we don't use literal tries
for searching, and ultimately (and compilation requires visiting every
transition anyway), we use a sparse representation for transitions. This
means we save on memory, at the expense of 'LiteralTrie::add' being perhaps
a bit slower.

While 'transitions' is pretty standard as far as tries goes, the 'chunks'
piece here is more unusual. In effect, 'chunks' defines a partitioning
of 'transitions', where each chunk corresponds to a distinct set of
transitions. The key invariant is that a transition in one chunk cannot
be moved to another chunk. This is the secret sauce that preserve
leftmost-first match semantics.

A new chunk is added whenever we mark a state as a match state. Once a
new chunk is added, the old active chunk is frozen and is never mutated
again. The new chunk becomes the active chunk, which is defined as
'&transitions[chunks.last().map_or(0, |c| c.1)..]'. Thus, a state where
'chunks' is empty actually contains one chunk. Thus, every state contains
at least one (possibly empty) chunk.

A "leaf" state is a state that has no outgoing transitions (so
'transitions' is empty). Note that there is no way for a leaf state to be a
non-matching state. (Although while building the trie, within 'add', a leaf
state may exist while not containing any matches. But this invariant is
only broken within 'add'. Once 'add' returns, the invariant is upheld.)

#### Implementations

- `fn add_match(self: &mut Self)`

- `fn is_leaf(self: &Self) -> bool`

- `fn chunks(self: &Self) -> StateChunksIter<'_>` — [`StateChunksIter`](#statechunksiter)

- `fn active_chunk(self: &Self) -> &[Transition]` — [`Transition`](#transition)

- `fn active_chunk_start(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for State`

- `fn default() -> State` — [`State`](#state)

### `StateChunksIter<'a>`

```rust
struct StateChunksIter<'a> {
    transitions: &'a [Transition],
    chunks: core::slice::Iter<'a, (usize, usize)>,
    active: Option<&'a [Transition]>,
}
```

An iterator over all of the chunks in a state, including the active chunk.

This iterator is created by `State::chunks`. We name this iterator so that
we can include it in the `Frame` type for non-recursive trie traversal.

#### Trait Implementations

##### `impl<'a> Debug for StateChunksIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for StateChunksIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for StateChunksIter<'a>`

- `type Item = &'a [Transition]`

- `fn next(self: &mut Self) -> Option<&'a [Transition]>` — [`Transition`](#transition)

### `Transition`

```rust
struct Transition {
    byte: u8,
    next: crate::util::primitives::StateID,
}
```

A single transition in a trie to another state.

#### Trait Implementations

##### `impl Clone for Transition`

- `fn clone(self: &Self) -> Transition` — [`Transition`](#transition)

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

