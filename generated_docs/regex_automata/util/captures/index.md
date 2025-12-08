*[regex_automata](../../index.md) / [util](../index.md) / [captures](index.md)*

---

# Module `captures`

Provides types for dealing with capturing groups.

Capturing groups refer to sub-patterns of regexes that some regex engines can
report matching offsets for. For example, matching `[a-z]([0-9]+)` against
`a789` would give `a789` as the overall match (for the implicit capturing group
at index `0`) and `789` as the match for the capturing group `([0-9]+)` (an
explicit capturing group at index `1`).

Not all regex engines can report match offsets for capturing groups. Indeed,
to a first approximation, regex engines that can report capturing group offsets
tend to be quite a bit slower than regex engines that can't. This is because
tracking capturing groups at search time usually requires more "power" that
in turn adds overhead.

Other regex implementations might call capturing groups "submatches."

# Overview

The main types in this module are:

* [`Captures`](#captures) records the capturing group offsets found during a search. It
provides convenience routines for looking up capturing group offsets by either
index or name.
* [`GroupInfo`](#groupinfo) records the mapping between capturing groups and "slots,"
where the latter are how capturing groups are recorded during a regex search.
This also keeps a mapping from capturing group name to index, and capture
group index to name. A `GroupInfo` is used by `Captures` internally to
provide a convenient API. It is unlikely that you'll use a `GroupInfo`
directly, but for example, if you've compiled an Thompson NFA, then you can use
[`thompson::NFA::group_info`](crate::nfa::thompson::NFA::group_info) to get its
underlying `GroupInfo`.

## Structs

### `Captures`

```rust
struct Captures {
    group_info: GroupInfo,
    pid: Option<crate::util::primitives::PatternID>,
    slots: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
}
```

The span offsets of capturing groups after a match has been found.

This type represents the output of regex engines that can report the
offsets at which capturing groups matches or "submatches" occur. For
example, the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). When a match
occurs, it will at minimum contain the [`PatternID`](../../index.md) of the pattern that
matched. Depending upon how it was constructed, it may also contain the
start/end offsets of the entire match of the pattern and the start/end
offsets of each capturing group that participated in the match.

Values of this type are always created for a specific [`GroupInfo`](#groupinfo). It is
unspecified behavior to use a `Captures` value in a search with any regex
engine that has a different `GroupInfo` than the one the `Captures` were
created with.

# Constructors

There are three constructors for this type that control what kind of
information is available upon a match:

* `Captures::all`: Will store overall pattern match offsets in addition
to the offsets of capturing groups that participated in the match.
* `Captures::matches`: Will store only the overall pattern
match offsets. The offsets of capturing groups (even ones that participated
in the match) are not available.
* `Captures::empty`: Will only store the pattern ID that matched. No
match offsets are available at all.

If you aren't sure which to choose, then pick the first one. The first one
is what convenience routines like,
[`PikeVM::create_captures`](crate::nfa::thompson::pikevm::PikeVM::create_captures),
will use automatically.

The main difference between these choices is performance. Namely, if you
ask for _less_ information, then the execution of regex search may be able
to run more quickly.

# Notes

It is worth pointing out that this type is not coupled to any one specific
regex engine. Instead, its coupling is with [`GroupInfo`](#groupinfo), which is the
thing that is responsible for mapping capturing groups to "slot" offsets.
Slot offsets are indices into a single sequence of memory at which matching
haystack offsets for the corresponding group are written by regex engines.

# Example

This example shows how to parse a simple date and extract the components of
the date via capturing groups:

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group(1));
assert_eq!(Some(Span::from(5..7)), caps.get_group(2));
assert_eq!(Some(Span::from(8..10)), caps.get_group(3));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: named capturing groups

This example is like the one above, but leverages the ability to name
capturing groups in order to make the code a bit clearer:

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"^(?P<y>[0-9]{4})-(?P<m>[0-9]{2})-(?P<d>[0-9]{2})$")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "2010-03-14", &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(0..4)), caps.get_group_by_name("y"));
assert_eq!(Some(Span::from(5..7)), caps.get_group_by_name("m"));
assert_eq!(Some(Span::from(8..10)), caps.get_group_by_name("d"));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`group_info`**: `GroupInfo`

  The group info that these capture groups are coupled to. This is what
  gives the "convenience" of the `Captures` API. Namely, it provides the
  slot mapping and the name|-->index mapping for capture lookups by name.

