*[regex_syntax](../../index.md) / [hir](../index.md) / [literal](index.md)*

---

# Module `literal`

Provides literal extraction from `Hir` expressions.

An [`Extractor`](#extractor) pulls literals out of [`Hir`](../index.md) expressions and returns a
[`Seq`](#seq) of [`Literal`](#literal)s.

The purpose of literal extraction is generally to provide avenues for
optimizing regex searches. The main idea is that substring searches can be an
order of magnitude faster than a regex search. Therefore, if one can execute
a substring search to find candidate match locations and only run the regex
search at those locations, then it is possible for huge improvements in
performance to be realized.

With that said, literal optimizations are generally a black art because even
though substring search is generally faster, if the number of candidates
produced is high, then it can create a lot of overhead by ping-ponging between
the substring search and the regex search.

Here are some heuristics that might be used to help increase the chances of
effective literal optimizations:

* Stick to small [`Seq`](#seq)s. If you search for too many literals, it's likely
to lead to substring search that is only a little faster than a regex search,
and thus the overhead of using literal optimizations in the first place might
make things slower overall.
* The literals in your [`Seq`](#seq) shouldn't be too short. In general, longer is
better. A sequence corresponding to single bytes that occur frequently in the
haystack, for example, is probably a bad literal optimization because it's
likely to produce many false positive candidates. Longer literals are less
likely to match, and thus probably produce fewer false positives.
* If it's possible to estimate the approximate frequency of each byte according
to some pre-computed background distribution, it is possible to compute a score
of how "good" a `Seq` is. If a `Seq` isn't good enough, you might consider
skipping the literal optimization and just use the regex engine.

(It should be noted that there are always pathological cases that can make
any kind of literal optimization be a net slower result. This is why it
might be a good idea to be conservative, or to even provide a means for
literal optimizations to be dynamically disabled if they are determined to be
ineffective according to some measure.)

You're encouraged to explore the methods on [`Seq`](#seq), which permit shrinking
the size of sequences in a preference-order preserving fashion.

Finally, note that it isn't strictly necessary to use an [`Extractor`](#extractor). Namely,
an `Extractor` only uses public APIs of the [`Seq`](#seq) and [`Literal`](#literal) types,
so it is possible to implement your own extractor. For example, for n-grams
or "inner" literals (i.e., not prefix or suffix literals). The `Extractor`
is mostly responsible for the case analysis over `Hir` expressions. Much of
the "trickier" parts are how to combine literal sequences, and that is all
implemented on [`Seq`](#seq).

## Contents

- [Structs](#structs)
  - [`Extractor`](#extractor)
  - [`Seq`](#seq)
  - [`Literal`](#literal)
  - [`PreferenceTrie`](#preferencetrie)
  - [`State`](#state)
- [Enums](#enums)
  - [`ExtractKind`](#extractkind)
- [Functions](#functions)
  - [`rank`](#rank)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Extractor`](#extractor) | struct | Extracts prefix or suffix literal sequences from [`Hir`] expressions. |
| [`Seq`](#seq) | struct | A sequence of literals. |
| [`Literal`](#literal) | struct | A single literal extracted from an [`Hir`] expression. |
| [`PreferenceTrie`](#preferencetrie) | struct | A "preference" trie that rejects literals that will never match when executing a leftmost first or "preference" search. |
| [`State`](#state) | struct | A single state in a trie. |
| [`ExtractKind`](#extractkind) | enum | The kind of literals to extract from an [`Hir`] expression. |
| [`rank`](#rank) | fn | Returns the "rank" of the given byte. |

## Structs

### `Extractor`

```rust
struct Extractor {
    kind: ExtractKind,
    limit_class: usize,
    limit_repeat: usize,
    limit_literal_len: usize,
    limit_total: usize,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:147-153`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L147-L153)*

Extracts prefix or suffix literal sequences from [`Hir`](../index.md) expressions.

Literal extraction is based on the following observations:

* Many regexes start with one or a small number of literals.
* Substring search for literals is often much faster (sometimes by an order
of magnitude) than a regex search.

Thus, in many cases, one can search for literals to find candidate starting
locations of a match, and then only run the full regex engine at each such
location instead of over the full haystack.

The main downside of literal extraction is that it can wind up causing a
search to be slower overall. For example, if there are many matches or if
there are many candidates that don't ultimately lead to a match, then a
lot of overhead will be spent in shuffling back-and-forth between substring
search and the regex engine. This is the fundamental reason why literal
optimizations for regex patterns is sometimes considered a "black art."

# Look-around assertions

Literal extraction treats all look-around assertions as-if they match every
empty string. So for example, the regex `\bquux\b` will yield a sequence
containing a single exact literal `quux`. However, not all occurrences
of `quux` correspond to a match a of the regex. For example, `\bquux\b`
does not match `ZquuxZ` anywhere because `quux` does not fall on a word
boundary.

In effect, if your regex contains look-around assertions, then a match of
an exact literal does not necessarily mean the regex overall matches. So
you may still need to run the regex engine in such cases to confirm the
match.

The precise guarantee you get from a literal sequence is: if every literal
in the sequence is exact and the original regex contains zero look-around
assertions, then a preference-order multi-substring search of those
literals will precisely match a preference-order search of the original
regex.

# Example

This shows how to extract prefixes:

```rust
use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

let hir = parse(r"(a|b|c)(x|y|z)[A-Z]+foo")?;
let got = Extractor::new().extract(&hir);
// All literals returned are "inexact" because none of them reach the
// match state.
let expected = Seq::from_iter([
    Literal::inexact("ax"),
    Literal::inexact("ay"),
    Literal::inexact("az"),
    Literal::inexact("bx"),
    Literal::inexact("by"),
    Literal::inexact("bz"),
    Literal::inexact("cx"),
    Literal::inexact("cy"),
    Literal::inexact("cz"),
]);
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

This shows how to extract suffixes:

```rust
use regex_syntax::{
    hir::literal::{Extractor, ExtractKind, Literal, Seq},
    parse,
};

let hir = parse(r"foo|[A-Z]+bar")?;
let got = Extractor::new().kind(ExtractKind::Suffix).extract(&hir);
// Since 'foo' gets to a match state, it is considered exact. But 'bar'
// does not because of the '[A-Z]+', and thus is marked inexact.
let expected = Seq::from_iter([
    Literal::exact("foo"),
    Literal::inexact("bar"),
]);
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="extractor-new"></span>`fn new() -> Extractor` — [`Extractor`](#extractor)

  Create a new extractor with a default configuration.

  

  The extractor can be optionally configured before calling

  `Extractor::extract` to get a literal sequence.

- <span id="extractor-extract"></span>`fn extract(&self, hir: &Hir) -> Seq` — [`Hir`](../index.md#hir), [`Seq`](#seq)

  Execute the extractor and return a sequence of literals.

- <span id="extractor-kind"></span>`fn kind(&mut self, kind: ExtractKind) -> &mut Extractor` — [`ExtractKind`](#extractkind), [`Extractor`](#extractor)

  Set the kind of literal sequence to extract from an [`Hir`](../index.md) expression.

  

  The default is to extract prefixes, but suffixes can be selected

  instead. The contract for prefixes is that every match of the

  corresponding `Hir` must start with one of the literals in the sequence

  returned. Moreover, the _order_ of the sequence returned corresponds to

  the preference order.

  

  Suffixes satisfy a similar contract in that every match of the

  corresponding `Hir` must end with one of the literals in the sequence

  returned. However, there is no guarantee that the literals are in

  preference order.

  

  Remember that a sequence can be infinite. For example, unless the

  limits are configured to be impractically large, attempting to extract

  prefixes (or suffixes) for the pattern `[A-Z]` will return an infinite

  sequence. Generally speaking, if the sequence returned is infinite,

  then it is presumed to be unwise to do prefix (or suffix) optimizations

  for the pattern.

- <span id="extractor-limit-class"></span>`fn limit_class(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

  Configure a limit on the length of the sequence that is permitted for

  a character class. If a character class exceeds this limit, then the

  sequence returned for it is infinite.

  

  This prevents classes like `[A-Z]` or `\pL` from getting turned into

  huge and likely unproductive sequences of literals.

  

  # Example

  

  This example shows how this limit can be lowered to decrease the tolerance

  for character classes being turned into literal sequences.

  

  ```rust

  use regex_syntax::{hir::literal::{Extractor, Seq}, parse};

  

  let hir = parse(r"[0-9]")?;

  

  let got = Extractor::new().extract(&hir);

  let expected = Seq::new([

      "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",

  ]);

  assert_eq!(expected, got);

  

  // Now let's shrink the limit and see how that changes things.

  let got = Extractor::new().limit_class(4).extract(&hir);

  let expected = Seq::infinite();

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="extractor-limit-repeat"></span>`fn limit_repeat(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

  Configure a limit on the total number of repetitions that is permitted

  before literal extraction is stopped.

  

  This is useful for limiting things like `(abcde){50}`, or more

  insidiously, `(?:){1000000000}`. This limit prevents any one single

  repetition from adding too much to a literal sequence.

  

  With this limit set, repetitions that exceed it will be stopped and any

  literals extracted up to that point will be made inexact.

  

  # Example

  

  This shows how to decrease the limit and compares it with the default.

  

  ```rust

  use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

  

  let hir = parse(r"(abc){8}")?;

  

  let got = Extractor::new().extract(&hir);

  let expected = Seq::new(["abcabcabcabcabcabcabcabc"]);

  assert_eq!(expected, got);

  

  // Now let's shrink the limit and see how that changes things.

  let got = Extractor::new().limit_repeat(4).extract(&hir);

  let expected = Seq::from_iter([

      Literal::inexact("abcabcabcabc"),

  ]);

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="extractor-limit-literal-len"></span>`fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

  Configure a limit on the maximum length of any literal in a sequence.

  

  This is useful for limiting things like `(abcde){5}{5}{5}{5}`. While

  each repetition or literal in that regex is small, when all the

  repetitions are applied, one ends up with a literal of length `5^4 =

  625`.

  

  With this limit set, literals that exceed it will be made inexact and

  thus prevented from growing.

  

  # Example

  

  This shows how to decrease the limit and compares it with the default.

  

  ```rust

  use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

  

  let hir = parse(r"(abc){2}{2}{2}")?;

  

  let got = Extractor::new().extract(&hir);

  let expected = Seq::new(["abcabcabcabcabcabcabcabc"]);

  assert_eq!(expected, got);

  

  // Now let's shrink the limit and see how that changes things.

  let got = Extractor::new().limit_literal_len(14).extract(&hir);

  let expected = Seq::from_iter([

      Literal::inexact("abcabcabcabcab"),

  ]);

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="extractor-limit-total"></span>`fn limit_total(&mut self, limit: usize) -> &mut Extractor` — [`Extractor`](#extractor)

  Configure a limit on the total number of literals that will be

  returned.

  

  This is useful as a practical measure for avoiding the creation of

  large sequences of literals. While the extractor will automatically

  handle local creations of large sequences (for example, `[A-Z]` yields

  an infinite sequence by default), large sequences can be created

  through non-local means as well.

  

  For example, `[ab]{3}{3}` would yield a sequence of length `512 = 2^9`

  despite each of the repetitions being small on their own. This limit

  thus represents a "catch all" for avoiding locally small sequences from

  combining into large sequences.

  

  # Example

  

  This example shows how reducing the limit will change the literal

  sequence returned.

  

  ```rust

  use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

  

  let hir = parse(r"[ab]{2}{2}")?;

  

  let got = Extractor::new().extract(&hir);

  let expected = Seq::new([

      "aaaa", "aaab", "aaba", "aabb",

      "abaa", "abab", "abba", "abbb",

      "baaa", "baab", "baba", "babb",

      "bbaa", "bbab", "bbba", "bbbb",

  ]);

  assert_eq!(expected, got);

  

  // The default limit is not too big, but big enough to extract all

  // literals from '[ab]{2}{2}'. If we shrink the limit to less than 16,

  // then we'll get a truncated set. Notice that it returns a sequence of

  // length 4 even though our limit was 10. This is because the sequence

  // is difficult to increase without blowing the limit. Notice also

  // that every literal in the sequence is now inexact because they were

  // stripped of some suffix.

  let got = Extractor::new().limit_total(10).extract(&hir);

  let expected = Seq::from_iter([

      Literal::inexact("aa"),

      Literal::inexact("ab"),

      Literal::inexact("ba"),

      Literal::inexact("bb"),

  ]);

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="extractor-extract-concat"></span>`fn extract_concat<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq` — [`Seq`](#seq)

  Extract a sequence from the given concatenation. Sequences from each of

  the child HIR expressions are combined via cross product.

  

  This short circuits once the cross product turns into a sequence

  containing only inexact literals.

- <span id="extractor-extract-alternation"></span>`fn extract_alternation<'a, I: Iterator<Item = &'a Hir>>(&self, it: I) -> Seq` — [`Seq`](#seq)

  Extract a sequence from the given alternation.

  

  This short circuits once the union turns into an infinite sequence.

- <span id="extractor-extract-repetition"></span>`fn extract_repetition(&self, rep: &hir::Repetition) -> Seq` — [`Repetition`](../index.md#repetition), [`Seq`](#seq)

  Extract a sequence of literals from the given repetition. We do our

  best, Some examples:

  

    'a*'    => [inexact(a), exact("")]

    'a*?'   => [exact(""), inexact(a)]

    'a+'    => [inexact(a)]

    'a{3}'  => [exact(aaa)]

    'a{3,5} => [inexact(aaa)]

  

  The key here really is making sure we get the 'inexact' vs 'exact'

  attributes correct on each of the literals we add. For example, the

  fact that 'a*' gives us an inexact 'a' and an exact empty string means

  that a regex like 'ab*c' will result in [inexact(ab), exact(ac)]

  literals being extracted, which might actually be a better prefilter

  than just 'a'.

- <span id="extractor-extract-class-unicode"></span>`fn extract_class_unicode(&self, cls: &hir::ClassUnicode) -> Seq` — [`ClassUnicode`](../index.md#classunicode), [`Seq`](#seq)

  Convert the given Unicode class into a sequence of literals if the

  class is small enough. If the class is too big, return an infinite

  sequence.

- <span id="extractor-extract-class-bytes"></span>`fn extract_class_bytes(&self, cls: &hir::ClassBytes) -> Seq` — [`ClassBytes`](../index.md#classbytes), [`Seq`](#seq)

  Convert the given byte class into a sequence of literals if the class

  is small enough. If the class is too big, return an infinite sequence.

- <span id="extractor-class-over-limit-unicode"></span>`fn class_over_limit_unicode(&self, cls: &hir::ClassUnicode) -> bool` — [`ClassUnicode`](../index.md#classunicode)

  Returns true if the given Unicode class exceeds the configured limits

  on this extractor.

- <span id="extractor-class-over-limit-bytes"></span>`fn class_over_limit_bytes(&self, cls: &hir::ClassBytes) -> bool` — [`ClassBytes`](../index.md#classbytes)

  Returns true if the given byte class exceeds the configured limits on

  this extractor.

- <span id="extractor-cross"></span>`fn cross(&self, seq1: Seq, seq2: &mut Seq) -> Seq` — [`Seq`](#seq)

  Compute the cross product of the two sequences if the result would be

  within configured limits. Otherwise, make `seq2` infinite and cross the

  infinite sequence with `seq1`.

- <span id="extractor-union"></span>`fn union(&self, seq1: Seq, seq2: &mut Seq) -> Seq` — [`Seq`](#seq)

  Union the two sequences if the result would be within configured

  limits. Otherwise, make `seq2` infinite and union the infinite sequence

  with `seq1`.

- <span id="extractor-enforce-literal-len"></span>`fn enforce_literal_len(&self, seq: &mut Seq)` — [`Seq`](#seq)

  Applies the literal length limit to the given sequence. If none of the

  literals in the sequence exceed the limit, then this is a no-op.

#### Trait Implementations

##### `impl Any for Extractor`

- <span id="extractor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Extractor`

- <span id="extractor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Extractor`

- <span id="extractor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Extractor`

- <span id="extractor-clone"></span>`fn clone(&self) -> Extractor` — [`Extractor`](#extractor)

##### `impl CloneToUninit for Extractor`

- <span id="extractor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Extractor`

- <span id="extractor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extractor`

- <span id="extractor-default"></span>`fn default() -> Extractor` — [`Extractor`](#extractor)

##### `impl<T> From for Extractor`

- <span id="extractor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Extractor`

- <span id="extractor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Extractor`

- <span id="extractor-toowned-type-owned"></span>`type Owned = T`

- <span id="extractor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="extractor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Extractor`

- <span id="extractor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extractor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Extractor`

- <span id="extractor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extractor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Seq`

```rust
struct Seq {
    literals: Option<alloc::vec::Vec<Literal>>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:733-745`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L733-L745)*

A sequence of literals.

A `Seq` is very much like a set in that it represents a union of its
members. That is, it corresponds to a set of literals where at least one
must match in order for a particular [`Hir`](../index.md) expression to match. (Whether
this corresponds to the entire `Hir` expression, a prefix of it or a suffix
of it depends on how the `Seq` was extracted from the `Hir`.)

It is also unlike a set in that multiple identical literals may appear,
and that the order of the literals in the `Seq` matters. For example, if
the sequence is `[sam, samwise]` and leftmost-first matching is used, then
`samwise` can never match and the sequence is equivalent to `[sam]`.

# States of a sequence

A `Seq` has a few different logical states to consider:

* The sequence can represent "any" literal. When this happens, the set does
not have a finite size. The purpose of this state is to inhibit callers
from making assumptions about what literals are required in order to match
a particular [`Hir`](../index.md) expression. Generally speaking, when a set is in this
state, literal optimizations are inhibited. A good example of a regex that
will cause this sort of set to appear is `[A-Za-z]`. The character class
is just too big (and also too narrow) to be usefully expanded into 52
different literals. (Note that the decision for when a seq should become
infinite is determined by the caller. A seq itself has no hard-coded
limits.)
* The sequence can be empty, in which case, it is an affirmative statement
that there are no literals that can match the corresponding `Hir`.
Consequently, the `Hir` never matches any input. For example, `[a&&b]`.
* The sequence can be non-empty, in which case, at least one of the
literals must match in order for the corresponding `Hir` to match.

# Example

This example shows how literal sequences can be simplified by stripping
suffixes and minimizing while maintaining preference order.

```rust
use regex_syntax::hir::literal::{Literal, Seq};

let mut seq = Seq::new(&[
    "farm",
    "appliance",
    "faraway",
    "apple",
    "fare",
    "gap",
    "applicant",
    "applaud",
]);
seq.keep_first_bytes(3);
seq.minimize_by_preference();
// Notice that 'far' comes before 'app', which matches the order in the
// original sequence. This guarantees that leftmost-first semantics are
// not altered by simplifying the set.
let expected = Seq::from_iter([
    Literal::inexact("far"),
    Literal::inexact("app"),
    Literal::exact("gap"),
]);
assert_eq!(expected, seq);
```

#### Fields

- **`literals`**: `Option<alloc::vec::Vec<Literal>>`

  The members of this seq.
  
  When `None`, the seq represents all possible literals. That is, it
  prevents one from making assumptions about specific literals in the
  seq, and forces one to treat it as if any literal might be in the seq.
  
  Note that `Some(vec![])` is valid and corresponds to the empty seq of
  literals, i.e., a regex that can never match. For example, `[a&&b]`.
  It is distinct from `Some(vec![""])`, which corresponds to the seq
  containing an empty string, which matches at every position.

#### Implementations

- <span id="seq-empty"></span>`fn empty() -> Seq` — [`Seq`](#seq)

  Returns an empty sequence.

  

  An empty sequence matches zero literals, and thus corresponds to a

  regex that itself can never match.

- <span id="seq-infinite"></span>`fn infinite() -> Seq` — [`Seq`](#seq)

  Returns a sequence of literals without a finite size and may contain

  any literal.

  

  A sequence without finite size does not reveal anything about the

  characteristics of the literals in its set. There are no fixed prefixes

  or suffixes, nor are lower or upper bounds on the length of the literals

  in the set known.

  

  This is useful to represent constructs in a regex that are "too big"

  to useful represent as a sequence of literals. For example, `[A-Za-z]`.

  When sequences get too big, they lose their discriminating nature and

  are more likely to produce false positives, which in turn makes them

  less likely to speed up searches.

  

  More pragmatically, for many regexes, enumerating all possible literals

  is itself not possible or might otherwise use too many resources. So

  constraining the size of sets during extraction is a practical trade

  off to make.

- <span id="seq-singleton"></span>`fn singleton(lit: Literal) -> Seq` — [`Literal`](#literal), [`Seq`](#seq)

  Returns a sequence containing a single literal.

- <span id="seq-new"></span>`fn new<I, B>(it: I) -> Seq` — [`Seq`](#seq)

  Returns a sequence of exact literals from the given byte strings.

- <span id="seq-literals"></span>`fn literals(&self) -> Option<&[Literal]>` — [`Literal`](#literal)

  If this is a finite sequence, return its members as a slice of

  literals.

  

  The slice returned may be empty, in which case, there are no literals

  that can match this sequence.

- <span id="seq-push"></span>`fn push(&mut self, lit: Literal)` — [`Literal`](#literal)

  Push a literal to the end of this sequence.

  

  If this sequence is not finite, then this is a no-op.

  

  Similarly, if the most recently added item of this sequence is

  equivalent to the literal given, then it is not added. This reflects

  a `Seq`'s "set like" behavior, and represents a practical trade off.

  Namely, there is never any need to have two adjacent and equivalent

  literals in the same sequence, _and_ it is easy to detect in some

  cases.

- <span id="seq-make-inexact"></span>`fn make_inexact(&mut self)`

  Make all of the literals in this sequence inexact.

  

  This is a no-op if this sequence is not finite.

- <span id="seq-make-infinite"></span>`fn make_infinite(&mut self)`

  Converts this sequence to an infinite sequence.

  

  This is a no-op if the sequence is already infinite.

- <span id="seq-cross-forward"></span>`fn cross_forward(&mut self, other: &mut Seq)` — [`Seq`](#seq)

  Modify this sequence to contain the cross product between it and the

  sequence given.

  

  The cross product only considers literals in this sequence that are

  exact. That is, inexact literals are not extended.

  

  The literals are always drained from `other`, even if none are used.

  This permits callers to reuse the sequence allocation elsewhere.

  

  If this sequence is infinite, then this is a no-op, regardless of what

  `other` contains (and in this case, the literals are still drained from

  `other`). If `other` is infinite and this sequence is finite, then this

  is a no-op, unless this sequence contains a zero-length literal. In

  which case, the infiniteness of `other` infects this sequence, and this

  sequence is itself made infinite.

  

  Like `Seq::union`, this may attempt to deduplicate literals. See

  `Seq::dedup` for how deduplication deals with exact and inexact

  literals.

  

  # Example

  

  This example shows basic usage and how exact and inexact literals

  interact.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("bar"),

  ]);

  let mut seq2 = Seq::from_iter([

      Literal::inexact("quux"),

      Literal::exact("baz"),

  ]);

  seq1.cross_forward(&mut seq2);

  

  // The literals are pulled out of seq2.

  assert_eq!(Some(0), seq2.len());

  

  let expected = Seq::from_iter([

      Literal::inexact("fooquux"),

      Literal::exact("foobaz"),

      Literal::inexact("bar"),

  ]);

  assert_eq!(expected, seq1);

  ```

  

  This example shows the behavior of when `other` is an infinite

  sequence.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("bar"),

  ]);

  let mut seq2 = Seq::infinite();

  seq1.cross_forward(&mut seq2);

  

  // When seq2 is infinite, cross product doesn't add anything, but

  // ensures all members of seq1 are inexact.

  let expected = Seq::from_iter([

      Literal::inexact("foo"),

      Literal::inexact("bar"),

  ]);

  assert_eq!(expected, seq1);

  ```

  

  This example is like the one above, but shows what happens when this

  sequence contains an empty string. In this case, an infinite `other`

  sequence infects this sequence (because the empty string means that

  there are no finite prefixes):

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::exact(""), // inexact provokes same behavior

      Literal::inexact("bar"),

  ]);

  let mut seq2 = Seq::infinite();

  seq1.cross_forward(&mut seq2);

  

  // seq1 is now infinite!

  assert!(!seq1.is_finite());

  ```

  

  This example shows the behavior of this sequence is infinite.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::infinite();

  let mut seq2 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("bar"),

  ]);

  seq1.cross_forward(&mut seq2);

  

  // seq1 remains unchanged.

  assert!(!seq1.is_finite());

  // Even though the literals in seq2 weren't used, it was still drained.

  assert_eq!(Some(0), seq2.len());

  ```

- <span id="seq-cross-reverse"></span>`fn cross_reverse(&mut self, other: &mut Seq)` — [`Seq`](#seq)

  Modify this sequence to contain the cross product between it and

  the sequence given, where the sequences are treated as suffixes

  instead of prefixes. Namely, the sequence `other` is *prepended*

  to `self` (as opposed to `other` being *appended* to `self` in

  `Seq::cross_forward`).

  

  The cross product only considers literals in this sequence that are

  exact. That is, inexact literals are not extended.

  

  The literals are always drained from `other`, even if none are used.

  This permits callers to reuse the sequence allocation elsewhere.

  

  If this sequence is infinite, then this is a no-op, regardless of what

  `other` contains (and in this case, the literals are still drained from

  `other`). If `other` is infinite and this sequence is finite, then this

  is a no-op, unless this sequence contains a zero-length literal. In

  which case, the infiniteness of `other` infects this sequence, and this

  sequence is itself made infinite.

  

  Like `Seq::union`, this may attempt to deduplicate literals. See

  `Seq::dedup` for how deduplication deals with exact and inexact

  literals.

  

  # Example

  

  This example shows basic usage and how exact and inexact literals

  interact.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("bar"),

  ]);

  let mut seq2 = Seq::from_iter([

      Literal::inexact("quux"),

      Literal::exact("baz"),

  ]);

  seq1.cross_reverse(&mut seq2);

  

  // The literals are pulled out of seq2.

  assert_eq!(Some(0), seq2.len());

  

  let expected = Seq::from_iter([

      Literal::inexact("quuxfoo"),

      Literal::inexact("bar"),

      Literal::exact("bazfoo"),

  ]);

  assert_eq!(expected, seq1);

  ```

  

  This example shows the behavior of when `other` is an infinite

  sequence.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("bar"),

  ]);

  let mut seq2 = Seq::infinite();

  seq1.cross_reverse(&mut seq2);

  

  // When seq2 is infinite, cross product doesn't add anything, but

  // ensures all members of seq1 are inexact.

  let expected = Seq::from_iter([

      Literal::inexact("foo"),

      Literal::inexact("bar"),

  ]);

  assert_eq!(expected, seq1);

  ```

  

  This example is like the one above, but shows what happens when this

  sequence contains an empty string. In this case, an infinite `other`

  sequence infects this sequence (because the empty string means that

  there are no finite suffixes):

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::exact(""), // inexact provokes same behavior

      Literal::inexact("bar"),

  ]);

  let mut seq2 = Seq::infinite();

  seq1.cross_reverse(&mut seq2);

  

  // seq1 is now infinite!

  assert!(!seq1.is_finite());

  ```

  

  This example shows the behavior when this sequence is infinite.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq1 = Seq::infinite();

  let mut seq2 = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("bar"),

  ]);

  seq1.cross_reverse(&mut seq2);

  

  // seq1 remains unchanged.

  assert!(!seq1.is_finite());

  // Even though the literals in seq2 weren't used, it was still drained.

  assert_eq!(Some(0), seq2.len());

  ```

- <span id="seq-cross-preamble"></span>`fn cross_preamble<'a>(self: &'a mut Self, other: &'a mut Seq) -> Option<(&'a mut Vec<Literal>, &'a mut Vec<Literal>)>` — [`Seq`](#seq), [`Literal`](#literal)

  A helper function the corresponds to the subtle preamble for both

  `cross_forward` and `cross_reverse`. In effect, it handles the cases

  of infinite sequences for both `self` and `other`, as well as ensuring

  that literals from `other` are drained even if they aren't used.

- <span id="seq-union"></span>`fn union(&mut self, other: &mut Seq)` — [`Seq`](#seq)

  Unions the `other` sequence into this one.

  

  The literals are always drained out of the given `other` sequence,

  even if they are being unioned into an infinite sequence. This permits

  the caller to reuse the `other` sequence in another context.

  

  Some literal deduping may be performed. If any deduping happens,

  any leftmost-first or "preference" order match semantics will be

  preserved.

  

  # Example

  

  This example shows basic usage.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let mut seq1 = Seq::new(&["foo", "bar"]);

  let mut seq2 = Seq::new(&["bar", "quux", "foo"]);

  seq1.union(&mut seq2);

  

  // The literals are pulled out of seq2.

  assert_eq!(Some(0), seq2.len());

  

  // Adjacent literals are deduped, but non-adjacent literals may not be.

  assert_eq!(Seq::new(&["foo", "bar", "quux", "foo"]), seq1);

  ```

  

  This example shows that literals are drained from `other` even when

  they aren't necessarily used.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let mut seq1 = Seq::infinite();

  // Infinite sequences have no finite length.

  assert_eq!(None, seq1.len());

  

  let mut seq2 = Seq::new(&["bar", "quux", "foo"]);

  seq1.union(&mut seq2);

  

  // seq1 is still infinite and seq2 has been drained.

  assert_eq!(None, seq1.len());

  assert_eq!(Some(0), seq2.len());

  ```

- <span id="seq-union-into-empty"></span>`fn union_into_empty(&mut self, other: &mut Seq)` — [`Seq`](#seq)

  Unions the `other` sequence into this one by splice the `other`

  sequence at the position of the first zero-length literal.

  

  This is useful for preserving preference order semantics when combining

  two literal sequences. For example, in the regex `(a||f)+foo`, the

  correct preference order prefix sequence is `[a, foo, f]`.

  

  The literals are always drained out of the given `other` sequence,

  even if they are being unioned into an infinite sequence. This permits

  the caller to reuse the `other` sequence in another context. Note that

  the literals are drained even if no union is performed as well, i.e.,

  when this sequence does not contain a zero-length literal.

  

  Some literal deduping may be performed. If any deduping happens,

  any leftmost-first or "preference" order match semantics will be

  preserved.

  

  # Example

  

  This example shows basic usage.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let mut seq1 = Seq::new(&["a", "", "f", ""]);

  let mut seq2 = Seq::new(&["foo"]);

  seq1.union_into_empty(&mut seq2);

  

  // The literals are pulled out of seq2.

  assert_eq!(Some(0), seq2.len());

  // 'foo' gets spliced into seq1 where the first empty string occurs.

  assert_eq!(Seq::new(&["a", "foo", "f"]), seq1);

  ```

  

  This example shows that literals are drained from `other` even when

  they aren't necessarily used.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let mut seq1 = Seq::new(&["foo", "bar"]);

  let mut seq2 = Seq::new(&["bar", "quux", "foo"]);

  seq1.union_into_empty(&mut seq2);

  

  // seq1 has no zero length literals, so no splicing happens.

  assert_eq!(Seq::new(&["foo", "bar"]), seq1);

  // Even though no splicing happens, seq2 is still drained.

  assert_eq!(Some(0), seq2.len());

  ```

- <span id="seq-dedup"></span>`fn dedup(&mut self)`

  Deduplicate adjacent equivalent literals in this sequence.

  

  If adjacent literals are equivalent strings but one is exact and the

  other inexact, the inexact literal is kept and the exact one is

  removed.

  

  Deduping an infinite sequence is a no-op.

  

  # Example

  

  This example shows how literals that are duplicate byte strings but

  are not equivalent with respect to exactness are resolved.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq = Seq::from_iter([

      Literal::exact("foo"),

      Literal::inexact("foo"),

  ]);

  seq.dedup();

  

  assert_eq!(Seq::from_iter([Literal::inexact("foo")]), seq);

  ```

- <span id="seq-sort"></span>`fn sort(&mut self)`

  Sorts this sequence of literals lexicographically.

  

  Note that if, before sorting, if a literal that is a prefix of another

  literal appears after it, then after sorting, the sequence will not

  represent the same preference order match semantics. For example,

  sorting the sequence `[samwise, sam]` yields the sequence `[sam,

  samwise]`. Under preference order semantics, the latter sequence will

  never match `samwise` where as the first sequence can.

  

  # Example

  

  This example shows basic usage.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let mut seq = Seq::new(&["foo", "quux", "bar"]);

  seq.sort();

  

  assert_eq!(Seq::new(&["bar", "foo", "quux"]), seq);

  ```

- <span id="seq-reverse-literals"></span>`fn reverse_literals(&mut self)`

  Reverses all of the literals in this sequence.

  

  The order of the sequence itself is preserved.

  

  # Example

  

  This example shows basic usage.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let mut seq = Seq::new(&["oof", "rab"]);

  seq.reverse_literals();

  assert_eq!(Seq::new(&["foo", "bar"]), seq);

  ```

- <span id="seq-minimize-by-preference"></span>`fn minimize_by_preference(&mut self)`

  Shrinks this seq to its minimal size while respecting the preference

  order of its literals.

  

  While this routine will remove duplicate literals from this seq, it

  will also remove literals that can never match in a leftmost-first or

  "preference order" search. Similar to `Seq::dedup`, if a literal is

  deduped, then the one that remains is made inexact.

  

  This is a no-op on seqs that are empty or not finite.

  

  # Example

  

  This example shows the difference between `{sam, samwise}` and

  `{samwise, sam}`.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  // If 'sam' comes before 'samwise' and a preference order search is

  // executed, then 'samwise' can never match.

  let mut seq = Seq::new(&["sam", "samwise"]);

  seq.minimize_by_preference();

  assert_eq!(Seq::from_iter([Literal::inexact("sam")]), seq);

  

  // But if they are reversed, then it's possible for 'samwise' to match

  // since it is given higher preference.

  let mut seq = Seq::new(&["samwise", "sam"]);

  seq.minimize_by_preference();

  assert_eq!(Seq::new(&["samwise", "sam"]), seq);

  ```

  

  This example shows that if an empty string is in this seq, then

  anything that comes after it can never match.

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  // An empty string is a prefix of all strings, so it automatically

  // inhibits any subsequent strings from matching.

  let mut seq = Seq::new(&["foo", "bar", "", "quux", "fox"]);

  seq.minimize_by_preference();

  let expected = Seq::from_iter([

      Literal::exact("foo"),

      Literal::exact("bar"),

      Literal::inexact(""),

  ]);

  assert_eq!(expected, seq);

  

  // And of course, if it's at the beginning, then it makes it impossible

  // for anything else to match.

  let mut seq = Seq::new(&["", "foo", "quux", "fox"]);

  seq.minimize_by_preference();

  assert_eq!(Seq::from_iter([Literal::inexact("")]), seq);

  ```

- <span id="seq-keep-first-bytes"></span>`fn keep_first_bytes(&mut self, len: usize)`

  Trims all literals in this seq such that only the first `len` bytes

  remain. If a literal has less than or equal to `len` bytes, then it

  remains unchanged. Otherwise, it is trimmed and made inexact.

  

  # Example

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq = Seq::new(&["a", "foo", "quux"]);

  seq.keep_first_bytes(2);

  

  let expected = Seq::from_iter([

      Literal::exact("a"),

      Literal::inexact("fo"),

      Literal::inexact("qu"),

  ]);

  assert_eq!(expected, seq);

  ```

- <span id="seq-keep-last-bytes"></span>`fn keep_last_bytes(&mut self, len: usize)`

  Trims all literals in this seq such that only the last `len` bytes

  remain. If a literal has less than or equal to `len` bytes, then it

  remains unchanged. Otherwise, it is trimmed and made inexact.

  

  # Example

  

  ```rust

  use regex_syntax::hir::literal::{Literal, Seq};

  

  let mut seq = Seq::new(&["a", "foo", "quux"]);

  seq.keep_last_bytes(2);

  

  let expected = Seq::from_iter([

      Literal::exact("a"),

      Literal::inexact("oo"),

      Literal::inexact("ux"),

  ]);

  assert_eq!(expected, seq);

  ```

- <span id="seq-is-finite"></span>`fn is_finite(&self) -> bool`

  Returns true if this sequence is finite.

  

  When false, this sequence is infinite and must be treated as if it

  contains every possible literal.

- <span id="seq-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this sequence is finite and empty.

  

  An empty sequence never matches anything. It can only be produced by

  literal extraction when the corresponding regex itself cannot match.

- <span id="seq-len"></span>`fn len(&self) -> Option<usize>`

  Returns the number of literals in this sequence if the sequence is

  finite. If the sequence is infinite, then `None` is returned.

- <span id="seq-is-exact"></span>`fn is_exact(&self) -> bool`

  Returns true if and only if all literals in this sequence are exact.

  

  This returns false if the sequence is infinite.

- <span id="seq-is-inexact"></span>`fn is_inexact(&self) -> bool`

  Returns true if and only if all literals in this sequence are inexact.

  

  This returns true if the sequence is infinite.

- <span id="seq-max-union-len"></span>`fn max_union_len(&self, other: &Seq) -> Option<usize>` — [`Seq`](#seq)

  Return the maximum length of the sequence that would result from

  unioning `self` with `other`. If either set is infinite, then this

  returns `None`.

- <span id="seq-max-cross-len"></span>`fn max_cross_len(&self, other: &Seq) -> Option<usize>` — [`Seq`](#seq)

  Return the maximum length of the sequence that would result from the

  cross product of `self` with `other`. If either set is infinite, then

  this returns `None`.

- <span id="seq-min-literal-len"></span>`fn min_literal_len(&self) -> Option<usize>`

  Returns the length of the shortest literal in this sequence.

  

  If the sequence is infinite or empty, then this returns `None`.

- <span id="seq-max-literal-len"></span>`fn max_literal_len(&self) -> Option<usize>`

  Returns the length of the longest literal in this sequence.

  

  If the sequence is infinite or empty, then this returns `None`.

- <span id="seq-longest-common-prefix"></span>`fn longest_common_prefix(&self) -> Option<&[u8]>`

  Returns the longest common prefix from this seq.

  

  If the seq matches any literal or other contains no literals, then

  there is no meaningful prefix and this returns `None`.

  

  # Example

  

  This shows some example seqs and their longest common prefix.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let seq = Seq::new(&["foo", "foobar", "fo"]);

  assert_eq!(Some(&b"fo"[..]), seq.longest_common_prefix());

  let seq = Seq::new(&["foo", "foo"]);

  assert_eq!(Some(&b"foo"[..]), seq.longest_common_prefix());

  let seq = Seq::new(&["foo", "bar"]);

  assert_eq!(Some(&b""[..]), seq.longest_common_prefix());

  let seq = Seq::new(&[""]);

  assert_eq!(Some(&b""[..]), seq.longest_common_prefix());

  

  let seq = Seq::infinite();

  assert_eq!(None, seq.longest_common_prefix());

  let seq = Seq::empty();

  assert_eq!(None, seq.longest_common_prefix());

  ```

- <span id="seq-longest-common-suffix"></span>`fn longest_common_suffix(&self) -> Option<&[u8]>`

  Returns the longest common suffix from this seq.

  

  If the seq matches any literal or other contains no literals, then

  there is no meaningful suffix and this returns `None`.

  

  # Example

  

  This shows some example seqs and their longest common suffix.

  

  ```rust

  use regex_syntax::hir::literal::Seq;

  

  let seq = Seq::new(&["oof", "raboof", "of"]);

  assert_eq!(Some(&b"of"[..]), seq.longest_common_suffix());

  let seq = Seq::new(&["foo", "foo"]);

  assert_eq!(Some(&b"foo"[..]), seq.longest_common_suffix());

  let seq = Seq::new(&["foo", "bar"]);

  assert_eq!(Some(&b""[..]), seq.longest_common_suffix());

  let seq = Seq::new(&[""]);

  assert_eq!(Some(&b""[..]), seq.longest_common_suffix());

  

  let seq = Seq::infinite();

  assert_eq!(None, seq.longest_common_suffix());

  let seq = Seq::empty();

  assert_eq!(None, seq.longest_common_suffix());

  ```

- <span id="seq-optimize-for-prefix-by-preference"></span>`fn optimize_for_prefix_by_preference(&mut self)`

  Optimizes this seq while treating its literals as prefixes and

  respecting the preference order of its literals.

  

  The specific way "optimization" works is meant to be an implementation

  detail, as it essentially represents a set of heuristics. The goal

  that optimization tries to accomplish is to make the literals in this

  set reflect inputs that will result in a more effective prefilter.

  Principally by reducing the false positive rate of candidates found by

  the literals in this sequence. That is, when a match of a literal is

  found, we would like it to be a strong predictor of the overall match

  of the regex. If it isn't, then much time will be spent starting and

  stopping the prefilter search and attempting to confirm the match only

  to have it fail.

  

  Some of those heuristics might be:

  

  * Identifying a common prefix from a larger sequence of literals, and

  shrinking the sequence down to that single common prefix.

  * Rejecting the sequence entirely if it is believed to result in very

  high false positive rate. When this happens, the sequence is made

  infinite.

  * Shrinking the sequence to a smaller number of literals representing

  prefixes, but not shrinking it so much as to make literals too short.

  (A sequence with very short literals, of 1 or 2 bytes, will typically

  result in a higher false positive rate.)

  

  Optimization should only be run once extraction is complete. Namely,

  optimization may make assumptions that do not compose with other

  operations in the middle of extraction. For example, optimization will

  reduce `[E(sam), E(samwise)]` to `[E(sam)]`, but such a transformation

  is only valid if no other extraction will occur. If other extraction

  may occur, then the correct transformation would be to `[I(sam)]`.

  

  The `Seq::optimize_for_suffix_by_preference` does the same thing, but

  for suffixes.

  

  # Example

  

  This shows how optimization might transform a sequence. Note that

  the specific behavior is not a documented guarantee. The heuristics

  used are an implementation detail and may change over time in semver

  compatible releases.

  

  ```rust

  use regex_syntax::hir::literal::{Seq, Literal};

  

  let mut seq = Seq::new(&[

      "samantha",

      "sam",

      "samwise",

      "frodo",

  ]);

  seq.optimize_for_prefix_by_preference();

  assert_eq!(Seq::from_iter([

      Literal::exact("samantha"),

      // Kept exact even though 'samwise' got pruned

      // because optimization assumes literal extraction

      // has finished.

      Literal::exact("sam"),

      Literal::exact("frodo"),

  ]), seq);

  ```

  

  # Example: optimization may make the sequence infinite

  

  If the heuristics deem that the sequence could cause a very high false

  positive rate, then it may make the sequence infinite, effectively

  disabling its use as a prefilter.

  

  ```rust

  use regex_syntax::hir::literal::{Seq, Literal};

  

  let mut seq = Seq::new(&[

      "samantha",

      // An empty string matches at every position,

      // thus rendering the prefilter completely

      // ineffective.

      "",

      "sam",

      "samwise",

      "frodo",

  ]);

  seq.optimize_for_prefix_by_preference();

  assert!(!seq.is_finite());

  ```

  

  Do note that just because there is a `" "` in the sequence, that

  doesn't mean the sequence will always be made infinite after it is

  optimized. Namely, if the sequence is considered exact (any match

  corresponds to an overall match of the original regex), then any match

  is an overall match, and so the false positive rate is always `0`.

  

  To demonstrate this, we remove `samwise` from our sequence. This

  results in no optimization happening and all literals remain exact.

  Thus the entire sequence is exact, and it is kept as-is, even though

  one is an ASCII space:

  

  ```rust

  use regex_syntax::hir::literal::{Seq, Literal};

  

  let mut seq = Seq::new(&[

      "samantha",

      " ",

      "sam",

      "frodo",

  ]);

  seq.optimize_for_prefix_by_preference();

  assert!(seq.is_finite());

  ```

- <span id="seq-optimize-for-suffix-by-preference"></span>`fn optimize_for_suffix_by_preference(&mut self)`

  Optimizes this seq while treating its literals as suffixes and

  respecting the preference order of its literals.

  

  Optimization should only be run once extraction is complete.

  

  The `Seq::optimize_for_prefix_by_preference` does the same thing, but

  for prefixes. See its documentation for more explanation.

- <span id="seq-optimize-by-preference"></span>`fn optimize_by_preference(&mut self, prefix: bool)`

#### Trait Implementations

##### `impl Any for Seq`

- <span id="seq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Seq`

- <span id="seq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Seq`

- <span id="seq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Seq`

- <span id="seq-clone"></span>`fn clone(&self) -> Seq` — [`Seq`](#seq)

##### `impl CloneToUninit for Seq`

- <span id="seq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Seq`

- <span id="seq-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Seq`

##### `impl<T> From for Seq`

- <span id="seq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Seq`

- <span id="seq-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq` — [`Seq`](#seq)

##### `impl<U> Into for Seq`

- <span id="seq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Seq`

- <span id="seq-partialeq-eq"></span>`fn eq(&self, other: &Seq) -> bool` — [`Seq`](#seq)

##### `impl StructuralPartialEq for Seq`

##### `impl ToOwned for Seq`

- <span id="seq-toowned-type-owned"></span>`type Owned = T`

- <span id="seq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="seq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Seq`

- <span id="seq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Seq`

- <span id="seq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Literal`

```rust
struct Literal {
    bytes: alloc::vec::Vec<u8>,
    exact: bool,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2049-2052`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L2049-L2052)*

A single literal extracted from an [`Hir`](../index.md) expression.

A literal is composed of two things:

* A sequence of bytes. No guarantees with respect to UTF-8 are provided.
In particular, even if the regex a literal is extracted from is UTF-8, the
literal extracted may not be valid UTF-8. (For example, if an [`Extractor`](#extractor)
limit resulted in trimming a literal in a way that splits a codepoint.)
* Whether the literal is "exact" or not. An "exact" literal means that it
has not been trimmed, and may continue to be extended. If a literal is
"exact" after visiting the entire `Hir` expression, then this implies that
the literal leads to a match state. (Although it doesn't necessarily imply
all occurrences of the literal correspond to a match of the regex, since
literal extraction ignores look-around assertions.)

#### Implementations

- <span id="literal-exact"></span>`fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal` — [`Literal`](#literal)

  Returns a new exact literal containing the bytes given.

- <span id="literal-inexact"></span>`fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal` — [`Literal`](#literal)

  Returns a new inexact literal containing the bytes given.

- <span id="literal-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

  Returns the bytes in this literal.

- <span id="literal-into-bytes"></span>`fn into_bytes(self) -> Vec<u8>`

  Yields ownership of the bytes inside this literal.

  

  Note that this throws away whether the literal is "exact" or not.

- <span id="literal-len"></span>`fn len(&self) -> usize`

  Returns the length of this literal in bytes.

- <span id="literal-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this literal has zero bytes.

- <span id="literal-is-exact"></span>`fn is_exact(&self) -> bool`

  Returns true if and only if this literal is exact.

- <span id="literal-make-inexact"></span>`fn make_inexact(&mut self)`

  Marks this literal as inexact.

  

  Inexact literals can never be extended. For example,

  `Seq::cross_forward` will not extend inexact literals.

- <span id="literal-reverse"></span>`fn reverse(&mut self)`

  Reverse the bytes in this literal.

- <span id="literal-extend"></span>`fn extend(&mut self, lit: &Literal)` — [`Literal`](#literal)

  Extend this literal with the literal given.

  

  If this literal is inexact, then this is a no-op.

- <span id="literal-keep-first-bytes"></span>`fn keep_first_bytes(&mut self, len: usize)`

  Trims this literal such that only the first `len` bytes remain. If

  this literal has fewer than `len` bytes, then it remains unchanged.

  Otherwise, the literal is marked as inexact.

- <span id="literal-keep-last-bytes"></span>`fn keep_last_bytes(&mut self, len: usize)`

  Trims this literal such that only the last `len` bytes remain. If this

  literal has fewer than `len` bytes, then it remains unchanged.

  Otherwise, the literal is marked as inexact.

- <span id="literal-is-poisonous"></span>`fn is_poisonous(&self) -> bool`

  Returns true if it is believe that this literal is likely to match very

  frequently, and is thus not a good candidate for a prefilter.

#### Trait Implementations

##### `impl Any for Literal`

- <span id="literal-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Literal`

- <span id="literal-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl<T> Borrow for Literal`

- <span id="literal-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Literal`

- <span id="literal-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Literal`

- <span id="literal-clone"></span>`fn clone(&self) -> Literal` — [`Literal`](#literal)

##### `impl CloneToUninit for Literal`

- <span id="literal-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Literal`

- <span id="literal-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Literal`

##### `impl<T> From for Literal`

- <span id="literal-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for Seq`

- <span id="seq-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq` — [`Seq`](#seq)

##### `impl<U> Into for Literal`

- <span id="literal-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Literal`

- <span id="literal-ord-cmp"></span>`fn cmp(&self, other: &Literal) -> cmp::Ordering` — [`Literal`](#literal)

##### `impl PartialEq for Literal`

- <span id="literal-partialeq-eq"></span>`fn eq(&self, other: &Literal) -> bool` — [`Literal`](#literal)

##### `impl PartialOrd for Literal`

- <span id="literal-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Literal) -> option::Option<cmp::Ordering>` — [`Literal`](#literal)

##### `impl StructuralPartialEq for Literal`

##### `impl ToOwned for Literal`

- <span id="literal-toowned-type-owned"></span>`type Owned = T`

- <span id="literal-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="literal-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Literal`

- <span id="literal-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="literal-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Literal`

- <span id="literal-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="literal-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PreferenceTrie`

```rust
struct PreferenceTrie {
    states: alloc::vec::Vec<State>,
    matches: alloc::vec::Vec<Option<core::num::NonZeroUsize>>,
    next_literal_index: usize,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2203-2216`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L2203-L2216)*

A "preference" trie that rejects literals that will never match when
executing a leftmost first or "preference" search.

For example, if 'sam' is inserted, then trying to insert 'samwise' will be
rejected because 'samwise' can never match since 'sam' will always take
priority. However, if 'samwise' is inserted first, then inserting 'sam'
after it is accepted. In this case, either 'samwise' or 'sam' can match in
a "preference" search.

Note that we only use this trie as a "set." That is, given a sequence of
literals, we insert each one in order. An `insert` will reject a literal
if a prefix of that literal already exists in the trie. Thus, to rebuild
the "minimal" sequence, we simply only keep literals that were successfully
inserted. (Since we don't need traversal, one wonders whether we can make
some simplifications here, but I haven't given it a ton of thought and I've
never seen this show up on a profile. Because of the heuristic limits
imposed on literal extractions, the size of the inputs here is usually
very small.)

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  The states in this trie. The index of a state in this vector is its ID.

- **`matches`**: `alloc::vec::Vec<Option<core::num::NonZeroUsize>>`

  This vec indicates which states are match states. It always has
  the same length as `states` and is indexed by the same state ID.
  A state with identifier `sid` is a match state if and only if
  `matches[sid].is_some()`. The option contains the index of the literal
  corresponding to the match. The index is offset by 1 so that it fits in
  a NonZeroUsize.

- **`next_literal_index`**: `usize`

  The index to allocate to the next literal added to this trie. Starts at
  1 and increments by 1 for every literal successfully added to the trie.

#### Implementations

- <span id="preferencetrie-minimize"></span>`fn minimize(literals: &mut Vec<Literal>, keep_exact: bool)` — [`Literal`](#literal)

  Minimizes the given sequence of literals while preserving preference

  order semantics.

  

  When `keep_exact` is true, the exactness of every literal retained is

  kept. This is useful when dealing with a fully extracted `Seq` that

  only contains exact literals. In that case, we can keep all retained

  literals as exact because we know we'll never need to match anything

  after them and because any removed literals are guaranteed to never

  match.

- <span id="preferencetrie-insert"></span>`fn insert(&mut self, bytes: &[u8]) -> Result<usize, usize>`

  Returns `Ok` if the given byte string is accepted into this trie and

  `Err` otherwise. The index for the success case corresponds to the

  index of the literal added. The index for the error case corresponds to

  the index of the literal already in the trie that prevented the given

  byte string from being added. (Which implies it is a prefix of the one

  given.)

  

  In short, the byte string given is accepted into the trie if and only

  if it is possible for it to match when executing a preference order

  search.

- <span id="preferencetrie-root"></span>`fn root(&mut self) -> usize`

  Returns the root state ID, and if it doesn't exist, creates it.

- <span id="preferencetrie-create-state"></span>`fn create_state(&mut self) -> usize`

  Creates a new empty state and returns its ID.

#### Trait Implementations

##### `impl Any for PreferenceTrie`

- <span id="preferencetrie-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PreferenceTrie`

- <span id="preferencetrie-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PreferenceTrie`

- <span id="preferencetrie-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PreferenceTrie`

- <span id="preferencetrie-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PreferenceTrie`

- <span id="preferencetrie-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PreferenceTrie`

- <span id="preferencetrie-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for PreferenceTrie`

- <span id="preferencetrie-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="preferencetrie-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PreferenceTrie`

- <span id="preferencetrie-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="preferencetrie-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `State`

```rust
struct State {
    trans: alloc::vec::Vec<(u8, usize)>,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2220-2225`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L2220-L2225)*

A single state in a trie. Uses a sparse representation for its transitions.

#### Fields

- **`trans`**: `alloc::vec::Vec<(u8, usize)>`

  Sparse representation of the transitions out of this state. Transitions
  are sorted by byte. There is at most one such transition for any
  particular byte.

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> State` — [`State`](#state)

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ExtractKind`

```rust
enum ExtractKind {
    Prefix,
    Suffix,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:640-649`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L640-L649)*

The kind of literals to extract from an [`Hir`](../index.md) expression.

The default extraction kind is `Prefix`.

#### Variants

- **`Prefix`**

  Extracts only prefix literals from a regex.

- **`Suffix`**

  Extracts only suffix literals from a regex.
  
  Note that the sequence returned by suffix literals currently may
  not correctly represent leftmost-first or "preference" order match
  semantics.

#### Implementations

- <span id="extractkind-is-prefix"></span>`fn is_prefix(&self) -> bool`

  Returns true if this kind is the `Prefix` variant.

- <span id="extractkind-is-suffix"></span>`fn is_suffix(&self) -> bool`

  Returns true if this kind is the `Suffix` variant.

#### Trait Implementations

##### `impl Any for ExtractKind`

- <span id="extractkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExtractKind`

- <span id="extractkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExtractKind`

- <span id="extractkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ExtractKind`

- <span id="extractkind-clone"></span>`fn clone(&self) -> ExtractKind` — [`ExtractKind`](#extractkind)

##### `impl CloneToUninit for ExtractKind`

- <span id="extractkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ExtractKind`

- <span id="extractkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ExtractKind`

- <span id="extractkind-default"></span>`fn default() -> ExtractKind` — [`ExtractKind`](#extractkind)

##### `impl<T> From for ExtractKind`

- <span id="extractkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExtractKind`

- <span id="extractkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ExtractKind`

- <span id="extractkind-toowned-type-owned"></span>`type Owned = T`

- <span id="extractkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="extractkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ExtractKind`

- <span id="extractkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extractkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExtractKind`

- <span id="extractkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extractkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `rank`

```rust
fn rank(byte: u8) -> u8
```

*Defined in [`regex-syntax-0.8.8/src/hir/literal.rs:2319-2321`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/literal.rs#L2319-L2321)*

Returns the "rank" of the given byte.

The minimum rank value is `0` and the maximum rank value is `255`.

The rank of a byte is derived from a heuristic background distribution of
relative frequencies of bytes. The heuristic says that lower the rank of a
byte, the less likely that byte is to appear in any arbitrary haystack.