- **`pid`**: `Option<crate::util::primitives::PatternID>`

  The ID of the pattern that matched. Regex engines must set this to
  None when no match occurs.

- **`slots`**: `alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>`

  The slot values, i.e., submatch offsets.
  
  In theory, the smallest sequence of slots would be something like
  `max(groups(pattern) for pattern in regex) * 2`, but instead, we use
  `sum(groups(pattern) for pattern in regex) * 2`. Why?
  
  Well, the former could be used in theory, because we don't generally
  have any overlapping APIs that involve capturing groups. Therefore,
  there's technically never any need to have slots set for multiple
  patterns. However, this might change some day, in which case, we would
  need to have slots available.
  
  The other reason is that during the execution of some regex engines,
  there exists a point in time where multiple slots for different
  patterns may be written to before knowing which pattern has matched.
  Therefore, the regex engines themselves, in order to support multiple
  patterns correctly, must have all slots available. If `Captures`
  doesn't have all slots available, then regex engines can't write
  directly into the caller provided `Captures` and must instead write
  into some other storage and then copy the slots involved in the match
  at the end of the search.
  
  So overall, at least as of the time of writing, it seems like the path
  of least resistance is to just require allocating all possible slots
  instead of the conceptual minimum. Another way to justify this is that
  the most common case is a single pattern, in which case, there is no
  inefficiency here since the 'max' and 'sum' calculations above are
  equivalent in that case.
  
  N.B. The mapping from group index to slot is maintained by `GroupInfo`
  and is considered an API guarantee. See `GroupInfo` for more details on
  that mapping.
  
  N.B. `Option<NonMaxUsize>` has the same size as a `usize`.

#### Implementations

- `fn clear(self: &mut Self)`

- `fn set_pattern(self: &mut Self, pid: Option<PatternID>)` — [`PatternID`](../../index.md)

- `fn slots(self: &Self) -> &[Option<NonMaxUsize>]` — [`NonMaxUsize`](../primitives/index.md)

- `fn slots_mut(self: &mut Self) -> &mut [Option<NonMaxUsize>]` — [`NonMaxUsize`](../primitives/index.md)

#### Trait Implementations

##### `impl Clone for Captures`

- `fn clone(self: &Self) -> Captures` — [`Captures`](#captures)

##### `impl Debug for Captures`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CapturesDebugMap<'a>`

```rust
struct CapturesDebugMap<'a> {
    pid: crate::util::primitives::PatternID,
    caps: &'a Captures,
}
```

A little helper type to provide a nice map-like debug representation for
our capturing group spans.

#### Trait Implementations

##### `impl<'a> Debug for CapturesDebugMap<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `CapturesPatternIter<'a>`

```rust
struct CapturesPatternIter<'a> {
    caps: &'a Captures,
    names: core::iter::Enumerate<GroupInfoPatternNames<'a>>,
}
```

An iterator over all capturing groups in a `Captures` value.

This iterator includes capturing groups that did not participate in a
match. See the `Captures::iter` method documentation for more details
and examples.

The lifetime parameter `'a` refers to the lifetime of the underlying
`Captures` value.

#### Trait Implementations

##### `impl<'a> Clone for CapturesPatternIter<'a>`

- `fn clone(self: &Self) -> CapturesPatternIter<'a>` — [`CapturesPatternIter`](#capturespatterniter)

##### `impl<'a> Debug for CapturesPatternIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> ExactSizeIterator for CapturesPatternIter<'a>`

##### `impl<'a> FusedIterator for CapturesPatternIter<'a>`

##### `impl<I> IntoIterator for CapturesPatternIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for CapturesPatternIter<'a>`

- `type Item = Option<Span>`

- `fn next(self: &mut Self) -> Option<Option<Span>>` — [`Span`](../../index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn count(self: Self) -> usize`

### `GroupInfo`

```rust
struct GroupInfo(alloc::sync::Arc<GroupInfoInner>);
```

Represents information about capturing groups in a compiled regex.

The information encapsulated by this type consists of the following. For
each pattern:

* A map from every capture group name to its corresponding capture group
index.
* A map from every capture group index to its corresponding capture group
name.
* A map from capture group index to its corresponding slot index. A slot
refers to one half of a capturing group. That is, a capture slot is either
the start or end of a capturing group. A slot is usually the mechanism
by which a regex engine records offsets for each capturing group during a
search.

A `GroupInfo` uses reference counting internally and is thus cheap to
clone.

# Mapping from capture groups to slots

One of the main responsibilities of a `GroupInfo` is to build a mapping
from `(PatternID, u32)` (where the `u32` is a capture index) to something
called a "slot." As mentioned above, a slot refers to one half of a
capturing group. Both combined provide the start and end offsets of
a capturing group that participated in a match.

**The mapping between group indices and slots is an API guarantee.** That
is, the mapping won't change within a semver compatible release.

Slots exist primarily because this is a convenient mechanism by which
regex engines report group offsets at search time. For example, the
[`nfa::thompson::State::Capture`](crate::nfa::thompson::State::Capture)
NFA state includes the slot index. When a regex engine transitions through
this state, it will likely use the slot index to write the current haystack
offset to some region of memory. When a match is found, those slots are
then reported to the caller, typically via a convenient abstraction like a
[`Captures`](#captures) value.

Because this crate provides first class support for multi-pattern regexes,
and because of some performance related reasons, the mapping between
capturing groups and slots is a little complex. However, in the case of a
single pattern, the mapping can be described very simply: for all capture
group indices `i`, its corresponding slots are at `i * 2` and `i * 2 + 1`.
Notice that the pattern ID isn't involved at all here, because it only
applies to a single-pattern regex, it is therefore always `0`.

In the multi-pattern case, the mapping is a bit more complicated. To talk
about it, we must define what we mean by "implicit" vs "explicit"
capturing groups:

* An **implicit** capturing group refers to the capturing group that is
present for every pattern automatically, and corresponds to the overall
match of a pattern. Every pattern has precisely one implicit capturing
group. It is always unnamed and it always corresponds to the capture group
index `0`.
* An **explicit** capturing group refers to any capturing group that
appears in the concrete syntax of the pattern. (Or, if an NFA was hand
built without any concrete syntax, it refers to any capturing group with an
index greater than `0`.)

Some examples:

* `\w+` has one implicit capturing group and zero explicit capturing
groups.
* `(\w+)` has one implicit group and one explicit group.
* `foo(\d+)(?:\pL+)(\d+)` has one implicit group and two explicit groups.

Turning back to the slot mapping, we can now state it as follows:

* Given a pattern ID `pid`, the slots for its implicit group are always
at `pid * 2` and `pid * 2 + 1`.
* Given a pattern ID `0`, the slots for its explicit groups start
at `group_info.pattern_len() * 2`.
* Given a pattern ID `pid > 0`, the slots for its explicit groups start
immediately following where the slots for the explicit groups of `pid - 1`
end.

In particular, while there is a concrete formula one can use to determine
where the slots for the implicit group of any pattern are, there is no
general formula for determining where the slots for explicit capturing
groups are. This is because each pattern can contain a different number
of groups.

The intended way of getting the slots for a particular capturing group
(whether implicit or explicit) is via the `GroupInfo::slot` or
`GroupInfo::slots` method.

See below for a concrete example of how capturing groups get mapped to
slots.

# Example

This example shows how to build a new `GroupInfo` and query it for
information.

```rust
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;
// The number of patterns being tracked.
assert_eq!(4, info.pattern_len());
// We can query the number of groups for any pattern.
assert_eq!(2, info.group_len(PatternID::must(0)));
assert_eq!(1, info.group_len(PatternID::must(1)));
assert_eq!(5, info.group_len(PatternID::must(2)));
assert_eq!(3, info.group_len(PatternID::must(3)));
// An invalid pattern always has zero groups.
assert_eq!(0, info.group_len(PatternID::must(999)));
// 2 slots per group
assert_eq!(22, info.slot_len());

// We can map a group index for a particular pattern to its name, if
// one exists.
assert_eq!(Some("foo"), info.to_name(PatternID::must(3), 2));
assert_eq!(None, info.to_name(PatternID::must(2), 4));
// Or map a name to its group index.
assert_eq!(Some(1), info.to_index(PatternID::must(0), "foo"));
assert_eq!(Some(2), info.to_index(PatternID::must(3), "foo"));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: mapping from capture groups to slots

This example shows the specific mapping from capture group indices for
each pattern to their corresponding slots. The slot values shown in this
example are considered an API guarantee.

```rust
use regex_automata::util::{captures::GroupInfo, primitives::PatternID};

let info = GroupInfo::new(vec![
    vec![None, Some("foo")],
    vec![None],
    vec![None, None, None, Some("bar"), None],
    vec![None, None, Some("foo")],
])?;

// We first show the slots for each pattern's implicit group.
assert_eq!(Some((0, 1)), info.slots(PatternID::must(0), 0));
assert_eq!(Some((2, 3)), info.slots(PatternID::must(1), 0));
assert_eq!(Some((4, 5)), info.slots(PatternID::must(2), 0));
assert_eq!(Some((6, 7)), info.slots(PatternID::must(3), 0));

// And now we show the slots for each pattern's explicit group.
assert_eq!(Some((8, 9)), info.slots(PatternID::must(0), 1));
assert_eq!(Some((10, 11)), info.slots(PatternID::must(2), 1));
assert_eq!(Some((12, 13)), info.slots(PatternID::must(2), 2));
assert_eq!(Some((14, 15)), info.slots(PatternID::must(2), 3));
assert_eq!(Some((16, 17)), info.slots(PatternID::must(2), 4));
assert_eq!(Some((18, 19)), info.slots(PatternID::must(3), 1));
assert_eq!(Some((20, 21)), info.slots(PatternID::must(3), 2));

// Asking for the slots for an invalid pattern ID or even for an invalid
// group index for a specific pattern will return None. So for example,
// you're guaranteed to not get the slots for a different pattern than the
// one requested.
assert_eq!(None, info.slots(PatternID::must(5), 0));
assert_eq!(None, info.slots(PatternID::must(1), 1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>` — [`GroupInfo`](#groupinfo), [`GroupInfoError`](#groupinfoerror)

- `fn empty() -> GroupInfo` — [`GroupInfo`](#groupinfo)

- `fn to_index(self: &Self, pid: PatternID, name: &str) -> Option<usize>` — [`PatternID`](../../index.md)

- `fn to_name(self: &Self, pid: PatternID, group_index: usize) -> Option<&str>` — [`PatternID`](../../index.md)

- `fn pattern_names(self: &Self, pid: PatternID) -> GroupInfoPatternNames<'_>` — [`PatternID`](../../index.md), [`GroupInfoPatternNames`](#groupinfopatternnames)

- `fn all_names(self: &Self) -> GroupInfoAllNames<'_>` — [`GroupInfoAllNames`](#groupinfoallnames)

- `fn slots(self: &Self, pid: PatternID, group_index: usize) -> Option<(usize, usize)>` — [`PatternID`](../../index.md)

- `fn slot(self: &Self, pid: PatternID, group_index: usize) -> Option<usize>` — [`PatternID`](../../index.md)

- `fn pattern_len(self: &Self) -> usize`

- `fn group_len(self: &Self, pid: PatternID) -> usize` — [`PatternID`](../../index.md)

- `fn all_group_len(self: &Self) -> usize`

- `fn slot_len(self: &Self) -> usize`

- `fn implicit_slot_len(self: &Self) -> usize`

- `fn explicit_slot_len(self: &Self) -> usize`

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for GroupInfo`

- `fn clone(self: &Self) -> GroupInfo` — [`GroupInfo`](#groupinfo)

##### `impl Debug for GroupInfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for GroupInfo`

- `fn default() -> GroupInfo` — [`GroupInfo`](#groupinfo)

### `GroupInfoInner`

```rust
struct GroupInfoInner {
    slot_ranges: alloc::vec::Vec<(crate::util::primitives::SmallIndex, crate::util::primitives::SmallIndex)>,
    name_to_index: alloc::vec::Vec<std::collections::HashMap<alloc::sync::Arc<str>, crate::util::primitives::SmallIndex>>,
    index_to_name: alloc::vec::Vec<alloc::vec::Vec<Option<alloc::sync::Arc<str>>>>,
    memory_extra: usize,
}
```

The inner guts of `GroupInfo`. This type only exists so that it can
be wrapped in an `Arc` to make `GroupInfo` reference counted.

#### Implementations

- `fn add_first_group(self: &mut Self, pid: PatternID)` — [`PatternID`](../../index.md)

- `fn add_explicit_group<N: AsRef<str>>(self: &mut Self, pid: PatternID, group: SmallIndex, maybe_name: Option<N>) -> Result<(), GroupInfoError>` — [`PatternID`](../../index.md), [`SmallIndex`](../primitives/index.md), [`GroupInfoError`](#groupinfoerror)

- `fn fixup_slot_ranges(self: &mut Self) -> Result<(), GroupInfoError>` — [`GroupInfoError`](#groupinfoerror)

- `fn pattern_len(self: &Self) -> usize`

- `fn group_len(self: &Self, pid: PatternID) -> usize` — [`PatternID`](../../index.md)

- `fn small_slot_len(self: &Self) -> SmallIndex` — [`SmallIndex`](../primitives/index.md)

#### Trait Implementations

##### `impl Debug for GroupInfoInner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for GroupInfoInner`

- `fn default() -> GroupInfoInner` — [`GroupInfoInner`](#groupinfoinner)

### `GroupInfoError`

```rust
struct GroupInfoError {
    kind: GroupInfoErrorKind,
}
```

An error that may occur when building a `GroupInfo`.

Building a `GroupInfo` does a variety of checks to make sure the
capturing groups satisfy a number of invariants. This includes, but is not
limited to, ensuring that the first capturing group is unnamed and that
there are no duplicate capture groups for a specific pattern.

#### Implementations

- `fn too_many_patterns(err: PatternIDError) -> GroupInfoError` — [`PatternIDError`](../primitives/index.md), [`GroupInfoError`](#groupinfoerror)

- `fn too_many_groups(pattern: PatternID, minimum: usize) -> GroupInfoError` — [`PatternID`](../../index.md), [`GroupInfoError`](#groupinfoerror)

- `fn missing_groups(pattern: PatternID) -> GroupInfoError` — [`PatternID`](../../index.md), [`GroupInfoError`](#groupinfoerror)

- `fn first_must_be_unnamed(pattern: PatternID) -> GroupInfoError` — [`PatternID`](../../index.md), [`GroupInfoError`](#groupinfoerror)

- `fn duplicate(pattern: PatternID, name: &str) -> GroupInfoError` — [`PatternID`](../../index.md), [`GroupInfoError`](#groupinfoerror)

#### Trait Implementations

##### `impl Clone for GroupInfoError`

- `fn clone(self: &Self) -> GroupInfoError` — [`GroupInfoError`](#groupinfoerror)

##### `impl Debug for GroupInfoError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for GroupInfoError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for GroupInfoError`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for GroupInfoError`

- `fn to_string(self: &Self) -> String`

### `GroupInfoPatternNames<'a>`

```rust
struct GroupInfoPatternNames<'a> {
    it: core::slice::Iter<'a, Option<alloc::sync::Arc<str>>>,
}
```

An iterator over capturing groups and their names for a specific pattern.

This iterator is created by `GroupInfo::pattern_names`.

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

#### Implementations

- `fn empty() -> GroupInfoPatternNames<'static>` — [`GroupInfoPatternNames`](#groupinfopatternnames)

#### Trait Implementations

##### `impl<'a> Clone for GroupInfoPatternNames<'a>`

- `fn clone(self: &Self) -> GroupInfoPatternNames<'a>` — [`GroupInfoPatternNames`](#groupinfopatternnames)

##### `impl<'a> Debug for GroupInfoPatternNames<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> ExactSizeIterator for GroupInfoPatternNames<'a>`

##### `impl<'a> FusedIterator for GroupInfoPatternNames<'a>`

##### `impl<I> IntoIterator for GroupInfoPatternNames<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for GroupInfoPatternNames<'a>`

- `type Item = Option<&'a str>`

- `fn next(self: &mut Self) -> Option<Option<&'a str>>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn count(self: Self) -> usize`

### `GroupInfoAllNames<'a>`

```rust
struct GroupInfoAllNames<'a> {
    group_info: &'a GroupInfo,
    pids: crate::util::primitives::PatternIDIter,
    current_pid: Option<crate::util::primitives::PatternID>,
    names: Option<core::iter::Enumerate<GroupInfoPatternNames<'a>>>,
}
```

An iterator over capturing groups and their names for a `GroupInfo`.

This iterator is created by `GroupInfo::all_names`.

The lifetime parameter `'a` refers to the lifetime of the `GroupInfo`
from which this iterator was created.

#### Trait Implementations

##### `impl<'a> Debug for GroupInfoAllNames<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for GroupInfoAllNames<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for GroupInfoAllNames<'a>`

- `type Item = (PatternID, usize, Option<&'a str>)`

- `fn next(self: &mut Self) -> Option<(PatternID, usize, Option<&'a str>)>` — [`PatternID`](../../index.md)

## Enums

### `GroupInfoErrorKind`

```rust
enum GroupInfoErrorKind {
    TooManyPatterns {
        err: crate::util::primitives::PatternIDError,
    },
    TooManyGroups {
        pattern: crate::util::primitives::PatternID,
        minimum: usize,
    },
    MissingGroups {
        pattern: crate::util::primitives::PatternID,
    },
    FirstMustBeUnnamed {
        pattern: crate::util::primitives::PatternID,
    },
    Duplicate {
        pattern: crate::util::primitives::PatternID,
        name: alloc::string::String,
    },
}
```

The kind of error that occurs when building a `GroupInfo` fails.

We keep this un-exported because it's not clear how useful it is to
export it.

#### Variants

- **`TooManyPatterns`**

  This occurs when too many patterns have been added. i.e., It would
  otherwise overflow a `PatternID`.

- **`TooManyGroups`**

  This occurs when too many capturing groups have been added for a
  particular pattern.

- **`MissingGroups`**

  An error that occurs when a pattern has no capture groups. Either the
  group info must be empty, or all patterns must have at least one group
  (corresponding to the unnamed group for the entire pattern).

- **`FirstMustBeUnnamed`**

  An error that occurs when one tries to provide a name for the capture
  group at index 0. This capturing group must currently always be
  unnamed.

- **`Duplicate`**

  An error that occurs when duplicate capture group names for the same
  pattern are added.
  
  NOTE: At time of writing, this error can never occur if you're using
  regex-syntax, since the parser itself will reject patterns with
  duplicate capture group names. This error can only occur when the
  builder is used to hand construct NFAs.

#### Trait Implementations

##### `impl Clone for GroupInfoErrorKind`

- `fn clone(self: &Self) -> GroupInfoErrorKind` — [`GroupInfoErrorKind`](#groupinfoerrorkind)

##### `impl Debug for GroupInfoErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Type Aliases

### `CaptureNameMap`

```rust
type CaptureNameMap = std::collections::HashMap<alloc::sync::Arc<str>, crate::util::primitives::SmallIndex>;
```

A map from capture group name to its corresponding capture group index.

This type is actually wrapped inside a Vec indexed by pattern ID on a
`GroupInfo`, since multiple patterns may have the same capture group name.
That is, each pattern gets its own namespace of capture group names.

Perhaps a more memory efficient representation would be
HashMap<(PatternID, Arc<str>), usize>, but this makes it difficult to look
up a capture index by name without producing a `Arc<str>`, which requires
an allocation. To fix this, I think we'd need to define our own unsized
type or something? Anyway, I didn't give this much thought since it
probably doesn't matter much in the grand scheme of things. But it did
stand out to me as mildly wasteful.

